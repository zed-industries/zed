// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use log::error;
use pet_core::python_environment::{PythonEnvironment, PythonEnvironmentKind};
use std::path::PathBuf;

pub fn get_environment_key(env: &PythonEnvironment) -> Option<PathBuf> {
    if let Some(exe) = &env.executable {
        Some(exe.clone())
    } else if let Some(prefix) = &env.prefix {
        // If this is a conda env without Python, then the exe will be prefix/bin/python
        if env.kind == Some(PythonEnvironmentKind::Conda) {
            #[cfg(windows)]
            {
                Some(prefix.join("python.exe"))
            }
            #[cfg(not(windows))]
            {
                Some(prefix.join("bin").join("python"))
            }
        } else {
            Some(prefix.clone())
        }
    } else {
        error!(
            "Failed to report environment due to lack of exe & prefix: {:?}",
            env
        );
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executable_is_used_as_environment_key() {
        let executable = PathBuf::from("/tmp/.venv/bin/python");
        let environment = PythonEnvironment::new(
            Some(executable.clone()),
            Some(PythonEnvironmentKind::Venv),
            Some(PathBuf::from("/tmp/.venv")),
            None,
            None,
        );

        assert_eq!(get_environment_key(&environment), Some(executable));
    }

    #[test]
    fn conda_prefix_without_executable_gets_default_python_path() {
        let prefix = PathBuf::from("/tmp/conda-env");
        let environment = PythonEnvironment::new(
            None,
            Some(PythonEnvironmentKind::Conda),
            Some(prefix.clone()),
            None,
            None,
        );

        assert_eq!(
            get_environment_key(&environment),
            Some(if cfg!(windows) {
                prefix.join("python.exe")
            } else {
                prefix.join("bin").join("python")
            })
        );
    }

    #[test]
    fn non_conda_prefix_without_executable_uses_prefix() {
        let prefix = PathBuf::from("/tmp/.venv");
        let environment = PythonEnvironment::new(
            None,
            Some(PythonEnvironmentKind::Venv),
            Some(prefix.clone()),
            None,
            None,
        );

        assert_eq!(get_environment_key(&environment), Some(prefix));
    }

    #[test]
    fn environment_without_executable_or_prefix_has_no_key() {
        assert_eq!(get_environment_key(&PythonEnvironment::default()), None);
    }
}
