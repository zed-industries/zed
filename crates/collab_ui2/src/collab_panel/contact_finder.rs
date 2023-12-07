use client::{ContactRequestStatus, User, UserStore};
use gpui::{
    div, img, svg, AnyElement, AppContext, DismissEvent, Div, Entity, EventEmitter, FocusHandle,
    FocusableView, Img, IntoElement, Model, ParentElement as _, Render, Styled, Task, View,
    ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use theme::ActiveTheme as _;
use ui::prelude::*;
use util::{ResultExt as _, TryFutureExt};

pub fn init(cx: &mut AppContext) {
    //Picker::<ContactFinderDelegate>::init(cx);
    //cx.add_action(ContactFinder::dismiss)
}

pub struct ContactFinder {
    picker: View<Picker<ContactFinderDelegate>>,
    has_focus: bool,
}

impl ContactFinder {
    pub fn new(user_store: Model<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let delegate = ContactFinderDelegate {
            parent: cx.view().downgrade(),
            user_store,
            potential_contacts: Arc::from([]),
            selected_index: 0,
        };
        let picker = cx.build_view(|cx| Picker::new(delegate, cx));

        Self {
            picker,
            has_focus: false,
        }
    }

    pub fn set_query(&mut self, query: String, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            // todo!()
            // picker.set_query(query, cx);
        });
    }
}

impl Render for ContactFinder {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        fn render_mode_button(text: &'static str) -> AnyElement {
            Label::new(text).into_any_element()
        }

        v_stack()
            .child(
                v_stack()
                    .child(Label::new("Contacts"))
                    .child(h_stack().children([render_mode_button("Invite new contacts")]))
                    .bg(cx.theme().colors().element_background),
            )
            .child(self.picker.clone())
            .w(rems(34.))
    }

    // fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
    //     self.has_focus = true;
    //     if cx.is_self_focused() {
    //         cx.focus(&self.picker)
    //     }
    // }

    // fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
    //     self.has_focus = false;
    // }

    type Element = Div;
}

// impl Modal for ContactFinder {
//     fn has_focus(&self) -> bool {
//         self.has_focus
//     }

//     fn dismiss_on_event(event: &Self::Event) -> bool {
//         match event {
//             PickerEvent::Dismiss => true,
//         }
//     }
// }

pub struct ContactFinderDelegate {
    parent: WeakView<ContactFinder>,
    potential_contacts: Arc<[Arc<User>]>,
    user_store: Model<UserStore>,
    selected_index: usize,
}

impl EventEmitter<DismissEvent> for ContactFinder {}

impl FocusableView for ContactFinder {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl PickerDelegate for ContactFinderDelegate {
    type ListItem = Div;
    fn match_count(&self) -> usize {
        self.potential_contacts.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self) -> Arc<str> {
        "Search collaborator by username...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search_users = self
            .user_store
            .update(cx, |store, cx| store.fuzzy_search_users(query, cx));

        cx.spawn(|picker, mut cx| async move {
            async {
                let potential_contacts = search_users.await?;
                picker.update(&mut cx, |picker, cx| {
                    picker.delegate.potential_contacts = potential_contacts.into();
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
        //cx.emit(PickerEvent::Dismiss);
        self.parent
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let user = &self.potential_contacts[ix];
        let request_status = self.user_store.read(cx).contact_request_status(user);

        let icon_path = match request_status {
            ContactRequestStatus::None | ContactRequestStatus::RequestReceived => {
                Some("icons/check.svg")
            }
            ContactRequestStatus::RequestSent => Some("icons/x.svg"),
            ContactRequestStatus::RequestAccepted => None,
        };
        Some(
            div()
                .flex_1()
                .justify_between()
                .children(user.avatar.clone().map(|avatar| img(avatar)))
                .child(Label::new(user.github_login.clone()))
                .children(icon_path.map(|icon_path| svg().path(icon_path))),
        )
        // Flex::row()
        //     .with_children(user.avatar.clone().map(|avatar| {
        //         Image::from_data(avatar)
        //             .with_style(theme.contact_avatar)
        //             .aligned()
        //             .left()
        //     }))
        //     .with_child(
        //         Label::new(user.github_login.clone(), style.label.clone())
        //             .contained()
        //             .with_style(theme.contact_username)
        //             .aligned()
        //             .left(),
        //     )
        //     .with_children(icon_path.map(|icon_path| {
        //         Svg::new(icon_path)
        //             .with_color(button_style.color)
        //             .constrained()
        //             .with_width(button_style.icon_width)
        //             .aligned()
        //             .contained()
        //             .with_style(button_style.container)
        //             .constrained()
        //             .with_width(button_style.button_width)
        //             .with_height(button_style.button_width)
        //             .aligned()
        //             .flex_float()
        //     }))
        //     .contained()
        //     .with_style(style.container)
        //     .constrained()
        //     .with_height(tabbed_modal.row_height)
        //     .into_any()
    }
}
