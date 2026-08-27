//! The `~/.claude/ide/<port>.lock` discovery file.
//!
//! Claude Code scans `~/.claude/ide/` (or `$CLAUDE_CONFIG_DIR/ide/`) for
//! `<port>.lock` files and connects to the port named by the file. The JSON holds
//! the connection metadata and the shared secret the CLI must send back on the
//! WebSocket upgrade.
//!
//! Lifecycle: a private `0700` directory, remove any stale file before writing,
//! best-effort unlink on release.

use anyhow::{Context as _, Result};
use rand::RngCore as _;
use serde::Serialize;
use std::path::PathBuf;

/// Name Claude Code shows for the connected editor.
pub const IDE_NAME: &str = "Zed";

/// The lockfile JSON. Field names are the wire contract; do not rename without
/// matching Claude Code.
#[derive(Debug, Serialize)]
pub struct LockData {
    pub pid: u32,
    #[serde(rename = "workspaceFolders")]
    pub workspace_folders: Vec<String>,
    #[serde(rename = "ideName")]
    pub ide_name: String,
    pub transport: String,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}

impl LockData {
    pub fn new(workspace_folders: Vec<String>, auth_token: String) -> Self {
        Self {
            pid: std::process::id(),
            workspace_folders,
            ide_name: IDE_NAME.to_string(),
            transport: "ws".to_string(),
            auth_token,
        }
    }
}

/// The directory Claude Code scans. Honours `CLAUDE_CONFIG_DIR` when set, else
/// `~/.claude/ide`.
pub fn ide_lock_dir() -> PathBuf {
    if let Some(config_dir) = std::env::var_os("CLAUDE_CONFIG_DIR") {
        PathBuf::from(config_dir).join("ide")
    } else {
        paths::home_dir().join(".claude").join("ide")
    }
}

/// Path of the lockfile for a bound port.
pub fn lock_path_for_port(port: u16) -> PathBuf {
    ide_lock_dir().join(format!("{port}.lock"))
}

/// A 32-character lowercase hex token (128 bits) from the OS CSPRNG, matching
/// the token width Claude Code's other IDE integrations use.
pub fn generate_auth_token() -> String {
    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    let mut token = String::with_capacity(32);
    for byte in bytes {
        use std::fmt::Write as _;
        // Infallible: writing to a String never errors.
        let _ = write!(token, "{byte:02x}");
    }
    token
}

/// Write the lockfile with `0600` permissions inside a `0700` directory,
/// removing any stale file at that port first.
pub fn write_lock(port: u16, data: &LockData) -> Result<PathBuf> {
    let dir = ide_lock_dir();
    create_private_dir(&dir).with_context(|| format!("creating ide lock dir {}", dir.display()))?;

    let path = lock_path_for_port(port);
    if path.exists() {
        std::fs::remove_file(&path)
            .with_context(|| format!("removing stale ide lock {}", path.display()))?;
        log::info!("claude_code_ide: removed stale lock {}", path.display());
    }

    let json = serde_json::to_vec(data).context("serializing ide lock data")?;
    std::fs::write(&path, &json)
        .with_context(|| format!("writing ide lock {}", path.display()))?;
    restrict_file(&path)
        .with_context(|| format!("restricting ide lock permissions {}", path.display()))?;
    log::info!(
        "claude_code_ide: wrote lock {} (port {port})",
        path.display()
    );
    Ok(path)
}

/// Remove the lockfile; a missing file is not an error (the workspace may have
/// been torn down more than once).
pub fn unlink_lock(path: &PathBuf) {
    match std::fs::remove_file(path) {
        Ok(()) => log::info!("claude_code_ide: unlinked lock {}", path.display()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => log::warn!(
            "claude_code_ide: failed to unlink lock {}: {err}",
            path.display()
        ),
    }
}

fn create_private_dir(dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let perms = std::fs::Permissions::from_mode(0o700);
        std::fs::set_permissions(dir, perms)?;
    }
    Ok(())
}

fn restrict_file(path: &std::path::Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms)?;
    }
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The token is the shared secret the CLI must present back, so its shape is a
    /// wire contract: 32 lowercase hex chars, and never the same twice.
    #[test]
    fn auth_token_is_unique_lowercase_hex() {
        let token = generate_auth_token();
        assert_eq!(token.len(), 32, "got {token:?}");
        assert!(
            token
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
            "should be lowercase hex, got {token:?}"
        );
        assert_ne!(token, generate_auth_token(), "tokens must differ");
    }

    /// The field names are the wire contract, so assert the serialised shape and
    /// that a written file round-trips and unlinks.
    #[test]
    fn lock_data_shape_survives_a_write_and_unlink() {
        let dir = tempfile::tempdir().expect("temp dir");
        // Keep the real ~/.claude untouched. SAFETY: single-threaded test setup.
        unsafe {
            std::env::set_var("CLAUDE_CONFIG_DIR", dir.path());
        }

        let data = LockData::new(vec!["/tmp/project".to_string()], "deadbeef".to_string());
        let path = write_lock(12345, &data).expect("write lock");
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).expect("read back")).expect("json");
        assert_eq!(json["ideName"], "Zed");
        assert_eq!(json["transport"], "ws");
        assert_eq!(json["authToken"], "deadbeef");
        assert_eq!(json["workspaceFolders"][0], "/tmp/project");
        assert!(json["pid"].is_number());

        unlink_lock(&path);
        assert!(!path.exists(), "unlink should remove it");
        unsafe {
            std::env::remove_var("CLAUDE_CONFIG_DIR");
        }
    }
}
