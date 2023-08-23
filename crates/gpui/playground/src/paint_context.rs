use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui::{scene::EventHandler, EngineLayout, EventContext, LayoutId, RenderContext, ViewContext};
pub use gpui::{LayoutContext, PaintContext as LegacyPaintContext};
use std::{any::TypeId, rc::Rc};
pub use taffy::tree::NodeId;

#[derive(Deref, DerefMut)]
pub struct PaintContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
    pub(crate) scene: &'d mut gpui::SceneBuilder,
}

impl<'a, 'b, V> RenderContext<'a, 'b, V> for PaintContext<'a, 'b, '_, '_, V> {
    fn text_style(&self) -> gpui::fonts::TextStyle {
        self.legacy_cx.text_style()
    }

    fn push_text_style(&mut self, style: gpui::fonts::TextStyle) {
        self.legacy_cx.push_text_style(style)
    }

    fn pop_text_style(&mut self) {
        self.legacy_cx.pop_text_style()
    }

    fn as_view_context(&mut self) -> &mut ViewContext<'a, 'b, V> {
        &mut self.view_context
    }
}

impl<'a, 'b, 'c, 'd, V: 'static> PaintContext<'a, 'b, 'c, 'd, V> {
    pub fn new(
        legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
        scene: &'d mut gpui::SceneBuilder,
    ) -> Self {
        Self { legacy_cx, scene }
    }

    pub fn on_event<E: 'static>(
        &mut self,
        order: u32,
        handler: impl Fn(&mut V, &E, &mut EventContext<V>) + 'static,
    ) {
        let view = self.weak_handle();

        self.scene.event_handlers.push(EventHandler {
            order,
            handler: Rc::new(move |event, window_cx| {
                if let Some(view) = view.upgrade(window_cx) {
                    view.update(window_cx, |view, view_cx| {
                        let mut event_cx = EventContext::new(view_cx);
                        handler(view, event.downcast_ref().unwrap(), &mut event_cx);
                        event_cx.bubble
                    })
                } else {
                    true
                }
            }),
            event_type: TypeId::of::<E>(),
        })
    }

    pub(crate) fn computed_layout(&mut self, layout_id: LayoutId) -> Result<EngineLayout> {
        self.layout_engine()
            .ok_or_else(|| anyhow!("no layout engine present"))?
            .computed_layout(layout_id)
    }
}
