use call::room::Room;
use call::ActiveCall;
use channel::ChannelStore;
use client::User;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, Render, ScreenCaptureSource, Subscription,
    Task, WeakEntity, Window,
};
use rpc::proto;
use std::rc::Rc;
use std::sync::Arc;
use title_bar::collab::{toggle_deafen, toggle_mute, toggle_screen_sharing};
use ui::{CollabOverlay, CollabOverlayControls, CollabOverlayHeader, ParticipantItem, prelude::*};

use workspace::Workspace;

pub struct CollabOverlayPanel {
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
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

        let should_share = !is_screen_sharing;

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
            .on_toggle_screen_share(move |_, window, cx| {
                window
                    .spawn(cx, async move |cx| {
                        let screen = if should_share {
                            cx.update(|_, cx| pick_default_screen(cx))?.await
                        } else {
                            Ok(None)
                        };
                        cx.update(|window, cx| toggle_screen_sharing(screen, window, cx))?;
                        anyhow::Ok(())
                    })
                    .detach();
            })
            .on_leave(|_, _, cx| {
                ActiveCall::global(cx)
                    .update(cx, |call, cx| call.hang_up(cx))
                    .detach_and_log_err(cx);
            })
    }
}

fn pick_default_screen(cx: &mut App) -> Task<anyhow::Result<Option<Rc<dyn ScreenCaptureSource>>>> {
    let source = cx.screen_capture_sources();
    cx.spawn(async move |_| {
        let available_sources = source.await??;
        Ok(available_sources
            .iter()
            .find(|it| {
                it.as_ref()
                    .metadata()
                    .is_ok_and(|meta| meta.is_main.unwrap_or_default())
            })
            .or_else(|| available_sources.first())
            .cloned())
    })
}

impl Render for CollabOverlayPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() else {
            return div().into_any_element();
        };

        let channel_name = self.channel_name(cx);
        let participants = self.render_participants(&room, cx);
        let controls = self.render_controls(&room, window, cx);

        CollabOverlay::new()
            .header(CollabOverlayHeader::new(channel_name).is_open(true))
            .children(participants)
            .controls(controls)
            .into_any_element()
    }
}
