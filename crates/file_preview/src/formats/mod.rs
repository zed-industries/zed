mod mermaid;
mod svg;

pub use mermaid::MermaidFormat;
pub use svg::SvgFormat;

use std::sync::Arc;

use anyhow::Result;
use gpui::{RenderImage, SvgRenderer};

/// Defines format-specific behaviour for a file preview.
/// Implementations must be `Send + Sync` so they can be
/// shared across threads and stored in `Arc`.
pub trait FilePreviewFormat: Send + Sync + 'static {
    /// File extensions this format handles (lowercase, no dot).
    /// e.g. `&["svg"]` or `&["mmd", "mermaid"]`
    fn extensions(&self) -> &'static [&'static str];

    /// Short human-readable name, used in tab labels and empty-state messages.
    /// e.g. `"SVG"` or `"Mermaid"`
    fn display_name(&self) -> &'static str;

    /// Telemetry event string.
    fn telemetry_event(&self) -> &'static str;

    /// Synchronously render `content` to a `RenderImage`.
    /// Called inside `cx.background_spawn`, so blocking is fine.
    fn render(&self, content: String, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>>;
}
