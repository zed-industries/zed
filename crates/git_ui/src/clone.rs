use gpui::{App, Context, Window};
use std::sync::Arc;
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

        let workspace_weak = cx.weak_entity();
        cx.spawn_in(window, async move |workspace, cx| {
            let mut paths = path_prompt.await.ok()?.ok()??;
            let path = paths.pop()?;

            let repo_name = repo_url
                .split('/')
                .next_back()
                .and_then(|name| name.strip_suffix(".git"))
                .unwrap_or("repository")
                .to_owned();

            let fs = workspace
                .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
                .ok()?;

            let clone_result = fs.git_clone(&repo_url, path.as_path()).await;

            workspace_weak
                .update_in(cx, |workspace, window, cx| {
                    workspace.hide_modal(window, cx);
                })
                .ok();

            clone_result.ok()?;

            let mut cloned_path = path;
            cloned_path.push(&repo_name);

            let prompt_answer = cx
                .update(|window, cx| {
                    window.prompt(
                        gpui::PromptLevel::Info,
                        &format!("Git Clone: \"{}\"", repo_name),
                        None,
                        &["Add to current project", "Open in new window"],
                        cx,
                    )
                })
                .ok()?
                .await
                .ok()?;

            match prompt_answer {
                0 => {
                    let on_success_clone = on_success.clone();
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            let worktree_task = workspace.project().update(cx, |project, cx| {
                                project.create_worktree(cloned_path.as_path(), true, cx)
                            });
                            let workspace_weak = cx.weak_entity();
                            let on_success_clone = on_success_clone.clone();
                            cx.spawn_in(window, async move |_window, cx| {
                                if worktree_task.await.log_err().is_some() {
                                    workspace_weak
                                        .update_in(cx, |workspace, window, cx| {
                                            (on_success_clone)(workspace, window, cx);
                                        })
                                        .ok();
                                }
                            })
                            .detach();
                        })
                        .ok()?;
                }
                1 => {
                    let on_success_clone = on_success.clone();
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
                                            project.create_worktree(&cloned_path, true, cx)
                                        });
                                    let workspace_weak = cx.weak_entity();
                                    let on_success_clone = on_success_clone.clone();
                                    cx.spawn_in(window, async move |_window, cx| {
                                        if worktree_task.await.log_err().is_some() {
                                            workspace_weak
                                                .update_in(cx, |workspace, window, cx| {
                                                    (on_success_clone)(workspace, window, cx);
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
