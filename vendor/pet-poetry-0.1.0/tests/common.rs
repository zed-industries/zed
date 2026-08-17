// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use pet_poetry::env_variables::EnvVariables;
use std::{collections::HashMap, path::PathBuf};

#[allow(dead_code)]
pub fn resolve_test_path(paths: &[&str]) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");

    paths.iter().for_each(|p| root.push(p));

    root
}

#[allow(dead_code)]
pub fn create_env_variables(home: PathBuf, root: PathBuf) -> EnvVariables {
    EnvVariables {
        home: Some(home),
        root: Some(root),
        path: None,
        app_data: None,
        poetry_cache_dir: None,
        poetry_config_dir: None,
        poetry_home: None,
        poetry_virtualenvs_in_project: None,
        poetry_virtualenvs_path: None,
    }
}

#[allow(dead_code)]
pub struct TestEnvironment {
    vars: HashMap<String, String>,
    home: Option<PathBuf>,
    root: Option<PathBuf>,
}

impl Environment for TestEnvironment {
    fn get_env_var(&self, key: String) -> Option<String> {
        self.vars.get(&key).cloned()
    }
    fn get_root(&self) -> Option<PathBuf> {
        self.root.clone()
    }
    fn get_user_home(&self) -> Option<PathBuf> {
        self.home.clone()
    }
    fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
        vec![]
    }
}

#[allow(dead_code)]
pub fn create_test_environment(
    vars: HashMap<String, String>,
    home: Option<PathBuf>,
    root: Option<PathBuf>,
) -> TestEnvironment {
    TestEnvironment { vars, home, root }
}
