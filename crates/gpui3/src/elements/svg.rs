use crate::{Element, Layout, LayoutId, Result, Style, StyleHelpers, Styled};
use refineable::RefinementCascade;
use std::{borrow::Cow, marker::PhantomData};

pub struct Svg<S> {
    path: Option<Cow<'static, str>>,
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
    pub fn path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
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
        _layout: Layout,
        _: &mut Self::State,
        _: &mut Self::FrameState,
        _cx: &mut crate::ViewContext<S>,
    ) -> Result<()>
    where
        Self: Sized,
    {
        // todo!
        // let fill_color = self.computed_style().fill.and_then(|fill| fill.color());
        // if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
        //     if let Some(svg_tree) = cx.asset_cache.svg(path).log_err() {
        //         let icon = scene::Icon {
        //             bounds: layout.bounds + parent_origin,
        //             svg: svg_tree,
        //             path: path.clone(),
        //             color: Rgba::from(fill_color).into(),
        //         };

        //         cx.scene().push_icon(icon);
        //     }
        // }
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
