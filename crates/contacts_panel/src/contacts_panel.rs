mod contact_finder;
mod contact_notification;

use client::{Contact, ContactEventKind, User, UserStore};
use contact_notification::ContactNotification;
use editor::{Cancel, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_actions, impl_internal_actions,
    platform::CursorStyle,
    AppContext, Element, ElementBox, Entity, LayoutContext, ModelHandle, MutableAppContext,
    RenderContext, Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use theme::IconButton;
use workspace::{
    menu::{Confirm, SelectNext, SelectPrev},
    sidebar::SidebarItem,
    AppState, JoinProject, Workspace,
};

impl_actions!(
    contacts_panel,
    [RequestContact, RemoveContact, RespondToContactRequest]
);

impl_internal_actions!(contacts_panel, [ToggleExpanded]);

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Section {
    Requests,
    Online,
    Offline,
}

#[derive(Clone, Debug)]
enum ContactEntry {
    Header(Section),
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    Contact(Arc<Contact>),
    ContactProject(Arc<Contact>, usize),
}

#[derive(Clone)]
struct ToggleExpanded(Section);

pub struct ContactsPanel {
    entries: Vec<ContactEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState,
    user_store: ModelHandle<UserStore>,
    filter_editor: ViewHandle<Editor>,
    collapsed_sections: Vec<Section>,
    selection: Option<usize>,
    app_state: Arc<AppState>,
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
    contact_notification::init(cx);
    cx.add_action(ContactsPanel::request_contact);
    cx.add_action(ContactsPanel::remove_contact);
    cx.add_action(ContactsPanel::respond_to_contact_request);
    cx.add_action(ContactsPanel::clear_filter);
    cx.add_action(ContactsPanel::select_next);
    cx.add_action(ContactsPanel::select_prev);
    cx.add_action(ContactsPanel::confirm);
    cx.add_action(ContactsPanel::toggle_expanded);
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
                this.selection.take();
                this.update_entries(cx)
            }
        })
        .detach();

        cx.subscribe(&app_state.user_store, {
            let user_store = app_state.user_store.downgrade();
            move |_, _, event, cx| {
                if let Some((workspace, user_store)) =
                    workspace.upgrade(cx).zip(user_store.upgrade(cx))
                {
                    workspace.update(cx, |workspace, cx| match event.kind {
                        ContactEventKind::Requested | ContactEventKind::Accepted => workspace
                            .show_notification(
                                cx.add_view(|cx| {
                                    ContactNotification::new(event.clone(), user_store, cx)
                                }),
                                cx,
                            ),
                        _ => {}
                    });
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
                    let is_selected = this.selection == Some(ix);

                    match &this.entries[ix] {
                        ContactEntry::Header(section) => {
                            let is_collapsed = this.collapsed_sections.contains(&section);
                            Self::render_header(*section, theme, is_selected, is_collapsed, cx)
                        }
                        ContactEntry::IncomingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            theme,
                            true,
                            is_selected,
                            cx,
                        ),
                        ContactEntry::OutgoingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            theme,
                            false,
                            is_selected,
                            cx,
                        ),
                        ContactEntry::Contact(contact) => {
                            Self::render_contact(contact.clone(), theme, is_selected)
                        }
                        ContactEntry::ContactProject(contact, project_ix) => {
                            let is_last_project_for_contact =
                                this.entries.get(ix + 1).map_or(true, |next| {
                                    if let ContactEntry::ContactProject(next_contact, _) = next {
                                        next_contact.user.id != contact.user.id
                                    } else {
                                        true
                                    }
                                });
                            Self::render_contact_project(
                                contact.clone(),
                                current_user_id,
                                *project_ix,
                                app_state.clone(),
                                theme,
                                is_last_project_for_contact,
                                is_selected,
                                cx,
                            )
                        }
                    }
                }
            }),
            selection: None,
            collapsed_sections: Default::default(),
            entries: Default::default(),
            match_candidates: Default::default(),
            filter_editor: user_query_editor,
            _maintain_contacts: cx
                .observe(&app_state.user_store, |this, _, cx| this.update_entries(cx)),
            user_store: app_state.user_store.clone(),
            app_state,
        };
        this.update_entries(cx);
        this
    }

    fn render_header(
        section: Section,
        theme: &theme::ContactsPanel,
        is_selected: bool,
        is_collapsed: bool,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        enum Header {}

        let header_style = theme.header_row.style_for(&Default::default(), is_selected);
        let text = match section {
            Section::Requests => "Requests",
            Section::Online => "Online",
            Section::Offline => "Offline",
        };
        let icon_size = theme.section_icon_size;
        MouseEventHandler::new::<Header, _, _>(section as usize, cx, |_, _| {
            Flex::row()
                .with_child(
                    Svg::new(if is_collapsed {
                        "icons/disclosure-closed.svg"
                    } else {
                        "icons/disclosure-open.svg"
                    })
                    .with_color(header_style.text.color)
                    .constrained()
                    .with_max_width(icon_size)
                    .with_max_height(icon_size)
                    .aligned()
                    .constrained()
                    .with_width(icon_size)
                    .boxed(),
                )
                .with_child(
                    Label::new(text.to_string(), header_style.text.clone())
                        .aligned()
                        .left()
                        .contained()
                        .with_margin_left(theme.contact_username.container.margin.left)
                        .flex(1., true)
                        .boxed(),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(header_style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(move |_, cx| cx.dispatch_action(ToggleExpanded(section)))
        .boxed()
    }

    fn render_contact(
        contact: Arc<Contact>,
        theme: &theme::ContactsPanel,
        is_selected: bool,
    ) -> ElementBox {
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
                .flex(1., true)
                .boxed(),
            )
            .constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(
                *theme
                    .contact_row
                    .style_for(&Default::default(), is_selected),
            )
            .boxed()
    }

    fn render_contact_project(
        contact: Arc<Contact>,
        current_user_id: Option<u64>,
        project_ix: usize,
        app_state: Arc<AppState>,
        theme: &theme::ContactsPanel,
        is_last_project: bool,
        is_selected: bool,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        let project = &contact.projects[project_ix];
        let project_id = project.id;
        let is_host = Some(contact.user.id) == current_user_id;
        let is_guest = !is_host
            && project
                .guests
                .iter()
                .any(|guest| Some(guest.id) == current_user_id);
        let is_shared = project.is_shared;

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.unshared_project_row.default;
        let tree_branch = theme.tree_branch.clone();
        let line_height = row.name.text.line_height(font_cache);
        let cap_height = row.name.text.cap_height(font_cache);
        let baseline_offset =
            row.name.text.baseline_offset(font_cache) + (theme.row_height - line_height) / 2.;

        MouseEventHandler::new::<JoinProject, _, _>(project_id as usize, cx, |mouse_state, _| {
            let tree_branch = *tree_branch.style_for(mouse_state, is_selected);
            let row = if project.is_shared {
                &theme.shared_project_row
            } else {
                &theme.unshared_project_row
            }
            .style_for(mouse_state, is_selected);

            Flex::row()
                .with_child(
                    Canvas::new(move |bounds, _, cx| {
                        let start_x =
                            bounds.min_x() + (bounds.width() / 2.) - (tree_branch.width / 2.);
                        let end_x = bounds.max_x();
                        let start_y = bounds.min_y();
                        let end_y = bounds.min_y() + baseline_offset - (cap_height / 2.);

                        cx.scene.push_quad(gpui::Quad {
                            bounds: RectF::from_points(
                                vec2f(start_x, start_y),
                                vec2f(
                                    start_x + tree_branch.width,
                                    if is_last_project {
                                        end_y
                                    } else {
                                        bounds.max_y()
                                    },
                                ),
                            ),
                            background: Some(tree_branch.color),
                            border: gpui::Border::default(),
                            corner_radius: 0.,
                        });
                        cx.scene.push_quad(gpui::Quad {
                            bounds: RectF::from_points(
                                vec2f(start_x, end_y),
                                vec2f(end_x, end_y + tree_branch.width),
                            ),
                            background: Some(tree_branch.color),
                            border: gpui::Border::default(),
                            corner_radius: 0.,
                        });
                    })
                    .constrained()
                    .with_width(host_avatar_height)
                    .boxed(),
                )
                .with_child(
                    Label::new(
                        project.worktree_root_names.join(", "),
                        row.name.text.clone(),
                    )
                    .aligned()
                    .left()
                    .contained()
                    .with_style(row.name.container)
                    .flex(1., false)
                    .boxed(),
                )
                .with_children(project.guests.iter().filter_map(|participant| {
                    participant.avatar.clone().map(|avatar| {
                        Image::new(avatar)
                            .with_style(row.guest_avatar)
                            .aligned()
                            .left()
                            .contained()
                            .with_margin_right(row.guest_avatar_spacing)
                            .boxed()
                    })
                }))
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(row.container)
                .boxed()
        })
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
        .boxed()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::ContactsPanel,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        enum Decline {}
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
                .flex(1., true)
                .boxed(),
            );

        let user_id = user.id;
        let is_contact_request_pending = user_store.read(cx).is_contact_request_pending(&user);
        let button_spacing = theme.contact_button_spacing;

        if is_incoming {
            row.add_children([
                MouseEventHandler::new::<Decline, _, _>(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_contact_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/decline.svg")
                        .aligned()
                        // .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| {
                    cx.dispatch_action(RespondToContactRequest {
                        user_id,
                        accept: false,
                    })
                })
                // .flex_float()
                .contained()
                .with_margin_right(button_spacing)
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
                    render_icon_button(button_style, "icons/decline.svg")
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
            .with_style(
                *theme
                    .contact_row
                    .style_for(&Default::default(), is_selected),
            )
            .boxed()
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.read(cx);
        let query = self.filter_editor.read(cx).text(cx);
        let executor = cx.background().clone();

        let prev_selected_entry = self.selection.and_then(|ix| self.entries.get(ix).cloned());
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
            request_entries.extend(
                matches
                    .iter()
                    .map(|mat| ContactEntry::IncomingRequest(incoming[mat.candidate_id].clone())),
            );
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
            request_entries.extend(
                matches
                    .iter()
                    .map(|mat| ContactEntry::OutgoingRequest(outgoing[mat.candidate_id].clone())),
            );
        }

        if !request_entries.is_empty() {
            self.entries.push(ContactEntry::Header(Section::Requests));
            if !self.collapsed_sections.contains(&Section::Requests) {
                self.entries.append(&mut request_entries);
            }
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

            for (matches, section) in [
                (online_contacts, Section::Online),
                (offline_contacts, Section::Offline),
            ] {
                if !matches.is_empty() {
                    self.entries.push(ContactEntry::Header(section));
                    if !self.collapsed_sections.contains(&section) {
                        for mat in matches {
                            let contact = &contacts[mat.candidate_id];
                            self.entries.push(ContactEntry::Contact(contact.clone()));
                            self.entries
                                .extend(contact.projects.iter().enumerate().filter_map(
                                    |(ix, project)| {
                                        if project.worktree_root_names.is_empty() {
                                            None
                                        } else {
                                            Some(ContactEntry::ContactProject(contact.clone(), ix))
                                        }
                                    },
                                ));
                        }
                    }
                }
            }
        }

        if let Some(selection) = &mut self.selection {
            for (ix, entry) in self.entries.iter().enumerate() {
                if Some(entry) == prev_selected_entry.as_ref() {
                    *selection = ix;
                    break;
                }
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

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selection {
            if self.entries.len() > ix + 1 {
                self.selection = Some(ix + 1);
            }
        } else if !self.entries.is_empty() {
            self.selection = Some(0);
        }
        cx.notify();
        self.list_state.reset(self.entries.len());
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selection {
            if ix > 0 {
                self.selection = Some(ix - 1);
            } else {
                self.selection = None;
            }
        }
        cx.notify();
        self.list_state.reset(self.entries.len());
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            if let Some(entry) = self.entries.get(selection) {
                match entry {
                    ContactEntry::Header(section) => {
                        let section = *section;
                        self.toggle_expanded(&ToggleExpanded(section), cx);
                    }
                    ContactEntry::ContactProject(contact, project_ix) => {
                        cx.dispatch_global_action(JoinProject {
                            project_id: contact.projects[*project_ix].id,
                            app_state: self.app_state.clone(),
                        })
                    }
                    _ => {}
                }
            }
        }
    }

    fn toggle_expanded(&mut self, action: &ToggleExpanded, cx: &mut ViewContext<Self>) {
        let section = action.0;
        if let Some(ix) = self.collapsed_sections.iter().position(|s| *s == section) {
            self.collapsed_sections.remove(ix);
        } else {
            self.collapsed_sections.push(section);
        }
        self.update_entries(cx);
    }
}

impl SidebarItem for ContactsPanel {
    fn should_show_badge(&self, cx: &AppContext) -> bool {
        !self
            .user_store
            .read(cx)
            .incoming_contact_requests()
            .is_empty()
    }

    fn contains_focused_view(&self, cx: &AppContext) -> bool {
        self.filter_editor.is_focused(cx)
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

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.filter_editor);
    }

    fn keymap_context(&self, _: &gpui::AppContext) -> gpui::keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }
}

impl PartialEq for ContactEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ContactEntry::Header(section_1) => {
                if let ContactEntry::Header(section_2) = other {
                    return section_1 == section_2;
                }
            }
            ContactEntry::IncomingRequest(user_1) => {
                if let ContactEntry::IncomingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ContactEntry::OutgoingRequest(user_1) => {
                if let ContactEntry::OutgoingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ContactEntry::Contact(contact_1) => {
                if let ContactEntry::Contact(contact_2) = other {
                    return contact_1.user.id == contact_2.user.id;
                }
            }
            ContactEntry::ContactProject(contact_1, ix_1) => {
                if let ContactEntry::ContactProject(contact_2, ix_2) = other {
                    return contact_1.user.id == contact_2.user.id && ix_1 == ix_2;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{proto, test::FakeServer, ChannelList, Client};
    use gpui::TestAppContext;
    use language::LanguageRegistry;
    use theme::ThemeRegistry;
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_contact_panel(cx: &mut TestAppContext) {
        let (app_state, server) = init(cx).await;
        let workspace_params = cx.update(WorkspaceParams::test);
        let workspace = cx.add_view(0, |cx| Workspace::new(&workspace_params, cx));
        let panel = cx.add_view(0, |cx| {
            ContactsPanel::new(app_state.clone(), workspace.downgrade(), cx)
        });

        let get_users_request = server.receive::<proto::GetUsers>().await.unwrap();
        server
            .respond(
                get_users_request.receipt(),
                proto::UsersResponse {
                    users: [
                        "user_zero",
                        "user_one",
                        "user_two",
                        "user_three",
                        "user_four",
                        "user_five",
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(id, name)| proto::User {
                        id: id as u64,
                        github_login: name.to_string(),
                        ..Default::default()
                    })
                    .collect(),
                },
            )
            .await;

        server.send(proto::UpdateContacts {
            incoming_requests: vec![proto::IncomingContactRequest {
                requester_id: 1,
                should_notify: false,
            }],
            outgoing_requests: vec![2],
            contacts: vec![
                proto::Contact {
                    user_id: 3,
                    online: true,
                    should_notify: false,
                    projects: vec![proto::ProjectMetadata {
                        id: 101,
                        worktree_root_names: vec!["dir1".to_string()],
                        is_shared: true,
                        guests: vec![2],
                    }],
                },
                proto::Contact {
                    user_id: 4,
                    online: true,
                    should_notify: false,
                    projects: vec![proto::ProjectMetadata {
                        id: 102,
                        worktree_root_names: vec!["dir2".to_string()],
                        is_shared: true,
                        guests: vec![2],
                    }],
                },
                proto::Contact {
                    user_id: 5,
                    online: false,
                    should_notify: false,
                    projects: vec![],
                },
            ],
            ..Default::default()
        });

        cx.foreground().run_until_parked();
        assert_eq!(
            render_to_strings(&panel, cx),
            &[
                "+",
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel
                .filter_editor
                .update(cx, |editor, cx| editor.set_text("f", cx))
        });
        cx.foreground().run_until_parked();
        assert_eq!(
            render_to_strings(&panel, cx),
            &[
                "+",
                "v Online",
                "  user_four",
                "    dir2",
                "v Offline",
                "  user_five",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
        });
        assert_eq!(
            render_to_strings(&panel, cx),
            &[
                "+",
                "v Online  <=== selected",
                "  user_four",
                "    dir2",
                "v Offline",
                "  user_five",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
        });
        assert_eq!(
            render_to_strings(&panel, cx),
            &[
                "+",
                "v Online",
                "  user_four  <=== selected",
                "    dir2",
                "v Offline",
                "  user_five",
            ]
        );
    }

    fn render_to_strings(panel: &ViewHandle<ContactsPanel>, cx: &TestAppContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            let mut entries = Vec::new();
            entries.push("+".to_string());
            entries.extend(panel.entries.iter().enumerate().map(|(ix, entry)| {
                let mut string = match entry {
                    ContactEntry::Header(name) => {
                        let icon = if panel.collapsed_sections.contains(name) {
                            ">"
                        } else {
                            "v"
                        };
                        format!("{} {:?}", icon, name)
                    }
                    ContactEntry::IncomingRequest(user) => {
                        format!("  incoming {}", user.github_login)
                    }
                    ContactEntry::OutgoingRequest(user) => {
                        format!("  outgoing {}", user.github_login)
                    }
                    ContactEntry::Contact(contact) => {
                        format!("  {}", contact.user.github_login)
                    }
                    ContactEntry::ContactProject(contact, project_ix) => {
                        format!(
                            "    {}",
                            contact.projects[*project_ix].worktree_root_names.join(", ")
                        )
                    }
                };

                if panel.selection == Some(ix) {
                    string.push_str("  <=== selected");
                }

                string
            }));
            entries
        })
    }

    async fn init(cx: &mut TestAppContext) -> (Arc<AppState>, FakeServer) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let themes = ThemeRegistry::new((), cx.font_cache());
        let fs = project::FakeFs::new(cx.background().clone());
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let mut client = Client::new(http_client.clone());
        let server = FakeServer::for_client(100, &mut client, &cx).await;
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let channel_list =
            cx.add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx));

        let get_channels = server.receive::<proto::GetChannels>().await.unwrap();
        server
            .respond(get_channels.receipt(), Default::default())
            .await;

        (
            Arc::new(AppState {
                languages,
                themes,
                client,
                user_store: user_store.clone(),
                fs,
                channel_list,
                build_window_options: || unimplemented!(),
                build_workspace: |_, _, _| unimplemented!(),
            }),
            server,
        )
    }
}
