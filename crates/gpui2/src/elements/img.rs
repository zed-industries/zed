use crate as gpui2;
use crate::style::{StyleHelpers, Styleable};
use crate::{style::Style, Element};
use futures::FutureExt;
use gpui::scene;
use gpui2_macros::IntoElement;
use refineable::RefinementCascade;
use util::arc_cow::ArcCow;
use util::ResultExt;

#[derive(IntoElement)]
pub struct Img {
    style: RefinementCascade<Style>,
    uri: Option<ArcCow<'static, str>>,
}

pub fn img() -> Img {
    Img {
        style: RefinementCascade::default(),
        uri: None,
    }
}

impl Img {
    pub fn uri(mut self, uri: impl Into<ArcCow<'static, str>>) -> Self {
        self.uri = Some(uri.into());
        self
    }
}

impl<V: 'static> Element<V> for Img {
    type PaintState = ();

    fn layout(
        &mut self,
        _: &mut V,
        cx: &mut crate::LayoutContext<V>,
    ) -> anyhow::Result<(gpui::LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        let style = self.computed_style();
        let layout_id = cx.add_layout_node(style, [])?;
        Ok((layout_id, ()))
    }

    fn paint(
        &mut self,
        _: &mut V,
        layout: &gpui::Layout,
        _: &mut Self::PaintState,
        cx: &mut crate::paint_context::PaintContext<V>,
    ) where
        Self: Sized,
    {
        let style = self.computed_style();

        style.paint_background(layout.bounds, cx);

        if let Some(uri) = &self.uri {
            let image_future = cx.image_cache.get(uri.clone());
            if let Some(data) = image_future
                .clone()
                .now_or_never()
                .and_then(ResultExt::log_err)
            {
                let rem_size = cx.rem_size();
                cx.scene.push_image(scene::Image {
                    bounds: layout.bounds,
                    border: gpui::Border {
                        color: style.border_color.unwrap_or_default().into(),
                        top: style.border_widths.top.to_pixels(rem_size),
                        right: style.border_widths.right.to_pixels(rem_size),
                        bottom: style.border_widths.bottom.to_pixels(rem_size),
                        left: style.border_widths.left.to_pixels(rem_size),
                    },
                    corner_radii: style.corner_radii.to_gpui(rem_size),
                    grayscale: false,
                    data,
                })
            } else {
                cx.spawn(|this, mut cx| async move {
                    if image_future.await.log_err().is_some() {
                        this.update(&mut cx, |_, cx| cx.notify()).ok();
                    }
                })
                .detach();
            }
        }
    }
}

impl Styleable for Img {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}

impl StyleHelpers for Img {}
