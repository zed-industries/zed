ALTER TABLE worktrees
    ALTER COLUMN is_complete SET DEFAULT FALSE,
    ADD COLUMN completed_scan_id INT8;
