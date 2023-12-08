#![allow(unused)]
mod channel_modal;
mod contact_finder;

// use crate::{
//     channel_view::{self, ChannelView},
//     chat_panel::ChatPanel,
//     face_pile::FacePile,
//     panel_settings, CollaborationPanelSettings,
// };
// use anyhow::Result;
// use call::ActiveCall;
// use channel::{Channel, ChannelEvent, ChannelId, ChannelStore};
// use channel_modal::ChannelModal;
// use client::{
//     proto::{self, PeerId},
//     Client, Contact, User, UserStore,
// };
use contact_finder::ContactFinder;
use menu::{Cancel, Confirm, SelectNext, SelectPrev};
use rpc::proto::{self, PeerId};
use theme::{ActiveTheme, ThemeSettings};
// use context_menu::{ContextMenu, ContextMenuItem};
// use db::kvp::KEY_VALUE_STORE;
// use drag_and_drop::{DragAndDrop, Draggable};
// use editor::{Cancel, Editor};
// use feature_flags::{ChannelsAlpha, FeatureFlagAppExt, FeatureFlagViewExt};
// use futures::StreamExt;
// use fuzzy::{match_strings, StringMatchCandidate};
// use gpui::{
//     actions,
//     elements::{
//         Canvas, ChildView, Component, ContainerStyle, Empty, Flex, Image, Label, List, ListOffset,
//         ListState, MouseEventHandler, Orientation, OverlayPositionMode, Padding, ParentElement,
//         SafeStylable, Stack, Svg,
//     },
//     fonts::TextStyle,
//     geometry::{
//         rect::RectF,
//         vector::{vec2f, Vector2F},
//     },
//     impl_actions,
//     platform::{CursorStyle, MouseButton, PromptLevel},
//     serde_json, AnyElement, AppContext, AsyncAppContext, ClipboardItem, Element, Entity, FontCache,
//     ModelHandle, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
// };
// use menu::{Confirm, SelectNext, SelectPrev};
// use project::{Fs, Project};
// use serde_derive::{Deserialize, Serialize};
// use settings::SettingsStore;
// use std::{borrow::Cow, hash::Hash, mem, sync::Arc};
// use theme::{components::ComponentExt, IconButton, Interactive};
// use util::{maybe, ResultExt, TryFutureExt};
// use workspace::{
//     dock::{DockPosition, Panel},
//     item::ItemHandle,
//     FollowNextCollaborator, Workspace,
// };

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct ToggleCollapse {
//     location: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct NewChannel {
//     location: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct RenameChannel {
//     channel_id: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct ToggleSelectedIx {
//     ix: usize,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct RemoveChannel {
//     channel_id: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct InviteMembers {
//     channel_id: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct ManageMembers {
//     channel_id: ChannelId,
// }

#[derive(Action, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OpenChannelNotes {
    pub channel_id: ChannelId,
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct JoinChannelCall {
//     pub channel_id: u64,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct JoinChannelChat {
//     pub channel_id: u64,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct CopyChannelLink {
//     pub channel_id: u64,
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct StartMoveChannelFor {
//     channel_id: ChannelId,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// struct MoveChannel {
//     to: ChannelId,
// }

actions!(
    ToggleFocus,
    Remove,
    Secondary,
    CollapseSelectedChannel,
    ExpandSelectedChannel,
    StartMoveChannel,
    MoveSelected,
    InsertSpace,
);

// impl_actions!(
//     collab_panel,
//     [
//         RemoveChannel,
//         NewChannel,
//         InviteMembers,
//         ManageMembers,
//         RenameChannel,
//         ToggleCollapse,
//         OpenChannelNotes,
//         JoinChannelCall,
//         JoinChannelChat,
//         CopyChannelLink,
//         StartMoveChannelFor,
//         MoveChannel,
//         ToggleSelectedIx
//     ]
// );

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ChannelMoveClipboard {
    channel_id: ChannelId,
}

const COLLABORATION_PANEL_KEY: &'static str = "CollaborationPanel";

use std::{iter::once, mem, sync::Arc};

use call::ActiveCall;
use channel::{Channel, ChannelEvent, ChannelId, ChannelStore};
use client::{Client, Contact, User, UserStore};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use feature_flags::{ChannelsAlpha, FeatureFlagAppExt, FeatureFlagViewExt};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions, canvas, div, img, overlay, point, prelude::*, px, rems, serde_json, size, Action,
    AppContext, AsyncWindowContext, Bounds, ClipboardItem, DismissEvent, Div, EventEmitter,
    FocusHandle, Focusable, FocusableView, Hsla, InteractiveElement, IntoElement, Length, Model,
    MouseDownEvent, ParentElement, Pixels, Point, PromptLevel, Quad, Render, RenderOnce,
    ScrollHandle, SharedString, Size, Stateful, Styled, Subscription, Task, View, ViewContext,
    VisualContext, WeakView,
};
use project::{Fs, Project};
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use ui::prelude::*;
use ui::{
    h_stack, v_stack, Avatar, Button, Color, ContextMenu, Icon, IconButton, IconElement, IconSize,
    Label, List, ListHeader, ListItem, Tooltip,
};
use util::{maybe, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::NotifyResultExt,
    Workspace,
};

use crate::channel_view::ChannelView;
use crate::chat_panel::ChatPanel;
use crate::{face_pile::FacePile, CollaborationPanelSettings};

use self::channel_modal::ChannelModal;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<CollabPanel>(cx);
        });
    })
    .detach();
    //     contact_finder::init(cx);
    //     channel_modal::init(cx);
    //     channel_view::init(cx);

    //     cx.add_action(CollabPanel::cancel);
    //     cx.add_action(CollabPanel::select_next);
    //     cx.add_action(CollabPanel::select_prev);
    //     cx.add_action(CollabPanel::confirm);
    //     cx.add_action(CollabPanel::insert_space);
    //     cx.add_action(CollabPanel::remove);
    //     cx.add_action(CollabPanel::remove_selected_channel);
    //     cx.add_action(CollabPanel::show_inline_context_menu);
    //     cx.add_action(CollabPanel::new_subchannel);
    //     cx.add_action(CollabPanel::invite_members);
    //     cx.add_action(CollabPanel::manage_members);
    //     cx.add_action(CollabPanel::rename_selected_channel);
    //     cx.add_action(CollabPanel::rename_channel);
    //     cx.add_action(CollabPanel::toggle_channel_collapsed_action);
    //     cx.add_action(CollabPanel::collapse_selected_channel);
    //     cx.add_action(CollabPanel::expand_selected_channel);
    //     cx.add_action(CollabPanel::open_channel_notes);
    //     cx.add_action(CollabPanel::join_channel_chat);
    //     cx.add_action(CollabPanel::copy_channel_link);

    //     cx.add_action(
    //         |panel: &mut CollabPanel, action: &ToggleSelectedIx, cx: &mut ViewContext<CollabPanel>| {
    //             if panel.selection.take() != Some(action.ix) {
    //                 panel.selection = Some(action.ix)
    //             }

    //             cx.notify();
    //         },
    //     );

    //     cx.add_action(
    //         |panel: &mut CollabPanel, _: &MoveSelected, cx: &mut ViewContext<CollabPanel>| {
    //             let Some(clipboard) = panel.channel_clipboard.take() else {
    //                 return;
    //             };
    //             let Some(selected_channel) = panel.selected_channel() else {
    //                 return;
    //             };

    //             panel
    //                 .channel_store
    //                 .update(cx, |channel_store, cx| {
    //                     channel_store.move_channel(clipboard.channel_id, Some(selected_channel.id), cx)
    //                 })
    //                 .detach_and_log_err(cx)
    //         },
    //     );

    //     cx.add_action(
    //         |panel: &mut CollabPanel, action: &MoveChannel, cx: &mut ViewContext<CollabPanel>| {
    //             if let Some(clipboard) = panel.channel_clipboard.take() {
    //                 panel.channel_store.update(cx, |channel_store, cx| {
    //                     channel_store
    //                         .move_channel(clipboard.channel_id, Some(action.to), cx)
    //                         .detach_and_log_err(cx)
    //                 })
    //             }
    //         },
    //     );
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
    scroll_handle: ScrollHandle,
    subscriptions: Vec<Subscription>,
    collapsed_sections: Vec<Section>,
    collapsed_channels: Vec<ChannelId>,
    drag_target_channel: ChannelDragTarget,
    workspace: WeakView<Workspace>,
    // context_menu_on_selected: bool,
}

#[derive(PartialEq, Eq)]
enum ChannelDragTarget {
    None,
    Root,
    Channel(ChannelId),
}

#[derive(Serialize, Deserialize)]
struct SerializedCollabPanel {
    width: Option<Pixels>,
    collapsed_channels: Option<Vec<u64>>,
}

// #[derive(Debug)]
// pub enum Event {
//     DockPositionChanged,
//     Focus,
//     Dismissed,
// }

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
    //     ChannelInvite(Arc<Channel>),
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
    Contact {
        contact: Arc<Contact>,
        calling: bool,
    },
    ContactPlaceholder,
}

impl CollabPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.build_view(|cx| {
            //             let view_id = cx.view_id();

            let filter_editor = cx.build_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter channels, contacts", cx);
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
                } else if let editor::EditorEvent::Blurred = event {
                    let query = this.filter_editor.read(cx).text(cx);
                    if query.is_empty() {
                        this.selection.take();
                        this.update_entries(true, cx);
                    }
                }
            })
            .detach();

            let channel_name_editor = cx.build_view(|cx| Editor::single_line(cx));

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

            //             let list_state =
            //                 ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
            //                     let theme = theme::current(cx).clone();
            //                     let is_selected = this.selection == Some(ix);
            //                     let current_project_id = this.project.read(cx).remote_id();

            //                     match &this.entries[ix] {
            //                         ListEntry::Header(section) => {
            //                             let is_collapsed = this.collapsed_sections.contains(section);
            //                             this.render_header(*section, &theme, is_selected, is_collapsed, cx)
            //                         }
            //                         ListEntry::CallParticipant {
            //                             user,
            //                             peer_id,
            //                             is_pending,
            //                         } => Self::render_call_participant(
            //                             user,
            //                             *peer_id,
            //                             this.user_store.clone(),
            //                             *is_pending,
            //                             is_selected,
            //                             &theme,
            //                             cx,
            //                         ),
            //                         ListEntry::ParticipantProject {
            //                             project_id,
            //                             worktree_root_names,
            //                             host_user_id,
            //                             is_last,
            //                         } => Self::render_participant_project(
            //                             *project_id,
            //                             worktree_root_names,
            //                             *host_user_id,
            //                             Some(*project_id) == current_project_id,
            //                             *is_last,
            //                             is_selected,
            //                             &theme,
            //                             cx,
            //                         ),
            //                         ListEntry::ParticipantScreen { peer_id, is_last } => {
            //                             Self::render_participant_screen(
            //                                 *peer_id,
            //                                 *is_last,
            //                                 is_selected,
            //                                 &theme.collab_panel,
            //                                 cx,
            //                             )
            //                         }
            //                         ListEntry::Channel {
            //                             channel,
            //                             depth,
            //                             has_children,
            //                         } => {
            //                             let channel_row = this.render_channel(
            //                                 &*channel,
            //                                 *depth,
            //                                 &theme,
            //                                 is_selected,
            //                                 *has_children,
            //                                 ix,
            //                                 cx,
            //                             );

            //                             if is_selected && this.context_menu_on_selected {
            //                                 Stack::new()
            //                                     .with_child(channel_row)
            //                                     .with_child(
            //                                         ChildView::new(&this.context_menu, cx)
            //                                             .aligned()
            //                                             .bottom()
            //                                             .right(),
            //                                     )
            //                                     .into_any()
            //                             } else {
            //                                 return channel_row;
            //                             }
            //                         }
            //                         ListEntry::ChannelNotes { channel_id } => this.render_channel_notes(
            //                             *channel_id,
            //                             &theme.collab_panel,
            //                             is_selected,
            //                             ix,
            //                             cx,
            //                         ),
            //                         ListEntry::ChannelChat { channel_id } => this.render_channel_chat(
            //                             *channel_id,
            //                             &theme.collab_panel,
            //                             is_selected,
            //                             ix,
            //                             cx,
            //                         ),
            //                         ListEntry::ChannelInvite(channel) => Self::render_channel_invite(
            //                             channel.clone(),
            //                             this.channel_store.clone(),
            //                             &theme.collab_panel,
            //                             is_selected,
            //                             cx,
            //                         ),
            //                         ListEntry::IncomingRequest(user) => Self::render_contact_request(
            //                             user.clone(),
            //                             this.user_store.clone(),
            //                             &theme.collab_panel,
            //                             true,
            //                             is_selected,
            //                             cx,
            //                         ),
            //                         ListEntry::OutgoingRequest(user) => Self::render_contact_request(
            //                             user.clone(),
            //                             this.user_store.clone(),
            //                             &theme.collab_panel,
            //                             false,
            //                             is_selected,
            //                             cx,
            //                         ),
            //                         ListEntry::Contact { contact, calling } => Self::render_contact(
            //                             contact,
            //                             *calling,
            //                             &this.project,
            //                             &theme,
            //                             is_selected,
            //                             cx,
            //                         ),
            //                         ListEntry::ChannelEditor { depth } => {
            //                             this.render_channel_editor(&theme, *depth, cx)
            //                         }
            //                         ListEntry::ContactPlaceholder => {
            //                             this.render_contact_placeholder(&theme.collab_panel, is_selected, cx)
            //                         }
            //                     }
            //                 });

            let mut this = Self {
                width: None,
                focus_handle: cx.focus_handle(),
                channel_clipboard: None,
                fs: workspace.app_state().fs.clone(),
                pending_serialization: Task::ready(None),
                context_menu: None,
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
                scroll_handle: ScrollHandle::new(),
                collapsed_sections: vec![Section::Offline],
                collapsed_channels: Vec::default(),
                workspace: workspace.weak_handle(),
                client: workspace.app_state().client.clone(),
                //                 context_menu_on_selected: true,
                drag_target_channel: ChannelDragTarget::None,
            };

            this.update_entries(false, cx);

            // Update the dock position when the setting changes.
            let mut old_dock_position = this.position(cx);
            this.subscriptions.push(cx.observe_global::<SettingsStore>(
                move |this: &mut Self, cx| {
                    let new_dock_position = this.position(cx);
                    if new_dock_position != old_dock_position {
                        old_dock_position = new_dock_position;
                        cx.emit(PanelEvent::ChangePosition);
                    }
                    cx.notify();
                },
            ));

            let active_call = ActiveCall::global(cx);
            this.subscriptions
                .push(cx.observe(&this.user_store, |this, _, cx| {
                    this.update_entries(true, cx)
                }));
            this.subscriptions
                .push(cx.observe(&this.channel_store, |this, _, cx| {
                    this.update_entries(true, cx)
                }));
            this.subscriptions
                .push(cx.observe(&active_call, |this, _, cx| this.update_entries(true, cx)));
            this.subscriptions
                .push(cx.observe_flag::<ChannelsAlpha, _>(move |_, this, cx| {
                    this.update_entries(true, cx)
                }));
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

    fn contacts(&self, cx: &AppContext) -> Option<Vec<Arc<Contact>>> {
        Some(self.user_store.read(cx).contacts().to_owned())
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
                    panel.width = serialized_panel.width;
                    panel.collapsed_channels = serialized_panel
                        .collapsed_channels
                        .unwrap_or_else(|| Vec::new());
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
                            collapsed_channels: Some(collapsed_channels),
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
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

                if let Some(channel_id) = room.channel_id() {
                    self.entries.push(ListEntry::ChannelNotes { channel_id });
                    self.entries.push(ListEntry::ChannelChat { channel_id })
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
                    self.entries.push(ListEntry::CallParticipant {
                        user: participant.user.clone(),
                        peer_id: Some(participant.peer_id),
                        is_pending: false,
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
                    }));
            }
        }

        let mut request_entries = Vec::new();

        if cx.has_flag::<ChannelsAlpha>() {
            self.entries.push(ListEntry::Header(Section::Channels));

            if channel_store.channel_count() > 0 || self.channel_editing_state.is_some() {
                self.match_candidates.clear();
                self.match_candidates
                    .extend(channel_store.ordered_channels().enumerate().map(
                        |(ix, (_, channel))| StringMatchCandidate {
                            id: ix,
                            string: channel.name.clone().into(),
                            char_bag: channel.name.chars().collect(),
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
                }
            }

            //             let channel_invites = channel_store.channel_invitations();
            //             if !channel_invites.is_empty() {
            //                 self.match_candidates.clear();
            //                 self.match_candidates
            //                     .extend(channel_invites.iter().enumerate().map(|(ix, channel)| {
            //                         StringMatchCandidate {
            //                             id: ix,
            //                             string: channel.name.clone(),
            //                             char_bag: channel.name.chars().collect(),
            //                         }
            //                     }));
            //                 let matches = executor.block(match_strings(
            //                     &self.match_candidates,
            //                     &query,
            //                     true,
            //                     usize::MAX,
            //                     &Default::default(),
            //                     executor.clone(),
            //                 ));
            //                 request_entries.extend(matches.iter().map(|mat| {
            //                     ListEntry::ChannelInvite(channel_invites[mat.candidate_id].clone())
            //                 }));

            //                 if !request_entries.is_empty() {
            //                     self.entries
            //                         .push(ListEntry::Header(Section::ChannelInvites));
            //                     if !self.collapsed_sections.contains(&Section::ChannelInvites) {
            //                         self.entries.append(&mut request_entries);
            //                     }
            //                 }
            //             }
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
                        self.scroll_handle.scroll_to_item(ix);
                        break;
                    }
                }
            }
        } else {
            self.selection = self.selection.and_then(|prev_selection| {
                if self.entries.is_empty() {
                    None
                } else {
                    let ix = prev_selection.min(self.entries.len() - 1);
                    self.scroll_handle.scroll_to_item(ix);
                    Some(ix)
                }
            });
        }

        if scroll_to_top {
            self.scroll_handle.scroll_to_item(0)
        } else {
            let (old_index, old_offset) = self.scroll_handle.logical_scroll_top();
            // Attempt to maintain the same scroll position.
            if let Some(old_top_entry) = old_entries.get(old_index) {
                let (new_index, new_offset) = self
                    .entries
                    .iter()
                    .position(|entry| entry == old_top_entry)
                    .map(|item_ix| (item_ix, old_offset))
                    .or_else(|| {
                        let entry_after_old_top = old_entries.get(old_index + 1)?;
                        let item_ix = self
                            .entries
                            .iter()
                            .position(|entry| entry == entry_after_old_top)?;
                        Some((item_ix, px(0.)))
                    })
                    .or_else(|| {
                        let entry_before_old_top = old_entries.get(old_index.saturating_sub(1))?;
                        let item_ix = self
                            .entries
                            .iter()
                            .position(|entry| entry == entry_before_old_top)?;
                        Some((item_ix, px(0.)))
                    })
                    .unwrap_or_else(|| (old_index, old_offset));

                self.scroll_handle
                    .set_logical_scroll_top(new_index, new_offset);
            }
        }

        cx.notify();
    }

    fn render_call_participant(
        &self,
        user: Arc<User>,
        peer_id: Option<PeerId>,
        is_pending: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let is_current_user =
            self.user_store.read(cx).current_user().map(|user| user.id) == Some(user.id);
        let tooltip = format!("Follow {}", user.github_login);

        ListItem::new(SharedString::from(user.github_login.clone()))
            .left_child(Avatar::data(user.avatar.clone().unwrap()))
            .child(
                h_stack()
                    .w_full()
                    .justify_between()
                    .child(Label::new(user.github_login.clone()))
                    .child(if is_pending {
                        Label::new("Calling").color(Color::Muted).into_any_element()
                    } else if is_current_user {
                        IconButton::new("leave-call", Icon::ArrowRight)
                            .on_click(cx.listener(move |this, _, cx| {
                                Self::leave_call(cx);
                            }))
                            .tooltip(|cx| Tooltip::text("Leave Call", cx))
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }),
            )
            .when_some(peer_id, |this, peer_id| {
                this.tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                    .on_click(cx.listener(move |this, _, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| workspace.follow(peer_id, cx));
                    }))
            })
    }

    fn render_participant_project(
        &self,
        project_id: u64,
        worktree_root_names: &[String],
        host_user_id: u64,
        //         is_current: bool,
        is_last: bool,
        //         is_selected: bool,
        //         theme: &theme::Theme,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let project_name: SharedString = if worktree_root_names.is_empty() {
            "untitled".to_string()
        } else {
            worktree_root_names.join(", ")
        }
        .into();

        let theme = cx.theme();

        ListItem::new(project_id as usize)
            .on_click(cx.listener(move |this, _, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let app_state = workspace.app_state().clone();
                    workspace::join_remote_project(project_id, host_user_id, app_state, cx)
                        .detach_and_log_err(cx);
                });
            }))
            .left_child(render_tree_branch(is_last, cx))
            .child(IconButton::new(0, Icon::Folder))
            .child(Label::new(project_name.clone()))
            .tooltip(move |cx| Tooltip::text(format!("Open {}", project_name), cx))

        //         enum JoinProject {}
        //         enum JoinProjectTooltip {}

        //         let collab_theme = &theme.collab_panel;
        //         let host_avatar_width = collab_theme
        //             .contact_avatar
        //             .width
        //             .or(collab_theme.contact_avatar.height)
        //             .unwrap_or(0.);
        //         let tree_branch = collab_theme.tree_branch;

        //         let content =
        //             MouseEventHandler::new::<JoinProject, _>(project_id as usize, cx, |mouse_state, cx| {
        //                 let tree_branch = *tree_branch.in_state(is_selected).style_for(mouse_state);
        //                 let row = if is_current {
        //                     collab_theme
        //                         .project_row
        //                         .in_state(true)
        //                         .style_for(&mut Default::default())
        //                 } else {
        //                     collab_theme
        //                         .project_row
        //                         .in_state(is_selected)
        //                         .style_for(mouse_state)
        //                 };

        //                 Flex::row()
        //                     .with_child(render_tree_branch(
        //                         tree_branch,
        //                         &row.name.text,
        //                         is_last,
        //                         vec2f(host_avatar_width, collab_theme.row_height),
        //                         cx.font_cache(),
        //                     ))
        //                     .with_child(
        //                         Svg::new("icons/file_icons/folder.svg")
        //                             .with_color(collab_theme.channel_hash.color)
        //                             .constrained()
        //                             .with_width(collab_theme.channel_hash.width)
        //                             .aligned()
        //                             .left(),
        //                     )
        //                     .with_child(
        //                         Label::new(project_name.clone(), row.name.text.clone())
        //                             .aligned()
        //                             .left()
        //                             .contained()
        //                             .with_style(row.name.container)
        //                             .flex(1., false),
        //                     )
        //                     .constrained()
        //                     .with_height(collab_theme.row_height)
        //                     .contained()
        //                     .with_style(row.container)
        //             });

        //         if is_current {
        //             return content.into_any();
        //         }

        //         content
        //             .with_cursor_style(CursorStyle::PointingHand)
        //             .on_click(MouseButton::Left, move |_, this, cx| {
        //                 if let Some(workspace) = this.workspace.upgrade(cx) {
        //                     let app_state = workspace.read(cx).app_state().clone();
        //                     workspace::join_remote_project(project_id, host_user_id, app_state, cx)
        //                         .detach_and_log_err(cx);
        //                 }
        //             })
        //             .with_tooltip::<JoinProjectTooltip>(
        //                 project_id as usize,
        //                 format!("Open {}", project_name),
        //                 None,
        //                 theme.tooltip.clone(),
        //                 cx,
        //             )
        //             .into_any()
    }

    fn render_participant_screen(
        &self,
        peer_id: Option<PeerId>,
        is_last: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let id = peer_id.map_or(usize::MAX, |id| id.as_u64() as usize);

        ListItem::new(("screen", id))
            .left_child(render_tree_branch(is_last, cx))
            .child(IconButton::new(0, Icon::Screen))
            .child(Label::new("Screen"))
            .when_some(peer_id, |this, _| {
                this.on_click(cx.listener(move |this, _, cx| {
                    this.workspace.update(cx, |workspace, cx| {
                        workspace.open_shared_screen(peer_id.unwrap(), cx)
                    });
                }))
                .tooltip(move |cx| Tooltip::text(format!("Open shared screen"), cx))
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

    //     fn render_contact_placeholder(
    //         &self,
    //         theme: &theme::CollabPanel,
    //         is_selected: bool,
    //         cx: &mut ViewContext<Self>,
    //     ) -> AnyElement<Self> {
    //         enum AddContacts {}
    //         MouseEventHandler::new::<AddContacts, _>(0, cx, |state, _| {
    //             let style = theme.list_empty_state.style_for(is_selected, state);
    //             Flex::row()
    //                 .with_child(
    //                     Svg::new("icons/plus.svg")
    //                         .with_color(theme.list_empty_icon.color)
    //                         .constrained()
    //                         .with_width(theme.list_empty_icon.width)
    //                         .aligned()
    //                         .left(),
    //                 )
    //                 .with_child(
    //                     Label::new("Add a contact", style.text.clone())
    //                         .contained()
    //                         .with_style(theme.list_empty_label_container),
    //                 )
    //                 .align_children_center()
    //                 .contained()
    //                 .with_style(style.container)
    //                 .into_any()
    //         })
    //         .on_click(MouseButton::Left, |_, this, cx| {
    //             this.toggle_contact_finder(cx);
    //         })
    //         .into_any()
    //     }

    fn render_channel_notes(
        &self,
        channel_id: ChannelId,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new("channel-notes")
            .on_click(cx.listener(move |this, _, cx| {
                this.open_channel_notes(channel_id, cx);
            }))
            .left_child(render_tree_branch(false, cx))
            .child(IconButton::new(0, Icon::File))
            .child(Label::new("notes"))
            .tooltip(move |cx| Tooltip::text("Open Channel Notes", cx))
    }

    fn render_channel_chat(
        &self,
        channel_id: ChannelId,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new("channel-chat")
            .on_click(cx.listener(move |this, _, cx| {
                this.join_channel_chat(channel_id, cx);
            }))
            .left_child(render_tree_branch(true, cx))
            .child(IconButton::new(0, Icon::MessageBubbles))
            .child(Label::new("chat"))
            .tooltip(move |cx| Tooltip::text("Open Chat", cx))
    }

    //     fn render_channel_invite(
    //         channel: Arc<Channel>,
    //         channel_store: ModelHandle<ChannelStore>,
    //         theme: &theme::CollabPanel,
    //         is_selected: bool,
    //         cx: &mut ViewContext<Self>,
    //     ) -> AnyElement<Self> {
    //         enum Decline {}
    //         enum Accept {}

    //         let channel_id = channel.id;
    //         let is_invite_pending = channel_store
    //             .read(cx)
    //             .has_pending_channel_invite_response(&channel);
    //         let button_spacing = theme.contact_button_spacing;

    //         Flex::row()
    //             .with_child(
    //                 Svg::new("icons/hash.svg")
    //                     .with_color(theme.channel_hash.color)
    //                     .constrained()
    //                     .with_width(theme.channel_hash.width)
    //                     .aligned()
    //                     .left(),
    //             )
    //             .with_child(
    //                 Label::new(channel.name.clone(), theme.contact_username.text.clone())
    //                     .contained()
    //                     .with_style(theme.contact_username.container)
    //                     .aligned()
    //                     .left()
    //                     .flex(1., true),
    //             )
    //             .with_child(
    //                 MouseEventHandler::new::<Decline, _>(channel.id as usize, cx, |mouse_state, _| {
    //                     let button_style = if is_invite_pending {
    //                         &theme.disabled_button
    //                     } else {
    //                         theme.contact_button.style_for(mouse_state)
    //                     };
    //                     render_icon_button(button_style, "icons/x.svg").aligned()
    //                 })
    //                 .with_cursor_style(CursorStyle::PointingHand)
    //                 .on_click(MouseButton::Left, move |_, this, cx| {
    //                     this.respond_to_channel_invite(channel_id, false, cx);
    //                 })
    //                 .contained()
    //                 .with_margin_right(button_spacing),
    //             )
    //             .with_child(
    //                 MouseEventHandler::new::<Accept, _>(channel.id as usize, cx, |mouse_state, _| {
    //                     let button_style = if is_invite_pending {
    //                         &theme.disabled_button
    //                     } else {
    //                         theme.contact_button.style_for(mouse_state)
    //                     };
    //                     render_icon_button(button_style, "icons/check.svg")
    //                         .aligned()
    //                         .flex_float()
    //                 })
    //                 .with_cursor_style(CursorStyle::PointingHand)
    //                 .on_click(MouseButton::Left, move |_, this, cx| {
    //                     this.respond_to_channel_invite(channel_id, true, cx);
    //                 }),
    //             )
    //             .constrained()
    //             .with_height(theme.row_height)
    //             .contained()
    //             .with_style(
    //                 *theme
    //                     .contact_row
    //                     .in_state(is_selected)
    //                     .style_for(&mut Default::default()),
    //             )
    //             .with_padding_left(
    //                 theme.contact_row.default_style().padding.left + theme.channel_indent,
    //             )
    //             .into_any()
    //     }

    fn has_subchannels(&self, ix: usize) -> bool {
        self.entries.get(ix).map_or(false, |entry| {
            if let ListEntry::Channel { has_children, .. } = entry {
                *has_children
            } else {
                false
            }
        })
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
                    cx.handler_for(&this, move |this, cx| {
                        this.toggle_channel_collapsed(channel_id, cx)
                    }),
                );
            }

            context_menu = context_menu
                .entry(
                    "Open Notes",
                    cx.handler_for(&this, move |this, cx| {
                        this.open_channel_notes(channel_id, cx)
                    }),
                )
                .entry(
                    "Open Chat",
                    cx.handler_for(&this, move |this, cx| {
                        this.join_channel_chat(channel_id, cx)
                    }),
                )
                .entry(
                    "Copy Channel Link",
                    cx.handler_for(&this, move |this, cx| {
                        this.copy_channel_link(channel_id, cx)
                    }),
                );

            if self.channel_store.read(cx).is_channel_admin(channel_id) {
                context_menu = context_menu
                    .separator()
                    .entry(
                        "New Subchannel",
                        cx.handler_for(&this, move |this, cx| this.new_subchannel(channel_id, cx)),
                    )
                    .entry(
                        "Rename",
                        cx.handler_for(&this, move |this, cx| this.rename_channel(channel_id, cx)),
                    )
                    .entry(
                        "Move this channel",
                        cx.handler_for(&this, move |this, cx| {
                            this.start_move_channel(channel_id, cx)
                        }),
                    );

                if let Some(channel_name) = clipboard_channel_name {
                    context_menu = context_menu.separator().entry(
                        format!("Move '#{}' here", channel_name),
                        cx.handler_for(&this, move |this, cx| {
                            this.move_channel_on_clipboard(channel_id, cx)
                        }),
                    );
                }

                context_menu = context_menu
                    .separator()
                    .entry(
                        "Invite Members",
                        cx.handler_for(&this, move |this, cx| this.invite_members(channel_id, cx)),
                    )
                    .entry(
                        "Manage Members",
                        cx.handler_for(&this, move |this, cx| this.manage_members(channel_id, cx)),
                    )
                    .entry(
                        "Delete",
                        cx.handler_for(&this, move |this, cx| this.remove_channel(channel_id, cx)),
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

        self.update_entries(false, cx);
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let ix = self.selection.map_or(0, |ix| ix + 1);
        if ix < self.entries.len() {
            self.selection = Some(ix);
        }

        if let Some(ix) = self.selection {
            self.scroll_handle.scroll_to_item(ix)
        }
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        let ix = self.selection.take().unwrap_or(0);
        if ix > 0 {
            self.selection = Some(ix - 1);
        }

        if let Some(ix) = self.selection {
            self.scroll_handle.scroll_to_item(ix)
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
                    // ListEntry::ParticipantProject {
                    //     project_id,
                    //     host_user_id,
                    //     ..
                    // } => {
                    //     if let Some(workspace) = self.workspace.upgrade(cx) {
                    //         let app_state = workspace.read(cx).app_state().clone();
                    //         workspace::join_remote_project(
                    //             *project_id,
                    //             *host_user_id,
                    //             app_state,
                    //             cx,
                    //         )
                    //         .detach_and_log_err(cx);
                    //     }
                    // }
                    // ListEntry::ParticipantScreen { peer_id, .. } => {
                    //     let Some(peer_id) = peer_id else {
                    //         return;
                    //     };
                    //     if let Some(workspace) = self.workspace.upgrade(cx) {
                    //         workspace.update(cx, |workspace, cx| {
                    //             workspace.open_shared_screen(*peer_id, cx)
                    //         });
                    //     }
                    // }
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
                    _ => {}
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

                    self.channel_store
                        .update(cx, |channel_store, cx| {
                            channel_store.create_channel(&channel_name, *location, cx)
                        })
                        .detach();
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

    //     fn toggle_channel_collapsed_action(
    //         &mut self,
    //         action: &ToggleCollapse,
    //         cx: &mut ViewContext<Self>,
    //     ) {
    //         self.toggle_channel_collapsed(action.location, cx);
    //     }

    fn toggle_channel_collapsed<'a>(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
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

    fn leave_call(cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.hang_up(cx))
            .detach_and_log_err(cx);
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

    fn invite_members(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        self.show_channel_modal(channel_id, channel_modal::Mode::InviteMembers, cx);
    }

    fn manage_members(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        self.show_channel_modal(channel_id, channel_modal::Mode::ManageMembers, cx);
    }

    fn remove_selected_channel(&mut self, _: &Remove, cx: &mut ViewContext<Self>) {
        if let Some(channel) = self.selected_channel() {
            self.remove_channel(channel.id, cx)
        }
    }

    fn rename_selected_channel(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
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

    fn start_move_channel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        self.channel_clipboard = Some(ChannelMoveClipboard { channel_id });
    }

    fn start_move_selected_channel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        if let Some(channel) = self.selected_channel() {
            self.channel_clipboard = Some(ChannelMoveClipboard {
                channel_id: channel.id,
            })
        }
    }

    fn move_channel_on_clipboard(
        &mut self,
        to_channel_id: ChannelId,
        cx: &mut ViewContext<CollabPanel>,
    ) {
        if let Some(clipboard) = self.channel_clipboard.take() {
            self.channel_store.update(cx, |channel_store, cx| {
                channel_store
                    .move_channel(clipboard.channel_id, Some(to_channel_id), cx)
                    .detach_and_log_err(cx)
            })
        }
    }

    fn open_channel_notes(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            ChannelView::open(channel_id, workspace, cx).detach();
        }
    }

    fn show_inline_context_menu(&mut self, _: &menu::ShowContextMenu, cx: &mut ViewContext<Self>) {
        let Some(channel) = self.selected_channel() else {
            return;
        };
        let Some(bounds) = self
            .selection
            .and_then(|ix| self.scroll_handle.bounds_for_item(ix))
        else {
            return;
        };

        self.deploy_channel_context_menu(bounds.center(), channel.id, self.selection.unwrap(), cx);
        cx.stop_propagation();
    }

    fn selected_channel(&self) -> Option<&Arc<Channel>> {
        self.selection
            .and_then(|ix| self.entries.get(ix))
            .and_then(|entry| match entry {
                ListEntry::Channel { channel, .. } => Some(channel),
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
        let members = self.channel_store.update(cx, |channel_store, cx| {
            channel_store.get_channel_member_details(channel_id, cx)
        });

        cx.spawn(|_, mut cx| async move {
            let members = members.await?;
            workspace.update(&mut cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    ChannelModal::new(
                        user_store.clone(),
                        channel_store.clone(),
                        channel_id,
                        mode,
                        members,
                        cx,
                    )
                });
            })
        })
        .detach();
    }

    //     fn remove_selected_channel(&mut self, action: &RemoveChannel, cx: &mut ViewContext<Self>) {
    //         self.remove_channel(action.channel_id, cx)
    //     }

    fn remove_channel(&mut self, channel_id: ChannelId, cx: &mut ViewContext<Self>) {
        let channel_store = self.channel_store.clone();
        if let Some(channel) = channel_store.read(cx).channel_for_id(channel_id) {
            let prompt_message = format!(
                "Are you sure you want to remove the channel \"{}\"?",
                channel.name
            );
            let mut answer =
                cx.prompt(PromptLevel::Warning, &prompt_message, &["Remove", "Cancel"]);
            let window = cx.window();
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

    //     // Should move to the filter editor if clicking on it
    //     // Should move selection to the channel editor if activating it

    fn remove_contact(&mut self, user_id: u64, github_login: &str, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.clone();
        let prompt_message = format!(
            "Are you sure you want to remove \"{}\" from your contacts?",
            github_login
        );
        let mut answer = cx.prompt(PromptLevel::Warning, &prompt_message, &["Remove", "Cancel"]);
        let window = cx.window();
        cx.spawn(|_, mut cx| async move {
            if answer.await? == 0 {
                user_store
                    .update(&mut cx, |store, cx| store.remove_contact(user_id, cx))?
                    .await
                    .notify_async_err(&mut cx);
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
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
            .detach_and_log_err(cx);
    }

    //     fn respond_to_channel_invite(
    //         &mut self,
    //         channel_id: u64,
    //         accept: bool,
    //         cx: &mut ViewContext<Self>,
    //     ) {
    //         self.channel_store
    //             .update(cx, |store, cx| {
    //                 store.respond_to_channel_invite(channel_id, accept, cx)
    //             })
    //             .detach();
    //     }

    fn call(&mut self, recipient_user_id: u64, cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |call, cx| {
                call.invite(recipient_user_id, Some(self.project.clone()), cx)
            })
            .detach_and_log_err(cx);
    }

    fn join_channel(&self, channel_id: u64, cx: &mut ViewContext<Self>) {
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
        .detach_and_log_err(cx)
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
                            .detach_and_log_err(cx);
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
        let item = ClipboardItem::new(channel.link());
        cx.write_to_clipboard(item)
    }

    fn render_signed_out(&mut self, cx: &mut ViewContext<Self>) -> Div {
        v_stack().child(
            Button::new("sign_in", "Sign in to collaborate").on_click(cx.listener(
                |this, _, cx| {
                    let client = this.client.clone();
                    cx.spawn(|_, mut cx| async move {
                        client
                            .authenticate_and_connect(true, &cx)
                            .await
                            .notify_async_err(&mut cx);
                    })
                    .detach()
                },
            )),
        )
    }

    fn render_signed_in(&mut self, cx: &mut ViewContext<Self>) -> Div {
        v_stack()
            .size_full()
            .child(
                div()
                    .p_2()
                    .child(div().rounded(px(2.0)).child(self.filter_editor.clone())),
            )
            .child(
                v_stack()
                    .size_full()
                    .id("scroll")
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(
                        self.entries
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(ix, entry)| {
                                let is_selected = self.selection == Some(ix);
                                match entry {
                                    ListEntry::Header(section) => {
                                        let is_collapsed =
                                            self.collapsed_sections.contains(&section);
                                        self.render_header(section, is_selected, is_collapsed, cx)
                                            .into_any_element()
                                    }
                                    ListEntry::Contact { contact, calling } => self
                                        .render_contact(&*contact, calling, is_selected, cx)
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
                                        .render_channel(
                                            &*channel,
                                            depth,
                                            has_children,
                                            is_selected,
                                            ix,
                                            cx,
                                        )
                                        .into_any_element(),
                                    ListEntry::ChannelEditor { depth } => {
                                        self.render_channel_editor(depth, cx).into_any_element()
                                    }
                                    ListEntry::CallParticipant {
                                        user,
                                        peer_id,
                                        is_pending,
                                    } => self
                                        .render_call_participant(user, peer_id, is_pending, cx)
                                        .into_any_element(),
                                    ListEntry::ParticipantProject {
                                        project_id,
                                        worktree_root_names,
                                        host_user_id,
                                        is_last,
                                    } => self
                                        .render_participant_project(
                                            project_id,
                                            &worktree_root_names,
                                            host_user_id,
                                            is_last,
                                            cx,
                                        )
                                        .into_any_element(),
                                    ListEntry::ParticipantScreen { peer_id, is_last } => self
                                        .render_participant_screen(peer_id, is_last, cx)
                                        .into_any_element(),
                                    ListEntry::ChannelNotes { channel_id } => {
                                        self.render_channel_notes(channel_id, cx).into_any_element()
                                    }
                                    ListEntry::ChannelChat { channel_id } => {
                                        self.render_channel_chat(channel_id, cx).into_any_element()
                                    }
                                }
                            }),
                    ),
            )
    }

    fn render_header(
        &mut self,
        section: Section,
        is_selected: bool,
        is_collapsed: bool,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let mut channel_link = None;
        let mut channel_tooltip_text = None;
        let mut channel_icon = None;
        // let mut is_dragged_over = false;

        let text = match section {
            Section::ActiveCall => {
                let channel_name = maybe!({
                    let channel_id = ActiveCall::global(cx).read(cx).channel_id(cx)?;

                    let channel = self.channel_store.read(cx).channel_for_id(channel_id)?;

                    channel_link = Some(channel.link());
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
                    SharedString::from(format!("{}", name))
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
                IconButton::new("channel-link", Icon::Check)
                    .on_click(move |_, cx| {
                        let item = ClipboardItem::new(channel_link_copy.clone());
                        cx.write_to_clipboard(item)
                    })
                    .tooltip(|cx| Tooltip::text("Copy channel link", cx))
            }),
            Section::Contacts => Some(
                IconButton::new("add-contact", Icon::Plus)
                    .on_click(cx.listener(|this, _, cx| this.toggle_contact_finder(cx)))
                    .tooltip(|cx| Tooltip::text("Search for new contact", cx)),
            ),
            Section::Channels => Some(
                IconButton::new("add-channel", Icon::Plus)
                    .on_click(cx.listener(|this, _, cx| this.new_root_channel(cx)))
                    .tooltip(|cx| Tooltip::text("Create a channel", cx)),
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

        h_stack()
            .w_full()
            .map(|el| {
                if can_collapse {
                    el.child(
                        ListItem::new(text.clone())
                            .child(div().w_full().child(Label::new(text)))
                            .selected(is_selected)
                            .toggle(Some(!is_collapsed))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.toggle_section_expanded(section, cx)
                            })),
                    )
                } else {
                    el.child(
                        ListHeader::new(text)
                            .when_some(button, |el, button| el.meta(button))
                            .selected(is_selected),
                    )
                }
            })
            .when(section == Section::Channels, |el| {
                el.drag_over::<DraggedChannelView>(|style| {
                    style.bg(cx.theme().colors().ghost_element_hover)
                })
                .on_drop(cx.listener(
                    move |this, view: &View<DraggedChannelView>, cx| {
                        this.channel_store
                            .update(cx, |channel_store, cx| {
                                channel_store.move_channel(view.read(cx).channel.id, None, cx)
                            })
                            .detach_and_log_err(cx)
                    },
                ))
            })
    }

    fn render_contact(
        &mut self,
        contact: &Contact,
        calling: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        enum ContactTooltip {}

        let online = contact.online;
        let busy = contact.busy || calling;
        let user_id = contact.user.id;
        let github_login = SharedString::from(contact.user.github_login.clone());
        let mut item = ListItem::new(github_login.clone())
            .on_click(cx.listener(move |this, _, cx| this.call(user_id, cx)))
            .child(
                h_stack()
                    .w_full()
                    .justify_between()
                    .child(Label::new(github_login.clone()))
                    .when(calling, |el| {
                        el.child(Label::new("Calling").color(Color::Muted))
                    })
                    .when(!calling, |el| {
                        el.child(
                            div()
                                .id("remove_contact")
                                .invisible()
                                .group_hover("", |style| style.visible())
                                .child(
                                    IconButton::new("remove_contact", Icon::Close)
                                        .icon_color(Color::Muted)
                                        .tooltip(|cx| Tooltip::text("Remove Contact", cx))
                                        .on_click(cx.listener({
                                            let github_login = github_login.clone();
                                            move |this, _, cx| {
                                                this.remove_contact(user_id, &github_login, cx);
                                            }
                                        })),
                                ),
                        )
                    }),
            )
            .left_child(
                // todo!() handle contacts with no avatar
                Avatar::data(contact.user.avatar.clone().unwrap())
                    .availability_indicator(if online { Some(!busy) } else { None }),
            )
            .when(online && !busy, |el| {
                el.on_click(cx.listener(move |this, _, cx| this.call(user_id, cx)))
            });

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
        &mut self,
        user: Arc<User>,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let github_login = SharedString::from(user.github_login.clone());
        let user_id = user.id;
        let is_contact_request_pending = self.user_store.read(cx).is_contact_request_pending(&user);
        let color = if is_contact_request_pending {
            Color::Muted
        } else {
            Color::Default
        };

        let controls = if is_incoming {
            vec![
                IconButton::new("remove_contact", Icon::Close)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.respond_to_contact_request(user_id, false, cx);
                    }))
                    .icon_color(color)
                    .tooltip(|cx| Tooltip::text("Decline invite", cx)),
                IconButton::new("remove_contact", Icon::Check)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.respond_to_contact_request(user_id, true, cx);
                    }))
                    .icon_color(color)
                    .tooltip(|cx| Tooltip::text("Accept invite", cx)),
            ]
        } else {
            let github_login = github_login.clone();
            vec![IconButton::new("remove_contact", Icon::Close)
                .on_click(cx.listener(move |this, _, cx| {
                    this.remove_contact(user_id, &github_login, cx);
                }))
                .icon_color(color)
                .tooltip(|cx| Tooltip::text("Cancel invite", cx))]
        };

        ListItem::new(github_login.clone())
            .child(
                h_stack()
                    .w_full()
                    .justify_between()
                    .child(Label::new(github_login.clone()))
                    .child(h_stack().children(controls)),
            )
            .when_some(user.avatar.clone(), |el, avatar| el.left_avatar(avatar))
    }

    fn render_contact_placeholder(
        &self,
        is_selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new("contact-placeholder")
            .child(IconElement::new(Icon::Plus))
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
        let is_public = self
            .channel_store
            .read(cx)
            .channel_for_id(channel_id)
            .map(|channel| channel.visibility)
            == Some(proto::ChannelVisibility::Public);
        let other_selected = self.selected_channel().map(|channel| channel.id) == Some(channel.id);
        let disclosed =
            has_children.then(|| !self.collapsed_channels.binary_search(&channel.id).is_ok());

        let has_messages_notification = channel.unseen_message_id.is_some();
        let has_notes_notification = channel.unseen_note_version.is_some();

        const FACEPILE_LIMIT: usize = 3;
        let participants = self.channel_store.read(cx).channel_participants(channel_id);

        let face_pile = if !participants.is_empty() {
            let extra_count = participants.len().saturating_sub(FACEPILE_LIMIT);
            let user = &participants[0];

            let result = FacePile {
                faces: participants
                    .iter()
                    .filter_map(|user| Some(Avatar::data(user.avatar.clone()?).into_any_element()))
                    .take(FACEPILE_LIMIT)
                    .chain(if extra_count > 0 {
                        // todo!() @nate - this label looks wrong.
                        Some(Label::new(format!("+{}", extra_count)).into_any_element())
                    } else {
                        None
                    })
                    .collect::<Vec<_>>(),
            };

            Some(result)
        } else {
            None
        };

        let width = self.width.unwrap_or(px(240.));

        div()
            .id(channel_id as usize)
            .group("")
            .on_drag({
                let channel = channel.clone();
                move |cx| {
                    let channel = channel.clone();
                    cx.build_view({ |cx| DraggedChannelView { channel, width } })
                }
            })
            .drag_over::<DraggedChannelView>(|style| {
                style.bg(cx.theme().colors().ghost_element_hover)
            })
            .on_drop(
                cx.listener(move |this, view: &View<DraggedChannelView>, cx| {
                    this.channel_store
                        .update(cx, |channel_store, cx| {
                            channel_store.move_channel(
                                view.read(cx).channel.id,
                                Some(channel_id),
                                cx,
                            )
                        })
                        .detach_and_log_err(cx)
                }),
            )
            .child(
                ListItem::new(channel_id as usize)
                    .indent_level(depth)
                    .indent_step_size(cx.rem_size() * 14.0 / 16.0) // @todo()! @nate this is to  step over the disclosure toggle
                    .left_icon(if is_public { Icon::Public } else { Icon::Hash })
                    .selected(is_selected || is_active)
                    .child(
                        h_stack()
                            .w_full()
                            .justify_between()
                            .child(
                                h_stack()
                                    .id(channel_id as usize)
                                    .child(Label::new(channel.name.clone()))
                                    .children(face_pile.map(|face_pile| face_pile.render(cx))),
                            )
                            .child(
                                h_stack()
                                    .child(
                                        div()
                                            .id("channel_chat")
                                            .when(!has_messages_notification, |el| el.invisible())
                                            .group_hover("", |style| style.visible())
                                            .child(
                                                IconButton::new(
                                                    "channel_chat",
                                                    Icon::MessageBubbles,
                                                )
                                                .icon_color(if has_messages_notification {
                                                    Color::Default
                                                } else {
                                                    Color::Muted
                                                })
                                                .on_click(cx.listener(move |this, _, cx| {
                                                    this.join_channel_chat(channel_id, cx)
                                                }))
                                                .tooltip(|cx| {
                                                    Tooltip::text("Open channel chat", cx)
                                                }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .id("channel_notes")
                                            .when(!has_notes_notification, |el| el.invisible())
                                            .group_hover("", |style| style.visible())
                                            .child(
                                                IconButton::new("channel_notes", Icon::File)
                                                    .icon_color(if has_notes_notification {
                                                        Color::Default
                                                    } else {
                                                        Color::Muted
                                                    })
                                                    .on_click(cx.listener(move |this, _, cx| {
                                                        this.open_channel_notes(channel_id, cx)
                                                    }))
                                                    .tooltip(|cx| {
                                                        Tooltip::text("Open channel notes", cx)
                                                    }),
                                            ),
                                    ),
                            ),
                    )
                    .toggle(disclosed)
                    .on_toggle(
                        cx.listener(move |this, _, cx| {
                            this.toggle_channel_collapsed(channel_id, cx)
                        }),
                    )
                    .on_click(cx.listener(move |this, _, cx| {
                        if this.drag_target_channel == ChannelDragTarget::None {
                            if is_active {
                                this.open_channel_notes(channel_id, cx)
                            } else {
                                this.join_channel(channel_id, cx)
                            }
                        }
                    }))
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, cx| {
                            this.deploy_channel_context_menu(event.position, channel_id, ix, cx)
                        },
                    )),
            )
            .tooltip(|cx| Tooltip::text("Join channel", cx))

        // let channel_id = channel.id;
        // let collab_theme = &theme.collab_panel;
        // let is_public = self
        //     .channel_store
        //     .read(cx)
        //     .channel_for_id(channel_id)
        //     .map(|channel| channel.visibility)
        //     == Some(proto::ChannelVisibility::Public);
        // let other_selected = self.selected_channel().map(|channel| channel.id) == Some(channel.id);
        // let disclosed =
        //     has_children.then(|| !self.collapsed_channels.binary_search(&channel.id).is_ok());

        // enum ChannelCall {}
        // enum ChannelNote {}
        // enum NotesTooltip {}
        // enum ChatTooltip {}
        // enum ChannelTooltip {}

        // let mut is_dragged_over = false;
        // if cx
        //     .global::<DragAndDrop<Workspace>>()
        //     .currently_dragged::<Channel>(cx.window())
        //     .is_some()
        //     && self.drag_target_channel == ChannelDragTarget::Channel(channel_id)
        // {
        //     is_dragged_over = true;
        // }

        // let has_messages_notification = channel.unseen_message_id.is_some();

        // MouseEventHandler::new::<Channel, _>(ix, cx, |state, cx| {
        //     let row_hovered = state.hovered();

        //     let mut select_state = |interactive: &Interactive<ContainerStyle>| {
        //         if state.clicked() == Some(MouseButton::Left) && interactive.clicked.is_some() {
        //             interactive.clicked.as_ref().unwrap().clone()
        //         } else if state.hovered() || other_selected {
        //             interactive
        //                 .hovered
        //                 .as_ref()
        //                 .unwrap_or(&interactive.default)
        //                 .clone()
        //         } else {
        //             interactive.default.clone()
        //         }
        //     };

        //     Flex::<Self>::row()
        //         .with_child(
        //             Svg::new(if is_public {
        //                 "icons/public.svg"
        //             } else {
        //                 "icons/hash.svg"
        //             })
        //             .with_color(collab_theme.channel_hash.color)
        //             .constrained()
        //             .with_width(collab_theme.channel_hash.width)
        //             .aligned()
        //             .left(),
        //         )
        //         .with_child({
        //             let style = collab_theme.channel_name.inactive_state();
        //             Flex::row()
        //                 .with_child(
        //                     Label::new(channel.name.clone(), style.text.clone())
        //                         .contained()
        //                         .with_style(style.container)
        //                         .aligned()
        //                         .left()
        //                         .with_tooltip::<ChannelTooltip>(
        //                             ix,
        //                             "Join channel",
        //                             None,
        //                             theme.tooltip.clone(),
        //                             cx,
        //                         ),
        //                 )
        //                 .with_children({
        //                     let participants =
        //                         self.channel_store.read(cx).channel_participants(channel_id);

        //                     if !participants.is_empty() {
        //                         let extra_count = participants.len().saturating_sub(FACEPILE_LIMIT);

        //                         let result = FacePile::new(collab_theme.face_overlap)
        //                             .with_children(
        //                                 participants
        //                                     .iter()
        //                                     .filter_map(|user| {
        //                                         Some(
        //                                             Image::from_data(user.avatar.clone()?)
        //                                                 .with_style(collab_theme.channel_avatar),
        //                                         )
        //                                     })
        //                                     .take(FACEPILE_LIMIT),
        //                             )
        //                             .with_children((extra_count > 0).then(|| {
        //                                 Label::new(
        //                                     format!("+{}", extra_count),
        //                                     collab_theme.extra_participant_label.text.clone(),
        //                                 )
        //                                 .contained()
        //                                 .with_style(collab_theme.extra_participant_label.container)
        //                             }));

        //                         Some(result)
        //                     } else {
        //                         None
        //                     }
        //                 })
        //                 .with_spacing(8.)
        //                 .align_children_center()
        //                 .flex(1., true)
        //         })
        //         .with_child(
        //             MouseEventHandler::new::<ChannelNote, _>(ix, cx, move |mouse_state, _| {
        //                 let container_style = collab_theme
        //                     .disclosure
        //                     .button
        //                     .style_for(mouse_state)
        //                     .container;

        //                 if channel.unseen_message_id.is_some() {
        //                     Svg::new("icons/conversations.svg")
        //                         .with_color(collab_theme.channel_note_active_color)
        //                         .constrained()
        //                         .with_width(collab_theme.channel_hash.width)
        //                         .contained()
        //                         .with_style(container_style)
        //                         .with_uniform_padding(4.)
        //                         .into_any()
        //                 } else if row_hovered {
        //                     Svg::new("icons/conversations.svg")
        //                         .with_color(collab_theme.channel_hash.color)
        //                         .constrained()
        //                         .with_width(collab_theme.channel_hash.width)
        //                         .contained()
        //                         .with_style(container_style)
        //                         .with_uniform_padding(4.)
        //                         .into_any()
        //                 } else {
        //                     Empty::new().into_any()
        //                 }
        //             })
        //             .on_click(MouseButton::Left, move |_, this, cx| {
        //                 this.join_channel_chat(&JoinChannelChat { channel_id }, cx);
        //             })
        //             .with_tooltip::<ChatTooltip>(
        //                 ix,
        //                 "Open channel chat",
        //                 None,
        //                 theme.tooltip.clone(),
        //                 cx,
        //             )
        //             .contained()
        //             .with_margin_right(4.),
        //         )
        //         .with_child(
        //             MouseEventHandler::new::<ChannelCall, _>(ix, cx, move |mouse_state, cx| {
        //                 let container_style = collab_theme
        //                     .disclosure
        //                     .button
        //                     .style_for(mouse_state)
        //                     .container;
        //                 if row_hovered || channel.unseen_note_version.is_some() {
        //                     Svg::new("icons/file.svg")
        //                         .with_color(if channel.unseen_note_version.is_some() {
        //                             collab_theme.channel_note_active_color
        //                         } else {
        //                             collab_theme.channel_hash.color
        //                         })
        //                         .constrained()
        //                         .with_width(collab_theme.channel_hash.width)
        //                         .contained()
        //                         .with_style(container_style)
        //                         .with_uniform_padding(4.)
        //                         .with_margin_right(collab_theme.channel_hash.container.margin.left)
        //                         .with_tooltip::<NotesTooltip>(
        //                             ix as usize,
        //                             "Open channel notes",
        //                             None,
        //                             theme.tooltip.clone(),
        //                             cx,
        //                         )
        //                         .into_any()
        //                 } else if has_messages_notification {
        //                     Empty::new()
        //                         .constrained()
        //                         .with_width(collab_theme.channel_hash.width)
        //                         .contained()
        //                         .with_uniform_padding(4.)
        //                         .with_margin_right(collab_theme.channel_hash.container.margin.left)
        //                         .into_any()
        //                 } else {
        //                     Empty::new().into_any()
        //                 }
        //             })
        //             .on_click(MouseButton::Left, move |_, this, cx| {
        //                 this.open_channel_notes(&OpenChannelNotes { channel_id }, cx);
        //             }),
        //         )
        //         .align_children_center()
        //         .styleable_component()
        //         .disclosable(
        //             disclosed,
        //             Box::new(ToggleCollapse {
        //                 location: channel.id.clone(),
        //             }),
        //         )
        //         .with_id(ix)
        //         .with_style(collab_theme.disclosure.clone())
        //         .element()
        //         .constrained()
        //         .with_height(collab_theme.row_height)
        //         .contained()
        //         .with_style(select_state(
        //             collab_theme
        //                 .channel_row
        //                 .in_state(is_selected || is_active || is_dragged_over),
        //         ))
        //         .with_padding_left(
        //             collab_theme.channel_row.default_style().padding.left
        //                 + collab_theme.channel_indent * depth as f32,
        //         )
        // })
        // .on_click(MouseButton::Left, move |_, this, cx| {
        //     if this.
        // drag_target_channel == ChannelDragTarget::None {
        //         if is_active {
        //             this.open_channel_notes(&OpenChannelNotes { channel_id }, cx)
        //         } else {
        //             this.join_channel(channel_id, cx)
        //         }
        //     }
        // })
        // .on_click(MouseButton::Right, {
        //     let channel = channel.clone();
        //     move |e, this, cx| {
        //         this.deploy_channel_context_menu(Some(e.position), &channel, ix, cx);
        //     }
        // })
        // .on_up(MouseButton::Left, move |_, this, cx| {
        //     if let Some((_, dragged_channel)) = cx
        //         .global::<DragAndDrop<Workspace>>()
        //         .currently_dragged::<Channel>(cx.window())
        //     {
        //         this.channel_store
        //             .update(cx, |channel_store, cx| {
        //                 channel_store.move_channel(dragged_channel.id, Some(channel_id), cx)
        //             })
        //             .detach_and_log_err(cx)
        //     }
        // })
        // .on_move({
        //     let channel = channel.clone();
        //     move |_, this, cx| {
        //         if let Some((_, dragged_channel)) = cx
        //             .global::<DragAndDrop<Workspace>>()
        //             .currently_dragged::<Channel>(cx.window())
        //         {
        //             if channel.id != dragged_channel.id {
        //                 this.drag_target_channel = ChannelDragTarget::Channel(channel.id);
        //             }
        //             cx.notify()
        //         }
        //     }
        // })
        // .as_draggable::<_, Channel>(
        //     channel.clone(),
        //     move |_, channel, cx: &mut ViewContext<Workspace>| {
        //         let theme = &theme::current(cx).collab_panel;

        //         Flex::<Workspace>::row()
        //             .with_child(
        //                 Svg::new("icons/hash.svg")
        //                     .with_color(theme.channel_hash.color)
        //                     .constrained()
        //                     .with_width(theme.channel_hash.width)
        //                     .aligned()
        //                     .left(),
        //             )
        //             .with_child(
        //                 Label::new(channel.name.clone(), theme.channel_name.text.clone())
        //                     .contained()
        //                     .with_style(theme.channel_name.container)
        //                     .aligned()
        //                     .left(),
        //             )
        //             .align_children_center()
        //             .contained()
        //             .with_background_color(
        //                 theme
        //                     .container
        //                     .background_color
        //                     .unwrap_or(gpui::color::Color::transparent_black()),
        //             )
        //             .contained()
        //             .with_padding_left(
        //                 theme.channel_row.default_style().padding.left
        //                     + theme.channel_indent * depth as f32,
        //             )
        //             .into_any()
        //     },
        // )
        // .with_cursor_style(CursorStyle::PointingHand)
        // .into_any()
    }

    fn render_channel_editor(
        &mut self,
        depth: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let item = ListItem::new("channel-editor")
            .inset(false)
            .indent_level(depth)
            .left_icon(Icon::Hash);

        if let Some(pending_name) = self
            .channel_editing_state
            .as_ref()
            .and_then(|state| state.pending_name())
        {
            item.child(Label::new(pending_name))
        } else {
            item.child(
                div()
                    .w_full()
                    .py_1() // todo!() @nate this is a px off at the default font size.
                    .child(self.channel_name_editor.clone()),
            )
        }
    }
}

fn render_tree_branch(is_last: bool, cx: &mut WindowContext) -> impl IntoElement {
    let rem_size = cx.rem_size();
    let line_height = cx.text_style().line_height_in_pixels(rem_size);
    let width = rem_size * 1.5;
    let thickness = px(2.);
    let color = cx.theme().colors().text;

    canvas(move |bounds, cx| {
        let start_x = (bounds.left() + bounds.right() - thickness) / 2.;
        let start_y = (bounds.top() + bounds.bottom() - thickness) / 2.;
        let right = bounds.right();
        let top = bounds.top();

        cx.paint_quad(
            Bounds::from_corners(
                point(start_x, top),
                point(
                    start_x + thickness,
                    if is_last { start_y } else { bounds.bottom() },
                ),
            ),
            Default::default(),
            color,
            Default::default(),
            Hsla::transparent_black(),
        );
        cx.paint_quad(
            Bounds::from_corners(point(start_x, start_y), point(right, start_y + thickness)),
            Default::default(),
            color,
            Default::default(),
            Hsla::transparent_black(),
        );
    })
    .w(width)
    .h(line_height)
}

impl Render for CollabPanel {
    type Element = Focusable<Div>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        v_stack()
            .key_context("CollabPanel")
            .on_action(cx.listener(CollabPanel::cancel))
            .on_action(cx.listener(CollabPanel::select_next))
            .on_action(cx.listener(CollabPanel::select_prev))
            .on_action(cx.listener(CollabPanel::confirm))
            .on_action(cx.listener(CollabPanel::insert_space))
            //     .on_action(cx.listener(CollabPanel::remove))
            .on_action(cx.listener(CollabPanel::remove_selected_channel))
            .on_action(cx.listener(CollabPanel::show_inline_context_menu))
            //     .on_action(cx.listener(CollabPanel::new_subchannel))
            //     .on_action(cx.listener(CollabPanel::invite_members))
            //     .on_action(cx.listener(CollabPanel::manage_members))
            .on_action(cx.listener(CollabPanel::rename_selected_channel))
            //     .on_action(cx.listener(CollabPanel::rename_channel))
            //     .on_action(cx.listener(CollabPanel::toggle_channel_collapsed_action))
            .on_action(cx.listener(CollabPanel::collapse_selected_channel))
            .on_action(cx.listener(CollabPanel::expand_selected_channel))
            //     .on_action(cx.listener(CollabPanel::open_channel_notes))
            //     .on_action(cx.listener(CollabPanel::join_channel_chat))
            //     .on_action(cx.listener(CollabPanel::copy_channel_link))
            .track_focus(&self.focus_handle)
            .size_full()
            .child(if self.user_store.read(cx).current_user().is_none() {
                self.render_signed_out(cx)
            } else {
                self.render_signed_in(cx)
            })
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                overlay()
                    .position(*position)
                    .anchor(gpui::AnchorCorner::TopLeft)
                    .child(menu.clone())
            }))
    }
}

// impl View for CollabPanel {
//     fn ui_name() -> &'static str {
//         "CollabPanel"
//     }

//     fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
//         if !self.has_focus {
//             self.has_focus = true;
//             if !self.context_menu.is_focused(cx) {
//                 if let Some(editing_state) = &self.channel_editing_state {
//                     if editing_state.pending_name().is_none() {
//                         cx.focus(&self.channel_name_editor);
//                     } else {
//                         cx.focus(&self.filter_editor);
//                     }
//                 } else {
//                     cx.focus(&self.filter_editor);
//                 }
//             }
//             cx.emit(Event::Focus);
//         }
//     }

//     fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
//         self.has_focus = false;
//     }

//     fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
//         let theme = &theme::current(cx).collab_panel;

//         if self.user_store.read(cx).current_user().is_none() {
//             enum LogInButton {}

//             return Flex::column()
//                 .with_child(
//                     MouseEventHandler::new::<LogInButton, _>(0, cx, |state, _| {
//                         let button = theme.log_in_button.style_for(state);
//                         Label::new("Sign in to collaborate", button.text.clone())
//                             .aligned()
//                             .left()
//                             .contained()
//                             .with_style(button.container)
//                     })
//                     .on_click(MouseButton::Left, |_, this, cx| {
//                         let client = this.client.clone();
//                         cx.spawn(|_, cx| async move {
//                             client.authenticate_and_connect(true, &cx).await.log_err();
//                         })
//                         .detach();
//                     })
//                     .with_cursor_style(CursorStyle::PointingHand),
//                 )
//                 .contained()
//                 .with_style(theme.container)
//                 .into_any();
//         }

//         enum PanelFocus {}
//         MouseEventHandler::new::<PanelFocus, _>(0, cx, |_, cx| {
//             Stack::new()
//                 .with_child(
//                     Flex::column()
//                         .with_child(
//                             Flex::row().with_child(
//                                 ChildView::new(&self.filter_editor, cx)
//                                     .contained()
//                                     .with_style(theme.user_query_editor.container)
//                                     .flex(1.0, true),
//                             ),
//                         )
//                         .with_child(List::new(self.list_state.clone()).flex(1., true).into_any())
//                         .contained()
//                         .with_style(theme.container)
//                         .into_any(),
//                 )
//                 .with_children(
//                     (!self.context_menu_on_selected)
//                         .then(|| ChildView::new(&self.context_menu, cx)),
//                 )
//                 .into_any()
//         })
//         .on_click(MouseButton::Left, |_, _, cx| cx.focus_self())
//         .into_any_named("collab panel")
//     }

//     fn update_keymap_context(
//         &self,
//         keymap: &mut gpui::keymap_matcher::KeymapContext,
//         _: &AppContext,
//     ) {
//         Self::reset_to_default_keymap_context(keymap);
//         if self.channel_editing_state.is_some() {
//             keymap.add_identifier("editing");
//         } else {
//             keymap.add_identifier("not_editing");
//         }
//     }
// }

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

    fn size(&self, cx: &gpui::WindowContext) -> f32 {
        self.width.map_or_else(
            || CollaborationPanelSettings::get_global(cx).default_width,
            |width| width.0,
        )
    }

    fn set_size(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>) {
        self.width = size.map(|s| px(s));
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &gpui::WindowContext) -> Option<ui::Icon> {
        CollaborationPanelSettings::get_global(cx)
            .button
            .then(|| ui::Icon::Collab)
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
            // ListEntry::ChannelInvite(channel_1) => {
            //     if let ListEntry::ChannelInvite(channel_2) = other {
            //         return channel_1.id == channel_2.id;
            //     }
            // }
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

// fn render_icon_button(style: &IconButton, svg_path: &'static str) -> impl Element<CollabPanel> {
//     Svg::new(svg_path)
//         .with_color(style.color)
//         .constrained()
//         .with_width(style.icon_width)
//         .aligned()
//         .constrained()
//         .with_width(style.button_width)
//         .with_height(style.button_width)
//         .contained()
//         .with_style(style.container)
// }

struct DraggedChannelView {
    channel: Channel,
    width: Pixels,
}

impl Render for DraggedChannelView {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let ui_font = ThemeSettings::get_global(cx).ui_font.family.clone();
        h_stack()
            .font(ui_font)
            .bg(cx.theme().colors().background)
            .w(self.width)
            .p_1()
            .gap_1()
            .child(
                IconElement::new(
                    if self.channel.visibility == proto::ChannelVisibility::Public {
                        Icon::Public
                    } else {
                        Icon::Hash
                    },
                )
                .size(IconSize::Small)
                .color(Color::Muted),
            )
            .child(Label::new(self.channel.name.clone()))
    }
}
