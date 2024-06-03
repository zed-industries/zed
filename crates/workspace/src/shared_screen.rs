use crate::{
    item::{Item, ItemEvent, TabContentParams},
    ItemNavHistory, WorkspaceId,
};
use anyhow::Result;
use call::participant::{Frame, RemoteVideoTrack};
use client::{proto::PeerId, User};
use futures::StreamExt;
use gpui::{
    div, img, AppContext, Element, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    ParentElement, Render, SharedString, Styled, Task, View, ViewContext, VisualContext,
    WindowContext,
};
use std::sync::{Arc, Weak};
use ui::{h_flex, prelude::*, Icon, IconName, Label};

pub enum Event {
    Close,
}

pub struct SharedScreen {
    track: Weak<RemoteVideoTrack>,
    frame: Option<Frame>,
    pub peer_id: PeerId,
    user: Arc<User>,
    nav_history: Option<ItemNavHistory>,
    _maintain_frame: Task<Result<()>>,
    focus: FocusHandle,
}

impl SharedScreen {
    pub fn new(
        track: &Arc<RemoteVideoTrack>,
        peer_id: PeerId,
        user: Arc<User>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.focus_handle();
        let mut frames = track.frames();
        Self {
            track: Arc::downgrade(track),
            frame: None,
            peer_id,
            user,
            nav_history: Default::default(),
            _maintain_frame: cx.spawn(|this, mut cx| async move {
                while let Some(frame) = frames.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.frame = Some(frame);
                        cx.notify();
                    })?;
                }
                this.update(&mut cx, |_, cx| cx.emit(Event::Close))?;
                Ok(())
            }),
            focus: cx.focus_handle(),
        }
    }
}

impl EventEmitter<Event> for SharedScreen {}

impl FocusableView for SharedScreen {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus.clone()
    }
}
impl Render for SharedScreen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus)
            .key_context("SharedScreen")
            .size_full()
            .children(
                self.frame
                    .as_ref()
                    .map(|frame| img(frame.image()).size_full()),
            )
    }
}

impl Item for SharedScreen {
    type Event = Event;

    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        Some(format!("{}'s screen", self.user.github_login).into())
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            nav_history.push::<()>(None, cx);
        }
    }

    fn tab_content(&self, params: TabContentParams, _: &WindowContext<'_>) -> gpui::AnyElement {
        h_flex()
            .gap_1()
            .child(Icon::new(IconName::Screen))
            .child(
                Label::new(format!("{}'s screen", self.user.github_login)).color(
                    if params.selected {
                        Color::Default
                    } else {
                        Color::Muted
                    },
                ),
            )
            .into_any()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        let track = self.track.upgrade()?;
        Some(cx.new_view(|cx| Self::new(&track, self.peer_id, self.user.clone(), cx)))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
        }
    }
}
