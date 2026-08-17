// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod common;

#[cfg(unix)]
#[cfg_attr(any(feature = "ci",), test)]
#[allow(dead_code)]
fn global_config_with_defaults() {
    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_poetry::config::Config;
    use pet_python_utils::platform_dirs::Platformdirs;

    let root = resolve_test_path(&["unix", "global_config_defaults", "root_empty"]);
    let home = resolve_test_path(&["unix", "global_config_defaults", "user_home"]);
    let mut env = create_env_variables(home, root);
    env.poetry_config_dir = Some(resolve_test_path(&[
        "unix",
        "global_config_defaults",
        "user_home",
        "config_dir",
    ]));

    let config = Config::find_global(&env);

    assert!(config.clone().is_some());
    assert_eq!(config.clone().unwrap().file, None);
    assert!(config.clone().unwrap().virtualenvs_in_project.is_none());
    assert_eq!(
        config.clone().unwrap().virtualenvs_path,
        Platformdirs::new("pypoetry".into(), false)
            .user_cache_path()
            .unwrap()
            .join("virtualenvs")
    );
}

#[cfg(unix)]
#[cfg_attr(any(feature = "ci",), test)]
#[allow(dead_code)]
fn global_config_with_specific_values() {
    use std::path::PathBuf;

    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_poetry::config::Config;

    let root = resolve_test_path(&["unix", "global_config_with_values", "root_empty"]);
    let home = resolve_test_path(&["unix", "global_config_with_values", "user_home"]);
    let mut env = create_env_variables(home, root);
    env.poetry_config_dir = Some(resolve_test_path(&[
        "unix",
        "global_config_with_values",
        "user_home",
        "config_dir",
    ]));

    let config = Config::find_global(&env);

    assert!(config.clone().is_some());
    assert_eq!(
        config.clone().unwrap().file,
        Some(resolve_test_path(&[
            "unix",
            "global_config_with_values",
            "user_home",
            "config_dir",
            "config.toml"
        ]))
    );
    assert!(config
        .clone()
        .unwrap()
        .virtualenvs_in_project
        .unwrap_or_default());
    assert_eq!(
        config.clone().unwrap().virtualenvs_path,
        PathBuf::from("some/path/virtualenvs".to_string())
    );
}

#[cfg(unix)]
#[cfg_attr(any(feature = "ci",), test)]
#[allow(dead_code)]
fn local_config_with_specific_values() {
    use std::path::PathBuf;

    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_poetry::config::Config;

    let root = resolve_test_path(&["unix", "local_config_with_values", "root_empty"]);
    let home = resolve_test_path(&["unix", "local_config_with_values", "user_home"]);
    let mut env = create_env_variables(home, root);
    env.poetry_config_dir = Some(resolve_test_path(&[
        "unix",
        "local_config_with_values",
        "user_home",
        "config_dir",
    ]));

    let project_dir = resolve_test_path(&["unix", "local_config_with_values", "project_dir"]);
    let config = Config::find_local(&project_dir, &env);

    assert!(config.clone().is_some());
    assert_eq!(
        config.clone().unwrap().file,
        Some(resolve_test_path(&[
            "unix",
            "local_config_with_values",
            "project_dir",
            "poetry.toml"
        ]))
    );
    assert!(!config
        .clone()
        .unwrap()
        .virtualenvs_in_project
        .unwrap_or_default());
    assert_eq!(
        config.clone().unwrap().virtualenvs_path,
        PathBuf::from("/directory/virtualenvs".to_string())
    );
}
