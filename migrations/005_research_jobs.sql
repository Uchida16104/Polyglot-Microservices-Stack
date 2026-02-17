CREATE TABLE IF NOT EXISTS research_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    compile_job_id UUID REFERENCES compile_jobs(id) ON DELETE SET NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    runtime STRING NOT NULL CHECK (runtime IN ('zig','mojo','dafny','fstar')),
    source_code STRING NOT NULL,
    flags STRING,
    status STRING NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','running','completed','failed','cancelled')),
    verification_output STRING,
    execution_output STRING,
    error_output STRING,
    exit_code INT,
    duration_ms INT,
    retry_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_research_jobs_user_id ON research_jobs(user_id);
CREATE INDEX IF NOT EXISTS idx_research_jobs_runtime ON research_jobs(runtime);
CREATE INDEX IF NOT EXISTS idx_research_jobs_status ON research_jobs(status);
CREATE INDEX IF NOT EXISTS idx_research_jobs_created_at ON research_jobs(created_at DESC);
