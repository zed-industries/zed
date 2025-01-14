ALTER TABLE worktree_repository_statuses
ADD COLUMN status_kind INTEGER NOT NULL DEFAULT 0,
ADD COLUMN first_status INTEGER,
ADD COLUMN second_status INTEGER,
DROP COLUMN status;
