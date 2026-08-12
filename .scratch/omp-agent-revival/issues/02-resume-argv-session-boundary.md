# 02 — Resume-argv and session-boundary decision logic

**What to build:** The pure, deterministic decision logic that turns a sleeping agent session into a resume command: `get_agent_resume_argv(agent, session, resume_path)` building `omp --resume <resume_path || id>`, plus claim-key derivation and newest-wins dedup, plus the 18-minute staleness judgment. All unit-tested in isolation (S2 seam).

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] `get_agent_resume_argv("omp", session, resumePath)` returns `["omp", "--resume", resumePath || id]`.
- [ ] Claim key is `worktreeId\0agent\0session_key\0session_id`; newer records win over older per claim key.
- [ ] Staleness: `!origin && state==="done"` or `state!=="done" && capturedAt-updatedAt > 18min` marks a record invalid.
- [ ] Session ids reject control characters, leading `-`, and >512 chars.
- [ ] Unit tests cover the argv shape, dedup, and staleness.