use std::{cell::Cell, marker::PhantomData, rc::Rc};

use gpui::{
    geometry::{rect::RectF, vector::Vector2F},
    scene::MouseMove,
    EngineLayout, ViewContext,
};
use refineable::Refineable;

use crate::{element::Element, style::StyleRefinement};

pub struct Hoverable<V, E> {
    hover_style: StyleRefinement,
    computed_style: Option<StyleRefinement>,
    view_type: PhantomData<V>,
    child: E,
}

impl<V, E> Hoverable<V, E> {
    pub fn new(child: E) -> Self {
        Self {
            hover_style: StyleRefinement::default(),
            computed_style: None,
            view_type: PhantomData,
            child,
        }
    }
}

impl<V: 'static, E: Element<V>> Element<V> for Hoverable<V, E> {
    type Layout = E::Layout;

    fn declared_style(&mut self) -> &mut StyleRefinement {
        &mut self.hover_style
    }

    fn computed_style(&mut self, cx: &mut ViewContext<V>) -> &StyleRefinement {
        self.computed_style.get_or_insert_with(|| {
            let mut style = self.child.computed_style(cx).clone();
            // if hovered
            style.refine(&self.hover_style);
            style
        })
    }

    fn handlers_mut(&mut self) -> &mut Vec<crate::element::EventHandler<V>> {
        self.child.handlers_mut()
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut gpui::LayoutContext<V>,
    ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
        self.child.layout(view, cx)
    }

    fn paint<'a>(
        &mut self,
        layout: crate::element::Layout<Self::Layout>,
        view: &mut V,
        cx: &mut crate::element::PaintContext<V>,
    ) -> anyhow::Result<()> {
        let EngineLayout { bounds, order } = layout.from_engine;
        let window_bounds = RectF::new(Vector2F::zero(), cx.window_size());
        let was_hovered = Rc::new(Cell::new(false));

        self.child.paint(layout, view, cx)?;
        cx.draw_interactive_region(
            order,
            window_bounds,
            false,
            move |view, event: &MouseMove, cx| {
                let is_hovered = bounds.contains_point(cx.mouse_position());
                if is_hovered != was_hovered.get() {
                    was_hovered.set(is_hovered);
                    cx.repaint();
                }
            },
        );
        Ok(())
    }
}
