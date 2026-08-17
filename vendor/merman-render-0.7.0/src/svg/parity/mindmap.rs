use super::*;

// Mindmap diagram SVG renderer implementation (split from parity.rs).

#[derive(Debug, Clone, Copy)]
enum MindmapPathNumberFormat {
    D3Path,
    JsNumber,
}

fn mindmap_path_number(v: f64, number_format: MindmapPathNumberFormat) -> String {
    match number_format {
        MindmapPathNumberFormat::D3Path => fmt_path(v),
        MindmapPathNumberFormat::JsNumber => fmt_string(v),
    }
}

fn mindmap_cloud_path_d(w: f64, h: f64, number_format: MindmapPathNumberFormat) -> String {
    let r1 = 0.15 * w;
    let r2 = 0.25 * w;
    let r3 = 0.35 * w;
    let r4 = 0.2 * w;
    let n = |v| mindmap_path_number(v, number_format);

    format!(
        "M0 0 a{r1},{r1} 0 0,1 {w25},{wn10} a{r3},{r3} 1 0,1 {w40},{wn10} a{r2},{r2} 1 0,1 {w35},{w20} a{r1},{r1} 1 0,1 {w15},{h35} a{r4},{r4} 1 0,1 {wn15},{h65} a{r2},{r1} 1 0,1 {wn25},{w15} a{r3},{r3} 1 0,1 {wn50},0 a{r1},{r1} 1 0,1 {wn25},{wn15} a{r1},{r1} 1 0,1 {wn10},{hn35} a{r4},{r4} 1 0,1 {w10},{hn65} H0 V0 Z",
        r1 = n(r1),
        r2 = n(r2),
        r3 = n(r3),
        r4 = n(r4),
        w25 = n(w * 0.25),
        w40 = n(w * 0.4),
        w35 = n(w * 0.35),
        w20 = n(w * 0.2),
        w15 = n(w * 0.15),
        w10 = n(w * 0.1),
        wn10 = n(-w * 0.1),
        wn15 = n(-w * 0.15),
        wn25 = n(-w * 0.25),
        wn50 = n(-w * 0.5),
        h35 = n(h * 0.35),
        h65 = n(h * 0.65),
        hn35 = n(-h * 0.35),
        hn65 = n(-h * 0.65),
    )
}

pub(super) fn mindmap_cloud_rendered_bbox_size_px(w: f64, h: f64) -> Option<(f64, f64)> {
    let d = mindmap_cloud_path_d(w, h, MindmapPathNumberFormat::JsNumber);
    let pb = svg_path_bounds_from_d(&d)?;
    Some((pb.max_x - pb.min_x, pb.max_y - pb.min_y))
}

fn mindmap_bang_path_d(
    w_base: f64,
    effective_w: f64,
    effective_h: f64,
    number_format: MindmapPathNumberFormat,
) -> String {
    let r = 0.15 * w_base;
    let n = |v| mindmap_path_number(v, number_format);

    format!(
        "M0 0 a{r},{r} 1 0,0 {w25},{hn10} a{r},{r} 1 0,0 {w25},0 a{r},{r} 1 0,0 {w25},0 a{r},{r} 1 0,0 {w25},{h10} a{r},{r} 1 0,0 {w15},{h33} a{r08},{r08} 1 0,0 0,{h34} a{r},{r} 1 0,0 {wn15},{h33} a{r},{r} 1 0,0 {wn25},{h15} a{r},{r} 1 0,0 {wn25},0 a{r},{r} 1 0,0 {wn25},0 a{r},{r} 1 0,0 {wn25},{hn15} a{r},{r} 1 0,0 {wn10},{hn33} a{r08},{r08} 1 0,0 0,{hn34} a{r},{r} 1 0,0 {w10},{hn33} H0 V0 Z",
        r = n(r),
        r08 = n(r * 0.8),
        w25 = n(effective_w * 0.25),
        w15 = n(effective_w * 0.15),
        w10 = n(effective_w * 0.1),
        wn10 = n(-effective_w * 0.1),
        wn15 = n(-effective_w * 0.15),
        wn25 = n(-effective_w * 0.25),
        h10 = n(effective_h * 0.1),
        hn10 = n(-effective_h * 0.1),
        h15 = n(effective_h * 0.15),
        hn15 = n(-effective_h * 0.15),
        h33 = n(effective_h * 0.33),
        hn33 = n(-effective_h * 0.33),
        h34 = n(effective_h * 0.34),
        hn34 = n(-effective_h * 0.34),
    )
}

fn include_mindmap_rect_bounds(
    bounds: &mut Option<Bounds>,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) {
    if let Some(cur) = bounds.as_mut() {
        cur.min_x = cur.min_x.min(min_x);
        cur.min_y = cur.min_y.min(min_y);
        cur.max_x = cur.max_x.max(max_x);
        cur.max_y = cur.max_y.max(max_y);
    } else {
        *bounds = Some(Bounds {
            min_x,
            min_y,
            max_x,
            max_y,
        });
    }
}

fn include_mindmap_node_rect_bounds(bounds: &mut Option<Bounds>, n: &LayoutNode) {
    include_mindmap_rect_bounds(
        bounds,
        n.x - n.width / 2.0,
        n.y - n.height / 2.0,
        n.x + n.width / 2.0,
        n.y + n.height / 2.0,
    );
}

fn include_mindmap_path_bounds(
    bounds: &mut Option<Bounds>,
    d: &str,
    translate_x: f64,
    translate_y: f64,
) -> bool {
    let Some(pb) = svg_path_bounds_from_d(d) else {
        return false;
    };
    include_mindmap_rect_bounds(
        bounds,
        pb.min_x + translate_x,
        pb.min_y + translate_y,
        pb.max_x + translate_x,
        pb.max_y + translate_y,
    );
    true
}

fn mindmap_viewport_bounds_from_layout(
    layout: &MindmapDiagramLayout,
    model: &merman_core::diagrams::mindmap::MindmapDiagramRenderModel,
) -> Option<Bounds> {
    let mut layout_nodes: std::collections::BTreeMap<&str, &LayoutNode> =
        std::collections::BTreeMap::new();
    for n in &layout.nodes {
        layout_nodes.insert(n.id.as_str(), n);
    }

    let mut bounds: Option<Bounds> = None;
    for n in &model.nodes {
        let Some(ln) = layout_nodes.get(n.id.as_str()) else {
            continue;
        };

        let padding = n.padding.max(0.0);
        let half_padding = padding / 2.0;
        match n.shape.as_str() {
            "cloud" => {
                let bbox_w = ln
                    .label_width
                    .unwrap_or_else(|| (ln.width - 2.0 * half_padding).max(1.0));
                let bbox_h = ln
                    .label_height
                    .unwrap_or_else(|| (ln.height - 2.0 * half_padding).max(1.0));
                let w = (bbox_w + 2.0 * half_padding).max(1.0);
                let h = (bbox_h + 2.0 * half_padding).max(1.0);
                let d = mindmap_cloud_path_d(w, h, MindmapPathNumberFormat::JsNumber);
                if !include_mindmap_path_bounds(&mut bounds, &d, ln.x - w / 2.0, ln.y - h / 2.0) {
                    include_mindmap_node_rect_bounds(&mut bounds, ln);
                }
                include_mindmap_rect_bounds(
                    &mut bounds,
                    ln.x - bbox_w / 2.0,
                    ln.y - bbox_h / 2.0,
                    ln.x + bbox_w / 2.0,
                    ln.y + bbox_h / 2.0,
                );
            }
            "bang" => {
                let w = ln.width.max(1.0);
                let h = ln.height.max(1.0);
                let bbox_w = ln
                    .label_width
                    .unwrap_or_else(|| (w - 10.0 * half_padding).max(1.0));
                let bbox_h = ln
                    .label_height
                    .unwrap_or_else(|| (h - 8.0 * half_padding).max(1.0));
                let w_base = bbox_w + 10.0 * half_padding;
                let d = mindmap_bang_path_d(w_base, w, h, MindmapPathNumberFormat::JsNumber);
                if !include_mindmap_path_bounds(&mut bounds, &d, ln.x - w / 2.0, ln.y - h / 2.0) {
                    include_mindmap_node_rect_bounds(&mut bounds, ln);
                }
                include_mindmap_rect_bounds(
                    &mut bounds,
                    ln.x - bbox_w / 2.0,
                    ln.y - bbox_h / 2.0,
                    ln.x + bbox_w / 2.0,
                    ln.y + bbox_h / 2.0,
                );
            }
            _ => include_mindmap_node_rect_bounds(&mut bounds, ln),
        }
    }

    for e in &layout.edges {
        for p in &e.points {
            include_mindmap_rect_bounds(&mut bounds, p.x, p.y, p.x, p.y);
        }
    }

    bounds
}

fn mindmap_model_look(model_look: &str, config: &merman_core::MermaidConfig) -> String {
    let model_look = model_look.trim();
    if model_look.is_empty() || model_look == "default" {
        crate::config::mermaid_config_diagram_look(config)
            .as_str()
            .to_string()
    } else {
        model_look.to_string()
    }
}

fn mindmap_data_look_attr(model_look: &str, config: &merman_core::MermaidConfig) -> String {
    let look = mindmap_model_look(model_look, config);
    if look.is_empty() {
        String::new()
    } else {
        format!(r#" data-look="{}""#, escape_attr(&look))
    }
}

fn mindmap_dom_id(diagram_id: &str, raw_id: &str) -> String {
    if diagram_id.is_empty() {
        raw_id.to_string()
    } else {
        format!("{diagram_id}-{raw_id}")
    }
}

fn mindmap_wrap_section_index(index: i64) -> i64 {
    if index >= 11 { index % 11 } else { index }
}

fn mindmap_normalize_section_class_token(token: &str) -> String {
    for prefix in ["section-edge-", "section-"] {
        let Some(rest) = token.strip_prefix(prefix) else {
            continue;
        };
        let Ok(index) = rest.parse::<i64>() else {
            return token.to_string();
        };
        return format!("{prefix}{}", mindmap_wrap_section_index(index));
    }
    token.to_string()
}

fn mindmap_normalize_section_classes(classes: &str) -> String {
    classes
        .split_whitespace()
        .map(mindmap_normalize_section_class_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn mindmap_gradient_defs(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    if !config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return String::new();
    }

    let Some(gradient_start) =
        config_string(effective_config, &["themeVariables", "gradientStart"])
    else {
        return String::new();
    };
    let Some(gradient_stop) = config_string(effective_config, &["themeVariables", "gradientStop"])
    else {
        return String::new();
    };

    format!(
        r#"<defs><linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient></defs>"#,
        escape_xml(diagram_id),
        escape_xml(&gradient_start),
        escape_xml(&gradient_stop)
    )
}

fn push_mindmap_shadow_defs(
    out: &mut String,
    diagram_id: &str,
    effective_config: &serde_json::Value,
) {
    let flood_color = effective_config
        .get("theme")
        .and_then(|v| v.as_str())
        .filter(|theme| theme.contains("dark"))
        .map(|_| "#FFFFFF")
        .unwrap_or("#000000");
    let diagram_id = escape_xml(diagram_id);
    let _ = write!(
        out,
        r#"<defs><filter id="{}-drop-shadow" height="130%" width="130%"><feDropShadow dx="4" dy="4" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs><defs><filter id="{}-drop-shadow-small" height="150%" width="150%"><feDropShadow dx="2" dy="2" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs>"#,
        diagram_id.as_str(),
        flood_color,
        diagram_id.as_str(),
        flood_color
    );
}

fn mindmap_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    // Mirrors Mermaid@11.15.0 `diagrams/mindmap/styles.ts` + shared base stylesheet ordering.
    //
    // Keep `:root` last (matches upstream fixtures).
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;

    let _ = write!(&mut out, r#"#{} .edge{{stroke-width:3;}}"#, id);

    // Mermaid default theme resolves `cScale0..11` into this palette for mindmap/kanban/timeline.
    // The first generated section is `section--1` (i=0).
    const DEFAULT_FILLS: [&str; 12] = [
        "hsl(240, 100%, 76.2745098039%)",
        "hsl(60, 100%, 73.5294117647%)",
        "hsl(80, 100%, 76.2745098039%)",
        "hsl(270, 100%, 76.2745098039%)",
        "hsl(300, 100%, 76.2745098039%)",
        "hsl(330, 100%, 76.2745098039%)",
        "hsl(0, 100%, 76.2745098039%)",
        "hsl(30, 100%, 76.2745098039%)",
        "hsl(90, 100%, 76.2745098039%)",
        "hsl(150, 100%, 76.2745098039%)",
        "hsl(180, 100%, 76.2745098039%)",
        "hsl(210, 100%, 76.2745098039%)",
    ];
    const DEFAULT_INV_FILLS: [&str; 12] = [
        "hsl(60, 100%, 86.2745098039%)",
        "hsl(240, 100%, 83.5294117647%)",
        "hsl(260, 100%, 86.2745098039%)",
        "hsl(90, 100%, 86.2745098039%)",
        "hsl(120, 100%, 86.2745098039%)",
        "hsl(150, 100%, 86.2745098039%)",
        "hsl(180, 100%, 86.2745098039%)",
        "hsl(210, 100%, 86.2745098039%)",
        "hsl(270, 100%, 86.2745098039%)",
        "hsl(330, 100%, 86.2745098039%)",
        "hsl(0, 100%, 86.2745098039%)",
        "hsl(30, 100%, 86.2745098039%)",
    ];

    fn default_mindmap_fill(i: usize) -> &'static str {
        DEFAULT_FILLS[i % DEFAULT_FILLS.len()]
    }

    fn default_mindmap_inv_fill(i: usize) -> &'static str {
        DEFAULT_INV_FILLS[i % DEFAULT_INV_FILLS.len()]
    }

    fn default_mindmap_label(i: usize) -> &'static str {
        if i == 0 || i == 3 { "#ffffff" } else { "black" }
    }

    let theme = config_string(effective_config, &["theme"]).unwrap_or_default();
    let look = config_diagram_look(effective_config);
    let theme_color_limit = config_f64(effective_config, &["themeVariables", "THEME_COLOR_LIMIT"])
        .map(|v| v.round() as i64)
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_FILLS.len() as i64)
        .clamp(1, 64) as usize;
    let root_fill = theme_color(effective_config, "git0", "hsl(240, 100%, 46.2745098039%)");
    let root_label = theme_color(effective_config, "gitBranchLabel0", "#ffffff");
    let node_border = theme_color(effective_config, "nodeBorder", root_label.as_str());
    let main_bkg = theme_color(effective_config, "mainBkg", root_fill.as_str());
    let stroke_width = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "strokeWidth"],
    )
    .unwrap_or_else(|| "2".to_string());
    let drop_shadow = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "dropShadow"],
    )
    .unwrap_or_else(|| "none".to_string());
    let scoped_drop_shadow =
        drop_shadow.replace("url(#drop-shadow)", &format!("url(#{id}-drop-shadow)"));
    let use_gradient =
        config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false);

    for i in 0..theme_color_limit {
        let section = i as i64 - 1;
        let c_scale = theme_color(
            effective_config,
            &format!("cScale{}", i),
            default_mindmap_fill(i),
        );
        let c_scale_inv = theme_color(
            effective_config,
            &format!("cScaleInv{}", i),
            default_mindmap_inv_fill(i),
        );
        let c_scale_label = theme_color(
            effective_config,
            &format!("cScaleLabel{}", i),
            default_mindmap_label(i),
        );
        let sw = if look.is_neo() {
            (10_i64 - (section * 2)).max(2)
        } else {
            17_i64 - 3_i64 * (i as i64)
        };
        let neo_node_fill = if theme == "redux" || theme == "redux-dark" || theme == "neutral" {
            main_bkg.as_str()
        } else {
            c_scale.as_str()
        };
        let neo_node_stroke = if theme == "redux" || theme == "redux-dark" {
            node_border.as_str()
        } else {
            c_scale.as_str()
        };
        let neo_edge_stroke = if theme.contains("redux") || theme == "neo-dark" {
            node_border.as_str()
        } else {
            c_scale.as_str()
        };
        let neo_text_label_index = if theme == "neutral" { 1 } else { i };
        let neo_text_label = if theme == "redux" || theme == "redux-dark" {
            node_border.clone()
        } else {
            theme_color(
                effective_config,
                &format!("cScaleLabel{}", neo_text_label_index),
                default_mindmap_label(neo_text_label_index),
            )
        };
        let _ = write!(
            &mut out,
            r#"#{} .section-{} rect,#{} .section-{} path,#{} .section-{} circle,#{} .section-{} polygon,#{} .section-{} path{{fill:{};}}"#,
            id, section, id, section, id, section, id, section, id, section, c_scale
        );
        let _ = write!(
            &mut out,
            r#"#{} .section-{} text{{fill:{};}}"#,
            id, section, c_scale_label
        );
        let _ = write!(
            &mut out,
            r#"#{} .section-{} span{{color:{};}}"#,
            id, section, c_scale_label
        );
        let _ = write!(
            &mut out,
            r#"#{} .node-icon-{}{{font-size:40px;color:{};}}"#,
            id, section, c_scale_label
        );
        let _ = write!(
            &mut out,
            r#"#{} .section-edge-{}{{stroke:{};}}"#,
            id, section, c_scale
        );
        let _ = write!(
            &mut out,
            r#"#{} .edge-depth-{}{{stroke-width:{};}}"#,
            id, section, sw
        );
        let _ = write!(
            &mut out,
            r#"#{} .section-{} line{{stroke:{};stroke-width:3;}}"#,
            id, section, c_scale_inv
        );
        let _ = write!(
            &mut out,
            r#"#{} .disabled,#{} .disabled circle,#{} .disabled text{{fill:lightgray;}}#{} .disabled text{{fill:#efefef;}}"#,
            id, id, id, id
        );
        let _ = write!(
            &mut out,
            r#"#{} [data-look="neo"].mindmap-node.section-{} rect,#{} [data-look="neo"].mindmap-node.section-{} path,#{} [data-look="neo"].mindmap-node.section-{} circle,#{} [data-look="neo"].mindmap-node.section-{} polygon{{fill:{};stroke:{};stroke-width:{}px;}}"#,
            id,
            section,
            id,
            section,
            id,
            section,
            id,
            section,
            neo_node_fill,
            neo_node_stroke,
            stroke_width
        );
        let _ = write!(
            &mut out,
            r#"#{} [data-look="neo"].section-edge-{}{{stroke:{};}}"#,
            id, section, neo_edge_stroke
        );
        let _ = write!(
            &mut out,
            r#"#{} [data-look="neo"].mindmap-node.section-{} text{{fill:{};}}"#,
            id, section, neo_text_label
        );
    }

    // Root section overrides.
    let root_span = if theme.contains("redux") {
        node_border.as_str()
    } else {
        root_label.as_str()
    };
    let _ = write!(
        &mut out,
        r#"#{} .section-root rect,#{} .section-root path,#{} .section-root circle,#{} .section-root polygon{{fill:{};}}"#,
        id, id, id, id, root_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} .section-root text{{fill:{};}}"#,
        id, root_label
    );
    let _ = write!(
        &mut out,
        r#"#{} .section-root span{{color:{};}}"#,
        id, root_span
    );
    let _ = write!(
        &mut out,
        r#"#{} .icon-container{{height:100%;display:flex;justify-content:center;align-items:center;}}"#,
        id
    );
    let _ = write!(&mut out, r#"#{} .edge{{fill:none;}}"#, id);
    let _ = write!(
        &mut out,
        r#"#{} .mindmap-node-label{{dy:1em;alignment-baseline:middle;text-anchor:middle;dominant-baseline:middle;text-align:center;}}"#,
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} [data-look="neo"].mindmap-node{{filter:{scoped_drop_shadow};}}"#,
        id
    );
    let neo_root_fill = if theme.contains("redux") {
        main_bkg.as_str()
    } else {
        root_fill.as_str()
    };
    let neo_root_text_label_index = if theme == "neutral" { 1 } else { 0 };
    let neo_root_text = if theme.contains("redux") {
        node_border
    } else {
        theme_color(
            effective_config,
            &format!("cScaleLabel{}", neo_root_text_label_index),
            default_mindmap_label(neo_root_text_label_index),
        )
    };
    let _ = write!(
        &mut out,
        r#"#{} [data-look="neo"].mindmap-node.section-root rect,#{} [data-look="neo"].mindmap-node.section-root path,#{} [data-look="neo"].mindmap-node.section-root circle,#{} [data-look="neo"].mindmap-node.section-root polygon{{fill:{};}}"#,
        id, id, id, id, neo_root_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} [data-look="neo"].mindmap-node.section-root .text-inner-tspan{{fill:{};}}"#,
        id, neo_root_text
    );
    if use_gradient {
        for i in 0..theme_color_limit {
            let section = i as i64 - 1;
            let _ = write!(
                &mut out,
                r#"#{} [data-look="neo"].mindmap-node.section-{} rect,#{} [data-look="neo"].mindmap-node.section-{} path,#{} [data-look="neo"].mindmap-node.section-{} circle,#{} [data-look="neo"].mindmap-node.section-{} polygon{{stroke:url(#{}-gradient);fill:{};}}"#,
                id, section, id, section, id, section, id, section, id, main_bkg
            );
            let _ = write!(
                &mut out,
                r#"#{} .section-{} line{{stroke-width:0;}}"#,
                id, section
            );
        }
    }

    out.push_str(&parts.root_rule);
    out
}

pub(super) fn render_mindmap_diagram_svg(
    layout: &MindmapDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: merman_core::diagrams::mindmap::MindmapDiagramRenderModel =
        crate::json::from_value_ref(semantic)?;
    render_mindmap_diagram_svg_model(layout, &model, _effective_config, options)
}

pub(super) fn render_mindmap_diagram_svg_with_config(
    layout: &MindmapDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: merman_core::diagrams::mindmap::MindmapDiagramRenderModel =
        { crate::json::from_value_ref(semantic)? };
    render_mindmap_diagram_svg_model_with_config(layout, &model, effective_config, options)
}

pub(super) fn render_mindmap_diagram_svg_model(
    layout: &MindmapDiagramLayout,
    model: &merman_core::diagrams::mindmap::MindmapDiagramRenderModel,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let config = merman_core::MermaidConfig::from_value(_effective_config.clone());
    render_mindmap_diagram_svg_model_with_config(layout, model, &config, options)
}

pub(super) fn render_mindmap_diagram_svg_model_with_config(
    layout: &MindmapDiagramLayout,
    model: &merman_core::diagrams::mindmap::MindmapDiagramRenderModel,
    config: &merman_core::MermaidConfig,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::timing::render_timing_enabled();
    let mut timings = super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();
    fn section<'a>(
        enabled: bool,
        dst: &'a mut web_time::Duration,
    ) -> Option<super::timing::TimingGuard<'a>> {
        enabled.then(|| super::timing::TimingGuard::new(dst))
    }

    #[derive(Debug, Clone, serde::Serialize)]
    struct Pt {
        x: f64,
        y: f64,
    }

    let max_node_width_px = crate::mindmap::mindmap_max_node_width_px(config.as_value());

    struct MindmapLabelSpec<'a> {
        text: &'a str,
        label_type: &'a str,
        label_bkg: bool,
        width: f64,
        height: f64,
        tx: f64,
        ty: f64,
        max_node_width_px: f64,
    }

    fn mk_label(out: &mut String, spec: MindmapLabelSpec<'_>, config: &merman_core::MermaidConfig) {
        let MindmapLabelSpec {
            text,
            label_type,
            label_bkg,
            width,
            height,
            tx,
            ty,
            max_node_width_px,
        } = spec;

        fn is_simple_markdown(text: &str) -> bool {
            // Conservative: only fast-path labels that would render as a plain `<p>text</p>`.
            if text.contains('\n') || text.contains('\r') {
                return false;
            }
            let trimmed = text.trim_start();
            let bytes = trimmed.as_bytes();
            // Line-leading markdown constructs that can change the HTML shape even without newlines.
            if bytes.first().is_some_and(|b| matches!(b, b'#' | b'>')) {
                return false;
            }
            if bytes.starts_with(b"- ") || bytes.starts_with(b"+ ") || bytes.starts_with(b"---") {
                return false;
            }
            // Ordered list: `1. item` / `1) item`
            let mut i = 0usize;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > 0
                && i + 1 < bytes.len()
                && (bytes[i] == b'.' || bytes[i] == b')')
                && bytes[i + 1] == b' '
            {
                return false;
            }
            // Block/inline markdown triggers we don't want to replicate here.
            if text.contains('*')
                || text.contains('_')
                || text.contains('`')
                || text.contains('~')
                || text.contains('[')
                || text.contains(']')
                || text.contains('!')
                || text.contains('\\')
            {
                return false;
            }
            // HTML passthrough / entity patterns: keep the full pulldown + sanitize path.
            if text.contains('<') || text.contains('>') || text.contains('&') {
                return false;
            }
            true
        }

        fn push_br_normalized_text_into(out: &mut String, text: &str) {
            // Mirror the existing `replace("<br>", "<br />").replace("<br/>", "<br />")` behavior,
            // but avoid allocating intermediate strings for the common case (no `<br>` tokens).
            let bytes = text.as_bytes();
            let mut i = 0usize;
            let mut start = 0usize;
            while i + 3 < bytes.len() {
                if bytes[i] == b'<' && bytes[i + 1] == b'b' && bytes[i + 2] == b'r' {
                    // "<br>"
                    if bytes[i + 3] == b'>' {
                        if start < i {
                            out.push_str(&text[start..i]);
                        }
                        out.push_str("<br />");
                        i += 4;
                        start = i;
                        continue;
                    }
                    // "<br/>"
                    if i + 4 < bytes.len() && bytes[i + 3] == b'/' && bytes[i + 4] == b'>' {
                        if start < i {
                            out.push_str(&text[start..i]);
                        }
                        out.push_str("<br />");
                        i += 5;
                        start = i;
                        continue;
                    }
                }
                i += 1;
            }
            if start < text.len() {
                out.push_str(&text[start..]);
            }
        }

        let div_class = if label_bkg {
            r#" class="labelBkg""#
        } else {
            ""
        };

        let max_node_width_px = if max_node_width_px.is_finite() && max_node_width_px > 0.0 {
            max_node_width_px
        } else {
            200.0
        };

        // Mermaid flips the `<div>` to a fixed-width wrapping container when the measured label
        // reaches/exceeds the configured max width (default 200px), even if the emitted
        // `<foreignObject width="...">` reflects the overflow width.
        let wrap_container = width >= max_node_width_px - 1e-3;
        out.push_str(r#"<g class="label" style="" transform="translate("#);
        fmt_into(out, tx);
        out.push_str(", ");
        fmt_into(out, ty);
        out.push_str(r#")"><rect/><foreignObject width=""#);
        fmt_into(out, width.max(1.0));
        out.push_str(r#"" height=""#);
        fmt_into(out, height.max(1.0));
        out.push_str(r#""><div xmlns="http://www.w3.org/1999/xhtml""#);
        out.push_str(div_class);
        out.push_str(r#" style=""#);
        if wrap_container {
            out.push_str(
                "display: table; white-space: break-spaces; line-height: 1.5; max-width: ",
            );
            fmt_into(out, max_node_width_px);
            out.push_str("px; text-align: center; width: ");
            fmt_into(out, max_node_width_px);
            out.push_str("px;");
        } else {
            out.push_str("display: table-cell; white-space: nowrap; line-height: 1.5; max-width: ");
            fmt_into(out, max_node_width_px);
            out.push_str("px; text-align: center;");
        }
        out.push_str(r#""><span class="nodeLabel markdown-node-label">"#);
        fn markdown_to_sanitized_xhtml(text: &str, config: &merman_core::MermaidConfig) -> String {
            let html_out = crate::text::mermaid_markdown_to_xhtml_label_fragment(text, true);
            let html_out = crate::text::replace_fontawesome_icons(&html_out);
            let html_out = merman_core::sanitize::sanitize_text(&html_out, config);
            html_out
                .replace("<br>", "<br />")
                .replace("<br/>", "<br />")
                .trim()
                .to_string()
        }

        fn is_single_img_fragment(html: &str) -> bool {
            // Mermaid does not wrap a single <img> label inside a <p> node for mindmap labels.
            let t = html.trim();
            let lower = t.to_ascii_lowercase();
            if lower.starts_with("<p>") && lower.ends_with("</p>") {
                let inner = t.strip_prefix("<p>").unwrap_or(t);
                let inner = inner.strip_suffix("</p>").unwrap_or(inner);
                return is_single_img_fragment(inner);
            }
            if !lower.starts_with("<img") {
                return false;
            }
            let Some(end) = t.find('>') else {
                return false;
            };
            t[end + 1..].trim().is_empty()
        }

        fn unwrap_single_img_p(html: &str) -> String {
            let t = html.trim();
            if !t.to_ascii_lowercase().starts_with("<p>")
                || !t.to_ascii_lowercase().ends_with("</p>")
            {
                return t.to_string();
            }
            let inner = t.strip_prefix("<p>").unwrap_or(t);
            let inner = inner.strip_suffix("</p>").unwrap_or(inner);
            inner.trim().to_string()
        }

        fn escape_amp_preserving_entities(raw: &str) -> String {
            fn is_valid_entity(entity: &str) -> bool {
                if entity.is_empty() {
                    return false;
                }
                if let Some(hex) = entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                {
                    return !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit());
                }
                if let Some(dec) = entity.strip_prefix('#') {
                    return !dec.is_empty() && dec.chars().all(|c| c.is_ascii_digit());
                }
                let mut it = entity.chars();
                let Some(first) = it.next() else {
                    return false;
                };
                if !first.is_ascii_alphabetic() {
                    return false;
                }
                it.all(|c| c.is_ascii_alphanumeric())
            }

            let mut out = String::with_capacity(raw.len());
            let mut i = 0usize;
            while let Some(rel) = raw[i..].find('&') {
                let amp = i + rel;
                out.push_str(&raw[i..amp]);
                let tail = &raw[amp + 1..];
                if let Some(semi_rel) = tail.find(';') {
                    let semi = amp + 1 + semi_rel;
                    let entity = &raw[amp + 1..semi];
                    if is_valid_entity(entity) {
                        out.push_str(&raw[amp..=semi]);
                        i = semi + 1;
                        continue;
                    }
                }
                out.push_str("&amp;");
                i = amp + 1;
            }
            out.push_str(&raw[i..]);
            out
        }

        if label_type == "markdown" {
            if is_simple_markdown(text) {
                let mut html_out = String::with_capacity(text.len() + 7);
                html_out.push_str("<p>");
                html_out.push_str(text);
                html_out.push_str("</p>");
                let html_out = crate::text::replace_fontawesome_icons(&html_out);
                let html_out = decode_mermaid_entities_for_render_text(&html_out);
                out.push_str(&escape_amp_preserving_entities(html_out.as_ref()));
            } else {
                let html = markdown_to_sanitized_xhtml(text, config);
                let html = decode_mermaid_entities_for_render_text(&html);
                out.push_str(&escape_amp_preserving_entities(html.as_ref()));
            }
        } else if text.contains('\n') || text.contains('\r') {
            // Mermaid's Cypress mindmap fixtures include multi-line labels inside node delimiters
            // (e.g. `root((\n  The root\n))`). Upstream preserves the raw whitespace/newlines as
            // a text node (no `<p>...</p>` wrapper) unless the label intentionally includes a
            // backtick snippet (which upstream keeps inside a `<p>` node).
            if text.contains('`') {
                let mut normalized;
                let normalized = if text.contains("<br>") || text.contains("<br/>") {
                    normalized = String::with_capacity(text.len() + 8);
                    push_br_normalized_text_into(&mut normalized, text);
                    normalized.as_str()
                } else {
                    text
                };
                out.push_str("<p>");
                out.push_str(&escape_xml(normalized));
                out.push_str("</p>");
            } else {
                out.push_str(&escape_xml(text));
            }
        } else {
            // Mermaid applies Markdown parsing semantics even for regular, single-line mindmap
            // labels. This matters for emphasis markers like `__proto__` (renders as `<strong>`).
            // Keep output XHTML-compatible and sanitizer-aligned.
            let mut normalized;
            let text = if text.contains("<br>") || text.contains("<br/>") {
                normalized = String::with_capacity(text.len() + 8);
                push_br_normalized_text_into(&mut normalized, text);
                normalized.as_str()
            } else {
                text
            };
            // Mindmap fixtures use *wrapping* backticks to denote "verbatim" labels. Mermaid keeps
            // those backticks as literal text (no Markdown evaluation) in that mode.
            //
            // Do not treat the presence of any backtick as verbatim. Upstream Mermaid's
            // `encodeEntities(...)` pass can introduce `&`-prefixed backticks (e.g. `&#96;` ->
            // `&ﬂ°°96¶ß` -> `&\``), and those should still participate in Markdown parsing.
            let trimmed = text.trim();
            let is_verbatim =
                trimmed.len() >= 2 && trimmed.starts_with('`') && trimmed.ends_with('`');
            if is_verbatim {
                out.push_str("<p>");
                out.push_str(&escape_xml(text));
                out.push_str("</p>");
            } else if is_simple_markdown(text) {
                let mut html_out = String::with_capacity(text.len() + 7);
                html_out.push_str("<p>");
                html_out.push_str(text);
                html_out.push_str("</p>");
                let html_out = crate::text::replace_fontawesome_icons(&html_out);
                let html_out = decode_mermaid_entities_for_render_text(&html_out);
                out.push_str(&escape_amp_preserving_entities(html_out.as_ref()));
            } else {
                let html = markdown_to_sanitized_xhtml(text, config);
                if is_single_img_fragment(&html) {
                    let html = unwrap_single_img_p(&html);
                    let html = decode_mermaid_entities_for_render_text(&html);
                    out.push_str(&escape_amp_preserving_entities(html.as_ref()));
                } else {
                    let html = decode_mermaid_entities_for_render_text(&html);
                    out.push_str(&escape_amp_preserving_entities(html.as_ref()));
                }
            }
        }

        out.push_str("</span></div></foreignObject></g>");
    }

    fn mk_edge_label(out: &mut String, edge_id: &str) {
        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{id}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
            id = escape_xml(edge_id),
        );
    }

    let _g_build_ctx = section(timing_enabled, &mut timings.build_ctx);

    let diagram_id = options.diagram_id.as_deref().unwrap_or("mindmap");
    let diagram_id_esc = escape_xml(diagram_id);

    let mut node_by_id: std::collections::BTreeMap<String, &crate::model::LayoutNode> =
        std::collections::BTreeMap::new();
    for n in &layout.nodes {
        node_by_id.insert(n.id.clone(), n);
    }

    drop(_g_build_ctx);

    let _g_viewbox = section(timing_enabled, &mut timings.viewbox);

    let padding = 10.0;
    let viewport_bounds =
        mindmap_viewport_bounds_from_layout(layout, model).or_else(|| layout.bounds.clone());
    let (vx, vy, vw, vh) = viewport_bounds
        .as_ref()
        .map(|b| {
            let w = (b.max_x - b.min_x).max(0.0);
            let h = (b.max_y - b.min_y).max(0.0);
            (
                b.min_x - padding,
                b.min_y - padding,
                w + 2.0 * padding,
                h + 2.0 * padding,
            )
        })
        .unwrap_or((0.0, 0.0, 100.0, 100.0));

    let mut view_box_attr = format!("{} {} {} {}", fmt(vx), fmt(vy), fmt(vw), fmt(vh));
    let mut max_w_attr = fmt_max_width_px(vw);
    let mut w_attr = fmt_string(vw);
    let mut h_attr = fmt_string(vh);
    apply_root_viewport_override(
        diagram_id,
        &mut view_box_attr,
        &mut w_attr,
        &mut h_attr,
        &mut max_w_attr,
        crate::generated::mindmap_root_overrides_11_12_2::lookup_mindmap_root_viewport_override,
    );

    drop(_g_viewbox);

    let _g_render_svg = section(timing_enabled, &mut timings.render_svg);

    let mut out = String::new();
    let style_attr = format!("max-width: {max_w_attr}px; background-color: white;");
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            class: Some("mindmapDiagram"),
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(view_box_attr.as_str()),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "mindmap")
        },
    );
    let css = mindmap_css(diagram_id, config.as_value());
    let _ = write!(&mut out, "<style>{}</style>", css);
    out.push_str(&mindmap_gradient_defs(diagram_id, config.as_value()));
    out.push_str("<g>");

    let _ = write!(
        &mut out,
        r#"<marker id="{id}_mindmap-pointEnd" class="marker mindmap" viewBox="0 0 10 10" refX="5" refY="5" markerUnits="userSpaceOnUse" markerWidth="8" markerHeight="8" orient="auto"><path d="M 0 0 L 10 5 L 0 10 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id = diagram_id_esc
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{id}_mindmap-pointStart" class="marker mindmap" viewBox="0 0 10 10" refX="4.5" refY="5" markerUnits="userSpaceOnUse" markerWidth="8" markerHeight="8" orient="auto"><path d="M 0 5 L 10 10 L 10 0 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        id = diagram_id_esc
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{id}_mindmap-pointEnd-margin" class="marker mindmap" viewBox="0 0 11.5 14" refX="11.5" refY="7" markerUnits="userSpaceOnUse" markerWidth="10.5" markerHeight="14" orient="auto"><path d="M 0 0 L 11.5 7 L 0 14 z" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id = diagram_id_esc
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{id}_mindmap-pointStart-margin" class="marker mindmap" viewBox="0 0 11.5 14" refX="1" refY="7" markerUnits="userSpaceOnUse" markerWidth="11.5" markerHeight="14" orient="auto"><polygon points="0,7 11.5,14 11.5,0" class="arrowMarkerPath" style="stroke-width: 0; stroke-dasharray: 1, 0;"/></marker>"#,
        id = diagram_id_esc
    );

    out.push_str(r#"<g class="subgraphs"/>"#);

    out.push_str(r#"<g class="edgePaths">"#);
    for e in &model.edges {
        let (sx, sy, tx, ty) = match (node_by_id.get(&e.start), node_by_id.get(&e.end)) {
            (Some(a), Some(b)) => (a.x, a.y, b.x, b.y),
            _ => (0.0, 0.0, 0.0, 0.0),
        };

        // Mermaid mindmap edges use `curveBasis` and offset endpoints from node centers
        // along the direction of the edge.
        let (vx, vy) = (tx - sx, ty - sy);
        let v_len = (vx * vx + vy * vy).sqrt();
        let (ux, uy) = if v_len == 0.0 {
            (0.0, 0.0)
        } else {
            (vx / v_len, vy / v_len)
        };
        let endpoint_offset = 15.0;
        let start_x = sx + endpoint_offset * ux;
        let start_y = sy + endpoint_offset * uy;
        let end_x = tx - endpoint_offset * ux;
        let end_y = ty - endpoint_offset * uy;
        let mid_x = (start_x + end_x) / 2.0;
        let mid_y = (start_y + end_y) / 2.0;

        let points = [
            Pt {
                x: start_x,
                y: start_y,
            },
            Pt { x: mid_x, y: mid_y },
            Pt { x: end_x, y: end_y },
        ];
        let points_for_data_points = points
            .iter()
            .map(|p| crate::model::LayoutPoint { x: p.x, y: p.y })
            .collect::<Vec<_>>();
        let data_points = base64::engine::general_purpose::STANDARD
            .encode(json_stringify_points(&points_for_data_points));

        let d = if e.curve.trim() == "basis" {
            curve::curve_basis_path_d(&points_for_data_points)
        } else {
            curve::curve_linear_path_d(&points_for_data_points)
        };
        let edge_classes = mindmap_normalize_section_classes(&e.classes);
        let class = format!(
            "edge-thickness-{} edge-pattern-solid {}",
            e.thickness.trim(),
            edge_classes.trim()
        );
        let data_look_attr = mindmap_data_look_attr(&e.look, config);
        let edge_dom_id = mindmap_dom_id(diagram_id, &e.id);
        let _ = write!(
            &mut out,
            r#"<path d="{d}" id="{dom_id}" class="{class}"{look_attr} data-edge="true" data-et="edge" data-id="{id}" data-points="{pts}"/>"#,
            d = escape_attr(&d),
            dom_id = escape_xml(&edge_dom_id),
            class = escape_xml(&class),
            look_attr = data_look_attr,
            id = escape_xml(&e.id),
            pts = escape_xml(&data_points),
        );
    }
    out.push_str("</g>");

    out.push_str(r#"<g class="edgeLabels">"#);
    for e in &model.edges {
        mk_edge_label(&mut out, &e.id);
    }
    out.push_str("</g>");

    out.push_str(r#"<g class="nodes">"#);
    for n in &model.nodes {
        let (x, y, w, h, label_w, label_h) = node_by_id
            .get(&n.id)
            .map(|ln| {
                (
                    ln.x,
                    ln.y,
                    ln.width,
                    ln.height,
                    ln.label_width,
                    ln.label_height,
                )
            })
            .unwrap_or((0.0, 0.0, 80.0, 44.0, None, None));
        let padding = n.padding.max(0.0);
        let half_padding = padding / 2.0;
        let node_classes = mindmap_normalize_section_classes(&n.css_classes);
        let class = format!("node {}", node_classes.trim());
        let data_look_attr = mindmap_data_look_attr(&n.look, config);
        let node_dom_id = mindmap_dom_id(diagram_id, &n.dom_id);
        let _ = write!(
            &mut out,
            r#"<g class="{class}" id="{dom_id}"{look_attr} transform="translate({x}, {y})">"#,
            class = escape_xml(&class),
            dom_id = escape_xml(&node_dom_id),
            look_attr = data_look_attr,
            x = fmt(x),
            y = fmt(y),
        );

        match n.shape.as_str() {
            "defaultMindmapNode" => {
                let rd = 5.0;
                let rect_path = format!(
                    "\n    M{} {}\n    v{}\n    q0,-{} {},-{}\n    h{}\n    q{},0 {},{}\n    v{}\n    q0,{} -{},{}\n    h{}\n    q-{},0 -{},-{}\n    Z\n  ",
                    fmt_path(-(w / 2.0)),
                    fmt_path(h / 2.0 - rd),
                    fmt_path(-h + 2.0 * rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(w - 2.0 * rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(h - 2.0 * rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(-w + 2.0 * rd),
                    fmt_path(rd),
                    fmt_path(rd),
                    fmt_path(rd),
                );

                // Recover label bbox dimensions from the rendered node size + padding rules.
                let bbox_w = (w - 8.0 * half_padding).max(1.0);
                let bbox_h = (h - 2.0 * half_padding).max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<path id="{id}" class="node-bkg node-0" style="" d="{d}"/>"#,
                    id = escape_xml(&node_dom_id),
                    d = escape_attr(&rect_path),
                );
                let _ = write!(
                    &mut out,
                    r#"<line class="node-line-" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = fmt(-(w / 2.0)),
                    x2 = fmt(w / 2.0),
                    y = fmt(h / 2.0),
                );
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "rect" => {
                // `rect` mindmap nodes use: w = bbox_w + 2*padding, h = bbox_h + padding.
                let bbox_w = (w - 2.0 * padding).max(1.0);
                let bbox_h = (h - padding).max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic label-container" style="" x="{x}" y="{y}" width="{w}" height="{h}"/>"#,
                    x = fmt(-(w / 2.0)),
                    y = fmt(-(h / 2.0)),
                    w = fmt(w.max(1.0)),
                    h = fmt(h.max(1.0)),
                );
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "rounded" => {
                let w = w.max(1.0);
                let h = h.max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic label-container" style="" rx="5" ry="5" x="{x}" y="{y}" width="{w}" height="{h}"/>"#,
                    x = fmt(-(w / 2.0)),
                    y = fmt(-(h / 2.0)),
                    w = fmt(w),
                    h = fmt(h),
                );

                let bbox_w = label_w.unwrap_or_else(|| (w - 2.0 * padding).max(1.0));
                let bbox_h = label_h.unwrap_or_else(|| (h - 2.0 * padding).max(1.0));
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "mindmapCircle" => {
                let r = (w.max(h) / 2.0).max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<circle class="basic label-container" style="" r="{r}" cx="0" cy="0"/>"#,
                    r = fmt(r),
                );
                // Mermaid sizes the circle diameter using `bbox.width`, but label placement still
                // uses the true label bbox height (not a square).
                let bbox_w = label_w.unwrap_or_else(|| (w - 2.0 * padding).max(1.0));
                let bbox_h = label_h.unwrap_or_else(|| (h - 2.0 * padding).max(1.0));
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "cloud" => {
                let bbox_w = label_w.unwrap_or_else(|| (w - 2.0 * half_padding).max(1.0));
                let bbox_h = label_h.unwrap_or_else(|| (h - 2.0 * half_padding).max(1.0));
                let w = (bbox_w + 2.0 * half_padding).max(1.0);
                let h = (bbox_h + 2.0 * half_padding).max(1.0);

                let cloud_path = mindmap_cloud_path_d(w, h, MindmapPathNumberFormat::D3Path);

                let _ = write!(
                    &mut out,
                    r#"<path class="basic label-container" style="" d="{d}" transform="translate({tx}, {ty})"/>"#,
                    d = escape_attr(&cloud_path),
                    tx = fmt(-(w / 2.0)),
                    ty = fmt(-(h / 2.0)),
                );
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "hexagon" => {
                let w = w.max(1.0);
                let h = h.max(1.0);
                let fixed_length = h / 4.0;
                let points = format!(
                    "{},0 {},0 {},{} {},{} {},{} 0,{}",
                    fmt_string(fixed_length),
                    fmt_string(w - fixed_length),
                    fmt_string(w),
                    fmt_string(-h / 2.0),
                    fmt_string(w - fixed_length),
                    fmt_string(-h),
                    fmt_string(fixed_length),
                    fmt_string(-h),
                    fmt_string(-h / 2.0),
                );
                let _ = write!(
                    &mut out,
                    r#"<polygon points="{points}" class="label-container" transform="translate({tx},{ty})"/>"#,
                    points = escape_attr(&points),
                    tx = fmt(-(w / 2.0)),
                    ty = fmt(h / 2.0),
                );
                let label_width = label_w.unwrap_or_else(|| w.max(1.0));
                let label_height = label_h.unwrap_or_else(|| h.max(1.0));
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: label_width,
                        height: label_height,
                        tx: -label_width / 2.0,
                        ty: -label_height / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            "bang" => {
                let bbox_w = label_w.unwrap_or_else(|| (w - 10.0 * half_padding).max(1.0));
                let bbox_h = label_h.unwrap_or_else(|| (h - 8.0 * half_padding).max(1.0));

                let w_base = bbox_w + 10.0 * half_padding;
                let effective_w = w.max(1.0);
                let effective_h = h.max(1.0);

                let bang_path = mindmap_bang_path_d(
                    w_base,
                    effective_w,
                    effective_h,
                    MindmapPathNumberFormat::D3Path,
                );

                let _ = write!(
                    &mut out,
                    r#"<path class="basic label-container" style="" d="{d}" transform="translate({tx}, {ty})"/>"#,
                    d = escape_attr(&bang_path),
                    tx = fmt(-(effective_w / 2.0)),
                    ty = fmt(-(effective_h / 2.0)),
                );
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: bbox_w,
                        height: bbox_h,
                        tx: -bbox_w / 2.0,
                        ty: -bbox_h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
            _ => {
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic label-container" style="" x="{x}" y="{y}" width="{w}" height="{h}"/>"#,
                    x = fmt(-(w / 2.0)),
                    y = fmt(-(h / 2.0)),
                    w = fmt(w.max(1.0)),
                    h = fmt(h.max(1.0)),
                );
                mk_label(
                    &mut out,
                    MindmapLabelSpec {
                        text: &n.label,
                        label_type: &n.label_type,
                        label_bkg: n.icon.is_some(),
                        width: w.max(1.0),
                        height: h.max(1.0),
                        tx: -w / 2.0,
                        ty: -h / 2.0,
                        max_node_width_px,
                    },
                    config,
                );
            }
        }

        out.push_str("</g>");
    }
    out.push_str("</g>");

    out.push_str("</g>");
    push_mindmap_shadow_defs(&mut out, diagram_id, config.as_value());
    out.push_str("</svg>\n");

    drop(_g_render_svg);

    timings.total = total_start.elapsed();
    if timing_enabled {
        eprintln!(
            "[render-timing] diagram=mindmap total={:?} deserialize={:?} build_ctx={:?} viewbox={:?} render_svg={:?} finalize={:?} nodes={} edges={}",
            timings.total,
            timings.deserialize_model,
            timings.build_ctx,
            timings.viewbox,
            timings.render_svg,
            timings.finalize_svg,
            model.nodes.len(),
            model.edges.len(),
        );
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mindmap_css_honors_mermaid_11_15_theme_sections() {
        let cfg = serde_json::json!({
            "theme": "redux",
            "look": "neo",
            "themeVariables": {
                "THEME_COLOR_LIMIT": 3,
                "cScale0": "#101010",
                "cScaleLabel0": "#f0f0f0",
                "cScaleInv0": "#202020",
                "cScale1": "#303030",
                "cScaleLabel1": "#404040",
                "cScaleInv1": "#505050",
                "cScale2": "#606060",
                "cScaleLabel2": "#707070",
                "cScaleInv2": "#808080",
                "git0": "#909090",
                "gitBranchLabel0": "#a0a0a0",
                "nodeBorder": "#b0b0b0"
            }
        });

        let css = mindmap_css("mm", &cfg);

        assert!(css.contains(r#"#mm .section--1 rect,#mm .section--1 path,#mm .section--1 circle,#mm .section--1 polygon,#mm .section--1 path{fill:#101010;}"#));
        assert!(css.contains(r#"#mm .section--1 span{color:#f0f0f0;}"#));
        assert!(css.contains(r#"#mm .section-0 span{color:#404040;}"#));
        assert!(css.contains(r#"#mm .section-1 line{stroke:#808080;stroke-width:3;}"#));
        assert!(css.contains(r#"#mm .edge-depth--1{stroke-width:12;}"#));
        assert!(css.contains(r#"#mm .edge-depth-0{stroke-width:10;}"#));
        assert!(css.contains(r#"#mm .section-root rect,#mm .section-root path,#mm .section-root circle,#mm .section-root polygon{fill:#909090;}"#));
        assert!(css.contains(r#"#mm .section-root text{fill:#a0a0a0;}"#));
        assert!(css.contains(r#"#mm .section-root span{color:#b0b0b0;}"#));
    }

    #[test]
    fn viewport_bounds_include_cloud_path_bbox() {
        let layout = MindmapDiagramLayout {
            nodes: vec![LayoutNode {
                id: "0".to_string(),
                x: 63.953125,
                y: 32.0,
                width: 97.90625,
                height: 34.0,
                is_cluster: false,
                label_width: Some(87.90625),
                label_height: Some(24.0),
            }],
            edges: Vec::new(),
            bounds: Some(Bounds {
                min_x: 15.0,
                min_y: 15.0,
                max_x: 112.90625,
                max_y: 49.0,
            }),
        };
        let model = merman_core::diagrams::mindmap::MindmapDiagramRenderModel {
            nodes: vec![merman_core::diagrams::mindmap::MindmapDiagramRenderNode {
                id: "0".to_string(),
                dom_id: "node_0".to_string(),
                label: "I am a cloud".to_string(),
                label_type: String::new(),
                is_group: false,
                shape: "cloud".to_string(),
                width: 0.0,
                height: 0.0,
                padding: 10.0,
                css_classes: "mindmap-node section-root section--1".to_string(),
                css_styles: Vec::new(),
                look: String::new(),
                icon: None,
                x: None,
                y: None,
                level: 0,
                node_id: "id".to_string(),
                node_type: -1,
                section: Some(-1),
            }],
            edges: Vec::new(),
        };

        let bounds = mindmap_viewport_bounds_from_layout(&layout, &model).expect("bounds");

        assert!(bounds.min_x < 15.0);
        assert!(bounds.min_y < 15.0);
        assert!(bounds.max_x > 112.90625);
        assert!(bounds.max_y > 49.0);
    }
}
