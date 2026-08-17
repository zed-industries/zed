// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::path::PathBuf;

use pet_core::python_environment::{
    PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind,
};
use pet_python_utils::{executable::find_executables, version};

use crate::manager::PoetryManager;

pub fn create_poetry_env(
    prefix: &PathBuf,
    project_dir: PathBuf,
    manager: Option<PoetryManager>,
) -> Option<PythonEnvironment> {
    if !prefix.exists() {
        return None;
    }
    let executables = find_executables(prefix);
    if executables.is_empty() {
        return None;
    }
    let version = version::from_creator_for_virtual_env(prefix);
    Some(
        PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Poetry))
            .executable(Some(executables[0].clone()))
            .prefix(Some(prefix.clone()))
            .version(version)
            .manager(manager.map(|m| m.to_manager()))
            .project(Some(project_dir.clone()))
            .symlinks(Some(executables))
            .build(),
    )
}
