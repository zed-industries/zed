use std::any::Any;

use ::settings::Settings;
use command_palette_hooks::CommandPaletteFilter;
use commit_modal::CommitModal;
use editor::{Editor, actions::DiffClipboardWithSelectionData};
mod blame_ui;
use git::{
    repository::{Branch, Upstream, UpstreamTracking, UpstreamTrackingStatus},
    status::{FileStatus, StatusCode, UnmergedStatus, UnmergedStatusCode},
};
use git_panel_settings::GitPanelSettings;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Window,
    actions,
};
use onboarding::GitOnboardingModal;
use project_diff::ProjectDiff;
use ui::prelude::*;
use workspace::{ModalView, Workspace};
use zed_actions;

use crate::{git_panel::GitPanel, text_diff_view::TextDiffView};

mod askpass_modal;
pub mod branch_picker;
mod commit_modal;
pub mod commit_tooltip;
mod commit_view;
mod conflict_view;
pub mod file_diff_view;
pub mod git_panel;
mod git_panel_settings;
pub mod onboarding;
pub mod picker_prompt;
pub mod project_diff;
pub(crate) mod remote_output;
pub mod repository_selector;
pub mod stash_picker;
pub mod text_diff_view;

actions!(
    git,
    [
        /// Resets the git onboarding state to show the tutorial again.
        ResetOnboarding
    ]
);

pub fn init(cx: &mut App) {
    GitPanelSettings::register(cx);

    editor::set_blame_renderer(blame_ui::GitBlameRenderer, cx);

    cx.observe_new(|editor: &mut Editor, _, cx| {
        conflict_view::register_editor(editor, editor.buffer().clone(), cx);
    })
    .detach();

    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        ProjectDiff::register(workspace, cx);
        CommitModal::register(workspace);
        git_panel::register(workspace);
        repository_selector::register(workspace);
        branch_picker::register(workspace);
        stash_picker::register(workspace);

        let project = workspace.project().read(cx);
        if project.is_read_only(cx) {
            return;
        }
        if !project.is_via_collab() {
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
                    panel.pull(window, cx);
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
        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&[
                zed_actions::OpenGitIntegrationOnboarding.type_id(),
                // ResetOnboarding.type_id(),
            ]);
        });
        workspace.register_action(
            move |workspace, _: &zed_actions::OpenGitIntegrationOnboarding, window, cx| {
                GitOnboardingModal::toggle(workspace, window, cx)
            },
        );
        workspace.register_action(move |_, _: &ResetOnboarding, window, cx| {
            window.dispatch_action(workspace::RestoreBanner.boxed_clone(), cx);
            window.refresh();
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

            workspace.toggle_modal(window, cx, |window, cx| {
                GitCloneModal::show(panel, window, cx)
            });
        });
        workspace.register_action(|workspace, _: &git::OpenModifiedFiles, window, cx| {
            open_modified_files(workspace, window, cx);
        });
        workspace.register_action(
            |workspace, action: &DiffClipboardWithSelectionData, window, cx| {
                if let Some(task) = TextDiffView::open(action, workspace, window, cx) {
                    task.detach();
                };
            },
        );
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

pub fn git_status_icon(status: FileStatus) -> impl IntoElement {
    GitStatusIcon::new(status)
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
            move |window, cx| {
                git_action_tooltip(
                    "Fetch updates from remote",
                    &git::Fetch,
                    "git fetch",
                    keybinding_target.clone(),
                    window,
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
            move |window, cx| {
                git_action_tooltip(
                    "Push committed changes to remote",
                    &git::Push,
                    "git push",
                    keybinding_target.clone(),
                    window,
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
            move |window, cx| {
                git_action_tooltip(
                    "Pull",
                    &git::Pull,
                    "git pull",
                    keybinding_target.clone(),
                    window,
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
            move |window, cx| {
                git_action_tooltip(
                    "Publish branch to remote",
                    &git::Push,
                    "git push --set-upstream",
                    keybinding_target.clone(),
                    window,
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
            move |window, cx| {
                git_action_tooltip(
                    "Re-publish branch to remote",
                    &git::Push,
                    "git push --set-upstream",
                    keybinding_target.clone(),
                    window,
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
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let label = label.into();
        let command = command.into();

        if let Some(handle) = focus_handle {
            Tooltip::with_meta_in(label, Some(action), command, &handle, window, cx)
        } else {
            Tooltip::with_meta(label, Some(action), command, window, cx)
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
                    .mr_1()
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
                    .mr_1()
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
    panel: Entity<GitPanel>,
    repo_input: Entity<Editor>,
    focus_handle: FocusHandle,
}

impl GitCloneModal {
    pub fn show(panel: Entity<GitPanel>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let repo_input = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter repository URL…", window, cx);
            editor
        });
        let focus_handle = repo_input.focus_handle(cx);

        window.focus(&focus_handle);

        Self {
            panel,
            repo_input,
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
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .flex_1()
            .overflow_hidden()
            .child(
                div()
                    .w_full()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.repo_input.clone()),
            )
            .child(
                h_flex()
                    .w_full()
                    .p_2()
                    .gap_0p5()
                    .rounded_b_sm()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Label::new("Clone a repository from GitHub or other sources.")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .child(
                        Button::new("learn-more", "Learn More")
                            .label_size(LabelSize::Small)
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .on_click(|_, _, cx| {
                                cx.open_url("https://github.com/git-guides/git-clone");
                            }),
                    ),
            )
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                let repo = this.repo_input.read(cx).text(cx);
                this.panel.update(cx, |panel, cx| {
                    panel.git_clone(repo, window, cx);
                });
                cx.emit(DismissEvent);
            }))
    }
}

impl EventEmitter<DismissEvent> for GitCloneModal {}

impl ModalView for GitCloneModal {}
