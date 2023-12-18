use crate::{
    Bounds, Element, ElementId, InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels,
    SharedString, StyleRefinement, Styled, WindowContext,
};
use util::ResultExt;

pub struct Svg {
    interactivity: Interactivity,
    path: Option<SharedString>,
}

pub fn svg() -> Svg {
    Svg {
        interactivity: Interactivity::default(),
        path: None,
    }
}

impl Svg {
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl Element for Svg {
    type FrameState = ();

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::FrameState) {
        let layout_id = self
            .interactivity
            .layout(cx, |style, _, cx| cx.request_layout(&style, None));
        (layout_id, ())
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, _: &mut Self::FrameState, cx: &mut WindowContext)
    where
        Self: Sized,
    {
        self.interactivity
            .paint(bounds, bounds.size, cx, |style, _, _, cx| {
                if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                    cx.paint_svg(bounds, path.clone(), color).log_err();
                }
            })
    }
}

impl IntoElement for Svg {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

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
