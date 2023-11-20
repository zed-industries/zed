use crate::{
    AnyElement, Bounds, Component, Element, ElementId, InteractiveComponent,
    InteractiveElementState, Interactivity, LayoutId, Pixels, SharedString, StyleRefinement,
    Styled, WindowContext,
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

impl Component for Svg {
    fn render(self) -> AnyElement {
        AnyElement::new(self)
    }
}

impl Element for Svg {
    type ElementState = InteractiveElementState;

    fn element_id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn layout(
        &mut self,
        element_state: Option<Self::ElementState>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::ElementState) {
        self.interactivity.layout(element_state, cx, |style, cx| {
            cx.request_layout(&style, None)
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        element_state: &mut Self::ElementState,
        cx: &mut WindowContext,
    ) where
        Self: Sized,
    {
        self.interactivity
            .paint(bounds, bounds.size, element_state, cx, |style, _, cx| {
                if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                    cx.paint_svg(bounds, path.clone(), color).log_err();
                }
            })
    }
}

impl Styled for Svg {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveComponent for Svg {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}
