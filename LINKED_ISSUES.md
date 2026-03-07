# Linked Issues for ZED-4GH

## High Confidence

No high-confidence related issues found. This appears to be a novel crash pattern related to executor shutdown timing during directory scanning.

## Medium Confidence

### #39191 - worktree: Remove unwrap in BackgroundScanner::update_ignore_status
- **URL:** https://github.com/zed-industries/zed/pull/39191
- **Status:** Closed (merged 2025-09-30)
- **Relevance:** Similar pattern of crash in BackgroundScanner during worktree operations. The PR addresses panics in the ignore status update path, which is part of the same scanning pipeline.

### #9174 - panic `called Option::unwrap() on a None value` in build_git_repository
- **URL:** https://github.com/zed-industries/zed/issues/9174
- **Status:** Unknown
- **Relevance:** Related crash in BackgroundScannerState during worktree operations. While not the same root cause, it demonstrates similar crash patterns in the background scanner code.

## Low Confidence

### #8528 - EXC_CRASH (SIGABRT) when opening 60MB .sql file
- **URL:** https://github.com/zed-industries/zed/issues/8528
- **Status:** Unknown
- **Relevance:** SIGABRT crash, but different cause (large file handling rather than executor shutdown).

### #19678 - Fix crash in collab when sending worktree updates
- **URL:** https://github.com/zed-industries/zed/pull/19678
- **Status:** Closed (merged 2024-10-24)
- **Relevance:** Worktree-related crash fix, but focused on collab/SSH server path rather than local fs metadata operations.

## Notes

The crash pattern ("Task polled after completion") is specific to async-task cancellation behavior and has not been previously reported as a Zed GitHub issue. The related commit `97b42c276c` (on branch `morgan/zed-4gh-fs-metadata-fix`) demonstrates that this issue was identified and a fix was developed but not yet merged to main.
