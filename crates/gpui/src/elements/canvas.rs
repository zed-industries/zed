use refineable::Refineable as _;

use crate::{Bounds, Element, ElementContext, IntoElement, Pixels, Style, StyleRefinement, Styled};

/// Construct a canvas element with the given paint callback.
/// Useful for adding short term custom drawing to a view.
pub fn canvas(paint: impl 'static + FnOnce(Bounds<Pixels>, &mut ElementContext)) -> Canvas {
    Canvas {
        paint: Some(Box::new(paint)),
        style: StyleRefinement::default(),
    }
}

/// A canvas element, meant for accessing the low level paint API without defining a whole
/// custom element
pub struct Canvas {
    paint: Option<Box<dyn FnOnce(Bounds<Pixels>, &mut ElementContext)>>,
    style: StyleRefinement,
}

impl IntoElement for Canvas {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Canvas {
    type BeforeLayout = Style;
    type AfterLayout = ();
    type BeforePaint = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (crate::LayoutId, Self::BeforeLayout) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.request_layout(&style, []);
        (layout_id, style)
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        _cx: &mut ElementContext,
    ) -> (Option<Bounds<Pixels>>, Self::AfterLayout) {
        (None, ())
    }

    fn before_paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _before_layout: &mut Style,
        _after_layout: &mut Self::AfterLayout,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        style: &mut Style,
        _after_layout: &mut Self::AfterLayout,
        _before_paint: &mut Self::BeforePaint,
        cx: &mut ElementContext,
    ) {
        style.paint(bounds, cx, |cx| (self.paint.take().unwrap())(bounds, cx));
    }
}

impl Styled for Canvas {
    fn style(&mut self) -> &mut crate::StyleRefinement {
        &mut self.style
    }
}
