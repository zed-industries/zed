// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    fs,
    path::{Path, PathBuf},
};

use log::{error, trace};
use pet_python_utils::platform_dirs::Platformdirs;

use crate::env_variables::EnvVariables;

static _APP_NAME: &str = "pypoetry";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub virtualenvs_in_project: Option<bool>,
    pub virtualenvs_path: PathBuf,
    pub cache_dir: Option<PathBuf>,
    pub file: Option<PathBuf>,
}

impl Config {
    fn new(
        file: Option<PathBuf>,
        virtualenvs_path: PathBuf,
        cache_dir: Option<PathBuf>,
        virtualenvs_in_project: Option<bool>,
    ) -> Self {
        trace!(
            "Poetry config file => {:?}, virtualenv.path => {:?}, cache_dir => {:?}, virtualenvs_in_project => {:?}",
            file,
            virtualenvs_path,
            cache_dir,
            virtualenvs_in_project
        );
        Config {
            file,
            virtualenvs_path,
            cache_dir,
            virtualenvs_in_project,
        }
    }
    pub fn find_global(env: &EnvVariables) -> Option<Self> {
        let file = find_config_file(env);
        create_config(file, env)
    }
    pub fn find_local(path: &Path, env: &EnvVariables) -> Option<Self> {
        let file = path.join("poetry.toml");
        if file.is_file() {
            create_config(Some(file), env)
        } else {
            None
        }
    }
}

fn create_config(file: Option<PathBuf>, env: &EnvVariables) -> Option<Config> {
    if let Some(file) = &file {
        trace!("Parsing Poetry config file => {:?}", file);
    }

    let cfg = file.clone().and_then(|f| parse(&f));
    let cache_dir = get_cache_dir(&cfg, env);
    let virtualenvs_path_from_env_var = env
        .poetry_virtualenvs_path
        .clone()
        .map(|p| resolve_virtualenvs_path(&p, &cache_dir));

    if let Some(virtualenvs_path) = &cfg.clone().and_then(|cfg| cfg.virtualenvs_path) {
        let virtualenvs_path = resolve_virtualenvs_path(&virtualenvs_path.clone(), &cache_dir);

        return Some(Config::new(
            file.clone(),
            // Give preference to the virtualenvs path from the env var
            virtualenvs_path_from_env_var.unwrap_or(virtualenvs_path.clone()),
            cache_dir,
            cfg.and_then(|cfg| cfg.virtualenvs_in_project),
        ));
    }

    // Give preference to the virtualenvs path from the env var
    if let Some(virtualenvs_path_from_env_var) = virtualenvs_path_from_env_var {
        if virtualenvs_path_from_env_var.exists() {
            return Some(Config::new(
                file,
                virtualenvs_path_from_env_var,
                cache_dir,
                cfg.and_then(|cfg| cfg.virtualenvs_in_project),
            ));
        }
    }

    cache_dir
        .map(|cache_dir| Config::new(file, cache_dir.join("virtualenvs"), Some(cache_dir), None))
}

/// Replaces {cache-dir} in virtualenvs path with the cache dir
fn resolve_virtualenvs_path(virtualenvs_path: &Path, cache_dir: &Option<PathBuf>) -> PathBuf {
    if virtualenvs_path
        .to_string_lossy()
        .to_lowercase()
        .contains("{cache-dir}")
    {
        if let Some(cache_dir) = &cache_dir {
            let virtualenvs_path = PathBuf::from(
                virtualenvs_path
                    .to_string_lossy()
                    .replace("{cache-dir}", cache_dir.to_string_lossy().as_ref()),
            );
            trace!(
                "Poetry virtualenvs path after replacing cache-dir => {:?}",
                virtualenvs_path
            );
            return virtualenvs_path;
        }
    }
    trace!("Poetry virtualenvs path => {:?}", virtualenvs_path);
    virtualenvs_path.to_path_buf()
}
/// Maps to DEFAULT_CACHE_DIR in poetry
fn get_cache_dir(cfg: &Option<ConfigToml>, env: &EnvVariables) -> Option<PathBuf> {
    // Cache dir in env variables takes precedence
    if let Some(cache_dir) = env.poetry_cache_dir.clone() {
        if cache_dir.is_dir() {
            trace!("Poetry cache dir from env variable: {:?}", cache_dir);
            return Some(cache_dir);
        }
    }
    // Check cache dir in config.
    if let Some(cache_dir) = cfg.as_ref().and_then(|cfg| cfg.cache_dir.clone()) {
        if cache_dir.is_dir() {
            trace!("Poetry cache dir from config: {:?}", cache_dir);
            return Some(cache_dir);
        }
    }

    let default_cache_dir = Platformdirs::new(_APP_NAME.into(), false).user_cache_path();
    trace!("Poetry cache (default): {:?}", default_cache_dir);
    default_cache_dir
}

/// Maps to CONFIG_DIR in poetry
fn get_config_dir(env: &EnvVariables) -> Option<PathBuf> {
    if let Some(config) = env.poetry_config_dir.clone() {
        // Ensure we have a valid directory setup in the env variables.
        if config.is_dir() {
            return Some(config);
        }
    }
    Platformdirs::new(_APP_NAME.into(), true).user_config_path()
}

pub fn find_config_file(env: &EnvVariables) -> Option<PathBuf> {
    let config_dir = get_config_dir(env)?;
    let file = config_dir.join("config.toml");
    if file.exists() {
        Some(file)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfigToml {
    virtualenvs_in_project: Option<bool>,
    cache_dir: Option<PathBuf>,
    virtualenvs_path: Option<PathBuf>,
}

fn parse(file: &Path) -> Option<ConfigToml> {
    let contents = fs::read_to_string(file).ok()?;
    let cfg = parse_contents(&contents);
    trace!("Poetry config file for {:?} is {:?}", file, cfg);
    cfg
}

fn parse_contents(contents: &str) -> Option<ConfigToml> {
    let mut virtualenvs_path = None;
    let mut cache_dir = None;
    let mut virtualenvs_in_project = None;
    match toml::from_str::<toml::Value>(contents) {
        Ok(value) => {
            if let Some(virtualenvs) = value.get("virtualenvs") {
                if let Some(path) = virtualenvs.get("path") {
                    // Can contain invalid toml, hence make no assumptions
                    // virtualenvs.in-project = null
                    // https://github.com/python-poetry/poetry/blob/5bab98c9500f1050c6bb6adfb55580a23173f18d/docs/configuration.md#L56
                    if path.is_str() {
                        virtualenvs_path = path.as_str().map(|s| s.trim()).map(PathBuf::from);
                    }
                }
                if let Some(in_project) = virtualenvs.get("in-project") {
                    // Can contain invalid toml, hence make no assumptions
                    // virtualenvs.in-project = null
                    // https://github.com/python-poetry/poetry/blob/5bab98c9500f1050c6bb6adfb55580a23173f18d/docs/configuration.md#L56
                    if in_project.is_bool() {
                        virtualenvs_in_project = in_project.as_bool();
                    }
                }
            }
            if let Some(value) = value.get("cache-dir") {
                // Can contain invalid toml, hence make no assumptions
                // virtualenvs.in-project = null
                // https://github.com/python-poetry/poetry/blob/5bab98c9500f1050c6bb6adfb55580a23173f18d/docs/configuration.md#L56
                if value.is_str() {
                    cache_dir = value.as_str().map(|s| s.trim()).map(PathBuf::from);

                    if let Some(cache_dir) = &cache_dir {
                        if virtualenvs_path.is_none() {
                            virtualenvs_path = Some(cache_dir.join("virtualenvs"));
                        }
                    }
                }
            }

            Some(ConfigToml {
                virtualenvs_in_project,
                virtualenvs_path,
                cache_dir,
            })
        }
        Err(e) => {
            error!("Error parsing poetry toml file: {:?}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_virtualenvs_in_poetry_toml() {
        let cfg = r#"
[virtualenvs]
in-project = false
create = false

"#;

        assert!(!parse_contents(cfg)
            .unwrap()
            .virtualenvs_in_project
            .unwrap_or_default());

        let cfg = r#"
[virtualenvs]
in-project = true
create = false

"#;
        assert!(parse_contents(cfg)
            .unwrap()
            .virtualenvs_in_project
            .unwrap_or_default());

        let cfg = r#"
[virtualenvs]
create = false

"#;
        assert!(!parse_contents(cfg)
            .unwrap()
            .virtualenvs_in_project
            .unwrap_or_default());

        let cfg = r#"
virtualenvs.in-project = true # comment
"#;
        assert!(parse_contents(cfg)
            .unwrap()
            .virtualenvs_in_project
            .unwrap_or_default());

        let cfg = r#"
"#;
        assert!(!parse_contents(cfg)
            .unwrap()
            .virtualenvs_in_project
            .unwrap_or_default());
    }

    #[test]
    fn parse_cache_dir_in_poetry_toml() {
        let cfg = r#"
cache-dir = "/path/to/cache/directory"

"#;
        assert_eq!(
            parse_contents(cfg).unwrap().cache_dir,
            Some(PathBuf::from("/path/to/cache/directory".to_string()))
        );

        let cfg = r#"
some-other-value = 1234

"#;
        assert_eq!(parse_contents(cfg).unwrap().cache_dir, None);
    }

    #[test]
    fn parse_virtualenvs_path_in_poetry_toml() {
        let cfg = r#"
virtualenvs.path = "/path/to/virtualenvs"

"#;
        assert_eq!(
            parse_contents(cfg).unwrap().virtualenvs_path,
            Some(PathBuf::from("/path/to/virtualenvs".to_string()))
        );

        let cfg = r#"
some-other-value = 1234

"#;
        assert_eq!(parse_contents(cfg).unwrap().virtualenvs_path, None);
    }

    #[test]
    fn use_cache_dir_to_build_virtualenvs_path() {
        let cfg = r#"
cache-dir = "/path/to/cache/directory"
"#;
        assert_eq!(
            parse_contents(cfg).unwrap().virtualenvs_path,
            Some(PathBuf::from("/path/to/cache/directory/virtualenvs"))
        );
    }
}
