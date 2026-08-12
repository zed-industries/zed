# 06 — Manual sleep, staleness, and close cleanup

**What to build:** A manually slept session resumes only when its tab is opened (restore-on-tab-open), not on activation. Stale records are cleaned up per the staleness rule, and explicitly closing a terminal clears its sleeping record so it never resurrects.

**Blocked by:** 04 — Sleeping transition and auto-resume on load.

**Status:** ready-for-agent

- [ ] A manually slept session does not auto-resume on activation; it resumes only when its tab opens.
- [ ] Stale sleeping records (invalid per the staleness rule) are cleared.
- [ ] Explicitly closing a terminal clears its sleeping record.
- [ ] A cleared record never comes back on a later activation.