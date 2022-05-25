mod theme_registry;

use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle, MouseState},
    fonts::{HighlightStyle, TextStyle},
    Border,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "cave-dark";

#[derive(Deserialize, Default)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub chat_panel: ChatPanel,
    pub contacts_panel: ContactsPanel,
    pub contact_finder: ContactFinder,
    pub project_panel: ProjectPanel,
    pub command_palette: CommandPalette,
    pub picker: Picker,
    pub editor: Editor,
    pub search: Search,
    pub project_diagnostics: ProjectDiagnostics,
    pub breadcrumbs: ContainedText,
    pub contact_notification: ContactNotification,
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
    pub sidebar_resize_handle: ContainerStyle,
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub disconnected_overlay: ContainedText,
    pub modal: ContainerStyle,
    pub notification: ContainerStyle,
    pub notifications: Notifications,
    pub joining_project_avatar: ImageStyle,
    pub joining_project_message: ContainedText,
}

#[derive(Clone, Deserialize, Default)]
pub struct Titlebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub title: TextStyle,
    pub avatar_width: f32,
    pub avatar_margin: f32,
    pub avatar_ribbon: AvatarRibbon,
    pub offline_icon: OfflineIcon,
    pub share_icon: Interactive<ShareIcon>,
    pub avatar: ImageStyle,
    pub sign_in_prompt: Interactive<ContainedText>,
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
pub struct ShareIcon {
    #[serde(flatten)]
    pub container: ContainerStyle,
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
pub struct Notifications {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct Search {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub editor: FindEditor,
    pub invalid_editor: ContainerStyle,
    pub option_button_group: ContainerStyle,
    pub option_button: Interactive<ContainedText>,
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
pub struct StatusBar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub item_spacing: f32,
    pub cursor_position: TextStyle,
    pub auto_update_progress_message: TextStyle,
    pub auto_update_done_message: TextStyle,
    pub lsp_status: Interactive<StatusBarLspStatus>,
    pub sidebar_buttons: StatusBarSidebarButtons,
    pub diagnostic_summary: Interactive<StatusBarDiagnosticSummary>,
    pub diagnostic_message: Interactive<ContainedText>,
}

#[derive(Deserialize, Default)]
pub struct StatusBarSidebarButtons {
    pub group_left: ContainerStyle,
    pub group_right: ContainerStyle,
    pub item: Interactive<SidebarItem>,
    pub badge: ContainerStyle,
}

#[derive(Deserialize, Default)]
pub struct StatusBarDiagnosticSummary {
    pub container_ok: ContainerStyle,
    pub container_warning: ContainerStyle,
    pub container_error: ContainerStyle,
    pub text: TextStyle,
    pub icon_color_ok: Color,
    pub icon_color_warning: Color,
    pub icon_color_error: Color,
    pub height: f32,
    pub icon_width: f32,
    pub icon_spacing: f32,
    pub summary_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct StatusBarLspStatus {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub icon_spacing: f32,
    pub icon_color: Color,
    pub icon_width: f32,
    pub message: TextStyle,
}

#[derive(Deserialize, Default)]
pub struct Sidebar {
    pub resize_handle: ContainerStyle,
}

#[derive(Clone, Copy, Deserialize, Default)]
pub struct SidebarItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub icon_color: Color,
    pub icon_size: f32,
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

#[derive(Deserialize, Default)]
pub struct ProjectPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub entry: Interactive<ProjectPanelEntry>,
    pub ignored_entry_fade: f32,
    pub filename_editor: FieldEditor,
    pub indent_width: f32,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ProjectPanelEntry {
    pub height: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    pub icon_color: Color,
    pub icon_size: f32,
    pub icon_spacing: f32,
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandPalette {
    pub key: Interactive<ContainedLabel>,
    pub keystroke_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct ContactsPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub user_query_editor: FieldEditor,
    pub user_query_editor_height: f32,
    pub add_contact_button: IconButton,
    pub header_row: Interactive<ContainedText>,
    pub contact_row: Interactive<ContainerStyle>,
    pub project_row: Interactive<ProjectRow>,
    pub row_height: f32,
    pub contact_avatar: ImageStyle,
    pub contact_username: ContainedText,
    pub contact_button: Interactive<IconButton>,
    pub contact_button_spacing: f32,
    pub disabled_contact_button: IconButton,
    pub tree_branch: Interactive<TreeBranch>,
    pub section_icon_size: f32,
    pub invite_row: Interactive<ContainedLabel>,
}

#[derive(Deserialize, Default)]
pub struct InviteLink {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon: Icon,
}

#[derive(Deserialize, Default, Clone, Copy)]
pub struct TreeBranch {
    pub width: f32,
    pub color: Color,
}

#[derive(Deserialize, Default)]
pub struct ContactFinder {
    pub row_height: f32,
    pub contact_avatar: ImageStyle,
    pub contact_username: ContainerStyle,
    pub contact_button: IconButton,
    pub disabled_contact_button: IconButton,
}

#[derive(Deserialize, Default)]
pub struct Icon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub width: f32,
    pub path: String,
}

#[derive(Deserialize, Default)]
pub struct IconButton {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub icon_width: f32,
    pub button_width: f32,
}

#[derive(Deserialize, Default)]
pub struct ProjectRow {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub name: ContainedText,
    pub guests: ContainerStyle,
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
pub struct Picker {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty: ContainedLabel,
    pub input_editor: FieldEditor,
    pub item: Interactive<ContainedLabel>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ContainedText {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub text: TextStyle,
}

#[derive(Clone, Debug, Deserialize, Default)]
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
    pub tab_icon_width: f32,
    pub tab_icon_spacing: f32,
    pub tab_summary_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct ContactNotification {
    pub header_avatar: ImageStyle,
    pub header_message: ContainedText,
    pub header_height: f32,
    pub body_message: ContainedText,
    pub button: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Interactive<T> {
    pub default: T,
    pub hover: Option<T>,
    pub active: Option<T>,
    pub active_hover: Option<T>,
}

impl<T> Interactive<T> {
    pub fn style_for(&self, state: &MouseState, active: bool) -> &T {
        if active {
            if state.hovered {
                self.active_hover
                    .as_ref()
                    .or(self.active.as_ref())
                    .unwrap_or(&self.default)
            } else {
                self.active.as_ref().unwrap_or(&self.default)
            }
        } else {
            if state.hovered {
                self.hover.as_ref().unwrap_or(&self.default)
            } else {
                &self.default
            }
        }
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Interactive<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(flatten)]
            default: Value,
            hover: Option<Value>,
            active: Option<Value>,
            active_hover: Option<Value>,
        }

        let json = Helper::deserialize(deserializer)?;

        let deserialize_state = |state_json: Option<Value>| -> Result<Option<T>, D::Error> {
            if let Some(mut state_json) = state_json {
                if let Value::Object(state_json) = &mut state_json {
                    if let Value::Object(default) = &json.default {
                        for (key, value) in default {
                            if !state_json.contains_key(key) {
                                state_json.insert(key.clone(), value.clone());
                            }
                        }
                    }
                }
                Ok(Some(
                    serde_json::from_value::<T>(state_json).map_err(serde::de::Error::custom)?,
                ))
            } else {
                Ok(None)
            }
        };

        let hover = deserialize_state(json.hover)?;
        let active = deserialize_state(json.active)?;
        let active_hover = deserialize_state(json.active_hover)?;
        let default = serde_json::from_value(json.default).map_err(serde::de::Error::custom)?;

        Ok(Interactive {
            default,
            hover,
            active,
            active_hover,
        })
    }
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
