mod contact_finder;
mod contact_notification;
mod join_project_notification;
mod notifications;

use client::{Contact, ContactEventKind, User, UserStore};
use contact_notification::ContactNotification;
use editor::{Cancel, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_actions, impl_internal_actions,
    platform::CursorStyle,
    AppContext, ClipboardItem, Element, ElementBox, Entity, ModelHandle, MutableAppContext,
    RenderContext, Subscription, View, ViewContext, ViewHandle, WeakModelHandle, WeakViewHandle,
};
use join_project_notification::JoinProjectNotification;
use menu::{Confirm, SelectNext, SelectPrev};
use project::{Project, ProjectStore};
use serde::Deserialize;
use settings::Settings;
use std::{ops::DerefMut, sync::Arc};
use theme::IconButton;
use workspace::{sidebar::SidebarItem, JoinProject, ToggleProjectOnline, Workspace};

actions!(contacts_panel, [Toggle]);

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

#[derive(Clone)]
enum ContactEntry {
    Header(Section),
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    Contact(Arc<Contact>),
    ContactProject(Arc<Contact>, usize, Option<WeakModelHandle<Project>>),
    OfflineProject(WeakModelHandle<Project>),
}

#[derive(Clone, PartialEq)]
struct ToggleExpanded(Section);

pub struct ContactsPanel {
    entries: Vec<ContactEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState,
    user_store: ModelHandle<UserStore>,
    project_store: ModelHandle<ProjectStore>,
    filter_editor: ViewHandle<Editor>,
    collapsed_sections: Vec<Section>,
    selection: Option<usize>,
    _maintain_contacts: Subscription,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct RequestContact(pub u64);

#[derive(Clone, Deserialize, PartialEq)]
pub struct RemoveContact(pub u64);

#[derive(Clone, Deserialize, PartialEq)]
pub struct RespondToContactRequest {
    pub user_id: u64,
    pub accept: bool,
}

pub fn init(cx: &mut MutableAppContext) {
    contact_finder::init(cx);
    contact_notification::init(cx);
    join_project_notification::init(cx);
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
        user_store: ModelHandle<UserStore>,
        project_store: ModelHandle<ProjectStore>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let filter_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(|theme| theme.contacts_panel.user_query_editor.clone()),
                cx,
            );
            editor.set_placeholder_text("Filter contacts", cx);
            editor
        });

        cx.subscribe(&filter_editor, |this, _, event, cx| {
            if let editor::Event::BufferEdited = event {
                let query = this.filter_editor.read(cx).text(cx);
                if !query.is_empty() {
                    this.selection.take();
                }
                this.update_entries(cx);
                if !query.is_empty() {
                    this.selection = this
                        .entries
                        .iter()
                        .position(|entry| !matches!(entry, ContactEntry::Header(_)));
                }
            }
        })
        .detach();

        cx.defer({
            let workspace = workspace.clone();
            move |_, cx| {
                if let Some(workspace_handle) = workspace.upgrade(cx) {
                    cx.subscribe(&workspace_handle.read(cx).project().clone(), {
                        let workspace = workspace.clone();
                        move |_, project, event, cx| match event {
                            project::Event::ContactRequestedJoin(user) => {
                                if let Some(workspace) = workspace.upgrade(cx) {
                                    workspace.update(cx, |workspace, cx| {
                                        workspace.show_notification(user.id as usize, cx, |cx| {
                                            cx.add_view(|cx| {
                                                JoinProjectNotification::new(
                                                    project,
                                                    user.clone(),
                                                    cx,
                                                )
                                            })
                                        })
                                    });
                                }
                            }
                            _ => {}
                        }
                    })
                    .detach();
                }
            }
        });

        cx.observe(&project_store, |this, _, cx| this.update_entries(cx))
            .detach();

        cx.subscribe(&user_store, move |_, user_store, event, cx| {
            if let Some(workspace) = workspace.upgrade(cx) {
                workspace.update(cx, |workspace, cx| match event {
                    client::Event::Contact { user, kind } => match kind {
                        ContactEventKind::Requested | ContactEventKind::Accepted => workspace
                            .show_notification(user.id as usize, cx, |cx| {
                                cx.add_view(|cx| {
                                    ContactNotification::new(user.clone(), *kind, user_store, cx)
                                })
                            }),
                        _ => {}
                    },
                    _ => {}
                });
            }

            if let client::Event::ShowContacts = event {
                cx.emit(Event::Activate);
            }
        })
        .detach();

        let list_state = ListState::new(0, Orientation::Top, 1000., cx, move |this, ix, cx| {
            let theme = cx.global::<Settings>().theme.clone();
            let current_user_id = this.user_store.read(cx).current_user().map(|user| user.id);
            let is_selected = this.selection == Some(ix);

            match &this.entries[ix] {
                ContactEntry::Header(section) => {
                    let is_collapsed = this.collapsed_sections.contains(&section);
                    Self::render_header(
                        *section,
                        &theme.contacts_panel,
                        is_selected,
                        is_collapsed,
                        cx,
                    )
                }
                ContactEntry::IncomingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contacts_panel,
                    true,
                    is_selected,
                    cx,
                ),
                ContactEntry::OutgoingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contacts_panel,
                    false,
                    is_selected,
                    cx,
                ),
                ContactEntry::Contact(contact) => {
                    Self::render_contact(&contact.user, &theme.contacts_panel, is_selected)
                }
                ContactEntry::ContactProject(contact, project_ix, open_project) => {
                    let is_last_project_for_contact =
                        this.entries.get(ix + 1).map_or(true, |next| {
                            if let ContactEntry::ContactProject(next_contact, _, _) = next {
                                next_contact.user.id != contact.user.id
                            } else {
                                true
                            }
                        });
                    Self::render_project(
                        contact.clone(),
                        current_user_id,
                        *project_ix,
                        open_project.clone(),
                        &theme.contacts_panel,
                        &theme.tooltip,
                        is_last_project_for_contact,
                        is_selected,
                        cx,
                    )
                }
                ContactEntry::OfflineProject(project) => Self::render_offline_project(
                    project.clone(),
                    &theme.contacts_panel,
                    &theme.tooltip,
                    is_selected,
                    cx,
                ),
            }
        });

        let mut this = Self {
            list_state,
            selection: None,
            collapsed_sections: Default::default(),
            entries: Default::default(),
            match_candidates: Default::default(),
            filter_editor,
            _maintain_contacts: cx.observe(&user_store, |this, _, cx| this.update_entries(cx)),
            user_store,
            project_store,
        };
        this.update_entries(cx);
        this
    }

    fn render_header(
        section: Section,
        theme: &theme::ContactsPanel,
        is_selected: bool,
        is_collapsed: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum Header {}

        let header_style = theme.header_row.style_for(Default::default(), is_selected);
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
        .on_click(move |_, _, cx| cx.dispatch_action(ToggleExpanded(section)))
        .boxed()
    }

    fn render_contact(user: &User, theme: &theme::ContactsPanel, is_selected: bool) -> ElementBox {
        Flex::row()
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
            )
            .constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(*theme.contact_row.style_for(Default::default(), is_selected))
            .boxed()
    }

    fn render_project(
        contact: Arc<Contact>,
        current_user_id: Option<u64>,
        project_index: usize,
        open_project: Option<WeakModelHandle<Project>>,
        theme: &theme::ContactsPanel,
        tooltip_style: &TooltipStyle,
        is_last_project: bool,
        is_selected: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum ToggleOnline {}

        let project = &contact.projects[project_index];
        let project_id = project.id;
        let is_host = Some(contact.user.id) == current_user_id;
        let open_project = open_project.and_then(|p| p.upgrade(cx.deref_mut()));

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.project_row.default;
        let tree_branch = theme.tree_branch.clone();
        let line_height = row.name.text.line_height(font_cache);
        let cap_height = row.name.text.cap_height(font_cache);
        let baseline_offset =
            row.name.text.baseline_offset(font_cache) + (theme.row_height - line_height) / 2.;

        MouseEventHandler::new::<JoinProject, _, _>(project_id as usize, cx, |mouse_state, cx| {
            let tree_branch = *tree_branch.style_for(mouse_state, is_selected);
            let row = theme.project_row.style_for(mouse_state, is_selected);

            Flex::row()
                .with_child(
                    Stack::new()
                        .with_child(
                            Canvas::new(move |bounds, _, cx| {
                                let start_x = bounds.min_x() + (bounds.width() / 2.)
                                    - (tree_branch.width / 2.);
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
                            .boxed(),
                        )
                        .with_children(open_project.and_then(|open_project| {
                            let is_going_offline = !open_project.read(cx).is_online();
                            if !mouse_state.hovered && !is_going_offline {
                                return None;
                            }

                            let button = MouseEventHandler::new::<ToggleProjectOnline, _, _>(
                                project_id as usize,
                                cx,
                                |state, _| {
                                    let mut icon_style =
                                        *theme.private_button.style_for(state, false);
                                    icon_style.container.background_color =
                                        row.container.background_color;
                                    if is_going_offline {
                                        icon_style.color = theme.disabled_button.color;
                                    }
                                    render_icon_button(&icon_style, "icons/lock-8.svg")
                                        .aligned()
                                        .boxed()
                                },
                            );

                            if is_going_offline {
                                Some(button.boxed())
                            } else {
                                Some(
                                    button
                                        .with_cursor_style(CursorStyle::PointingHand)
                                        .on_click(move |_, _, cx| {
                                            cx.dispatch_action(ToggleProjectOnline {
                                                project: Some(open_project.clone()),
                                            })
                                        })
                                        .with_tooltip::<ToggleOnline, _>(
                                            project_id as usize,
                                            "Take project offline".to_string(),
                                            None,
                                            tooltip_style.clone(),
                                            cx,
                                        )
                                        .boxed(),
                                )
                            }
                        }))
                        .constrained()
                        .with_width(host_avatar_height)
                        .boxed(),
                )
                .with_child(
                    Label::new(
                        project.visible_worktree_root_names.join(", "),
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
        .with_cursor_style(if !is_host {
            CursorStyle::PointingHand
        } else {
            CursorStyle::Arrow
        })
        .on_click(move |_, _, cx| {
            if !is_host {
                cx.dispatch_global_action(JoinProject {
                    contact: contact.clone(),
                    project_index,
                });
            }
        })
        .boxed()
    }

    fn render_offline_project(
        project_handle: WeakModelHandle<Project>,
        theme: &theme::ContactsPanel,
        tooltip_style: &TooltipStyle,
        is_selected: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);

        enum LocalProject {}
        enum ToggleOnline {}

        let project_id = project_handle.id();
        MouseEventHandler::new::<LocalProject, _, _>(project_id, cx, |state, cx| {
            let row = theme.project_row.style_for(state, is_selected);
            let mut worktree_root_names = String::new();
            let project = if let Some(project) = project_handle.upgrade(cx.deref_mut()) {
                project.read(cx)
            } else {
                return Empty::new().boxed();
            };
            let is_going_online = project.is_online();
            for tree in project.visible_worktrees(cx) {
                if !worktree_root_names.is_empty() {
                    worktree_root_names.push_str(", ");
                }
                worktree_root_names.push_str(tree.read(cx).root_name());
            }

            Flex::row()
                .with_child({
                    let button =
                        MouseEventHandler::new::<ToggleOnline, _, _>(project_id, cx, |state, _| {
                            let mut style = *theme.private_button.style_for(state, false);
                            if is_going_online {
                                style.color = theme.disabled_button.color;
                            }
                            render_icon_button(&style, "icons/lock-8.svg")
                                .aligned()
                                .constrained()
                                .with_width(host_avatar_height)
                                .boxed()
                        });

                    if is_going_online {
                        button.boxed()
                    } else {
                        button
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(move |_, _, cx| {
                                let project = project_handle.upgrade(cx.deref_mut());
                                cx.dispatch_action(ToggleProjectOnline { project })
                            })
                            .with_tooltip::<ToggleOnline, _>(
                                project_id,
                                "Take project online".to_string(),
                                None,
                                tooltip_style.clone(),
                                cx,
                            )
                            .boxed()
                    }
                })
                .with_child(
                    Label::new(worktree_root_names, row.name.text.clone())
                        .aligned()
                        .left()
                        .contained()
                        .with_style(row.name.container)
                        .flex(1., false)
                        .boxed(),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(row.container)
                .boxed()
        })
        .boxed()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::ContactsPanel,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut RenderContext<ContactsPanel>,
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
                        &theme.disabled_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/decline.svg")
                        .aligned()
                        // .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, _, cx| {
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
                        &theme.disabled_button
                    } else {
                        &theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/accept.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, _, cx| {
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
                        &theme.disabled_button
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
                .on_click(move |_, _, cx| cx.dispatch_action(RemoveContact(user_id)))
                .flex_float()
                .boxed(),
            );
        }

        row.constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(*theme.contact_row.style_for(Default::default(), is_selected))
            .boxed()
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.read(cx);
        let project_store = self.project_store.read(cx);
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

        let current_user = user_store.current_user();

        let contacts = user_store.contacts();
        if !contacts.is_empty() {
            // Always put the current user first.
            self.match_candidates.clear();
            self.match_candidates.reserve(contacts.len());
            self.match_candidates.push(StringMatchCandidate {
                id: 0,
                string: Default::default(),
                char_bag: Default::default(),
            });
            for (ix, contact) in contacts.iter().enumerate() {
                let candidate = StringMatchCandidate {
                    id: ix,
                    string: contact.user.github_login.clone(),
                    char_bag: contact.user.github_login.chars().collect(),
                };
                if current_user
                    .as_ref()
                    .map_or(false, |current_user| current_user.id == contact.user.id)
                {
                    self.match_candidates[0] = candidate;
                } else {
                    self.match_candidates.push(candidate);
                }
            }
            if self.match_candidates[0].string.is_empty() {
                self.match_candidates.remove(0);
            }

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

                            let is_current_user = current_user
                                .as_ref()
                                .map_or(false, |user| user.id == contact.user.id);
                            if is_current_user {
                                let mut open_projects =
                                    project_store.projects(cx).collect::<Vec<_>>();
                                self.entries.extend(
                                    contact.projects.iter().enumerate().filter_map(
                                        |(ix, project)| {
                                            let open_project = open_projects
                                                .iter()
                                                .position(|p| {
                                                    p.read(cx).remote_id() == Some(project.id)
                                                })
                                                .map(|ix| open_projects.remove(ix).downgrade());
                                            if project.visible_worktree_root_names.is_empty() {
                                                None
                                            } else {
                                                Some(ContactEntry::ContactProject(
                                                    contact.clone(),
                                                    ix,
                                                    open_project,
                                                ))
                                            }
                                        },
                                    ),
                                );
                                self.entries.extend(open_projects.into_iter().filter_map(
                                    |project| {
                                        if project.read(cx).visible_worktrees(cx).next().is_none() {
                                            None
                                        } else {
                                            Some(ContactEntry::OfflineProject(project.downgrade()))
                                        }
                                    },
                                ));
                            } else {
                                self.entries.extend(
                                    contact.projects.iter().enumerate().filter_map(
                                        |(ix, project)| {
                                            if project.visible_worktree_root_names.is_empty() {
                                                None
                                            } else {
                                                Some(ContactEntry::ContactProject(
                                                    contact.clone(),
                                                    ix,
                                                    None,
                                                ))
                                            }
                                        },
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }

        if let Some(prev_selected_entry) = prev_selected_entry {
            self.selection.take();
            for (ix, entry) in self.entries.iter().enumerate() {
                if *entry == prev_selected_entry {
                    self.selection = Some(ix);
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
        let did_clear = self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx) > 0 {
                editor.set_text("", cx);
                true
            } else {
                false
            }
        });
        if !did_clear {
            cx.propagate_action();
        }
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
                    ContactEntry::ContactProject(contact, project_index, open_project) => {
                        if let Some(open_project) = open_project {
                            workspace::activate_workspace_for_project(cx, |_, cx| {
                                cx.model_id() == open_project.id()
                            });
                        } else {
                            cx.dispatch_global_action(JoinProject {
                                contact: contact.clone(),
                                project_index: *project_index,
                            })
                        }
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

    fn should_activate_item_on_event(&self, event: &Event, _: &AppContext) -> bool {
        matches!(event, Event::Activate)
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

pub enum Event {
    Activate,
}

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
                            .on_click(|_, _, cx| cx.dispatch_action(contact_finder::Toggle))
                            .boxed(),
                        )
                        .constrained()
                        .with_height(theme.user_query_editor_height)
                        .boxed(),
                )
                .with_child(List::new(self.list_state.clone()).flex(1., false).boxed())
                .with_children(
                    self.user_store
                        .read(cx)
                        .invite_info()
                        .cloned()
                        .and_then(|info| {
                            enum InviteLink {}

                            if info.count > 0 {
                                Some(
                                    MouseEventHandler::new::<InviteLink, _, _>(
                                        0,
                                        cx,
                                        |state, cx| {
                                            let style =
                                                theme.invite_row.style_for(state, false).clone();

                                            let copied =
                                                cx.read_from_clipboard().map_or(false, |item| {
                                                    item.text().as_str() == info.url.as_ref()
                                                });

                                            Label::new(
                                                format!(
                                                    "{} invite link ({} left)",
                                                    if copied { "Copied" } else { "Copy" },
                                                    info.count
                                                ),
                                                style.label.clone(),
                                            )
                                            .aligned()
                                            .left()
                                            .constrained()
                                            .with_height(theme.row_height)
                                            .contained()
                                            .with_style(style.container)
                                            .boxed()
                                        },
                                    )
                                    .with_cursor_style(CursorStyle::PointingHand)
                                    .on_click(move |_, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new(
                                            info.url.to_string(),
                                        ));
                                        cx.notify();
                                    })
                                    .boxed(),
                                )
                            } else {
                                None
                            }
                        }),
                )
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
            ContactEntry::ContactProject(contact_1, ix_1, _) => {
                if let ContactEntry::ContactProject(contact_2, ix_2, _) = other {
                    return contact_1.user.id == contact_2.user.id && ix_1 == ix_2;
                }
            }
            ContactEntry::OfflineProject(project_1) => {
                if let ContactEntry::OfflineProject(project_2) = other {
                    return project_1.id() == project_2.id();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{
        proto,
        test::{FakeHttpClient, FakeServer},
        Client,
    };
    use collections::HashSet;
    use gpui::{serde_json::json, TestAppContext};
    use language::LanguageRegistry;
    use project::{FakeFs, Project};

    #[gpui::test]
    async fn test_contact_panel(cx: &mut TestAppContext) {
        Settings::test_async(cx);
        let current_user_id = 100;

        let languages = Arc::new(LanguageRegistry::test());
        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project_store = cx.add_model(|_| ProjectStore::new(project::Db::open_fake()));
        let server = FakeServer::for_client(current_user_id, &client, &cx).await;
        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/private_dir", json!({ "one.rs": "" }))
            .await;
        let project = cx.update(|cx| {
            Project::local(
                false,
                client.clone(),
                user_store.clone(),
                project_store.clone(),
                languages,
                fs,
                cx,
            )
        });
        let worktree_id = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/private_dir", true, cx)
            })
            .await
            .unwrap()
            .0
            .read_with(cx, |worktree, _| worktree.id().to_proto());

        let workspace = cx.add_view(0, |cx| Workspace::new(project.clone(), cx));
        let panel = cx.add_view(0, |cx| {
            ContactsPanel::new(
                user_store.clone(),
                project_store.clone(),
                workspace.downgrade(),
                cx,
            )
        });

        workspace.update(cx, |_, cx| {
            cx.observe(&panel, |_, panel, cx| {
                let entries = render_to_strings(&panel, cx);
                assert!(
                    entries.iter().collect::<HashSet<_>>().len() == entries.len(),
                    "Duplicate contact panel entries {:?}",
                    entries
                )
            })
            .detach();
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
                    .chain([proto::User {
                        id: current_user_id,
                        github_login: "the_current_user".to_string(),
                        ..Default::default()
                    }])
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
                        visible_worktree_root_names: vec!["dir1".to_string()],
                        guests: vec![2],
                    }],
                },
                proto::Contact {
                    user_id: 4,
                    online: true,
                    should_notify: false,
                    projects: vec![proto::ProjectMetadata {
                        id: 102,
                        visible_worktree_root_names: vec!["dir2".to_string()],
                        guests: vec![2],
                    }],
                },
                proto::Contact {
                    user_id: 5,
                    online: false,
                    should_notify: false,
                    projects: vec![],
                },
                proto::Contact {
                    user_id: current_user_id,
                    online: true,
                    should_notify: false,
                    projects: vec![proto::ProjectMetadata {
                        id: 103,
                        visible_worktree_root_names: vec!["dir3".to_string()],
                        guests: vec![3],
                    }],
                },
            ],
            ..Default::default()
        });

        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    🔒 private_dir",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // Take a project online. It appears as loading, since the project
        // isn't yet visible to other contacts.
        project.update(cx, |project, cx| project.set_online(true, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    🔒 private_dir (going online...)",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // The server responds, assigning the project a remote id. It still appears
        // as loading, because the server hasn't yet sent out the updated contact
        // state for the current user.
        let request = server.receive::<proto::RegisterProject>().await.unwrap();
        server
            .respond(
                request.receipt(),
                proto::RegisterProjectResponse { project_id: 200 },
            )
            .await;
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    🔒 private_dir (going online...)",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // The server receives the project's metadata and updates the contact metadata
        // for the current user. Now the project appears as online.
        assert_eq!(
            server
                .receive::<proto::UpdateProject>()
                .await
                .unwrap()
                .payload
                .worktrees,
            &[proto::WorktreeMetadata {
                id: worktree_id,
                root_name: "private_dir".to_string(),
                visible: true,
            }],
        );
        server.send(proto::UpdateContacts {
            contacts: vec![proto::Contact {
                user_id: current_user_id,
                online: true,
                should_notify: false,
                projects: vec![
                    proto::ProjectMetadata {
                        id: 103,
                        visible_worktree_root_names: vec!["dir3".to_string()],
                        guests: vec![3],
                    },
                    proto::ProjectMetadata {
                        id: 200,
                        visible_worktree_root_names: vec!["private_dir".to_string()],
                        guests: vec![3],
                    },
                ],
            }],
            ..Default::default()
        });
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    private_dir",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // Take the project offline. It appears as loading.
        project.update(cx, |project, cx| project.set_online(false, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    private_dir (going offline...)",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // The server receives the unregister request and updates the contact
        // metadata for the current user. The project is now offline.
        let request = server.receive::<proto::UnregisterProject>().await.unwrap();
        server.send(proto::UpdateContacts {
            contacts: vec![proto::Contact {
                user_id: current_user_id,
                online: true,
                should_notify: false,
                projects: vec![proto::ProjectMetadata {
                    id: 103,
                    visible_worktree_root_names: vec!["dir3".to_string()],
                    guests: vec![3],
                }],
            }],
            ..Default::default()
        });
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    🔒 private_dir",
                "  user_four",
                "    dir2",
                "  user_three",
                "    dir1",
                "v Offline",
                "  user_five",
            ]
        );

        // The server responds to the unregister request.
        server.respond(request.receipt(), proto::Ack {}).await;
        cx.foreground().run_until_parked();
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Requests",
                "  incoming user_one",
                "  outgoing user_two",
                "v Online",
                "  the_current_user",
                "    dir3",
                "    🔒 private_dir",
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
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Online",
                "  user_four  <=== selected",
                "    dir2",
                "v Offline",
                "  user_five",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
        });
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Online",
                "  user_four",
                "    dir2  <=== selected",
                "v Offline",
                "  user_five",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
        });
        assert_eq!(
            cx.read(|cx| render_to_strings(&panel, cx)),
            &[
                "v Online",
                "  user_four",
                "    dir2",
                "v Offline  <=== selected",
                "  user_five",
            ]
        );
    }

    fn render_to_strings(panel: &ViewHandle<ContactsPanel>, cx: &AppContext) -> Vec<String> {
        let panel = panel.read(cx);
        let mut entries = Vec::new();
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
                ContactEntry::ContactProject(contact, project_ix, project) => {
                    let project = project
                        .and_then(|p| p.upgrade(cx))
                        .map(|project| project.read(cx));
                    format!(
                        "    {}{}",
                        contact.projects[*project_ix]
                            .visible_worktree_root_names
                            .join(", "),
                        if project.map_or(true, |project| project.is_online()) {
                            ""
                        } else {
                            " (going offline...)"
                        },
                    )
                }
                ContactEntry::OfflineProject(project) => {
                    let project = project.upgrade(cx).unwrap().read(cx);
                    format!(
                        "    🔒 {}{}",
                        project
                            .worktree_root_names(cx)
                            .collect::<Vec<_>>()
                            .join(", "),
                        if project.is_online() {
                            " (going online...)"
                        } else {
                            ""
                        },
                    )
                }
            };

            if panel.selection == Some(ix) {
                string.push_str("  <=== selected");
            }

            string
        }));
        entries
    }
}
