// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod common;

#[cfg(unix)]
#[test]
fn no_conda_rc() {
    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_conda::conda_rc::Condarc;

    let root = resolve_test_path(&["unix", "root_empty"]);
    let home = resolve_test_path(&["unix", "user_home_with_environments_txt"]);
    let env = create_env_variables(home, root);

    let conda_rc = Condarc::from(&env);

    assert!(conda_rc.is_none());
}

#[cfg(unix)]
#[test]
fn finds_conda_rc() {
    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_conda::conda_rc::Condarc;
    use std::path::PathBuf;

    let root = resolve_test_path(&["unix", "conda_rc", "root"]);
    let home = resolve_test_path(&["unix", "conda_rc", "user_home"]);
    let env = create_env_variables(home, root);

    let conda_rc = Condarc::from(&env).unwrap();

    assert_eq!(
        conda_rc.env_dirs,
        vec![
            PathBuf::from("/Users/donjayamanne/temp/sample-conda-envs-folder2"),
            PathBuf::from("/Users/donjayamanne/temp/sample-conda-envs-folder")
        ]
    );
}

#[cfg(unix)]
#[test]
fn finds_conda_rc_from_conda_root_env_variable() {
    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_conda::conda_rc::Condarc;
    use std::path::PathBuf;

    let root = resolve_test_path(&["unix", "conda_rc_conda_root_var", "root"]);
    let home = resolve_test_path(&["unix", "conda_rc_conda_root_var", "user_home"]);
    let mut env = create_env_variables(home, root);
    env.conda_root = Some(
        resolve_test_path(&[
            "unix",
            "conda_rc_conda_root_var",
            "user_home",
            "conda_root_variable_path",
        ])
        .to_str()
        .unwrap_or_default()
        .to_string(),
    );

    let conda_rc = Condarc::from(&env).unwrap();

    assert!(conda_rc.env_dirs.contains(&PathBuf::from(
        "/Users/donjayamanne/sample-conda-envs-folder2-from_conda_root"
    )));
    assert!(conda_rc.env_dirs.contains(&PathBuf::from(
        "/Users/donjayamanne/sample-conda-envs-folder-from_conda_root"
    )));
}

#[cfg(unix)]
#[test]
fn finds_conda_rc_from_root() {
    use common::create_env_variables;
    use common::resolve_test_path;
    use pet_conda::conda_rc::Condarc;
    use std::path::PathBuf;

    let root = resolve_test_path(&["unix", "conda_rc_root", "root"]);
    let home = resolve_test_path(&["unix", "conda_rc_root", "user_home"]);
    let env = create_env_variables(home, root);

    let conda_rc = Condarc::from(&env).unwrap();

    assert!(conda_rc
        .env_dirs
        .contains(&PathBuf::from("/Users/donjayamanne/root-folder2")));
    assert!(conda_rc
        .env_dirs
        .contains(&PathBuf::from("/Users/donjayamanne/root-folder")));
}
