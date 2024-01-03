use refineable::Refineable as _;

use crate::{Bounds, Element, IntoElement, Pixels, Style, StyleRefinement, Styled, WindowContext};

pub fn canvas(callback: impl 'static + FnOnce(&Bounds<Pixels>, &mut WindowContext)) -> Canvas {
    Canvas {
        paint_callback: Some(Box::new(callback)),
        style: StyleRefinement::default(),
    }
}

pub struct Canvas {
    paint_callback: Option<Box<dyn FnOnce(&Bounds<Pixels>, &mut WindowContext)>>,
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
    type State = Style;

    fn request_layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (crate::LayoutId, Self::State) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.request_layout(&style, []);
        (layout_id, style)
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, style: &mut Style, cx: &mut WindowContext) {
        style.paint(bounds, cx, |cx| {
            (self.paint_callback.take().unwrap())(&bounds, cx)
        });
    }
}

impl Styled for Canvas {
    fn style(&mut self) -> &mut crate::StyleRefinement {
        &mut self.style
    }
}
