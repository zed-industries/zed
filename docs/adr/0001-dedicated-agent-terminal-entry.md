# Dedicated agent-terminal entry kind with Zed-controlled resume path

The agent panel gains a dedicated "new OMP agent terminal" entry, separate from plain terminal threads that reuse `terminal_init_command`. Zed assigns each such terminal its own resume path and launches OMP with `--session-dir <path>`; revival relaunches with `omp --resume <path>`. The opaque resume path is the persisted locator, never the command line or environment.

**Considered:** reusing `terminal_init_command` with a resume flag — rejected because a single opaque shell string cannot distinguish fresh launch from resume, select a harness, or interpolate a per-thread identity. **Consequences:** a new terminal entry kind and a DB column migration on `sidebar_terminal_threads`.