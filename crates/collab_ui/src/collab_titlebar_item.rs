use crate::{
    collaborator_list_popover, collaborator_list_popover::CollaboratorListPopover,
    contact_notification::ContactNotification, contacts_popover, face_pile::FacePile,
    ToggleScreenSharing,
};
use call::{ActiveCall, ParticipantLocation, Room};
use client::{proto::PeerId, Authenticate, ContactEventKind, User, UserStore};
use clock::ReplicaId;
use contacts_popover::ContactsPopover;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    impl_internal_actions,
    json::{self, ToJson},
    CursorStyle, Entity, ImageData, ModelHandle, MouseButton, MutableAppContext, RenderContext,
    Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use std::{ops::Range, sync::Arc};
use theme::{AvatarStyle, Theme};
use util::ResultExt;
use workspace::{FollowNextCollaborator, JoinProject, ToggleFollow, Workspace};

actions!(
    collab,
    [
        ToggleCollaboratorList,
        ToggleContactsMenu,
        ShareProject,
        UnshareProject
    ]
);

impl_internal_actions!(collab, [LeaveCall]);

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct LeaveCall;

#[derive(PartialEq, Eq)]
enum ContactsPopoverSide {
    Left,
    Right,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CollabTitlebarItem::toggle_collaborator_list_popover);
    cx.add_action(CollabTitlebarItem::toggle_contacts_popover);
    cx.add_action(CollabTitlebarItem::share_project);
    cx.add_action(CollabTitlebarItem::unshare_project);
    cx.add_action(CollabTitlebarItem::leave_call);
}

pub struct CollabTitlebarItem {
    workspace: WeakViewHandle<Workspace>,
    user_store: ModelHandle<UserStore>,
    contacts_popover: Option<ViewHandle<ContactsPopover>>,
    contacts_popover_side: ContactsPopoverSide,
    collaborator_list_popover: Option<ViewHandle<CollaboratorListPopover>>,
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

        let project = workspace.read(cx).project().read(cx);
        let mut project_title = String::new();
        for (i, name) in project.worktree_root_names(cx).enumerate() {
            if i > 0 {
                project_title.push_str(", ");
            }
            project_title.push_str(name);
        }
        if project_title.is_empty() {
            project_title = "empty project".to_owned();
        }

        let theme = cx.global::<Settings>().theme.clone();
        let user = workspace.read(cx).user_store().read(cx).current_user();

        let mut left_container = Flex::row();

        left_container.add_child(
            Label::new(project_title, theme.workspace.titlebar.title.clone())
                .contained()
                .with_margin_right(theme.workspace.titlebar.item_spacing)
                .aligned()
                .left()
                .boxed(),
        );

        if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
            left_container.add_child(self.render_current_user(&workspace, &theme, &user, cx));
            left_container.add_children(self.render_collaborators(&workspace, &theme, room, cx));
            left_container.add_child(self.render_toggle_contacts_button(&theme, cx));
        }

        let mut right_container = Flex::row();

        if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
            right_container.add_child(self.render_toggle_screen_sharing_button(&theme, &room, cx));
            right_container.add_child(self.render_leave_call_button(&theme, cx));
            right_container
                .add_children(self.render_in_call_share_unshare_button(&workspace, &theme, cx));
        } else {
            right_container.add_child(self.render_outside_call_share_button(&theme, cx));
        }

        right_container.add_children(self.render_connection_status(&workspace, cx));

        if let Some(user) = user {
            //TODO: Add style
            right_container.add_child(
                Label::new(
                    user.github_login.clone(),
                    theme.workspace.titlebar.title.clone(),
                )
                .aligned()
                .contained()
                .with_margin_left(theme.workspace.titlebar.item_spacing)
                .boxed(),
            );
        } else {
            right_container.add_child(Self::render_authenticate(&theme, cx));
        }

        Stack::new()
            .with_child(left_container.boxed())
            .with_child(right_container.aligned().right().boxed())
            .boxed()
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
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));
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
            contacts_popover_side: ContactsPopoverSide::Right,
            collaborator_list_popover: None,
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

    fn active_call_changed(&mut self, cx: &mut ViewContext<Self>) {
        if ActiveCall::global(cx).read(cx).room().is_none() {
            self.contacts_popover = None;
        }
        cx.notify();
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

    fn unshare_project(&mut self, _: &UnshareProject, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            let active_call = ActiveCall::global(cx);
            let project = workspace.read(cx).project().clone();
            active_call
                .update(cx, |call, cx| call.unshare_project(project, cx))
                .log_err();
        }
    }

    pub fn toggle_collaborator_list_popover(
        &mut self,
        _: &ToggleCollaboratorList,
        cx: &mut ViewContext<Self>,
    ) {
        match self.collaborator_list_popover.take() {
            Some(_) => {}
            None => {
                if let Some(workspace) = self.workspace.upgrade(cx) {
                    let user_store = workspace.read(cx).user_store().clone();
                    let view = cx.add_view(|cx| CollaboratorListPopover::new(user_store, cx));

                    cx.subscribe(&view, |this, _, event, cx| {
                        match event {
                            collaborator_list_popover::Event::Dismissed => {
                                this.collaborator_list_popover = None;
                            }
                        }

                        cx.notify();
                    })
                    .detach();

                    self.collaborator_list_popover = Some(view);
                }
            }
        }
        cx.notify();
    }

    pub fn toggle_contacts_popover(&mut self, _: &ToggleContactsMenu, cx: &mut ViewContext<Self>) {
        if self.contacts_popover.take().is_none() {
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

                self.contacts_popover_side = match ActiveCall::global(cx).read(cx).room() {
                    Some(_) => ContactsPopoverSide::Left,
                    None => ContactsPopoverSide::Right,
                };

                self.contacts_popover = Some(view);
            }
        }

        cx.notify();
    }

    fn leave_call(&mut self, _: &LeaveCall, cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.hang_up(cx))
            .log_err();
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
                MouseEventHandler::<ToggleContactsMenu>::new(0, cx, |state, _| {
                    let style = titlebar.toggle_contacts_button.style_for(
                        state,
                        self.contacts_popover.is_some()
                            && self.contacts_popover_side == ContactsPopoverSide::Left,
                    );
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
                    cx.dispatch_action(ToggleContactsMenu);
                })
                .with_tooltip::<ToggleContactsMenu, _>(
                    0,
                    "Show contacts menu".into(),
                    Some(Box::new(ToggleContactsMenu)),
                    theme.tooltip.clone(),
                    cx,
                )
                .aligned()
                .boxed(),
            )
            .with_children(badge)
            .with_children(self.render_contacts_popover_host(
                ContactsPopoverSide::Left,
                titlebar,
                cx,
            ))
            .boxed()
    }

    fn render_toggle_screen_sharing_button(
        &self,
        theme: &Theme,
        room: &ModelHandle<Room>,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
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
        .boxed()
    }

    fn render_leave_call_button(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        let titlebar = &theme.workspace.titlebar;

        MouseEventHandler::<LeaveCall>::new(0, cx, |state, _| {
            let style = titlebar.call_control.style_for(state, false);
            Svg::new("icons/leave_12.svg")
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
            cx.dispatch_action(LeaveCall);
        })
        .with_tooltip::<LeaveCall, _>(
            0,
            "Leave call".to_owned(),
            Some(Box::new(LeaveCall)),
            theme.tooltip.clone(),
            cx,
        )
        .contained()
        .with_margin_left(theme.workspace.titlebar.item_spacing)
        .aligned()
        .boxed()
    }

    fn render_in_call_share_unshare_button(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let project = workspace.read(cx).project();
        if project.read(cx).is_remote() {
            return None;
        }

        let is_shared = project.read(cx).is_shared();
        let label = if is_shared { "Unshare" } else { "Share" };
        let tooltip = if is_shared {
            "Unshare project from call participants"
        } else {
            "Share project with call participants"
        };

        let titlebar = &theme.workspace.titlebar;

        enum ShareUnshare {}
        Some(
            Stack::new()
                .with_child(
                    MouseEventHandler::<ShareUnshare>::new(0, cx, |state, _| {
                        //TODO: Ensure this button has consistant width for both text variations
                        let style = titlebar.share_button.style_for(
                            state,
                            self.contacts_popover.is_some()
                                && self.contacts_popover_side == ContactsPopoverSide::Right,
                        );
                        Label::new(label, style.text.clone())
                            .contained()
                            .with_style(style.container)
                            .boxed()
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_click(MouseButton::Left, move |_, cx| {
                        if is_shared {
                            cx.dispatch_action(UnshareProject);
                        } else {
                            cx.dispatch_action(ShareProject);
                        }
                    })
                    .with_tooltip::<ShareUnshare, _>(
                        0,
                        tooltip.to_owned(),
                        None,
                        theme.tooltip.clone(),
                        cx,
                    )
                    .boxed(),
                )
                .with_children(self.render_contacts_popover_host(
                    ContactsPopoverSide::Right,
                    titlebar,
                    cx,
                ))
                .aligned()
                .contained()
                .with_margin_left(theme.workspace.titlebar.item_spacing)
                .boxed(),
        )
    }

    fn render_outside_call_share_button(
        &self,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let tooltip = "Share project with new call";
        let titlebar = &theme.workspace.titlebar;

        enum OutsideCallShare {}
        Stack::new()
            .with_child(
                MouseEventHandler::<OutsideCallShare>::new(0, cx, |state, _| {
                    //TODO: Ensure this button has consistant width for both text variations
                    let style = titlebar.share_button.style_for(
                        state,
                        self.contacts_popover.is_some()
                            && self.contacts_popover_side == ContactsPopoverSide::Right,
                    );
                    Label::new("Share".to_owned(), style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(ToggleContactsMenu);
                })
                .with_tooltip::<OutsideCallShare, _>(
                    0,
                    tooltip.to_owned(),
                    None,
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .with_children(self.render_contacts_popover_host(
                ContactsPopoverSide::Right,
                titlebar,
                cx,
            ))
            .aligned()
            .contained()
            .with_margin_left(theme.workspace.titlebar.item_spacing)
            .boxed()
    }

    fn render_contacts_popover_host<'a>(
        &'a self,
        side: ContactsPopoverSide,
        theme: &'a theme::Titlebar,
        cx: &'a RenderContext<Self>,
    ) -> impl Iterator<Item = ElementBox> + 'a {
        self.contacts_popover
            .iter()
            .filter(move |_| self.contacts_popover_side == side)
            .map(|popover| {
                Overlay::new(
                    ChildView::new(popover, cx)
                        .contained()
                        .with_margin_top(theme.height)
                        .with_margin_left(theme.toggle_contacts_button.default.button_width)
                        .with_margin_right(-theme.toggle_contacts_button.default.button_width)
                        .boxed(),
                )
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_corner(AnchorCorner::BottomLeft)
                .with_z_index(999)
                .boxed()
            })
    }

    fn render_collaborators(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        room: ModelHandle<Room>,
        cx: &mut RenderContext<Self>,
    ) -> Vec<ElementBox> {
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
                Some(
                    Container::new(self.render_face_pile(
                        &user,
                        replica_id,
                        participant.peer_id,
                        Some(participant.location),
                        workspace,
                        theme,
                        cx,
                    ))
                    .with_margin_left(theme.workspace.titlebar.face_pile_spacing)
                    .boxed(),
                )
            })
            .collect()
    }

    fn render_current_user(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        user: &Option<Arc<User>>,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let user = user.as_ref().expect("Active call without user");
        let replica_id = workspace.read(cx).project().read(cx).replica_id();
        let peer_id = workspace
            .read(cx)
            .client()
            .peer_id()
            .expect("Active call without peer id");
        self.render_face_pile(user, Some(replica_id), peer_id, None, workspace, theme, cx)
    }

    fn render_authenticate(theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        MouseEventHandler::<Authenticate>::new(0, cx, |state, _| {
            let style = theme
                .workspace
                .titlebar
                .sign_in_prompt
                .style_for(state, false);
            Label::new("Sign in", style.text.clone())
                .contained()
                .with_style(style.container)
                .with_margin_left(theme.workspace.titlebar.item_spacing)
                .boxed()
        })
        .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(Authenticate))
        .with_cursor_style(CursorStyle::PointingHand)
        .aligned()
        .boxed()
    }

    fn render_face_pile(
        &self,
        user: &User,
        replica_id: Option<ReplicaId>,
        peer_id: PeerId,
        location: Option<ParticipantLocation>,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let room = ActiveCall::global(cx).read(cx).room();
        let is_being_followed = workspace.read(cx).is_being_followed(peer_id);
        let followed_by_self = room
            .map(|room| {
                is_being_followed
                    && room
                        .read(cx)
                        .followers_for(peer_id)
                        .iter()
                        .any(|&follower| Some(follower) == workspace.read(cx).client().peer_id())
            })
            .unwrap_or(false);

        let avatar_style;
        if let Some(location) = location {
            if let ParticipantLocation::SharedProject { project_id } = location {
                if Some(project_id) == workspace.read(cx).project().read(cx).remote_id() {
                    avatar_style = &theme.workspace.titlebar.avatar;
                } else {
                    avatar_style = &theme.workspace.titlebar.inactive_avatar;
                }
            } else {
                avatar_style = &theme.workspace.titlebar.inactive_avatar;
            }
        } else {
            avatar_style = &theme.workspace.titlebar.avatar;
        }

        let mut background_color = theme
            .workspace
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

        let content = Stack::new()
            .with_children(user.avatar.as_ref().map(|avatar| {
                let face_pile = FacePile::new(theme.workspace.titlebar.follower_avatar_overlap)
                    .with_child(Self::render_face(
                        avatar.clone(),
                        avatar_style.clone(),
                        background_color,
                    ))
                    .with_children(
                        (|| {
                            let room = room?.read(cx);
                            let followers = room.followers_for(peer_id);

                            Some(followers.into_iter().flat_map(|&follower| {
                                let avatar = room
                                    .remote_participant_for_peer_id(follower)
                                    .and_then(|participant| participant.user.avatar.clone())
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
                                    theme.workspace.titlebar.follower_avatar.clone(),
                                    background_color,
                                ))
                            }))
                        })()
                        .into_iter()
                        .flatten(),
                    );

                let mut container = face_pile
                    .contained()
                    .with_style(theme.workspace.titlebar.leader_selection);

                if let Some(replica_id) = replica_id {
                    if followed_by_self {
                        let color = theme.editor.replica_selection_style(replica_id).selection;
                        container = container.with_background_color(color);
                    }
                }

                container.boxed()
            }))
            .with_children((|| {
                let replica_id = replica_id?;
                let color = theme.editor.replica_selection_style(replica_id).cursor;
                Some(
                    AvatarRibbon::new(color)
                        .constrained()
                        .with_width(theme.workspace.titlebar.avatar_ribbon.width)
                        .with_height(theme.workspace.titlebar.avatar_ribbon.height)
                        .aligned()
                        .bottom()
                        .boxed(),
                )
            })())
            .boxed();

        if let Some(location) = location {
            if let Some(replica_id) = replica_id {
                MouseEventHandler::<ToggleFollow>::new(replica_id.into(), cx, move |_, _| content)
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_click(MouseButton::Left, move |_, cx| {
                        cx.dispatch_action(ToggleFollow(peer_id))
                    })
                    .with_tooltip::<ToggleFollow, _>(
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
                    format!("Follow {} into external project", user.github_login),
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

    fn render_face(
        avatar: Arc<ImageData>,
        avatar_style: AvatarStyle,
        background_color: Color,
    ) -> ElementBox {
        Image::new(avatar)
            .with_style(avatar_style.image)
            .aligned()
            .contained()
            .with_background_color(background_color)
            .with_corner_radius(avatar_style.outer_corner_radius)
            .constrained()
            .with_width(avatar_style.outer_width)
            .with_height(avatar_style.outer_width)
            .aligned()
            .boxed()
    }

    fn render_connection_status(
        &self,
        workspace: &ViewHandle<Workspace>,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        enum ConnectionStatusButton {}

        let theme = &cx.global::<Settings>().theme.clone();
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
                MouseEventHandler::<ConnectionStatusButton>::new(0, cx, |_, _| {
                    Label::new(
                        "Please update Zed to collaborate",
                        theme.workspace.titlebar.outdated_warning.text.clone(),
                    )
                    .contained()
                    .with_style(theme.workspace.titlebar.outdated_warning.container)
                    .aligned()
                    .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(auto_update::Check);
                })
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
