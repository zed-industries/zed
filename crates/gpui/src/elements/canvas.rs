use refineable::Refineable as _;

use crate::{Bounds, Element, ElementContext, IntoElement, Pixels, Style, StyleRefinement, Styled};

/// Construct a canvas element with the given paint callback.
/// Useful for adding short term custom drawing to a view.
pub fn canvas(callback: impl 'static + FnOnce(&Bounds<Pixels>, &mut ElementContext)) -> Canvas {
    Canvas {
        paint_callback: Some(Box::new(callback)),
        style: StyleRefinement::default(),
    }
}

/// A canvas element, meant for accessing the low level paint API without defining a whole
/// custom element
pub struct Canvas {
    paint_callback: Option<Box<dyn FnOnce(&Bounds<Pixels>, &mut ElementContext)>>,
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
        cx: &mut ElementContext,
    ) -> (crate::LayoutId, Self::State) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.request_layout(&style, []);
        (layout_id, style)
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, style: &mut Style, cx: &mut ElementContext) {
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
