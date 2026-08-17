use super::builtin::{
    attr_sanitize::sanitize_element_attributes,
    css_sanitize::sanitize_style_elements,
    foreign_object::{foreign_object_fallback_svg, strip_foreign_objects},
};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SvgPipelinePreset {
    /// Preserve Mermaid-like SVG output without consumer-oriented cleanup.
    #[default]
    Parity,
    /// Add best-effort SVG text fallbacks for labels rendered as `<foreignObject>`.
    ///
    /// This keeps the original `<foreignObject>` labels for browser parity, so consumers that
    /// render both native HTML labels and fallback text may display duplicate text.
    Readable,
    /// Produce output for resvg/usvg-like consumers.
    ///
    /// This starts from the readable fallback path, strips native `<foreignObject>` labels, and
    /// removes known rasterization hazards such as unsupported CSS animation constructs and invalid
    /// numeric attributes.
    ResvgSafe,
}

pub(crate) fn apply_preset(preset: SvgPipelinePreset, svg: &str) -> Cow<'_, str> {
    match preset {
        SvgPipelinePreset::Parity => Cow::Borrowed(svg),
        SvgPipelinePreset::Readable => Cow::Owned(foreign_object_fallback_svg(svg)),
        SvgPipelinePreset::ResvgSafe => Cow::Owned(resvg_safe_svg(svg)),
    }
}

/// Converts Mermaid-like SVG into a best-effort resvg/usvg compatible SVG string.
pub fn resvg_safe_svg(svg: &str) -> String {
    let svg = foreign_object_fallback_svg(svg);
    let svg = strip_foreign_objects(&svg);
    let svg = sanitize_style_elements(&svg);
    sanitize_element_attributes(&svg)
}
