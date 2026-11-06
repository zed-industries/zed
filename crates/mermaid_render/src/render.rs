use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context as _, Result, anyhow};
use merman::{Engine, OperationControl, RenderOutput, RenderRequest, Renderer, SvgRequest};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(source: &str, theme: &MermaidTheme) -> Result<String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let pipeline = merman::svg::SvgPipeline::resvg_safe()
        .with_postprocessor(merman::svg::CssOverridePostprocessor::strip_existing_important());
    // Apply merman's raster-safe pipeline before Zed-specific styling. The
    // pipeline handles generic rasterizer compatibility cleanup: foreignObject
    // fallback text, unsupported CSS removal, and invalid SVG attribute cleanup.
    // Zed also strips merman's existing `!important` declarations before
    // injecting its own theme CSS so host styling wins consistently in usvg/resvg.
    let request = SvgRequest {
        environment: merman::SvgEnvironment::deterministic(),
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
        "fontFamily": theme.font_family,
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
        // Mermaid 12 defaults to the neo look; Zed's injected CSS targets classic output.
        "look": "classic",
        "darkMode": theme.dark_mode,
        "fontFamily": theme.font_family,
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
        let html_label_svg =
            render_mermaid(html_label_source, &MermaidTheme::default()).expect("render failed");

        assert!(
            !html_label_svg.contains("<foreignObject"),
            "got: {html_label_svg}"
        );
        assert!(
            !html_label_svg.contains("&amp;lt;"),
            "got: {html_label_svg}"
        );

        let css_source = "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi";
        let css_svg = render_mermaid(css_source, &MermaidTheme::default()).expect("render failed");

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
        let svg = render_mermaid(source, &MermaidTheme::default()).expect("render failed");

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
    fn class_diagram_labels_keep_sixteen_pixel_font_size() {
        let source = r#"classDiagram
            class User {
                +String id
                +String name
                +signIn()
            }
        "#;
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        assert_eq!(text_font_size(tree.root(), "User"), Some(16.0));
        assert_eq!(text_font_size(tree.root(), "+String name"), Some(16.0));
    }

    #[test]
    fn state_diagram_descriptions_do_not_overlap_title() {
        // Regression for zed#66582: merman < 0.9.1 dropped foreignObject transforms,
        // so state descriptions stacked on top of each other at the title position.
        let source = r#"stateDiagram-v2
    [*] --> Teardown
    Teardown : members kicked
    Teardown : group deleted
"#;
        let svg = render_mermaid(source, &MermaidTheme::default()).expect("render failed");

        assert!(
            svg.contains("members kicked"),
            "missing first description: {svg}"
        );
        assert!(
            svg.contains("group deleted"),
            "missing second description: {svg}"
        );
        assert!(
            !svg.contains("<foreignObject"),
            "expected resvg-safe fallback text, got foreignObject: {svg}"
        );

        // The second description must keep its vertical offset. In merman < 0.9.1
        // both labels landed on the same baseline; 0.9.1+ preserves the
        // foreignObject translate as a matrix(... 33) on the fallback group.
        assert!(
            svg.contains("matrix(1 0 0 1 ") && svg.contains(" 33)"),
            "expected a preserved description transform with a 33px y offset: {svg}"
        );
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
        let svg = render_mermaid(source, &theme).expect("render failed");
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

    #[test]
    fn class_diagram_respects_non_default_font_size() {
        // Regression for the merman upgrade: layout honors themeVariables.fontSize,
        // so Zed must not force fallback text back to 16px in CSS.
        let source = r#"classDiagram
%%{init: {"themeVariables": {"fontSize": "8px"}}}%%
class User {
    +String WWWWWWWWWWWWWWWWWWWW
    +String WWWWWWWWWWWWWWWWWWWW
    +String WWWWWWWWWWWWWWWWWWWW
}
"#;
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        assert_eq!(text_font_size(tree.root(), "User"), Some(8.0));
        assert_eq!(
            text_font_size(tree.root(), "+String WWWWWWWWWWWWWWWWWWWW"),
            Some(8.0)
        );
    }

    #[test]
    fn mindmap_fallback_text_recovers_section_classes() {
        // merman parks section-* tokens on data-merman-source-classes for
        // foreignObject fallback labels. Promote them back onto class so the
        // injected section text-color CSS can win over gitBranchLabel fills.
        let source = r##"mindmap
%%{init: {"themeVariables": {"gitBranchLabel0": "#101010"}}}%%
    root[First<br/>Second]
"##;
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");

        for label in ["First", "Second"] {
            let needle_start = svg.find(&format!(">{label}</text>")).expect(label);
            let tag_start = svg[..needle_start].rfind("<text").expect("text tag");
            let tag = &svg[tag_start..needle_start];
            assert!(
                tag.contains("section-root") && tag.contains("section--1"),
                "{label} text should carry recovered section classes, got: {tag}"
            );
            assert!(
                tag.contains("class=\"")
                    && tag.contains("section-root")
                    && tag.contains("section--1"),
                "{label} section classes should be live class tokens, got: {tag}"
            );
        }

        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");
        let expected = {
            let rgba = gpui::Rgba::from(theme.text_color);
            // usvg exposes fills; compare via re-rendered fill on the text span.
            (rgba.r, rgba.g, rgba.b)
        };
        for label in ["First", "Second"] {
            let fill = text_fill_rgb(tree.root(), label).expect(label);
            assert!(
                (fill.0 - expected.0).abs() < 0.02
                    && (fill.1 - expected.1).abs() < 0.02
                    && (fill.2 - expected.2).abs() < 0.02,
                "{label} fill should follow theme text color, got {fill:?} expected {expected:?}"
            );
        }
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

    fn text_fill_rgb(group: &usvg::Group, expected_text: &str) -> Option<(f32, f32, f32)> {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => {
                    if let Some(fill) = text_fill_rgb(group, expected_text) {
                        return Some(fill);
                    }
                }
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        if chunk.text() == expected_text {
                            let span = chunk.spans().first()?;
                            return match span.fill() {
                                Some(paint) => match paint.paint() {
                                    usvg::Paint::Color(c) => Some((
                                        c.red as f32 / 255.0,
                                        c.green as f32 / 255.0,
                                        c.blue as f32 / 255.0,
                                    )),
                                    _ => None,
                                },
                                None => None,
                            };
                        }
                    }
                }
                usvg::Node::Path(_) | usvg::Node::Image(_) => {}
            }
        }

        None
    }
}
