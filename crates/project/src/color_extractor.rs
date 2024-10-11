use std::sync::LazyLock;

use gpui::{Hsla, Rgba};
use lsp::{CompletionItem, Documentation};
use regex::{Regex, RegexBuilder};

const HEX: &'static str = r#"(#(?:[\da-fA-F]{3}){1,2})"#;
const RGB_OR_HSL: &'static str = r#"(rgba?|hsla?)\(\s*(\d{1,3}%?)\s*,\s*(\d{1,3}%?)\s*,\s*(\d{1,3}%?)\s*(?:,\s*(1|0?\.\d+))?\s*\)"#;

static RELAXED_HEX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(HEX)
        .case_insensitive(false)
        .build()
        .expect("Failed to create RELAXED_HEX_REGEX")
});

static STRICT_HEX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(&format!("^{HEX}$"))
        .case_insensitive(true)
        .build()
        .expect("Failed to create STRICT_HEX_REGEX")
});

static RELAXED_RGB_OR_HSL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(RGB_OR_HSL)
        .case_insensitive(false)
        .build()
        .expect("Failed to create RELAXED_RGB_OR_HSL_REGEX")
});

static STRICT_RGB_OR_HSL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(&format!("^{RGB_OR_HSL}$"))
        .case_insensitive(true)
        .build()
        .expect("Failed to create STRICT_RGB_OR_HSL_REGEX")
});

/// Extracts a color from an LSP [`CompletionItem`].
///
/// Adapted from https://github.com/microsoft/vscode/blob/a6870fcb6d79093738c17e8319b760cf1c41764a/src/vs/editor/contrib/suggest/browser/suggestWidgetRenderer.ts#L34-L61
pub fn extract_color(item: &CompletionItem) -> Option<Hsla> {
    // Try to extract from entire `label` field.
    parse(&item.label, ParseMode::Strict)
        // Try to extract from entire `detail` field.
        .or_else(|| {
            item.detail
                .as_ref()
                .and_then(|detail| parse(detail, ParseMode::Strict))
        })
        // Try to extract from beginning or end of `documentation` field.
        .or_else(|| match item.documentation {
            Some(Documentation::String(ref str)) => parse(str, ParseMode::Relaxed),
            Some(Documentation::MarkupContent(ref markup)) => {
                parse(&markup.value, ParseMode::Relaxed)
            }
            None => None,
        })
}

enum ParseMode {
    Strict,
    Relaxed,
}

fn parse(str: &str, mode: ParseMode) -> Option<Hsla> {
    let (hex, rgb) = match mode {
        ParseMode::Strict => (&STRICT_HEX_REGEX, &STRICT_RGB_OR_HSL_REGEX),
        ParseMode::Relaxed => (&RELAXED_HEX_REGEX, &RELAXED_RGB_OR_HSL_REGEX),
    };

    if let Some(captures) = hex.captures(str) {
        let rmatch = captures.get(0)?;

        // Color must be anchored to start or end of string.
        if rmatch.start() > 0 && rmatch.end() != str.len() {
            return None;
        }

        let hex = captures.get(1)?.as_str();

        return from_hex(hex);
    }

    if let Some(captures) = rgb.captures(str) {
        let rmatch = captures.get(0)?;

        // Color must be anchored to start or end of string.
        if rmatch.start() > 0 && rmatch.end() != str.len() {
            return None;
        }

        let typ = captures.get(1)?.as_str();
        let r_or_h = captures.get(2)?.as_str();
        let g_or_s = captures.get(3)?.as_str();
        let b_or_l = captures.get(4)?.as_str();
        let a = captures.get(5).map(|a| a.as_str());

        return match (typ, a) {
            ("rgb", None) | ("rgba", Some(_)) => from_rgb(r_or_h, g_or_s, b_or_l, a),
            ("hsl", None) | ("hsla", Some(_)) => from_hsl(r_or_h, g_or_s, b_or_l, a),
            _ => None,
        };
    }

    return None;
}

fn parse_component(value: &str, max: f32) -> Option<f32> {
    if let Some(field) = value.strip_suffix("%") {
        field.parse::<f32>().map(|value| value / 100.).ok()
    } else {
        value.parse::<f32>().map(|value| value / max).ok()
    }
}

fn from_hex(hex: &str) -> Option<Hsla> {
    Rgba::try_from(hex).map(Hsla::from).ok()
}

fn from_rgb(r: &str, g: &str, b: &str, a: Option<&str>) -> Option<Hsla> {
    let r = parse_component(r, 255.)?;
    let g = parse_component(g, 255.)?;
    let b = parse_component(b, 255.)?;
    let a = a.and_then(|a| parse_component(a, 1.0)).unwrap_or(1.0);

    Some(Rgba { r, g, b, a }.into())
}

fn from_hsl(h: &str, s: &str, l: &str, a: Option<&str>) -> Option<Hsla> {
    let h = parse_component(h, 360.)?;
    let s = parse_component(s, 100.)?;
    let l = parse_component(l, 100.)?;
    let a = a.and_then(|a| parse_component(a, 1.0)).unwrap_or(1.0);

    Some(Hsla { h, s, l, a })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgba;
    use lsp::{CompletionItem, CompletionItemKind};

    pub static COLOR_TABLE: LazyLock<Vec<(&'static str, Option<u32>)>> = LazyLock::new(|| {
        vec![
            // -- Invalid --
            // Invalid hex
            ("f0f", None),
            ("#fof", None),
            // Extra field
            ("rgb(255, 0, 0, 0.0)", None),
            ("hsl(120, 0, 0, 0.0)", None),
            // Missing field
            ("rgba(255, 0, 0)", None),
            ("hsla(120, 0, 0)", None),
            // No decimal after zero
            ("rgba(255, 0, 0, 0)", None),
            ("hsla(120, 0, 0, 0)", None),
            // Decimal after one
            ("rgba(255, 0, 0, 1.0)", None),
            ("hsla(120, 0, 0, 1.0)", None),
            // HEX (sRGB)
            ("#f0f", Some(0xFF00FFFF)),
            ("#ff0000", Some(0xFF0000FF)),
            // RGB / RGBA (sRGB)
            ("rgb(255, 0, 0)", Some(0xFF0000FF)),
            ("rgba(255, 0, 0, 0.4)", Some(0xFF000066)),
            ("rgba(255, 0, 0, 1)", Some(0xFF0000FF)),
            ("rgb(20%, 0%, 0%)", Some(0x330000FF)),
            ("rgba(20%, 0%, 0%, 1)", Some(0x330000FF)),
            ("rgb(0%, 20%, 0%)", Some(0x003300FF)),
            ("rgba(0%, 20%, 0%, 1)", Some(0x003300FF)),
            ("rgb(0%, 0%, 20%)", Some(0x000033FF)),
            ("rgba(0%, 0%, 20%, 1)", Some(0x000033FF)),
            // HSL / HSLA (sRGB)
            ("hsl(0, 100%, 50%)", Some(0xFF0000FF)),
            ("hsl(120, 100%, 50%)", Some(0x00FF00FF)),
            ("hsla(0, 100%, 50%, 0.0)", Some(0xFF000000)),
            ("hsla(0, 100%, 50%, 0.4)", Some(0xFF000066)),
            ("hsla(0, 100%, 50%, 1)", Some(0xFF0000FF)),
            ("hsla(120, 100%, 50%, 0.0)", Some(0x00FF0000)),
            ("hsla(120, 100%, 50%, 0.4)", Some(0x00FF0066)),
            ("hsla(120, 100%, 50%, 1)", Some(0x00FF00FF)),
        ]
    });

    #[test]
    fn can_extract_from_label() {
        for (color_str, color_val) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: color_str.to_string(),
                detail: None,
                documentation: None,
                ..Default::default()
            });

            assert_eq!(color, color_val.map(|v| Hsla::from(rgba(v))));
        }
    }

    #[test]
    fn only_whole_label_matches_are_allowed() {
        for (color_str, _) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: format!("{} foo", color_str).to_string(),
                detail: None,
                documentation: None,
                ..Default::default()
            });

            assert_eq!(color, None);
        }
    }

    #[test]
    fn can_extract_from_detail() {
        for (color_str, color_val) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: "".to_string(),
                detail: Some(color_str.to_string()),
                documentation: None,
                ..Default::default()
            });

            assert_eq!(color, color_val.map(|v| Hsla::from(rgba(v))));
        }
    }

    #[test]
    fn only_whole_detail_matches_are_allowed() {
        for (color_str, _) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: "".to_string(),
                detail: Some(format!("{} foo", color_str).to_string()),
                documentation: None,
                ..Default::default()
            });

            assert_eq!(color, None);
        }
    }

    #[test]
    fn can_extract_from_documentation_start() {
        for (color_str, color_val) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: "".to_string(),
                detail: None,
                documentation: Some(Documentation::String(
                    format!("{} foo", color_str).to_string(),
                )),
                ..Default::default()
            });

            assert_eq!(color, color_val.map(|v| Hsla::from(rgba(v))));
        }
    }

    #[test]
    fn can_extract_from_documentation_end() {
        for (color_str, color_val) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: "".to_string(),
                detail: None,
                documentation: Some(Documentation::String(
                    format!("foo {}", color_str).to_string(),
                )),
                ..Default::default()
            });

            assert_eq!(color, color_val.map(|v| Hsla::from(rgba(v))));
        }
    }

    #[test]
    fn cannot_extract_from_documentation_middle() {
        for (color_str, _) in COLOR_TABLE.iter() {
            let color = extract_color(&CompletionItem {
                kind: Some(CompletionItemKind::COLOR),
                label: "".to_string(),
                detail: None,
                documentation: Some(Documentation::String(
                    format!("foo {} foo", color_str).to_string(),
                )),
                ..Default::default()
            });

            assert_eq!(color, None);
        }
    }
}
