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
use util::MergeFrom;

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
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub scrollbar: Scrollbar,
    pub minimap: Minimap,
    pub gutter: Gutter,
    pub scroll_beyond_last_line: ScrollBeyondLastLine,
    pub vertical_scroll_margin: f32,
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
pub struct StatusBar {
    /// Whether to display the active language button in the status bar.
    ///
    /// Default: true
    pub active_language_button: bool,
    /// Whether to show the cursor position button in the status bar.
    ///
    /// Default: true
    pub cursor_position_button: bool,
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
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let editor = content.editor.clone();
        let scrollbar = editor.scrollbar.unwrap();
        let minimap = editor.minimap.unwrap();
        let gutter = editor.gutter.unwrap();
        let axes = scrollbar.axes.unwrap();
        let status_bar = editor.status_bar.unwrap();
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
            status_bar: StatusBar {
                active_language_button: status_bar.active_language_button.unwrap(),
                cursor_position_button: status_bar.cursor_position_button.unwrap(),
            },
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
                current_line_highlight: minimap.current_line_highlight.flatten(),
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
            vertical_scroll_margin: editor.vertical_scroll_margin.unwrap(),
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
            minimum_contrast_for_highlights: editor.minimum_contrast_for_highlights.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut App) {
        let editor = &content.editor;
        self.cursor_blink.merge_from(&editor.cursor_blink);
        if let Some(cursor_shape) = editor.cursor_shape {
            self.cursor_shape = Some(cursor_shape.into())
        }
        self.current_line_highlight
            .merge_from(&editor.current_line_highlight);
        self.selection_highlight
            .merge_from(&editor.selection_highlight);
        self.rounded_selection.merge_from(&editor.rounded_selection);
        self.lsp_highlight_debounce
            .merge_from(&editor.lsp_highlight_debounce);
        self.hover_popover_enabled
            .merge_from(&editor.hover_popover_enabled);
        self.hover_popover_delay
            .merge_from(&editor.hover_popover_delay);
        self.scroll_beyond_last_line
            .merge_from(&editor.scroll_beyond_last_line);
        self.vertical_scroll_margin
            .merge_from(&editor.vertical_scroll_margin);
        self.autoscroll_on_clicks
            .merge_from(&editor.autoscroll_on_clicks);
        self.horizontal_scroll_margin
            .merge_from(&editor.horizontal_scroll_margin);
        self.scroll_sensitivity
            .merge_from(&editor.scroll_sensitivity);
        self.fast_scroll_sensitivity
            .merge_from(&editor.fast_scroll_sensitivity);
        self.relative_line_numbers
            .merge_from(&editor.relative_line_numbers);
        self.seed_search_query_from_cursor
            .merge_from(&editor.seed_search_query_from_cursor);
        self.use_smartcase_search
            .merge_from(&editor.use_smartcase_search);
        self.multi_cursor_modifier
            .merge_from(&editor.multi_cursor_modifier);
        self.redact_private_values
            .merge_from(&editor.redact_private_values);
        self.expand_excerpt_lines
            .merge_from(&editor.expand_excerpt_lines);
        self.excerpt_context_lines
            .merge_from(&editor.excerpt_context_lines);
        self.middle_click_paste
            .merge_from(&editor.middle_click_paste);
        self.double_click_in_multibuffer
            .merge_from(&editor.double_click_in_multibuffer);
        self.search_wrap.merge_from(&editor.search_wrap);
        self.auto_signature_help
            .merge_from(&editor.auto_signature_help);
        self.show_signature_help_after_edits
            .merge_from(&editor.show_signature_help_after_edits);
        self.go_to_definition_fallback
            .merge_from(&editor.go_to_definition_fallback);
        if let Some(hide_mouse) = editor.hide_mouse {
            self.hide_mouse = Some(hide_mouse)
        }
        self.snippet_sort_order
            .merge_from(&editor.snippet_sort_order);
        if let Some(diagnostics_max_severity) = editor.diagnostics_max_severity {
            self.diagnostics_max_severity = Some(diagnostics_max_severity.into());
        }
        self.inline_code_actions
            .merge_from(&editor.inline_code_actions);
        self.lsp_document_colors
            .merge_from(&editor.lsp_document_colors);
        self.minimum_contrast_for_highlights
            .merge_from(&editor.minimum_contrast_for_highlights);

        if let Some(status_bar) = &editor.status_bar {
            self.status_bar
                .active_language_button
                .merge_from(&status_bar.active_language_button);
            self.status_bar
                .cursor_position_button
                .merge_from(&status_bar.cursor_position_button);
        }
        if let Some(toolbar) = &editor.toolbar {
            self.toolbar.breadcrumbs.merge_from(&toolbar.breadcrumbs);
            self.toolbar
                .quick_actions
                .merge_from(&toolbar.quick_actions);
            self.toolbar
                .selections_menu
                .merge_from(&toolbar.selections_menu);
            self.toolbar.agent_review.merge_from(&toolbar.agent_review);
            self.toolbar.code_actions.merge_from(&toolbar.code_actions);
        }
        if let Some(scrollbar) = &editor.scrollbar {
            self.scrollbar
                .show
                .merge_from(&scrollbar.show.map(Into::into));
            self.scrollbar.git_diff.merge_from(&scrollbar.git_diff);
            self.scrollbar
                .selected_text
                .merge_from(&scrollbar.selected_text);
            self.scrollbar
                .selected_symbol
                .merge_from(&scrollbar.selected_symbol);
            self.scrollbar
                .search_results
                .merge_from(&scrollbar.search_results);
            self.scrollbar
                .diagnostics
                .merge_from(&scrollbar.diagnostics);
            self.scrollbar.cursors.merge_from(&scrollbar.cursors);
            if let Some(axes) = &scrollbar.axes {
                self.scrollbar.axes.horizontal.merge_from(&axes.horizontal);
                self.scrollbar.axes.vertical.merge_from(&axes.vertical);
            }
        }
        if let Some(minimap) = &editor.minimap {
            self.minimap.show.merge_from(&minimap.show);
            self.minimap.display_in.merge_from(&minimap.display_in);
            self.minimap.thumb.merge_from(&minimap.thumb);
            self.minimap.thumb_border.merge_from(&minimap.thumb_border);
            self.minimap
                .current_line_highlight
                .merge_from(&minimap.current_line_highlight);
            self.minimap
                .max_width_columns
                .merge_from(&minimap.max_width_columns);
        }
        if let Some(gutter) = &editor.gutter {
            self.gutter
                .min_line_number_digits
                .merge_from(&gutter.min_line_number_digits);
            self.gutter.line_numbers.merge_from(&gutter.line_numbers);
            self.gutter.runnables.merge_from(&gutter.runnables);
            self.gutter.breakpoints.merge_from(&gutter.breakpoints);
            self.gutter.folds.merge_from(&gutter.folds);
        }
        if let Some(search) = &editor.search {
            self.search.button.merge_from(&search.button);
            self.search.whole_word.merge_from(&search.whole_word);
            self.search
                .case_sensitive
                .merge_from(&search.case_sensitive);
            self.search
                .include_ignored
                .merge_from(&search.include_ignored);
            self.search.regex.merge_from(&search.regex);
        }
        if let Some(enabled) = editor.jupyter.as_ref().and_then(|jupyter| jupyter.enabled) {
            self.jupyter.enabled = enabled;
        }
        if let Some(drag_and_drop_selection) = &editor.drag_and_drop_selection {
            self.drag_and_drop_selection
                .enabled
                .merge_from(&drag_and_drop_selection.enabled);
            self.drag_and_drop_selection
                .delay
                .merge_from(&drag_and_drop_selection.delay);
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
