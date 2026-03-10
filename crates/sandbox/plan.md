<!-- DO NOT CHECK IN OR DELETE THIS FILE. It is a working plan for the sandbox implementation. -->

# Sandbox implementation plan

See `README.md` for the design rationale behind each decision here.

## Phase 1: Sandbox crate extraction

Move sandbox code out of the `terminal` crate into the `sandbox` crate so
that process-tracking logic has a proper home and can be used by both the
terminal spawn path and the cleanup path.

**1.1 Move existing sandbox modules**

Move the following files from `crates/terminal/src/` to `crates/sandbox/src/`:
- `sandbox_exec.rs` → entry point for `--sandbox-exec`
- `sandbox_macos.rs` → Seatbelt SBPL generation and application
- `sandbox_linux.rs` → Landlock implementation
- `sandbox_tests.rs` → tests

Update `crates/terminal/Cargo.toml` to depend on `sandbox`, and update
`terminal.rs` to re-export or delegate to the sandbox crate.

**1.2 Move `SandboxConfig` and related types**

Move `SandboxConfig`, `ResolvedSystemPaths`, and `SandboxConfig::from_settings`
from `terminal_settings.rs` into the sandbox crate. The terminal crate
re-exports these types for backward compatibility.

**1.3 Extract shared sandbox resolution logic**

The sandbox config resolution logic is currently duplicated between
`crates/project/src/terminals.rs` and `crates/acp_thread/src/terminal.rs`.
Extract this into a shared helper on `SandboxConfig` (or a new function in the
sandbox crate) that both call sites use. This addresses code review item #5.

## Phase 2: Session fingerprint (macOS)

Implement the sandbox fingerprint mechanism so that every terminal session's
processes can be reliably identified via `sandbox_check()`.

**2.1 Add `SessionFingerprint` type**

Create a `SessionFingerprint` struct that generates and manages the per-session
UUID marker:

- `SessionFingerprint::new()` — generates a UUID, creates
  `/tmp/.zed-sandbox-<uuid>/allow/` and the parent directory (but not
  `/tmp/.zed-sandbox-<uuid>/deny/`)
- `SessionFingerprint::matches_pid(pid) -> bool` — probes the process with
  `sandbox_check()` using the two-point allow/deny test
- `SessionFingerprint::cleanup()` — deletes the temporary directory

**2.2 Add FFI bindings for `sandbox_check`**

Add `extern "C"` declarations for `sandbox_check()` and the
`SANDBOX_FILTER_PATH` constant to `sandbox_macos.rs`. These are declared in
`<sandbox.h>`.

**2.3 Embed fingerprint in SBPL profiles**

Modify `generate_sbpl_profile()` in `sandbox_macos.rs` to accept a
`SessionFingerprint` and emit the allow/deny rules for the marker paths.

**2.4 Add fingerprint-only SBPL profile**

Add a new function (e.g., `generate_fingerprint_only_profile()`) that produces
a minimal profile:

```
(version 1)
(allow default)
(deny file-read* (subpath "/tmp/.zed-sandbox-<uuid>/deny"))
(allow file-read* (subpath "/tmp/.zed-sandbox-<uuid>/allow"))
```

This is used when no sandbox restrictions are configured but process tracking
is still needed.

**2.5 Support both profile modes in `sandbox_exec_main()`**

Modify `sandbox_exec_main()` so that it can apply either a full restrictive
profile or a fingerprint-only profile, depending on what config it receives.
The actual plumbing to always invoke the wrapper (even without sandbox
restrictions) happens in Phase 5, after Linux cgroup support is also in place.

## Phase 3: Convergent cleanup (macOS)

Replace the current `Drop` cleanup (100ms timer + `kill_child_process`) with
the convergent scan-and-kill loop.

**3.1 Add process enumeration**

Add a function that enumerates all PIDs owned by the current UID using
`sysctl` with `KERN_PROC_UID`. This returns a `Vec<pid_t>`.

**3.2 Implement the cleanup loop**

Add a `SessionFingerprint::kill_all_processes()` method that implements:

1. `killpg(pgid, SIGKILL)` (best-effort, the group may already be gone) —
   kills the majority of descendants instantly
2. Loop: enumerate all PIDs by UID (via `sysctl` `KERN_PROC_UID`) → skip
   zombies (`kp_proc.p_stat == SZOMB`) → filter by fingerprint match →
   `SIGKILL` every match → repeat until no matches found
3. Delete the fingerprint directory

This runs on a background thread (not async — it's a tight loop that should
complete quickly).

Note: zombie processes must be skipped because they can't be killed by any
signal (they're already dead, awaiting reaping by their parent). If
`sandbox_check` still reports the sandbox profile for zombies, failing to skip
them would cause the loop to spin. The zombie state is detectable from the
same `sysctl` data used for enumeration.

**3.3 Integrate into `Terminal::Drop`**

Replace the current `Drop` implementation. Instead of the 100ms timer +
`kill_child_process()`, spawn a background task that runs
`fingerprint.kill_all_processes()`. The fingerprint is stored alongside the
`PtyProcessInfo` in `TerminalType::Pty`.

Also update `kill_active_task()` to use the same mechanism.

Note: the cleanup task must complete even if Zed is exiting. The current `Drop`
impl uses `detach()`, which risks the task being cancelled if the executor
shuts down. Consider blocking briefly in `Drop` or using a mechanism that
guarantees completion (e.g., a dedicated cleanup thread that outlives the
executor).

**3.4 Wire fingerprint through terminal creation**

- `TerminalBuilder::new()` creates the `SessionFingerprint` and passes it to
  the sandbox wrapper.
- The fingerprint is stored in `TerminalType::Pty` alongside `info` and
  `pty_tx`.
- On drop, the fingerprint is moved into the cleanup task.

## Phase 4: cgroups v2 (Linux)

Implement cgroup-based process tracking for Linux, providing the same
always-on process-lifetime guarantee.

**4.1 Add cgroup session management**

Add a `CgroupSession` type (Linux-only) that:

- `CgroupSession::new()` — creates a new cgroup under the user's systemd
  slice (e.g.,
  `/sys/fs/cgroup/user.slice/user-<uid>.slice/user@<uid>.service/zed-terminal-<uuid>.scope`)
  by writing to the cgroup filesystem
- `CgroupSession::add_process(pid)` — writes the PID to `cgroup.procs`
- `CgroupSession::kill_all()` — writes `1` to `cgroup.freeze`, then writes
  `SIGKILL` to `cgroup.kill` (kernel 5.14+), or falls back to reading
  `cgroup.procs` and killing each PID
- `CgroupSession::cleanup()` — removes the cgroup directory

**4.2 Integrate into sandbox exec**

Modify the `--sandbox-exec` entry point on Linux to accept a cgroup path.
Before exec-ing the real shell, the wrapper moves itself into the specified
cgroup (by writing its own PID to `cgroup.procs`). All descendants
automatically inherit cgroup membership.

**4.3 Integrate into terminal lifecycle**

Same pattern as macOS: `TerminalBuilder::new()` creates the `CgroupSession`,
passes the cgroup path to the sandbox wrapper, stores the session in
`TerminalType::Pty`, and uses it for cleanup in `Drop`.

**4.4 Fallback for old kernels**

If cgroup creation fails (old kernel, cgroups v2 not mounted, no permission),
fall back to the current `killpg` + `kill_child_process` behavior. Log a
warning so the user knows process tracking is degraded.

## Phase 5: Always-on wrapper

With both macOS fingerprinting (Phase 2) and Linux cgroups (Phase 4) in place,
wire them up so the `--sandbox-exec` wrapper runs for every terminal session,
not only when sandbox restrictions are configured.

**5.1 Decouple wrapper invocation from `SandboxConfig`**

Currently `TerminalBuilder::new()` only wraps the shell in `--sandbox-exec`
when `sandbox_config.is_some()`. Change this so the wrapper is always used on
Unix platforms. The wrapper receives either:
- A full `SandboxExecConfig` (restrictions + fingerprint/cgroup), or
- A tracking-only config (fingerprint on macOS, cgroup path on Linux, no
  filesystem restrictions)

Update `SandboxExecConfig` to have an optional restrictions payload and a
required tracking payload.

**5.2 Update both resolution sites**

Modify `crates/project/src/terminals.rs` and `crates/acp_thread/src/terminal.rs`
to always produce a tracking config. The sandbox restrictions remain gated
behind the feature flag and `enabled` setting, but the tracking config is
unconditional.

**5.3 Update `--sandbox-exec` entry point**

Modify `sandbox_exec_main()` to handle the tracking-only case:
- On macOS: apply the fingerprint-only Seatbelt profile (no restrictions)
- On Linux: move into the cgroup (no Landlock restrictions)
- Then exec the real shell as before

## Phase 6: Tests

**6.1 Fingerprint tests (macOS)**

- Test that `SessionFingerprint::matches_pid()` returns true for a process
  launched with the session's Seatbelt profile.
- Test that it returns false for an unsandboxed process.
- Test that it returns false for a process with a different session's profile.
- Test the two-point fingerprint: a process with blanket `/tmp` access does
  not match.

**6.2 Convergent cleanup tests (macOS)**

- Test that a simple child process is killed.
- Test that a process that calls `setsid()` is still found and killed.
- Test that a double-forking daemon (fork → setsid → fork → parent exits) is
  still found and killed.
- Test that the loop terminates.

**6.3 Cgroup tests (Linux)**

- Test that `CgroupSession::kill_all()` kills a child process.
- Test that a process that calls `setsid()` is still killed (it's in the
  cgroup).
- Test the fallback path when cgroups are unavailable.

**6.4 Fingerprint-only mode tests (macOS)**

- Test that a terminal spawned without sandbox restrictions still gets the
  fingerprint profile applied.
- Test that cleanup works correctly in fingerprint-only mode.
- Test that the process is not restricted (can access arbitrary paths, use
  network, etc.).

## Phase 7: Cleanup of existing code review items

With the new architecture in place, address the remaining items from the code
review that haven't been handled by earlier phases:

- **Item #1**: Change `(allow signal)` to `(allow signal (target children))`.
- **Item #4**: Change `current_exe()` fallback to propagate the error with `?`.
- **Item #6**: Replace `let _ = write!(...)` with `push_str` + `format!` or
  `.unwrap()`.
- **Items #7, #8**: Add tests for `additional_executable_paths` and
  `canonicalize_paths()` with symlinks.
