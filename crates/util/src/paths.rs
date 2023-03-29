use std::path::{Path, PathBuf};

lazy_static::lazy_static! {
    pub static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    pub static ref CONFIG_DIR: PathBuf = HOME.join(".config").join("zed");
    pub static ref LOGS_DIR: PathBuf = HOME.join("Library/Logs/Zed");
    pub static ref SUPPORT_DIR: PathBuf = HOME.join("Library/Application Support/Zed");
    pub static ref LANGUAGES_DIR: PathBuf = HOME.join("Library/Application Support/Zed/languages");
    pub static ref DB_DIR: PathBuf = HOME.join("Library/Application Support/Zed/db");
    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref LAST_USERNAME: PathBuf = CONFIG_DIR.join("last-username.txt");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
}

pub mod legacy {
    use std::path::PathBuf;

    lazy_static::lazy_static! {
        static ref CONFIG_DIR: PathBuf = super::HOME.join(".zed");
        pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
        pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    }
}

/// Compacts a given file path by replacing the user's home directory
/// prefix with a tilde (`~`).
///
/// # Arguments
///
/// * `path` - A reference to a `Path` representing the file path to compact.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use util::paths::compact;
/// let path: PathBuf = [
///     util::paths::HOME.to_string_lossy().to_string(),
///     "some_file.txt".to_string(),
///  ]
///  .iter()
///  .collect();
/// if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
///     assert_eq!(compact(&path).to_str(), Some("~/some_file.txt"));
/// } else {
///     assert_eq!(compact(&path).to_str(), path.to_str());
/// }
/// ```
///
/// # Returns
///
/// * A `PathBuf` containing the compacted file path. If the input path
///   does not have the user's home directory prefix, or if we are not on
///   Linux or macOS, the original path is returned unchanged.
///
/// # Errors
///
/// This function will not produce any errors, but in case the input path
/// cannot be stripped of the home directory prefix, the original path
/// will be returned unchanged.
pub fn compact(path: &Path) -> PathBuf {
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        match path.strip_prefix(HOME.as_path()) {
            Ok(relative_path) => {
                let mut shortened_path = PathBuf::new();
                shortened_path.push("~");
                shortened_path.push(relative_path);
                shortened_path
            }
            Err(_) => path.to_path_buf(),
        }
    } else {
        path.to_path_buf()
    }
}
