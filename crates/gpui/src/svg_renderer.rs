use crate::{AssetSource, DevicePixels, IsZero, Result, SharedString, Size};
use anyhow::anyhow;
use std::{
    hash::Hash,
    sync::{Arc, OnceLock},
};

#[derive(Clone, PartialEq, Hash, Eq)]
pub(crate) struct RenderSvgParams {
    pub(crate) path: SharedString,
    pub(crate) size: Size<DevicePixels>,
}

pub(crate) struct SvgRenderer {
    asset_source: Arc<dyn AssetSource>,
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
        let tree =
            resvg::usvg::Tree::from_data(&bytes, &resvg::usvg::Options::default(), svg_fontdb())?;

        // Render the SVG to a pixmap with the specified width and height.
        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(params.size.width.into(), params.size.height.into())
                .unwrap();

        let ratio = params.size.width.0 as f32 / tree.size().width();
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::from_scale(ratio, ratio),
            &mut pixmap.as_mut(),
        );

        // Convert the pixmap's pixels into an alpha mask.
        let alpha_mask = pixmap
            .pixels()
            .iter()
            .map(|p| p.alpha())
            .collect::<Vec<_>>();
        Ok(alpha_mask)
    }
}

/// Returns the global font database used for SVG rendering.
fn svg_fontdb() -> &'static resvg::usvg::fontdb::Database {
    static FONTDB: OnceLock<resvg::usvg::fontdb::Database> = OnceLock::new();
    FONTDB.get_or_init(|| {
        let mut fontdb = resvg::usvg::fontdb::Database::new();
        fontdb.load_system_fonts();
        fontdb
    })
}
