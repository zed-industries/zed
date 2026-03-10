#[cfg(target_os = "linux")]
mod cgroup;
#[cfg(unix)]
mod sandbox_exec;
#[cfg(target_os = "linux")]
mod sandbox_linux;
#[cfg(target_os = "macos")]
pub(crate) mod sandbox_macos;
#[cfg(all(test, unix))]
mod sandbox_tests;

#[cfg(target_os = "linux")]
pub use cgroup::CgroupSession;
#[cfg(target_os = "macos")]
pub use sandbox_macos::SessionFingerprint;

#[cfg(unix)]
pub use sandbox_exec::{SandboxExecConfig, sandbox_exec_main};

use std::path::PathBuf;

/// Resolved sandbox configuration with all defaults applied.
/// This is the concrete type passed to the terminal spawning code.
#[derive(Clone, Debug)]
pub struct SandboxConfig {
    pub project_dir: PathBuf,
    pub system_paths: ResolvedSystemPaths,
    pub additional_executable_paths: Vec<PathBuf>,
    pub additional_read_only_paths: Vec<PathBuf>,
    pub additional_read_write_paths: Vec<PathBuf>,
    pub allow_network: bool,
    pub allowed_env_vars: Vec<String>,
}

/// Resolved system paths with OS-specific defaults applied.
#[derive(Clone, Debug)]
pub struct ResolvedSystemPaths {
    pub executable: Vec<PathBuf>,
    pub read_only: Vec<PathBuf>,
    pub read_write: Vec<PathBuf>,
}

impl ResolvedSystemPaths {
    pub fn from_settings(settings: &settings_content::SystemPathsSettingsContent) -> Self {
        Self {
            executable: settings
                .executable
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_executable),
            read_only: settings
                .read_only
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_only),
            read_write: settings
                .read_write
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_write),
        }
    }

    pub fn with_defaults() -> Self {
        Self {
            executable: Self::default_executable(),
            read_only: Self::default_read_only(),
            read_write: Self::default_read_write(),
        }
    }

    #[cfg(target_os = "macos")]
    fn default_executable() -> Vec<PathBuf> {
        vec![
            "/bin".into(),
            "/usr/bin".into(),
            "/usr/sbin".into(),
            "/sbin".into(),
            "/usr/lib".into(),
            "/usr/libexec".into(),
            "/System/Library/dyld".into(),
            "/System/Cryptexes".into(),
            "/Library/Developer/CommandLineTools/usr/bin".into(),
            "/Library/Developer/CommandLineTools/usr/lib".into(),
            "/Library/Apple/usr/bin".into(),
            "/opt/homebrew/bin".into(),
            "/opt/homebrew/sbin".into(),
            "/opt/homebrew/Cellar".into(),
            "/opt/homebrew/lib".into(),
            "/usr/local/bin".into(),
            "/usr/local/lib".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_executable() -> Vec<PathBuf> {
        vec![
            "/usr/bin".into(),
            "/usr/sbin".into(),
            "/usr/lib".into(),
            "/usr/lib64".into(),
            "/usr/libexec".into(),
            "/lib".into(),
            "/lib64".into(),
            "/bin".into(),
            "/sbin".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_executable() -> Vec<PathBuf> {
        vec![]
    }

    #[cfg(target_os = "macos")]
    fn default_read_only() -> Vec<PathBuf> {
        vec![
            "/private/etc".into(),
            "/usr/share".into(),
            "/System/Library/Keychains".into(),
            "/Library/Developer/CommandLineTools/SDKs".into(),
            "/Library/Preferences/SystemConfiguration".into(),
            "/opt/homebrew/share".into(),
            "/opt/homebrew/etc".into(),
            "/usr/local/share".into(),
            "/usr/local/etc".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_read_only() -> Vec<PathBuf> {
        vec![
            "/etc".into(),
            "/usr/share".into(),
            "/usr/include".into(),
            "/usr/lib/locale".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_read_only() -> Vec<PathBuf> {
        vec![]
    }

    #[cfg(target_os = "macos")]
    fn default_read_write() -> Vec<PathBuf> {
        vec![
            "/dev".into(),
            "/private/tmp".into(),
            "/var/folders".into(),
            "/private/var/run/mDNSResponder".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_read_write() -> Vec<PathBuf> {
        vec![
            "/dev".into(),
            "/tmp".into(),
            "/var/tmp".into(),
            "/dev/shm".into(),
            "/run/user".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_read_write() -> Vec<PathBuf> {
        vec![]
    }
}

impl SandboxConfig {
    /// Shell configuration dotfiles that need read-only access.
    /// Both macOS and Linux sandbox implementations use this list.
    pub const READ_ONLY_DOTFILES: &[&str] = &[
        ".bashrc",
        ".bash_login",
        ".bash_profile",
        ".gitconfig",
        ".inputrc",
        ".profile",
        ".terminfo",
        ".zlogin",
        ".zlogout",
        ".zprofile",
        ".zshenv",
        ".zshrc",
    ];

    /// Shell history dotfiles that need read-write access so shells can
    /// persist command history without silent failures.
    pub const READ_WRITE_DOTFILES: &[&str] = &[
        ".bash_history",
        ".zsh_history",
    ];

    /// Default environment variables to pass through to sandboxed terminals.
    pub fn default_allowed_env_vars() -> Vec<String> {
        vec![
            "PATH".into(),
            "HOME".into(),
            "USER".into(),
            "SHELL".into(),
            "LANG".into(),
            "TERM".into(),
            "TERM_PROGRAM".into(),
            "CARGO_HOME".into(),
            "RUSTUP_HOME".into(),
            "GOPATH".into(),
            "EDITOR".into(),
            "VISUAL".into(),
            "XDG_CONFIG_HOME".into(),
            "XDG_DATA_HOME".into(),
            "XDG_RUNTIME_DIR".into(),
            "SSH_AUTH_SOCK".into(),
            "GPG_TTY".into(),
            "COLORTERM".into(),
        ]
    }

    /// Resolve a `SandboxConfig` from settings, applying all defaults.
    pub fn from_settings(
        sandbox_settings: &settings_content::SandboxSettingsContent,
        project_dir: PathBuf,
    ) -> Self {
        let system_paths = sandbox_settings
            .system_paths
            .as_ref()
            .map(|sp| ResolvedSystemPaths::from_settings(sp))
            .unwrap_or_else(ResolvedSystemPaths::with_defaults);

        let home_dir = std::env::var("HOME").ok().map(PathBuf::from);
        let expand_paths = |paths: &Option<Vec<String>>| -> Vec<PathBuf> {
            paths
                .as_ref()
                .map(|v| {
                    v.iter()
                        .map(|p| {
                            if let Some(rest) = p.strip_prefix("~/") {
                                if let Some(ref home) = home_dir {
                                    return home.join(rest);
                                }
                            }
                            PathBuf::from(p)
                        })
                        .collect()
                })
                .unwrap_or_default()
        };

        Self {
            project_dir,
            system_paths,
            additional_executable_paths: expand_paths(
                &sandbox_settings.additional_executable_paths,
            ),
            additional_read_only_paths: expand_paths(&sandbox_settings.additional_read_only_paths),
            additional_read_write_paths: expand_paths(
                &sandbox_settings.additional_read_write_paths,
            ),
            allow_network: sandbox_settings.allow_network.unwrap_or(true),
            allowed_env_vars: sandbox_settings
                .allowed_env_vars
                .clone()
                .unwrap_or_else(Self::default_allowed_env_vars),
        }
    }

    /// Resolve sandbox config from settings if enabled and applicable for the given target.
    ///
    /// The caller is responsible for checking feature flags before calling this.
    /// `target` should be `SandboxApplyTo::Terminal` for user terminals or
    /// `SandboxApplyTo::Tool` for agent terminal tools.
    pub fn resolve_if_enabled(
        sandbox_settings: &settings_content::SandboxSettingsContent,
        target: settings_content::SandboxApplyTo,
        project_dir: PathBuf,
    ) -> Option<Self> {
        if !sandbox_settings.enabled.unwrap_or(false) {
            return None;
        }
        let apply_to = sandbox_settings.apply_to.unwrap_or_default();
        let applies = match target {
            settings_content::SandboxApplyTo::Terminal => matches!(
                apply_to,
                settings_content::SandboxApplyTo::Terminal | settings_content::SandboxApplyTo::Both
            ),
            settings_content::SandboxApplyTo::Tool => matches!(
                apply_to,
                settings_content::SandboxApplyTo::Tool | settings_content::SandboxApplyTo::Both
            ),
            settings_content::SandboxApplyTo::Both => {
                matches!(apply_to, settings_content::SandboxApplyTo::Both)
            }
            settings_content::SandboxApplyTo::Neither => false,
        };
        if !applies {
            return None;
        }
        Some(Self::from_settings(sandbox_settings, project_dir))
    }

    pub fn canonicalize_paths(&mut self) {
        match std::fs::canonicalize(&self.project_dir) {
            Ok(canonical) => self.project_dir = canonical,
            Err(err) => log::warn!(
                "Failed to canonicalize project dir {:?}: {}",
                self.project_dir,
                err
            ),
        }
        canonicalize_path_list(&mut self.system_paths.executable);
        canonicalize_path_list(&mut self.system_paths.read_only);
        canonicalize_path_list(&mut self.system_paths.read_write);
        canonicalize_path_list(&mut self.additional_executable_paths);
        canonicalize_path_list(&mut self.additional_read_only_paths);
        canonicalize_path_list(&mut self.additional_read_write_paths);
    }
}

fn try_canonicalize(path: &mut PathBuf) {
    if let Ok(canonical) = std::fs::canonicalize(&*path) {
        *path = canonical;
    }
}

fn canonicalize_path_list(paths: &mut Vec<PathBuf>) {
    for path in paths.iter_mut() {
        try_canonicalize(path);
    }
}

/// Platform-specific session tracker for process lifetime management.
///
/// On macOS, uses Seatbelt fingerprinting with `sandbox_check()`.
/// On Linux, uses cgroups v2.
/// On other platforms, tracking is not available.
#[cfg(target_os = "macos")]
pub struct SessionTracker {
    pub(crate) fingerprint: sandbox_macos::SessionFingerprint,
}

#[cfg(target_os = "linux")]
pub struct SessionTracker {
    pub(crate) cgroup: Option<cgroup::CgroupSession>,
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub struct SessionTracker;

#[cfg(target_os = "macos")]
impl SessionTracker {
    /// Create a new session tracker. On macOS, creates a SessionFingerprint.
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            fingerprint: sandbox_macos::SessionFingerprint::new()?,
        })
    }

    /// Get the fingerprint UUID string for passing to the child process.
    pub fn fingerprint_uuid(&self) -> Option<String> {
        Some(self.fingerprint.uuid_string())
    }

    /// Get the cgroup path for passing to the child process (macOS: always None).
    pub fn cgroup_path(&self) -> Option<String> {
        None
    }

    /// Kill all processes belonging to this session.
    pub fn kill_all_processes(&self, process_group_id: Option<libc::pid_t>) {
        self.fingerprint.kill_all_processes(process_group_id);
    }
}

#[cfg(target_os = "linux")]
impl SessionTracker {
    /// Create a new session tracker. On Linux, creates a CgroupSession.
    pub fn new() -> std::io::Result<Self> {
        match cgroup::CgroupSession::new() {
            Ok(cgroup) => Ok(Self {
                cgroup: Some(cgroup),
            }),
            Err(err) => {
                log::warn!("Failed to create cgroup session, process tracking degraded: {err}");
                Ok(Self { cgroup: None })
            }
        }
    }

    /// Get the fingerprint UUID string (Linux: always None).
    pub fn fingerprint_uuid(&self) -> Option<String> {
        None
    }

    /// Get the cgroup path for passing to the child process.
    pub fn cgroup_path(&self) -> Option<String> {
        self.cgroup.as_ref().map(|c| c.path_string())
    }

    /// Kill all processes belonging to this session.
    pub fn kill_all_processes(&self, process_group_id: Option<libc::pid_t>) {
        // Best-effort process group kill first
        if let Some(pgid) = process_group_id {
            unsafe {
                libc::killpg(pgid, libc::SIGKILL);
            }
        }

        if let Some(ref cgroup) = self.cgroup {
            cgroup.kill_all_and_cleanup();
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
impl SessionTracker {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self)
    }

    pub fn fingerprint_uuid(&self) -> Option<String> {
        None
    }

    pub fn cgroup_path(&self) -> Option<String> {
        None
    }

    pub fn kill_all_processes(&self, _process_group_id: Option<libc::pid_t>) {}
}
