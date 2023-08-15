use client::{proto, ChannelId, ChannelMembership, ChannelStore, User, UserId, UserStore};
use context_menu::{ContextMenu, ContextMenuItem};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    AppContext, Entity, ModelHandle, MouseState, Task, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::sync::Arc;
use util::TryFutureExt;
use workspace::Modal;

actions!(
    channel_modal,
    [
        SelectNextControl,
        ToggleMode,
        ToggleMemberAdmin,
        RemoveMember
    ]
);

pub fn init(cx: &mut AppContext) {
    Picker::<ChannelModalDelegate>::init(cx);
    cx.add_action(ChannelModal::toggle_mode);
    cx.add_action(ChannelModal::toggle_member_admin);
    cx.add_action(ChannelModal::remove_member);
    cx.add_action(ChannelModal::dismiss);
}

pub struct ChannelModal {
    picker: ViewHandle<Picker<ChannelModalDelegate>>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    has_focus: bool,
}

impl ChannelModal {
    pub fn new(
        user_store: ModelHandle<UserStore>,
        channel_store: ModelHandle<ChannelStore>,
        channel_id: ChannelId,
        mode: Mode,
        members: Vec<ChannelMembership>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&channel_store, |_, _, cx| cx.notify()).detach();
        let picker = cx.add_view(|cx| {
            Picker::new(
                ChannelModalDelegate {
                    matching_users: Vec::new(),
                    matching_member_indices: Vec::new(),
                    selected_index: 0,
                    user_store: user_store.clone(),
                    channel_store: channel_store.clone(),
                    channel_id,
                    match_candidates: Vec::new(),
                    members,
                    mode,
                    context_menu: cx.add_view(|cx| {
                        let mut menu = ContextMenu::new(cx.view_id(), cx);
                        menu.set_position_mode(OverlayPositionMode::Local);
                        menu
                    }),
                },
                cx,
            )
            .with_theme(|theme| theme.collab_panel.tabbed_modal.picker.clone())
        });

        cx.subscribe(&picker, |_, _, e, cx| cx.emit(*e)).detach();

        let has_focus = picker.read(cx).has_focus();

        Self {
            picker,
            channel_store,
            channel_id,
            has_focus,
        }
    }

    fn toggle_mode(&mut self, _: &ToggleMode, cx: &mut ViewContext<Self>) {
        let mode = match self.picker.read(cx).delegate().mode {
            Mode::ManageMembers => Mode::InviteMembers,
            Mode::InviteMembers => Mode::ManageMembers,
        };
        self.set_mode(mode, cx);
    }

    fn set_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.clone();
        let channel_id = self.channel_id;
        cx.spawn(|this, mut cx| async move {
            if mode == Mode::ManageMembers {
                let members = channel_store
                    .update(&mut cx, |channel_store, cx| {
                        channel_store.get_channel_member_details(channel_id, cx)
                    })
                    .await?;
                this.update(&mut cx, |this, cx| {
                    this.picker
                        .update(cx, |picker, _| picker.delegate_mut().members = members);
                })?;
            }

            this.update(&mut cx, |this, cx| {
                this.picker.update(cx, |picker, cx| {
                    let delegate = picker.delegate_mut();
                    delegate.mode = mode;
                    delegate.selected_index = 0;
                    picker.set_query("", cx);
                    picker.update_matches(picker.query(cx), cx);
                    cx.notify()
                });
                cx.notify()
            })
        })
        .detach();
    }

    fn toggle_member_admin(&mut self, _: &ToggleMemberAdmin, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate_mut().toggle_selected_member_admin(cx);
        })
    }

    fn remove_member(&mut self, _: &RemoveMember, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate_mut().remove_selected_member(cx);
        });
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(PickerEvent::Dismiss);
    }
}

impl Entity for ChannelModal {
    type Event = PickerEvent;
}

impl View for ChannelModal {
    fn ui_name() -> &'static str {
        "ChannelModal"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &theme::current(cx).collab_panel.tabbed_modal;

        let mode = self.picker.read(cx).delegate().mode;
        let Some(channel) = self
            .channel_store
            .read(cx)
            .channel_for_id(self.channel_id) else {
                return Empty::new().into_any()
            };

        enum InviteMembers {}
        enum ManageMembers {}

        fn render_mode_button<T: 'static>(
            mode: Mode,
            text: &'static str,
            current_mode: Mode,
            theme: &theme::TabbedModal,
            cx: &mut ViewContext<ChannelModal>,
        ) -> AnyElement<ChannelModal> {
            let active = mode == current_mode;
            MouseEventHandler::new::<T, _>(0, cx, move |state, _| {
                let contained_text = theme.tab_button.style_for(active, state);
                Label::new(text, contained_text.text.clone())
                    .contained()
                    .with_style(contained_text.container.clone())
            })
            .on_click(MouseButton::Left, move |_, this, cx| {
                if !active {
                    this.set_mode(mode, cx);
                }
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .into_any()
        }

        Flex::column()
            .with_child(
                Flex::column()
                    .with_child(
                        Label::new(format!("#{}", channel.name), theme.title.text.clone())
                            .contained()
                            .with_style(theme.title.container.clone()),
                    )
                    .with_child(Flex::row().with_children([
                        render_mode_button::<InviteMembers>(
                            Mode::InviteMembers,
                            "Invite members",
                            mode,
                            theme,
                            cx,
                        ),
                        render_mode_button::<ManageMembers>(
                            Mode::ManageMembers,
                            "Manage members",
                            mode,
                            theme,
                            cx,
                        ),
                    ]))
                    .expanded()
                    .contained()
                    .with_style(theme.header),
            )
            .with_child(
                ChildView::new(&self.picker, cx)
                    .contained()
                    .with_style(theme.body),
            )
            .constrained()
            .with_max_height(theme.max_height)
            .with_max_width(theme.max_width)
            .contained()
            .with_style(theme.modal)
            .into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = true;
        if cx.is_self_focused() {
            cx.focus(&self.picker)
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Modal for ChannelModal {
    fn has_focus(&self) -> bool {
        self.has_focus
    }

    fn dismiss_on_event(event: &Self::Event) -> bool {
        match event {
            PickerEvent::Dismiss => true,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    ManageMembers,
    InviteMembers,
}

pub struct ChannelModalDelegate {
    matching_users: Vec<Arc<User>>,
    matching_member_indices: Vec<usize>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    mode: Mode,
    match_candidates: Vec<StringMatchCandidate>,
    members: Vec<ChannelMembership>,
    context_menu: ViewHandle<ContextMenu>,
}

impl PickerDelegate for ChannelModalDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search collaborator by username...".into()
    }

    fn match_count(&self) -> usize {
        match self.mode {
            Mode::ManageMembers => self.matching_member_indices.len(),
            Mode::InviteMembers => self.matching_users.len(),
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        match self.mode {
            Mode::ManageMembers => {
                self.match_candidates.clear();
                self.match_candidates
                    .extend(self.members.iter().enumerate().map(|(id, member)| {
                        StringMatchCandidate {
                            id,
                            string: member.user.github_login.clone(),
                            char_bag: member.user.github_login.chars().collect(),
                        }
                    }));

                let matches = cx.background().block(match_strings(
                    &self.match_candidates,
                    &query,
                    true,
                    usize::MAX,
                    &Default::default(),
                    cx.background().clone(),
                ));

                cx.spawn(|picker, mut cx| async move {
                    picker
                        .update(&mut cx, |picker, cx| {
                            let delegate = picker.delegate_mut();
                            delegate.matching_member_indices.clear();
                            delegate
                                .matching_member_indices
                                .extend(matches.into_iter().map(|m| m.candidate_id));
                            cx.notify();
                        })
                        .ok();
                })
            }
            Mode::InviteMembers => {
                let search_users = self
                    .user_store
                    .update(cx, |store, cx| store.fuzzy_search_users(query, cx));
                cx.spawn(|picker, mut cx| async move {
                    async {
                        let users = search_users.await?;
                        picker.update(&mut cx, |picker, cx| {
                            let delegate = picker.delegate_mut();
                            delegate.matching_users = users;
                            cx.notify();
                        })?;
                        anyhow::Ok(())
                    }
                    .log_err()
                    .await;
                })
            }
        }
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some((selected_user, admin)) = self.user_at_index(self.selected_index) {
            match self.mode {
                Mode::ManageMembers => self.show_context_menu(admin.unwrap_or(false), cx),
                Mode::InviteMembers => match self.member_status(selected_user.id, cx) {
                    Some(proto::channel_member::Kind::Invitee) => {
                        self.remove_selected_member(cx);
                    }
                    Some(proto::channel_member::Kind::AncestorMember) | None => {
                        self.invite_member(selected_user, cx)
                    }
                    Some(proto::channel_member::Kind::Member) => {}
                },
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(PickerEvent::Dismiss);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let full_theme = &theme::current(cx);
        let theme = &full_theme.collab_panel.channel_modal;
        let tabbed_modal = &full_theme.collab_panel.tabbed_modal;
        let (user, admin) = self.user_at_index(ix).unwrap();
        let request_status = self.member_status(user.id, cx);

        let style = tabbed_modal
            .picker
            .item
            .in_state(selected)
            .style_for(mouse_state);

        let in_manage = matches!(self.mode, Mode::ManageMembers);

        let mut result = Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
            }))
            .with_child(
                Label::new(user.github_login.clone(), style.label.clone())
                    .contained()
                    .with_style(theme.contact_username)
                    .aligned()
                    .left(),
            )
            .with_children({
                (in_manage && request_status == Some(proto::channel_member::Kind::Invitee)).then(
                    || {
                        Label::new("Invited", theme.member_tag.text.clone())
                            .contained()
                            .with_style(theme.member_tag.container)
                            .aligned()
                            .left()
                    },
                )
            })
            .with_children(admin.and_then(|admin| {
                (in_manage && admin).then(|| {
                    Label::new("Admin", theme.member_tag.text.clone())
                        .contained()
                        .with_style(theme.member_tag.container)
                        .aligned()
                        .left()
                })
            }))
            .with_children({
                let svg = match self.mode {
                    Mode::ManageMembers => Some(
                        Svg::new("icons/ellipsis_14.svg")
                            .with_color(theme.member_icon.color)
                            .constrained()
                            .with_width(theme.member_icon.width)
                            .aligned()
                            .contained()
                            .with_style(theme.member_icon.container),
                    ),
                    Mode::InviteMembers => match request_status {
                        Some(proto::channel_member::Kind::Member) => Some(
                            Svg::new("icons/check_8.svg")
                                .with_color(theme.member_icon.color)
                                .constrained()
                                .with_width(theme.member_icon.width)
                                .aligned()
                                .contained()
                                .with_style(theme.member_icon.container),
                        ),
                        Some(proto::channel_member::Kind::Invitee) => Some(
                            Svg::new("icons/check_8.svg")
                                .with_color(theme.invitee_icon.color)
                                .constrained()
                                .with_width(theme.invitee_icon.width)
                                .aligned()
                                .contained()
                                .with_style(theme.invitee_icon.container),
                        ),
                        Some(proto::channel_member::Kind::AncestorMember) | None => None,
                    },
                };

                svg.map(|svg| svg.aligned().flex_float().into_any())
            })
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(tabbed_modal.row_height)
            .into_any();

        if selected {
            result = Stack::new()
                .with_child(result)
                .with_child(
                    ChildView::new(&self.context_menu, cx)
                        .aligned()
                        .top()
                        .right(),
                )
                .into_any();
        }

        result
    }
}

impl ChannelModalDelegate {
    fn member_status(
        &self,
        user_id: UserId,
        cx: &AppContext,
    ) -> Option<proto::channel_member::Kind> {
        self.members
            .iter()
            .find_map(|membership| (membership.user.id == user_id).then_some(membership.kind))
            .or_else(|| {
                self.channel_store
                    .read(cx)
                    .has_pending_channel_invite(self.channel_id, user_id)
                    .then_some(proto::channel_member::Kind::Invitee)
            })
    }

    fn user_at_index(&self, ix: usize) -> Option<(Arc<User>, Option<bool>)> {
        match self.mode {
            Mode::ManageMembers => self.matching_member_indices.get(ix).and_then(|ix| {
                let channel_membership = self.members.get(*ix)?;
                Some((
                    channel_membership.user.clone(),
                    Some(channel_membership.admin),
                ))
            }),
            Mode::InviteMembers => Some((self.matching_users.get(ix).cloned()?, None)),
        }
    }

    fn toggle_selected_member_admin(&mut self, cx: &mut ViewContext<Picker<Self>>) -> Option<()> {
        let (user, admin) = self.user_at_index(self.selected_index)?;
        let admin = !admin.unwrap_or(false);
        let update = self.channel_store.update(cx, |store, cx| {
            store.set_member_admin(self.channel_id, user.id, admin, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
                let this = picker.delegate_mut();
                if let Some(member) = this.members.iter_mut().find(|m| m.user.id == user.id) {
                    member.admin = admin;
                }
                cx.focus_self();
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
        Some(())
    }

    fn remove_selected_member(&mut self, cx: &mut ViewContext<Picker<Self>>) -> Option<()> {
        let (user, _) = self.user_at_index(self.selected_index)?;
        let user_id = user.id;
        let update = self.channel_store.update(cx, |store, cx| {
            store.remove_member(self.channel_id, user_id, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
                let this = picker.delegate_mut();
                if let Some(ix) = this.members.iter_mut().position(|m| m.user.id == user_id) {
                    this.members.remove(ix);
                    this.matching_member_indices.retain_mut(|member_ix| {
                        if *member_ix == ix {
                            return false;
                        } else if *member_ix > ix {
                            *member_ix -= 1;
                        }
                        true
                    })
                }

                this.selected_index = this
                    .selected_index
                    .min(this.matching_member_indices.len().saturating_sub(1));

                cx.focus_self();
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
        Some(())
    }

    fn invite_member(&mut self, user: Arc<User>, cx: &mut ViewContext<Picker<Self>>) {
        let invite_member = self.channel_store.update(cx, |store, cx| {
            store.invite_member(self.channel_id, user.id, false, cx)
        });

        cx.spawn(|this, mut cx| async move {
            invite_member.await?;

            this.update(&mut cx, |this, cx| {
                this.delegate_mut().members.push(ChannelMembership {
                    user,
                    kind: proto::channel_member::Kind::Invitee,
                    admin: false,
                });
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    fn show_context_menu(&mut self, user_is_admin: bool, cx: &mut ViewContext<Picker<Self>>) {
        self.context_menu.update(cx, |context_menu, cx| {
            context_menu.show(
                Default::default(),
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::action("Remove", RemoveMember),
                    ContextMenuItem::action(
                        if user_is_admin {
                            "Make non-admin"
                        } else {
                            "Make admin"
                        },
                        ToggleMemberAdmin,
                    ),
                ],
                cx,
            )
        })
    }
}
