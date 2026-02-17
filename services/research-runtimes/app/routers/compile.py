import uuid
import redis.asyncio as aioredis
import os
from fastapi import APIRouter, Depends, Request, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

from app.database import get_db
from app.models_db import ResearchJob
from app.schemas.compile import ResearchJobCreate, ResearchJobResponse

router = APIRouter()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379")


async def get_redis():
    r = await aioredis.from_url(REDIS_URL)
    try:
        yield r
    finally:
        await r.aclose()


@router.get("/", response_model=list[ResearchJobResponse])
async def list_jobs(
    request: Request,
    runtime: str | None = None,
    page: int = 1,
    page_size: int = 20,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    query = select(ResearchJob).where(ResearchJob.user_id == user_id)
    if runtime:
        query = query.where(ResearchJob.runtime == runtime)
    query = query.order_by(ResearchJob.created_at.desc()).offset((page - 1) * page_size).limit(page_size)
    result = await db.execute(query)
    return result.scalars().all()


@router.post("/", response_model=ResearchJobResponse, status_code=202)
async def submit_job(
    request: Request,
    payload: ResearchJobCreate,
    db: AsyncSession = Depends(get_db),
    redis: aioredis.Redis = Depends(get_redis),
):
    user_id = uuid.UUID(request.state.user_id)

    job = ResearchJob(
        id=uuid.uuid4(),
        user_id=user_id,
        runtime=payload.runtime,
        source_code=payload.source_code,
        flags=payload.flags,
        status="queued",
    )
    db.add(job)
    await db.commit()
    await db.refresh(job)

    await redis.lpush("research_queue", str(job.id))

    return job


@router.get("/{job_id}", response_model=ResearchJobResponse)
async def get_job(
    job_id: uuid.UUID,
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(ResearchJob).where(ResearchJob.id == job_id, ResearchJob.user_id == user_id)
    )
    job = result.scalar_one_or_none()
    if not job:
        raise HTTPException(status_code=404, detail="Research job not found")
    return job
