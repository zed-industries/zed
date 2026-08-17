//! Text measurement trait shared by renderers and wrapping helpers.

use super::{TextMetrics, TextStyle, WrapMode};

pub trait TextMeasurer {
    fn measure(&self, text: &str, style: &TextStyle) -> TextMetrics;

    /// Measures SVG `<tspan>.getComputedTextLength()`-like widths (advance length along the
    /// baseline).
    ///
    /// Mermaid's Timeline diagram uses `getComputedTextLength()` to decide when to wrap tokens
    /// into additional `<tspan>` lines. This length can differ meaningfully from `getBBox().width`
    /// (which includes glyph overhang), especially near wrapping boundaries.
    ///
    /// Default implementation falls back to bbox-derived widths.
    fn measure_svg_text_computed_length_px(&self, text: &str, style: &TextStyle) -> f64 {
        self.measure_svg_simple_text_bbox_width_px(text, style)
    }

    /// Measures the horizontal extents of an SVG `<text>` element relative to its anchor `x`.
    ///
    /// Mermaid's flowchart-v2 viewport sizing uses `getBBox()` on the rendered SVG. For `<text>`
    /// elements this bbox can be slightly asymmetric around the anchor due to glyph overhangs.
    ///
    /// Default implementation assumes a symmetric bbox: `left = right = width/2`.
    fn measure_svg_text_bbox_x(&self, text: &str, style: &TextStyle) -> (f64, f64) {
        let m = self.measure(text, style);
        let half = (m.width.max(0.0)) / 2.0;
        (half, half)
    }

    /// Measures SVG `<text>.getBBox()` horizontal extents while including ASCII overhang.
    ///
    /// Upstream Mermaid bbox behavior can be asymmetric even for ASCII strings due to glyph
    /// outlines and hinting. Most diagrams in this codebase intentionally ignore ASCII overhang
    /// to avoid systematic `viewBox` drift, but some diagrams (notably `timeline`) rely on the
    /// actual `getBBox()` extents when labels can overflow node shapes.
    ///
    /// Default implementation falls back to the symmetric bbox measurement.
    fn measure_svg_text_bbox_x_with_ascii_overhang(
        &self,
        text: &str,
        style: &TextStyle,
    ) -> (f64, f64) {
        self.measure_svg_text_bbox_x(text, style)
    }

    /// Measures the horizontal extents for Mermaid diagram titles rendered as a single `<text>`
    /// node (no whitespace-tokenized `<tspan>` runs).
    ///
    /// Mermaid flowchart-v2 uses this style for `flowchartTitleText`, and the bbox impacts the
    /// final `viewBox` / `max-width` computed via `getBBox()`.
    fn measure_svg_title_bbox_x(&self, text: &str, style: &TextStyle) -> (f64, f64) {
        self.measure_svg_text_bbox_x(text, style)
    }

    /// Measures the bbox width for Mermaid `drawSimpleText(...).getBBox().width`-style probes
    /// (used by upstream `calculateTextWidth`).
    ///
    /// This should reflect actual glyph outline extents (including ASCII overhang where present),
    /// rather than the symmetric/center-anchored title bbox approximation.
    fn measure_svg_simple_text_bbox_width_px(&self, text: &str, style: &TextStyle) -> f64 {
        let (l, r) = self.measure_svg_title_bbox_x(text, style);
        (l + r).max(0.0)
    }

    /// Measures raw SVG `<text>.getBBox().width` for diagram renderers that append text directly.
    ///
    /// Unlike [`TextMeasurer::measure_svg_simple_text_bbox_width_px`], this intentionally avoids
    /// diagram-specific `drawSimpleText(...)` compatibility overrides.
    fn measure_svg_raw_text_bbox_width_px(&self, text: &str, style: &TextStyle) -> f64 {
        self.measure_svg_simple_text_bbox_width_px(text, style)
    }

    /// Measures simple SVG text for wrap decisions.
    ///
    /// Some implementations carry fixture-derived exact text-width overrides for final layout
    /// sizing. Those can be too sharp for incremental `wrapLabel(...)` probes, where changing one
    /// candidate prefix width changes the emitted DOM line structure. Implementations may override
    /// this to use their smoother base font model for wrap decisions.
    fn measure_svg_simple_text_bbox_width_for_wrap_px(&self, text: &str, style: &TextStyle) -> f64 {
        self.measure_svg_simple_text_bbox_width_px(text, style)
    }

    /// Measures the bbox height for Mermaid `drawSimpleText(...).getBBox().height`-style probes.
    ///
    /// Upstream Mermaid uses `<text>.getBBox()` for some diagrams (notably `gitGraph` commit/tag
    /// labels). Those `<text>` nodes are not split into `<tspan>` runs, and empirically their
    /// bbox height behaves closer to ~`1.1em` than the slightly taller first-line heuristic used
    /// by `measure_wrapped(..., WrapMode::SvgLike)`.
    ///
    /// Default implementation falls back to `measure(...).height`.
    fn measure_svg_simple_text_bbox_height_px(&self, text: &str, style: &TextStyle) -> f64 {
        let m = self.measure(text, style);
        m.height.max(0.0)
    }

    fn measure_wrapped(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> TextMetrics {
        let _ = max_width;
        let _ = wrap_mode;
        self.measure(text, style)
    }

    /// Measures wrapped text and (optionally) returns the unwrapped width for the same payload.
    ///
    /// This exists mainly to avoid redundant measurement passes in diagrams that need both:
    /// - wrapped metrics (for height/line breaks), and
    /// - a raw "overflow width" probe (for sizing containers that can visually overflow).
    ///
    /// Default implementation returns `None` for `raw_width_px` and callers may fall back to an
    /// explicit second measurement if needed.
    fn measure_wrapped_with_raw_width(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> (TextMetrics, Option<f64>) {
        (
            self.measure_wrapped(text, style, max_width, wrap_mode),
            None,
        )
    }

    /// Measures wrapped text while disabling any implementation-specific HTML overrides.
    ///
    /// This is primarily used for Markdown labels measured via DOM in upstream Mermaid, where we
    /// want a raw regular-weight baseline before applying `<strong>/<em>` deltas.
    fn measure_wrapped_raw(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> TextMetrics {
        self.measure_wrapped(text, style, max_width, wrap_mode)
    }
}
