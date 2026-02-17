import uuid
from datetime import datetime
from typing import Any, Optional
from pydantic import BaseModel, Field


class MlModelCreate(BaseModel):
    name: str = Field(..., max_length=255)
    framework: str = Field(..., pattern="^(pytorch|tensorflow|sklearn|xgboost|custom)$")
    version: str = Field(default="1.0.0")
    hyperparams: Optional[dict[str, Any]] = None


class MlModelResponse(BaseModel):
    id: uuid.UUID
    owner_id: uuid.UUID
    name: str
    framework: str
    version: str
    status: str
    artifact_path: Optional[str]
    hyperparams: Optional[dict[str, Any]]
    trained_at: Optional[datetime]
    created_at: datetime

    model_config = {"from_attributes": True}


class MlJobCreate(BaseModel):
    model_id: uuid.UUID
    job_type: str = Field(..., pattern="^(train|infer|eval|finetune)$")
    config: Optional[dict[str, Any]] = None


class MlJobResponse(BaseModel):
    id: uuid.UUID
    model_id: uuid.UUID
    user_id: uuid.UUID
    job_type: str
    status: str
    config: Optional[dict[str, Any]]
    result_metrics: Optional[dict[str, Any]]
    duration_ms: Optional[int]
    retry_count: int
    created_at: datetime
    completed_at: Optional[datetime]

    model_config = {"from_attributes": True}
