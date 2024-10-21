ALTER TABLE worktree_entries ADD COLUMN canonical_path text;
ALTER TABLE worktree_entries ALTER COLUMN is_symlink SET DEFAULT false;
