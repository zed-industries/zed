// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_conda::env_variables::EnvVariables;
use pet_core::os_environment::Environment;
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
        allusersprofile: None,
        conda_prefix: None,
        conda_root: None,
        conda: None,
        condarc: None,
        homedrive: None,
        known_global_search_locations: vec![],
        path: None,
        programdata: None,
        userprofile: None,
        xdg_config_home: None,
        conda_envs_path: None,
        conda_dir: None,
        anaconda_project_envs_path: None,
        project_dir: None,
        mamba_root_prefix: None,
        mambarc: None,
    }
}

#[allow(dead_code)]
pub struct TestEnvironment {
    vars: HashMap<String, String>,
    home: Option<PathBuf>,
    root: Option<PathBuf>,
    globals_locations: Vec<PathBuf>,
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
        self.globals_locations.clone()
    }
}

#[allow(dead_code)]
pub fn create_test_environment(
    vars: HashMap<String, String>,
    home: Option<PathBuf>,
    globals_locations: Vec<PathBuf>,
    root: Option<PathBuf>,
) -> TestEnvironment {
    TestEnvironment {
        vars,
        home,
        root,
        globals_locations,
    }
}
