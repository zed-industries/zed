use client::{proto, ChannelId, ChannelStore, User, UserId, UserStore};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AppContext, Entity, ModelHandle, MouseState, Task, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::sync::Arc;
use util::TryFutureExt;
use workspace::Modal;

pub fn init(cx: &mut AppContext) {
    Picker::<ChannelModalDelegate>::init(cx);
}

pub struct ChannelModal {
    picker: ViewHandle<Picker<ChannelModalDelegate>>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    has_focus: bool,
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
                    this.picker.update(cx, |picker, cx| {
                        picker.delegate_mut().mode = mode;
                        picker.update_matches(picker.query(cx), cx);
                        cx.notify();
                    })
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

pub fn build_channel_modal(
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    mode: Mode,
    members: Vec<(Arc<User>, proto::channel_member::Kind)>,
    cx: &mut ViewContext<ChannelModal>,
) -> ChannelModal {
    let picker = cx.add_view(|cx| {
        Picker::new(
            ChannelModalDelegate {
                matches: Vec::new(),
                selected_index: 0,
                user_store: user_store.clone(),
                channel_store: channel_store.clone(),
                channel_id,
                match_candidates: members
                    .iter()
                    .enumerate()
                    .map(|(id, member)| StringMatchCandidate {
                        id,
                        string: member.0.github_login.clone(),
                        char_bag: member.0.github_login.chars().collect(),
                    })
                    .collect(),
                members,
                mode,
            },
            cx,
        )
        .with_theme(|theme| theme.collab_panel.channel_modal.picker.clone())
    });

    cx.subscribe(&picker, |_, _, e, cx| cx.emit(*e)).detach();
    let has_focus = picker.read(cx).has_focus();

    ChannelModal {
        picker,
        channel_store,
        channel_id,
        has_focus,
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    ManageMembers,
    InviteMembers,
}

pub struct ChannelModalDelegate {
    matches: Vec<(Arc<User>, Option<proto::channel_member::Kind>)>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    mode: Mode,
    match_candidates: Arc<[StringMatchCandidate]>,
    members: Vec<(Arc<User>, proto::channel_member::Kind)>,
}

impl PickerDelegate for ChannelModalDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search collaborator by username...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
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
                            delegate.matches.clear();
                            delegate.matches.extend(matches.into_iter().map(|m| {
                                let member = &delegate.members[m.candidate_id];
                                (member.0.clone(), Some(member.1))
                            }));
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
                            delegate.matches.clear();
                            delegate
                                .matches
                                .extend(users.into_iter().map(|user| (user, None)));
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
        if let Some((user, _)) = self.matches.get(self.selected_index) {
            match self.mode {
                Mode::ManageMembers => {
                    //
                }
                Mode::InviteMembers => match self.member_status(user.id, cx) {
                    Some(proto::channel_member::Kind::Member) => {}
                    Some(proto::channel_member::Kind::Invitee) => self
                        .channel_store
                        .update(cx, |store, cx| {
                            store.remove_member(self.channel_id, user.id, cx)
                        })
                        .detach(),
                    Some(proto::channel_member::Kind::AncestorMember) | None => self
                        .channel_store
                        .update(cx, |store, cx| {
                            store.invite_member(self.channel_id, user.id, false, cx)
                        })
                        .detach(),
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
        let theme = &theme::current(cx).collab_panel.channel_modal;
        let (user, _) = &self.matches[ix];
        let request_status = self.member_status(user.id, cx);

        let icon_path = match request_status {
            Some(proto::channel_member::Kind::AncestorMember) => Some("icons/check_8.svg"),
            Some(proto::channel_member::Kind::Member) => Some("icons/check_8.svg"),
            Some(proto::channel_member::Kind::Invitee) => Some("icons/x_mark_8.svg"),
            None => None,
        };
        let button_style = &theme.contact_button;

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
            .with_children(icon_path.map(|icon_path| {
                Svg::new(icon_path)
                    .with_color(button_style.color)
                    .constrained()
                    .with_width(button_style.icon_width)
                    .aligned()
                    .contained()
                    .with_style(button_style.container)
                    .constrained()
                    .with_width(button_style.button_width)
                    .with_height(button_style.button_width)
                    .aligned()
                    .flex_float()
            }))
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
            .find_map(|(user, status)| (user.id == user_id).then_some(*status))
            .or(self
                .channel_store
                .read(cx)
                .has_pending_channel_invite(self.channel_id, user_id)
                .then_some(proto::channel_member::Kind::Invitee))
    }
}
