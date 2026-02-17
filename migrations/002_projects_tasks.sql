CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name STRING NOT NULL,
    description STRING,
    visibility STRING NOT NULL DEFAULT 'private' CHECK (visibility IN ('public','private','team')),
    status STRING NOT NULL DEFAULT 'active' CHECK (status IN ('active','archived','deleted')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS project_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role STRING NOT NULL DEFAULT 'member' CHECK (role IN ('owner','admin','member','viewer')),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (project_id, user_id)
);

CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    assignee_id UUID REFERENCES users(id) ON DELETE SET NULL,
    creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title STRING NOT NULL,
    description STRING,
    priority STRING NOT NULL DEFAULT 'medium' CHECK (priority IN ('low','medium','high','critical')),
    status STRING NOT NULL DEFAULT 'backlog' CHECK (status IN ('backlog','todo','in_progress','blocked','in_review','done','cancelled')),
    order_index INT NOT NULL DEFAULT 0,
    due_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    uploader_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    filename STRING NOT NULL,
    path STRING NOT NULL,
    mime_type STRING NOT NULL,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    checksum STRING NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_assignee_id ON tasks(assignee_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_project_members_project_id ON project_members(project_id);
CREATE INDEX idx_project_members_user_id ON project_members(user_id);
CREATE INDEX idx_files_project_id ON files(project_id);
