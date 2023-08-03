use client::{
    ChannelId, ChannelMemberStatus, ChannelStore, ContactRequestStatus, User, UserId, UserStore,
};
use collections::HashMap;
use gpui::{elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::sync::Arc;
use util::TryFutureExt;

pub fn init(cx: &mut AppContext) {
    Picker::<ChannelModalDelegate>::init(cx);
}

pub type ChannelModal = Picker<ChannelModalDelegate>;

pub fn build_channel_modal(
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel: ChannelId,
    members: HashMap<UserId, ChannelMemberStatus>,
    cx: &mut ViewContext<ChannelModal>,
) -> ChannelModal {
    Picker::new(
        ChannelModalDelegate {
            potential_contacts: Arc::from([]),
            selected_index: 0,
            user_store,
            channel_store,
            channel_id: channel,
            member_statuses: members,
        },
        cx,
    )
    .with_theme(|theme| theme.picker.clone())
}

pub struct ChannelModalDelegate {
    potential_contacts: Arc<[Arc<User>]>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    member_statuses: HashMap<UserId, ChannelMemberStatus>,
}

impl PickerDelegate for ChannelModalDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search collaborator by username...".into()
    }

    fn match_count(&self) -> usize {
        self.potential_contacts.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search_users = self
            .user_store
            .update(cx, |store, cx| store.fuzzy_search_users(query, cx));

        cx.spawn(|picker, mut cx| async move {
            async {
                let potential_contacts = search_users.await?;
                picker.update(&mut cx, |picker, cx| {
                    picker.delegate_mut().potential_contacts = potential_contacts.into();
                    cx.notify();
                })?;
                anyhow::Ok(())
            }
            .log_err()
            .await;
        })
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(user) = self.potential_contacts.get(self.selected_index) {
            let user_store = self.user_store.read(cx);
            match user_store.contact_request_status(user) {
                ContactRequestStatus::None | ContactRequestStatus::RequestReceived => {
                    self.user_store
                        .update(cx, |store, cx| store.request_contact(user.id, cx))
                        .detach();
                }
                ContactRequestStatus::RequestSent => {
                    self.user_store
                        .update(cx, |store, cx| store.remove_contact(user.id, cx))
                        .detach();
                }
                _ => {}
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(PickerEvent::Dismiss);
    }

    fn render_header(
        &self,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<AnyElement<Picker<Self>>> {
        let theme = &theme::current(cx).collab_panel.channel_modal;

        self.channel_store
            .read(cx)
            .channel_for_id(self.channel_id)
            .map(|channel| {
                Label::new(
                    format!("Add members for #{}", channel.name),
                    theme.picker.item.default_style().label.clone(),
                )
                .into_any()
            })
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = &theme::current(cx).collab_panel.channel_modal;
        let user = &self.potential_contacts[ix];
        let request_status = self.member_statuses.get(&user.id);

        let icon_path = match request_status {
            Some(ChannelMemberStatus::Member) => Some("icons/check_8.svg"),
            Some(ChannelMemberStatus::Invited) => Some("icons/x_mark_8.svg"),
            None => None,
        };
        let button_style = if self.user_store.read(cx).is_contact_request_pending(user) {
            &theme.disabled_contact_button
        } else {
            &theme.contact_button
        };
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
