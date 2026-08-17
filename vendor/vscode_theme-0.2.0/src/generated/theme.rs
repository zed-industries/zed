// This is a generated file.
// Do not modify by hand!

use crate::serde::empty_string_as_none;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// An extra border around active elements to separate them from others for greater contrast.
    #[serde(
        default,
        rename = "contrastActiveBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub contrast_active_border: Option<String>,
    /// An extra border around elements to separate them from others for greater contrast.
    #[serde(
        default,
        rename = "contrastBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub contrast_border: Option<String>,
    /// Overall border color for focused elements. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,
    /// Overall foreground color. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Overall foreground for disabled elements. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "disabledForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub disabled_foreground: Option<String>,
    /// Foreground color for description text providing additional information, for example for a label.
    #[serde(
        default,
        rename = "descriptionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub description_foreground: Option<String>,
    /// Overall foreground color for error messages (this color is only used if not overridden by a component).
    #[serde(
        default,
        rename = "errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    #[serde(flatten)]
    pub widget: WidgetColors,
    #[serde(flatten)]
    pub selection: SelectionColors,
    #[serde(flatten)]
    pub icon: IconColors,
    #[serde(flatten)]
    pub sash: SashColors,
    #[serde(flatten)]
    pub window: WindowColors,
    #[serde(flatten)]
    pub text_block_quote: TextBlockQuoteColors,
    #[serde(flatten)]
    pub text_code_block: TextCodeBlockColors,
    #[serde(flatten)]
    pub text_link: TextLinkColors,
    #[serde(flatten)]
    pub text_preformat: TextPreformatColors,
    #[serde(flatten)]
    pub text_separator: TextSeparatorColors,
    #[serde(flatten)]
    pub toolbar: ToolbarColors,
    #[serde(flatten)]
    pub button: ButtonColors,
    #[serde(flatten)]
    pub checkbox: CheckboxColors,
    #[serde(flatten)]
    pub dropdown: DropdownColors,
    #[serde(flatten)]
    pub input: InputColors,
    #[serde(flatten)]
    pub input_option: InputOptionColors,
    #[serde(flatten)]
    pub input_validation: InputValidationColors,
    #[serde(flatten)]
    pub scrollbar: ScrollbarColors,
    #[serde(flatten)]
    pub scrollbar_slider: ScrollbarSliderColors,
    #[serde(flatten)]
    pub badge: BadgeColors,
    #[serde(flatten)]
    pub progress_bar: ProgressBarColors,
    #[serde(flatten)]
    pub list: ListColors,
    #[serde(flatten)]
    pub list_filter_widget: ListFilterWidgetColors,
    #[serde(flatten)]
    pub tree: TreeColors,
    #[serde(flatten)]
    pub activity_bar: ActivityBarColors,
    #[serde(flatten)]
    pub activity_bar_badge: ActivityBarBadgeColors,
    #[serde(flatten)]
    pub profile_badge: ProfileBadgeColors,
    #[serde(flatten)]
    pub side_bar: SideBarColors,
    #[serde(flatten)]
    pub side_bar_title: SideBarTitleColors,
    #[serde(flatten)]
    pub side_bar_section_header: SideBarSectionHeaderColors,
    #[serde(flatten)]
    pub minimap: MinimapColors,
    #[serde(flatten)]
    pub minimap_slider: MinimapSliderColors,
    #[serde(flatten)]
    pub minimap_gutter: MinimapGutterColors,
    #[serde(flatten)]
    pub editor_group: EditorGroupColors,
    #[serde(flatten)]
    pub editor_group_header: EditorGroupHeaderColors,
    #[serde(flatten)]
    pub tab: TabColors,
    #[serde(flatten)]
    pub editor_pane: EditorPaneColors,
    #[serde(flatten)]
    pub side_by_side_editor: SideBySideEditorColors,
    #[serde(flatten)]
    pub editor: EditorColors,
    #[serde(flatten)]
    pub editor_line_number: EditorLineNumberColors,
    #[serde(flatten)]
    pub editor_cursor: EditorCursorColors,
    #[serde(flatten)]
    pub search: SearchColors,
    #[serde(flatten)]
    pub search_editor: SearchEditorColors,
    #[serde(flatten)]
    pub editor_unicode_highlight: EditorUnicodeHighlightColors,
    #[serde(flatten)]
    pub editor_link: EditorLinkColors,
    #[serde(flatten)]
    pub editor_whitespace: EditorWhitespaceColors,
    #[serde(flatten)]
    pub editor_indent_guide: EditorIndentGuideColors,
    #[serde(flatten)]
    pub editor_inlay_hint: EditorInlayHintColors,
    #[serde(flatten)]
    pub editor_ruler: EditorRulerColors,
    #[serde(flatten)]
    pub editor_code_lens: EditorCodeLensColors,
    #[serde(flatten)]
    pub editor_light_bulb: EditorLightBulbColors,
    #[serde(flatten)]
    pub editor_light_bulb_auto_fix: EditorLightBulbAutoFixColors,
    #[serde(flatten)]
    pub editor_light_bulb_ai: EditorLightBulbAiColors,
    #[serde(flatten)]
    pub editor_bracket_match: EditorBracketMatchColors,
    #[serde(flatten)]
    pub editor_bracket_highlight: EditorBracketHighlightColors,
    #[serde(flatten)]
    pub editor_bracket_pair_guide: EditorBracketPairGuideColors,
    #[serde(flatten)]
    pub editor_overview_ruler: EditorOverviewRulerColors,
    #[serde(flatten)]
    pub editor_error: EditorErrorColors,
    #[serde(flatten)]
    pub editor_warning: EditorWarningColors,
    #[serde(flatten)]
    pub editor_info: EditorInfoColors,
    #[serde(flatten)]
    pub editor_hint: EditorHintColors,
    #[serde(flatten)]
    pub problems_error_icon: ProblemsErrorIconColors,
    #[serde(flatten)]
    pub problems_warning_icon: ProblemsWarningIconColors,
    #[serde(flatten)]
    pub problems_info_icon: ProblemsInfoIconColors,
    #[serde(flatten)]
    pub editor_unnecessary_code: EditorUnnecessaryCodeColors,
    #[serde(flatten)]
    pub editor_gutter: EditorGutterColors,
    #[serde(flatten)]
    pub editor_comments_widget: EditorCommentsWidgetColors,
    #[serde(flatten)]
    pub diff_editor: DiffEditorColors,
    #[serde(flatten)]
    pub diff_editor_gutter: DiffEditorGutterColors,
    #[serde(flatten)]
    pub diff_editor_overview: DiffEditorOverviewColors,
    #[serde(flatten)]
    pub multi_diff_editor: MultiDiffEditorColors,
    #[serde(flatten)]
    pub chat: ChatColors,
    #[serde(flatten)]
    pub inline_chat: InlineChatColors,
    #[serde(flatten)]
    pub inline_chat_input: InlineChatInputColors,
    #[serde(flatten)]
    pub inline_chat_diff: InlineChatDiffColors,
    #[serde(flatten)]
    pub editor_widget: EditorWidgetColors,
    #[serde(flatten)]
    pub editor_suggest_widget: EditorSuggestWidgetColors,
    #[serde(flatten)]
    pub editor_suggest_widget_status: EditorSuggestWidgetStatusColors,
    #[serde(flatten)]
    pub editor_hover_widget: EditorHoverWidgetColors,
    #[serde(flatten)]
    pub editor_ghost_text: EditorGhostTextColors,
    #[serde(flatten)]
    pub editor_sticky_scroll: EditorStickyScrollColors,
    #[serde(flatten)]
    pub editor_sticky_scroll_hover: EditorStickyScrollHoverColors,
    #[serde(flatten)]
    pub debug_exception_widget: DebugExceptionWidgetColors,
    #[serde(flatten)]
    pub editor_marker_navigation: EditorMarkerNavigationColors,
    #[serde(flatten)]
    pub editor_marker_navigation_error: EditorMarkerNavigationErrorColors,
    #[serde(flatten)]
    pub editor_marker_navigation_warning: EditorMarkerNavigationWarningColors,
    #[serde(flatten)]
    pub editor_marker_navigation_info: EditorMarkerNavigationInfoColors,
    #[serde(flatten)]
    pub peek_view: PeekViewColors,
    #[serde(flatten)]
    pub peek_view_editor: PeekViewEditorColors,
    #[serde(flatten)]
    pub peek_view_editor_gutter: PeekViewEditorGutterColors,
    #[serde(flatten)]
    pub peek_view_result: PeekViewResultColors,
    #[serde(flatten)]
    pub peek_view_title: PeekViewTitleColors,
    #[serde(flatten)]
    pub peek_view_title_description: PeekViewTitleDescriptionColors,
    #[serde(flatten)]
    pub peek_view_title_label: PeekViewTitleLabelColors,
    #[serde(flatten)]
    pub peek_view_editor_sticky_scroll: PeekViewEditorStickyScrollColors,
    #[serde(flatten)]
    pub merge: MergeColors,
    #[serde(flatten)]
    pub merge_editor: MergeEditorColors,
    #[serde(flatten)]
    pub panel: PanelColors,
    #[serde(flatten)]
    pub panel_title: PanelTitleColors,
    #[serde(flatten)]
    pub panel_input: PanelInputColors,
    #[serde(flatten)]
    pub panel_section: PanelSectionColors,
    #[serde(flatten)]
    pub panel_section_header: PanelSectionHeaderColors,
    #[serde(flatten)]
    pub status_bar: StatusBarColors,
    #[serde(flatten)]
    pub status_bar_item: StatusBarItemColors,
    #[serde(flatten)]
    pub title_bar: TitleBarColors,
    #[serde(flatten)]
    pub menubar: MenubarColors,
    #[serde(flatten)]
    pub menu: MenuColors,
    #[serde(flatten)]
    pub command_center: CommandCenterColors,
    #[serde(flatten)]
    pub notification_center: NotificationCenterColors,
    #[serde(flatten)]
    pub notification_center_header: NotificationCenterHeaderColors,
    #[serde(flatten)]
    pub notification_toast: NotificationToastColors,
    #[serde(flatten)]
    pub notifications: NotificationsColors,
    #[serde(flatten)]
    pub notification_link: NotificationLinkColors,
    #[serde(flatten)]
    pub notifications_error_icon: NotificationsErrorIconColors,
    #[serde(flatten)]
    pub notifications_warning_icon: NotificationsWarningIconColors,
    #[serde(flatten)]
    pub notifications_info_icon: NotificationsInfoIconColors,
    #[serde(flatten)]
    pub banner: BannerColors,
    #[serde(flatten)]
    pub extension_button: ExtensionButtonColors,
    #[serde(flatten)]
    pub extension_badge: ExtensionBadgeColors,
    #[serde(flatten)]
    pub extension_icon: ExtensionIconColors,
    #[serde(flatten)]
    pub picker_group: PickerGroupColors,
    #[serde(flatten)]
    pub quick_input: QuickInputColors,
    #[serde(flatten)]
    pub quick_input_list: QuickInputListColors,
    #[serde(flatten)]
    pub quick_input_title: QuickInputTitleColors,
    #[serde(flatten)]
    pub keybinding_label: KeybindingLabelColors,
    #[serde(flatten)]
    pub keybinding_table: KeybindingTableColors,
    #[serde(flatten)]
    pub terminal: TerminalColors,
    #[serde(flatten)]
    pub terminal_cursor: TerminalCursorColors,
    #[serde(flatten)]
    pub terminal_command_decoration: TerminalCommandDecorationColors,
    #[serde(flatten)]
    pub terminal_overview_ruler: TerminalOverviewRulerColors,
    #[serde(flatten)]
    pub terminal_sticky_scroll: TerminalStickyScrollColors,
    #[serde(flatten)]
    pub terminal_sticky_scroll_hover: TerminalStickyScrollHoverColors,
    #[serde(flatten)]
    pub debug_tool_bar: DebugToolBarColors,
    #[serde(flatten)]
    pub debug_view: DebugViewColors,
    #[serde(flatten)]
    pub debug_token_expression: DebugTokenExpressionColors,
    #[serde(flatten)]
    pub testing: TestingColors,
    #[serde(flatten)]
    pub welcome_page: WelcomePageColors,
    #[serde(flatten)]
    pub walk_through: WalkThroughColors,
    #[serde(flatten)]
    pub walkthrough: WalkthroughColors,
    #[serde(flatten)]
    pub git_decoration: GitDecorationColors,
    #[serde(flatten)]
    pub settings: SettingsColors,
    #[serde(flatten)]
    pub breadcrumb: BreadcrumbColors,
    #[serde(flatten)]
    pub breadcrumb_picker: BreadcrumbPickerColors,
    #[serde(flatten)]
    pub symbol_icon: SymbolIconColors,
    #[serde(flatten)]
    pub debug_icon: DebugIconColors,
    #[serde(flatten)]
    pub debug_console: DebugConsoleColors,
    #[serde(flatten)]
    pub debug_console_input_icon: DebugConsoleInputIconColors,
    #[serde(flatten)]
    pub notebook: NotebookColors,
    #[serde(flatten)]
    pub notebook_scrollbar_slider: NotebookScrollbarSliderColors,
    #[serde(flatten)]
    pub notebook_status_error_icon: NotebookStatusErrorIconColors,
    #[serde(flatten)]
    pub notebook_status_running_icon: NotebookStatusRunningIconColors,
    #[serde(flatten)]
    pub notebook_status_success_icon: NotebookStatusSuccessIconColors,
    #[serde(flatten)]
    pub notebook_editor_overview_ruler: NotebookEditorOverviewRulerColors,
    #[serde(flatten)]
    pub charts: ChartsColors,
    #[serde(flatten)]
    pub ports: PortsColors,
    #[serde(flatten)]
    pub comments_view: CommentsViewColors,
    #[serde(flatten)]
    pub action_bar: ActionBarColors,
    #[serde(flatten)]
    pub simple_find_widget: SimpleFindWidgetColors,
    #[serde(flatten)]
    pub scm: ScmColors,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetColors {
    /// Border color of widgets such as Find/Replace inside the editor.
    #[serde(
        default,
        rename = "widget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Shadow color of widgets such as Find/Replace inside the editor.
    #[serde(
        default,
        rename = "widget.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionColors {
    /// Background color of text selections in the workbench (for input fields or text areas, does not apply to selections within the editor and the terminal).
    #[serde(
        default,
        rename = "selection.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconColors {
    /// The default color for icons in the workbench.
    #[serde(
        default,
        rename = "icon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SashColors {
    /// The hover border color for draggable sashes.
    #[serde(
        default,
        rename = "sash.hoverBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowColors {
    /// Border color for the active (focused) window.
    #[serde(
        default,
        rename = "window.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Border color for the inactive (unfocused) windows.
    #[serde(
        default,
        rename = "window.inactiveBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlockQuoteColors {
    /// Background color for block quotes in text.
    #[serde(
        default,
        rename = "textBlockQuote.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color for block quotes in text.
    #[serde(
        default,
        rename = "textBlockQuote.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextCodeBlockColors {
    /// Background color for code blocks in text.
    #[serde(
        default,
        rename = "textCodeBlock.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLinkColors {
    /// Foreground color for links in text when clicked on and on mouse hover.
    #[serde(
        default,
        rename = "textLink.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Foreground color for links in text.
    #[serde(
        default,
        rename = "textLink.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPreformatColors {
    /// Foreground color for preformatted text segments.
    #[serde(
        default,
        rename = "textPreformat.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color for preformatted text segments.
    #[serde(
        default,
        rename = "textPreformat.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSeparatorColors {
    /// Color for text separators.
    #[serde(
        default,
        rename = "textSeparator.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarColors {
    /// Toolbar background when hovering over actions using the mouse
    #[serde(
        default,
        rename = "toolbar.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Toolbar outline when hovering over actions using the mouse
    #[serde(
        default,
        rename = "toolbar.hoverOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_outline: Option<String>,
    /// Toolbar background when holding the mouse over actions
    #[serde(
        default,
        rename = "toolbar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColors {
    /// Button background color.
    #[serde(
        default,
        rename = "button.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Button foreground color.
    #[serde(
        default,
        rename = "button.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Button border color.
    #[serde(
        default,
        rename = "button.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Button separator color.
    #[serde(
        default,
        rename = "button.separator",
        deserialize_with = "empty_string_as_none"
    )]
    pub separator: Option<String>,
    /// Button background color when hovering.
    #[serde(
        default,
        rename = "button.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Secondary button foreground color.
    #[serde(
        default,
        rename = "button.secondaryForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_foreground: Option<String>,
    /// Secondary button background color.
    #[serde(
        default,
        rename = "button.secondaryBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_background: Option<String>,
    /// Secondary button background color when hovering.
    #[serde(
        default,
        rename = "button.secondaryHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxColors {
    /// Background color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of checkbox widget when the element it's in is selected.
    #[serde(
        default,
        rename = "checkbox.selectBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub select_background: Option<String>,
    /// Border color of checkbox widget when the element it's in is selected.
    #[serde(
        default,
        rename = "checkbox.selectBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub select_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownColors {
    /// Dropdown background.
    #[serde(
        default,
        rename = "dropdown.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Dropdown list background.
    #[serde(
        default,
        rename = "dropdown.listBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_background: Option<String>,
    /// Dropdown border.
    #[serde(
        default,
        rename = "dropdown.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Dropdown foreground.
    #[serde(
        default,
        rename = "dropdown.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputColors {
    /// Input box background.
    #[serde(
        default,
        rename = "input.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Input box border.
    #[serde(
        default,
        rename = "input.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Input box foreground.
    #[serde(
        default,
        rename = "input.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Input box foreground color for placeholder text.
    #[serde(
        default,
        rename = "input.placeholderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub placeholder_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOptionColors {
    /// Background color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Border color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Foreground color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Background color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationColors {
    /// Input validation background color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_background: Option<String>,
    /// Input validation foreground color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Input validation border color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_border: Option<String>,
    /// Input validation background color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_background: Option<String>,
    /// Input validation foreground color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_foreground: Option<String>,
    /// Input validation border color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_border: Option<String>,
    /// Input validation background color for information warning.
    #[serde(
        default,
        rename = "inputValidation.warningBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_background: Option<String>,
    /// Input validation foreground color for warning severity.
    #[serde(
        default,
        rename = "inputValidation.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Input validation border color for warning severity.
    #[serde(
        default,
        rename = "inputValidation.warningBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarColors {
    /// Scrollbar slider shadow to indicate that the view is scrolled.
    #[serde(
        default,
        rename = "scrollbar.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarSliderColors {
    /// Scrollbar slider background color when clicked on.
    #[serde(
        default,
        rename = "scrollbarSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Scrollbar slider background color.
    #[serde(
        default,
        rename = "scrollbarSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Scrollbar slider background color when hovering.
    #[serde(
        default,
        rename = "scrollbarSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeColors {
    /// Badge foreground color.
    #[serde(
        default,
        rename = "badge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Badge background color.
    #[serde(
        default,
        rename = "badge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarColors {
    /// Background color of the progress bar shown for long running operations.
    #[serde(
        default,
        rename = "progressBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListColors {
    /// List/Tree background color for the selected item when the list/tree is active.
    #[serde(
        default,
        rename = "list.activeSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is active.
    #[serde(
        default,
        rename = "list.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.activeSelectionIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_icon_foreground: Option<String>,
    /// List/Tree drag and drop background when moving items around using the mouse.
    #[serde(
        default,
        rename = "list.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
    /// List/Tree background color for the focused item when the list/tree is active.
    #[serde(
        default,
        rename = "list.focusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_background: Option<String>,
    /// List/Tree foreground color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// List/Tree foreground color of the match highlights on actively focused items when searching inside the list/tree.
    #[serde(
        default,
        rename = "list.focusHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_highlight_foreground: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_outline: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active and selected. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusAndSelectionOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_and_selection_outline: Option<String>,
    /// List/Tree foreground color of the match highlights when searching inside the list/tree.
    #[serde(
        default,
        rename = "list.highlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub highlight_foreground: Option<String>,
    /// List/Tree background when hovering over items using the mouse.
    #[serde(
        default,
        rename = "list.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// List/Tree foreground when hovering over items using the mouse.
    #[serde(
        default,
        rename = "list.hoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_foreground: Option<String>,
    /// List/Tree background color for the selected item when the list/tree is inactive.
    #[serde(
        default,
        rename = "list.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveSelectionIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_icon_foreground: Option<String>,
    /// List background color for the focused item when the list is inactive. An active list has keyboard focus, an inactive does not. Currently only supported in lists.
    #[serde(
        default,
        rename = "list.inactiveFocusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_focus_background: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveFocusOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_focus_outline: Option<String>,
    /// List/Tree foreground color for invalid items, for example an unresolved root in explorer.
    #[serde(
        default,
        rename = "list.invalidItemForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub invalid_item_foreground: Option<String>,
    /// Foreground color of list items containing errors.
    #[serde(
        default,
        rename = "list.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Foreground color of list items containing warnings.
    #[serde(
        default,
        rename = "list.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Background color of the filtered matches in lists and trees.
    #[serde(
        default,
        rename = "list.filterMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub filter_match_background: Option<String>,
    /// Border color of the filtered matches in lists and trees.
    #[serde(
        default,
        rename = "list.filterMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub filter_match_border: Option<String>,
    /// List/Tree foreground color for items that are deemphasized.
    #[serde(
        default,
        rename = "list.deemphasizedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deemphasized_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilterWidgetColors {
    /// List/Tree Filter background color of typed text when searching inside the list/tree.
    #[serde(
        default,
        rename = "listFilterWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// List/Tree Filter Widget's outline color of typed text when searching inside the list/tree.
    #[serde(
        default,
        rename = "listFilterWidget.outline",
        deserialize_with = "empty_string_as_none"
    )]
    pub outline: Option<String>,
    /// List/Tree Filter Widget's outline color when no match is found of typed text when searching inside the list/tree.
    #[serde(
        default,
        rename = "listFilterWidget.noMatchesOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_matches_outline: Option<String>,
    /// Shadow color of the type filter widget in lists and tree.
    #[serde(
        default,
        rename = "listFilterWidget.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeColors {
    /// Tree Widget's stroke color for indent guides.
    #[serde(
        default,
        rename = "tree.indentGuidesStroke",
        deserialize_with = "empty_string_as_none"
    )]
    pub indent_guides_stroke: Option<String>,
    /// Tree stroke color for the indentation guides that are not active.
    #[serde(
        default,
        rename = "tree.inactiveIndentGuidesStroke",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_indent_guides_stroke: Option<String>,
    /// Tree stroke color for the indentation guides.
    #[serde(
        default,
        rename = "tree.tableColumnsBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub table_columns_border: Option<String>,
    /// Background color for odd table rows.
    #[serde(
        default,
        rename = "tree.tableOddRowsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub table_odd_rows_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBarColors {
    /// Activity Bar background color.
    #[serde(
        default,
        rename = "activityBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Drag and drop feedback color for the activity bar items. The activity bar is showing on the far left or right and allows to switch between views of the side bar.
    #[serde(
        default,
        rename = "activityBar.dropBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_border: Option<String>,
    /// Activity Bar foreground color (for example used for the icons).
    #[serde(
        default,
        rename = "activityBar.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Activity Bar item foreground color when it is inactive.
    #[serde(
        default,
        rename = "activityBar.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_foreground: Option<String>,
    /// Activity Bar border color with the Side Bar.
    #[serde(
        default,
        rename = "activityBar.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Activity Bar active indicator border color.
    #[serde(
        default,
        rename = "activityBar.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Activity Bar optional background color for the active element.
    #[serde(
        default,
        rename = "activityBar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Activity bar focus border color for the active item.
    #[serde(
        default,
        rename = "activityBar.activeFocusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_focus_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBarBadgeColors {
    /// Activity notification badge background color.
    #[serde(
        default,
        rename = "activityBarBadge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Activity notification badge foreground color.
    #[serde(
        default,
        rename = "activityBarBadge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBadgeColors {
    /// Profile badge background color. The profile badge shows on top of the settings gear icon in the activity bar.
    #[serde(
        default,
        rename = "profileBadge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Profile badge foreground color. The profile badge shows on top of the settings gear icon in the activity bar.
    #[serde(
        default,
        rename = "profileBadge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBarColors {
    /// Side Bar background color.
    #[serde(
        default,
        rename = "sideBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Side Bar foreground color. The Side Bar is the container for views like Explorer and Search.
    #[serde(
        default,
        rename = "sideBar.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Side Bar border color on the side separating the editor.
    #[serde(
        default,
        rename = "sideBar.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Drag and drop feedback color for the side bar sections. The color should have transparency so that the side bar sections can still shine through.
    #[serde(
        default,
        rename = "sideBar.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBarTitleColors {
    /// Side Bar title foreground color.
    #[serde(
        default,
        rename = "sideBarTitle.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBarSectionHeaderColors {
    /// Side Bar section header background color.
    #[serde(
        default,
        rename = "sideBarSectionHeader.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Side Bar section header foreground color.
    #[serde(
        default,
        rename = "sideBarSectionHeader.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Side bar section header border color.
    #[serde(
        default,
        rename = "sideBarSectionHeader.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapColors {
    /// Highlight color for matches from search within files.
    #[serde(
        default,
        rename = "minimap.findMatchHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight: Option<String>,
    /// Highlight color for the editor selection.
    #[serde(
        default,
        rename = "minimap.selectionHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight: Option<String>,
    /// Highlight color for errors within the editor.
    #[serde(
        default,
        rename = "minimap.errorHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_highlight: Option<String>,
    /// Highlight color for warnings within the editor.
    #[serde(
        default,
        rename = "minimap.warningHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_highlight: Option<String>,
    /// Minimap background color.
    #[serde(
        default,
        rename = "minimap.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Minimap marker color for repeating editor selections.
    #[serde(
        default,
        rename = "minimap.selectionOccurrenceHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_occurrence_highlight: Option<String>,
    /// Opacity of foreground elements rendered in the minimap. For example, "#000000c0" will render the elements with 75% opacity.
    #[serde(
        default,
        rename = "minimap.foregroundOpacity",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground_opacity: Option<String>,
    /// Minimap marker color for infos.
    #[serde(
        default,
        rename = "minimap.infoHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_highlight: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapSliderColors {
    /// Minimap slider background color.
    #[serde(
        default,
        rename = "minimapSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Minimap slider background color when hovering.
    #[serde(
        default,
        rename = "minimapSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Minimap slider background color when clicked on.
    #[serde(
        default,
        rename = "minimapSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapGutterColors {
    /// Minimap gutter color for added content.
    #[serde(
        default,
        rename = "minimapGutter.addedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub added_background: Option<String>,
    /// Minimap gutter color for modified content.
    #[serde(
        default,
        rename = "minimapGutter.modifiedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub modified_background: Option<String>,
    /// Minimap gutter color for deleted content.
    #[serde(
        default,
        rename = "minimapGutter.deletedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deleted_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGroupColors {
    /// Color to separate multiple editor groups from each other.
    #[serde(
        default,
        rename = "editorGroup.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color when dragging editors around.
    #[serde(
        default,
        rename = "editorGroup.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
    /// Background color of an empty editor group.
    #[serde(
        default,
        rename = "editorGroup.emptyBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub empty_background: Option<String>,
    /// Border color of an empty editor group that is focused.
    #[serde(
        default,
        rename = "editorGroup.focusedEmptyBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_empty_border: Option<String>,
    /// Foreground color of text shown over editors when dragging files. This text informs the user that they can hold shift to drop into the editor.
    #[serde(
        default,
        rename = "editorGroup.dropIntoPromptForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_into_prompt_foreground: Option<String>,
    /// Background color of text shown over editors when dragging files. This text informs the user that they can hold shift to drop into the editor.
    #[serde(
        default,
        rename = "editorGroup.dropIntoPromptBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_into_prompt_background: Option<String>,
    /// Border color of text shown over editors when dragging files. This text informs the user that they can hold shift to drop into the editor.
    #[serde(
        default,
        rename = "editorGroup.dropIntoPromptBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_into_prompt_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGroupHeaderColors {
    /// Background color of the editor group title header when using single Tab (set `"workbench.editor.showTabs": "single"`).
    #[serde(
        default,
        rename = "editorGroupHeader.noTabsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_tabs_background: Option<String>,
    /// Background color of the Tabs container.
    #[serde(
        default,
        rename = "editorGroupHeader.tabsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tabs_background: Option<String>,
    /// Border color below the editor tabs control when tabs are enabled.
    #[serde(
        default,
        rename = "editorGroupHeader.tabsBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub tabs_border: Option<String>,
    /// Border color between editor group header and editor (below breadcrumbs if enabled).
    #[serde(
        default,
        rename = "editorGroupHeader.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabColors {
    /// Active Tab background color in an active group.
    #[serde(
        default,
        rename = "tab.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Active Tab background color in an inactive editor group.
    #[serde(
        default,
        rename = "tab.unfocusedActiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_active_background: Option<String>,
    /// Active Tab foreground color in an active group.
    #[serde(
        default,
        rename = "tab.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Border to separate Tabs from each other.
    #[serde(
        default,
        rename = "tab.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Bottom border for the active tab.
    #[serde(
        default,
        rename = "tab.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Bottom border for the active tab in an inactive editor group.
    #[serde(
        default,
        rename = "tab.unfocusedActiveBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_active_border: Option<String>,
    /// Top border for the active tab.
    #[serde(
        default,
        rename = "tab.activeBorderTop",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border_top: Option<String>,
    /// Top border for the active tab in an inactive editor group
    #[serde(
        default,
        rename = "tab.unfocusedActiveBorderTop",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_active_border_top: Option<String>,
    /// Border on the right of the last pinned editor to separate from unpinned editors.
    #[serde(
        default,
        rename = "tab.lastPinnedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub last_pinned_border: Option<String>,
    /// Inactive Tab background color.
    #[serde(
        default,
        rename = "tab.inactiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_background: Option<String>,
    /// Inactive Tab background color in an unfocused group
    #[serde(
        default,
        rename = "tab.unfocusedInactiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_inactive_background: Option<String>,
    /// Inactive Tab foreground color in an active group.
    #[serde(
        default,
        rename = "tab.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_foreground: Option<String>,
    /// Active tab foreground color in an inactive editor group.
    #[serde(
        default,
        rename = "tab.unfocusedActiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_active_foreground: Option<String>,
    /// Inactive tab foreground color in an inactive editor group.
    #[serde(
        default,
        rename = "tab.unfocusedInactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_inactive_foreground: Option<String>,
    /// Tab background color when hovering
    #[serde(
        default,
        rename = "tab.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Tab background color in an unfocused group when hovering
    #[serde(
        default,
        rename = "tab.unfocusedHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_hover_background: Option<String>,
    /// Tab foreground color when hovering
    #[serde(
        default,
        rename = "tab.hoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_foreground: Option<String>,
    /// Tab foreground color in an unfocused group when hovering
    #[serde(
        default,
        rename = "tab.unfocusedHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_hover_foreground: Option<String>,
    /// Border to highlight tabs when hovering
    #[serde(
        default,
        rename = "tab.hoverBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_border: Option<String>,
    /// Border to highlight tabs in an unfocused group when hovering
    #[serde(
        default,
        rename = "tab.unfocusedHoverBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_hover_border: Option<String>,
    /// Border on the top of modified (dirty) active tabs in an active group.
    #[serde(
        default,
        rename = "tab.activeModifiedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_modified_border: Option<String>,
    /// Border on the top of modified (dirty) inactive tabs in an active group.
    #[serde(
        default,
        rename = "tab.inactiveModifiedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_modified_border: Option<String>,
    /// Border on the top of modified (dirty) active tabs in an unfocused group.
    #[serde(
        default,
        rename = "tab.unfocusedActiveModifiedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_active_modified_border: Option<String>,
    /// Border on the top of modified (dirty) inactive tabs in an unfocused group.
    #[serde(
        default,
        rename = "tab.unfocusedInactiveModifiedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub unfocused_inactive_modified_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorPaneColors {
    /// Background color of the editor pane visible on the left and right side of the centered editor layout.
    #[serde(
        default,
        rename = "editorPane.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBySideEditorColors {
    /// Color to separate two editors from each other when shown side by side in an editor group from top to bottom.
    #[serde(
        default,
        rename = "sideBySideEditor.horizontalBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub horizontal_border: Option<String>,
    /// Color to separate two editors from each other when shown side by side in an editor group from left to right.
    #[serde(
        default,
        rename = "sideBySideEditor.verticalBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub vertical_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorColors {
    /// Editor background color.
    #[serde(
        default,
        rename = "editor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor default foreground color.
    #[serde(
        default,
        rename = "editor.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color of the editor selection.
    #[serde(
        default,
        rename = "editor.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Color of the selected text for high contrast.
    #[serde(
        default,
        rename = "editor.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// Color of the selection in an inactive editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_background: Option<String>,
    /// Color for regions with the same content as the selection. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.selectionHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_background: Option<String>,
    /// Border color for regions with the same content as the selection.
    #[serde(
        default,
        rename = "editor.selectionHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_border: Option<String>,
    /// Background color of a symbol during read-access, for example when reading a variable. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.wordHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_background: Option<String>,
    /// Border color of a symbol during read-access, for example when reading a variable.
    #[serde(
        default,
        rename = "editor.wordHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_border: Option<String>,
    /// Background color of a symbol during write-access, for example when writing to a variable. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.wordHighlightStrongBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_strong_background: Option<String>,
    /// Border color of a symbol during write-access, for example when writing to a variable.
    #[serde(
        default,
        rename = "editor.wordHighlightStrongBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_strong_border: Option<String>,
    /// Background color of a textual occurrence for a symbol. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.wordHighlightTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_text_background: Option<String>,
    /// Border color of a textual occurrence for a symbol.
    #[serde(
        default,
        rename = "editor.wordHighlightTextBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_text_border: Option<String>,
    /// Color of the current search match.
    #[serde(
        default,
        rename = "editor.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_background: Option<String>,
    /// Color of the other search matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.findMatchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_background: Option<String>,
    /// Color the range limiting the search (Enable 'Find in Selection' in the find widget). The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.findRangeHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_range_highlight_background: Option<String>,
    /// Border color of the current search match.
    #[serde(
        default,
        rename = "editor.findMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_border: Option<String>,
    /// Border color of the other search matches.
    #[serde(
        default,
        rename = "editor.findMatchHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_border: Option<String>,
    /// Border color the range limiting the search (Enable 'Find in Selection' in the find widget).
    #[serde(
        default,
        rename = "editor.findRangeHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_range_highlight_border: Option<String>,
    /// Highlight below the word for which a hover is shown. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.hoverHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_highlight_background: Option<String>,
    /// Background color for the highlight of line at the cursor position.
    #[serde(
        default,
        rename = "editor.lineHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub line_highlight_background: Option<String>,
    /// Background color for the border around the line at the cursor position.
    #[serde(
        default,
        rename = "editor.lineHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub line_highlight_border: Option<String>,
    /// Background color of highlighted ranges, used by Quick Open, Symbol in File and Find features. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.rangeHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub range_highlight_background: Option<String>,
    /// Background color of the border around highlighted ranges.
    #[serde(
        default,
        rename = "editor.rangeHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub range_highlight_border: Option<String>,
    /// Background color of highlighted symbol. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.symbolHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub symbol_highlight_background: Option<String>,
    /// Background color of the border around highlighted symbols.
    #[serde(
        default,
        rename = "editor.symbolHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub symbol_highlight_border: Option<String>,
    /// Background color when the editor is in linked editing mode.
    #[serde(
        default,
        rename = "editor.linkedEditingBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub linked_editing_background: Option<String>,
    /// Background color for folded ranges. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.foldBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub fold_background: Option<String>,
    /// Background color of the top stack frame highlight in the editor.
    #[serde(
        default,
        rename = "editor.stackFrameHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub stack_frame_highlight_background: Option<String>,
    /// Background color of the focused stack frame highlight in the editor.
    #[serde(
        default,
        rename = "editor.focusedStackFrameHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_stack_frame_highlight_background: Option<String>,
    /// Color for the debug inline value text.
    #[serde(
        default,
        rename = "editor.inlineValuesForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inline_values_foreground: Option<String>,
    /// Color for the debug inline value background.
    #[serde(
        default,
        rename = "editor.inlineValuesBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inline_values_background: Option<String>,
    /// Highlight background color of a snippet tabstop.
    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_tabstop_highlight_background: Option<String>,
    /// Highlight border color of a snippet tabstop.
    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_tabstop_highlight_border: Option<String>,
    /// Highlight background color of the final tabstop of a snippet.
    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_final_tabstop_highlight_background: Option<String>,
    /// Highlight border color of the final tabstop of a snippet.
    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_final_tabstop_highlight_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLineNumberColors {
    /// Color of editor line numbers.
    #[serde(
        default,
        rename = "editorLineNumber.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color of the active editor line number.
    #[serde(
        default,
        rename = "editorLineNumber.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Color of the final editor line when editor.renderFinalNewline is set to dimmed.
    #[serde(
        default,
        rename = "editorLineNumber.dimmedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub dimmed_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorCursorColors {
    /// The background color of the editor cursor. Allows customizing the color of a character overlapped by a block cursor.
    #[serde(
        default,
        rename = "editorCursor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color of the editor cursor.
    #[serde(
        default,
        rename = "editorCursor.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchColors {
    /// Color of the text in the search viewlet's completion message. For example, this color is used in the text that says "`{x} results in {y} files`".
    #[serde(
        default,
        rename = "search.resultsInfoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub results_info_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEditorColors {
    /// Color of the editor's results.
    #[serde(
        default,
        rename = "searchEditor.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_background: Option<String>,
    /// Border color of the editor's results.
    #[serde(
        default,
        rename = "searchEditor.findMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_border: Option<String>,
    /// Search editor text input box border.
    #[serde(
        default,
        rename = "searchEditor.textInputBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub text_input_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorUnicodeHighlightColors {
    /// Border color used to highlight unicode characters.
    #[serde(
        default,
        rename = "editorUnicodeHighlight.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color used to highlight unicode characters.
    #[serde(
        default,
        rename = "editorUnicodeHighlight.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLinkColors {
    /// Color of active links.
    #[serde(
        default,
        rename = "editorLink.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWhitespaceColors {
    /// Color of whitespace characters in the editor.
    #[serde(
        default,
        rename = "editorWhitespace.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorIndentGuideColors {
    /// Color of the editor indentation guides.
    #[serde(
        default,
        rename = "editorIndentGuide.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color of the editor indentation guides (1).
    #[serde(
        default,
        rename = "editorIndentGuide.background1",
        deserialize_with = "empty_string_as_none"
    )]
    pub background1: Option<String>,
    /// Color of the editor indentation guides (2).
    #[serde(
        default,
        rename = "editorIndentGuide.background2",
        deserialize_with = "empty_string_as_none"
    )]
    pub background2: Option<String>,
    /// Color of the editor indentation guides (3).
    #[serde(
        default,
        rename = "editorIndentGuide.background3",
        deserialize_with = "empty_string_as_none"
    )]
    pub background3: Option<String>,
    /// Color of the editor indentation guides (4).
    #[serde(
        default,
        rename = "editorIndentGuide.background4",
        deserialize_with = "empty_string_as_none"
    )]
    pub background4: Option<String>,
    /// Color of the editor indentation guides (5).
    #[serde(
        default,
        rename = "editorIndentGuide.background5",
        deserialize_with = "empty_string_as_none"
    )]
    pub background5: Option<String>,
    /// Color of the editor indentation guides (6).
    #[serde(
        default,
        rename = "editorIndentGuide.background6",
        deserialize_with = "empty_string_as_none"
    )]
    pub background6: Option<String>,
    /// Color of the active editor indentation guide.
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Color of the active editor indentation guides (1).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground1",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background1: Option<String>,
    /// Color of the active editor indentation guides (2).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground2",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background2: Option<String>,
    /// Color of the active editor indentation guides (3).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground3",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background3: Option<String>,
    /// Color of the active editor indentation guides (4).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground4",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background4: Option<String>,
    /// Color of the active editor indentation guides (5).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground5",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background5: Option<String>,
    /// Color of the active editor indentation guides (6).
    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground6",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background6: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInlayHintColors {
    /// Background color of inline hints.
    #[serde(
        default,
        rename = "editorInlayHint.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of inline hints.
    #[serde(
        default,
        rename = "editorInlayHint.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Foreground color of inline hints for types
    #[serde(
        default,
        rename = "editorInlayHint.typeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub type_foreground: Option<String>,
    /// Background color of inline hints for types
    #[serde(
        default,
        rename = "editorInlayHint.typeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub type_background: Option<String>,
    /// Foreground color of inline hints for parameters
    #[serde(
        default,
        rename = "editorInlayHint.parameterForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub parameter_foreground: Option<String>,
    /// Background color of inline hints for parameters
    #[serde(
        default,
        rename = "editorInlayHint.parameterBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub parameter_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorRulerColors {
    /// Color of the editor rulers.
    #[serde(
        default,
        rename = "editorRuler.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorCodeLensColors {
    /// Foreground color of an editor CodeLens.
    #[serde(
        default,
        rename = "editorCodeLens.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbColors {
    /// The color used for the lightbulb actions icon.
    #[serde(
        default,
        rename = "editorLightBulb.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAutoFixColors {
    /// The color used for the lightbulb auto fix actions icon.
    #[serde(
        default,
        rename = "editorLightBulbAutoFix.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAiColors {
    /// The color used for the lightbulb AI icon.
    #[serde(
        default,
        rename = "editorLightBulbAi.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorBracketMatchColors {
    /// Background color behind matching brackets.
    #[serde(
        default,
        rename = "editorBracketMatch.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color for matching brackets boxes.
    #[serde(
        default,
        rename = "editorBracketMatch.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorBracketHighlightColors {
    /// Foreground color of brackets (1). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground1",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground1: Option<String>,
    /// Foreground color of brackets (2). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground2",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground2: Option<String>,
    /// Foreground color of brackets (3). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground3",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground3: Option<String>,
    /// Foreground color of brackets (4). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground4",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground4: Option<String>,
    /// Foreground color of brackets (5). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground5",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground5: Option<String>,
    /// Foreground color of brackets (6). Requires enabling bracket pair colorization.
    #[serde(
        default,
        rename = "editorBracketHighlight.foreground6",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground6: Option<String>,
    /// Foreground color of unexpected brackets.
    #[serde(
        default,
        rename = "editorBracketHighlight.unexpectedBracket.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unexpected_bracket_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorBracketPairGuideColors {
    /// Background color of active bracket pair guides (1). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground1",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background1: Option<String>,
    /// Background color of active bracket pair guides (2). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground2",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background2: Option<String>,
    /// Background color of active bracket pair guides (3). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground3",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background3: Option<String>,
    /// Background color of active bracket pair guides (4). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground4",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background4: Option<String>,
    /// Background color of active bracket pair guides (5). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground5",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background5: Option<String>,
    /// Background color of active bracket pair guides (6). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.activeBackground6",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background6: Option<String>,
    /// Background color of inactive bracket pair guides (1). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background1",
        deserialize_with = "empty_string_as_none"
    )]
    pub background1: Option<String>,
    /// Background color of inactive bracket pair guides (2). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background2",
        deserialize_with = "empty_string_as_none"
    )]
    pub background2: Option<String>,
    /// Background color of inactive bracket pair guides (3). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background3",
        deserialize_with = "empty_string_as_none"
    )]
    pub background3: Option<String>,
    /// Background color of inactive bracket pair guides (4). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background4",
        deserialize_with = "empty_string_as_none"
    )]
    pub background4: Option<String>,
    /// Background color of inactive bracket pair guides (5). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background5",
        deserialize_with = "empty_string_as_none"
    )]
    pub background5: Option<String>,
    /// Background color of inactive bracket pair guides (6). Requires enabling bracket pair guides.
    #[serde(
        default,
        rename = "editorBracketPairGuide.background6",
        deserialize_with = "empty_string_as_none"
    )]
    pub background6: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorOverviewRulerColors {
    /// Background color of the editor overview ruler. Only used when the minimap is enabled and placed on the right side of the editor.
    #[serde(
        default,
        rename = "editorOverviewRuler.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color of the overview ruler border.
    #[serde(
        default,
        rename = "editorOverviewRuler.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Overview ruler marker color for find matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.findMatchForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_foreground: Option<String>,
    /// Overview ruler marker color for highlighted ranges, like by the Quick Open, Symbol in File and Find features. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.rangeHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub range_highlight_foreground: Option<String>,
    /// Overview ruler marker color for selection highlights. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.selectionHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_foreground: Option<String>,
    /// Overview ruler marker color for symbol highlights. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.wordHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_foreground: Option<String>,
    /// Overview ruler marker color for write-access symbol highlights. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.wordHighlightStrongForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_strong_foreground: Option<String>,
    /// Overview ruler marker color of a textual occurrence for a symbol. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.wordHighlightTextForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub word_highlight_text_foreground: Option<String>,
    /// Overview ruler marker color for modified content.
    #[serde(
        default,
        rename = "editorOverviewRuler.modifiedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub modified_foreground: Option<String>,
    /// Overview ruler marker color for added content.
    #[serde(
        default,
        rename = "editorOverviewRuler.addedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub added_foreground: Option<String>,
    /// Overview ruler marker color for deleted content.
    #[serde(
        default,
        rename = "editorOverviewRuler.deletedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deleted_foreground: Option<String>,
    /// Overview ruler marker color for errors.
    #[serde(
        default,
        rename = "editorOverviewRuler.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Overview ruler marker color for warnings.
    #[serde(
        default,
        rename = "editorOverviewRuler.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Overview ruler marker color for infos.
    #[serde(
        default,
        rename = "editorOverviewRuler.infoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_foreground: Option<String>,
    /// Overview ruler marker color for matching brackets.
    #[serde(
        default,
        rename = "editorOverviewRuler.bracketMatchForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub bracket_match_foreground: Option<String>,
    /// Current overview ruler foreground for inline merge conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.currentContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_content_foreground: Option<String>,
    /// Incoming overview ruler foreground for inline merge conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.incomingContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_content_foreground: Option<String>,
    /// Common ancestor overview ruler foreground for inline merge conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.commonContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_content_foreground: Option<String>,
    /// Editor overview ruler decoration color for resolved comments. This color should be opaque.
    #[serde(
        default,
        rename = "editorOverviewRuler.commentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub comment_foreground: Option<String>,
    /// Editor overview ruler decoration color for unresolved comments. This color should be opaque.
    #[serde(
        default,
        rename = "editorOverviewRuler.commentUnresolvedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub comment_unresolved_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorErrorColors {
    /// Foreground color of error squiggles in the editor.
    #[serde(
        default,
        rename = "editorError.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of error boxes in the editor.
    #[serde(
        default,
        rename = "editorError.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of error text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorError.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWarningColors {
    /// Foreground color of warning squiggles in the editor.
    #[serde(
        default,
        rename = "editorWarning.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of warning boxes in the editor.
    #[serde(
        default,
        rename = "editorWarning.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of warning text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorWarning.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInfoColors {
    /// Foreground color of info squiggles in the editor.
    #[serde(
        default,
        rename = "editorInfo.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of info boxes in the editor.
    #[serde(
        default,
        rename = "editorInfo.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of info text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorInfo.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHintColors {
    /// Foreground color of hints in the editor.
    #[serde(
        default,
        rename = "editorHint.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of hint boxes in the editor.
    #[serde(
        default,
        rename = "editorHint.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsErrorIconColors {
    /// The color used for the problems error icon.
    #[serde(
        default,
        rename = "problemsErrorIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsWarningIconColors {
    /// The color used for the problems warning icon.
    #[serde(
        default,
        rename = "problemsWarningIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsInfoIconColors {
    /// The color used for the problems info icon.
    #[serde(
        default,
        rename = "problemsInfoIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorUnnecessaryCodeColors {
    /// Border color of unnecessary (unused) source code in the editor.
    #[serde(
        default,
        rename = "editorUnnecessaryCode.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Opacity of unnecessary (unused) source code in the editor. For example, `"#000000c0"` will render the code with 75% opacity. For high contrast themes, use the `"editorUnnecessaryCode.border"` theme color to underline unnecessary code instead of fading it out.
    #[serde(
        default,
        rename = "editorUnnecessaryCode.opacity",
        deserialize_with = "empty_string_as_none"
    )]
    pub opacity: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGutterColors {
    /// Background color of the editor gutter. The gutter contains the glyph margins and the line numbers.
    #[serde(
        default,
        rename = "editorGutter.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor gutter background color for lines that are modified.
    #[serde(
        default,
        rename = "editorGutter.modifiedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub modified_background: Option<String>,
    /// Editor gutter background color for lines that are added.
    #[serde(
        default,
        rename = "editorGutter.addedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub added_background: Option<String>,
    /// Editor gutter background color for lines that are deleted.
    #[serde(
        default,
        rename = "editorGutter.deletedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deleted_background: Option<String>,
    /// Editor gutter decoration color for commenting ranges.
    #[serde(
        default,
        rename = "editorGutter.commentRangeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub comment_range_foreground: Option<String>,
    /// Editor gutter decoration color for commenting glyphs.
    #[serde(
        default,
        rename = "editorGutter.commentGlyphForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub comment_glyph_foreground: Option<String>,
    /// Editor gutter decoration color for commenting glyphs for unresolved comment threads.
    #[serde(
        default,
        rename = "editorGutter.commentUnresolvedGlyphForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub comment_unresolved_glyph_foreground: Option<String>,
    /// Color of the folding control in the editor gutter.
    #[serde(
        default,
        rename = "editorGutter.foldingControlForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub folding_control_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorCommentsWidgetColors {
    /// Color of borders and arrow for resolved comments.
    #[serde(
        default,
        rename = "editorCommentsWidget.resolvedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub resolved_border: Option<String>,
    /// Color of borders and arrow for unresolved comments.
    #[serde(
        default,
        rename = "editorCommentsWidget.unresolvedBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub unresolved_border: Option<String>,
    /// Color of background for comment ranges.
    #[serde(
        default,
        rename = "editorCommentsWidget.rangeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub range_background: Option<String>,
    /// Color of background for currently selected or hovered comment range.
    #[serde(
        default,
        rename = "editorCommentsWidget.rangeActiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub range_active_background: Option<String>,
    /// Background color for comment reply input box.
    #[serde(
        default,
        rename = "editorCommentsWidget.replyInputBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub reply_input_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorColors {
    /// Background color for text that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.insertedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_text_background: Option<String>,
    /// Outline color for the text that got inserted.
    #[serde(
        default,
        rename = "diffEditor.insertedTextBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_text_border: Option<String>,
    /// Background color for text that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.removedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_text_background: Option<String>,
    /// Outline color for text that got removed.
    #[serde(
        default,
        rename = "diffEditor.removedTextBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_text_border: Option<String>,
    /// Border color between the two text editors.
    #[serde(
        default,
        rename = "diffEditor.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Color of the diff editor's diagonal fill. The diagonal fill is used in side-by-side diff views.
    #[serde(
        default,
        rename = "diffEditor.diagonalFill",
        deserialize_with = "empty_string_as_none"
    )]
    pub diagonal_fill: Option<String>,
    /// Background color for lines that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.insertedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_line_background: Option<String>,
    /// Background color for lines that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.removedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_line_background: Option<String>,
    /// The color of unchanged blocks in diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedRegionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_region_background: Option<String>,
    /// The foreground color of unchanged blocks in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedRegionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_region_foreground: Option<String>,
    /// The color of the shadow around unchanged region widgets.
    #[serde(
        default,
        rename = "diffEditor.unchangedRegionShadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_region_shadow: Option<String>,
    /// The background color of unchanged code in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedCodeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_code_background: Option<String>,
    /// The border color for text that got moved in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.move.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub move_border: Option<String>,
    /// The active border color for text that got moved in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.moveActive.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub move_active_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorGutterColors {
    /// Background color for the margin where lines got inserted.
    #[serde(
        default,
        rename = "diffEditorGutter.insertedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_line_background: Option<String>,
    /// Background color for the margin where lines got removed.
    #[serde(
        default,
        rename = "diffEditorGutter.removedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_line_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorOverviewColors {
    /// Diff overview ruler foreground for inserted content.
    #[serde(
        default,
        rename = "diffEditorOverview.insertedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_foreground: Option<String>,
    /// Diff overview ruler foreground for removed content.
    #[serde(
        default,
        rename = "diffEditorOverview.removedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiDiffEditorColors {
    /// The background color of the diff editor's header
    #[serde(
        default,
        rename = "multiDiffEditor.headerBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatColors {
    /// The border color of a chat request.
    #[serde(
        default,
        rename = "chat.requestBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub request_border: Option<String>,
    /// The background color of a chat slash command.
    #[serde(
        default,
        rename = "chat.slashCommandBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub slash_command_background: Option<String>,
    /// The foreground color of a chat slash command.
    #[serde(
        default,
        rename = "chat.slashCommandForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub slash_command_foreground: Option<String>,
    /// The background color of a chat avatar.
    #[serde(
        default,
        rename = "chat.avatarBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub avatar_background: Option<String>,
    /// The foreground color of a chat avatar.
    #[serde(
        default,
        rename = "chat.avatarForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub avatar_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineChatColors {
    /// Background color of the interactive editor widget.
    #[serde(
        default,
        rename = "inlineChat.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color of the interactive editor widget.
    #[serde(
        default,
        rename = "inlineChat.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Shadow color of the interactive editor widget.
    #[serde(
        default,
        rename = "inlineChat.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
    /// Background highlighting of the current interactive region. Must be transparent.
    #[serde(
        default,
        rename = "inlineChat.regionHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub region_highlight: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineChatInputColors {
    /// Border color of the interactive editor input.
    #[serde(
        default,
        rename = "inlineChatInput.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Border color of the interactive editor input when focused.
    #[serde(
        default,
        rename = "inlineChatInput.focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,
    /// Foreground color of the interactive editor input placeholder.
    #[serde(
        default,
        rename = "inlineChatInput.placeholderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub placeholder_foreground: Option<String>,
    /// Background color of the interactive editor input.
    #[serde(
        default,
        rename = "inlineChatInput.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineChatDiffColors {
    /// Background color of inserted text in the interactive editor input.
    #[serde(
        default,
        rename = "inlineChatDiff.inserted",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted: Option<String>,
    /// Background color of removed text in the interactive editor input.
    #[serde(
        default,
        rename = "inlineChatDiff.removed",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWidgetColors {
    /// Foreground color of editor widgets, such as find/replace.
    #[serde(
        default,
        rename = "editorWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of editor widgets, such as Find/Replace.
    #[serde(
        default,
        rename = "editorWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color of the editor widget unless the widget does not contain a border or defines its own border color.
    #[serde(
        default,
        rename = "editorWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Border color of the resize bar of editor widgets. The color is only used if the widget chooses to have a resize border and if the color is not overridden by a widget.
    #[serde(
        default,
        rename = "editorWidget.resizeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub resize_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSuggestWidgetColors {
    /// Background color of the suggestion widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color of the suggestion widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Foreground color of the suggestion widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color of the match highlights in the suggest widget when an item is focused.
    #[serde(
        default,
        rename = "editorSuggestWidget.focusHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_highlight_foreground: Option<String>,
    /// Color of the match highlights in the suggestion widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.highlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub highlight_foreground: Option<String>,
    /// Background color of the selected entry in the suggestion widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.selectedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selected_background: Option<String>,
    /// Foreground color of the selected entry in the suggest widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.selectedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selected_foreground: Option<String>,
    /// Icon foreground color of the selected entry in the suggest widget.
    #[serde(
        default,
        rename = "editorSuggestWidget.selectedIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selected_icon_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSuggestWidgetStatusColors {
    /// Foreground color of the suggest widget status.
    #[serde(
        default,
        rename = "editorSuggestWidgetStatus.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHoverWidgetColors {
    /// Foreground color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Foreground color of the active item in the parameter hint.
    #[serde(
        default,
        rename = "editorHoverWidget.highlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub highlight_foreground: Option<String>,
    /// Background color of the editor hover status bar.
    #[serde(
        default,
        rename = "editorHoverWidget.statusBarBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGhostTextColors {
    /// Border color of the ghost text shown by inline completion providers and the suggest preview.
    #[serde(
        default,
        rename = "editorGhostText.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of the ghost text in the editor.
    #[serde(
        default,
        rename = "editorGhostText.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of the ghost text shown by inline completion providers and the suggest preview.
    #[serde(
        default,
        rename = "editorGhostText.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollColors {
    /// Editor sticky scroll background color.
    #[serde(
        default,
        rename = "editorStickyScroll.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollHoverColors {
    /// Editor sticky scroll on hover background color.
    #[serde(
        default,
        rename = "editorStickyScrollHover.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugExceptionWidgetColors {
    /// Exception widget background color.
    #[serde(
        default,
        rename = "debugExceptionWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Exception widget border color.
    #[serde(
        default,
        rename = "debugExceptionWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorMarkerNavigationColors {
    /// Editor marker navigation widget background.
    #[serde(
        default,
        rename = "editorMarkerNavigation.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorMarkerNavigationErrorColors {
    /// Editor marker navigation widget error color.
    #[serde(
        default,
        rename = "editorMarkerNavigationError.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor marker navigation widget error heading background.
    #[serde(
        default,
        rename = "editorMarkerNavigationError.headerBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorMarkerNavigationWarningColors {
    /// Editor marker navigation widget warning color.
    #[serde(
        default,
        rename = "editorMarkerNavigationWarning.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor marker navigation widget warning heading background.
    #[serde(
        default,
        rename = "editorMarkerNavigationWarning.headerBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorMarkerNavigationInfoColors {
    /// Editor marker navigation widget info color.
    #[serde(
        default,
        rename = "editorMarkerNavigationInfo.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor marker navigation widget info heading background.
    #[serde(
        default,
        rename = "editorMarkerNavigationInfo.headerBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewColors {
    /// Color of the peek view borders and arrow.
    #[serde(
        default,
        rename = "peekView.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewEditorColors {
    /// Background color of the peek view editor.
    #[serde(
        default,
        rename = "peekViewEditor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Match highlight color in the peek view editor.
    #[serde(
        default,
        rename = "peekViewEditor.matchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub match_highlight_background: Option<String>,
    /// Match highlight border color in the peek view editor.
    #[serde(
        default,
        rename = "peekViewEditor.matchHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub match_highlight_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewEditorGutterColors {
    /// Background color of the gutter in the peek view editor.
    #[serde(
        default,
        rename = "peekViewEditorGutter.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewResultColors {
    /// Background color of the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color for file nodes in the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.fileForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub file_foreground: Option<String>,
    /// Foreground color for line nodes in the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.lineForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub line_foreground: Option<String>,
    /// Match highlight color in the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.matchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub match_highlight_background: Option<String>,
    /// Background color of the selected entry in the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Foreground color of the selected entry in the peek view result list.
    #[serde(
        default,
        rename = "peekViewResult.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewTitleColors {
    /// Background color of the peek view title area.
    #[serde(
        default,
        rename = "peekViewTitle.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewTitleDescriptionColors {
    /// Color of the peek view title info.
    #[serde(
        default,
        rename = "peekViewTitleDescription.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewTitleLabelColors {
    /// Color of the peek view title.
    #[serde(
        default,
        rename = "peekViewTitleLabel.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekViewEditorStickyScrollColors {
    /// Background color of sticky scroll in the peek view editor.
    #[serde(
        default,
        rename = "peekViewEditorStickyScroll.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeColors {
    /// Current header background in inline merge conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.currentHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_header_background: Option<String>,
    /// Current content background in inline merge conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.currentContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_content_background: Option<String>,
    /// Incoming header background in inline merge conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.incomingHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_header_background: Option<String>,
    /// Incoming content background in inline merge conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.incomingContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_content_background: Option<String>,
    /// Border color on headers and the splitter in inline merge conflicts.
    #[serde(
        default,
        rename = "merge.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Common ancestor content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.commonContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_content_background: Option<String>,
    /// Common ancestor header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.commonHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_header_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeEditorColors {
    /// The background color for changes.
    #[serde(
        default,
        rename = "mergeEditor.change.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub change_background: Option<String>,
    /// The background color for word changes.
    #[serde(
        default,
        rename = "mergeEditor.change.word.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub change_word_background: Option<String>,
    /// The border color of unhandled unfocused conflicts.
    #[serde(
        default,
        rename = "mergeEditor.conflict.unhandledUnfocused.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_unhandled_unfocused_border: Option<String>,
    /// The border color of unhandled focused conflicts.
    #[serde(
        default,
        rename = "mergeEditor.conflict.unhandledFocused.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_unhandled_focused_border: Option<String>,
    /// The border color of handled unfocused conflicts.
    #[serde(
        default,
        rename = "mergeEditor.conflict.handledUnfocused.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_handled_unfocused_border: Option<String>,
    /// The border color of handled focused conflicts.
    #[serde(
        default,
        rename = "mergeEditor.conflict.handledFocused.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_handled_focused_border: Option<String>,
    /// The foreground color for changes in input 1.
    #[serde(
        default,
        rename = "mergeEditor.conflict.handled.minimapOverViewRuler",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_handled_minimap_over_view_ruler: Option<String>,
    /// The foreground color for changes in input 1.
    #[serde(
        default,
        rename = "mergeEditor.conflict.unhandled.minimapOverViewRuler",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_unhandled_minimap_over_view_ruler: Option<String>,
    /// The background of the "Conflicting Lines" text.
    #[serde(
        default,
        rename = "mergeEditor.conflictingLines.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflicting_lines_background: Option<String>,
    /// The background color for changes in base.
    #[serde(
        default,
        rename = "mergeEditor.changeBase.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub change_base_background: Option<String>,
    /// The background color for word changes in base.
    #[serde(
        default,
        rename = "mergeEditor.changeBase.word.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub change_base_word_background: Option<String>,
    /// The background color of decorations in input 1.
    #[serde(
        default,
        rename = "mergeEditor.conflict.input1.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_input1_background: Option<String>,
    /// The background color of decorations in input 2.
    #[serde(
        default,
        rename = "mergeEditor.conflict.input2.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflict_input2_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelColors {
    /// Panel background color.
    #[serde(
        default,
        rename = "panel.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Panel border color to separate the panel from the editor.
    #[serde(
        default,
        rename = "panel.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Drag and drop feedback color for the panel titles. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panel.dropBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelTitleColors {
    /// Border color for the active panel title.
    #[serde(
        default,
        rename = "panelTitle.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Title color for the active panel.
    #[serde(
        default,
        rename = "panelTitle.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Title color for the inactive panel.
    #[serde(
        default,
        rename = "panelTitle.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelInputColors {
    /// Input box border for inputs in the panel.
    #[serde(
        default,
        rename = "panelInput.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelSectionColors {
    /// Panel section border color used when multiple views are stacked horizontally in the panel. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panelSection.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Drag and drop feedback color for the panel sections. The color should have transparency so that the panel sections can still shine through. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panelSection.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelSectionHeaderColors {
    /// Panel section header background color. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panelSectionHeader.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Panel section header foreground color. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panelSectionHeader.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Panel section header border color used when multiple views are stacked vertically in the panel. Panels are shown below the editor area and contain views like output and integrated terminal.
    #[serde(
        default,
        rename = "panelSectionHeader.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarColors {
    /// Standard Status Bar background color.
    #[serde(
        default,
        rename = "statusBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Status Bar foreground color.
    #[serde(
        default,
        rename = "statusBar.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Status Bar border color separating the Status Bar and editor.
    #[serde(
        default,
        rename = "statusBar.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Status Bar background color when a program is being debugged.
    #[serde(
        default,
        rename = "statusBar.debuggingBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub debugging_background: Option<String>,
    /// Status Bar foreground color when a program is being debugged.
    #[serde(
        default,
        rename = "statusBar.debuggingForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub debugging_foreground: Option<String>,
    /// Status Bar border color separating the Status Bar and editor when a program is being debugged.
    #[serde(
        default,
        rename = "statusBar.debuggingBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub debugging_border: Option<String>,
    /// Status Bar foreground color when no folder is opened.
    #[serde(
        default,
        rename = "statusBar.noFolderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_folder_foreground: Option<String>,
    /// Status Bar background color when no folder is opened.
    #[serde(
        default,
        rename = "statusBar.noFolderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_folder_background: Option<String>,
    /// Status Bar border color separating the Status Bar and editor when no folder is opened.
    #[serde(
        default,
        rename = "statusBar.noFolderBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_folder_border: Option<String>,
    /// Status bar border color when focused on keyboard navigation. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBar.focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarItemColors {
    /// Status Bar item background color when clicking.
    #[serde(
        default,
        rename = "statusBarItem.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Status bar item foreground color when hovering. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.hoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_foreground: Option<String>,
    /// Status Bar item background color when hovering.
    #[serde(
        default,
        rename = "statusBarItem.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Status Bar prominent items foreground color.
    #[serde(
        default,
        rename = "statusBarItem.prominentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_foreground: Option<String>,
    /// Status Bar prominent items background color.
    #[serde(
        default,
        rename = "statusBarItem.prominentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_background: Option<String>,
    /// Status bar prominent items foreground color when hovering. Prominent items stand out from other status bar entries to indicate importance. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.prominentHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_hover_foreground: Option<String>,
    /// Status Bar prominent items background color when hovering.
    #[serde(
        default,
        rename = "statusBarItem.prominentHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_hover_background: Option<String>,
    /// Background color for the remote indicator on the status bar.
    #[serde(
        default,
        rename = "statusBarItem.remoteBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_background: Option<String>,
    /// Foreground color for the remote indicator on the status bar.
    #[serde(
        default,
        rename = "statusBarItem.remoteForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_foreground: Option<String>,
    /// Background color for the remote indicator on the status bar when hovering.
    #[serde(
        default,
        rename = "statusBarItem.remoteHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_hover_background: Option<String>,
    /// Foreground color for the remote indicator on the status bar when hovering.
    #[serde(
        default,
        rename = "statusBarItem.remoteHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_hover_foreground: Option<String>,
    /// Status bar error items background color. Error items stand out from other status bar entries to indicate error conditions.
    #[serde(
        default,
        rename = "statusBarItem.errorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_background: Option<String>,
    /// Status bar error items foreground color. Error items stand out from other status bar entries to indicate error conditions.
    #[serde(
        default,
        rename = "statusBarItem.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Status bar error items background color when hovering. Error items stand out from other status bar entries to indicate error conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.errorHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_hover_background: Option<String>,
    /// Status bar error items foreground color when hovering. Error items stand out from other status bar entries to indicate error conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.errorHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_hover_foreground: Option<String>,
    /// Status bar warning items background color. Warning items stand out from other status bar entries to indicate warning conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.warningBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_background: Option<String>,
    /// Status bar warning items foreground color. Warning items stand out from other status bar entries to indicate warning conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Status bar warning items background color when hovering. Warning items stand out from other status bar entries to indicate warning conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.warningHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_hover_background: Option<String>,
    /// Status bar warning items foreground color when hovering. Warning items stand out from other status bar entries to indicate warning conditions. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.warningHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_hover_foreground: Option<String>,
    /// Status bar item background color when hovering an item that contains two hovers. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.compactHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub compact_hover_background: Option<String>,
    /// Status bar item border color when focused on keyboard navigation. The status bar is shown in the bottom of the window.
    #[serde(
        default,
        rename = "statusBarItem.focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,
    /// Status bar item background color when the workbench is offline.
    #[serde(
        default,
        rename = "statusBarItem.offlineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub offline_background: Option<String>,
    /// Status bar item foreground color when the workbench is offline.
    #[serde(
        default,
        rename = "statusBarItem.offlineForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub offline_foreground: Option<String>,
    /// Status bar item foreground hover color when the workbench is offline.
    #[serde(
        default,
        rename = "statusBarItem.offlineHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub offline_hover_foreground: Option<String>,
    /// Status bar item background hover color when the workbench is offline.
    #[serde(
        default,
        rename = "statusBarItem.offlineHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub offline_hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleBarColors {
    /// Title Bar background when the window is active.
    #[serde(
        default,
        rename = "titleBar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Title Bar foreground when the window is active.
    #[serde(
        default,
        rename = "titleBar.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Title Bar background when the window is inactive.
    #[serde(
        default,
        rename = "titleBar.inactiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_background: Option<String>,
    /// Title Bar foreground when the window is inactive.
    #[serde(
        default,
        rename = "titleBar.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_foreground: Option<String>,
    /// Title bar border color.
    #[serde(
        default,
        rename = "titleBar.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenubarColors {
    /// Foreground color of the selected menu item in the menubar.
    #[serde(
        default,
        rename = "menubar.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// Background color of the selected menu item in the menubar.
    #[serde(
        default,
        rename = "menubar.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Border color of the selected menu item in the menubar.
    #[serde(
        default,
        rename = "menubar.selectionBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuColors {
    /// Foreground color of menu items.
    #[serde(
        default,
        rename = "menu.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of menu items.
    #[serde(
        default,
        rename = "menu.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// Background color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Border color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_border: Option<String>,
    /// Color of a separator menu item in menus.
    #[serde(
        default,
        rename = "menu.separatorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub separator_background: Option<String>,
    /// Border color of menus.
    #[serde(
        default,
        rename = "menu.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCenterColors {
    /// Foreground color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Active foreground color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
    /// Background color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Active background color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Border color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Foreground color of the Command Center when the window is inactive.
    #[serde(
        default,
        rename = "commandCenter.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_foreground: Option<String>,
    /// Border color of the Command Center when the window is inactive.
    #[serde(
        default,
        rename = "commandCenter.inactiveBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_border: Option<String>,
    /// Active border color of the Command Center.
    #[serde(
        default,
        rename = "commandCenter.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Command Center background color when a program is being debugged.
    #[serde(
        default,
        rename = "commandCenter.debuggingBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub debugging_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCenterColors {
    /// Notification Center border color.
    #[serde(
        default,
        rename = "notificationCenter.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCenterHeaderColors {
    /// Notification Center header foreground color.
    #[serde(
        default,
        rename = "notificationCenterHeader.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Notification Center header background color.
    #[serde(
        default,
        rename = "notificationCenterHeader.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationToastColors {
    /// Notification toast border color.
    #[serde(
        default,
        rename = "notificationToast.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsColors {
    /// Notification foreground color.
    #[serde(
        default,
        rename = "notifications.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Notification background color.
    #[serde(
        default,
        rename = "notifications.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Notification border color separating from other notifications in the Notification Center.
    #[serde(
        default,
        rename = "notifications.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationLinkColors {
    /// Notification links foreground color.
    #[serde(
        default,
        rename = "notificationLink.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsErrorIconColors {
    /// The color used for the notification error icon.
    #[serde(
        default,
        rename = "notificationsErrorIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsWarningIconColors {
    /// The color used for the notification warning icon.
    #[serde(
        default,
        rename = "notificationsWarningIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsInfoIconColors {
    /// The color used for the notification info icon.
    #[serde(
        default,
        rename = "notificationsInfoIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerColors {
    /// Banner background color.
    #[serde(
        default,
        rename = "banner.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Banner foreground color.
    #[serde(
        default,
        rename = "banner.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color for the icon in front of the banner text.
    #[serde(
        default,
        rename = "banner.iconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionButtonColors {
    /// Extension view button foreground color (for example **Install** button).
    #[serde(
        default,
        rename = "extensionButton.prominentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_foreground: Option<String>,
    /// Extension view button background color.
    #[serde(
        default,
        rename = "extensionButton.prominentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_background: Option<String>,
    /// Extension view button background hover color.
    #[serde(
        default,
        rename = "extensionButton.prominentHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub prominent_hover_background: Option<String>,
    /// Button background color for extension actions.
    #[serde(
        default,
        rename = "extensionButton.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Button foreground color for extension actions.
    #[serde(
        default,
        rename = "extensionButton.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Button background hover color for extension actions.
    #[serde(
        default,
        rename = "extensionButton.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Button separator color for extension actions.
    #[serde(
        default,
        rename = "extensionButton.separator",
        deserialize_with = "empty_string_as_none"
    )]
    pub separator: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionBadgeColors {
    /// Background color for the remote badge in the extensions view.
    #[serde(
        default,
        rename = "extensionBadge.remoteBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_background: Option<String>,
    /// Foreground color for the remote badge in the extensions view.
    #[serde(
        default,
        rename = "extensionBadge.remoteForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub remote_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionIconColors {
    /// The icon color for extension ratings.
    #[serde(
        default,
        rename = "extensionIcon.starForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub star_foreground: Option<String>,
    /// The icon color for extension verified publisher.
    #[serde(
        default,
        rename = "extensionIcon.verifiedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub verified_foreground: Option<String>,
    /// The icon color for pre-release extension.
    #[serde(
        default,
        rename = "extensionIcon.preReleaseForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub pre_release_foreground: Option<String>,
    /// The icon color for extension sponsor.
    #[serde(
        default,
        rename = "extensionIcon.sponsorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub sponsor_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickerGroupColors {
    /// Quick picker (Quick Open) color for grouping borders.
    #[serde(
        default,
        rename = "pickerGroup.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Quick picker (Quick Open) color for grouping labels.
    #[serde(
        default,
        rename = "pickerGroup.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputColors {
    /// Quick input background color. The quick input widget is the container for views like the color theme picker.
    #[serde(
        default,
        rename = "quickInput.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Quick input foreground color. The quick input widget is the container for views like the color theme picker.
    #[serde(
        default,
        rename = "quickInput.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputListColors {
    /// Quick picker background color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_background: Option<String>,
    /// Quick picker foreground color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// Quick picker icon foreground color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_icon_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputTitleColors {
    /// Quick picker title background color. The quick picker widget is the container for pickers like the Command Palette.
    #[serde(
        default,
        rename = "quickInputTitle.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingLabelColors {
    /// Keybinding label background color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Keybinding label foreground color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Keybinding label border color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Keybinding label border bottom color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.bottomBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub bottom_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingTableColors {
    /// Background color for the keyboard shortcuts table header.
    #[serde(
        default,
        rename = "keybindingTable.headerBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_background: Option<String>,
    /// Background color for the keyboard shortcuts table alternating rows.
    #[serde(
        default,
        rename = "keybindingTable.rowsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub rows_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColors {
    /// The background of the Integrated Terminal's viewport.
    #[serde(
        default,
        rename = "terminal.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// The color of the border that separates split panes within the terminal. This defaults to panel.border.
    #[serde(
        default,
        rename = "terminal.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// The default foreground color of the Integrated Terminal.
    #[serde(
        default,
        rename = "terminal.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// 'Black' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBlack",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_black: Option<String>,
    /// 'Blue' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBlue",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_blue: Option<String>,
    /// 'BrightBlack' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightBlack",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_black: Option<String>,
    /// 'BrightBlue' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightBlue",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_blue: Option<String>,
    /// 'BrightCyan' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightCyan",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_cyan: Option<String>,
    /// 'BrightGreen' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightGreen",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_green: Option<String>,
    /// 'BrightMagenta' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightMagenta",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_magenta: Option<String>,
    /// 'BrightRed' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightRed",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_red: Option<String>,
    /// 'BrightWhite' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightWhite",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_white: Option<String>,
    /// 'BrightYellow' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiBrightYellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_bright_yellow: Option<String>,
    /// 'Cyan' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiCyan",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_cyan: Option<String>,
    /// 'Green' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiGreen",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_green: Option<String>,
    /// 'Magenta' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiMagenta",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_magenta: Option<String>,
    /// 'Red' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiRed",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_red: Option<String>,
    /// 'White' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiWhite",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_white: Option<String>,
    /// 'Yellow' ANSI color in the terminal.
    #[serde(
        default,
        rename = "terminal.ansiYellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub ansi_yellow: Option<String>,
    /// The selection background color of the terminal.
    #[serde(
        default,
        rename = "terminal.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// The selection foreground color of the terminal. When this is null the selection foreground will be retained and have the minimum contrast ratio feature applied.
    #[serde(
        default,
        rename = "terminal.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// The selection background color of the terminal when it does not have focus.
    #[serde(
        default,
        rename = "terminal.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_background: Option<String>,
    /// Color of the current search match in the terminal. The color must not be opaque so as not to hide underlying terminal content.
    #[serde(
        default,
        rename = "terminal.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_background: Option<String>,
    /// Border color of the current search match in the terminal.
    #[serde(
        default,
        rename = "terminal.findMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_border: Option<String>,
    /// Color of the other search matches in the terminal. The color must not be opaque so as not to hide underlying terminal content.
    #[serde(
        default,
        rename = "terminal.findMatchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_background: Option<String>,
    /// Border color of the other search matches in the terminal.
    #[serde(
        default,
        rename = "terminal.findMatchHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_border: Option<String>,
    /// Color of the highlight when hovering a link in the terminal.
    #[serde(
        default,
        rename = "terminal.hoverHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_highlight_background: Option<String>,
    /// The background color when dragging on top of terminals. The color should have transparency so that the terminal contents can still shine through.
    #[serde(
        default,
        rename = "terminal.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
    /// Border on the side of the terminal tab in the panel. This defaults to `tab.activeBorder`.
    #[serde(
        default,
        rename = "terminal.tab.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_active_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCursorColors {
    /// The background color of the terminal cursor. Allows customizing the color of a character overlapped by a block cursor.
    #[serde(
        default,
        rename = "terminalCursor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// The foreground color of the terminal cursor.
    #[serde(
        default,
        rename = "terminalCursor.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCommandDecorationColors {
    /// The default terminal command decoration background color.
    #[serde(
        default,
        rename = "terminalCommandDecoration.defaultBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub default_background: Option<String>,
    /// The terminal command decoration background color for successful commands.
    #[serde(
        default,
        rename = "terminalCommandDecoration.successBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub success_background: Option<String>,
    /// The terminal command decoration background color for error commands.
    #[serde(
        default,
        rename = "terminalCommandDecoration.errorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOverviewRulerColors {
    /// The overview ruler cursor color.
    #[serde(
        default,
        rename = "terminalOverviewRuler.cursorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub cursor_foreground: Option<String>,
    /// Overview ruler marker color for find matches in the terminal.
    #[serde(
        default,
        rename = "terminalOverviewRuler.findMatchForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalStickyScrollColors {
    /// The background color of the sticky scroll overlay in the terminal.
    #[serde(
        default,
        rename = "terminalStickyScroll.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalStickyScrollHoverColors {
    /// The background color of the sticky scroll overlay in the terminal when hovered.
    #[serde(
        default,
        rename = "terminalStickyScrollHover.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugToolBarColors {
    /// Debug toolbar background color.
    #[serde(
        default,
        rename = "debugToolBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Debug toolbar border color.
    #[serde(
        default,
        rename = "debugToolBar.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugViewColors {
    /// Foreground color for a label shown in the CALL STACK view when the debugger breaks on an exception.
    #[serde(
        default,
        rename = "debugView.exceptionLabelForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub exception_label_foreground: Option<String>,
    /// Background color for a label shown in the CALL STACK view when the debugger breaks on an exception.
    #[serde(
        default,
        rename = "debugView.exceptionLabelBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub exception_label_background: Option<String>,
    /// Foreground color for a label in the CALL STACK view showing the current session's or thread's state.
    #[serde(
        default,
        rename = "debugView.stateLabelForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub state_label_foreground: Option<String>,
    /// Background color for a label in the CALL STACK view showing the current session's or thread's state.
    #[serde(
        default,
        rename = "debugView.stateLabelBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub state_label_background: Option<String>,
    /// Color used to highlight value changes in the debug views (such as in the Variables view).
    #[serde(
        default,
        rename = "debugView.valueChangedHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub value_changed_highlight: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugTokenExpressionColors {
    /// Foreground color for the token names shown in debug views (such as in the Variables or Watch view).
    #[serde(
        default,
        rename = "debugTokenExpression.name",
        deserialize_with = "empty_string_as_none"
    )]
    pub name: Option<String>,
    /// Foreground color for the token values shown in debug views.
    #[serde(
        default,
        rename = "debugTokenExpression.value",
        deserialize_with = "empty_string_as_none"
    )]
    pub value: Option<String>,
    /// Foreground color for strings in debug views.
    #[serde(
        default,
        rename = "debugTokenExpression.string",
        deserialize_with = "empty_string_as_none"
    )]
    pub string: Option<String>,
    /// Foreground color for booleans in debug views.
    #[serde(
        default,
        rename = "debugTokenExpression.boolean",
        deserialize_with = "empty_string_as_none"
    )]
    pub boolean: Option<String>,
    /// Foreground color for numbers in debug views.
    #[serde(
        default,
        rename = "debugTokenExpression.number",
        deserialize_with = "empty_string_as_none"
    )]
    pub number: Option<String>,
    /// Foreground color for expression errors in debug views.
    #[serde(
        default,
        rename = "debugTokenExpression.error",
        deserialize_with = "empty_string_as_none"
    )]
    pub error: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingColors {
    /// Color for the 'failed' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconFailed",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_failed: Option<String>,
    /// Color for the 'Errored' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconErrored",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_errored: Option<String>,
    /// Color for the 'passed' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconPassed",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_passed: Option<String>,
    /// Color for 'run' icons in the editor.
    #[serde(
        default,
        rename = "testing.runAction",
        deserialize_with = "empty_string_as_none"
    )]
    pub run_action: Option<String>,
    /// Color for the 'Queued' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconQueued",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_queued: Option<String>,
    /// Color for the 'Unset' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconUnset",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_unset: Option<String>,
    /// Color for the 'Skipped' icon in the test explorer.
    #[serde(
        default,
        rename = "testing.iconSkipped",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_skipped: Option<String>,
    /// Color of the peek view borders and arrow.
    #[serde(
        default,
        rename = "testing.peekBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_border: Option<String>,
    /// Color of the peek view borders and arrow.
    #[serde(
        default,
        rename = "testing.peekHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_header_background: Option<String>,
    /// Text color of test error messages shown inline in the editor.
    #[serde(
        default,
        rename = "testing.message.error.decorationForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub message_error_decoration_foreground: Option<String>,
    /// Margin color beside error messages shown inline in the editor.
    #[serde(
        default,
        rename = "testing.message.error.lineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub message_error_line_background: Option<String>,
    /// Text color of test info messages shown inline in the editor.
    #[serde(
        default,
        rename = "testing.message.info.decorationForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub message_info_decoration_foreground: Option<String>,
    /// Margin color beside info messages shown inline in the editor.
    #[serde(
        default,
        rename = "testing.message.info.lineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub message_info_line_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomePageColors {
    /// Background color for the Welcome page.
    #[serde(
        default,
        rename = "welcomePage.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color for the Welcome page progress bars.
    #[serde(
        default,
        rename = "welcomePage.progress.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub progress_background: Option<String>,
    /// Background color for the Welcome page progress bars.
    #[serde(
        default,
        rename = "welcomePage.progress.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub progress_foreground: Option<String>,
    /// Background color for the tiles on the Welcome page.
    #[serde(
        default,
        rename = "welcomePage.tileBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tile_background: Option<String>,
    /// Hover background color for the tiles on the Welcome page.
    #[serde(
        default,
        rename = "welcomePage.tileHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tile_hover_background: Option<String>,
    /// Border color for the tiles on the Welcome page.
    #[serde(
        default,
        rename = "welcomePage.tileBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub tile_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkThroughColors {
    /// Background color for the embedded editors on the Interactive Playground.
    #[serde(
        default,
        rename = "walkThrough.embeddedEditorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub embedded_editor_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughColors {
    /// Foreground color of the heading of each walkthrough step.
    #[serde(
        default,
        rename = "walkthrough.stepTitle.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub step_title_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitDecorationColors {
    /// Color for added Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.addedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub added_resource_foreground: Option<String>,
    /// Color for modified Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.modifiedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub modified_resource_foreground: Option<String>,
    /// Color for deleted Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.deletedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deleted_resource_foreground: Option<String>,
    /// Color for renamed or copied Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.renamedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub renamed_resource_foreground: Option<String>,
    /// Color for staged modifications git decorations.  Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.stageModifiedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub stage_modified_resource_foreground: Option<String>,
    /// Color for staged deletions git decorations.  Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.stageDeletedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub stage_deleted_resource_foreground: Option<String>,
    /// Color for untracked Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.untrackedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub untracked_resource_foreground: Option<String>,
    /// Color for ignored Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.ignoredResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub ignored_resource_foreground: Option<String>,
    /// Color for conflicting Git resources. Used for file labels and the SCM viewlet.
    #[serde(
        default,
        rename = "gitDecoration.conflictingResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub conflicting_resource_foreground: Option<String>,
    /// Color for submodule resources.
    #[serde(
        default,
        rename = "gitDecoration.submoduleResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub submodule_resource_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsColors {
    /// The foreground color for a section header or active title.
    #[serde(
        default,
        rename = "settings.headerForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_foreground: Option<String>,
    /// The line that indicates a modified setting.
    #[serde(
        default,
        rename = "settings.modifiedItemIndicator",
        deserialize_with = "empty_string_as_none"
    )]
    pub modified_item_indicator: Option<String>,
    /// Dropdown background.
    #[serde(
        default,
        rename = "settings.dropdownBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_background: Option<String>,
    /// Dropdown foreground.
    #[serde(
        default,
        rename = "settings.dropdownForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_foreground: Option<String>,
    /// Dropdown border.
    #[serde(
        default,
        rename = "settings.dropdownBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_border: Option<String>,
    /// Dropdown list border.
    #[serde(
        default,
        rename = "settings.dropdownListBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_list_border: Option<String>,
    /// Checkbox background.
    #[serde(
        default,
        rename = "settings.checkboxBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub checkbox_background: Option<String>,
    /// Checkbox foreground.
    #[serde(
        default,
        rename = "settings.checkboxForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub checkbox_foreground: Option<String>,
    /// Checkbox border.
    #[serde(
        default,
        rename = "settings.checkboxBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub checkbox_border: Option<String>,
    /// The background color of a settings row when hovered.
    #[serde(
        default,
        rename = "settings.rowHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub row_hover_background: Option<String>,
    /// Text input box background.
    #[serde(
        default,
        rename = "settings.textInputBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub text_input_background: Option<String>,
    /// Text input box foreground.
    #[serde(
        default,
        rename = "settings.textInputForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub text_input_foreground: Option<String>,
    /// Text input box border.
    #[serde(
        default,
        rename = "settings.textInputBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub text_input_border: Option<String>,
    /// Number input box background.
    #[serde(
        default,
        rename = "settings.numberInputBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub number_input_background: Option<String>,
    /// Number input box foreground.
    #[serde(
        default,
        rename = "settings.numberInputForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub number_input_foreground: Option<String>,
    /// Number input box border.
    #[serde(
        default,
        rename = "settings.numberInputBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub number_input_border: Option<String>,
    /// Background color of a focused setting row.
    #[serde(
        default,
        rename = "settings.focusedRowBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_row_background: Option<String>,
    /// The color of the row's top and bottom border when the row is focused.
    #[serde(
        default,
        rename = "settings.focusedRowBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_row_border: Option<String>,
    /// The color of the header container border.
    #[serde(
        default,
        rename = "settings.headerBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub header_border: Option<String>,
    /// The color of the Settings editor splitview sash border.
    #[serde(
        default,
        rename = "settings.sashBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub sash_border: Option<String>,
    /// The foreground color for a section header or hovered title.
    #[serde(
        default,
        rename = "settings.settingsHeaderHoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_header_hover_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbColors {
    /// Color of breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color of focused breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// Color of selected breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbPickerColors {
    /// Background color of breadcrumb item picker.
    #[serde(
        default,
        rename = "breadcrumbPicker.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolIconColors {
    /// The foreground color for array symbols.
    #[serde(
        default,
        rename = "symbolIcon.arrayForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub array_foreground: Option<String>,
    /// The foreground color for boolean symbols.
    #[serde(
        default,
        rename = "symbolIcon.booleanForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub boolean_foreground: Option<String>,
    /// The foreground color for class symbols.
    #[serde(
        default,
        rename = "symbolIcon.classForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub class_foreground: Option<String>,
    /// The foreground color for color symbols.
    #[serde(
        default,
        rename = "symbolIcon.colorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub color_foreground: Option<String>,
    /// The foreground color for constant symbols.
    #[serde(
        default,
        rename = "symbolIcon.constantForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub constant_foreground: Option<String>,
    /// The foreground color for constructor symbols.
    #[serde(
        default,
        rename = "symbolIcon.constructorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub constructor_foreground: Option<String>,
    /// The foreground color for enumerator symbols.
    #[serde(
        default,
        rename = "symbolIcon.enumeratorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub enumerator_foreground: Option<String>,
    /// The foreground color for enumerator member symbols.
    #[serde(
        default,
        rename = "symbolIcon.enumeratorMemberForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub enumerator_member_foreground: Option<String>,
    /// The foreground color for event symbols.
    #[serde(
        default,
        rename = "symbolIcon.eventForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub event_foreground: Option<String>,
    /// The foreground color for field symbols.
    #[serde(
        default,
        rename = "symbolIcon.fieldForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub field_foreground: Option<String>,
    /// The foreground color for file symbols.
    #[serde(
        default,
        rename = "symbolIcon.fileForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub file_foreground: Option<String>,
    /// The foreground color for folder symbols.
    #[serde(
        default,
        rename = "symbolIcon.folderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub folder_foreground: Option<String>,
    /// The foreground color for function symbols.
    #[serde(
        default,
        rename = "symbolIcon.functionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub function_foreground: Option<String>,
    /// The foreground color for interface symbols.
    #[serde(
        default,
        rename = "symbolIcon.interfaceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub interface_foreground: Option<String>,
    /// The foreground color for key symbols.
    #[serde(
        default,
        rename = "symbolIcon.keyForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub key_foreground: Option<String>,
    /// The foreground color for keyword symbols.
    #[serde(
        default,
        rename = "symbolIcon.keywordForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub keyword_foreground: Option<String>,
    /// The foreground color for method symbols.
    #[serde(
        default,
        rename = "symbolIcon.methodForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub method_foreground: Option<String>,
    /// The foreground color for module symbols.
    #[serde(
        default,
        rename = "symbolIcon.moduleForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub module_foreground: Option<String>,
    /// The foreground color for namespace symbols.
    #[serde(
        default,
        rename = "symbolIcon.namespaceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub namespace_foreground: Option<String>,
    /// The foreground color for null symbols.
    #[serde(
        default,
        rename = "symbolIcon.nullForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub null_foreground: Option<String>,
    /// The foreground color for number symbols.
    #[serde(
        default,
        rename = "symbolIcon.numberForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub number_foreground: Option<String>,
    /// The foreground color for object symbols.
    #[serde(
        default,
        rename = "symbolIcon.objectForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub object_foreground: Option<String>,
    /// The foreground color for operator symbols.
    #[serde(
        default,
        rename = "symbolIcon.operatorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub operator_foreground: Option<String>,
    /// The foreground color for package symbols.
    #[serde(
        default,
        rename = "symbolIcon.packageForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub package_foreground: Option<String>,
    /// The foreground color for property symbols.
    #[serde(
        default,
        rename = "symbolIcon.propertyForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub property_foreground: Option<String>,
    /// The foreground color for reference symbols.
    #[serde(
        default,
        rename = "symbolIcon.referenceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub reference_foreground: Option<String>,
    /// The foreground color for snippet symbols.
    #[serde(
        default,
        rename = "symbolIcon.snippetForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_foreground: Option<String>,
    /// The foreground color for string symbols.
    #[serde(
        default,
        rename = "symbolIcon.stringForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub string_foreground: Option<String>,
    /// The foreground color for struct symbols.
    #[serde(
        default,
        rename = "symbolIcon.structForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub struct_foreground: Option<String>,
    /// The foreground color for text symbols.
    #[serde(
        default,
        rename = "symbolIcon.textForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub text_foreground: Option<String>,
    /// The foreground color for type parameter symbols.
    #[serde(
        default,
        rename = "symbolIcon.typeParameterForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub type_parameter_foreground: Option<String>,
    /// The foreground color for unit symbols.
    #[serde(
        default,
        rename = "symbolIcon.unitForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_foreground: Option<String>,
    /// The foreground color for variable symbols.
    #[serde(
        default,
        rename = "symbolIcon.variableForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub variable_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugIconColors {
    /// Icon color for breakpoints.
    #[serde(
        default,
        rename = "debugIcon.breakpointForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breakpoint_foreground: Option<String>,
    /// Icon color for disabled breakpoints.
    #[serde(
        default,
        rename = "debugIcon.breakpointDisabledForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breakpoint_disabled_foreground: Option<String>,
    /// Icon color for unverified breakpoints.
    #[serde(
        default,
        rename = "debugIcon.breakpointUnverifiedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breakpoint_unverified_foreground: Option<String>,
    /// Icon color for the current breakpoint stack frame.
    #[serde(
        default,
        rename = "debugIcon.breakpointCurrentStackframeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breakpoint_current_stackframe_foreground: Option<String>,
    /// Icon color for all breakpoint stack frames.
    #[serde(
        default,
        rename = "debugIcon.breakpointStackframeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breakpoint_stackframe_foreground: Option<String>,
    /// Debug toolbar icon for start debugging.
    #[serde(
        default,
        rename = "debugIcon.startForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub start_foreground: Option<String>,
    /// Debug toolbar icon for pause.
    #[serde(
        default,
        rename = "debugIcon.pauseForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub pause_foreground: Option<String>,
    /// Debug toolbar icon for stop.
    #[serde(
        default,
        rename = "debugIcon.stopForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub stop_foreground: Option<String>,
    /// Debug toolbar icon for disconnect.
    #[serde(
        default,
        rename = "debugIcon.disconnectForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub disconnect_foreground: Option<String>,
    /// Debug toolbar icon for restart.
    #[serde(
        default,
        rename = "debugIcon.restartForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub restart_foreground: Option<String>,
    /// Debug toolbar icon for step over.
    #[serde(
        default,
        rename = "debugIcon.stepOverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub step_over_foreground: Option<String>,
    /// Debug toolbar icon for step into.
    #[serde(
        default,
        rename = "debugIcon.stepIntoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub step_into_foreground: Option<String>,
    /// Debug toolbar icon for step over.
    #[serde(
        default,
        rename = "debugIcon.stepOutForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub step_out_foreground: Option<String>,
    /// Debug toolbar icon for continue.
    #[serde(
        default,
        rename = "debugIcon.continueForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub continue_foreground: Option<String>,
    /// Debug toolbar icon for step back.
    #[serde(
        default,
        rename = "debugIcon.stepBackForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub step_back_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConsoleColors {
    /// Foreground color for info messages in debug REPL console.
    #[serde(
        default,
        rename = "debugConsole.infoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_foreground: Option<String>,
    /// Foreground color for warning messages in debug REPL console.
    #[serde(
        default,
        rename = "debugConsole.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Foreground color for error messages in debug REPL console.
    #[serde(
        default,
        rename = "debugConsole.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Foreground color for source filenames in debug REPL console.
    #[serde(
        default,
        rename = "debugConsole.sourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub source_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConsoleInputIconColors {
    /// Foreground color for debug console input marker icon.
    #[serde(
        default,
        rename = "debugConsoleInputIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookColors {
    /// Notebook background color.
    #[serde(
        default,
        rename = "notebook.editorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_background: Option<String>,
    /// The border color for notebook cells.
    #[serde(
        default,
        rename = "notebook.cellBorderColor",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_border_color: Option<String>,
    /// The background color of a cell when the cell is hovered.
    #[serde(
        default,
        rename = "notebook.cellHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_hover_background: Option<String>,
    /// The color of the notebook cell insertion indicator.
    #[serde(
        default,
        rename = "notebook.cellInsertionIndicator",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_insertion_indicator: Option<String>,
    /// The background color of notebook cell status bar items.
    #[serde(
        default,
        rename = "notebook.cellStatusBarItemHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_status_bar_item_hover_background: Option<String>,
    /// The color of the separator in the cell bottom toolbar
    #[serde(
        default,
        rename = "notebook.cellToolbarSeparator",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_toolbar_separator: Option<String>,
    /// The color of the notebook cell editor background
    #[serde(
        default,
        rename = "notebook.cellEditorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub cell_editor_background: Option<String>,
    /// The background color of a cell when the cell is focused.
    #[serde(
        default,
        rename = "notebook.focusedCellBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_cell_background: Option<String>,
    /// The color of the cell's focus indicator borders when the cell is focused.
    #[serde(
        default,
        rename = "notebook.focusedCellBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_cell_border: Option<String>,
    /// The color of the notebook cell editor border.
    #[serde(
        default,
        rename = "notebook.focusedEditorBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focused_editor_border: Option<String>,
    /// The color of the cell's top and bottom border when a cell is focused while the primary focus is outside of the editor.
    #[serde(
        default,
        rename = "notebook.inactiveFocusedCellBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_focused_cell_border: Option<String>,
    /// The color of the cell's borders when multiple cells are selected.
    #[serde(
        default,
        rename = "notebook.inactiveSelectedCellBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selected_cell_border: Option<String>,
    /// The Color of the notebook output container background.
    #[serde(
        default,
        rename = "notebook.outputContainerBackgroundColor",
        deserialize_with = "empty_string_as_none"
    )]
    pub output_container_background_color: Option<String>,
    /// The border color of the notebook output container.
    #[serde(
        default,
        rename = "notebook.outputContainerBorderColor",
        deserialize_with = "empty_string_as_none"
    )]
    pub output_container_border_color: Option<String>,
    /// The background color of a cell when the cell is selected.
    #[serde(
        default,
        rename = "notebook.selectedCellBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selected_cell_background: Option<String>,
    /// The color of the cell's top and bottom border when the cell is selected but not focused.
    #[serde(
        default,
        rename = "notebook.selectedCellBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selected_cell_border: Option<String>,
    /// Background color of highlighted cell
    #[serde(
        default,
        rename = "notebook.symbolHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub symbol_highlight_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookScrollbarSliderColors {
    /// Notebook scrollbar slider background color when clicked on.
    #[serde(
        default,
        rename = "notebookScrollbarSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Notebook scrollbar slider background color.
    #[serde(
        default,
        rename = "notebookScrollbarSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Notebook scrollbar slider background color when hovering.
    #[serde(
        default,
        rename = "notebookScrollbarSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookStatusErrorIconColors {
    /// The error icon color of notebook cells in the cell status bar.
    #[serde(
        default,
        rename = "notebookStatusErrorIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookStatusRunningIconColors {
    /// The running icon color of notebook cells in the cell status bar.
    #[serde(
        default,
        rename = "notebookStatusRunningIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookStatusSuccessIconColors {
    /// The success icon color of notebook cells in the cell status bar.
    #[serde(
        default,
        rename = "notebookStatusSuccessIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEditorOverviewRulerColors {
    /// The color of the running cell decoration in the notebook editor overview ruler.
    #[serde(
        default,
        rename = "notebookEditorOverviewRuler.runningCellForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub running_cell_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartsColors {
    /// Contrast color for text in charts.
    #[serde(
        default,
        rename = "charts.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color for lines in charts.
    #[serde(
        default,
        rename = "charts.lines",
        deserialize_with = "empty_string_as_none"
    )]
    pub lines: Option<String>,
    /// Color for red elements in charts.
    #[serde(
        default,
        rename = "charts.red",
        deserialize_with = "empty_string_as_none"
    )]
    pub red: Option<String>,
    /// Color for blue elements in charts.
    #[serde(
        default,
        rename = "charts.blue",
        deserialize_with = "empty_string_as_none"
    )]
    pub blue: Option<String>,
    /// Color for yellow elements in charts.
    #[serde(
        default,
        rename = "charts.yellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub yellow: Option<String>,
    /// Color for orange elements in charts.
    #[serde(
        default,
        rename = "charts.orange",
        deserialize_with = "empty_string_as_none"
    )]
    pub orange: Option<String>,
    /// Color for green elements in charts.
    #[serde(
        default,
        rename = "charts.green",
        deserialize_with = "empty_string_as_none"
    )]
    pub green: Option<String>,
    /// Color for purple elements in charts.
    #[serde(
        default,
        rename = "charts.purple",
        deserialize_with = "empty_string_as_none"
    )]
    pub purple: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortsColors {
    /// The color of the icon for a port that has an associated running process.
    #[serde(
        default,
        rename = "ports.iconRunningProcessForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub icon_running_process_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsViewColors {
    /// Icon color for resolved comments.
    #[serde(
        default,
        rename = "commentsView.resolvedIcon",
        deserialize_with = "empty_string_as_none"
    )]
    pub resolved_icon: Option<String>,
    /// Icon color for unresolved comments.
    #[serde(
        default,
        rename = "commentsView.unresolvedIcon",
        deserialize_with = "empty_string_as_none"
    )]
    pub unresolved_icon: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBarColors {
    /// Background color for toggled action items in action bar.
    #[serde(
        default,
        rename = "actionBar.toggledBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub toggled_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleFindWidgetColors {
    /// Border color of the sash border.
    #[serde(
        default,
        rename = "simpleFindWidget.sashBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub sash_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScmColors {
    /// History item additions foreground color.
    #[serde(
        default,
        rename = "scm.historyItemAdditionsForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub history_item_additions_foreground: Option<String>,
    /// History item deletions foreground color.
    #[serde(
        default,
        rename = "scm.historyItemDeletionsForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub history_item_deletions_foreground: Option<String>,
    /// History item statistics border color.
    #[serde(
        default,
        rename = "scm.historyItemStatisticsBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub history_item_statistics_border: Option<String>,
    /// History item selected statistics border color.
    #[serde(
        default,
        rename = "scm.historyItemSelectedStatisticsBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub history_item_selected_statistics_border: Option<String>,
}
