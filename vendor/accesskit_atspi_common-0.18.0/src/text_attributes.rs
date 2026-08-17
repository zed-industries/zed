// Copyright 2026 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{Color, TextAlign, TextDecorationStyle};
use accesskit_consumer::Node;
use phf::phf_map;

fn color_to_string(color: Color) -> String {
    format!("{},{},{}", color.red, color.green, color.blue)
}

fn family_name(node: &Node) -> Option<String> {
    node.font_family().map(String::from)
}

fn size(node: &Node) -> Option<String> {
    node.font_size().map(|value| value.to_string())
}

fn weight(node: &Node) -> Option<String> {
    node.font_weight().map(|value| value.to_string())
}

fn style(node: &Node) -> Option<String> {
    node.is_italic().then(|| "italic".into())
}

fn strikethrough(node: &Node) -> Option<String> {
    node.strikethrough().map(|_| "true".into())
}

fn underline(node: &Node) -> Option<String> {
    node.underline().map(|deco| {
        match deco.style {
            TextDecorationStyle::Double => "double",
            _ => "single",
        }
        .into()
    })
}

fn bg_color(node: &Node) -> Option<String> {
    node.background_color().map(color_to_string)
}

fn fg_color(node: &Node) -> Option<String> {
    node.foreground_color().map(color_to_string)
}

fn language(node: &Node) -> Option<String> {
    node.language().map(String::from)
}

fn justification(node: &Node) -> Option<String> {
    node.text_align().map(|align| {
        match align {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "fill",
        }
        .into()
    })
}

pub(crate) const ATTRIBUTE_GETTERS: phf::Map<&'static str, fn(&Node) -> Option<String>> = phf_map! {
    "family-name" => family_name,
    "size" => size,
    "weight" => weight,
    "style" => style,
    "strikethrough" => strikethrough,
    "underline" => underline,
    "bg-color" => bg_color,
    "fg-color" => fg_color,
    "language" => language,
    "justification" => justification,
};
