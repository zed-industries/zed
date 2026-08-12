# 05 — Restored banner and resume failure UX

**What to build:** A successfully resumed session shows a "session restored (new process, same session)" banner so the user knows the conversation continued but the process is new. A failed resume leaves a usable shell with an explicit error; it never silently starts a new agent session.

**Blocked by:** 04 — Sleeping transition and auto-resume on load.

**Status:** ready-for-agent

- [ ] Resumed sessions show the restored banner.
- [ ] A failed resume shows an explicit error and keeps a usable shell.
- [ ] A failed resume does not silently launch a fresh agent.
- [ ] The banner text makes clear the process is new even though the session resumed.