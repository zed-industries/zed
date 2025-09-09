//! Paths to locations used by Zed.

use std::env;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub use util::paths::home_dir;
use util::paths::{SanitizedPath, SanitizedPathBuf};

/// A default editorconfig file name to use when resolving project settings.
pub const EDITORCONFIG_NAME: &str = ".editorconfig";

/// A custom data directory override, set only by `set_custom_data_dir`.
/// This is used to override the default data directory location.
/// The directory will be created if it doesn't exist when set.
static CUSTOM_DATA_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();

/// The resolved data directory, combining custom override or platform defaults.
/// This is set once and cached for subsequent calls.
/// On macOS, this is `~/Library/Application Support/Zed`.
/// On Linux/FreeBSD, this is `$XDG_DATA_HOME/zed`.
/// On Windows, this is `%LOCALAPPDATA%\Zed`.
static CURRENT_DATA_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();

/// The resolved config directory, combining custom override or platform defaults.
/// This is set once and cached for subsequent calls.
/// On macOS, this is `~/.config/zed`.
/// On Linux/FreeBSD, this is `$XDG_CONFIG_HOME/zed`.
/// On Windows, this is `%APPDATA%\Zed`.
static CONFIG_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();

/// Returns the relative path to the zed_server directory on the ssh host.
pub fn remote_server_dir_relative() -> &'static SanitizedPath {
    SanitizedPath::new(".zed_server")
}

/// Returns the relative path to the zed_wsl_server directory on the wsl host.
pub fn remote_wsl_server_dir_relative() -> &'static SanitizedPath {
    SanitizedPath::new(".zed_wsl_server")
}

/// Sets a custom directory for all user data, overriding the default data directory.
/// This function must be called before any other path operations that depend on the data directory.
/// The directory's path will be canonicalized to an absolute path by a blocking FS operation.
/// The directory will be created if it doesn't exist.
///
/// # Arguments
///
/// * `dir` - The path to use as the custom data directory. This will be used as the base
///   directory for all user data, including databases, extensions, and logs.
///
/// # Returns
///
/// A reference to the static `PathBuf` containing the custom data directory path.
///
/// # Panics
///
/// Panics if:
/// * Called after the data directory has been initialized (e.g., via `data_dir` or `config_dir`)
/// * The directory's path cannot be canonicalized to an absolute path
/// * The directory cannot be created
pub fn set_custom_data_dir(dir: &str) -> &'static SanitizedPathBuf {
    if CURRENT_DATA_DIR.get().is_some() || CONFIG_DIR.get().is_some() {
        panic!("set_custom_data_dir called after data_dir or config_dir was initialized");
    }
    CUSTOM_DATA_DIR.get_or_init(|| {
        let mut path = SanitizedPathBuf::from(dir);
        if path.is_relative() {
            let abs_path = path
                .canonicalize()
                .expect("failed to canonicalize custom data directory's path to an absolute path");
            path = util::paths::SanitizedPath::new(&abs_path).into()
        }
        std::fs::create_dir_all(&path).expect("failed to create custom data directory");
        path
    })
}

/// Returns the path to the configuration directory used by Zed.
pub fn config_dir() -> &'static SanitizedPathBuf {
    CONFIG_DIR.get_or_init(|| {
        if let Some(custom_dir) = CUSTOM_DATA_DIR.get() {
            custom_dir.join("config")
        } else if cfg!(target_os = "windows") {
            dirs::config_dir()
                .expect("failed to determine RoamingAppData directory")
                .join("Zed")
                .into()
        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                flatpak_xdg_config.into()
            } else {
                dirs::config_dir().expect("failed to determine XDG_CONFIG_HOME directory")
            }
            .join("zed")
            .into()
        } else {
            home_dir().join(".config").join("zed")
        }
    })
}

/// Returns the path to the data directory used by Zed.
pub fn data_dir() -> &'static SanitizedPathBuf {
    CURRENT_DATA_DIR.get_or_init(|| {
        if let Some(custom_dir) = CUSTOM_DATA_DIR.get() {
            custom_dir.clone()
        } else if cfg!(target_os = "macos") {
            home_dir().join("Library/Application Support/Zed")
        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
                flatpak_xdg_data.into()
            } else {
                dirs::data_local_dir().expect("failed to determine XDG_DATA_HOME directory")
            }
            .join("zed")
            .into()
        } else if cfg!(target_os = "windows") {
            dirs::data_local_dir()
                .expect("failed to determine LocalAppData directory")
                .join("Zed")
                .into()
        } else {
            config_dir().clone() // Fallback
        }
    })
}

/// Returns the path to the temp directory used by Zed.
pub fn temp_dir() -> &'static SanitizedPathBuf {
    static TEMP_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    TEMP_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            return dirs::cache_dir()
                .expect("failed to determine cachesDirectory directory")
                .join("Zed")
                .into();
        }

        if cfg!(target_os = "windows") {
            return dirs::cache_dir()
                .expect("failed to determine LocalAppData directory")
                .join("Zed")
                .into();
        }

        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            return if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
                flatpak_xdg_cache.into()
            } else {
                dirs::cache_dir().expect("failed to determine XDG_CACHE_HOME directory")
            }
            .join("zed")
            .into();
        }

        home_dir().join(".cache").join("zed")
    })
}

/// Returns the path to the logs directory.
pub fn logs_dir() -> &'static SanitizedPathBuf {
    static LOGS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    LOGS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            home_dir().join("Library/Logs/Zed")
        } else {
            data_dir().join("logs")
        }
    })
}

/// Returns the path to the Zed server directory on this SSH host.
pub fn remote_server_state_dir() -> &'static SanitizedPathBuf {
    static REMOTE_SERVER_STATE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    REMOTE_SERVER_STATE.get_or_init(|| data_dir().join("server_state"))
}

/// Returns the path to the `Zed.log` file.
pub fn log_file() -> &'static SanitizedPathBuf {
    static LOG_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    LOG_FILE.get_or_init(|| logs_dir().join("Zed.log"))
}

/// Returns the path to the `Zed.log.old` file.
pub fn old_log_file() -> &'static SanitizedPathBuf {
    static OLD_LOG_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    OLD_LOG_FILE.get_or_init(|| logs_dir().join("Zed.log.old"))
}

/// Returns the path to the database directory.
pub fn database_dir() -> &'static SanitizedPathBuf {
    static DATABASE_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    DATABASE_DIR.get_or_init(|| data_dir().join("db"))
}

/// Returns the path to the crashes directory, if it exists for the current platform.
pub fn crashes_dir() -> &'static Option<SanitizedPathBuf> {
    static CRASHES_DIR: OnceLock<Option<SanitizedPathBuf>> = OnceLock::new();
    CRASHES_DIR.get_or_init(|| {
        cfg!(target_os = "macos").then_some(home_dir().join("Library/Logs/DiagnosticReports"))
    })
}

/// Returns the path to the retired crashes directory, if it exists for the current platform.
pub fn crashes_retired_dir() -> &'static Option<SanitizedPathBuf> {
    static CRASHES_RETIRED_DIR: OnceLock<Option<SanitizedPathBuf>> = OnceLock::new();
    CRASHES_RETIRED_DIR.get_or_init(|| crashes_dir().as_ref().map(|dir| dir.join("Retired")))
}

/// Returns the path to the `settings.json` file.
pub fn settings_file() -> &'static SanitizedPathBuf {
    static SETTINGS_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    SETTINGS_FILE.get_or_init(|| config_dir().join("settings.json"))
}

/// Returns the path to the global settings file.
pub fn global_settings_file() -> &'static SanitizedPathBuf {
    static GLOBAL_SETTINGS_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    GLOBAL_SETTINGS_FILE.get_or_init(|| config_dir().join("global_settings.json"))
}

/// Returns the path to the `settings_backup.json` file.
pub fn settings_backup_file() -> &'static SanitizedPathBuf {
    static SETTINGS_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    SETTINGS_FILE.get_or_init(|| config_dir().join("settings_backup.json"))
}

/// Returns the path to the `keymap.json` file.
pub fn keymap_file() -> &'static SanitizedPathBuf {
    static KEYMAP_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    KEYMAP_FILE.get_or_init(|| config_dir().join("keymap.json"))
}

/// Returns the path to the `keymap_backup.json` file.
pub fn keymap_backup_file() -> &'static SanitizedPathBuf {
    static KEYMAP_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    KEYMAP_FILE.get_or_init(|| config_dir().join("keymap_backup.json"))
}

/// Returns the path to the `tasks.json` file.
pub fn tasks_file() -> &'static SanitizedPathBuf {
    static TASKS_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    TASKS_FILE.get_or_init(|| config_dir().join("tasks.json"))
}

/// Returns the path to the `debug.json` file.
pub fn debug_scenarios_file() -> &'static SanitizedPathBuf {
    static DEBUG_SCENARIOS_FILE: OnceLock<SanitizedPathBuf> = OnceLock::new();
    DEBUG_SCENARIOS_FILE.get_or_init(|| config_dir().join("debug.json"))
}

/// Returns the path to the extensions directory.
///
/// This is where installed extensions are stored.
pub fn extensions_dir() -> &'static SanitizedPathBuf {
    static EXTENSIONS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    EXTENSIONS_DIR.get_or_init(|| data_dir().join("extensions"))
}

/// Returns the path to the extensions directory.
///
/// This is where installed extensions are stored on a remote.
pub fn remote_extensions_dir() -> &'static SanitizedPathBuf {
    static EXTENSIONS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    EXTENSIONS_DIR.get_or_init(|| data_dir().join("remote_extensions"))
}

/// Returns the path to the extensions directory.
///
/// This is where installed extensions are stored on a remote.
pub fn remote_extensions_uploads_dir() -> &'static SanitizedPathBuf {
    static UPLOAD_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    UPLOAD_DIR.get_or_init(|| remote_extensions_dir().join("uploads"))
}

/// Returns the path to the themes directory.
///
/// This is where themes that are not provided by extensions are stored.
pub fn themes_dir() -> &'static SanitizedPathBuf {
    static THEMES_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    THEMES_DIR.get_or_init(|| config_dir().join("themes"))
}

/// Returns the path to the snippets directory.
pub fn snippets_dir() -> &'static SanitizedPathBuf {
    static SNIPPETS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    SNIPPETS_DIR.get_or_init(|| config_dir().join("snippets"))
}

/// Returns the path to the contexts directory.
///
/// This is where the saved contexts from the Assistant are stored.
pub fn contexts_dir() -> &'static SanitizedPathBuf {
    static CONTEXTS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    CONTEXTS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("conversations")
        } else {
            data_dir().join("conversations")
        }
    })
}

/// Returns the path to the contexts directory.
///
/// This is where the prompts for use with the Assistant are stored.
pub fn prompts_dir() -> &'static SanitizedPathBuf {
    static PROMPTS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    PROMPTS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("prompts")
        } else {
            data_dir().join("prompts")
        }
    })
}

/// Returns the path to the prompt templates directory.
///
/// This is where the prompt templates for core features can be overridden with templates.
///
/// # Arguments
///
/// * `dev_mode` - If true, assumes the current working directory is the Zed repository.
pub fn prompt_overrides_dir(repo_path: Option<&SanitizedPath>) -> SanitizedPathBuf {
    if let Some(path) = repo_path {
        let dev_path = path.join("assets").join("prompts");
        if dev_path.exists() {
            return dev_path.into();
        }
    }

    static PROMPT_TEMPLATES_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    PROMPT_TEMPLATES_DIR
        .get_or_init(|| {
            if cfg!(target_os = "macos") {
                config_dir().join("prompt_overrides")
            } else {
                data_dir().join("prompt_overrides")
            }
        })
        .clone()
}

/// Returns the path to the semantic search's embeddings directory.
///
/// This is where the embeddings used to power semantic search are stored.
pub fn embeddings_dir() -> &'static SanitizedPathBuf {
    static EMBEDDINGS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    EMBEDDINGS_DIR.get_or_init(|| {
        if cfg!(target_os = "macos") {
            config_dir().join("embeddings")
        } else {
            data_dir().join("embeddings")
        }
    })
}

/// Returns the path to the languages directory.
///
/// This is where language servers are downloaded to for languages built-in to Zed.
pub fn languages_dir() -> &'static SanitizedPathBuf {
    static LANGUAGES_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    LANGUAGES_DIR.get_or_init(|| data_dir().join("languages"))
}

/// Returns the path to the debug adapters directory
///
/// This is where debug adapters are downloaded to for DAPs that are built-in to Zed.
pub fn debug_adapters_dir() -> &'static SanitizedPathBuf {
    static DEBUG_ADAPTERS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    DEBUG_ADAPTERS_DIR.get_or_init(|| data_dir().join("debug_adapters"))
}

/// Returns the path to the agent servers directory
///
/// This is where agent servers are downloaded to
pub fn agent_servers_dir() -> &'static SanitizedPathBuf {
    static AGENT_SERVERS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    AGENT_SERVERS_DIR.get_or_init(|| data_dir().join("agent_servers"))
}

/// Returns the path to the Copilot directory.
pub fn copilot_dir() -> &'static SanitizedPathBuf {
    static COPILOT_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    COPILOT_DIR.get_or_init(|| data_dir().join("copilot"))
}

/// Returns the path to the Supermaven directory.
pub fn supermaven_dir() -> &'static SanitizedPathBuf {
    static SUPERMAVEN_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    SUPERMAVEN_DIR.get_or_init(|| data_dir().join("supermaven"))
}

/// Returns the path to the default Prettier directory.
pub fn default_prettier_dir() -> &'static SanitizedPathBuf {
    static DEFAULT_PRETTIER_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    DEFAULT_PRETTIER_DIR.get_or_init(|| data_dir().join("prettier"))
}

/// Returns the path to the remote server binaries directory.
pub fn remote_servers_dir() -> &'static SanitizedPathBuf {
    static REMOTE_SERVERS_DIR: OnceLock<SanitizedPathBuf> = OnceLock::new();
    REMOTE_SERVERS_DIR.get_or_init(|| data_dir().join("remote_servers"))
}

/// Returns the relative path to a `.zed` folder within a project.
pub fn local_settings_folder_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".zed")
}

/// Returns the relative path to a `.vscode` folder within a project.
pub fn local_vscode_folder_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".vscode")
}

/// Returns the relative path to a `settings.json` file within a project.
pub fn local_settings_file_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".zed/settings.json")
}

/// Returns the relative path to a `tasks.json` file within a project.
pub fn local_tasks_file_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".zed/tasks.json")
}

/// Returns the relative path to a `.vscode/tasks.json` file within a project.
pub fn local_vscode_tasks_file_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".vscode/tasks.json")
}

pub fn debug_task_file_name() -> &'static str {
    "debug.json"
}

pub fn task_file_name() -> &'static str {
    "tasks.json"
}

/// Returns the relative path to a `debug.json` file within a project.
/// .zed/debug.json
pub fn local_debug_file_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".zed/debug.json")
}

/// Returns the relative path to a `.vscode/launch.json` file within a project.
pub fn local_vscode_launch_file_relative_path() -> &'static SanitizedPath {
    SanitizedPath::new(".vscode/launch.json")
}

pub fn user_ssh_config_file() -> SanitizedPathBuf {
    home_dir().join(".ssh/config")
}

pub fn global_ssh_config_file() -> &'static SanitizedPath {
    SanitizedPath::new("/etc/ssh/ssh_config")
}

/// Returns candidate paths for the vscode user settings file
pub fn vscode_settings_file_paths() -> Vec<SanitizedPathBuf> {
    let mut paths = vscode_user_data_paths();
    for path in paths.iter_mut() {
        path.push("User/settings.json");
    }
    paths
}

/// Returns candidate paths for the cursor user settings file
pub fn cursor_settings_file_paths() -> Vec<SanitizedPathBuf> {
    let mut paths = cursor_user_data_paths();
    for path in paths.iter_mut() {
        path.push("User/settings.json");
    }
    paths
}

fn vscode_user_data_paths() -> Vec<SanitizedPathBuf> {
    // https://github.com/microsoft/vscode/blob/23e7148cdb6d8a27f0109ff77e5b1e019f8da051/src/vs/platform/environment/node/userDataPath.ts#L45
    const VSCODE_PRODUCT_NAMES: &[&str] = &[
        "Code",
        "Code - OSS",
        "VSCodium",
        "Code Dev",
        "Code - OSS Dev",
        "code-oss-dev",
    ];
    let mut paths = Vec::new();
    if let Ok(portable_path) = env::var("VSCODE_PORTABLE") {
        paths.push(SanitizedPath::new(&portable_path).join("user-data"));
    }
    if let Ok(vscode_appdata) = env::var("VSCODE_APPDATA") {
        for product_name in VSCODE_PRODUCT_NAMES {
            paths.push(SanitizedPath::new(&vscode_appdata).join(product_name));
        }
    }
    for product_name in VSCODE_PRODUCT_NAMES {
        add_vscode_user_data_paths(&mut paths, product_name);
    }
    paths
}

fn cursor_user_data_paths() -> Vec<SanitizedPathBuf> {
    let mut paths = Vec::new();
    add_vscode_user_data_paths(&mut paths, "Cursor");
    paths
}

fn add_vscode_user_data_paths(paths: &mut Vec<SanitizedPathBuf>, product_name: &str) {
    if cfg!(target_os = "macos") {
        paths.push(
            home_dir()
                .join("Library/Application Support")
                .join(product_name),
        );
    } else if cfg!(target_os = "windows") {
        if let Some(data_local_dir) = dirs::data_local_dir() {
            paths.push(data_local_dir.join(product_name).into());
        }
        if let Some(data_dir) = dirs::data_dir() {
            paths.push(data_dir.join(product_name).into());
        }
    } else {
        paths.push(
            dirs::config_dir()
                .map(|e| e.into())
                .unwrap_or(home_dir().join(".config"))
                .join(product_name),
        );
    }
}
