CREATE TABLE "project_repositories" (
    "project_id" INTEGER NOT NULL,
    "abs_path" VARCHAR,
    "id" INTEGER NOT NULL,
    "entry_ids" INTEGER ARRAY NOT NULL,
    "branch" VARCHAR,
    "scan_id" INTEGER NOT NULL,
    "is_deleted" BOOL NOT NULL,
    "current_merge_conflicts" VARCHAR,
    "branch_summary" VARCHAR,
);

CREATE INDEX "index_project_repositories_on_project_id" ON "project_repositories" ("project_id");

CREATE TABLE "project_repository_statuses" (
    "project_id" INTEGER NOT NULL,
    "id" INTEGER NOT NULL,
    "repo_path" VARCHAR NOT NULL,
    "status" INT8 NOT NULL,
    "status_kind" INT4 NOT NULL,
    "first_status" INT4 NULL,
    "second_status" INT4 NULL,
    "scan_id" INT8 NOT NULL,
    "is_deleted" BOOL NOT NULL,
    PRIMARY KEY (project_id, id, repo_path),
);

CREATE INDEX "index_project_repos_statuses_on_project_id" ON "worktree_repository_statuses" ("project_id");

CREATE INDEX "index_project_repos_statuses_on_project_id_and_repo_id" ON "worktree_repository_statuses" ("project_id", "id");
