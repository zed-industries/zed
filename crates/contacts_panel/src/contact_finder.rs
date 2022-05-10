use client::{ContactRequestStatus, User, UserStore};
use editor::Editor;
use gpui::{
    color::Color, elements::*, platform::CursorStyle, Entity, LayoutContext, ModelHandle,
    RenderContext, Task, View, ViewContext, ViewHandle,
};
use settings::Settings;
use std::sync::Arc;
use util::TryFutureExt;

use crate::{RemoveContact, RequestContact};

pub struct ContactFinder {
    query_editor: ViewHandle<Editor>,
    list_state: UniformListState,
    potential_contacts: Arc<[Arc<User>]>,
    user_store: ModelHandle<UserStore>,
    contacts_search_task: Option<Task<Option<()>>>,
}

impl Entity for ContactFinder {
    type Event = ();
}

impl View for ContactFinder {
    fn ui_name() -> &'static str {
        "ContactFinder"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let user_store = self.user_store.clone();
        let potential_contacts = self.potential_contacts.clone();
        Flex::column()
            .with_child(
                ChildView::new(self.query_editor.clone())
                    .contained()
                    .with_style(theme.contact_finder.query_editor.container)
                    .boxed(),
            )
            .with_child(
                UniformList::new(self.list_state.clone(), self.potential_contacts.len(), {
                    let theme = theme.clone();
                    move |range, items, cx| {
                        items.extend(range.map(|ix| {
                            Self::render_potential_contact(
                                &potential_contacts[ix],
                                &user_store,
                                &theme.contact_finder,
                                cx,
                            )
                        }))
                    }
                })
                .flex(1., false)
                .boxed(),
            )
            .contained()
            .with_style(theme.contact_finder.container)
            .constrained()
            .with_max_width(theme.contact_finder.max_width)
            .with_max_height(theme.contact_finder.max_height)
            .boxed()
    }
}

impl ContactFinder {
    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(Some(|theme| theme.contact_finder.query_editor.clone()), cx)
        });

        cx.subscribe(&query_editor, |this, _, event, cx| {
            if let editor::Event::BufferEdited = event {
                this.query_changed(cx)
            }
        })
        .detach();
        Self {
            query_editor,
            list_state: Default::default(),
            potential_contacts: Arc::from([]),
            user_store,
            contacts_search_task: None,
        }
    }

    fn render_potential_contact(
        contact: &User,
        user_store: &ModelHandle<UserStore>,
        theme: &theme::ContactFinder,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        enum RequestContactButton {}

        let contact_id = contact.id;
        let request_status = user_store.read(cx).contact_request_status(&contact);

        Flex::row()
            .with_children(contact.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    contact.github_login.clone(),
                    theme.contact_username.text.clone(),
                )
                .contained()
                .with_style(theme.contact_username.container)
                .aligned()
                .left()
                .boxed(),
            )
            .with_child(
                MouseEventHandler::new::<RequestContactButton, _, _>(
                    contact.id as usize,
                    cx,
                    |_, _| {
                        let label = match request_status {
                            ContactRequestStatus::None | ContactRequestStatus::RequestReceived => {
                                "+"
                            }
                            ContactRequestStatus::RequestSent => "-",
                            ContactRequestStatus::Pending
                            | ContactRequestStatus::RequestAccepted => "…",
                        };

                        Label::new(label.to_string(), theme.contact_button.text.clone())
                            .contained()
                            .with_style(theme.contact_button.container)
                            .aligned()
                            .flex_float()
                            .boxed()
                    },
                )
                .on_click(move |_, cx| match request_status {
                    ContactRequestStatus::None => {
                        cx.dispatch_action(RequestContact(contact_id));
                    }
                    ContactRequestStatus::RequestSent => {
                        cx.dispatch_action(RemoveContact(contact_id));
                    }
                    _ => {}
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .boxed(),
            )
            .constrained()
            .with_height(theme.row_height)
            .boxed()
    }

    fn query_changed(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.read(cx).text(cx);
        let search_users = self
            .user_store
            .update(cx, |store, cx| store.fuzzy_search_users(query, cx));

        self.contacts_search_task = Some(cx.spawn(|this, mut cx| {
            async move {
                let potential_contacts = search_users.await?;
                this.update(&mut cx, |this, cx| {
                    this.potential_contacts = potential_contacts.into();
                    cx.notify();
                });
                Ok(())
            }
            .log_err()
        }));
    }
}
