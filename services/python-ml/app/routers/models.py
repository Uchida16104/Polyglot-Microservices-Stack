import uuid
from fastapi import APIRouter, Depends, Request, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

from app.database import get_db
from app.models_db import MlModel
from app.schemas.ml_job import MlModelCreate, MlModelResponse

router = APIRouter()


@router.get("/", response_model=list[MlModelResponse])
async def list_models(
    request: Request,
    page: int = 1,
    page_size: int = 20,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlModel)
        .where(MlModel.owner_id == user_id)
        .where(MlModel.status != "deprecated")
        .order_by(MlModel.created_at.desc())
        .offset((page - 1) * page_size)
        .limit(page_size)
    )
    return result.scalars().all()


@router.post("/", response_model=MlModelResponse, status_code=201)
async def create_model(
    request: Request,
    payload: MlModelCreate,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    model = MlModel(
        id=uuid.uuid4(),
        owner_id=user_id,
        name=payload.name,
        framework=payload.framework,
        version=payload.version,
        hyperparams=payload.hyperparams,
        status="untrained",
    )
    db.add(model)
    await db.commit()
    await db.refresh(model)
    return model


@router.get("/{model_id}", response_model=MlModelResponse)
async def get_model(
    model_id: uuid.UUID,
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlModel).where(MlModel.id == model_id, MlModel.owner_id == user_id)
    )
    model = result.scalar_one_or_none()
    if not model:
        raise HTTPException(status_code=404, detail="Model not found")
    return model


@router.delete("/{model_id}", status_code=204)
async def delete_model(
    model_id: uuid.UUID,
    request: Request,
    db: AsyncSession = Depends(get_db),
):
    user_id = uuid.UUID(request.state.user_id)
    result = await db.execute(
        select(MlModel).where(MlModel.id == model_id, MlModel.owner_id == user_id)
    )
    model = result.scalar_one_or_none()
    if not model:
        raise HTTPException(status_code=404, detail="Model not found")
    model.status = "deprecated"
    await db.commit()
