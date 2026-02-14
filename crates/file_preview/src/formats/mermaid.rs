use std::sync::Arc;

use anyhow::{Result, anyhow};
use gpui::{RenderImage, SvgRenderer};

use super::FilePreviewFormat;

pub struct MermaidFormat;

impl FilePreviewFormat for MermaidFormat {
    fn extensions(&self) -> &'static [&'static str] {
        &["mmd", "mermaid"]
    }

    fn display_name(&self) -> &'static str {
        "Mermaid"
    }

    fn telemetry_event(&self) -> &'static str {
        "mermaid preview: open"
    }

    fn render(&self, content: String, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>> {
        let svg = mermaid_rs_renderer::render(&content)
            .map_err(|e| anyhow!("Mermaid render error: {e}"))?;
        svg_renderer
            .render_single_frame(svg.as_bytes(), 1.0, true)
            .map_err(|e| anyhow!("{e}"))
    }
}
