use std::path::Path;
use std::sync::Arc;

use fs::Fs;
use futures::StreamExt;
use gpui::{App, PathPromptOptions, WeakEntity, Window};
use notifications::status_toast::StatusToast;
use ui::{Color, Icon, IconName, IconSize};
use util::ResultExt;
use util::command::new_command;
use workspace::{self, Workspace};

use crate::scaffold;

/// Writes the scaffold into `destination` if it is empty; returns an error
/// (writing nothing) otherwise. `destination` must already exist as a
/// directory (the caller picks it via a folder-selection prompt).
async fn write_scaffold(fs: Arc<dyn Fs>, destination: &Path) -> anyhow::Result<()> {
    let mut entries = fs.read_dir(destination).await?;
    if entries.next().await.is_some() {
        anyhow::bail!("Selected folder is not empty");
    }

    let project_name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");

    for (relative_path, contents) in scaffold::scaffold_files(project_name) {
        let absolute_path = destination.join(&relative_path);
        if let Some(parent) = absolute_path.parent() {
            fs.create_dir(parent).await?;
        }
        fs.write(&absolute_path, contents.as_bytes()).await?;
    }

    Ok(())
}

async fn git_init_and_commit(fs: Arc<dyn Fs>, destination: &Path) -> anyhow::Result<()> {
    fs.git_init(destination, "main".to_string()).await?;

    let add_status = new_command("git")
        .current_dir(destination)
        .args(["add", "-A"])
        .status()
        .await?;
    anyhow::ensure!(add_status.success(), "git add -A failed");

    let commit_status = new_command("git")
        .current_dir(destination)
        .args(["commit", "-m", "Initial commit"])
        .status()
        .await?;
    anyhow::ensure!(commit_status.success(), "git commit failed");

    Ok(())
}

pub fn new_project(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) {
    let destination_prompt = cx.prompt_for_paths(PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Select Project Location".into()),
    });

    window
        .spawn(cx, async move |cx| {
            let mut paths = destination_prompt.await.ok()?.ok()??;
            let destination = paths.pop()?;

            let fs = workspace
                .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
                .ok()?;

            let scaffold_result = write_scaffold(fs.clone(), &destination).await;
            if let Err(error) = scaffold_result {
                workspace
                    .update(cx, |workspace, cx| {
                        let toast = StatusToast::new(error.to_string(), cx, |this, _| {
                            this.icon(
                                Icon::new(IconName::XCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Error),
                            )
                            .dismiss_button(true)
                        });
                        workspace.toggle_status_toast(toast, cx);
                    })
                    .ok()?;
                return None;
            }

            if let Err(error) = git_init_and_commit(fs.clone(), &destination).await {
                workspace
                    .update(cx, |workspace, cx| {
                        let toast = StatusToast::new(error.to_string(), cx, |this, _| {
                            this.icon(
                                Icon::new(IconName::XCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Error),
                            )
                            .dismiss_button(true)
                        });
                        workspace.toggle_status_toast(toast, cx);
                    })
                    .ok()?;
                return None;
            }

            workspace
                .update(cx, move |workspace, cx| {
                    let app_state = workspace.app_state().clone();
                    workspace::open_new(Default::default(), app_state, cx, {
                        let destination = destination.clone();
                        move |workspace, window, cx| {
                            cx.activate(true);
                            let create_task = workspace.project().update(cx, |project, cx| {
                                project.create_worktree(destination.as_path(), true, cx)
                            });
                            cx.spawn_in(window, async move |_window, _cx| {
                                create_task.await.log_err();
                            })
                            .detach();
                        }
                    })
                    .detach();
                })
                .ok();

            Some(())
        })
        .detach();
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;

    #[gpui::test]
    async fn writes_expected_files_into_an_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "empty-project": {} })).await;

        write_scaffold(fs.clone(), std::path::Path::new("/root/empty-project"))
            .await
            .unwrap();

        assert_eq!(
            fs.load(std::path::Path::new("/root/empty-project/cpp/io.cpp"))
                .await
                .unwrap(),
            scaffold::scaffold_files("empty-project")
                .into_iter()
                .find(|(p, _)| p == std::path::Path::new("cpp/io.cpp"))
                .unwrap()
                .1
        );
        assert_eq!(
            fs.load(std::path::Path::new(
                "/root/empty-project/.claude/CLAUDE.md"
            ))
            .await
            .unwrap(),
            scaffold::scaffold_files("empty-project")
                .into_iter()
                .find(|(p, _)| p == std::path::Path::new(".claude/CLAUDE.md"))
                .unwrap()
                .1
        );
    }

    #[gpui::test]
    async fn rejects_a_non_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({ "not-empty": { "existing.txt": "hello" } }),
        )
        .await;

        let result = write_scaffold(fs.clone(), std::path::Path::new("/root/not-empty")).await;

        assert!(result.is_err());
        assert!(
            fs.load(std::path::Path::new("/root/not-empty/cpp/io.cpp"))
                .await
                .is_err(),
            "scaffold must not write anything into a non-empty directory"
        );
    }
}
