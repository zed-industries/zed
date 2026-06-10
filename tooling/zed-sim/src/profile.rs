//! Disposable Zed profiles: a throwaway `--user-data-dir` per launch.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use serde_json::{Map, Value};
use smol::process::Command;
use uuid::Uuid;

/// Root directory under which all scratch profiles live, so they can be wiped
/// as a group without affecting the user's real Zed data.
pub fn profiles_root() -> PathBuf {
    std::env::temp_dir().join("zed-sim")
}

/// A single disposable profile on disk.
pub struct Profile {
    pub dir: PathBuf,
    pub id: String,
}

impl Profile {
    /// Creates a fresh, empty profile directory (plus its `config` subdir).
    pub fn create() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let dir = profiles_root().join(&id);

        // With a custom data dir, Zed reads settings from `<data-dir>/config`
        // (see `paths::config_dir`).
        let config_dir = dir.join("config");
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("creating profile config dir at {}", config_dir.display()))?;

        Ok(Self { dir, id })
    }

    /// Writes `settings.json` into the profile. A no-op when `settings` is empty.
    pub fn write_settings(&self, settings: Map<String, Value>) -> Result<()> {
        if settings.is_empty() {
            return Ok(());
        }
        let settings_path = self.dir.join("config").join("settings.json");
        let body = serde_json::to_string_pretty(&Value::Object(settings))?;
        fs::write(&settings_path, body)
            .with_context(|| format!("writing {}", settings_path.display()))?;
        Ok(())
    }

    /// Spawns Zed pointed at this profile, with the given extra environment.
    ///
    /// Stdio is inherited so a terminal-launched session keeps a TTY stdout —
    /// which stock Zed requires before it will honor `ZED_IMPERSONATE`. Returns
    /// immediately; the spawned process is detached and outlives this tool.
    pub fn launch(&self, zed_binary: &Path, env: &[(&str, String)]) -> Result<()> {
        let mut command = Command::new(zed_binary);
        command.arg("--user-data-dir").arg(&self.dir);
        for (key, value) in env {
            command.env(key, value);
        }
        command
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
