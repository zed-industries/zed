# Remaining Code Review Items — `local-sandboxing`

Items from the original review that have been addressed are not listed here.
Only items that are still present in the current code are included.

---

## 1. Dotfile lists are incomplete

**Files:** `crates/sandbox/src/sandbox_macos.rs` and `sandbox_linux.rs`

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

## 2. `/proc/self` only gets read access on Linux

**File:** `crates/sandbox/src/sandbox_linux.rs`, line ~132

Bash process substitution (e.g., `<(command)`) creates FIFOs under
`/proc/self/fd/`. These FIFOs need write access — the shell writes to them.
The current `fs_read()` permission may cause process substitution to fail.

**Fix:** Grant `fs_all()` (or at least read+write) on `/proc/self` instead of
`fs_read()`.

---

## 3. macOS: `$TMPDIR` grants broad access via `/var/folders`

**File:** `crates/sandbox/src/sandbox.rs` (default read-write paths in
`ResolvedSystemPaths::default_read_write`)

The default macOS read-write paths include `/var/folders`, which is
the parent of every user's per-session temp directory. This means the sandbox
grants read-write access to all temp files on the system, not just the
current user's.

A tighter approach would resolve `$TMPDIR` at spawn time (which gives the
per-user, per-session temp directory like
`/private/var/folders/xx/xxxx/T/`) and only allow that specific
subdirectory. This would still let the shell use temp files but prevent
access to other users' temp directories.
