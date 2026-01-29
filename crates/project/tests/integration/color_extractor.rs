use gpui::{Hsla, rgba};
use lsp::{CompletionItem, CompletionItemKind, Documentation};
use project::color_extractor::*;

pub const COLOR_TABLE: &[(&str, Option<u32>)] = &[
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
];

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
