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

/// Returns the path to the support directory used by Zed.
pub fn support_dir() -> &'static PathBuf {
    static SUPPORT_DIR: OnceLock<PathBuf> = OnceLock::new();
    SUPPORT_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            return HOME.join("Library/Application Support/Zed");
        }

        if cfg!(target_os = "linux") {
            return if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
                flatpak_xdg_data.into()
            } else {
                dirs::data_local_dir().expect("failed to determine XDG_DATA_HOME directory")
            }
            .join("zed");
        }

        if cfg!(target_os = "windows") {
            return dirs::data_local_dir()
                .expect("failed to determine LocalAppData directory")
                .join("Zed");
        }

        config_dir().clone()
    })
}

/// Returns the path to the temp directory used by Zed.
pub fn temp_dir() -> &'static PathBuf {
    static TEMP_DIR: OnceLock<PathBuf> = OnceLock::new();
    TEMP_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            return dirs::cache_dir()
                .expect("failed to determine LocalAppData directory")
                .join("Zed");
        }

        if cfg!(target_os = "linux") {
            return if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
                flatpak_xdg_cache.into()
            } else {
                dirs::cache_dir().expect("failed to determine XDG_CACHE_HOME directory")
            }
            .join("zed");
        }

        HOME.join(".cache").join("zed")
    })
}

/// Returns the path to the logs directory.
pub fn logs_dir() -> &'static PathBuf {
    static LOGS_DIR: OnceLock<PathBuf> = OnceLock::new();
    LOGS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            HOME.join("Library/Logs/Zed")
        } else {
            support_dir().join("logs")
        }
    })
}

/// Returns the path to the `Zed.log` file.
pub fn log_file() -> &'static PathBuf {
    static LOG_FILE: OnceLock<PathBuf> = OnceLock::new();
    LOG_FILE.get_or_init(|| logs_dir().join("Zed.log"))
}

/// Returns the path to the `Zed.log.old` file.
pub fn old_log_file() -> &'static PathBuf {
    static OLD_LOG_FILE: OnceLock<PathBuf> = OnceLock::new();
    OLD_LOG_FILE.get_or_init(|| logs_dir().join("Zed.log.old"))
}

/// Returns the path to the extensions directory.
///
/// This is where installed extensions are stored.
pub fn extensions_dir() -> &'static PathBuf {
    static EXTENSIONS_DIR: OnceLock<PathBuf> = OnceLock::new();
    EXTENSIONS_DIR.get_or_init(|| support_dir().join("extensions"))
}

/// Returns the path to the themes directory.
///
/// This is where themes that are not provided by extensions are stored.
pub fn themes_dir() -> &'static PathBuf {
    static THEMES_DIR: OnceLock<PathBuf> = OnceLock::new();
    THEMES_DIR.get_or_init(|| config_dir().join("themes"))
}

/// Returns the path to the contexts directory.
///
/// This is where the saved contexts from the Assistant are stored.
pub fn contexts_dir() -> &'static PathBuf {
    static CONTEXTS_DIR: OnceLock<PathBuf> = OnceLock::new();
    CONTEXTS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("conversations")
        } else {
            support_dir().join("conversations")
        }
    })
}

/// Returns the path to the contexts directory.
///
/// This is where the prompts for use with the Assistant are stored.
pub fn prompts_dir() -> &'static PathBuf {
    static PROMPTS_DIR: OnceLock<PathBuf> = OnceLock::new();
    PROMPTS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("prompts")
        } else {
            support_dir().join("prompts")
        }
    })
}

/// Returns the path to the semantic search's embeddings directory.
///
/// This is where the embeddings used to power semantic search are stored.
pub fn embeddings_dir() -> &'static PathBuf {
    static EMBEDDINGS_DIR: OnceLock<PathBuf> = OnceLock::new();
    EMBEDDINGS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("embeddings")
        } else {
            support_dir().join("embeddings")
        }
    })
}

lazy_static::lazy_static! {
    pub static ref LANGUAGES_DIR: PathBuf = support_dir().join("languages");
    pub static ref COPILOT_DIR: PathBuf = support_dir().join("copilot");
    pub static ref SUPERMAVEN_DIR: PathBuf = support_dir().join("supermaven");
    pub static ref DEFAULT_PRETTIER_DIR: PathBuf = support_dir().join("prettier");
    pub static ref DB_DIR: PathBuf = support_dir().join("db");
    pub static ref CRASHES_DIR: Option<PathBuf> = cfg!(target_os = "macos")
        .then_some(HOME.join("Library/Logs/DiagnosticReports"));
    pub static ref CRASHES_RETIRED_DIR: Option<PathBuf> = CRASHES_DIR
        .as_ref()
        .map(|dir| dir.join("Retired"));

    pub static ref SETTINGS: PathBuf = config_dir().join("settings.json");
    pub static ref KEYMAP: PathBuf = config_dir().join("keymap.json");
    pub static ref TASKS: PathBuf = config_dir().join("tasks.json");
    pub static ref LAST_USERNAME: PathBuf = config_dir().join("last-username.txt");
    pub static ref LOCAL_SETTINGS_RELATIVE_PATH: &'static Path = Path::new(".zed/settings.json");
    pub static ref LOCAL_TASKS_RELATIVE_PATH: &'static Path = Path::new(".zed/tasks.json");
    pub static ref LOCAL_VSCODE_TASKS_RELATIVE_PATH: &'static Path = Path::new(".vscode/tasks.json");
}
