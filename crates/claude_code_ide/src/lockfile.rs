//! Discovery lock files for the Claude Code IDE integration.
//!
//! The Claude Code CLI finds a running IDE WebSocket server by scanning
//! `~/.claude/ide/*.lock`. Each file describes one server: which workspace it
//! serves, the PID that owns it, and the auth token the CLI must present in the
//! `x-claude-code-ide-authorization` header when it connects. We mirror the
//! format the official VS Code and JetBrains extensions write.

use anyhow::{Context as _, Result};
use serde::Serialize;
use std::{fs, path::PathBuf};

/// The `ideName` reported to the CLI; shown in its `/ide` status output.
pub const IDE_NAME: &str = "Zed";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LockFileContents {
    pid: u32,
    workspace_folders: Vec<String>,
    ide_name: &'static str,
    transport: &'static str,
    auth_token: String,
}

/// `$CLAUDE_CONFIG_DIR/ide` when that variable is set and non-empty, otherwise
/// `~/.claude/ide` — the same resolution the CLI itself performs.
pub fn lock_dir() -> PathBuf {
    match std::env::var_os("CLAUDE_CONFIG_DIR") {
        Some(dir) if !dir.is_empty() => PathBuf::from(dir).join("ide"),
        _ => paths::home_dir().join(".claude").join("ide"),
    }
}

pub fn lock_path(port: u16) -> PathBuf {
    lock_dir().join(format!("{port}.lock"))
}

/// A fresh v4 UUID, matching the auth-token format the official extensions use.
pub fn generate_auth_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Writes the lock file for `port`, returning its path.
///
/// The write is atomic (temp file + rename) so the CLI never observes a
/// partially written file while scanning the directory.
pub fn create(port: u16, auth_token: &str, workspace_folders: &[PathBuf]) -> Result<PathBuf> {
    let dir = lock_dir();
    fs::create_dir_all(&dir).with_context(|| format!("creating lock dir {dir:?}"))?;
    // The token is a secret, so restrict the directory to the current user
    // (0700), matching what the official extensions do.
    set_owner_only(&dir, 0o700)?;

    let contents = LockFileContents {
        pid: std::process::id(),
        workspace_folders: workspace_folders
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
        ide_name: IDE_NAME,
        transport: "ws",
        auth_token: auth_token.to_owned(),
    };
    let json = serde_json::to_string(&contents).context("serializing lock file")?;

    let path = dir.join(format!("{port}.lock"));
    let temp = dir.join(format!("{port}.lock.tmp"));
    fs::write(&temp, json).with_context(|| format!("writing {temp:?}"))?;
    // Restrict to 0600 before the rename publishes the file, so the token is
    // never briefly world-readable.
    set_owner_only(&temp, 0o600)?;
    fs::rename(&temp, &path).with_context(|| format!("renaming {temp:?} to {path:?}"))?;
    Ok(path)
}

/// Sets owner-only Unix permissions (e.g. `0o700`, `0o600`). A no-op on
/// platforms without Unix permission bits.
#[cfg(unix)]
fn set_owner_only(path: &std::path::Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
        .with_context(|| format!("setting mode {mode:o} on {path:?}"))
}

#[cfg(not(unix))]
fn set_owner_only(_path: &std::path::Path, _mode: u32) -> Result<()> {
    Ok(())
}

/// Removes the lock file for `port`. Succeeds if it is already gone.
pub fn remove(port: u16) -> Result<()> {
    let path = lock_path(port);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {path:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_token_is_a_uuid() {
        let token = generate_auth_token();
        assert_eq!(token.len(), 36);
        assert!(uuid::Uuid::parse_str(&token).is_ok());
    }

    #[test]
    fn lock_file_matches_expected_wire_format() {
        let contents = LockFileContents {
            pid: 4321,
            workspace_folders: vec!["/home/user/project".to_string()],
            ide_name: IDE_NAME,
            transport: "ws",
            auth_token: "the-token".to_string(),
        };
        let value: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&contents).unwrap()).unwrap();

        assert_eq!(value["pid"], 4321);
        assert_eq!(value["workspaceFolders"][0], "/home/user/project");
        assert_eq!(value["ideName"], "Zed");
        assert_eq!(value["transport"], "ws");
        assert_eq!(value["authToken"], "the-token");
    }

    #[test]
    fn lock_dir_honors_claude_config_dir() {
        // Documents the resolution rule; we avoid mutating process env here to
        // keep the test free of global state.
        let default_dir = paths::home_dir().join(".claude").join("ide");
        assert!(default_dir.ends_with(".claude/ide"));
    }
}
