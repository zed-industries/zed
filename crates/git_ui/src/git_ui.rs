use ::settings::Settings;
use git::{
    repository::{Branch, Upstream, UpstreamTracking, UpstreamTrackingStatus},
    status::FileStatus,
};
use git_panel_settings::GitPanelSettings;
use gpui::{App, Entity, FocusHandle};
use project::Project;
use project_diff::ProjectDiff;
use ui::{ActiveTheme, Color, Icon, IconName, IntoElement, SharedString};
use workspace::Workspace;

mod askpass_modal;
pub mod branch_picker;
mod commit_modal;
pub mod git_panel;
mod git_panel_settings;
pub mod picker_prompt;
pub mod project_diff;
mod remote_output_toast;
pub mod repository_selector;

pub fn init(cx: &mut App) {
    GitPanelSettings::register(cx);
    branch_picker::init(cx);
    cx.observe_new(ProjectDiff::register).detach();
    commit_modal::init(cx);
    git_panel::init(cx);

    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        let project = workspace.project().read(cx);
        if project.is_via_collab() {
            return;
        }
        workspace.register_action(|workspace, _: &git::Fetch, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.fetch(window, cx);
            });
        });
        workspace.register_action(|workspace, _: &git::Push, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.push(false, window, cx);
            });
        });
        workspace.register_action(|workspace, _: &git::ForcePush, window, cx| {
            let Some(panel) = workspace.panel::<git_panel::GitPanel>(cx) else {
                return;
            };
            panel.update(cx, |panel, cx| {
                panel.push(true, window, cx);
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
    })
    .detach();
}

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: FileStatus, cx: &App) -> impl IntoElement {
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

fn can_push_and_pull(project: &Entity<Project>, cx: &App) -> bool {
    !project.read(cx).is_via_collab()
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
                keybinding_target.clone(),
                id,
                ahead,
            )),
            (ahead, behind) => Some(remote_button::render_pull_button(
                keybinding_target.clone(),
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
    use gpui::{hsla, point, Action, AnyView, BoxShadow, ClickEvent, Corner, FocusHandle};
    use ui::{
        div, h_flex, px, rems, ActiveTheme, AnyElement, App, ButtonCommon, ButtonLike, Clickable,
        ContextMenu, ElementId, ElevationIndex, FluentBuilder, Icon, IconName, IconSize,
        IntoElement, Label, LabelCommon, LabelSize, LineHeightStyle, ParentElement, PopoverMenu,
        RenderOnce, SharedString, Styled, Tooltip, Window,
    };

    pub fn render_fetch_button(
        keybinding_target: Option<FocusHandle>,
        id: SharedString,
    ) -> SplitButton {
        SplitButton::new(
            id,
            "Fetch",
            0,
            0,
            Some(IconName::ArrowCircle),
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
        SplitButton::new(
            id,
            "Push",
            ahead as usize,
            0,
            None,
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
        SplitButton::new(
            id,
            "Pull",
            ahead as usize,
            behind as usize,
            None,
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
        SplitButton::new(
            id,
            "Publish",
            0,
            0,
            Some(IconName::ArrowUpFromLine),
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
        SplitButton::new(
            id,
            "Republish",
            0,
            0,
            Some(IconName::ArrowUpFromLine),
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
            Tooltip::with_meta_in(
                label.clone(),
                Some(action),
                command.clone(),
                &handle,
                window,
                cx,
            )
        } else {
            Tooltip::with_meta(label.clone(), Some(action), command.clone(), window, cx)
        }
    }

    fn render_git_action_menu(id: impl Into<ElementId>) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                ui::ButtonLike::new_rounded_right("split-button-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ui::ButtonSize::None)
                    .child(
                        div()
                            .px_1()
                            .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall)),
                    ),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                    context_menu
                        .action("Fetch", git::Fetch.boxed_clone())
                        .action("Pull", git::Pull.boxed_clone())
                        .separator()
                        .action("Push", git::Push.boxed_clone())
                        .action("Force Push", git::ForcePush.boxed_clone())
                }))
            })
            .anchor(Corner::TopRight)
    }

    #[derive(IntoElement)]
    pub struct SplitButton {
        pub left: ButtonLike,
        pub right: AnyElement,
    }

    impl SplitButton {
        fn new(
            id: impl Into<SharedString>,
            left_label: impl Into<SharedString>,
            ahead_count: usize,
            behind_count: usize,
            left_icon: Option<IconName>,
            left_on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
        ) -> Self {
            let id = id.into();

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

            let right = render_git_action_menu(ElementId::Name(
                format!("split-button-right-{}", id).into(),
            ))
            .into_any_element();

            Self { left, right }
        }
    }

    impl RenderOnce for SplitButton {
        fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
            h_flex()
                .rounded_sm()
                .border_1()
                .border_color(cx.theme().colors().text_muted.alpha(0.12))
                .child(div().flex_grow().child(self.left))
                .child(
                    div()
                        .h_full()
                        .w_px()
                        .bg(cx.theme().colors().text_muted.alpha(0.16)),
                )
                .child(self.right)
                .bg(ElevationIndex::Surface.on_elevation_bg(cx))
                .shadow(smallvec::smallvec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.16),
                    offset: point(px(0.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                }])
        }
    }
}
