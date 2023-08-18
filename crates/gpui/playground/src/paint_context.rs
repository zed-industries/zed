use std::{any::TypeId, rc::Rc};

use derive_more::{Deref, DerefMut};
use gpui::{geometry::rect::RectF, EventContext, RenderContext, ViewContext};
pub use gpui::{LayoutContext, PaintContext as LegacyPaintContext};
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

    pub fn draw_interactive_region<E: 'static>(
        &mut self,
        order: u32,
        bounds: RectF,
        outside_bounds: bool,
        event_handler: impl Fn(&mut V, &E, &mut EventContext<V>) + 'static,
    ) {
        // We'll sort these later when `take_interactive_regions` is called.
        self.scene
            .interactive_regions
            .push(gpui::scene::InteractiveRegion {
                order,
                bounds,
                outside_bounds,
                event_handler: Rc::new(move |view, event, window_cx, view_id| {
                    let mut view_context = ViewContext::mutable(window_cx, view_id);
                    let mut event_context = EventContext::new(&mut view_context);
                    event_handler(
                        view.downcast_mut().unwrap(),
                        event.downcast_ref().unwrap(),
                        &mut event_context,
                    );
                }),
                event_type: TypeId::of::<E>(),
                view_id: self.view_id(),
            });
    }
}
