import uuid
from fastapi import APIRouter, Depends, Request
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func

from app.database import get_db
from app.models_db import MlJob, MlModel

router = APIRouter()


@router.get("/summary")
async def get_summary(
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)

    total_models = await db.scalar(
        select(func.count()).select_from(MlModel).where(MlModel.owner_id == user_id)
    )
    trained_models = await db.scalar(
        select(func.count()).select_from(MlModel).where(
            MlModel.owner_id == user_id, MlModel.status == "trained"
        )
    )
    total_jobs = await db.scalar(
        select(func.count()).select_from(MlJob).where(MlJob.user_id == user_id)
    )
    completed_jobs = await db.scalar(
        select(func.count()).select_from(MlJob).where(
            MlJob.user_id == user_id, MlJob.status == "completed"
        )
    )
    failed_jobs = await db.scalar(
        select(func.count()).select_from(MlJob).where(
            MlJob.user_id == user_id, MlJob.status == "failed"
        )
    )

    return {
        "models": {"total": total_models, "trained": trained_models},
        "jobs": {"total": total_jobs, "completed": completed_jobs, "failed": failed_jobs},
    }


@router.get("/jobs/by-status")
async def jobs_by_status(
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlJob.status, func.count().label("count"))
        .where(MlJob.user_id == user_id)
        .group_by(MlJob.status)
    )
    return [{"status": row.status, "count": row.count} for row in result.all()]
