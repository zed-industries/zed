# Potentially Related GitHub Issues

## High Confidence
- None found

## Medium Confidence
- [#47682](https://github.com/zed-industries/zed/issues/47682) — Support git reftables via a fallback to the git CLI
  - Why: This PR already establishes the pattern of falling back to git CLI when libgit2 fails
  - Evidence: Same subsystem (git repository operations), same solution pattern (CLI fallback)

- [#33499](https://github.com/zed-industries/zed/pull/33499) — Fix crash in git checkout (uses git cli to avoid libgit2 crash)
  - Why: Previous libgit2 crash fix that used the same approach (CLI instead of libgit2)
  - Evidence: Same root cause category (libgit2 crash), same fix pattern (use git CLI)

- [#29351](https://github.com/zed-industries/zed/pull/29351) — git: Use the CLI for loading commit SHAs and details
  - Why: Moved git operations from libgit2 to CLI to avoid deadlocks/crashes
  - Evidence: Same subsystem, same motivation (libgit2 reliability issues)

## Low Confidence
- [#46747](https://github.com/zed-industries/zed/issues/46747) — Git status indicators not working with reftable storage backend
  - Why: Related to libgit2 limitations, though different symptom (no indicators vs crash)
  - Evidence: Same underlying cause (libgit2 limitations), different manifestation

- [#33438](https://github.com/zed-industries/zed/issues/33438) — Zed crashed when trying to switch branch
  - Why: libgit2 crash during git operations
  - Evidence: Same crash type (memory access), different operation (checkout vs blob read)

## Reviewer Checklist
- [ ] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [ ] Reject false positives before merge
