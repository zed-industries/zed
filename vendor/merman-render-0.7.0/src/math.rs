//! Optional math rendering hooks.
//!
//! Upstream Mermaid renders `$$...$$` fragments via KaTeX and measures the resulting HTML in a
//! browser DOM. merman is headless and pure-Rust by default, so math rendering is modeled as an
//! optional, pluggable backend.
//!
//! The default implementation is a no-op. For parity work, a Node.js-backed KaTeX renderer is
//! provided, and the `ratex-math` feature enables a pure-Rust RaTeX renderer for supported labels.

#[cfg(feature = "ratex-math")]
use crate::text::split_html_br_lines;
use crate::text::{TextMetrics, TextStyle, WrapMode};
use merman_core::MermaidConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;

/// Optional math renderer used to transform label HTML and (optionally) provide measurements.
///
/// Implementations should be:
/// - deterministic (stable output across runs),
/// - side-effect free (no global mutations),
/// - non-panicking (return `None` to decline handling).
pub trait MathRenderer: std::fmt::Debug {
    /// Attempts to render math fragments within an HTML label string.
    ///
    /// If the renderer declines to handle the input, it should return `None`.
    ///
    /// The returned string is treated as raw HTML and will still be sanitized by merman before
    /// emitting into an SVG `<foreignObject>`.
    fn render_html_label(&self, text: &str, config: &MermaidConfig) -> Option<String>;

    /// Attempts to render a Sequence `drawKatex(...)` label.
    ///
    /// Sequence uses a bare `foreignObject` with `width: fit-content` rather than Flowchart's
    /// HTML-label shell, so math backends may support a slightly different surface here.
    fn render_sequence_html_label(&self, text: &str, config: &MermaidConfig) -> Option<String> {
        self.render_html_label(text, config)
    }

    /// Optionally measures the rendered HTML label in pixels.
    ///
    /// This is intended to mirror upstream Mermaid's DOM measurement behavior for math labels.
    /// The default implementation returns `None`.
    fn measure_html_label(
        &self,
        _text: &str,
        _config: &MermaidConfig,
        _style: &TextStyle,
        _max_width_px: Option<f64>,
        _wrap_mode: WrapMode,
    ) -> Option<TextMetrics> {
        None
    }

    /// Optionally measures a Sequence `drawKatex(...)` label in pixels.
    ///
    /// Mermaid Sequence does not wrap KaTeX labels in the flowchart HTML-label shell; it appends
    /// a bare `<foreignObject><div style="width: fit-content;">...</div></foreignObject>`.
    /// This hook lets Sequence callers avoid inheriting flowchart-specific table-cell metrics.
    fn measure_sequence_html_label(
        &self,
        _text: &str,
        _config: &MermaidConfig,
    ) -> Option<TextMetrics> {
        None
    }
}

/// Default math renderer: does nothing.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopMathRenderer;

impl MathRenderer for NoopMathRenderer {
    fn render_html_label(&self, _text: &str, _config: &MermaidConfig) -> Option<String> {
        None
    }
}

/// Pure-Rust math renderer backed by RaTeX.
///
/// The first Flowchart surface is intentionally narrow: labels where each non-empty line is a
/// single `$$...$$` formula. Sequence additionally supports one formula embedded in surrounding
/// prose per line, matching Mermaid's `drawKatex(...)` shell.
#[cfg(feature = "ratex-math")]
#[derive(Debug, Default, Clone, Copy)]
pub struct RatexMathRenderer;

#[cfg(feature = "ratex-math")]
#[derive(Debug, Clone)]
struct RatexRenderedMath {
    width_em: f64,
    height_em: f64,
    line_count: usize,
}

#[cfg(feature = "ratex-math")]
impl RatexMathRenderer {
    fn normalized_text(text: &str) -> String {
        text.replace("\\\\", "\\")
    }

    fn math_only_lines(text: &str) -> Option<Vec<String>> {
        let normalized = Self::normalized_text(text);
        let mut formulas = Vec::new();
        for raw_line in split_html_br_lines(&normalized) {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }
            let inner = line.strip_prefix("$$")?.strip_suffix("$$")?;
            if inner.contains("$$") {
                return None;
            }
            formulas.push(inner.to_string());
        }
        if formulas.is_empty() {
            None
        } else {
            Some(formulas)
        }
    }

    fn render_formula_svg_em(latex: &str) -> Option<(String, f64, f64)> {
        let ast = ratex_parser::parse(latex).ok()?;
        let layout_options = ratex_layout::LayoutOptions::default()
            .with_style(ratex_types::MathStyle::Display)
            .with_color(ratex_types::Color::BLACK);
        let layout_box = ratex_layout::layout(&ast, &layout_options);
        let display_list = ratex_layout::to_display_list(&layout_box);
        let width_em = display_list.width.max(0.0);
        let height_em = display_list.total_height().max(0.0);
        let svg = ratex_svg::render_to_svg(
            &display_list,
            &ratex_svg::SvgOptions {
                font_size: 1.0,
                padding: 0.0,
                stroke_width: 0.04,
                embed_glyphs: true,
                font_dir: String::new(),
            },
        );
        Some((
            Self::svg_with_em_size(svg, width_em, height_em),
            width_em,
            height_em,
        ))
    }

    fn svg_with_em_size(svg: String, width_em: f64, height_em: f64) -> String {
        let Some(open_end) = svg.find('>') else {
            return svg;
        };
        let Some(body_with_close) = svg.get(open_end + 1..) else {
            return svg;
        };
        let Some(body) = body_with_close.strip_suffix("</svg>") else {
            return svg;
        };
        let width = Self::fmt_num(width_em);
        let height = Self::fmt_num(height_em);
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}em" height="{height}em">{body}</svg>"#
        )
    }

    fn render_math_only_label(text: &str) -> Option<RatexRenderedMath> {
        let formulas = Self::math_only_lines(text)?;
        let mut width_em: f64 = 0.0;
        let mut height_em: f64 = 0.0;
        let mut line_count = 0usize;
        for formula in formulas {
            let (_svg, line_width_em, line_height_em) = Self::render_formula_svg_em(&formula)?;
            width_em = width_em.max(line_width_em);
            height_em += line_height_em;
            line_count += 1;
        }
        Some(RatexRenderedMath {
            width_em,
            height_em,
            line_count: line_count.max(1),
        })
    }

    fn render_katex_like_line_html(line: &str) -> Option<String> {
        if !line.contains("$$") {
            return Some(line.to_string());
        }
        let start = line.find("$$")?;
        let content_start = start + 2;
        let end_start = line[content_start..].rfind("$$")? + content_start;
        if end_start < content_start {
            return None;
        }
        let formula = &line[content_start..end_start];
        if formula.contains("$$") {
            return None;
        }
        let (svg, _width_em, _height_em) = Self::render_formula_svg_em(formula)?;
        let mut html = String::with_capacity(line.len() + svg.len());
        html.push_str(&line[..start]);
        html.push_str(&svg);
        html.push_str(&line[end_start + 2..]);
        Some(html)
    }

    fn render_katex_like_label(text: &str) -> Option<String> {
        let normalized = Self::normalized_text(text);
        if !normalized.contains("$$") {
            return None;
        }

        let mut html = String::new();
        let mut saw_math = false;
        for line in split_html_br_lines(&normalized) {
            if line.contains("$$") {
                saw_math = true;
                let rendered_line = Self::render_katex_like_line_html(line)?;
                let _ = write!(
                    &mut html,
                    r#"<div style="display: flex; align-items: center; justify-content: center; white-space: nowrap;">{rendered_line}</div>"#
                );
            } else {
                let _ = write!(&mut html, "<div>{line}</div>");
            }
        }

        saw_math.then_some(html)
    }

    fn metrics_from_em(rendered: &RatexRenderedMath, font_size: f64) -> TextMetrics {
        let font_size = font_size.max(1.0);
        TextMetrics {
            width: crate::text::round_to_1_64_px(rendered.width_em * font_size),
            height: crate::text::round_to_1_64_px(rendered.height_em * font_size),
            line_count: rendered.line_count,
        }
    }

    fn fmt_num(n: f64) -> String {
        let s = format!("{n:.6}");
        let s = s.trim_end_matches('0').trim_end_matches('.');
        if s.is_empty() || s == "-" {
            "0".to_string()
        } else {
            s.to_string()
        }
    }
}

#[cfg(feature = "ratex-math")]
impl MathRenderer for RatexMathRenderer {
    fn render_html_label(&self, text: &str, _config: &MermaidConfig) -> Option<String> {
        if !text.contains("$$") {
            return None;
        }
        Self::render_katex_like_label(text)
    }

    fn render_sequence_html_label(&self, text: &str, _config: &MermaidConfig) -> Option<String> {
        Self::render_katex_like_label(text)
    }

    fn measure_html_label(
        &self,
        text: &str,
        _config: &MermaidConfig,
        style: &TextStyle,
        _max_width_px: Option<f64>,
        wrap_mode: WrapMode,
    ) -> Option<TextMetrics> {
        if wrap_mode != WrapMode::HtmlLike || !text.contains("$$") {
            return None;
        }
        let rendered = Self::render_math_only_label(text)?;
        Some(Self::metrics_from_em(&rendered, style.font_size))
    }

    fn measure_sequence_html_label(
        &self,
        text: &str,
        _config: &MermaidConfig,
    ) -> Option<TextMetrics> {
        if !text.contains("$$") {
            return None;
        }
        let rendered = Self::render_math_only_label(text)?;
        Some(Self::metrics_from_em(
            &rendered,
            TextStyle::default().font_size,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RenderCacheKey {
    text: String,
    legacy_mathml: bool,
    force_legacy_mathml: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProbeCacheKey {
    render: RenderCacheKey,
    font_family: Option<String>,
    font_size_bits: u64,
    font_weight: Option<String>,
    max_width_bits: u64,
}

#[derive(Debug, Clone)]
struct ProbeCacheValue {
    html: String,
    width: f64,
    height: f64,
    line_count: usize,
}

#[derive(Debug, Serialize)]
struct NodeRenderRequest {
    text: String,
    config: NodeMathConfig,
}

#[derive(Debug, Serialize)]
struct NodeProbeRequest {
    text: String,
    config: NodeMathConfig,
    #[serde(rename = "styleCss")]
    style_css: String,
    #[serde(rename = "maxWidthPx")]
    max_width_px: f64,
}

#[derive(Debug, Serialize)]
struct NodeMathConfig {
    #[serde(rename = "legacyMathML")]
    legacy_mathml: bool,
    #[serde(rename = "forceLegacyMathML")]
    force_legacy_mathml: bool,
}

#[derive(Debug, Deserialize)]
struct NodeRenderResponse {
    html: String,
}

#[derive(Debug, Deserialize)]
struct NodeProbeResponse {
    html: String,
    width: f64,
    height: f64,
}

/// Optional KaTeX backend that shells out to a local Node.js toolchain.
///
/// This backend is intended for parity work where a real browser DOM is available. It mirrors
/// Mermaid's flowchart HTML-label KaTeX path closely by:
/// - rendering KaTeX through the local `katex` npm package, and
/// - measuring the wrapped `<foreignObject>` HTML through local `puppeteer`.
///
/// The backend is completely opt-in; if the configured Node.js environment is unavailable or the
/// probe fails, it simply returns `None` and lets callers fall back to the default text path.
#[derive(Debug)]
pub struct NodeKatexMathRenderer {
    node_cwd: PathBuf,
    node_command: PathBuf,
    render_cache: Mutex<HashMap<RenderCacheKey, Option<String>>>,
    probe_cache: Mutex<HashMap<ProbeCacheKey, Option<ProbeCacheValue>>>,
    sequence_probe_cache: Mutex<HashMap<RenderCacheKey, Option<ProbeCacheValue>>>,
}

impl NodeKatexMathRenderer {
    pub fn new(node_cwd: impl Into<PathBuf>) -> Self {
        Self {
            node_cwd: node_cwd.into(),
            node_command: PathBuf::from("node"),
            render_cache: Mutex::new(HashMap::new()),
            probe_cache: Mutex::new(HashMap::new()),
            sequence_probe_cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_node_command(mut self, node_command: impl Into<PathBuf>) -> Self {
        self.node_command = node_command.into();
        self
    }

    fn script_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("katex_flowchart_probe.cjs")
    }

    fn normalized_text(text: &str) -> String {
        text.replace("\\\\", "\\")
    }

    fn math_config(config: &MermaidConfig) -> NodeMathConfig {
        let config_value = config.as_value();
        let legacy_mathml = config_value
            .get("legacyMathML")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let force_legacy_mathml = config_value
            .get("forceLegacyMathML")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        NodeMathConfig {
            legacy_mathml,
            force_legacy_mathml,
        }
    }

    fn render_key(text: &str, config: &MermaidConfig) -> RenderCacheKey {
        let config = Self::math_config(config);
        RenderCacheKey {
            text: Self::normalized_text(text),
            legacy_mathml: config.legacy_mathml,
            force_legacy_mathml: config.force_legacy_mathml,
        }
    }

    fn style_css(style: &TextStyle) -> String {
        let mut out = String::new();
        let font_family = style
            .font_family
            .as_deref()
            .unwrap_or("\"trebuchet ms\",verdana,arial,sans-serif");
        let _ = write!(&mut out, "font-size: {}px;", style.font_size);
        let _ = write!(&mut out, "font-family: {};", font_family);
        if let Some(font_weight) = style.font_weight.as_deref() {
            if !font_weight.trim().is_empty() {
                let _ = write!(&mut out, "font-weight: {};", font_weight.trim());
            }
        }
        out
    }

    fn run_node_request<T, R>(&self, mode: &str, payload: &T) -> Option<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        if !self.node_cwd.join("package.json").is_file() {
            return None;
        }

        let mut child = Command::new(&self.node_command)
            .arg(Self::script_path())
            .arg(mode)
            .current_dir(&self.node_cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;

        if let Some(mut stdin) = child.stdin.take() {
            if serde_json::to_writer(&mut stdin, payload).is_err() {
                return None;
            }
            let _ = stdin.flush();
        }

        let output = child.wait_with_output().ok()?;
        if !output.status.success() {
            return None;
        }

        serde_json::from_slice(&output.stdout).ok()
    }

    fn render_cached(&self, text: &str, config: &MermaidConfig) -> Option<String> {
        let key = Self::render_key(text, config);
        if let Some(cached) = self
            .render_cache
            .lock()
            .ok()
            .and_then(|cache| cache.get(&key).cloned())
        {
            return cached;
        }

        let response: Option<NodeRenderResponse> = self.run_node_request(
            "render",
            &NodeRenderRequest {
                text: key.text.clone(),
                config: NodeMathConfig {
                    legacy_mathml: key.legacy_mathml,
                    force_legacy_mathml: key.force_legacy_mathml,
                },
            },
        );
        let html = response.map(|value| value.html);

        if let Ok(mut cache) = self.render_cache.lock() {
            cache.insert(key, html.clone());
        }

        html
    }

    fn probe_cached(
        &self,
        text: &str,
        config: &MermaidConfig,
        style: &TextStyle,
        max_width_px: Option<f64>,
        _wrap_mode: WrapMode,
    ) -> Option<ProbeCacheValue> {
        let render = Self::render_key(text, config);
        let max_width = max_width_px.unwrap_or(200.0).max(1.0);
        let key = ProbeCacheKey {
            render: render.clone(),
            font_family: style.font_family.clone(),
            font_size_bits: style.font_size.to_bits(),
            font_weight: style.font_weight.clone(),
            max_width_bits: max_width.to_bits(),
        };
        if let Some(cached) = self
            .probe_cache
            .lock()
            .ok()
            .and_then(|cache| cache.get(&key).cloned())
        {
            return cached;
        }

        let style_css = Self::style_css(style);
        let response: Option<NodeProbeResponse> = self.run_node_request(
            "probe",
            &NodeProbeRequest {
                text: render.text.clone(),
                config: NodeMathConfig {
                    legacy_mathml: render.legacy_mathml,
                    force_legacy_mathml: render.force_legacy_mathml,
                },
                style_css,
                max_width_px: max_width,
            },
        );
        let probed = response.and_then(|value| {
            if !value.width.is_finite() || !value.height.is_finite() {
                return None;
            }
            let line_count = value.html.match_indices("<div").count().max(1);
            Some(ProbeCacheValue {
                html: value.html,
                width: value.width.max(0.0),
                height: value.height.max(0.0),
                line_count,
            })
        });

        if let Some(probed_value) = probed.clone() {
            if let Ok(mut render_cache) = self.render_cache.lock() {
                render_cache
                    .entry(render)
                    .or_insert_with(|| Some(probed_value.html.clone()));
            }
        }
        if let Ok(mut cache) = self.probe_cache.lock() {
            cache.insert(key, probed.clone());
        }

        probed
    }

    fn sequence_probe_cached(&self, text: &str, config: &MermaidConfig) -> Option<ProbeCacheValue> {
        let key = Self::render_key(text, config);
        if let Some(cached) = self
            .sequence_probe_cache
            .lock()
            .ok()
            .and_then(|cache| cache.get(&key).cloned())
        {
            return cached;
        }

        let response: Option<NodeProbeResponse> = self.run_node_request(
            "probe-sequence",
            &NodeRenderRequest {
                text: key.text.clone(),
                config: NodeMathConfig {
                    legacy_mathml: key.legacy_mathml,
                    force_legacy_mathml: key.force_legacy_mathml,
                },
            },
        );
        let probed = response.and_then(|value| {
            if !value.width.is_finite() || !value.height.is_finite() {
                return None;
            }
            let line_count = value.html.match_indices("<div").count().max(1);
            Some(ProbeCacheValue {
                html: value.html,
                width: value.width.max(0.0),
                height: value.height.max(0.0),
                line_count,
            })
        });

        if let Some(probed_value) = probed.clone() {
            if let Ok(mut render_cache) = self.render_cache.lock() {
                render_cache
                    .entry(key.clone())
                    .or_insert_with(|| Some(probed_value.html.clone()));
            }
        }
        if let Ok(mut cache) = self.sequence_probe_cache.lock() {
            cache.insert(key, probed.clone());
        }

        probed
    }
}

impl MathRenderer for NodeKatexMathRenderer {
    fn render_html_label(&self, text: &str, config: &MermaidConfig) -> Option<String> {
        if !text.contains("$$") {
            return None;
        }
        self.render_cached(text, config)
    }

    fn measure_html_label(
        &self,
        text: &str,
        config: &MermaidConfig,
        style: &TextStyle,
        max_width_px: Option<f64>,
        wrap_mode: WrapMode,
    ) -> Option<TextMetrics> {
        if wrap_mode != WrapMode::HtmlLike || !text.contains("$$") {
            return None;
        }
        let probed = self.probe_cached(text, config, style, max_width_px, wrap_mode)?;
        Some(TextMetrics {
            width: crate::text::round_to_1_64_px(probed.width),
            height: crate::text::round_to_1_64_px(probed.height),
            line_count: probed.line_count,
        })
    }

    fn measure_sequence_html_label(
        &self,
        text: &str,
        config: &MermaidConfig,
    ) -> Option<TextMetrics> {
        if !text.contains("$$") {
            return None;
        }
        let probed = self.sequence_probe_cached(text, config)?;
        Some(TextMetrics {
            width: crate::text::round_to_1_64_px(probed.width),
            height: crate::text::round_to_1_64_px(probed.height),
            line_count: probed.line_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ratex-math")]
    #[test]
    fn ratex_math_renderer_splits_math_only_labels_with_source_br_shape() {
        assert_eq!(
            RatexMathRenderer::math_only_lines("$$x$$<BR /> $$y$$<bR\t/>$$z$$"),
            Some(vec!["x".to_string(), "y".to_string(), "z".to_string()])
        );
        assert!(
            RatexMathRenderer::math_only_lines("$$x$$<brx>$$y$$").is_none(),
            "non-source <br> lookalikes must not split a same-line multi-formula label"
        );
    }

    #[cfg(feature = "ratex-math")]
    #[test]
    fn ratex_math_renderer_renders_pure_math_label_as_inline_svg() {
        let renderer = RatexMathRenderer;
        let config = MermaidConfig::from_value(serde_json::json!({ "securityLevel": "loose" }));

        let html = renderer
            .render_html_label("$$x^2$$", &config)
            .expect("ratex should render pure math labels");

        assert!(html.contains("<svg"), "expected inline SVG: {html}");
        assert!(
            html.contains("<path"),
            "expected outlined glyph paths: {html}"
        );
        assert!(
            html.contains(r#"width="0.97153em""#),
            "unexpected SVG size: {html}"
        );
        let sanitized = merman_core::sanitize::sanitize_text(&html, &config);
        assert!(
            sanitized.contains("<svg") && sanitized.contains("<path"),
            "sanitizer should preserve RaTeX inline SVG: {sanitized}"
        );
        let mixed_html = renderer
            .render_html_label("value: $$x^2$$", &config)
            .expect("RaTeX HTML rendering should support prose plus math");
        assert!(
            mixed_html.contains("value: ") && mixed_html.contains("<svg"),
            "unexpected mixed math HTML: {mixed_html}"
        );
        assert!(
            !mixed_html.contains("$$"),
            "mixed math HTML should replace source delimiters: {mixed_html}"
        );

        let mixed_sequence = renderer
            .render_sequence_html_label("value: $$x^2$$", &config)
            .expect("Sequence RaTeX labels should support prose plus math");
        assert!(
            mixed_sequence.contains("value: ") && mixed_sequence.contains("<svg"),
            "unexpected Sequence mixed math HTML: {mixed_sequence}"
        );
        assert!(
            !mixed_sequence.contains("$$"),
            "Sequence mixed math HTML should replace source delimiters: {mixed_sequence}"
        );
    }

    #[cfg(feature = "ratex-math")]
    #[test]
    fn ratex_math_renderer_rejects_multiple_formulas_on_one_line() {
        let renderer = RatexMathRenderer;
        let config = MermaidConfig::default();
        let style = TextStyle::default();
        let label = "a $$x$$ b $$y$$ c";

        assert!(
            renderer.render_html_label(label, &config).is_none(),
            "Mermaid's greedy $$...$$ regex treats same-line multiple formulas as one invalid formula, so RaTeX should not render them non-greedily"
        );
        assert!(
            renderer
                .measure_html_label(label, &config, &style, Some(200.0), WrapMode::HtmlLike)
                .is_none(),
            "same-line multiple formulas should remain unsupported in measurement too"
        );
        assert!(
            renderer
                .measure_sequence_html_label(label, &config)
                .is_none(),
            "Sequence should keep the same delimiter policy"
        );
    }

    #[cfg(feature = "ratex-math")]
    #[test]
    fn ratex_math_renderer_measures_flowchart_and_sequence_math_labels() {
        let renderer = RatexMathRenderer;
        let config = MermaidConfig::default();
        let style = TextStyle::default();

        let flowchart = renderer
            .measure_html_label("$$x^2$$", &config, &style, Some(200.0), WrapMode::HtmlLike)
            .expect("ratex should measure pure flowchart math labels");
        assert_eq!(flowchart.width, 15.546875);
        assert_eq!(flowchart.height, 13.828125);
        assert_eq!(flowchart.line_count, 1);

        let sequence = renderer
            .measure_sequence_html_label("$$x^2$$", &config)
            .expect("ratex should measure pure sequence math labels");
        assert_eq!(sequence.width, flowchart.width);
        assert_eq!(sequence.height, flowchart.height);
        assert_eq!(sequence.line_count, 1);

        let flowchart_request = crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &crate::text::DeterministicTextMeasurer::default(),
            raw_label: "$$x^2$$",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &config,
            math_renderer: Some(&renderer),
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        };
        let through_flowchart =
            crate::flowchart::flowchart_label_metrics_for_layout(flowchart_request);
        assert_eq!(through_flowchart.width, flowchart.width);
        assert_eq!(through_flowchart.height, flowchart.height);
    }

    #[test]
    fn node_katex_math_renderer_smoke() {
        let node_cwd = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("tools")
            .join("mermaid-cli");
        if !node_cwd.join("package.json").is_file() || !node_cwd.join("node_modules").is_dir() {
            return;
        }

        let renderer = NodeKatexMathRenderer::new(node_cwd);
        let config = MermaidConfig::default();
        let style = TextStyle::default();

        let Some(html) = renderer.render_html_label("$$x^2$$", &config) else {
            return;
        };
        assert!(html.contains("katex"), "unexpected HTML: {html}");

        let Some(metrics) = renderer.measure_html_label(
            "$$x^2$$",
            &config,
            &style,
            Some(200.0),
            WrapMode::HtmlLike,
        ) else {
            return;
        };
        assert!(metrics.width.is_finite() && metrics.width > 0.0);
        assert!(metrics.height.is_finite() && metrics.height > 0.0);
    }

    #[test]
    fn node_katex_math_renderer_measures_sanitized_flowchart_browser_shell() {
        let node_cwd = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("tools")
            .join("mermaid-cli");
        if !node_cwd.join("package.json").is_file() || !node_cwd.join("node_modules").is_dir() {
            return;
        }

        let renderer = NodeKatexMathRenderer::new(node_cwd);
        let config = MermaidConfig::default();
        let style = TextStyle::default();

        let long_integral = "$$f(\\relax{x}) = \\int_{-\\infty}^\\infty \\hat{f}(\\xi)\\,e^{2 \\pi i \\xi x}\\,d\\xi$$";
        let Some(node_metrics) = renderer.measure_html_label(
            long_integral,
            &config,
            &style,
            Some(200.0),
            WrapMode::HtmlLike,
        ) else {
            return;
        };
        assert!(
            (150.0..=260.0).contains(&node_metrics.width),
            "node width = {}",
            node_metrics.width
        );
        assert!(
            (20.0..=70.0).contains(&node_metrics.height),
            "node height = {}",
            node_metrics.height
        );

        let matrix_label =
            "$$x(t)=c_1\\begin{bmatrix}-\\cos{t}+\\sin{t}\\\\ 2\\cos{t} \\end{bmatrix}e^{2t}$$";
        let Some(matrix_metrics) = renderer.measure_html_label(
            matrix_label,
            &config,
            &style,
            Some(200.0),
            WrapMode::HtmlLike,
        ) else {
            return;
        };
        // This is a Node/KaTeX shell smoke, not a browser-font parity gate.
        assert!(
            (250.0..=290.0).contains(&matrix_metrics.width),
            "matrix width = {}",
            matrix_metrics.width
        );
        assert!(
            (20.0..=32.0).contains(&matrix_metrics.height),
            "matrix height = {}",
            matrix_metrics.height
        );

        let Some(html) = renderer.render_html_label(long_integral, &config) else {
            panic!("expected rendered math HTML after successful probe");
        };
        assert!(html.contains("<math"), "unexpected HTML: {html}");
        assert!(!html.contains("<semantics>"), "unsanitized HTML: {html}");

        let nested_delimiters = "$$\\Bigg(\\bigg(\\Big(\\big((\\frac{-b\\pm\\sqrt{b^2-4ac}}{2a})\\big)\\Big)\\bigg)\\Bigg)$$";
        let Some(edge_metrics) = renderer.measure_html_label(
            nested_delimiters,
            &config,
            &style,
            Some(200.0),
            WrapMode::HtmlLike,
        ) else {
            return;
        };
        assert!(
            (150.0..=320.0).contains(&edge_metrics.width),
            "edge width = {}",
            edge_metrics.width
        );
        assert!(
            (30.0..=100.0).contains(&edge_metrics.height),
            "edge height = {}",
            edge_metrics.height
        );
    }
}
