pub mod components;
mod theme_registry;
mod theme_settings;
pub mod ui;

use components::{action_button::ButtonStyle, disclosure::DisclosureStyle, ToggleIconButtonStyle};
use gpui::{
    color::Color,
    elements::{ContainerStyle, ImageStyle, LabelStyle, Shadow, SvgStyle, TooltipStyle},
    fonts::{HighlightStyle, TextStyle},
    platform, AppContext, AssetSource, Border, MouseState,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use settings::SettingsStore;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use ui::{CheckboxStyle, CopilotCTAButton, IconStyle, ModalStyle};

pub use theme_registry::*;
pub use theme_settings::*;

pub fn current(cx: &AppContext) -> Arc<Theme> {
    settings::get::<ThemeSettings>(cx).theme.clone()
}

pub fn init(source: impl AssetSource, cx: &mut AppContext) {
    cx.set_global(ThemeRegistry::new(source, cx.font_cache().clone()));
    settings::register::<ThemeSettings>(cx);

    let mut prev_buffer_font_size = settings::get::<ThemeSettings>(cx).buffer_font_size;
    cx.observe_global::<SettingsStore, _>(move |cx| {
        let buffer_font_size = settings::get::<ThemeSettings>(cx).buffer_font_size;
        if buffer_font_size != prev_buffer_font_size {
            prev_buffer_font_size = buffer_font_size;
            reset_font_size(cx);
        }
    })
    .detach();
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct Theme {
    #[serde(default)]
    pub meta: ThemeMeta,
    pub workspace: Workspace,
    pub context_menu: ContextMenu,
    pub toolbar_dropdown_menu: DropdownMenu,
    pub copilot: Copilot,
    pub collab_panel: CollabPanel,
    pub project_panel: ProjectPanel,
    pub command_palette: CommandPalette,
    pub picker: Picker,
    pub editor: Editor,
    pub search: Search,
    pub project_diagnostics: ProjectDiagnostics,
    pub shared_screen: ContainerStyle,
    pub contact_notification: ContactNotification,
    pub update_notification: UpdateNotification,
    pub simple_message_notification: MessageNotification,
    pub project_shared_notification: ProjectSharedNotification,
    pub incoming_call_notification: IncomingCallNotification,
    pub tooltip: TooltipStyle,
    pub terminal: TerminalStyle,
    pub assistant: AssistantStyle,
    pub feedback: FeedbackStyle,
    pub welcome: WelcomeStyle,
    pub titlebar: Titlebar,
    pub component_test: ComponentTest,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct ThemeMeta {
    #[serde(skip_deserializing)]
    pub id: usize,
    pub name: String,
    pub is_light: bool,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct Workspace {
    pub background: Color,
    pub blank_pane: BlankPaneStyle,
    pub tab_bar: TabBar,
    pub pane_divider: Border,
    pub leader_border_opacity: f32,
    pub leader_border_width: f32,
    pub dock: Dock,
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub breadcrumb_height: f32,
    pub breadcrumbs: Interactive<ContainedText>,
    pub disconnected_overlay: ContainedText,
    pub modal: ContainerStyle,
    pub zoomed_panel_foreground: ContainerStyle,
    pub zoomed_pane_foreground: ContainerStyle,
    pub zoomed_background: ContainerStyle,
    pub notification: ContainerStyle,
    pub notifications: Notifications,
    pub joining_project_avatar: ImageStyle,
    pub joining_project_message: ContainedText,
    pub external_location_message: ContainedText,
    pub drop_target_overlay_color: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct BlankPaneStyle {
    pub logo: SvgStyle,
    pub logo_shadow: SvgStyle,
    pub logo_container: ContainerStyle,
    pub keyboard_hints: ContainerStyle,
    pub keyboard_hint: Interactive<ContainedText>,
    pub keyboard_hint_width: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Titlebar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub menu: TitlebarMenu,
    pub project_menu_button: Toggleable<Interactive<ContainedText>>,
    pub project_name_divider: ContainedText,
    pub git_menu_button: Toggleable<Interactive<ContainedText>>,
    pub item_spacing: f32,
    pub face_pile_spacing: f32,
    pub avatar_ribbon: AvatarRibbon,
    pub follower_avatar_overlap: f32,
    pub leader_selection: ContainerStyle,
    pub offline_icon: OfflineIcon,
    pub leader_avatar: AvatarStyle,
    pub follower_avatar: AvatarStyle,
    pub inactive_avatar_grayscale: bool,
    pub sign_in_button: Toggleable<Interactive<ContainedText>>,
    pub outdated_warning: ContainedText,
    pub share_button: Toggleable<Interactive<ContainedText>>,
    pub muted: Color,
    pub speaking: Color,
    pub screen_share_button: Toggleable<Interactive<IconButton>>,
    pub toggle_contacts_button: Toggleable<Interactive<IconButton>>,
    pub toggle_microphone_button: Toggleable<Interactive<IconButton>>,
    pub toggle_speakers_button: Toggleable<Interactive<IconButton>>,
    pub leave_call_button: Interactive<IconButton>,
    pub toggle_contacts_badge: ContainerStyle,
    pub user_menu: UserMenu,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct TitlebarMenu {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct UserMenu {
    pub user_menu_button_online: UserMenuButton,
    pub user_menu_button_offline: UserMenuButton,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct UserMenuButton {
    pub user_menu: Toggleable<Interactive<Icon>>,
    pub avatar: AvatarStyle,
    pub icon: Icon,
}

#[derive(Copy, Clone, Deserialize, Default, JsonSchema)]
pub struct AvatarStyle {
    #[serde(flatten)]
    pub image: ImageStyle,
    pub outer_width: f32,
    pub outer_corner_radius: f32,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct Copilot {
    pub out_link_icon: Interactive<IconStyle>,
    pub modal: ModalStyle,
    pub auth: CopilotAuth,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct CopilotAuth {
    pub content_width: f32,
    pub prompting: CopilotAuthPrompting,
    pub not_authorized: CopilotAuthNotAuthorized,
    pub authorized: CopilotAuthAuthorized,
    pub cta_button: CopilotCTAButton,
    pub header: IconStyle,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct CopilotAuthPrompting {
    pub subheading: ContainedText,
    pub hint: ContainedText,
    pub device_code: DeviceCode,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct DeviceCode {
    pub text: TextStyle,
    pub cta: CopilotCTAButton,
    pub left: f32,
    pub left_container: ContainerStyle,
    pub right: f32,
    pub right_container: Interactive<ContainerStyle>,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct CopilotAuthNotAuthorized {
    pub subheading: ContainedText,
    pub warning: ContainedText,
}

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct CopilotAuthAuthorized {
    pub subheading: ContainedText,
    pub hint: ContainedText,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct CollabPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub disclosure: DisclosureStyle<()>,
    pub list_empty_state: Toggleable<Interactive<ContainedText>>,
    pub list_empty_icon: Icon,
    pub list_empty_label_container: ContainerStyle,
    pub log_in_button: Interactive<ContainedText>,
    pub channel_editor: ContainerStyle,
    pub channel_hash: Icon,
    pub tabbed_modal: TabbedModal,
    pub contact_finder: ContactFinder,
    pub channel_modal: ChannelModal,
    pub user_query_editor: FieldEditor,
    pub user_query_editor_height: f32,
    pub leave_call_button: Toggleable<Interactive<IconButton>>,
    pub add_contact_button: Toggleable<Interactive<IconButton>>,
    pub add_channel_button: Toggleable<Interactive<IconButton>>,
    pub header_row: ContainedText,
    pub subheader_row: Toggleable<Interactive<ContainedText>>,
    pub leave_call: Interactive<ContainedText>,
    pub contact_row: Toggleable<Interactive<ContainerStyle>>,
    pub channel_row: Toggleable<Interactive<ContainerStyle>>,
    pub channel_name: ContainedText,
    pub row_height: f32,
    pub project_row: Toggleable<Interactive<ProjectRow>>,
    pub tree_branch: Toggleable<Interactive<TreeBranch>>,
    pub contact_avatar: ImageStyle,
    pub channel_avatar: ImageStyle,
    pub extra_participant_label: ContainedText,
    pub contact_status_free: ContainerStyle,
    pub contact_status_busy: ContainerStyle,
    pub contact_username: ContainedText,
    pub contact_button: Interactive<IconButton>,
    pub contact_button_spacing: f32,
    pub channel_indent: f32,
    pub disabled_button: IconButton,
    pub section_icon_size: f32,
    pub calling_indicator: ContainedText,
    pub face_overlap: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ComponentTest {
    pub button: Interactive<ButtonStyle<TextStyle>>,
    pub toggle: Toggleable<Interactive<ButtonStyle<TextStyle>>>,
    pub disclosure: DisclosureStyle<TextStyle>,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct TabbedModal {
    pub tab_button: Toggleable<Interactive<ContainedText>>,
    pub modal: ContainerStyle,
    pub header: ContainerStyle,
    pub body: ContainerStyle,
    pub title: ContainedText,
    pub picker: Picker,
    pub max_height: f32,
    pub max_width: f32,
    pub row_height: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ChannelModal {
    pub contact_avatar: ImageStyle,
    pub contact_username: ContainerStyle,
    pub remove_member_button: ContainedText,
    pub cancel_invite_button: ContainedText,
    pub member_icon: IconButton,
    pub invitee_icon: IconButton,
    pub member_tag: ContainedText,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ProjectRow {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub icon: Icon,
    pub name: ContainedText,
}

#[derive(Deserialize, Default, Clone, Copy, JsonSchema)]
pub struct TreeBranch {
    pub width: f32,
    pub color: Color,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ContactFinder {
    pub contact_avatar: ImageStyle,
    pub contact_username: ContainerStyle,
    pub contact_button: IconButton,
    pub disabled_contact_button: IconButton,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct DropdownMenu {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub header: Interactive<DropdownMenuItem>,
    pub section_header: ContainedText,
    pub item: Toggleable<Interactive<DropdownMenuItem>>,
    pub row_height: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct DropdownMenuItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub text: TextStyle,
    pub secondary_text: Option<TextStyle>,
    #[serde(default)]
    pub secondary_text_spacing: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct TabBar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub pane_button: Toggleable<Interactive<IconButton>>,
    pub pane_button_container: ContainerStyle,
    pub active_pane: TabStyles,
    pub inactive_pane: TabStyles,
    pub dragged_tab: Tab,
    pub height: f32,
    pub nav_button: Interactive<IconButton>,
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct TabStyles {
    pub active_tab: Tab,
    pub inactive_tab: Tab,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct AvatarRibbon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct OfflineIcon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
    pub color: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Tab {
    pub height: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub description: ContainedText,
    pub spacing: f32,
    pub close_icon_width: f32,
    pub type_icon_width: f32,
    pub icon_close: Color,
    pub icon_close_active: Color,
    pub icon_dirty: Color,
    pub icon_conflict: Color,
    pub git: GitProjectStatus,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Toolbar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub item_spacing: f32,
    pub toggleable_tool: Toggleable<Interactive<IconButton>>,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Notifications {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub width: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Search {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub editor: FindEditor,
    pub invalid_editor: ContainerStyle,
    pub option_button_group: ContainerStyle,
    pub include_exclude_editor: FindEditor,
    pub invalid_include_exclude_editor: ContainerStyle,
    pub include_exclude_inputs: ContainedText,
    pub option_button: Toggleable<Interactive<IconButton>>,
    pub option_button_component: ToggleIconButtonStyle,
    pub action_button: Toggleable<Interactive<ContainedText>>,
    pub match_background: Color,
    pub match_index: ContainedText,
    pub major_results_status: TextStyle,
    pub minor_results_status: TextStyle,
    pub dismiss_button: Interactive<IconButton>,
    pub editor_icon: IconStyle,
    pub mode_button: Toggleable<Interactive<ContainedText>>,
    pub nav_button: Toggleable<Interactive<ContainedLabel>>,
    pub search_bar_row_height: f32,
    pub option_button_height: f32,
    pub modes_container: ContainerStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct FindEditor {
    #[serde(flatten)]
    pub input: FieldEditor,
    pub min_width: f32,
    pub max_width: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct StatusBar {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub item_spacing: f32,
    pub cursor_position: TextStyle,
    pub vim_mode_indicator: ContainedText,
    pub active_language: Interactive<ContainedText>,
    pub auto_update_progress_message: TextStyle,
    pub auto_update_done_message: TextStyle,
    pub lsp_status: Interactive<StatusBarLspStatus>,
    pub panel_buttons: StatusBarPanelButtons,
    pub diagnostic_summary: Interactive<StatusBarDiagnosticSummary>,
    pub diagnostic_message: Interactive<ContainedText>,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct StatusBarPanelButtons {
    pub group_left: ContainerStyle,
    pub group_bottom: ContainerStyle,
    pub group_right: ContainerStyle,
    pub button: Toggleable<Interactive<PanelButton>>,
}

#[derive(Deserialize, Default, JsonSchema)]
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

#[derive(Deserialize, Default, JsonSchema)]
pub struct StatusBarLspStatus {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub height: f32,
    pub icon_spacing: f32,
    pub icon_color: Color,
    pub icon_width: f32,
    pub message: TextStyle,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct Dock {
    pub left: ContainerStyle,
    pub bottom: ContainerStyle,
    pub right: ContainerStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct PanelButton {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub icon_color: Color,
    pub icon_size: f32,
    pub label: ContainedText,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ProjectPanel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub entry: Toggleable<Interactive<ProjectPanelEntry>>,
    pub dragged_entry: ProjectPanelEntry,
    pub ignored_entry: Toggleable<Interactive<ProjectPanelEntry>>,
    pub cut_entry: Toggleable<Interactive<ProjectPanelEntry>>,
    pub filename_editor: FieldEditor,
    pub indent_width: f32,
    pub open_project_button: Interactive<ContainedText>,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct ProjectPanelEntry {
    pub height: f32,
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    pub icon_size: f32,
    pub icon_color: Color,
    pub chevron_color: Color,
    pub chevron_size: f32,
    pub icon_spacing: f32,
    pub status: EntryStatus,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct EntryStatus {
    pub git: GitProjectStatus,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct GitProjectStatus {
    pub modified: Color,
    pub inserted: Color,
    pub conflict: Color,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct ContextMenu {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub item: Toggleable<Interactive<ContextMenuItem>>,
    pub keystroke_margin: f32,
    pub separator: ContainerStyle,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct ContextMenuItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub label: TextStyle,
    pub keystroke: ContainedText,
    pub icon_width: f32,
    pub icon_spacing: f32,
}

#[derive(Debug, Deserialize, Default, JsonSchema)]
pub struct CommandPalette {
    pub key: Toggleable<ContainedLabel>,
    pub keystroke_spacing: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct InviteLink {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon: Icon,
}

#[derive(Deserialize, Clone, Copy, Default, JsonSchema)]
pub struct Icon {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub width: f32,
}

#[derive(Deserialize, Clone, Copy, Default, JsonSchema)]
pub struct IconButton {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub color: Color,
    pub icon_width: f32,
    pub button_width: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ChatMessage {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub body: TextStyle,
    pub sender: ContainedText,
    pub timestamp: ContainedText,
}

#[derive(Deserialize, Default, JsonSchema)]
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

#[derive(Deserialize, Default, JsonSchema)]
pub struct ChannelName {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub hash: ContainedText,
    pub name: TextStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Picker {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty_container: ContainerStyle,
    pub input_editor: FieldEditor,
    pub empty_input_editor: FieldEditor,
    pub no_matches: ContainedLabel,
    pub item: Toggleable<Interactive<ContainedLabel>>,
    pub header: ContainedLabel,
    pub footer: Interactive<ContainedLabel>,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct ContainedText {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub text: TextStyle,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct ContainedLabel {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct ProjectDiagnostics {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub empty_message: TextStyle,
    pub tab_icon_width: f32,
    pub tab_icon_spacing: f32,
    pub tab_summary_spacing: f32,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct ContactNotification {
    pub header_avatar: ImageStyle,
    pub header_message: ContainedText,
    pub header_height: f32,
    pub body_message: ContainedText,
    pub button: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct UpdateNotification {
    pub message: ContainedText,
    pub action_message: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct MessageNotification {
    pub message: ContainedText,
    pub action_message: Interactive<ContainedText>,
    pub dismiss_button: Interactive<IconButton>,
}

#[derive(Deserialize, Default, JsonSchema)]
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

#[derive(Deserialize, Default, JsonSchema)]
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
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
    pub wrap_guide: Color,
    pub active_wrap_guide: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub guest_selections: Vec<SelectionStyle>,
    pub absent_selection: SelectionStyle,
    pub syntax: Arc<SyntaxTheme>,
    pub hint: HighlightStyle,
    pub suggestion: HighlightStyle,
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
    pub folds: Folds,
    pub unnecessary_code_fade: f32,
    pub hover_popover: HoverPopover,
    pub link_definition: HighlightStyle,
    pub composition_mark: HighlightStyle,
    pub jump_icon: Interactive<IconButton>,
    pub scrollbar: Scrollbar,
    pub whitespace: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Scrollbar {
    pub track: ContainerStyle,
    pub thumb: ContainerStyle,
    pub width: f32,
    pub min_height_factor: f32,
    pub git: BufferGitDiffColors,
    pub selections: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct BufferGitDiffColors {
    pub inserted: Color,
    pub modified: Color,
    pub deleted: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct DiagnosticPathHeader {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub filename: ContainedText,
    pub path: ContainedText,
    pub text_scale_factor: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct DiagnosticHeader {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub source: ContainedLabel,
    pub message: ContainedLabel,
    pub code: ContainedText,
    pub text_scale_factor: f32,
    pub icon_width_factor: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct DiagnosticStyle {
    pub message: LabelStyle,
    #[serde(default)]
    pub header: ContainerStyle,
    pub text_scale_factor: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct AutocompleteStyle {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub item: ContainerStyle,
    pub selected_item: ContainerStyle,
    pub hovered_item: ContainerStyle,
    pub match_highlight: HighlightStyle,
}

#[derive(Clone, Copy, Default, Deserialize, JsonSchema)]
pub struct SelectionStyle {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct FieldEditor {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    #[serde(default)]
    pub placeholder_text: Option<TextStyle>,
    pub selection: SelectionStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct InteractiveColor {
    pub color: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct CodeActions {
    #[serde(default)]
    pub indicator: Toggleable<Interactive<InteractiveColor>>,
    pub vertical_scale: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Folds {
    pub indicator: Toggleable<Interactive<InteractiveColor>>,
    pub ellipses: FoldEllipses,
    pub fold_background: Color,
    pub icon_margin_scale: f32,
    pub folded_icon: String,
    pub foldable_icon: String,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct FoldEllipses {
    pub text_color: Color,
    pub background: Interactive<InteractiveColor>,
    pub corner_radius_factor: f32,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct DiffStyle {
    pub inserted: Color,
    pub modified: Color,
    pub deleted: Color,
    pub removed_width_em: f32,
    pub width_em: f32,
    pub corner_radius: f32,
}

#[derive(Debug, Default, Clone, Copy, JsonSchema)]
pub struct Interactive<T> {
    pub default: T,
    pub hovered: Option<T>,
    pub clicked: Option<T>,
    pub disabled: Option<T>,
}

impl<T> Deref for Interactive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.default
    }
}

impl Interactive<()> {
    pub fn new_blank() -> Self {
        Self {
            default: (),
            hovered: None,
            clicked: None,
            disabled: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, JsonSchema)]
pub struct Toggleable<T> {
    active: T,
    inactive: T,
}

impl<T> Deref for Toggleable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inactive
    }
}

impl Toggleable<()> {
    pub fn new_blank() -> Self {
        Self {
            active: (),
            inactive: (),
        }
    }
}

impl<T> Toggleable<T> {
    pub fn new(active: T, inactive: T) -> Self {
        Self { active, inactive }
    }
    pub fn in_state(&self, active: bool) -> &T {
        if active {
            &self.active
        } else {
            &self.inactive
        }
    }
    pub fn active_state(&self) -> &T {
        self.in_state(true)
    }

    pub fn inactive_state(&self) -> &T {
        self.in_state(false)
    }
}

impl<T> Interactive<T> {
    pub fn style_for(&self, state: &mut MouseState) -> &T {
        if state.clicked() == Some(platform::MouseButton::Left) && self.clicked.is_some() {
            self.clicked.as_ref().unwrap()
        } else if state.hovered() {
            self.hovered.as_ref().unwrap_or(&self.default)
        } else {
            &self.default
        }
    }
    pub fn disabled_style(&self) -> &T {
        self.disabled.as_ref().unwrap_or(&self.default)
    }
}

impl<T> Toggleable<Interactive<T>> {
    pub fn style_for(&self, active: bool, state: &mut MouseState) -> &T {
        self.in_state(active).style_for(state)
    }

    pub fn default_style(&self) -> &T {
        &self.inactive.default
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Interactive<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            default: Value,
            hovered: Option<Value>,
            clicked: Option<Value>,
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

        let hovered = deserialize_state(json.hovered)?;
        let clicked = deserialize_state(json.clicked)?;
        let disabled = deserialize_state(json.disabled)?;
        let default = serde_json::from_value(json.default).map_err(serde::de::Error::custom)?;

        Ok(Interactive {
            default,
            hovered,
            clicked,
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

#[derive(Default, JsonSchema)]
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct HoverPopover {
    pub container: ContainerStyle,
    pub info_container: ContainerStyle,
    pub warning_container: ContainerStyle,
    pub error_container: ContainerStyle,
    pub block_style: ContainerStyle,
    pub prose: TextStyle,
    pub diagnostic_source_highlight: HighlightStyle,
    pub highlight: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct AssistantStyle {
    pub container: ContainerStyle,
    pub hamburger_button: Interactive<IconStyle>,
    pub split_button: Interactive<IconStyle>,
    pub assist_button: Interactive<IconStyle>,
    pub quote_button: Interactive<IconStyle>,
    pub zoom_in_button: Interactive<IconStyle>,
    pub zoom_out_button: Interactive<IconStyle>,
    pub plus_button: Interactive<IconStyle>,
    pub title: ContainedText,
    pub message_header: ContainerStyle,
    pub sent_at: ContainedText,
    pub user_sender: Interactive<ContainedText>,
    pub assistant_sender: Interactive<ContainedText>,
    pub system_sender: Interactive<ContainedText>,
    pub model: Interactive<ContainedText>,
    pub remaining_tokens: ContainedText,
    pub low_remaining_tokens: ContainedText,
    pub no_remaining_tokens: ContainedText,
    pub error_icon: Icon,
    pub api_key_editor: FieldEditor,
    pub api_key_prompt: ContainedText,
    pub saved_conversation: SavedConversation,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Contained<T> {
    container: ContainerStyle,
    contained: T,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct SavedConversation {
    pub container: Interactive<ContainerStyle>,
    pub saved_at: ContainedText,
    pub title: ContainedText,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct FeedbackStyle {
    pub submit_button: Interactive<ContainedText>,
    pub button_margin: f32,
    pub info_text_default: ContainedText,
    pub link_text_default: ContainedText,
    pub link_text_hover: ContainedText,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct WelcomeStyle {
    pub page_width: f32,
    pub logo: SvgStyle,
    pub logo_subheading: ContainedText,
    pub usage_note: ContainedText,
    pub checkbox: CheckboxStyle,
    pub checkbox_container: ContainerStyle,
    pub button: Interactive<ContainedText>,
    pub button_group: ContainerStyle,
    pub heading_group: ContainerStyle,
    pub checkbox_group: ContainerStyle,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Player {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
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

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Layer {
    pub base: StyleSet,
    pub variant: StyleSet,
    pub on: StyleSet,
    pub accent: StyleSet,
    pub positive: StyleSet,
    pub warning: StyleSet,
    pub negative: StyleSet,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct StyleSet {
    pub default: Style,
    pub active: Style,
    pub disabled: Style,
    pub hovered: Style,
    pub pressed: Style,
    pub inverted: Style,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Style {
    pub background: Color,
    pub border: Color,
    pub foreground: Color,
}
