use crate::channel_view::ChannelView;
use call::room::Room;
use call::ActiveCall;
use channel::ChannelStore;
use gpui::{
    AnyElement, App, Context, Corner, Entity, EventEmitter, Render, SharedString, Subscription,
    WeakEntity, Window,
};
use rpc::proto;
use title_bar::collab::{toggle_deafen, toggle_mute, toggle_screen_sharing};
use ui::{
    CollabOverlay, CollabOverlayControls, CollabOverlayHeader, ContextMenu, ContextMenuItem, Icon,
    IconButton, IconName, IconSize, Label, LabelSize, ParticipantItem, ParticipantProject,
    ParticipantScreen, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use workspace::notifications::DetachAndPromptErr;
use workspace::Workspace;

pub struct CollabOverlayPanel {
    workspace: WeakEntity<Workspace>,
    screen_share_menu_handle: PopoverMenuHandle<ContextMenu>,
    collapsed: bool,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    CallStateChanged,
}

impl EventEmitter<Event> for CollabOverlayPanel {}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let weak_workspace = cx.weak_entity();
        let panel = cx.new(|cx| CollabOverlayPanel::new(weak_workspace, cx));
        workspace.set_collab_overlay_panel(panel.into(), window, cx);
    })
    .detach();
}

impl CollabOverlayPanel {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        let active_call = ActiveCall::global(cx);

        let mut subscriptions = vec![cx.observe(&active_call, |this, _, cx| {
            cx.emit(Event::CallStateChanged);
            this.subscribe_to_room(cx);
            cx.notify();
        })];

        let room = active_call.read(cx).room().cloned();
        if let Some(room) = room {
            subscriptions.push(cx.subscribe(&room, |_, _, _, cx| {
                cx.notify();
            }));
        }

        Self {
            workspace,
            screen_share_menu_handle: PopoverMenuHandle::default(),
            collapsed: false,
            _subscriptions: subscriptions,
        }
    }

    fn subscribe_to_room(&mut self, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let room = active_call.read(cx).room().cloned();
        if let Some(room) = room {
            self._subscriptions.push(cx.subscribe(&room, |_, _, _, cx| {
                cx.notify();
            }));
        }
    }

    fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    fn open_channel_notes(&self, window: &mut Window, cx: &mut App) {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return;
        };

        let Some(channel_id) = room.read(cx).channel_id() else {
            return;
        };

        if let Some(workspace) = self.workspace.upgrade() {
            ChannelView::open(channel_id, None, workspace, window, cx).detach();
        }
    }

    fn channel_name(&self, cx: &App) -> SharedString {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return "Call".into();
        };

        let channel_id = room.read(cx).channel_id();

        if let Some(channel_id) = channel_id {
            let channel_store = ChannelStore::global(cx);
            if let Some(channel) = channel_store.read(cx).channel_for_id(channel_id) {
                return channel.name.clone();
            }
        }

        "Call".into()
    }

    fn render_remote_participant(
        &self,
        participant: &call::participant::RemoteParticipant,
        cx: &App,
    ) -> AnyElement {
        let peer_id = participant.peer_id;
        let user_id = participant.user.id;
        let is_guest = participant.role == proto::ChannelRole::Guest;
        let participant_index = participant.participant_index.0;

        let is_following = self
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).is_being_followed(peer_id))
            .unwrap_or(false);

        let player_color = cx
            .theme()
            .players()
            .color_for_participant(participant_index)
            .cursor;

        let projects_data: Vec<(u64, SharedString)> = participant
            .projects
            .iter()
            .map(|project| {
                let name: SharedString = if project.worktree_root_names.is_empty() {
                    "untitled".into()
                } else {
                    project.worktree_root_names.join(", ").into()
                };
                (project.id, name)
            })
            .collect();

        let has_screen = participant.has_video_tracks();
        let projects: Vec<ParticipantProject> = projects_data
            .iter()
            .enumerate()
            .map(|(index, (_, name))| {
                let is_last = index == projects_data.len() - 1 && !has_screen;
                ParticipantProject {
                    name: name.clone(),
                    is_last,
                }
            })
            .collect();

        let screen = if has_screen {
            Some(ParticipantScreen { is_last: true })
        } else {
            None
        };

        let workspace = self.workspace.clone();
        let workspace_for_project = self.workspace.clone();
        let workspace_for_screen = self.workspace.clone();

        let project_ids: Vec<u64> = projects_data.iter().map(|(id, _)| *id).collect();

        ParticipantItem::new(participant.user.github_login.clone())
            .avatar(participant.user.avatar_uri.to_string())
            .current_user(false)
            .muted(participant.muted)
            .speaking(participant.speaking)
            .deafened(false)
            .guest(is_guest)
            .following(is_following)
            .player_color(player_color)
            .projects(projects)
            .when_some(screen, |this, screen| this.screen(screen))
            .on_click(move |_, window, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    let is_currently_following = workspace.read(cx).is_being_followed(peer_id);
                    workspace.update(cx, |workspace, cx| {
                        if is_currently_following {
                            workspace.unfollow(peer_id, window, cx);
                        } else {
                            workspace.follow(peer_id, window, cx);
                        }
                    });
                }
            })
            .on_project_click(move |index, _, window, cx| {
                if let Some(project_id) = project_ids.get(index) {
                    if let Some(workspace) = workspace_for_project.upgrade() {
                        let project_id = *project_id;
                        workspace.update(cx, |workspace, cx| {
                            let app_state = workspace.app_state().clone();
                            workspace::join_in_room_project(project_id, user_id, app_state, cx)
                                .detach_and_prompt_err(
                                    "Failed to join project",
                                    window,
                                    cx,
                                    |_, _, _| None,
                                );
                        });
                    }
                }
            })
            .on_screen_click(move |_, window, cx| {
                if let Some(workspace) = workspace_for_screen.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        workspace.open_shared_screen(peer_id, window, cx);
                    });
                }
            })
            .into_any_element()
    }

    fn render_participants(&self, room: &Entity<Room>, cx: &App) -> Vec<AnyElement> {
        let room_read = room.read(cx);
        let mut participants = Vec::new();

        if let Some(current_user) = room_read.local_participant_user(cx) {
            let is_muted = room_read.is_muted();
            let is_speaking = room_read.is_speaking();
            let is_deafened = room_read.is_deafened().unwrap_or(false);
            let is_guest = room_read.local_participant_is_guest();

            let local_projects = room_read.local_participant().projects.clone();
            let has_screen = room_read.is_sharing_screen();

            let projects: Vec<ParticipantProject> = local_projects
                .iter()
                .enumerate()
                .map(|(index, project)| {
                    let name: SharedString = if project.worktree_root_names.is_empty() {
                        "untitled".into()
                    } else {
                        project.worktree_root_names.join(", ").into()
                    };
                    let is_last = index == local_projects.len() - 1 && !has_screen;
                    ParticipantProject { name, is_last }
                })
                .collect();

            let screen = if has_screen {
                Some(ParticipantScreen { is_last: true })
            } else {
                None
            };

            participants.push(
                ParticipantItem::new(current_user.github_login.clone())
                    .avatar(current_user.avatar_uri.to_string())
                    .current_user(true)
                    .muted(is_muted)
                    .speaking(is_speaking)
                    .deafened(is_deafened)
                    .guest(is_guest)
                    .projects(projects)
                    .when_some(screen, |this, screen| this.screen(screen))
                    .into_any_element(),
            );
        }

        for (_, remote_participant) in room_read.remote_participants() {
            participants.push(self.render_remote_participant(remote_participant, cx));
        }

        participants
    }

    fn render_screen_share_menu(&self, is_screen_sharing: bool) -> impl IntoElement {
        PopoverMenu::new("collab-overlay-screen-share-menu")
            .with_handle(self.screen_share_menu_handle.clone())
            .anchor(Corner::BottomLeft)
            .trigger(
                IconButton::new("screen-share-trigger", IconName::Screen)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text(if is_screen_sharing {
                        "Stop Sharing Screen"
                    } else {
                        "Share Screen"
                    }))
                    .when(is_screen_sharing, |this| this.icon_color(Color::Accent))
                    .toggle_state(self.screen_share_menu_handle.is_deployed()),
            )
            .menu(move |window, cx| {
                let screens = cx.screen_capture_sources();
                Some(ContextMenu::build(window, cx, |context_menu, _, cx| {
                    cx.spawn(async move |this: WeakEntity<ContextMenu>, cx| {
                        let screens = screens.await??;
                        this.update(cx, |this, cx| {
                            let active_screenshare_id = ActiveCall::global(cx)
                                .read(cx)
                                .room()
                                .and_then(|room| room.read(cx).shared_screen_id());

                            for screen in screens {
                                let Ok(meta) = screen.metadata() else {
                                    continue;
                                };

                                let label = meta
                                    .label
                                    .clone()
                                    .unwrap_or_else(|| SharedString::from("Unknown screen"));
                                let resolution = SharedString::from(format!(
                                    "{} × {}",
                                    meta.resolution.width.0, meta.resolution.height.0
                                ));
                                let is_active = active_screenshare_id == Some(meta.id);

                                this.push_item(ContextMenuItem::CustomEntry {
                                    entry_render: Box::new(move |_, _| {
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                Icon::new(IconName::Screen)
                                                    .size(IconSize::XSmall)
                                                    .color(if is_active {
                                                        Color::Accent
                                                    } else {
                                                        Color::Muted
                                                    }),
                                            )
                                            .child(Label::new(label.clone()))
                                            .child(
                                                Label::new(resolution.clone())
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            )
                                            .into_any()
                                    }),
                                    selectable: true,
                                    documentation_aside: None,
                                    handler: std::rc::Rc::new(move |_, window, cx| {
                                        toggle_screen_sharing(
                                            Ok(Some(screen.clone())),
                                            window,
                                            cx,
                                        );
                                    }),
                                });
                            }
                        })
                    })
                    .detach_and_log_err(cx);
                    context_menu
                }))
            })
    }

    fn render_header(&self, cx: &mut Context<Self>) -> CollabOverlayHeader {
        let channel_name = self.channel_name(cx);

        CollabOverlayHeader::new(channel_name)
            .is_open(!self.collapsed)
            .on_toggle(cx.listener(|this, _, _, cx| {
                this.toggle_collapsed(cx);
            }))
            .on_channel_notes(cx.listener(|this, _, window, cx| {
                this.open_channel_notes(window, cx);
            }))
    }

    fn render_controls(
        &self,
        room: &Entity<Room>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CollabOverlayControls {
        let room_read = room.read(cx);

        let avatar_uri = room_read
            .local_participant_user(cx)
            .map(|user| user.avatar_uri.to_string())
            .unwrap_or_else(|| "https://avatars.githubusercontent.com/u/1?v=4".into());

        let is_muted = room_read.is_muted();
        let is_deafened = room_read.is_deafened().unwrap_or(false);
        let is_screen_sharing = room_read.is_sharing_screen();

        CollabOverlayControls::new(avatar_uri)
            .is_muted(is_muted)
            .is_deafened(is_deafened)
            .is_screen_sharing(is_screen_sharing)
            .on_toggle_mute(|_, _, cx| {
                toggle_mute(cx);
            })
            .on_toggle_deafen(|_, _, cx| {
                toggle_deafen(cx);
            })
            .screen_share_menu(self.render_screen_share_menu(is_screen_sharing))
            .on_leave(|_, _, cx| {
                ActiveCall::global(cx)
                    .update(cx, |call, cx| call.hang_up(cx))
                    .detach_and_log_err(cx);
            })
    }
}

impl Render for CollabOverlayPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() else {
            return div().into_any_element();
        };

        let header = self.render_header(cx);
        let controls = self.render_controls(&room, window, cx);

        if self.collapsed {
            CollabOverlay::new()
                .header(header)
                .controls(controls)
                .into_any_element()
        } else {
            let participants = self.render_participants(&room, cx);

            CollabOverlay::new()
                .header(header)
                .children(participants)
                .controls(controls)
                .into_any_element()
        }
    }
}
