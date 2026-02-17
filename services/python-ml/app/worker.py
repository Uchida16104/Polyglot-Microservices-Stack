import uuid
import time
import asyncio
import os
import structlog
import redis.asyncio as aioredis
from datetime import datetime
from sqlalchemy import select

from app.database import AsyncSessionLocal
from app.models_db import MlJob, MlModel

logger = structlog.get_logger()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379")
MAX_RETRIES = 3


async def start_ml_worker():
    logger.info("ML worker started")
    redis = await aioredis.from_url(REDIS_URL)

    while True:
        try:
            result = await redis.brpop("ml_queue", timeout=5)
            if result is None:
                continue

            _, job_id_bytes = result
            job_id = uuid.UUID(job_id_bytes.decode())
            asyncio.create_task(process_ml_job(redis, job_id))

        except Exception as e:
            logger.error("ML worker queue error", error=str(e))
            await asyncio.sleep(2)


async def process_ml_job(redis: aioredis.Redis, job_id: uuid.UUID):
    async with AsyncSessionLocal() as db:
        result = await db.execute(
            select(MlJob).where(MlJob.id == job_id, MlJob.status == "queued")
        )
        job = result.scalar_one_or_none()
        if not job:
            return

        job.status = "running"
        await db.commit()

        start_time = time.monotonic()
        try:
            metrics = await run_job(db, job)
            duration_ms = int((time.monotonic() - start_time) * 1000)

            job.status = "completed"
            job.result_metrics = metrics
            job.duration_ms = duration_ms
            job.completed_at = datetime.utcnow()

            if job.job_type in ("train", "finetune"):
                model_result = await db.execute(
                    select(MlModel).where(MlModel.id == job.model_id)
                )
                model = model_result.scalar_one_or_none()
                if model:
                    model.status = "trained"
                    model.trained_at = datetime.utcnow()

            await db.commit()
            logger.info("ML job completed", job_id=str(job_id), duration_ms=duration_ms)

        except Exception as e:
            duration_ms = int((time.monotonic() - start_time) * 1000)
            logger.error("ML job failed", job_id=str(job_id), error=str(e))

            if job.retry_count < MAX_RETRIES:
                job.status = "queued"
                job.retry_count += 1
                await db.commit()
                await redis.lpush("ml_queue", str(job_id))
            else:
                job.status = "failed"
                job.duration_ms = duration_ms
                job.completed_at = datetime.utcnow()
                await db.commit()


async def run_job(db, job: MlJob) -> dict:
    await asyncio.sleep(0.1)

    config = job.config or {}

    if job.job_type == "train":
        epochs = config.get("epochs", 10)
        await asyncio.sleep(min(epochs * 0.05, 2.0))
        return {
            "final_loss": 0.042,
            "final_accuracy": 0.957,
            "epochs_completed": epochs,
            "framework": "simulated",
        }

    elif job.job_type == "infer":
        return {
            "predictions": [0.91, 0.07, 0.02],
            "confidence": 0.91,
            "latency_ms": 12,
        }

    elif job.job_type == "eval":
        return {
            "accuracy": 0.953,
            "precision": 0.948,
            "recall": 0.961,
            "f1": 0.954,
        }

    elif job.job_type == "finetune":
        return {
            "final_loss": 0.031,
            "final_accuracy": 0.971,
            "steps_completed": config.get("steps", 100),
        }

    return {}
