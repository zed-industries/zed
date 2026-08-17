// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    manager::CondaManager,
    package::{CondaPackageInfo, Package},
    utils::{is_conda_env, is_conda_install},
};
use log::{trace, warn};
use pet_core::{
    arch::Architecture,
    manager::EnvManager,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
};
use pet_fs::path::{norm_case, resolve_symlink};
use pet_python_utils::executable::{find_executable, find_executables};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CondaEnvironment {
    pub prefix: PathBuf,
    pub executable: Option<PathBuf>,
    pub version: Option<String>,
    pub conda_dir: Option<PathBuf>,
    pub arch: Option<Architecture>,
    pub name: Option<String>,
}

impl CondaEnvironment {
    pub fn from(path: &Path, manager: &Option<CondaManager>) -> Option<Self> {
        get_conda_environment_info(path, manager)
    }

    pub fn to_python_environment(&self, conda_manager: Option<EnvManager>) -> PythonEnvironment {
        // This is a root env.
        let builder = PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Conda))
            .executable(self.executable.clone())
            .version(self.version.clone())
            .prefix(Some(self.prefix.clone()))
            .arch(self.arch.clone())
            .symlinks(Some(find_executables(&self.prefix)))
            .name(self.name.clone())
            .manager(conda_manager);

        builder.build()
    }
}

pub fn get_conda_environment_info(
    env_path: &Path,
    manager: &Option<CondaManager>,
) -> Option<CondaEnvironment> {
    get_conda_environment_info_with_history_reader(env_path, manager, |env_path| {
        std::fs::read_to_string(env_path.join("conda-meta").join("history")).ok()
    })
}

fn get_conda_environment_info_with_history_reader<F>(
    env_path: &Path,
    manager: &Option<CondaManager>,
    read_history: F,
) -> Option<CondaEnvironment>
where
    F: FnOnce(&Path) -> Option<String>,
{
    if !is_conda_env(env_path) {
        return None;
    }

    let history = read_history(env_path);
    let creation_line = history.as_deref().and_then(get_conda_creation_line);
    let mut conda_install_folder =
        get_conda_installation_used_to_create_conda_env_from_creation_line(
            env_path,
            creation_line.as_deref(),
        )
        .or_else(|| manager.clone().and_then(|manager| manager.conda_dir));

    if let Some(conda_dir) = &conda_install_folder {
        if conda_dir.exists() {
            trace!(
                "Conda install folder {}, found, & will be used for the Conda Env: {}",
                conda_dir.display(),
                env_path.display()
            );
        } else {
            warn!(
                "Conda install folder {}, does not exist, hence will not be used for the Conda Env: {}",
                conda_dir.display(),
                env_path.display()
            );
            conda_install_folder = None;
        }
    } else {
        trace!("Conda install folder not found for {}", env_path.display());
    }

    let executable = find_executable(env_path);
    let package_info = executable.as_ref().and_then(|_| {
        CondaPackageInfo::from_history(env_path, &Package::Python, history.as_deref())
    });
    let name = get_conda_env_name(env_path, &conda_install_folder, creation_line.as_deref());

    Some(CondaEnvironment {
        prefix: env_path.into(),
        executable,
        version: package_info.as_ref().map(|info| info.version.clone()),
        conda_dir: conda_install_folder,
        arch: package_info.and_then(|info| info.arch),
        name,
    })
}

/**
 * The conda-meta/history file in conda environments contains the command used to create the environment.
 * This function returns the path to the conda installation that created the environment.
 */
pub fn get_conda_installation_used_to_create_conda_env(env_path: &Path) -> Option<PathBuf> {
    let history = std::fs::read_to_string(env_path.join("conda-meta").join("history")).ok();
    let creation_line = history.as_deref().and_then(get_conda_creation_line);
    get_conda_installation_used_to_create_conda_env_from_creation_line(
        env_path,
        creation_line.as_deref(),
    )
}

fn get_conda_installation_used_to_create_conda_env_from_creation_line(
    env_path: &Path,
    creation_line: Option<&str>,
) -> Option<PathBuf> {
    if let Some(parent) = env_path.ancestors().nth(2) {
        if is_conda_install(parent) {
            return Some(parent.to_path_buf());
        }
    }

    if let Some(line) = creation_line {
        if let Some(conda_dir) = get_conda_dir_from_cmd(line) {
            if is_conda_install(&conda_dir) {
                return Some(conda_dir);
            }
            if let Some(conda_dir) = conda_dir.parent() {
                if is_conda_install(conda_dir) {
                    return Some(conda_dir.into());
                }
            }
        }
    }

    if is_conda_install(env_path) {
        Some(env_path.to_path_buf())
    } else {
        None
    }
}

pub fn get_conda_creation_line_from_history(env_path: &Path) -> Option<String> {
    let history = std::fs::read_to_string(env_path.join("conda-meta").join("history")).ok()?;
    get_conda_creation_line(&history)
}

fn get_conda_creation_line(history: &str) -> Option<String> {
    let line = history.lines().map(str::trim).find(|line| {
        let line = line.to_lowercase();
        line.starts_with("# cmd:") && line.contains(" create -")
    })?;
    trace!("Conda creation line from history is {:?}", line);
    Some(line.into())
}

fn get_conda_env_name(
    prefix: &Path,
    conda_dir: &Option<PathBuf>,
    creation_line: Option<&str>,
) -> Option<String> {
    let mut name = if is_conda_install(prefix) {
        Some("base".to_string())
    } else {
        prefix
            .file_name()
            .map(|name| name.to_str().unwrap_or_default().to_string())
    };

    if let Some(conda_dir) = conda_dir {
        if !prefix.starts_with(conda_dir) {
            name = get_conda_env_name_from_creation_line(prefix, creation_line);
        }
    }

    name
}

fn get_conda_env_name_from_creation_line(
    prefix: &Path,
    creation_line: Option<&str>,
) -> Option<String> {
    let name = prefix.file_name()?.to_str()?.to_string();
    let line = creation_line?;
    if is_conda_env_name_in_cmd(line, &name) {
        Some(name)
    } else {
        None
    }
}

fn is_conda_env_name_in_cmd(cmd_line: &str, name: &str) -> bool {
    cmd_line.contains(format!("-n {name}").as_str())
        || cmd_line.contains(format!("--name {name}").as_str())
}
fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .as_bytes()
        .windows(needle.len())
        .position(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

fn get_conda_executable_from_cmd(cmd_line: &str) -> Option<PathBuf> {
    let start_index = find_ascii_case_insensitive(cmd_line, "# cmd:")? + "# cmd:".len();
    let end_index = find_ascii_case_insensitive(cmd_line, " create -")?;
    let executable = cmd_line.get(start_index..end_index)?.trim();
    (!executable.is_empty()).then(|| PathBuf::from(executable))
}

fn get_conda_dir_from_cmd(cmd_line: &str) -> Option<PathBuf> {
    // Sample lines
    // # cmd: <conda install directory>\Scripts\conda-script.py create -n sample
    // # cmd: <conda install directory>\Scripts\conda-script.py create -p <full path>
    // # cmd: /Users/donjayamanne/miniconda3/bin/conda create -n conda1
    // cmd_line: "# cmd: /usr/bin/conda create -p ./prefix-envs/.conda1 python=3.12 -y"
    let conda_exe = get_conda_executable_from_cmd(cmd_line)?; // Sometimes the path can be as follows, where `/usr/bin/conda` could be a symlink.
                                                              // cmd_line: "# cmd: /usr/bin/conda create -p ./prefix-envs/.conda1 python=3.12 -y"
    let conda_exe = resolve_symlink(&conda_exe).unwrap_or(conda_exe);
    if let Some(cmd_line) = conda_exe.parent() {
        if let Some(conda_dir) = cmd_line.file_name() {
            if conda_dir.to_string_lossy().to_lowercase() == "bin"
                || conda_dir.to_string_lossy().to_lowercase() == "scripts"
                || conda_dir.to_string_lossy().to_lowercase() == "condabin"
            {
                if let Some(conda_dir) = cmd_line.parent() {
                    // Ensure the casing of the paths are correct.
                    // Its possible the actual path is in a different case.
                    // The casing in history might not be same as that on disc
                    // We do not want to have duplicates in different cases.
                    // & we'd like to preserve the case of the original path as on disc.
                    return Some(norm_case(conda_dir).to_path_buf());
                }
            }
            // Sometimes we can have paths like
            // # cmd: C:\Users\donja\miniconda3\lib\site-packages\conda\__main__.py create --yes --prefix .conda python=3.9
            // # cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes --prefix .conda python=3.12

            let mut cmd_line = cmd_line.to_path_buf();
            if cmd_line
                .to_str()
                .unwrap_or_default()
                .contains("site-packages")
                && cmd_line.to_str().unwrap_or_default().contains("lib")
            {
                loop {
                    if cmd_line.to_str().unwrap_or_default().contains("lib")
                        && !cmd_line.to_str().unwrap_or_default().ends_with("lib")
                    {
                        let _ = cmd_line.pop();
                    } else {
                        break;
                    }
                }
                if cmd_line.ends_with("lib") {
                    let _ = cmd_line.pop();
                }
            }
            // Ensure the casing of the paths are correct.
            // Its possible the actual path is in a different case.
            // The casing in history might not be same as that on disc
            // We do not want to have duplicates in different cases.
            // & we'd like to preserve the case of the original path as on disc.
            return Some(norm_case(&cmd_line).to_path_buf());
        }
    }
    None
}

pub fn get_activation_command(
    env: &CondaEnvironment,
    manager: &EnvManager,
    name: Option<String>,
) -> Option<Vec<String>> {
    let conda_exe = manager.executable.to_str().unwrap_or_default().to_string();
    if let Some(name) = name {
        Some(vec![
            conda_exe,
            "run".to_string(),
            "-n".to_string(),
            name,
            "python".to_string(),
        ])
    } else {
        Some(vec![
            conda_exe,
            "run".to_string(),
            "-p".to_string(),
            env.prefix.to_str().unwrap_or_default().to_string(),
            "python".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unicode_conda_executable_without_invalid_byte_indices() {
        let line = "# CMD: /Users/İpek/miniconda3/bin/conda CREATE -n sample";

        assert_eq!(
            get_conda_executable_from_cmd(line),
            Some(PathBuf::from("/Users/İpek/miniconda3/bin/conda"))
        );
    }
    #[test]
    #[cfg(windows)]
    fn parse_cmd_line() {
        let line = "# cmd: C:\\Users\\donja\\miniconda3\\lib\\site-packages\\conda\\__main__.py create --yes --prefix .conda python=3.9";
        let conda_dir = get_conda_dir_from_cmd(line).unwrap();

        assert_eq!(conda_dir, PathBuf::from("C:\\Users\\donja\\miniconda3"));

        let line =
            "# cmd: C:\\Users\\donja\\miniconda3\\Scripts\\conda-script.py create -n samlpe1";
        let conda_dir = get_conda_dir_from_cmd(line).unwrap();

        assert_eq!(conda_dir, PathBuf::from("C:\\Users\\donja\\miniconda3"));

        // From root install folder
        let line = "# cmd: build.py --product miniconda --python 3.9 --installer-type exe --output-dir C:\\ci\\containers\\000029l07m4\\tmp\\build\\dd3144c1\\output-installer/220421/ --standalone C:\\ci\\containers\\000029l07m4\\tmp\\build\\dd3144c1\\mc/standalone_conda/conda.exe";
        let conda_dir = get_conda_dir_from_cmd(line);

        assert!(conda_dir.is_none());
    }

    #[test]
    #[cfg(unix)]
    fn parse_cmd_line() {
        let line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes --prefix .conda python=3.12";
        let conda_dir = get_conda_dir_from_cmd(line).unwrap();

        assert_eq!(
            conda_dir,
            PathBuf::from("/Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3")
        );
    }

    #[test]
    #[cfg(unix)]
    fn verify_conda_env_name() {
        let line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes --name .conda python=3.12";
        assert!(is_conda_env_name_in_cmd(line, ".conda"));

        let mut line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes -n .conda python=3.12";
        assert!(is_conda_env_name_in_cmd(line, ".conda"));

        line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes --name .conda python=3.12";
        assert!(!is_conda_env_name_in_cmd(line, "base"));

        line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes -p .conda python=3.12";
        assert!(!is_conda_env_name_in_cmd(line, "base"));

        line = "# cmd: /Users/donjayamanne/.pyenv/versions/mambaforge-22.11.1-3/lib/python3.10/site-packages/conda/__main__.py create --yes -p .conda python=3.12";
        assert!(!is_conda_env_name_in_cmd(line, ".conda"));
    }

    /// Test that external environments (not under conda_dir) created with --prefix
    /// return None for name, so activation uses path instead of name.
    /// This is the fix for issue #329.
    #[test]
    fn external_path_based_env_returns_none_name() {
        // Create a temp directory simulating an external path-based conda env
        let temp_dir = std::env::temp_dir().join("pet_test_external_path_env");
        let conda_meta_dir = temp_dir.join(".conda").join("conda-meta");
        std::fs::create_dir_all(&conda_meta_dir).unwrap();

        // Write a history file showing the env was created with --prefix (path-based)
        let history_file = conda_meta_dir.join("history");
        std::fs::write(
            &history_file,
            "# cmd: /usr/bin/conda create --yes --prefix .conda python=3.12\n",
        )
        .unwrap();

        let env_path = temp_dir.join(".conda");
        // conda_dir is known but env is NOT under it (external environment)
        let conda_dir = Some(std::path::PathBuf::from("/some/other/conda"));

        let history = std::fs::read_to_string(&history_file).unwrap();
        let creation_line = get_conda_creation_line(&history);
        let name = get_conda_env_name(&env_path, &conda_dir, creation_line.as_deref());
        assert!(
            name.is_none(),
            "Path-based external env should return None for name, got {:?}",
            name
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    /// Test that external environments (not under conda_dir) created with -n
    /// return the name for name-based activation, but only if the folder name matches.
    #[test]
    fn external_name_based_env_returns_name() {
        // Create a temp directory simulating an external name-based conda env
        let temp_dir = std::env::temp_dir().join("pet_test_external_name_env");
        let conda_meta_dir = temp_dir.join("myenv").join("conda-meta");
        std::fs::create_dir_all(&conda_meta_dir).unwrap();

        // Write a history file showing the env was created with -n myenv (name-based)
        // Note: the folder name "myenv" matches the -n argument "myenv"
        let history_file = conda_meta_dir.join("history");
        std::fs::write(
            &history_file,
            "# cmd: /usr/bin/conda create -n myenv python=3.12\n",
        )
        .unwrap();

        let env_path = temp_dir.join("myenv");
        // conda_dir is known but env is NOT under it (external environment)
        let conda_dir = Some(std::path::PathBuf::from("/some/other/conda"));

        let history = std::fs::read_to_string(&history_file).unwrap();
        let creation_line = get_conda_creation_line(&history);
        let name = get_conda_env_name(&env_path, &conda_dir, creation_line.as_deref());
        assert_eq!(
            name,
            Some("myenv".to_string()),
            "Name-based external env should return the name when folder matches"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    /// Test that environments under conda_dir/envs/ return the folder name.
    /// This is the most common case for named conda environments.
    #[test]
    fn env_under_conda_dir_returns_folder_name() {
        // Create a temp directory simulating conda_dir/envs/myenv structure
        let temp_dir = std::env::temp_dir().join("pet_test_env_under_conda");
        let conda_dir = temp_dir.join("miniconda3");
        let env_path = conda_dir.join("envs").join("myenv");
        let conda_meta_dir = env_path.join("conda-meta");
        std::fs::create_dir_all(&conda_meta_dir).unwrap();

        // When env is under conda_dir/envs/, name should be the folder name
        let name = get_conda_env_name(&env_path, &Some(conda_dir), None);
        assert_eq!(
            name,
            Some("myenv".to_string()),
            "Env under conda_dir/envs/ should return folder name"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    /// Test that external env with no history file returns None for name.
    /// This ensures safe path-based activation when we can't determine how it was created.
    #[test]
    fn external_env_without_history_returns_none_name() {
        // Create a temp directory simulating an external conda env without history
        let temp_dir = std::env::temp_dir().join("pet_test_external_no_history");
        let conda_meta_dir = temp_dir.join("myenv").join("conda-meta");
        std::fs::create_dir_all(&conda_meta_dir).unwrap();
        // Note: NOT creating a history file

        let env_path = temp_dir.join("myenv");
        // conda_dir is known but env is NOT under it (external environment)
        let conda_dir = Some(std::path::PathBuf::from("/some/other/conda"));

        let name = get_conda_env_name(&env_path, &conda_dir, None);
        assert!(
            name.is_none(),
            "External env without history should return None for safe path-based activation, got {:?}",
            name
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    /// Test that external env with history but folder name doesn't match -n argument returns None.
    /// This prevents wrong activation when env was moved/renamed after creation.
    #[test]
    fn external_env_with_mismatched_name_returns_none() {
        // Create a temp directory simulating an external conda env
        let temp_dir = std::env::temp_dir().join("pet_test_external_mismatch");
        // Folder is named "renamed_env" but was created with -n "original_name"
        let conda_meta_dir = temp_dir.join("renamed_env").join("conda-meta");
        std::fs::create_dir_all(&conda_meta_dir).unwrap();

        let history_file = conda_meta_dir.join("history");
        std::fs::write(
            &history_file,
            "# cmd: /usr/bin/conda create -n original_name python=3.12\n",
        )
        .unwrap();

        let env_path = temp_dir.join("renamed_env");
        let conda_dir = Some(std::path::PathBuf::from("/some/other/conda"));

        let history = std::fs::read_to_string(&history_file).unwrap();
        let creation_line = get_conda_creation_line(&history);
        let name = get_conda_env_name(&env_path, &conda_dir, creation_line.as_deref());
        assert!(
            name.is_none(),
            "External env with mismatched name should return None, got {:?}",
            name
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
    #[test]
    fn environment_info_reads_history_once() {
        let temp_dir =
            std::env::temp_dir().join(format!("pet-conda-history-read-{}", std::process::id()));
        let env_path = temp_dir.join("env");
        std::fs::create_dir_all(env_path.join("conda-meta")).unwrap();

        let reads = std::cell::Cell::new(0);
        let environment = get_conda_environment_info_with_history_reader(&env_path, &None, |_| {
            reads.set(reads.get() + 1);
            Some("# cmd: conda create -p env\n+defaults::python-3.12.0-build".into())
        })
        .unwrap();

        assert_eq!(reads.get(), 1);
        assert_eq!(environment.prefix, env_path);

        std::fs::remove_dir_all(temp_dir).unwrap();
    }
}
