# Crash Analysis: libgit2 pack file memory access during blob lookup

## Crash Summary
- **Sentry Issue:** ZED-1F (https://sentry.io/organizations/zed-dev/issues/6798101892/)
- **Error:** EXC_BAD_ACCESS / KERN_MEMORY_ERROR / 0x130c6c3a0
- **Crash Site:** `git2::repo::Repository::find_blob` called from `git::repository::RealGitRepository::load_index_text`
- **First Seen:** 2025-08-08
- **Event Count:** 936 crashes

## Root Cause

The crash occurs in `load_index_text` when calling `repo.find_blob(oid)`. The stack trace shows the crash originates deep in libgit2's pack file handling code:

```
pack_entry_find_offset
git_pack_entry_find
pack_backend__read
git_odb_read
git_object_lookup_prefix
git2::repo::Repository::find_blob
```

This is a memory access violation caused by **pack file invalidation during concurrent operations**. The sequence is:

1. Zed calls `load_index_text` which reads the git index to find an OID for a file
2. libgit2 has memory-mapped pack files containing git objects
3. An external process (e.g., `git gc`, `git fetch`, `git repack`) modifies or replaces pack files
4. When `find_blob(oid)` attempts to read from the pack file, it accesses memory that has been unmapped or remapped, causing SIGBUS/EXC_BAD_ACCESS

According to libgit2's threading documentation, most objects are NOT thread-safe and pack file operations can be affected by external processes modifying the repository. The `git_odb` object has internal locking, but this doesn't protect against the underlying files being modified by external processes.

This crash is fundamentally a **race condition between Zed and external git operations**, not a bug in Zed's own code logic. The issue is that libgit2's error handling cannot catch memory access violations that occur when memory-mapped files become invalid.

## Reproduction

This crash is difficult to reproduce deterministically in tests because it requires:
1. A blob lookup to be in progress
2. An external process to modify/replace the pack file at exactly the right moment
3. The memory mapping to become invalid before the read completes

A theoretical test would:
1. Create a git repository with a file
2. Start a `load_index_text` operation
3. Run `git gc` concurrently to repack objects
4. The `find_blob` call may crash if timing aligns

However, this is inherently timing-dependent and may not reliably reproduce.

## Suggested Fix

Since we cannot prevent libgit2 from crashing on invalid memory mappings, the fix should focus on **reducing the window of opportunity** for the race condition and **gracefully handling errors**:

### Option A: Use git CLI instead of libgit2 for blob content (Recommended)

Replace the `find_blob` call with a git CLI command (`git cat-file blob <oid>`) which:
- Runs in a separate process
- Has its own file handles that won't be affected by pack file changes mid-operation
- Will return an error instead of crashing if the object is missing

This is the most robust solution as it completely avoids the memory mapping issue.

### Option B: Catch panics around find_blob (Not viable)

Using `std::panic::catch_unwind` cannot catch memory access violations (SIGBUS/EXC_BAD_ACCESS) as they are signals, not Rust panics.

### Option C: Refresh repository state before blob lookup

Call `repo.odb()?.refresh()` before `find_blob` to refresh libgit2's view of pack files. This reduces but doesn't eliminate the race window.

### Recommendation

**Option A** is recommended because:
1. It eliminates the crash entirely by using process isolation
2. The git CLI handles pack file changes gracefully
3. This pattern is already used elsewhere in the codebase for similar operations
4. The performance impact is acceptable since `load_index_text` is already async

## Implemented Fix

The fix replaces libgit2's `find_blob` calls with git CLI commands:

1. `load_index_text`: Now uses `git show :0:<path>` to read file content from the index
2. `load_committed_text`: Now uses `git show HEAD:<path>` to read file content from HEAD  
3. `load_blob_content`: Now uses `git cat-file blob <oid>` to read blob content by OID

This approach:
- Runs git operations in separate processes, avoiding memory mapping issues
- Returns graceful errors instead of crashing when objects are missing
- Follows the established pattern in the codebase (see #33499, #29351 for precedent)

## Reproduction Test

A test `test_load_index_and_committed_text` was added that verifies:
- `load_index_text` correctly reads staged file content
- `load_committed_text` correctly reads committed file content
- Both functions return `None` for non-existent files

Run with: `cargo test -p git test_load_index_and_committed_text`
