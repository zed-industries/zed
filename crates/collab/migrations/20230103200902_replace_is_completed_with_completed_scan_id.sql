ALTER TABLE worktrees
    DROP COLUMN is_complete,
    ADD COLUMN completed_scan_id INT8;
