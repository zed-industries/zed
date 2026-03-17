use crate::{
    ItemNavHistory, WorkspaceId,
    item::{Item, ItemEvent},
};
use client::{User, proto::PeerId};
use gpui::{
    AnyView, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Render, SharedString, Styled, Task, div,
};
use std::sync::Arc;
use ui::{Icon, IconName, prelude::*};

pub enum Event {
    Close,
}

pub struct SharedScreen {
    pub peer_id: PeerId,
    user: Arc<User>,
    nav_history: Option<ItemNavHistory>,
    view: AnyView,
    clone_view: fn(&AnyView, &mut Window, &mut App) -> AnyView,
    focus: FocusHandle,
}

impl SharedScreen {
    pub fn new(
        peer_id: PeerId,
        user: Arc<User>,
        view: AnyView,
        clone_view: fn(&AnyView, &mut Window, &mut App) -> AnyView,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            view,
            peer_id,
            user,
            nav_history: Default::default(),
            focus: cx.focus_handle(),
            clone_view,
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

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{}'s screen", self.user.github_login).into()
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

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        let clone_view = self.clone_view;
        let cloned_view = clone_view(&self.view, window, cx);
        Task::ready(Some(cx.new(|cx| Self {
            view: cloned_view,
            peer_id: self.peer_id,
            user: self.user.clone(),
            nav_history: Default::default(),
            focus: cx.focus_handle(),
            clone_view,
        })))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
        }
    }
}
