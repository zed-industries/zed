//! Paths to locations used by Zed.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use util::paths::HOME;

/// Returns the path to the configuration directory used by Zed.
pub fn config_dir() -> &'static PathBuf {
    static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();
    CONFIG_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            return dirs::config_dir()
                .expect("failed to determine RoamingAppData directory")
                .join("Zed");
        }

        if cfg!(target_os = "linux") {
            return if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                flatpak_xdg_config.into()
            } else {
                dirs::config_dir().expect("failed to determine XDG_CONFIG_HOME directory")
            }
            .join("zed");
        }

        HOME.join(".config").join("zed")
    })
}

lazy_static::lazy_static! {
    pub static ref CONTEXTS_DIR: PathBuf = if cfg!(target_os = "macos") {
        config_dir().join("conversations")
    } else {
        SUPPORT_DIR.join("conversations")
    };
    pub static ref PROMPTS_DIR: PathBuf = if cfg!(target_os = "macos") {
        config_dir().join("prompts")
    } else {
        SUPPORT_DIR.join("prompts")
    };
    pub static ref EMBEDDINGS_DIR: PathBuf = if cfg!(target_os = "macos") {
        config_dir().join("embeddings")
    } else {
        SUPPORT_DIR.join("embeddings")
    };
    pub static ref THEMES_DIR: PathBuf = config_dir().join("themes");

    pub static ref SUPPORT_DIR: PathBuf = if cfg!(target_os = "macos") {
        HOME.join("Library/Application Support/Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
            flatpak_xdg_data.into()
        } else {
            dirs::data_local_dir().expect("failed to determine XDG_DATA_HOME directory")
        }.join("zed")
    } else if cfg!(target_os = "windows") {
        dirs::data_local_dir()
            .expect("failed to determine LocalAppData directory")
            .join("Zed")
    } else {
        config_dir().clone()
    };
    pub static ref LOGS_DIR: PathBuf = if cfg!(target_os = "macos") {
        HOME.join("Library/Logs/Zed")
    } else {
        SUPPORT_DIR.join("logs")
    };
    pub static ref EXTENSIONS_DIR: PathBuf = SUPPORT_DIR.join("extensions");
    pub static ref LANGUAGES_DIR: PathBuf = SUPPORT_DIR.join("languages");
    pub static ref COPILOT_DIR: PathBuf = SUPPORT_DIR.join("copilot");
    pub static ref SUPERMAVEN_DIR: PathBuf = SUPPORT_DIR.join("supermaven");
    pub static ref DEFAULT_PRETTIER_DIR: PathBuf = SUPPORT_DIR.join("prettier");
    pub static ref DB_DIR: PathBuf = SUPPORT_DIR.join("db");
    pub static ref CRASHES_DIR: Option<PathBuf> = cfg!(target_os = "macos")
        .then_some(HOME.join("Library/Logs/DiagnosticReports"));
    pub static ref CRASHES_RETIRED_DIR: Option<PathBuf> = CRASHES_DIR
        .as_ref()
        .map(|dir| dir.join("Retired"));

    pub static ref SETTINGS: PathBuf = config_dir().join("settings.json");
    pub static ref KEYMAP: PathBuf = config_dir().join("keymap.json");
    pub static ref TASKS: PathBuf = config_dir().join("tasks.json");
    pub static ref LAST_USERNAME: PathBuf = config_dir().join("last-username.txt");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
    pub static ref LOCAL_SETTINGS_RELATIVE_PATH: &'static Path = Path::new(".zed/settings.json");
    pub static ref LOCAL_TASKS_RELATIVE_PATH: &'static Path = Path::new(".zed/tasks.json");
    pub static ref LOCAL_VSCODE_TASKS_RELATIVE_PATH: &'static Path = Path::new(".vscode/tasks.json");
    pub static ref TEMP_DIR: PathBuf = if cfg!(target_os = "windows") {
        dirs::cache_dir()
            .expect("failed to determine LocalAppData directory")
            .join("Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
            flatpak_xdg_cache.into()
        } else {
            dirs::cache_dir().expect("failed to determine XDG_CACHE_HOME directory")
        }.join("zed")
    } else {
        HOME.join(".cache").join("zed")
    };
}
