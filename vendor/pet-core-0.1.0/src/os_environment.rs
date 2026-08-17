// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use log::trace;
use pet_fs::path::norm_case;

pub trait Environment: Send + Sync {
    fn get_user_home(&self) -> Option<PathBuf>;
    /// Only used in tests, None in production.
    #[allow(dead_code)]
    fn get_root(&self) -> Option<PathBuf>;
    fn get_env_var(&self, key: String) -> Option<String>;
    fn get_know_global_search_locations(&self) -> Vec<PathBuf>;
}

pub struct EnvironmentApi {
    global_search_locations: Arc<Mutex<Vec<PathBuf>>>,
}
impl EnvironmentApi {
    pub fn new() -> Self {
        EnvironmentApi {
            global_search_locations: Arc::new(Mutex::new(vec![])),
        }
    }
}
impl Default for EnvironmentApi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl Environment for EnvironmentApi {
    fn get_user_home(&self) -> Option<PathBuf> {
        get_user_home()
    }
    fn get_root(&self) -> Option<PathBuf> {
        None
    }
    fn get_env_var(&self, key: String) -> Option<String> {
        get_env_var(key)
    }
    fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
        if self
            .global_search_locations
            .lock()
            .expect("global_search_locations mutex poisoned")
            .is_empty()
        {
            let mut paths =
                env::split_paths(&self.get_env_var("PATH".to_string()).unwrap_or_default())
                    .filter(|p| p.exists())
                    .collect::<Vec<PathBuf>>();
            trace!("Env PATH: {:?}", paths);
            self.global_search_locations
                .lock()
                .expect("global_search_locations mutex poisoned")
                .append(&mut paths);
        }
        self.global_search_locations
            .lock()
            .expect("global_search_locations mutex poisoned")
            .clone()
    }
}

#[cfg(unix)]
impl Environment for EnvironmentApi {
    fn get_user_home(&self) -> Option<PathBuf> {
        get_user_home()
    }
    fn get_root(&self) -> Option<PathBuf> {
        None
    }
    fn get_env_var(&self, key: String) -> Option<String> {
        get_env_var(key)
    }
    fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
        if self
            .global_search_locations
            .lock()
            .expect("global_search_locations mutex poisoned")
            .is_empty()
        {
            let mut paths =
                env::split_paths(&self.get_env_var("PATH".to_string()).unwrap_or_default())
                    .collect::<Vec<PathBuf>>();
            trace!("Env PATH: {:?}", paths);
            vec![
                PathBuf::from("/bin"),
                PathBuf::from("/etc"),
                PathBuf::from("/lib"),
                PathBuf::from("/lib/x86_64-linux-gnu"),
                PathBuf::from("/lib64"),
                PathBuf::from("/sbin"),
                PathBuf::from("/snap/bin"),
                PathBuf::from("/usr/bin"),
                PathBuf::from("/usr/games"),
                PathBuf::from("/usr/include"),
                PathBuf::from("/usr/lib"),
                PathBuf::from("/usr/lib/x86_64-linux-gnu"),
                PathBuf::from("/usr/lib64"),
                PathBuf::from("/usr/libexec"),
                PathBuf::from("/usr/local"),
                PathBuf::from("/usr/local/bin"),
                PathBuf::from("/usr/local/etc"),
                PathBuf::from("/usr/local/games"),
                PathBuf::from("/usr/local/lib"),
                PathBuf::from("/usr/local/sbin"),
                PathBuf::from("/usr/sbin"),
                PathBuf::from("/usr/share"),
                PathBuf::from("/home/bin"),
                PathBuf::from("/home/sbin"),
                PathBuf::from("/opt"),
                PathBuf::from("/opt/bin"),
                PathBuf::from("/opt/sbin"),
            ]
            .iter()
            .for_each(|p| {
                if !paths.contains(p) {
                    paths.push(p.clone());
                }
            });

            if let Some(home) = self.get_user_home() {
                paths.push(home.join(".local").join("bin"));
            }

            let mut paths = paths
                .into_iter()
                .filter(|p| p.exists())
                .collect::<Vec<PathBuf>>();

            self.global_search_locations
                .lock()
                .expect("global_search_locations mutex poisoned")
                .append(&mut paths);
        }
        self.global_search_locations
            .lock()
            .expect("global_search_locations mutex poisoned")
            .clone()
    }
}

#[cfg(windows)]
fn get_user_home() -> Option<PathBuf> {
    let home = env::var("USERPROFILE").or_else(|_| env::var("HOME"));
    match home {
        Ok(home) => Some(norm_case(PathBuf::from(home))),
        Err(_) => None,
    }
}

#[cfg(unix)]
fn get_user_home() -> Option<PathBuf> {
    let home = env::var("HOME");
    match home {
        Ok(home) => Some(norm_case(PathBuf::from(home))),
        Err(_) => None,
    }
}

fn get_env_var(key: String) -> Option<String> {
    env::var(key).ok()
}
