import uuid
import redis.asyncio as aioredis
import os
from fastapi import APIRouter, Depends, Request, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

from app.database import get_db
from app.models_db import MlJob, MlModel
from app.schemas.ml_job import MlJobCreate, MlJobResponse

router = APIRouter()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379")


async def get_redis():
    r = await aioredis.from_url(REDIS_URL)
    try:
        yield r
    finally:
        await r.aclose()


@router.get("/", response_model=list[MlJobResponse])
async def list_jobs(
    request: Request,
    page: int = 1,
    page_size: int = 20,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlJob)
        .where(MlJob.user_id == user_id)
        .order_by(MlJob.created_at.desc())
        .offset((page - 1) * page_size)
        .limit(page_size)
    )
    return result.scalars().all()


@router.post("/", response_model=MlJobResponse, status_code=202)
async def create_job(
    request: Request,
    payload: MlJobCreate,
    db: AsyncSession = Depends(get_db),
    redis: aioredis.Redis = Depends(get_redis),
):
    user_id = uuid.UUID(request.state.user_id)

    model_result = await db.execute(
        select(MlModel).where(MlModel.id == payload.model_id)
    )
    model = model_result.scalar_one_or_none()
    if not model:
        raise HTTPException(status_code=404, detail="ML model not found")

    if payload.job_type in ("infer", "eval") and model.status != "trained":
        raise HTTPException(status_code=400, detail="Model must be in 'trained' state for inference/evaluation")

    job = MlJob(
        id=uuid.uuid4(),
        model_id=payload.model_id,
        user_id=user_id,
        job_type=payload.job_type,
        status="queued",
        config=payload.config,
    )
    db.add(job)
    await db.commit()
    await db.refresh(job)

    await redis.lpush("ml_queue", str(job.id))

    return job


@router.get("/{job_id}", response_model=MlJobResponse)
async def get_job(
    job_id: uuid.UUID,
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlJob).where(MlJob.id == job_id, MlJob.user_id == user_id)
    )
    job = result.scalar_one_or_none()
    if not job:
        raise HTTPException(status_code=404, detail="Job not found")
    return job
