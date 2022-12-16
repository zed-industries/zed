use crate::{contact_notification::ContactNotification, contacts_popover};
use call::{ActiveCall, ParticipantLocation};
use client::{proto::PeerId, Authenticate, ContactEventKind, User, UserStore};
use clock::ReplicaId;
use contacts_popover::ContactsPopover;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    json::{self, ToJson},
    Border, CursorStyle, Entity, ModelHandle, MouseButton, MutableAppContext, RenderContext,
    Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use std::ops::Range;
use theme::Theme;
use workspace::{FollowNextCollaborator, JoinProject, ToggleFollow, Workspace};

actions!(
    collab,
    [ToggleCollaborationMenu, ToggleScreenSharing, ShareProject]
);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CollabTitlebarItem::toggle_contacts_popover);
    cx.add_action(CollabTitlebarItem::toggle_screen_sharing);
    cx.add_action(CollabTitlebarItem::share_project);
}

pub struct CollabTitlebarItem {
    workspace: WeakViewHandle<Workspace>,
    user_store: ModelHandle<UserStore>,
    contacts_popover: Option<ViewHandle<ContactsPopover>>,
    _subscriptions: Vec<Subscription>,
}

impl Entity for CollabTitlebarItem {
    type Event = ();
}

impl View for CollabTitlebarItem {
    fn ui_name() -> &'static str {
        "CollabTitlebarItem"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let workspace = if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace
        } else {
            return Empty::new().boxed();
        };

        let theme = cx.global::<Settings>().theme.clone();

        let mut container = Flex::row();

        container.add_children(self.render_toggle_screen_sharing_button(&theme, cx));

        if workspace.read(cx).client().status().borrow().is_connected() {
            let project = workspace.read(cx).project().read(cx);
            if project.is_shared()
                || project.is_remote()
                || ActiveCall::global(cx).read(cx).room().is_none()
            {
                container.add_child(self.render_toggle_contacts_button(&theme, cx));
            } else {
                container.add_child(self.render_share_button(&theme, cx));
            }
        }
        container.add_children(self.render_collaborators(&workspace, &theme, cx));
        container.add_children(self.render_current_user(&workspace, &theme, cx));
        container.add_children(self.render_connection_status(&workspace, cx));
        container.boxed()
    }
}

impl CollabTitlebarItem {
    pub fn new(
        workspace: &ViewHandle<Workspace>,
        user_store: &ModelHandle<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let active_call = ActiveCall::global(cx);
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(workspace, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe(&active_call, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe_window_activation(|this, active, cx| {
            this.window_activation_changed(active, cx)
        }));
        subscriptions.push(cx.observe(user_store, |_, _, cx| cx.notify()));
        subscriptions.push(
            cx.subscribe(user_store, move |this, user_store, event, cx| {
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
            workspace: workspace.downgrade(),
            user_store: user_store.clone(),
            contacts_popover: None,
            _subscriptions: subscriptions,
        }
    }

    fn window_activation_changed(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            let project = if active {
                Some(workspace.read(cx).project().clone())
            } else {
                None
            };
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(project.as_ref(), cx))
                .detach_and_log_err(cx);
        }
    }

    fn share_project(&mut self, _: &ShareProject, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            let active_call = ActiveCall::global(cx);
            let project = workspace.read(cx).project().clone();
            active_call
                .update(cx, |call, cx| call.share_project(project, cx))
                .detach_and_log_err(cx);
        }
    }

    pub fn toggle_contacts_popover(
        &mut self,
        _: &ToggleCollaborationMenu,
        cx: &mut ViewContext<Self>,
    ) {
        match self.contacts_popover.take() {
            Some(_) => {}
            None => {
                if let Some(workspace) = self.workspace.upgrade(cx) {
                    let project = workspace.read(cx).project().clone();
                    let user_store = workspace.read(cx).user_store().clone();
                    let view = cx.add_view(|cx| ContactsPopover::new(project, user_store, cx));
                    cx.subscribe(&view, |this, _, event, cx| {
                        match event {
                            contacts_popover::Event::Dismissed => {
                                this.contacts_popover = None;
                            }
                        }

                        cx.notify();
                    })
                    .detach();
                    self.contacts_popover = Some(view);
                }
            }
        }
        cx.notify();
    }

    pub fn toggle_screen_sharing(&mut self, _: &ToggleScreenSharing, cx: &mut ViewContext<Self>) {
        if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
            let toggle_screen_sharing = room.update(cx, |room, cx| {
                if room.is_screen_sharing() {
                    Task::ready(room.unshare_screen(cx))
                } else {
                    room.share_screen(cx)
                }
            });
            toggle_screen_sharing.detach_and_log_err(cx);
        }
    }

    fn render_toggle_contacts_button(
        &self,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let titlebar = &theme.workspace.titlebar;
        let badge = if self
            .user_store
            .read(cx)
            .incoming_contact_requests()
            .is_empty()
        {
            None
        } else {
            Some(
                Empty::new()
                    .collapsed()
                    .contained()
                    .with_style(titlebar.toggle_contacts_badge)
                    .contained()
                    .with_margin_left(titlebar.toggle_contacts_button.default.icon_width)
                    .with_margin_top(titlebar.toggle_contacts_button.default.icon_width)
                    .aligned()
                    .boxed(),
            )
        };
        Stack::new()
            .with_child(
                MouseEventHandler::<ToggleCollaborationMenu>::new(0, cx, |state, _| {
                    let style = titlebar
                        .toggle_contacts_button
                        .style_for(state, self.contacts_popover.is_some());
                    Svg::new("icons/plus_8.svg")
                        .with_color(style.color)
                        .constrained()
                        .with_width(style.icon_width)
                        .aligned()
                        .constrained()
                        .with_width(style.button_width)
                        .with_height(style.button_width)
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(ToggleCollaborationMenu);
                })
                .aligned()
                .boxed(),
            )
            .with_children(badge)
            .with_children(self.contacts_popover.as_ref().map(|popover| {
                Overlay::new(
                    ChildView::new(popover, cx)
                        .contained()
                        .with_margin_top(titlebar.height)
                        .with_margin_left(titlebar.toggle_contacts_button.default.button_width)
                        .with_margin_right(-titlebar.toggle_contacts_button.default.button_width)
                        .boxed(),
                )
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_corner(AnchorCorner::BottomLeft)
                .with_z_index(999)
                .boxed()
            }))
            .boxed()
    }

    fn render_toggle_screen_sharing_button(
        &self,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let active_call = ActiveCall::global(cx);
        let room = active_call.read(cx).room().cloned()?;
        let icon;
        let tooltip;

        if room.read(cx).is_screen_sharing() {
            icon = "icons/disable_screen_sharing_12.svg";
            tooltip = "Stop Sharing Screen"
        } else {
            icon = "icons/enable_screen_sharing_12.svg";
            tooltip = "Share Screen";
        }

        let titlebar = &theme.workspace.titlebar;
        Some(
            MouseEventHandler::<ToggleScreenSharing>::new(0, cx, |state, _| {
                let style = titlebar.call_control.style_for(state, false);
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
                    .boxed()
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, move |_, cx| {
                cx.dispatch_action(ToggleScreenSharing);
            })
            .with_tooltip::<ToggleScreenSharing, _>(
                0,
                tooltip.into(),
                Some(Box::new(ToggleScreenSharing)),
                theme.tooltip.clone(),
                cx,
            )
            .aligned()
            .boxed(),
        )
    }

    fn render_share_button(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Share {}

        let titlebar = &theme.workspace.titlebar;
        MouseEventHandler::<Share>::new(0, cx, |state, _| {
            let style = titlebar.share_button.style_for(state, false);
            Label::new("Share".into(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(ShareProject))
        .with_tooltip::<Share, _>(
            0,
            "Share project with call participants".into(),
            None,
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .contained()
        .with_margin_left(theme.workspace.titlebar.avatar_margin)
        .boxed()
    }

    fn render_collaborators(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Vec<ElementBox> {
        let active_call = ActiveCall::global(cx);
        if let Some(room) = active_call.read(cx).room().cloned() {
            let project = workspace.read(cx).project().read(cx);
            let mut participants = room
                .read(cx)
                .remote_participants()
                .values()
                .cloned()
                .collect::<Vec<_>>();
            participants.sort_by_key(|p| Some(project.collaborators().get(&p.peer_id)?.replica_id));
            participants
                .into_iter()
                .filter_map(|participant| {
                    let project = workspace.read(cx).project().read(cx);
                    let replica_id = project
                        .collaborators()
                        .get(&participant.peer_id)
                        .map(|collaborator| collaborator.replica_id);
                    let user = participant.user.clone();
                    Some(self.render_avatar(
                        &user,
                        replica_id,
                        Some((
                            participant.peer_id,
                            &user.github_login,
                            participant.location,
                        )),
                        workspace,
                        theme,
                        cx,
                    ))
                })
                .collect()
        } else {
            Default::default()
        }
    }

    fn render_current_user(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let user = workspace.read(cx).user_store().read(cx).current_user();
        let replica_id = workspace.read(cx).project().read(cx).replica_id();
        let status = *workspace.read(cx).client().status().borrow();
        if let Some(user) = user {
            Some(self.render_avatar(&user, Some(replica_id), None, workspace, theme, cx))
        } else if matches!(status, client::Status::UpgradeRequired) {
            None
        } else {
            Some(
                MouseEventHandler::<Authenticate>::new(0, cx, |state, _| {
                    let style = theme
                        .workspace
                        .titlebar
                        .sign_in_prompt
                        .style_for(state, false);
                    Label::new("Sign in".to_string(), style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(Authenticate))
                .with_cursor_style(CursorStyle::PointingHand)
                .aligned()
                .boxed(),
            )
        }
    }

    fn render_avatar(
        &self,
        user: &User,
        replica_id: Option<ReplicaId>,
        peer: Option<(PeerId, &str, ParticipantLocation)>,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let is_followed = peer.map_or(false, |(peer_id, _, _)| {
            workspace.read(cx).is_following(peer_id)
        });

        let mut avatar_style;
        if let Some((_, _, location)) = peer.as_ref() {
            if let ParticipantLocation::SharedProject { project_id } = *location {
                if Some(project_id) == workspace.read(cx).project().read(cx).remote_id() {
                    avatar_style = theme.workspace.titlebar.avatar;
                } else {
                    avatar_style = theme.workspace.titlebar.inactive_avatar;
                }
            } else {
                avatar_style = theme.workspace.titlebar.inactive_avatar;
            }
        } else {
            avatar_style = theme.workspace.titlebar.avatar;
        }

        let mut replica_color = None;
        if let Some(replica_id) = replica_id {
            let color = theme.editor.replica_selection_style(replica_id).cursor;
            replica_color = Some(color);
            if is_followed {
                avatar_style.border = Border::all(1.0, color);
            }
        }

        let content = Stack::new()
            .with_children(user.avatar.as_ref().map(|avatar| {
                Image::new(avatar.clone())
                    .with_style(avatar_style)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_width)
                    .aligned()
                    .boxed()
            }))
            .with_children(replica_color.map(|replica_color| {
                AvatarRibbon::new(replica_color)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_ribbon.width)
                    .with_height(theme.workspace.titlebar.avatar_ribbon.height)
                    .aligned()
                    .bottom()
                    .boxed()
            }))
            .constrained()
            .with_width(theme.workspace.titlebar.avatar_width)
            .contained()
            .with_margin_left(theme.workspace.titlebar.avatar_margin)
            .boxed();

        if let Some((peer_id, peer_github_login, location)) = peer {
            if let Some(replica_id) = replica_id {
                MouseEventHandler::<ToggleFollow>::new(replica_id.into(), cx, move |_, _| content)
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_click(MouseButton::Left, move |_, cx| {
                        cx.dispatch_action(ToggleFollow(peer_id))
                    })
                    .with_tooltip::<ToggleFollow, _>(
                        peer_id.as_u64() as usize,
                        if is_followed {
                            format!("Unfollow {}", peer_github_login)
                        } else {
                            format!("Follow {}", peer_github_login)
                        },
                        Some(Box::new(FollowNextCollaborator)),
                        theme.tooltip.clone(),
                        cx,
                    )
                    .boxed()
            } else if let ParticipantLocation::SharedProject { project_id } = location {
                let user_id = user.id;
                MouseEventHandler::<JoinProject>::new(peer_id.as_u64() as usize, cx, move |_, _| {
                    content
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(JoinProject {
                        project_id,
                        follow_user_id: user_id,
                    })
                })
                .with_tooltip::<JoinProject, _>(
                    peer_id.as_u64() as usize,
                    format!("Follow {} into external project", peer_github_login),
                    Some(Box::new(FollowNextCollaborator)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed()
            } else {
                content
            }
        } else {
            content
        }
    }

    fn render_connection_status(
        &self,
        workspace: &ViewHandle<Workspace>,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let theme = &cx.global::<Settings>().theme;
        match &*workspace.read(cx).client().status().borrow() {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating { .. }
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                Container::new(
                    Align::new(
                        ConstrainedBox::new(
                            Svg::new("icons/cloud_slash_12.svg")
                                .with_color(theme.workspace.titlebar.offline_icon.color)
                                .boxed(),
                        )
                        .with_width(theme.workspace.titlebar.offline_icon.width)
                        .boxed(),
                    )
                    .boxed(),
                )
                .with_style(theme.workspace.titlebar.offline_icon.container)
                .boxed(),
            ),
            client::Status::UpgradeRequired => Some(
                Label::new(
                    "Please update Zed to collaborate".to_string(),
                    theme.workspace.titlebar.outdated_warning.text.clone(),
                )
                .contained()
                .with_style(theme.workspace.titlebar.outdated_warning.container)
                .aligned()
                .boxed(),
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

impl Element for AvatarRibbon {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        _: gpui::geometry::rect::RectF,
        _: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
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
        cx.scene.push_path(path.build(self.color, None));
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::MeasurementContext,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::DebugContext,
    ) -> gpui::json::Value {
        json::json!({
            "type": "AvatarRibbon",
            "bounds": bounds.to_json(),
            "color": self.color.to_json(),
        })
    }
}
