use client::{ContactRequestStatus, User, UserStore};
use gpui::{elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext};
use picker::{Picker, PickerDelegate, PickerEvent};
use settings::Settings;
use std::sync::Arc;
use util::TryFutureExt;

pub fn init(cx: &mut AppContext) {
    Picker::<ContactFinderDelegate>::init(cx);
}

pub type ContactFinder = Picker<ContactFinderDelegate>;

pub fn build_contact_finder(
    user_store: ModelHandle<UserStore>,
    cx: &mut ViewContext<ContactFinder>,
) -> ContactFinder {
    Picker::new(
        ContactFinderDelegate {
            user_store,
            potential_contacts: Arc::from([]),
            selected_index: 0,
        },
        cx,
    )
    .with_theme(|theme| theme.contact_finder.picker.clone())
}

pub struct ContactFinderDelegate {
    potential_contacts: Arc<[Arc<User>]>,
    user_store: ModelHandle<UserStore>,
    selected_index: usize,
}

impl PickerDelegate for ContactFinderDelegate {
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

    fn confirm(&mut self, cx: &mut ViewContext<Picker<Self>>) {
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

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = &cx.global::<Settings>().theme;
        let user = &self.potential_contacts[ix];
        let request_status = self.user_store.read(cx).contact_request_status(user);

        let icon_path = match request_status {
            ContactRequestStatus::None | ContactRequestStatus::RequestReceived => {
                Some("icons/check_8.svg")
            }
            ContactRequestStatus::RequestSent => Some("icons/x_mark_8.svg"),
            ContactRequestStatus::RequestAccepted => None,
        };
        let button_style = if self.user_store.read(cx).is_contact_request_pending(user) {
            &theme.contact_finder.disabled_contact_button
        } else {
            &theme.contact_finder.contact_button
        };
        let style = theme
            .contact_finder
            .picker
            .item
            .style_for(mouse_state, selected);
        Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.contact_finder.contact_avatar)
                    .aligned()
                    .left()
            }))
            .with_child(
                Label::new(user.github_login.clone(), style.label.clone())
                    .contained()
                    .with_style(theme.contact_finder.contact_username)
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
            .with_height(theme.contact_finder.row_height)
            .into_any()
    }
}
