# 04 — Sleeping transition and auto-resume on load

**What to build:** An OMP agent terminal transitions to `sleeping` when its process exits or the app quits. On worktree/panel activation, Zed automatically resumes sleeping sessions exactly once, using claim-key fencing so duplicate or older records are cleared and only one resume launches. This is the primary user-visible behavior (P seam), covered by an AgentPanel GPUI test.

**Blocked by:** 02 — Resume-argv and session-boundary decision logic; 03 — Dedicated OMP agent terminal entry.

**Status:** ready-for-agent

- [ ] OMP process exit or app quit marks the terminal `sleeping`.
- [ ] Reopening the worktree auto-resumes the sleeping session exactly once.
- [ ] Duplicate or older records for the same claim key are cleared, not double-resumed.
- [ ] An already-live or queued session is not resumed again.
- [ ] AgentPanel GPUI test asserts exactly-once auto-resume on load.