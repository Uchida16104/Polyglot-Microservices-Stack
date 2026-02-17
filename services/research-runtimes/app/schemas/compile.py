import uuid
from datetime import datetime
from typing import Optional, Literal
from pydantic import BaseModel, Field

ResearchRuntime = Literal["zig", "mojo", "dafny", "fstar"]


class ResearchJobCreate(BaseModel):
    runtime: ResearchRuntime
    source_code: str = Field(..., min_length=1)
    flags: Optional[str] = None


class ResearchJobResponse(BaseModel):
    id: uuid.UUID
    user_id: uuid.UUID
    runtime: str
    source_code: str
    flags: Optional[str]
    status: str
    verification_output: Optional[str]
    execution_output: Optional[str]
    error_output: Optional[str]
    exit_code: Optional[int]
    duration_ms: Optional[int]
    retry_count: int
    created_at: datetime
    completed_at: Optional[datetime]

    model_config = {"from_attributes": True}
