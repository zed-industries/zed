CREATE TABLE "worktree_repositories" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INT8 NOT NULL,
    "work_directory_id" INT8 NOT NULL,
    "scan_id" INT8 NOT NULL,
    "branch" VARCHAR,
    "is_deleted" BOOL NOT NULL,
    PRIMARY KEY(project_id, worktree_id, work_directory_id),
    FOREIGN KEY(project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE,
    FOREIGN KEY(project_id, worktree_id, work_directory_id) REFERENCES worktree_entries (project_id, worktree_id, id) ON DELETE CASCADE
);
CREATE INDEX "index_worktree_repositories_on_project_id" ON "worktree_repositories" ("project_id");
CREATE INDEX "index_worktree_repositories_on_project_id_and_worktree_id" ON "worktree_repositories" ("project_id", "worktree_id");
