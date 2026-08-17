// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::path::{Path, PathBuf};

use pet_core::{
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_python_utils::executable::find_executables;
use pet_python_utils::version;

pub fn is_virtualenv(env: &PythonEnv) -> bool {
    if env.prefix.is_none() {
        let mut bin = env.executable.clone();
        bin.pop();
        // Check if the executable is in a bin or Scripts directory.
        // Possible for some reason we do not have the prefix.
        if !bin.ends_with("bin") && !bin.ends_with("Scripts") {
            return false;
        }
    }
    if let Some(bin) = env.executable.parent() {
        return is_virtualenv_dir(bin);
    }

    false
}

pub fn is_virtualenv_dir(path: &Path) -> bool {
    if cfg!(windows) {
        is_virtualenv_dir_impl(path, "Scripts") || is_virtualenv_dir_impl(path, "bin")
    } else {
        is_virtualenv_dir_impl(path, "bin")
    }
}

fn is_virtualenv_dir_impl(path: &Path, bin: &str) -> bool {
    // Check if the executable is in a bin or Scripts directory.
    // Possible for some reason we do not have the prefix.
    let mut path = path.to_path_buf();
    if !path.ends_with("bin") && !path.ends_with("Scripts") {
        path = path.join(bin);
    }

    // Never consider global locations to be virtualenvs
    // in case there is a false positive match from checks below.
    if [
        PathBuf::from(r"/bin"),
        PathBuf::from(r"/usr/bin"),
        PathBuf::from(r"/usr/local/bin"),
    ]
    .contains(&path)
    {
        return false;
    }

    // Check if there are any activate.* files in the same directory as the interpreter.
    //
    // env
    // |__ activate, activate.*  <--- check if any of these files exist
    // |__ python  <--- interpreterPath

    // if let Some(parent_path) = PathBuf::from(env.)
    // const directory = path.dirname(interpreterPath);
    // const files = await fsapi.readdir(directory);
    // const regex = /^activate(\.([A-z]|\d)+)?$/i;
    if path.join("activate").exists() || path.join("activate.bat").exists() {
        return true;
    }

    // Support for activate.ps, etc.
    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(Result::ok).map(|e| e.path()) {
            if file
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .starts_with("activate.")
            {
                return true;
            }
        }
    }

    false
}

pub struct VirtualEnv {}

impl VirtualEnv {
    pub fn new() -> VirtualEnv {
        VirtualEnv {}
    }
}
impl Default for VirtualEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator for VirtualEnv {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::VirtualEnv
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::VirtualEnv]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if is_virtualenv(env) {
            let version = match env.version {
                Some(ref v) => Some(v.clone()),
                None => match &env.prefix {
                    Some(prefix) => version::from_creator_for_virtual_env(prefix),
                    None => None,
                },
            };
            let mut symlinks = vec![];
            if let Some(ref prefix) = env.prefix {
                symlinks.append(&mut find_executables(prefix));
            }
            Some(
                PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::VirtualEnv))
                    .executable(Some(env.executable.clone()))
                    .version(version)
                    .prefix(env.prefix.clone())
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
    use tempfile::tempdir;

    #[test]
    fn test_is_virtualenv_dir_with_activate() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate")).unwrap();

        assert!(is_virtualenv_dir(dir.path()));
    }

    #[test]
    fn test_is_virtualenv_dir_with_activate_bat() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate.bat")).unwrap();

        assert!(is_virtualenv_dir(dir.path()));
    }

    #[test]
    fn test_is_virtualenv_dir_with_activate_ps1() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate.ps1")).unwrap();

        assert!(is_virtualenv_dir(dir.path()));
    }

    #[test]
    fn test_is_virtualenv_dir_from_bin() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate")).unwrap();

        // Pass the bin directory itself
        assert!(is_virtualenv_dir(&bin_dir));
    }

    #[test]
    fn test_is_virtualenv_dir_without_activate() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        assert!(!is_virtualenv_dir(dir.path()));
    }

    #[test]
    fn test_is_virtualenv_dir_global_paths_excluded() {
        // Global paths should not be considered virtualenvs
        assert!(!is_virtualenv_dir(&PathBuf::from("/bin")));
        assert!(!is_virtualenv_dir(&PathBuf::from("/usr/bin")));
        assert!(!is_virtualenv_dir(&PathBuf::from("/usr/local/bin")));
    }

    #[test]
    fn test_is_virtualenv_with_activate() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate")).unwrap();

        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        assert!(is_virtualenv(&env));
    }

    #[test]
    fn test_is_virtualenv_without_activate() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, Some(dir.path().to_path_buf()), None);
        assert!(!is_virtualenv(&env));
    }

    #[test]
    fn test_is_virtualenv_without_prefix() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate")).unwrap();

        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        // No prefix provided
        let env = PythonEnv::new(python_path, None, None);
        assert!(is_virtualenv(&env));
    }

    #[test]
    fn test_is_virtualenv_without_prefix_and_not_in_bin() {
        let dir = tempdir().unwrap();
        // Not in bin or Scripts directory
        let python_path = dir.path().join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path, None, None);
        assert!(!is_virtualenv(&env));
    }

    #[test]
    fn test_virtualenv_locator_kind() {
        let venv = VirtualEnv::new();
        assert_eq!(venv.get_kind(), LocatorKind::VirtualEnv);
    }

    #[test]
    fn test_virtualenv_supported_categories() {
        let venv = VirtualEnv::new();
        let categories = venv.supported_categories();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0], PythonEnvironmentKind::VirtualEnv);
    }

    #[test]
    fn test_virtualenv_default() {
        let venv = VirtualEnv::default();
        assert_eq!(venv.get_kind(), LocatorKind::VirtualEnv);
    }

    #[test]
    fn test_virtualenv_try_from_valid() {
        let dir = tempdir().unwrap();
        #[cfg(windows)]
        let bin_dir = dir.path().join("Scripts");
        #[cfg(unix)]
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        #[cfg(windows)]
        fs::File::create(bin_dir.join("activate.bat")).unwrap();
        #[cfg(unix)]
        fs::File::create(bin_dir.join("activate")).unwrap();

        #[cfg(windows)]
        let python_path = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_path = bin_dir.join("python");
        fs::File::create(&python_path).unwrap();

        let env = PythonEnv::new(python_path.clone(), Some(dir.path().to_path_buf()), None);
        let venv = VirtualEnv::new();
        let result = venv.try_from(&env);

        assert!(result.is_some());
        let py_env = result.unwrap();
        assert_eq!(py_env.kind, Some(PythonEnvironmentKind::VirtualEnv));
        // Compare file names rather than full paths to avoid Windows 8.3 short path issues
        assert!(py_env.executable.is_some());
        assert_eq!(
            py_env.executable.as_ref().unwrap().file_name(),
            python_path.file_name()
        );
        assert!(py_env.prefix.is_some());
    }

    #[test]
    fn test_virtualenv_try_from_non_virtualenv() {
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
        let venv = VirtualEnv::new();
        let result = venv.try_from(&env);

        assert!(result.is_none());
    }
}
