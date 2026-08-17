// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    conda_rc::{get_conda_rc_search_paths, Condarc},
    env_variables::EnvVariables,
    manager::{find_conda_binary, find_mamba_binary},
    utils::{is_conda_env, is_conda_install},
};
use log::trace;
use pet_fs::path::{expand_path, norm_case};
use pet_python_utils::platform_dirs::Platformdirs;
use rayon::prelude::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
    thread,
    time::SystemTime,
};

const APP_NAME: &str = "conda";

/// Checks whether the current environment is Windows Subsystem for Linux.
/// Result is cached after the first call.
#[cfg(unix)]
fn is_wsl() -> bool {
    use std::sync::OnceLock;
    static IS_WSL: OnceLock<bool> = OnceLock::new();
    *IS_WSL.get_or_init(|| {
        std::fs::read_to_string("/proc/version")
            .map(|v| v.to_lowercase().contains("microsoft"))
            .unwrap_or(false)
    })
}

/// Returns true if the path is a Windows drive mounted via WSL (e.g. `/mnt/c/...`).
/// These paths contain Windows PE executables that cannot be run from Linux.
#[cfg(unix)]
fn is_wsl_windows_drive_path(path: &Path) -> bool {
    if !is_wsl() {
        return false;
    }
    is_windows_drive_mount(path)
}

/// Checks if a path matches the WSL Windows drive mount pattern `/mnt/<drive_letter>/`.
#[cfg(unix)]
fn is_windows_drive_mount(path: &Path) -> bool {
    if let Some(path_str) = path.to_str() {
        if let Some(rest) = path_str.strip_prefix("/mnt/") {
            let mut chars = rest.chars();
            if let Some(drive_letter) = chars.next() {
                if drive_letter.is_ascii_alphabetic() {
                    match chars.next() {
                        None | Some('/') => return true,
                        _ => {}
                    }
                }
            }
        }
    }
    false
}

pub fn get_conda_environment_paths(
    env_vars: &EnvVariables,
    conda_executable: &Option<PathBuf>,
) -> Vec<PathBuf> {
    let start = SystemTime::now();
    let mut env_paths = thread::scope(|s| {
        let mut envs = vec![];
        for thread in [
            s.spawn(|| get_conda_envs_from_environment_txt(env_vars)),
            s.spawn(|| get_conda_environment_paths_from_conda_rc(env_vars)),
            s.spawn(|| get_conda_environment_paths_from_known_paths(env_vars)),
            s.spawn(|| get_known_conda_install_locations(env_vars, conda_executable)),
        ] {
            if let Ok(mut env_paths) = thread.join() {
                envs.append(&mut env_paths);
            }
        }
        envs
    });

    env_paths = env_paths.iter().map(norm_case).collect();
    env_paths.sort();
    env_paths.dedup();

    // On WSL, filter out paths on Windows drive mounts (/mnt/<drive>/) since they
    // contain Windows PE executables that cannot be run from Linux.
    #[cfg(unix)]
    {
        env_paths.retain(|p| {
            if is_wsl_windows_drive_path(p) {
                trace!(
                    "Skipping conda path on Windows drive (not usable from WSL): {:?}",
                    p
                );
                false
            } else {
                true
            }
        });
    }

    // For each env, check if we have a conda install directory in them and
    // & then iterate through the list of envs in the envs directory.
    let mut result: Vec<PathBuf> = env_paths
        .par_iter()
        .filter(|f| f.exists())
        .flat_map(|path| get_environments(path))
        .collect();

    result.sort();
    result.dedup();
    trace!(
        "Time taken to get conda environment paths: {:?}",
        start.elapsed().unwrap()
    );
    result
}

/**
 * Get the list of conda environments found in conda rc files
 * as well as the directories where conda rc files can be found.
 */
fn get_conda_environment_paths_from_conda_rc(env_vars: &EnvVariables) -> Vec<PathBuf> {
    // Use the conda rc directories as well.
    let mut env_dirs = vec![];
    for rc_file_dir in get_conda_rc_search_paths(env_vars) {
        if !rc_file_dir.exists() {
            trace!(".condarc not found ({:?})", rc_file_dir);
            continue;
        }

        if let Some(conda_rc) = Condarc::from_path(&rc_file_dir) {
            trace!(
                "Conda environments in .condarc {:?} {:?}",
                conda_rc.files,
                conda_rc.env_dirs
            );
            env_dirs.append(
                &mut conda_rc
                    .env_dirs
                    .clone()
                    .into_iter()
                    .filter(|f| f.exists())
                    .collect(),
            );
        }

        if rc_file_dir.is_dir() {
            env_dirs.push(rc_file_dir);
        } else if rc_file_dir.is_file() {
            if let Some(dir) = rc_file_dir.parent() {
                env_dirs.push(dir.to_path_buf());
            }
        }
    }

    if let Some(conda_rc) = Condarc::from(env_vars) {
        trace!(
            "Conda environments in .condarc {:?} {:?}",
            conda_rc.files,
            conda_rc.env_dirs
        );
        env_dirs.append(&mut conda_rc.env_dirs.clone());
    } else {
        trace!("No Conda environments in .condarc {:?}", env_dirs);
    }
    env_dirs
}

fn get_conda_environment_paths_from_known_paths(env_vars: &EnvVariables) -> Vec<PathBuf> {
    let mut env_paths: Vec<PathBuf> = vec![];
    if let Some(ref home) = env_vars.home {
        let mut known_conda_paths = vec![
            PathBuf::from(".conda/envs"),
            PathBuf::from("/opt/conda/envs"),
            PathBuf::from("C:/Anaconda/envs"),
            PathBuf::from("AppData/Local/conda/envs"),
            PathBuf::from("AppData/Local/conda/conda/envs"),
            // https://docs.conda.io/projects/conda/en/22.11.x/user-guide/configuration/use-condarc.html
            PathBuf::from("envs"),
            PathBuf::from("my-envs"),
        ]
        .into_iter()
        .map(|p| home.join(p))
        .collect::<Vec<PathBuf>>();

        // https://github.com/conda/conda/blob/d88fc157818cd5542029e116dcf4ec427512be82/conda/base/context.py#L143
        if let Some(user_data_dir) = Platformdirs::new(APP_NAME.into(), false).user_data_dir() {
            known_conda_paths.push(user_data_dir.join("envs"));
        }

        // Expland variables in some of these
        // https://docs.conda.io/projects/conda/en/4.13.x/user-guide/configuration/use-condarc.html#expansion-of-environment-variables
        if let Some(conda_envs_path) = &env_vars.conda_envs_path {
            for path in env::split_paths(&conda_envs_path) {
                known_conda_paths.push(expand_path(path));
            }
        }
        // https://anaconda-project.readthedocs.io/en/latest/config.html
        if let Some(conda_envs_path) = &env_vars.anaconda_project_envs_path {
            for path in env::split_paths(&conda_envs_path) {
                known_conda_paths.push(expand_path(path));
            }
        }
        // https://anaconda-project.readthedocs.io/en/latest/config.html
        if let Some(project_dir) = &env_vars.project_dir {
            known_conda_paths.push(expand_path(PathBuf::from(project_dir)));
        }

        for path in known_conda_paths {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();
                    if path.is_dir() {
                        env_paths.push(path);
                    }
                }
            }
        }
    }
    env_paths.append(&mut env_vars.known_global_search_locations.clone());
    env_paths.sort();
    env_paths.dedup();
    let env_paths = env_paths.into_iter().filter(|f| f.exists()).collect();
    trace!("Conda environments in known paths {:?}", env_paths);
    env_paths
}

pub fn get_environments(conda_dir: &Path) -> Vec<PathBuf> {
    let mut envs: Vec<PathBuf> = vec![];

    if is_conda_install(conda_dir) {
        envs.push(conda_dir.to_path_buf());

        if let Ok(entries) = fs::read_dir(conda_dir.join("envs")) {
            envs.append(
                &mut entries
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|p| is_conda_env(p))
                    .collect(),
            );
        }
        // Then read the .condarc in the conda install folder as well.
        if let Some(mut conda_rc) = Condarc::from_path(conda_dir) {
            envs.append(&mut conda_rc.env_dirs);
        }
    } else if is_conda_env(conda_dir) {
        envs.push(conda_dir.to_path_buf());
        // If this is a conda environment under an `envs` folder, check if the grandparent
        // is the conda install directory (base env) and include it as well.
        // E.g. if conda_dir is `/opt/homebrew/Caskroom/miniforge/base/envs/test`,
        // then the grandparent `/opt/homebrew/Caskroom/miniforge/base` is the base env.
        // This ensures the base conda environment is discovered when only child envs are
        // listed in environments.txt (see https://github.com/microsoft/python-environment-tools/issues/236)
        if let Some(parent) = conda_dir.parent() {
            if parent.file_name().map(|n| n == "envs").unwrap_or(false) {
                if let Some(grandparent) = parent.parent() {
                    if is_conda_install(grandparent) && !envs.contains(&grandparent.to_path_buf()) {
                        // Recursively get environments from the conda install directory
                        // This will add the base env and any other sibling envs
                        envs.append(&mut get_environments(grandparent));
                    }
                }
            }
        }
    } else if conda_dir.join("envs").exists() {
        // This could be a directory where conda environments are stored.
        // I.e. its not necessarily the root conda install directory.
        // E.g. C:\Users\donjayamanne\.conda
        if let Ok(entries) = fs::read_dir(conda_dir.join("envs")) {
            envs.append(
                &mut entries
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|p| is_conda_env(p))
                    .collect(),
            );
        }
    } else {
        // The dir could already be the `envs` directory.
        if let Ok(entries) = fs::read_dir(conda_dir) {
            envs.append(
                &mut entries
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|p| is_conda_env(p))
                    .collect(),
            );
        }
    }

    envs.sort();
    envs.dedup();
    envs
}

pub fn get_conda_envs_from_environment_txt(env_vars: &EnvVariables) -> Vec<PathBuf> {
    let mut envs: Vec<PathBuf> = vec![];
    if let Some(ref home) = env_vars.home {
        let home = Path::new(&home);
        let environment_txt = home.join(".conda").join("environments.txt");
        if let Ok(reader) = fs::read_to_string(environment_txt.clone()) {
            trace!("Found environments.txt file {:?}", environment_txt);
            for line in reader.lines() {
                let line = norm_case(PathBuf::from(line.to_string()));
                trace!("Conda env in environments.txt file {:?}", line);
                if line.exists() {
                    envs.push(line);
                }
            }
        }
    }

    envs
}

#[cfg(windows)]
pub fn get_known_conda_install_locations(
    env_vars: &EnvVariables,
    conda_executable: &Option<PathBuf>,
) -> Vec<PathBuf> {
    use pet_fs::path::norm_case;

    // First, try to find conda from PATH - this handles conda installations on mapped drives
    // and other non-standard locations that aren't in the hardcoded search paths.
    let conda_from_path = if conda_executable.is_none() {
        find_conda_binary(env_vars)
    } else {
        None
    };

    let user_profile = env_vars.userprofile.clone().unwrap_or_default();
    let program_data = env_vars.programdata.clone().unwrap_or_default();
    let all_user_profile = env_vars.allusersprofile.clone().unwrap_or_default();
    let mut home_drive = env_vars.homedrive.clone().unwrap_or_default();
    let mut known_paths = vec![];
    for env_variable in &[program_data, all_user_profile, user_profile] {
        if !env_variable.is_empty() {
            known_paths.push(Path::new(&env_variable).join("anaconda3"));
            known_paths.push(Path::new(&env_variable).join("miniconda3"));
            known_paths.push(Path::new(&env_variable).join("miniforge3"));
            known_paths.push(Path::new(&env_variable).join("micromamba"));
        }
    }
    if !home_drive.is_empty() {
        if home_drive.ends_with(':') {
            home_drive = format!("{}\\", home_drive);
        }
        known_paths.push(Path::new(&home_drive).join("anaconda3"));
        known_paths.push(Path::new(&home_drive).join("miniconda"));
        known_paths.push(Path::new(&home_drive).join("miniforge3"));
        known_paths.push(Path::new(&home_drive).join("micromamba"));
    }
    if let Some(ref conda_root) = env_vars.conda_root {
        known_paths.push(expand_path(PathBuf::from(conda_root.clone())));
    }
    if let Some(ref conda_prefix) = env_vars.conda_prefix {
        known_paths.push(expand_path(PathBuf::from(conda_prefix.clone())));
    }
    if let Some(ref conda_dir) = env_vars.conda_dir {
        known_paths.push(expand_path(PathBuf::from(conda_dir.clone())));
    }
    if let Some(ref conda) = env_vars.conda {
        known_paths.push(expand_path(PathBuf::from(conda)));
    }
    let app_data = PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_default());
    if let Some(home) = env_vars.clone().home {
        for prefix in [
            home.clone(),
            // https://stackoverflow.com/questions/35709497/anaconda-python-where-are-the-virtual-environments-stored
            home.join(".conda"),
            // https://stackoverflow.com/questions/35709497/anaconda-python-where-are-the-virtual-environments-stored
            home.join(".local"),
            // https://stackoverflow.com/questions/35709497/anaconda-python-where-are-the-virtual-environments-stored
            PathBuf::from("C:\\ProgramData"),
            PathBuf::from(format!(
                "{}:\\ProgramData",
                env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
            )),
            // https://docs.conda.io/projects/conda/en/latest/user-guide/concepts/environments.html
            PathBuf::from("C:\\"),
            PathBuf::from(format!(
                "{}:\\",
                env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
            )),
            // https://community.anaconda.cloud/t/conda-update-anaconda/43656/7
            app_data.clone(),
        ] {
            known_paths.push(prefix.clone().join("anaconda"));
            known_paths.push(prefix.clone().join("anaconda3"));
            known_paths.push(prefix.clone().join("miniconda"));
            known_paths.push(prefix.clone().join("miniconda3"));
            known_paths.push(prefix.clone().join("miniforge3"));
            known_paths.push(prefix.clone().join("micromamba"));
        }
        // From ./conda/base/constants.py (conda repo)
        known_paths.push(PathBuf::from("C:\\ProgramData\\conda\\conda"));
        known_paths.push(PathBuf::from(format!(
            "{}:\\ProgramData\\conda\\conda",
            env::var("SYSTEMDRIVE").unwrap_or("C".to_string())
        )));
        // E.g. C:\Users\user name\.conda where we have `envs`` under this directory.
        known_paths.push(home.join(".conda"));
        known_paths.push(home.join(".local"));
        // E.g. C:\Users\user name\AppData\Local\conda\conda\envs
        known_paths.push(app_data.join("conda").join("conda"));
        known_paths.push(
            home.join("AppData")
                .join("Local")
                .join("conda")
                .join("conda"),
        );
    }
    known_paths.sort();
    known_paths.dedup();
    // Ensure the casing of the paths are correct.
    // Its possible the actual path is in a different case.
    // E.g. instead of C:\username\miniconda it might bt C:\username\Miniconda
    // We use lower cases above, but it could be in any case on disc.
    // We do not want to have duplicates in different cases.
    // & we'd like to preserve the case of the original path as on disc.
    known_paths = known_paths.iter().map(norm_case).collect();
    if let Some(conda_dir) = get_conda_dir_from_exe(conda_executable) {
        known_paths.push(conda_dir);
    }
    // Add conda installation found from PATH (handles mapped drives and non-standard locations)
    if let Some(conda_dir) = get_conda_dir_from_exe(&conda_from_path) {
        known_paths.push(conda_dir);
    }
    // Also check for mamba/micromamba on PATH to discover its install directory
    if let Some(mamba_dir) = get_conda_dir_from_exe(&find_mamba_binary(env_vars)) {
        known_paths.push(mamba_dir);
    }
    known_paths.sort();
    known_paths.dedup();

    known_paths
}

#[cfg(unix)]
pub fn get_known_conda_install_locations(
    env_vars: &EnvVariables,
    conda_executable: &Option<PathBuf>,
) -> Vec<PathBuf> {
    // First, try to find conda from PATH - this handles conda installations in
    // non-standard locations that aren't in the hardcoded search paths.
    let conda_from_path = if conda_executable.is_none() {
        find_conda_binary(env_vars)
    } else {
        None
    };

    let mut known_paths = vec![
        // We need to look in `/anaconda3` and `/miniconda3` as well.
        PathBuf::from("/anaconda"),
        PathBuf::from("/anaconda3"),
        PathBuf::from("/miniconda"),
        PathBuf::from("/miniconda3"),
        PathBuf::from("/miniforge"),
        PathBuf::from("/miniforge3"),
        PathBuf::from("/micromamba"),
    ];
    if let Some(ref conda_root) = env_vars.conda_root {
        known_paths.push(expand_path(PathBuf::from(conda_root.clone())));
    }
    if let Some(ref conda_prefix) = env_vars.conda_prefix {
        known_paths.push(expand_path(PathBuf::from(conda_prefix.clone())));
    }
    if let Some(ref mamba_root_prefix) = env_vars.mamba_root_prefix {
        known_paths.push(expand_path(PathBuf::from(mamba_root_prefix.clone())));
    }
    if let Some(ref conda_dir) = env_vars.conda_dir {
        known_paths.push(expand_path(PathBuf::from(conda_dir.clone())));
    }
    if let Some(ref conda) = env_vars.conda {
        known_paths.push(expand_path(PathBuf::from(conda)));
    }
    if let Some(home) = env_vars.home.clone() {
        // https://stackoverflow.com/questions/35709497/anaconda-python-where-are-the-virtual-environments-stored
        let mut prefixes = vec![
            home.clone(),
            // https://towardsdatascience.com/manage-your-python-virtual-environment-with-conda-a0d2934d5195
            home.join("opt"),
            home.join(".conda"),
            home.join(".local"),
            // https://docs.conda.io/projects/conda/en/latest/user-guide/concepts/environments.html
            PathBuf::from("/opt"),
            PathBuf::from("/usr/share"),
            PathBuf::from("/usr/local"),
            PathBuf::from("/usr"),
        ];
        if std::env::consts::OS == "macos" {
            prefixes.push(PathBuf::from("/opt/homebrew"));
        } else {
            prefixes.push(PathBuf::from("/home/linuxbrew/.linuxbrew"));
        }

        for prefix in prefixes {
            known_paths.push(prefix.clone().join("anaconda"));
            known_paths.push(prefix.clone().join("anaconda3"));
            known_paths.push(prefix.clone().join("miniconda"));
            known_paths.push(prefix.clone().join("miniconda3"));
            known_paths.push(prefix.clone().join("miniforge3"));
            known_paths.push(prefix.clone().join("micromamba"));
        }

        known_paths.push(PathBuf::from("/opt").join("conda"));
        known_paths.push(home.join(".conda"));
        known_paths.push(home.join(".local"));
    }
    if let Some(conda_dir) = get_conda_dir_from_exe(conda_executable) {
        known_paths.push(conda_dir);
    }
    // Add conda installation found from PATH (handles non-standard locations)
    if let Some(conda_dir) = get_conda_dir_from_exe(&conda_from_path) {
        known_paths.push(conda_dir);
    }
    // Also check for mamba/micromamba on PATH to discover its install directory
    if let Some(mamba_dir) = get_conda_dir_from_exe(&find_mamba_binary(env_vars)) {
        known_paths.push(mamba_dir);
    }
    known_paths.sort();
    known_paths.dedup();
    known_paths.into_iter().filter(|f| f.exists()).collect()
}

pub fn get_conda_dir_from_exe(conda_executable: &Option<PathBuf>) -> Option<PathBuf> {
    if let Some(conda_executable) = conda_executable {
        if conda_executable.is_file() {
            if let Some(conda_dir) = conda_executable.parent() {
                // Possible exe is in the install (root prefix) directory.
                if is_conda_env(conda_dir) {
                    return Some(conda_dir.to_path_buf());
                } else if let Some(conda_dir) = conda_dir.parent() {
                    // Possible the exe is in the `bin` or `Scripts` directory.
                    if is_conda_env(conda_dir) {
                        return Some(conda_dir.to_path_buf());
                    }
                }
            }
        } else {
            let conda_dir = conda_executable.clone();
            // Possible exe is in the install (root prefix) directory.
            if is_conda_env(&conda_dir) {
                return Some(conda_dir.to_path_buf());
            } else if let Some(conda_dir) = conda_dir.parent() {
                // Possible the exe is in the `bin` or `Scripts` directory.
                if is_conda_env(conda_dir) {
                    return Some(conda_dir.to_path_buf());
                }
            }
        }
    }
    None
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn test_windows_drive_mount_detection() {
        // Typical WSL Windows drive mount paths
        assert!(is_windows_drive_mount(Path::new("/mnt/c/")));
        assert!(is_windows_drive_mount(Path::new("/mnt/c/Users")));
        assert!(is_windows_drive_mount(Path::new(
            "/mnt/d/Tools/Anaconda/envs/myenv"
        )));
        assert!(is_windows_drive_mount(Path::new("/mnt/C/Users")));
        assert!(is_windows_drive_mount(Path::new("/mnt/D/Tools")));

        // Just the drive letter with no trailing content
        assert!(is_windows_drive_mount(Path::new("/mnt/c")));

        // Not a Windows drive mount
        assert!(!is_windows_drive_mount(Path::new("/home/user/miniconda3")));
        assert!(!is_windows_drive_mount(Path::new("/opt/conda")));
        assert!(!is_windows_drive_mount(Path::new("/mnt/data/conda")));
        assert!(!is_windows_drive_mount(Path::new("/mnt/cdrom/something")));
        assert!(!is_windows_drive_mount(Path::new("/mnt/")));
        assert!(!is_windows_drive_mount(Path::new("/mnt")));
        assert!(!is_windows_drive_mount(Path::new("/mnt/12/path")));
    }
}
