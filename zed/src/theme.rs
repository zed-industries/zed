mod highlight_map;
mod theme_registry;

use anyhow::Result;
use gpui::{
    color::Color,
    elements::{ContainerStyle, LabelStyle},
    fonts::{HighlightStyle, TextStyle},
};
use serde::{de, Deserialize};
use std::collections::HashMap;

pub use highlight_map::*;
pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "dark";

#[derive(Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub chat_panel: ChatPanel,
    pub selector: Selector,
    pub editor: Editor,
    pub syntax: SyntaxTheme,
}

pub struct SyntaxTheme {
    highlights: Vec<(String, HighlightStyle)>,
    default_style: HighlightStyle,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub background: Color,
    pub tab: Tab,
    pub active_tab: Tab,
    pub sidebar: Sidebar,
    pub sidebar_icon: SidebarIcon,
    pub active_sidebar_icon: SidebarIcon,
}

#[derive(Deserialize)]
pub struct Tab {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon_close: Color,
    pub icon_dirty: Color,
    pub icon_conflict: Color,
}

#[derive(Deserialize)]
pub struct Sidebar {
    pub icons: ContainerStyle,
    pub resize_handle: ContainerStyle,
}

#[derive(Deserialize)]
pub struct SidebarIcon {
    pub color: Color,
}

#[derive(Deserialize)]
pub struct ChatPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub message: ChatMessage,
    pub channel_select: ChannelSelect,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    pub body: TextStyle,
    pub sender: ContainedText,
    pub timestamp: ContainedText,
}

#[derive(Deserialize)]
pub struct ChannelSelect {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub header: ChannelName,
    pub item: ChannelName,
    pub active_item: ChannelName,
    pub hovered_item: ChannelName,
    pub hovered_active_item: ChannelName,
    pub menu: ContainerStyle,
}

#[derive(Deserialize)]
pub struct ChannelName {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub hash: ContainedText,
    pub name: TextStyle,
}

#[derive(Deserialize)]
pub struct Selector {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,

    pub item: ContainedLabel,
    pub active_item: ContainedLabel,
}

#[derive(Deserialize)]
pub struct ContainedText {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub text: TextStyle,
}

#[derive(Deserialize)]
pub struct ContainedLabel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
}

#[derive(Deserialize)]
pub struct Editor {
    pub background: Color,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub replicas: Vec<Replica>,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Replica {
    pub cursor: Color,
    pub selection: Color,
}

impl SyntaxTheme {
    pub fn new(default_style: HighlightStyle, highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self {
            default_style,
            highlights,
        }
    }

    pub fn highlight_style(&self, id: HighlightId) -> HighlightStyle {
        self.highlights
            .get(id.0 as usize)
            .map(|entry| entry.1.clone())
            .unwrap_or_else(|| self.default_style.clone())
    }

    #[cfg(test)]
    pub fn highlight_name(&self, id: HighlightId) -> Option<&str> {
        self.highlights.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

impl<'de> Deserialize<'de> for SyntaxTheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut syntax_data: HashMap<String, HighlightStyle> =
            Deserialize::deserialize(deserializer)?;

        let mut result = Self {
            highlights: Vec::<(String, HighlightStyle)>::new(),
            default_style: syntax_data
                .remove("default")
                .ok_or_else(|| de::Error::custom("must specify a default color in syntax theme"))?,
        };

        for (key, style) in syntax_data {
            match result
                .highlights
                .binary_search_by(|(needle, _)| needle.cmp(&key))
            {
                Ok(i) | Err(i) => {
                    result.highlights.insert(i, (key, style));
                }
            }
        }

        Ok(result)
    }
}
