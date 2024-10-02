use gpui::{Hsla, Rgba};
use lsp::{CompletionItem, Documentation};
use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

const HEX: &'static str = r#"(#(?:[\da-fA-F]{3}){1,2})"#;
const RGB_OR_HSL: &'static str = r#"(rgba?|hsla?)\(\s*(\d{1,3}%?)\s*,\s*(\d{1,3}%?)\s*,\s*(\d{1,3}%?)\s*(?:,\s*(1|0?\.\d+))?\s*\)"#;

pub static RELAXED_HEX_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    RegexBuilder::new(HEX)
        .case_insensitive(false)
        .build()
        .expect("Failed to create RELAXED_HEX_REGEX")
});

pub static STRICT_HEX_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    RegexBuilder::new(&format!("^{HEX}$"))
        .case_insensitive(true)
        .build()
        .expect("Failed to create STRICT_HEX_REGEX")
});

pub static RELAXED_RGB_OR_HSL_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    RegexBuilder::new(RGB_OR_HSL)
        .case_insensitive(false)
        .build()
        .expect("Failed to create RELAXED_RGB_OR_HSL_REGEX")
});

pub static STRICT_RGB_OR_HSL_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    RegexBuilder::new(&format!("^{RGB_OR_HSL}$"))
        .case_insensitive(true)
        .build()
        .expect("Failed to create STRICT_RGB_OR_HSL_REGEX")
});

pub struct ColorExtractor {}

impl ColorExtractor {
    pub fn extract(item: &CompletionItem) -> Option<Hsla> {
        let hex = &STRICT_HEX_REGEX;
        let rgb = &STRICT_RGB_OR_HSL_REGEX;

        // Try to extract from entire `label` field
        Self::parse(&item.label, &hex, &rgb)
            // Try to extract from entire `detail` field
            .or_else(|| match item.detail {
                Some(ref detail) => Self::parse(detail, &hex, &rgb),
                None => None,
            })
            // Try extract from beginning or end of `documentation` field
            .or_else(|| {
                let hex = &RELAXED_HEX_REGEX;
                let rgb = &RELAXED_RGB_OR_HSL_REGEX;

                match item.documentation {
                    Some(Documentation::String(ref str)) => Self::parse(str, &hex, &rgb),
                    Some(Documentation::MarkupContent(ref markup)) => {
                        Self::parse(&markup.value, &hex, &rgb)
                    }
                    None => None,
                }
            })
    }

    fn parse(str: &str, hex: &Regex, rgb: &Regex) -> Option<Hsla> {
        if let Some(captures) = hex.captures(str) {
            let rmatch = captures.get(0)?;

            // Color must be anchored to start or end of string
            if rmatch.start() > 0 && rmatch.end() != str.len() {
                return None;
            }

            let hex = captures.get(1)?.as_str();

            return Self::from_hex(hex);
        }

        if let Some(captures) = rgb.captures(str) {
            let rmatch = captures.get(0)?;

            // Color must be anchored to start or end of string
            if rmatch.start() > 0 && rmatch.end() != str.len() {
                return None;
            }

            let typ = captures.get(1)?.as_str();
            let rh = captures.get(2)?.as_str();
            let gs = captures.get(3)?.as_str();
            let bl = captures.get(4)?.as_str();
            let a = captures.get(5).map(|c| c.as_str());

            return match (typ, a) {
                ("rgb", None) | ("rgba", Some(_)) => Self::from_rgb(rh, gs, bl, a),
                ("hsl", None) | ("hsla", Some(_)) => Self::from_hsl(rh, gs, bl, a),
                _ => None,
            };
        }

        return None;
    }

    fn from_hex(hex: &str) -> Option<Hsla> {
        return Rgba::try_from(hex).map(|color| color.into()).ok();
    }

    fn parse_field(field: &str, max: f32) -> Option<f32> {
        if field.ends_with("%") {
            field[..field.len() - 1]
                .parse::<f32>()
                .map(|v| v / 100.)
                .ok()
        } else {
            field.parse::<f32>().map(|v| v / max).ok()
        }
    }

    fn from_rgb(r: &str, g: &str, b: &str, a: Option<&str>) -> Option<Hsla> {
        let r = Self::parse_field(r, 255.)?;
        let g = Self::parse_field(g, 255.)?;
        let b = Self::parse_field(b, 255.)?;
        let a = match a {
            Some(a) => Self::parse_field(a, 1.0),
            None => Some(1.0),
        }?;

        Some(Rgba { r, g, b, a }.into())
    }

    fn from_hsl(h: &str, s: &str, l: &str, a: Option<&str>) -> Option<Hsla> {
        let h = Self::parse_field(h, 360.)?;
        let s = Self::parse_field(s, 100.)?;
        let l = Self::parse_field(l, 100.)?;
        let a = match a {
            Some(a) => Self::parse_field(a, 1.0),
            None => Some(1.0),
        }?;

        Some(Hsla { h, s, l, a })
    }
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
            let color = ColorExtractor::extract(&CompletionItem {
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
