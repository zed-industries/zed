// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{collections::HashSet, path::PathBuf};

use log::warn;
use pet_core::{
    python_environment::PythonEnvironment,
    reporter::Reporter,
    telemetry::{missing_poetry_info::MissingPoetryEnvironments, TelemetryEvent},
    LocatorResult,
};

use crate::{config::Config, env_variables::EnvVariables, environment_locations_spawn::get_config};

pub fn report_missing_envs(
    reporter: &dyn Reporter,
    executable: &PathBuf,
    workspace_dirs: Vec<PathBuf>,
    env_vars: &EnvVariables,
    envs_discovered_by_poetry: &[PythonEnvironment],
    envs_discovered_by_us: Option<LocatorResult>,
    user_provided_poetry_exe: bool,
) -> Option<()> {
    for workspace_dir in workspace_dirs {
        let config = get_config(executable, &workspace_dir);
        let global_config = Config::find_global(env_vars);
        let local_config = Config::find_local(&workspace_dir, env_vars);

        let global_virtualenvs_path = global_config.clone().map(|c| c.virtualenvs_path.clone());
        let local_virtualenvs_path = local_config.clone().map(|c| c.virtualenvs_path.clone());
        let virtualenvs_path = local_virtualenvs_path.or(global_virtualenvs_path);

        let global_in_project = global_config.clone().and_then(|c| c.virtualenvs_in_project);
        let local_in_project = local_config.clone().and_then(|c| c.virtualenvs_in_project);
        let virtualenvs_in_project = local_in_project.or(global_in_project);

        let global_cache_dir = global_config.clone().and_then(|c| c.cache_dir.clone());
        let local_cache_dir = local_config.clone().and_then(|c| c.cache_dir.clone());
        let cache_dir = local_cache_dir.or(global_cache_dir);

        let poetry_exe_not_found = envs_discovered_by_us.clone().map(|e| e.managers.is_empty());
        let global_config_not_found = Some(global_config.is_none());
        let envs_discovered_by_us: HashSet<_> = envs_discovered_by_us
            .clone()
            .as_ref()
            .map(|e| e.environments.clone())
            .unwrap_or_default()
            .iter()
            .filter(|e| e.project == Some(workspace_dir.clone()))
            .flat_map(|e| e.prefix.clone())
            .collect();
        let envs_discovered_by_poetry: HashSet<_> = envs_discovered_by_poetry
            .iter()
            .filter(|e| e.project == Some(workspace_dir.clone()))
            .flat_map(|e| e.prefix.clone())
            .collect();

        let missing_envs = envs_discovered_by_poetry
            .difference(&envs_discovered_by_us)
            .collect::<Vec<_>>();
        let missing_path = config.virtualenvs_path.as_ref().map(|path| {
            missing_envs
                .iter()
                .filter(|e| e.starts_with(path))
                .collect::<Vec<_>>()
                .len() as u16
        });

        let mut cache_dir_not_found = None;
        let mut cache_dir_is_different = None;
        if let Some(cache_dir) = cache_dir {
            if let Some(poetry_cache_dir) = config.cache_dir.as_ref() {
                if poetry_cache_dir.exists() && cache_dir != *poetry_cache_dir {
                    cache_dir_is_different = Some(true);
                    warn!(
                        "Poetry cache dir is different from the one we found: {:?} != {:?}",
                        cache_dir, poetry_cache_dir
                    );
                }
            }
        } else {
            cache_dir_not_found = Some(true);
            warn!("Poetry cache dir not found");
        }

        let mut virtualenvs_path_not_found = None;
        let mut virtualenvs_path_is_different = None;
        if let Some(virtualenvs_path) = virtualenvs_path {
            if let Some(poetry_virtualenvs_path) = config.virtualenvs_path {
                if poetry_virtualenvs_path.exists() && virtualenvs_path != *poetry_virtualenvs_path
                {
                    virtualenvs_path_is_different = Some(true);
                    warn!(
                        "Poetry virtualenvs_path is different from the one we found: {:?} != {:?}",
                        virtualenvs_path, poetry_virtualenvs_path
                    );
                }
            }
        } else {
            virtualenvs_path_not_found = Some(true);
            warn!("Poetry virtualenvs_path not found");
        }

        let mut in_project_is_different = None;
        if (virtualenvs_in_project.is_some() || config.virtualenvs_in_project.is_some())
            && virtualenvs_in_project != config.virtualenvs_in_project
        {
            in_project_is_different = Some(true);
            warn!(
                "Poetry virtualenvs.in-project is different from the one we found: {:?} != {:?}",
                virtualenvs_in_project, config.virtualenvs_in_project
            );
        }

        if missing_envs.is_empty() {
            continue;
        }
        warn!(
            "Missing Poetry envs: {:?} for {:?}",
            missing_envs, workspace_dir
        );

        let missing_info = MissingPoetryEnvironments {
            missing: missing_envs.len() as u16,
            missing_in_path: missing_path.unwrap_or_default(),
            user_provided_poetry_exe: Some(user_provided_poetry_exe),
            poetry_exe_not_found,
            global_config_not_found,
            cache_dir_not_found,
            cache_dir_is_different,
            virtualenvs_path_not_found,
            virtualenvs_path_is_different,
            in_project_is_different,
        };

        reporter.report_telemetry(&TelemetryEvent::MissingPoetryEnvironments(missing_info));
    }
    Some(())
}
