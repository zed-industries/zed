use crate::{Element, Layout, LayoutId, Result, Style, Styled};
use refineable::RefinementCascade;
use std::marker::PhantomData;
use util::arc_cow::ArcCow;

pub struct Img<S> {
    style: RefinementCascade<Style>,
    uri: Option<ArcCow<'static, str>>,
    state_type: PhantomData<S>,
}

pub fn img<S>() -> Img<S> {
    Img {
        style: RefinementCascade::default(),
        uri: None,
        state_type: PhantomData,
    }
}

impl<S> Img<S> {
    pub fn uri(mut self, uri: impl Into<ArcCow<'static, str>>) -> Self {
        self.uri = Some(uri.into());
        self
    }
}

impl<S: 'static> Element for Img<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut crate::ViewContext<Self::State>,
    ) -> anyhow::Result<(LayoutId, Self::FrameState)>
    where
        Self: Sized,
    {
        let style = self.computed_style();
        let layout_id = cx.request_layout(style, [])?;
        Ok((layout_id, ()))
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        _: &mut Self::FrameState,
        cx: &mut crate::ViewContext<Self::State>,
    ) -> Result<()> {
        let style = self.computed_style();
        let bounds = layout.bounds;

        style.paint_background(bounds, cx);

        // if let Some(uri) = &self.uri {
        //     let image_future = cx.image_cache.get(uri.clone());
        //     if let Some(data) = image_future
        //         .clone()
        //         .now_or_never()
        //         .and_then(ResultExt::log_err)
        //     {
        //         let rem_size = cx.rem_size();
        //         cx.scene().push_image(scene::Image {
        //             bounds,
        //             border: gpui::Border {
        //                 color: style.border_color.unwrap_or_default().into(),
        //                 top: style.border_widths.top.to_pixels(rem_size),
        //                 right: style.border_widths.right.to_pixels(rem_size),
        //                 bottom: style.border_widths.bottom.to_pixels(rem_size),
        //                 left: style.border_widths.left.to_pixels(rem_size),
        //             },
        //             corner_radii: style.corner_radii.to_gpui(bounds.size(), rem_size),
        //             grayscale: false,
        //             data,
        //         })
        //     } else {
        //         cx.spawn(|this, mut cx| async move {
        //             if image_future.await.log_err().is_some() {
        //                 this.update(&mut cx, |_, cx| cx.notify()).ok();
        //             }
        //         })
        //         .detach();
        //     }
        // }
        Ok(())
    }
}

impl<S> Styled for Img<S> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}
