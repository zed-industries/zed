use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context as _, Result, anyhow};
use merman::svg::export::{NativeFontContext, NativeFontLimits};
use merman::{
    Engine, OperationControl, RenderOutput, RenderRequest, Renderer, SvgEnvironment, SvgRequest,
};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(source: &str, theme: &MermaidTheme) -> Result<String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let source = crate::native_typography::prepare_source(source);
    let environment = SvgEnvironment::deterministic()
        .with_native_font_context(shared_native_font_context()?.clone())
        .context("configure native Mermaid text measurement")?;
    render_with_environment(source.as_ref(), theme, &diagram_id, environment)
}

fn render_with_environment(
    source: &str,
    theme: &MermaidTheme,
    diagram_id: &str,
    environment: SvgEnvironment,
) -> Result<String> {
    let pipeline = merman::svg::SvgPipeline::resvg_safe()
        .with_postprocessor(merman::svg::CssOverridePostprocessor::strip_existing_important());
    // Apply merman's raster-safe pipeline before Zed-specific styling. The
    // pipeline handles generic rasterizer compatibility cleanup: foreignObject
    // fallback text, unsupported CSS removal, and invalid SVG attribute cleanup.
    // Zed also strips merman's existing `!important` declarations before
    // injecting its own theme CSS so host styling wins consistently in usvg/resvg.
    let request = SvgRequest {
        environment,
        options: merman::svg::SvgRenderOptions {
            diagram_id: Some(diagram_id.to_owned()),
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

fn shared_native_font_context() -> Result<&'static NativeFontContext> {
    static CONTEXT: OnceLock<NativeFontContext> = OnceLock::new();
    if let Some(context) = CONTEXT.get() {
        return Ok(context);
    }

    // Same faces and order as `gpui::BUNDLED_SVG_FONT_PATHS`, so a requested
    // family such as Lilex is measured with the face usvg paints.
    const REGULAR: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
    const ITALIC: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf");
    const SEMIBOLD: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf");
    const SEMIBOLD_ITALIC: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf");
    const LILEX: &[u8] = include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf");

    let context = NativeFontContext::from_font_bytes(
        [
            REGULAR.to_vec(),
            ITALIC.to_vec(),
            SEMIBOLD.to_vec(),
            SEMIBOLD_ITALIC.to_vec(),
            LILEX.to_vec(),
        ],
        NativeFontLimits::default(),
        &OperationControl::new(),
    )
    .context("load bundled fonts for Mermaid label measurement")?;

    Ok(CONTEXT.get_or_init(|| context))
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
        // Zed's injected CSS targets classic output.
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
            let element = quick_xml::events::BytesStart::from_content(
                tag.trim_start_matches('<'),
                "text".len(),
            );
            let class = element.try_get_attribute("class").unwrap().unwrap();
            let classes = std::str::from_utf8(&class.value).unwrap();
            assert!(
                ["section-root", "section--1"]
                    .iter()
                    .all(|expected| classes.split_whitespace().any(|class| class == *expected)),
                "{label} missing section classes: {classes}"
            );
        }

        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");
        let expected = {
            let rgba = gpui::Rgba::from(theme.text_color);
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

    #[test]
    fn class_method_label_fits_fallback_viewport() {
        // Wide method labels must fit their fallback viewports.
        let source = r#"classDiagram
    class User {
        +MMMMMMMMMMMMMMMM()
    }
"#;
        let theme = ibm_plex_theme();
        let svg = crate::render_to_svg(source, &theme).expect("render failed");
        assert_label_fits_fallback_viewport(&svg, "+MMMMMMMMMMMMMMMM()");
    }

    #[test]
    fn sequence_participant_label_fits_box() {
        // Participant boxes must contain the rendered label.
        let source = "sequenceDiagram\n    participant MMMMMMMMMMMMMMMM";
        let theme = ibm_plex_theme();
        let svg = crate::render_to_svg(source, &theme).expect("render failed");
        let tree = parse_svg_with_ibm_plex(&svg);
        let glyph_width =
            text_glyph_width(tree.root(), "MMMMMMMMMMMMMMMM").expect("missing participant glyphs");
        let box_width =
            actor_box_width_for_label(&svg, "MMMMMMMMMMMMMMMM").expect("missing actor box");
        assert!(
            glyph_width <= box_width + 1.0,
            "participant glyphs {glyph_width} overflow actor box {box_width}"
        );
    }

    #[test]
    fn state_description_fits_fallback_viewport() {
        // Description text must fit without being clipped by its fallback viewport.
        let source = r#"stateDiagram-v2
    [*] --> Teardown
    Teardown : members kicked
    Teardown : group deleted
"#;
        let theme = ibm_plex_theme();
        let svg = crate::render_to_svg(source, &theme).expect("render failed");
        assert_label_fits_fallback_viewport(&svg, "group deleted");
        assert_label_fits_fallback_viewport(&svg, "members kicked");
    }

    #[test]
    fn class_diagram_custom_font_size_keeps_visible_labels() {
        // Layout and rendering must use the same configured font size.
        let source = r#"classDiagram
%%{init: {"themeVariables": {"fontSize": "8px"}}}%%
class User {
    +String WWWWWWWWWWWWWWWWWWWW
    +String WWWWWWWWWWWWWWWWWWWW
    +String WWWWWWWWWWWWWWWWWWWW
}
"#;
        let theme = ibm_plex_theme();
        let svg = crate::render_to_svg(source, &theme).expect("render failed");
        let tree = parse_svg_with_ibm_plex(&svg);
        assert_eq!(text_font_size(tree.root(), "User"), Some(8.0));
        assert_eq!(
            text_font_size(tree.root(), "+String WWWWWWWWWWWWWWWWWWWW"),
            Some(8.0)
        );
        assert_label_fits_fallback_viewport(&svg, "User");
        assert_label_fits_fallback_viewport(&svg, "+String WWWWWWWWWWWWWWWWWWWW");
    }

    #[test]
    fn unsupported_text_styles_still_render_labels() {
        // These declarations are removed so measurement can stay on real font metrics.
        let theme = MermaidTheme::default();
        for source in [
            "flowchart LR\n    A[ABC]\n    style A letter-spacing:1px\n",
            "flowchart LR\n    A[ABC]\n    style A word-spacing:1px\n",
            "flowchart LR\n    A[ABC]\n    style A text-transform:uppercase\n",
        ] {
            let svg = render_mermaid(source, &theme)
                .unwrap_or_else(|error| panic!("expected render for {source}: {error:#}"));
            assert!(
                svg.contains(">ABC</text>"),
                "missing label text for {source}: {svg}"
            );
        }
    }

    #[test]
    fn class_diagram_non_px_font_size_matches_painted_glyphs() {
        let member = "+String WWWWWWWWWWWWWWWWWWWW";
        let theme = ibm_plex_theme();
        for (size, painted) in [("24", 16.0), (r#""24""#, 16.0), (r#""1.5em""#, 24.0)] {
            let source = format!(
                r#"classDiagram
%%{{init: {{"themeVariables": {{"fontSize": {size}}}}}}}%%
class User {{
    {member}
    {member}
}}
"#
            );
            let svg = crate::render_to_svg(&source, &theme)
                .unwrap_or_else(|error| panic!("expected render for fontSize {size}: {error:#}"));
            let tree = parse_svg_with_ibm_plex(&svg);
            assert_eq!(
                text_font_size(tree.root(), "User"),
                Some(painted),
                "fontSize {size}"
            );
            assert_label_matches_fallback_viewport(&svg, "User");
            let mut labels = Vec::new();
            collect_text_labels(tree.root(), &mut labels);
            let member_lines: Vec<_> = labels
                .into_iter()
                .filter(|label| label.contains('W'))
                .collect();
            assert!(
                !member_lines.is_empty(),
                "fontSize {size} dropped the member label"
            );
            for label in member_lines {
                assert_label_fits_fallback_viewport(&svg, &label);
                assert_text_inside_widest_path(&svg, &label);
            }
        }
    }

    #[test]
    fn flowchart_noop_letter_spacing_keeps_neighbor_label_inside_node() {
        let label = "W".repeat(160);
        let source =
            format!("flowchart LR\n    A[{label}] --> B[OK]\n    style B letter-spacing:0px\n");
        let svg = crate::render_to_svg(&source, &ibm_plex_theme()).expect("render failed");
        assert_label_matches_fallback_viewport(&svg, &label);
        assert_text_inside_widest_path(&svg, &label);
    }

    #[test]
    fn class_diagram_lilex_title_uses_lilex_metrics() {
        let source = r#"classDiagram
class iiiiiiiiii {
    +WWWWWWWW()
}
"#;
        let mut lilex = ibm_plex_theme();
        lilex.font_family = "Lilex, sans-serif".to_string();
        let lilex_svg = crate::render_to_svg(source, &lilex).expect("render failed");
        let ibm_svg = crate::render_to_svg(source, &ibm_plex_theme()).expect("render failed");
        assert_label_matches_fallback_viewport(&lilex_svg, "iiiiiiiiii");
        let lilex_width = fallback_viewport_width_for_label(&lilex_svg, "iiiiiiiiii")
            .expect("lilex title viewport");
        let ibm_width =
            fallback_viewport_width_for_label(&ibm_svg, "iiiiiiiiii").expect("ibm title viewport");
        assert!(
            lilex_width > ibm_width + 20.0,
            "Lilex title viewport {lilex_width} should exceed IBM Plex {ibm_width}"
        );
    }

    #[test]
    fn class_diagram_wrapped_member_clears_title() {
        let source = r#"classDiagram
%%{init: {"themeVariables": {"fontSize": "64px"}}}%%
class User {
    +String identifier
    +signIn()
}
"#;
        let svg = crate::render_to_svg(source, &ibm_plex_theme()).expect("render failed");
        let tree = parse_svg_with_ibm_plex(&svg);
        assert_eq!(text_font_size(tree.root(), "User"), Some(64.0));
        let title = text_abs_bounds(tree.root(), "User").expect("title bounds");
        let member = text_abs_bounds_trimmed(tree.root(), "+String").expect("member bounds");
        let identifier = text_abs_bounds(tree.root(), "identifier").expect("identifier bounds");
        let method = text_abs_bounds(tree.root(), "+signIn()").expect("method bounds");
        let title_bottom = f64::from(title.y() + title.height());
        let member_top = f64::from(member.y());
        assert!(
            member_top + 1.0 >= title_bottom,
            "wrapped member overlaps title: title bottom {title_bottom} member top {member_top}"
        );
        let identifier_bottom = f64::from(identifier.y() + identifier.height());
        let method_top = f64::from(method.y());
        assert!(
            method_top + 1.0 >= identifier_bottom,
            "wrapped member overlaps the next row: member bottom {identifier_bottom} method top {method_top}"
        );
        let line_gap = f64::from(identifier.y() - member.y());
        assert!(
            (line_gap - 96.0).abs() < 2.0,
            "wrapped lines should stay a 96px line apart, gap {line_gap}"
        );
    }

    #[test]
    fn italic_flowchart_label_fits_with_bundled_faces() {
        let label = "W".repeat(160);
        let source = format!("flowchart LR\n    A[{label}]\n    style A font-style:italic\n");
        let theme = ibm_plex_theme();
        let svg = crate::render_to_svg(&source, &theme).expect("render failed");
        assert_label_fits_fallback_viewport(&svg, &label);
    }

    #[test]
    fn bundled_svg_fonts_resolve_ibm_plex_italic() {
        let mut db = usvg::fontdb::Database::new();
        for path in gpui::BUNDLED_SVG_FONT_PATHS {
            db.load_font_data(font_bytes_for_bundled_path(path).to_vec());
        }
        let id = db
            .query(&usvg::fontdb::Query {
                families: &[usvg::fontdb::Family::Name("IBM Plex Sans")],
                style: usvg::fontdb::Style::Italic,
                ..Default::default()
            })
            .expect("BUNDLED_SVG_FONT_PATHS must resolve IBM Plex Sans Italic");
        let face = db.face(id).expect("italic face");
        assert_eq!(face.style, usvg::fontdb::Style::Italic);
        assert_eq!(face.weight, usvg::fontdb::Weight(400));
    }

    #[test]
    fn mindmap_rectangular_child_uses_theme_text_color() {
        // Single-line and multiline labels must use the theme text color.
        let theme = dark_theme();
        for (source, labels) in [
            (
                r#"mindmap
    root((Root))
        First
        second[AB]
"#,
                &["AB"][..],
            ),
            (
                r#"mindmap
    root((Root))
        First
        second[A<br/>B]
"#,
                &["A", "B"][..],
            ),
        ] {
            let svg = crate::render_to_svg(source, &theme).expect("render failed");
            let tree = parse_svg_with_ibm_plex(&svg);
            let expected = {
                let rgba = gpui::Rgba::from(theme.text_color);
                (rgba.r, rgba.g, rgba.b)
            };
            for label in labels {
                let fill = text_fill_rgb(tree.root(), label)
                    .unwrap_or_else(|| panic!("missing label {label} in {source}"));
                assert!(
                    (fill.0 - expected.0).abs() < 1.0 / 255.0
                        && (fill.1 - expected.1).abs() < 1.0 / 255.0
                        && (fill.2 - expected.2).abs() < 1.0 / 255.0,
                    "{label} fill {fill:?} should be theme text {expected:?}"
                );
            }
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

    fn ibm_plex_theme() -> MermaidTheme {
        let mut theme = MermaidTheme::default();
        theme.font_family = "IBM Plex Sans, sans-serif".to_string();
        theme
    }

    fn dark_theme() -> MermaidTheme {
        use gpui::rgb;
        let mut theme = MermaidTheme::default();
        theme.dark_mode = true;
        theme.font_family = "IBM Plex Sans, sans-serif".to_string();
        theme.background = rgb(0x282c33).into();
        theme.primary_color = rgb(0x2f343e).into();
        theme.primary_text_color = rgb(0xdce0e5).into();
        theme.text_color = rgb(0xdce0e5).into();
        theme.secondary_color = rgb(0x2e343e).into();
        theme.tertiary_color = rgb(0x363c46).into();
        theme.line_color = rgb(0x464b57).into();
        theme.primary_border_color = rgb(0x464b57).into();
        theme.git_branch_label_colors = theme
            .git_branch_colors
            .map(crate::text_color_for_background);
        theme
    }

    fn font_bytes_for_bundled_path(path: &str) -> &'static [u8] {
        match path {
            "fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf" => {
                include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf")
            }
            "fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf" => {
                include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf")
            }
            "fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf" => {
                include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf")
            }
            "fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf" => {
                include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf")
            }
            "fonts/lilex/Lilex-Regular.ttf" => {
                include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf")
            }
            other => panic!("update font_bytes_for_bundled_path for {other}"),
        }
    }

    fn parse_svg_with_ibm_plex(svg: &str) -> usvg::Tree {
        // Use the same face list the SVG renderer bundles.
        let mut db = usvg::fontdb::Database::new();
        for path in gpui::BUNDLED_SVG_FONT_PATHS {
            db.load_font_data(font_bytes_for_bundled_path(path).to_vec());
        }
        db.set_sans_serif_family("IBM Plex Sans");
        let options = usvg::Options {
            fontdb: std::sync::Arc::new(db),
            ..Default::default()
        };
        usvg::Tree::from_str(svg, &options).expect("SVG parsing failed")
    }

    fn assert_label_fits_fallback_viewport(svg: &str, label: &str) {
        let (glyph_width, viewport_width) = label_and_viewport_widths(svg, label);
        assert!(
            glyph_width <= viewport_width + 1.0,
            "{label}: glyph width {glyph_width} overflows fallback viewport {viewport_width}"
        );
    }

    fn assert_label_matches_fallback_viewport(svg: &str, label: &str) {
        let (glyph_width, viewport_width) = label_and_viewport_widths(svg, label);
        assert!(
            (glyph_width - viewport_width).abs() <= 1.0,
            "{label}: glyph width {glyph_width} does not match fallback viewport {viewport_width}"
        );
    }

    fn label_and_viewport_widths(svg: &str, label: &str) -> (f64, f64) {
        let tree = parse_svg_with_ibm_plex(svg);
        let glyph_width = text_glyph_width(tree.root(), label)
            .unwrap_or_else(|| panic!("missing glyph metrics for {label}"));
        let viewport_width = fallback_viewport_width_for_label(svg, label)
            .unwrap_or_else(|| panic!("missing fallback viewport for {label}"));
        (glyph_width, viewport_width)
    }

    fn assert_text_inside_widest_path(svg: &str, label: &str) {
        let tree = parse_svg_with_ibm_plex(svg);
        let text_box = text_abs_bounds(tree.root(), label)
            .unwrap_or_else(|| panic!("missing bounds for {label}"));
        let path_box =
            widest_path_overlapping_y(tree.root(), text_box.y() + text_box.height() / 2.0)
                .unwrap_or_else(|| panic!("missing path around {label}"));
        let text_left = f64::from(text_box.x());
        let text_right = f64::from(text_box.x() + text_box.width());
        let path_left = f64::from(path_box.x());
        let path_right = f64::from(path_box.x() + path_box.width());
        assert!(
            text_left >= path_left - 0.5 && text_right <= path_right + 0.5,
            "{label} glyphs {text_left}..{text_right} outside node {path_left}..{path_right}"
        );
    }

    fn actor_box_width_for_label(svg: &str, label: &str) -> Option<f64> {
        let marker = format!("name=\"{label}\"");
        let mut search = svg;
        while let Some(idx) = search.find(&marker) {
            let abs = svg.len() - search.len() + idx;
            if let Some(rect_start) = svg[..abs].rfind("<rect") {
                let tag_end = svg[rect_start..].find('>')? + rect_start;
                let tag = &svg[rect_start..tag_end];
                if tag.contains("class=\"actor") {
                    let width_key = "width=\"";
                    let w0 = tag.find(width_key)? + width_key.len();
                    let w1 = tag[w0..].find('"')? + w0;
                    return tag[w0..w1].parse().ok();
                }
            }
            search = &svg[abs + marker.len()..];
        }
        None
    }

    fn fallback_viewport_width_for_label(svg: &str, label: &str) -> Option<f64> {
        let needle = format!(">{label}</text>");
        let end = svg.find(&needle)?;
        let text_start = svg[..end].rfind("<text")?;
        // Read the enclosing fallback viewport in the label definition.
        let def_svg_start = svg[..text_start].rfind("<svg ")?;
        let def_svg_tag_end = svg[def_svg_start..].find('>')? + def_svg_start;
        let tag = &svg[def_svg_start..def_svg_tag_end];
        let width_key = "width=\"";
        let w0 = tag.find(width_key)? + width_key.len();
        let w1 = tag[w0..].find('"')? + w0;
        tag[w0..w1].parse().ok()
    }

    fn collect_text_labels(group: &usvg::Group, labels: &mut Vec<String>) {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => collect_text_labels(group, labels),
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        labels.push(chunk.text().to_string());
                    }
                }
                usvg::Node::Path(_) | usvg::Node::Image(_) => {}
            }
        }
    }

    fn text_abs_bounds(group: &usvg::Group, expected_text: &str) -> Option<usvg::Rect> {
        text_abs_bounds_matching(group, &|text| text == expected_text)
    }

    fn text_abs_bounds_trimmed(group: &usvg::Group, expected_text: &str) -> Option<usvg::Rect> {
        text_abs_bounds_matching(group, &|text| text.trim() == expected_text)
    }

    fn text_abs_bounds_matching<F>(group: &usvg::Group, predicate: &F) -> Option<usvg::Rect>
    where
        F: Fn(&str) -> bool,
    {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => {
                    if let Some(bounds) = text_abs_bounds_matching(group, predicate) {
                        return Some(bounds);
                    }
                }
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        if predicate(chunk.text()) {
                            return Some(text.abs_bounding_box());
                        }
                    }
                }
                usvg::Node::Path(_) | usvg::Node::Image(_) => {}
            }
        }
        None
    }

    fn widest_path_overlapping_y(group: &usvg::Group, y: f32) -> Option<usvg::Rect> {
        fn walk(group: &usvg::Group, y: f32, best: &mut Option<usvg::Rect>) {
            for node in group.children() {
                match node {
                    usvg::Node::Group(group) => walk(group, y, best),
                    usvg::Node::Path(path) => {
                        let bounds = path.abs_bounding_box();
                        let overlaps = y >= bounds.y() && y <= bounds.y() + bounds.height();
                        if overlaps
                            && best
                                .as_ref()
                                .is_none_or(|current| bounds.width() > current.width())
                        {
                            *best = Some(bounds);
                        }
                    }
                    usvg::Node::Text(_) | usvg::Node::Image(_) => {}
                }
            }
        }
        let mut best = None;
        walk(group, y, &mut best);
        best
    }

    fn text_glyph_width(group: &usvg::Group, expected_text: &str) -> Option<f64> {
        for node in group.children() {
            match node {
                usvg::Node::Group(group) => {
                    if let Some(width) = text_glyph_width(group, expected_text) {
                        return Some(width);
                    }
                }
                usvg::Node::Text(text) => {
                    for chunk in text.chunks() {
                        if chunk.text() == expected_text {
                            let bbox = text.bounding_box();
                            return Some(f64::from(bbox.width()));
                        }
                    }
                }
                usvg::Node::Path(_) | usvg::Node::Image(_) => {}
            }
        }
        None
    }
}
