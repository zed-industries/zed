use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock, PoisonError,
        atomic::{AtomicU64, Ordering},
    },
};

use anyhow::{Context as _, Result, anyhow};
use gpui::{SvgRenderer, SvgTextFont};
use merman::svg::{
    DeterministicTextMeasurer, MeasurementProfileId, TextMeasurementPolicy, TextMeasurementProfile,
    TextMeasurementProfileIdentity, TextStyle,
};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(
    source: &str,
    theme: &MermaidTheme,
    svg_renderer: &SvgRenderer,
) -> Result<String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let edge_label_background = css_color(theme.edge_label_background);
    // Apply merman's raster-safe pipeline before Zed-specific styling. The
    // pipeline handles generic rasterizer compatibility cleanup: foreignObject
    // fallback text, unsupported CSS removal, and invalid SVG attribute cleanup.
    // Zed also strips merman's existing `!important` declarations before
    // injecting its own theme CSS so host styling wins consistently in usvg/resvg.
    let pipeline = merman::svg::SvgPipeline::resvg_safe()
        .with_postprocessor(
            merman::svg::ScopedCssPostprocessor::new(format!(
                "span.edgeLabel.edgeLabel {{ background: {edge_label_background}; }}"
            ))
            .with_existing_style_merge(),
        )
        .with_postprocessor(merman::svg::CssOverridePostprocessor::strip_existing_important());
    let control = merman::OperationControl::new();
    let label_measurer = Arc::new(LabelMeasurer {
        svg_renderer: svg_renderer.clone(),
        font_family: theme.font_family.clone(),
        measurements: Mutex::new(Measurements {
            widths: HashMap::new(),
            remaining_budget: MEASUREMENT_BUDGET,
        }),
        error: OnceLock::new(),
        control: control.clone(),
    });
    let text_measurer = DeterministicTextMeasurer::default().with_width_callback({
        let label_measurer = label_measurer.clone();
        move |text, style| label_measurer.width(text, style)
    });
    let measurement_profile = TextMeasurementProfile::new(
        TextMeasurementProfileIdentity::new(MeasurementProfileId::new("zed.svg-renderer")?, "1")?,
        text_measurer,
    );
    let request = merman::SvgRequest {
        environment: merman::SvgEnvironment::deterministic()
            .with_text_measurement_policy(TextMeasurementPolicy::uniform(measurement_profile)),
        options: merman::svg::SvgRenderOptions {
            diagram_id: Some(diagram_id),
            ..merman::svg::SvgRenderOptions::default()
        },
        pipeline: Some(pipeline),
        ..merman::SvgRequest::default()
    };

    let output = merman::Renderer::new()
        .with_engine(merman::Engine::new().with_site_config(to_merman_config(theme)?))
        .render(merman::RenderRequest::svg(source, control, request));
    if let Some(error) = label_measurer.error.get() {
        return Err(anyhow!("failed to measure mermaid text: {error}"));
    }
    let merman::RenderOutput::Svg(Some(svg)) = output.context("merman render failed")? else {
        return Err(anyhow!("merman returned no SVG for the given input"));
    };
    let (svg, _) = svg.into_parts();

    Ok(svg)
}

const MEASUREMENT_BUDGET: usize = 2 << 20;

struct LabelMeasurer {
    svg_renderer: SvgRenderer,
    font_family: String,
    measurements: Mutex<Measurements>,
    error: OnceLock<String>,
    control: merman::OperationControl,
}

struct Measurements {
    widths: HashMap<(String, u64, Option<String>, Option<String>), f64>,
    remaining_budget: usize,
}

impl LabelMeasurer {
    fn width(&self, text: &str, style: &TextStyle) -> f64 {
        if self.error.get().is_some() {
            return 0.0;
        }
        let key = (
            text.to_owned(),
            style.font_size.to_bits(),
            style.font_weight.clone(),
            style.font_style.clone(),
        );
        let mut measurements = self
            .measurements
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(width) = measurements.widths.get(&key) {
            return *width;
        }
        let Some(remaining_budget) = measurements
            .remaining_budget
            .checked_sub(measurement_cost(text))
        else {
            return self.fail("the diagram has too much text".to_string());
        };
        measurements.remaining_budget = remaining_budget;
        let font = SvgTextFont {
            family: &self.font_family,
            size: style.font_size as f32,
            weight: style.font_weight.as_deref().unwrap_or("normal"),
            style: style.font_style.as_deref().unwrap_or("normal"),
        };
        let width = match self.svg_renderer.measure_text_width(text, &font) {
            Ok(width) => f64::from(width),
            Err(error) => return self.fail(error.to_string()),
        };
        measurements.widths.insert(key, width);
        width
    }

    fn fail(&self, error: String) -> f64 {
        self.error.get_or_init(|| error);
        self.control.cancel();
        0.0
    }
}

fn measurement_cost(text: &str) -> usize {
    let len = text.len();
    64 + len + len.saturating_mul(len) / 4096
}

fn to_merman_config(theme: &MermaidTheme) -> Result<merman::MermaidConfig> {
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

    let mut secure_keys = merman::generated::default_site_config()
        .as_value()
        .get("secure")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .context("merman has no default secure keys")?;
    secure_keys.extend(["themeVariables", "fontSize"].map(serde_json::Value::from));

    Ok(merman::MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        "look": "classic",
        "secure": secure_keys,
        "darkMode": theme.dark_mode,
        "fontFamily": theme.font_family,
        // resvg can't rasterize HTML `<foreignObject>` labels, so merman's
        // raster-safe pipeline replaces them with wrapped native SVG text.
        "htmlLabels": true,
        "flowchart": {
            "htmlLabels": true,
            "padding": 16,
            "wrappingWidth": 200,
            "minNodeWidth": 0,
        },
        "state": {
            "wrappingWidth": 200,
            "minNodeWidth": 0,
        },
        "themeVariables": theme_vars,
    })))
}

#[cfg(test)]
mod tests {
    use assets::Assets;
    use quick_xml::{Reader, events::Event};

    use super::*;

    #[test]
    fn render_stage_applies_resvg_safe_pipeline() {
        let html_label_source =
            "classDiagram\n    class Shelter {\n        -List~Animal~ animals\n    }";
        let html_label_svg =
            render_mermaid(html_label_source, &MermaidTheme::default(), &svg_renderer())
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
        let css_svg = render_mermaid(css_source, &MermaidTheme::default(), &svg_renderer())
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
        let svg = render_mermaid(source, &MermaidTheme::default(), &svg_renderer())
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
    fn class_diagram_labels_keep_sixteen_pixel_font_size() {
        let source = r#"classDiagram
            class User {
                +String id
                +String name
                +signIn()
            }
        "#;
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
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
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
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
    fn html_labels_fit_their_viewports() {
        let wide_members = "class User {\n    +String WWWWWWWWWWWWWWWWWWWW\n}";
        let diagrams = [
            (
                "IBM Plex Sans",
                "classDiagram\nclass A {\n    +MMMMMMMMMMMMMMMM()\n}".to_string(),
            ),
            (
                "Lilex",
                "classDiagram\nclass iiiiiiiiii {\n    +WWWWWWWW()\n}".to_string(),
            ),
            (
                "IBM Plex Sans",
                format!(
                    "classDiagram\n%%{{init: {{\"themeVariables\": {{\"fontSize\": \"1.5em\"}}}}}}%%\n{wide_members}"
                ),
            ),
            (
                "IBM Plex Sans",
                format!(
                    "classDiagram\n%%{{init: {{\"themeVariables\": {{\"fontSize\": \"64px\"}}}}}}%%\n{wide_members}"
                ),
            ),
            (
                "IBM Plex Sans",
                "stateDiagram-v2\n    [*] --> Teardown\n    Teardown : members kicked\n    Teardown : group deleted"
                    .to_string(),
            ),
            (
                "IBM Plex Sans",
                "erDiagram\n    ORDER {\n        string shipping_address_line_one\n    }".to_string(),
            ),
            (
                "IBM Plex Sans",
                "mindmap\n    root((Root))\n        First\n        second[A<br/>B]".to_string(),
            ),
            (
                "IBM Plex Sans",
                "classDiagram\nclass 顧客 {\n    +文字列 名前😀\n}".to_string(),
            ),
        ];
        let svg_renderer = svg_renderer();

        for (font_family, source) in diagrams {
            let theme = MermaidTheme {
                font_family: font_family.to_string(),
                ..MermaidTheme::default()
            };
            let svg = render_mermaid(&source, &theme, &svg_renderer).expect("render failed");
            let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
            let labels = viewport_labels(&svg);
            assert!(!labels.is_empty(), "no label viewports in: {svg}");

            for label in labels {
                let font = SvgTextFont {
                    family: font_family,
                    size: label.font_size,
                    weight: &label.font_weight,
                    style: &label.font_style,
                };
                let text_width = svg_renderer
                    .measure_text_width(&label.text, &font)
                    .expect("measuring failed");
                assert!(
                    text_width <= label.viewport_width + 0.01,
                    "{:?} is {text_width}px wide in a {}px viewport of: {source}",
                    label.text,
                    label.viewport_width,
                );
            }
        }
    }

    #[test]
    fn state_descriptions_do_not_overlap() {
        let source = "stateDiagram-v2\n    [*] --> Teardown\n    Teardown : members kicked\n    Teardown : group deleted";
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");
        let bounds = |expected_text| text_node(tree.root(), expected_text).abs_bounding_box();

        let first = bounds("members kicked");
        let second = bounds("group deleted");
        assert!(
            first.bottom() <= second.top(),
            "descriptions overlap: {first:?} and {second:?}"
        );
    }

    #[test]
    fn mindmap_labels_use_theme_text_color() {
        let source = "mindmap\n    root((Root))\n        First\n        second[A<br/>B]";
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        let fills = text_nodes(tree.root())
            .into_iter()
            .flat_map(|text| text.chunks())
            .flat_map(|chunk| chunk.spans())
            .map(|span| match span.fill().map(|fill| fill.paint()) {
                Some(usvg::Paint::Color(color)) => {
                    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
                }
                _ => String::new(),
            })
            .collect::<Vec<_>>();
        assert_eq!(fills, vec![css_color(theme.text_color); 4]);
    }

    #[test]
    fn inherited_styles_do_not_change_label_typography() {
        let theme = MermaidTheme::default();
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let render = |source| {
            let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
            let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
            usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed")
        };

        let class_diagram = render("classDiagram\n    class User");
        let title = text_node(class_diagram.root(), "User");
        assert_eq!(title.chunks()[0].spans()[0].font().weight(), 700);

        let flowchart =
            render("flowchart LR\n    A[Label]\n    style A letter-spacing:3px,word-spacing:6px");
        let label = &text_node(flowchart.root(), "Label").chunks()[0].spans()[0];
        assert_eq!((label.letter_spacing(), label.word_spacing()), (0.0, 0.0));
    }

    #[test]
    fn diagram_directives_cannot_restyle_diagrams() {
        let sources = [
            "%%{init: {\"themeCSS\": \".node rect { fill: #abcdef; }\"}}%%\nflowchart LR\n    A --> B",
            "flowchart LR\n%%{init: {\"themeVariables\": {\"primaryColor\": \"#abcdef\", \"lineColor\": \"#abcdef\"}}}%%\n    A --> B",
        ];

        for source in sources {
            let svg = render_mermaid(source, &MermaidTheme::default(), &svg_renderer())
                .expect("render failed");
            assert_eq!(svg.matches("#abcdef").count(), 0, "{source}");
        }
    }

    #[test]
    fn edge_label_backgrounds_use_theme_color() {
        let source = "classDiagram\n    Animal <|-- Dog : inherits";
        let theme = MermaidTheme {
            edge_label_background: gpui::Hsla::from(gpui::rgb(0x123456)),
            ..MermaidTheme::default()
        };
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");

        assert_eq!(svg.matches(r##"fill="#123456""##).count(), 1);
    }

    #[test]
    fn label_viewports_do_not_clip_text() {
        let source = "classDiagram\n    class A {\n        +e\u{301}\u{302}\u{303}\u{304}\u{306}\u{307}\u{308}\u{30a}\n    }";
        let theme = MermaidTheme::default();
        let svg = render_mermaid(source, &theme, &svg_renderer()).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme).expect("postprocess failed");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("SVG parsing failed");

        assert_eq!(text_nodes(tree.root()).len(), 2);
        assert_eq!(clipped_group_count(tree.root()), 0);
    }

    #[test]
    fn diagrams_with_too_much_text_to_measure_fail_to_render() {
        let source = format!(
            "flowchart TD\nA[\"{}{}\"]",
            "\u{200b}\u{200c}".repeat(20_000),
            "M ".repeat(100)
        );
        let error = render_mermaid(&source, &MermaidTheme::default(), &svg_renderer())
            .expect_err("render succeeded");

        assert_eq!(
            error.to_string(),
            "failed to measure mermaid text: the diagram has too much text"
        );
    }

    #[test]
    fn measuring_past_the_budget_cancels_the_render() {
        let control = merman::OperationControl::new();
        let label_measurer = LabelMeasurer {
            svg_renderer: svg_renderer(),
            font_family: "IBM Plex Sans".to_string(),
            measurements: Mutex::new(Measurements {
                widths: HashMap::new(),
                remaining_budget: 0,
            }),
            error: OnceLock::new(),
            control: control.clone(),
        };

        assert_eq!(label_measurer.width("label", &TextStyle::default()), 0.0);
        assert!(control.is_cancelled());
    }

    struct ViewportLabel {
        viewport_width: f32,
        text: String,
        font_size: f32,
        font_weight: String,
        font_style: String,
    }

    fn svg_renderer() -> SvgRenderer {
        SvgRenderer::new(Arc::new(Assets))
    }

    fn viewport_labels(svg: &str) -> Vec<ViewportLabel> {
        let mut labels = Vec::new();
        let mut viewport_widths = Vec::<f32>::new();
        let mut label_style = None;
        let mut reader = Reader::from_str(svg);
        loop {
            match reader.read_event().expect("SVG is not well-formed") {
                Event::Eof => break,
                Event::Start(element) if element.name().as_ref() == b"svg" => {
                    let width = attribute(&element, "width");
                    viewport_widths.push(width.parse().unwrap_or(f32::INFINITY));
                }
                Event::End(element) if element.name().as_ref() == b"svg" => {
                    viewport_widths.pop();
                }
                Event::Start(element)
                    if element.name().as_ref() == b"text" && viewport_widths.len() > 1 =>
                {
                    label_style = Some(attribute(&element, "style"));
                }
                Event::Text(text) => {
                    if let Some(style) = label_style.take() {
                        let declaration = |name: &str| {
                            style
                                .split(';')
                                .filter_map(|declaration| declaration.split_once(':'))
                                .find(|(property, _)| property.trim() == name)
                                .map(|(_, value)| value.trim().to_string())
                                .expect("label style is missing a declaration")
                        };
                        labels.push(ViewportLabel {
                            viewport_width: viewport_widths
                                .last()
                                .copied()
                                .unwrap_or(f32::INFINITY),
                            text: text.decode().expect("label is not UTF-8").into_owned(),
                            font_size: declaration("font-size")
                                .trim_end_matches("px")
                                .parse()
                                .expect("label font size is not in pixels"),
                            font_weight: declaration("font-weight"),
                            font_style: declaration("font-style"),
                        });
                    }
                }
                _ => {}
            }
        }
        labels
    }

    fn attribute(element: &quick_xml::events::BytesStart<'_>, name: &str) -> String {
        element
            .try_get_attribute(name)
            .expect("attribute is not well-formed")
            .map(|attribute| String::from_utf8_lossy(&attribute.value).into_owned())
            .unwrap_or_default()
    }

    fn clipped_group_count(group: &usvg::Group) -> usize {
        group
            .children()
            .iter()
            .map(|node| match node {
                usvg::Node::Group(group) => {
                    usize::from(group.clip_path().is_some()) + clipped_group_count(group)
                }
                usvg::Node::Text(_) | usvg::Node::Path(_) | usvg::Node::Image(_) => 0,
            })
            .sum()
    }

    fn text_node<'a>(group: &'a usvg::Group, expected_text: &str) -> &'a usvg::Text {
        text_nodes(group)
            .into_iter()
            .find(|text| {
                text.chunks()
                    .iter()
                    .any(|chunk| chunk.text() == expected_text)
            })
            .expect("text is missing")
    }

    fn text_nodes(group: &usvg::Group) -> Vec<&usvg::Text> {
        group
            .children()
            .iter()
            .flat_map(|node| match node {
                usvg::Node::Group(group) => text_nodes(group),
                usvg::Node::Text(text) => vec![&**text],
                usvg::Node::Path(_) | usvg::Node::Image(_) => Vec::new(),
            })
            .collect()
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
