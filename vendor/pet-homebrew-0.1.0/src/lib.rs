// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_variables::EnvVariables;
use environment_locations::get_homebrew_prefix_bin;
use environments::get_python_info;
use pet_conda::utils::is_conda_env;
use pet_core::{
    env::PythonEnv,
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_fs::path::resolve_symlink;
use pet_python_utils::executable::find_executables;
use pet_virtualenv::is_virtualenv;
use std::{fs, path::PathBuf, thread};
use sym_links::is_homebrew_python;

mod env_variables;
mod environment_locations;
mod environments;
mod sym_links;

pub struct Homebrew {
    environment: EnvVariables,
}

impl Homebrew {
    pub fn from(environment: &dyn Environment) -> Homebrew {
        Homebrew {
            environment: EnvVariables::from(environment),
        }
    }
}

/// Deafult prefix paths for Homebrew
/// Below are from the docs `man brew`      Display Homebrew’s install path. Default:
/// - macOS ARM: /opt/homebrew
/// - macOS Intel: /usr/local
/// - Linux: /home/linuxbrew/.linuxbrew
fn from(env: &PythonEnv) -> Option<PythonEnvironment> {
    // Assume we create a virtual env from a homebrew python install,
    // Then the exe in the virtual env bin will be a symlink to the homebrew python install.
    // Hence the first part of the condition will be true, but the second part will be false.
    if is_virtualenv(env) {
        return None;
    }

    if let Some(prefix) = &env.prefix {
        if is_conda_env(prefix) {
            return None;
        }
    }
    // Possible this is a root conda env (hence parent directory is conda install dir).
    if is_conda_env(env.executable.parent()?) {
        return None;
    }
    // Possible this is a conda env (hence parent directory is Scripts/bin dir).
    if is_conda_env(env.executable.parent()?.parent()?) {
        return None;
    }

    // Note: Sometimes if Python 3.10 was installed by other means (e.g. from python.org or other)
    // & then you install Python 3.10 via Homebrew, then some files will get installed via homebrew,
    // However everything (symlinks, Python executable `sys.executable`, `sys.prefix`) eventually point back to the existing installation.
    // Thus we do not end up with two versions of python 3.10, i.e. the existing installation is not overwritten nor duplicated.
    // Hence we never end up reporting 3.10 for home brew (as mentioned when you try to resolve the exe it points to existing install, now homebrew).
    let exe = env.executable.clone();
    let exe_file_name = exe.file_name()?;
    let mut resolved_file = resolve_symlink(&exe).unwrap_or(exe.clone());

    // Possible the resolve exe needs to be resolved once again using canonicalize.
    // Sometimes a symlink points to another symlink, and we need to resolve it to get the real exe.
    // And for some reason even though they are symlinks, they are not resolved by `resolve_symlink`.
    if let Some(resolved) = resolve_symlink(&exe).or(fs::canonicalize(&exe).ok()) {
        if is_homebrew_python(&resolved) {
            resolved_file = resolved;
        }
    }

    // Cellar is where the executables will be installed, see below link
    // https://docs.brew.sh/Formula-Cookbook#an-introduction
    // From above link > Homebrew installs formulae to the Cellar at $(brew --cellar)
    // and then symlinks some of the installation into the prefix at $(brew --prefix) (e.g. /opt/homebrew) so that other programs can see what’s going on.
    // Hence look in `Cellar` directory
    if resolved_file.starts_with("/opt/homebrew") {
        // Symlink  - /opt/homebrew/bin/python3.12
        // Symlink  - /opt/homebrew/opt/python3/bin/python3.12
        // Symlink  - /opt/homebrew/opt/python@3.12/bin/python3.12
        // Symlink  - /opt/homebrew/Cellar/python@3.12/3.12.3/bin/python3.12
        // Real exe - /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12
        // Symlink  - /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/Current/bin/python3.12
        // Symlink  - /opt/homebrew/Frameworks/Python.framework/Versions/3.12/bin/python3.12
        // Symlink  - /opt/homebrew/Frameworks/Python.framework/Versions/Current/bin/python3.12
        // SysPrefix- /opt/homebrew/opt/python@3.12/Frameworks/Python.framework/Versions/3.12
        get_python_info(
            &PathBuf::from("/opt/homebrew/bin").join(exe_file_name),
            &resolved_file,
        )
    } else if resolved_file.starts_with("/home/linuxbrew/.linuxbrew") {
        // Symlink  - /home/linuxbrew/.linuxbrew/bin/python3.12
        // Symlink  - /home/linuxbrew/.linuxbrew/opt/python@3.12/bin/python3.12
        // Real exe - /home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.3/bin/python3.12
        // SysPrefix- /home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.3

        get_python_info(
            &PathBuf::from("/home/linuxbrew/.linuxbrew/bin").join(exe_file_name),
            &resolved_file,
        )
    } else if resolved_file.starts_with("/usr/local/Cellar") {
        // Symlink  - /usr/local/bin/python3.8
        // Symlink  - /usr/local/opt/python@3.8/bin/python3.8
        // Symlink  - /usr/local/Cellar/python@3.8/3.8.20/bin/python3.8
        // Real exe - /usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8
        // SysPrefix- /usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8
        get_python_info(
            &PathBuf::from("/usr/local/bin").join(exe_file_name),
            &resolved_file,
        )
    } else {
        None
    }
}

impl Locator for Homebrew {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::Homebrew
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::Homebrew]
    }
    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        from(env)
    }

    fn find(&self, reporter: &dyn Reporter) {
        let homebrew_prefix_bins = get_homebrew_prefix_bin(&self.environment);
        thread::scope(|s| {
            for homebrew_prefix_bin in &homebrew_prefix_bins {
                let homebrew_python_exes = find_executables(homebrew_prefix_bin);
                for file in homebrew_python_exes.iter().filter(|f| {
                    let file_name = f
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_lowercase();
                    file_name.starts_with("python")
                    // If this file name is `python3`, then ignore this for now.
                    // We would prefer to use `python3.x` instead of `python3`.
                    // That way its more consistent and future proof
                        && file_name != "python3"
                        && file_name != "python"
                }) {
                    let file = file.clone();
                    s.spawn(move || {
                        // Sometimes we end up with other python installs in the Homebrew bin directory.
                        // E.g. /usr/local/bin is treated as a location where homebrew can be found (homebrew bin)
                        // However this is a very generic location, and we might end up with other python installs here.
                        // Hence call `resolve` to correctly identify homebrew python installs.
                        let env_to_resolve = PythonEnv::new(file.clone(), None, None);
                        if let Some(env) = from(&env_to_resolve) {
                            reporter.report_environment(&env);
                        }
                    });
                }
            }
        });
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use tempfile::tempdir;

    struct TestEnvironment {
        homebrew_prefix: Option<String>,
    }

    impl Environment for TestEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            None
        }

        fn get_root(&self) -> Option<PathBuf> {
            None
        }

        fn get_env_var(&self, key: String) -> Option<String> {
            if key == "HOMEBREW_PREFIX" {
                self.homebrew_prefix.clone()
            } else {
                None
            }
        }

        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            vec![]
        }
    }

    #[test]
    fn homebrew_locator_reports_kind_and_supported_category() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });

        assert_eq!(locator.get_kind(), LocatorKind::Homebrew);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::Homebrew]
        );
    }

    #[test]
    fn try_from_identifies_linuxbrew_python_executable() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        let env = PythonEnv::new(
            PathBuf::from("/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12"),
            None,
            None,
        );

        let homebrew_env = locator.try_from(&env).unwrap();

        assert_eq!(homebrew_env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(
            homebrew_env.executable,
            Some(PathBuf::from("/home/linuxbrew/.linuxbrew/bin/python3.12"))
        );
        assert_eq!(homebrew_env.version, Some("3.12.4".to_string()));
        assert_eq!(homebrew_env.prefix, None);
        assert!(homebrew_env
            .symlinks
            .as_ref()
            .unwrap()
            .contains(&PathBuf::from(
                "/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12"
            )));
    }

    #[test]
    fn try_from_rejects_non_homebrew_python() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        let env = PythonEnv::new(PathBuf::from("/usr/bin/python3.12"), None, None);

        assert!(locator.try_from(&env).is_none());
    }

    #[test]
    fn try_from_rejects_virtualenv_and_conda_prefixes() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });

        let venv_root = tempdir().unwrap();
        let venv_bin = venv_root.path().join("bin");
        fs::create_dir_all(&venv_bin).unwrap();
        fs::write(venv_bin.join("activate"), b"").unwrap();
        let venv_executable = venv_bin.join("python3.12");
        fs::write(&venv_executable, b"").unwrap();
        let venv = PythonEnv::new(venv_executable, Some(venv_root.path().to_path_buf()), None);
        assert!(locator.try_from(&venv).is_none());

        let conda_root = tempdir().unwrap();
        fs::create_dir_all(conda_root.path().join("conda-meta")).unwrap();
        let conda_executable = conda_root.path().join("bin").join("python3.12");
        fs::create_dir_all(conda_executable.parent().unwrap()).unwrap();
        fs::write(&conda_executable, b"").unwrap();
        let conda = PythonEnv::new(
            conda_executable,
            Some(conda_root.path().to_path_buf()),
            None,
        );
        assert!(locator.try_from(&conda).is_none());
    }

    #[test]
    fn try_from_identifies_opt_homebrew_python() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        let env = PythonEnv::new(
            PathBuf::from(
                "/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12",
            ),
            None,
            None,
        );

        let homebrew_env = locator.try_from(&env).unwrap();

        assert_eq!(homebrew_env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(
            homebrew_env.executable,
            Some(PathBuf::from("/opt/homebrew/bin/python3.12"))
        );
        assert_eq!(homebrew_env.version, Some("3.12.3".to_string()));
    }

    #[test]
    fn try_from_identifies_usr_local_cellar_python() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        let env = PythonEnv::new(
            PathBuf::from(
                "/usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8",
            ),
            None,
            None,
        );

        let homebrew_env = locator.try_from(&env).unwrap();

        assert_eq!(homebrew_env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(
            homebrew_env.executable,
            Some(PathBuf::from("/usr/local/bin/python3.8"))
        );
        assert_eq!(homebrew_env.version, Some("3.8.20".to_string()));
    }

    #[test]
    fn try_from_rejects_conda_env_when_parent_is_conda() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        // Create a directory that looks like a conda env (has conda-meta)
        let conda_root = tempdir().unwrap();
        fs::create_dir_all(conda_root.path().join("conda-meta")).unwrap();
        // Place executable directly in the conda-meta parent directory
        let exe = conda_root.path().join("python3.12");
        fs::write(&exe, b"").unwrap();

        let env = PythonEnv::new(exe, None, None);
        assert!(locator.try_from(&env).is_none());
    }

    #[test]
    fn try_from_rejects_conda_env_when_grandparent_is_conda() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        // Create a directory that looks like a conda env (has conda-meta)
        let conda_root = tempdir().unwrap();
        fs::create_dir_all(conda_root.path().join("conda-meta")).unwrap();
        let bin_dir = conda_root.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = bin_dir.join("python3.12");
        fs::write(&exe, b"").unwrap();

        let env = PythonEnv::new(exe, None, None);
        assert!(locator.try_from(&env).is_none());
    }

    #[test]
    fn try_from_rejects_conda_env_via_prefix() {
        let locator = Homebrew::from(&TestEnvironment {
            homebrew_prefix: None,
        });
        // Conda env detected via prefix having conda-meta
        let conda_root = tempdir().unwrap();
        fs::create_dir_all(conda_root.path().join("conda-meta")).unwrap();
        let bin_dir = conda_root.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = bin_dir.join("python3.12");
        fs::write(&exe, b"").unwrap();

        let env = PythonEnv::new(exe, Some(conda_root.path().to_path_buf()), None);
        assert!(locator.try_from(&env).is_none());
    }
}
