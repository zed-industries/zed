//! Shared helpers for state diagram layout.

use super::StateNode;
pub(super) use crate::config::{config_bool, config_f64};
use crate::config::{config_f64_css_px, config_string_or_first_array};
use crate::text::TextStyle;
use dugong::RankDir;
use serde_json::Value;

pub(super) fn state_node_is_effective_group(n: &StateNode) -> bool {
    n.is_group && n.shape != "note"
}

pub(super) fn normalize_dir(direction: &str) -> String {
    match direction.trim().to_uppercase().as_str() {
        "TB" | "TD" => "TB".to_string(),
        "BT" => "BT".to_string(),
        "LR" => "LR".to_string(),
        "RL" => "RL".to_string(),
        other => other.to_string(),
    }
}

pub(super) fn rank_dir_from(direction: &str) -> RankDir {
    match normalize_dir(direction).as_str() {
        "TB" => RankDir::TB,
        "BT" => RankDir::BT,
        "LR" => RankDir::LR,
        "RL" => RankDir::RL,
        _ => RankDir::TB,
    }
}

pub(super) fn value_to_label_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(a) => a
            .first()
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
        _ => "".to_string(),
    }
}

pub(crate) fn state_html_label_wrapping_width(cfg: &Value) -> f64 {
    config_f64_css_px(cfg, &["flowchart", "wrappingWidth"])
        .unwrap_or(200.0)
        .max(0.0)
}

pub(super) fn decode_html_entities_once(text: &str) -> std::borrow::Cow<'_, str> {
    if text.contains('ﬂ') || text.contains('¶') || text.contains('#') {
        return merman_core::entities::decode_mermaid_entities_to_unicode(text);
    }

    fn decode_html_entity(entity: &str) -> Option<char> {
        match entity {
            "nbsp" => Some(' '),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "#39" => Some('\''),
            "colon" => Some(':'),
            "equals" => Some('='),
            _ => {
                if let Some(hex) = entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                {
                    u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                } else if let Some(dec) = entity.strip_prefix('#') {
                    dec.parse::<u32>().ok().and_then(char::from_u32)
                } else {
                    None
                }
            }
        }
    }

    if !text.contains('&') {
        return std::borrow::Cow::Borrowed(text);
    }

    let mut out = String::with_capacity(text.len());
    let mut i = 0usize;
    while let Some(rel) = text[i..].find('&') {
        let amp = i + rel;
        out.push_str(&text[i..amp]);
        let tail = &text[amp + 1..];
        if let Some(semi_rel) = tail.find(';') {
            let semi = amp + 1 + semi_rel;
            let entity = &text[amp + 1..semi];
            if let Some(decoded) = decode_html_entity(entity) {
                out.push(decoded);
            } else {
                out.push_str(&text[amp..=semi]);
            }
            i = semi + 1;
            continue;
        }
        out.push('&');
        i = amp + 1;
    }
    out.push_str(&text[i..]);
    std::borrow::Cow::Owned(out)
}

pub(crate) fn state_text_style(effective_config: &Value) -> TextStyle {
    // Mermaid state diagram v2 uses HTML labels (foreignObject) by default, inheriting the global
    // `#id{font-size: ...}` rule (defaults to 16px). The 10px `g.stateGroup text{font-size:10px}`
    // rule applies to SVG `<text>` elements, not HTML labels.
    let font_family = config_string_or_first_array(effective_config, &["fontFamily"])
        .or_else(|| {
            config_string_or_first_array(effective_config, &["themeVariables", "fontFamily"])
        })
        .or_else(|| Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()));
    // Mermaid CLI baselines show state labels inheriting the SVG root font-size rule
    // (`themeVariables.fontSize`, typically a `"NNpx"` string).
    let font_size =
        crate::config::config_theme_or_root_font_size_px(effective_config, 16.0).max(1.0);
    TextStyle {
        font_family,
        font_size,
        font_weight: None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn state_html_label_wrapping_width_honors_number_and_px_string() {
        let numeric = serde_json::json!({
            "flowchart": {
                "wrappingWidth": 320
            }
        });
        assert_eq!(super::state_html_label_wrapping_width(&numeric), 320.0);

        let px_string = serde_json::json!({
            "flowchart": {
                "wrappingWidth": "280px"
            }
        });
        assert_eq!(super::state_html_label_wrapping_width(&px_string), 280.0);

        let fallback = serde_json::json!({});
        assert_eq!(super::state_html_label_wrapping_width(&fallback), 200.0);
    }

    #[test]
    fn state_entity_decode_handles_mermaid_placeholders_and_colon_entity() {
        assert_eq!(
            super::decode_html_entities_once("test({ fooﬂ°colon¶ß 'far' })"),
            "test({ foo: 'far' })"
        );
        assert_eq!(
            super::decode_html_entities_once("test({ foo&colon; 'far' })"),
            "test({ foo: 'far' })"
        );
    }
}
