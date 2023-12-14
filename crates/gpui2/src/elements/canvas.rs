use refineable::Refineable as _;

use crate::{Bounds, Element, IntoElement, Pixels, Style, StyleRefinement, Styled, WindowContext};

pub fn canvas(callback: impl 'static + FnOnce(&Bounds<Pixels>, &mut WindowContext)) -> Canvas {
    Canvas {
        paint_callback: Box::new(callback),
        style: StyleRefinement::default(),
    }
}

pub struct Canvas {
    paint_callback: Box<dyn FnOnce(&Bounds<Pixels>, &mut WindowContext)>,
    style: StyleRefinement,
}

impl IntoElement for Canvas {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Canvas {
    type State = ();

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (crate::LayoutId, Self::State) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.request_layout(&style, []);
        (layout_id, ())
    }

    fn paint(self, bounds: Bounds<Pixels>, _: &mut (), cx: &mut WindowContext) {
        (self.paint_callback)(&bounds, cx)
    }
}

impl Styled for Canvas {
    fn style(&mut self) -> &mut crate::StyleRefinement {
        &mut self.style
    }
}
