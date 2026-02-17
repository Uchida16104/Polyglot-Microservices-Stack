import os
import structlog
from contextlib import asynccontextmanager
from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from app.database import engine, Base
from app.routers import jobs, models, analytics
from app.worker import start_ml_worker

structlog.configure(
    processors=[
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.stdlib.add_log_level,
        structlog.processors.JSONRenderer(),
    ]
)

logger = structlog.get_logger()


@asynccontextmanager
async def lifespan(app: FastAPI):
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)
    import asyncio
    asyncio.create_task(start_ml_worker())
    logger.info("python-ml service started")
    yield
    logger.info("python-ml service shutting down")


app = FastAPI(
    title="ML/Data Service",
    version="1.0.0",
    lifespan=lifespan,
    docs_url="/docs" if os.getenv("ENV") != "production" else None,
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.middleware("http")
async def internal_auth_middleware(request: Request, call_next):
    if request.url.path in ("/health", "/docs", "/openapi.json"):
        return await call_next(request)

    user_id = request.headers.get("x-user-id")
    if not user_id:
        return JSONResponse(status_code=401, content={"message": "Unauthorized"})

    request.state.user_id = user_id
    request.state.user_role = request.headers.get("x-user-role", "user")
    return await call_next(request)


app.include_router(jobs.router, prefix="/jobs", tags=["ML Jobs"])
app.include_router(models.router, prefix="/models", tags=["ML Models"])
app.include_router(analytics.router, prefix="/analytics", tags=["Analytics"])


@app.get("/health")
async def health():
    return {"status": "ok", "service": "python-ml"}
