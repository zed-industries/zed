use crate::channel_view::ChannelView;
use call::room::Room;
use call::ActiveCall;
use channel::ChannelStore;
use client::User;
use gpui::{
    AnyElement, App, Context, Corner, Entity, EventEmitter, Render, SharedString, Subscription,
    WeakEntity, Window,
};
use rpc::proto;
use std::sync::Arc;
use title_bar::collab::{toggle_deafen, toggle_mute, toggle_screen_sharing};
use ui::{
    CollabOverlay, CollabOverlayControls, CollabOverlayHeader, ContextMenu, ContextMenuItem, Icon,
    IconButton, IconName, IconSize, Label, LabelSize, ParticipantItem, PopoverMenu,
    PopoverMenuHandle, Tooltip, prelude::*,
};
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

    fn render_participant(
        user: &Arc<User>,
        is_current_user: bool,
        is_muted: bool,
        is_speaking: bool,
        is_deafened: bool,
        is_guest: bool,
    ) -> AnyElement {
        ParticipantItem::new(user.github_login.clone())
            .avatar(user.avatar_uri.to_string())
            .current_user(is_current_user)
            .muted(is_muted)
            .speaking(is_speaking)
            .deafened(is_deafened)
            .guest(is_guest)
            .into_any_element()
    }

    fn render_participants(&self, room: &Entity<Room>, cx: &App) -> Vec<AnyElement> {
        let room = room.read(cx);
        let mut participants = Vec::new();

        if let Some(current_user) = room.local_participant_user(cx) {
            let is_muted = room.is_muted();
            let is_speaking = room.is_speaking();
            let is_deafened = room.is_deafened().unwrap_or(false);
            let is_guest = room.local_participant_is_guest();

            participants.push(Self::render_participant(
                &current_user,
                true,
                is_muted,
                is_speaking,
                is_deafened,
                is_guest,
            ));
        }

        for (_, remote_participant) in room.remote_participants() {
            let is_guest = remote_participant.role == proto::ChannelRole::Guest;

            participants.push(Self::render_participant(
                &remote_participant.user,
                false,
                remote_participant.muted,
                remote_participant.speaking,
                false,
                is_guest,
            ));
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
