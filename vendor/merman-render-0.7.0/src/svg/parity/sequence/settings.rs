use crate::text::TextStyle;

pub(super) struct SequenceRenderSettings {
    pub(super) force_menus: bool,
    pub(super) mirror_actors: bool,
    pub(super) diagram_margin_x: f64,
    pub(super) box_margin: f64,
    pub(super) actor_height: f64,
    pub(super) box_text_margin: f64,
    pub(super) message_align: String,
    pub(super) label_box_height: f64,
    pub(super) right_angles: bool,
    pub(super) wrap_padding: f64,
    pub(super) sequence_width: f64,
    pub(super) activation_width: f64,
    pub(super) actor_label_font_size: f64,
    pub(super) actor_wrap_width: f64,
    pub(super) loop_text_style: TextStyle,
    pub(super) note_text_style: TextStyle,
}

impl SequenceRenderSettings {
    pub(super) fn from_config(
        effective_config: &serde_json::Value,
        seq_cfg: &serde_json::Value,
    ) -> Self {
        let force_menus = seq_cfg
            .get("forceMenus")
            .or_else(|| effective_config.get("forceMenus"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mirror_actors = seq_cfg
            .get("mirrorActors")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let diagram_margin_x = seq_cfg
            .get("diagramMarginX")
            .and_then(|v| v.as_f64())
            .unwrap_or(50.0)
            .max(0.0);
        let box_margin = seq_cfg
            .get("boxMargin")
            .and_then(|v| v.as_f64())
            .unwrap_or(10.0)
            .max(0.0);
        let actor_height = seq_cfg
            .get("height")
            .and_then(|v| v.as_f64())
            .unwrap_or(65.0)
            .max(1.0);
        let box_text_margin = seq_cfg
            .get("boxTextMargin")
            .and_then(|v| v.as_f64())
            .unwrap_or(5.0)
            .max(0.0);
        let message_align = seq_cfg
            .get("messageAlign")
            .and_then(|v| v.as_str())
            .unwrap_or("center")
            .to_string();
        let label_box_height = seq_cfg
            .get("labelBoxHeight")
            .and_then(|v| v.as_f64())
            .unwrap_or(20.0)
            .max(0.0);
        let right_angles = seq_cfg
            .get("rightAngles")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let wrap_padding = seq_cfg
            .get("wrapPadding")
            .and_then(|v| v.as_f64())
            .unwrap_or(10.0)
            .max(0.0);
        let sequence_width = seq_cfg
            .get("width")
            .and_then(|v| v.as_f64())
            .unwrap_or(150.0)
            .max(1.0);
        let activation_width = seq_cfg
            .get("activationWidth")
            .and_then(|v| v.as_f64())
            .unwrap_or(10.0)
            .max(1.0);

        // Upstream Mermaid's Sequence renderer treats the global `fontSize` as authoritative.
        // Per-sequence overrides like `sequence.messageFontSize` apply only when the global value
        // is absent.
        let actor_label_font_size = effective_config
            .get("fontSize")
            .and_then(|v| v.as_f64())
            .or_else(|| seq_cfg.get("messageFontSize").and_then(|v| v.as_f64()))
            .unwrap_or(16.0)
            .max(1.0);
        let loop_text_style = TextStyle {
            font_family: effective_config
                .get("fontFamily")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            font_size: actor_label_font_size,
            font_weight: Some("400".to_string()),
        };
        let note_text_style = TextStyle {
            font_family: loop_text_style.font_family.clone(),
            font_size: actor_label_font_size,
            font_weight: Some("400".to_string()),
        };
        let actor_wrap_width = (sequence_width - 2.0 * wrap_padding).max(1.0);

        Self {
            force_menus,
            mirror_actors,
            diagram_margin_x,
            box_margin,
            actor_height,
            box_text_margin,
            message_align,
            label_box_height,
            right_angles,
            wrap_padding,
            sequence_width,
            activation_width,
            actor_label_font_size,
            actor_wrap_width,
            loop_text_style,
            note_text_style,
        }
    }
}
