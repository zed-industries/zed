mod highlight_map;
mod resolution;
mod theme_registry;

use crate::editor::{EditorStyle, SelectionStyle};
use anyhow::Result;
use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle},
    fonts::{HighlightStyle, TextStyle},
    Border,
};
use serde::Deserialize;
use std::collections::HashMap;

pub use highlight_map::*;
pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "black";

#[derive(Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub chat_panel: ChatPanel,
    pub people_panel: PeoplePanel,
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
    pub titlebar: Titlebar,
    pub tab: Tab,
    pub active_tab: Tab,
    pub pane_divider: Border,
    pub left_sidebar: Sidebar,
    pub right_sidebar: Sidebar,
}

#[derive(Clone, Deserialize)]
pub struct Titlebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub title: TextStyle,
    pub avatar_width: f32,
    pub offline_icon: OfflineIcon,
    pub icon_color: Color,
    pub avatar: ImageStyle,
}

#[derive(Clone, Deserialize)]
pub struct OfflineIcon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
}

#[derive(Clone, Deserialize)]
pub struct Tab {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub spacing: f32,
    pub icon_width: f32,
    pub icon_close: Color,
    pub icon_close_active: Color,
    pub icon_dirty: Color,
    pub icon_conflict: Color,
}

#[derive(Deserialize)]
pub struct Sidebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub icon: SidebarIcon,
    pub active_icon: SidebarIcon,
    pub resize_handle: ContainerStyle,
}

#[derive(Deserialize)]
pub struct SidebarIcon {
    pub color: Color,
    pub height: f32,
}

#[derive(Deserialize)]
pub struct ChatPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub message: ChatMessage,
    pub pending_message: ChatMessage,
    pub channel_select: ChannelSelect,
    pub input_editor: InputEditorStyle,
    pub sign_in_prompt: TextStyle,
    pub hovered_sign_in_prompt: TextStyle,
}

#[derive(Deserialize)]
pub struct PeoplePanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub host_avatar: ImageStyle,
    pub host_username: ContainedText,
    pub own_worktree: ContainedText,
    pub joined_worktree: ContainedText,
    pub shared_worktree: ContainedText,
    pub hovered_shared_worktree: ContainedText,
    pub unshared_worktree: ContainedText,
    pub guest_avatar: ImageStyle,
    pub guest_avatar_spacing: f32,
    pub tree_branch: TreeBranch,
}

#[derive(Copy, Clone, Deserialize)]
pub struct TreeBranch {
    pub width: f32,
    pub color: Color,
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
    pub empty: ContainedLabel,
    pub input_editor: InputEditorStyle,
    pub item: ContainedLabel,
    pub active_item: ContainedLabel,
}

#[derive(Debug, Deserialize)]
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
pub struct InputEditorStyle {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    #[serde(default)]
    pub placeholder_text: Option<TextStyle>,
    pub selection: SelectionStyle,
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

impl InputEditorStyle {
    pub fn as_editor(&self) -> EditorStyle {
        EditorStyle {
            text: self.text.clone(),
            placeholder_text: self.placeholder_text.clone(),
            background: self
                .container
                .background_color
                .unwrap_or(Color::transparent_black()),
            selection: self.selection,
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            guest_selections: Default::default(),
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
