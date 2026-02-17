CREATE TABLE IF NOT EXISTS compile_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    language STRING NOT NULL,
    source_code STRING NOT NULL,
    compiler_flags STRING,
    status STRING NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','running','completed','failed','cancelled')),
    output STRING,
    error_output STRING,
    exit_code INT,
    duration_ms INT,
    retry_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS execute_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    compile_job_id UUID NOT NULL REFERENCES compile_jobs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stdin_data STRING,
    stdout_data STRING,
    stderr_data STRING,
    exit_code INT,
    duration_ms INT,
    status STRING NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','running','completed','failed','cancelled')),
    retry_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_compile_jobs_user_id ON compile_jobs(user_id);
CREATE INDEX idx_compile_jobs_project_id ON compile_jobs(project_id);
CREATE INDEX idx_compile_jobs_status ON compile_jobs(status);
CREATE INDEX idx_execute_jobs_compile_job_id ON execute_jobs(compile_job_id);
CREATE INDEX idx_execute_jobs_user_id ON execute_jobs(user_id);
