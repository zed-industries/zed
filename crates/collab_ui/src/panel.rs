mod channel_modal;
mod contact_finder;
mod panel_settings;

use anyhow::Result;
use call::ActiveCall;
use client::{proto::PeerId, Channel, ChannelStore, Client, Contact, User, UserStore};
use contact_finder::build_contact_finder;
use context_menu::{ContextMenu, ContextMenuItem};
use db::kvp::KEY_VALUE_STORE;
use editor::{Cancel, Editor};
use futures::StreamExt;
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions,
    elements::{
        Canvas, ChildView, Empty, Flex, Image, Label, List, ListOffset, ListState,
        MouseEventHandler, Orientation, Padding, ParentElement, Stack, Svg,
    },
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    impl_actions,
    platform::{CursorStyle, MouseButton, PromptLevel},
    serde_json, AnyElement, AppContext, AsyncAppContext, Element, Entity, ModelHandle,
    Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use menu::{Confirm, SelectNext, SelectPrev};
use panel_settings::{ChannelsPanelDockPosition, ChannelsPanelSettings};
use project::{Fs, Project};
use serde_derive::{Deserialize, Serialize};
use settings::SettingsStore;
use std::{mem, sync::Arc};
use theme::IconButton;
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    item::ItemHandle,
    Workspace,
};

use crate::face_pile::FacePile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct RemoveChannel {
    channel_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct NewChannel {
    channel_id: u64,
}

actions!(collab_panel, [ToggleFocus]);

impl_actions!(collab_panel, [RemoveChannel, NewChannel]);

const CHANNELS_PANEL_KEY: &'static str = "ChannelsPanel";

pub fn init(_client: Arc<Client>, cx: &mut AppContext) {
    settings::register::<panel_settings::ChannelsPanelSettings>(cx);
    contact_finder::init(cx);
    channel_modal::init(cx);

    cx.add_action(CollabPanel::cancel);
    cx.add_action(CollabPanel::select_next);
    cx.add_action(CollabPanel::select_prev);
    cx.add_action(CollabPanel::confirm);
    cx.add_action(CollabPanel::remove_channel);
    cx.add_action(CollabPanel::new_subchannel);
}

#[derive(Debug, Default)]
pub struct ChannelEditingState {
    parent_id: Option<u64>,
}

pub struct CollabPanel {
    width: Option<f32>,
    fs: Arc<dyn Fs>,
    has_focus: bool,
    pending_serialization: Task<Option<()>>,
    context_menu: ViewHandle<ContextMenu>,
    filter_editor: ViewHandle<Editor>,
    channel_name_editor: ViewHandle<Editor>,
    channel_editing_state: Option<ChannelEditingState>,
    entries: Vec<ListEntry>,
    selection: Option<usize>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    project: ModelHandle<Project>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState<Self>,
    subscriptions: Vec<Subscription>,
    collapsed_sections: Vec<Section>,
    workspace: WeakViewHandle<Workspace>,
}

#[derive(Serialize, Deserialize)]
struct SerializedChannelsPanel {
    width: Option<f32>,
}

#[derive(Debug)]
pub enum Event {
    DockPositionChanged,
    Focus,
    Dismissed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Section {
    ActiveCall,
    Channels,
    Requests,
    Contacts,
    Online,
    Offline,
}

#[derive(Clone, Debug)]
enum ListEntry {
    Header(Section, usize),
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
    ChannelInvite(Arc<Channel>),
    Channel(Arc<Channel>),
    ChannelEditor {
        depth: usize,
    },
    Contact {
        contact: Arc<Contact>,
        calling: bool,
    },
}

impl Entity for CollabPanel {
    type Event = Event;
}

impl CollabPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        cx.add_view::<Self, _>(|cx| {
            let view_id = cx.view_id();

            let filter_editor = cx.add_view(|cx| {
                let mut editor = Editor::single_line(
                    Some(Arc::new(|theme| {
                        theme.collab_panel.user_query_editor.clone()
                    })),
                    cx,
                );
                editor.set_placeholder_text("Filter channels, contacts", cx);
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
                            .position(|entry| !matches!(entry, ListEntry::Header(_, _)));
                    }
                }
            })
            .detach();

            let channel_name_editor = cx.add_view(|cx| {
                Editor::single_line(
                    Some(Arc::new(|theme| {
                        theme.collab_panel.user_query_editor.clone()
                    })),
                    cx,
                )
            });

            cx.subscribe(&channel_name_editor, |this, _, event, cx| {
                if let editor::Event::Blurred = event {
                    this.take_editing_state(cx);
                    this.update_entries(cx);
                    cx.notify();
                }
            })
            .detach();

            let list_state =
                ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
                    let theme = theme::current(cx).clone();
                    let is_selected = this.selection == Some(ix);
                    let current_project_id = this.project.read(cx).remote_id();

                    match &this.entries[ix] {
                        ListEntry::Header(section, depth) => {
                            let is_collapsed = this.collapsed_sections.contains(section);
                            this.render_header(
                                *section,
                                &theme,
                                *depth,
                                is_selected,
                                is_collapsed,
                                cx,
                            )
                        }
                        ListEntry::CallParticipant { user, is_pending } => {
                            Self::render_call_participant(
                                user,
                                *is_pending,
                                is_selected,
                                &theme.collab_panel,
                            )
                        }
                        ListEntry::ParticipantProject {
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
                            &theme.collab_panel,
                            cx,
                        ),
                        ListEntry::ParticipantScreen { peer_id, is_last } => {
                            Self::render_participant_screen(
                                *peer_id,
                                *is_last,
                                is_selected,
                                &theme.collab_panel,
                                cx,
                            )
                        }
                        ListEntry::Channel(channel) => {
                            this.render_channel(&*channel, &theme.collab_panel, is_selected, cx)
                        }
                        ListEntry::ChannelInvite(channel) => Self::render_channel_invite(
                            channel.clone(),
                            this.channel_store.clone(),
                            &theme.collab_panel,
                            is_selected,
                            cx,
                        ),
                        ListEntry::IncomingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            &theme.collab_panel,
                            true,
                            is_selected,
                            cx,
                        ),
                        ListEntry::OutgoingRequest(user) => Self::render_contact_request(
                            user.clone(),
                            this.user_store.clone(),
                            &theme.collab_panel,
                            false,
                            is_selected,
                            cx,
                        ),
                        ListEntry::Contact { contact, calling } => Self::render_contact(
                            contact,
                            *calling,
                            &this.project,
                            &theme.collab_panel,
                            is_selected,
                            cx,
                        ),
                        ListEntry::ChannelEditor { depth } => {
                            this.render_channel_editor(&theme.collab_panel, *depth, cx)
                        }
                    }
                });

            let mut this = Self {
                width: None,
                has_focus: false,
                fs: workspace.app_state().fs.clone(),
                pending_serialization: Task::ready(None),
                context_menu: cx.add_view(|cx| ContextMenu::new(view_id, cx)),
                channel_name_editor,
                filter_editor,
                entries: Vec::default(),
                channel_editing_state: None,
                selection: None,
                user_store: workspace.user_store().clone(),
                channel_store: workspace.app_state().channel_store.clone(),
                project: workspace.project().clone(),
                subscriptions: Vec::default(),
                match_candidates: Vec::default(),
                collapsed_sections: Vec::default(),
                workspace: workspace.weak_handle(),
                list_state,
            };
            this.update_entries(cx);

            // Update the dock position when the setting changes.
            let mut old_dock_position = this.position(cx);
            this.subscriptions
                .push(
                    cx.observe_global::<SettingsStore, _>(move |this: &mut CollabPanel, cx| {
                        let new_dock_position = this.position(cx);
                        if new_dock_position != old_dock_position {
                            old_dock_position = new_dock_position;
                            cx.emit(Event::DockPositionChanged);
                        }
                    }),
                );

            let active_call = ActiveCall::global(cx);
            this.subscriptions
                .push(cx.observe(&this.user_store, |this, _, cx| this.update_entries(cx)));
            this.subscriptions
                .push(cx.observe(&this.channel_store, |this, _, cx| this.update_entries(cx)));
            this.subscriptions
                .push(cx.observe(&active_call, |this, _, cx| this.update_entries(cx)));

            this
        })
    }

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(CHANNELS_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedChannelsPanel>(&panel)?)
            } else {
                None
            };

            workspace.update(&mut cx, |workspace, cx| {
                let panel = CollabPanel::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        CHANNELS_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedChannelsPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.read(cx);
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
                    participant_entries.push(ListEntry::CallParticipant {
                        user,
                        is_pending: false,
                    });
                    let mut projects = room.local_participant().projects.iter().peekable();
                    while let Some(project) = projects.next() {
                        participant_entries.push(ListEntry::ParticipantProject {
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
                participant_entries.push(ListEntry::CallParticipant {
                    user: participant.user.clone(),
                    is_pending: false,
                });
                let mut projects = participant.projects.iter().peekable();
                while let Some(project) = projects.next() {
                    participant_entries.push(ListEntry::ParticipantProject {
                        project_id: project.id,
                        worktree_root_names: project.worktree_root_names.clone(),
                        host_user_id: participant.user.id,
                        is_last: projects.peek().is_none() && participant.video_tracks.is_empty(),
                    });
                }
                if !participant.video_tracks.is_empty() {
                    participant_entries.push(ListEntry::ParticipantScreen {
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
            participant_entries.extend(matches.iter().map(|mat| ListEntry::CallParticipant {
                user: room.pending_participants()[mat.candidate_id].clone(),
                is_pending: true,
            }));

            if !participant_entries.is_empty() {
                self.entries.push(ListEntry::Header(Section::ActiveCall, 0));
                if !self.collapsed_sections.contains(&Section::ActiveCall) {
                    self.entries.extend(participant_entries);
                }
            }
        }

        self.entries.push(ListEntry::Header(Section::Channels, 0));

        let channels = channel_store.channels();
        if !(channels.is_empty() && self.channel_editing_state.is_none()) {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    channels
                        .iter()
                        .enumerate()
                        .map(|(ix, channel)| StringMatchCandidate {
                            id: ix,
                            string: channel.name.clone(),
                            char_bag: channel.name.chars().collect(),
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
            if let Some(state) = &self.channel_editing_state {
                if state.parent_id.is_none() {
                    self.entries.push(ListEntry::ChannelEditor { depth: 0 });
                }
            }
            for mat in matches {
                let channel = &channels[mat.candidate_id];
                self.entries.push(ListEntry::Channel(channel.clone()));
                if let Some(state) = &self.channel_editing_state {
                    if state.parent_id == Some(channel.id) {
                        self.entries.push(ListEntry::ChannelEditor {
                            depth: channel.depth + 1,
                        });
                    }
                }
            }
        }

        self.entries.push(ListEntry::Header(Section::Contacts, 0));

        let mut request_entries = Vec::new();
        let channel_invites = channel_store.channel_invitations();
        if !channel_invites.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(channel_invites.iter().enumerate().map(|(ix, channel)| {
                    StringMatchCandidate {
                        id: ix,
                        string: channel.name.clone(),
                        char_bag: channel.name.chars().collect(),
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
            request_entries.extend(
                matches
                    .iter()
                    .map(|mat| ListEntry::ChannelInvite(channel_invites[mat.candidate_id].clone())),
            );
        }

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
                    .map(|mat| ListEntry::IncomingRequest(incoming[mat.candidate_id].clone())),
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
                    .map(|mat| ListEntry::OutgoingRequest(outgoing[mat.candidate_id].clone())),
            );
        }

        if !request_entries.is_empty() {
            self.entries.push(ListEntry::Header(Section::Requests, 1));
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
                    self.entries.push(ListEntry::Header(section, 1));
                    if !self.collapsed_sections.contains(&section) {
                        let active_call = &ActiveCall::global(cx).read(cx);
                        for mat in matches {
                            let contact = &contacts[mat.candidate_id];
                            self.entries.push(ListEntry::Contact {
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
        theme: &theme::CollabPanel,
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
                    .in_state(is_selected)
                    .style_for(&mut Default::default()),
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
        theme: &theme::CollabPanel,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum JoinProject {}

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.project_row.inactive_state().default;
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
            let tree_branch = *tree_branch.in_state(is_selected).style_for(mouse_state);
            let row = theme
                .project_row
                .in_state(is_selected)
                .style_for(mouse_state);

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
        theme: &theme::CollabPanel,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum OpenSharedScreen {}

        let font_cache = cx.font_cache();
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);
        let row = &theme.project_row.inactive_state().default;
        let tree_branch = theme.tree_branch;
        let line_height = row.name.text.line_height(font_cache);
        let cap_height = row.name.text.cap_height(font_cache);
        let baseline_offset =
            row.name.text.baseline_offset(font_cache) + (theme.row_height - line_height) / 2.;

        MouseEventHandler::<OpenSharedScreen, Self>::new(
            peer_id.as_u64() as usize,
            cx,
            |mouse_state, _| {
                let tree_branch = *tree_branch.in_state(is_selected).style_for(mouse_state);
                let row = theme
                    .project_row
                    .in_state(is_selected)
                    .style_for(mouse_state);

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

    fn take_editing_state(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<(ChannelEditingState, String)> {
        let result = self
            .channel_editing_state
            .take()
            .map(|state| (state, self.channel_name_editor.read(cx).text(cx)));

        self.channel_name_editor
            .update(cx, |editor, cx| editor.set_text("", cx));

        result
    }

    fn render_header(
        &self,
        section: Section,
        theme: &theme::Theme,
        depth: usize,
        is_selected: bool,
        is_collapsed: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum Header {}
        enum LeaveCallContactList {}
        enum AddChannel {}

        let tooltip_style = &theme.tooltip;
        let text = match section {
            Section::ActiveCall => "Current Call",
            Section::Requests => "Requests",
            Section::Contacts => "Contacts",
            Section::Channels => "Channels",
            Section::Online => "Online",
            Section::Offline => "Offline",
        };

        enum AddContact {}
        let button = match section {
            Section::ActiveCall => Some(
                MouseEventHandler::<AddContact, Self>::new(0, cx, |_, _| {
                    render_icon_button(
                        &theme.collab_panel.leave_call_button,
                        "icons/radix/exit.svg",
                    )
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, _, cx| {
                    ActiveCall::global(cx)
                        .update(cx, |call, cx| call.hang_up(cx))
                        .detach_and_log_err(cx);
                })
                .with_tooltip::<AddContact>(
                    0,
                    "Leave call".into(),
                    None,
                    tooltip_style.clone(),
                    cx,
                ),
            ),
            Section::Contacts => Some(
                MouseEventHandler::<LeaveCallContactList, Self>::new(0, cx, |_, _| {
                    render_icon_button(
                        &theme.collab_panel.add_contact_button,
                        "icons/user_plus_16.svg",
                    )
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, this, cx| {
                    this.toggle_contact_finder(cx);
                })
                .with_tooltip::<LeaveCallContactList>(
                    0,
                    "Search for new contact".into(),
                    None,
                    tooltip_style.clone(),
                    cx,
                ),
            ),
            Section::Channels => Some(
                MouseEventHandler::<AddChannel, Self>::new(0, cx, |_, _| {
                    render_icon_button(&theme.collab_panel.add_contact_button, "icons/plus_16.svg")
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, this, cx| this.new_root_channel(cx))
                .with_tooltip::<AddChannel>(
                    0,
                    "Add or join a channel".into(),
                    None,
                    tooltip_style.clone(),
                    cx,
                ),
            ),
            _ => None,
        };

        let can_collapse = depth > 0;
        let icon_size = (&theme.collab_panel).section_icon_size;
        MouseEventHandler::<Header, Self>::new(section as usize, cx, |state, _| {
            let header_style = if can_collapse {
                theme
                    .collab_panel
                    .subheader_row
                    .in_state(is_selected)
                    .style_for(state)
            } else {
                &theme.collab_panel.header_row
            };

            Flex::row()
                .with_children(if can_collapse {
                    Some(
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
                        .with_width(icon_size)
                        .contained()
                        .with_margin_right(
                            theme.collab_panel.contact_username.container.margin.left,
                        ),
                    )
                } else {
                    None
                })
                .with_child(
                    Label::new(text, header_style.text.clone())
                        .aligned()
                        .left()
                        .flex(1., true),
                )
                .with_children(button.map(|button| button.aligned().right()))
                .constrained()
                .with_height(theme.collab_panel.row_height)
                .contained()
                .with_style(header_style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            if can_collapse {
                this.toggle_expanded(section, cx);
            }
        })
        .into_any()
    }

    fn render_contact(
        contact: &Contact,
        calling: bool,
        project: &ModelHandle<Project>,
        theme: &theme::CollabPanel,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let online = contact.online;
        let busy = contact.busy || calling;
        let user_id = contact.user.id;
        let github_login = contact.user.github_login.clone();
        let initial_project = project.clone();
        let mut event_handler =
            MouseEventHandler::<Contact, Self>::new(contact.user.id as usize, cx, |state, cx| {
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
                                let button_style = theme.contact_button.style_for(mouse_state);
                                render_icon_button(button_style, "icons/x_mark_8.svg")
                                    .aligned()
                                    .flex_float()
                            },
                        )
                        .with_padding(Padding::uniform(2.))
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, move |_, this, cx| {
                            this.remove_contact(user_id, &github_login, cx);
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
                    .with_style(*theme.contact_row.in_state(is_selected).style_for(state))
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

    fn render_channel_editor(
        &self,
        theme: &theme::CollabPanel,
        depth: usize,
        cx: &AppContext,
    ) -> AnyElement<Self> {
        ChildView::new(&self.channel_name_editor, cx).into_any()
    }

    fn render_channel(
        &self,
        channel: &Channel,
        theme: &theme::CollabPanel,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let channel_id = channel.id;
        MouseEventHandler::<Channel, Self>::new(channel.id as usize, cx, |state, cx| {
            Flex::row()
                .with_child({ Svg::new("icons/file_icons/hash.svg").aligned().left() })
                .with_child(
                    Label::new(channel.name.clone(), theme.contact_username.text.clone())
                        .contained()
                        .with_style(theme.contact_username.container)
                        .aligned()
                        .left()
                        .flex(1., true),
                )
                .with_child(
                    FacePile::new(theme.face_overlap).with_children(
                        self.channel_store
                            .read(cx)
                            .channel_participants(channel_id)
                            .iter()
                            .filter_map(|user| {
                                Some(
                                    Image::from_data(user.avatar.clone()?)
                                        .with_style(theme.contact_avatar),
                                )
                            }),
                    ),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(*theme.contact_row.in_state(is_selected).style_for(state))
                .with_margin_left(20. * channel.depth as f32)
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.join_channel(channel_id, cx);
        })
        .on_click(MouseButton::Right, move |e, this, cx| {
            this.deploy_channel_context_menu(e.position, channel_id, cx);
        })
        .into_any()
    }

    fn render_channel_invite(
        channel: Arc<Channel>,
        channel_store: ModelHandle<ChannelStore>,
        theme: &theme::CollabPanel,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum Decline {}
        enum Accept {}

        let channel_id = channel.id;
        let is_invite_pending = channel_store.read(cx).is_channel_invite_pending(&channel);
        let button_spacing = theme.contact_button_spacing;

        Flex::row()
            .with_child({
                Svg::new("icons/file_icons/hash.svg")
                    // .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
            })
            .with_child(
                Label::new(channel.name.clone(), theme.contact_username.text.clone())
                    .contained()
                    .with_style(theme.contact_username.container)
                    .aligned()
                    .left()
                    .flex(1., true),
            )
            .with_child(
                MouseEventHandler::<Decline, Self>::new(
                    channel.id as usize,
                    cx,
                    |mouse_state, _| {
                        let button_style = if is_invite_pending {
                            &theme.disabled_button
                        } else {
                            theme.contact_button.style_for(mouse_state)
                        };
                        render_icon_button(button_style, "icons/x_mark_8.svg").aligned()
                    },
                )
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_channel_invite(channel_id, false, cx);
                })
                .contained()
                .with_margin_right(button_spacing),
            )
            .with_child(
                MouseEventHandler::<Accept, Self>::new(
                    channel.id as usize,
                    cx,
                    |mouse_state, _| {
                        let button_style = if is_invite_pending {
                            &theme.disabled_button
                        } else {
                            theme.contact_button.style_for(mouse_state)
                        };
                        render_icon_button(button_style, "icons/check_8.svg")
                            .aligned()
                            .flex_float()
                    },
                )
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_channel_invite(channel_id, true, cx);
                }),
            )
            .constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(
                *theme
                    .contact_row
                    .in_state(is_selected)
                    .style_for(&mut Default::default()),
            )
            .into_any()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::CollabPanel,
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
                        theme.contact_button.style_for(mouse_state)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg").aligned()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_contact_request(user_id, false, cx);
                })
                .contained()
                .with_margin_right(button_spacing),
            );

            row.add_child(
                MouseEventHandler::<Accept, Self>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state)
                    };
                    render_icon_button(button_style, "icons/check_8.svg")
                        .aligned()
                        .flex_float()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.respond_to_contact_request(user_id, true, cx);
                }),
            );
        } else {
            row.add_child(
                MouseEventHandler::<Cancel, Self>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg")
                        .aligned()
                        .flex_float()
                })
                .with_padding(Padding::uniform(2.))
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.remove_contact(user_id, &github_login, cx);
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
                    .in_state(is_selected)
                    .style_for(&mut Default::default()),
            )
            .into_any()
    }

    fn deploy_channel_context_menu(
        &mut self,
        position: Vector2F,
        channel_id: u64,
        cx: &mut ViewContext<Self>,
    ) {
        self.context_menu.update(cx, |context_menu, cx| {
            context_menu.show(
                position,
                gpui::elements::AnchorCorner::BottomLeft,
                vec![
                    ContextMenuItem::action("New Channel", NewChannel { channel_id }),
                    ContextMenuItem::action("Remove Channel", RemoveChannel { channel_id }),
                ],
                cx,
            );
        });
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        let mut did_clear = self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx) > 0 {
                editor.set_text("", cx);
                true
            } else {
                false
            }
        });

        did_clear |= self.take_editing_state(cx).is_some();

        if !did_clear {
            cx.emit(Event::Dismissed);
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let mut ix = self.selection.map_or(0, |ix| ix + 1);
        while let Some(entry) = self.entries.get(ix) {
            if entry.is_selectable() {
                self.selection = Some(ix);
                break;
            }
            ix += 1;
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
        if let Some(mut ix) = self.selection.take() {
            while ix > 0 {
                ix -= 1;
                if let Some(entry) = self.entries.get(ix) {
                    if entry.is_selectable() {
                        self.selection = Some(ix);
                        break;
                    }
                }
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
                    ListEntry::Header(section, _) => {
                        self.toggle_expanded(*section, cx);
                    }
                    ListEntry::Contact { contact, calling } => {
                        if contact.online && !contact.busy && !calling {
                            self.call(contact.user.id, Some(self.project.clone()), cx);
                        }
                    }
                    ListEntry::ParticipantProject {
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
                    ListEntry::ParticipantScreen { peer_id, .. } => {
                        if let Some(workspace) = self.workspace.upgrade(cx) {
                            workspace.update(cx, |workspace, cx| {
                                workspace.open_shared_screen(*peer_id, cx)
                            });
                        }
                    }
                    _ => {}
                }
            }
        } else if let Some((editing_state, channel_name)) = self.take_editing_state(cx) {
            let create_channel = self.channel_store.update(cx, |channel_store, cx| {
                channel_store.create_channel(&channel_name, editing_state.parent_id)
            });

            cx.foreground()
                .spawn(async move {
                    create_channel.await.ok();
                })
                .detach();
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

    fn toggle_contact_finder(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(cx, |_, cx| {
                    cx.add_view(|cx| {
                        let finder = build_contact_finder(self.user_store.clone(), cx);
                        finder.set_query(self.filter_editor.read(cx).text(cx), cx);
                        finder
                    })
                });
            });
        }
    }

    fn new_root_channel(&mut self, cx: &mut ViewContext<Self>) {
        if self.channel_editing_state.is_none() {
            self.channel_editing_state = Some(ChannelEditingState { parent_id: None });
            self.update_entries(cx);
        }

        cx.focus(self.channel_name_editor.as_any());
        cx.notify();
    }

    fn new_subchannel(&mut self, action: &NewChannel, cx: &mut ViewContext<Self>) {
        if self.channel_editing_state.is_none() {
            self.channel_editing_state = Some(ChannelEditingState {
                parent_id: Some(action.channel_id),
            });
            self.update_entries(cx);
        }

        cx.focus(self.channel_name_editor.as_any());
        cx.notify();
    }

    fn remove_channel(&mut self, action: &RemoveChannel, cx: &mut ViewContext<Self>) {
        let channel_id = action.channel_id;
        let channel_store = self.channel_store.clone();
        if let Some(channel) = channel_store.read(cx).channel_for_id(channel_id) {
            let prompt_message = format!(
                "Are you sure you want to remove the channel \"{}\"?",
                channel.name
            );
            let mut answer =
                cx.prompt(PromptLevel::Warning, &prompt_message, &["Remove", "Cancel"]);
            let window_id = cx.window_id();
            cx.spawn(|_, mut cx| async move {
                if answer.next().await == Some(0) {
                    if let Err(e) = channel_store
                        .update(&mut cx, |channels, cx| channels.remove_channel(channel_id))
                        .await
                    {
                        cx.prompt(
                            window_id,
                            PromptLevel::Info,
                            &format!("Failed to remove channel: {}", e),
                            &["Ok"],
                        );
                    }
                }
            })
            .detach();
        }
    }

    fn remove_contact(&mut self, user_id: u64, github_login: &str, cx: &mut ViewContext<Self>) {
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
        user_id: u64,
        accept: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.user_store
            .update(cx, |store, cx| {
                store.respond_to_contact_request(user_id, accept, cx)
            })
            .detach();
    }

    fn respond_to_channel_invite(
        &mut self,
        channel_id: u64,
        accept: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let respond = self.channel_store.update(cx, |store, _| {
            store.respond_to_channel_invite(channel_id, accept)
        });
        cx.foreground().spawn(respond).detach();
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

    fn join_channel(&self, channel: u64, cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.join_channel(channel, cx))
            .detach_and_log_err(cx);
    }
}

impl View for CollabPanel {
    fn ui_name() -> &'static str {
        "CollabPanel"
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.has_focus {
            self.has_focus = true;
            cx.emit(Event::Focus);
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        let theme = &theme::current(cx).collab_panel;

        enum PanelFocus {}
        MouseEventHandler::<PanelFocus, _>::new(0, cx, |_, cx| {
            Stack::new()
                .with_child(
                    Flex::column()
                        .with_child(
                            Flex::row()
                                .with_child(
                                    ChildView::new(&self.filter_editor, cx)
                                        .contained()
                                        .with_style(theme.user_query_editor.container)
                                        .flex(1.0, true),
                                )
                                .constrained()
                                .with_width(self.size(cx)),
                        )
                        .with_child(
                            List::new(self.list_state.clone())
                                .constrained()
                                .with_width(self.size(cx))
                                .flex(1., true)
                                .into_any(),
                        )
                        .contained()
                        .with_style(theme.container)
                        .constrained()
                        .with_width(self.size(cx))
                        .into_any(),
                )
                .with_child(ChildView::new(&self.context_menu, cx))
                .into_any()
        })
        .on_click(MouseButton::Left, |_, _, cx| cx.focus_self())
        .into_any_named("channels panel")
    }
}

impl Panel for CollabPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        match settings::get::<ChannelsPanelSettings>(cx).dock {
            ChannelsPanelDockPosition::Left => DockPosition::Left,
            ChannelsPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ChannelsPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => ChannelsPanelDockPosition::Left,
                    DockPosition::Right => ChannelsPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> f32 {
        self.width
            .unwrap_or_else(|| settings::get::<ChannelsPanelSettings>(cx).default_width)
    }

    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        self.width = Some(size);
        self.serialize(cx);
        cx.notify();
    }

    fn icon_path(&self) -> &'static str {
        "icons/radix/person.svg"
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn gpui::Action>>) {
        ("Channels Panel".to_string(), Some(Box::new(ToggleFocus)))
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn has_focus(&self, _cx: &gpui::WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}

impl ListEntry {
    fn is_selectable(&self) -> bool {
        if let ListEntry::Header(_, 0) = self {
            false
        } else {
            true
        }
    }
}

impl PartialEq for ListEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ListEntry::Header(section_1, depth_1) => {
                if let ListEntry::Header(section_2, depth_2) = other {
                    return section_1 == section_2 && depth_1 == depth_2;
                }
            }
            ListEntry::CallParticipant { user: user_1, .. } => {
                if let ListEntry::CallParticipant { user: user_2, .. } = other {
                    return user_1.id == user_2.id;
                }
            }
            ListEntry::ParticipantProject {
                project_id: project_id_1,
                ..
            } => {
                if let ListEntry::ParticipantProject {
                    project_id: project_id_2,
                    ..
                } = other
                {
                    return project_id_1 == project_id_2;
                }
            }
            ListEntry::ParticipantScreen {
                peer_id: peer_id_1, ..
            } => {
                if let ListEntry::ParticipantScreen {
                    peer_id: peer_id_2, ..
                } = other
                {
                    return peer_id_1 == peer_id_2;
                }
            }
            ListEntry::Channel(channel_1) => {
                if let ListEntry::Channel(channel_2) = other {
                    return channel_1.id == channel_2.id;
                }
            }
            ListEntry::ChannelInvite(channel_1) => {
                if let ListEntry::ChannelInvite(channel_2) = other {
                    return channel_1.id == channel_2.id;
                }
            }
            ListEntry::IncomingRequest(user_1) => {
                if let ListEntry::IncomingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ListEntry::OutgoingRequest(user_1) => {
                if let ListEntry::OutgoingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ListEntry::Contact {
                contact: contact_1, ..
            } => {
                if let ListEntry::Contact {
                    contact: contact_2, ..
                } = other
                {
                    return contact_1.user.id == contact_2.user.id;
                }
            }
            ListEntry::ChannelEditor { depth } => {
                if let ListEntry::ChannelEditor { depth: other_depth } = other {
                    return depth == other_depth;
                }
            }
        }
        false
    }
}

fn render_icon_button(style: &IconButton, svg_path: &'static str) -> impl Element<CollabPanel> {
    Svg::new(svg_path)
        .with_color(style.color)
        .constrained()
        .with_width(style.icon_width)
        .aligned()
        .constrained()
        .with_width(style.button_width)
        .with_height(style.button_width)
        .contained()
        .with_style(style.container)
}
