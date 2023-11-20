use crate::{
    Bounds, Element, ElementId, InteractiveElement, InteractiveElementState, Interactivity,
    LayoutId, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, ViewContext,
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

impl<V> Element<V> for Svg<V> {
    type State = InteractiveElementState;

    fn layout(
        &mut self,
        _view_state: &mut V,
        element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        self.interactivity.layout(element_state, cx, |style, cx| {
            cx.request_layout(&style, None)
        })
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        _view_state: &mut V,
        element_state: &mut Self::State,
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

impl<V: 'static> RenderOnce<V> for Svg<V> {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

impl<V> Styled for Svg<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<V> InteractiveElement<V> for Svg<V> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        &mut self.interactivity
    }
}
