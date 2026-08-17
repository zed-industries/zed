// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    pub home: Option<PathBuf>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        EnvVariables {
            home: env.get_user_home(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEnvironment {
        user_home: Option<PathBuf>,
    }

    impl Environment for TestEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            self.user_home.clone()
        }

        fn get_root(&self) -> Option<PathBuf> {
            None
        }

        fn get_env_var(&self, _key: String) -> Option<String> {
            None
        }

        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            vec![]
        }
    }

    #[test]
    fn env_variables_reads_home() {
        let environment = TestEnvironment {
            user_home: Some(PathBuf::from(r"C:\\Users\\User")),
        };

        assert_eq!(
            EnvVariables::from(&environment).home,
            Some(PathBuf::from(r"C:\\Users\\User"))
        );
    }

    #[test]
    fn env_variables_preserves_missing_home() {
        let environment = TestEnvironment { user_home: None };

        assert_eq!(EnvVariables::from(&environment).home, None);
    }
}
