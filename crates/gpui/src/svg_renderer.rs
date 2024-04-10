use crate::{AssetSource, DevicePixels, IsZero, Result, SharedString, Size};
use anyhow::anyhow;
use std::{hash::Hash, sync::Arc};
use tiny_skia::Pixmap;

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

    pub fn render(&self, params: &RenderSvgParams) -> Result<Vec<u8>> {
        if params.size.is_zero() {
            return Err(anyhow!("can't render at a zero size"));
        }

        // Load the tree.
        let bytes = self.asset_source.load(&params.path)?;

        let pixmap = self.render_pixmap(&bytes, SvgSize::Size(params.size))?;

        // Convert the pixmap's pixels into an alpha mask.
        let alpha_mask = pixmap
            .pixels()
            .iter()
            .map(|p| p.alpha())
            .collect::<Vec<_>>();
        Ok(alpha_mask)
    }

    pub fn render_pixmap(&self, bytes: &[u8], size: SvgSize) -> Result<Pixmap, usvg::Error> {
        let tree = usvg::Tree::from_data(&bytes, &usvg::Options::default())?;

        let tree_size = tree.svg_node().size;

        let size = match size {
            SvgSize::Size(size) => size,
            SvgSize::ScaleFactor(scale) => crate::size(
                DevicePixels((tree_size.width() * scale as f64) as i32),
                DevicePixels((tree_size.height() * scale as f64) as i32),
            ),
        };

        // Render the SVG to a pixmap with the specified width and height.
        let mut pixmap = tiny_skia::Pixmap::new(size.width.into(), size.height.into()).unwrap();

        resvg::render(
            &tree,
            usvg::FitTo::Width(size.width.into()),
            pixmap.as_mut(),
        );

        Ok(pixmap)
    }
}
