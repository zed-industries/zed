//! Linux Landlock sandbox implementation.
//!
//! Uses the Landlock LSM to restrict filesystem access for the current process.
//! Must be called after fork(), before exec().

use landlock::{
    ABI, Access, AccessFs, AccessNet, PathBeneath, PathFd, Ruleset, RulesetAttr,
    RulesetCreatedAttr, RulesetStatus,
};
use std::io::{Error, Result};
use std::path::Path;

use crate::SandboxConfig;

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
            log::debug!(
                "Landlock: skipping nonexistent path {}: {e}",
                path.display()
            );
            Ok(ruleset)
        }
    }
}

/// Apply a Landlock sandbox to the current process.
/// Must be called after fork(), before exec().
pub fn apply_sandbox(config: &SandboxConfig) -> Result<()> {
    let ret = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
    if ret != 0 {
        return Err(Error::last_os_error());
    }

    let ruleset_base = Ruleset::default()
        .handle_access(AccessFs::from_all(TARGET_ABI))
        .map_err(|e| Error::other(format!("landlock ruleset create: {e}")))?;

    let ruleset_with_net = if !config.allow_network {
        ruleset_base
            .handle_access(AccessNet::from_all(TARGET_ABI))
            .map_err(|e| {
                Error::other(format!(
                    "landlock network restriction not supported (requires kernel 6.4+): {e}"
                ))
            })?
    } else {
        ruleset_base
    };

    let mut ruleset = ruleset_with_net
        .create()
        .map_err(|e| Error::other(format!("landlock ruleset init: {e}")))?;

    for path in &config.system_paths.executable {
        ruleset = add_path_rule(ruleset, path, fs_read_exec())
            .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
    }

    for path in &config.system_paths.read_only {
        ruleset = add_path_rule(ruleset, path, fs_read())
            .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
    }

    for path in &config.system_paths.read_write {
        ruleset = add_path_rule(ruleset, path, fs_all())
            .map_err(|e| Error::other(format!("landlock rule: {e}")))?;
    }

    ruleset = add_path_rule(ruleset, &config.project_dir, fs_all())
        .map_err(|e| Error::other(format!("landlock project rule: {e}")))?;

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

    if let Ok(home) = std::env::var("HOME") {
        let home = Path::new(&home);
        for dotfile in &[
            ".bashrc",
            ".bash_profile",
            ".bash_login",
            ".profile",
            ".zshrc",
            ".zshenv",
            ".zprofile",
            ".zlogin",
            ".zlogout",
            ".inputrc",
            ".terminfo",
            ".gitconfig",
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
        let proc_self = Path::new("/proc/self");
        if proc_self.exists() {
            ruleset = add_path_rule(ruleset, proc_self, fs_read())
                .map_err(|e| Error::other(format!("landlock /proc/self rule: {e}")))?;
        }
    }

    let status = ruleset
        .restrict_self()
        .map_err(|e| Error::other(format!("landlock restrict_self: {e}")))?;

    match status.ruleset {
        RulesetStatus::FullyEnforced => {
            log::info!("Landlock sandbox fully enforced");
        }
        RulesetStatus::PartiallyEnforced => {
            return Err(Error::other(
                "Landlock sandbox only partially enforced on this kernel. \
                 The sandbox cannot guarantee the requested restrictions. \
                 Upgrade to kernel 6.4+ for full enforcement, or disable sandboxing.",
            ));
        }
        RulesetStatus::NotEnforced => {
            return Err(Error::other(
                "Landlock is not supported on this kernel (requires 5.13+). \
                 The terminal cannot be sandboxed. \
                 Upgrade your kernel or disable sandboxing.",
            ));
        }
    }

    Ok(())
}
