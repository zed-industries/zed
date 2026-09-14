use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context as _, Result, anyhow};

use crate::{MermaidTheme, css_color};

pub(super) fn render_mermaid(source: &str, theme: &MermaidTheme) -> Result<(String, bool)> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let (config, custom_palette) = config_for_source(source, theme)?;
    let engine = merman::Engine::new().with_site_config(config);
    let renderer = merman::Renderer::new().with_engine(engine);
    let semantic = renderer
        .prepare_semantic(source, merman::OperationControl::new())
        .context("merman parse failed")?
        .context("merman returned no diagram for the given input")?;
    let background = semantic
        .metadata()
        .effective_config
        .get_str("themeVariables.background")
        .map(str::to_owned);
    // Apply merman's raster-safe pipeline before Zed-specific styling. The
    // pipeline handles generic rasterizer compatibility cleanup: foreignObject
    // fallback text, unsupported CSS removal, and invalid SVG attribute cleanup.
    // Author styles must retain their priority over host defaults.
    let mut pipeline = merman::svg::SvgPipeline::resvg_safe();
    if let Some(background) = background {
        pipeline =
            pipeline.with_postprocessor(merman::svg::RootBackgroundPostprocessor::new(background));
    }

    let request = merman::SvgRequest {
        options: merman::svg::SvgRenderOptions {
            diagram_id: Some(diagram_id),
            ..Default::default()
        },
        pipeline: Some(pipeline),
        ..Default::default()
    };
    let output = semantic
        .render(merman::RenderTarget::Svg(request))
        .context("merman render failed")?;
    let merman::RenderOutput::Svg(Some(svg)) = output else {
        return Err(anyhow!("merman returned no SVG for the given input"));
    };

    Ok((svg.svg().to_owned(), custom_palette))
}

fn config_for_source(source: &str, theme: &MermaidTheme) -> Result<(merman::MermaidConfig, bool)> {
    let source = merman::preprocess_diagram(source, &merman::DetectorRegistry::default())?;
    let explicit_theme = source
        .config
        .get_str("theme")
        .is_some_and(|name| name != "base");
    let mut config = if explicit_theme {
        // A named Mermaid preset supplies its own palette instead of host defaults.
        merman::MermaidConfig::from_value(serde_json::json!({"htmlLabels": true}))
    } else {
        to_merman_config(theme)
    };
    // Treemap reuses scale label colors on translucent sections and leaves.
    if !explicit_theme && source.code().trim_start().starts_with("treemap") {
        for index in 0..12 {
            config.set_value(
                &format!("themeVariables.cScaleLabel{index}"),
                css_color(theme.text_color).into(),
            );
        }
    }
    let mut custom_palette = explicit_theme;

    if let Some(variables) = source.config.as_value().get("themeVariables") {
        let variables = variables
            .as_object()
            .context("Mermaid themeVariables must be an object")?;
        custom_palette |= !variables.is_empty();
        apply_theme_variables(&mut config, "themeVariables", variables)?;
        // These site defaults otherwise mask Mermaid's derived primary colors.
        for (primary, aliases) in [
            ("primaryColor", &["mainBkg"][..]),
            ("primaryBorderColor", &["nodeBorder"][..]),
            ("primaryTextColor", &["nodeTextColor", "labelColor"][..]),
        ] {
            if let Some(value) = config
                .get_str(&format!("themeVariables.{primary}"))
                .map(str::to_owned)
                && variables.contains_key(primary)
            {
                for alias in aliases {
                    if !variables.contains_key(*alias) {
                        config.set_value(&format!("themeVariables.{alias}"), value.clone().into());
                    }
                }
            }
        }
    }
    if let Some(family) = source.config.as_value().get("fontFamily") {
        apply_theme_variables(
            &mut config,
            "themeVariables",
            &serde_json::Map::from_iter([("fontFamily".to_owned(), family.clone())]),
        )?;
        custom_palette = true;
    }
    if source
        .config
        .as_value()
        .get("packet")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|packet| {
            [
                "startByteColor",
                "endByteColor",
                "labelColor",
                "titleColor",
                "blockStrokeColor",
                "blockFillColor",
            ]
            .iter()
            .any(|key| packet.contains_key(*key))
        })
    {
        custom_palette = true;
    }
    Ok((config, custom_palette))
}

fn apply_theme_variables(
    config: &mut merman::MermaidConfig,
    prefix: &str,
    variables: &serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    for (key, value) in variables {
        anyhow::ensure!(
            !key.is_empty()
                && key
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'),
            "Invalid Mermaid theme variable: {key}"
        );
        let path = format!("{prefix}.{key}");
        if let Some(nested) = value.as_object() {
            apply_theme_variables(config, &path, nested)?;
        } else if prefix == "themeVariables" && key == "darkMode" {
            anyhow::ensure!(value.is_boolean(), "Mermaid {path} must be a boolean");
            config.set_value(&path, value.clone());
            config.set_value("darkMode", value.clone());
        } else if prefix == "themeVariables" && key == "fontFamily" {
            let family = value
                .as_str()
                .context("Mermaid fontFamily must be a string")?;
            anyhow::ensure!(
                !family.trim().is_empty()
                    && family.chars().all(|character| character.is_alphanumeric()
                        || matches!(character, ' ' | '-' | '_' | ',' | '\'' | '"')),
                "Unsupported Mermaid fontFamily"
            );
            config.set_value(&path, value.clone());
            config.set_value("fontFamily", value.clone());
        } else if prefix == "themeVariables" && key == "fontSize" {
            let size = value
                .as_str()
                .context("Mermaid fontSize must be a CSS pixel size")?;
            let pixels = size
                .trim()
                .strip_suffix("px")
                .unwrap_or(size.trim())
                .parse::<f64>()?;
            anyhow::ensure!(
                pixels.is_finite() && pixels > 0.0,
                "Invalid Mermaid fontSize"
            );
            config.set_value(&path, format!("{pixels}px").into());
        } else {
            // Keep merman's secure source policy intact: promote only parsed colors.
            let value = value
                .as_str()
                .with_context(|| format!("Mermaid color {path} must be a string"))?;
            let color = merman::theme_color::ThemeColor::parse(value)
                .with_context(|| format!("Unsupported Mermaid color {path}"))?;
            config.set_value(&path, color.stringify().into());
        }
    }
    Ok(())
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
        "emUiFill": primary,
        "emUiStroke": primary_border,
        "emProcessorFill": secondary,
        "emProcessorStroke": primary_border,
        "emReadModelFill": tertiary,
        "emReadModelStroke": primary_border,
        "emCommandFill": primary,
        "emCommandStroke": primary_border,
        "emEventFill": secondary,
        "emEventStroke": primary_border,
        "emSwimlaneBackgroundOdd": background,
        "emSwimlaneBackgroundStroke": primary_border,
        "emRelationStroke": text,
        "emArrowhead": text,
        "vennSetTextColor": text,
        "vennTitleTextColor": text,
        "treeView": {
            "labelColor": text,
            "lineColor": text,
            "iconColor": text,
            "descriptionColor": text,
        },
        "wardley": {
            "backgroundColor": background,
            "axisColor": text,
            "axisTextColor": text,
            "gridColor": primary_border,
            "componentFill": primary,
            "componentStroke": text,
            "componentLabelColor": text,
            "linkStroke": text,
        },
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
        "darkMode": theme.dark_mode,
        "fontFamily": theme.font_family,
        // resvg can't rasterize HTML `<foreignObject>` labels, so merman's
        // raster-safe pipeline replaces them with wrapped native SVG text.
        "htmlLabels": true,
        "flowchart": {
            "htmlLabels": true,
            "padding": 16,
        },
        "packet": {
            "startByteColor": text,
            "endByteColor": text,
            "labelColor": primary_text,
            "titleColor": text,
            "blockStrokeColor": primary_border,
            "blockFillColor": primary,
        },
        "themeVariables": theme_vars,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_palette_preserves_host_styling() {
        let theme = MermaidTheme::default();
        for variables in ["", "---\nconfig:\n  themeVariables: {}\n---\n"] {
            let (_, custom_palette) =
                config_for_source(&format!("{variables}flowchart LR\n A --> B"), &theme)
                    .expect("config");
            assert!(!custom_palette);
        }
    }

    #[test]
    fn explicit_typography_survives_native_rendering() {
        let source = "---\nconfig:\n  theme: base\n  themeVariables:\n    fontFamily: monospace\n    fontSize: 24px\n    darkMode: true\n---\nflowchart LR\n A[Custom font] --> B[Other]";
        let theme = MermaidTheme::default();
        let (config, _) = config_for_source(source, &theme).expect("config");
        assert_eq!(config.get_str("fontFamily"), Some("monospace"));
        assert_eq!(config.as_value()["themeVariables"]["darkMode"], true);
        let svg = crate::render_to_svg(source, &theme).expect("render");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("native SVG");
        assert_eq!(text_font_size(tree.root(), "Custom font"), Some(24.0));
    }

    #[test]
    fn typography_cannot_inject_css() {
        for variables in [
            "fontFamily: 'serif;fill:red'",
            "fontFamily: 'url(example)'",
            "fontSize: '24px;fill:red'",
            "fontSize: '-1px'",
            "darkMode: 'yes'",
        ] {
            let source = format!(
                "---\nconfig:\n  themeVariables:\n    {variables}\n---\nflowchart LR\n A --> B"
            );
            assert!(crate::render_to_svg(&source, &MermaidTheme::default()).is_err());
        }
    }

    #[test]
    fn sequence_front_matter_reaches_renderer_and_renders() {
        let source = "---\nconfig:\n  sequence:\n    wrap: false\n    width: 190\n    actorMargin: 85\n    messageMargin: 38\n    noteMargin: 18\n    diagramMarginX: 24\n    diagramMarginY: 24\n    mirrorActors: false\n    actorFontSize: 16\n    messageFontSize: 16\n    noteFontSize: 15\n---\nsequenceDiagram\n    participant Alice\n    participant Bob\n    Alice->>Bob: Hello\n    Note over Alice,Bob: Note";
        let theme = MermaidTheme::default();
        let (config, custom_palette) = config_for_source(source, &theme).expect("config");
        assert!(!custom_palette);
        let effective = merman::Engine::new()
            .with_site_config(config)
            .parse_metadata_sync(source)
            .expect("metadata")
            .effective_config;
        let original = merman::preprocess_diagram(source, &merman::DetectorRegistry::default())
            .expect("front matter");
        for (key, value) in original.config.as_value()["sequence"]
            .as_object()
            .expect("sequence options")
        {
            assert_eq!(
                &effective.as_value()["sequence"][key],
                value,
                "option {key}"
            );
        }
        let svg = crate::render_to_svg(source, &theme).expect("render");
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(&svg, &options).expect("native SVG");
        assert_eq!(text_font_size(tree.root(), "Alice"), Some(16.0));
        assert_eq!(text_font_size(tree.root(), "Hello"), Some(16.0));
        // Mermaid 12's sequence setConf applies the global 16px size to notes too.
        assert_eq!(text_font_size(tree.root(), "Note"), Some(16.0));
        assert!(svg.contains("width=\"190\""), "configured actor width");
    }

    #[test]
    fn render_stage_applies_resvg_safe_pipeline() {
        let html_label_source =
            "classDiagram\n    class Shelter {\n        -List~Animal~ animals\n    }";
        let (html_label_svg, _) =
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
        let (css_svg, _) =
            render_mermaid(css_source, &MermaidTheme::default()).expect("render failed");

        assert!(!css_svg.contains("@keyframes"), "got: {css_svg}");
        assert!(!css_svg.contains("@-webkit-keyframes"), "got: {css_svg}");
        assert!(!css_svg.contains(":root"), "got: {css_svg}");
        assert!(!css_svg.contains("animation:"), "got: {css_svg}");
        assert!(!css_svg.contains("animation-name:"), "got: {css_svg}");
    }

    #[test]
    fn long_flowchart_labels_render_as_wrapped_svg_text() {
        let source = "flowchart TD\n    \
            A[\"Pass 2: search transcript with annotation blocks excised, \
            map offsets back to buffer space\"] --> \
            |ambiguous or zero| B[\"Error describing where matches were found\"]";
        let (svg, _) = render_mermaid(source, &MermaidTheme::default()).expect("render failed");

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
        let (svg, _) = render_mermaid(source, &theme).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme, false).expect("postprocess failed");
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
        let (svg, _) = render_mermaid(source, &theme).expect("render failed");
        let svg = crate::postprocess::postprocess(&svg, &theme, false).expect("postprocess failed");
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
