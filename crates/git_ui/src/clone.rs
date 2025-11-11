use gpui::{App, Context, Window};
use notifications::status_toast::{StatusToast, ToastIcon};
use std::sync::Arc;
use ui::{Color, IconName};
use util::ResultExt;
use workspace::{self, AppState};

pub fn clone_and_open(
    repo_url: String,
    app_state: Arc<AppState>,
    cx: &mut App,
    on_success: Arc<
        dyn Fn(&mut workspace::Workspace, &mut Window, &mut Context<workspace::Workspace>)
            + Send
            + Sync
            + 'static,
    >,
) {
    workspace::with_active_or_new_workspace(cx, |_workspace, window, cx| {
        let path_prompt = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select directory for cloned repository".into()),
        });

        cx.spawn_in(window, async move |workspace, cx| {
            let mut paths = path_prompt.await.ok()?.ok()??;
            let mut path = paths.pop()?;

            let repo_name = repo_url
                .split('/')
                .next_back()
                .and_then(|name| name.strip_suffix(".git"))
                .unwrap_or("repository")
                .to_owned();

            let fs = workspace
                .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
                .ok()?;

            let prompt_answer = match fs.git_clone(&repo_url, path.as_path()).await {
                Ok(_) => cx.update(|window, cx| {
                    window.prompt(
                        gpui::PromptLevel::Info,
                        &format!("Git Clone: \"{}\"", repo_name),
                        None,
                        &["Add to current project", "Open in new window"],
                        cx,
                    )
                }),
                Err(e) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            let toast = StatusToast::new(e.to_string(), cx, |this, _| {
                                this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                                    .dismiss_button(true)
                            });
                            workspace.toggle_status_toast(toast, cx);
                        })
                        .ok()?;

                    return None;
                }
            }
            .ok()?;

            path.push(&repo_name);

            match prompt_answer.await.ok()? {
                0 => {
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            let worktree_task = workspace.project().update(cx, |project, cx| {
                                project.create_worktree(path.as_path(), true, cx)
                            });
                            let workspace_weak = cx.weak_entity();
                            cx.spawn_in(window, async move |_window, cx| {
                                if worktree_task.await.log_err().is_some() {
                                    workspace_weak
                                        .update_in(cx, |workspace, window, cx| {
                                            (on_success)(workspace, window, cx);
                                        })
                                        .ok();
                                }
                            })
                            .detach();
                        })
                        .ok()?;
                }
                1 => {
                    workspace
                        .update(cx, move |_workspace, cx| {
                            workspace::open_new(
                                Default::default(),
                                app_state,
                                cx,
                                move |workspace, window, cx| {
                                    cx.activate(true);
                                    let worktree_task =
                                        workspace.project().update(cx, |project, cx| {
                                            project.create_worktree(&path, true, cx)
                                        });
                                    let workspace_weak = cx.weak_entity();
                                    cx.spawn_in(window, async move |_window, cx| {
                                        if worktree_task.await.log_err().is_some() {
                                            workspace_weak
                                                .update_in(cx, |workspace, window, cx| {
                                                    (on_success)(workspace, window, cx);
                                                })
                                                .ok();
                                        }
                                    })
                                    .detach();
                                },
                            )
                            .detach();
                        })
                        .ok();
                }
                _ => {}
            }

            Some(())
        })
        .detach();
    });
}
