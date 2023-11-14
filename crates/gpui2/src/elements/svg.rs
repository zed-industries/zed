use crate::{
    AnyElement, Bounds, Component, Element, ElementId, InteractiveComponent,
    InteractiveElementState, Interactivity, LayoutId, Pixels, SharedString, StyleRefinement,
    Styled, ViewContext,
};
use util::ResultExt;

pub struct Svg<V: 'static> {
    interactivity: Interactivity<V>,
    path: Option<SharedString>,
}

pub fn svg<V: 'static>() -> Svg<V> {
    Svg {
        interactivity: Interactivity::default(),
        path: None,
    }
}

impl<V> Svg<V> {
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V> Component<V> for Svg<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V> Element<V> for Svg<V> {
    type ElementState = InteractiveElementState;

    fn element_id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        self.interactivity.initialize(element_state, cx)
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        self.interactivity.layout(element_state, cx, |style, cx| {
            cx.request_layout(&style, None)
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
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

impl<V> Styled for Svg<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<V> InteractiveComponent<V> for Svg<V> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        &mut self.interactivity
    }
}
