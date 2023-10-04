use crate::{Element, Layout, LayoutId, Result, SharedString, Style, StyleHelpers, Styled};
use refineable::RefinementCascade;
use std::marker::PhantomData;

pub struct Svg<S> {
    path: Option<SharedString>,
    style: RefinementCascade<Style>,
    state_type: PhantomData<S>,
}

pub fn svg<S>() -> Svg<S> {
    Svg {
        path: None,
        style: RefinementCascade::<Style>::default(),
        state_type: PhantomData,
    }
}

impl<S> Svg<S> {
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<S: 'static> Element for Svg<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        _: &mut S,
        cx: &mut crate::ViewContext<S>,
    ) -> anyhow::Result<(LayoutId, Self::FrameState)>
    where
        Self: Sized,
    {
        let style = self.computed_style();
        Ok((cx.request_layout(style, [])?, ()))
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        _: &mut Self::FrameState,
        cx: &mut crate::ViewContext<S>,
    ) -> Result<()>
    where
        Self: Sized,
    {
        let fill_color = self.computed_style().fill.and_then(|fill| fill.color());
        if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
            cx.paint_svg(layout.bounds, layout.order, path.clone(), fill_color)?;
        }
        Ok(())
    }
}

impl<S> Styled for Svg<S> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut refineable::RefinementCascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}

impl<S> StyleHelpers for Svg<S> {}
