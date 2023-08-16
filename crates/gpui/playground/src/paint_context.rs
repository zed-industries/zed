use std::{
    any::{Any, TypeId},
    collections::BTreeSet,
    rc::Rc,
};

use derive_more::{Deref, DerefMut};
use gpui::{geometry::rect::RectF, EventContext, RenderContext, ViewContext, WindowContext};
pub use gpui::{LayoutContext, PaintContext as LegacyPaintContext};
pub use taffy::tree::NodeId;

#[derive(Deref, DerefMut)]
pub struct PaintContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
    pub(crate) scene: &'d mut gpui::SceneBuilder,
    regions: BTreeSet<InteractiveRegion>,
}

impl<V> RenderContext for PaintContext<'_, '_, '_, '_, V> {
    fn text_style(&self) -> gpui::fonts::TextStyle {
        self.legacy_cx.text_style()
    }

    fn push_text_style(&mut self, style: gpui::fonts::TextStyle) {
        self.legacy_cx.push_text_style(style)
    }

    fn pop_text_style(&mut self) {
        self.legacy_cx.pop_text_style()
    }
}

impl<'a, 'b, 'c, 'd, V: 'static> PaintContext<'a, 'b, 'c, 'd, V> {
    pub fn new(
        legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
        scene: &'d mut gpui::SceneBuilder,
    ) -> Self {
        Self {
            legacy_cx,
            scene,
            regions: BTreeSet::new(),
        }
    }

    pub fn paint_interactive<E: 'static>(
        &mut self,
        order: u32,
        bounds: RectF,
        handler: impl Fn(&mut V, E, &mut EventContext<V>) + 'static,
    ) {
        self.regions.insert(InteractiveRegion {
            order,
            bounds,
            event_handler: Rc::new(move |view, event, window_cx, view_id| {
                let mut cx = ViewContext::mutable(window_cx, view_id);
                let mut cx = EventContext::new(&mut cx);
                handler(
                    view.downcast_mut().unwrap(),
                    *event.downcast().unwrap(),
                    &mut cx,
                )
            }),
            event_type: TypeId::of::<E>(),
        });
    }
}

struct InteractiveRegion {
    order: u32,
    bounds: RectF,
    event_handler: Rc<dyn Fn(&mut dyn Any, Box<dyn Any>, &mut WindowContext, usize)>,
    event_type: TypeId,
}

impl Eq for InteractiveRegion {}

impl PartialEq for InteractiveRegion {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl PartialOrd for InteractiveRegion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for InteractiveRegion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
