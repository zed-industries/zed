CREATE TABLE IF NOT EXISTS "breakpoints" (
    "id" SERIAL PRIMARY KEY,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "position" INTEGER NOT NULL,
    "log_message" TEXT NULL,
    "worktree_id" BIGINT NOT NULL,
    "path" TEXT NOT NULL,
    "kind" VARCHAR NOT NULL
);

CREATE INDEX "index_breakpoints_on_project_id" ON "breakpoints" ("project_id");
