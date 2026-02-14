use std::sync::Arc;

use anyhow::Result;
use gpui::{RenderImage, SvgRenderer};

use super::FilePreviewFormat;

pub struct SvgFormat;

impl FilePreviewFormat for SvgFormat {
    fn extensions(&self) -> &'static [&'static str] {
        &["svg"]
    }

    fn display_name(&self) -> &'static str {
        "SVG"
    }

    fn telemetry_event(&self) -> &'static str {
        "svg preview: open"
    }

    fn render(&self, content: String, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>> {
        svg_renderer
            .render_single_frame(content.as_bytes(), 1.0, true)
            .map_err(|e| anyhow::anyhow!("{e}"))
    }
}
