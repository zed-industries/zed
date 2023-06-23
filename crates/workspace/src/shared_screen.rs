use crate::{
    item::{Item, ItemEvent},
    ItemNavHistory, WorkspaceId,
};
use anyhow::Result;
use call::participant::{Frame, RemoteVideoTrack};
use client::{proto::PeerId, User};
use futures::StreamExt;
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    platform::MouseButton,
    AppContext, Entity, Task, View, ViewContext,
};
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    sync::{Arc, Weak},
};

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
}

impl SharedScreen {
    pub fn new(
        track: &Arc<RemoteVideoTrack>,
        peer_id: PeerId,
        user: Arc<User>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
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
        }
    }
}

impl Entity for SharedScreen {
    type Event = Event;
}

impl View for SharedScreen {
    fn ui_name() -> &'static str {
        "SharedScreen"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum Focus {}

        let frame = self.frame.clone();
        MouseEventHandler::new::<Focus, _>(0, cx, |_, cx| {
            Canvas::new(move |scene, bounds, _, _, _| {
                if let Some(frame) = frame.clone() {
                    let size = constrain_size_preserving_aspect_ratio(
                        bounds.size(),
                        vec2f(frame.width() as f32, frame.height() as f32),
                    );
                    let origin = bounds.origin() + (bounds.size() / 2.) - size / 2.;
                    scene.push_surface(gpui::scene::Surface {
                        bounds: RectF::new(origin, size),
                        image_buffer: frame.image(),
                    });
                }
            })
            .contained()
            .with_style(theme::current(cx).shared_screen)
        })
        .on_down(MouseButton::Left, |_, _, cx| cx.focus_parent())
        .into_any()
    }
}

impl Item for SharedScreen {
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<Cow<str>> {
        Some(format!("{}'s screen", self.user.github_login).into())
    }
    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            nav_history.push::<()>(None, cx);
        }
    }

    fn tab_content<V: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> gpui::AnyElement<V> {
        Flex::row()
            .with_child(
                Svg::new("icons/disable_screen_sharing_12.svg")
                    .with_color(style.label.text.color)
                    .constrained()
                    .with_width(style.type_icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(style.spacing),
            )
            .with_child(
                Label::new(
                    format!("{}'s screen", self.user.github_login),
                    style.label.clone(),
                )
                .aligned(),
            )
            .into_any()
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self> {
        let track = self.track.upgrade()?;
        Some(Self::new(&track, self.peer_id, self.user.clone(), cx))
    }

    fn to_item_events(event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        match event {
            Event::Close => smallvec::smallvec!(ItemEvent::CloseItem),
        }
    }
}
