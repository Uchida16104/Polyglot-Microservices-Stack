CREATE TABLE IF NOT EXISTS ml_models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name STRING NOT NULL,
    framework STRING NOT NULL CHECK (framework IN ('pytorch','tensorflow','sklearn','xgboost','custom')),
    version STRING NOT NULL DEFAULT '1.0.0',
    status STRING NOT NULL DEFAULT 'untrained' CHECK (status IN ('untrained','training','trained','failed','deprecated')),
    artifact_path STRING,
    hyperparams JSONB,
    trained_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS ml_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model_id UUID NOT NULL REFERENCES ml_models(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    job_type STRING NOT NULL CHECK (job_type IN ('train','infer','eval','finetune')),
    status STRING NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','running','completed','failed','cancelled')),
    config JSONB,
    result_metrics JSONB,
    log_path STRING,
    duration_ms INT,
    retry_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    service STRING NOT NULL,
    action STRING NOT NULL,
    resource_type STRING NOT NULL,
    resource_id STRING,
    metadata JSONB,
    ip_address STRING,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_ml_models_owner_id ON ml_models(owner_id);
CREATE INDEX idx_ml_jobs_model_id ON ml_jobs(model_id);
CREATE INDEX idx_ml_jobs_user_id ON ml_jobs(user_id);
CREATE INDEX idx_ml_jobs_status ON ml_jobs(status);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
