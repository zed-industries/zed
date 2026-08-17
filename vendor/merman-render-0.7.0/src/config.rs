use serde_json::Value;

pub(crate) const MERMAID_DEFAULT_FONT_FAMILY_CSS: &str =
    r#""trebuchet ms",verdana,arial,sans-serif"#;
pub(crate) const DEFAULT_DIAGRAM_LOOK: &str = "classic";

pub(crate) fn value_at<'a>(cfg: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut cur = cfg;
    for key in path {
        cur = cur.get(*key)?;
    }
    Some(cur)
}

pub(crate) fn config_string(cfg: &Value, path: &[&str]) -> Option<String> {
    value_at(cfg, path).and_then(|v| v.as_str().map(str::to_string))
}

pub(crate) fn json_string_or_first_array(value: &Value) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| {
        value
            .as_array()
            .and_then(|values| values.first()?.as_str())
            .map(str::to_string)
    })
}

pub(crate) fn config_string_or_first_array(cfg: &Value, path: &[&str]) -> Option<String> {
    value_at(cfg, path).and_then(json_string_or_first_array)
}

pub(crate) fn json_string_vec(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|values| {
            values
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub(crate) fn config_string_vec(cfg: &Value, path: &[&str]) -> Vec<String> {
    value_at(cfg, path).map_or_else(Vec::new, json_string_vec)
}

pub(crate) fn config_bool(cfg: &Value, path: &[&str]) -> Option<bool> {
    value_at(cfg, path).and_then(Value::as_bool)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct DiagramLook<'a> {
    value: &'a str,
}

impl<'a> DiagramLook<'a> {
    pub(crate) fn from_raw(raw: Option<&'a str>) -> Self {
        let value = raw
            .map(str::trim)
            .filter(|look| !look.is_empty())
            .unwrap_or(DEFAULT_DIAGRAM_LOOK);
        Self { value }
    }

    pub(crate) fn as_str(&self) -> &'a str {
        self.value
    }

    pub(crate) fn is_neo(&self) -> bool {
        self.value == "neo"
    }

    pub(crate) fn is_hand_drawn(&self) -> bool {
        self.value == "handDrawn"
    }
}

impl std::fmt::Display for DiagramLook<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value)
    }
}

pub(crate) fn config_diagram_look(cfg: &Value) -> DiagramLook<'_> {
    DiagramLook::from_raw(value_at(cfg, &["look"]).and_then(Value::as_str))
}

pub(crate) fn mermaid_config_diagram_look(cfg: &merman_core::MermaidConfig) -> DiagramLook<'_> {
    DiagramLook::from_raw(cfg.get_str("look"))
}

pub(crate) fn normalize_css_font_family(font_family: &str) -> String {
    let s = font_family.trim().trim_end_matches(';').trim();
    if s.is_empty() {
        return String::new();
    }

    // Mermaid serializes generated CSS with comma-separated font-family lists and no extra
    // whitespace around commas. Keep that spelling stable for SVG parity and font-metric keys.
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for ch in s.chars() {
        match ch {
            '\'' if !in_double => {
                in_single = !in_single;
                cur.push(ch);
            }
            '"' if !in_single => {
                in_double = !in_double;
                cur.push(ch);
            }
            ',' if !in_single && !in_double => {
                let p = cur.trim();
                if !p.is_empty() {
                    parts.push(p.to_string());
                }
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }

    let p = cur.trim();
    if !p.is_empty() {
        parts.push(p.to_string());
    }

    parts.join(",")
}

pub(crate) fn config_font_family_css(cfg: &Value) -> String {
    let font_family = config_string(cfg, &["themeVariables", "fontFamily"])
        .or_else(|| config_string(cfg, &["fontFamily"]))
        .unwrap_or_else(|| MERMAID_DEFAULT_FONT_FAMILY_CSS.to_string());
    font_family_css(font_family)
}

pub(crate) fn config_font_family_or_first_array_css(cfg: &Value) -> String {
    let font_family = config_string_or_first_array(cfg, &["themeVariables", "fontFamily"])
        .or_else(|| config_string_or_first_array(cfg, &["fontFamily"]))
        .unwrap_or_else(|| MERMAID_DEFAULT_FONT_FAMILY_CSS.to_string());
    font_family_css(font_family)
}

fn font_family_css(font_family: String) -> String {
    let font_family = normalize_css_font_family(font_family.as_str());
    if font_family.is_empty() {
        MERMAID_DEFAULT_FONT_FAMILY_CSS.to_string()
    } else {
        font_family
    }
}

pub(crate) fn config_theme_or_root_font_size_px_opt(cfg: &Value) -> Option<f64> {
    config_f64_css_px(cfg, &["themeVariables", "fontSize"])
        .or_else(|| config_f64_css_px(cfg, &["fontSize"]))
}

pub(crate) fn config_theme_or_root_font_size_px(cfg: &Value, default: f64) -> f64 {
    config_theme_or_root_font_size_px_opt(cfg).unwrap_or(default)
}

pub(crate) fn config_theme_font_size_css_or_root_number_px_opt(cfg: &Value) -> Option<f64> {
    config_f64_css_px(cfg, &["themeVariables", "fontSize"])
        .or_else(|| config_f64(cfg, &["fontSize"]))
}

pub(crate) fn config_theme_font_size_css_or_root_number_px(cfg: &Value, default: f64) -> f64 {
    config_theme_font_size_css_or_root_number_px_opt(cfg).unwrap_or(default)
}

pub(crate) fn json_f64(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_i64().map(|n| n as f64))
        .or_else(|| value.as_u64().map(|n| n as f64))
        .or_else(|| {
            let n = value.as_str()?.trim().parse::<f64>().ok()?;
            n.is_finite().then_some(n)
        })
}

pub(crate) fn config_f64(cfg: &Value, path: &[&str]) -> Option<f64> {
    value_at(cfg, path).and_then(json_f64)
}

pub(crate) fn config_f64_or(cfg: &Value, path: &[&str], default: f64) -> f64 {
    config_f64(cfg, path).unwrap_or(default)
}

pub(crate) fn json_f64_css_px(value: &Value) -> Option<f64> {
    json_f64(value).or_else(|| value.as_str().and_then(parse_css_px_to_f64))
}

pub(crate) fn config_f64_css_px(cfg: &Value, path: &[&str]) -> Option<f64> {
    value_at(cfg, path).and_then(json_f64_css_px)
}

pub(crate) fn json_f64_explicit_css_px(value: &Value) -> Option<f64> {
    value.as_str().and_then(parse_explicit_css_px_to_f64)
}

pub(crate) fn config_f64_explicit_css_px(cfg: &Value, path: &[&str]) -> Option<f64> {
    value_at(cfg, path).and_then(json_f64_explicit_css_px)
}

pub(crate) fn json_css_number_or_string(value: &Value) -> Option<String> {
    if let Some(raw) = value.as_str() {
        let text = normalize_css_raw_value_text(raw);
        return (!text.is_empty()).then(|| text.to_string());
    }

    let value = json_f64(value)?;
    value.is_finite().then(|| {
        let text = value.to_string();
        if text == "-0" { "0".to_string() } else { text }
    })
}

pub(crate) fn config_css_number_or_string(cfg: &Value, path: &[&str]) -> Option<String> {
    value_at(cfg, path).and_then(json_css_number_or_string)
}

fn parse_css_px_to_f64(text: &str) -> Option<f64> {
    let text = normalize_css_value_text(text);
    let text = text.strip_suffix("px").unwrap_or(text).trim();
    let value = text.parse::<f64>().ok()?;
    value.is_finite().then_some(value)
}

fn parse_explicit_css_px_to_f64(text: &str) -> Option<f64> {
    let text = normalize_css_value_text(text);
    let text = text.strip_suffix("px")?.trim();
    let value = text.parse::<f64>().ok()?;
    value.is_finite().then_some(value)
}

fn normalize_css_value_text(text: &str) -> &str {
    let text = text.trim().trim_end_matches(';').trim();
    text.trim_end_matches("!important").trim()
}

fn normalize_css_raw_value_text(text: &str) -> &str {
    text.trim().trim_end_matches(';').trim()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_f64_accepts_json_numbers_and_plain_numeric_strings() {
        assert_eq!(json_f64(&json!(100)), Some(100.0));
        assert_eq!(json_f64(&json!(70.5)), Some(70.5));
        assert_eq!(json_f64(&json!("100")), Some(100.0));
        assert_eq!(json_f64(&json!(" 70.5 ")), Some(70.5));
    }

    #[test]
    fn json_f64_rejects_css_or_nonfinite_strings() {
        assert_eq!(json_f64(&json!("100px")), None);
        assert_eq!(json_f64(&json!("NaN")), None);
        assert_eq!(json_f64(&json!("inf")), None);
    }

    #[test]
    fn config_f64_walks_paths_and_accepts_yaml_string_numbers() {
        let cfg = json!({
            "flowchart": {
                "rankSpacing": "100",
                "nodeSpacing": "70.5"
            }
        });

        assert_eq!(config_f64(&cfg, &["flowchart", "rankSpacing"]), Some(100.0));
        assert_eq!(config_f64(&cfg, &["flowchart", "nodeSpacing"]), Some(70.5));
        assert_eq!(config_f64(&cfg, &["missing", "rankSpacing"]), None);
    }

    #[test]
    fn config_string_or_first_array_accepts_string_and_array_first_item() {
        let cfg = json!({
            "themeVariables": {
                "fontFamily": ["Courier", "Ignored"],
                "textColor": "#333"
            }
        });

        assert_eq!(
            config_string_or_first_array(&cfg, &["themeVariables", "fontFamily"]),
            Some("Courier".to_string())
        );
        assert_eq!(
            config_string_or_first_array(&cfg, &["themeVariables", "textColor"]),
            Some("#333".to_string())
        );
        assert_eq!(
            config_string_or_first_array(&cfg, &["themeVariables", "missing"]),
            None
        );
    }

    #[test]
    fn config_string_vec_accepts_string_arrays_and_ignores_non_arrays() {
        let cfg = json!({
            "journey": {
                "actorColours": ["#111", 2, "#333"],
                "sectionFills": "not-an-array"
            }
        });

        assert_eq!(
            config_string_vec(&cfg, &["journey", "actorColours"]),
            vec!["#111".to_string(), "#333".to_string()]
        );
        assert_eq!(
            config_string_vec(&cfg, &["journey", "sectionFills"]),
            Vec::<String>::new()
        );
        assert_eq!(
            config_string_vec(&cfg, &["journey", "missing"]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn config_bool_accepts_only_json_bool() {
        let cfg = json!({
            "a": true,
            "b": "true"
        });

        assert_eq!(config_bool(&cfg, &["a"]), Some(true));
        assert_eq!(config_bool(&cfg, &["b"]), None);
    }

    #[test]
    fn config_diagram_look_trims_and_defaults_to_classic() {
        assert_eq!(
            config_diagram_look(&json!({ "look": " neo " })).as_str(),
            "neo"
        );
        assert!(config_diagram_look(&json!({ "look": "neo" })).is_neo());
        assert!(config_diagram_look(&json!({ "look": "handDrawn" })).is_hand_drawn());
        assert_eq!(
            config_diagram_look(&json!({})).as_str(),
            DEFAULT_DIAGRAM_LOOK
        );
        assert_eq!(
            config_diagram_look(&json!({ "look": "" })).as_str(),
            DEFAULT_DIAGRAM_LOOK
        );
    }

    #[test]
    fn json_f64_css_px_accepts_plain_and_css_numeric_strings() {
        assert_eq!(json_f64_css_px(&json!("24")), Some(24.0));
        assert_eq!(json_f64_css_px(&json!("24px")), Some(24.0));
        assert_eq!(json_f64_css_px(&json!("24px !important;")), Some(24.0));
        assert_eq!(json_f64_css_px(&json!("24pt")), None);
        assert_eq!(json_f64_css_px(&json!("NaNpx")), None);
    }

    #[test]
    fn json_f64_explicit_css_px_accepts_only_px_strings() {
        assert_eq!(json_f64_explicit_css_px(&json!(24)), None);
        assert_eq!(json_f64_explicit_css_px(&json!("24")), None);
        assert_eq!(json_f64_explicit_css_px(&json!("24px")), Some(24.0));
        assert_eq!(
            json_f64_explicit_css_px(&json!("24px !important;")),
            Some(24.0)
        );
        assert_eq!(json_f64_explicit_css_px(&json!("24pt")), None);
    }

    #[test]
    fn json_css_number_or_string_keeps_mermaid_style_interpolation_spelling() {
        assert_eq!(
            json_css_number_or_string(&json!(24)),
            Some("24".to_string())
        );
        assert_eq!(
            json_css_number_or_string(&json!(24.5)),
            Some("24.5".to_string())
        );
        assert_eq!(
            json_css_number_or_string(&json!(" 24px; ")),
            Some("24px".to_string())
        );
        assert_eq!(
            json_css_number_or_string(&json!("24px !important;")),
            Some("24px !important".to_string())
        );
        assert_eq!(json_css_number_or_string(&json!(true)), None);
    }

    #[test]
    fn normalize_css_font_family_matches_mermaid_spacing() {
        assert_eq!(
            normalize_css_font_family(r#" "trebuchet ms", verdana, arial, sans-serif; "#),
            r#""trebuchet ms",verdana,arial,sans-serif"#
        );
        assert_eq!(
            normalize_css_font_family(r#"'Open Sans', "IBM Plex Sans", sans-serif"#),
            r#"'Open Sans',"IBM Plex Sans",sans-serif"#
        );
    }

    #[test]
    fn config_font_family_css_uses_theme_then_legacy_then_default() {
        assert_eq!(
            config_font_family_css(&json!({
                "fontFamily": "Courier, monospace",
                "themeVariables": {
                    "fontFamily": "\"IBM Plex Sans\", Arial, sans-serif"
                }
            })),
            r#""IBM Plex Sans",Arial,sans-serif"#
        );
        assert_eq!(
            config_font_family_css(&json!({
                "fontFamily": "Courier, monospace"
            })),
            "Courier,monospace"
        );
        assert_eq!(
            config_font_family_css(&json!({
                "themeVariables": {
                    "fontFamily": " ; "
                }
            })),
            MERMAID_DEFAULT_FONT_FAMILY_CSS
        );
    }

    #[test]
    fn config_font_family_or_first_array_css_uses_theme_then_legacy_then_default() {
        assert_eq!(
            config_font_family_or_first_array_css(&json!({
                "fontFamily": ["Courier, monospace", "Ignored Sans"],
                "themeVariables": {
                    "fontFamily": ["\"IBM Plex Sans\", Arial, sans-serif", "Ignored Sans"]
                }
            })),
            r#""IBM Plex Sans",Arial,sans-serif"#
        );
        assert_eq!(
            config_font_family_or_first_array_css(&json!({
                "fontFamily": ["Courier, monospace", "Ignored Sans"]
            })),
            "Courier,monospace"
        );
        assert_eq!(
            config_font_family_or_first_array_css(&json!({
                "fontFamily": []
            })),
            MERMAID_DEFAULT_FONT_FAMILY_CSS
        );
    }

    #[test]
    fn config_theme_or_root_font_size_px_uses_theme_then_legacy_then_default() {
        assert_eq!(
            config_theme_or_root_font_size_px(
                &json!({
                    "fontSize": "18px",
                    "themeVariables": {
                        "fontSize": "24px"
                    }
                }),
                16.0,
            ),
            24.0
        );
        assert_eq!(
            config_theme_or_root_font_size_px(
                &json!({
                    "fontSize": "18px"
                }),
                16.0,
            ),
            18.0
        );
        assert_eq!(config_theme_or_root_font_size_px(&json!({}), 16.0), 16.0);
    }

    #[test]
    fn config_theme_font_size_css_or_root_number_px_keeps_root_number_semantics() {
        assert_eq!(
            config_theme_font_size_css_or_root_number_px(
                &json!({
                    "fontSize": 18,
                    "themeVariables": {
                        "fontSize": "24px"
                    }
                }),
                16.0,
            ),
            24.0
        );
        assert_eq!(
            config_theme_font_size_css_or_root_number_px(
                &json!({
                    "fontSize": "18"
                }),
                16.0,
            ),
            18.0
        );
        assert_eq!(
            config_theme_font_size_css_or_root_number_px(
                &json!({
                    "fontSize": "18px"
                }),
                16.0,
            ),
            16.0
        );
    }
}
