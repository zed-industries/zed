use crate::{
    Bounds, Element, ElementContext, InteractiveElement, Interactivity, IntoElement, LayoutId,
    Pixels, SharedString, StyleRefinement, Styled,
};
use util::ResultExt;

/// An SVG element.
pub struct Svg {
    interactivity: Interactivity,
    path: Option<SharedString>,
}

/// Create a new SVG element.
pub fn svg() -> Svg {
    Svg {
        interactivity: Interactivity::default(),
        path: None,
    }
}

impl Svg {
    /// Set the path to the SVG file for this element.
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl Element for Svg {
    type FrameState = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::FrameState) {
        let layout_id = self
            .interactivity
            .layout(cx, |style, cx| cx.before_layout(&style, None));
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        _frame_state: &mut Self::FrameState,
        cx: &mut ElementContext,
    ) {
        self.interactivity
            .after_layout(bounds, bounds.size, cx, |_, _, _| {})
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _frame_state: &mut Self::FrameState,
        cx: &mut ElementContext,
    ) where
        Self: Sized,
    {
        self.interactivity.paint(bounds, cx, |style, cx| {
            if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                cx.paint_svg(bounds, path.clone(), color).log_err();
            }
        })
    }
}

impl IntoElement for Svg {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Svg {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Svg {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}
