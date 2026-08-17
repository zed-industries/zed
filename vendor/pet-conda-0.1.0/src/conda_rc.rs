// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env_variables::EnvVariables;
use log::trace;
use pet_fs::path::expand_path;
use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};
use yaml_rust2::YamlLoader;

#[derive(Debug)]
pub struct Condarc {
    pub files: Vec<PathBuf>,
    pub env_dirs: Vec<PathBuf>,
}

impl Condarc {
    pub fn from(env_vars: &EnvVariables) -> Option<Condarc> {
        get_conda_conda_rc(env_vars)
    }
    pub fn from_path(path: &Path) -> Option<Condarc> {
        get_conda_conda_rc_from_path(&path.to_path_buf())
    }
}

// Search paths documented here
// https://conda.io/projects/conda/en/latest/user-guide/configuration/use-condarc.html#searching-for-condarc
// https://github.com/conda/conda/blob/3ae5d7cf6cbe2b0ff9532359456b7244ae1ea5ef/conda/base/constants.py#L28
pub fn get_conda_rc_search_paths(env_vars: &EnvVariables) -> Vec<PathBuf> {
    use crate::utils::change_root_of_path;

    let mut search_paths: Vec<PathBuf> = vec![];

    if std::env::consts::OS == "windows" {
        search_paths.append(
            &mut [
                "C:\\ProgramData\\conda\\.condarc",
                "C:\\ProgramData\\conda\\condarc",
                "C:\\ProgramData\\conda\\condarc.d",
                "C:\\ProgramData\\miniconda\\.condarc",
                "C:\\ProgramData\\miniconda\\condarc",
                "C:\\ProgramData\\miniconda\\condarc.d",
                "C:\\ProgramData\\miniconda3\\.condarc",
                "C:\\ProgramData\\miniconda3\\condarc",
                "C:\\ProgramData\\miniconda3\\condarc.d",
                "C:\\ProgramData\\conda\\.mambarc",
                format!(
                    "{}:\\ProgramData\\conda\\.condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\conda\\condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\conda\\condarc.d",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda\\.condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda\\condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda\\condarc.d",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda3\\.condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda3\\condarc",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
                format!(
                    "{}:\\ProgramData\\miniconda3\\condarc.d",
                    env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
                )
                .as_str(),
            ]
            .iter()
            .map(PathBuf::from)
            .collect(),
        );
    } else {
        search_paths.append(
            &mut [
                "/etc/conda/.condarc",
                "/etc/conda/condarc",
                "/etc/conda/condarc.d",
                "/etc/conda/mambarc",
                "/var/lib/conda/.condarc",
                "/var/lib/conda/condarc",
                "/var/lib/conda/condarc.d",
                "/var/lib/conda/.mambarc",
                "/etc/miniconda/.condarc",
                "/etc/miniconda/condarc",
                "/etc/miniconda/condarc.d",
                "/etc/miniconda/mambarc",
                "/var/lib/miniconda/.condarc",
                "/var/lib/miniconda/condarc",
                "/var/lib/miniconda/condarc.d",
                "/var/lib/miniconda/.mambarc",
                "/etc/miniconda3/.condarc",
                "/etc/miniconda3/condarc",
                "/etc/miniconda3/condarc.d",
                "/etc/miniconda3/mambarc",
                "/var/lib/miniconda3/.condarc",
                "/var/lib/miniconda3/condarc",
                "/var/lib/miniconda3/condarc.d",
                "/var/lib/miniconda3/.mambarc",
            ]
            .iter()
            .map(PathBuf::from)
            // This is done only for testing purposes, hacky, but we need some tests with different paths.
            .map(|p| change_root_of_path(&p, &env_vars.root))
            .collect(),
        );
    }
    if let Some(ref conda_root) = env_vars.conda_root {
        let conda_root = expand_path(PathBuf::from(conda_root.clone()));
        search_paths.append(&mut vec![
            conda_root.join(".condarc"),
            conda_root.join("condarc"),
            conda_root.join(".condarc.d"),
            conda_root.join(".mambarc"),
        ]);
    }
    if let Some(ref xdg_config_home) = env_vars.xdg_config_home {
        search_paths.append(&mut vec![
            PathBuf::from(xdg_config_home.clone()).join(".condarc"),
            PathBuf::from(xdg_config_home.clone()).join("condarc"),
            PathBuf::from(xdg_config_home.clone()).join(".condarc.d"),
            PathBuf::from(xdg_config_home.clone()).join(".mambarc"),
        ]);
    }
    if let Some(ref home) = env_vars.home {
        search_paths.append(&mut vec![
            home.join(".config").join("conda").join(".condarc"),
            home.join(".config").join("conda").join("condarc"),
            home.join(".config").join("conda").join("condarc.d"),
            home.join(".conda").join(".condarc"),
            home.join(".conda").join("condarc"),
            home.join(".conda").join("condarc.d"),
            home.join(".condarc"),
            home.join("condarc"),
            home.join("condarc.d"),
            home.join(".mambarc"),
        ]);
    }
    if let Some(ref conda_prefix) = env_vars.conda_prefix {
        let conda_prefix = expand_path(PathBuf::from(conda_prefix.clone()));
        search_paths.append(&mut vec![
            conda_prefix.join(".condarc"),
            conda_prefix.join("condarc"),
            conda_prefix.join(".condarc.d"),
            conda_prefix.join(".mamabarc"),
        ]);
    }
    if let Some(ref conda_dir) = env_vars.conda_dir {
        let conda_dir = expand_path(PathBuf::from(conda_dir.clone()));
        search_paths.append(&mut vec![
            conda_dir.join(".condarc"),
            conda_dir.join("condarc"),
            conda_dir.join(".condarc.d"),
        ]);
    }
    if let Some(ref condarc) = env_vars.condarc {
        search_paths.append(&mut vec![expand_path(PathBuf::from(condarc))]);
    }
    if let Some(ref mambarc) = env_vars.mambarc {
        search_paths.append(&mut vec![expand_path(PathBuf::from(mambarc))]);
    }

    let search_paths: HashSet<_> = search_paths.into_iter().collect();
    search_paths.into_iter().collect()
}

// https://github.com/conda/conda/blob/3ae5d7cf6cbe2b0ff9532359456b7244ae1ea5ef/conda/common/configuration.py#L1315
static POSSIBLE_CONDA_RC_FILES: &[&str] = &[".condarc", "condarc", ".condarc.d"];
static SUPPORTED_EXTENSIONS: &[&str] = &["yaml", "yml"];

/**
 * The .condarc file contains a list of directories where conda environments are created.
 * https://conda.io/projects/conda/en/latest/configuration.html#envs-dirs
 *
 * More info here
 * https://conda.io/projects/conda/en/latest/user-guide/configuration/use-condarc.html#searching-for-condarc
 * https://github.com/conda/conda/blob/3ae5d7cf6cbe2b0ff9532359456b7244ae1ea5ef/conda/base/constants.py#L28
 */
fn get_conda_conda_rc(env_vars: &EnvVariables) -> Option<Condarc> {
    let mut env_dirs = vec![];
    let mut files = vec![];
    for conda_rc in get_conda_rc_search_paths(env_vars).into_iter() {
        if let Some(ref mut cfg) = get_conda_conda_rc_from_path(&conda_rc) {
            env_dirs.append(&mut cfg.env_dirs);
            files.append(&mut cfg.files);
        }
    }

    if env_dirs.is_empty() && files.is_empty() {
        None
    } else {
        Some(Condarc { env_dirs, files })
    }
}

fn get_conda_conda_rc_from_path(conda_rc: &PathBuf) -> Option<Condarc> {
    let mut env_dirs = vec![];
    let mut files = vec![];
    if conda_rc.is_file() {
        if let Some(ref mut cfg) = parse_conda_rc(conda_rc) {
            env_dirs.append(&mut cfg.env_dirs);
            files.push(conda_rc.clone());
        }
    } else if conda_rc.is_dir() {
        // There can be different types of conda rc files in the directory.
        // .condarc, condarc, .condarc.yml, condarc.yaml, etc.
        // https://github.com/conda/conda/blob/3ae5d7cf6cbe2b0ff9532359456b7244ae1ea5ef/conda/common/configuration.py#L1315
        // https://conda.io/projects/conda/en/latest/user-guide/configuration/use-condarc.html
        if let Ok(reader) = fs::read_dir(conda_rc) {
            for path in reader
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_file())
            {
                let file_name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap_or_default()
                    .to_lowercase();
                let extension = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_lowercase();

                if POSSIBLE_CONDA_RC_FILES.contains(&file_name.as_str())
                    || SUPPORTED_EXTENSIONS.contains(&extension.as_str())
                    || file_name.contains("condarc")
                {
                    if let Some(ref mut cfg) = parse_conda_rc(&path) {
                        env_dirs.append(&mut cfg.env_dirs);
                        files.push(path);
                    }
                }
            }
        }
    }

    if env_dirs.is_empty() && files.is_empty() {
        None
    } else {
        trace!("conda_rc: {:?} with env_dirs {:?}", conda_rc, env_dirs);
        Some(Condarc { env_dirs, files })
    }
}

fn parse_conda_rc(conda_rc: &Path) -> Option<Condarc> {
    let reader = fs::read_to_string(conda_rc).ok()?;
    if let Some(cfg) = parse_conda_rc_contents(&reader) {
        trace!("conda_rc: {:?} with env_dirs {:?}", conda_rc, cfg.env_dirs);
        Some(Condarc {
            env_dirs: cfg.env_dirs,
            files: vec![conda_rc.to_path_buf()],
        })
    } else {
        trace!("Failed to parse or empty conda_rc: {:?}", conda_rc);
        Some(Condarc {
            env_dirs: vec![],
            files: vec![conda_rc.to_path_buf()],
        })
    }
}

fn parse_conda_rc_contents(contents: &str) -> Option<Condarc> {
    let mut env_dirs = vec![];

    if let Ok(docs) = YamlLoader::load_from_str(contents) {
        if docs.is_empty() {
            return Some(Condarc {
                env_dirs: vec![],
                files: vec![],
            });
        }
        let doc = &docs[0];
        // Expland variables in some of these
        // https://docs.conda.io/projects/conda/en/4.13.x/user-guide/configuration/use-condarc.html#expansion-of-environment-variables

        if let Some(items) = doc["envs_dirs"].as_vec() {
            for item in items {
                let item_str = item.as_str().unwrap().to_string();
                if item_str.is_empty() {
                    continue;
                }
                let env_dir = expand_path(PathBuf::from(item_str.trim()));
                trace!("env_dir: {:?} parsed as {:?}", item_str.trim(), env_dir);
                env_dirs.push(env_dir);
            }
        }
        if let Some(items) = doc["envs_path"].as_vec() {
            for item in items {
                let item_str = item.as_str().unwrap().to_string();
                if item_str.is_empty() {
                    continue;
                }
                let env_dir = expand_path(PathBuf::from(item_str.trim()));
                trace!("env_path: {:?} parsed as {:?}", item_str.trim(), env_dir);
                env_dirs.push(env_dir);
            }
        }
    }
    Some(Condarc {
        env_dirs,
        files: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_conda_rc() {
        let cfg = r#"
channels:
  - conda-forge
  - nodefaults
channel_priority: strict
envs_dirs:
  - /Users/username/dev/envs # Testing 1,2,3
  - /opt/conda/envs
envs_path:
  - /opt/somep lace/envs
  - ~/dev/envs2 # Testing 1,2,3
"#;

        assert_eq!(
            parse_conda_rc_contents(cfg).unwrap().env_dirs,
            [
                PathBuf::from("/Users/username/dev/envs"),
                PathBuf::from("/opt/conda/envs"),
                PathBuf::from("/opt/somep lace/envs"),
                expand_path(PathBuf::from("~/dev/envs2"))
            ]
        );

        let cfg = r#"
channels:
  - conda-forge
  - nodefaults
channel_priority: strict
envs_dirs:
  - /Users/username/dev/envs # Testing 1,2,3
  - /opt/conda/envs
"#;

        assert_eq!(
            parse_conda_rc_contents(cfg).unwrap().env_dirs,
            ["/Users/username/dev/envs", "/opt/conda/envs",].map(PathBuf::from)
        );

        let cfg = r#"
channels:
  - conda-forge
  - nodefaults
channel_priority: strict
envs_path:
  - /opt/somep lace/envs
  - ~/dev/envs2 # Testing 1,2,3
"#;

        assert_eq!(
            parse_conda_rc_contents(cfg).unwrap().env_dirs,
            [
                PathBuf::from("/opt/somep lace/envs"),
                expand_path(PathBuf::from("~/dev/envs2"))
            ]
        );

        let cfg = r#"
channels:
  - conda-forge
  - nodefaults
channel_priority: strict
"#;

        assert!(parse_conda_rc_contents(cfg).unwrap().env_dirs.is_empty(),);
        assert!(parse_conda_rc_contents(cfg).unwrap().files.is_empty(),);
    }
}
