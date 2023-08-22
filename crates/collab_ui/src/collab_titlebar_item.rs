use crate::{
    contact_notification::ContactNotification, face_pile::FacePile, toggle_deafen, toggle_mute,
    toggle_screen_sharing, LeaveCall, ToggleDeafen, ToggleMute, ToggleScreenSharing,
};
use call::{ActiveCall, ParticipantLocation, Room};
use client::{proto::PeerId, Client, ContactEventKind, SignIn, SignOut, User, UserStore};
use clock::ReplicaId;
use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    json::{self, ToJson},
    platform::{CursorStyle, MouseButton},
    AppContext, Entity, ImageData, LayoutContext, ModelHandle, PaintContext, SceneBuilder,
    Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use picker::PickerEvent;
use project::{Project, RepositoryEntry};
use recent_projects::{build_recent_projects, RecentProjects};
use std::{ops::Range, sync::Arc};
use theme::{AvatarStyle, Theme};
use util::ResultExt;
use vcs_menu::{build_branch_list, BranchList, OpenRecent as ToggleVcsMenu};
use workspace::{FollowNextCollaborator, Workspace, WORKSPACE_DB};

const MAX_PROJECT_NAME_LENGTH: usize = 40;
const MAX_BRANCH_NAME_LENGTH: usize = 40;

actions!(
    collab,
    [
        ToggleUserMenu,
        ToggleProjectMenu,
        SwitchBranch,
        ShareProject,
        UnshareProject,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(CollabTitlebarItem::share_project);
    cx.add_action(CollabTitlebarItem::unshare_project);
    cx.add_action(CollabTitlebarItem::toggle_user_menu);
    cx.add_action(CollabTitlebarItem::toggle_vcs_menu);
    cx.add_action(CollabTitlebarItem::toggle_project_menu);
}

pub struct CollabTitlebarItem {
    project: ModelHandle<Project>,
    user_store: ModelHandle<UserStore>,
    client: Arc<Client>,
    workspace: WeakViewHandle<Workspace>,
    branch_popover: Option<ViewHandle<BranchList>>,
    project_popover: Option<ViewHandle<recent_projects::RecentProjects>>,
    user_menu: ViewHandle<ContextMenu>,
    _subscriptions: Vec<Subscription>,
}

impl Entity for CollabTitlebarItem {
    type Event = ();
}

impl View for CollabTitlebarItem {
    fn ui_name() -> &'static str {
        "CollabTitlebarItem"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let workspace = if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace
        } else {
            return Empty::new().into_any();
        };

        let theme = theme::current(cx).clone();
        let mut left_container = Flex::row();
        let mut right_container = Flex::row().align_children_center();

        left_container.add_child(self.collect_title_root_names(theme.clone(), cx));

        let user = self.user_store.read(cx).current_user();
        let peer_id = self.client.peer_id();
        if let Some(((user, peer_id), room)) = user
            .as_ref()
            .zip(peer_id)
            .zip(ActiveCall::global(cx).read(cx).room().cloned())
        {
            right_container
                .add_children(self.render_in_call_share_unshare_button(&workspace, &theme, cx));
            right_container.add_child(self.render_leave_call(&theme, cx));
            let muted = room.read(cx).is_muted(cx);
            let speaking = room.read(cx).is_speaking();
            left_container.add_child(
                self.render_current_user(&workspace, &theme, &user, peer_id, muted, speaking, cx),
            );
            left_container.add_children(self.render_collaborators(&workspace, &theme, &room, cx));
            right_container.add_child(self.render_toggle_mute(&theme, &room, cx));
            right_container.add_child(self.render_toggle_deafen(&theme, &room, cx));
            right_container.add_child(self.render_toggle_screen_sharing_button(&theme, &room, cx));
        }

        let status = workspace.read(cx).client().status();
        let status = &*status.borrow();
        if matches!(status, client::Status::Connected { .. }) {
            let avatar = user.as_ref().and_then(|user| user.avatar.clone());
            right_container.add_child(self.render_user_menu_button(&theme, avatar, cx));
        } else {
            right_container.add_children(self.render_connection_status(status, cx));
            right_container.add_child(self.render_sign_in_button(&theme, cx));
            right_container.add_child(self.render_user_menu_button(&theme, None, cx));
        }

        Stack::new()
            .with_child(left_container)
            .with_child(
                Flex::row()
                    .with_child(
                        right_container.contained().with_background_color(
                            theme
                                .titlebar
                                .container
                                .background_color
                                .unwrap_or_else(|| Color::transparent_black()),
                        ),
                    )
                    .aligned()
                    .right(),
            )
            .into_any()
    }
}

impl CollabTitlebarItem {
    pub fn new(
        workspace: &Workspace,
        workspace_handle: &ViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let project = workspace.project().clone();
        let user_store = workspace.app_state().user_store.clone();
        let client = workspace.app_state().client.clone();
        let active_call = ActiveCall::global(cx);
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(workspace_handle, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe(&project, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));
        subscriptions.push(cx.observe_window_activation(|this, active, cx| {
            this.window_activation_changed(active, cx)
        }));
        subscriptions.push(cx.observe(&user_store, |_, _, cx| cx.notify()));
        subscriptions.push(
            cx.subscribe(&user_store, move |this, user_store, event, cx| {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    workspace.update(cx, |workspace, cx| {
                        if let client::Event::Contact { user, kind } = event {
                            if let ContactEventKind::Requested | ContactEventKind::Accepted = kind {
                                workspace.show_notification(user.id as usize, cx, |cx| {
                                    cx.add_view(|cx| {
                                        ContactNotification::new(
                                            user.clone(),
                                            *kind,
                                            user_store,
                                            cx,
                                        )
                                    })
                                })
                            }
                        }
                    });
                }
            }),
        );

        Self {
            workspace: workspace.weak_handle(),
            project,
            user_store,
            client,
            user_menu: cx.add_view(|cx| {
                let view_id = cx.view_id();
                let mut menu = ContextMenu::new(view_id, cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
            branch_popover: None,
            project_popover: None,
            _subscriptions: subscriptions,
        }
    }

    fn collect_title_root_names(
        &self,
        theme: Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let project = self.project.read(cx);

        let (name, entry) = {
            let mut names_and_branches = project.visible_worktrees(cx).map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.root_name(), worktree.root_git_entry())
            });

            names_and_branches.next().unwrap_or(("", None))
        };

        let name = util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH);
        let branch_prepended = entry
            .as_ref()
            .and_then(RepositoryEntry::branch)
            .map(|branch| util::truncate_and_trailoff(&branch, MAX_BRANCH_NAME_LENGTH));
        let project_style = theme.titlebar.project_menu_button.clone();
        let git_style = theme.titlebar.git_menu_button.clone();
        let divider_style = theme.titlebar.project_name_divider.clone();
        let item_spacing = theme.titlebar.item_spacing;

        let mut ret = Flex::row().with_child(
            Stack::new()
                .with_child(
                    MouseEventHandler::new::<ToggleProjectMenu, _>(0, cx, |mouse_state, cx| {
                        let style = project_style
                            .in_state(self.project_popover.is_some())
                            .style_for(mouse_state);
                        enum RecentProjectsTooltip {}
                        Label::new(name, style.text.clone())
                            .contained()
                            .with_style(style.container)
                            .aligned()
                            .left()
                            .with_tooltip::<RecentProjectsTooltip>(
                                0,
                                "Recent projects",
                                Some(Box::new(recent_projects::OpenRecent)),
                                theme.tooltip.clone(),
                                cx,
                            )
                            .into_any_named("title-project-name")
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_down(MouseButton::Left, move |_, this, cx| {
                        this.toggle_project_menu(&Default::default(), cx)
                    })
                    .on_click(MouseButton::Left, move |_, _, _| {}),
                )
                .with_children(self.render_project_popover_host(&theme.titlebar, cx)),
        );
        if let Some(git_branch) = branch_prepended {
            ret = ret.with_child(
                Flex::row()
                    .with_child(
                        Label::new("/", divider_style.text)
                            .contained()
                            .with_style(divider_style.container)
                            .aligned()
                            .left(),
                    )
                    .with_child(
                        Stack::new()
                            .with_child(
                                MouseEventHandler::new::<ToggleVcsMenu, _>(
                                    0,
                                    cx,
                                    |mouse_state, cx| {
                                        enum BranchPopoverTooltip {}
                                        let style = git_style
                                            .in_state(self.branch_popover.is_some())
                                            .style_for(mouse_state);
                                        Label::new(git_branch, style.text.clone())
                                            .contained()
                                            .with_style(style.container.clone())
                                            .with_margin_right(item_spacing)
                                            .aligned()
                                            .left()
                                            .with_tooltip::<BranchPopoverTooltip>(
                                                0,
                                                "Recent branches",
                                                Some(Box::new(ToggleVcsMenu)),
                                                theme.tooltip.clone(),
                                                cx,
                                            )
                                            .into_any_named("title-project-branch")
                                    },
                                )
                                .with_cursor_style(CursorStyle::PointingHand)
                                .on_down(MouseButton::Left, move |_, this, cx| {
                                    this.toggle_vcs_menu(&Default::default(), cx)
                                })
                                .on_click(MouseButton::Left, move |_, _, _| {}),
                            )
                            .with_children(self.render_branches_popover_host(&theme.titlebar, cx)),
                    ),
            )
        }
        ret.into_any()
    }

    fn window_activation_changed(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        let project = if active {
            Some(self.project.clone())
        } else {
            None
        };
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.set_location(project.as_ref(), cx))
            .detach_and_log_err(cx);
    }

    fn active_call_changed(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn share_project(&mut self, _: &ShareProject, cx: &mut ViewContext<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.share_project(project, cx))
            .detach_and_log_err(cx);
    }

    fn unshare_project(&mut self, _: &UnshareProject, cx: &mut ViewContext<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.unshare_project(project, cx))
            .log_err();
    }

    pub fn toggle_user_menu(&mut self, _: &ToggleUserMenu, cx: &mut ViewContext<Self>) {
        self.user_menu.update(cx, |user_menu, cx| {
            let items = if let Some(_) = self.user_store.read(cx).current_user() {
                vec![
                    ContextMenuItem::action("Settings", zed_actions::OpenSettings),
                    ContextMenuItem::action("Theme", theme_selector::Toggle),
                    ContextMenuItem::separator(),
                    ContextMenuItem::action(
                        "Share Feedback",
                        feedback::feedback_editor::GiveFeedback,
                    ),
                    ContextMenuItem::action("Sign Out", SignOut),
                ]
            } else {
                vec![
                    ContextMenuItem::action("Settings", zed_actions::OpenSettings),
                    ContextMenuItem::action("Theme", theme_selector::Toggle),
                    ContextMenuItem::separator(),
                    ContextMenuItem::action(
                        "Share Feedback",
                        feedback::feedback_editor::GiveFeedback,
                    ),
                ]
            };
            user_menu.toggle(Default::default(), AnchorCorner::TopRight, items, cx);
        });
    }

    fn render_branches_popover_host<'a>(
        &'a self,
        _theme: &'a theme::Titlebar,
        cx: &'a mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        self.branch_popover.as_ref().map(|child| {
            let theme = theme::current(cx).clone();
            let child = ChildView::new(child, cx);
            let child = MouseEventHandler::new::<BranchList, _>(0, cx, |_, _| {
                child
                    .flex(1., true)
                    .contained()
                    .constrained()
                    .with_width(theme.titlebar.menu.width)
                    .with_height(theme.titlebar.menu.height)
            })
            .on_click(MouseButton::Left, |_, _, _| {})
            .on_down_out(MouseButton::Left, move |_, this, cx| {
                this.branch_popover.take();
                cx.emit(());
                cx.notify();
            })
            .contained()
            .into_any();

            Overlay::new(child)
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_corner(AnchorCorner::TopLeft)
                .with_z_index(999)
                .aligned()
                .bottom()
                .left()
                .into_any()
        })
    }

    fn render_project_popover_host<'a>(
        &'a self,
        _theme: &'a theme::Titlebar,
        cx: &'a mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        self.project_popover.as_ref().map(|child| {
            let theme = theme::current(cx).clone();
            let child = ChildView::new(child, cx);
            let child = MouseEventHandler::new::<RecentProjects, _>(0, cx, |_, _| {
                child
                    .flex(1., true)
                    .contained()
                    .constrained()
                    .with_width(theme.titlebar.menu.width)
                    .with_height(theme.titlebar.menu.height)
            })
            .on_click(MouseButton::Left, |_, _, _| {})
            .on_down_out(MouseButton::Left, move |_, this, cx| {
                this.project_popover.take();
                cx.emit(());
                cx.notify();
            })
            .into_any();

            Overlay::new(child)
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_corner(AnchorCorner::TopLeft)
                .with_z_index(999)
                .aligned()
                .bottom()
                .left()
                .into_any()
        })
    }

    pub fn toggle_vcs_menu(&mut self, _: &ToggleVcsMenu, cx: &mut ViewContext<Self>) {
        if self.branch_popover.take().is_none() {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                let view = cx.add_view(|cx| build_branch_list(workspace, cx));
                cx.subscribe(&view, |this, _, event, cx| {
                    match event {
                        PickerEvent::Dismiss => {
                            this.branch_popover = None;
                        }
                    }

                    cx.notify();
                })
                .detach();
                self.project_popover.take();
                cx.focus(&view);
                self.branch_popover = Some(view);
            }
        }

        cx.notify();
    }

    pub fn toggle_project_menu(&mut self, _: &ToggleProjectMenu, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        if self.project_popover.take().is_none() {
            cx.spawn(|this, mut cx| async move {
                let workspaces = WORKSPACE_DB
                    .recent_workspaces_on_disk()
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(_, location)| location)
                    .collect();

                let workspace = workspace.clone();
                this.update(&mut cx, move |this, cx| {
                    let view = cx.add_view(|cx| build_recent_projects(workspace, workspaces, cx));

                    cx.subscribe(&view, |this, _, event, cx| {
                        match event {
                            PickerEvent::Dismiss => {
                                this.project_popover = None;
                            }
                        }

                        cx.notify();
                    })
                    .detach();
                    cx.focus(&view);
                    this.branch_popover.take();
                    this.project_popover = Some(view);
                    cx.notify();
                })
                .log_err();
            })
            .detach();
        }
        cx.notify();
    }

    fn render_toggle_screen_sharing_button(
        &self,
        theme: &Theme,
        room: &ModelHandle<Room>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let icon;
        let tooltip;
        if room.read(cx).is_screen_sharing() {
            icon = "icons/radix/desktop.svg";
            tooltip = "Stop Sharing Screen"
        } else {
            icon = "icons/radix/desktop.svg";
            tooltip = "Share Screen";
        }

        let active = room.read(cx).is_screen_sharing();
        let titlebar = &theme.titlebar;
        MouseEventHandler::new::<ToggleScreenSharing, _>(0, cx, |state, _| {
            let style = titlebar
                .screen_share_button
                .in_state(active)
                .style_for(state);

            Svg::new(icon)
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .with_height(style.button_width)
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, _, cx| {
            toggle_screen_sharing(&Default::default(), cx)
        })
        .with_tooltip::<ToggleScreenSharing>(
            0,
            tooltip,
            Some(Box::new(ToggleScreenSharing)),
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .into_any()
    }
    fn render_toggle_mute(
        &self,
        theme: &Theme,
        room: &ModelHandle<Room>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let icon;
        let tooltip;
        let is_muted = room.read(cx).is_muted(cx);
        if is_muted {
            icon = "icons/radix/mic-mute.svg";
            tooltip = "Unmute microphone";
        } else {
            icon = "icons/radix/mic.svg";
            tooltip = "Mute microphone";
        }

        let titlebar = &theme.titlebar;
        MouseEventHandler::new::<ToggleMute, _>(0, cx, |state, _| {
            let style = titlebar
                .toggle_microphone_button
                .in_state(is_muted)
                .style_for(state);
            let image = Svg::new(icon)
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .with_height(style.button_width)
                .contained()
                .with_style(style.container);
            if let Some(color) = style.container.background_color {
                image.with_background_color(color)
            } else {
                image
            }
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, _, cx| {
            toggle_mute(&Default::default(), cx)
        })
        .with_tooltip::<ToggleMute>(
            0,
            tooltip,
            Some(Box::new(ToggleMute)),
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .into_any()
    }
    fn render_toggle_deafen(
        &self,
        theme: &Theme,
        room: &ModelHandle<Room>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let icon;
        let tooltip;
        let is_deafened = room.read(cx).is_deafened().unwrap_or(false);
        if is_deafened {
            icon = "icons/radix/speaker-off.svg";
            tooltip = "Unmute speakers";
        } else {
            icon = "icons/radix/speaker-loud.svg";
            tooltip = "Mute speakers";
        }

        let titlebar = &theme.titlebar;
        MouseEventHandler::new::<ToggleDeafen, _>(0, cx, |state, _| {
            let style = titlebar
                .toggle_speakers_button
                .in_state(is_deafened)
                .style_for(state);
            Svg::new(icon)
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .with_height(style.button_width)
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, _, cx| {
            toggle_deafen(&Default::default(), cx)
        })
        .with_tooltip::<ToggleDeafen>(
            0,
            tooltip,
            Some(Box::new(ToggleDeafen)),
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .into_any()
    }
    fn render_leave_call(&self, theme: &Theme, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let icon = "icons/radix/exit.svg";
        let tooltip = "Leave call";

        let titlebar = &theme.titlebar;
        MouseEventHandler::new::<LeaveCall, _>(0, cx, |state, _| {
            let style = titlebar.leave_call_button.style_for(state);
            Svg::new(icon)
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .with_height(style.button_width)
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, _, cx| {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.hang_up(cx))
                .detach_and_log_err(cx);
        })
        .with_tooltip::<LeaveCall>(
            0,
            tooltip,
            Some(Box::new(LeaveCall)),
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .into_any()
    }
    fn render_in_call_share_unshare_button(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        let project = workspace.read(cx).project();
        if project.read(cx).is_remote() {
            return None;
        }

        let is_shared = project.read(cx).is_shared();
        let label = if is_shared { "Stop Sharing" } else { "Share" };
        let tooltip = if is_shared {
            "Stop sharing project with call participants"
        } else {
            "Share project with call participants"
        };

        let titlebar = &theme.titlebar;

        enum ShareUnshare {}
        Some(
            Stack::new()
                .with_child(
                    MouseEventHandler::new::<ShareUnshare, _>(0, cx, |state, _| {
                        //TODO: Ensure this button has consistent width for both text variations
                        let style = titlebar.share_button.inactive_state().style_for(state);
                        Label::new(label, style.text.clone())
                            .contained()
                            .with_style(style.container)
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_click(MouseButton::Left, move |_, this, cx| {
                        if is_shared {
                            this.unshare_project(&Default::default(), cx);
                        } else {
                            this.share_project(&Default::default(), cx);
                        }
                    })
                    .with_tooltip::<ShareUnshare>(
                        0,
                        tooltip.to_owned(),
                        None,
                        theme.tooltip.clone(),
                        cx,
                    ),
                )
                .aligned()
                .contained()
                .with_margin_left(theme.titlebar.item_spacing)
                .into_any(),
        )
    }

    fn render_user_menu_button(
        &self,
        theme: &Theme,
        avatar: Option<Arc<ImageData>>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let tooltip = theme.tooltip.clone();
        let user_menu_button_style = if avatar.is_some() {
            &theme.titlebar.user_menu.user_menu_button_online
        } else {
            &theme.titlebar.user_menu.user_menu_button_offline
        };

        let avatar_style = &user_menu_button_style.avatar;
        Stack::new()
            .with_child(
                MouseEventHandler::new::<ToggleUserMenu, _>(0, cx, |state, _| {
                    let style = user_menu_button_style
                        .user_menu
                        .inactive_state()
                        .style_for(state);

                    let mut dropdown = Flex::row().align_children_center();

                    if let Some(avatar_img) = avatar {
                        dropdown = dropdown.with_child(Self::render_face(
                            avatar_img,
                            *avatar_style,
                            Color::transparent_black(),
                            None,
                        ));
                    };

                    dropdown
                        .with_child(
                            Svg::new("icons/caret_down_8.svg")
                                .with_color(user_menu_button_style.icon.color)
                                .constrained()
                                .with_width(user_menu_button_style.icon.width)
                                .contained()
                                .into_any(),
                        )
                        .aligned()
                        .constrained()
                        .with_height(style.width)
                        .contained()
                        .with_style(style.container)
                        .into_any()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_down(MouseButton::Left, move |_, this, cx| {
                    this.user_menu.update(cx, |menu, _| menu.delay_cancel());
                })
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.toggle_user_menu(&Default::default(), cx)
                })
                .with_tooltip::<ToggleUserMenu>(
                    0,
                    "Toggle user menu".to_owned(),
                    Some(Box::new(ToggleUserMenu)),
                    tooltip,
                    cx,
                )
                .contained(),
            )
            .with_child(
                ChildView::new(&self.user_menu, cx)
                    .aligned()
                    .bottom()
                    .right(),
            )
            .into_any()
    }

    fn render_sign_in_button(&self, theme: &Theme, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let titlebar = &theme.titlebar;
        MouseEventHandler::new::<SignIn, _>(0, cx, |state, _| {
            let style = titlebar.sign_in_button.inactive_state().style_for(state);
            Label::new("Sign In", style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            let client = this.client.clone();
            cx.app_context()
                .spawn(|cx| async move { client.authenticate_and_connect(true, &cx).await })
                .detach_and_log_err(cx);
        })
        .into_any()
    }

    fn render_collaborators(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        room: &ModelHandle<Room>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<Container<Self>> {
        let mut participants = room
            .read(cx)
            .remote_participants()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        participants.sort_by_cached_key(|p| p.user.github_login.clone());

        participants
            .into_iter()
            .filter_map(|participant| {
                let project = workspace.read(cx).project().read(cx);
                let replica_id = project
                    .collaborators()
                    .get(&participant.peer_id)
                    .map(|collaborator| collaborator.replica_id);
                let user = participant.user.clone();
                Some(
                    Container::new(self.render_face_pile(
                        &user,
                        replica_id,
                        participant.peer_id,
                        Some(participant.location),
                        participant.muted,
                        participant.speaking,
                        workspace,
                        theme,
                        cx,
                    ))
                    .with_margin_right(theme.titlebar.face_pile_spacing),
                )
            })
            .collect()
    }

    fn render_current_user(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        user: &Arc<User>,
        peer_id: PeerId,
        muted: bool,
        speaking: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let replica_id = workspace.read(cx).project().read(cx).replica_id();

        Container::new(self.render_face_pile(
            user,
            Some(replica_id),
            peer_id,
            None,
            muted,
            speaking,
            workspace,
            theme,
            cx,
        ))
        .with_margin_right(theme.titlebar.item_spacing)
        .into_any()
    }

    fn render_face_pile(
        &self,
        user: &User,
        replica_id: Option<ReplicaId>,
        peer_id: PeerId,
        location: Option<ParticipantLocation>,
        muted: bool,
        speaking: bool,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let project_id = workspace.read(cx).project().read(cx).remote_id();
        let room = ActiveCall::global(cx).read(cx).room();
        let is_being_followed = workspace.read(cx).is_being_followed(peer_id);
        let followed_by_self = room
            .and_then(|room| {
                Some(
                    is_being_followed
                        && room
                            .read(cx)
                            .followers_for(peer_id, project_id?)
                            .iter()
                            .any(|&follower| {
                                Some(follower) == workspace.read(cx).client().peer_id()
                            }),
                )
            })
            .unwrap_or(false);

        let leader_style = theme.titlebar.leader_avatar;
        let follower_style = theme.titlebar.follower_avatar;

        let microphone_state = if muted {
            Some(theme.titlebar.muted)
        } else if speaking {
            Some(theme.titlebar.speaking)
        } else {
            None
        };

        let mut background_color = theme
            .titlebar
            .container
            .background_color
            .unwrap_or_default();

        if let Some(replica_id) = replica_id {
            if followed_by_self {
                let selection = theme.editor.replica_selection_style(replica_id).selection;
                background_color = Color::blend(selection, background_color);
                background_color.a = 255;
            }
        }

        let mut content = Stack::new()
            .with_children(user.avatar.as_ref().map(|avatar| {
                let face_pile = FacePile::new(theme.titlebar.follower_avatar_overlap)
                    .with_child(Self::render_face(
                        avatar.clone(),
                        Self::location_style(workspace, location, leader_style, cx),
                        background_color,
                        microphone_state,
                    ))
                    .with_children(
                        (|| {
                            let project_id = project_id?;
                            let room = room?.read(cx);
                            let followers = room.followers_for(peer_id, project_id);

                            Some(followers.into_iter().flat_map(|&follower| {
                                let remote_participant =
                                    room.remote_participant_for_peer_id(follower);

                                let avatar = remote_participant
                                    .and_then(|p| p.user.avatar.clone())
                                    .or_else(|| {
                                        if follower == workspace.read(cx).client().peer_id()? {
                                            workspace
                                                .read(cx)
                                                .user_store()
                                                .read(cx)
                                                .current_user()?
                                                .avatar
                                                .clone()
                                        } else {
                                            None
                                        }
                                    })?;

                                Some(Self::render_face(
                                    avatar.clone(),
                                    follower_style,
                                    background_color,
                                    None,
                                ))
                            }))
                        })()
                        .into_iter()
                        .flatten(),
                    );

                let mut container = face_pile
                    .contained()
                    .with_style(theme.titlebar.leader_selection);

                if let Some(replica_id) = replica_id {
                    if followed_by_self {
                        let color = theme.editor.replica_selection_style(replica_id).selection;
                        container = container.with_background_color(color);
                    }
                }

                container
            }))
            .with_children((|| {
                let replica_id = replica_id?;
                let color = theme.editor.replica_selection_style(replica_id).cursor;
                Some(
                    AvatarRibbon::new(color)
                        .constrained()
                        .with_width(theme.titlebar.avatar_ribbon.width)
                        .with_height(theme.titlebar.avatar_ribbon.height)
                        .aligned()
                        .bottom(),
                )
            })())
            .into_any();

        if let Some(location) = location {
            if let Some(replica_id) = replica_id {
                enum ToggleFollow {}

                content = MouseEventHandler::new::<ToggleFollow, _>(
                    replica_id.into(),
                    cx,
                    move |_, _| content,
                )
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, item, cx| {
                    if let Some(workspace) = item.workspace.upgrade(cx) {
                        if let Some(task) = workspace
                            .update(cx, |workspace, cx| workspace.toggle_follow(peer_id, cx))
                        {
                            task.detach_and_log_err(cx);
                        }
                    }
                })
                .with_tooltip::<ToggleFollow>(
                    peer_id.as_u64() as usize,
                    if is_being_followed {
                        format!("Unfollow {}", user.github_login)
                    } else {
                        format!("Follow {}", user.github_login)
                    },
                    Some(Box::new(FollowNextCollaborator)),
                    theme.tooltip.clone(),
                    cx,
                )
                .into_any();
            } else if let ParticipantLocation::SharedProject { project_id } = location {
                enum JoinProject {}

                let user_id = user.id;
                content = MouseEventHandler::new::<JoinProject, _>(
                    peer_id.as_u64() as usize,
                    cx,
                    move |_, _| content,
                )
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    if let Some(workspace) = this.workspace.upgrade(cx) {
                        let app_state = workspace.read(cx).app_state().clone();
                        workspace::join_remote_project(project_id, user_id, app_state, cx)
                            .detach_and_log_err(cx);
                    }
                })
                .with_tooltip::<JoinProject>(
                    peer_id.as_u64() as usize,
                    format!("Follow {} into external project", user.github_login),
                    Some(Box::new(FollowNextCollaborator)),
                    theme.tooltip.clone(),
                    cx,
                )
                .into_any();
            }
        }
        content
    }

    fn location_style(
        workspace: &ViewHandle<Workspace>,
        location: Option<ParticipantLocation>,
        mut style: AvatarStyle,
        cx: &ViewContext<Self>,
    ) -> AvatarStyle {
        if let Some(location) = location {
            if let ParticipantLocation::SharedProject { project_id } = location {
                if Some(project_id) != workspace.read(cx).project().read(cx).remote_id() {
                    style.image.grayscale = true;
                }
            } else {
                style.image.grayscale = true;
            }
        }

        style
    }

    fn render_face<V: View>(
        avatar: Arc<ImageData>,
        avatar_style: AvatarStyle,
        background_color: Color,
        microphone_state: Option<Color>,
    ) -> AnyElement<V> {
        Image::from_data(avatar)
            .with_style(avatar_style.image)
            .aligned()
            .contained()
            .with_background_color(microphone_state.unwrap_or(background_color))
            .with_corner_radius(avatar_style.outer_corner_radius)
            .constrained()
            .with_width(avatar_style.outer_width)
            .with_height(avatar_style.outer_width)
            .aligned()
            .into_any()
    }

    fn render_connection_status(
        &self,
        status: &client::Status,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        enum ConnectionStatusButton {}

        let theme = &theme::current(cx).clone();
        match status {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating { .. }
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                Svg::new("icons/cloud_slash_12.svg")
                    .with_color(theme.titlebar.offline_icon.color)
                    .constrained()
                    .with_width(theme.titlebar.offline_icon.width)
                    .aligned()
                    .contained()
                    .with_style(theme.titlebar.offline_icon.container)
                    .into_any(),
            ),
            client::Status::UpgradeRequired => Some(
                MouseEventHandler::new::<ConnectionStatusButton, _>(0, cx, |_, _| {
                    Label::new(
                        "Please update Zed to collaborate",
                        theme.titlebar.outdated_warning.text.clone(),
                    )
                    .contained()
                    .with_style(theme.titlebar.outdated_warning.container)
                    .aligned()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, _, cx| {
                    auto_update::check(&Default::default(), cx);
                })
                .into_any(),
            ),
            _ => None,
        }
    }
}

pub struct AvatarRibbon {
    color: Color,
}

impl AvatarRibbon {
    pub fn new(color: Color) -> AvatarRibbon {
        AvatarRibbon { color }
    }
}

impl Element<CollabTitlebarItem> for AvatarRibbon {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut CollabTitlebarItem,
        _: &mut LayoutContext<CollabTitlebarItem>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut CollabTitlebarItem,
        _: &mut PaintContext<CollabTitlebarItem>,
    ) -> Self::PaintState {
        let mut path = PathBuilder::new();
        path.reset(bounds.lower_left());
        path.curve_to(
            bounds.origin() + vec2f(bounds.height(), 0.),
            bounds.origin(),
        );
        path.line_to(bounds.upper_right() - vec2f(bounds.height(), 0.));
        path.curve_to(bounds.lower_right(), bounds.upper_right());
        path.line_to(bounds.lower_left());
        scene.push_path(path.build(self.color, None));
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &CollabTitlebarItem,
        _: &ViewContext<CollabTitlebarItem>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &CollabTitlebarItem,
        _: &ViewContext<CollabTitlebarItem>,
    ) -> gpui::json::Value {
        json::json!({
            "type": "AvatarRibbon",
            "bounds": bounds.to_json(),
            "color": self.color.to_json(),
        })
    }
}
