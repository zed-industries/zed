use crate::{Bounds, Element, LayoutId, Pixels, SharedString, Style, Styled};
use refineable::Cascade;
use std::marker::PhantomData;
use util::ResultExt;

pub struct Svg<S> {
    path: Option<SharedString>,
    style: Cascade<Style>,
    state_type: PhantomData<S>,
}

pub fn svg<S>() -> Svg<S> {
    Svg {
        path: None,
        style: Cascade::<Style>::default(),
        state_type: PhantomData,
    }
}

impl<S> Svg<S> {
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<S: 'static + Send + Sync> Element for Svg<S> {
    type ViewState = S;
    type ElementState = ();

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn layout(
        &mut self,
        _: &mut S,
        _: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<S>,
    ) -> (LayoutId, Self::ElementState)
    where
        Self: Sized,
    {
        let style = self.computed_style();
        (cx.request_layout(style, []), ())
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::ViewState,
        _: &mut Self::ElementState,
        cx: &mut crate::ViewContext<S>,
    ) where
        Self: Sized,
    {
        let fill_color = self.computed_style().fill.and_then(|fill| fill.color());
        if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
            cx.paint_svg(bounds, path.clone(), fill_color).log_err();
        }
    }
}

impl<S: 'static + Send + Sync> Styled for Svg<S> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut refineable::Cascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}
