use super::*;

// Shared Mermaid diagram CSS fragments (split from parity.rs).
//
// Keep Mermaid@11.12.2 ordering quirks to preserve DOM parity.

pub(super) fn info_css_into(out: &mut String, diagram_id: &str) {
    let id = escape_xml(diagram_id);
    let font = r#""trebuchet ms",verdana,arial,sans-serif"#;
    let _ = write!(
        out,
        r#"#{}{{font-family:{};font-size:16px;fill:#333;}}"#,
        id, font
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id, id
    );
    let _ = write!(
        out,
        r#"#{} .error-icon{{fill:#552222;}}#{} .error-text{{fill:#552222;stroke:#552222;}}"#,
        id, id
    );
    let _ = write!(
        out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        out,
        r#"#{} .marker{{fill:#333333;stroke:#333333;}}#{} .marker.cross{{stroke:#333333;}}"#,
        id, id
    );
    let _ = write!(
        out,
        r#"#{} svg{{font-family:{};font-size:16px;}}#{} p{{margin:0;}}#{} :root{{--mermaid-font-family:{};}}"#,
        id, font, id, id, font
    );
}

pub(super) struct InfoCssParts {
    pub(super) css_prefix: String,
    pub(super) root_rule: String,
    pub(super) font_family: String,
    pub(super) text_color: String,
    pub(super) line_color: String,
}

#[derive(Clone, Copy)]
enum InfoCssFontSizeSource {
    ThemeThenTopLevel,
    ThemeOnly,
}

pub(super) fn info_css_parts_with_config(
    diagram_id: &str,
    effective_config: &serde_json::Value,
) -> InfoCssParts {
    info_css_parts_with_font_size_source(
        diagram_id,
        effective_config,
        InfoCssFontSizeSource::ThemeThenTopLevel,
    )
}

pub(super) fn info_css_parts_with_theme_font_size_only(
    diagram_id: &str,
    effective_config: &serde_json::Value,
) -> InfoCssParts {
    info_css_parts_with_font_size_source(
        diagram_id,
        effective_config,
        InfoCssFontSizeSource::ThemeOnly,
    )
}

fn info_css_parts_with_font_size_source(
    diagram_id: &str,
    effective_config: &serde_json::Value,
    font_size_source: InfoCssFontSizeSource,
) -> InfoCssParts {
    let id = escape_xml(diagram_id);

    let font_family = crate::config::config_font_family_css(effective_config);
    let font_size = match font_size_source {
        InfoCssFontSizeSource::ThemeThenTopLevel => {
            crate::config::config_theme_font_size_css_or_root_number_px_opt(effective_config)
        }
        InfoCssFontSizeSource::ThemeOnly => {
            config_f64_css_px(effective_config, &["themeVariables", "fontSize"])
        }
    }
    .unwrap_or(16.0)
    .max(1.0);

    let text_color = theme_color(effective_config, "textColor", "#333");
    let line_color = theme_color(effective_config, "lineColor", "#333333");
    let error_bkg = theme_color(effective_config, "errorBkgColor", "#552222");
    let error_text = theme_color(effective_config, "errorTextColor", "#552222");

    // Keep `:root` last (matches upstream Mermaid SVG baselines).
    let root_rule = format!(r#"#{} :root{{--mermaid-font-family:{};}}"#, id, font_family);

    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id,
        font_family,
        fmt(font_size),
        text_color
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .error-icon{{fill:{};}}#{} .error-text{{fill:{};stroke:{};}}"#,
        id, error_bkg, id, error_text, error_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id, line_color, line_color, id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}px;}}#{} p{{margin:0;}}"#,
        id,
        font_family,
        fmt(font_size),
        id
    );

    InfoCssParts {
        css_prefix: out,
        root_rule,
        font_family,
        text_color,
        line_color,
    }
}

pub(super) fn info_css_with_config(
    diagram_id: &str,
    effective_config: &serde_json::Value,
) -> String {
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;
    out.push_str(&parts.root_rule);
    out
}

pub(super) fn architecture_css_with_config(
    diagram_id: &str,
    effective_config: &serde_json::Value,
) -> String {
    // Architecture uses the same "info-like" base stylesheet as Mermaid, but should honor
    // user-configured `fontFamily` / `fontSize` and theme variable colors.
    let id = escape_xml(diagram_id);

    let font_family = SvgTheme::new(effective_config).font_family_css();
    let font_size =
        crate::config::config_theme_font_size_css_or_root_number_px(effective_config, 16.0)
            .max(1.0);

    let text_color = theme_color(effective_config, "textColor", "#333");
    let line_color = theme_color(effective_config, "lineColor", "#333333");
    let error_bkg = theme_color(effective_config, "errorBkgColor", "#552222");
    let error_text = theme_color(effective_config, "errorTextColor", "#552222");
    let primary_border = theme_color(
        effective_config,
        "primaryBorderColor",
        "hsl(240, 60%, 86.2745098039%)",
    );
    let arch_edge_color = theme_color(effective_config, "archEdgeColor", &line_color);
    let arch_edge_arrow_color =
        theme_color(effective_config, "archEdgeArrowColor", &arch_edge_color);
    let arch_edge_width = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "archEdgeWidth"],
    )
    .unwrap_or_else(|| "3".to_string());
    let arch_group_border_color =
        theme_color(effective_config, "archGroupBorderColor", &primary_border);
    let arch_group_border_width = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "archGroupBorderWidth"],
    )
    .unwrap_or_else(|| "2px".to_string());

    // Keep `:root` last (matches upstream Mermaid SVG baselines).
    let root_rule = format!(r#"#{} :root{{--mermaid-font-family:{};}}"#, id, font_family);

    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id,
        font_family,
        fmt(font_size),
        text_color
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .error-icon{{fill:{};}}#{} .error-text{{fill:{};stroke:{};}}"#,
        id, error_bkg, id, error_text, error_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id, line_color, line_color, id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}px;}}#{} p{{margin:0;}}"#,
        id,
        font_family,
        fmt(font_size),
        id
    );

    let _ = write!(
        &mut out,
        r#"#{} .edge{{stroke-width:{};stroke:{};fill:none;}}"#,
        id, arch_edge_width, arch_edge_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .arrow{{fill:{};}}"#,
        id, arch_edge_arrow_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .node-bkg{{fill:none;stroke:{};stroke-width:{};stroke-dasharray:8;}}"#,
        id, arch_group_border_color, arch_group_border_width
    );
    let _ = write!(
        &mut out,
        r#"#{} .node-icon-text{{display:flex;align-items:center;}}"#,
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} .node-icon-text>div{{color:#fff;margin:1px;height:fit-content;text-align:center;overflow:hidden;display:-webkit-box;-webkit-box-orient:vertical;}}"#,
        id
    );

    out.push_str(&root_rule);
    out
}

pub(super) fn requirement_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mirrors Mermaid 11.15 `diagrams/requirement/styles.js` + shared base stylesheet ordering.
    // Keep `:root` last to match upstream fixtures.
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;
    let font = parts.font_family;
    let text_color = parts.text_color;
    let node_text_color = config_string(effective_config, &["themeVariables", "nodeTextColor"])
        .unwrap_or_else(|| text_color.clone());

    let option = |key: &str, default_value: &str| -> String {
        crate::config::config_css_number_or_string(effective_config, &["themeVariables", key])
            .unwrap_or_else(|| default_value.to_string())
    };

    let relation_color = option("relationColor", "#333333");
    let line_color = option("lineColor", "#333333");
    let font_size = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "fontSize"],
    )
    .or_else(|| crate::config::config_css_number_or_string(effective_config, &["fontSize"]))
    .unwrap_or_else(|| "16px".to_string());
    let requirement_background = option("requirementBackground", "#ECECFF");
    let requirement_border_color =
        option("requirementBorderColor", "hsl(240, 60%, 86.2745098039%)");
    let requirement_border_size = option("requirementBorderSize", "1");
    let requirement_text_color = option("requirementTextColor", "#131300");
    let relation_label_background = option("relationLabelBackground", "rgba(232,232,232, 0.8)");
    let relation_label_color = option("relationLabelColor", "black");
    let edge_label_background = option("edgeLabelBackground", "rgba(232,232,232, 0.8)");
    let requirement_edge_label_background = config_string(
        effective_config,
        &["themeVariables", "requirementEdgeLabelBackground"],
    )
    .unwrap_or_else(|| edge_label_background.clone());
    let node_border = option("nodeBorder", "#9370DB");
    let look = config_diagram_look(effective_config);
    let relationship_line_stroke_width = if look.is_neo() {
        option("strokeWidth", "1")
    } else {
        "1px".to_string()
    };
    let neo_node_stroke_width = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "strokeWidth"],
    )
    .map(|value| {
        let value = value.trim().to_string();
        if value.parse::<f64>().is_ok() {
            format!("{value}px")
        } else {
            value
        }
    })
    .unwrap_or_else(|| "1px".to_string());

    let _ = write!(
        &mut out,
        r#"#{} marker{{fill:{};stroke:{};}}#{} marker.cross{{stroke:{};}}"#,
        id, relation_color, relation_color, id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}}}#{} .reqBox{{fill:{};fill-opacity:1.0;stroke:{};stroke-width:{};}}#{} .reqTitle,#{} .reqLabel{{fill:{};}}#{} .reqLabelBox{{fill:{};fill-opacity:1.0;}}#{} .req-title-line{{stroke:{};stroke-width:{};}}#{} .relationshipLine{{stroke:{};stroke-width:{};}}#{} .relationshipLabel{{fill:{};}}#{} .edgeLabel{{background-color:{};}}#{} .edgeLabel .label rect{{fill:{};}}#{} .edgeLabel .label text{{fill:{};}}#{} .divider{{stroke:{};stroke-width:1;}}#{} .label{{font-family:{};color:{};}}#{} .label text,#{} span{{fill:{};color:{};}}#{} .labelBkg{{background-color:{};}}"#,
        id,
        font,
        font_size,
        id,
        requirement_background,
        requirement_border_color,
        requirement_border_size,
        id,
        id,
        requirement_text_color,
        id,
        relation_label_background,
        id,
        requirement_border_color,
        requirement_border_size,
        id,
        relation_color,
        relationship_line_stroke_width,
        id,
        relation_label_color,
        id,
        edge_label_background,
        id,
        edge_label_background,
        id,
        relation_label_color,
        id,
        node_border,
        id,
        font,
        node_text_color,
        id,
        id,
        node_text_color,
        node_text_color,
        id,
        requirement_edge_label_background
    );
    if look.is_neo() {
        let _ = write!(
            &mut out,
            r#"#{} .node .neo-node{{stroke:{};}}#{} [data-look="neo"].node path{{stroke:{};stroke-width:{};}}"#,
            id, node_border, id, node_border, neo_node_stroke_width
        );
    }
    out.push_str(&parts.root_rule);
    out
}

pub(super) fn er_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mirrors Mermaid@11.15.0 ER unified renderer stylesheet ordering (see `diagrams/er/styles.ts`
    // and shared base stylesheet).
    // Keep `:root` last (matches upstream fixtures).
    let id = escape_xml(diagram_id);
    let theme = SvgTheme::new(effective_config);
    let font = theme.font_family_css();
    let font_size = crate::config::config_theme_or_root_font_size_px_opt(effective_config)
        .or_else(|| config_f64_css_px(effective_config, &["er", "fontSize"]))
        .unwrap_or(16.0)
        .max(1.0);
    let text_color = theme.color("textColor", "#333");
    let line_color = theme.color("lineColor", "#333333");
    let error_bkg = theme.color("errorBkgColor", "#552222");
    let error_text = theme.color("errorTextColor", "#552222");
    let main_bkg = theme.color("mainBkg", "#ECECFF");
    let node_border = theme.color("nodeBorder", "#9370DB");
    let node_text_color = theme
        .optional_color("nodeTextColor")
        .unwrap_or_else(|| text_color.clone());
    const DEFAULT_ER_TERTIARY: &str = "hsl(80, 100%, 96.2745098039%)";
    const DEFAULT_ER_TERTIARY_FADE: &str = "rgba(248.6666666666, 255, 235.9999999999, 0.5)";
    let tertiary_color = theme.color("tertiaryColor", DEFAULT_ER_TERTIARY);
    let edge_label_background = theme.color("edgeLabelBackground", "rgba(232,232,232, 0.8)");
    let er_edge_label_background = match theme.theme_name().as_str() {
        "redux-color" | "redux-dark-color" => theme.optional_color("erEdgeLabelBackground"),
        _ => None,
    };
    let label_background = er_edge_label_background.clone().unwrap_or_else(|| {
        if tertiary_color == DEFAULT_ER_TERTIARY {
            DEFAULT_ER_TERTIARY_FADE.to_string()
        } else {
            css_rgba_fade(&tertiary_color, 0.5)
                .unwrap_or_else(|| DEFAULT_ER_TERTIARY_FADE.to_string())
        }
    });
    let edge_label_background = er_edge_label_background.unwrap_or(edge_label_background);
    let stroke_width = if theme.look() == "neo" {
        theme.css_value("strokeWidth", "1px")
    } else {
        "1px".to_string()
    };
    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id,
        font,
        fmt(font_size),
        text_color
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .error-icon{{fill:{};}}#{} .error-text{{fill:{};stroke:{};}}"#,
        id, error_bkg, id, error_text, error_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id, line_color, line_color, id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}px;}}#{} p{{margin:0;}}"#,
        id,
        font,
        fmt(font_size),
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} .entityBox{{fill:{};stroke:{};}}"#,
        id, main_bkg, node_border
    );
    let _ = write!(
        &mut out,
        r#"#{} .relationshipLabelBox{{fill:{};opacity:0.7;background-color:{};}}"#,
        id, tertiary_color, tertiary_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .relationshipLabelBox rect{{opacity:0.5;}}"#,
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} .labelBkg{{background-color:{};}}"#,
        id, label_background
    );
    let _ = write!(
        &mut out,
        r#"#{} .edgeLabel{{background-color:{};}}#{} .edgeLabel .label rect{{fill:{};}}#{} .edgeLabel .label text{{fill:{};}}#{} .edgeLabel .label{{fill:{};font-size:14px;}}"#,
        id, edge_label_background, id, edge_label_background, id, text_color, id, node_border
    );
    let _ = write!(
        &mut out,
        r#"#{} .label{{font-family:{};color:{};}}"#,
        id, font, node_text_color
    );
    // Mermaid duplicates `.edge-pattern-dashed` (base rule earlier sets dasharray:3).
    let _ = write!(
        &mut out,
        r#"#{} .edge-pattern-dashed{{stroke-dasharray:8,8;}}"#,
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} .node rect,#{} .node circle,#{} .node ellipse,#{} .node polygon{{fill:{};stroke:{};stroke-width:{};}}"#,
        id, id, id, id, main_bkg, node_border, stroke_width
    );
    let _ = write!(
        &mut out,
        r#"#{} .relationshipLine{{stroke:{};stroke-width:{};fill:none;}}"#,
        id, line_color, stroke_width
    );
    // Mermaid duplicates `.marker` (base rule earlier sets fill/stroke from the line color).
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:none!important;stroke:{}!important;stroke-width:1;}}"#,
        id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} :root{{--mermaid-font-family:{};}}"#,
        id, font
    );
    out
}

fn pie_theme_option(
    effective_config: &serde_json::Value,
    key: &str,
    default_value: &str,
) -> String {
    SvgTheme::new(effective_config).css_value(key, default_value)
}

pub(super) fn pie_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mirrors Mermaid@11.15.0 `diagrams/pie/pieStyles.ts`. Keep `:root` last to match the
    // config-aware CSS emitters used by the other diagram families.
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;
    let font = parts.font_family;
    let theme = SvgTheme::new(effective_config);
    let task_text_dark_color = theme.color("taskTextDarkColor", "black");
    let pie_stroke_color = pie_theme_option(effective_config, "pieStrokeColor", "black");
    let pie_stroke_width = pie_theme_option(effective_config, "pieStrokeWidth", "2px");
    let pie_opacity = pie_theme_option(effective_config, "pieOpacity", "0.7");
    let pie_outer_stroke_color = pie_theme_option(effective_config, "pieOuterStrokeColor", "black");
    let pie_outer_stroke_width = pie_theme_option(effective_config, "pieOuterStrokeWidth", "2px");
    let pie_title_text_size = pie_theme_option(effective_config, "pieTitleTextSize", "25px");
    let pie_title_text_color = theme.color("pieTitleTextColor", task_text_dark_color.as_str());
    let pie_section_text_size = pie_theme_option(effective_config, "pieSectionTextSize", "17px");
    let pie_section_text_color = theme.color("pieSectionTextColor", parts.text_color.as_str());
    let pie_legend_text_size = pie_theme_option(effective_config, "pieLegendTextSize", "17px");
    let pie_legend_text_color = theme.color("pieLegendTextColor", task_text_dark_color.as_str());
    let _ = write!(
        &mut out,
        r#"#{} .pieCircle{{stroke:{};stroke-width:{};opacity:{};}}#{} .pieOuterCircle{{stroke:{};stroke-width:{};fill:none;}}#{} .pieTitleText{{text-anchor:middle;font-size:{};fill:{};font-family:{};}}#{} .slice{{font-family:{};fill:{};font-size:{};}}#{} .legend text{{fill:{};font-family:{};font-size:{};}}"#,
        id,
        pie_stroke_color,
        pie_stroke_width,
        pie_opacity,
        id,
        pie_outer_stroke_color,
        pie_outer_stroke_width,
        id,
        pie_title_text_size,
        pie_title_text_color,
        font,
        id,
        font,
        pie_section_text_color,
        pie_section_text_size,
        id,
        pie_legend_text_color,
        font,
        pie_legend_text_size
    );
    out.push_str(&parts.root_rule);
    out
}

pub(super) fn sankey_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mermaid's sankey diagram uses the same base CSS as "info-like" diagrams, then appends
    // `sankey/styles.js` rules. Keep `:root` last to match upstream SVG baselines.
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;
    let label_background = config_string(effective_config, &["themeVariables", "mainBkg"])
        .or_else(|| config_string(effective_config, &["themeVariables", "background"]))
        .unwrap_or_else(|| "#fff".to_string());
    let _ = write!(
        &mut out,
        r#"#{} .label{{font-family:{};}}#{} .node-labels{{font-family:{};}}#{} .sankey-label-bg{{stroke:{};stroke-width:4px;stroke-linejoin:round;paint-order:stroke;}}#{} .sankey-label-fg{{fill:{};}}#{} .node rect{{shape-rendering:crispEdges;}}#{} .link{{fill:none;stroke-opacity:0.5;mix-blend-mode:multiply;}}"#,
        id,
        parts.font_family,
        id,
        parts.font_family,
        id,
        label_background,
        id,
        parts.text_color,
        id,
        id
    );
    out.push_str(&parts.root_rule);
    out
}

pub(super) fn treemap_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mermaid's treemap styles merge `treemap.*` options with theme title/text colors. Keep
    // `:root` last to match upstream SVG baselines.
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let theme = PresentationTheme::new(effective_config).treemap();
    let mut out = parts.css_prefix;

    let _ = write!(
        &mut out,
        r#"#{} .treemapNode.section{{stroke:{};stroke-width:{};fill:{};}}#{} .treemapNode.leaf{{stroke:{};stroke-width:{};fill:{};}}#{} .treemapLabel{{fill:{};font-size:{};}}#{} .treemapValue{{fill:{};font-size:{};}}#{} .treemapTitle{{fill:{};font-size:{};}}"#,
        id,
        theme.section_stroke_color,
        theme.section_stroke_width,
        theme.section_fill_color,
        id,
        theme.leaf_stroke_color,
        theme.leaf_stroke_width,
        theme.leaf_fill_color,
        id,
        theme.label_color,
        theme.label_font_size,
        id,
        theme.value_color,
        theme.value_font_size,
        id,
        theme.title_color,
        theme.title_font_size
    );
    out.push_str(&parts.root_rule);
    out
}

pub(super) fn push_xychart_css(out: &mut String, diagram_id: &str) {
    // Mermaid does not ship dedicated XYChart styles at 11.12.2 (it relies on theme variables and
    // inline attributes). Keep the shared base stylesheet for consistency with upstream SVG
    // baselines. The compare tooling ignores `<style>` content in parity mode.
    info_css_into(out, diagram_id);
}

pub(super) fn gantt_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let theme = PresentationTheme::new(effective_config).gantt();
    let mut out = parts.css_prefix;
    let font = &theme.font_family;
    let text_color = &theme.text_color;
    let exclude_bkg_color = &theme.exclude_bkg_color;
    let section_bkg_color = &theme.section_bkg_color;
    let section_bkg_color2 = &theme.section_bkg_color2;
    let alt_section_bkg_color = &theme.alt_section_bkg_color;
    let title_color = &theme.title_color;
    let grid_color = &theme.grid_color;
    let today_line_color = &theme.today_line_color;
    let task_text_dark_color = &theme.task_text_dark_color;
    let task_text_clickable_color = &theme.task_text_clickable_color;
    let task_text_color = &theme.task_text_color;
    let task_bkg_color = &theme.task_bkg_color;
    let task_border_color = &theme.task_border_color;
    let task_text_outside_color = &theme.task_text_outside_color;
    let active_task_bkg_color = &theme.active_task_bkg_color;
    let active_task_border_color = &theme.active_task_border_color;
    let done_task_border_color = &theme.done_task_border_color;
    let done_task_bkg_color = &theme.done_task_bkg_color;
    let crit_border_color = &theme.crit_border_color;
    let crit_bkg_color = &theme.crit_bkg_color;
    let vert_line_color = &theme.vert_line_color;
    let title_text_color = &theme.title_text_color;

    fn push_outside_done_text_rules(out: &mut String, id: &str, class_prefix: &str, color: &str) {
        let mut first = true;
        for i in 0..4 {
            for side in ["Left", "Right"] {
                if first {
                    first = false;
                } else {
                    out.push(',');
                }
                let _ = write!(
                    out,
                    "#{} .{}{}.taskTextOutside{}",
                    id, class_prefix, i, side
                );
            }
        }
        let _ = write!(out, "{{fill:{}!important;}}", color);
    }

    let _ = write!(
        &mut out,
        r#"#{} .mermaid-main-font{{font-family:{};}}"#,
        id, font
    );
    let _ = write!(
        &mut out,
        r#"#{} .exclude-range{{fill:{};}}"#,
        id, exclude_bkg_color
    );
    let _ = write!(&mut out, r#"#{} .section{{stroke:none;opacity:0.2;}}"#, id);
    let _ = write!(
        &mut out,
        r#"#{} .section0{{fill:{};}}"#,
        id, section_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .section2{{fill:{};}}"#,
        id, section_bkg_color2
    );
    let _ = write!(
        &mut out,
        r#"#{} .section1,#{} .section3{{fill:{};opacity:0.2;}}"#,
        id, id, alt_section_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .sectionTitle0{{fill:{};}}#{} .sectionTitle1{{fill:{};}}#{} .sectionTitle2{{fill:{};}}#{} .sectionTitle3{{fill:{};}}"#,
        id, title_color, id, title_color, id, title_color, id, title_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .sectionTitle{{text-anchor:start;font-family:{};}}"#,
        id, font
    );
    let _ = write!(
        &mut out,
        r#"#{} .grid .tick{{stroke:{};opacity:0.8;shape-rendering:crispEdges;}}#{} .grid .tick text{{font-family:{};fill:{};}}#{} .grid path{{stroke-width:0;}}"#,
        id, grid_color, id, font, text_color, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .today{{fill:none;stroke:{};stroke-width:2px;}}"#,
        id, today_line_color
    );
    let _ = write!(&mut out, r#"#{} .task{{stroke-width:2;}}"#, id);
    let _ = write!(
        &mut out,
        r#"#{} .taskText{{text-anchor:middle;font-family:{};}}#{} .taskTextOutsideRight{{fill:{};text-anchor:start;font-family:{};}}#{} .taskTextOutsideLeft{{fill:{};text-anchor:end;}}"#,
        id, font, id, task_text_dark_color, font, id, task_text_dark_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .task.clickable{{cursor:pointer;}}#{} .taskText.clickable{{cursor:pointer;fill:{}!important;font-weight:bold;}}#{} .taskTextOutsideLeft.clickable{{cursor:pointer;fill:{}!important;font-weight:bold;}}#{} .taskTextOutsideRight.clickable{{cursor:pointer;fill:{}!important;font-weight:bold;}}"#,
        id,
        id,
        task_text_clickable_color,
        id,
        task_text_clickable_color,
        id,
        task_text_clickable_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .taskText0,#{} .taskText1,#{} .taskText2,#{} .taskText3{{fill:{};}}"#,
        id, id, id, id, task_text_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .task0,#{} .task1,#{} .task2,#{} .task3{{fill:{};stroke:{};}}"#,
        id, id, id, id, task_bkg_color, task_border_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .taskTextOutside0,#{} .taskTextOutside2{{fill:{};}}#{} .taskTextOutside1,#{} .taskTextOutside3{{fill:{};}}"#,
        id, id, task_text_outside_color, id, id, task_text_outside_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .active0,#{} .active1,#{} .active2,#{} .active3{{fill:{};stroke:{};}}"#,
        id, id, id, id, active_task_bkg_color, active_task_border_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .activeText0,#{} .activeText1,#{} .activeText2,#{} .activeText3{{fill:{}!important;}}"#,
        id, id, id, id, task_text_dark_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .done0,#{} .done1,#{} .done2,#{} .done3{{stroke:{};fill:{};stroke-width:2;}}"#,
        id, id, id, id, done_task_border_color, done_task_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .doneText0,#{} .doneText1,#{} .doneText2,#{} .doneText3{{fill:{}!important;}}"#,
        id, id, id, id, task_text_dark_color
    );
    push_outside_done_text_rules(&mut out, &id, "doneText", &task_text_outside_color);
    let _ = write!(
        &mut out,
        r#"#{} .crit0,#{} .crit1,#{} .crit2,#{} .crit3{{stroke:{};fill:{};stroke-width:2;}}"#,
        id, id, id, id, crit_border_color, crit_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .activeCrit0,#{} .activeCrit1,#{} .activeCrit2,#{} .activeCrit3{{stroke:{};fill:{};stroke-width:2;}}"#,
        id, id, id, id, crit_border_color, active_task_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .doneCrit0,#{} .doneCrit1,#{} .doneCrit2,#{} .doneCrit3{{stroke:{};fill:{};stroke-width:2;cursor:pointer;shape-rendering:crispEdges;}}"#,
        id, id, id, id, crit_border_color, done_task_bkg_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .milestone{{transform:rotate(45deg) scale(0.8,0.8);}}#{} .milestoneText{{font-style:italic;}}"#,
        id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .doneCritText0,#{} .doneCritText1,#{} .doneCritText2,#{} .doneCritText3{{fill:{}!important;}}"#,
        id, id, id, id, task_text_dark_color
    );
    push_outside_done_text_rules(&mut out, &id, "doneCritText", &task_text_outside_color);
    let _ = write!(
        &mut out,
        r#"#{} .vert{{stroke:{};}}#{} .vertText{{font-size:15px;text-anchor:middle;fill:{}!important;}}"#,
        id, vert_line_color, id, vert_line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .activeCritText0,#{} .activeCritText1,#{} .activeCritText2,#{} .activeCritText3{{fill:{}!important;}}"#,
        id, id, id, id, task_text_dark_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .titleText{{text-anchor:middle;font-size:18px;fill:{};font-family:{};}}"#,
        id, title_text_color, font
    );

    out.push_str(&parts.root_rule);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_css_with_config_honors_font_and_theme_colors() {
        let cfg = serde_json::json!({
            "fontFamily": "\"courier new\", courier, monospace;",
            "fontSize": 18,
            "themeVariables": {
                "textColor": "#112233",
                "lineColor": "#445566",
                "primaryBorderColor": "#778899",
                "archEdgeColor": "#010203",
                "archEdgeArrowColor": "#040506",
                "archEdgeWidth": 7,
                "archGroupBorderColor": "#070809",
                "archGroupBorderWidth": "6px",
            }
        });

        let css = architecture_css_with_config("diag", &cfg);

        assert!(css.contains(
            r#"#diag{font-family:"courier new",courier,monospace;font-size:18px;fill:#112233;}"#
        ));
        assert!(css.contains(r#"#diag .edge{stroke-width:7;stroke:#010203;fill:none;}"#));
        assert!(css.contains(r#"#diag .arrow{fill:#040506;}"#));
        assert!(css.contains(
            r#"#diag .node-bkg{fill:none;stroke:#070809;stroke-width:6px;stroke-dasharray:8;}"#
        ));
        assert!(
            css.contains(r#"#diag :root{--mermaid-font-family:"courier new",courier,monospace;}"#)
        );
    }

    #[test]
    fn architecture_css_prefers_theme_font_family_over_legacy_root() {
        let cfg = serde_json::json!({
            "fontFamily": "Courier, monospace",
            "themeVariables": {
                "fontFamily": "\"IBM Plex Sans\", Arial, sans-serif"
            }
        });

        let css = architecture_css_with_config("diag", &cfg);

        assert!(css.contains(
            r#"#diag{font-family:"IBM Plex Sans",Arial,sans-serif;font-size:16px;fill:#333;}"#
        ));
        assert!(
            css.contains(r#"#diag :root{--mermaid-font-family:"IBM Plex Sans",Arial,sans-serif;}"#)
        );
    }

    #[test]
    fn sankey_css_honors_mermaid_11_15_theme_options() {
        let cfg = serde_json::json!({
            "fontFamily": "\"source sans\", arial, sans-serif",
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#123456",
                "mainBkg": "#abcdef",
            }
        });

        let css = sankey_css("sk", &cfg);

        assert!(css.contains(r#"#sk .label{font-family:"ibm plex sans",arial,sans-serif;}"#));
        assert!(css.contains(r#"#sk .node-labels{font-family:"ibm plex sans",arial,sans-serif;}"#));
        assert!(css.contains(r#"#sk .sankey-label-bg{stroke:#abcdef;"#));
        assert!(css.contains(r#"#sk .sankey-label-fg{fill:#123456;}"#));
        assert!(
            css.contains(r#"#sk :root{--mermaid-font-family:"ibm plex sans",arial,sans-serif;}"#)
        );
    }

    #[test]
    fn pie_css_honors_mermaid_11_15_theme_options() {
        let cfg = serde_json::json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#111111",
                "taskTextDarkColor": "#222222",
                "pieStrokeColor": "#333333",
                "pieStrokeWidth": "4px",
                "pieOpacity": "0.9",
                "pieOuterStrokeColor": "#444444",
                "pieOuterStrokeWidth": "5px",
                "pieTitleTextSize": "26px",
                "pieTitleTextColor": "#555555",
                "pieSectionTextSize": "18px",
                "pieSectionTextColor": "#666666",
                "pieLegendTextSize": "19px",
                "pieLegendTextColor": "#777777"
            }
        });

        let css = pie_css("pie", &cfg);

        assert!(css.contains(r#"#pie .pieCircle{stroke:#333333;stroke-width:4px;opacity:0.9;}"#));
        assert!(
            css.contains(r#"#pie .pieOuterCircle{stroke:#444444;stroke-width:5px;fill:none;}"#)
        );
        assert!(css.contains(r#"#pie .pieTitleText{text-anchor:middle;font-size:26px;fill:#555555;font-family:"ibm plex sans",arial,sans-serif;}"#));
        assert!(css.contains(r#"#pie .slice{font-family:"ibm plex sans",arial,sans-serif;fill:#666666;font-size:18px;}"#));
        assert!(css.contains(r#"#pie .legend text{fill:#777777;font-family:"ibm plex sans",arial,sans-serif;font-size:19px;}"#));
    }

    #[test]
    fn er_css_honors_mermaid_11_15_theme_options() {
        let cfg = serde_json::json!({
            "look": "neo",
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "fontSize": "18px",
                "textColor": "#101010",
                "lineColor": "#202020",
                "errorBkgColor": "#303030",
                "errorTextColor": "#404040",
                "mainBkg": "#505050",
                "nodeBorder": "#606060",
                "nodeTextColor": "#707070",
                "tertiaryColor": "#8090a0",
                "edgeLabelBackground": "#b0c0d0",
                "strokeWidth": 3
            }
        });

        let css = er_css("er", &cfg);

        assert!(css.contains(
            r#"#er{font-family:"ibm plex sans",arial,sans-serif;font-size:18px;fill:#101010;}"#
        ));
        assert!(css.contains(
            r#"#er .error-icon{fill:#303030;}#er .error-text{fill:#404040;stroke:#404040;}"#
        ));
        assert!(css.contains(r#"#er .marker{fill:#202020;stroke:#202020;}"#));
        assert!(css.contains(r#"#er .entityBox{fill:#505050;stroke:#606060;}"#));
        assert!(css.contains(
            r#"#er .relationshipLabelBox{fill:#8090a0;opacity:0.7;background-color:#8090a0;}"#
        ));
        assert!(css.contains(r#"#er .labelBkg{background-color:rgba(128, 144, 160, 0.5);}"#));
        assert!(css.contains(r#"#er .edgeLabel{background-color:#b0c0d0;}#er .edgeLabel .label rect{fill:#b0c0d0;}#er .edgeLabel .label text{fill:#101010;}#er .edgeLabel .label{fill:#606060;font-size:14px;}"#));
        assert!(css.contains(
            r#"#er .label{font-family:"ibm plex sans",arial,sans-serif;color:#707070;}"#
        ));
        assert!(css.contains(r#"#er .node rect,#er .node circle,#er .node ellipse,#er .node polygon{fill:#505050;stroke:#606060;stroke-width:3;}"#));
        assert!(css.contains(r#"#er .relationshipLine{stroke:#202020;stroke-width:3;fill:none;}"#));
        assert!(css.contains(
            r#"#er .marker{fill:none!important;stroke:#202020!important;stroke-width:1;}"#
        ));
    }

    #[test]
    fn gantt_css_honors_mermaid_11_15_theme_options() {
        let cfg = serde_json::json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#707070",
                "excludeBkgColor": "#101010",
                "sectionBkgColor": "#202020",
                "sectionBkgColor2": "#303030",
                "altSectionBkgColor": "#404040",
                "titleColor": "#505050",
                "gridColor": "#606060",
                "todayLineColor": "#808080",
                "taskTextDarkColor": "#909090",
                "taskTextClickableColor": "#a0a0a0",
                "taskTextColor": "#b0b0b0",
                "taskBkgColor": "#c0c0c0",
                "taskBorderColor": "#d0d0d0",
                "taskTextOutsideColor": "#e0e0e0",
                "activeTaskBkgColor": "#111111",
                "activeTaskBorderColor": "#222222",
                "doneTaskBorderColor": "#333333",
                "doneTaskBkgColor": "#444444",
                "critBorderColor": "#555555",
                "critBkgColor": "#666666",
                "vertLineColor": "#777777"
            }
        });

        let css = gantt_css("g", &cfg);

        assert!(css.contains(r#"#g .exclude-range{fill:#101010;}"#));
        assert!(css.contains(r#"#g .section0{fill:#202020;}"#));
        assert!(css.contains(r#"#g .section2{fill:#303030;}"#));
        assert!(css.contains(r#"#g .grid .tick{stroke:#606060;"#));
        assert!(
            css.contains(
                r#"#g .taskText0,#g .taskText1,#g .taskText2,#g .taskText3{fill:#b0b0b0;}"#
            )
        );
        assert!(
            css.contains(
                r#"#g .task0,#g .task1,#g .task2,#g .task3{fill:#c0c0c0;stroke:#d0d0d0;}"#
            )
        );
        assert!(
            css.contains(r#"#g .doneText0.taskTextOutsideLeft,#g .doneText0.taskTextOutsideRight"#)
        );
        assert!(css.contains(r#"fill:#e0e0e0!important;"#));
        assert!(css.contains(r#"#g .titleText{text-anchor:middle;font-size:18px;fill:#505050;font-family:"ibm plex sans",arial,sans-serif;}"#));
    }

    #[test]
    fn treemap_css_honors_mermaid_11_15_style_options() {
        let cfg = serde_json::json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#123456",
                "titleColor": "#654321"
            },
            "treemap": {
                "sectionStrokeColor": "#111111",
                "sectionStrokeWidth": 2,
                "sectionFillColor": "#222222",
                "leafStrokeColor": "#333333",
                "leafStrokeWidth": "3",
                "leafFillColor": "#444444",
                "labelColor": "#555555",
                "valueColor": "#666666",
                "titleColor": "#777777",
                "labelFontSize": "13px",
                "valueFontSize": "11px",
                "titleFontSize": "15px"
            }
        });

        let css = treemap_css("tm", &cfg);

        assert!(
            css.contains("#tm .treemapNode.section{stroke:#111111;stroke-width:2;fill:#222222;}")
        );
        assert!(css.contains("#tm .treemapNode.leaf{stroke:#333333;stroke-width:3;fill:#444444;}"));
        assert!(css.contains("#tm .treemapLabel{fill:#555555;font-size:13px;}"));
        assert!(css.contains("#tm .treemapValue{fill:#666666;font-size:11px;}"));
        assert!(css.contains("#tm .treemapTitle{fill:#777777;font-size:15px;}"));
    }

    #[test]
    fn requirement_css_honors_mermaid_11_15_theme_options() {
        let cfg = serde_json::json!({
            "look": "neo",
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "fontSize": "18px",
                "textColor": "#101010",
                "nodeTextColor": "#111111",
                "relationColor": "#222222",
                "lineColor": "#333333",
                "requirementBackground": "#444444",
                "requirementBorderColor": "#555555",
                "requirementBorderSize": 2,
                "requirementTextColor": "#666666",
                "relationLabelBackground": "#777777",
                "relationLabelColor": "#888888",
                "edgeLabelBackground": "#999999",
                "requirementEdgeLabelBackground": "#aaaaaa",
                "nodeBorder": "#bbbbbb",
                "strokeWidth": 3
            }
        });

        let css = requirement_css("req", &cfg);

        assert!(css.contains(r#"#req marker{fill:#222222;stroke:#222222;}"#));
        assert!(css.contains(r#"#req marker.cross{stroke:#333333;}"#));
        assert!(css.contains(
            r#"#req .reqBox{fill:#444444;fill-opacity:1.0;stroke:#555555;stroke-width:2;}"#
        ));
        assert!(css.contains(r#"#req .reqTitle,#req .reqLabel{fill:#666666;}"#));
        assert!(css.contains(r#"#req .reqLabelBox{fill:#777777;fill-opacity:1.0;}"#));
        assert!(css.contains(r#"#req .relationshipLine{stroke:#222222;stroke-width:3;}"#));
        assert!(css.contains(r#"#req .relationshipLabel{fill:#888888;}"#));
        assert!(css.contains(r#"#req .edgeLabel .label rect{fill:#999999;}"#));
        assert!(css.contains(r#"#req .labelBkg{background-color:#aaaaaa;}"#));
        assert!(
            css.contains(r#"#req [data-look="neo"].node path{stroke:#bbbbbb;stroke-width:3px;}"#)
        );
    }
}
