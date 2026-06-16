# Bubblewrap sandbox — remaining work

Status of the Landlock → bwrap migration and what's left. Concise by
intent; see `sandbox-bubblewrap-migration.md` for the original rationale.

## Done

- Landlock removed (code, dep, `Cargo.lock`).
- `linux_bubblewrap`: locate + setuid rejection, `build_bwrap_args` (incl.
  `--tmpfs /tmp`), launcher (`run_launcher_if_invoked`, encode/decode), unit
  tests.
- `apply_sandbox_wrap` wired to bwrap; `main.rs` hook.
- Linux: no special `$TMPDIR` (relies on tmpfs `/tmp`); prompt updated.
- **Network deny via `bwrap --unshare-net`.** The current policy is a coarse
  on/off, which `bwrap`'s own network namespace expresses directly — no
  seccomp, `NO_NEW_PRIVS`, or `seccompiler` dependency. Tradeoff: a
  network-denied command can't reach *abstract* `AF_UNIX` endpoints (D-Bus
  etc.); pathname unix sockets still work. Seccomp returns only when we need a
  finer policy (allow-list/egress proxy).
- **Thick launcher + status reporting (was §1).** All sandbox work
  (locate/probe) lives in the launcher — check and run are one process, no
  parent/launcher TOCTOU. The launcher reports a `LauncherStatus` as a one-shot
  datagram (`SOCK_DGRAM`) over an abstract `AF_UNIX` socket (`StatusChannel`)
  whose address is passed in argv. Statuses:
  `Success | BwrapNotFound | SetuidRejected | SandboxProbeFailed`. The probe
  runs `bwrap <real args> -- true`, so it validates the *exact* policy we're
  about to use.
  - The launcher never runs a command unsandboxed: on any non-`Success`
    outcome it reports the reason and **aborts**. Choosing what to do about a
    failure is the *consumer's* job:
    - The agent (`run_terminal_tool`) **fails open** for now — it checks
      `SandboxWrap::can_create_sandbox` up front and, if the sandbox can't be
      created, runs the command without one and prepends a note for the model
      ("failed to create the sandbox … ran without a sandbox"). No UI yet; see
      "UI tiers" below.
    - The NixOS tests **fail closed** — they assert the reported status and that
      the command did *not* run.
- **`apply_sandbox_wrap` rework (was §2).** Parent passes raw policy (socket,
  permissions, cwd, writable dirs, program, args) to the launcher and listens
  on the channel in a background thread (diagnostic only now). Parent-side
  `locate_bwrap`/`is_available` + their `OnceLock` caches deleted; the up-front
  viability check is `linux_bubblewrap::check_can_create_sandbox`.
- **NixOS tests (was §5).** Landlock kernel matrix replaced by three scenarios
  (`sandbox-userns-enabled`, `sandbox-userns-disabled`, `sandbox-no-bwrap`) under
  `nix/tests/sandboxing`. New `bwrap_test_helper` bin (sandbox crate,
  `nixos-test` feature) drives the real launcher and reads the status; an
  enforcement probe **skips** (not fails) when the VM can't enforce. Checks
  wired into the flake (`packages.nix`); xtask prefix is now `sandbox-`.

## Remaining

### 1. UI tiers

- `Success` → sandboxed, no warning.
- Any failure → surface the reason and offer **Run unsandboxed / Deny**
  (unidirectional: on "run unsandboxed" the parent re-spawns a normal terminal;
  the launcher never blocks for a decision).
- Orange "unsandboxed" indicator on terminals that ran without the sandbox.

### 2. Bundled bwrap

`bundled_bwrap_path()` is a `None` stub, so today sandboxing needs a system
`bwrap`. Build a static musl, non-setuid `bwrap` (Nix `pkgsStatic`,
`-Dselinux=disabled`) per arch, bundle it, ship LGPL source/notice. Open: bundle
vs download.
