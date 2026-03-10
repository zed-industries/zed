# Sandbox

OS-level sandboxing for terminal processes spawned by Zed — both interactive
user terminals and agent tool invocations. The sandbox restricts filesystem
access, network access, and other capabilities so that commands run in the
terminal can only affect what they're explicitly permitted to.

## Platform mechanisms

- **macOS**: Seatbelt (SBPL profiles applied via `sandbox_init()`)
- **Linux**: Landlock LSM for filesystem restrictions, cgroups v2 for process
  lifetime management

Both mechanisms are inherited by child processes and cannot be removed. A
sandboxed shell and everything it spawns remain sandboxed for their entire
lifetime.

## Always-on process tracking

Reliable process cleanup is valuable even when the user has not configured any
sandbox restrictions. The standard approach of `killpg()` (kill by process
group) is unreliable — a process can escape via `setsid()` or `setpgid()`, and
the terminal's `Drop` impl will miss it.

For this reason, **process tracking is always enabled for every terminal
session**, regardless of whether sandbox restrictions are configured:

- **macOS**: A minimal Seatbelt profile is applied containing only the session
  fingerprint (see below) and `(allow default)` for everything else. This
  doesn't restrict the process at all, but gives us the `sandbox_check()`
  fingerprint needed to reliably find and kill all descendants. When full
  sandbox restrictions are also enabled, the fingerprint is embedded in the
  restrictive profile instead.

- **Linux**: A cgroup is created for every terminal session. On cleanup, the
  cgroup is frozen and all members are killed. This works regardless of whether
  Landlock filesystem restrictions are also enabled.

This replaces the current cleanup approach (100ms delay + `kill_child_process`)
with a convergent, reliable mechanism on both platforms.

## Process cleanup on terminal close

When a terminal session ends, all processes it spawned must be killed. This is
straightforward on Linux (cgroups v2 provides an atomic, inescapable kill), but
requires careful handling on macOS where no equivalent kernel primitive exists.

### The problem

A process inside the sandbox can call `setsid()` or `setpgid()` to leave the
shell's process group. After that, `killpg()` (which kills by process group)
won't reach it. If the process also double-forks and the intermediate parent
exits, the grandchild is reparented to PID 1 (launchd), severing the parent
chain entirely. This means:

- **Process group killing** misses it (different group).
- **Parent chain walking** can't find it (parent is PID 1).
- The process persists after the terminal closes, retaining whatever sandbox
  permissions it was granted at spawn time.

macOS Seatbelt has no operation for `setsid()` — it isn't a filterable
operation in SBPL, so the sandbox can't prevent this. (On Linux, seccomp could
block `setsid()`, but it would break legitimate programs like `ssh`.)

### Why stale permissions matter

The sandbox profile is a snapshot frozen at spawn time. If a process escapes
cleanup, it retains the original permissions indefinitely. This is a problem
because:

- The user might later add secrets to a directory that was in the sandbox's
  allowed paths.
- The user might change sandbox settings for future sessions, but the escaped
  process still has the old, more-permissive profile.
- For agent tool use especially, the sandbox permissions are granted for a
  specific task. An escaped process retaining those permissions after the task
  is complete violates the principle of least privilege.

### Linux: cgroups v2

On Linux, the solution is to place the shell in a dedicated cgroup. All
descendants are automatically tracked in the cgroup regardless of `setsid()`,
`setpgid()`, or reparenting. No process can leave a cgroup without
`CAP_SYS_ADMIN`. On terminal close:

1. Freeze the cgroup (prevents new forks).
2. Kill all processes in the cgroup.
3. Delete the cgroup.

This is a hard guarantee — the same mechanism containers use.

cgroups v2 is the default on all modern Linux distributions (Ubuntu 21.10+,
Fedora 31+, Debian 11+, Arch 2020+, RHEL 9+). No installation or
configuration is needed. Regular (non-root) users can create child cgroups
within their own systemd user slice, so no elevated privileges are required.

### macOS: sandbox fingerprinting with convergent cleanup

macOS has no public equivalent to cgroups. The approach is a convergent
scan-and-kill loop that uses the Seatbelt sandbox profile itself as an
unforgeable fingerprint.

#### Sandbox fingerprint

Each terminal session embeds a unique fingerprint in its SBPL profile: a
per-session UUID path where one child path is allowed and a sibling is denied.

```
(allow file-read* (subpath "/tmp/.zed-sandbox-<uuid>/allow"))
;; /tmp/.zed-sandbox-<uuid>/deny is implicitly denied by (deny default)
```

When the session has no sandbox restrictions (fingerprint-only mode), the
profile uses `(allow default)` instead of `(deny default)`, but still includes
an explicit deny for the fingerprint's deny-side path:

```
(version 1)
(allow default)
(deny file-read* (subpath "/tmp/.zed-sandbox-<uuid>/deny"))
(allow file-read* (subpath "/tmp/.zed-sandbox-<uuid>/allow"))
```

This two-point fingerprint cannot be produced by any other sandbox profile:

- A sandbox that blanket-allows `/tmp` would allow **both** paths — fails the
  deny check.
- A sandbox that blanket-denies `/tmp` would deny **both** paths — fails the
  allow check.
- An unsandboxed process allows everything — fails the deny check.
- Only a process with our exact profile allows one and denies the other.

The fingerprint is checked from outside the process using `sandbox_check()`:

```c
int allows = sandbox_check(pid, "file-read-data",
    SANDBOX_FILTER_PATH, "/tmp/.zed-sandbox-<uuid>/allow") == 0;
int denies = sandbox_check(pid, "file-read-data",
    SANDBOX_FILTER_PATH, "/tmp/.zed-sandbox-<uuid>/deny") != 0;
// Match requires: allows && denies
```

The fingerprint is unforgeable because the Seatbelt sandbox is a kernel-level
invariant — no process can modify or remove its own sandbox profile.

#### Convergent cleanup loop

On terminal close:

1. `killpg(pgid, SIGKILL)` — kill the process group. This instantly handles
   the vast majority of descendants (everything that didn't escape the group).
2. Enumerate all processes owned by the current UID (via `sysctl`
   `KERN_PROC_UID`).
3. For each process, probe with `sandbox_check` using the session fingerprint.
4. `SIGKILL` every match.
5. Go to step 2.
6. When a full scan finds zero matches, every process from this session is
   dead.
7. Delete the fingerprint directory.

**Why this terminates:** Each iteration either discovers processes (and kills
them) or discovers none (loop exits). The total number of processes is finite,
and the set of living fingerprinted processes shrinks monotonically.

**Why this is correct:** The Seatbelt sandbox is inherited by all descendants
and cannot be removed. Every descendant of the sandboxed shell — regardless of
`setsid()`, `setpgid()`, double-forking, or reparenting to PID 1 — carries the
session fingerprint. `sandbox_check` finds them by probing the kernel, not by
walking the process tree.

**Why SIGKILL on sight instead of SIGSTOP:** An earlier design froze escapees
with `SIGSTOP` during scanning, then killed them all at the end. But `SIGSTOP`
only stops the process you send it to, not its children — so children of a
stopped process are still running and can fork. `SIGKILL` is equally effective:
a dead process can't fork, and any children it already created are findable by
fingerprint on the next scan iteration. The simpler approach is just to kill
everything on sight and keep scanning until the scan comes back empty.

**Why not process-group operations after step 1:** After `killpg` handles the
initial process group, any remaining processes are by definition ones that
escaped via `setsid()` or `setpgid()`. They're in different process groups (or
their own sessions), so further `killpg` calls can't target them without
knowing their group IDs. Worse, if a process double-forks and the intermediate
parent exits, the grandchild is reparented to PID 1 (launchd) — there's no
parent chain linking it back to the original shell, and its process group is
unrelated to ours. The only reliable way to find these escapees is the
fingerprint probe, which works regardless of process group, session, or parent
relationship.

**Zombie handling:** After `SIGKILL`, a process becomes a zombie until its
parent reaps it. If `sandbox_check` still reports the sandbox profile for
zombies, the loop could spin on unkillable processes. The scan should skip
processes in the zombie state (detectable via `kinfo_proc.kp_proc.p_stat ==
SZOMB` from the same `sysctl` call used for enumeration). Zombies are harmless
— they can't execute code or fork — so skipping them is correct.

**Residual race:** Between discovering a process (step 3) and killing it (step
4), the process could fork. But the child inherits the fingerprint, so the next
iteration of the loop finds it. The loop continues until no such children
remain. The only way a process could escape is to fork a child that somehow
doesn't inherit the sandbox — which the kernel guarantees cannot happen.

### Alternatives considered and rejected

#### Audit session IDs (BSM)

macOS's BSM audit framework assigns each process an audit session ID
(`ai_asid`) that is inherited by children. In principle, this could track
descendants. Rejected because:

- `getaudit_addr()` requires elevated privileges.
- There is no "kill all processes in this audit session" syscall — you still
  end up enumerating and killing individually.
- macOS doesn't consistently use POSIX sessions (`ps -e -o sess` shows 0 for
  all processes on many systems).

#### Endpoint Security framework

Apple's Endpoint Security framework provides kernel-level notifications for
every fork/exec event, which would allow perfectly reliable tracking. Rejected
because:

- Requires the `com.apple.developer.endpoint-security.client` entitlement,
  which must be approved by Apple.
- Designed for security products (antivirus, MDM), not general-purpose apps.
- Significantly increases the complexity and privilege requirements of Zed.

#### XNU coalitions

macOS has a kernel concept called "coalitions" that groups related processes for
resource tracking and lifecycle management — essentially Apple's internal
equivalent of cgroups. Rejected because:

- The APIs (`coalition_create()`, `coalition_terminate()`) are private SPI.
- They require entitlements not available to third-party apps.

#### Temporary copy / overlay of project directory

Instead of granting sandbox access to the real project directory, use a
temporary copy or FUSE overlay, then delete it on terminal close. Rejected
because:

- Copying large projects is expensive.
- File watching, symlinks, and build tool caching break.
- FUSE on macOS requires macFUSE (third-party kext) or FSKit (macOS 15+).
- Tools that embed absolute paths (compiler errors, debugger info) would show
  wrong paths.

#### Symlink indirection

Grant sandbox access to a symlink path (e.g., `/tmp/.zed-link-<uuid>` →
`/real/project/`), then delete the symlink on cleanup. Rejected because:

- Seatbelt resolves symlinks to canonical paths when checking access (this is
  why `canonicalize_paths()` is called before building the profile).
- Deleting the symlink wouldn't revoke access to the underlying real path.

#### Blocking `setsid()` / `setpgid()`

Prevent processes from leaving the process group in the first place. Rejected
because:

- Seatbelt has no filterable operation for these syscalls.
- On Linux, seccomp could block them, but this breaks legitimate programs
  (`ssh`, some build tools, process managers).

#### Lightweight VM via Virtualization framework

Run agent commands inside a macOS Virtualization framework VM. This would give a
hard process-lifetime guarantee (shutting down the VM kills everything).
Rejected (for now) because:

- Massive architectural change.
- The VM runs Linux, not macOS — macOS-specific tools wouldn't work.
- Resource overhead (memory, CPU, startup time).
- Overkill for the current threat model.

## Signal scoping (macOS)

The SBPL profile uses `(allow signal (target children))` rather than a bare
`(allow signal)`. This prevents the sandboxed process from signaling arbitrary
same-user processes (other Zed instances, browsers, etc.) while still allowing
the shell to:

- Manage jobs (`kill %1`, `bg`, `fg`)
- Use the `kill` command on child processes
- Clean up background jobs on exit (SIGHUP)

Note that Ctrl+C and Ctrl+Z are sent by the kernel's TTY driver, not by the
shell, so they work regardless of signal sandbox rules.

`(target self)` was considered but rejected because it would break all job
control and shell cleanup of background processes.

In fingerprint-only mode (no sandbox restrictions), `(allow default)` already
permits all signals, so no explicit signal rule is needed.
