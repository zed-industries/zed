// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::path::Path;

use pet_core::{
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    pyvenv_cfg::PyVenvCfg,
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_python_utils::executable::{find_executable_or_broken, find_executables, ExecutableResult};
use pet_python_utils::version;

fn is_venv_internal(env: &PythonEnv) -> Option<bool> {
    // env path cannot be empty.
    Some(
        PyVenvCfg::find(env.executable.parent()?).is_some()
            || PyVenvCfg::find(&env.prefix.clone()?).is_some(),
    )
}
pub fn is_venv(env: &PythonEnv) -> bool {
    is_venv_internal(env).unwrap_or_default()
}
pub fn is_venv_dir(path: &Path) -> bool {
    PyVenvCfg::find(path).is_some()
}

/// Tries to create a PythonEnvironment from a directory that might be a venv.
/// This function can detect broken environments (e.g., with broken symlinks)
/// and will return them with an error field set.
pub fn try_environment_from_venv_dir(path: &Path) -> Option<PythonEnvironment> {
    // Check if this is a venv directory
    let cfg = PyVenvCfg::find(path)?;

    let prefix = path.to_path_buf();
    let version = version::from_creator_for_virtual_env(&prefix).or(cfg.version.clone());
    let name = cfg.prompt;

    match find_executable_or_broken(path) {
        ExecutableResult::Found(executable) => {
            let symlinks = find_executables(&prefix);
            Some(
                PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Venv))
                    .name(name)
                    .executable(Some(executable))
                    .version(version)
                    .prefix(Some(prefix))
                    .symlinks(Some(symlinks))
                    .build(),
            )
        }
        ExecutableResult::Broken(executable) => Some(
            PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Venv))
                .name(name)
                .executable(Some(executable))
                .version(version)
                .prefix(Some(prefix))
                .error(Some("Python executable is a broken symlink".to_string()))
                .build(),
        ),
        ExecutableResult::NotFound => {
            // pyvenv.cfg exists but no Python executable found at all
            Some(
                PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Venv))
                    .name(name)
                    .version(version)
                    .prefix(Some(prefix))
                    .error(Some("Python executable not found".to_string()))
                    .build(),
            )
        }
    }
}

pub struct Venv {}

impl Venv {
    pub fn new() -> Venv {
        Venv {}
    }
}
impl Default for Venv {
    fn default() -> Self {
        Self::new()
    }
}
impl Locator for Venv {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::Venv
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::Venv]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if is_venv(env) {
            let mut prefix = env.prefix.clone();
            if prefix.is_none() {
                prefix = Some(env.executable.parent()?.parent()?.to_path_buf());
            }
            let version = match env.version {
                Some(ref v) => Some(v.clone()),
                None => match &prefix {
                    Some(prefix) => version::from_creator_for_virtual_env(prefix),
                    None => None,
                },
            };
            let mut symlinks = vec![];
            if let Some(ref prefix) = prefix {
                symlinks.append(&mut find_executables(prefix));
            }

            // Get the name from the prefix if it exists.
            let cfg = PyVenvCfg::find(env.executable.parent()?)
                .or_else(|| PyVenvCfg::find(&env.prefix.clone()?));
            let name = cfg.and_then(|cfg| cfg.prompt);

            Some(
                PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Venv))
                    .name(name)
                    .executable(Some(env.executable.clone()))
                    .version(version)
                    .prefix(prefix)
                    .symlinks(Some(symlinks))
                    .build(),
            )
        } else {
            None
        }
    }

    fn find(&self, _reporter: &dyn Reporter) {
        // There are no common global locations for virtual environments.
        // We expect the user of this class to call `is_compatible`
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_try_environment_from_venv_dir_not_a_venv() {
        // A directory without pyvenv.cfg should return None
        let temp_dir = std::env::temp_dir().join("pet_test_not_a_venv");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let result = try_environment_from_venv_dir(&temp_dir);
        assert!(result.is_none());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_try_environment_from_venv_dir_missing_executable() {
        // A venv with pyvenv.cfg but no Python executable
        let temp_dir = std::env::temp_dir().join("pet_test_venv_no_exe");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create pyvenv.cfg
        fs::write(
            temp_dir.join("pyvenv.cfg"),
            "version = 3.10.0\nprompt = test-env\n",
        )
        .unwrap();

        let result = try_environment_from_venv_dir(&temp_dir);
        assert!(result.is_some());

        let env = result.unwrap();
        assert_eq!(env.kind, Some(PythonEnvironmentKind::Venv));
        assert!(env.error.is_some());
        assert!(env.error.unwrap().contains("not found"));
        assert_eq!(env.name, Some("test-env".to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_try_environment_from_venv_dir_valid() {
        // A valid venv with pyvenv.cfg and Python executable
        let temp_dir = std::env::temp_dir().join("pet_test_venv_valid");
        let _ = fs::remove_dir_all(&temp_dir);

        #[cfg(windows)]
        let bin_dir = temp_dir.join("Scripts");
        #[cfg(unix)]
        let bin_dir = temp_dir.join("bin");

        fs::create_dir_all(&bin_dir).unwrap();

        // Create pyvenv.cfg
        fs::write(
            temp_dir.join("pyvenv.cfg"),
            "version = 3.11.0\nprompt = my-project\n",
        )
        .unwrap();

        // Create python executable
        #[cfg(windows)]
        let python_exe = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_exe = bin_dir.join("python");

        fs::write(&python_exe, "fake python").unwrap();

        let result = try_environment_from_venv_dir(&temp_dir);
        assert!(result.is_some());

        let env = result.unwrap();
        assert_eq!(env.kind, Some(PythonEnvironmentKind::Venv));
        assert!(env.error.is_none());
        assert!(env.executable.is_some());
        assert_eq!(env.name, Some("my-project".to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_try_environment_from_venv_dir_broken_symlink() {
        use std::os::unix::fs::symlink;

        // A venv with pyvenv.cfg but a broken symlink for Python
        let temp_dir = std::env::temp_dir().join("pet_test_venv_broken_symlink");
        let _ = fs::remove_dir_all(&temp_dir);

        let bin_dir = temp_dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        // Create pyvenv.cfg
        fs::write(
            temp_dir.join("pyvenv.cfg"),
            "version = 3.9.0\nprompt = broken-env\n",
        )
        .unwrap();

        // Create a broken symlink
        let python_exe = bin_dir.join("python");
        let nonexistent_target = std::path::PathBuf::from("/nonexistent/python3.9");
        symlink(&nonexistent_target, &python_exe).unwrap();

        let result = try_environment_from_venv_dir(&temp_dir);
        assert!(result.is_some());

        let env = result.unwrap();
        assert_eq!(env.kind, Some(PythonEnvironmentKind::Venv));
        assert!(env.error.is_some());
        assert!(env.error.as_ref().unwrap().contains("broken symlink"));
        assert_eq!(env.name, Some("broken-env".to_string()));
        assert!(env.executable.is_some());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_is_venv_dir_with_pyvenv_cfg() {
        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.11.4").unwrap();

        assert!(is_venv_dir(dir.path()));
    }

    #[test]
    fn test_is_venv_dir_without_pyvenv_cfg() {
        let dir = tempdir().unwrap();
        assert!(!is_venv_dir(dir.path()));
    }

    #[test]
    fn test_is_venv_with_pyvenv_cfg_in_parent() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.11.4").unwrap();

        // Create a fake python executable
        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        assert!(is_venv(&env));
    }

    #[test]
    fn test_is_venv_without_pyvenv_cfg() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        assert!(!is_venv(&env));
    }

    #[test]
    fn test_venv_locator_kind() {
        let venv = Venv::new();
        assert_eq!(venv.get_kind(), LocatorKind::Venv);
    }

    #[test]
    fn test_venv_supported_categories() {
        let venv = Venv::new();
        let categories = venv.supported_categories();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0], PythonEnvironmentKind::Venv);
    }

    #[test]
    fn test_venv_default() {
        let venv = Venv::default();
        assert_eq!(venv.get_kind(), LocatorKind::Venv);
    }

    #[test]
    fn test_venv_try_from_valid_venv() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.11.4").unwrap();
        writeln!(file, "prompt = my-test-env").unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path.clone(), Some(dir.path().to_path_buf()), None);
        let venv = Venv::new();
        let result = venv.try_from(&env);

        assert!(result.is_some());
        let py_env = result.unwrap();
        assert_eq!(py_env.kind, Some(PythonEnvironmentKind::Venv));
        assert_eq!(py_env.name, Some("my-test-env".to_string()));
        // Compare file names rather than full paths to avoid Windows 8.3 short path issues
        assert!(py_env.executable.is_some());
        assert_eq!(
            py_env.executable.as_ref().unwrap().file_name(),
            python_path.file_name()
        );
    }

    #[test]
    fn test_venv_try_from_non_venv() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        let venv = Venv::new();
        let result = venv.try_from(&env);

        assert!(result.is_none());
    }

    #[test]
    fn test_venv_try_from_without_prefix_derives_from_executable() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.12.0").unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        // No prefix provided — should derive from executable parent's parent
        let env = PythonEnv::new(python_path, None, None);
        let venv = Venv::new();
        let result = venv.try_from(&env);

        assert!(result.is_some());
        let py_env = result.unwrap();
        assert_eq!(py_env.kind, Some(PythonEnvironmentKind::Venv));
        assert!(py_env.prefix.is_some());
    }

    #[test]
    fn test_venv_try_from_with_version_in_env() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.11.0").unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        // Version provided in PythonEnv should be used as-is
        let mut env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        env.version = Some("3.11.5".to_string());
        let venv = Venv::new();
        let result = venv.try_from(&env);

        assert!(result.is_some());
        let py_env = result.unwrap();
        assert_eq!(py_env.version, Some("3.11.5".to_string()));
    }

    #[test]
    fn test_try_environment_from_venv_dir_no_prompt() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        // pyvenv.cfg without prompt
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.10.0").unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let result = try_environment_from_venv_dir(dir.path()).unwrap();

        assert_eq!(result.kind, Some(PythonEnvironmentKind::Venv));
        assert!(result.name.is_none());
        assert_eq!(result.version, Some("3.10.0".to_string()));
    }

    #[test]
    fn test_is_venv_via_executable_parent() {
        // Test that is_venv works when pyvenv.cfg is found via executable parent
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        // Place pyvenv.cfg next to bin dir (parent of executable's parent = dir)
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.10.0").unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        // No prefix — relies on executable.parent() finding pyvenv.cfg in bin_dir
        // Actually pyvenv.cfg is in dir, and PyVenvCfg::find checks the path and parent
        let env = PythonEnv::new(python_path, None, None);
        assert!(is_venv(&env));
    }
}
