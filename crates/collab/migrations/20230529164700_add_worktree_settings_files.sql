CREATE TABLE "worktree_settings_files" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INT8 NOT NULL,
    "path" VARCHAR NOT NULL,
    "content" TEXT NOT NULL,
    PRIMARY KEY(project_id, worktree_id, path),
    FOREIGN KEY(project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);
CREATE INDEX "index_settings_files_on_project_id" ON "worktree_settings_files" ("project_id");
CREATE INDEX "index_settings_files_on_project_id_and_wt_id" ON "worktree_settings_files" ("project_id", "worktree_id");
