// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod common;

#[test]
fn sync_refresh_state_full_replaces_all_caches() {
    use common::create_test_environment;
    use pet_conda::Conda;
    use pet_core::{
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Locator, RefreshStateSyncScope,
    };
    use std::{collections::HashMap, path::PathBuf};

    let env = create_test_environment(HashMap::new(), None, vec![], None);
    let shared = Conda::from(&env);
    let transient = Conda::from(&env);

    // Populate shared with two environments from a "previous" refresh.
    let env_a = PythonEnvironment::new(
        Some(PathBuf::from("/envs/a/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/a")),
        None,
        Some("3.10.0".into()),
    );
    let env_b = PythonEnvironment::new(
        Some(PathBuf::from("/envs/b/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/b")),
        None,
        Some("3.11.0".into()),
    );
    shared
        .environments
        .insert(PathBuf::from("/envs/a"), env_a.clone());
    shared
        .environments
        .insert(PathBuf::from("/envs/b"), env_b.clone());

    // Transient only discovered env_a (e.g. env_b was deleted).
    transient
        .environments
        .insert(PathBuf::from("/envs/a"), env_a);

    // Full sync should replace: shared loses env_b.
    shared.sync_refresh_state_from(&transient, &RefreshStateSyncScope::Full);

    assert_eq!(shared.environments.len(), 1);
    assert!(shared.environments.get(&PathBuf::from("/envs/a")).is_some());
    assert!(shared.environments.get(&PathBuf::from("/envs/b")).is_none());
}

#[test]
fn sync_refresh_state_global_filtered_merges_caches() {
    use common::create_test_environment;
    use pet_conda::{manager::CondaManager, Conda};
    use pet_core::{
        manager::EnvManagerType,
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Locator, RefreshStateSyncScope,
    };
    use std::{collections::HashMap, path::PathBuf};

    let env = create_test_environment(HashMap::new(), None, vec![], None);
    let shared = Conda::from(&env);
    let transient = Conda::from(&env);

    // Populate shared with two environments from a "previous" refresh.
    let env_a = PythonEnvironment::new(
        Some(PathBuf::from("/envs/a/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/a")),
        None,
        Some("3.10.0".into()),
    );
    let env_b = PythonEnvironment::new(
        Some(PathBuf::from("/envs/b/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/b")),
        None,
        Some("3.11.0".into()),
    );
    shared.environments.insert(PathBuf::from("/envs/a"), env_a);
    shared.environments.insert(PathBuf::from("/envs/b"), env_b);

    // Also populate shared with a manager.
    let mgr_old = CondaManager {
        executable: PathBuf::from("/conda/bin/conda"),
        version: Some("23.0.0".into()),
        conda_dir: Some(PathBuf::from("/conda")),
        manager_type: EnvManagerType::Conda,
    };
    shared.managers.insert(PathBuf::from("/conda"), mgr_old);

    // Transient discovered env_a (updated) and a new env_c, plus an updated manager.
    let env_a_updated = PythonEnvironment::new(
        Some(PathBuf::from("/envs/a/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/a")),
        None,
        Some("3.10.1".into()),
    );
    let env_c = PythonEnvironment::new(
        Some(PathBuf::from("/envs/c/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/c")),
        None,
        Some("3.12.0".into()),
    );
    transient
        .environments
        .insert(PathBuf::from("/envs/a"), env_a_updated);
    transient
        .environments
        .insert(PathBuf::from("/envs/c"), env_c);

    let mgr_new = CondaManager {
        executable: PathBuf::from("/conda/bin/conda"),
        version: Some("24.0.0".into()),
        conda_dir: Some(PathBuf::from("/conda")),
        manager_type: EnvManagerType::Conda,
    };
    transient.managers.insert(PathBuf::from("/conda"), mgr_new);

    // GlobalFiltered(Conda) should merge: shared keeps env_b, updates env_a, adds env_c.
    shared.sync_refresh_state_from(
        &transient,
        &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Conda),
    );

    assert_eq!(shared.environments.len(), 3);
    assert_eq!(
        shared
            .environments
            .get(&PathBuf::from("/envs/a"))
            .unwrap()
            .version,
        Some("3.10.1".into())
    );
    assert!(shared.environments.get(&PathBuf::from("/envs/b")).is_some());
    assert!(shared.environments.get(&PathBuf::from("/envs/c")).is_some());

    // Manager cache should also be merged (updated, not cleared).
    assert_eq!(shared.managers.len(), 1);
    assert_eq!(
        shared
            .managers
            .get(&PathBuf::from("/conda"))
            .unwrap()
            .version,
        Some("24.0.0".into())
    );
}

#[test]
fn sync_refresh_state_full_then_global_filtered_preserves_entries() {
    use common::create_test_environment;
    use pet_conda::Conda;
    use pet_core::{
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Locator, RefreshStateSyncScope,
    };
    use std::{collections::HashMap, path::PathBuf};

    let env = create_test_environment(HashMap::new(), None, vec![], None);
    let shared = Conda::from(&env);

    // Step 1: Full refresh populates shared with A and B.
    let transient_full = Conda::from(&env);
    transient_full.environments.insert(
        PathBuf::from("/envs/a"),
        PythonEnvironment::new(
            Some(PathBuf::from("/envs/a/bin/python")),
            Some(PythonEnvironmentKind::Conda),
            Some(PathBuf::from("/envs/a")),
            None,
            Some("3.10.0".into()),
        ),
    );
    transient_full.environments.insert(
        PathBuf::from("/envs/b"),
        PythonEnvironment::new(
            Some(PathBuf::from("/envs/b/bin/python")),
            Some(PythonEnvironmentKind::Conda),
            Some(PathBuf::from("/envs/b")),
            None,
            Some("3.11.0".into()),
        ),
    );
    shared.sync_refresh_state_from(&transient_full, &RefreshStateSyncScope::Full);
    assert_eq!(shared.environments.len(), 2);

    // Step 2: GlobalFiltered refresh discovers only C; A and B must survive.
    let transient_filtered = Conda::from(&env);
    transient_filtered.environments.insert(
        PathBuf::from("/envs/c"),
        PythonEnvironment::new(
            Some(PathBuf::from("/envs/c/bin/python")),
            Some(PythonEnvironmentKind::Conda),
            Some(PathBuf::from("/envs/c")),
            None,
            Some("3.12.0".into()),
        ),
    );
    shared.sync_refresh_state_from(
        &transient_filtered,
        &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Conda),
    );

    assert_eq!(shared.environments.len(), 3);
    assert!(shared.environments.get(&PathBuf::from("/envs/a")).is_some());
    assert!(shared.environments.get(&PathBuf::from("/envs/b")).is_some());
    assert!(shared.environments.get(&PathBuf::from("/envs/c")).is_some());
}

#[test]
fn sync_refresh_state_irrelevant_scope_is_noop() {
    use common::create_test_environment;
    use pet_conda::Conda;
    use pet_core::{
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Locator, RefreshStateSyncScope,
    };
    use std::{collections::HashMap, path::PathBuf};

    let env = create_test_environment(HashMap::new(), None, vec![], None);
    let shared = Conda::from(&env);
    let transient = Conda::from(&env);

    let env_a = PythonEnvironment::new(
        Some(PathBuf::from("/envs/a/bin/python")),
        Some(PythonEnvironmentKind::Conda),
        Some(PathBuf::from("/envs/a")),
        None,
        Some("3.10.0".into()),
    );
    shared.environments.insert(PathBuf::from("/envs/a"), env_a);

    // Workspace and unrelated GlobalFiltered should not touch conda caches.
    shared.sync_refresh_state_from(&transient, &RefreshStateSyncScope::Workspace);
    assert_eq!(shared.environments.len(), 1);

    shared.sync_refresh_state_from(
        &transient,
        &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Poetry),
    );
    assert_eq!(shared.environments.len(), 1);
}

#[cfg(unix)]
#[test]
fn find_conda_env_without_manager() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::Conda;
    use pet_core::{
        self, arch::Architecture, env::PythonEnv, python_environment::PythonEnvironmentKind,
        Locator,
    };
    use std::collections::HashMap;

    let environment = create_test_environment(HashMap::new(), None, vec![], None);
    let locator = Conda::from(&environment);
    let path = resolve_test_path(&["unix", "conda_env_without_manager", "env_python_3"]);

    let env = locator
        .try_from(&PythonEnv::new(
            path.join("bin").join("python"),
            Some(path.clone()),
            None,
        ))
        .unwrap();

    assert_eq!(env.prefix, path.clone().into());
    assert_eq!(env.arch, Architecture::X64.into());
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, path.join("bin").join("python").into());
    assert_eq!(env.version, "3.12.2".to_string().into());
    assert_eq!(env.manager, None);
    assert_eq!(env.name, "env_python_3".to_string().into());
}

#[cfg(unix)]
#[test]
fn find_conda_env_without_manager_but_detect_manager_from_history() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::Conda;
    use pet_core::{
        self, arch::Architecture, env::PythonEnv, python_environment::PythonEnvironmentKind,
        Locator,
    };
    use std::{
        collections::HashMap,
        fs::{self},
    };

    let environment = create_test_environment(HashMap::new(), None, vec![], None);
    let locator = Conda::from(&environment);
    let path = resolve_test_path(&["unix", "conda_hist", "env_python_3"]);
    let conda_dir =
        resolve_test_path(&["unix", "conda_hist", "some_other_location", "conda_install"]);
    let history_file = path.join("conda-meta").join("history");
    let history_file_template = path.join("conda-meta").join("history_template");
    let history_contents = fs::read_to_string(&history_file_template)
        .unwrap()
        .replace("<CONDA_INSTALL>", conda_dir.to_str().unwrap_or_default());
    fs::write(history_file, history_contents).unwrap();

    let env = locator
        .try_from(&PythonEnv::new(
            path.join("bin").join("python"),
            Some(path.clone()),
            None,
        ))
        .unwrap();

    assert_eq!(env.prefix, path.clone().into());
    assert_eq!(env.arch, Architecture::X64.into());
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, path.join("bin").join("python").into());
    assert_eq!(env.version, "3.12.2".to_string().into());
    assert_eq!(
        env.manager.clone().unwrap().executable,
        conda_dir.join("bin").join("conda")
    );
    assert_eq!(
        env.manager.clone().unwrap().version,
        "23.1.0".to_string().into()
    );
    assert_eq!(env.name, None);
}
