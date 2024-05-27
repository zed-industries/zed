mod channel_modal;
mod contact_finder;

use self::channel_modal::ChannelModal;
use crate::{
    channel_view::ChannelView, chat_panel::ChatPanel, face_pile::FacePile,
    CollaborationPanelSettings,
};
use call::ActiveCall;
use channel::{Channel, ChannelEvent, ChannelStore};
use client::{ChannelId, Client, Contact, ProjectId, User, UserStore};
use contact_finder::ContactFinder;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorElement, EditorStyle};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions, anchored, canvas, deferred, div, fill, list, point, prelude::*, px, AnyElement,
    AppContext, AsyncWindowContext, Bounds, ClickEvent, ClipboardItem, DismissEvent, Div,
    EventEmitter, FocusHandle, FocusableView, FontStyle, InteractiveElement, IntoElement,
    ListOffset, ListState, Model, MouseDownEvent, ParentElement, Pixels, Point, PromptLevel,
    Render, SharedString, Styled, Subscription, Task, TextStyle, View, ViewContext, VisualContext,
    WeakView, WhiteSpace,
};
use menu::{Cancel, Confirm, SecondaryConfirm, SelectNext, SelectPrev};
use project::{Fs, Project};
use rpc::{
    proto::{self, ChannelVisibility, PeerId},
    ErrorCode, ErrorExt,
};
use serde_derive::{Deserialize, Serialize};
use settings::Settings;
use smallvec::SmallVec;
use std::{mem, sync::Arc};
use theme::{ActiveTheme, ThemeSettings};
use ui::{
    prelude::*, tooltip_container, Avatar, AvatarAvailabilityIndicator, Button, Color, ContextMenu,
    Icon, IconButton, IconName, IconSize, Indicator, Label, ListHeader, ListItem, Tooltip,
};
use util::{maybe, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotifyResultExt, NotifyTaskExt},
    OpenChannelNotes, Workspace,
};

actions!(
    collab_panel,
    [
        ToggleFocus,
        Remove,
        Secondary,
        CollapseSelectedChannel,
        ExpandSelectedChannel,
        StartMoveChannel,
        MoveSelected,
        InsertSpace,
    ]
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ChannelMoveClipboard {
    channel_id: ChannelId,
}

const COLLABORATION_PANEL_KEY: &str = "CollaborationPanel";

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<CollabPanel>(cx);
        });
        workspace.register_action(|_, _: &OpenChannelNotes, cx| {
            let channel_id = ActiveCall::global(cx)
                .read(cx)
                .room()
                .and_then(|room| room.read(cx).channel_id());

            if let Some(channel_id) = channel_id {
                let workspace = cx.view().clone();
                cx.window_context().defer(move |cx| {
                    ChannelView::open(channel_id, None, workspace, cx).detach_and_log_err(cx)
                });
            }
        });
    })
    .detach();
}

#[derive(Debug)]
pub enum ChannelEditingState {
    Create {
        location: Option<ChannelId>,
        pending_name: Option<String>,
    },
    Rename {
        location: ChannelId,
        pending_name: Option<String>,
    },
}

impl ChannelEditingState {
    fn pending_name(&self) -> Option<String> {
        match self {
            ChannelEditingState::Create { pending_name, .. } => pending_name.clone(),
            ChannelEditingState::Rename { pending_name, .. } => pending_name.clone(),
        }
    }
}

pub struct CollabPanel {
    width: Option<Pixels>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    channel_clipboard: Option<ChannelMoveClipboard>,
    pending_serialization: Task<Option<()>>,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    list_state: ListState,
    filter_editor: View<Editor>,
    channel_name_editor: View<Editor>,
    channel_editing_state: Option<ChannelEditingState>,
    entries: Vec<ListEntry>,
    selection: Option<usize>,
    channel_store: Model<ChannelStore>,
    user_store: Model<UserStore>,
    client: Arc<Client>,
    project: Model<Project>,
    match_candidates: Vec<StringMatchCandidate>,
    subscriptions: Vec<Subscription>,
    collapsed_sections: Vec<Section>,
    collapsed_channels: Vec<ChannelId>,
    workspace: WeakView<Workspace>,
}

#[derive(Serialize, Deserialize)]
struct SerializedCollabPanel {
    width: Option<Pixels>,
    collapsed_channels: Option<Vec<u64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Section {
    ActiveCall,
    Channels,
    ChannelInvites,
    ContactRequests,
    Contacts,
    Online,
    Offline,
}

#[derive(Clone, Debug)]
enum ListEntry {
    Header(Section),
    CallParticipant {
        user: Arc<User>,
        peer_id: Option<PeerId>,
        is_pending: bool,
        role: proto::ChannelRole,
    },
    ParticipantProject {
        project_id: u64,
        worktree_root_names: Vec<String>,
        host_user_id: u64,
        is_last: bool,
    },
    ParticipantScreen {
        peer_id: Option<PeerId>,
        is_last: bool,
    },
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    ChannelInvite(Arc<Channel>),
    Channel {
        channel: Arc<Channel>,
        depth: usize,
        has_children: bool,
    },
    ChannelNotes {
        channel_id: ChannelId,
    },
    ChannelChat {
        channel_id: ChannelId,
    },
    ChannelEditor {
        depth: usize,
    },
    HostedProject {
        id: ProjectId,
        name: SharedString,
    },
    Contact {
        contact: Arc<Contact>,
        calling: bool,
    },
    ContactPlaceholder,
}

impl CollabPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });

            cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
                if let editor::EditorEvent::BufferEdited = event {
                    let query = this.filter_editor.read(cx).text(cx);
                    if !query.is_empty() {
                        this.selection.take();
                    }
                    this.update_entries(true, cx);
                    if !query.is_empty() {
                        this.selection = this
                            .entries
                            .iter()
                            .position(|entry| !matches!(entry, ListEntry::Header(_)));
                    }
                }
            })
            .detach();

            let channel_name_editor = cx.new_view(|cx| Editor::single_line(cx));

            cx.subscribe(&channel_name_editor, |this: &mut Self, _, event, cx| {
                if let editor::EditorEvent::Blurred = event {
                    if let Some(state) = &this.channel_editing_state {
                        if state.pending_name().is_some() {
                            return;
                        }
                    }
                    this.take_editing_state(cx);
                    this.update_entries(false, cx);
                    cx.notify();
                }
            })
            .detach();

            let view = cx.view().downgrade();
            let list_state =
                ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| view.render_list_entry(ix, cx))
                    } else {
                        div().into_any()
                    }
                });

            let mut this = Self {
                width: None,
                focus_handle: cx.focus_handle(),
                channel_clipboard: None,
                fs: workspace.app_state().fs.clone(),
                pending_serialization: Task::ready(None),
                context_menu: None,
                list_state,
                channel_name_editor,
                filter_editor,
                entries: Vec::default(),
                channel_editing_state: None,
                selection: None,
                channel_store: ChannelStore::global(cx),
                user_store: workspace.user_store().clone(),
                project: workspace.project().clone(),
                subscriptions: Vec::default(),
                match_candidates: Vec::default(),
                collapsed_sections: vec![Section::Offline],
                collapsed_channels: Vec::default(),
                workspace: workspace.weak_handle(),
                client: workspace.app_state().client.clone(),
            };

            this.update_entries(false, cx);

            let active_call = ActiveCall::global(cx);
            this.subscriptions
                .push(cx.observe(&this.user_store, |this, _, cx| {
                    this.update_entries(true, cx)
                }));
            this.subscriptions
                .push(cx.observe(&this.channel_store, move |this, _, cx| {
                    this.update_entries(true, cx)
                }));
            this.subscriptions
                .push(cx.observe(&active_call, |this, _, cx| this.update_entries(true, cx)));
            this.subscriptions.push(cx.subscribe(
                &this.channel_store,
                |this, _channel_store, e, cx| match e {
                    ChannelEvent::ChannelCreated(channel_id)
                    | ChannelEvent::ChannelRenamed(channel_id) => {
                        if this.take_editing_state(cx) {
                            this.update_entries(false, cx);
                            this.selection = this.entries.iter().position(|entry| {
                                if let ListEntry::Channel { channel, .. } = entry {
                                    channel.id == *channel_id
                                } else {
                                    false
                                }
                            });
                        }
                    }
                },
            ));

            this
        })
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(COLLABORATION_PANEL_KEY) })
            .await
            .map_err(|_| anyhow::anyhow!("Failed to read collaboration panel from key value store"))
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedCollabPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = CollabPanel::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|w| w.round());
                    panel.collapsed_channels = serialized_panel
                        .collapsed_channels
                        .unwrap_or_else(|| Vec::new())
                        .iter()
                        .map(|cid| ChannelId(*cid))
                        .collect();
                    cx.notify();
                });
            }
            panel
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        let collapsed_channels = self.collapsed_channels.clone();
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        COLLABORATION_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedCollabPanel {
                            width,
                            collapsed_channels: Some(
                                collapsed_channels.iter().map(|cid| cid.0).collect(),
                            ),
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn scroll_to_item(&mut self, ix: usize) {
        self.list_state.scroll_to_reveal_item(ix)
    }

    fn update_entries(&mut self, select_same_item: bool, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.read(cx);
        let user_store = self.user_store.read(cx);
        let query = self.filter_editor.read(cx).text(cx);
        let executor = cx.background_executor().clone();

        let prev_selected_entry = self.selection.and_then(|ix| self.entries.get(ix).cloned());
        let old_entries = mem::take(&mut self.entries);
        let mut scroll_to_top = false;

        if let Some(room) = ActiveCall::global(cx).read(cx).room() {
            self.entries.push(ListEntry::Header(Section::ActiveCall));
            if !old_entries
                .iter()
                .any(|entry| matches!(entry, ListEntry::Header(Section::ActiveCall)))
            {
                scroll_to_top = true;
            }

            if !self.collapsed_sections.contains(&Section::ActiveCall) {
                let room = room.read(cx);

                if query.is_empty() {
                    if let Some(channel_id) = room.channel_id() {
                        self.entries.push(ListEntry::ChannelNotes { channel_id });
                        self.entries.push(ListEntry::ChannelChat { channel_id });
                    }
                }

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
                        self.entries.push(ListEntry::CallParticipant {
                            user,
                            peer_id: None,
                            is_pending: false,
                            role: room.local_participant().role,
                        });
                        let mut projects = room.local_participant().projects.iter().peekable();
                        while let Some(project) = projects.next() {
                            self.entries.push(ListEntry::ParticipantProject {
                                project_id: project.id,
                                worktree_root_names: project.worktree_root_names.clone(),
                                host_user_id: user_id,
                                is_last: projects.peek().is_none() && !room.is_screen_sharing(),
                            });
                        }
                        if room.is_screen_sharing() {
                            self.entries.push(ListEntry::ParticipantScreen {
                                peer_id: None,
                                is_last: true,
                            });
                        }
                    }
                }

                // Populate remote participants.
                self.match_candidates.clear();
                self.match_candidates
                    .extend(room.remote_participants().values().map(|participant| {
                        StringMatchCandidate {
                            id: participant.user.id as usize,
                            string: participant.user.github_login.clone(),
                            char_bag: participant.user.github_login.chars().collect(),
                        }
                    }));
                let mut matches = executor.block(match_strings(
                    &self.match_candidates,
                    &query,
                    true,
                    usize::MAX,
                    &Default::default(),
                    executor.clone(),
                ));
                matches.sort_by(|a, b| {
                    let a_is_guest = room.role_for_user(a.candidate_id as u64)
                        == Some(proto::ChannelRole::Guest);
                    let b_is_guest = room.role_for_user(b.candidate_id as u64)
                        == Some(proto::ChannelRole::Guest);
                    a_is_guest
                        .cmp(&b_is_guest)
                        .then_with(|| a.string.cmp(&b.string))
                });
                for mat in matches {
                    let user_id = mat.candidate_id as u64;
                    let participant = &room.remote_participants()[&user_id];
                    self.entries.push(ListEntry::CallParticipant {
                        user: participant.user.clone(),
                        peer_id: Some(participant.peer_id),
                        is_pending: false,
                        role: participant.role,
                    });
                    let mut projects = participant.projects.iter().peekable();
                    while let Some(project) = projects.next() {
                        self.entries.push(ListEntry::ParticipantProject {
                            project_id: project.id,
                            worktree_root_names: project.worktree_root_names.clone(),
                            host_user_id: participant.user.id,
                            is_last: projects.peek().is_none()
                                && participant.video_tracks.is_empty(),
                        });
                    }
                    if !participant.video_tracks.is_empty() {
                        self.entries.push(ListEntry::ParticipantScreen {
                            peer_id: Some(participant.peer_id),
                            is_last: true,
                        });
                    }
                }

                // Populate pending participants.
                self.match_candidates.clear();
                self.match_candidates
                    .extend(room.pending_participants().iter().enumerate().map(
                        |(id, participant)| StringMatchCandidate {
                            id,
                            string: participant.github_login.clone(),
                            char_bag: participant.github_login.chars().collect(),
                        },
                    ));
                let matches = executor.block(match_strings(
                    &self.match_candidates,
                    &query,
                    true,
                    usize::MAX,
                    &Default::default(),
                    executor.clone(),
                ));
                self.entries
                    .extend(matches.iter().map(|mat| ListEntry::CallParticipant {
                        user: room.pending_participants()[mat.candidate_id].clone(),
                        peer_id: None,
                        is_pending: true,
                        role: proto::ChannelRole::Member,
                    }));
            }
        }

        let mut request_entries = Vec::new();

        self.entries.push(ListEntry::Header(Section::Channels));

        if channel_store.channel_count() > 0 || self.channel_editing_state.is_some() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    channel_store
                        .ordered_channels()
                        .enumerate()
                        .map(|(ix, (_, channel))| StringMatchCandidate {
                            id: ix,
                            string: channel.name.clone().into(),
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
                if matches!(state, ChannelEditingState::Create { location: None, .. }) {
                    self.entries.push(ListEntry::ChannelEditor { depth: 0 });
                }
            }
            let mut collapse_depth = None;
            for mat in matches {
                let channel = channel_store.channel_at_index(mat.candidate_id).unwrap();
                let depth = channel.parent_path.len();

                if collapse_depth.is_none() && self.is_channel_collapsed(channel.id) {
                    collapse_depth = Some(depth);
                } else if let Some(collapsed_depth) = collapse_depth {
                    if depth > collapsed_depth {
                        continue;
                    }
                    if self.is_channel_collapsed(channel.id) {
                        collapse_depth = Some(depth);
                    } else {
                        collapse_depth = None;
                    }
                }

                let hosted_projects = channel_store.projects_for_id(channel.id);
                let has_children = channel_store
                    .channel_at_index(mat.candidate_id + 1)
                    .map_or(false, |next_channel| {
                        next_channel.parent_path.ends_with(&[channel.id])
                    });

                match &self.channel_editing_state {
                    Some(ChannelEditingState::Create {
                        location: parent_id,
                        ..
                    }) if *parent_id == Some(channel.id) => {
                        self.entries.push(ListEntry::Channel {
                            channel: channel.clone(),
                            depth,
                            has_children: false,
                        });
                        self.entries
                            .push(ListEntry::ChannelEditor { depth: depth + 1 });
                    }
                    Some(ChannelEditingState::Rename {
                        location: parent_id,
                        ..
                    }) if parent_id == &channel.id => {
                        self.entries.push(ListEntry::ChannelEditor { depth });
                    }
                    _ => {
                        self.entries.push(ListEntry::Channel {
                            channel: channel.clone(),
                            depth,
                            has_children,
                        });
                    }
                }

                for (name, id) in hosted_projects {
                    self.entries.push(ListEntry::HostedProject { id, name });
                }
            }
        }

        let channel_invites = channel_store.channel_invitations();
        if !channel_invites.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(channel_invites.iter().enumerate().map(|(ix, channel)| {
                    StringMatchCandidate {
                        id: ix,
                        string: channel.name.clone().into(),
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

            if !request_entries.is_empty() {
                self.entries
                    .push(ListEntry::Header(Section::ChannelInvites));
                if !self.collapsed_sections.contains(&Section::ChannelInvites) {
                    self.entries.append(&mut request_entries);
                }
            }
        }

        self.entries.push(ListEntry::Header(Section::Contacts));

        request_entries.clear();
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
            self.entries
                .push(ListEntry::Header(Section::ContactRequests));
            if !self.collapsed_sections.contains(&Section::ContactRequests) {
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
                    self.entries.push(ListEntry::Header(section));
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

        if incoming.is_empty() && outgoing.is_empty() && contacts.is_empty() {
            self.entries.push(ListEntry::ContactPlaceholder);
        }

        if select_same_item {
            if let Some(prev_selected_entry) = prev_selected_entry {
                self.selection.take();
                for (ix, entry) in self.entries.iter().enumerate() {
                    if *entry == prev_selected_entry {
                        self.selection = Some(ix);
                        break;
                    }
                }
            }
        } else {
            self.selection = self.selection.and_then(|prev_selection| {
                if self.entries.is_empty() {
                    None
                } else {
                    Some(prev_selection.min(self.entries.len() - 1))
                }
            });
        }

        let old_scroll_top = self.list_state.logical_scroll_top();
        self.list_state.reset(self.entries.len());

        if scroll_to_top {
            self.list_state.scroll_to(ListOffset::default());
        } else {
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
                            offset_in_item: Pixels::ZERO,
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
                            offset_in_item: Pixels::ZERO,
                        })
                    });

                self.list_state
                    .scroll_to(new_scroll_top.unwrap_or(old_scroll_top));
            }
        }

        cx.notify();
    }

    fn render_call_participant(
        &self,
        user: &Arc<User>,
        peer_id: Option<PeerId>,
        is_pending: bool,
        role: proto::ChannelRole,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> ListItem {
        let user_id = user.id;
        let is_current_user =
            self.user_store.read(cx).current_user().map(|user| user.id) == Some(user_id);
        let tooltip = format!("Follow {}", user.github_login);

        let is_call_admin = ActiveCall::global(cx).read(cx).room().is_some_and(|room| {
            room.read(cx).local_participant().role == proto::ChannelRole::Admin
        });

        ListItem::new(SharedString::from(user.github_login.clone()))
            .start_slot(Avatar::new(user.avatar_uri.clone()))
            .child(Label::new(user.github_login.clone()))
            .selected(is_selected)
            .end_slot(if is_pending {
                Label::new("Calling").color(Color::Muted).into_any_element()
            } else if is_current_user {
                IconButton::new("leave-call", IconName::Exit)
                    .style(ButtonStyle::Subtle)
                    .on_click(move |_, cx| Self::leave_call(cx))
                    .tooltip(|cx| Tooltip::text("Leave Call", cx))
                    .into_any_element()
            } else if role == proto::ChannelRole::Guest {
                Label::new("Guest").color(Color::Muted).into_any_element()
            } else if role == proto::ChannelRole::Talker {
                Label::new("Mic only")
                    .color(Color::Muted)
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .when_some(peer_id, |el, peer_id| {
                if role == proto::ChannelRole::Guest {
                    return el;
                }
                el.tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                    .on_click(cx.listener(move |this, _, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| workspace.follow(peer_id, cx))
                            .ok();
                    }))
            })
            .when(is_call_admin, |el| {
                el.on_secondary_mouse_down(cx.listener(move |this, event: &MouseDownEvent, cx| {
                    this.deploy_participant_context_menu(event.position, user_id, role, cx)
                }))
            })
    }

    fn render_participant_project(
        &self,
        project_id: u64,
        worktree_root_names: &[String],
        host_user_id: u64,
        is_last: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let project_name: SharedString = if worktree_root_names.is_empty() {
            "untitled".to_string()
        } else {
            worktree_root_names.join(", ")
        }
        .into();

        ListItem::new(project_id as usize)
            .selected(is_selected)
            .on_click(cx.listener(move |this, _, cx| {
                this.workspace
                    .update(cx, |workspace, cx| {
                        let app_state = workspace.app_state().clone();
                        workspace::join_in_room_project(project_id, host_user_id, app_state, cx)
                            .detach_and_prompt_err("Failed to join project", cx, |_, _| None);
                    })
                    .ok();
            }))
            .start_slot(
                h_flex()
                    .gap_1()
                    .child(render_tree_branch(is_last, false, cx))
                    .child(IconButton::new(0, IconName::Folder)),
            )
            .child(Label::new(project_name.clone()))
            .tooltip(move |cx| Tooltip::text(format!("Open {}", project_name), cx))
    }

    fn render_participant_screen(
        &self,
        peer_id: Option<PeerId>,
        is_last: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let id = peer_id.map_or(usize::MAX, |id| id.as_u64() as usize);

        ListItem::new(("screen", id))
            .selected(is_selected)
            .start_slot(
                h_flex()
                    .gap_1()
                    .child(render_tree_branch(is_last, false, cx))
                    .child(IconButton::new(0, IconName::Screen)),
            )
            .child(Label::new("Screen"))
            .when_some(peer_id, |this, _| {
                this.on_click(cx.listener(move |this, _, cx| {
                    this.workspace
                        .update(cx, |workspace, cx| {
                            workspace.open_shared_screen(peer_id.unwrap(), cx)
                        })
                        .ok();
                }))
                .tooltip(move |cx| Tooltip::text("Open shared screen", cx))
            })
    }

    fn take_editing_state(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(_) = self.channel_editing_state.take() {
            self.channel_name_editor.update(cx, |editor, cx| {
                editor.set_text("", cx);
            });
            true
        } else {
            false
        }
    }

    fn render_channel_notes(
        &self,
        channel_id: ChannelId,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let has_channel_buffer_changed = channel_store.has_channel_buffer_changed(channel_id);
        ListItem::new("channel-notes")
            .selected(is_selected)
            .on_click(cx.listener(move |this, _, cx| {
                this.open_channel_notes(channel_id, cx);
            }))
            .start_slot(
                h_flex()
                    .relative()
                    .gap_1()
                    .child(render_tree_branch(false, true, cx))
                    .child(IconButton::new(0, IconName::File))
                    .children(has_channel_buffer_changed.then(|| {
                        div()
                            .w_1p5()
                            .absolute()
                            .right(px(2.))
                            .top(px(2.))
                            .child(Indicator::dot().color(Color::Info))
                    })),
            )
            .child(Label::new("notes"))
            .tooltip(move |cx| Tooltip::text("Open Channel Notes", cx))
    }

    fn render_channel_chat(
        &self,
        channel_id: ChannelId,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let has_messages_notification = channel_store.has_new_messages(channel_id);
        ListItem::new("channel-chat")
            .selected(is_selected)
            .on_click(cx.listener(move |this, _, cx| {
                this.join_channel_chat(channel_id, cx);
            }))
            .start_slot(
                h_flex()
                    .relative()
                    .gap_1()
                    .child(render_tree_branch(false, false, cx))
                    .child(IconButton::new(0, IconName::MessageBubbles))
                    .children(has_messages_notification.then(|| {
                        div()
                            .w_1p5()
                            .absolute()
                            .right(px(2.))
                            .top(px(4.))
                            .child(Indicator::dot().color(Color::Info))
                    })),
            )
            .child(Label::new("chat"))
            .tooltip(move |cx| Tooltip::text("Open Chat", cx))
    }

    fn render_channel_project(
        &self,
        id: ProjectId,
        name: &SharedString,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new(ElementId::NamedInteger(
            "channel-project".into(),
            id.0 as usize,
        ))
        .indent_level(2)
        .indent_step_size(px(20.))
        .selected(is_selected)
        .on_click(cx.listener(move |this, _, cx| {
            if let Some(workspace) = this.workspace.upgrade() {
                let app_state = workspace.read(cx).app_state().clone();
                workspace::join_hosted_project(id, app_state, cx).detach_and_prompt_err(
                    "Failed to open project",
                    cx,
                    |_, _| None,
                )
            }
        }))
        .start_slot(
            h_flex()
                .relative()
                .gap_1()
                .child(IconButton::new(0, IconName::FileTree)),
        )
        .child(Label::new(name.clone()))
        .tooltip(move |cx| Tooltip::text("Open Project", cx))
    }

    fn has_subchannels(&self, ix: usize) -> bool {
        self.entries.get(ix).map_or(false, |entry| {
            if let ListEntry::Channel { has_children, .. } = entry {
                *has_children
            } else {
                false
            }
        })
    }

    fn deploy_participant_context_menu(
        &mut self,
        position: Point<Pixels>,
        user_id: u64,
        role: proto::ChannelRole,
        cx: &mut ViewContext<Self>,
    ) {
        let this = cx.view().clone();
        if !(role == proto::ChannelRole::Guest
            || role == proto::ChannelRole::Talker
            || role == proto::ChannelRole::Member)
        {
            return;
        }

        let context_menu = ContextMenu::build(cx, |mut context_menu, cx| {
            if role == proto::ChannelRole::Guest {
                context_menu = context_menu.entry(
                    "Grant Mic Access",
                    None,
                    cx.handler_for(&this, move |_, cx| {
                        ActiveCall::global(cx)
                            .update(cx, |call, cx| {
                                let Some(room) = call.room() else {
                                    return Task::ready(Ok(()));
                                };
                                room.update(cx, |room, cx| {
                                    room.set_participant_role(
                                        user_id,
                                        proto::ChannelRole::Talker,
                                        cx,
                                    )
                                })
                            })
                            .detach_and_prompt_err("Failed to grant mic access", cx, |_, _| None)
                    }),
                );
            }
            if role == proto::ChannelRole::Guest || role == proto::ChannelRole::Talker {
                context_menu = context_menu.entry(
                    "Grant Write Access",
                    None,
                    cx.handler_for(&this, move |_, cx| {
                        ActiveCall::global(cx)
                            .update(cx, |call, cx| {
                                let Some(room) = call.room() else {
                                    return Task::ready(Ok(()));
                                };
                                room.update(cx, |room, cx| {
                                    room.set_participant_role(
                                        user_id,
                                        proto::ChannelRole::Member,
                                        cx,
                                    )
                                })
                            })
                            .detach_and_prompt_err("Failed to grant write access", cx, |e, _| {
                                match e.error_code() {
                                    ErrorCode::NeedsCla => Some("This user has not yet signed the CLA at https://zed.dev/cla.".into()),
                                    _ => None,
                                }
                            })
                    }),
                );
            }
            if role == proto::ChannelRole::Member || role == proto::ChannelRole::Talker {
                let label = if role == proto::ChannelRole::Talker {
                    "Mute"
                } else {
                    "Revoke Access"
                };
                context_menu = context_menu.entry(
                    label,
                    None,
                    cx.handler_for(&this, move |_, cx| {
                        ActiveCall::global(cx)
                            .update(cx, |call, cx| {
                                let Some(room) = call.room() else {
                                    return Task::ready(Ok(()));
                                };
                                room.update(cx, |room, cx| {
                                    room.set_participant_role(
                                        user_id,
                                        proto::ChannelRole::Guest,
                                        cx,
                                    )
                                })
                            })
                            .detach_and_prompt_err("Failed to revoke access", cx, |_, _| None)
                    }),
                );
            }

            context_menu
        });

        cx.focus_view(&context_menu);
        let subscription =
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(cx)
                }) {
                    cx.focus_self();
                }
                this.context_menu.take();
                cx.notify();
            });
        self.context_menu = Some((context_menu, position, subscription));
    }

    fn deploy_channel_context_menu(
        &mut self,
        position: Point<Pixels>,
        channel_id: ChannelId,
        ix: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let clipboard_channel_name = self.channel_clipboard.as_ref().and_then(|clipboard| {
            self.channel_store
                .read(cx)
                .channel_for_id(clipboard.channel_id)
                .map(|channel| channel.name.clone())
        });
        let this = cx.view().clone();

        let context_menu = ContextMenu::build(cx, |mut context_menu, cx| {
            if self.has_subchannels(ix) {
                let expand_action_name = if self.is_channel_collapsed(channel_id) {
                    "Expand Subchannels"
                } else {
                    "Collapse Subchannels"
                };
                context_menu = context_menu.entry(
                    expand_action_name,
                    None,
                    cx.handler_for(&this, move |this, cx| {
                        this.toggle_channel_collapsed(channel_id, cx)
                    }),
                );
            }

            context_menu = context_menu
                .entry(
                    "Open Notes",
                    None,
                    cx.handler_for(&this, move |this, cx| {
                        this.open_channel_notes(channel_id, cx)
                    }),
                )
                .entry(
                    "Open Chat",
                    None,
                    cx.handler_for(&this, move |this, cx| {
                        this.join_channel_chat(channel_id, cx)
                    }),
                )
                .entry(
                    "Copy Channel Link",
                    None,
                    cx.handler_for(&this, move |this, cx| {
                        this.copy_channel_link(channel_id, cx)
                    }),
                );

            let mut has_destructive_actions = false;
            if self.channel_store.read(cx).is_channel_admin(channel_id) {
                has_destructive_actions = true;
                context_menu = context_menu
                    .separator()
                    .entry(
                        "New Subchannel",
                        None,
                        cx.handler_for(&this, move |this, cx| this.new_subchannel(channel_id, cx)),
                    )
                    .entry(
                        "Rename",
                        Some(Box::new(SecondaryConfirm)),
                        cx.handler_for(&this, move |this, cx| this.rename_channel(channel_id, cx)),
                    );

                if let Some(channel_name) = clipboard_channel_name {
                    context_menu = context_menu.separator().entry(
                        format!("Move '#{}' here", channel_name),
                        None,
                        cx.handler_for(&this, move |this, cx| {
                            this.move_channel_on_clipboard(channel_id, cx)
                        }),
                    );
                }

                if self.channel_store.read(cx).is_root_channel(channel_id) {
                    context_menu = context_menu.separator().entry(
                        "Manage Members",
                        None,
                        cx.handler_for(&this, move |this, cx| this.manage_members(channel_id, cx)),
                    )
                } else {
                    context_menu = context_menu.entry(
                        "Move this channel",
                        None,
                        cx.handler_for(&this, move |this, cx| {
                            this.start_move_channel(channel_id, cx)
                        }),
                    );
                    if self.channel_store.read(cx).is_public_channel(channel_id) {
                        context_menu = context_menu.separator().entry(
                            "Make Channel Private",
                            None,
                            cx.handler_for(&this, move |this, cx| {
                                this.set_channel_visibility(
                                    channel_id,
                                    ChannelVisibility::Members,
                                    cx,
                                )
                            }),
                        )
                    } else {
                        context_menu = context_menu.separator().entry(
                            "Make Channel Public",
                            None,
                            cx.handler_for(&this, move |this, cx| {
                                this.set_channel_visibility(
                                    channel_id,
                                    ChannelVisibility::Public,
                                    cx,
                                )
                            }),
                        )
                    }
                }

                context_menu = context_menu.entry(
                    "Delete",
                    None,
                    cx.handler_for(&this, move |this, cx| this.remove_channel(channel_id, cx)),
                );
            }

            if self.channel_store.read(cx).is_root_channel(channel_id) {
                if !has_destructive_actions {
                    context_menu = context_menu.separator()
                }
                context_menu = context_menu.entry(
                    "Leave Channel",
                    None,
                    cx.handler_for(&this, move |this, cx| this.leave_channel(channel_id, cx)),
                );
            }

            context_menu
        });

        cx.focus_view(&context_menu);
        let subscription =
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(cx)
                }) {
                    cx.focus_self();
                }
                this.context_menu.take();
                cx.notify();
            });
        self.context_menu = Some((context_menu, position, subscription));

        cx.notify();
    }

    fn deploy_contact_context_menu(
        &mut self,
        position: Point<Pixels>,
        contact: Arc<Contact>,
        cx: &mut ViewContext<Self>,
    ) {
        let this = cx.view().clone();
        let in_room = ActiveCall::global(cx).read(cx).room().is_some();

        let context_menu = ContextMenu::build(cx, |mut context_menu, _| {
            let user_id = contact.user.id;

            if contact.online && !contact.busy {
                let label = if in_room {
                    format!("Invite {} to join", contact.user.github_login)
                } else {
                    format!("Call {}", contact.user.github_login)
                };
                context_menu = context_menu.entry(label, None, {
                    let this = this.clone();
                    move |cx| {
                        this.update(cx, |this, cx| {
                            this.call(user_id, cx);
                        });
                    }
                });
            }

            context_menu.entry("Remove Contact", None, {
                let this = this.clone();
                move |cx| {
                    this.update(cx, |this, cx| {
                        this.remove_contact(contact.user.id, &contact.user.github_login, cx);
                    });
                }
            })
        });

        cx.focus_view(&context_menu);
        let subscription =
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(cx)
                }) {
                    cx.focus_self();
                }
                this.context_menu.take();
                cx.notify();
            });
        self.context_menu = Some((context_menu, position, subscription));

        cx.notify();
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.take_editing_state(cx) {
            cx.focus_view(&self.filter_editor);
        } else {
            self.filter_editor.update(cx, |editor, cx| {
                if editor.buffer().read(cx).len(cx) > 0 {
                    editor.set_text("", cx);
                }
            });
        }

        if self.context_menu.is_some() {
            self.context_menu.take();
            cx.notify();
        }

        self.update_entries(false, cx);
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let ix = self.selection.map_or(0, |ix| ix + 1);
        if ix < self.entries.len() {
            self.selection = Some(ix);
        }

        if let Some(ix) = self.selection {
            self.scroll_to_item(ix)
        }
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        let ix = self.selection.take().unwrap_or(0);
        if ix > 0 {
            self.selection = Some(ix - 1);
        }

        if let Some(ix) = self.selection {
            self.scroll_to_item(ix)
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if self.confirm_channel_edit(cx) {
            return;
        }

        if let Some(selection) = self.selection {
            if let Some(entry) = self.entries.get(selection) {
                match entry {
                    ListEntry::Header(section) => match section {
                        Section::ActiveCall => Self::leave_call(cx),
                        Section::Channels => self.new_root_channel(cx),
                        Section::Contacts => self.toggle_contact_finder(cx),
                        Section::ContactRequests
                        | Section::Online
                        | Section::Offline
                        | Section::ChannelInvites => {
                            self.toggle_section_expanded(*section, cx);
                        }
                    },
                    ListEntry::Contact { contact, calling } => {
                        if contact.online && !contact.busy && !calling {
                            self.call(contact.user.id, cx);
                        }
                    }
                    ListEntry::ParticipantProject {
                        project_id,
                        host_user_id,
                        ..
                    } => {
                        if let Some(workspace) = self.workspace.upgrade() {
                            let app_state = workspace.read(cx).app_state().clone();
                            workspace::join_in_room_project(
                                *project_id,
                                *host_user_id,
                                app_state,
                                cx,
                            )
                            .detach_and_prompt_err(
                                "Failed to join project",
                                cx,
                                |_, _| None,
                            );
                        }
                    }
                    ListEntry::ParticipantScreen { peer_id, .. } => {
                        let Some(peer_id) = peer_id else {
                            return;
                        };
                        if let Some(workspace) = self.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                workspace.open_shared_screen(*peer_id, cx)
                            });
                        }
                    }
                    ListEntry::Channel { channel, .. } => {
                        let is_active = maybe!({
                            let call_channel = ActiveCall::global(cx)
                                .read(cx)
                                .room()?
                                .read(cx)
                                .channel_id()?;

                            Some(call_channel == channel.id)
                        })
                        .unwrap_or(false);
                        if is_active {
                            self.open_channel_notes(channel.id, cx)
                        } else {
                            self.join_channel(channel.id, cx)
                        }
                    }
                    ListEntry::ContactPlaceholder => self.toggle_contact_finder(cx),
                    ListEntry::CallParticipant { user, peer_id, .. } => {
                        if Some(user) == self.user_store.read(cx).current_user().as_ref() {
                            Self::leave_call(cx);
                        } else if let Some(peer_id) = peer_id {
                            self.workspace
                                .update(cx, |workspace, cx| workspace.follow(*peer_id, cx))
                                .ok();
                        }
                    }
                    ListEntry::IncomingRequest(user) => {
                        self.respond_to_contact_request(user.id, true, cx)
                    }
                    ListEntry::ChannelInvite(channel) => {
                        self.respond_to_channel_invite(channel.id, true, cx)
                    }
                    ListEntry::ChannelNotes { channel_id } => {
                        self.open_channel_notes(*channel_id, cx)
                    }
                    ListEntry::ChannelChat { channel_id } => {
                        self.join_channel_chat(*channel_id, cx)
                    }
                    ListEntry::HostedProject {
                        id: _id,
                        name: _name,
                    } => {
                        // todo()
                    }
                    ListEntry::OutgoingRequest(_) => {}
                    ListEntry::ChannelEditor { .. } => {}
                }
            }
        }
    }

    fn insert_space(&mut self, _: &InsertSpace, cx: &mut ViewContext<Self>) {
        if self.channel_editing_state.is_some() {
            self.channel_name_editor.update(cx, |editor, cx| {
                editor.insert(" ", cx);
            });
        }
    }

    fn confirm_channel_edit(&mut self, cx: &mut ViewContext<CollabPanel>) -> bool {
        if let Some(editing_state) = &mut self.channel_editing_state {
            match editing_state {
                ChannelEditingState::Create {
                    location,
                    pending_name,
                    ..
                } => {
                    if pending_name.is_some() {
                        return false;
                    }
                    let channel_name = self.channel_name_editor.read(cx).text(cx);

                    *pending_name = Some(channel_name.clone());

                    let create = self.channel_store.update(cx, |channel_store, cx| {
                        channel_store.create_channel(&channel_name, *location, cx)
                    });
                    if location.is_none() {
                        cx.spawn(|this, mut cx| async move {
                            let channel_id = create.await?;
                            this.update(&mut cx, |this, cx| {
                                this.show_channel_modal(
                                    channel_id,
                                    channel_modal::Mode::InviteMembers,
                                    cx,
                                )
                            })
                        })
                        .detach_and_prompt_err(
                            "Failed to create channel",
                            cx,
                            |_, _| None,
                        );
                    } else {
                        create.detach_and_prompt_err("Failed to create channel", cx, |_, _| None);
                    }
                    cx.notify();
                }
                ChannelEditingState::Rename {
                    location,
                    pending_name,
                } => {
                    if pending_name.is_some() {
                        return false;
                    }
                    let channel_name = self.channel_name_editor.read(cx).text(cx);
                    *pending_name = Some(channel_name.clone());

                    self.channel_store
                        .update(cx, |channel_store, cx| {
                            channel_store.rename(*location, &channel_name, cx)
                        })
                        .detach();
                    cx.notify();
                }
            }
            cx.focus_self();
            true
        } else {
            false
        }
    }

    fn toggle_section_expanded(&mut self, section: Section, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.collapsed_sections.iter().position(|s| *s == section) {
            self.collapsed_sections.remove(ix);
        } else {
            self.collapsed_sections.push(section);
        }
        self.update_entries(false, cx);
    }

    fn collapse_selected_channel(
        &mut self,
        _: &CollapseSelectedChannel,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(channel_id) = self.selected_channel().map(|channel| channel.id) else {
            return;
        };

        if self.is_channel_collapsed(channel_id) {
            return;
        }

        self.toggle_channel_collapsed(channel_id, cx);
    }

    fn expand_selected_channel(&mut self, _: &ExpandSelectedChannel, cx: &mut ViewContext<Self>) {
        let Some(id) = self.selected_channel().map(|channel| channel.id) else {
            return;
        };

        if !self.is_channel_collapsed(id) {
            return;
        }

        self.toggle_channel_collapsed(id, cx)
    }

    fn toggle_channel_collapsed(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        match self.collapsed_channels.binary_search(&channel_id) {
            Ok(ix) => {
                self.collapsed_channels.remove(ix);
            }
            Err(ix) => {
                self.collapsed_channels.insert(ix, channel_id);
            }
        };
        self.serialize(cx);
        self.update_entries(true, cx);
        cx.notify();
        cx.focus_self();
    }

    fn is_channel_collapsed(&self, channel_id: ChannelId) -> bool {
        self.collapsed_channels.binary_search(&channel_id).is_ok()
    }

    fn leave_call(cx: &mut WindowContext) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.hang_up(cx))
            .detach_and_prompt_err("Failed to hang up", cx, |_, _| None);
    }

    fn toggle_contact_finder(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    let mut finder = ContactFinder::new(self.user_store.clone(), cx);
                    finder.set_query(self.filter_editor.read(cx).text(cx), cx);
                    finder
                });
            });
        }
    }

    fn new_root_channel(&mut self, cx: &mut ViewContext<Self>) {
        self.channel_editing_state = Some(ChannelEditingState::Create {
            location: None,
            pending_name: None,
        });
        self.update_entries(false, cx);
        self.select_channel_editor();
        cx.focus_view(&self.channel_name_editor);
        cx.notify();
    }

    fn select_channel_editor(&mut self) {
        self.selection = self.entries.iter().position(|entry| match entry {
            ListEntry::ChannelEditor { .. } => true,
            _ => false,
        });
    }

    fn new_subchannel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        self.collapsed_channels
            .retain(|channel| *channel != channel_id);
        self.channel_editing_state = Some(ChannelEditingState::Create {
            location: Some(channel_id),
            pending_name: None,
        });
        self.update_entries(false, cx);
        self.select_channel_editor();
        cx.focus_view(&self.channel_name_editor);
        cx.notify();
    }

    fn manage_members(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        self.show_channel_modal(channel_id, channel_modal::Mode::ManageMembers, cx);
    }

    fn remove_selected_channel(&mut self, _: &Remove, cx: &mut ViewContext<Self>) {
        if let Some(channel) = self.selected_channel() {
            self.remove_channel(channel.id, cx)
        }
    }

    fn rename_selected_channel(&mut self, _: &SecondaryConfirm, cx: &mut ViewContext<Self>) {
        if let Some(channel) = self.selected_channel() {
            self.rename_channel(channel.id, cx);
        }
    }

    fn rename_channel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.read(cx);
        if !channel_store.is_channel_admin(channel_id) {
            return;
        }
        if let Some(channel) = channel_store.channel_for_id(channel_id).cloned() {
            self.channel_editing_state = Some(ChannelEditingState::Rename {
                location: channel_id,
                pending_name: None,
            });
            self.channel_name_editor.update(cx, |editor, cx| {
                editor.set_text(channel.name.clone(), cx);
                editor.select_all(&Default::default(), cx);
            });
            cx.focus_view(&self.channel_name_editor);
            self.update_entries(false, cx);
            self.select_channel_editor();
        }
    }

    fn set_channel_visibility(
        &mut self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        cx: &mut ViewContext<Self>,
    ) {
        self.channel_store
            .update(cx, |channel_store, cx| {
                channel_store.set_channel_visibility(channel_id, visibility, cx)
            })
            .detach_and_prompt_err("Failed to set channel visibility", cx, |e, _| match e.error_code() {
                ErrorCode::BadPublicNesting =>
                    if e.error_tag("direction") == Some("parent") {
                        Some("To make a channel public, its parent channel must be public.".to_string())
                    } else {
                        Some("To make a channel private, all of its subchannels must be private.".to_string())
                    },
                _ => None
            });
    }

    fn start_move_channel(&mut self, channel_id: ChannelId, _cx: &mut ViewContext<Self>) {
        self.channel_clipboard = Some(ChannelMoveClipboard { channel_id });
    }

    fn start_move_selected_channel(&mut self, _: &StartMoveChannel, cx: &mut ViewContext<Self>) {
        if let Some(channel) = self.selected_channel() {
            self.start_move_channel(channel.id, cx);
        }
    }

    fn move_channel_on_clipboard(
        &mut self,
        to_channel_id: ChannelId,
        cx: &mut ViewContext<CollabPanel>,
    ) {
        if let Some(clipboard) = self.channel_clipboard.take() {
            self.move_channel(clipboard.channel_id, to_channel_id, cx)
        }
    }

    fn move_channel(&self, channel_id: ChannelId, to: ChannelId, cx: &mut ViewContext<Self>) {
        self.channel_store
            .update(cx, |channel_store, cx| {
                channel_store.move_channel(channel_id, to, cx)
            })
            .detach_and_prompt_err("Failed to move channel", cx, |e, _| match e.error_code() {
                ErrorCode::BadPublicNesting => {
                    Some("Public channels must have public parents".into())
                }
                ErrorCode::CircularNesting => Some("You cannot move a channel into itself".into()),
                ErrorCode::WrongMoveTarget => {
                    Some("You cannot move a channel into a different root channel".into())
                }
                _ => None,
            })
    }

    fn open_channel_notes(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            ChannelView::open(channel_id, None, workspace, cx).detach();
        }
    }

    fn show_inline_context_menu(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
        let Some(bounds) = self
            .selection
            .and_then(|ix| self.list_state.bounds_for_item(ix))
        else {
            return;
        };

        if let Some(channel) = self.selected_channel() {
            self.deploy_channel_context_menu(
                bounds.center(),
                channel.id,
                self.selection.unwrap(),
                cx,
            );
            cx.stop_propagation();
            return;
        };

        if let Some(contact) = self.selected_contact() {
            self.deploy_contact_context_menu(bounds.center(), contact, cx);
            cx.stop_propagation();
            return;
        };
    }

    fn selected_channel(&self) -> Option<&Arc<Channel>> {
        self.selection
            .and_then(|ix| self.entries.get(ix))
            .and_then(|entry| match entry {
                ListEntry::Channel { channel, .. } => Some(channel),
                _ => None,
            })
    }

    fn selected_contact(&self) -> Option<Arc<Contact>> {
        self.selection
            .and_then(|ix| self.entries.get(ix))
            .and_then(|entry| match entry {
                ListEntry::Contact { contact, .. } => Some(contact.clone()),
                _ => None,
            })
    }

    fn show_channel_modal(
        &mut self,
        channel_id: ChannelId,
        mode: channel_modal::Mode,
        cx: &mut ViewContext<Self>,
    ) {
        let workspace = self.workspace.clone();
        let user_store = self.user_store.clone();
        let channel_store = self.channel_store.clone();

        cx.spawn(|_, mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    ChannelModal::new(
                        user_store.clone(),
                        channel_store.clone(),
                        channel_id,
                        mode,
                        cx,
                    )
                });
            })
        })
        .detach();
    }

    fn leave_channel(&self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let Some(user_id) = self.user_store.read(cx).current_user().map(|u| u.id) else {
            return;
        };
        let Some(channel) = self.channel_store.read(cx).channel_for_id(channel_id) else {
            return;
        };
        let prompt_message = format!("Are you sure you want to leave \"#{}\"?", channel.name);
        let answer = cx.prompt(
            PromptLevel::Warning,
            &prompt_message,
            None,
            &["Leave", "Cancel"],
        );
        cx.spawn(|this, mut cx| async move {
            if answer.await? != 0 {
                return Ok(());
            }
            this.update(&mut cx, |this, cx| {
                this.channel_store.update(cx, |channel_store, cx| {
                    channel_store.remove_member(channel_id, user_id, cx)
                })
            })?
            .await
        })
        .detach_and_prompt_err("Failed to leave channel", cx, |_, _| None)
    }

    fn remove_channel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.clone();
        if let Some(channel) = channel_store.read(cx).channel_for_id(channel_id) {
            let prompt_message = format!(
                "Are you sure you want to remove the channel \"{}\"?",
                channel.name
            );
            let answer = cx.prompt(
                PromptLevel::Warning,
                &prompt_message,
                None,
                &["Remove", "Cancel"],
            );
            cx.spawn(|this, mut cx| async move {
                if answer.await? == 0 {
                    channel_store
                        .update(&mut cx, |channels, _| channels.remove_channel(channel_id))?
                        .await
                        .notify_async_err(&mut cx);
                    this.update(&mut cx, |_, cx| cx.focus_self()).ok();
                }
                anyhow::Ok(())
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
        let answer = cx.prompt(
            PromptLevel::Warning,
            &prompt_message,
            None,
            &["Remove", "Cancel"],
        );
        cx.spawn(|_, mut cx| async move {
            if answer.await? == 0 {
                user_store
                    .update(&mut cx, |store, cx| store.remove_contact(user_id, cx))?
                    .await
                    .notify_async_err(&mut cx);
            }
            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to remove contact", cx, |_, _| None);
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
            .detach_and_prompt_err("Failed to respond to contact request", cx, |_, _| None);
    }

    fn respond_to_channel_invite(
        &mut self,
        channel_id: ChannelId,
        accept: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.channel_store
            .update(cx, |store, cx| {
                store.respond_to_channel_invite(channel_id, accept, cx)
            })
            .detach();
    }

    fn call(&mut self, recipient_user_id: u64, cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| {
                call.invite(recipient_user_id, Some(self.project.clone()), cx)
            })
            .detach_and_prompt_err("Call failed", cx, |_, _| None);
    }

    fn join_channel(&self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(handle) = cx.window_handle().downcast::<Workspace>() else {
            return;
        };
        workspace::join_channel(
            channel_id,
            workspace.read(cx).app_state().clone(),
            Some(handle),
            cx,
        )
        .detach_and_prompt_err("Failed to join channel", cx, |_, _| None)
    }

    fn join_channel_chat(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        cx.window_context().defer(move |cx| {
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.focus_panel::<ChatPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel
                            .select_channel(channel_id, None, cx)
                            .detach_and_notify_err(cx);
                    });
                }
            });
        });
    }

    fn copy_channel_link(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.read(cx);
        let Some(channel) = channel_store.channel_for_id(channel_id) else {
            return;
        };
        let item = ClipboardItem::new(channel.link(cx));
        cx.write_to_clipboard(item)
    }

    fn render_signed_out(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let collab_blurb = "Work with your team in realtime with collaborative editing, voice, shared notes and more.";

        v_flex()
            .gap_6()
            .p_4()
            .child(Label::new(collab_blurb))
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Button::new("sign_in", "Sign in")
                            .icon_color(Color::Muted)
                            .icon(IconName::Github)
                            .icon_position(IconPosition::Start)
                            .style(ButtonStyle::Filled)
                            .full_width()
                            .on_click(cx.listener(|this, _, cx| {
                                let client = this.client.clone();
                                cx.spawn(|_, mut cx| async move {
                                    client
                                        .authenticate_and_connect(true, &cx)
                                        .await
                                        .notify_async_err(&mut cx);
                                })
                                .detach()
                            })),
                    )
                    .child(
                        div().flex().w_full().items_center().child(
                            Label::new("Sign in to enable collaboration.")
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                    ),
            )
    }

    fn render_list_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let entry = &self.entries[ix];

        let is_selected = self.selection == Some(ix);
        match entry {
            ListEntry::Header(section) => {
                let is_collapsed = self.collapsed_sections.contains(section);
                self.render_header(*section, is_selected, is_collapsed, cx)
                    .into_any_element()
            }
            ListEntry::Contact { contact, calling } => self
                .render_contact(contact, *calling, is_selected, cx)
                .into_any_element(),
            ListEntry::ContactPlaceholder => self
                .render_contact_placeholder(is_selected, cx)
                .into_any_element(),
            ListEntry::IncomingRequest(user) => self
                .render_contact_request(user, true, is_selected, cx)
                .into_any_element(),
            ListEntry::OutgoingRequest(user) => self
                .render_contact_request(user, false, is_selected, cx)
                .into_any_element(),
            ListEntry::Channel {
                channel,
                depth,
                has_children,
            } => self
                .render_channel(channel, *depth, *has_children, is_selected, ix, cx)
                .into_any_element(),
            ListEntry::ChannelEditor { depth } => {
                self.render_channel_editor(*depth, cx).into_any_element()
            }
            ListEntry::ChannelInvite(channel) => self
                .render_channel_invite(channel, is_selected, cx)
                .into_any_element(),
            ListEntry::CallParticipant {
                user,
                peer_id,
                is_pending,
                role,
            } => self
                .render_call_participant(user, *peer_id, *is_pending, *role, is_selected, cx)
                .into_any_element(),
            ListEntry::ParticipantProject {
                project_id,
                worktree_root_names,
                host_user_id,
                is_last,
            } => self
                .render_participant_project(
                    *project_id,
                    &worktree_root_names,
                    *host_user_id,
                    *is_last,
                    is_selected,
                    cx,
                )
                .into_any_element(),
            ListEntry::ParticipantScreen { peer_id, is_last } => self
                .render_participant_screen(*peer_id, *is_last, is_selected, cx)
                .into_any_element(),
            ListEntry::ChannelNotes { channel_id } => self
                .render_channel_notes(*channel_id, is_selected, cx)
                .into_any_element(),
            ListEntry::ChannelChat { channel_id } => self
                .render_channel_chat(*channel_id, is_selected, cx)
                .into_any_element(),

            ListEntry::HostedProject { id, name } => self
                .render_channel_project(*id, name, is_selected, cx)
                .into_any_element(),
        }
    }

    fn render_signed_in(&mut self, cx: &mut ViewContext<Self>) -> Div {
        v_flex()
            .size_full()
            .child(list(self.list_state.clone()).size_full())
            .child(
                v_flex()
                    .child(div().mx_2().border_primary(cx).border_t_1())
                    .child(
                        v_flex()
                            .p_2()
                            .child(self.render_filter_input(&self.filter_editor, cx)),
                    ),
            )
    }

    fn render_filter_input(
        &self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };

        EditorElement::new(
            editor,
            EditorStyle {
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_header(
        &self,
        section: Section,
        is_selected: bool,
        is_collapsed: bool,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let mut channel_link = None;
        let mut channel_tooltip_text = None;
        let mut channel_icon = None;

        let text = match section {
            Section::ActiveCall => {
                let channel_name = maybe!({
                    let channel_id = ActiveCall::global(cx).read(cx).channel_id(cx)?;

                    let channel = self.channel_store.read(cx).channel_for_id(channel_id)?;

                    channel_link = Some(channel.link(cx));
                    (channel_icon, channel_tooltip_text) = match channel.visibility {
                        proto::ChannelVisibility::Public => {
                            (Some("icons/public.svg"), Some("Copy public channel link."))
                        }
                        proto::ChannelVisibility::Members => {
                            (Some("icons/hash.svg"), Some("Copy private channel link."))
                        }
                    };

                    Some(channel.name.as_ref())
                });

                if let Some(name) = channel_name {
                    SharedString::from(name.to_string())
                } else {
                    SharedString::from("Current Call")
                }
            }
            Section::ContactRequests => SharedString::from("Requests"),
            Section::Contacts => SharedString::from("Contacts"),
            Section::Channels => SharedString::from("Channels"),
            Section::ChannelInvites => SharedString::from("Invites"),
            Section::Online => SharedString::from("Online"),
            Section::Offline => SharedString::from("Offline"),
        };

        let button = match section {
            Section::ActiveCall => channel_link.map(|channel_link| {
                let channel_link_copy = channel_link.clone();
                IconButton::new("channel-link", IconName::Copy)
                    .icon_size(IconSize::Small)
                    .size(ButtonSize::None)
                    .visible_on_hover("section-header")
                    .on_click(move |_, cx| {
                        let item = ClipboardItem::new(channel_link_copy.clone());
                        cx.write_to_clipboard(item)
                    })
                    .tooltip(|cx| Tooltip::text("Copy channel link", cx))
                    .into_any_element()
            }),
            Section::Contacts => Some(
                IconButton::new("add-contact", IconName::Plus)
                    .on_click(cx.listener(|this, _, cx| this.toggle_contact_finder(cx)))
                    .tooltip(|cx| Tooltip::text("Search for new contact", cx))
                    .into_any_element(),
            ),
            Section::Channels => Some(
                IconButton::new("add-channel", IconName::Plus)
                    .on_click(cx.listener(|this, _, cx| this.new_root_channel(cx)))
                    .tooltip(|cx| Tooltip::text("Create a channel", cx))
                    .into_any_element(),
            ),
            _ => None,
        };

        let can_collapse = match section {
            Section::ActiveCall | Section::Channels | Section::Contacts => false,
            Section::ChannelInvites
            | Section::ContactRequests
            | Section::Online
            | Section::Offline => true,
        };

        h_flex().w_full().group("section-header").child(
            ListHeader::new(text)
                .when(can_collapse, |header| {
                    header
                        .toggle(Some(!is_collapsed))
                        .on_toggle(cx.listener(move |this, _, cx| {
                            this.toggle_section_expanded(section, cx);
                        }))
                })
                .inset(true)
                .end_slot::<AnyElement>(button)
                .selected(is_selected),
        )
    }

    fn render_contact(
        &self,
        contact: &Arc<Contact>,
        calling: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let online = contact.online;
        let busy = contact.busy || calling;
        let github_login = SharedString::from(contact.user.github_login.clone());
        let item = ListItem::new(github_login.clone())
            .indent_level(1)
            .indent_step_size(px(20.))
            .selected(is_selected)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Label::new(github_login.clone()))
                    .when(calling, |el| {
                        el.child(Label::new("Calling").color(Color::Muted))
                    })
                    .when(!calling, |el| {
                        el.child(
                            IconButton::new("contact context menu", IconName::Ellipsis)
                                .icon_color(Color::Muted)
                                .visible_on_hover("")
                                .on_click(cx.listener({
                                    let contact = contact.clone();
                                    move |this, event: &ClickEvent, cx| {
                                        this.deploy_contact_context_menu(
                                            event.down.position,
                                            contact.clone(),
                                            cx,
                                        );
                                    }
                                })),
                        )
                    }),
            )
            .on_secondary_mouse_down(cx.listener({
                let contact = contact.clone();
                move |this, event: &MouseDownEvent, cx| {
                    this.deploy_contact_context_menu(event.position, contact.clone(), cx);
                }
            }))
            .start_slot(
                // todo handle contacts with no avatar
                Avatar::new(contact.user.avatar_uri.clone())
                    .indicator::<AvatarAvailabilityIndicator>(if online {
                        Some(AvatarAvailabilityIndicator::new(match busy {
                            true => ui::Availability::Busy,
                            false => ui::Availability::Free,
                        }))
                    } else {
                        None
                    }),
            );

        div()
            .id(github_login.clone())
            .group("")
            .child(item)
            .tooltip(move |cx| {
                let text = if !online {
                    format!(" {} is offline", &github_login)
                } else if busy {
                    format!(" {} is on a call", &github_login)
                } else {
                    let room = ActiveCall::global(cx).read(cx).room();
                    if room.is_some() {
                        format!("Invite {} to join call", &github_login)
                    } else {
                        format!("Call {}", &github_login)
                    }
                };
                Tooltip::text(text, cx)
            })
    }

    fn render_contact_request(
        &self,
        user: &Arc<User>,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let github_login = SharedString::from(user.github_login.clone());
        let user_id = user.id;
        let is_response_pending = self.user_store.read(cx).is_contact_request_pending(&user);
        let color = if is_response_pending {
            Color::Muted
        } else {
            Color::Default
        };

        let controls = if is_incoming {
            vec![
                IconButton::new("decline-contact", IconName::Close)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.respond_to_contact_request(user_id, false, cx);
                    }))
                    .icon_color(color)
                    .tooltip(|cx| Tooltip::text("Decline invite", cx)),
                IconButton::new("accept-contact", IconName::Check)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.respond_to_contact_request(user_id, true, cx);
                    }))
                    .icon_color(color)
                    .tooltip(|cx| Tooltip::text("Accept invite", cx)),
            ]
        } else {
            let github_login = github_login.clone();
            vec![IconButton::new("remove_contact", IconName::Close)
                .on_click(cx.listener(move |this, _, cx| {
                    this.remove_contact(user_id, &github_login, cx);
                }))
                .icon_color(color)
                .tooltip(|cx| Tooltip::text("Cancel invite", cx))]
        };

        ListItem::new(github_login.clone())
            .indent_level(1)
            .indent_step_size(px(20.))
            .selected(is_selected)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Label::new(github_login.clone()))
                    .child(h_flex().children(controls)),
            )
            .start_slot(Avatar::new(user.avatar_uri.clone()))
    }

    fn render_channel_invite(
        &self,
        channel: &Arc<Channel>,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> ListItem {
        let channel_id = channel.id;
        let response_is_pending = self
            .channel_store
            .read(cx)
            .has_pending_channel_invite_response(&channel);
        let color = if response_is_pending {
            Color::Muted
        } else {
            Color::Default
        };

        let controls = [
            IconButton::new("reject-invite", IconName::Close)
                .on_click(cx.listener(move |this, _, cx| {
                    this.respond_to_channel_invite(channel_id, false, cx);
                }))
                .icon_color(color)
                .tooltip(|cx| Tooltip::text("Decline invite", cx)),
            IconButton::new("accept-invite", IconName::Check)
                .on_click(cx.listener(move |this, _, cx| {
                    this.respond_to_channel_invite(channel_id, true, cx);
                }))
                .icon_color(color)
                .tooltip(|cx| Tooltip::text("Accept invite", cx)),
        ];

        ListItem::new(("channel-invite", channel.id.0 as usize))
            .selected(is_selected)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Label::new(channel.name.clone()))
                    .child(h_flex().children(controls)),
            )
            .start_slot(
                Icon::new(IconName::Hash)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
    }

    fn render_contact_placeholder(
        &self,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> ListItem {
        ListItem::new("contact-placeholder")
            .child(Icon::new(IconName::Plus))
            .child(Label::new("Add a Contact"))
            .selected(is_selected)
            .on_click(cx.listener(|this, _, cx| this.toggle_contact_finder(cx)))
    }

    fn render_channel(
        &self,
        channel: &Channel,
        depth: usize,
        has_children: bool,
        is_selected: bool,
        ix: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let channel_id = channel.id;

        let is_active = maybe!({
            let call_channel = ActiveCall::global(cx)
                .read(cx)
                .room()?
                .read(cx)
                .channel_id()?;
            Some(call_channel == channel_id)
        })
        .unwrap_or(false);
        let channel_store = self.channel_store.read(cx);
        let is_public = channel_store
            .channel_for_id(channel_id)
            .map(|channel| channel.visibility)
            == Some(proto::ChannelVisibility::Public);
        let disclosed =
            has_children.then(|| self.collapsed_channels.binary_search(&channel.id).is_err());

        let has_messages_notification = channel_store.has_new_messages(channel_id);
        let has_notes_notification = channel_store.has_channel_buffer_changed(channel_id);

        const FACEPILE_LIMIT: usize = 3;
        let participants = self.channel_store.read(cx).channel_participants(channel_id);

        let face_pile = if participants.is_empty() {
            None
        } else {
            let extra_count = participants.len().saturating_sub(FACEPILE_LIMIT);
            let result = FacePile::new(
                participants
                    .iter()
                    .map(|user| Avatar::new(user.avatar_uri.clone()).into_any_element())
                    .take(FACEPILE_LIMIT)
                    .chain(if extra_count > 0 {
                        Some(
                            div()
                                .ml_2()
                                .child(Label::new(format!("+{extra_count}")))
                                .into_any_element(),
                        )
                    } else {
                        None
                    })
                    .collect::<SmallVec<_>>(),
            );

            Some(result)
        };

        let width = self.width.unwrap_or(px(240.));
        let root_id = channel.root_id();

        div()
            .h_6()
            .id(channel_id.0 as usize)
            .group("")
            .flex()
            .w_full()
            .when(!channel.is_root_channel(), |el| {
                el.on_drag(channel.clone(), move |channel, cx| {
                    cx.new_view(|_| DraggedChannelView {
                        channel: channel.clone(),
                        width,
                    })
                })
            })
            .drag_over::<Channel>({
                move |style, dragged_channel: &Channel, cx| {
                    if dragged_channel.root_id() == root_id {
                        style.bg(cx.theme().colors().ghost_element_hover)
                    } else {
                        style
                    }
                }
            })
            .on_drop(cx.listener(move |this, dragged_channel: &Channel, cx| {
                if dragged_channel.root_id() != root_id {
                    return;
                }
                this.move_channel(dragged_channel.id, channel_id, cx);
            }))
            .child(
                ListItem::new(channel_id.0 as usize)
                    // Add one level of depth for the disclosure arrow.
                    .indent_level(depth + 1)
                    .indent_step_size(px(20.))
                    .selected(is_selected || is_active)
                    .toggle(disclosed)
                    .on_toggle(
                        cx.listener(move |this, _, cx| {
                            this.toggle_channel_collapsed(channel_id, cx)
                        }),
                    )
                    .on_click(cx.listener(move |this, _, cx| {
                        if is_active {
                            this.open_channel_notes(channel_id, cx)
                        } else {
                            this.join_channel(channel_id, cx)
                        }
                    }))
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, cx| {
                            this.deploy_channel_context_menu(event.position, channel_id, ix, cx)
                        },
                    ))
                    .start_slot(
                        div()
                            .relative()
                            .child(
                                Icon::new(if is_public {
                                    IconName::Public
                                } else {
                                    IconName::Hash
                                })
                                .size(IconSize::Small)
                                .color(Color::Muted),
                            )
                            .children(has_notes_notification.then(|| {
                                div()
                                    .w_1p5()
                                    .absolute()
                                    .right(px(-1.))
                                    .top(px(-1.))
                                    .child(Indicator::dot().color(Color::Info))
                            })),
                    )
                    .child(
                        h_flex()
                            .id(channel_id.0 as usize)
                            .child(Label::new(channel.name.clone()))
                            .children(face_pile.map(|face_pile| face_pile.p_1())),
                    ),
            )
            .child(
                h_flex().absolute().right(rems(0.)).h_full().child(
                    h_flex()
                        .h_full()
                        .gap_1()
                        .px_1()
                        .child(
                            IconButton::new("channel_chat", IconName::MessageBubbles)
                                .style(ButtonStyle::Filled)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::Small)
                                .icon_color(if has_messages_notification {
                                    Color::Default
                                } else {
                                    Color::Muted
                                })
                                .on_click(cx.listener(move |this, _, cx| {
                                    this.join_channel_chat(channel_id, cx)
                                }))
                                .tooltip(|cx| Tooltip::text("Open channel chat", cx))
                                .visible_on_hover(""),
                        )
                        .child(
                            IconButton::new("channel_notes", IconName::File)
                                .style(ButtonStyle::Filled)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::Small)
                                .icon_color(if has_notes_notification {
                                    Color::Default
                                } else {
                                    Color::Muted
                                })
                                .on_click(cx.listener(move |this, _, cx| {
                                    this.open_channel_notes(channel_id, cx)
                                }))
                                .tooltip(|cx| Tooltip::text("Open channel notes", cx))
                                .visible_on_hover(""),
                        ),
                ),
            )
            .tooltip({
                let channel_store = self.channel_store.clone();
                move |cx| {
                    cx.new_view(|_| JoinChannelTooltip {
                        channel_store: channel_store.clone(),
                        channel_id,
                        has_notes_notification,
                    })
                    .into()
                }
            })
    }

    fn render_channel_editor(&self, depth: usize, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let item = ListItem::new("channel-editor")
            .inset(false)
            // Add one level of depth for the disclosure arrow.
            .indent_level(depth + 1)
            .indent_step_size(px(20.))
            .start_slot(
                Icon::new(IconName::Hash)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            );

        if let Some(pending_name) = self
            .channel_editing_state
            .as_ref()
            .and_then(|state| state.pending_name())
        {
            item.child(Label::new(pending_name))
        } else {
            item.child(self.channel_name_editor.clone())
        }
    }
}

fn render_tree_branch(is_last: bool, overdraw: bool, cx: &mut WindowContext) -> impl IntoElement {
    let rem_size = cx.rem_size();
    let line_height = cx.text_style().line_height_in_pixels(rem_size);
    let width = rem_size * 1.5;
    let thickness = px(1.);
    let color = cx.theme().colors().text;

    canvas(
        |_, _| {},
        move |bounds, _, cx| {
            let start_x = (bounds.left() + bounds.right() - thickness) / 2.;
            let start_y = (bounds.top() + bounds.bottom() - thickness) / 2.;
            let right = bounds.right();
            let top = bounds.top();

            cx.paint_quad(fill(
                Bounds::from_corners(
                    point(start_x, top),
                    point(
                        start_x + thickness,
                        if is_last {
                            start_y
                        } else {
                            bounds.bottom() + if overdraw { px(1.) } else { px(0.) }
                        },
                    ),
                ),
                color,
            ));
            cx.paint_quad(fill(
                Bounds::from_corners(point(start_x, start_y), point(right, start_y + thickness)),
                color,
            ));
        },
    )
    .w(width)
    .h(line_height)
}

impl Render for CollabPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CollabPanel")
            .on_action(cx.listener(CollabPanel::cancel))
            .on_action(cx.listener(CollabPanel::select_next))
            .on_action(cx.listener(CollabPanel::select_prev))
            .on_action(cx.listener(CollabPanel::confirm))
            .on_action(cx.listener(CollabPanel::insert_space))
            .on_action(cx.listener(CollabPanel::remove_selected_channel))
            .on_action(cx.listener(CollabPanel::show_inline_context_menu))
            .on_action(cx.listener(CollabPanel::rename_selected_channel))
            .on_action(cx.listener(CollabPanel::collapse_selected_channel))
            .on_action(cx.listener(CollabPanel::expand_selected_channel))
            .on_action(cx.listener(CollabPanel::start_move_selected_channel))
            .track_focus(&self.focus_handle)
            .size_full()
            .child(if self.user_store.read(cx).current_user().is_none() {
                self.render_signed_out(cx)
            } else {
                self.render_signed_in(cx)
            })
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::AnchorCorner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl EventEmitter<PanelEvent> for CollabPanel {}

impl Panel for CollabPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        CollaborationPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<CollaborationPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| settings.dock = Some(position),
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| CollaborationPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &gpui::WindowContext) -> Option<ui::IconName> {
        CollaborationPanelSettings::get_global(cx)
            .button
            .then(|| ui::IconName::Collab)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Collab Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "CollabPanel"
    }
}

impl FocusableView for CollabPanel {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.filter_editor.focus_handle(cx).clone()
    }
}

impl PartialEq for ListEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ListEntry::Header(section_1) => {
                if let ListEntry::Header(section_2) = other {
                    return section_1 == section_2;
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
            ListEntry::Channel {
                channel: channel_1, ..
            } => {
                if let ListEntry::Channel {
                    channel: channel_2, ..
                } = other
                {
                    return channel_1.id == channel_2.id;
                }
            }
            ListEntry::HostedProject { id, .. } => {
                if let ListEntry::HostedProject { id: other_id, .. } = other {
                    return id == other_id;
                }
            }
            ListEntry::ChannelNotes { channel_id } => {
                if let ListEntry::ChannelNotes {
                    channel_id: other_id,
                } = other
                {
                    return channel_id == other_id;
                }
            }
            ListEntry::ChannelChat { channel_id } => {
                if let ListEntry::ChannelChat {
                    channel_id: other_id,
                } = other
                {
                    return channel_id == other_id;
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
            ListEntry::ContactPlaceholder => {
                if let ListEntry::ContactPlaceholder = other {
                    return true;
                }
            }
        }
        false
    }
}

struct DraggedChannelView {
    channel: Channel,
    width: Pixels,
}

impl Render for DraggedChannelView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.family.clone();
        h_flex()
            .font_family(ui_font)
            .bg(cx.theme().colors().background)
            .w(self.width)
            .p_1()
            .gap_1()
            .child(
                Icon::new(
                    if self.channel.visibility == proto::ChannelVisibility::Public {
                        IconName::Public
                    } else {
                        IconName::Hash
                    },
                )
                .size(IconSize::Small)
                .color(Color::Muted),
            )
            .child(Label::new(self.channel.name.clone()))
    }
}

struct JoinChannelTooltip {
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    #[allow(unused)]
    has_notes_notification: bool,
}

impl Render for JoinChannelTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        tooltip_container(cx, |container, cx| {
            let participants = self
                .channel_store
                .read(cx)
                .channel_participants(self.channel_id);

            container
                .child(Label::new("Join channel"))
                .children(participants.iter().map(|participant| {
                    h_flex()
                        .gap_2()
                        .child(Avatar::new(participant.avatar_uri.clone()))
                        .child(Label::new(participant.github_login.clone()))
                }))
        })
    }
}
