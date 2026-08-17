use super::theme::RadarTheme;
use super::*;
use merman_core::diagrams::radar::RadarDiagramRenderModel;

// Radar diagram SVG renderer implementation (split from parity.rs).

fn radar_css(diagram_id: &str, theme: &RadarTheme) -> String {
    // Keep `:root` last (matches upstream Mermaid radar SVG baselines).
    let id = escape_xml(diagram_id);

    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{};fill:{};}}"#,
        id, theme.font_family_css, theme.base_font_size_css, theme.text_color
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
        id, theme.error_bkg_color, id, theme.error_text_color, theme.error_text_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id, theme.line_color, theme.line_color, id, theme.line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{};}}#{} p{{margin:0;}}"#,
        id, theme.font_family_css, theme.base_font_size_css, id
    );

    let _ = write!(
        &mut out,
        r#"#{} .radarTitle{{font-size:{};color:{};dominant-baseline:hanging;text-anchor:middle;}}"#,
        id, theme.title_font_size_css, theme.title_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .radarAxisLine{{stroke:{};stroke-width:{};}}"#,
        id,
        theme.axis_color,
        fmt(theme.axis_stroke_width)
    );
    let _ = write!(
        &mut out,
        r#"#{} .radarAxisLabel{{dominant-baseline:middle;text-anchor:middle;font-size:{}px;color:{};}}"#,
        id,
        fmt(theme.axis_label_font_size),
        theme.axis_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .radarGraticule{{fill:{};fill-opacity:{};stroke:{};stroke-width:{};}}"#,
        id,
        theme.graticule_color,
        fmt(theme.graticule_opacity),
        theme.graticule_color,
        fmt(theme.graticule_stroke_width)
    );
    let _ = write!(
        &mut out,
        r#"#{} .radarLegendText{{text-anchor:start;font-size:{}px;dominant-baseline:hanging;}}"#,
        id,
        fmt(theme.legend_font_size)
    );

    for (i, c) in theme.series_colors.iter().enumerate() {
        let _ = write!(
            &mut out,
            r#"#{} .radarCurve-{}{{color:{};fill:{};fill-opacity:{};stroke:{};stroke-width:{};}}#{} .radarLegendBox-{}{{fill:{};fill-opacity:{};stroke:{};}}"#,
            id,
            i,
            c,
            c,
            fmt(theme.curve_opacity),
            c,
            fmt(theme.curve_stroke_width),
            id,
            i,
            c,
            fmt(theme.curve_opacity),
            c
        );
    }

    let _ = write!(
        &mut out,
        r#"#{} :root{{--mermaid-font-family:{};}}"#,
        id, theme.font_family_css
    );

    out
}

pub(super) fn render_radar_diagram_svg(
    layout: &RadarDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: RadarDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_radar_diagram_svg_model(layout, &model, effective_config, options)
}

pub(super) fn render_radar_diagram_svg_model(
    layout: &RadarDiagramLayout,
    model: &RadarDiagramRenderModel,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("radar");
    let diagram_id_esc = escape_xml(diagram_id);

    let has_acc_title = model
        .acc_title
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty());
    let has_acc_descr = model
        .acc_descr
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty());

    let viewbox_attr = format!("0 0 {} {}", fmt(layout.svg_width), fmt(layout.svg_height));
    let max_w_attr = fmt_max_width_px(layout.svg_width);
    let style_attr = format!("max-width: {max_w_attr}px; background-color: white;");

    let aria_describedby = has_acc_descr.then(|| format!("chart-desc-{diagram_id_esc}"));
    let aria_labelledby = has_acc_title.then(|| format!("chart-title-{diagram_id_esc}"));

    let mut out = String::new();
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(viewbox_attr.as_str()),
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "radar")
        },
    );

    if has_acc_title {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{id}">{text}</title>"#,
            id = diagram_id_esc,
            text = escape_xml(model.acc_title.as_deref().unwrap_or_default())
        );
    }
    if has_acc_descr {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{id}">{text}</desc>"#,
            id = diagram_id_esc,
            text = escape_xml(model.acc_descr.as_deref().unwrap_or_default())
        );
    }

    let theme = PresentationTheme::new(effective_config).radar();
    let css = radar_css(diagram_id, &theme);
    let _ = write!(&mut out, "<style>{}</style>", css);
    out.push_str("<g/>");

    let _ = write!(
        &mut out,
        r#"<g transform="translate({x}, {y})">"#,
        x = fmt_display(layout.center_x),
        y = fmt_display(layout.center_y)
    );

    for g in &layout.graticules {
        if g.kind == "polygon" {
            if g.points.is_empty() {
                out.push_str(r#"<polygon points="" class="radarGraticule"/>"#);
            } else {
                let points = fmt_points(&g.points);
                let _ = write!(
                    &mut out,
                    r#"<polygon points="{points}" class="radarGraticule"/>"#,
                    points = escape_xml(&points)
                );
            }
        } else if let Some(r) = g.r {
            let _ = write!(
                &mut out,
                r#"<circle r="{r}" class="radarGraticule"/>"#,
                r = fmt_display(r)
            );
        }
    }

    for a in &layout.axes {
        let _ = write!(
            &mut out,
            r#"<line x1="0" y1="0" x2="{x2}" y2="{y2}" class="radarAxisLine"/>"#,
            x2 = fmt_display(a.line_x2),
            y2 = fmt_display(a.line_y2)
        );
        let _ = write!(
            &mut out,
            r#"<text x="{x}" y="{y}" class="radarAxisLabel">{label}</text>"#,
            x = fmt_display(a.label_x),
            y = fmt_display(a.label_y),
            label = escape_xml(&a.label)
        );
    }

    let polygon_curves = layout
        .graticules
        .first()
        .is_some_and(|g| g.kind.trim() == "polygon");
    for c in &layout.curves {
        if polygon_curves && !c.points.is_empty() {
            let points = fmt_points(&c.points);
            let _ = write!(
                &mut out,
                r#"<polygon points="{points}" class="radarCurve-{idx}"/>"#,
                points = escape_xml(&points),
                idx = c.class_index
            );
        } else {
            let _ = write!(
                &mut out,
                r#"<path d="{d}" class="radarCurve-{idx}"/>"#,
                d = escape_xml(&c.path_d),
                idx = c.class_index
            );
        }
    }

    for item in &layout.legend_items {
        let _ = write!(
            &mut out,
            r#"<g transform="translate({x}, {y})">"#,
            x = fmt_display(item.x),
            y = fmt_display(item.y)
        );
        let _ = write!(
            &mut out,
            r#"<rect width="{size}" height="{size}" class="radarLegendBox-{idx}"/>"#,
            size = fmt_display(12.0),
            idx = item.class_index
        );
        let label = model
            .curves
            .get(item.class_index as usize)
            .map(|c| c.label.as_str())
            .unwrap_or("");
        let _ = write!(
            &mut out,
            r#"<text x="{x}" y="{y}" class="radarLegendText">{text}</text>"#,
            x = fmt_display(16.0),
            y = fmt_display(0.0),
            text = escape_xml(label)
        );
        out.push_str("</g>");
    }

    match model.title.as_deref() {
        Some(t) => {
            let _ = write!(
                &mut out,
                r#"<text class="radarTitle" x="0" y="{y}">{text}</text>"#,
                y = fmt(layout.title_y),
                text = escape_xml(t)
            );
        }
        None => {
            let _ = write!(
                &mut out,
                r#"<text class="radarTitle" x="0" y="{y}"/>"#,
                y = fmt(layout.title_y)
            );
        }
    }

    out.push_str("</g></svg>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radar_css_honors_top_level_style_overrides() {
        let cfg = serde_json::json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "fontSize": "18px",
                "textColor": "#101010",
                "titleColor": "#202020",
                "cScale0": "#303030",
                "radar": {
                    "axisColor": "#404040",
                    "axisStrokeWidth": 2,
                    "axisLabelFontSize": 12,
                    "graticuleColor": "#505050",
                    "graticuleOpacity": 0.3,
                    "graticuleStrokeWidth": 1,
                    "legendFontSize": 12,
                    "curveOpacity": 0.5,
                    "curveStrokeWidth": 2
                }
            },
            "radar": {
                "axisColor": "#606060",
                "axisStrokeWidth": 4,
                "axisLabelFontSize": 14,
                "graticuleColor": "#707070",
                "graticuleOpacity": 0.8,
                "graticuleStrokeWidth": 5,
                "legendFontSize": 16,
                "curveOpacity": 0.9,
                "curveStrokeWidth": 6
            }
        });

        let theme = PresentationTheme::new(&cfg).radar();
        let css = radar_css("radar", &theme);

        assert!(css.contains(r#"#radar .radarTitle{font-size:18px;color:#202020;"#));
        assert!(css.contains(r#"#radar .radarAxisLine{stroke:#606060;stroke-width:4;}"#));
        assert!(css.contains(
            r#"#radar .radarAxisLabel{dominant-baseline:middle;text-anchor:middle;font-size:14px;color:#606060;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarGraticule{fill:#707070;fill-opacity:0.8;stroke:#707070;stroke-width:5;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarLegendText{text-anchor:start;font-size:16px;dominant-baseline:hanging;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarCurve-0{color:#303030;fill:#303030;fill-opacity:0.9;stroke:#303030;stroke-width:6;}"#
        ));
    }

    #[test]
    fn radar_css_uses_scoped_theme_variables_when_top_level_is_missing() {
        let cfg = serde_json::json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "fontSize": "18px",
                "textColor": "#101010",
                "titleColor": "#202020",
                "cScale0": "#303030",
                "radar": {
                    "axisColor": "#404040",
                    "axisStrokeWidth": 2,
                    "axisLabelFontSize": 12,
                    "graticuleColor": "#505050",
                    "graticuleOpacity": 0.3,
                    "graticuleStrokeWidth": 1,
                    "legendFontSize": 12,
                    "curveOpacity": 0.5,
                    "curveStrokeWidth": 2
                }
            }
        });

        let theme = PresentationTheme::new(&cfg).radar();
        let css = radar_css("radar", &theme);

        assert!(css.contains(r#"#radar .radarAxisLine{stroke:#404040;stroke-width:2;}"#));
        assert!(css.contains(
            r#"#radar .radarAxisLabel{dominant-baseline:middle;text-anchor:middle;font-size:12px;color:#404040;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarGraticule{fill:#505050;fill-opacity:0.3;stroke:#505050;stroke-width:1;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarLegendText{text-anchor:start;font-size:12px;dominant-baseline:hanging;}"#
        ));
        assert!(css.contains(
            r#"#radar .radarCurve-0{color:#303030;fill:#303030;fill-opacity:0.5;stroke:#303030;stroke-width:2;}"#
        ));
    }

    #[test]
    fn radar_root_uses_responsive_width_and_max_width_style() {
        let layout = RadarDiagramLayout {
            bounds: None,
            svg_width: 700.0,
            svg_height: 700.0,
            center_x: 350.0,
            center_y: 350.0,
            radius: 300.0,
            axis_label_factor: 1.05,
            title_y: -350.0,
            axes: Vec::new(),
            graticules: Vec::new(),
            curves: Vec::new(),
            legend_items: Vec::new(),
        };
        let options = SvgRenderOptions {
            diagram_id: Some("radarRoot".to_string()),
            ..SvgRenderOptions::default()
        };

        let svg = render_radar_diagram_svg_model(
            &layout,
            &RadarDiagramRenderModel::default(),
            &serde_json::json!({}),
            &options,
        )
        .unwrap();
        let root_open = svg.split_once('>').expect("root svg open tag").0;

        assert!(root_open.contains(r#"width="100%""#), "{root_open}");
        assert!(
            root_open.contains(
                r#"style="max-width: 700px; background-color: white;" viewBox="0 0 700 700""#
            ),
            "{root_open}"
        );
        assert!(
            !root_open.contains(r#"height=""#),
            "radar root should not emit fixed height: {root_open}"
        );
    }
}
