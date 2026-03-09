//! Sandbox executor: the `--sandbox-exec` entry point.
//!
//! When Zed is invoked with `--sandbox-exec <json-config> -- <shell> [args...]`,
//! this module takes over. It:
//! 1. Parses the sandbox config from the JSON argument
//! 2. Filters environment variables to the allowed set
//! 3. Applies the OS-level sandbox (Seatbelt on macOS, Landlock on Linux)
//! 4. Execs the real shell (never returns)
//!
//! This approach avoids modifying the alacritty fork — alacritty spawns the
//! Zed binary as the "shell", and the Zed binary applies the sandbox before
//! exec-ing the real shell. Since both Seatbelt and Landlock sandboxes are
//! inherited by child processes, the real shell and everything it spawns
//! are sandboxed.
//!
//! Note: passing JSON directly via a CLI argument is safe because
//! `std::process::Command::arg()` passes arguments to `execve` without
//! shell interpretation, so no quoting issues arise.

use crate::terminal_settings::SandboxConfig;
use std::os::unix::process::CommandExt;
use std::process::Command;

/// Serializable sandbox config for passing between processes via a JSON CLI arg.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SandboxExecConfig {
    pub project_dir: String,
    pub executable_paths: Vec<String>,
    pub read_only_paths: Vec<String>,
    pub read_write_paths: Vec<String>,
    pub additional_executable_paths: Vec<String>,
    pub additional_read_only_paths: Vec<String>,
    pub additional_read_write_paths: Vec<String>,
    pub allow_network: bool,
    pub allowed_env_vars: Vec<String>,
}

impl SandboxExecConfig {
    /// Convert from the resolved `SandboxConfig` to the serializable form.
    pub fn from_sandbox_config(config: &SandboxConfig) -> Self {
        Self {
            project_dir: config.project_dir.to_string_lossy().into_owned(),
            executable_paths: config
                .system_paths
                .executable
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            read_only_paths: config
                .system_paths
                .read_only
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            read_write_paths: config
                .system_paths
                .read_write
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            additional_executable_paths: config
                .additional_executable_paths
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            additional_read_only_paths: config
                .additional_read_only_paths
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            additional_read_write_paths: config
                .additional_read_write_paths
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            allow_network: config.allow_network,
            allowed_env_vars: config.allowed_env_vars.clone(),
        }
    }

    /// Convert back to a `SandboxConfig` for the sandbox implementation functions.
    pub fn to_sandbox_config(&self) -> SandboxConfig {
        use std::path::PathBuf;

        use crate::terminal_settings::ResolvedSystemPaths;
        SandboxConfig {
            project_dir: PathBuf::from(&self.project_dir),
            system_paths: ResolvedSystemPaths {
                executable: self.executable_paths.iter().map(PathBuf::from).collect(),
                read_only: self.read_only_paths.iter().map(PathBuf::from).collect(),
                read_write: self.read_write_paths.iter().map(PathBuf::from).collect(),
            },
            additional_executable_paths: self
                .additional_executable_paths
                .iter()
                .map(PathBuf::from)
                .collect(),
            additional_read_only_paths: self
                .additional_read_only_paths
                .iter()
                .map(PathBuf::from)
                .collect(),
            additional_read_write_paths: self
                .additional_read_write_paths
                .iter()
                .map(PathBuf::from)
                .collect(),
            allow_network: self.allow_network,
            allowed_env_vars: self.allowed_env_vars.clone(),
        }
    }

    /// Serialize the config to a JSON string for passing via CLI arg.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize sandbox config")
    }

    /// Deserialize a config from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("invalid sandbox config JSON: {e}"))
    }
}

/// Main entry point for `zed --sandbox-exec <json-config> [-- shell args...]`.
///
/// This function never returns — it applies the sandbox and execs the real shell.
/// The `shell_args` are the remaining positional arguments after `--`.
pub fn sandbox_exec_main(config_json: &str, shell_args: &[String]) -> ! {
    let config = match SandboxExecConfig::from_json(config_json) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("zed --sandbox-exec: failed to parse config: {e}");
            std::process::exit(1);
        }
    };

    if shell_args.is_empty() {
        eprintln!("zed --sandbox-exec: no shell command specified");
        std::process::exit(1);
    }

    let mut sandbox_config = config.to_sandbox_config();
    sandbox_config.canonicalize_paths();

    // Step 1: Filter environment variables.
    // Keep only allowed vars + a few Zed-specific ones.
    let zed_vars = [
        "ZED_TERM",
        "TERM_PROGRAM",
        "TERM",
        "COLORTERM",
        "TERM_PROGRAM_VERSION",
    ];
    let allowed: std::collections::HashSet<&str> =
        config.allowed_env_vars.iter().map(|s| s.as_str()).collect();

    // Collect vars to remove (can't modify env while iterating)
    let vars_to_remove: Vec<String> = std::env::vars()
        .filter_map(|(key, _)| {
            if allowed.contains(key.as_str()) || zed_vars.contains(&key.as_str()) {
                None
            } else {
                Some(key)
            }
        })
        .collect();

    for key in &vars_to_remove {
        // SAFETY: We are in a single-threaded sandbox wrapper process
        // (the Zed binary invoked with --sandbox-exec), so there are no
        // other threads that could be reading env vars concurrently.
        unsafe {
            std::env::remove_var(key);
        }
    }

    // Step 2: Apply the OS-level sandbox.
    #[cfg(target_os = "macos")]
    {
        if let Err(e) = crate::sandbox_macos::apply_sandbox(&sandbox_config) {
            eprintln!("zed --sandbox-exec: failed to apply macOS sandbox: {e}");
            std::process::exit(1);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = crate::sandbox_linux::apply_sandbox(&sandbox_config) {
            eprintln!("zed --sandbox-exec: failed to apply Linux sandbox: {e}");
            std::process::exit(1);
        }
    }

    // Step 3: Exec the real shell. This replaces the current process.
    let program = &shell_args[0];
    let args = &shell_args[1..];
    let err = Command::new(program).args(args).exec();

    // exec() only returns on error
    eprintln!("zed --sandbox-exec: failed to exec {program}: {err}");
    std::process::exit(1);
}
