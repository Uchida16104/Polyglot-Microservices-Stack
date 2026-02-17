import uuid
from datetime import datetime
from sqlalchemy import String, Integer, DateTime, Text, ForeignKey
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy.dialects.postgresql import UUID

from app.database import Base


class ResearchJob(Base):
    __tablename__ = "research_jobs"

    id: Mapped[uuid.UUID] = mapped_column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    compile_job_id: Mapped[uuid.UUID | None] = mapped_column(UUID(as_uuid=True), nullable=True)
    user_id: Mapped[uuid.UUID] = mapped_column(UUID(as_uuid=True), nullable=False)
    runtime: Mapped[str] = mapped_column(String(50), nullable=False)
    source_code: Mapped[str] = mapped_column(Text, nullable=False)
    flags: Mapped[str | None] = mapped_column(Text)
    status: Mapped[str] = mapped_column(String(50), default="queued")
    verification_output: Mapped[str | None] = mapped_column(Text)
    execution_output: Mapped[str | None] = mapped_column(Text)
    error_output: Mapped[str | None] = mapped_column(Text)
    exit_code: Mapped[int | None] = mapped_column(Integer)
    duration_ms: Mapped[int | None] = mapped_column(Integer)
    retry_count: Mapped[int] = mapped_column(Integer, default=0)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.utcnow)
    completed_at: Mapped[datetime | None] = mapped_column(DateTime(timezone=True))
