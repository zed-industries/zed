CREATE TABLE "worktree_repository_statuses" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INT8 NOT NULL,
    "work_directory_id" INT8 NOT NULL,
    "repo_path" VARCHAR NOT NULL,
    "status" INT8 NOT NULL,
    "scan_id" INT8 NOT NULL,
    "is_deleted" BOOL NOT NULL,
    PRIMARY KEY(project_id, worktree_id, work_directory_id, repo_path),
    FOREIGN KEY(project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE,
    FOREIGN KEY(project_id, worktree_id, work_directory_id) REFERENCES worktree_entries (project_id, worktree_id, id) ON DELETE CASCADE
);
CREATE INDEX "index_wt_repos_statuses_on_project_id" ON "worktree_repository_statuses" ("project_id");
CREATE INDEX "index_wt_repos_statuses_on_project_id_and_wt_id" ON "worktree_repository_statuses" ("project_id", "worktree_id");
CREATE INDEX "index_wt_repos_statuses_on_project_id_and_wt_id_and_wd_id" ON "worktree_repository_statuses" ("project_id", "worktree_id", "work_directory_id");
