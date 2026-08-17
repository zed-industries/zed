// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{env, path::PathBuf};

/// Maps to platformdirs package in Python
pub struct Platformdirs {
    app_name: String,
    version: Option<String>,
    roaming: bool,
}

impl Platformdirs {
    pub fn new(app_name: String, roaming: bool) -> Self {
        Self {
            app_name,
            version: None,
            roaming,
        }
    }

    /// Maps to the user_cache_path function in platformdirs package (Python)
    pub fn user_cache_path(&self) -> Option<PathBuf> {
        self.user_cache_dir()
    }

    /// Maps to the user_cache_dir function in platformdirs package (Python)
    pub fn user_cache_dir(&self) -> Option<PathBuf> {
        if cfg!(windows) {
            env::var("CSIDL_LOCAL_APPDATA")
                .ok()
                .map(PathBuf::from)
                .or(env::var("USERPROFILE") // Fallback for CSIDL_LOCAL_APPDATA
                    .ok()
                    .map(|user| PathBuf::from(user).join("AppData").join("Local")))
                .map(|app_data| self.append_app_name_and_version(app_data, Some("Cache")))
        } else if std::env::consts::OS == "macos" {
            env::var("HOME").ok().map(PathBuf::from).map(|home| {
                self.append_app_name_and_version(home.join("Library").join("Caches"), None)
            })
        } else {
            let mut path = env::var("XDG_CACHE_HOME").ok().map(PathBuf::from);
            if path.is_none() {
                path = env::var("HOME")
                    .ok()
                    .map(PathBuf::from)
                    .map(|home| home.join(".cache"));
            }
            path.map(|path| self.append_app_name_and_version(path, None))
        }
    }

    /// Maps to the user_config_path function in platformdirs package (Python)
    pub fn user_config_path(&self) -> Option<PathBuf> {
        if std::env::consts::OS == "windows" || std::env::consts::OS == "macos" {
            self.user_data_dir()
        } else {
            let mut path = env::var("XDG_CONFIG_HOME").ok().map(PathBuf::from);
            if path.is_none() {
                path = env::var("HOME")
                    .ok()
                    .map(PathBuf::from)
                    .map(|home| home.join(".config"));
            }
            path.map(|path| self.append_app_name_and_version(path, None))
        }
    }
    /// Maps to the user_data_dir function in platformdirs package (Python)
    pub fn user_data_dir(&self) -> Option<PathBuf> {
        if std::env::consts::OS == "windows" {
            let app_data = if self.roaming {
                env::var("CSIDL_APPDATA").ok().map(PathBuf::from).or(
                    env::var("USERPROFILE") // Fallback for CSIDL_LOCAL_APPDATA
                        .ok()
                        .map(|user| PathBuf::from(user).join("AppData").join("Roaming")),
                )
            } else {
                env::var("CSIDL_LOCAL_APPDATA").ok().map(PathBuf::from).or(
                    env::var("USERPROFILE") // Fallback for CSIDL_LOCAL_APPDATA
                        .ok()
                        .map(|user| PathBuf::from(user).join("AppData").join("Local")),
                )
            };
            app_data.map(|app_data| self.append_app_name_and_version(app_data, None))
        } else if std::env::consts::OS == "macos" {
            env::var("HOME").ok().map(PathBuf::from).map(|home| {
                self.append_app_name_and_version(
                    home.join("Library").join("Application Support"),
                    None,
                )
            })
        } else {
            let mut path = env::var("XDG_DATA_HOME").ok().map(PathBuf::from);
            if path.is_none() {
                path = env::var("HOME")
                    .ok()
                    .map(PathBuf::from)
                    .map(|home| home.join(".local").join("share"));
            }
            path.map(|path| self.append_app_name_and_version(path, None))
        }
    }

    fn append_app_name_and_version(&self, path: PathBuf, suffix: Option<&str>) -> PathBuf {
        let mut path = path.join(&self.app_name);
        if let Some(version) = &self.version {
            path = path.join(version)
        }
        if let Some(suffix) = suffix {
            path.join(suffix)
        } else {
            path
        }
    }
}
