CREATE TABLE IF NOT EXISTS "projects" (
    "id" SERIAL PRIMARY KEY,
    "host_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "unregistered" BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS "worktree_extensions" (
    "id" SERIAL PRIMARY KEY,
    "project_id" INTEGER REFERENCES projects (id) NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "extension" VARCHAR(255),
    "count" INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS "project_activity_periods" (
    "id" SERIAL PRIMARY KEY,
    "duration_millis" INTEGER NOT NULL,
    "ended_at" TIMESTAMP NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "project_id" INTEGER REFERENCES projects (id) NOT NULL
);

CREATE INDEX "index_project_activity_periods_on_ended_at" ON "project_activity_periods" ("ended_at");
CREATE UNIQUE INDEX "index_worktree_extensions_on_project_id_and_worktree_id_and_extension" ON "worktree_extensions" ("project_id", "worktree_id", "extension");