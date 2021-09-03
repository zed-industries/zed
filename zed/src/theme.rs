mod highlight_map;
mod theme_registry;

use anyhow::Result;
use gpui::{
    color::Color,
    elements::{ContainerStyle, LabelStyle},
    fonts::{HighlightStyle, TextStyle},
};
use serde::Deserialize;
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
    pub editor: EditorStyle,
    pub syntax: SyntaxTheme,
}

pub struct SyntaxTheme {
    highlights: Vec<(String, HighlightStyle)>,
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
    pub input_editor: InputEditorStyle,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    #[serde(flatten)]
    pub container: ContainerStyle,
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

    pub input_editor: InputEditorStyle,
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

#[derive(Clone, Deserialize)]
pub struct EditorStyle {
    pub text: HighlightStyle,
    #[serde(default)]
    pub placeholder_text: HighlightStyle,
    pub background: Color,
    pub selection: SelectionStyle,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub guest_selections: Vec<SelectionStyle>,
}

#[derive(Clone, Deserialize)]
pub struct InputEditorStyle {
    pub text: HighlightStyle,
    pub placeholder_text: HighlightStyle,
    pub background: Color,
    pub selection: SelectionStyle,
}

#[derive(Clone, Copy, Default, Deserialize)]
pub struct SelectionStyle {
    pub cursor: Color,
    pub selection: Color,
}

impl SyntaxTheme {
    pub fn new(highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self { highlights }
    }

    pub fn highlight_style(&self, id: HighlightId) -> Option<HighlightStyle> {
        self.highlights
            .get(id.0 as usize)
            .map(|entry| entry.1.clone())
    }

    #[cfg(test)]
    pub fn highlight_name(&self, id: HighlightId) -> Option<&str> {
        self.highlights.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self {
            text: HighlightStyle {
                color: Color::from_u32(0xff0000ff),
                font_properties: Default::default(),
            },
            placeholder_text: HighlightStyle {
                color: Color::from_u32(0x00ff00ff),
                font_properties: Default::default(),
            },
            background: Default::default(),
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            selection: Default::default(),
            guest_selections: Default::default(),
        }
    }
}

impl InputEditorStyle {
    pub fn as_editor(&self) -> EditorStyle {
        EditorStyle {
            text: self.text.clone(),
            placeholder_text: self.placeholder_text.clone(),
            background: self.background,
            selection: self.selection,
            ..Default::default()
        }
    }
}

impl<'de> Deserialize<'de> for SyntaxTheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let syntax_data: HashMap<String, HighlightStyle> = Deserialize::deserialize(deserializer)?;

        let mut result = Self::new(Vec::new());
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
