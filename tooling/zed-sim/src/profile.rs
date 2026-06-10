//! Disposable Zed profiles: a throwaway `--user-data-dir` per launch.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use smol::process::Command;
use uuid::Uuid;

use crate::states::SimState;

/// Root directory under which all scratch profiles live, so they can be wiped
/// as a group without affecting the user's real Zed data.
pub fn profiles_root() -> PathBuf {
    std::env::temp_dir().join("zed-sim")
}

/// A single disposable profile on disk.
pub struct Profile {
    pub dir: PathBuf,
}

impl Profile {
    /// Creates a fresh profile directory and writes any state-specific settings.
    pub fn create(state: SimState) -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let dir = profiles_root().join(&id);

        // With a custom data dir, Zed reads settings from `<data-dir>/config`
        // (see `paths::config_dir`).
        let config_dir = dir.join("config");
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("creating profile config dir at {}", config_dir.display()))?;

        let config = state.launch_config();
        let mut settings = serde_json::Map::new();
        if config.isolate_credentials {
            settings.insert(
                "credentials_url".to_string(),
                serde_json::Value::String(format!("zed-sim://{id}")),
            );
        }
        if let Some(server_url) = &config.server_url {
            settings.insert(
                "server_url".to_string(),
                serde_json::Value::String(server_url.clone()),
            );
        }
        if !settings.is_empty() {
            let settings_path = config_dir.join("settings.json");
            let body = serde_json::to_string_pretty(&serde_json::Value::Object(settings))?;
            fs::write(&settings_path, body)
                .with_context(|| format!("writing {}", settings_path.display()))?;
        }

        Ok(Self { dir })
    }

    /// Spawns Zed pointed at this profile. Returns immediately; the spawned
    /// process is detached and keeps running after this tool drops the handle.
    pub fn launch(&self, zed_binary: &Path) -> Result<()> {
        Command::new(zed_binary)
            .arg("--user-data-dir")
            .arg(&self.dir)
            .spawn()
            .with_context(|| format!("launching {}", zed_binary.display()))?;
        Ok(())
    }
}

/// Removes every scratch profile created by this tool. Returns how many were
/// removed.
pub fn wipe_profiles() -> Result<usize> {
    let root = profiles_root();
    if !root.exists() {
        return Ok(0);
    }

    let mut removed = 0;
    for entry in fs::read_dir(&root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path).with_context(|| format!("removing {}", path.display()))?;
            removed += 1;
        }
    }
    Ok(removed)
}
