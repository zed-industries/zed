//! Real-process checks for standalone system Node discovery.
//!
//! These intentionally non-deterministic tests run fake executables using the
//! real command implementation, not GPUI's deterministic scheduler or an installed
//! Node. Pure lookup and version parsing are covered by the crate's unit tests.

#![cfg(unix)]

use anyhow::{Context as _, Result};
use async_compression::futures::bufread::GzipEncoder;
use futures::AsyncReadExt as _;
use http_client::{FakeHttpClient, Response};
use managed_resources::{ManagedResource, ManagedResourceCommand};
use node_runtime::{
    InstallNpmPackages, NodeBinaryOptions, NodeDiscoveryError, NodeRuntime, Npm, NpmContext,
    SystemNode,
};
use semver::Version;
use std::{
    env, fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};

#[test]
fn configured_runtime_keeps_node_available_when_npm_is_missing() -> Result<()> {
    if env::var_os(CHILD).is_some() {
        let directory = env::current_dir()?;
        paths::set_custom_data_dir(&directory.join("data").to_string_lossy());
        let node = directory.join("node");
        let npm = directory.join("npm");
        return smol::block_on(async {
            for configured_npm in [PathBuf::from("npm"), PathBuf::from("./npm"), npm.clone()] {
                let runtime = NodeRuntime::new(
                    Arc::new(http_client::BlockedHttpClient),
                    None,
                    watch::channel(Some(NodeBinaryOptions {
                        use_paths: Some((node.clone(), configured_npm)),
                        ..Default::default()
                    }))
                    .1,
                );
                assert_eq!(runtime.binary_path().await?, node);
                let error = runtime
                    .npm_command(None, "info", &[])
                    .await
                    .expect_err("npm is missing");
                assert!(error.to_string().contains("finding npm executable"));
                assert_eq!(runtime.binary_path().await?, node);

                fs::write(&npm, "#!/bin/sh\npwd -P\n")?;
                fs::set_permissions(&npm, fs::Permissions::from_mode(0o755))?;
                let command = runtime.npm_command(None, "info", &[]).await?;
                assert_eq!(command.path, npm);
                let output = runtime.run_npm_subcommand(None, "info", &[]).await?;
                assert_eq!(
                    String::from_utf8(output.stdout)?.trim(),
                    fs::canonicalize(command.current_dir)?.to_string_lossy(),
                );
                fs::remove_file(&npm)?;
            }
            Ok(())
        });
    }

    let directory = tempfile::tempdir()?;
    fake_node(directory.path(), "printf 'v22.0.0\\n'")?;
    run_child_test(
        "configured_runtime_keeps_node_available_when_npm_is_missing",
        directory.path(),
        directory.path(),
    )
}

#[test]
fn managed_npm_obtains_and_installs_packages() -> Result<()> {
    if env::var_os(CHILD).is_some() {
        let directory = env::current_dir()?;
        paths::set_custom_data_dir(&directory.join("data").to_string_lossy());
        return smol::block_on(async {
            let npm_path = directory.join("npm");
            let key = "example-language-server".to_owned();
            assert_eq!(Npm::id(&key).key, key);
            assert_ne!(Npm::id(&key), Npm::id(&"another-language-server".into()));
            let mut context = NpmContext {
                http: Arc::new(http_client::BlockedHttpClient),
                options: NodeBinaryOptions {
                    use_paths: Some((directory.join("node"), npm_path.clone())),
                    ..Default::default()
                },
            };
            assert!(Npm::obtain(&key, &context).await.is_err());

            fs::write(
                &npm_path,
                "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.args\"\npwd -P > \"$0.cwd\"\n",
            )?;
            fs::set_permissions(&npm_path, fs::Permissions::from_mode(0o755))?;
            let npm = Arc::new(Npm::obtain(&key, &context).await?);
            let arguments = fs::read_to_string(directory.join("npm.args"))?;
            assert!(arguments.lines().any(|argument| argument == "--version"));

            let options = std::mem::take(&mut context.options);
            assert!(Npm::obtain(&key, &context).await.is_err());
            let installation_directory = directory.join("language server");
            fs::create_dir(&installation_directory)?;
            let install = || InstallNpmPackages {
                directory: installation_directory.clone(),
                packages: vec![
                    ("@example/server".into(), "1.2.3".into()),
                    ("another-package".into(), "4.5.6".into()),
                ],
            };
            install().run(npm.clone()).await?;
            let arguments = fs::read_to_string(directory.join("npm.args"))?;
            let arguments = arguments.lines().collect::<Vec<_>>();
            assert!(
                arguments.starts_with(&[
                    "--prefix",
                    installation_directory.to_str().expect("UTF-8 test path"),
                    "install",
                ]),
                "{arguments:?}"
            );
            assert!(
                arguments.windows(2).any(
                    |arguments| arguments == ["@example/server@1.2.3", "another-package@4.5.6"]
                ),
                "{arguments:?}"
            );
            assert_eq!(
                fs::read_to_string(directory.join("npm.cwd"))?.trim(),
                fs::canonicalize(&installation_directory)?.to_string_lossy()
            );

            fs::write(
                &npm_path,
                "#!/bin/sh\necho 'installation failed' >&2\nexit 7\n",
            )?;
            let error = install()
                .run(npm)
                .await
                .expect_err("npm failure propagates");
            assert!(error.to_string().contains("installation failed"));
            context.options = options;
            assert!(Npm::obtain(&key, &context).await.is_err());
            Ok(())
        });
    }

    let directory = tempfile::tempdir()?;
    fake_node(directory.path(), "printf 'v22.0.0\\n'")?;
    run_child_test(
        "managed_npm_obtains_and_installs_packages",
        directory.path(),
        directory.path(),
    )
}

#[test]
fn managed_npm_downloads_only_when_allowed() -> Result<()> {
    if env::var_os(CHILD).is_some() {
        let directory = env::current_dir()?;
        let data_directory = directory.join("data");
        fs::create_dir(&data_directory)?;
        paths::set_custom_data_dir(&data_directory.to_string_lossy());
        return smol::block_on(async {
            let downloads = Arc::new(AtomicUsize::new(0));
            let http = FakeHttpClient::create({
                let downloads = downloads.clone();
                move |request| {
                    downloads.fetch_add(1, SeqCst);
                    async move {
                        assert_eq!(request.uri().host(), Some("nodejs.org"));
                        let folder = request
                            .uri()
                            .path()
                            .rsplit('/')
                            .next()
                            .and_then(|name| name.strip_suffix(".tar.gz"))
                            .context("expected a Node tarball request")?;
                        let mut archive = async_tar::Builder::new(Vec::new());
                        for (path, contents) in [
                            ("bin/node", "#!/bin/sh\nexec /bin/sh \"$@\"\n"),
                            (
                                "bin/npm",
                                "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.args\"\n",
                            ),
                        ] {
                            let mut header = async_tar::Header::new_gnu();
                            header.set_size(contents.len() as u64);
                            header.set_mode(0o755);
                            archive
                                .append_data(
                                    &mut header,
                                    format!("{folder}/{path}"),
                                    contents.as_bytes(),
                                )
                                .await?;
                        }
                        let archive = archive.into_inner().await?;
                        let mut compressed = Vec::new();
                        GzipEncoder::new(archive.as_slice())
                            .read_to_end(&mut compressed)
                            .await?;
                        Ok(Response::new(compressed.into()))
                    }
                }
            });
            let key = "example-language-server".to_owned();
            let mut context = NpmContext {
                http,
                options: NodeBinaryOptions::default(),
            };
            assert!(Npm::obtain(&key, &context).await.is_err());
            context.options.allow_path_lookup = true;
            assert!(Npm::obtain(&key, &context).await.is_err());
            assert_eq!(downloads.load(SeqCst), 0);
            assert!(!data_directory.join("node").exists());

            context.options = NodeBinaryOptions {
                allow_binary_download: true,
                ..Default::default()
            };
            let npm = Arc::new(Npm::obtain(&key, &context).await?);
            let installation = fs::read_dir(data_directory.join("node"))?
                .next()
                .transpose()?
                .context("expected an installed Node runtime")?
                .path();
            let arguments = fs::read_to_string(installation.join("bin/npm.args"))?;
            assert!(arguments.lines().any(|argument| argument == "--version"));
            let package_directory = directory.join("language server");
            fs::create_dir(&package_directory)?;
            InstallNpmPackages {
                directory: package_directory,
                packages: vec![("@example/server".into(), "1.2.3".into())],
            }
            .run(npm)
            .await?;
            let arguments = fs::read_to_string(installation.join("bin/npm.args"))?;
            assert!(arguments.lines().any(|argument| argument == "install"));
            assert!(
                arguments
                    .lines()
                    .any(|argument| argument == "@example/server@1.2.3")
            );

            Npm::obtain(&key, &context).await?;
            assert_eq!(downloads.load(SeqCst), 1);
            context.options = NodeBinaryOptions::default();
            assert!(Npm::obtain(&key, &context).await.is_err());
            assert_eq!(downloads.load(SeqCst), 1);
            Ok(())
        });
    }

    let directory = tempfile::tempdir()?;
    run_child_test(
        "managed_npm_downloads_only_when_allowed",
        directory.path(),
        directory.path(),
    )
}

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
