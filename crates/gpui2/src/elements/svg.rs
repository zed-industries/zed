use crate::{
    self as gpui2, scene,
    style::{Style, StyleHelpers, Styleable},
    Element, IntoElement, Layout, LayoutId, Rgba,
};
use gpui::geometry::vector::Vector2F;
use refineable::RefinementCascade;
use std::borrow::Cow;
use util::ResultExt;

#[derive(IntoElement)]
pub struct Svg {
    path: Option<Cow<'static, str>>,
    style: RefinementCascade<Style>,
}

pub fn svg() -> Svg {
    Svg {
        path: None,
        style: RefinementCascade::<Style>::default(),
    }
}

impl Svg {
    pub fn path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V: 'static> Element<V> for Svg {
    type PaintState = ();

    fn layout(
        &mut self,
        _: &mut V,
        cx: &mut crate::ViewContext<V>,
    ) -> anyhow::Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        let style = self.computed_style();
        Ok((cx.add_layout_node(style, [])?, ()))
    }

    fn paint(
        &mut self,
        _: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        _: &mut Self::PaintState,
        cx: &mut crate::paint_context::PaintContext<V>,
    ) where
        Self: Sized,
    {
        let fill_color = self.computed_style().fill.and_then(|fill| fill.color());
        if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
            if let Some(svg_tree) = cx.asset_cache.svg(path).log_err() {
                let icon = scene::Icon {
                    bounds: layout.bounds + parent_origin,
                    svg: svg_tree,
                    path: path.clone(),
                    color: Rgba::from(fill_color).into(),
                };

                cx.scene().push_icon(icon);
            }
        }
    }
}

impl Styleable for Svg {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut refineable::RefinementCascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}

impl StyleHelpers for Svg {}
