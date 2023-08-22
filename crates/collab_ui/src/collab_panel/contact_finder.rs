use client::{ContactRequestStatus, User, UserStore};
use gpui::{
    elements::*, AppContext, Entity, ModelHandle, MouseState, Task, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::sync::Arc;
use util::TryFutureExt;
use workspace::Modal;

pub fn init(cx: &mut AppContext) {
    Picker::<ContactFinderDelegate>::init(cx);
    cx.add_action(ContactFinder::dismiss)
}

pub struct ContactFinder {
    picker: ViewHandle<Picker<ContactFinderDelegate>>,
    has_focus: bool,
}

impl ContactFinder {
    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.add_view(|cx| {
            Picker::new(
                ContactFinderDelegate {
                    user_store,
                    potential_contacts: Arc::from([]),
                    selected_index: 0,
                },
                cx,
            )
            .with_theme(|theme| theme.collab_panel.tabbed_modal.picker.clone())
        });

        cx.subscribe(&picker, |_, _, e, cx| cx.emit(*e)).detach();

        Self {
            picker,
            has_focus: false,
        }
    }

    pub fn set_query(&mut self, query: String, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.set_query(query, cx);
        });
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(PickerEvent::Dismiss);
    }
}

impl Entity for ContactFinder {
    type Event = PickerEvent;
}

impl View for ContactFinder {
    fn ui_name() -> &'static str {
        "ContactFinder"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let full_theme = &theme::current(cx);
        let theme = &full_theme.collab_panel.tabbed_modal;

        fn render_mode_button(
            text: &'static str,
            theme: &theme::TabbedModal,
            _cx: &mut ViewContext<ContactFinder>,
        ) -> AnyElement<ContactFinder> {
            let contained_text = &theme.tab_button.active_state().default;
            Label::new(text, contained_text.text.clone())
                .contained()
                .with_style(contained_text.container.clone())
                .into_any()
        }

        Flex::column()
            .with_child(
                Flex::column()
                    .with_child(
                        Label::new("Contacts", theme.title.text.clone())
                            .contained()
                            .with_style(theme.title.container.clone()),
                    )
                    .with_child(Flex::row().with_children([render_mode_button(
                        "Invite new contacts",
                        &theme,
                        cx,
                    )]))
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

impl Modal for ContactFinder {
    fn has_focus(&self) -> bool {
        self.has_focus
    }

    fn dismiss_on_event(event: &Self::Event) -> bool {
        match event {
            PickerEvent::Dismiss => true,
        }
    }
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

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let full_theme = &theme::current(cx);
        let theme = &full_theme.collab_panel.contact_finder;
        let tabbed_modal = &full_theme.collab_panel.tabbed_modal;
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
            &theme.disabled_contact_button
        } else {
            &theme.contact_button
        };
        let style = tabbed_modal
            .picker
            .item
            .in_state(selected)
            .style_for(mouse_state);
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
            .with_height(tabbed_modal.row_height)
            .into_any()
    }
}
