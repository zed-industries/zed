use anyhow::{Context as _, anyhow};
use commit_modal::CommitModal;
use editor::{Editor, actions::DiffClipboardWithSelectionData};
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl, HttpRequestExt, Request};
use picker::{Picker, PickerDelegate, PickerEditorPosition};

use project::ProjectPath;
use serde::Deserialize;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use ui::{
    Divider, Headline, HeadlineSize, Icon, IconName, IconSize, IntoElement, ParentElement,
    Render, Styled, StyledExt, div, h_flex, rems, v_flex,
};
use ui_input::ErasedEditor;

mod blame_ui;
pub mod clone;

use git::{
    repository::{Branch, Upstream, UpstreamTracking, UpstreamTrackingStatus},
    status::{FileStatus, StatusCode, UnmergedStatus, UnmergedStatusCode},
};
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    SharedString, Subscription, Task, Window,
};
use menu::{Cancel, Confirm};
use project::git_store::Repository;
use project_diff::ProjectDiff;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace, notifications::DetachAndPromptErr};
use zed_actions;

use crate::{git_panel::GitPanel, text_diff_view::TextDiffView};

mod askpass_modal;
pub mod branch_picker;
mod commit_modal;
pub mod commit_tooltip;
pub mod commit_view;
mod conflict_view;
pub mod file_diff_view;
pub mod file_history_view;
pub mod git_panel;
mod git_panel_settings;
pub mod git_picker;
pub mod multi_diff_view;
pub mod picker_prompt;
pub mod project_diff;
pub(crate) mod remote_output;
pub mod repository_selector;
pub mod stash_picker;
pub mod text_diff_view;
pub mod worktree_picker;

const CLONE_SUGGESTIONS_DEBOUNCE: Duration = Duration::from_millis(200);
const MAX_AUTH_SUGGESTIONS: usize = 30;

pub fn init(cx: &mut App) {
    editor::set_blame_renderer(blame_ui::GitBlameRenderer, cx);
    commit_view::init(cx);

    cx.observe_new(|editor: &mut Editor, _, cx| {
        conflict_view::register_editor(editor, editor.buffer().clone(), cx);
    })
    .detach();

    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        ProjectDiff::register(workspace, cx);
        CommitModal::register(workspace);
        git_panel::register(workspace);
        repository_selector::register(workspace);
        git_picker::register(workspace);
        conflict_view::register_conflict_notification(workspace, cx);

        let project = workspace.project().read(cx);
        if project.is_read_only(cx) {
            return;
        }
        if !project.is_via_collab() {
            workspace.register_action(
                |workspace, _: &zed_actions::git::CreatePullRequest, window, cx| {
                    if let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.create_pull_request(window, cx);
                        });
                    }
                },
            );
            workspace.register_action(|workspace, _: &git::Fetch, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.fetch(true, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::FetchFrom, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.fetch(false, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::Push, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.push(false, false, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::PushTo, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.push(false, true, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::ForcePush, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.push(true, false, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::Pull, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.pull(false, window, cx);
                });
            });
            workspace.register_action(|workspace, _: &git::PullRebase, window, cx| {
                let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.pull(true, window, cx);
                });
            });
        }
        workspace.register_action(|workspace, action: &git::StashAll, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.stash_all(action, window, cx);
            });
        });
        workspace.register_action(|workspace, action: &git::StashPop, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.stash_pop(action, window, cx);
            });
        });
        workspace.register_action(|workspace, action: &git::StashApply, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.stash_apply(action, window, cx);
            });
        });
        workspace.register_action(|workspace, action: &git::StageAll, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.stage_all(action, window, cx);
            });
        });
        workspace.register_action(|workspace, action: &git::UnstageAll, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.unstage_all(action, window, cx);
            });
        });
        workspace.register_action(|workspace, _: &git::Uncommit, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.uncommit(window, cx);
            })
        });
        workspace.register_action(|workspace, _action: &git::Init, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.git_init(window, cx);
            });
        });
        workspace.register_action(|workspace, _action: &git::Clone, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            let http_client = workspace.client().http_client();

            workspace.toggle_modal(window, cx, |window, cx| {
                GitCloneModal::show(panel, http_client, window, cx)
            });
        });
        workspace.register_action(|workspace, _: &git::OpenModifiedFiles, window, cx| {
            open_modified_files(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &git::RenameBranch, window, cx| {
            rename_current_branch(workspace, window, cx);
        });
        workspace.register_action(
            |workspace, action: &DiffClipboardWithSelectionData, window, cx| {
                if let Some(task) = TextDiffView::open(action, workspace, window, cx) {
                    task.detach();
                };
            },
        );
        workspace.register_action(|workspace, _: &git::FileHistory, window, cx| {
            let Some(active_item) = workspace.active_item(cx) else {
                return;
            };
            let Some(editor) = active_item.downcast::<Editor>() else {
                return;
            };
            let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() else {
                return;
            };
            let Some(file) = buffer.read(cx).file() else {
                return;
            };
            let worktree_id = file.worktree_id(cx);
            let project_path = ProjectPath {
                worktree_id,
                path: file.path().clone(),
            };
            let project = workspace.project();
            let git_store = project.read(cx).git_store();
            let Some((repo, repo_path)) = git_store
                .read(cx)
                .repository_and_path_for_project_path(&project_path, cx)
            else {
                return;
            };
            file_history_view::FileHistoryView::open(
                repo_path,
                git_store.downgrade(),
                repo.downgrade(),
                workspace.weak_handle(),
                window,
                cx,
            );
        });
    })
    .detach();
}

fn open_modified_files(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
        return;
    };
    let modified_paths: Vec<_> = panel.update(cx, |panel, cx| {
        let Some(repo) = panel.active_repository.as_ref() else {
            return Vec::new();
        };
        let repo = repo.read(cx);
        repo.cached_status()
            .filter_map(|entry| {
                if entry.status.is_modified() {
                    repo.repo_path_to_project_path(&entry.repo_path, cx)
                } else {
                    None
                }
            })
            .collect()
    });
    for path in modified_paths {
        workspace.open_path(path, None, true, window, cx).detach();
    }
}

/// Resolves the repository for git operations, respecting the workspace's
/// active worktree override from the project dropdown.
pub fn resolve_active_repository(workspace: &Workspace, cx: &App) -> Option<Entity<Repository>> {
    let project = workspace.project().read(cx);
    workspace
        .active_worktree_override()
        .and_then(|override_id| {
            project
                .worktree_for_id(override_id, cx)
                .and_then(|worktree| {
                    let worktree_abs_path = worktree.read(cx).abs_path();
                    let git_store = project.git_store().read(cx);
                    git_store
                        .repositories()
                        .values()
                        .find(|repo| {
                            let repo_path = &repo.read(cx).work_directory_abs_path;
                            *repo_path == worktree_abs_path
                                || worktree_abs_path.starts_with(repo_path.as_ref())
                        })
                        .cloned()
                })
        })
        .or_else(|| project.active_repository(cx))
}

pub fn git_status_icon(status: FileStatus) -> impl IntoElement {
    GitStatusIcon::new(status)
}

struct RenameBranchModal {
    current_branch: SharedString,
    editor: Entity<Editor>,
    repo: Entity<Repository>,
}

impl RenameBranchModal {
    fn new(
        current_branch: String,
        repo: Entity<Repository>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(current_branch.clone(), window, cx);
            editor
        });
        Self {
            current_branch: current_branch.into(),
            editor,
            repo,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        if new_name.is_empty() || new_name == self.current_branch.as_ref() {
            cx.emit(DismissEvent);
            return;
        }

        let repo = self.repo.clone();
        let current_branch = self.current_branch.to_string();
        cx.spawn(async move |_, cx| {
            match repo
                .update(cx, |repo, _| {
                    repo.rename_branch(current_branch, new_name.clone())
                })
                .await
            {
                Ok(Ok(_)) => Ok(()),
                Ok(Err(error)) => Err(error),
                Err(_) => Err(anyhow!("Operation was canceled")),
            }
        })
        .detach_and_prompt_err("Failed to rename branch", window, cx, |_, _, _| None);
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RenameBranchModal {}
impl ModalView for RenameBranchModal {}
impl Focusable for RenameBranchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for RenameBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameBranchModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .w_full()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(
                        Headline::new(format!("Rename Branch ({})", self.current_branch))
                            .size(HeadlineSize::XSmall),
                    ),
            )
            .child(div().px_3().pb_3().w_full().child(self.editor.clone()))
    }
}

fn rename_current_branch(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
        return;
    };
    let current_branch: Option<String> = panel.update(cx, |panel, cx| {
        let repo = panel.active_repository.as_ref()?;
        let repo = repo.read(cx);
        repo.branch.as_ref().map(|branch| branch.name().to_string())
    });

    let Some(current_branch_name) = current_branch else {
        return;
    };

    let repo = panel.read(cx).active_repository.clone();
    let Some(repo) = repo else {
        return;
    };

    workspace.toggle_modal(window, cx, |window, cx| {
        RenameBranchModal::new(current_branch_name, repo, window, cx)
    });
}

fn render_remote_button(
    id: impl Into<SharedString>,
    branch: &Branch,
    keybinding_target: Option<FocusHandle>,
    show_fetch_button: bool,
) -> Option<impl IntoElement> {
    let id = id.into();
    let upstream = branch.upstream.as_ref();
    match upstream {
        Some(Upstream {
            tracking: UpstreamTracking::Tracked(UpstreamTrackingStatus { ahead, behind }),
            ..
        }) => match (*ahead, *behind) {
            (0, 0) if show_fetch_button => {
                Some(remote_button::render_fetch_button(keybinding_target, id))
            }
            (0, 0) => None,
            (ahead, 0) => Some(remote_button::render_push_button(
                keybinding_target,
                id,
                ahead,
            )),
            (ahead, behind) => Some(remote_button::render_pull_button(
                keybinding_target,
                id,
                ahead,
                behind,
            )),
        },
        Some(Upstream {
            tracking: UpstreamTracking::Gone,
            ..
        }) => Some(remote_button::render_republish_button(
            keybinding_target,
            id,
        )),
        None => Some(remote_button::render_publish_button(keybinding_target, id)),
    }
}

mod remote_button {
    use gpui::{Action, AnyView, ClickEvent, Corner, FocusHandle};
    use ui::{
        App, ButtonCommon, Clickable, ContextMenu, ElementId, FluentBuilder, Icon, IconName,
        IconSize, IntoElement, Label, LabelCommon, LabelSize, LineHeightStyle, ParentElement,
        PopoverMenu, SharedString, SplitButton, Styled, Tooltip, Window, div, h_flex, rems,
    };

    pub fn render_fetch_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
    ) -> SplitButton {
        split_button(
            id,
            "Fetch",
            0,
            0,
            Some(IconName::ArrowCircle),
            keybinding_target.clone(),
            move |_, window, cx| {
                window.dispatch_action(Box::new(git::Fetch), cx);
            },
            move |_window, cx| {
                git_action_tooltip(
                    "Fetch updates from remote",
                    &git::Fetch,
                    "git fetch",
                    keybinding_target.clone(),
                    cx,
                )
            },
        )
    }

    pub fn render_push_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
        ahead: u32,
    ) -> SplitButton {
        split_button(
            id,
            "Push",
            ahead as usize,
            0,
            None,
            keybinding_target.clone(),
            move |_, window, cx| {
                window.dispatch_action(Box::new(git::Push), cx);
            },
            move |_window, cx| {
                git_action_tooltip(
                    "Push committed changes to remote",
                    &git::Push,
                    "git push",
                    keybinding_target.clone(),
                    cx,
                )
            },
        )
    }

    pub fn render_pull_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
        ahead: u32,
        behind: u32,
    ) -> SplitButton {
        split_button(
            id,
            "Pull",
            ahead as usize,
            behind as usize,
            None,
            keybinding_target.clone(),
            move |_, window, cx| {
                window.dispatch_action(Box::new(git::Pull), cx);
            },
            move |_window, cx| {
                git_action_tooltip(
                    "Pull",
                    &git::Pull,
                    "git pull",
                    keybinding_target.clone(),
                    cx,
                )
            },
        )
    }

    pub fn render_publish_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
    ) -> SplitButton {
        split_button(
            id,
            "Publish",
            0,
            0,
            Some(IconName::ExpandUp),
            keybinding_target.clone(),
            move |_, window, cx| {
                window.dispatch_action(Box::new(git::Push), cx);
            },
            move |_window, cx| {
                git_action_tooltip(
                    "Publish branch to remote",
                    &git::Push,
                    "git push --set-upstream",
                    keybinding_target.clone(),
                    cx,
                )
            },
        )
    }

    pub fn render_republish_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
    ) -> SplitButton {
        split_button(
            id,
            "Republish",
            0,
            0,
            Some(IconName::ExpandUp),
            keybinding_target.clone(),
            move |_, window, cx| {
                window.dispatch_action(Box::new(git::Push), cx);
            },
            move |_window, cx| {
                git_action_tooltip(
                    "Re-publish branch to remote",
                    &git::Push,
                    "git push --set-upstream",
                    keybinding_target.clone(),
                    cx,
                )
            },
        )
    }

    fn git_action_tooltip(
        label: impl Into<SharedString>,
        action: &dyn Action,
        command: impl Into<SharedString>,
        focus_handle: Option<FocusHandle>,
        cx: &mut App,
    ) -> AnyView {
        let label = label.into();
        let command = command.into();

        if let Some(handle) = focus_handle {
            Tooltip::with_meta_in(label, Some(action), command, &handle, cx)
        } else {
            Tooltip::with_meta(label, Some(action), command, cx)
        }
    }

    fn render_git_action_menu(
        id: impl Into<ElementId>,
        keybinding_target: Option<FocusHandle>,
    ) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                ui::ButtonLike::new_rounded_right("split-button-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ui::ButtonSize::None)
                    .child(
                        div()
                            .px_1()
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    ),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                    context_menu
                        .when_some(keybinding_target.clone(), |el, keybinding_target| {
                            el.context(keybinding_target)
                        })
                        .action("Fetch", git::Fetch.boxed_clone())
                        .action("Fetch From", git::FetchFrom.boxed_clone())
                        .action("Pull", git::Pull.boxed_clone())
                        .action("Pull (Rebase)", git::PullRebase.boxed_clone())
                        .separator()
                        .action("Push", git::Push.boxed_clone())
                        .action("Push To", git::PushTo.boxed_clone())
                        .action("Force Push", git::ForcePush.boxed_clone())
                }))
            })
            .anchor(Corner::TopRight)
    }

    #[allow(clippy::too_many_arguments)]
    fn split_button(
        id: SharedString,
        left_label: impl Into<SharedString>,
        ahead_count: usize,
        behind_count: usize,
        left_icon: Option<IconName>,
        keybinding_target: Option<FocusHandle>,
        left_on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> SplitButton {
        fn count(count: usize) -> impl IntoElement {
            h_flex()
                .ml_neg_px()
                .h(rems(0.875))
                .items_center()
                .overflow_hidden()
                .px_0p5()
                .child(
                    Label::new(count.to_string())
                        .size(LabelSize::XSmall)
                        .line_height_style(LineHeightStyle::UiLabel),
                )
        }

        let should_render_counts = left_icon.is_none() && (ahead_count > 0 || behind_count > 0);

        let left = ui::ButtonLike::new_rounded_left(ElementId::Name(
            format!("split-button-left-{}", id).into(),
        ))
        .layer(ui::ElevationIndex::ModalSurface)
        .size(ui::ButtonSize::Compact)
        .when(should_render_counts, |this| {
            this.child(
                h_flex()
                    .ml_neg_0p5()
                    .when(behind_count > 0, |this| {
                        this.child(Icon::new(IconName::ArrowDown).size(IconSize::XSmall))
                            .child(count(behind_count))
                    })
                    .when(ahead_count > 0, |this| {
                        this.child(Icon::new(IconName::ArrowUp).size(IconSize::XSmall))
                            .child(count(ahead_count))
                    }),
            )
        })
        .when_some(left_icon, |this, left_icon| {
            this.child(
                h_flex()
                    .ml_neg_0p5()
                    .child(Icon::new(left_icon).size(IconSize::XSmall)),
            )
        })
        .child(
            div()
                .child(Label::new(left_label).size(LabelSize::Small))
                .mr_0p5(),
        )
        .on_click(left_on_click)
        .tooltip(tooltip);

        let right = render_git_action_menu(
            ElementId::Name(format!("split-button-right-{}", id).into()),
            keybinding_target,
        )
        .into_any_element();

        SplitButton::new(left, right)
    }
}

/// A visual representation of a file's Git status.
#[derive(IntoElement, RegisterComponent)]
pub struct GitStatusIcon {
    status: FileStatus,
}

impl GitStatusIcon {
    pub fn new(status: FileStatus) -> Self {
        Self { status }
    }
}

impl RenderOnce for GitStatusIcon {
    fn render(self, _window: &mut ui::Window, cx: &mut App) -> impl IntoElement {
        let status = self.status;

        let (icon_name, color) = if status.is_conflicted() {
            (
                IconName::Warning,
                cx.theme().colors().version_control_conflict,
            )
        } else if status.is_deleted() {
            (
                IconName::SquareMinus,
                cx.theme().colors().version_control_deleted,
            )
        } else if status.is_modified() {
            (
                IconName::SquareDot,
                cx.theme().colors().version_control_modified,
            )
        } else {
            (
                IconName::SquarePlus,
                cx.theme().colors().version_control_added,
            )
        };

        Icon::new(icon_name).color(Color::Custom(color))
    }
}

// View this component preview using `workspace: open component-preview`
impl Component for GitStatusIcon {
    fn scope() -> ComponentScope {
        ComponentScope::VersionControl
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        fn tracked_file_status(code: StatusCode) -> FileStatus {
            FileStatus::Tracked(git::status::TrackedStatus {
                index_status: code,
                worktree_status: code,
            })
        }

        let modified = tracked_file_status(StatusCode::Modified);
        let added = tracked_file_status(StatusCode::Added);
        let deleted = tracked_file_status(StatusCode::Deleted);
        let conflict = UnmergedStatus {
            first_head: UnmergedStatusCode::Updated,
            second_head: UnmergedStatusCode::Updated,
        }
        .into();

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![
                    single_example("Modified", GitStatusIcon::new(modified).into_any_element()),
                    single_example("Added", GitStatusIcon::new(added).into_any_element()),
                    single_example("Deleted", GitStatusIcon::new(deleted).into_any_element()),
                    single_example(
                        "Conflicted",
                        GitStatusIcon::new(conflict).into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}

struct GitCloneModal {
    picker: Entity<Picker<GitCloneDelegate>>,
    _subscription: Subscription,
    focus_handle: FocusHandle,
}

impl GitCloneModal {
    pub fn show(
        panel: Entity<GitPanel>,
        http_client: Arc<HttpClientWithUrl>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = GitCloneDelegate::new(panel, http_client);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .max_height(Some(rems(16.).into()))
                .show_scrollbar(true)
                .modal(false)
        });
        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| cx.emit(DismissEvent));
        let focus_handle = picker.focus_handle(cx);

        window.focus(&focus_handle, cx);

        Self {
            picker,
            _subscription: subscription,
            focus_handle,
        }
    }
}

impl Focusable for GitCloneModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for GitCloneModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .occlude()
            .elevation_3(cx)
            .w(rems(34.))
            .flex_1()
            .overflow_hidden()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .child(
                h_flex()
                    .w_full()
                    .flex_none()
                    .px_2()
                    .py_1p5()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Label::new("Clone a repository from GitHub or other sources.")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .child(
                        Button::new("learn-more", "Learn More")
                            .label_size(LabelSize::Small)
                            .end_icon(Icon::new(IconName::ArrowUpRight).size(IconSize::XSmall))
                            .on_click(|_, _: &mut Window, cx: &mut App| {
                                cx.open_url("https://github.com/git-guides/git-clone");
                            }),
                    ),
            )
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
    }
}

#[derive(Clone)]
struct CloneSuggestion {
    title: SharedString,
    detail: Option<SharedString>,
    repo_url: SharedString,
}

struct AuthenticatedRepoCacheEntry {
    token: String,
    suggestions: Vec<CloneSuggestion>,
}

static AUTHENTICATED_REPO_CACHE: OnceLock<Mutex<Option<AuthenticatedRepoCacheEntry>>> =
    OnceLock::new();

#[derive(Deserialize)]
struct GithubSearchResponse {
    items: Vec<GithubSearchRepository>,
}

#[derive(Deserialize)]
struct GithubSearchRepository {
    full_name: String,
    clone_url: String,
    private: bool,
    description: Option<String>,
}

#[derive(Deserialize)]
struct GithubUserRepository {
    full_name: String,
    clone_url: String,
    private: bool,
    description: Option<String>,
}

struct GitCloneDelegate {
    panel: Entity<GitPanel>,
    http_client: Arc<HttpClientWithUrl>,
    suggestions: Vec<CloneSuggestion>,
    selected_index: usize,
    latest_request_id: usize,
}

impl GitCloneDelegate {
    fn new(panel: Entity<GitPanel>, http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            panel,
            http_client,
            suggestions: Vec::new(),
            selected_index: 0,
            latest_request_id: 0,
        }
    }
}

impl PickerDelegate for GitCloneDelegate {
    type ListItem = ListItem;

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Div {
        v_flex()
            .when(
                self.editor_position() == PickerEditorPosition::End,
                |this| {
                    this.child(Divider::horizontal())
                        .child(Divider::horizontal())
                },
            )
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .child(editor.render(_window, _cx)),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| {
                    this.child(Divider::horizontal())
                        .child(Divider::horizontal())
                },
            )
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Enter repository URL or owner/repo…".into()
    }

    fn match_count(&self) -> usize {
        self.suggestions.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = index.min(self.suggestions.len().saturating_sub(1));
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let request_id = self.latest_request_id.wrapping_add(1);
        self.latest_request_id = request_id;

        let query = query.trim().to_string();
        let typed_suggestion = build_typed_clone_suggestion(&query);
        let should_query_github = should_query_github(&query);
        let http_client = self.http_client.clone();
        let query_for_fetch = query.clone();

        cx.spawn_in(window, async move |picker, cx| {
            if should_query_github {
                cx.background_executor().timer(CLONE_SUGGESTIONS_DEBOUNCE).await;

                let is_latest = picker
                    .read_with(cx, |picker, _| request_id == picker.delegate.latest_request_id)
                    .unwrap_or(false);
                if !is_latest {
                    return;
                }
            }

            let mut suggestions = Vec::new();

            if let Some(typed_suggestion) = typed_suggestion {
                suggestions.push(typed_suggestion);
            }

            if should_query_github {
                let github_suggestions = cx
                    .background_spawn(async move {
                        fetch_github_repository_suggestions(http_client, query_for_fetch).await
                    })
                    .await;

                match github_suggestions {
                    Ok(mut github_suggestions) => suggestions.append(&mut github_suggestions),
                    Err(error) => {
                        log::debug!("failed to fetch clone suggestions from GitHub: {error}");
                    }
                }
            }

            let mut unique_suggestions = Vec::new();
            for suggestion in suggestions {
                if unique_suggestions
                    .iter()
                    .all(|existing: &CloneSuggestion| existing.repo_url != suggestion.repo_url)
                {
                    unique_suggestions.push(suggestion);
                }
            }

            picker
                .update(cx, |picker, _| {
                    if request_id != picker.delegate.latest_request_id {
                        return;
                    }

                    picker.delegate.suggestions = unique_suggestions;
                    picker.delegate.selected_index = 0;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(suggestion) = self.suggestions.get(self.selected_index) else {
            return;
        };

        let repo = suggestion.repo_url.to_string();
        self.panel.update(cx, |panel, cx| {
            panel.git_clone(repo, window, cx);
        });
        window.dispatch_action(menu::Cancel.boxed_clone(), cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let suggestion = self.suggestions.get(index)?;

        Some(
            ListItem::new(format!("clone-suggestion-{index}"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .gap_0p5()
                        .child(Label::new(suggestion.title.clone()))
                        .when_some(suggestion.detail.clone(), |this, detail| {
                            this.child(
                                Label::new(detail)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        }),
                ),
        )
    }
}

fn should_query_github(query: &str) -> bool {
    let query = query.trim();
    !query.is_empty()
        && query.len() >= 2
        && !query.contains("://")
        && !query.starts_with("git@")
        && !query.starts_with("ssh://")
}

fn build_typed_clone_suggestion(query: &str) -> Option<CloneSuggestion> {
    let query = query.trim();
    if query.is_empty() {
        return None;
    }

    let normalized: SharedString = crate::clone::normalize_repository_input(query)
        .unwrap_or_else(|| query.to_string())
        .into();

    Some(CloneSuggestion {
        title: "Clone entered repository".into(),
        detail: Some(normalized.clone()),
        repo_url: normalized,
    })
}

async fn fetch_github_repository_suggestions(
    http_client: Arc<HttpClientWithUrl>,
    query: String,
) -> anyhow::Result<Vec<CloneSuggestion>> {
    let github_token = github_api_token();
    let mut suggestions = Vec::new();

    if let Some(token) = github_token.as_deref() {
        let cache = AUTHENTICATED_REPO_CACHE.get_or_init(|| Mutex::new(None));

        let cached_authenticated_repositories = {
            let guard = cache
                .lock()
                .expect("AUTHENTICATED_REPO_CACHE mutex poisoned");

            guard
                .as_ref()
                .filter(|entry| entry.token == token)
                .map(|entry| entry.suggestions.clone())
        };

        let authenticated_repositories = if let Some(repositories) = cached_authenticated_repositories
        {
            repositories
        } else {
            match fetch_authenticated_github_repositories(http_client.clone(), token).await {
                Ok(repositories) => {
                    let mut guard = cache
                        .lock()
                        .expect("AUTHENTICATED_REPO_CACHE mutex poisoned");
                    *guard = Some(AuthenticatedRepoCacheEntry {
                        token: token.to_string(),
                        suggestions: repositories.clone(),
                    });
                    repositories
                }
                Err(error) => {
                    log::debug!(
                        "failed to fetch authenticated GitHub repositories for clone suggestions: {error}"
                    );
                    Vec::new()
                }
            }
        };

        suggestions.extend(filter_authenticated_repo_suggestions(
            &authenticated_repositories,
            &query,
        ));
    }

    let encoded_query = percent_encode_query(&query);
    let search_url = format!(
        "https://api.github.com/search/repositories?q={encoded_query}&sort=updated&per_page=8"
    );

    let mut request = Request::get(&search_url)
        .header("accept", "application/vnd.github+json")
        .follow_redirects(http_client::RedirectPolicy::FollowAll);
    if let Some(token) = github_token {
        request = request.header("authorization", format!("Bearer {token}"));
    }

    let mut response = http_client.send(request.body(AsyncBody::default())?).await?;
    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading GitHub clone suggestions")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "GitHub suggestions request failed with status {}",
            response.status().as_u16()
        );
    }

    let parsed: GithubSearchResponse = serde_json::from_slice(&body)
        .context("failed to parse GitHub repository suggestions")?;

    suggestions.extend(
        parsed
            .items
            .into_iter()
            .take(8)
            .map(|repository| to_clone_suggestion(
                repository.full_name,
                repository.clone_url,
                repository.private,
                repository.description,
            )),
    );

    Ok(suggestions)
}

async fn fetch_authenticated_github_repositories(
    http_client: Arc<HttpClientWithUrl>,
    github_token: &str,
) -> anyhow::Result<Vec<CloneSuggestion>> {
    const MAX_AUTH_REPOSITORY_PAGES: usize = 3;

    let mut repository_pages = Vec::new();
    let mut next_page_url = Some(
        "https://api.github.com/user/repos?per_page=100&sort=updated&affiliation=owner,collaborator,organization_member"
            .to_string(),
    );

    for _ in 0..MAX_AUTH_REPOSITORY_PAGES {
        let Some(page_url) = next_page_url.take() else {
            break;
        };

        let request = Request::get(&page_url)
            .header("accept", "application/vnd.github+json")
            .header("authorization", format!("Bearer {github_token}"))
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = http_client.send(request.body(AsyncBody::default())?).await?;
        let next_link = response
            .headers()
            .get("link")
            .and_then(|value| value.to_str().ok())
            .and_then(github_link_next_page_url);

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading authenticated GitHub repositories")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "authenticated GitHub repositories request failed with status {}",
                response.status().as_u16()
            );
        }

        let mut repositories: Vec<GithubUserRepository> = serde_json::from_slice(&body)
            .context("failed to parse authenticated GitHub repositories")?;
        repository_pages.append(&mut repositories);

        next_page_url = next_link;

        if repository_pages.len() >= MAX_AUTH_SUGGESTIONS * 2 {
            break;
        }
    }

    Ok(repository_pages
        .into_iter()
        .take(MAX_AUTH_SUGGESTIONS)
        .map(|repository| {
            to_clone_suggestion(
                repository.full_name,
                repository.clone_url,
                repository.private,
                repository.description,
            )
        })
        .collect())
}

fn filter_authenticated_repo_suggestions(
    suggestions: &[CloneSuggestion],
    query: &str,
) -> Vec<CloneSuggestion> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return suggestions.iter().take(MAX_AUTH_SUGGESTIONS).cloned().collect();
    }

    suggestions
        .iter()
        .filter(|suggestion| {
            suggestion.title.to_lowercase().contains(&query)
                || suggestion
                    .detail
                    .as_ref()
                    .is_some_and(|detail| detail.to_lowercase().contains(&query))
        })
        .take(MAX_AUTH_SUGGESTIONS)
        .cloned()
        .collect()
}

fn github_link_next_page_url(link_header: &str) -> Option<String> {
    for link_entry in link_header.split(',') {
        let mut parts = link_entry.trim().split(';');
        let Some(url_part) = parts.next().map(str::trim) else {
            continue;
        };

        let is_next = parts.any(|part| part.trim() == r#"rel="next""#);
        if !is_next {
            continue;
        }

        let Some(url_part) = url_part.strip_prefix('<') else {
            continue;
        };
        let Some(url_part) = url_part.strip_suffix('>') else {
            continue;
        };

        return Some(url_part.to_string());
    }

    None
}

fn to_clone_suggestion(
    full_name: String,
    clone_url: String,
    private: bool,
    description: Option<String>,
) -> CloneSuggestion {
    let visibility = if private { "private" } else { "public" };

    CloneSuggestion {
        title: full_name.into(),
        detail: Some(
            description
                .map(|description| format!("{visibility} - {description}"))
                .unwrap_or_else(|| visibility.to_string())
                .into(),
        ),
        repo_url: clone_url.into(),
    }
}

fn percent_encode_query(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());

    for byte in input.bytes() {
        let is_unreserved = byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~');
        if is_unreserved {
            encoded.push(byte as char);
        } else if byte == b' ' {
            encoded.push('+');
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }

    encoded
}

fn github_api_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .or_else(|| std::env::var("GH_COPILOT_TOKEN").ok())
        .or_else(|| std::env::var("GH_TOKEN").ok())
}

impl EventEmitter<DismissEvent> for GitCloneModal {}

impl ModalView for GitCloneModal {}
