use call::ActiveCall;
use client::{proto::PeerId, Contact, User, UserStore};
use editor::{Cancel, Editor};
use futures::StreamExt;
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton, PromptLevel},
    AppContext, Entity, ModelHandle, Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use menu::{Confirm, SelectNext, SelectPrev};
use project::Project;
use serde::Deserialize;
use settings::Settings;
use std::{mem, sync::Arc};
use theme::IconButton;
use workspace::Workspace;

impl_actions!(contact_list, [RemoveContact, RespondToContactRequest]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(ContactList::remove_contact);
    cx.add_action(ContactList::respond_to_contact_request);
    cx.add_action(ContactList::cancel);
    cx.add_action(ContactList::select_next);
    cx.add_action(ContactList::select_prev);
    cx.add_action(ContactList::confirm);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Section {
    ActiveCall,
    Requests,
    Online,
    Offline,
}

#[derive(Clone)]
enum ContactEntry {
    Header(Section),
    CallParticipant {
        user: Arc<User>,
        is_pending: bool,
    },
    ParticipantProject {
        project_id: u64,
        worktree_root_names: Vec<String>,
        host_user_id: u64,
        is_last: bool,
    },
    ParticipantScreen {
        peer_id: PeerId,
        is_last: bool,
    },
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    Contact {
        contact: Arc<Contact>,
        calling: bool,
    },
}

impl PartialEq for ContactEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ContactEntry::Header(section_1) => {
                if let ContactEntry::Header(section_2) = other {
                    return section_1 == section_2;
                }
            }
            ContactEntry::CallParticipant { user: user_1, .. } => {
                if let ContactEntry::CallParticipant { user: user_2, .. } = other {
                    return user_1.id == user_2.id;
                }
            }
            ContactEntry::ParticipantProject {
                project_id: project_id_1,
                ..
            } => {
                if let ContactEntry::ParticipantProject {
                    project_id: project_id_2,
                    ..
                } = other
                {
                    return project_id_1 == project_id_2;
                }
            }
            ContactEntry::ParticipantScreen {
                peer_id: peer_id_1, ..
            } => {
                if let ContactEntry::ParticipantScreen {
                    peer_id: peer_id_2, ..
                } = other
                {
                    return peer_id_1 == peer_id_2;
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
            ContactEntry::Contact {
                contact: contact_1, ..
            } => {
                if let ContactEntry::Contact {
                    contact: contact_2, ..
                } = other
                {
                    return contact_1.user.id == contact_2.user.id;
                }
            }
        }
        false
    }
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct RequestContact(pub u64);

#[derive(Clone, Deserialize, PartialEq)]
pub struct RemoveContact {
    user_id: u64,
    github_login: String,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct RespondToContactRequest {
    pub user_id: u64,
    pub accept: bool,
}

pub enum Event {
    ToggleContactFinder,
    Dismissed,
}

pub struct ContactList {
    entries: Vec<ContactEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState<Self>,
    project: ModelHandle<Project>,
    workspace: WeakViewHandle<Workspace>,
    user_store: ModelHandle<UserStore>,
    filter_editor: ViewHandle<Editor>,
    collapsed_sections: Vec<Section>,
    selection: Option<usize>,
    _subscriptions: Vec<Subscription>,
}

impl ContactList {
    pub fn new(
        project: ModelHandle<Project>,
        user_store: ModelHandle<UserStore>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let filter_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| {
                    theme.contact_list.user_query_editor.clone()
                })),
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

        let list_state = ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
            let theme = cx.global::<Settings>().theme.clone();
            let is_selected = this.selection == Some(ix);
            let current_project_id = this.project.read(cx).remote_id();

            match &this.entries[ix] {
                ContactEntry::Header(section) => {
                    let is_collapsed = this.collapsed_sections.contains(section);
                    Self::render_header(
                        *section,
                        &theme.contact_list,
                        is_selected,
                        is_collapsed,
                        cx,
                    )
                }
                ContactEntry::CallParticipant { user, is_pending } => {
                    Self::render_call_participant(
                        user,
                        *is_pending,
                        is_selected,
                        &theme.contact_list,
                    )
                }
                ContactEntry::ParticipantProject {
                    project_id,
                    worktree_root_names,
                    host_user_id,
                    is_last,
                } => Self::render_participant_project(
                    *project_id,
                    worktree_root_names,
                    *host_user_id,
                    Some(*project_id) == current_project_id,
                    *is_last,
                    is_selected,
                    &theme.contact_list,
                    cx,
                ),
                ContactEntry::ParticipantScreen { peer_id, is_last } => {
                    Self::render_participant_screen(
                        *peer_id,
                        *is_last,
                        is_selected,
                        &theme.contact_list,
                        cx,
                    )
                }
                ContactEntry::IncomingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contact_list,
                    true,
                    is_selected,
                    cx,
                ),
                ContactEntry::OutgoingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contact_list,
                    false,
                    is_selected,
                    cx,
                ),
                ContactEntry::Contact { contact, calling } => Self::render_contact(
                    contact,
                    *calling,
                    &this.project,
                    &theme.contact_list,
                    is_selected,
                    cx,
                ),
            }
        });

        let active_call = ActiveCall::global(cx);
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(&user_store, |this, _, cx| this.update_entries(cx)));
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.update_entries(cx)));

        let mut this = Self {
            list_state,
            selection: None,
            collapsed_sections: Default::default(),
            entries: Default::default(),
            match_candidates: Default::default(),
            filter_editor,
            _subscriptions: subscriptions,
            project,
            workspace,
            user_store,
        };
        this.update_entries(cx);
        this
    }

    pub fn editor_text(&self, cx: &AppContext) -> String {
        self.filter_editor.read(cx).text(cx)
    }

    pub fn with_editor_text(self, editor_text: String, cx: &mut ViewContext<Self>) -> Self {
        self.filter_editor
            .update(cx, |picker, cx| picker.set_text(editor_text, cx));
        self
    }

    fn remove_contact(&mut self, request: &RemoveContact, cx: &mut ViewContext<Self>) {
        let user_id = request.user_id;
        let github_login = &request.github_login;
        let user_store = self.user_store.clone();
        let prompt_message = format!(
            "Are you sure you want to remove \"{}\" from your contacts?",
            github_login
        );
        let mut answer = cx.prompt(PromptLevel::Warning, &prompt_message, &["Remove", "Cancel"]);
        let window_id = cx.window_id();
        cx.spawn(|_, mut cx| async move {
            if answer.next().await == Some(0) {
                if let Err(e) = user_store
                    .update(&mut cx, |store, cx| store.remove_contact(user_id, cx))
                    .await
                {
                    cx.prompt(
                        window_id,
                        PromptLevel::Info,
                        &format!("Failed to remove contact: {}", e),
                        &["Ok"],
                    );
                }
            }
        })
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

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        let did_clear = self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx) > 0 {
                editor.set_text("", cx);
                true
            } else {
                false
            }
        });

        if !did_clear {
            cx.emit(Event::Dismissed);
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
        self.list_state.reset(self.entries.len());
        if let Some(ix) = self.selection {
            self.list_state.scroll_to(ListOffset {
                item_ix: ix,
                offset_in_item: 0.,
            });
        }
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selection {
            if ix > 0 {
                self.selection = Some(ix - 1);
            } else {
                self.selection = None;
            }
        }
        self.list_state.reset(self.entries.len());
        if let Some(ix) = self.selection {
            self.list_state.scroll_to(ListOffset {
                item_ix: ix,
                offset_in_item: 0.,
            });
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            if let Some(entry) = self.entries.get(selection) {
                match entry {
                    ContactEntry::Header(section) => {
                        self.toggle_expanded(*section, cx);
                    }
                    ContactEntry::Contact { contact, calling } => {
                        if contact.online && !contact.busy && !calling {
                            self.call(contact.user.id, Some(self.project.clone()), cx);
                        }
                    }
                    ContactEntry::ParticipantProject {
                        project_id,
                        host_user_id,
                        ..
                    } => {
                        if let Some(workspace) = self.workspace.upgrade(cx) {
                            let app_state = workspace.read(cx).app_state().clone();
                            workspace::join_remote_project(
                                *project_id,
                                *host_user_id,
                                app_state,
                                cx,
                            )
                            .detach_and_log_err(cx);
                        }
                    }
                    ContactEntry::ParticipantScreen { peer_id, .. } => {
                        if let Some(workspace) = self.workspace.upgrade(cx) {
                            workspace.update(cx, |workspace, cx| {
                                workspace.open_shared_screen(*peer_id, cx)
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn toggle_expanded(&mut self, section: Section, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.collapsed_sections.iter().position(|s| *s == section) {
            self.collapsed_sections.remove(ix);
        } else {
            self.collapsed_sections.push(section);
        }
        self.update_entries(cx);
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.read(cx);
        let query = self.filter_editor.read(cx).text(cx);
        let executor = cx.background().clone();

        let prev_selected_entry = self.selection.and_then(|ix| self.entries.get(ix).cloned());
        let old_entries = mem::take(&mut self.entries);

        if let Some(room) = ActiveCall::global(cx).read(cx).room() {
            let room = room.read(cx);
            let mut participant_entries = Vec::new();

            // Populate the active user.
            if let Some(user) = user_store.current_user() {
                self.match_candidates.clear();
                self.match_candidates.push(StringMatchCandidate {
                    id: 0,
                    string: user.github_login.clone(),
                    char_bag: user.github_login.chars().collect(),
                });
                let matches = executor.block(match_strings(
                    &self.match_candidates,
                    &query,
                    true,
                    usize::MAX,
                    &Default::default(),
                    executor.clone(),
                ));
                if !matches.is_empty() {
                    let user_id = user.id;
                    participant_entries.push(ContactEntry::CallParticipant {
                        user,
                        is_pending: false,
                    });
                    let mut projects = room.local_participant().projects.iter().peekable();
                    while let Some(project) = projects.next() {
                        participant_entries.push(ContactEntry::ParticipantProject {
                            project_id: project.id,
                            worktree_root_names: project.worktree_root_names.clone(),
                            host_user_id: user_id,
                            is_last: projects.peek().is_none(),
                        });
                    }
                }
            }

            // Populate remote participants.
            self.match_candidates.clear();
            self.match_candidates
                .extend(room.remote_participants().iter().map(|(_, participant)| {
                    StringMatchCandidate {
                        id: participant.user.id as usize,
                        string: participant.user.github_login.clone(),
                        char_bag: participant.user.github_login.chars().collect(),
                    }
                }));
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));
            for mat in matches {
                let user_id = mat.candidate_id as u64;
                let participant = &room.remote_participants()[&user_id];
                participant_entries.push(ContactEntry::CallParticipant {
                    user: participant.user.clone(),
                    is_pending: false,
                });
                let mut projects = participant.projects.iter().peekable();
                while let Some(project) = projects.next() {
                    participant_entries.push(ContactEntry::ParticipantProject {
                        project_id: project.id,
                        worktree_root_names: project.worktree_root_names.clone(),
                        host_user_id: participant.user.id,
                        is_last: projects.peek().is_none() && participant.tracks.is_empty(),
                    });
                }
                if !participant.tracks.is_empty() {
                    participant_entries.push(ContactEntry::ParticipantScreen {
                        peer_id: participant.peer_id,
                        is_last: true,
                    });
                }
            }

            // Populate pending participants.
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    room.pending_participants()
                        .iter()
                        .enumerate()
                        .map(|(id, participant)| StringMatchCandidate {
                            id,
                            string: participant.github_login.clone(),
                            char_bag: participant.github_login.chars().collect(),
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
            participant_entries.extend(matches.iter().map(|mat| ContactEntry::CallParticipant {
                user: room.pending_participants()[mat.candidate_id].clone(),
                is_pending: true,
            }));

            if !participant_entries.is_empty() {
                self.entries.push(ContactEntry::Header(Section::ActiveCall));
                if !self.collapsed_sections.contains(&Section::ActiveCall) {
                    self.entries.extend(participant_entries);
                }
            }
        }

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

            let (mut online_contacts, offline_contacts) = matches
                .iter()
                .partition::<Vec<_>, _>(|mat| contacts[mat.candidate_id].online);
            if let Some(room) = ActiveCall::global(cx).read(cx).room() {
                let room = room.read(cx);
                online_contacts.retain(|contact| {
                    let contact = &contacts[contact.candidate_id];
                    !room.contains_participant(contact.user.id)
                });
            }

            for (matches, section) in [
                (online_contacts, Section::Online),
                (offline_contacts, Section::Offline),
            ] {
                if !matches.is_empty() {
                    self.entries.push(ContactEntry::Header(section));
                    if !self.collapsed_sections.contains(&section) {
                        let active_call = &ActiveCall::global(cx).read(cx);
                        for mat in matches {
                            let contact = &contacts[mat.candidate_id];
                            self.entries.push(ContactEntry::Contact {
                                contact: contact.clone(),
                                calling: active_call.pending_invites().contains(&contact.user.id),
                            });
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

        let old_scroll_top = self.list_state.logical_scroll_top();
        self.list_state.reset(self.entries.len());

        // Attempt to maintain the same scroll position.
        if let Some(old_top_entry) = old_entries.get(old_scroll_top.item_ix) {
            let new_scroll_top = self
                .entries
                .iter()
                .position(|entry| entry == old_top_entry)
                .map(|item_ix| ListOffset {
                    item_ix,
                    offset_in_item: old_scroll_top.offset_in_item,
                })
                .or_else(|| {
                    let entry_after_old_top = old_entries.get(old_scroll_top.item_ix + 1)?;
                    let item_ix = self
                        .entries
                        .iter()
                        .position(|entry| entry == entry_after_old_top)?;
                    Some(ListOffset {
                        item_ix,
                        offset_in_item: 0.,
                    })
                })
                .or_else(|| {
                    let entry_before_old_top =
                        old_entries.get(old_scroll_top.item_ix.saturating_sub(1))?;
                    let item_ix = self
                        .entries
                        .iter()
                        .position(|entry| entry == entry_before_old_top)?;
                    Some(ListOffset {
                        item_ix,
                        offset_in_item: 0.,
                    })
                });

            self.list_state
                .scroll_to(new_scroll_top.unwrap_or(old_scroll_top));
        }

        cx.notify();
    }

    fn render_call_participant(
        user: &User,
        is_pending: bool,
        is_selected: bool,
        theme: &theme::ContactList,
    ) -> AnyElement<Self> {
        Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
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
                .flex(1., true),
            )
            .with_children(if is_pending {
                Some(
                    Label::new("Calling", theme.calling_indicator.text.clone())
                        .contained()
                        .with_style(theme.calling_indicator.container)
                        .aligned(),
                )
            } else {
                None
            })
            .constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(
                *theme
                    .contact_row
                    .style_for(&mut Default::default(), is_selected),
            )
            .into_any()
    }

    fn render_participant_project(
        project_id: u64,
        worktree_root_names: &[String],
        host_user_id: u64,
        is_current: bool,
        is_last: bool,
        is_selected: bool,
        theme: &theme::ContactList,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum JoinProject {}

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.project_row.default;
        let tree_branch = theme.tree_branch;
        let line_height = row.name.text.line_height(font_cache);
        let cap_height = row.name.text.cap_height(font_cache);
        let baseline_offset =
            row.name.text.baseline_offset(font_cache) + (theme.row_height - line_height) / 2.;
        let project_name = if worktree_root_names.is_empty() {
            "untitled".to_string()
        } else {
            worktree_root_names.join(", ")
        };

        MouseEventHandler::<JoinProject, Self>::new(project_id as usize, cx, |mouse_state, _| {
            let tree_branch = *tree_branch.style_for(mouse_state, is_selected);
            let row = theme.project_row.style_for(mouse_state, is_selected);

            Flex::row()
                .with_child(
                    Stack::new()
                        .with_child(Canvas::new(move |scene, bounds, _, _, _| {
                            let start_x =
                                bounds.min_x() + (bounds.width() / 2.) - (tree_branch.width / 2.);
                            let end_x = bounds.max_x();
                            let start_y = bounds.min_y();
                            let end_y = bounds.min_y() + baseline_offset - (cap_height / 2.);

                            scene.push_quad(gpui::Quad {
                                bounds: RectF::from_points(
                                    vec2f(start_x, start_y),
                                    vec2f(
                                        start_x + tree_branch.width,
                                        if is_last { end_y } else { bounds.max_y() },
                                    ),
                                ),
                                background: Some(tree_branch.color),
                                border: gpui::Border::default(),
                                corner_radius: 0.,
                            });
                            scene.push_quad(gpui::Quad {
                                bounds: RectF::from_points(
                                    vec2f(start_x, end_y),
                                    vec2f(end_x, end_y + tree_branch.width),
                                ),
                                background: Some(tree_branch.color),
                                border: gpui::Border::default(),
                                corner_radius: 0.,
                            });
                        }))
                        .constrained()
                        .with_width(host_avatar_height),
                )
                .with_child(
                    Label::new(project_name, row.name.text.clone())
                        .aligned()
                        .left()
                        .contained()
                        .with_style(row.name.container)
                        .flex(1., false),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(row.container)
        })
        .with_cursor_style(if !is_current {
            CursorStyle::PointingHand
        } else {
            CursorStyle::Arrow
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            if !is_current {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    let app_state = workspace.read(cx).app_state().clone();
                    workspace::join_remote_project(project_id, host_user_id, app_state, cx)
                        .detach_and_log_err(cx);
                }
            }
        })
        .into_any()
    }

    fn render_participant_screen(
        peer_id: PeerId,
        is_last: bool,
        is_selected: bool,
        theme: &theme::ContactList,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum OpenSharedScreen {}

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.project_row.default;
        let tree_branch = theme.tree_branch;
        let line_height = row.name.text.line_height(font_cache);
        let cap_height = row.name.text.cap_height(font_cache);
        let baseline_offset =
            row.name.text.baseline_offset(font_cache) + (theme.row_height - line_height) / 2.;

        MouseEventHandler::<OpenSharedScreen, Self>::new(
            peer_id.as_u64() as usize,
            cx,
            |mouse_state, _| {
                let tree_branch = *tree_branch.style_for(mouse_state, is_selected);
                let row = theme.project_row.style_for(mouse_state, is_selected);

                Flex::row()
                    .with_child(
                        Stack::new()
                            .with_child(Canvas::new(move |scene, bounds, _, _, _| {
                                let start_x = bounds.min_x() + (bounds.width() / 2.)
                                    - (tree_branch.width / 2.);
                                let end_x = bounds.max_x();
                                let start_y = bounds.min_y();
                                let end_y = bounds.min_y() + baseline_offset - (cap_height / 2.);

                                scene.push_quad(gpui::Quad {
                                    bounds: RectF::from_points(
                                        vec2f(start_x, start_y),
                                        vec2f(
                                            start_x + tree_branch.width,
                                            if is_last { end_y } else { bounds.max_y() },
                                        ),
                                    ),
                                    background: Some(tree_branch.color),
                                    border: gpui::Border::default(),
                                    corner_radius: 0.,
                                });
                                scene.push_quad(gpui::Quad {
                                    bounds: RectF::from_points(
                                        vec2f(start_x, end_y),
                                        vec2f(end_x, end_y + tree_branch.width),
                                    ),
                                    background: Some(tree_branch.color),
                                    border: gpui::Border::default(),
                                    corner_radius: 0.,
                                });
                            }))
                            .constrained()
                            .with_width(host_avatar_height),
                    )
                    .with_child(
                        Svg::new("icons/disable_screen_sharing_12.svg")
                            .with_color(row.icon.color)
                            .constrained()
                            .with_width(row.icon.width)
                            .aligned()
                            .left()
                            .contained()
                            .with_style(row.icon.container),
                    )
                    .with_child(
                        Label::new("Screen", row.name.text.clone())
                            .aligned()
                            .left()
                            .contained()
                            .with_style(row.name.container)
                            .flex(1., false),
                    )
                    .constrained()
                    .with_height(theme.row_height)
                    .contained()
                    .with_style(row.container)
            },
        )
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            if let Some(workspace) = this.workspace.upgrade(cx) {
                workspace.update(cx, |workspace, cx| {
                    workspace.open_shared_screen(peer_id, cx)
                });
            }
        })
        .into_any()
    }

    fn render_header(
        section: Section,
        theme: &theme::ContactList,
        is_selected: bool,
        is_collapsed: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum Header {}
        enum LeaveCallContactList {}

        let header_style = theme
            .header_row
            .style_for(&mut Default::default(), is_selected);
        let text = match section {
            Section::ActiveCall => "Collaborators",
            Section::Requests => "Contact Requests",
            Section::Online => "Online",
            Section::Offline => "Offline",
        };
        let leave_call = if section == Section::ActiveCall {
            Some(
                MouseEventHandler::<LeaveCallContactList, Self>::new(0, cx, |state, _| {
                    let style = theme.leave_call.style_for(state, false);
                    Label::new("Leave Call", style.text.clone())
                        .contained()
                        .with_style(style.container)
                })
                .on_click(MouseButton::Left, |_, _, cx| {
                    ActiveCall::global(cx)
                        .update(cx, |call, cx| call.hang_up(cx))
                        .detach_and_log_err(cx);
                })
                .aligned(),
            )
        } else {
            None
        };

        let icon_size = theme.section_icon_size;
        MouseEventHandler::<Header, Self>::new(section as usize, cx, |_, _| {
            Flex::row()
                .with_child(
                    Svg::new(if is_collapsed {
                        "icons/chevron_right_8.svg"
                    } else {
                        "icons/chevron_down_8.svg"
                    })
                    .with_color(header_style.text.color)
                    .constrained()
                    .with_max_width(icon_size)
                    .with_max_height(icon_size)
                    .aligned()
                    .constrained()
                    .with_width(icon_size),
                )
                .with_child(
                    Label::new(text, header_style.text.clone())
                        .aligned()
                        .left()
                        .contained()
                        .with_margin_left(theme.contact_username.container.margin.left)
                        .flex(1., true),
                )
                .with_children(leave_call)
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(header_style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.toggle_expanded(section, cx);
        })
        .into_any()
    }

    fn render_contact(
        contact: &Contact,
        calling: bool,
        project: &ModelHandle<Project>,
        theme: &theme::ContactList,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let online = contact.online;
        let busy = contact.busy || calling;
        let user_id = contact.user.id;
        let github_login = contact.user.github_login.clone();
        let initial_project = project.clone();
        let mut event_handler =
            MouseEventHandler::<Contact, Self>::new(contact.user.id as usize, cx, |_, cx| {
                Flex::row()
                    .with_children(contact.user.avatar.clone().map(|avatar| {
                        let status_badge = if contact.online {
                            Some(
                                Empty::new()
                                    .collapsed()
                                    .contained()
                                    .with_style(if busy {
                                        theme.contact_status_busy
                                    } else {
                                        theme.contact_status_free
                                    })
                                    .aligned(),
                            )
                        } else {
                            None
                        };
                        Stack::new()
                            .with_child(
                                Image::from_data(avatar)
                                    .with_style(theme.contact_avatar)
                                    .aligned()
                                    .left(),
                            )
                            .with_children(status_badge)
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
                        .flex(1., true),
                    )
                    .with_child(
                        MouseEventHandler::<Cancel, Self>::new(
                            contact.user.id as usize,
                            cx,
                            |mouse_state, _| {
                                let button_style =
                                    theme.contact_button.style_for(mouse_state, false);
                                render_icon_button(button_style, "icons/x_mark_8.svg")
                                    .aligned()
                                    .flex_float()
                            },
                        )
                        .with_padding(Padding::uniform(2.))
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, move |_, this, cx| {
                            this.remove_contact(
                                &RemoveContact {
                                    user_id,
                                    github_login: github_login.clone(),
                                },
                                cx,
                            );
                        })
                        .flex_float(),
                    )
                    .with_children(if calling {
                        Some(
                            Label::new("Calling", theme.calling_indicator.text.clone())
                                .contained()
                                .with_style(theme.calling_indicator.container)
                                .aligned(),
                        )
                    } else {
                        None
                    })
                    .constrained()
                    .with_height(theme.row_height)
                    .contained()
                    .with_style(
                        *theme
                            .contact_row
                            .style_for(&mut Default::default(), is_selected),
                    )
            })
            .on_click(MouseButton::Left, move |_, this, cx| {
                if online && !busy {
                    this.call(user_id, Some(initial_project.clone()), cx);
                }
            });

        if online {
            event_handler = event_handler.with_cursor_style(CursorStyle::PointingHand);
        }

        event_handler.into_any()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::ContactList,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum Decline {}
        enum Accept {}
        enum Cancel {}

        let mut row = Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::from_data(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
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
                .flex(1., true),
            );

        let user_id = user.id;
        let github_login = user.github_login.clone();
        let is_contact_request_pending = user_store.read(cx).is_contact_request_pending(&user);
        let button_spacing = theme.contact_button_spacing;

        if is_incoming {
            row.add_child(
                MouseEventHandler::<Decline, Self>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg").aligned()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_contact_request(
                        &RespondToContactRequest {
                            user_id,
                            accept: false,
                        },
                        cx,
                    );
                })
                .contained()
                .with_margin_right(button_spacing),
            );

            row.add_child(
                MouseEventHandler::<Accept, Self>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/check_8.svg")
                        .aligned()
                        .flex_float()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_contact_request(
                        &RespondToContactRequest {
                            user_id,
                            accept: true,
                        },
                        cx,
                    );
                }),
            );
        } else {
            row.add_child(
                MouseEventHandler::<Cancel, Self>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg")
                        .aligned()
                        .flex_float()
                })
                .with_padding(Padding::uniform(2.))
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.remove_contact(
                        &RemoveContact {
                            user_id,
                            github_login: github_login.clone(),
                        },
                        cx,
                    );
                })
                .flex_float(),
            );
        }

        row.constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(
                *theme
                    .contact_row
                    .style_for(&mut Default::default(), is_selected),
            )
            .into_any()
    }

    fn call(
        &mut self,
        recipient_user_id: u64,
        initial_project: Option<ModelHandle<Project>>,
        cx: &mut ViewContext<Self>,
    ) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| {
                call.invite(recipient_user_id, initial_project, cx)
            })
            .detach_and_log_err(cx);
    }
}

impl Entity for ContactList {
    type Event = Event;
}

impl View for ContactList {
    fn ui_name() -> &'static str {
        "ContactList"
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
        keymap.add_identifier("menu");
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum AddContact {}
        let theme = cx.global::<Settings>().theme.clone();

        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(
                        ChildView::new(&self.filter_editor, cx)
                            .contained()
                            .with_style(theme.contact_list.user_query_editor.container)
                            .flex(1., true),
                    )
                    .with_child(
                        MouseEventHandler::<AddContact, Self>::new(0, cx, |_, _| {
                            render_icon_button(
                                &theme.contact_list.add_contact_button,
                                "icons/user_plus_16.svg",
                            )
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, |_, _, cx| {
                            cx.emit(Event::ToggleContactFinder)
                        })
                        .with_tooltip::<AddContact>(
                            0,
                            "Search for new contact".into(),
                            None,
                            theme.tooltip.clone(),
                            cx,
                        ),
                    )
                    .constrained()
                    .with_height(theme.contact_list.user_query_editor_height),
            )
            .with_child(List::new(self.list_state.clone()).flex(1., false))
            .into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.filter_editor.is_focused(cx) {
            cx.focus(&self.filter_editor);
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.filter_editor.is_focused(cx) {
            cx.emit(Event::Dismissed);
        }
    }
}

fn render_icon_button(style: &IconButton, svg_path: &'static str) -> impl Element<ContactList> {
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
