use std::{any::TypeId, rc::Rc};

use crate::{element::LayoutId, style::Style};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui::{geometry::Size, scene::EventHandler, EventContext, Layout, MeasureParams};
pub use gpui::{taffy::tree::NodeId, ViewContext as LegacyViewContext};

#[derive(Deref, DerefMut)]
pub struct ViewContext<'a, 'b, 'c, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'c mut LegacyViewContext<'a, 'b, V>,
}

impl<'a, 'b, 'c, V: 'static> ViewContext<'a, 'b, 'c, V> {
    pub fn new(legacy_cx: &'c mut LegacyViewContext<'a, 'b, V>) -> Self {
        Self { legacy_cx }
    }

    pub fn add_layout_node(
        &mut self,
        style: Style,
        children: impl IntoIterator<Item = NodeId>,
    ) -> Result<LayoutId> {
        let rem_size = self.rem_size();
        let style = style.to_taffy(rem_size);
        let id = self
            .legacy_cx
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(style, children)?;

        Ok(id)
    }

    pub fn add_measured_layout_node<F>(&mut self, style: Style, measure: F) -> Result<LayoutId>
    where
        F: Fn(MeasureParams) -> Size<f32> + Sync + Send + 'static,
    {
        let rem_size = self.rem_size();
        let layout_id = self
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_measured_node(style.to_taffy(rem_size), measure)?;

        Ok(layout_id)
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
