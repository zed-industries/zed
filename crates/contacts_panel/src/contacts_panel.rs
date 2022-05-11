mod contact_finder;
mod notifications;

use client::{Contact, User, UserStore};
use editor::{Cancel, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_actions,
    platform::CursorStyle,
    Element, ElementBox, Entity, LayoutContext, ModelHandle, MutableAppContext, RenderContext,
    Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use notifications::IncomingRequestNotification;
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use theme::IconButton;
use workspace::{AppState, JoinProject, Workspace};

impl_actions!(
    contacts_panel,
    [RequestContact, RemoveContact, RespondToContactRequest]
);

#[derive(Debug)]
enum ContactEntry {
    Header(&'static str),
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    Contact(Arc<Contact>),
}

pub struct ContactsPanel {
    entries: Vec<ContactEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState,
    user_store: ModelHandle<UserStore>,
    filter_editor: ViewHandle<Editor>,
    _maintain_contacts: Subscription,
}

#[derive(Clone, Deserialize)]
pub struct RequestContact(pub u64);

#[derive(Clone, Deserialize)]
pub struct RemoveContact(pub u64);

#[derive(Clone, Deserialize)]
pub struct RespondToContactRequest {
    pub user_id: u64,
    pub accept: bool,
}

pub fn init(cx: &mut MutableAppContext) {
    contact_finder::init(cx);
    cx.add_action(ContactsPanel::request_contact);
    cx.add_action(ContactsPanel::remove_contact);
    cx.add_action(ContactsPanel::respond_to_contact_request);
    cx.add_action(ContactsPanel::clear_filter);
}

impl ContactsPanel {
    pub fn new(
        app_state: Arc<AppState>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let user_query_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(|theme| theme.contacts_panel.user_query_editor.clone()),
                cx,
            );
            editor.set_placeholder_text("Filter contacts", cx);
            editor
        });

        cx.subscribe(&user_query_editor, |this, _, event, cx| {
            if let editor::Event::BufferEdited = event {
                this.update_entries(cx)
            }
        })
        .detach();

        cx.subscribe(&app_state.user_store, {
            let user_store = app_state.user_store.clone();
            move |_, _, event, cx| match event {
                client::Event::NotifyIncomingRequest(user) => {
                    if let Some(workspace) = workspace.upgrade(cx) {
                        workspace.update(cx, |workspace, cx| {
                            workspace.show_notification(
                                cx.add_view(|_| {
                                    IncomingRequestNotification::new(
                                        user.clone(),
                                        user_store.clone(),
                                    )
                                }),
                                cx,
                            )
                        })
                    }
                }
            }
        })
        .detach();

        let mut this = Self {
            list_state: ListState::new(0, Orientation::Top, 1000., {
                let this = cx.weak_handle();
                let app_state = app_state.clone();
                move |ix, cx| {
                    let this = this.upgrade(cx).unwrap();
                    let this = this.read(cx);
                    let theme = cx.global::<Settings>().theme.clone();
                    let theme = &theme.contacts_panel;
                    let current_user_id =
                        this.user_store.read(cx).current_user().map(|user| user.id);

                    match &this.entries[ix] {
                        ContactEntry::Header(text) => {
                            Label::new(text.to_string(), theme.header.text.clone())
                                .contained()
                                .aligned()
                                .left()
                                .constrained()
                                .with_height(theme.row_height)
                                .contained()
                                .with_style(theme.header.container)
                                .boxed()
                        }
                        ContactEntry::IncomingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            theme,
                            true,
                            cx,
                        ),
                        ContactEntry::OutgoingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            theme,
                            false,
                            cx,
                        ),
                        ContactEntry::Contact(contact) => Self::render_contact(
                            contact.clone(),
                            current_user_id,
                            app_state.clone(),
                            theme,
                            cx,
                        ),
                    }
                }
            }),
            entries: Default::default(),
            match_candidates: Default::default(),
            filter_editor: user_query_editor,
            _maintain_contacts: cx
                .observe(&app_state.user_store, |this, _, cx| this.update_entries(cx)),
            user_store: app_state.user_store.clone(),
        };
        this.update_entries(cx);
        this
    }

    fn render_contact(
        contact: Arc<Contact>,
        current_user_id: Option<u64>,
        app_state: Arc<AppState>,
        theme: &theme::ContactsPanel,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        let project_count = contact.non_empty_projects().count();
        let font_cache = cx.font_cache();
        let line_height = theme.unshared_project.name.text.line_height(font_cache);
        let cap_height = theme.unshared_project.name.text.cap_height(font_cache);
        let baseline_offset = theme.unshared_project.name.text.baseline_offset(font_cache)
            + (theme.unshared_project.height - line_height) / 2.;
        let tree_branch_width = theme.tree_branch_width;
        let tree_branch_color = theme.tree_branch_color;
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(contact.user.avatar.clone().map(|avatar| {
                        Image::new(avatar)
                            .with_style(theme.contact_avatar)
                            .aligned()
                            .left()
                            .boxed()
                    }))
                    .with_child(
                        Label::new(
                            contact.user.github_login.clone(),
                            theme.contact_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.contact_username.container)
                        .aligned()
                        .left()
                        .boxed(),
                    )
                    .constrained()
                    .with_height(theme.row_height)
                    .boxed(),
            )
            .with_children(
                contact
                    .non_empty_projects()
                    .enumerate()
                    .map(|(ix, project)| {
                        let project_id = project.id;
                        Flex::row()
                            .with_child(
                                Canvas::new(move |bounds, _, cx| {
                                    let start_x = bounds.min_x() + (bounds.width() / 2.)
                                        - (tree_branch_width / 2.);
                                    let end_x = bounds.max_x();
                                    let start_y = bounds.min_y();
                                    let end_y =
                                        bounds.min_y() + baseline_offset - (cap_height / 2.);

                                    cx.scene.push_quad(gpui::Quad {
                                        bounds: RectF::from_points(
                                            vec2f(start_x, start_y),
                                            vec2f(
                                                start_x + tree_branch_width,
                                                if ix + 1 == project_count {
                                                    end_y
                                                } else {
                                                    bounds.max_y()
                                                },
                                            ),
                                        ),
                                        background: Some(tree_branch_color),
                                        border: gpui::Border::default(),
                                        corner_radius: 0.,
                                    });
                                    cx.scene.push_quad(gpui::Quad {
                                        bounds: RectF::from_points(
                                            vec2f(start_x, end_y),
                                            vec2f(end_x, end_y + tree_branch_width),
                                        ),
                                        background: Some(tree_branch_color),
                                        border: gpui::Border::default(),
                                        corner_radius: 0.,
                                    });
                                })
                                .constrained()
                                .with_width(host_avatar_height)
                                .boxed(),
                            )
                            .with_child({
                                let is_host = Some(contact.user.id) == current_user_id;
                                let is_guest = !is_host
                                    && project
                                        .guests
                                        .iter()
                                        .any(|guest| Some(guest.id) == current_user_id);
                                let is_shared = project.is_shared;
                                let app_state = app_state.clone();

                                MouseEventHandler::new::<ContactsPanel, _, _>(
                                    project_id as usize,
                                    cx,
                                    |mouse_state, _| {
                                        let style = match (project.is_shared, mouse_state.hovered) {
                                            (false, false) => &theme.unshared_project,
                                            (false, true) => &theme.hovered_unshared_project,
                                            (true, false) => &theme.shared_project,
                                            (true, true) => &theme.hovered_shared_project,
                                        };

                                        Flex::row()
                                            .with_child(
                                                Label::new(
                                                    project.worktree_root_names.join(", "),
                                                    style.name.text.clone(),
                                                )
                                                .aligned()
                                                .left()
                                                .contained()
                                                .with_style(style.name.container)
                                                .boxed(),
                                            )
                                            .with_children(project.guests.iter().filter_map(
                                                |participant| {
                                                    participant.avatar.clone().map(|avatar| {
                                                        Image::new(avatar)
                                                            .with_style(style.guest_avatar)
                                                            .aligned()
                                                            .left()
                                                            .contained()
                                                            .with_margin_right(
                                                                style.guest_avatar_spacing,
                                                            )
                                                            .boxed()
                                                    })
                                                },
                                            ))
                                            .contained()
                                            .with_style(style.container)
                                            .constrained()
                                            .with_height(style.height)
                                            .boxed()
                                    },
                                )
                                .with_cursor_style(if !is_host && is_shared {
                                    CursorStyle::PointingHand
                                } else {
                                    CursorStyle::Arrow
                                })
                                .on_click(move |_, cx| {
                                    if !is_host && !is_guest {
                                        cx.dispatch_global_action(JoinProject {
                                            project_id,
                                            app_state: app_state.clone(),
                                        });
                                    }
                                })
                                .flex(1., true)
                                .boxed()
                            })
                            .constrained()
                            .with_height(theme.unshared_project.height)
                            .boxed()
                    }),
            )
            .contained()
            .with_style(theme.row.clone())
            .boxed()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::ContactsPanel,
        is_incoming: bool,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        enum Reject {}
        enum Accept {}
        enum Cancel {}

        let mut row = Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    user.github_login.clone(),
                    theme.contact_username.text.clone(),
                )
                .contained()
                .with_style(theme.contact_username.container)
                .aligned()
                .left()
                .boxed(),
            );

        let user_id = user.id;
        let is_contact_request_pending = user_store.read(cx).is_contact_request_pending(&user);

        if is_incoming {
            row.add_children([
                MouseEventHandler::new::<Reject, _, _>(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_contact_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/reject.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| {
                    cx.dispatch_action(RespondToContactRequest {
                        user_id,
                        accept: false,
                    })
                })
                .flex_float()
                .boxed(),
                MouseEventHandler::new::<Accept, _, _>(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_contact_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/accept.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| {
                    cx.dispatch_action(RespondToContactRequest {
                        user_id,
                        accept: true,
                    })
                })
                .boxed(),
            ]);
        } else {
            row.add_child(
                MouseEventHandler::new::<Cancel, _, _>(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_contact_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/reject.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_padding(Padding::uniform(2.))
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| cx.dispatch_action(RemoveContact(user_id)))
                .flex_float()
                .boxed(),
            );
        }

        row.constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(theme.row)
            .boxed()
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.read(cx);
        let query = self.filter_editor.read(cx).text(cx);
        let executor = cx.background().clone();

        self.entries.clear();

        let mut request_entries = Vec::new();
        let incoming = user_store.incoming_contact_requests();
        if !incoming.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    incoming
                        .iter()
                        .enumerate()
                        .map(|(ix, user)| StringMatchCandidate {
                            id: ix,
                            string: user.github_login.clone(),
                            char_bag: user.github_login.chars().collect(),
                        }),
                );
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));
            if !matches.is_empty() {
                request_entries.extend(
                    matches.iter().map(|mat| {
                        ContactEntry::IncomingRequest(incoming[mat.candidate_id].clone())
                    }),
                );
            }
        }

        let outgoing = user_store.outgoing_contact_requests();
        if !outgoing.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    outgoing
                        .iter()
                        .enumerate()
                        .map(|(ix, user)| StringMatchCandidate {
                            id: ix,
                            string: user.github_login.clone(),
                            char_bag: user.github_login.chars().collect(),
                        }),
                );
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));
            if !matches.is_empty() {
                request_entries.extend(
                    matches.iter().map(|mat| {
                        ContactEntry::OutgoingRequest(outgoing[mat.candidate_id].clone())
                    }),
                );
            }
        }

        if !request_entries.is_empty() {
            self.entries.push(ContactEntry::Header("Requests"));
            self.entries.append(&mut request_entries);
        }

        let contacts = user_store.contacts();
        if !contacts.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    contacts
                        .iter()
                        .enumerate()
                        .map(|(ix, contact)| StringMatchCandidate {
                            id: ix,
                            string: contact.user.github_login.clone(),
                            char_bag: contact.user.github_login.chars().collect(),
                        }),
                );
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));

            let (online_contacts, offline_contacts) = matches
                .iter()
                .partition::<Vec<_>, _>(|mat| contacts[mat.candidate_id].online);

            if !online_contacts.is_empty() {
                self.entries.push(ContactEntry::Header("Online"));
                self.entries.extend(
                    online_contacts
                        .into_iter()
                        .map(|mat| ContactEntry::Contact(contacts[mat.candidate_id].clone())),
                );
            }

            if !offline_contacts.is_empty() {
                self.entries.push(ContactEntry::Header("Offline"));
                self.entries.extend(
                    offline_contacts
                        .into_iter()
                        .map(|mat| ContactEntry::Contact(contacts[mat.candidate_id].clone())),
                );
            }
        }

        self.list_state.reset(self.entries.len());
        cx.notify();
    }

    fn request_contact(&mut self, request: &RequestContact, cx: &mut ViewContext<Self>) {
        self.user_store
            .update(cx, |store, cx| store.request_contact(request.0, cx))
            .detach();
    }

    fn remove_contact(&mut self, request: &RemoveContact, cx: &mut ViewContext<Self>) {
        self.user_store
            .update(cx, |store, cx| store.remove_contact(request.0, cx))
            .detach();
    }

    fn respond_to_contact_request(
        &mut self,
        action: &RespondToContactRequest,
        cx: &mut ViewContext<Self>,
    ) {
        self.user_store
            .update(cx, |store, cx| {
                store.respond_to_contact_request(action.user_id, action.accept, cx)
            })
            .detach();
    }

    fn clear_filter(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.filter_editor
            .update(cx, |editor, cx| editor.set_text("", cx));
    }
}

fn render_icon_button(style: &IconButton, svg_path: &'static str) -> impl Element {
    Svg::new(svg_path)
        .with_color(style.color)
        .constrained()
        .with_width(style.icon_width)
        .aligned()
        .contained()
        .with_style(style.container)
        .constrained()
        .with_width(style.button_width)
        .with_height(style.button_width)
}

pub enum Event {}

impl Entity for ContactsPanel {
    type Event = Event;
}

impl View for ContactsPanel {
    fn ui_name() -> &'static str {
        "ContactsPanel"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum AddContact {}

        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.contacts_panel;
        Container::new(
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(
                            ChildView::new(self.filter_editor.clone())
                                .contained()
                                .with_style(theme.user_query_editor.container)
                                .flex(1., true)
                                .boxed(),
                        )
                        .with_child(
                            MouseEventHandler::new::<AddContact, _, _>(0, cx, |_, _| {
                                Svg::new("icons/add-contact.svg")
                                    .with_color(theme.add_contact_button.color)
                                    .constrained()
                                    .with_height(12.)
                                    .contained()
                                    .with_style(theme.add_contact_button.container)
                                    .aligned()
                                    .boxed()
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(|_, cx| cx.dispatch_action(contact_finder::Toggle))
                            .boxed(),
                        )
                        .constrained()
                        .with_height(theme.user_query_editor_height)
                        .boxed(),
                )
                .with_child(List::new(self.list_state.clone()).flex(1., false).boxed())
                .boxed(),
        )
        .with_style(theme.container)
        .boxed()
    }
}
