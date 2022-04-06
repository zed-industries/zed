mod theme_registry;

use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle},
    fonts::{HighlightStyle, TextStyle},
    Border,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "dark";

#[derive(Deserialize, Default)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub chat_panel: ChatPanel,
    pub contacts_panel: ContactsPanel,
    pub project_panel: ProjectPanel,
    pub selector: Selector,
    pub editor: Editor,
    pub search: Search,
    pub project_diagnostics: ProjectDiagnostics,
    pub breadcrumbs: ContainedText,
}

#[derive(Deserialize, Default)]
pub struct Workspace {
    pub background: Color,
    pub titlebar: Titlebar,
    pub tab: Tab,
    pub active_tab: Tab,
    pub pane_divider: Border,
    pub leader_border_opacity: f32,
    pub leader_border_width: f32,
    pub left_sidebar: Sidebar,
    pub right_sidebar: Sidebar,
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub disconnected_overlay: ContainedText,
}

#[derive(Clone, Deserialize, Default)]
pub struct Titlebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub title: TextStyle,
    pub avatar_width: f32,
    pub avatar_ribbon: AvatarRibbon,
    pub offline_icon: OfflineIcon,
    pub share_icon_color: Color,
    pub share_icon_active_color: Color,
    pub avatar: ImageStyle,
    pub sign_in_prompt: ContainedText,
    pub hovered_sign_in_prompt: ContainedText,
    pub outdated_warning: ContainedText,
}

#[derive(Clone, Deserialize, Default)]
pub struct AvatarRibbon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct OfflineIcon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub color: Color,
}

#[derive(Clone, Deserialize, Default)]
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

#[derive(Clone, Deserialize, Default)]
pub struct Toolbar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub item_spacing: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct Search {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub editor: FindEditor,
    pub invalid_editor: ContainerStyle,
    pub option_button_group: ContainerStyle,
    pub option_button: ContainedText,
    pub active_option_button: ContainedText,
    pub hovered_option_button: ContainedText,
    pub active_hovered_option_button: ContainedText,
    pub match_background: Color,
    pub match_index: ContainedText,
    pub results_status: TextStyle,
    pub tab_icon_width: f32,
    pub tab_icon_spacing: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct FindEditor {
    #[serde(flatten)]
    pub input: FieldEditor,
    pub min_width: f32,
    pub max_width: f32,
}

#[derive(Deserialize, Default)]
pub struct Sidebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub item: SidebarItem,
    pub active_item: SidebarItem,
    pub resize_handle: ContainerStyle,
}

#[derive(Deserialize, Default)]
pub struct SidebarItem {
    pub icon_color: Color,
    pub icon_size: f32,
    pub height: f32,
}

#[derive(Deserialize, Default)]
pub struct StatusBar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub item_spacing: f32,
    pub cursor_position: TextStyle,
    pub diagnostic_message: TextStyle,
    pub lsp_message: TextStyle,
}

#[derive(Deserialize, Default)]
pub struct ChatPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub message: ChatMessage,
    pub pending_message: ChatMessage,
    pub channel_select: ChannelSelect,
    pub input_editor: FieldEditor,
    pub sign_in_prompt: TextStyle,
    pub hovered_sign_in_prompt: TextStyle,
}

#[derive(Debug, Deserialize, Default)]
pub struct ProjectPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub entry: ProjectPanelEntry,
    pub hovered_entry: ProjectPanelEntry,
    pub selected_entry: ProjectPanelEntry,
    pub hovered_selected_entry: ProjectPanelEntry,
}

#[derive(Debug, Deserialize, Default)]
pub struct ProjectPanelEntry {
    pub height: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    pub icon_color: Color,
    pub icon_size: f32,
    pub icon_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct ContactsPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub host_row_height: f32,
    pub host_avatar: ImageStyle,
    pub host_username: ContainedText,
    pub tree_branch_width: f32,
    pub tree_branch_color: Color,
    pub shared_project: WorktreeRow,
    pub hovered_shared_project: WorktreeRow,
    pub unshared_project: WorktreeRow,
    pub hovered_unshared_project: WorktreeRow,
}

#[derive(Deserialize, Default)]
pub struct WorktreeRow {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub name: ContainedText,
    pub guest_avatar: ImageStyle,
    pub guest_avatar_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct ChatMessage {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub body: TextStyle,
    pub sender: ContainedText,
    pub timestamp: ContainedText,
}

#[derive(Deserialize, Default)]
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

#[derive(Deserialize, Default)]
pub struct ChannelName {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub hash: ContainedText,
    pub name: TextStyle,
}

#[derive(Deserialize, Default)]
pub struct Selector {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty: ContainedLabel,
    pub input_editor: FieldEditor,
    pub item: ContainedLabel,
    pub active_item: ContainedLabel,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ContainedText {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub text: TextStyle,
}

#[derive(Clone, Deserialize, Default)]
pub struct ContainedLabel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
}

#[derive(Clone, Deserialize, Default)]
pub struct ProjectDiagnostics {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty_message: TextStyle,
    pub status_bar_item: ContainedText,
    pub tab_icon_width: f32,
    pub tab_icon_spacing: f32,
    pub tab_summary_spacing: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct Editor {
    pub text_color: Color,
    #[serde(default)]
    pub background: Color,
    pub selection: SelectionStyle,
    pub gutter_background: Color,
    pub gutter_padding_factor: f32,
    pub active_line_background: Color,
    pub highlighted_line_background: Color,
    pub rename_fade: f32,
    pub document_highlight_read_background: Color,
    pub document_highlight_write_background: Color,
    pub diff_background_deleted: Color,
    pub diff_background_inserted: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub guest_selections: Vec<SelectionStyle>,
    pub syntax: Arc<SyntaxTheme>,
    pub diagnostic_path_header: DiagnosticPathHeader,
    pub diagnostic_header: DiagnosticHeader,
    pub error_diagnostic: DiagnosticStyle,
    pub invalid_error_diagnostic: DiagnosticStyle,
    pub warning_diagnostic: DiagnosticStyle,
    pub invalid_warning_diagnostic: DiagnosticStyle,
    pub information_diagnostic: DiagnosticStyle,
    pub invalid_information_diagnostic: DiagnosticStyle,
    pub hint_diagnostic: DiagnosticStyle,
    pub invalid_hint_diagnostic: DiagnosticStyle,
    pub autocomplete: AutocompleteStyle,
    pub code_actions_indicator: Color,
    pub unnecessary_code_fade: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct DiagnosticPathHeader {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub filename: ContainedText,
    pub path: ContainedText,
    pub text_scale_factor: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct DiagnosticHeader {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub message: ContainedLabel,
    pub code: ContainedText,
    pub text_scale_factor: f32,
    pub icon_width_factor: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct DiagnosticStyle {
    pub message: LabelStyle,
    #[serde(default)]
    pub header: ContainerStyle,
    pub text_scale_factor: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct AutocompleteStyle {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub item: ContainerStyle,
    pub selected_item: ContainerStyle,
    pub hovered_item: ContainerStyle,
    pub match_highlight: HighlightStyle,
}

#[derive(Clone, Copy, Default, Deserialize)]
pub struct SelectionStyle {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct FieldEditor {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    #[serde(default)]
    pub placeholder_text: Option<TextStyle>,
    pub selection: SelectionStyle,
}

impl Editor {
    pub fn replica_selection_style(&self, replica_id: u16) -> &SelectionStyle {
        let style_ix = replica_id as usize % (self.guest_selections.len() + 1);
        if style_ix == 0 {
            &self.selection
        } else {
            &self.guest_selections[style_ix - 1]
        }
    }
}

#[derive(Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    pub fn new(highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self { highlights }
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
