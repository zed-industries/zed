ALTER TABLE worktree_entries ADD COLUMN canonical_path text;
ALTER TABLE worktree_entries DROP COLUMN is_symlink;
