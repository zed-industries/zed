mod theme_registry;

use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle, Shadow, TooltipStyle},
    fonts::{HighlightStyle, TextStyle},
    Border, MouseState,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub use theme_registry::*;

#[derive(Deserialize, Default)]
pub struct Theme {
    #[serde(default)]
    pub meta: ThemeMeta,
    pub workspace: Workspace,
    pub context_menu: ContextMenu,
    pub contacts_popover: ContactsPopover,
    pub contact_list: ContactList,
    pub contact_finder: ContactFinder,
    pub project_panel: ProjectPanel,
    pub command_palette: CommandPalette,
    pub picker: Picker,
    pub editor: Editor,
    pub search: Search,
    pub project_diagnostics: ProjectDiagnostics,
    pub breadcrumbs: ContainedText,
    pub shared_screen: ContainerStyle,
    pub contact_notification: ContactNotification,
    pub update_notification: UpdateNotification,
    pub simple_message_notification: MessageNotification,
    pub project_shared_notification: ProjectSharedNotification,
    pub incoming_call_notification: IncomingCallNotification,
    pub tooltip: TooltipStyle,
    pub terminal: TerminalStyle,
    pub feedback: FeedbackStyle,
    pub color_scheme: ColorScheme,
}

#[derive(Deserialize, Default, Clone)]
pub struct ThemeMeta {
    pub name: String,
    pub is_light: bool,
}

#[derive(Deserialize, Default)]
pub struct Workspace {
    pub background: Color,
    pub titlebar: Titlebar,
    pub tab_bar: TabBar,
    pub pane_divider: Border,
    pub leader_border_opacity: f32,
    pub leader_border_width: f32,
    pub sidebar: Sidebar,
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub disconnected_overlay: ContainedText,
    pub modal: ContainerStyle,
    pub notification: ContainerStyle,
    pub notifications: Notifications,
    pub joining_project_avatar: ImageStyle,
    pub joining_project_message: ContainedText,
    pub external_location_message: ContainedText,
    pub dock: Dock,
    pub drop_target_overlay_color: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct Titlebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub title: TextStyle,
    pub item_spacing: f32,
    pub face_pile_spacing: f32,
    pub avatar_ribbon: AvatarRibbon,
    pub follower_avatar_overlap: f32,
    pub leader_selection: ContainerStyle,
    pub offline_icon: OfflineIcon,
    pub leader_avatar: AvatarStyle,
    pub follower_avatar: AvatarStyle,
    pub inactive_avatar_grayscale: bool,
    pub sign_in_prompt: Interactive<ContainedText>,
    pub outdated_warning: ContainedText,
    pub share_button: Interactive<ContainedText>,
    pub call_control: Interactive<IconButton>,
    pub toggle_contacts_button: Interactive<IconButton>,
    pub user_menu_button: Interactive<IconButton>,
    pub toggle_contacts_badge: ContainerStyle,
}

#[derive(Copy, Clone, Deserialize, Default)]
pub struct AvatarStyle {
    #[serde(flatten)]
    pub image: ImageStyle,
    pub outer_width: f32,
    pub outer_corner_radius: f32,
}

#[derive(Deserialize, Default)]
pub struct ContactsPopover {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub width: f32,
    pub invite_row_height: f32,
    pub invite_row: Interactive<ContainedLabel>,
}

#[derive(Deserialize, Default)]
pub struct ContactList {
    pub user_query_editor: FieldEditor,
    pub user_query_editor_height: f32,
    pub add_contact_button: IconButton,
    pub header_row: Interactive<ContainedText>,
    pub leave_call: Interactive<ContainedText>,
    pub contact_row: Interactive<ContainerStyle>,
    pub row_height: f32,
    pub project_row: Interactive<ProjectRow>,
    pub tree_branch: Interactive<TreeBranch>,
    pub contact_avatar: ImageStyle,
    pub contact_status_free: ContainerStyle,
    pub contact_status_busy: ContainerStyle,
    pub contact_username: ContainedText,
    pub contact_button: Interactive<IconButton>,
    pub contact_button_spacing: f32,
    pub disabled_button: IconButton,
    pub section_icon_size: f32,
    pub calling_indicator: ContainedText,
}

#[derive(Deserialize, Default)]
pub struct ProjectRow {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub icon: Icon,
    pub name: ContainedText,
}

#[derive(Deserialize, Default, Clone, Copy)]
pub struct TreeBranch {
    pub width: f32,
    pub color: Color,
}

#[derive(Deserialize, Default)]
pub struct ContactFinder {
    pub picker: Picker,
    pub row_height: f32,
    pub contact_avatar: ImageStyle,
    pub contact_username: ContainerStyle,
    pub contact_button: IconButton,
    pub disabled_contact_button: IconButton,
}

#[derive(Clone, Deserialize, Default)]
pub struct TabBar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub pane_button: Interactive<IconButton>,
    pub pane_button_container: ContainerStyle,
    pub active_pane: TabStyles,
    pub inactive_pane: TabStyles,
    pub dragged_tab: Tab,
    pub height: f32,
}

impl TabBar {
    pub fn tab_style(&self, pane_active: bool, tab_active: bool) -> &Tab {
        let tabs = if pane_active {
            &self.active_pane
        } else {
            &self.inactive_pane
        };

        if tab_active {
            &tabs.active_tab
        } else {
            &tabs.inactive_tab
        }
    }
}

#[derive(Clone, Deserialize, Default)]
pub struct TabStyles {
    pub active_tab: Tab,
    pub inactive_tab: Tab,
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
    pub description: ContainedText,
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
    pub nav_button: Interactive<IconButton>,
}

#[derive(Clone, Deserialize, Default)]
pub struct Dock {
    pub initial_size_right: f32,
    pub initial_size_bottom: f32,
    pub wash_color: Color,
    pub panel: ContainerStyle,
    pub maximized: ContainerStyle,
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
    pub dismiss_button: Interactive<IconButton>,
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
    pub feedback: Interactive<TextStyle>,
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
    pub initial_size: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
}

#[derive(Clone, Copy, Deserialize, Default)]
pub struct SidebarItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub icon_color: Color,
    pub icon_size: f32,
}

#[derive(Deserialize, Default)]
pub struct ProjectPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub entry: Interactive<ProjectPanelEntry>,
    pub dragged_entry: ProjectPanelEntry,
    pub ignored_entry: Interactive<ProjectPanelEntry>,
    pub cut_entry: Interactive<ProjectPanelEntry>,
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

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ContextMenu {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub item: Interactive<ContextMenuItem>,
    pub keystroke_margin: f32,
    pub separator: ContainerStyle,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ContextMenuItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub label: TextStyle,
    pub keystroke: ContainedText,
    pub icon_width: f32,
    pub icon_spacing: f32,
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandPalette {
    pub key: Interactive<ContainedLabel>,
    pub keystroke_spacing: f32,
}

#[derive(Deserialize, Default)]
pub struct InviteLink {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon: Icon,
}

#[derive(Deserialize, Clone, Copy, Default)]
pub struct Icon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub width: f32,
}

#[derive(Deserialize, Clone, Copy, Default)]
pub struct IconButton {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub icon_width: f32,
    pub button_width: f32,
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

#[derive(Clone, Deserialize, Default)]
pub struct Picker {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty_container: ContainerStyle,
    pub input_editor: FieldEditor,
    pub empty_input_editor: FieldEditor,
    pub no_matches: ContainedLabel,
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

#[derive(Deserialize, Default)]
pub struct UpdateNotification {
    pub message: ContainedText,
    pub action_message: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
}

#[derive(Deserialize, Default)]
pub struct MessageNotification {
    pub message: ContainedText,
    pub action_message: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
}

#[derive(Deserialize, Default)]
pub struct ProjectSharedNotification {
    pub window_height: f32,
    pub window_width: f32,
    #[serde(default)]
    pub background: Color,
    pub owner_container: ContainerStyle,
    pub owner_avatar: ImageStyle,
    pub owner_metadata: ContainerStyle,
    pub owner_username: ContainedText,
    pub message: ContainedText,
    pub worktree_roots: ContainedText,
    pub button_width: f32,
    pub open_button: ContainedText,
    pub dismiss_button: ContainedText,
}

#[derive(Deserialize, Default)]
pub struct IncomingCallNotification {
    pub window_height: f32,
    pub window_width: f32,
    #[serde(default)]
    pub background: Color,
    pub caller_container: ContainerStyle,
    pub caller_avatar: ImageStyle,
    pub caller_metadata: ContainerStyle,
    pub caller_username: ContainedText,
    pub caller_message: ContainedText,
    pub worktree_roots: ContainedText,
    pub button_width: f32,
    pub accept_button: ContainedText,
    pub decline_button: ContainedText,
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
    pub diff: DiffStyle,
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
    pub code_actions: CodeActions,
    pub unnecessary_code_fade: f32,
    pub hover_popover: HoverPopover,
    pub link_definition: HighlightStyle,
    pub composition_mark: HighlightStyle,
    pub jump_icon: Interactive<IconButton>,
    pub scrollbar: Scrollbar,
}

#[derive(Clone, Deserialize, Default)]
pub struct Scrollbar {
    pub track: ContainerStyle,
    pub thumb: ContainerStyle,
    pub width: f32,
    pub min_height_factor: f32,
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

#[derive(Clone, Deserialize, Default)]
pub struct CodeActions {
    #[serde(default)]
    pub indicator: Color,
    pub vertical_scale: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct DiffStyle {
    pub inserted: Color,
    pub modified: Color,
    pub deleted: Color,
    pub removed_width_em: f32,
    pub width_em: f32,
    pub corner_radius: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Interactive<T> {
    pub default: T,
    pub hover: Option<T>,
    pub clicked: Option<T>,
    pub active: Option<T>,
    pub disabled: Option<T>,
}

impl<T> Interactive<T> {
    pub fn style_for(&self, state: &mut MouseState, active: bool) -> &T {
        if active {
            self.active.as_ref().unwrap_or(&self.default)
        } else if state.clicked() == Some(gpui::MouseButton::Left) && self.clicked.is_some() {
            self.clicked.as_ref().unwrap()
        } else if state.hovered() {
            self.hover.as_ref().unwrap_or(&self.default)
        } else {
            &self.default
        }
    }

    pub fn disabled_style(&self) -> &T {
        self.disabled.as_ref().unwrap_or(&self.default)
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
            clicked: Option<Value>,
            active: Option<Value>,
            disabled: Option<Value>,
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
        let clicked = deserialize_state(json.clicked)?;
        let active = deserialize_state(json.active)?;
        let disabled = deserialize_state(json.disabled)?;
        let default = serde_json::from_value(json.default).map_err(serde::de::Error::custom)?;

        Ok(Interactive {
            default,
            hover,
            clicked,
            active,
            disabled,
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

#[derive(Clone, Deserialize, Default)]
pub struct HoverPopover {
    pub container: ContainerStyle,
    pub info_container: ContainerStyle,
    pub warning_container: ContainerStyle,
    pub error_container: ContainerStyle,
    pub block_style: ContainerStyle,
    pub prose: TextStyle,
    pub highlight: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct TerminalStyle {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
    pub foreground: Color,
    pub background: Color,
    pub modal_background: Color,
    pub cursor: Color,
    pub dim_black: Color,
    pub dim_red: Color,
    pub dim_green: Color,
    pub dim_yellow: Color,
    pub dim_blue: Color,
    pub dim_magenta: Color,
    pub dim_cyan: Color,
    pub dim_white: Color,
    pub bright_foreground: Color,
    pub dim_foreground: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct FeedbackStyle {
    pub submit_button: Interactive<ContainedText>,
    pub button_margin: f32,
    pub info_text: ContainedText,
}

#[derive(Clone, Deserialize, Default)]
pub struct ColorScheme {
    pub name: String,
    pub is_light: bool,

    pub ramps: RampSet,

    pub lowest: Layer,
    pub middle: Layer,
    pub highest: Layer,

    pub popover_shadow: Shadow,
    pub modal_shadow: Shadow,

    pub players: Vec<Player>,
}

#[derive(Clone, Deserialize, Default)]
pub struct Player {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct RampSet {
    pub neutral: Vec<Color>,
    pub red: Vec<Color>,
    pub orange: Vec<Color>,
    pub yellow: Vec<Color>,
    pub green: Vec<Color>,
    pub cyan: Vec<Color>,
    pub blue: Vec<Color>,
    pub violet: Vec<Color>,
    pub magenta: Vec<Color>,
}

#[derive(Clone, Deserialize, Default)]
pub struct Layer {
    pub base: StyleSet,
    pub variant: StyleSet,
    pub on: StyleSet,
    pub accent: StyleSet,
    pub positive: StyleSet,
    pub warning: StyleSet,
    pub negative: StyleSet,
}

#[derive(Clone, Deserialize, Default)]
pub struct StyleSet {
    pub default: Style,
    pub active: Style,
    pub disabled: Style,
    pub hovered: Style,
    pub pressed: Style,
    pub inverted: Style,
}

#[derive(Clone, Deserialize, Default)]
pub struct Style {
    pub background: Color,
    pub border: Color,
    pub foreground: Color,
}
