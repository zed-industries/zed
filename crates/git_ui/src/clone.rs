use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, SharedString, Window, div,
};
use std::sync::Arc;
use ui::{Color, CommonAnimationExt, Icon, IconName, Label, h_flex, prelude::*, v_flex};
use util::ResultExt;
use workspace::{self, AppState, ModalView};

struct CloneProgressModal {
    repo_name: SharedString,
    focus_handle: FocusHandle,
}

impl CloneProgressModal {
    fn new(repo_name: String, cx: &mut Context<Self>) -> Self {
        Self {
            repo_name: repo_name.into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for CloneProgressModal {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CloneProgressModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().elevation_3(cx).w(ui::rems(34.)).p_4().child(
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Icon::new(IconName::ArrowCircle)
                                .size(ui::IconSize::Small)
                                .color(Color::Accent)
                                .with_rotate_animation(2),
                        )
                        .child(
                            Label::new(format!("Cloning \"{}\"", self.repo_name))
                                .size(ui::LabelSize::Large),
                        ),
                )
                .child(
                    Label::new("Please wait while the repository is being cloned...")
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}

impl EventEmitter<DismissEvent> for CloneProgressModal {}
impl ModalView for CloneProgressModal {}

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
            let result = async {
                let mut paths = path_prompt.await.ok()?.ok()??;
                let path = paths.pop()?;

                let repo_name = repo_url
                    .split('/')
                    .next_back()
                    .and_then(|name| name.strip_suffix(".git"))
                    .unwrap_or("repository")
                    .to_owned();

                workspace_weak
                    .update_in(cx, |workspace, window, cx| {
                        workspace.toggle_modal(window, cx, |_window, cx| {
                            CloneProgressModal::new(repo_name.clone(), cx)
                        });
                    })
                    .ok()?;

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
                                let worktree_task =
                                    workspace.project().update(cx, |project, cx| {
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
            }
            .await;

            if result.is_none() {
                workspace_weak
                    .update_in(cx, |workspace, window, cx| {
                        workspace.hide_modal(window, cx);
                    })
                    .ok();
                log::error!("Failed to clone git repository");
            }
        })
        .detach();
    });
}
