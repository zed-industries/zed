//! Real-process checks for standalone system Node discovery.
//!
//! These intentionally non-deterministic tests run fake executables using the
//! real command implementation, not GPUI's deterministic scheduler or an installed
//! Node. Pure lookup and version parsing are covered by the crate's unit tests.

#![cfg(unix)]

use anyhow::Result;
use node_runtime::{NodeDiscoveryError, SystemNode};
use semver::Version;
use std::{
    env, fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::{Path, PathBuf},
};

#[test]
fn discovers_node_without_npm_and_passes_the_search_path_to_the_probe() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let target = fake_node(
        &directory.path().join("target"),
        r#"
test "$1" = "--version" || exit 1
test "$PATH/node" = "$0" || exit 2
printf 'v22.0.0\n'
"#,
    )?;
    let node = directory.path().join("node");
    symlink(target, &node)?;
    let discovered = smol::block_on(SystemNode::discover(
        None,
        Some(directory.path().as_os_str().to_owned()),
    ))?;
    assert_eq!(discovered.path(), node);
    assert_eq!(discovered.version(), &Version::new(22, 0, 0));
    Ok(())
}

#[test]
fn empty_path_entry_runs_the_node_in_the_working_directory() -> Result<()> {
    if env::var_os(CHILD).is_some() {
        let discovered = smol::block_on(SystemNode::discover(None, Some("".into())))?;
        assert_eq!(discovered.version(), &Version::new(22, 0, 0));
        assert_eq!(discovered.path(), env::current_dir()?.join("node"));
        for search_path in [
            None,
            Some(env::current_dir()?.join("missing").into_os_string()),
        ] {
            assert!(matches!(
                smol::block_on(SystemNode::discover(None, search_path)),
                Err(NodeDiscoveryError::NotFound { .. })
            ));
        }
        return Ok(());
    }

    let directory = tempfile::tempdir()?;
    fake_node(directory.path(), "printf 'v22.0.0\\n'")?;
    let inherited_path = directory.path().join("other node");
    fake_node(&inherited_path, "printf 'v24.0.0\\n'")?;
    run_child_test(
        "empty_path_entry_runs_the_node_in_the_working_directory",
        directory.path(),
        &inherited_path,
    )
}

#[test]
fn absolute_node_paths_work_after_working_directory_is_removed() -> Result<()> {
    if env::var_os(CHILD).is_some() {
        let directory = env::current_dir()?;
        let node = directory.join("node");
        let removed_directory = directory.join("removed");
        fs::create_dir(&removed_directory)?;
        env::set_current_dir(&removed_directory)?;
        fs::remove_dir(&removed_directory)?;
        for (configured_path, search_path) in [
            (Some(node.clone()), None),
            (None, Some(directory.into_os_string())),
        ] {
            let discovered = smol::block_on(SystemNode::discover(configured_path, search_path))?;
            assert_eq!(discovered.path(), node);
            assert_eq!(discovered.version(), &Version::new(22, 0, 0));
        }
        return Ok(());
    }

    let directory = tempfile::tempdir()?;
    fake_node(directory.path(), "printf 'v22.0.0\\n'")?;
    run_child_test(
        "absolute_node_paths_work_after_working_directory_is_removed",
        directory.path(),
        directory.path(),
    )
}

#[test]
fn configured_node_takes_precedence_and_failed_overrides_do_not_fall_back() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let node = fake_node(&directory.path().join("path node"), "printf 'v22.0.0\\n'")?;
    let configured = fake_node(
        &directory.path().join("configured node"),
        "printf 'v24.1.0\\n'",
    )?;
    let search_path = node
        .parent()
        .expect("node directory")
        .as_os_str()
        .to_owned();
    let discovered = smol::block_on(SystemNode::discover(
        Some(configured.clone()),
        Some(search_path.clone()),
    ))?;
    assert_eq!(discovered.path(), configured);
    assert_eq!(discovered.version(), &Version::new(24, 1, 0));

    fake_node(
        configured.parent().expect("configured directory"),
        "printf 'v20.0.0\\n'",
    )?;
    assert!(matches!(
        smol::block_on(SystemNode::discover(
            Some(configured.clone()),
            Some(search_path.clone())
        )),
        Err(NodeDiscoveryError::TooOld { path, .. }) if path == configured
    ));
    fs::remove_file(&configured)?;
    assert!(matches!(
        smol::block_on(SystemNode::discover(Some(configured), Some(search_path))),
        Err(NodeDiscoveryError::NotFound { .. })
    ));
    Ok(())
}

#[test]
fn path_order_is_preserved_when_the_first_node_is_too_old() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let first = directory.path().join("first");
    let second = directory.path().join("second");
    let old = fake_node(&first, "printf 'v21.7.3\\n'")?;
    fake_node(&second, "printf 'v24.0.0\\n'")?;
    assert!(matches!(
        smol::block_on(SystemNode::discover(None, Some(env::join_paths([first, second])?))),
        Err(NodeDiscoveryError::TooOld { path, .. }) if path == old
    ));
    Ok(())
}

#[test]
fn reports_execution_failures_with_the_executable_path() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let node = fake_node(
        directory.path(),
        "printf 'v22.0.0\\n'; printf 'broken installation' >&2; exit 7",
    )?;
    let error =
        smol::block_on(SystemNode::discover(Some(node.clone()), None)).expect_err("nonzero exit");
    assert!(
        matches!(&error, NodeDiscoveryError::UnsuccessfulExit { path, output }
        if path == &node && output.status.code() == Some(7))
    );
    assert!(error.to_string().contains("broken installation"));

    fs::write(&node, b"#!/nonexistent-node-interpreter\n")?;
    assert!(matches!(
        smol::block_on(SystemNode::discover(Some(node.clone()), None)),
        Err(NodeDiscoveryError::Probe { path, .. }) if path == node
    ));
    Ok(())
}

const CHILD: &str = "ZED_TEST_NODE_DISCOVERY_CHILD";

/// Isolates cwd and inherited PATH changes from the other tests.
fn run_child_test(test: &str, directory: &Path, inherited_path: &Path) -> Result<()> {
    let output = smol::block_on(
        util::command::new_command(env::current_exe()?)
            .args(["--exact", test, "--nocapture"])
            .current_dir(directory)
            .env(CHILD, "1")
            .env("PATH", inherited_path)
            .output(),
    )?;
    assert!(output.status.success(), "{output:?}");
    Ok(())
}

fn fake_node(directory: &Path, body: &str) -> Result<PathBuf> {
    fs::create_dir_all(directory)?;
    let path = directory.join("node");
    fs::write(&path, format!("#!/bin/sh\n{body}\n"))?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
    Ok(path)
}
