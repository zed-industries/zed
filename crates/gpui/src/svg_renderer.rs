use crate::{AssetSource, DevicePixels, IsZero, Result, SharedString, Size};
use anyhow::anyhow;
use resvg::tiny_skia::Pixmap;
use std::{hash::Hash, sync::Arc};

#[derive(Clone, PartialEq, Hash, Eq)]
pub(crate) struct RenderSvgParams {
    pub(crate) path: SharedString,
    pub(crate) size: Size<DevicePixels>,
}

#[derive(Clone)]
pub(crate) struct SvgRenderer {
    asset_source: Arc<dyn AssetSource>,
}

pub enum SvgSize {
    Size(Size<DevicePixels>),
    ScaleFactor(f32),
}

impl SvgRenderer {
    pub fn new(asset_source: Arc<dyn AssetSource>) -> Self {
        Self { asset_source }
    }

    pub fn render(&self, params: &RenderSvgParams) -> Result<Option<Vec<u8>>> {
        if params.size.is_zero() {
            return Err(anyhow!("can't render at a zero size"));
        }

        // Load the tree.
        let Some(bytes) = self.asset_source.load(&params.path)? else {
            return Ok(None);
        };

        let pixmap = self.render_pixmap(&bytes, SvgSize::Size(params.size))?;

        // Convert the pixmap's pixels into an alpha mask.
        let alpha_mask = pixmap
            .pixels()
            .iter()
            .map(|p| p.alpha())
            .collect::<Vec<_>>();
        Ok(Some(alpha_mask))
    }

    pub fn render_pixmap(&self, bytes: &[u8], size: SvgSize) -> Result<Pixmap, usvg::Error> {
        let tree = usvg::Tree::from_data(&bytes, &usvg::Options::default())?;

        let size = match size {
            SvgSize::Size(size) => size,
            SvgSize::ScaleFactor(scale) => crate::size(
                DevicePixels((tree.size().width() * scale) as i32),
                DevicePixels((tree.size().height() * scale) as i32),
            ),
        };

        // Render the SVG to a pixmap with the specified width and height.
        let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width.into(), size.height.into())
            .ok_or(usvg::Error::InvalidSize)?;

        let transform = tree.view_box().to_transform(
            resvg::tiny_skia::Size::from_wh(size.width.0 as f32, size.height.0 as f32)
                .ok_or(usvg::Error::InvalidSize)?,
        );

        resvg::render(&tree, transform, &mut pixmap.as_mut());

        Ok(pixmap)
    }
}
