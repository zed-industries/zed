DROP TABLE "worktree_repository_statuses";

ALTER TABLE "worktree_entries"
ADD "git_status" INT8;
