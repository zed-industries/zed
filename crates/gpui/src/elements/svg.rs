use crate::{
    Bounds, Element, ElementContext, Hitbox, InteractiveElement, Interactivity, IntoElement,
    LayoutId, Pixels, SharedString, StyleRefinement, Styled,
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
    type BeforeLayout = ();
    type AfterLayout = Option<Hitbox>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let layout_id = self
            .interactivity
            .before_layout(cx, |style, cx| cx.request_layout(&style, None));
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Option<Hitbox> {
        self.interactivity
            .after_layout(bounds, bounds.size, cx, |_, _, hitbox, _| hitbox)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        hitbox: &mut Option<Hitbox>,
        cx: &mut ElementContext,
    ) where
        Self: Sized,
    {
        self.interactivity
            .paint(bounds, hitbox.as_ref(), cx, |style, cx| {
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
