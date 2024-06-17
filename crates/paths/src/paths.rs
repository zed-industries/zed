//! Paths to locations used by Zed.

use std::path::{Path, PathBuf};

use util::paths::HOME;

lazy_static::lazy_static! {
    pub static ref CONFIG_DIR: PathBuf = if cfg!(target_os = "windows") {
        dirs::config_dir()
            .expect("failed to determine RoamingAppData directory")
            .join("Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
           flatpak_xdg_config.into()
        } else {
            dirs::config_dir().expect("failed to determine XDG_CONFIG_HOME directory")
        }.join("zed")
    } else {
        HOME.join(".config").join("zed")
    };
    pub static ref CONTEXTS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("conversations")
    } else {
        SUPPORT_DIR.join("conversations")
    };
    pub static ref PROMPTS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("prompts")
    } else {
        SUPPORT_DIR.join("prompts")
    };
    pub static ref EMBEDDINGS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("embeddings")
    } else {
        SUPPORT_DIR.join("embeddings")
    };
    pub static ref THEMES_DIR: PathBuf = CONFIG_DIR.join("themes");

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
        CONFIG_DIR.clone()
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

    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref TASKS: PathBuf = CONFIG_DIR.join("tasks.json");
    pub static ref LAST_USERNAME: PathBuf = CONFIG_DIR.join("last-username.txt");
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
