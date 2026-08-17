#[cfg(feature = "ratex-math")]
pub use merman_render::math::RatexMathRenderer;
pub use merman_render::math::{MathRenderer, NoopMathRenderer};
pub use merman_render::model::LayoutedDiagram;
pub use merman_render::svg::{
    CompiledHostTheme, CompiledHostThemeOutput, CssOverridePolicy, CssOverridePostprocessor,
    DropNativeDuplicateFallbacksPostprocessor, ForeignObjectFallbackPostprocessor,
    HostThemeAppearance, HostThemeOutput, HostThemePipelinePreset, HostThemePreset,
    HostThemeProfile, HostThemeProfileBuilder, HostThemeRoles, HostThemeRootBackground,
    IconRegistry, IconRegistryError, IconSvg, RootBackgroundPostprocessor,
    SanitizeCssPostprocessor, SanitizeSvgAttributesPostprocessor, ScopedCssPostprocessor,
    StripForeignObjectPostprocessor, SvgPipeline, SvgPipelinePreset, SvgPostprocessContext,
    SvgPostprocessMetadata, SvgPostprocessor, SvgRenderOptions,
    foreign_object_label_fallback_svg_text, resvg_safe_svg, supported_host_theme_presets,
};
pub use merman_render::text::{
    DeterministicTextMeasurer, TextMeasurer, VendoredFontMetricsTextMeasurer,
};
pub use merman_render::{
    Error as RenderError, LayoutOptions, Result as RenderResult, layout_parsed,
};

mod operation;

#[cfg(feature = "raster")]
pub mod raster;

#[derive(Debug, thiserror::Error)]
pub enum HeadlessError {
    #[error(transparent)]
    Parse(#[from] merman_core::Error),
    #[error(transparent)]
    Render(#[from] merman_render::Error),
}

pub type Result<T> = std::result::Result<T, HeadlessError>;

/// Converts an arbitrary string into a conservative SVG `id` token suitable for embedding
/// multiple Mermaid diagrams in the same UI tree.
///
/// Mermaid uses the root `<svg id="...">` value as a prefix for internal ids like
/// `chart-title-<id>` and marker ids under `<defs>`. If you inline multiple SVGs with the same
/// id, those internal ids may collide.
///
/// This helper:
/// - trims whitespace
/// - replaces unsupported characters with `-`
/// - ensures the id starts with an ASCII letter by prefixing `m-` when needed
pub fn sanitize_svg_id(raw: &str) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return "m-untitled".to_string();
    }

    let mut iter = raw.chars();
    let Some(first_raw) = iter.next() else {
        return "m-untitled".to_string();
    };

    fn sanitize_char(ch: char) -> char {
        let ok = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == ':' || ch == '.';
        if ok { ch } else { '-' }
    }

    let first = sanitize_char(first_raw);
    let mut out = String::with_capacity(raw.len() + 2);
    let mut prev_dash = false;

    if !first.is_ascii_alphabetic() {
        out.push('m');
        if first != '-' {
            out.push('-');
            prev_dash = true;
        }
    }

    let push_sanitized = |ch: char, out: &mut String, prev_dash: &mut bool| {
        if ch == '-' {
            if *prev_dash {
                return;
            }
            *prev_dash = true;
        } else {
            *prev_dash = false;
        }
        out.push(ch);
    };

    push_sanitized(first, &mut out, &mut prev_dash);
    for ch in iter {
        push_sanitized(sanitize_char(ch), &mut out, &mut prev_dash);
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() || out == "m" {
        return "m-untitled".to_string();
    }
    out
}

#[cfg(test)]
mod sanitize_svg_id_tests {
    use super::sanitize_svg_id;

    #[test]
    fn sanitize_svg_id_empty_is_untitled() {
        assert_eq!(sanitize_svg_id(""), "m-untitled");
        assert_eq!(sanitize_svg_id("   "), "m-untitled");
    }

    #[test]
    fn sanitize_svg_id_trims_and_replaces() {
        assert_eq!(sanitize_svg_id(" my diagram "), "my-diagram");
        assert_eq!(sanitize_svg_id("a b\tc"), "a-b-c");
    }

    #[test]
    fn sanitize_svg_id_prefixes_when_needed() {
        assert_eq!(sanitize_svg_id("1a"), "m-1a");
        assert_eq!(sanitize_svg_id("_a"), "m-_a");
        assert_eq!(sanitize_svg_id("-a"), "m-a");
    }

    #[test]
    fn sanitize_svg_id_collapses_and_trims_dashes() {
        assert_eq!(sanitize_svg_id("a----b"), "a-b");
        assert_eq!(sanitize_svg_id("abc--"), "abc");
        assert_eq!(sanitize_svg_id("--"), "m-untitled");
        assert_eq!(sanitize_svg_id("-"), "m-untitled");
    }

    #[test]
    fn sanitize_svg_id_keeps_allowed_punctuation() {
        assert_eq!(sanitize_svg_id("a:b.c_d"), "a:b.c_d");
    }

    #[test]
    fn sanitize_svg_id_m_is_reserved_for_untitled() {
        assert_eq!(sanitize_svg_id("m"), "m-untitled");
        assert_eq!(sanitize_svg_id("m-"), "m-untitled");
        assert_eq!(sanitize_svg_id("m--"), "m-untitled");
    }
}

/// Synchronous layout helper (executor-free).
pub fn layout_diagram_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
) -> Result<Option<LayoutedDiagram>> {
    operation::HeadlessOperation::new(engine, text, parse_options, layout_options).layout_diagram()
}

/// Returns layout defaults intended for UI integrations that render headless SVG.
///
/// This is a convenience wrapper around [`LayoutOptions::headless_svg_defaults`].
pub fn headless_layout_options() -> LayoutOptions {
    LayoutOptions::headless_svg_defaults()
}

pub async fn layout_diagram(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
) -> Result<Option<LayoutedDiagram>> {
    // This async API is runtime-agnostic: layout is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    layout_diagram_sync(engine, text, parse_options, layout_options)
}

pub fn render_layouted_svg(
    diagram: &LayoutedDiagram,
    measurer: &dyn TextMeasurer,
    svg_options: &SvgRenderOptions,
) -> Result<String> {
    Ok(merman_render::svg::render_layouted_svg(
        diagram,
        measurer,
        svg_options,
    )?)
}

/// Synchronous SVG render helper (executor-free).
pub fn render_svg_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    operation::HeadlessOperation::new(engine, text, parse_options, layout_options)
        .render_svg(svg_options)
}

pub fn apply_svg_pipeline(svg: &str, pipeline: &SvgPipeline) -> Result<String> {
    Ok(pipeline.process_to_string(svg)?)
}

pub fn apply_svg_pipeline_with_metadata(
    svg: &str,
    pipeline: &SvgPipeline,
    metadata: &SvgPostprocessMetadata,
) -> Result<String> {
    Ok(pipeline.process_to_string_with_metadata(svg, metadata)?)
}

pub fn svg_readable(svg: &str) -> Result<String> {
    apply_svg_pipeline(svg, &SvgPipeline::readable())
}

pub fn svg_resvg_safe(svg: &str) -> Result<String> {
    apply_svg_pipeline(svg, &SvgPipeline::resvg_safe())
}

pub fn render_svg_with_pipeline_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
    pipeline: &SvgPipeline,
) -> Result<Option<String>> {
    operation::HeadlessOperation::new(engine, text, parse_options, layout_options)
        .render_svg_with_pipeline(svg_options, pipeline)
}

/// Synchronous SVG render helper that applies a best-effort readability fallback for
/// `<foreignObject>` labels.
///
/// This is intended for raster outputs and UI previews where `<foreignObject>` is not
/// supported. It does not aim for upstream Mermaid DOM parity.
pub fn render_svg_readable_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        &SvgPipeline::readable(),
    )
}

pub fn render_svg_resvg_safe_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        &SvgPipeline::resvg_safe(),
    )
}

pub async fn render_svg(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    // This async API is runtime-agnostic: rendering is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    render_svg_sync(engine, text, parse_options, layout_options, svg_options)
}

pub async fn render_svg_readable(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    // This async API is runtime-agnostic: rendering is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    render_svg_readable_sync(engine, text, parse_options, layout_options, svg_options)
}

pub async fn render_svg_with_pipeline(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
    pipeline: &SvgPipeline,
) -> Result<Option<String>> {
    // This async API is runtime-agnostic: rendering is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        pipeline,
    )
}

pub async fn render_svg_resvg_safe(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<String>> {
    // This async API is runtime-agnostic: rendering is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    render_svg_resvg_safe_sync(engine, text, parse_options, layout_options, svg_options)
}

#[cfg(test)]
mod svg_pipeline_tests {
    use super::*;
    use serde_json::{Value, json};
    use std::borrow::Cow;

    fn task_by_id<'a>(model: &'a Value, id: &str) -> &'a Value {
        model["tasks"]
            .as_array()
            .expect("Gantt tasks should be an array")
            .iter()
            .find(|task| task["id"].as_str() == Some(id))
            .unwrap_or_else(|| panic!("missing Gantt task {id} in {model}"))
    }

    #[test]
    fn readable_helper_routes_through_readable_pipeline() {
        let engine = merman_core::Engine::new();
        let layout = LayoutOptions::headless_svg_defaults();
        let svg = SvgRenderOptions::default();
        let source = "flowchart TD\nA[Hello] --> B[World]";

        let helper = render_svg_readable_sync(
            &engine,
            source,
            merman_core::ParseOptions::default(),
            &layout,
            &svg,
        )
        .unwrap()
        .unwrap();
        let pipeline = render_svg_with_pipeline_sync(
            &engine,
            source,
            merman_core::ParseOptions::default(),
            &layout,
            &svg,
            &SvgPipeline::readable(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(helper, pipeline);
        assert!(pipeline.contains("data-merman-foreignobject"));
    }

    #[test]
    fn default_svg_helper_stays_parity_without_pipeline_cleanup() {
        let engine = merman_core::Engine::new();
        let layout = LayoutOptions::headless_svg_defaults();
        let svg = SvgRenderOptions::default();
        let source = "flowchart TD\nA[Hello] --> B[World]";

        let default_svg = render_svg_sync(
            &engine,
            source,
            merman_core::ParseOptions::default(),
            &layout,
            &svg,
        )
        .unwrap()
        .unwrap();
        let parity_pipeline = render_svg_with_pipeline_sync(
            &engine,
            source,
            merman_core::ParseOptions::default(),
            &layout,
            &svg,
            &SvgPipeline::parity(),
        )
        .unwrap()
        .unwrap();
        let readable = render_svg_readable_sync(
            &engine,
            source,
            merman_core::ParseOptions::default(),
            &layout,
            &svg,
        )
        .unwrap()
        .unwrap();

        assert_eq!(default_svg, parity_pipeline);
        assert_ne!(default_svg, readable);
    }

    #[test]
    fn layout_helpers_share_headless_operation_semantics() {
        let source = "flowchart TD\nA[Hello] --> B[World]";
        let renderer = HeadlessRenderer::new().with_lenient_parsing();
        let free_layout =
            layout_diagram_sync(&renderer.engine, source, renderer.parse, &renderer.layout)
                .unwrap()
                .unwrap();
        let renderer_layout = renderer.layout_diagram_sync(source).unwrap().unwrap();

        assert_eq!(free_layout.semantic, renderer_layout.semantic);
        assert_eq!(
            free_layout.meta.diagram_type,
            renderer_layout.meta.diagram_type
        );
        assert!(
            renderer
                .layout_diagram_sync("not a mermaid diagram")
                .unwrap()
                .is_none()
        );
    }

    struct MetadataComment;

    impl SvgPostprocessor for MetadataComment {
        fn name(&self) -> &'static str {
            "metadata-comment"
        }

        fn process<'a>(
            &self,
            svg: Cow<'a, str>,
            ctx: &SvgPostprocessContext<'_>,
        ) -> RenderResult<Cow<'a, str>> {
            Ok(Cow::Owned(format!(
                "{}<!--type={};title={};id={}-->",
                svg,
                ctx.diagram_type().unwrap_or(""),
                ctx.diagram_title().unwrap_or(""),
                ctx.svg_id().unwrap_or("")
            )))
        }
    }

    #[test]
    fn render_svg_with_pipeline_passes_parsed_metadata() {
        let renderer = HeadlessRenderer::new().with_diagram_id("host-style");
        let source = "---\ntitle: Host Pipeline\n---\nflowchart TD\nA --> B";
        let pipeline = SvgPipeline::parity().with_postprocessor(MetadataComment);

        let svg = renderer
            .render_svg_with_pipeline_sync(source, &pipeline)
            .unwrap()
            .unwrap();

        assert!(svg.contains("type=flowchart"));
        assert!(svg.contains("title=Host Pipeline"));
        assert!(svg.contains("id=host-style"));
    }

    #[test]
    fn render_svg_sync_applies_scoped_theme_css_once() {
        let renderer = HeadlessRenderer::new().with_diagram_id("theme-css");
        let source = r##"%%{init: {"themeCSS": ".node rect { fill: #123456; } @media (max-width: 600px) { text { fill: #654321; } }"}}%%
flowchart TD
  A[Hello] --> B[World]
"##;

        let svg = renderer.render_svg_sync(source).unwrap().unwrap();

        assert_eq!(
            svg.matches(r#"data-merman-postprocess="scoped-css""#)
                .count(),
            1
        );
        assert!(svg.contains("#theme-css .node rect { fill: #123456; }"));
        assert!(svg.contains("@media (max-width: 600px) {"));
        assert!(svg.contains("#theme-css text { fill: #654321; }"));
    }

    #[test]
    fn render_svg_sync_applies_external_site_theme_to_plain_source() {
        let renderer = HeadlessRenderer::new()
            .with_site_config(merman_core::MermaidConfig::from_value(json!({
                "theme": "neutral"
            })))
            .with_diagram_id("external-theme");
        let source = "flowchart TD\n  A[Plain source] --> B[External theme]";

        let svg = renderer.render_svg_sync(source).unwrap().unwrap();

        assert!(
            svg.contains("#external-theme .labelBkg{background-color:rgba(255, 255, 255, 0.5);}")
        );
    }

    #[test]
    fn render_svg_sync_applies_external_neo_theme_to_plain_source() {
        let renderer = HeadlessRenderer::new()
            .with_site_config(merman_core::MermaidConfig::from_value(json!({
                "theme": "neo"
            })))
            .with_diagram_id("external-neo");
        let source = "flowchart TD\n  A[Plain source] --> B[Neo theme]";

        let svg = renderer.render_svg_sync(source).unwrap().unwrap();

        assert!(svg.contains("fill:#ffffff;stroke:#000000;stroke-width:2px;"));
        assert!(
            svg.contains("#external-neo .labelBkg{background-color:rgba(204, 204, 204, 0.5);}")
        );
    }

    #[test]
    fn render_svg_sync_falls_back_for_unknown_external_theme() {
        let renderer = HeadlessRenderer::new()
            .with_site_config(merman_core::MermaidConfig::from_value(json!({
                "theme": "unknown"
            })))
            .with_diagram_id("external-unknown");
        let source = "flowchart TD\n  A[Plain source] --> B[Unknown theme]";

        let svg = renderer.render_svg_sync(source).unwrap().unwrap();

        assert!(svg.contains("fill:#ECECFF;stroke:#9370DB;stroke-width:1px;"));
        assert!(
            svg.contains("#external-unknown .labelBkg{background-color:rgba(232, 232, 232, 0.5);}")
        );
    }

    #[test]
    fn supported_host_theme_presets_are_separate_from_mermaid_themes() {
        assert_eq!(
            supported_host_theme_presets(),
            &[
                "editor-light",
                "editor-dark",
                "one-dark",
                "gruvbox-light",
                "gruvbox-dark",
                "ayu-light",
                "ayu-dark"
            ]
        );
        assert!(merman_core::supported_themes().contains(&"default"));
        assert!(!merman_core::supported_themes().contains(&"one-dark"));
    }

    #[test]
    fn render_svg_with_site_config_is_request_scoped() {
        let renderer = HeadlessRenderer::new().with_diagram_id("request-site-theme");
        let source = "flowchart TD\n  A[Plain source] --> B[Request theme]";

        let themed = renderer
            .render_svg_with_site_config_sync(
                source,
                merman_core::MermaidConfig::from_value(json!({
                    "theme": "neutral"
                })),
            )
            .unwrap()
            .unwrap();
        let plain = renderer.render_svg_sync(source).unwrap().unwrap();

        assert!(
            themed.contains(
                "#request-site-theme .labelBkg{background-color:rgba(255, 255, 255, 0.5);}"
            )
        );
        assert!(
            plain.contains(
                "#request-site-theme .labelBkg{background-color:rgba(232, 232, 232, 0.5);}"
            )
        );
    }

    #[test]
    fn host_theme_profile_applies_editor_roles_and_pipeline() {
        let profile = HostThemeProfile::editor_dark();
        let compiled = profile.compile();
        let renderer = HeadlessRenderer::new()
            .with_compiled_host_theme(&compiled)
            .with_diagram_id("host-theme-profile");

        let svg = renderer
            .render_svg_sync(
                r##"%%{init: {"themeCSS": ".node rect { stroke-width: 3px !important; }"}}%%
flowchart TD
  A[Host] --> B[Theme]
"##,
            )
            .unwrap()
            .unwrap();

        assert!(svg.contains("#111827"), "{svg}");
        assert!(svg.contains("#e5e7eb"), "{svg}");
        assert!(svg.contains("#94a3b8"), "{svg}");
        assert!(svg.contains("background-color: #0f172a;"), "{svg}");
        assert!(!svg.contains("<foreignObject"), "{svg}");
        assert!(!svg.contains("!important"), "{svg}");
    }

    #[test]
    fn host_theme_profile_raw_theme_variables_win_over_roles() {
        let profile = HostThemeProfile::builder()
            .roles(HostThemeRoles {
                border: Some("#111111".to_string()),
                text: Some("#eeeeee".to_string()),
                ..HostThemeRoles::default()
            })
            .theme_variable("nodeBorder", "#abcdef")
            .build();
        let renderer = HeadlessRenderer::new()
            .with_host_theme(&profile)
            .with_diagram_id("host-theme-override");

        let svg = renderer
            .render_svg_sync("flowchart TD\n  A[Host]")
            .unwrap()
            .unwrap();

        assert!(svg.contains("#abcdef"), "{svg}");
        assert!(svg.contains("#eeeeee"), "{svg}");
    }

    #[test]
    fn render_svg_with_host_theme_is_request_scoped() {
        let renderer = HeadlessRenderer::new().with_diagram_id("request-host-theme");
        let profile = HostThemeProfile::editor_dark();
        let source = "flowchart TD\n  A[Host] --> B[Theme]";

        let themed = renderer
            .render_svg_with_host_theme_sync(source, &profile)
            .unwrap()
            .unwrap();
        let plain = renderer.render_svg_sync(source).unwrap().unwrap();

        assert!(themed.contains("#111827"), "{themed}");
        assert!(themed.contains("background-color: #0f172a;"), "{themed}");
        assert!(!themed.contains("<foreignObject"), "{themed}");
        assert!(!plain.contains("background-color: #0f172a;"), "{plain}");
        assert!(plain.contains("<foreignObject"), "{plain}");
    }

    #[test]
    fn render_svg_with_compiled_host_theme_reuses_compiled_profile() {
        let renderer = HeadlessRenderer::new().with_diagram_id("request-compiled-theme");
        let compiled = HostThemeProfile::one_dark().compile();
        let source = "flowchart TD\n  A[Compiled] --> B[Theme]";

        let svg = renderer
            .render_svg_with_compiled_host_theme_sync(source, &compiled)
            .unwrap()
            .unwrap();

        assert!(svg.contains("#21252b"), "{svg}");
        assert!(svg.contains("background-color: #282c34;"), "{svg}");
        assert!(!svg.contains("<foreignObject"), "{svg}");
    }

    #[test]
    fn headless_renderer_fixed_time_controls_semantic_parse() {
        let renderer = HeadlessRenderer::new()
            .with_fixed_today(Some(
                chrono::NaiveDate::from_ymd_opt(2026, 2, 15).expect("valid fixed today"),
            ))
            .with_fixed_local_offset_minutes(Some(0));
        let parsed = renderer
            .parse_diagram_sync(
                r#"gantt
dateFormat MM-DD
section Demo
Missing year: id1,03-01,1d
Missing ref: id2,after missing,1d
"#,
            )
            .unwrap()
            .unwrap();

        assert_eq!(
            task_by_id(&parsed.model, "id1")["startTime"].as_i64(),
            Some(1_772_323_200_000)
        );
        assert_eq!(
            task_by_id(&parsed.model, "id2")["startTime"].as_i64(),
            Some(1_771_113_600_000)
        );
    }
}

/// Convenience wrapper that bundles a [`merman_core::Engine`] and common options for headless rendering.
///
/// This is intended for UI integrations where passing 4-5 separate parameters per call is
/// noisy. It stays runtime-agnostic: all work is CPU-bound and does not perform I/O.
#[derive(Clone)]
pub struct HeadlessRenderer {
    pub engine: merman_core::Engine,
    pub parse: merman_core::ParseOptions,
    pub layout: LayoutOptions,
    pub svg: SvgRenderOptions,
    /// Optional renderer-owned SVG output pipeline.
    ///
    /// A fresh renderer leaves this unset so `render_svg_sync` keeps the Mermaid-parity SVG
    /// contract. Host theme helpers set this to the compiled profile pipeline so the profile's
    /// output settings travel with the renderer instead of requiring each call to pass them again.
    pub svg_pipeline: Option<SvgPipeline>,
}

impl Default for HeadlessRenderer {
    fn default() -> Self {
        Self {
            engine: merman_core::Engine::new(),
            parse: merman_core::ParseOptions::default(),
            layout: LayoutOptions::headless_svg_defaults(),
            svg: SvgRenderOptions::default(),
            svg_pipeline: None,
        }
    }
}

impl HeadlessRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_site_config(mut self, site_config: merman_core::MermaidConfig) -> Self {
        self.engine = self.engine.with_site_config(site_config);
        self
    }

    pub fn with_host_theme(mut self, profile: &HostThemeProfile) -> Self {
        let compiled = profile.compile();
        let pipeline = compiled.pipeline();
        self.engine = self.engine.with_site_config(compiled.site_config);
        self.svg_pipeline = Some(pipeline);
        self
    }

    pub fn with_compiled_host_theme(mut self, theme: &CompiledHostTheme) -> Self {
        self.engine = self.engine.with_site_config(theme.site_config.clone());
        self.svg_pipeline = Some(theme.pipeline());
        self
    }

    pub fn with_svg_pipeline(mut self, pipeline: SvgPipeline) -> Self {
        self.svg_pipeline = Some(pipeline);
        self
    }

    pub fn with_fixed_today(mut self, today: Option<chrono::NaiveDate>) -> Self {
        self.engine = self.engine.with_fixed_today(today);
        self
    }

    pub fn with_fixed_local_offset_minutes(mut self, offset_minutes: Option<i32>) -> Self {
        self.engine = self.engine.with_fixed_local_offset_minutes(offset_minutes);
        self
    }

    pub fn with_parse_options(mut self, parse: merman_core::ParseOptions) -> Self {
        self.parse = parse;
        self
    }

    pub fn with_strict_parsing(self) -> Self {
        self.with_parse_options(merman_core::ParseOptions::strict())
    }

    pub fn with_lenient_parsing(self) -> Self {
        self.with_parse_options(merman_core::ParseOptions::lenient())
    }

    pub fn with_layout_options(mut self, layout: LayoutOptions) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_svg_options(mut self, svg: SvgRenderOptions) -> Self {
        self.svg = svg;
        self
    }

    pub fn with_diagram_id(mut self, diagram_id: &str) -> Self {
        self.svg.diagram_id = Some(sanitize_svg_id(diagram_id));
        self
    }

    pub fn with_text_measurer(
        mut self,
        measurer: std::sync::Arc<dyn TextMeasurer + Send + Sync>,
    ) -> Self {
        self.layout = self.layout.with_text_measurer(measurer);
        self
    }

    pub fn with_math_renderer(
        mut self,
        renderer: std::sync::Arc<dyn MathRenderer + Send + Sync>,
    ) -> Self {
        self.layout = self.layout.with_math_renderer(renderer.clone());
        self.svg.math_renderer = Some(renderer);
        self
    }

    pub fn with_vendored_text_measurer(self) -> Self {
        self.with_text_measurer(std::sync::Arc::new(
            VendoredFontMetricsTextMeasurer::default(),
        ))
    }

    pub fn with_deterministic_text_measurer(self) -> Self {
        self.with_text_measurer(std::sync::Arc::new(DeterministicTextMeasurer::default()))
    }

    pub fn parse_metadata_sync(&self, text: &str) -> Result<Option<merman_core::ParseMetadata>> {
        Ok(self.engine.parse_metadata_sync(text, self.parse)?)
    }

    pub fn parse_diagram_sync(&self, text: &str) -> Result<Option<merman_core::ParsedDiagram>> {
        Ok(self.engine.parse_diagram_sync(text, self.parse)?)
    }

    pub fn layout_diagram_sync(&self, text: &str) -> Result<Option<LayoutedDiagram>> {
        layout_diagram_sync(&self.engine, text, self.parse, &self.layout)
    }

    pub fn render_layouted_svg_sync(&self, diagram: &LayoutedDiagram) -> Result<String> {
        self.apply_default_svg_pipeline(
            render_layouted_svg(diagram, self.layout.text_measurer.as_ref(), &self.svg)?,
            &diagram.meta,
        )
    }

    pub fn render_layouted_svg_sync_with(
        &self,
        diagram: &LayoutedDiagram,
        svg: &SvgRenderOptions,
    ) -> Result<String> {
        self.apply_default_svg_pipeline(
            render_layouted_svg(diagram, self.layout.text_measurer.as_ref(), svg)?,
            &diagram.meta,
        )
    }

    pub fn render_svg_sync(&self, text: &str) -> Result<Option<String>> {
        if let Some(pipeline) = &self.svg_pipeline {
            return self.render_svg_with_pipeline_sync(text, pipeline);
        }
        render_svg_sync(&self.engine, text, self.parse, &self.layout, &self.svg)
    }

    /// Renders one diagram with additional Mermaid site config defaults.
    ///
    /// The override applies only to this call. Diagram frontmatter and `%%{init}%%` directives
    /// still merge on top of the supplied site config, matching Mermaid's per-diagram config
    /// precedence.
    pub fn render_svg_with_site_config_sync(
        &self,
        text: &str,
        site_config: merman_core::MermaidConfig,
    ) -> Result<Option<String>> {
        self.clone()
            .with_site_config(site_config)
            .render_svg_sync(text)
    }

    /// Renders one diagram with a host/editor theme profile.
    ///
    /// This is the request-level counterpart to [`HeadlessRenderer::with_host_theme`]: it does not
    /// mutate this renderer, and it applies the profile's compiled SVG output pipeline only to this
    /// render call.
    pub fn render_svg_with_host_theme_sync(
        &self,
        text: &str,
        profile: &HostThemeProfile,
    ) -> Result<Option<String>> {
        self.clone().with_host_theme(profile).render_svg_sync(text)
    }

    /// Renders one diagram with a precompiled host/editor theme.
    ///
    /// Prefer this in hot editor paths when the same profile is reused for many diagrams.
    pub fn render_svg_with_compiled_host_theme_sync(
        &self,
        text: &str,
        theme: &CompiledHostTheme,
    ) -> Result<Option<String>> {
        self.clone()
            .with_compiled_host_theme(theme)
            .render_svg_sync(text)
    }

    pub fn render_svg_with_pipeline_sync(
        &self,
        text: &str,
        pipeline: &SvgPipeline,
    ) -> Result<Option<String>> {
        render_svg_with_pipeline_sync(
            &self.engine,
            text,
            self.parse,
            &self.layout,
            &self.svg,
            pipeline,
        )
    }

    /// Renders SVG and applies a best-effort readability fallback for `<foreignObject>` labels.
    ///
    /// Many headless SVG renderers and rasterizers do not fully support HTML inside
    /// `<foreignObject>`. This helper overlays extracted label text as `<text>/<tspan>` so
    /// consumers can still display something readable.
    pub fn render_svg_readable_sync(&self, text: &str) -> Result<Option<String>> {
        self.render_svg_with_pipeline_sync(text, &SvgPipeline::readable())
    }

    pub fn render_svg_resvg_safe_sync(&self, text: &str) -> Result<Option<String>> {
        self.render_svg_with_pipeline_sync(text, &SvgPipeline::resvg_safe())
    }

    fn apply_default_svg_pipeline(
        &self,
        svg: String,
        meta: &merman_render::model::LayoutMeta,
    ) -> Result<String> {
        let Some(pipeline) = &self.svg_pipeline else {
            return Ok(svg);
        };
        let metadata = SvgPostprocessMetadata::from_svg(&svg)
            .with_diagram_type(meta.diagram_type.clone())
            .with_optional_diagram_title(meta.title.clone());
        apply_svg_pipeline_with_metadata(&svg, pipeline, &metadata)
    }

    #[cfg(feature = "raster")]
    pub fn render_png_sync(
        &self,
        text: &str,
        raster: &raster::RasterOptions,
    ) -> raster::Result<Option<Vec<u8>>> {
        let pipeline = self
            .svg_pipeline
            .clone()
            .unwrap_or_else(SvgPipeline::resvg_safe);
        let Some(svg) = self.render_svg_with_pipeline_sync(text, &pipeline)? else {
            return Ok(None);
        };
        Ok(Some(raster::svg_to_png(&svg, raster)?))
    }

    #[cfg(feature = "raster")]
    pub fn render_jpeg_sync(
        &self,
        text: &str,
        raster: &raster::RasterOptions,
    ) -> raster::Result<Option<Vec<u8>>> {
        let pipeline = self
            .svg_pipeline
            .clone()
            .unwrap_or_else(SvgPipeline::resvg_safe);
        let Some(svg) = self.render_svg_with_pipeline_sync(text, &pipeline)? else {
            return Ok(None);
        };
        Ok(Some(raster::svg_to_jpeg(&svg, raster)?))
    }

    #[cfg(feature = "raster")]
    pub fn render_pdf_sync(&self, text: &str) -> raster::Result<Option<Vec<u8>>> {
        let pipeline = self
            .svg_pipeline
            .clone()
            .unwrap_or_else(SvgPipeline::resvg_safe);
        let Some(svg) = self.render_svg_with_pipeline_sync(text, &pipeline)? else {
            return Ok(None);
        };
        Ok(Some(raster::svg_to_pdf(&svg)?))
    }
}
