# Bubblewrap sandbox — remaining work

Status of the Landlock → bwrap+seccomp migration and what's left. Concise by
intent; see `sandbox-bubblewrap-migration.md` for the original rationale.

## Done

- Landlock removed (code, dep, `Cargo.lock`).
- `linux_bubblewrap`: locate + setuid rejection, `is_available()` probe,
  `build_bwrap_args` (incl. `--tmpfs /tmp`), seccomp network-deny filter,
  launcher (`run_launcher_if_invoked`, encode/decode), unit tests.
- `apply_sandbox_wrap` wired to bwrap; `main.rs` hook; fail-closed launcher.
- Linux: no special `$TMPDIR` (relies on tmpfs `/tmp`); prompt updated.

## 1. Thick launcher + status reporting (next)

Move all sandbox logic into the launcher (the process that runs the command)
so check and run are one process — no parent/launcher TOCTOU. Report the
outcome to the parent over a one-shot `SOCK_DGRAM` unix socket addressed by a
**path** passed in argv/env (we can't pass fds through the PTY).

Establish the channel *before* any sandbox layer, so reporting is
policy-independent, not a carve-out:

```
0. socket(AF_UNIX, SOCK_DGRAM | SOCK_CLOEXEC) + connect   # pre-everything
1. setuid check     → SetuidRejected,  exit
2. locate bwrap     → NotFound,        exit
3. install seccomp  → SeccompFailed,   exit
4. probe bwrap      → NamespaceFailed, exit   # `bwrap … -- true` (approach b)
5. send Success → exec real bwrap
```

- `enum LauncherStatus { Success, SetuidRejected, NotFound, NamespaceFailed, SeccompFailed }`.
- `Success` sent only after seccomp **and** probe succeed → it provably means
  fully sandboxed. Residual probe→exec micro-TOCTOU is fail-closed (hard
  command failure, never silent unsandboxed run).
- `SOCK_CLOEXEC`: fd never inherited by probe child, bwrap, or the command.
- Command output/exit flow through the PTY as normal — no extra channel.

## 2. `apply_sandbox_wrap` rework

- Pass **raw policy** (writable dirs, permissions, cwd, program, args, network
  policy, socket path) to the launcher; the launcher assembles the bwrap line.
- Parent binds the socket before spawn, awaits one `LauncherStatus`, uses it to
  pick the UI tier.
- Delete parent-side `is_available()`/`locate_bwrap()` and their `OnceLock`
  caches (logic now lives in the launcher; no caching needed — the check is
  cheap and a stale value would only be wrong).

## 3. UI tiers

- `Success` → sandboxed, no warning.
- Any failure → surface the reason and offer **Run unsandboxed / Deny**
  (unidirectional: on "run unsandboxed" the parent re-spawns a normal terminal;
  the launcher never blocks for a decision).
- Orange "unsandboxed" indicator on terminals that ran without the sandbox.

## 4. Bundled bwrap

`bundled_bwrap_path()` is a `None` stub, so today sandboxing needs a system
`bwrap`. Build a static musl, non-setuid `bwrap` (Nix `pkgsStatic`,
`-Dselinux=disabled`) per arch, bundle it, ship LGPL source/notice. Open: bundle
vs download.

## 5. NixOS tests (currently broken)

`nix/tests/sandboxing/{default.nix,helper.nix}` still reference the deleted
`landlock_test_helper` bin and `sandbox/nixos-test` feature, so the suite won't
build. Rewrite:

- **Drop the kernel matrix.** It existed because Landlock capability scaled with
  kernel ABI (V1–V7). bwrap capability is flat on any kernel with unprivileged
  user namespaces, so per-kernel scenarios add little.
- **Keep two VM scenarios instead:** (a) unprivileged userns *enabled* — assert
  enforcement (write outside writable dir denied, inside allowed, outbound TCP
  blocked, `AF_UNIX` works, setuid bwrap rejected, `LauncherStatus::Success`);
  (b) userns *disabled* (e.g. `sysctl user.max_user_namespaces=0`) — assert
  graceful degradation reports `NamespaceFailed` rather than running unsandboxed.
- **New bwrap test helper** replacing `landlock_test_helper.rs`: drives the
  launcher exactly as Zed does and reads the status datagram. Gate enforcement
  assertions on a runtime `bwrap_enforces()` probe that **skips** (not fails)
  when the VM can't enforce — the CI trap we already hit.
- Re-add the corresponding cargo feature/`[[bin]]` for the new helper.
