//! Flowchart render configuration preparation.

use crate::text::{TextStyle, WrapMode};

use super::super::{config_f64, config_string, normalize_css_font_family, theme_color};

pub(in crate::svg::parity::flowchart) struct FlowchartRenderConfig {
    pub font_family: String,
    pub font_size: f64,
    pub wrapping_width: f64,
    pub node_html_labels: bool,
    pub edge_html_labels: bool,
    pub node_wrap_mode: WrapMode,
    pub edge_wrap_mode: WrapMode,
    pub diagram_padding: f64,
    pub use_max_width: bool,
    pub title_top_margin: f64,
    pub node_padding: f64,
    pub text_style: TextStyle,
    pub html_label_text_style: TextStyle,
    pub default_edge_interpolate: String,
    pub default_edge_style: Vec<String>,
    pub node_border_color: String,
    pub node_fill_color: String,
}

pub(in crate::svg::parity::flowchart) fn prepare_flowchart_render_config(
    model: &crate::flowchart::FlowchartV2Model,
    effective_config_value: &serde_json::Value,
) -> FlowchartRenderConfig {
    let default_theme_font_family = "\"trebuchet ms\",verdana,arial,sans-serif".to_string();
    let theme_font_family =
        config_string(effective_config_value, &["themeVariables", "fontFamily"])
            .map(|s| normalize_css_font_family(&s));
    let top_font_family = config_string(effective_config_value, &["fontFamily"])
        .map(|s| normalize_css_font_family(&s));
    let font_family = match (top_font_family, theme_font_family) {
        (Some(top), Some(theme)) if theme == default_theme_font_family => top,
        (_, Some(theme)) => theme,
        (Some(top), None) => top,
        (None, None) => default_theme_font_family,
    };
    let font_size = effective_config_value
        .get("themeVariables")
        .and_then(|tv| tv.get("fontSize"))
        .and_then(parse_font_size_px)
        .unwrap_or(16.0)
        .max(1.0);

    let wrapping_width = config_f64(effective_config_value, &["flowchart", "wrappingWidth"])
        .unwrap_or(200.0)
        .max(1.0);
    let node_html_labels =
        crate::flowchart::flowchart_effective_node_html_labels(effective_config_value);
    let flowchart_html_labels =
        crate::flowchart::flowchart_effective_html_labels(effective_config_value);
    let edge_html_labels = flowchart_html_labels;
    let node_wrap_mode = if node_html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    let edge_wrap_mode = if edge_html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    let diagram_padding = config_f64(effective_config_value, &["flowchart", "diagramPadding"])
        .unwrap_or(8.0)
        .max(0.0);
    let use_max_width = effective_config_value
        .get("flowchart")
        .and_then(|v| v.get("useMaxWidth"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let title_top_margin = config_f64(effective_config_value, &["flowchart", "titleTopMargin"])
        .unwrap_or(25.0)
        .max(0.0);
    let node_padding = config_f64(effective_config_value, &["flowchart", "padding"])
        .unwrap_or(15.0)
        .max(0.0);

    let text_style = TextStyle {
        font_family: Some(font_family.clone()),
        font_size,
        font_weight: None,
    };
    let html_label_text_style = crate::flowchart::flowchart_html_label_measurement_base_style(
        &text_style,
        effective_config_value,
    );

    let cfg_curve = config_string(effective_config_value, &["flowchart", "curve"]);
    let default_edge_interpolate = model
        .edge_defaults
        .as_ref()
        .and_then(|d| d.interpolate.as_deref())
        .or(cfg_curve.as_deref())
        .unwrap_or("basis")
        .to_string();
    let default_edge_style = model
        .edge_defaults
        .as_ref()
        .map(|d| d.style.clone())
        .unwrap_or_default();

    let node_border_color = theme_color(effective_config_value, "nodeBorder", "#9370DB");
    let node_fill_color = theme_color(effective_config_value, "mainBkg", "#ECECFF");

    FlowchartRenderConfig {
        font_family,
        font_size,
        wrapping_width,
        node_html_labels,
        edge_html_labels,
        node_wrap_mode,
        edge_wrap_mode,
        diagram_padding,
        use_max_width,
        title_top_margin,
        node_padding,
        text_style,
        html_label_text_style,
        default_edge_interpolate,
        default_edge_style,
        node_border_color,
        node_fill_color,
    }
}

fn parse_font_size_px(v: &serde_json::Value) -> Option<f64> {
    if let Some(n) = v.as_f64() {
        return Some(n);
    }
    if let Some(n) = v.as_i64() {
        return Some(n as f64);
    }
    if let Some(n) = v.as_u64() {
        return Some(n as f64);
    }
    let s = v.as_str()?.trim();
    if s.is_empty() {
        return None;
    }
    let mut num = String::new();
    for (idx, ch) in s.chars().enumerate() {
        if ch.is_ascii_digit() {
            num.push(ch);
            continue;
        }
        if idx == 0 && (ch == '-' || ch == '+') {
            num.push(ch);
            continue;
        }
        break;
    }
    if num.trim().is_empty() {
        return None;
    }
    num.parse::<f64>().ok()
}
