# Remaining Code Review Items — `local-sandboxing`

Items from the original review that have been addressed are not listed here.
Only items that are still present in the current code are included.

---

## 1. `(allow signal)` is unrestricted on macOS

**File:** `crates/terminal/src/sandbox_macos.rs`, line ~54

The SBPL profile contains a bare `(allow signal)` which permits the sandboxed
process to send signals (including SIGKILL) to any process owned by the same
user. This should be scoped:

```
(allow signal (target self))
```

or at minimum `(target children)` if child-process signaling is needed.

---

## 2. Dotfile lists are incomplete

**Files:** `crates/terminal/src/sandbox_macos.rs` and `sandbox_linux.rs`

The hardcoded dotfile lists cover zsh, bash, and a few generic files but miss:

- **Shell history files** (`.bash_history`, `.zsh_history`) — if the shell
  can't write history, users will get silent failures or error messages on
  every command. Read-write access to these is likely needed.
- **Fish shell** — fish config lives in `~/.config/fish/`, which is partially
  covered by the `~/.config` subpath rule but only if `~/.config` exists.
- **Nushell, PowerShell, elvish** — no coverage at all.

The lists are also in different orders between the two files, adding
maintenance overhead for no benefit.

**Fix:** Extract the dotfile list to a shared constant (e.g., on
`SandboxConfig`) so both platform implementations use the same list. Consider
adding history files with read-write access rather than read-only.

---

## 3. `/proc/self` only gets read access on Linux

**File:** `crates/terminal/src/sandbox_linux.rs`, line ~143

Bash process substitution (e.g., `<(command)`) creates FIFOs under
`/proc/self/fd/`. These FIFOs need write access — the shell writes to them.
The current `fs_read()` permission may cause process substitution to fail.

**Fix:** Grant `fs_all()` (or at least read+write) on `/proc/self` instead of
`fs_read()`.

---

## 4. `current_exe()` failure silently falls back to `"zed"`

**File:** `crates/terminal/src/terminal.rs`, line ~553

```rust
let zed_binary = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("zed"));
```

If `current_exe()` fails, this falls back to `"zed"` which relies on PATH
lookup. Inside a sandbox where PATH is restricted, this could fail in
confusing ways — the user would see a "command not found" error with no
indication that the sandbox wrapper binary couldn't be located.

**Fix:** Propagate the error with `?` instead of silently falling back. The
terminal builder already returns `Result`.

---

## 5. Duplicated sandbox config resolution logic

**Files:** `crates/project/src/terminals.rs` (~lines 413-435) and
`crates/acp_thread/src/terminal.rs` (~lines 245-275)

The sandbox config resolution (check feature flag → check enabled → check
`apply_to` → fallback project dir → build config) is duplicated verbatim
across the user-terminal and agent-terminal code paths. The only meaningful
differences are:

- Which `SandboxApplyTo` variant to match (`Terminal` vs `Tool`)
- Where the project directory comes from

**Fix:** Extract into a shared helper, e.g.:

```rust
impl SandboxConfig {
    pub fn resolve_if_enabled(
        sandbox_settings: &SandboxSettingsContent,
        target: SandboxApplyTo,
        project_dir: PathBuf,
        cx: &App,
    ) -> Option<Self> { ... }
}
```

---

## 6. `let _ = write!(...)` suppresses errors

**File:** `crates/terminal/src/sandbox_macos.rs`, lines ~191 and ~223

The project `.rules` say: "Never silently discard errors with `let _ =` on
fallible operations." While `write!` to a `String` is infallible in practice
(the `fmt::Write` impl for `String` cannot fail), the pattern still violates
the rule.

**Fix:** Use `write!(...).unwrap()` (justified since `String` fmt::Write is
infallible) or restructure to use `push_str` + `format!`.

---

## 7. No test for `additional_executable_paths`

**File:** `crates/terminal/src/sandbox_tests.rs`

There are integration tests for `additional_read_write_paths` and
`additional_read_only_paths`, but not for `additional_executable_paths`. A
test should verify that a binary placed in an additional executable path can
actually be executed by the sandboxed shell, and that binaries outside all
allowed paths cannot.

---

## 8. No test for `canonicalize_paths()` with symlinks

**File:** `crates/terminal/src/sandbox_tests.rs`

The `canonicalize_paths` function is exercised indirectly (the test helper
calls it), but no test explicitly verifies that a symlinked project directory
or additional path is resolved before being added to sandbox rules. A test
could create a symlink to a temp directory, use it as the project dir, and
verify the sandbox enforces on the real path.

---

## 9. macOS: `$TMPDIR` grants broad access via `/var/folders`

**File:** `crates/terminal/src/terminal_settings.rs` (default read-write
paths)

The default macOS read-write paths include `/private/var/folders`, which is
the parent of every user's per-session temp directory. This means the sandbox
grants read-write access to all temp files on the system, not just the
current user's.

A tighter approach would resolve `$TMPDIR` at spawn time (which gives the
per-user, per-session temp directory like
`/private/var/folders/xx/xxxx/T/`) and only allow that specific
subdirectory. This would still let the shell use temp files but prevent
access to other users' temp directories.
