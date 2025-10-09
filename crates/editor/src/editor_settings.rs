use core::num;
use std::num::NonZeroU32;

use gpui::App;
use language::CursorShape;
use project::project_settings::DiagnosticSeverity;
pub use settings::{
    CurrentLineHighlight, DisplayIn, DocumentColorsRenderMode, DoubleClickInMultibuffer,
    GoToDefinitionFallback, HideMouseMode, MinimapThumb, MinimapThumbBorder, MultiCursorModifier,
    ScrollBeyondLastLine, ScrollbarDiagnostics, SeedQuerySetting, ShowMinimap, SnippetSortOrder,
    VsCodeSettings,
};
use settings::{Settings, SettingsContent};
use ui::scrollbars::{ScrollbarVisibility, ShowScrollbar};

/// Imports from the VSCode settings at
/// https://code.visualstudio.com/docs/reference/default-settings
#[derive(Clone)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub cursor_shape: Option<CursorShape>,
    pub current_line_highlight: CurrentLineHighlight,
    pub selection_highlight: bool,
    pub rounded_selection: bool,
    pub lsp_highlight_debounce: u64,
    pub hover_popover_enabled: bool,
    pub hover_popover_delay: u64,
    pub toolbar: Toolbar,
    pub scrollbar: Scrollbar,
    pub minimap: Minimap,
    pub gutter: Gutter,
    pub scroll_beyond_last_line: ScrollBeyondLastLine,
    pub vertical_scroll_margin: f64,
    pub autoscroll_on_clicks: bool,
    pub horizontal_scroll_margin: f32,
    pub scroll_sensitivity: f32,
    pub fast_scroll_sensitivity: f32,
    pub relative_line_numbers: bool,
    pub seed_search_query_from_cursor: SeedQuerySetting,
    pub use_smartcase_search: bool,
    pub multi_cursor_modifier: MultiCursorModifier,
    pub redact_private_values: bool,
    pub expand_excerpt_lines: u32,
    pub excerpt_context_lines: u32,
    pub middle_click_paste: bool,
    pub double_click_in_multibuffer: DoubleClickInMultibuffer,
    pub search_wrap: bool,
    pub search: SearchSettings,
    pub auto_signature_help: bool,
    pub show_signature_help_after_edits: bool,
    pub go_to_definition_fallback: GoToDefinitionFallback,
    pub jupyter: Jupyter,
    pub hide_mouse: Option<HideMouseMode>,
    pub snippet_sort_order: SnippetSortOrder,
    pub diagnostics_max_severity: Option<DiagnosticSeverity>,
    pub inline_code_actions: bool,
    pub drag_and_drop_selection: DragAndDropSelection,
    pub lsp_document_colors: DocumentColorsRenderMode,
    pub minimum_contrast_for_highlights: f32,
}
#[derive(Debug, Clone)]
pub struct Jupyter {
    /// Whether the Jupyter feature is enabled.
    ///
    /// Default: true
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
    pub quick_actions: bool,
    pub selections_menu: bool,
    pub agent_review: bool,
    pub code_actions: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
    pub selected_text: bool,
    pub selected_symbol: bool,
    pub search_results: bool,
    pub diagnostics: ScrollbarDiagnostics,
    pub cursors: bool,
    pub axes: ScrollbarAxes,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Minimap {
    pub show: ShowMinimap,
    pub display_in: DisplayIn,
    pub thumb: MinimapThumb,
    pub thumb_border: MinimapThumbBorder,
    pub current_line_highlight: Option<CurrentLineHighlight>,
    pub max_width_columns: num::NonZeroU32,
}

impl Minimap {
    pub fn minimap_enabled(&self) -> bool {
        self.show != ShowMinimap::Never
    }

    #[inline]
    pub fn on_active_editor(&self) -> bool {
        self.display_in == DisplayIn::ActiveEditor
    }

    pub fn with_show_override(self) -> Self {
        Self {
            show: ShowMinimap::Always,
            ..self
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gutter {
    pub min_line_number_digits: usize,
    pub line_numbers: bool,
    pub runnables: bool,
    pub breakpoints: bool,
    pub folds: bool,
}

/// Forcefully enable or disable the scrollbar for each axis
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarAxes {
    /// When false, forcefully disables the horizontal scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub horizontal: bool,

    /// When false, forcefully disables the vertical scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub vertical: bool,
}

/// Whether to allow drag and drop text selection in buffer.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct DragAndDropSelection {
    /// When true, enables drag and drop text selection in buffer.
    ///
    /// Default: true
    pub enabled: bool,

    /// The delay in milliseconds that must elapse before drag and drop is allowed. Otherwise, a new text selection is created.
    ///
    /// Default: 300
    pub delay: u64,
}

/// Default options for buffer and project search items.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SearchSettings {
    /// Whether to show the project search button in the status bar.
    pub button: bool,
    pub whole_word: bool,
    pub case_sensitive: bool,
    pub include_ignored: bool,
    pub regex: bool,
}

impl EditorSettings {
    pub fn jupyter_enabled(cx: &App) -> bool {
        EditorSettings::get_global(cx).jupyter.enabled
    }
}

impl ScrollbarVisibility for EditorSettings {
    fn visibility(&self, _cx: &App) -> ShowScrollbar {
        self.scrollbar.show
    }
}

impl Settings for EditorSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let editor = content.editor.clone();
        let scrollbar = editor.scrollbar.unwrap();
        let minimap = editor.minimap.unwrap();
        let gutter = editor.gutter.unwrap();
        let axes = scrollbar.axes.unwrap();
        let toolbar = editor.toolbar.unwrap();
        let search = editor.search.unwrap();
        let drag_and_drop_selection = editor.drag_and_drop_selection.unwrap();
        Self {
            cursor_blink: editor.cursor_blink.unwrap(),
            cursor_shape: editor.cursor_shape.map(Into::into),
            current_line_highlight: editor.current_line_highlight.unwrap(),
            selection_highlight: editor.selection_highlight.unwrap(),
            rounded_selection: editor.rounded_selection.unwrap(),
            lsp_highlight_debounce: editor.lsp_highlight_debounce.unwrap(),
            hover_popover_enabled: editor.hover_popover_enabled.unwrap(),
            hover_popover_delay: editor.hover_popover_delay.unwrap(),
            toolbar: Toolbar {
                breadcrumbs: toolbar.breadcrumbs.unwrap(),
                quick_actions: toolbar.quick_actions.unwrap(),
                selections_menu: toolbar.selections_menu.unwrap(),
                agent_review: toolbar.agent_review.unwrap(),
                code_actions: toolbar.code_actions.unwrap(),
            },
            scrollbar: Scrollbar {
                show: scrollbar.show.map(Into::into).unwrap(),
                git_diff: scrollbar.git_diff.unwrap(),
                selected_text: scrollbar.selected_text.unwrap(),
                selected_symbol: scrollbar.selected_symbol.unwrap(),
                search_results: scrollbar.search_results.unwrap(),
                diagnostics: scrollbar.diagnostics.unwrap(),
                cursors: scrollbar.cursors.unwrap(),
                axes: ScrollbarAxes {
                    horizontal: axes.horizontal.unwrap(),
                    vertical: axes.vertical.unwrap(),
                },
            },
            minimap: Minimap {
                show: minimap.show.unwrap(),
                display_in: minimap.display_in.unwrap(),
                thumb: minimap.thumb.unwrap(),
                thumb_border: minimap.thumb_border.unwrap(),
                current_line_highlight: minimap.current_line_highlight,
                max_width_columns: minimap.max_width_columns.unwrap(),
            },
            gutter: Gutter {
                min_line_number_digits: gutter.min_line_number_digits.unwrap(),
                line_numbers: gutter.line_numbers.unwrap(),
                runnables: gutter.runnables.unwrap(),
                breakpoints: gutter.breakpoints.unwrap(),
                folds: gutter.folds.unwrap(),
            },
            scroll_beyond_last_line: editor.scroll_beyond_last_line.unwrap(),
            vertical_scroll_margin: editor.vertical_scroll_margin.unwrap() as f64,
            autoscroll_on_clicks: editor.autoscroll_on_clicks.unwrap(),
            horizontal_scroll_margin: editor.horizontal_scroll_margin.unwrap(),
            scroll_sensitivity: editor.scroll_sensitivity.unwrap(),
            fast_scroll_sensitivity: editor.fast_scroll_sensitivity.unwrap(),
            relative_line_numbers: editor.relative_line_numbers.unwrap(),
            seed_search_query_from_cursor: editor.seed_search_query_from_cursor.unwrap(),
            use_smartcase_search: editor.use_smartcase_search.unwrap(),
            multi_cursor_modifier: editor.multi_cursor_modifier.unwrap(),
            redact_private_values: editor.redact_private_values.unwrap(),
            expand_excerpt_lines: editor.expand_excerpt_lines.unwrap(),
            excerpt_context_lines: editor.excerpt_context_lines.unwrap(),
            middle_click_paste: editor.middle_click_paste.unwrap(),
            double_click_in_multibuffer: editor.double_click_in_multibuffer.unwrap(),
            search_wrap: editor.search_wrap.unwrap(),
            search: SearchSettings {
                button: search.button.unwrap(),
                whole_word: search.whole_word.unwrap(),
                case_sensitive: search.case_sensitive.unwrap(),
                include_ignored: search.include_ignored.unwrap(),
                regex: search.regex.unwrap(),
            },
            auto_signature_help: editor.auto_signature_help.unwrap(),
            show_signature_help_after_edits: editor.show_signature_help_after_edits.unwrap(),
            go_to_definition_fallback: editor.go_to_definition_fallback.unwrap(),
            jupyter: Jupyter {
                enabled: editor.jupyter.unwrap().enabled.unwrap(),
            },
            hide_mouse: editor.hide_mouse,
            snippet_sort_order: editor.snippet_sort_order.unwrap(),
            diagnostics_max_severity: editor.diagnostics_max_severity.map(Into::into),
            inline_code_actions: editor.inline_code_actions.unwrap(),
            drag_and_drop_selection: DragAndDropSelection {
                enabled: drag_and_drop_selection.enabled.unwrap(),
                delay: drag_and_drop_selection.delay.unwrap(),
            },
            lsp_document_colors: editor.lsp_document_colors.unwrap(),
            minimum_contrast_for_highlights: editor.minimum_contrast_for_highlights.unwrap().0,
        }
    }

    fn import_from_vscode(vscode: &VsCodeSettings, current: &mut SettingsContent) {
        vscode.enum_setting(
            "editor.cursorBlinking",
            &mut current.editor.cursor_blink,
            |s| match s {
                "blink" | "phase" | "expand" | "smooth" => Some(true),
                "solid" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.cursorStyle",
            &mut current.editor.cursor_shape,
            |s| match s {
                "block" => Some(settings::CursorShape::Block),
                "block-outline" => Some(settings::CursorShape::Hollow),
                "line" | "line-thin" => Some(settings::CursorShape::Bar),
                "underline" | "underline-thin" => Some(settings::CursorShape::Underline),
                _ => None,
            },
        );

        vscode.enum_setting(
            "editor.renderLineHighlight",
            &mut current.editor.current_line_highlight,
            |s| match s {
                "gutter" => Some(CurrentLineHighlight::Gutter),
                "line" => Some(CurrentLineHighlight::Line),
                "all" => Some(CurrentLineHighlight::All),
                _ => None,
            },
        );

        vscode.bool_setting(
            "editor.selectionHighlight",
            &mut current.editor.selection_highlight,
        );
        vscode.bool_setting(
            "editor.roundedSelection",
            &mut current.editor.rounded_selection,
        );
        vscode.bool_setting(
            "editor.hover.enabled",
            &mut current.editor.hover_popover_enabled,
        );
        vscode.u64_setting(
            "editor.hover.delay",
            &mut current.editor.hover_popover_delay,
        );

        let mut gutter = settings::GutterContent::default();
        vscode.enum_setting(
            "editor.showFoldingControls",
            &mut gutter.folds,
            |s| match s {
                "always" | "mouseover" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.lineNumbers",
            &mut gutter.line_numbers,
            |s| match s {
                "on" | "relative" => Some(true),
                "off" => Some(false),
                _ => None,
            },
        );
        if let Some(old_gutter) = current.editor.gutter.as_mut() {
            if gutter.folds.is_some() {
                old_gutter.folds = gutter.folds
            }
            if gutter.line_numbers.is_some() {
                old_gutter.line_numbers = gutter.line_numbers
            }
        } else if gutter != settings::GutterContent::default() {
            current.editor.gutter = Some(gutter)
        }
        if let Some(b) = vscode.read_bool("editor.scrollBeyondLastLine") {
            current.editor.scroll_beyond_last_line = Some(if b {
                ScrollBeyondLastLine::OnePage
            } else {
                ScrollBeyondLastLine::Off
            })
        }

        let mut scrollbar_axes = settings::ScrollbarAxesContent::default();
        vscode.enum_setting(
            "editor.scrollbar.horizontal",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.scrollbar.vertical",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );

        if scrollbar_axes != settings::ScrollbarAxesContent::default() {
            let scrollbar_settings = current.editor.scrollbar.get_or_insert_default();
            let axes_settings = scrollbar_settings.axes.get_or_insert_default();

            if let Some(vertical) = scrollbar_axes.vertical {
                axes_settings.vertical = Some(vertical);
            }
            if let Some(horizontal) = scrollbar_axes.horizontal {
                axes_settings.horizontal = Some(horizontal);
            }
        }

        // TODO: check if this does the int->float conversion?
        vscode.f32_setting(
            "editor.cursorSurroundingLines",
            &mut current.editor.vertical_scroll_margin,
        );
        vscode.f32_setting(
            "editor.mouseWheelScrollSensitivity",
            &mut current.editor.scroll_sensitivity,
        );
        vscode.f32_setting(
            "editor.fastScrollSensitivity",
            &mut current.editor.fast_scroll_sensitivity,
        );
        if Some("relative") == vscode.read_string("editor.lineNumbers") {
            current.editor.relative_line_numbers = Some(true);
        }

        vscode.enum_setting(
            "editor.find.seedSearchStringFromSelection",
            &mut current.editor.seed_search_query_from_cursor,
            |s| match s {
                "always" => Some(SeedQuerySetting::Always),
                "selection" => Some(SeedQuerySetting::Selection),
                "never" => Some(SeedQuerySetting::Never),
                _ => None,
            },
        );
        vscode.bool_setting("search.smartCase", &mut current.editor.use_smartcase_search);
        vscode.enum_setting(
            "editor.multiCursorModifier",
            &mut current.editor.multi_cursor_modifier,
            |s| match s {
                "ctrlCmd" => Some(MultiCursorModifier::CmdOrCtrl),
                "alt" => Some(MultiCursorModifier::Alt),
                _ => None,
            },
        );

        vscode.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.editor.auto_signature_help,
        );
        vscode.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.editor.show_signature_help_after_edits,
        );

        if let Some(use_ignored) = vscode.read_bool("search.useIgnoreFiles") {
            let search = current.editor.search.get_or_insert_default();
            search.include_ignored = Some(use_ignored);
        }

        let mut minimap = settings::MinimapContent::default();
        let minimap_enabled = vscode.read_bool("editor.minimap.enabled").unwrap_or(true);
        let autohide = vscode.read_bool("editor.minimap.autohide");
        let mut max_width_columns: Option<u32> = None;
        vscode.u32_setting("editor.minimap.maxColumn", &mut max_width_columns);
        if minimap_enabled {
            if let Some(false) = autohide {
                minimap.show = Some(ShowMinimap::Always);
            } else {
                minimap.show = Some(ShowMinimap::Auto);
            }
        } else {
            minimap.show = Some(ShowMinimap::Never);
        }
        if let Some(max_width_columns) = max_width_columns {
            minimap.max_width_columns = NonZeroU32::new(max_width_columns);
        }

        vscode.enum_setting(
            "editor.minimap.showSlider",
            &mut minimap.thumb,
            |s| match s {
                "always" => Some(MinimapThumb::Always),
                "mouseover" => Some(MinimapThumb::Hover),
                _ => None,
            },
        );

        if minimap != settings::MinimapContent::default() {
            current.editor.minimap = Some(minimap)
        }
    }
}
