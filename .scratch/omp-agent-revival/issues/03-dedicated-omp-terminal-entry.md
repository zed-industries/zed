# 03 — Dedicated OMP agent terminal entry

**What to build:** The agent panel gains a dedicated "new OMP agent terminal" entry, distinct from plain terminal threads that reuse `terminal_init_command`. Creating it assigns a per-terminal resume path, launches OMP with `--session-dir <path>`, records the session as `live`, and reads a minimal OMP settings block for the program path.

**Blocked by:** 01 — Revival metadata schema and persistence.

**Status:** ready-for-agent

- [ ] The dedicated entry creates an OMP terminal separate from `terminal_init_command` shells.
- [ ] Creating it assigns a resume path and launches `omp --session-dir <path>`.
- [ ] The terminal is persisted in the `live` session-boundary state.
- [ ] A plain shell that later runs `omp` is not promoted to a revivable session.