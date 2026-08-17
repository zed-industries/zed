use super::*;

#[derive(Debug, Clone)]
pub(super) struct StateThemeDefaults {
    pub(super) background: String,
    pub(super) main_bkg: String,
    pub(super) state_bkg: String,
    pub(super) state_border: String,
    pub(super) stroke_width: String,
    pub(super) stroke_width_px: String,
    pub(super) rough_stroke_width_value: f64,
    pub(super) special_state_color: String,
    pub(super) inner_end_background: String,
    pub(super) end_outer_fill: String,
    pub(super) end_outer_stroke: String,
    pub(super) end_inner_stroke: String,
    pub(super) note_bkg: String,
    pub(super) note_border: String,
}

impl StateThemeDefaults {
    pub(super) fn from_config(effective_config: &serde_json::Value) -> Self {
        let theme = PresentationTheme::new(effective_config).state_diagram();

        Self {
            background: theme.background,
            main_bkg: theme.main_bkg,
            state_bkg: theme.state_bkg,
            state_border: theme.state_border,
            stroke_width: theme.stroke_width,
            stroke_width_px: theme.stroke_width_px,
            rough_stroke_width_value: theme.rough_stroke_width_value,
            special_state_color: theme.special_state_color,
            inner_end_background: theme.inner_end_background,
            end_outer_fill: theme.end_outer_fill,
            end_outer_stroke: theme.end_outer_stroke,
            end_inner_stroke: theme.end_inner_stroke,
            note_bkg: theme.note_bkg,
            note_border: theme.note_border,
        }
    }
}

fn state_shadow_defs(out: &mut String, diagram_id: &str, effective_config: &serde_json::Value) {
    let flood_color = PresentationTheme::new(effective_config)
        .common()
        .is_dark_theme()
        .then_some("#FFFFFF")
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

fn state_gradient_defs(out: &mut String, diagram_id: &str, effective_config: &serde_json::Value) {
    if !config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return;
    }

    let gradient_start = config_string(effective_config, &["themeVariables", "gradientStart"])
        .or_else(|| config_string(effective_config, &["themeVariables", "primaryBorderColor"]))
        .unwrap_or_else(|| "#9370DB".to_string());
    let gradient_stop = config_string(effective_config, &["themeVariables", "gradientStop"])
        .or_else(|| {
            config_string(
                effective_config,
                &["themeVariables", "secondaryBorderColor"],
            )
        })
        .unwrap_or_else(|| gradient_start.clone());

    let diagram_id = escape_xml(diagram_id);
    let gradient_start = escape_xml(&gradient_start);
    let gradient_stop = escape_xml(&gradient_stop);
    let _ = write!(
        out,
        r#"<defs><linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient></defs>"#,
        diagram_id.as_str(),
        gradient_start.as_str(),
        gradient_stop.as_str()
    );
}

pub(super) fn state_markers(
    out: &mut String,
    diagram_id: &str,
    effective_config: &serde_json::Value,
) {
    let theme = PresentationTheme::new(effective_config).state_diagram();
    let diagram_id = escape_xml(diagram_id);
    let transition_color = theme.transition_color.as_str();

    if theme.common.is_neo() {
        let _ = write!(
            out,
            r#"<defs><marker id="{diagram_id}_stateDiagram-barbEnd" refX="19" refY="7" markerWidth="20" markerHeight="14" markerUnits="strokeWidth" orient="auto"><path d="M 19,7 L11,14 L13,7 L11,0 Z"/></marker></defs><defs><marker id="{diagram_id}_stateDiagram-barbEnd-margin" refX="17" refY="7" markerWidth="20" markerHeight="14" markerUnits="userSpaceOnUse" orient="auto"><path d="M 19,7 L11,14 L13,7 L11,0 Z" fill="{}"/></marker></defs>"#,
            escape_xml(&transition_color)
        );
        state_shadow_defs(out, diagram_id.as_str(), effective_config);
        state_gradient_defs(out, diagram_id.as_str(), effective_config);
    } else {
        let _ = write!(
            out,
            r#"<defs><marker id="{diagram_id}_stateDiagram-barbEnd" refX="19" refY="7" markerWidth="20" markerHeight="14" markerUnits="userSpaceOnUse" orient="auto"><path d="M 19,7 L9,13 L14,7 L9,1 Z"/></marker></defs>"#
        );
    }
}

pub(super) fn state_css(
    diagram_id: &str,
    model: &StateSvgModel,
    effective_config: &serde_json::Value,
) -> String {
    fn normalize_decl(s: &str) -> Option<(String, String)> {
        let s = s.trim().trim_end_matches(';').trim();
        if s.is_empty() {
            return None;
        }
        let (k, v) = s.split_once(':')?;
        let key = k.trim().to_string();
        let mut val = v.trim().to_string();
        // Mermaid emits class styles with `!important` (no spaces).
        if !val.to_lowercase().contains("!important") {
            val.push_str("!important");
        } else {
            val = val.replace(" !important", "!important");
        }
        Some((key, val))
    }

    fn class_decl_block(styles: &[String], text_styles: &[String]) -> String {
        let mut out = String::new();
        for raw in styles.iter().chain(text_styles.iter()) {
            let Some((k, v)) = normalize_decl(raw) else {
                continue;
            };
            // Mermaid tightens `prop: value` -> `prop:value`.
            let _ = write!(&mut out, "{}:{};", k, v);
        }
        out
    }

    fn should_duplicate_class_rules(styles: &[String], text_styles: &[String]) -> bool {
        let has_fontish = |s: &str| {
            let s = s.trim_start().to_lowercase();
            s.starts_with("font-") || s.starts_with("text-")
        };
        styles.iter().any(|s| has_fontish(s)) || text_styles.iter().any(|s| has_fontish(s))
    }

    let theme = PresentationTheme::new(effective_config).state_diagram();
    let ff = theme.common.font_family_css.as_str();
    let font_size = theme.common.font_size_px;
    let id = escape_xml(diagram_id);
    let text_color = theme.common.text_color.as_str();
    let error_bkg = theme.common.error_bkg.as_str();
    let error_text = theme.common.error_text.as_str();
    let line_color = theme.common.line_color.as_str();
    let transition_color = theme.transition_color.as_str();
    let node_border = theme.node_border.as_str();
    let state_label_color = theme.state_label_color.as_str();
    let defaults = StateThemeDefaults::from_config(effective_config);
    let main_bkg = &defaults.main_bkg;
    let background = &defaults.background;
    let alt_background = theme.alt_background.as_str();
    let stroke_width = &defaults.stroke_width;
    let stroke_width_px = &defaults.stroke_width_px;
    let note_border = &defaults.note_border;
    let note_bkg = &defaults.note_bkg;
    let note_text = theme.note_text.as_str();
    let label_background = theme.label_background.as_str();
    let edge_label_background = theme.edge_label_background.as_str();
    let transition_label_color = theme.transition_label_color.as_str();
    let special_state_color = &defaults.special_state_color;
    let inner_end_background = &defaults.inner_end_background;
    let composite_background = theme.composite_background.as_str();
    let state_bkg = &defaults.state_bkg;
    let state_border = &defaults.state_border;
    let composite_title_background = theme.composite_title_background.as_str();
    let use_gradient =
        config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false);
    let neo_cluster_stroke = if use_gradient {
        format!("url(#{diagram_id}-gradient)")
    } else {
        state_border.clone()
    };
    let neo_radius =
        crate::config::config_f64_css_px(effective_config, &["themeVariables", "radius"])
            .unwrap_or(5.0)
            .max(0.0);
    let neo_drop_shadow = theme.drop_shadow.replace(
        "url(#drop-shadow)",
        &format!("url(#{diagram_id}-drop-shadow)"),
    );

    // Mirrors Mermaid 11.15 `diagrams/state/styles.js` + shared base stylesheet ordering.
    let mut css = String::new();
    let font_size_s = fmt(font_size);
    let _ = write!(
        &mut css,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id, ff, font_size_s, text_color
    );
    css.push_str("@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}");
    css.push_str("@keyframes dash{to{stroke-dashoffset:0;}}");
    let _ = write!(
        &mut css,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id
    );
    let _ = write!(&mut css, r#"#{} .error-icon{{fill:{};}}"#, id, error_bkg);
    let _ = write!(
        &mut css,
        r#"#{} .error-text{{fill:{};stroke:{};}}"#,
        id, error_text, error_text
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-thickness-thick{{stroke-width:3.5px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-pattern-solid{{stroke-dasharray:0;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-pattern-dashed{{stroke-dasharray:3;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .marker{{fill:{};stroke:{};}}"#,
        id, line_color, line_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .marker.cross{{stroke:{};}}"#,
        id, line_color
    );
    let _ = write!(
        &mut css,
        r#"#{} svg{{font-family:{};font-size:{}px;}}"#,
        id, ff, font_size_s
    );
    let _ = write!(&mut css, r#"#{} p{{margin:0;}}"#, id);
    let _ = write!(
        &mut css,
        r#"#{} defs [id$="-barbEnd"]{{fill:{};stroke:{};}}"#,
        id, transition_color, transition_color
    );
    let _ = write!(
        &mut css,
        r#"#{} g.stateGroup text{{fill:{};stroke:none;font-size:10px;}}"#,
        id, node_border
    );
    let _ = write!(
        &mut css,
        r#"#{} g.stateGroup text{{fill:{};stroke:none;font-size:10px;}}"#,
        id, text_color
    );
    let _ = write!(
        &mut css,
        r#"#{} g.stateGroup .state-title{{font-weight:bolder;fill:{};}}"#,
        id, state_label_color
    );
    let _ = write!(
        &mut css,
        r#"#{} g.stateGroup rect{{fill:{};stroke:{};}}"#,
        id, main_bkg, node_border
    );
    let _ = write!(
        &mut css,
        r#"#{} g.stateGroup line{{stroke:{};stroke-width:{};}}"#,
        id, line_color, stroke_width
    );
    let _ = write!(
        &mut css,
        r#"#{} .transition{{stroke:{};stroke-width:{};fill:none;}}"#,
        id, transition_color, stroke_width
    );
    let _ = write!(
        &mut css,
        r#"#{} .stateGroup .composit{{fill:{};border-bottom:1px;}}"#,
        id, background
    );
    let _ = write!(
        &mut css,
        r#"#{} .stateGroup .alt-composit{{fill:#e0e0e0;border-bottom:1px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .state-note{{stroke:{};fill:{};}}"#,
        id, note_border, note_bkg
    );
    let _ = write!(
        &mut css,
        r#"#{} .state-note text{{fill:{};stroke:none;font-size:10px;}}"#,
        id, note_text
    );
    let _ = write!(
        &mut css,
        r#"#{} .stateLabel .box{{stroke:none;stroke-width:0;fill:{};opacity:0.5;}}"#,
        id, main_bkg
    );
    let _ = write!(
        &mut css,
        r#"#{} .edgeLabel .label rect{{fill:{};opacity:0.5;}}"#,
        id, label_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .edgeLabel{{background-color:{};text-align:center;}}"#,
        id, edge_label_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .edgeLabel p{{background-color:{};}}"#,
        id, edge_label_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .edgeLabel rect{{opacity:0.5;background-color:{};fill:{};}}"#,
        id, edge_label_background, edge_label_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .edgeLabel .label text{{fill:{};}}"#,
        id, transition_label_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .label div .edgeLabel{{color:{};}}"#,
        id, transition_label_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .stateLabel text{{fill:{};font-size:10px;font-weight:bold;}}"#,
        id, state_label_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .node circle.state-start{{fill:{};stroke:{};}}"#,
        id, special_state_color, special_state_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .node .fork-join{{fill:{};stroke:{};}}"#,
        id, special_state_color, special_state_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .node circle.state-end{{fill:{};stroke:{};stroke-width:1.5;}}"#,
        id, inner_end_background, background
    );
    let _ = write!(
        &mut css,
        r#"#{} .end-state-inner{{fill:{};stroke-width:1.5;}}"#,
        id, composite_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .node rect{{fill:{};stroke:{};stroke-width:{};}}"#,
        id, state_bkg, state_border, stroke_width_px
    );
    let _ = write!(
        &mut css,
        r#"#{} .node polygon{{fill:{};stroke:{};stroke-width:{};}}"#,
        id, main_bkg, state_border, stroke_width_px
    );
    let _ = write!(
        &mut css,
        r#"#{} [id$="-barbEnd"]{{fill:{};}}"#,
        id, line_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-cluster rect{{fill:{};stroke:{};stroke-width:{};}}"#,
        id, composite_title_background, state_border, stroke_width_px
    );
    let _ = write!(
        &mut css,
        r#"#{} .cluster-label,#{} .nodeLabel{{color:{};}}"#,
        id, id, state_label_color
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-cluster rect.outer{{rx:5px;ry:5px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-state .divider{{stroke:{};}}"#,
        id, state_border
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-state .title-state{{rx:5px;ry:5px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-cluster.statediagram-cluster .inner{{fill:{};}}"#,
        id, composite_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-cluster.statediagram-cluster-alt .inner{{fill:{};}}"#,
        id, alt_background
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-cluster .inner{{rx:0;ry:0;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-state rect.basic{{rx:5px;ry:5px;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-state rect.divider{{stroke-dasharray:10,10;fill:{};}}"#,
        id, alt_background
    );
    let _ = write!(&mut css, r#"#{} .note-edge{{stroke-dasharray:5;}}"#, id);
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-note rect{{fill:{};stroke:{};stroke-width:1px;rx:0;ry:0;}}"#,
        id, note_bkg, note_border
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-note rect{{fill:{};stroke:{};stroke-width:1px;rx:0;ry:0;}}"#,
        id, note_bkg, note_border
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-note text{{fill:{};}}"#,
        id, note_text
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram-note .nodeLabel{{color:{};}}"#,
        id, note_text
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagram .edgeLabel{{color:red;}}"#,
        id
    );
    let _ = write!(
        &mut css,
        r#"#{} .statediagramTitleText{{text-anchor:middle;font-size:18px;fill:{};}}"#,
        id, text_color
    );
    let _ = write!(
        &mut css,
        r#"#{} [data-look="neo"].statediagram-cluster rect{{fill:{};stroke:{};stroke-width:{};}}"#,
        id, main_bkg, neo_cluster_stroke, stroke_width
    );
    let _ = write!(
        &mut css,
        r#"#{} [data-look="neo"].statediagram-cluster rect.outer{{rx:{}px;ry:{}px;filter:{};}}"#,
        id,
        fmt(neo_radius),
        fmt(neo_radius),
        neo_drop_shadow
    );
    let _ = write!(
        &mut css,
        r#"#{} :root{{--mermaid-font-family:{};}}"#,
        id, ff
    );

    if !model.style_classes.is_empty() {
        // Mermaid keeps classDef ordering stable and appends each class as:
        //   `#id .class&gt;*{...}#id .class span{...}`
        for sc in model.style_classes.values() {
            let decls = class_decl_block(&sc.styles, &sc.text_styles);
            if decls.is_empty() {
                continue;
            }
            let repeats = if should_duplicate_class_rules(&sc.styles, &sc.text_styles) {
                2
            } else {
                1
            };
            for _ in 0..repeats {
                let _ = write!(
                    &mut css,
                    r#"#{} .{}&gt;*{{{}}}#{} .{} span{{{}}}"#,
                    id, sc.id, decls, id, sc.id, decls
                );
            }
        }
    }

    css
}

pub(super) fn state_value_to_label_text(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(a) => {
            let mut parts: Vec<&str> = Vec::new();
            for item in a {
                if let Some(s) = item.as_str() {
                    parts.push(s);
                }
            }
            if parts.is_empty() {
                return "".to_string();
            }
            parts.join("\n")
        }
        _ => "".to_string(),
    }
}

pub(super) fn state_node_label_text(n: &StateSvgNode) -> String {
    n.label
        .as_ref()
        .map(state_value_to_label_text)
        .unwrap_or_else(|| n.id.clone())
}

#[derive(Debug, Clone, Copy)]
pub(super) struct StateInlineDecl<'a> {
    pub(super) key: &'a str,
    pub(super) val: &'a str,
}

pub(super) fn state_parse_inline_decl(raw: &str) -> Option<StateInlineDecl<'_>> {
    let raw = raw.trim().trim_end_matches(';').trim();
    if raw.is_empty() {
        return None;
    }
    let (k, v) = raw.split_once(':')?;
    let key = k.trim();
    let val = v.trim();
    if key.is_empty() || val.is_empty() {
        return None;
    }
    Some(StateInlineDecl { key, val })
}

pub(super) fn state_is_text_style_key(key: &str) -> bool {
    let k = key.trim().to_ascii_lowercase();
    k == "color" || k.starts_with("font-") || k.starts_with("text-")
}

pub(super) fn state_compact_style_attr(decls: &[StateInlineDecl<'_>]) -> String {
    let mut out = String::new();
    for (idx, d) in decls.iter().enumerate() {
        if idx > 0 {
            out.push(';');
        }
        out.push_str(d.key.trim());
        out.push(':');
        out.push_str(d.val.trim());
        if !d.val.to_ascii_lowercase().contains("!important") {
            out.push_str(" !important");
        }
    }
    out
}

pub(super) fn state_div_style_prefix(decls: &[StateInlineDecl<'_>]) -> String {
    let mut out = String::new();
    for d in decls {
        out.push_str(d.key.trim());
        out.push_str(": ");
        out.push_str(d.val.trim());
        if !d.val.to_ascii_lowercase().contains("!important") {
            out.push_str(" !important");
        }
        out.push_str("; ");
    }
    out
}

pub(super) fn state_node_label_html_with_style(raw: &str, span_style: Option<&str>) -> String {
    let style_attr = span_style
        .filter(|s| !s.is_empty())
        .map(|s| format!(r#" style="{}""#, escape_xml_display(s)))
        .unwrap_or_default();
    format!(
        r#"<span{} class="nodeLabel">{}</span>"#,
        style_attr,
        html_paragraph_with_br(raw)
    )
}

fn state_is_valid_html_entity(entity: &str) -> bool {
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

fn state_escape_amp_preserving_entities(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;
    while let Some(rel) = raw[i..].find('&') {
        let amp = i + rel;
        out.push_str(&raw[i..amp]);
        let tail = &raw[amp + 1..];
        if let Some(semi_rel) = tail.find(';') {
            let semi = amp + 1 + semi_rel;
            let entity = &raw[amp + 1..semi];
            if state_is_valid_html_entity(entity) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn state_css_honors_mermaid_11_15_theme_options() {
        let cfg = json!({
            "themeVariables": {
                "fontFamily": "Inter, Arial",
                "textColor": "#101010",
                "errorBkgColor": "#111111",
                "errorTextColor": "#121212",
                "transitionColor": "#202020",
                "lineColor": "#303030",
                "nodeBorder": "#404040",
                "stateLabelColor": "#505050",
                "mainBkg": "#606060",
                "background": "#707070",
                "altBackground": "#808080",
                "strokeWidth": 4,
                "noteBorderColor": "#909090",
                "noteBkgColor": "#a0a0a0",
                "noteTextColor": "#b0b0b0",
                "labelBackgroundColor": "#c0c0c0",
                "edgeLabelBackground": "#d0d0d0",
                "transitionLabelColor": "#e0e0e0",
                "specialStateColor": "#f0f0f0",
                "innerEndBackground": "#010101",
                "compositeBackground": "#020202",
                "stateBkg": "#030303",
                "stateBorder": "#040404",
                "compositeTitleBackground": "#050505"
            }
        });

        let css = state_css("st", &StateSvgModel::default(), &cfg);

        assert!(css.contains(r#"#st{font-family:Inter,Arial;font-size:16px;fill:#101010;}"#));
        assert!(css.contains(
            r#"#st .error-icon{fill:#111111;}#st .error-text{fill:#121212;stroke:#121212;}"#
        ));
        assert!(css.contains(
            r#"#st .marker{fill:#303030;stroke:#303030;}#st .marker.cross{stroke:#303030;}"#
        ));
        assert!(css.contains(r#"#st defs [id$="-barbEnd"]{fill:#202020;stroke:#202020;}"#));
        assert!(css.contains(r#"#st g.stateGroup rect{fill:#606060;stroke:#404040;}"#));
        assert!(css.contains(r#"#st .transition{stroke:#202020;stroke-width:4;fill:none;}"#));
        assert!(css.contains(r#"#st .state-note{stroke:#909090;fill:#a0a0a0;}"#));
        assert!(css.contains(r#"#st .edgeLabel .label rect{fill:#c0c0c0;opacity:0.5;}"#));
        assert!(css.contains(r#"#st .edgeLabel{background-color:#d0d0d0;text-align:center;}"#));
        assert!(css.contains(r#"#st .edgeLabel .label text{fill:#e0e0e0;}"#));
        assert!(
            css.contains(r#"#st .stateLabel text{fill:#505050;font-size:10px;font-weight:bold;}"#)
        );
        assert!(css.contains(r#"#st .node circle.state-start{fill:#f0f0f0;stroke:#f0f0f0;}"#));
        assert!(css.contains(
            r#"#st .node circle.state-end{fill:#010101;stroke:#707070;stroke-width:1.5;}"#
        ));
        assert!(css.contains(r#"#st .node rect{fill:#030303;stroke:#040404;stroke-width:4px;}"#));
        assert!(css.contains(
            r#"#st .statediagram-cluster rect{fill:#050505;stroke:#040404;stroke-width:4px;}"#
        ));
        assert!(css.contains(r#"#st .statediagram-note text{fill:#b0b0b0;}"#));
        assert!(css.contains(
            r#"#st .statediagramTitleText{text-anchor:middle;font-size:18px;fill:#101010;}"#
        ));
        assert!(
            !css.contains("dependencyStart"),
            "local State SVG does not emit dependency markers, so CSS should not advertise them"
        );
    }

    #[test]
    fn state_css_emits_neo_cluster_theme_rules() {
        let cfg = json!({
            "look": "neo",
            "themeVariables": {
                "mainBkg": "#606060",
                "stateBorder": "#040404",
                "strokeWidth": 4,
                "useGradient": true,
                "gradientStart": "#112233",
                "gradientStop": "#445566",
                "dropShadow": "url(#drop-shadow)",
                "radius": 3
            }
        });

        let css = state_css("st", &StateSvgModel::default(), &cfg);

        assert!(css.contains(
            r##"#st [data-look="neo"].statediagram-cluster rect{fill:#606060;stroke:url(#st-gradient);stroke-width:4;}"##
        ));
        assert!(css.contains(
            r##"#st [data-look="neo"].statediagram-cluster rect.outer{rx:3px;ry:3px;filter:url(#st-drop-shadow);}"##
        ));
    }
}

fn state_normalize_br_tags(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut cur = 0usize;
    let mut i = 0usize;
    while i + 2 < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        let b1 = bytes[i + 1];
        let b2 = bytes[i + 2];
        if !matches!(b1, b'b' | b'B') || !matches!(b2, b'r' | b'R') {
            i += 1;
            continue;
        }
        let next = bytes.get(i + 3).copied();
        if let Some(n) = next {
            if !matches!(n, b'>' | b'/' | b' ' | b'\t' | b'\r' | b'\n') {
                i += 1;
                continue;
            }
        }
        if i > cur {
            out.push_str(&raw[cur..i]);
        }
        let Some(end_rel) = bytes[i..].iter().position(|&c| c == b'>') else {
            cur = i;
            break;
        };
        out.push('\n');
        i = i + end_rel + 1;
        cur = i;
    }
    if cur < raw.len() {
        out.push_str(&raw[cur..]);
    }
    out
}

fn write_state_html_lines_with_br(out: &mut String, normalized: &str) {
    for (idx, line) in normalized.split('\n').enumerate() {
        if idx > 0 {
            out.push_str("<br />");
        }
        // State diagram labels are sanitized upstream (entities + limited tags). Preserve entities
        // like `&lt;` without double-escaping, while still making stray `&` XML-safe.
        out.push_str(&state_escape_amp_preserving_entities(line));
    }
}

fn state_html_with_br(raw: &str, wrap_paragraph: bool) -> String {
    let decoded = crate::svg::parity::util::decode_mermaid_entities_for_render_text(raw);
    let normalized = state_normalize_br_tags(decoded.as_ref());
    let mut out = String::new();
    if wrap_paragraph {
        out.push_str("<p>");
    }
    write_state_html_lines_with_br(&mut out, &normalized);
    if wrap_paragraph {
        out.push_str("</p>");
    }
    out
}

fn html_paragraph_with_br(raw: &str) -> String {
    state_html_with_br(raw, true)
}

fn html_inline_with_br(raw: &str) -> String {
    state_html_with_br(raw, false)
}

pub(super) fn state_node_label_html(raw: &str) -> String {
    format!(
        r#"<span class="nodeLabel">{}</span>"#,
        html_paragraph_with_br(raw)
    )
}

pub(super) fn state_node_label_inline_html(raw: &str) -> String {
    format!(
        r#"<span class="nodeLabel">{}</span>"#,
        html_inline_with_br(raw)
    )
}

pub(super) fn state_edge_label_html(raw: &str) -> String {
    // Mermaid runs edge labels through its `markdownToHTML()` pipeline when `htmlLabels=true`.
    // Keep the XHTML fragment XML-safe while preserving inline HTML like `<br/>` and Mermaid's
    // minimal emphasis/strong subset.
    let decoded = crate::svg::parity::util::decode_mermaid_entities_for_render_text(raw);
    crate::text::mermaid_markdown_to_xhtml_label_fragment(decoded.as_ref(), true)
}
