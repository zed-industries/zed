use crate::{
    item::{Item, ItemEvent},
    ItemNavHistory, WorkspaceId,
};
use call::{RemoteVideoTrack, RemoteVideoTrackView};
use client::{proto::PeerId, User};
use gpui::{
    div, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Render, SharedString, Styled,
};
use std::sync::Arc;
use ui::{prelude::*, Icon, IconName};

pub enum Event {
    Close,
}

pub struct SharedScreen {
    pub peer_id: PeerId,
    user: Arc<User>,
    nav_history: Option<ItemNavHistory>,
    view: Entity<RemoteVideoTrackView>,
    focus: FocusHandle,
}

impl SharedScreen {
    pub fn new(
        track: RemoteVideoTrack,
        peer_id: PeerId,
        user: Arc<User>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let view = cx.new(|cx| RemoteVideoTrackView::new(track.clone(), window, cx));
        cx.subscribe(&view, |_, _, ev, cx| match ev {
            call::RemoteVideoTrackViewEvent::Close => cx.emit(Event::Close),
        })
        .detach();
        Self {
            view,
            peer_id,
            user,
            nav_history: Default::default(),
            focus: cx.focus_handle(),
        }
    }
}

impl EventEmitter<Event> for SharedScreen {}

impl Focusable for SharedScreen {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
impl Render for SharedScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus)
            .key_context("SharedScreen")
            .size_full()
            .child(self.view.clone())
    }
}

impl Item for SharedScreen {
    type Event = Event;

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some(format!("{}'s screen", self.user.github_login).into())
    }

    fn deactivated(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            nav_history.push::<()>(None, cx);
        }
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Screen))
    }

    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
        Some(format!("{}'s screen", self.user.github_login).into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn set_nav_history(
        &mut self,
        history: ItemNavHistory,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.nav_history = Some(history);
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        Some(cx.new(|cx| Self {
            view: self.view.update(cx, |view, cx| view.clone(window, cx)),
            peer_id: self.peer_id,
            user: self.user.clone(),
            nav_history: Default::default(),
            focus: cx.focus_handle(),
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
        }
    }
}
