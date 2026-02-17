import uuid
import time
import asyncio
import os
import structlog
import redis.asyncio as aioredis
from datetime import datetime
from sqlalchemy import select

from app.database import AsyncSessionLocal
from app.models_db import ResearchJob
from app.compiler import dispatch

logger = structlog.get_logger()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379")
MAX_RETRIES = 3


async def start_research_worker():
    logger.info("Research runtimes worker started")
    redis = await aioredis.from_url(REDIS_URL)

    while True:
        try:
            result = await redis.brpop("research_queue", timeout=5)
            if result is None:
                continue
            _, job_id_bytes = result
            job_id = uuid.UUID(job_id_bytes.decode())
            asyncio.create_task(process_research_job(redis, job_id))
        except Exception as e:
            logger.error("Research worker queue error", error=str(e))
            await asyncio.sleep(2)


async def process_research_job(redis: aioredis.Redis, job_id: uuid.UUID):
    async with AsyncSessionLocal() as db:
        result = await db.execute(
            select(ResearchJob).where(ResearchJob.id == job_id, ResearchJob.status == "queued")
        )
        job = result.scalar_one_or_none()
        if not job:
            return

        job.status = "running"
        await db.commit()

        start_time = time.monotonic()
        try:
            compile_result = await dispatch(job.runtime, job.source_code, job.flags)
            duration_ms = int((time.monotonic() - start_time) * 1000)

            job.status = "completed" if compile_result.exit_code == 0 else "failed"
            job.verification_output = compile_result.verification_output
            job.execution_output = compile_result.execution_output
            job.error_output = compile_result.error_output
            job.exit_code = compile_result.exit_code
            job.duration_ms = duration_ms
            job.completed_at = datetime.utcnow()
            await db.commit()

            logger.info(
                "Research job finished",
                job_id=str(job_id),
                runtime=job.runtime,
                status=job.status,
                duration_ms=duration_ms,
            )

        except Exception as e:
            duration_ms = int((time.monotonic() - start_time) * 1000)
            logger.error("Research job error", job_id=str(job_id), error=str(e))

            if job.retry_count < MAX_RETRIES:
                job.status = "queued"
                job.retry_count += 1
                await db.commit()
                await redis.lpush("research_queue", str(job_id))
            else:
                job.status = "failed"
                job.error_output = str(e)
                job.duration_ms = duration_ms
                job.completed_at = datetime.utcnow()
                await db.commit()
