use crate::participant::{Frame, RemoteVideoTrack};
use anyhow::Result;
use client::{proto::PeerId, User};
use futures::StreamExt;
use gpui::{
    div, AppContext, Div, Element, EventEmitter, FocusHandle, FocusableView, ParentElement, Render,
    SharedString, Task, View, ViewContext, VisualContext, WindowContext,
};
use std::sync::{Arc, Weak};
use workspace::{item::Item, ItemNavHistory, WorkspaceId};

pub enum Event {
    Close,
}

pub struct SharedScreen {
    track: Weak<RemoteVideoTrack>,
    frame: Option<Frame>,
    // temporary addition just to render something interactive.
    current_frame_id: usize,
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
            current_frame_id: 0,
        }
    }
}

impl EventEmitter<Event> for SharedScreen {}
impl EventEmitter<workspace::item::ItemEvent> for SharedScreen {}

impl FocusableView for SharedScreen {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus.clone()
    }
}
impl Render for SharedScreen {
    type Element = Div;
    fn render(&mut self, _: &mut ViewContext<Self>) -> Self::Element {
        let frame = self.frame.clone();
        let frame_id = self.current_frame_id;
        self.current_frame_id = self.current_frame_id.wrapping_add(1);
        div().children(frame.map(|_| {
            ui::Label::new(frame_id.to_string()).color(ui::Color::Error)
            // img().data(Arc::new(ImageData::new(image::ImageBuffer::new(
            //     frame.width() as u32,
            //     frame.height() as u32,
            // ))))
        }))
    }
}
// impl View for SharedScreen {
//     fn ui_name() -> &'static str {
//         "SharedScreen"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         enum Focus {}

//         let frame = self.frame.clone();
//         MouseEventHandler::new::<Focus, _>(0, cx, |_, cx| {
//             Canvas::new(move |bounds, _, _, cx| {
//                 if let Some(frame) = frame.clone() {
//                     let size = constrain_size_preserving_aspect_ratio(
//                         bounds.size(),
//                         vec2f(frame.width() as f32, frame.height() as f32),
//                     );
//                     let origin = bounds.origin() + (bounds.size() / 2.) - size / 2.;
//                     cx.scene().push_surface(gpui::platform::mac::Surface {
//                         bounds: RectF::new(origin, size),
//                         image_buffer: frame.image(),
//                     });
//                 }
//             })
//             .contained()
//             .with_style(theme::current(cx).shared_screen)
//         })
//         .on_down(MouseButton::Left, |_, _, cx| cx.focus_parent())
//         .into_any()
//     }
// }

impl Item for SharedScreen {
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        Some(format!("{}'s screen", self.user.github_login).into())
    }
    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            nav_history.push::<()>(None, cx);
        }
    }

    fn tab_content(&self, _: Option<usize>, _: &WindowContext<'_>) -> gpui::AnyElement {
        div().child("Shared screen").into_any()
        // Flex::row()
        //     .with_child(
        //         Svg::new("icons/desktop.svg")
        //             .with_color(style.label.text.color)
        //             .constrained()
        //             .with_width(style.type_icon_width)
        //             .aligned()
        //             .contained()
        //             .with_margin_right(style.spacing),
        //     )
        //     .with_child(
        //         Label::new(
        //             format!("{}'s screen", self.user.github_login),
        //             style.label.clone(),
        //         )
        //         .aligned(),
        //     )
        //     .into_any()
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        let track = self.track.upgrade()?;
        Some(cx.build_view(|cx| Self::new(&track, self.peer_id, self.user.clone(), cx)))
    }
}
