use crate::{LspAdapter, LspAdapterDelegate, LspInstaller, ResolvedLanguageServerBinary};
use anyhow::{Context as _, Result};
use fs::{FileLock, FileLockKind, Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::AsyncApp;
use language_core::BinaryStatus;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use util::ResultExt as _;

#[derive(Default, Deserialize, Serialize)]
struct Installations {
    current: Option<String>,
    previous: Option<String>,
}

fn installation_root(container: &Path, pre_release: bool) -> Option<PathBuf> {
    // Older installers scan and sometimes clear their entire download directory.
    // Keep managed installations outside it so that migrating doesn't change their cache lookup.
    Some(
        container
            .parent()?
            .join(".zed-managed-lsp")
            .join(container.file_name()?)
            .join(if pre_release { "pre-release" } else { "stable" }),
    )
}

fn is_generation(name: &str) -> bool {
    name.strip_prefix("installation-").is_some_and(|suffix| {
        !suffix.is_empty()
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'-')
    })
}

fn uses_directory(binary: &ResolvedLanguageServerBinary, directory: &Path) -> bool {
    binary.path.starts_with(directory)
        || binary
            .arguments
            .iter()
            .any(|argument| Path::new(argument).starts_with(directory))
}

fn lease_path(root: &Path, generation: &str) -> PathBuf {
    root.join(format!("{generation}.lease"))
}

async fn read_installations(fs: &dyn Fs, root: &Path) -> Result<Installations> {
    let path = root.join("active.json");
    if !fs.is_file(&path).await {
        return Ok(Installations::default());
    }
    let installations: Installations = serde_json::from_str(&fs.load(&path).await?)?;
    for generation in [&installations.current, &installations.previous]
        .into_iter()
        .flatten()
    {
        anyhow::ensure!(is_generation(generation), "invalid LSP installation name");
    }
    Ok(installations)
}

async fn lease_installation(
    fs: &dyn Fs,
    root: &Path,
    generation: Option<&str>,
) -> Result<Option<(PathBuf, Arc<FileLock>)>> {
    let Some(generation) = generation else {
        return Ok(None);
    };
    let directory = root.join(generation);
    if !fs.is_dir(&directory).await {
        return Ok(None);
    }
    let lease = fs
        .lock_file(&lease_path(root, generation), FileLockKind::Shared)
        .await?;
    Ok(Some((directory, Arc::new(lease))))
}

async fn resolve<I: LspInstaller>(
    installer: &I,
    delegate: &dyn LspAdapterDelegate,
    installation: Option<(PathBuf, Arc<FileLock>)>,
) -> Option<ResolvedLanguageServerBinary> {
    let (directory, lease) = installation?;
    let binary = installer.cached_server_binary(directory, delegate).await?;
    if !delegate.fs().is_file(&binary.path).await {
        return None;
    }
    Some(ResolvedLanguageServerBinary {
        binary,
        installation_lease: Some(lease),
        previous: None,
    })
}

pub(super) async fn cached_binary<I: LspInstaller>(
    installer: &I,
    delegate: &Arc<dyn LspAdapterDelegate>,
    container: &Path,
    pre_release: bool,
) -> Option<ResolvedLanguageServerBinary> {
    let fs = delegate.fs();
    if let Some(root) = installation_root(container, pre_release)
        && fs.is_dir(&root).await
    {
        let leased = async {
            let _lock = fs
                .lock_file(&root.join("leases.lock"), FileLockKind::Exclusive)
                .await?;
            let installations = read_installations(fs.as_ref(), &root).await?;
            let current =
                lease_installation(fs.as_ref(), &root, installations.current.as_deref()).await?;
            let previous =
                lease_installation(fs.as_ref(), &root, installations.previous.as_deref()).await?;
            anyhow::Ok((current, previous))
        }
        .await
        .log_err();
        if let Some((current, previous)) = leased {
            let current = resolve(installer, delegate.as_ref(), current).await;
            let previous = resolve(installer, delegate.as_ref(), previous).await;
            if let Some(mut current) = current {
                current.previous = match previous {
                    Some(previous) => Some(Box::new(previous)),
                    None => installer
                        .cached_server_binary(container.to_path_buf(), delegate.as_ref())
                        .await
                        .map(|binary| Box::new(binary.into())),
                };
                return Some(current);
            }
            if previous.is_some() {
                return previous;
            }
        }
    }
    let binary = installer
        .cached_server_binary(container.to_path_buf(), delegate.as_ref())
        .await?;
    fs.is_file(&binary.path).await.then(|| binary.into())
}

pub(super) async fn update<I: LspInstaller + LspAdapter>(
    installer: &I,
    delegate: &Arc<dyn LspAdapterDelegate>,
    container: &Path,
    pre_release: bool,
    cx: &mut AsyncApp,
) -> Result<ResolvedLanguageServerBinary>
where
    I::BinaryVersion: Send + Sync,
{
    let fs = delegate.fs();
    let root =
        installation_root(container, pre_release).context("invalid LSP download directory")?;
    fs.create_dir(&root).await?;
    // Take this lock before querying the release so that another process's completed
    // installation can be reused. Cached startup never waits for this lock.
    let _update_lock = fs
        .lock_file(&root.join("update.lock"), FileLockKind::Exclusive)
        .await?;
    let installations = read_installations(fs.as_ref(), &root).await?;
    delegate.update_status(installer.name(), BinaryStatus::CheckingForUpdate);
    let version = installer
        .fetch_latest_server_version(delegate, pre_release, cx)
        .await?;
    let version_key = installer.version_key(&version);

    if let Some(generation) = &installations.current {
        let directory = root.join(generation);
        let marker = root.join(format!("{generation}.version"));
        let installed = if let Some(key) = &version_key {
            fs.load(&marker)
                .await
                .is_ok_and(|installed| installed == *key)
        } else {
            cx.background_executor()
                .spawn(installer.check_if_version_installed(&version, &directory, delegate))
                .await
                .is_some()
        };
        if installed {
            if let Some(binary) = cached_binary(installer, delegate, container, pre_release).await
                && uses_directory(&binary, &directory)
            {
                delegate.update_status(installer.name(), BinaryStatus::None);
                cleanup(fs.as_ref(), &root, &installations).await.log_err();
                return Ok(binary);
            }
        }
    } else if version_key.is_none()
        && let Some(binary) = cx
            .background_executor()
            .spawn(installer.check_if_version_installed(
                &version,
                &container.to_path_buf(),
                delegate,
            ))
            .await
    {
        delegate.update_status(installer.name(), BinaryStatus::None);
        cleanup(fs.as_ref(), &root, &installations).await.log_err();
        return Ok(binary.into());
    }

    let mut previous = cached_binary(installer, delegate, container, pre_release).await;
    if let Some(previous) = &mut previous {
        previous.previous = None;
    }
    let generation = format!(
        "installation-{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
        std::process::id()
    );
    let directory = root.join(&generation);
    fs.create_dir(&directory).await?;
    delegate.update_status(installer.name(), BinaryStatus::Downloading);
    let result = cx
        .background_executor()
        .spawn(installer.fetch_server_binary(version, directory.clone(), delegate))
        .await;
    let binary = match result {
        Ok(binary) => binary,
        Err(error) => {
            fs.remove_dir(
                &directory,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .log_err();
            return Err(error);
        }
    };
    // Some installers launch via a runtime (Node, Python); their executable may be
    // outside the installation directory. The adapter's cache lookup must still resolve it.
    let verified = installer
        .cached_server_binary(directory.clone(), delegate.as_ref())
        .await;
    if !fs.is_file(&binary.path).await
        || verified.is_none_or(|verified| {
            verified.path != binary.path || verified.arguments != binary.arguments
        })
    {
        fs.remove_dir(
            &directory,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await
        .log_err();
        anyhow::bail!("installed language server could not be resolved from its cache");
    }
    if let Some(key) = version_key {
        fs.atomic_write(root.join(format!("{generation}.version")), key)
            .await?;
    }
    let previous_generation = previous.as_ref().and_then(|binary| {
        [installations.current, installations.previous]
            .into_iter()
            .flatten()
            .find(|generation| uses_directory(binary, &root.join(generation)))
    });
    let new_installations = Installations {
        current: Some(generation.clone()),
        previous: previous_generation,
    };
    let lease = {
        let _lock = fs
            .lock_file(&root.join("leases.lock"), FileLockKind::Exclusive)
            .await?;
        let lease = fs
            .lock_file(&lease_path(&root, &generation), FileLockKind::Shared)
            .await?;
        fs.atomic_write(
            root.join("active.json"),
            serde_json::to_string(&new_installations)?,
        )
        .await?;
        lease
    };
    delegate.update_status(installer.name(), BinaryStatus::None);
    cleanup(fs.as_ref(), &root, &new_installations)
        .await
        .log_err();
    Ok(ResolvedLanguageServerBinary {
        binary,
        installation_lease: Some(Arc::new(lease)),
        previous: previous.map(Box::new),
    })
}

async fn cleanup(fs: &dyn Fs, root: &Path, installations: &Installations) -> Result<()> {
    let mut retired = Vec::new();
    {
        let _lock = fs
            .lock_file(&root.join("leases.lock"), FileLockKind::Exclusive)
            .await?;
        let mut entries = fs.read_dir(root).await?;
        while let Some(entry) = entries.next().await {
            let path = entry?;
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("retired-installation-") && fs.is_dir(&path).await {
                retired.push(path);
                continue;
            }
            if !is_generation(name)
                || !fs.is_dir(&path).await
                || installations.current.as_deref() == Some(name)
                || installations.previous.as_deref() == Some(name)
            {
                continue;
            }
            let lease_path = lease_path(root, name);
            let Some(lease) = fs
                .try_lock_file(&lease_path, FileLockKind::Exclusive)
                .await?
            else {
                continue;
            };
            let retired_path = root.join(format!("retired-{name}"));
            if fs
                .rename(&path, &retired_path, RenameOptions::default())
                .await
                .log_err()
                .is_some()
            {
                retired.push(retired_path);
                drop(lease);
                fs.remove_file(&lease_path, RemoveOptions::default())
                    .await
                    .log_err();
                fs.remove_file(
                    &root.join(format!("{name}.version")),
                    RemoveOptions {
                        ignore_if_not_exists: true,
                        ..Default::default()
                    },
                )
                .await
                .log_err();
            }
        }
    }
    for path in retired {
        fs.remove_dir(
            &path,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await
        .log_err();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_cleanup_keeps_current_previous_and_leased_installations(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let root = Path::new(util::path!("/installations"));
        fs.create_dir(root).await.unwrap();
        for name in [
            "installation-1",
            "installation-2",
            "installation-3",
            "installation-4",
        ] {
            fs.create_dir(&root.join(name)).await.unwrap();
        }
        let lease = fs
            .lock_file(&lease_path(root, "installation-1"), FileLockKind::Shared)
            .await
            .unwrap();
        let installations = Installations {
            current: Some("installation-4".into()),
            previous: Some("installation-3".into()),
        };
        cleanup(fs.as_ref(), root, &installations).await.unwrap();
        assert!(fs.is_dir(&root.join("installation-1")).await);
        assert!(!fs.is_dir(&root.join("installation-2")).await);
        assert!(fs.is_dir(&root.join("installation-3")).await);
        assert!(fs.is_dir(&root.join("installation-4")).await);
        drop(lease);
        cleanup(fs.as_ref(), root, &installations).await.unwrap();
        assert!(!fs.is_dir(&root.join("installation-1")).await);
    }

    #[gpui::test]
    async fn test_installation_pointer_is_not_changed_by_cleanup(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let root = Path::new(util::path!("/installations"));
        fs.create_dir(root).await.unwrap();
        for name in ["installation-1", "installation-2", "installation-3"] {
            fs.create_dir(&root.join(name)).await.unwrap();
        }
        let installations = Installations {
            current: Some("installation-2".into()),
            previous: Some("installation-1".into()),
        };
        fs.atomic_write(
            root.join("active.json"),
            serde_json::to_string(&installations).unwrap(),
        )
        .await
        .unwrap();
        cleanup(fs.as_ref(), root, &installations).await.unwrap();
        let current = read_installations(fs.as_ref(), root).await.unwrap();
        assert_eq!(current.current, installations.current);
        assert_eq!(current.previous, installations.previous);
        assert!(!fs.is_dir(&root.join("installation-3")).await);
    }

    #[test]
    fn test_generation_names() {
        assert!(is_generation("installation-123-456"));
        assert!(!is_generation("installation-../other"));
        assert!(!is_generation("installation-1.lease"));
    }
}
