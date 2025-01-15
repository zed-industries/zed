ALTER TABLE worktree_repository_statuses
ADD COLUMN status_kind INTEGER,
ADD COLUMN first_status INTEGER,
ADD COLUMN second_status INTEGER;

UPDATE worktree_repository_statuses
SET
    status_kind = 0;

ALTER TABLE worktree_repository_statuses
ALTER COLUMN status_kind
SET
    NOT NULL;
