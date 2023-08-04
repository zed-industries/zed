use client::{proto, ChannelId, ChannelMembership, ChannelStore, User, UserId, UserStore};
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

actions!(channel_modal, [SelectNextControl, ToggleMode]);

pub fn init(cx: &mut AppContext) {
    Picker::<ChannelModalDelegate>::init(cx);
    cx.add_action(ChannelModal::toggle_mode);
    cx.add_action(ChannelModal::select_next_control);
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
                    match_candidates: members
                        .iter()
                        .enumerate()
                        .map(|(id, member)| StringMatchCandidate {
                            id,
                            string: member.user.github_login.clone(),
                            char_bag: member.user.github_login.chars().collect(),
                        })
                        .collect(),
                    members,
                    mode,
                    selected_column: None,
                },
                cx,
            )
            .with_theme(|theme| theme.collab_panel.channel_modal.picker.clone())
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
                    picker.update_matches(picker.query(cx), cx);
                    cx.notify()
                });
            })
        })
        .detach();
    }

    fn select_next_control(&mut self, _: &SelectNextControl, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            let delegate = picker.delegate_mut();
            match delegate.mode {
                Mode::ManageMembers => match delegate.selected_column {
                    Some(UserColumn::Remove) => {
                        delegate.selected_column = Some(UserColumn::ToggleAdmin)
                    }
                    Some(UserColumn::ToggleAdmin) => {
                        delegate.selected_column = Some(UserColumn::Remove)
                    }
                    None => todo!(),
                },
                Mode::InviteMembers => {}
            }
            cx.notify()
        });
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
        let theme = &theme::current(cx).collab_panel.channel_modal;

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
            theme: &theme::ChannelModal,
            cx: &mut ViewContext<ChannelModal>,
        ) -> AnyElement<ChannelModal> {
            let active = mode == current_mode;
            MouseEventHandler::<T, _>::new(0, cx, move |state, _| {
                let contained_text = theme.mode_button.style_for(active, state);
                Label::new(text, contained_text.text.clone())
                    .contained()
                    .with_style(contained_text.container.clone())
            })
            .on_click(MouseButton::Left, move |_, this, cx| {
                if !active {
                    this.set_mode(mode, cx);
                }
            })
            .with_cursor_style(if active {
                CursorStyle::Arrow
            } else {
                CursorStyle::PointingHand
            })
            .into_any()
        }

        Flex::column()
            .with_child(Label::new(
                format!("#{}", channel.name),
                theme.header.clone(),
            ))
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
            .with_child(ChildView::new(&self.picker, cx))
            .constrained()
            .with_height(theme.height)
            .contained()
            .with_style(theme.container)
            .into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = true;
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

#[derive(Copy, Clone, PartialEq)]
pub enum UserColumn {
    ToggleAdmin,
    Remove,
}

pub struct ChannelModalDelegate {
    matching_users: Vec<Arc<User>>,
    matching_member_indices: Vec<usize>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    mode: Mode,
    selected_column: Option<UserColumn>,
    match_candidates: Arc<[StringMatchCandidate]>,
    members: Vec<ChannelMembership>,
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
        self.selected_column = match self.mode {
            Mode::ManageMembers => Some(UserColumn::ToggleAdmin),
            Mode::InviteMembers => None,
        };
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        match self.mode {
            Mode::ManageMembers => {
                let match_candidates = self.match_candidates.clone();
                cx.spawn(|picker, mut cx| async move {
                    async move {
                        let matches = match_strings(
                            &match_candidates,
                            &query,
                            true,
                            usize::MAX,
                            &Default::default(),
                            cx.background().clone(),
                        )
                        .await;
                        picker.update(&mut cx, |picker, cx| {
                            let delegate = picker.delegate_mut();
                            delegate.matching_member_indices.clear();
                            delegate
                                .matching_member_indices
                                .extend(matches.into_iter().map(|m| m.candidate_id));
                            cx.notify();
                        })?;
                        anyhow::Ok(())
                    }
                    .log_err()
                    .await;
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
            match self.member_status(selected_user.id, cx) {
                Some(proto::channel_member::Kind::Member)
                | Some(proto::channel_member::Kind::Invitee) => {
                    if self.selected_column == Some(UserColumn::ToggleAdmin) {
                        self.set_member_admin(selected_user.id, !admin.unwrap_or(false), cx);
                    } else {
                        self.remove_member(selected_user.id, cx);
                    }
                }
                Some(proto::channel_member::Kind::AncestorMember) | None => {
                    self.channel_store
                        .update(cx, |store, cx| {
                            store.invite_member(self.channel_id, selected_user.id, false, cx)
                        })
                        .detach();
                }
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
        let theme = &theme::current(cx).collab_panel.channel_modal;
        let (user, admin) = self.user_at_index(ix).unwrap();
        let request_status = self.member_status(user.id, cx);

        let style = theme.picker.item.in_state(selected).style_for(mouse_state);
        Flex::row()
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
            .with_children(admin.map(|admin| {
                let member_style = theme.admin_toggle_part.in_state(!admin);
                let admin_style = theme.admin_toggle_part.in_state(admin);
                Flex::row()
                    .with_child(
                        Label::new("member", member_style.text.clone())
                            .contained()
                            .with_style(member_style.container),
                    )
                    .with_child(
                        Label::new("admin", admin_style.text.clone())
                            .contained()
                            .with_style(admin_style.container),
                    )
                    .contained()
                    .with_style(theme.admin_toggle)
                    .aligned()
                    .flex_float()
            }))
            .with_children({
                match self.mode {
                    Mode::ManageMembers => match request_status {
                        Some(proto::channel_member::Kind::Member) => Some(
                            Label::new("remove member", theme.remove_member_button.text.clone())
                                .contained()
                                .with_style(theme.remove_member_button.container)
                                .into_any(),
                        ),
                        Some(proto::channel_member::Kind::Invitee) => Some(
                            Label::new("cancel invite", theme.cancel_invite_button.text.clone())
                                .contained()
                                .with_style(theme.cancel_invite_button.container)
                                .into_any(),
                        ),
                        Some(proto::channel_member::Kind::AncestorMember) | None => None,
                    },
                    Mode::InviteMembers => {
                        let svg = match request_status {
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
                        };

                        svg.map(|svg| svg.aligned().flex_float().into_any())
                    }
                }
            })
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(theme.row_height)
            .into_any()
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
            .or(self
                .channel_store
                .read(cx)
                .has_pending_channel_invite(self.channel_id, user_id)
                .then_some(proto::channel_member::Kind::Invitee))
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

    fn set_member_admin(&mut self, user_id: u64, admin: bool, cx: &mut ViewContext<Picker<Self>>) {
        let update = self.channel_store.update(cx, |store, cx| {
            store.set_member_admin(self.channel_id, user_id, admin, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
                let this = picker.delegate_mut();
                if let Some(member) = this.members.iter_mut().find(|m| m.user.id == user_id) {
                    member.admin = admin;
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn remove_member(&mut self, user_id: u64, cx: &mut ViewContext<Picker<Self>>) {
        let update = self.channel_store.update(cx, |store, cx| {
            store.remove_member(self.channel_id, user_id, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
                let this = picker.delegate_mut();
                if let Some(ix) = this.members.iter_mut().position(|m| m.user.id == user_id) {
                    this.members.remove(ix);
                }
            })
        })
        .detach_and_log_err(cx);
    }
}
