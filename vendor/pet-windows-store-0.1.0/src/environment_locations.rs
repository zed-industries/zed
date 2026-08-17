// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#[cfg(windows)]
use crate::env_variables::EnvVariables;
#[cfg(windows)]
use std::path::PathBuf;

#[cfg(windows)]
pub fn get_search_locations(environment: &EnvVariables) -> Option<PathBuf> {
    Some(
        environment
            .home
            .clone()?
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps"),
    )
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn search_locations_use_windowsapps_under_user_home() {
        let home = PathBuf::from(r"C:\\Users\\User");
        let env_variables = EnvVariables {
            home: Some(home.clone()),
        };

        assert_eq!(
            get_search_locations(&env_variables),
            Some(
                home.join("AppData")
                    .join("Local")
                    .join("Microsoft")
                    .join("WindowsApps")
            )
        );
    }

    #[test]
    fn search_locations_return_none_without_home() {
        assert_eq!(get_search_locations(&EnvVariables { home: None }), None);
    }
}
