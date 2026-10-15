use anyhow::{Context as _, Result, ensure};
use fs::Fs;
use settings::{RegisterSetting, Settings};
use std::path::{Path, PathBuf};
use util::command::new_command;

#[derive(Clone, RegisterSetting)]
pub struct ArcGitSettings {
    pub executable: Option<String>,
}

impl Settings for ArcGitSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let git = content.git.as_ref();
        Self {
            // Executables come only from user/server settings, never project settings.
            executable: git
                .filter(|git| {
                    git.enabled.as_ref().is_none_or(|enabled| {
                        enabled.is_git_status_enabled() || enabled.is_git_diff_enabled()
                    })
                })
                .and_then(|git| git.arc_executable.clone()),
        }
    }
}

pub async fn find_root(fs: &dyn Fs, directory: &Path) -> Result<Option<PathBuf>> {
    for ancestor in directory.ancestors() {
        let dot_git = ancestor.join(".git");
        let metadata = fs.metadata(&dot_git).await?;
        let owned = metadata.as_ref().is_some_and(|metadata| metadata.is_dir)
            && fs.is_file(&dot_git.join(".arc2git-zed")).await;
        // A real Git repository takes precedence over any enclosing Arc mount.
        if metadata.is_some() && !owned {
            return Ok(None);
        }
        if owned
            || fs
                .metadata(&ancestor.join(".arcadia.root"))
                .await?
                .is_some()
            || fs
                .metadata(&ancestor.join(".cloudia.root"))
                .await?
                .is_some()
        {
            return Ok(Some(ancestor.to_path_buf()));
        }
    }
    Ok(None)
}

fn expand_executable(executable: &str) -> Result<PathBuf> {
    let path = if let Some(relative) = executable.strip_prefix("~/") {
        paths::home_dir().join(relative)
    } else {
        PathBuf::from(executable)
    };
    ensure!(
        path.is_absolute(),
        "git.arc_executable must be an absolute path or start with ~/"
    );
    Ok(path)
}

pub async fn executable_for_directory(
    fs: &dyn Fs,
    directory: &Path,
    configured: Option<&str>,
) -> Result<Option<PathBuf>> {
    let Some(configured) = configured else {
        return Ok(None);
    };
    if find_root(fs, directory).await?.is_none() {
        return Ok(None);
    }
    let executable = expand_executable(configured)?;
    let metadata = fs.metadata(&executable).await?;
    ensure!(
        metadata.is_some_and(|metadata| !metadata.is_dir && metadata.is_executable),
        "Arc Git adapter is not executable: {}. Check git.arc_executable in user/server settings",
        executable.display()
    );
    Ok(Some(executable))
}

pub async fn initialize(fs: &dyn Fs, directory: &Path, configured: Option<&str>) -> Result<()> {
    let Some(executable) = executable_for_directory(fs, directory, configured).await? else {
        return Ok(());
    };
    let root = find_root(fs, directory)
        .await?
        .context("Arc repository disappeared before initialization")?;
    // Ancestor discovery runs before GitStore exists. Bootstrap here so that
    // the first scan can see the adapter's lazily-created repository metadata.
    let output = new_command(&executable)
        .arg("-C")
        .arg(directory)
        .arg("--version")
        .current_dir(directory)
        .output()
        .await
        .with_context(|| format!("starting Arc Git adapter {}", executable.display()))?;
    ensure!(
        output.status.success(),
        "Arc Git adapter {} failed ({}): {}",
        executable.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr).trim()
    );
    ensure!(
        fs.is_file(&root.join(".git/.arc2git-zed")).await
            && fs.is_file(&root.join(".git/HEAD")).await,
        "Arc Git adapter {} did not initialize {}. Expected .git/.arc2git-zed and .git/HEAD",
        executable.display(),
        root.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;

    #[cfg(unix)]
    #[gpui::test]
    async fn distinguishes_arc_mounts_and_real_git_boundaries(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/repos", json!({
            "arc": {
                ".arcadia.root": "",
                "project": {},
                "nested": {".git": {}, "project": {}},
                "linked": {".git": "gitdir: /elsewhere", "project": {}}
            },
            "cloud": {".cloudia.root": "", "project": {}},
            "mixed": {".arcadia.root": "", ".git": {}, "project": {}},
            "owned": {".git": {".arc2git-zed": "", "HEAD": "ref: refs/heads/main"}, "project": {}},
            "git": {".git": {}, "project": {}},
            "plain": {}
        })).await;
        for (directory, expected) in [
            ("/repos/arc/project", Some("/repos/arc")),
            ("/repos/arc", Some("/repos/arc")),
            ("/repos/cloud/project", Some("/repos/cloud")),
            ("/repos/owned/project", Some("/repos/owned")),
            ("/repos/arc/nested/project", None),
            ("/repos/arc/linked/project", None),
            ("/repos/git/project", None),
            ("/repos/mixed/project", None),
            ("/repos/plain", None),
        ] {
            assert_eq!(
                find_root(fs.as_ref(), Path::new(directory)).await.unwrap(),
                expected.map(PathBuf::from),
                "{directory}"
            );
        }
        assert!(
            executable_for_directory(
                fs.as_ref(),
                Path::new("/repos/arc/project"),
                Some("/missing/arc2git-zed")
            )
            .await
            .is_err()
        );
        assert!(
            executable_for_directory(
                fs.as_ref(),
                Path::new("/repos/git/project"),
                Some("/missing/arc2git-zed")
            )
            .await
            .unwrap()
            .is_none()
        );
        initialize(fs.as_ref(), Path::new("/repos/arc/project"), None)
            .await
            .unwrap();
        assert!(!fs.is_dir(Path::new("/repos/arc/.git")).await);
    }

    #[test]
    fn executable_paths_are_explicit_and_expand_remote_home() {
        assert!(expand_executable("git").is_err());
        assert!(expand_executable("").is_err());
        assert_eq!(
            expand_executable("~/.local/libexec/arc2git-zed").unwrap(),
            paths::home_dir().join(".local/libexec/arc2git-zed")
        );
    }

    #[gpui::test]
    fn executable_is_selected_from_server_settings_not_project_settings(cx: &mut TestAppContext) {
        use gpui::BorrowAppContext as _;
        use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore, WorktreeId};
        use std::sync::Arc;
        use util::rel_path::RelPath;

        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store
                    .set_server_settings(r#"{"git":{"arc_executable":"/server/arc2git-zed"}}"#, cx)
                    .unwrap();
                store
                    .set_local_settings(
                        WorktreeId::from_proto(1),
                        LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                        LocalSettingsKind::Settings,
                        Some(r#"{"git":{"arc_executable":"/project/other-git"}}"#),
                        cx,
                    )
                    .unwrap();
            });
            assert_eq!(
                ArcGitSettings::get_global(cx).executable.as_deref(),
                Some("/server/arc2git-zed")
            );
        });
    }
}
