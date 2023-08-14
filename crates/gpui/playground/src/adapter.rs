use crate::element::{LayoutContext, PaintContext};
use gpui::geometry::rect::RectF;
use util::ResultExt;

use crate::element::AnyElement;

#[derive(Clone)]
pub struct Adapter<V> {
    view: V,
    element: AnyElement<V>,
}

impl<V: 'static> gpui::Element<Adapter<V>> for Adapter<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut Self,
        legacy_cx: &mut gpui::LayoutContext<Self>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        legacy_cx.push_layout_engine();
        let node = self
            .element
            .layout(&mut self.view, &mut LayoutContext { legacy_cx })
            .log_err();

        if let Some(node) = node {
            let layout_engine = legacy_cx.layout_engine().unwrap();
            layout_engine.compute_layout(node, constraint.max).log_err();
        }
        legacy_cx.pop_layout_engine();

        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut (),
        adapter: &mut Self,
        legacy_cx: &mut gpui::PaintContext<Self>,
    ) -> Self::PaintState {
        let mut cx = PaintContext { legacy_cx, scene };
        self.element.paint(&mut adapter.view, &mut cx).log_err();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &Adapter<V>,
        cx: &gpui::ViewContext<Adapter<V>>,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &Adapter<V>,
        cx: &gpui::ViewContext<Adapter<V>>,
    ) -> gpui::serde_json::Value {
        todo!()
    }
}

impl<V: 'static> gpui::Entity for Adapter<V> {
    type Event = ();
}

impl<V: 'static> gpui::View for Adapter<V>
where
    V: Clone,
{
    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        gpui::Element::into_any(self.clone())
    }
}
