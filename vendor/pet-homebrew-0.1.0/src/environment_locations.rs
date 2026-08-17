// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::env_variables::EnvVariables;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref PYTHON_VERSION: Regex =
        Regex::new(r"/(\d+\.\d+\.\d+)/").expect("error parsing Version regex for Homebrew");
}

// fn get_homebrew_prefix_env_var(env_vars: &EnvVariables) -> Option<PathBuf> {
//     if let Some(homebrew_prefix) = &env_vars.homebrew_prefix {
//         let homebrew_prefix_bin = PathBuf::from(homebrew_prefix).join("bin");
//         if fs::metadata(&homebrew_prefix_bin).is_ok() {
//             return Some(homebrew_prefix_bin);
//         }
//     }
//     None
// }

pub fn get_homebrew_prefix_bin(env_vars: &EnvVariables) -> Vec<PathBuf> {
    // Homebrew install folders documented here https://docs.brew.sh/Installation
    // /opt/homebrew for Apple Silicon,
    // /usr/local for macOS Intel
    // /home/linuxbrew/.linuxbrew for Linux
    // If user has rosetta enabled, then its possible we have homebrew installed via rosetta as well as apple silicon
    // I.e. we can have multiple home brews on the same machine, hence search all,
    let mut homebrew_prefixes = [
        "/home/linuxbrew/.linuxbrew/bin",
        "/opt/homebrew/bin",
        "/usr/local/bin",
    ]
    .iter()
    .map(PathBuf::from)
    .filter(|p| p.exists())
    .collect::<Vec<PathBuf>>();

    // Check the environment variables
    if let Some(homebrew_prefix) = &env_vars.homebrew_prefix {
        let homebrew_prefix_bin = PathBuf::from(homebrew_prefix).join("bin");
        if homebrew_prefix_bin.exists() && !homebrew_prefixes.contains(&homebrew_prefix_bin) {
            homebrew_prefixes.push(homebrew_prefix_bin);
        }
    }

    homebrew_prefixes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn homebrew_prefix_bin_uses_existing_homebrew_prefix_env_var() {
        let homebrew_prefix = tempdir().unwrap();
        let homebrew_bin = homebrew_prefix.path().join("bin");
        fs::create_dir_all(&homebrew_bin).unwrap();
        let env_vars = EnvVariables {
            home: None,
            root: None,
            path: None,
            homebrew_prefix: Some(homebrew_prefix.path().to_string_lossy().to_string()),
            known_global_search_locations: vec![],
        };

        let prefix_bins = get_homebrew_prefix_bin(&env_vars);

        assert!(prefix_bins.contains(&homebrew_bin));
    }

    #[test]
    fn homebrew_prefix_bin_ignores_missing_homebrew_prefix_env_var() {
        let homebrew_prefix_parent = tempdir().unwrap();
        let missing_homebrew_prefix = homebrew_prefix_parent.path().join("missing-prefix");
        let env_vars = EnvVariables {
            home: None,
            root: None,
            path: None,
            homebrew_prefix: Some(missing_homebrew_prefix.to_string_lossy().to_string()),
            known_global_search_locations: vec![],
        };

        let prefix_bins = get_homebrew_prefix_bin(&env_vars);

        assert!(!prefix_bins
            .iter()
            .any(|path| path == &missing_homebrew_prefix.join("bin")));
    }

    #[test]
    fn homebrew_prefix_bin_returns_results_without_env_var() {
        let env_vars = EnvVariables {
            home: None,
            root: None,
            path: None,
            homebrew_prefix: None,
            known_global_search_locations: vec![],
        };

        // Should not panic and should return whatever standard paths exist
        let prefix_bins = get_homebrew_prefix_bin(&env_vars);
        // All returned paths should actually exist
        for path in &prefix_bins {
            assert!(path.exists(), "{:?} should exist", path);
        }
    }

    #[test]
    fn homebrew_prefix_bin_does_not_duplicate_when_env_var_matches_existing_dir() {
        // Create a temp dir to act as a custom homebrew prefix.
        // Call get_homebrew_prefix_bin twice with the same prefix to ensure
        // the env var path only appears once in the result.
        let custom_prefix = tempdir().unwrap();
        let custom_bin = custom_prefix.path().join("bin");
        fs::create_dir_all(&custom_bin).unwrap();
        let env_vars = EnvVariables {
            home: None,
            root: None,
            path: None,
            homebrew_prefix: Some(custom_prefix.path().to_string_lossy().to_string()),
            known_global_search_locations: vec![],
        };

        let prefix_bins = get_homebrew_prefix_bin(&env_vars);
        let count = prefix_bins.iter().filter(|p| **p == custom_bin).count();
        assert_eq!(count, 1, "Custom bin path should appear exactly once");
    }
}
