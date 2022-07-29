use std::{env, path::PathBuf};

use lazy_static::lazy_static;

lazy_static! {
    static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    static ref CACHE_DIR: PathBuf = dirs::cache_dir()
        .expect("failed to determine cache directory")
        .join("Zed");
    pub static ref CONFIG_DIR: PathBuf = env::var_os("XDG_CONFIG_HOME")
        .map(|home| home.into())
        .unwrap_or_else(|| HOME.join(".config"))
        .join("zed");
    pub static ref LOGS_DIR: PathBuf = HOME.join("Library/Logs/Zed");
    pub static ref LANGUAGES_DIR: PathBuf = CACHE_DIR.join("languages");
    pub static ref DB_DIR: PathBuf = CACHE_DIR.join("db");
    pub static ref DB: PathBuf = DB_DIR.join("zed.db");
    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
}
