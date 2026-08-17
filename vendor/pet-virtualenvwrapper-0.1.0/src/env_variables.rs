// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    pub home: Option<PathBuf>,
    pub workon_home: Option<String>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        EnvVariables {
            home: env.get_user_home(),
            workon_home: env.get_env_var("WORKON_HOME".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestEnvironment {
        user_home: Option<PathBuf>,
        workon_home: Option<String>,
    }

    impl Environment for TestEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            self.user_home.clone()
        }

        fn get_root(&self) -> Option<PathBuf> {
            None
        }

        fn get_env_var(&self, key: String) -> Option<String> {
            if key == "WORKON_HOME" {
                self.workon_home.clone()
            } else {
                None
            }
        }

        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            vec![]
        }
    }

    #[test]
    fn env_variables_reads_home_and_workon_home() {
        let environment = TestEnvironment {
            user_home: Some(PathBuf::from("/home/user")),
            workon_home: Some("/tmp/workon-home".to_string()),
        };

        let env_variables = EnvVariables::from(&environment);

        assert_eq!(env_variables.home, Some(PathBuf::from("/home/user")));
        assert_eq!(
            env_variables.workon_home,
            Some("/tmp/workon-home".to_string())
        );
    }

    #[test]
    fn env_variables_preserves_missing_values() {
        let environment = TestEnvironment {
            user_home: None,
            workon_home: None,
        };

        let env_variables = EnvVariables::from(&environment);

        assert_eq!(env_variables.home, None);
        assert_eq!(env_variables.workon_home, None);
    }
}
