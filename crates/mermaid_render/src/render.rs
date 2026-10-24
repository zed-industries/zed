use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use anyhow::{Context as _, Result, anyhow};
use gpui::{Font, FontStyle, FontWeight, SharedString, TextRun, TextSystem, WindowTextSystem, px};
use merman::svg::{
    DeterministicTextMeasurer, MeasurementProfileId, TextMeasurementPolicy, TextMeasurementProfile,
    TextMeasurementProfileIdentity, TextStyle,
};
use merman::{
    Engine, OperationControl, RenderOutput, RenderRequest, Renderer, SvgEnvironment, SvgRequest,
};

use crate::{MermaidTheme, css_color};

struct GpuiTextWidth {
    text_system: WindowTextSystem,
    font: Font,
}

impl GpuiTextWidth {
    fn new(text_system: Arc<TextSystem>, font: Font) -> Self {
        Self {
            text_system: WindowTextSystem::new(text_system),
            font,
        }
    }

    fn measure(&self, text: &str, style: &TextStyle) -> f64 {
        let text = text.strip_suffix('\r').unwrap_or(text);
        if text.is_empty() {
            return 0.0;
        }

        let mut font = self.font.clone();
        if let Some(weight) = style.font_weight.as_deref().and_then(parse_font_weight) {
            font.weight = weight;
        }
        font.style = match style.font_style.as_deref().map(str::trim) {
            Some(style) if style.eq_ignore_ascii_case("italic") => FontStyle::Italic,
            Some(style) if style.to_ascii_lowercase().starts_with("oblique") => FontStyle::Oblique,
            _ => FontStyle::Normal,
        };

        let text = SharedString::from(text.to_owned());
        let run = TextRun {
            len: text.len(),
            font,
            ..Default::default()
        };
        f64::from(f32::from(
            self.text_system
                .shape_line(text, px(style.font_size as f32), &[run], None)
                .width,
        ))
    }
}

fn parse_font_weight(weight: &str) -> Option<FontWeight> {
    match weight.trim().to_ascii_lowercase().as_str() {
        "normal" => Some(FontWeight::NORMAL),
        "bold" | "bolder" => Some(FontWeight::BOLD),
        "lighter" => Some(FontWeight::LIGHT),
        weight => weight
            .parse::<f32>()
            .ok()
            .filter(|weight| weight.is_finite() && (1.0..=1000.0).contains(weight))
            .map(FontWeight),
    }
}

pub(super) fn render_mermaid(
    source: &str,
    theme: &MermaidTheme,
    text_system: Arc<TextSystem>,
) -> Result<String> {
    let identity = TextMeasurementProfileIdentity::new(
        MeasurementProfileId::new("zed.gpui")?,
        env!("CARGO_PKG_VERSION"),
    )?;
    let gpui_text_width = GpuiTextWidth::new(text_system, theme.measurement_font());
    let measurer = DeterministicTextMeasurer::default()
        .with_width_callback(move |text, style| gpui_text_width.measure(text, style));
    let profile = TextMeasurementProfile::new(identity, measurer);
    render_mermaid_with_text_policy(source, theme, TextMeasurementPolicy::uniform(profile))
}

fn render_mermaid_with_text_policy(
    source: &str,
    theme: &MermaidTheme,
    text_measurement_policy: TextMeasurementPolicy,
) -> Result<String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let environment =
        SvgEnvironment::deterministic().with_text_measurement_policy(text_measurement_policy);

    // Apply merman's raster-safe pipeline before Zed-specific styling. The
    // pipeline handles generic rasterizer compatibility cleanup: foreignObject
    // fallback text, unsupported CSS removal, and invalid SVG attribute cleanup.
    // Zed also strips merman's existing `!important` declarations before
    // injecting its own theme CSS so host styling wins consistently in usvg/resvg.
    let pipeline = merman::svg::SvgPipeline::resvg_safe()
        .with_postprocessor(merman::svg::CssOverridePostprocessor::strip_existing_important());
    let request = SvgRequest {
        environment,
        options: merman::svg::SvgRenderOptions {
            diagram_id: Some(diagram_id),
            ..Default::default()
        },
        pipeline: Some(pipeline),
        ..Default::default()
    };
    let output = Renderer::new()
        .with_engine(Engine::new().with_site_config(to_merman_config(theme)))
        .render(RenderRequest::svg(source, OperationControl::new(), request))
        .context("merman render failed")?;
    let RenderOutput::Svg(Some(svg)) = output else {
        return Err(anyhow!("merman returned no SVG for the given input"));
    };

    let (svg, _) = svg.into_parts();
    Ok(svg)
}

fn to_merman_config(theme: &MermaidTheme) -> merman::MermaidConfig {
    let primary = css_color(theme.primary_color);
    let primary_text = css_color(theme.primary_text_color);
    let primary_border = css_color(theme.primary_border_color);
    let line = css_color(theme.line_color);
    let secondary = css_color(theme.secondary_color);
    let tertiary = css_color(theme.tertiary_color);
    let background = css_color(theme.background);
    let cluster_bg = css_color(theme.cluster_background);
    let cluster_border = css_color(theme.cluster_border);
    let edge_label_bg = css_color(theme.edge_label_background);
    let text = css_color(theme.text_color);
    let note_bg = css_color(theme.note_background);
    let note_border = css_color(theme.note_border);
    let actor_bg = css_color(theme.actor_background);
    let actor_border = css_color(theme.actor_border);
    let activation_bg = css_color(theme.activation_background);
    let activation_border = css_color(theme.activation_border);
    let er_odd = css_color(theme.er_attr_bg_odd);
    let er_even = css_color(theme.er_attr_bg_even);
    let git: [String; 8] = theme.git_branch_colors.map(css_color);
    let git_lbl: [String; 8] = theme.git_branch_label_colors.map(css_color);
    let svg_font_family = theme.svg_font_family();

    let mut theme_vars = serde_json::json!({
        "primaryColor": primary,
        "primaryTextColor": primary_text,
        "primaryBorderColor": primary_border,
        "lineColor": line,
        "secondaryColor": secondary,
        "secondaryTextColor": text,
        "tertiaryColor": tertiary,
        "tertiaryTextColor": text,
        "background": background,
        "mainBkg": primary,
        "nodeBorder": primary_border,
        "nodeTextColor": primary_text,
        "clusterBkg": cluster_bg,
        "clusterBorder": cluster_border,
        "titleColor": text,
        "edgeLabelBackground": edge_label_bg,
        "textColor": text,
        "fontFamily": svg_font_family,
        "noteBkgColor": note_bg,
        "noteBorderColor": note_border,
        "noteTextColor": text,
        "actorBkg": actor_bg,
        "actorBorder": actor_border,
        "actorTextColor": primary_text,
        "labelTextColor": text,
        "loopTextColor": text,
        "signalColor": text,
        "signalTextColor": text,
        "activationBkgColor": activation_bg,
        "activationBorderColor": activation_border,
        "classText": text,
        "labelColor": primary_text,
        "attributeBackgroundColorOdd": er_odd,
        "attributeBackgroundColorEven": er_even,
        "pieTitleTextColor": text,
        "pieSectionTextColor": text,
        "pieLegendTextColor": text,
        "pieStrokeColor": primary_border,
        "pieOuterStrokeColor": primary_border,
        "quadrant1Fill": primary,
        "quadrant2Fill": primary,
        "quadrant3Fill": primary,
        "quadrant4Fill": primary,
        "quadrant1TextFill": text,
        "quadrant2TextFill": text,
        "quadrant3TextFill": text,
        "quadrant4TextFill": text,
        "quadrantPointFill": line,
        "quadrantPointTextFill": text,
        "quadrantTitleFill": text,
        "quadrantXAxisTextFill": text,
        "quadrantYAxisTextFill": text,
        "quadrantExternalBorderStrokeFill": primary_border,
        "quadrantInternalBorderStrokeFill": primary_border,
    });

    if let Some(map) = theme_vars.as_object_mut() {
        for (((i, color), label), pie_number) in git.iter().enumerate().zip(&git_lbl).zip(1..) {
            map.insert(format!("cScale{i}"), color.clone().into());
            map.insert(format!("cScaleLabel{i}"), label.clone().into());
            map.insert(format!("pie{pie_number}"), color.clone().into());
        }
    }

    merman::MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        // Mermaid 12 defaults to the `neo` look; Zed's injected CSS targets classic output.
        "look": "classic",
        "darkMode": theme.dark_mode,
        "fontFamily": svg_font_family,
        // resvg can't rasterize HTML `<foreignObject>` labels, so merman's
        // raster-safe pipeline replaces them with wrapped native SVG text.
        "htmlLabels": true,
        "flowchart": {
            "htmlLabels": true,
            "padding": 16,
        },
        "themeVariables": theme_vars,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_stage_applies_resvg_safe_pipeline() {
        let html_label_source =
            "classDiagram\n    class Shelter {\n        -List~Animal~ animals\n    }";
        let html_label_svg = render_mermaid_with_text_policy(
            html_label_source,
            &MermaidTheme::default(),
            TextMeasurementPolicy::deterministic(),
        )
        .expect("render failed");

        assert!(
            !html_label_svg.contains("<foreignObject"),
            "got: {html_label_svg}"
        );
        assert!(
            !html_label_svg.contains("&amp;lt;"),
            "got: {html_label_svg}"
        );

        let css_source = "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi";
        let css_svg = render_mermaid_with_text_policy(
            css_source,
            &MermaidTheme::default(),
            TextMeasurementPolicy::deterministic(),
        )
        .expect("render failed");

        assert!(!css_svg.contains("@keyframes"), "got: {css_svg}");
        assert!(!css_svg.contains("@-webkit-keyframes"), "got: {css_svg}");
        assert!(!css_svg.contains(":root"), "got: {css_svg}");
        assert!(!css_svg.contains("animation:"), "got: {css_svg}");
        assert!(!css_svg.contains("animation-name:"), "got: {css_svg}");
        assert!(!css_svg.contains("!important"), "got: {css_svg}");
    }

    #[test]
    fn long_flowchart_labels_render_as_wrapped_svg_text() {
        let source = "flowchart TD\n    \
            A[\"Pass 2: search transcript with annotation blocks excised, \
            map offsets back to buffer space\"] --> \
            |ambiguous or zero| B[\"Error describing where matches were found\"]";
        let svg = render_mermaid_with_text_policy(
            source,
            &MermaidTheme::default(),
            TextMeasurementPolicy::deterministic(),
        )
        .expect("render failed");

        assert!(!svg.contains("<foreignObject"), "got: {svg}");
        assert!(!svg.contains(
            "Pass 2: search transcript with annotation blocks excised, map offsets back to buffer space</text>"
        ));
        let wrapped_line_count = svg.matches("merman-foreignobject-fallback-text").count();
        assert!(
            wrapped_line_count > 3,
            "expected long labels to wrap onto multiple SVG text lines, got {wrapped_line_count}: {svg}"
        );
    }

    #[test]
    fn gpui_width_callback_is_used_for_flowchart_wrapping() {
        let source = r#"%%{init: {"flowchart": {"wrappingWidth": 220}}}%%
flowchart TD
    A["Receive a new deployment request from the automated release pipeline"] --> B["Prepare release notes<br/>Notify release team"]
"#;
        let text_system = Arc::new(TextSystem::new(Arc::new(gpui::NoopTextSystem::new())));
        let mut theme = MermaidTheme::default();
        theme.font = gpui::font("Lilex");
        let default_svg =
            render_mermaid_with_text_policy(source, &theme, TextMeasurementPolicy::deterministic())
                .expect("default render failed");
        let svg = render_mermaid(source, &theme, text_system).expect("GPUI render failed");

        let default_line_count = default_svg
            .matches("merman-foreignobject-fallback-text")
            .count();
        let gpui_line_count = svg.matches("merman-foreignobject-fallback-text").count();
        assert!(
            gpui_line_count > default_line_count,
            "GPUI widths did not change flowchart wrapping: default={default_line_count}, GPUI={gpui_line_count}"
        );
        assert!(svg.contains("Prepare release notes"));
        assert!(svg.contains("Notify release team"));
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        assert!(svg.contains(r#"font-family="Lilex, sans-serif""#));
    }

    #[test]
    fn class_diagram_labels_keep_sixteen_pixel_font_size() {
        let source = r#"classDiagram
            class User {
                +String id
                +String name
                +signIn()
            }
        "#;
        let theme = MermaidTheme::default();
        let svg =
            render_mermaid_with_text_policy(source, &theme, TextMeasurementPolicy::deterministic())
                .expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        assert_eq!(text_font_size(tree.root(), "User"), Some(16.0));
        assert_eq!(text_font_size(tree.root(), "+String name"), Some(16.0));
    }

    #[test]
    fn mindmap_renders_drawable_svg() {
        let source = r#"mindmap
            root((Application))
                Frontend
                    Components
                    State
                    Routing
                Backend
                    API
                    Authentication
                    Jobs
                Data
                    Database
                    Cache
                    Backups
                Operations
                    Monitoring
                    Deployment
        "#;
        let theme = MermaidTheme::default();
        let svg =
            render_mermaid_with_text_policy(source, &theme, TextMeasurementPolicy::deterministic())
                .expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        assert!(tree.root().has_children(), "got: {svg}");
        assert!(
            text_font_size(tree.root(), "Application").is_some(),
            "got: {svg}"
        );
    }

    fn text_font_size(group: &usvg::Group, expected_text: &str) -> Option<f32> {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => {
                    if let Some(font_size) = text_font_size(group, expected_text) {
                        return Some(font_size);
                    }
                }
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        if chunk.text() == expected_text {
                            return chunk.spans().first().map(|span| span.font_size().get());
                        }
                    }
                }
                usvg::Node::Path(_) | usvg::Node::Image(_) => {}
            }
        }

        None
    }
}
