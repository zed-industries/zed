use super::*;

pub(super) struct ClassRenderSettings {
    pub(super) diagram_use_html_labels: bool,
    pub(super) edge_use_html_labels: bool,
    pub(super) font_size: f64,
    pub(super) font_size_css: String,
    pub(super) wrap_probe_font_size: f64,
    pub(super) html_calc_text_style: TextStyle,
    pub(super) line_height: f64,
    pub(super) class_padding: f64,
    pub(super) text_style: TextStyle,
    pub(super) viewport_padding: f64,
    pub(super) hide_empty_members_box: bool,
    pub(super) default_node_fill: String,
    pub(super) default_node_stroke: String,
    pub(super) look: String,
}

impl ClassRenderSettings {
    pub(super) fn from_config(effective_config: &serde_json::Value) -> Self {
        let diagram_use_html_labels =
            config_bool(effective_config, &["htmlLabels"]).unwrap_or(true);
        let edge_use_html_labels = config_bool(effective_config, &["htmlLabels"])
            .or_else(|| config_bool(effective_config, &["flowchart", "htmlLabels"]))
            .unwrap_or(true);
        let font_size = if diagram_use_html_labels {
            // Mermaid class diagram labels are rendered via HTML `<foreignObject>`. Mermaid CLI
            // baselines show that those HTML labels do not reliably inherit the surrounding SVG-root
            // `font-size` rules, so they effectively render at the browser default (16px) even when
            // users override `fontSize` / `themeVariables.fontSize`.
            16.0
        } else {
            // Mermaid injects `themeVariables.fontSize` into CSS as `font-size: ${fontSize};`
            // without forcing a unit. Unitless values are emitted into upstream SVGs but do not
            // affect browser text sizing; a value like `"24px"` does. Keep the raw CSS spelling
            // separately for emitted CSS parity.
            crate::config::config_f64_explicit_css_px(
                effective_config,
                &["themeVariables", "fontSize"],
            )
            .unwrap_or(16.0)
        }
        .max(1.0);
        let font_size_css = crate::config::config_css_number_or_string(
            effective_config,
            &["themeVariables", "fontSize"],
        )
        .unwrap_or_else(|| "16px".to_string());
        let wrap_probe_font_size = config_f64(effective_config, &["fontSize"])
            .unwrap_or(16.0)
            .max(1.0);
        let html_calc_text_style = crate::class::class_html_calculate_text_style(effective_config);
        let line_height = font_size * 1.5;
        let class_padding = effective_config
            .get("class")
            .and_then(|v| v.get("padding"))
            .and_then(|v| v.as_f64())
            .unwrap_or(12.0)
            .max(0.0);
        let text_style = TextStyle {
            font_family: config_string(effective_config, &["fontFamily"])
                .or_else(|| config_string(effective_config, &["themeVariables", "fontFamily"]))
                .or_else(|| Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string())),
            font_size,
            font_weight: None,
        };

        // Mermaid's classRenderer-v2 reads `flowchart ?? class` for diagram padding.
        let conf = effective_config
            .get("flowchart")
            .or_else(|| effective_config.get("class"))
            .unwrap_or(effective_config);
        let viewport_padding = config_f64(conf, &["diagramPadding"])
            .unwrap_or(8.0)
            .max(0.0);
        let hide_empty_members_box =
            config_bool(effective_config, &["class", "hideEmptyMembersBox"]).unwrap_or(false);
        let default_node_fill = config_string(effective_config, &["themeVariables", "mainBkg"])
            .or_else(|| config_string(effective_config, &["themeVariables", "primaryColor"]))
            .unwrap_or_else(|| "#ECECFF".to_string());
        let default_node_stroke =
            config_string(effective_config, &["themeVariables", "nodeBorder"])
                .or_else(|| {
                    config_string(effective_config, &["themeVariables", "primaryBorderColor"])
                })
                .unwrap_or_else(|| "#9370DB".to_string());
        let look = config_diagram_look(effective_config).as_str().to_string();

        Self {
            diagram_use_html_labels,
            edge_use_html_labels,
            font_size,
            font_size_css,
            wrap_probe_font_size,
            html_calc_text_style,
            line_height,
            class_padding,
            text_style,
            viewport_padding,
            hide_empty_members_box,
            default_node_fill,
            default_node_stroke,
            look,
        }
    }
}
