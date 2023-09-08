use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
pub use gpui::taffy::tree::NodeId;
use gpui::{
    scene::EventHandler, EventContext, Layout, LayoutId, PaintContext as LegacyPaintContext,
};
use std::{any::TypeId, rc::Rc};

#[derive(Deref, DerefMut)]
pub struct PaintContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
}

impl<'a, 'b, 'c, 'd, V: 'static> PaintContext<'a, 'b, 'c, 'd, V> {
    pub fn new(legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>) -> Self {
        Self { legacy_cx }
    }

    pub fn on_event<E: 'static>(
        &mut self,
        order: u32,
        handler: impl Fn(&mut V, &E, &mut EventContext<V>) + 'static,
    ) {
        let view = self.weak_handle();

        self.scene().event_handlers.push(EventHandler {
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

    pub(crate) fn computed_layout(&mut self, layout_id: LayoutId) -> Result<Layout> {
        self.layout_engine()
            .ok_or_else(|| anyhow!("no layout engine present"))?
            .computed_layout(layout_id)
    }
}
