use client::{ContactRequestStatus, User, UserStore};
use gpui::{
    actions, elements::*, Entity, ModelHandle, MutableAppContext, RenderContext, Task, View,
    ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate};
use settings::Settings;
use std::sync::Arc;
use util::TryFutureExt;
use workspace::Workspace;

use crate::render_icon_button;

actions!(contact_finder, [Toggle]);

pub fn init(cx: &mut MutableAppContext) {
    Picker::<ContactFinder>::init(cx);
    cx.add_action(ContactFinder::toggle);
}

pub struct ContactFinder {
    picker: ViewHandle<Picker<Self>>,
    potential_contacts: Arc<[Arc<User>]>,
    user_store: ModelHandle<UserStore>,
    selected_index: usize,
}

pub enum Event {
    Dismissed,
}

impl Entity for ContactFinder {
    type Event = Event;
}

impl View for ContactFinder {
    fn ui_name() -> &'static str {
        "ContactFinder"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.picker);
    }
}

impl PickerDelegate for ContactFinder {
    fn match_count(&self) -> usize {
        self.potential_contacts.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Self>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> Task<()> {
        let search_users = self
            .user_store
            .update(cx, |store, cx| store.fuzzy_search_users(query, cx));

        cx.spawn(|this, mut cx| async move {
            async {
                let potential_contacts = search_users.await?;
                this.update(&mut cx, |this, cx| {
                    this.potential_contacts = potential_contacts.into();
                    cx.notify();
                });
                Ok(())
            }
            .log_err()
            .await;
        })
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
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

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let theme = &cx.global::<Settings>().theme;
        let user = &self.potential_contacts[ix];
        let request_status = self.user_store.read(cx).contact_request_status(&user);

        let icon_path = match request_status {
            ContactRequestStatus::None | ContactRequestStatus::RequestReceived => {
                "icons/accept.svg"
            }
            ContactRequestStatus::RequestSent | ContactRequestStatus::RequestAccepted => {
                "icons/reject.svg"
            }
        };
        let button_style = if self.user_store.read(cx).is_contact_request_pending(&user) {
            &theme.contact_finder.disabled_contact_button
        } else {
            &theme.contact_finder.contact_button
        };
        let style = theme.picker.item.style_for(mouse_state, selected);
        Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_finder.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(user.github_login.clone(), style.label.clone())
                    .contained()
                    .with_style(theme.contact_finder.contact_username)
                    .aligned()
                    .left()
                    .boxed(),
            )
            .with_child(
                render_icon_button(button_style, icon_path)
                    .aligned()
                    .flex_float()
                    .boxed(),
            )
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(theme.contact_finder.row_height)
            .boxed()
    }
}

impl ContactFinder {
    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |workspace, cx| {
            let finder = cx.add_view(|cx| Self::new(workspace.user_store().clone(), cx));
            cx.subscribe(&finder, Self::on_event).detach();
            finder
        });
    }

    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let this = cx.weak_handle();
        Self {
            picker: cx.add_view(|cx| Picker::new(this, cx)),
            potential_contacts: Arc::from([]),
            user_store,
            selected_index: 0,
        }
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }
}
