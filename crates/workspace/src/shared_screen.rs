#[cfg(any(
    all(
        target_os = "macos",
        feature = "livekit-cross-platform",
        not(feature = "livekit-macos"),
    ),
    all(not(target_os = "macos"), feature = "livekit-cross-platform"),
))]
mod cross_platform {
    use crate::{
        item::{Item, ItemEvent},
        ItemNavHistory, WorkspaceId,
    };
    use call::{RemoteVideoTrack, RemoteVideoTrackView};
    use client::{proto::PeerId, User};
    use gpui::{
        div, AppContext, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
        ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext,
        WindowContext,
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
        view: View<RemoteVideoTrackView>,
        focus: FocusHandle,
    }

    impl SharedScreen {
        pub fn new(
            track: RemoteVideoTrack,
            peer_id: PeerId,
            user: Arc<User>,
            cx: &mut ViewContext<Self>,
        ) -> Self {
            let view = cx.new_view(|cx| RemoteVideoTrackView::new(track.clone(), cx));
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
                .child(self.view.clone())
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

        fn tab_icon(&self, _cx: &WindowContext) -> Option<Icon> {
            Some(Icon::new(IconName::Screen))
        }

        fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
            Some(format!("{}'s screen", self.user.github_login).into())
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
            Some(cx.new_view(|cx| Self {
                view: self.view.update(cx, |view, cx| view.clone(cx)),
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
}

#[cfg(any(
    all(
        target_os = "macos",
        feature = "livekit-cross-platform",
        not(feature = "livekit-macos"),
    ),
    all(not(target_os = "macos"), feature = "livekit-cross-platform"),
))]
pub use cross_platform::*;

#[cfg(any(
    all(target_os = "macos", feature = "livekit-macos"),
    all(
        not(target_os = "macos"),
        feature = "livekit-macos",
        not(feature = "livekit-cross-platform")
    )
))]
mod macos {
    use crate::{
        item::{Item, ItemEvent},
        ItemNavHistory, WorkspaceId,
    };
    use anyhow::Result;
    use call::participant::{Frame, RemoteVideoTrack};
    use client::{proto::PeerId, User};
    use futures::StreamExt;
    use gpui::{
        div, surface, AppContext, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
        ParentElement, Render, SharedString, Styled, Task, View, ViewContext, VisualContext,
        WindowContext,
    };
    use std::sync::{Arc, Weak};
    use ui::{prelude::*, Icon, IconName};

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
            track: Arc<RemoteVideoTrack>,
            peer_id: PeerId,
            user: Arc<User>,
            cx: &mut ViewContext<Self>,
        ) -> Self {
            cx.focus_handle();
            let mut frames = track.frames();
            Self {
                track: Arc::downgrade(&track),
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
                        .map(|frame| surface(frame.image()).size_full()),
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

        fn tab_icon(&self, _cx: &WindowContext) -> Option<Icon> {
            Some(Icon::new(IconName::Screen))
        }

        fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
            Some(format!("{}'s screen", self.user.github_login).into())
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
            Some(cx.new_view(|cx| Self::new(track, self.peer_id, self.user.clone(), cx)))
        }

        fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
            match event {
                Event::Close => f(ItemEvent::CloseItem),
            }
        }
    }
}

#[cfg(any(
    all(target_os = "macos", feature = "livekit-macos"),
    all(
        not(target_os = "macos"),
        feature = "livekit-macos",
        not(feature = "livekit-cross-platform")
    )
))]
pub use macos::*;
