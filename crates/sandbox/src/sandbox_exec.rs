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

use crate::{ResolvedSystemPaths, SandboxConfig};
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
    /// Optional fingerprint UUID for session tracking (macOS).
    #[serde(default)]
    pub fingerprint_uuid: Option<String>,
    /// Whether this is a tracking-only config (no filesystem restrictions).
    #[serde(default)]
    pub tracking_only: bool,
    /// Optional cgroup path for Linux process tracking.
    #[serde(default)]
    pub cgroup_path: Option<String>,
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
            fingerprint_uuid: None,
            tracking_only: false,
            cgroup_path: None,
        }
    }

    /// Convert back to a `SandboxConfig` for the sandbox implementation functions.
    pub fn to_sandbox_config(&self) -> SandboxConfig {
        use std::path::PathBuf;

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
        serde_json::to_string(self).expect("SandboxExecConfig is non-cyclic")
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

    // Step 1: Collect allowed environment variables.
    let zed_vars = [
        "ZED_TERM",
        "TERM_PROGRAM",
        "TERM",
        "COLORTERM",
        "TERM_PROGRAM_VERSION",
    ];
    let allowed: std::collections::HashSet<&str> =
        config.allowed_env_vars.iter().map(|s| s.as_str()).collect();

    let filtered_env: Vec<(String, String)> = std::env::vars()
        .filter(|(key, _)| allowed.contains(key.as_str()) || zed_vars.contains(&key.as_str()))
        .collect();

    // Step 2: Apply the OS-level sandbox.
    #[cfg(target_os = "macos")]
    {
        if config.tracking_only {
            if let Some(ref uuid_str) = config.fingerprint_uuid {
                let fingerprint =
                    match crate::sandbox_macos::SessionFingerprint::from_uuid_str(uuid_str) {
                        Ok(fp) => fp,
                        Err(e) => {
                            eprintln!("zed --sandbox-exec: invalid fingerprint UUID: {e}");
                            std::process::exit(1);
                        }
                    };
                if let Err(e) = crate::sandbox_macos::apply_fingerprint_only(&fingerprint) {
                    eprintln!("zed --sandbox-exec: failed to apply fingerprint profile: {e}");
                    std::process::exit(1);
                }
            }
        } else {
            let result = match config.fingerprint_uuid.as_ref() {
                Some(uuid_str) => {
                    match crate::sandbox_macos::SessionFingerprint::from_uuid_str(uuid_str) {
                        Ok(fingerprint) => crate::sandbox_macos::apply_sandbox_with_fingerprint(
                            &sandbox_config,
                            &fingerprint,
                        ),
                        Err(e) => {
                            eprintln!("zed --sandbox-exec: invalid fingerprint UUID: {e}");
                            std::process::exit(1);
                        }
                    }
                }
                None => crate::sandbox_macos::apply_sandbox(&sandbox_config),
            };
            if let Err(e) = result {
                eprintln!("zed --sandbox-exec: failed to apply macOS sandbox: {e}");
                std::process::exit(1);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Move into the session cgroup for process tracking
        if let Some(ref cgroup_path) = config.cgroup_path {
            let session = crate::cgroup::CgroupSession::from_path(cgroup_path);
            let pid = unsafe { libc::getpid() };
            if let Err(e) = session.add_process(pid) {
                eprintln!("zed --sandbox-exec: failed to join cgroup: {e}");
                std::process::exit(1);
            }
        }

        // Apply Landlock restrictions (only if not tracking-only)
        if !config.tracking_only {
            if let Err(e) = crate::sandbox_linux::apply_sandbox(&sandbox_config) {
                eprintln!("zed --sandbox-exec: failed to apply Linux sandbox: {e}");
                std::process::exit(1);
            }
        }
    }

    // Step 3: Exec the real shell.
    let program = &shell_args[0];
    let args = &shell_args[1..];
    let err = Command::new(program)
        .args(args)
        .env_clear()
        .envs(filtered_env)
        .exec();

    eprintln!("zed --sandbox-exec: failed to exec {program}: {err}");
    std::process::exit(1);
}
