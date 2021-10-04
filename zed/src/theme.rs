mod resolution;
mod theme_registry;

use crate::editor::{EditorStyle, SelectionStyle};
use buffer::SyntaxTheme;
use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle},
    fonts::TextStyle,
    Border,
};
use serde::Deserialize;

pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "black";

#[derive(Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub chat_panel: ChatPanel,
    pub people_panel: PeoplePanel,
    pub project_panel: ProjectPanel,
    pub selector: Selector,
    pub editor: EditorStyle,
    pub syntax: SyntaxTheme,
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
    pub outdated_warning: ContainedText,
}

#[derive(Clone, Deserialize)]
pub struct OfflineIcon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
}

#[derive(Clone, Deserialize)]
pub struct Tab {
    pub height: f32,
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
    pub item: SidebarItem,
    pub active_item: SidebarItem,
    pub resize_handle: ContainerStyle,
}

#[derive(Deserialize)]
pub struct SidebarItem {
    pub icon_color: Color,
    pub icon_size: f32,
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

#[derive(Debug, Deserialize)]
pub struct ProjectPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub entry: ProjectPanelEntry,
    pub hovered_entry: ProjectPanelEntry,
    pub selected_entry: ProjectPanelEntry,
    pub hovered_selected_entry: ProjectPanelEntry,
}

#[derive(Debug, Deserialize)]
pub struct ProjectPanelEntry {
    pub height: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    pub icon_color: Color,
    pub icon_size: f32,
    pub icon_spacing: f32,
}

#[derive(Deserialize)]
pub struct PeoplePanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub host_row_height: f32,
    pub host_avatar: ImageStyle,
    pub host_username: ContainedText,
    pub tree_branch_width: f32,
    pub tree_branch_color: Color,
    pub shared_worktree: WorktreeRow,
    pub hovered_shared_worktree: WorktreeRow,
    pub unshared_worktree: WorktreeRow,
    pub hovered_unshared_worktree: WorktreeRow,
}

#[derive(Deserialize)]
pub struct WorktreeRow {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub name: ContainedText,
    pub guest_avatar: ImageStyle,
    pub guest_avatar_spacing: f32,
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

#[derive(Clone, Debug, Deserialize)]
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
