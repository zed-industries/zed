use call::ActiveCall;
use channel::ChannelStore;
use gpui::{App, Context, EventEmitter, Render, Subscription, WeakEntity, Window};
use ui::{CollabOverlay, CollabOverlayHeader, prelude::*};
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

    pub fn is_in_call(&self, cx: &App) -> bool {
        ActiveCall::global(cx).read(cx).room().is_some()
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
}

impl Render for CollabOverlayPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_in_call(cx) {
            return div().into_any_element();
        }

        let channel_name = self.channel_name(cx);

        CollabOverlay::new()
            .header(CollabOverlayHeader::new(channel_name).is_open(true))
            .into_any_element()
    }
}
