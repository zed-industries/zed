use crate::{
    ActiveDiagnostic, BlockId, COLUMNAR_SELECTION_MODIFIERS, CURSORS_VISIBLE_FOR,
    ChunkRendererContext, ChunkReplacement, ConflictsOurs, ConflictsOursMarker, ConflictsOuter,
    ConflictsTheirs, ConflictsTheirsMarker, ContextMenuPlacement, CursorShape, CustomBlockId,
    DisplayDiffHunk, DisplayPoint, DisplayRow, DocumentHighlightRead, DocumentHighlightWrite,
    EditDisplayMode, Editor, EditorMode, EditorSettings, EditorSnapshot, EditorStyle,
    FILE_HEADER_HEIGHT, FocusedBlock, GutterDimensions, HalfPageDown, HalfPageUp, HandleInput,
    HoveredCursor, InlayHintRefreshReason, InlineCompletion, JumpData, LineDown, LineHighlight,
    LineUp, MAX_LINE_LEN, MIN_LINE_NUMBER_DIGITS, MULTI_BUFFER_EXCERPT_HEADER_HEIGHT, OpenExcerpts,
    PageDown, PageUp, PhantomBreakpointIndicator, Point, RowExt, RowRangeExt, SelectPhase,
    SelectedTextHighlight, Selection, SoftWrap, StickyHeaderExcerpt, ToPoint, ToggleFold,
    code_context_menus::{CodeActionsMenu, MENU_ASIDE_MAX_WIDTH, MENU_ASIDE_MIN_WIDTH, MENU_GAP},
    display_map::{
        Block, BlockContext, BlockStyle, DisplaySnapshot, FoldId, HighlightedChunk, ToDisplayPoint,
    },
    editor_settings::{
        CurrentLineHighlight, DoubleClickInMultibuffer, MultiCursorModifier, ScrollBeyondLastLine,
        ScrollbarAxes, ScrollbarDiagnostics, ShowScrollbar,
    },
    git::blame::{BlameRenderer, GitBlame, GlobalBlameRenderer},
    hover_popover::{
        self, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT,
        POPOVER_RIGHT_OFFSET, hover_at,
    },
    inlay_hint_settings,
    items::BufferSearchHighlights,
    mouse_context_menu::{self, MenuPosition},
    scroll::scroll_amount::ScrollAmount,
};
use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
use client::ParticipantIndex;
use collections::{BTreeMap, HashMap};
use feature_flags::{DebuggerFeatureFlag, FeatureFlagAppExt};
use file_icons::FileIcons;
use git::{
    Oid,
    blame::{BlameEntry, ParsedCommitMessage},
    status::FileStatus,
};
use gpui::{
    Action, Along, AnyElement, App, AppContext, AvailableSpace, Axis as ScrollbarAxis, BorderStyle,
    Bounds, ClickEvent, ContentMask, Context, Corner, Corners, CursorStyle, DispatchPhase, Edges,
    Element, ElementInputHandler, Entity, Focusable as _, FontId, GlobalElementId, Hitbox, Hsla,
    InteractiveElement, IntoElement, Keystroke, Length, ModifiersChangedEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, ParentElement, Pixels, ScrollDelta,
    ScrollHandle, ScrollWheelEvent, ShapedLine, SharedString, Size, StatefulInteractiveElement,
    Style, Styled, TextRun, TextStyleRefinement, WeakEntity, Window, anchored, deferred, div, fill,
    linear_color_stop, linear_gradient, outline, point, px, quad, relative, size, solid_background,
    transparent_black,
};
use itertools::Itertools;
use language::language_settings::{
    IndentGuideBackgroundColoring, IndentGuideColoring, IndentGuideSettings, ShowWhitespaceSetting,
};
use lsp::DiagnosticSeverity;
use markdown::Markdown;
use multi_buffer::{
    Anchor, ExcerptId, ExcerptInfo, ExpandExcerptDirection, ExpandInfo, MultiBufferPoint,
    MultiBufferRow, RowInfo,
};
use project::{
    debugger::breakpoint_store::Breakpoint,
    project_settings::{self, GitGutterSetting, GitHunkStyleSetting, ProjectSettings},
};
use settings::Settings;
use smallvec::{SmallVec, smallvec};
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering},
    fmt::{self, Write},
    iter, mem,
    ops::{Deref, Range},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use sum_tree::Bias;
use text::BufferId;
use theme::{ActiveTheme, Appearance, BufferLineHeight, PlayerColor};
use ui::{ButtonLike, KeyBinding, POPOVER_Y_PADDING, Tooltip, h_flex, prelude::*};
use unicode_segmentation::UnicodeSegmentation;
use util::{RangeExt, ResultExt, debug_panic};
use workspace::{Workspace, item::Item, notifications::NotifyTaskExt};

const INLINE_BLAME_PADDING_EM_WIDTHS: f32 = 7.;

/// Determines what kinds of highlights should be applied to a lines background.
#[derive(Clone, Copy, Default)]
struct LineHighlightSpec {
    selection: bool,
    breakpoint: bool,
    _active_stack_frame: bool,
}

struct SelectionLayout {
    head: DisplayPoint,
    cursor_shape: CursorShape,
    is_newest: bool,
    is_local: bool,
    range: Range<DisplayPoint>,
    active_rows: Range<DisplayRow>,
    user_name: Option<SharedString>,
}

impl SelectionLayout {
    fn new<T: ToPoint + ToDisplayPoint + Clone>(
        selection: Selection<T>,
        line_mode: bool,
        cursor_shape: CursorShape,
        map: &DisplaySnapshot,
        is_newest: bool,
        is_local: bool,
        user_name: Option<SharedString>,
    ) -> Self {
        let point_selection = selection.map(|p| p.to_point(&map.buffer_snapshot));
        let display_selection = point_selection.map(|p| p.to_display_point(map));
        let mut range = display_selection.range();
        let mut head = display_selection.head();
        let mut active_rows = map.prev_line_boundary(point_selection.start).1.row()
            ..map.next_line_boundary(point_selection.end).1.row();

        // vim visual line mode
        if line_mode {
            let point_range = map.expand_to_line(point_selection.range());
            range = point_range.start.to_display_point(map)..point_range.end.to_display_point(map);
        }

        // any vim visual mode (including line mode)
        if (cursor_shape == CursorShape::Block || cursor_shape == CursorShape::Hollow)
            && !range.is_empty()
            && !selection.reversed
        {
            if head.column() > 0 {
                head = map.clip_point(DisplayPoint::new(head.row(), head.column() - 1), Bias::Left)
            } else if head.row().0 > 0 && head != map.max_point() {
                head = map.clip_point(
                    DisplayPoint::new(
                        head.row().previous_row(),
                        map.line_len(head.row().previous_row()),
                    ),
                    Bias::Left,
                );
                // updating range.end is a no-op unless you're cursor is
                // on the newline containing a multi-buffer divider
                // in which case the clip_point may have moved the head up
                // an additional row.
                range.end = DisplayPoint::new(head.row().next_row(), 0);
                active_rows.end = head.row();
            }
        }

        Self {
            head,
            cursor_shape,
            is_newest,
            is_local,
            range,
            active_rows,
            user_name,
        }
    }
}

pub struct EditorElement {
    editor: Entity<Editor>,
    style: EditorStyle,
}

type DisplayRowDelta = u32;

impl EditorElement {
    pub(crate) const SCROLLBAR_WIDTH: Pixels = px(15.);

    pub fn new(editor: &Entity<Editor>, style: EditorStyle) -> Self {
        Self {
            editor: editor.clone(),
            style,
        }
    }

    fn register_actions(&self, window: &mut Window, cx: &mut App) {
        let editor = &self.editor;
        editor.update(cx, |editor, cx| {
            for action in editor.editor_actions.borrow().values() {
                (action)(window, cx)
            }
        });

        crate::rust_analyzer_ext::apply_related_actions(editor, window, cx);
        crate::clangd_ext::apply_related_actions(editor, window, cx);

        register_action(editor, window, Editor::open_context_menu);
        register_action(editor, window, Editor::move_left);
        register_action(editor, window, Editor::move_right);
        register_action(editor, window, Editor::move_down);
        register_action(editor, window, Editor::move_down_by_lines);
        register_action(editor, window, Editor::select_down_by_lines);
        register_action(editor, window, Editor::move_up);
        register_action(editor, window, Editor::move_up_by_lines);
        register_action(editor, window, Editor::select_up_by_lines);
        register_action(editor, window, Editor::select_page_down);
        register_action(editor, window, Editor::select_page_up);
        register_action(editor, window, Editor::cancel);
        register_action(editor, window, Editor::newline);
        register_action(editor, window, Editor::newline_above);
        register_action(editor, window, Editor::newline_below);
        register_action(editor, window, Editor::backspace);
        register_action(editor, window, Editor::delete);
        register_action(editor, window, Editor::tab);
        register_action(editor, window, Editor::backtab);
        register_action(editor, window, Editor::indent);
        register_action(editor, window, Editor::outdent);
        register_action(editor, window, Editor::autoindent);
        register_action(editor, window, Editor::delete_line);
        register_action(editor, window, Editor::join_lines);
        register_action(editor, window, Editor::sort_lines_case_sensitive);
        register_action(editor, window, Editor::sort_lines_case_insensitive);
        register_action(editor, window, Editor::reverse_lines);
        register_action(editor, window, Editor::shuffle_lines);
        register_action(editor, window, Editor::toggle_case);
        register_action(editor, window, Editor::convert_to_upper_case);
        register_action(editor, window, Editor::convert_to_lower_case);
        register_action(editor, window, Editor::convert_to_title_case);
        register_action(editor, window, Editor::convert_to_snake_case);
        register_action(editor, window, Editor::convert_to_kebab_case);
        register_action(editor, window, Editor::convert_to_upper_camel_case);
        register_action(editor, window, Editor::convert_to_lower_camel_case);
        register_action(editor, window, Editor::convert_to_opposite_case);
        register_action(editor, window, Editor::convert_to_rot13);
        register_action(editor, window, Editor::convert_to_rot47);
        register_action(editor, window, Editor::delete_to_previous_word_start);
        register_action(editor, window, Editor::delete_to_previous_subword_start);
        register_action(editor, window, Editor::delete_to_next_word_end);
        register_action(editor, window, Editor::delete_to_next_subword_end);
        register_action(editor, window, Editor::delete_to_beginning_of_line);
        register_action(editor, window, Editor::delete_to_end_of_line);
        register_action(editor, window, Editor::cut_to_end_of_line);
        register_action(editor, window, Editor::duplicate_line_up);
        register_action(editor, window, Editor::duplicate_line_down);
        register_action(editor, window, Editor::duplicate_selection);
        register_action(editor, window, Editor::move_line_up);
        register_action(editor, window, Editor::move_line_down);
        register_action(editor, window, Editor::transpose);
        register_action(editor, window, Editor::rewrap);
        register_action(editor, window, Editor::cut);
        register_action(editor, window, Editor::kill_ring_cut);
        register_action(editor, window, Editor::kill_ring_yank);
        register_action(editor, window, Editor::copy);
        register_action(editor, window, Editor::copy_and_trim);
        register_action(editor, window, Editor::paste);
        register_action(editor, window, Editor::undo);
        register_action(editor, window, Editor::redo);
        register_action(editor, window, Editor::move_page_up);
        register_action(editor, window, Editor::move_page_down);
        register_action(editor, window, Editor::next_screen);
        register_action(editor, window, Editor::scroll_cursor_top);
        register_action(editor, window, Editor::scroll_cursor_center);
        register_action(editor, window, Editor::scroll_cursor_bottom);
        register_action(editor, window, Editor::scroll_cursor_center_top_bottom);
        register_action(editor, window, |editor, _: &LineDown, window, cx| {
            editor.scroll_screen(&ScrollAmount::Line(1.), window, cx)
        });
        register_action(editor, window, |editor, _: &LineUp, window, cx| {
            editor.scroll_screen(&ScrollAmount::Line(-1.), window, cx)
        });
        register_action(editor, window, |editor, _: &HalfPageDown, window, cx| {
            editor.scroll_screen(&ScrollAmount::Page(0.5), window, cx)
        });
        register_action(
            editor,
            window,
            |editor, HandleInput(text): &HandleInput, window, cx| {
                if text.is_empty() {
                    return;
                }
                editor.handle_input(text, window, cx);
            },
        );
        register_action(editor, window, |editor, _: &HalfPageUp, window, cx| {
            editor.scroll_screen(&ScrollAmount::Page(-0.5), window, cx)
        });
        register_action(editor, window, |editor, _: &PageDown, window, cx| {
            editor.scroll_screen(&ScrollAmount::Page(1.), window, cx)
        });
        register_action(editor, window, |editor, _: &PageUp, window, cx| {
            editor.scroll_screen(&ScrollAmount::Page(-1.), window, cx)
        });
        register_action(editor, window, Editor::move_to_previous_word_start);
        register_action(editor, window, Editor::move_to_previous_subword_start);
        register_action(editor, window, Editor::move_to_next_word_end);
        register_action(editor, window, Editor::move_to_next_subword_end);
        register_action(editor, window, Editor::move_to_beginning_of_line);
        register_action(editor, window, Editor::move_to_end_of_line);
        register_action(editor, window, Editor::move_to_start_of_paragraph);
        register_action(editor, window, Editor::move_to_end_of_paragraph);
        register_action(editor, window, Editor::move_to_beginning);
        register_action(editor, window, Editor::move_to_end);
        register_action(editor, window, Editor::move_to_start_of_excerpt);
        register_action(editor, window, Editor::move_to_start_of_next_excerpt);
        register_action(editor, window, Editor::move_to_end_of_excerpt);
        register_action(editor, window, Editor::move_to_end_of_previous_excerpt);
        register_action(editor, window, Editor::select_up);
        register_action(editor, window, Editor::select_down);
        register_action(editor, window, Editor::select_left);
        register_action(editor, window, Editor::select_right);
        register_action(editor, window, Editor::select_to_previous_word_start);
        register_action(editor, window, Editor::select_to_previous_subword_start);
        register_action(editor, window, Editor::select_to_next_word_end);
        register_action(editor, window, Editor::select_to_next_subword_end);
        register_action(editor, window, Editor::select_to_beginning_of_line);
        register_action(editor, window, Editor::select_to_end_of_line);
        register_action(editor, window, Editor::select_to_start_of_paragraph);
        register_action(editor, window, Editor::select_to_end_of_paragraph);
        register_action(editor, window, Editor::select_to_start_of_excerpt);
        register_action(editor, window, Editor::select_to_start_of_next_excerpt);
        register_action(editor, window, Editor::select_to_end_of_excerpt);
        register_action(editor, window, Editor::select_to_end_of_previous_excerpt);
        register_action(editor, window, Editor::select_to_beginning);
        register_action(editor, window, Editor::select_to_end);
        register_action(editor, window, Editor::select_all);
        register_action(editor, window, |editor, action, window, cx| {
            editor.select_all_matches(action, window, cx).log_err();
        });
        register_action(editor, window, Editor::select_line);
        register_action(editor, window, Editor::split_selection_into_lines);
        register_action(editor, window, Editor::add_selection_above);
        register_action(editor, window, Editor::add_selection_below);
        register_action(editor, window, |editor, action, window, cx| {
            editor.select_next(action, window, cx).log_err();
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor.select_previous(action, window, cx).log_err();
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor.find_next_match(action, window, cx).log_err();
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor.find_previous_match(action, window, cx).log_err();
        });
        register_action(editor, window, Editor::toggle_comments);
        register_action(editor, window, Editor::select_larger_syntax_node);
        register_action(editor, window, Editor::select_smaller_syntax_node);
        register_action(editor, window, Editor::select_enclosing_symbol);
        register_action(editor, window, Editor::move_to_enclosing_bracket);
        register_action(editor, window, Editor::undo_selection);
        register_action(editor, window, Editor::redo_selection);
        if !editor.read(cx).is_singleton(cx) {
            register_action(editor, window, Editor::expand_excerpts);
            register_action(editor, window, Editor::expand_excerpts_up);
            register_action(editor, window, Editor::expand_excerpts_down);
        }
        register_action(editor, window, Editor::go_to_diagnostic);
        register_action(editor, window, Editor::go_to_prev_diagnostic);
        register_action(editor, window, Editor::go_to_next_hunk);
        register_action(editor, window, Editor::go_to_prev_hunk);
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_definition(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_definition_split(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_declaration(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_declaration_split(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_implementation(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_implementation_split(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_type_definition(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, |editor, action, window, cx| {
            editor
                .go_to_type_definition_split(action, window, cx)
                .detach_and_log_err(cx);
        });
        register_action(editor, window, Editor::open_url);
        register_action(editor, window, Editor::open_selected_filename);
        register_action(editor, window, Editor::fold);
        register_action(editor, window, Editor::fold_at_level);
        register_action(editor, window, Editor::fold_all);
        register_action(editor, window, Editor::fold_function_bodies);
        register_action(editor, window, Editor::fold_recursive);
        register_action(editor, window, Editor::toggle_fold);
        register_action(editor, window, Editor::toggle_fold_recursive);
        register_action(editor, window, Editor::unfold_lines);
        register_action(editor, window, Editor::unfold_recursive);
        register_action(editor, window, Editor::unfold_all);
        register_action(editor, window, Editor::fold_selected_ranges);
        register_action(editor, window, Editor::set_mark);
        register_action(editor, window, Editor::swap_selection_ends);
        register_action(editor, window, Editor::show_completions);
        register_action(editor, window, Editor::show_word_completions);
        register_action(editor, window, Editor::toggle_code_actions);
        register_action(editor, window, Editor::open_excerpts);
        register_action(editor, window, Editor::open_excerpts_in_split);
        register_action(editor, window, Editor::open_proposed_changes_editor);
        register_action(editor, window, Editor::toggle_soft_wrap);
        register_action(editor, window, Editor::toggle_tab_bar);
        register_action(editor, window, Editor::toggle_line_numbers);
        register_action(editor, window, Editor::toggle_relative_line_numbers);
        register_action(editor, window, Editor::toggle_indent_guides);
        register_action(editor, window, Editor::toggle_inlay_hints);
        register_action(editor, window, Editor::toggle_edit_predictions);
        register_action(editor, window, Editor::toggle_inline_diagnostics);
        register_action(editor, window, hover_popover::hover);
        register_action(editor, window, Editor::reveal_in_finder);
        register_action(editor, window, Editor::copy_path);
        register_action(editor, window, Editor::copy_relative_path);
        register_action(editor, window, Editor::copy_file_name);
        register_action(editor, window, Editor::copy_file_name_without_extension);
        register_action(editor, window, Editor::copy_highlight_json);
        register_action(editor, window, Editor::copy_permalink_to_line);
        register_action(editor, window, Editor::open_permalink_to_line);
        register_action(editor, window, Editor::copy_file_location);
        register_action(editor, window, Editor::toggle_git_blame);
        register_action(editor, window, Editor::toggle_git_blame_inline);
        register_action(editor, window, Editor::open_git_blame_commit);
        register_action(editor, window, Editor::toggle_selected_diff_hunks);
        register_action(editor, window, Editor::toggle_staged_selected_diff_hunks);
        register_action(editor, window, Editor::stage_and_next);
        register_action(editor, window, Editor::unstage_and_next);
        register_action(editor, window, Editor::expand_all_diff_hunks);
        register_action(editor, window, Editor::go_to_previous_change);
        register_action(editor, window, Editor::go_to_next_change);

        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.format(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.format_selections(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.organize_imports(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, Editor::restart_language_server);
        register_action(editor, window, Editor::stop_language_server);
        register_action(editor, window, Editor::show_character_palette);
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.confirm_completion(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.confirm_completion_replace(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.confirm_completion_insert(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.compose_completion(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.confirm_code_action(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.rename(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.confirm_rename(action, window, cx) {
                task.detach_and_notify_err(window, cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.find_all_references(action, window, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(editor, window, Editor::show_signature_help);
        register_action(editor, window, Editor::next_edit_prediction);
        register_action(editor, window, Editor::previous_edit_prediction);
        register_action(editor, window, Editor::show_inline_completion);
        register_action(editor, window, Editor::context_menu_first);
        register_action(editor, window, Editor::context_menu_prev);
        register_action(editor, window, Editor::context_menu_next);
        register_action(editor, window, Editor::context_menu_last);
        register_action(editor, window, Editor::display_cursor_names);
        register_action(editor, window, Editor::unique_lines_case_insensitive);
        register_action(editor, window, Editor::unique_lines_case_sensitive);
        register_action(editor, window, Editor::accept_partial_inline_completion);
        register_action(editor, window, Editor::accept_edit_prediction);
        register_action(editor, window, Editor::restore_file);
        register_action(editor, window, Editor::git_restore);
        register_action(editor, window, Editor::apply_all_diff_hunks);
        register_action(editor, window, Editor::apply_selected_diff_hunks);
        register_action(editor, window, Editor::open_active_item_in_terminal);
        register_action(editor, window, Editor::reload_file);
        register_action(editor, window, Editor::spawn_nearest_task);
        register_action(editor, window, Editor::insert_uuid_v4);
        register_action(editor, window, Editor::insert_uuid_v7);
        register_action(editor, window, Editor::open_selections_in_multibuffer);
        if cx.has_flag::<DebuggerFeatureFlag>() {
            register_action(editor, window, Editor::toggle_breakpoint);
            register_action(editor, window, Editor::edit_log_breakpoint);
            register_action(editor, window, Editor::enable_breakpoint);
            register_action(editor, window, Editor::disable_breakpoint);
        }
    }

    fn register_key_listeners(&self, window: &mut Window, _: &mut App, layout: &EditorLayout) {
        let position_map = layout.position_map.clone();
        window.on_key_event({
            let editor = self.editor.clone();
            move |event: &ModifiersChangedEvent, phase, window, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }
                editor.update(cx, |editor, cx| {
                    let inlay_hint_settings = inlay_hint_settings(
                        editor.selections.newest_anchor().head(),
                        &editor.buffer.read(cx).snapshot(cx),
                        cx,
                    );

                    if let Some(inlay_modifiers) = inlay_hint_settings
                        .toggle_on_modifiers_press
                        .as_ref()
                        .filter(|modifiers| modifiers.modified())
                    {
                        editor.refresh_inlay_hints(
                            InlayHintRefreshReason::ModifiersChanged(
                                inlay_modifiers == &event.modifiers,
                            ),
                            cx,
                        );
                    }

                    if editor.hover_state.focused(window, cx) {
                        return;
                    }

                    editor.handle_modifiers_changed(event.modifiers, &position_map, window, cx);
                })
            }
        });
    }

    fn mouse_left_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        hovered_hunk: Option<Range<Anchor>>,
        position_map: &PositionMap,
        line_numbers: &HashMap<MultiBufferRow, LineNumberLayout>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if window.default_prevented() {
            return;
        }

        let text_hitbox = &position_map.text_hitbox;
        let gutter_hitbox = &position_map.gutter_hitbox;
        let mut click_count = event.click_count;
        let mut modifiers = event.modifiers;

        if let Some(hovered_hunk) = hovered_hunk {
            editor.toggle_single_diff_hunk(hovered_hunk, cx);
            cx.notify();
            return;
        } else if gutter_hitbox.is_hovered(window) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !text_hitbox.is_hovered(window) {
            return;
        }

        let is_singleton = editor.buffer().read(cx).is_singleton();

        if click_count == 2 && !is_singleton {
            match EditorSettings::get_global(cx).double_click_in_multibuffer {
                DoubleClickInMultibuffer::Select => {
                    // do nothing special on double click, all selection logic is below
                }
                DoubleClickInMultibuffer::Open => {
                    if modifiers.alt {
                        // if double click is made with alt, pretend it's a regular double click without opening and alt,
                        // and run the selection logic.
                        modifiers.alt = false;
                    } else {
                        let scroll_position_row =
                            position_map.scroll_pixel_position.y / position_map.line_height;
                        let display_row = (((event.position - gutter_hitbox.bounds.origin).y
                            + position_map.scroll_pixel_position.y)
                            / position_map.line_height)
                            as u32;
                        let multi_buffer_row = position_map
                            .snapshot
                            .display_point_to_point(
                                DisplayPoint::new(DisplayRow(display_row), 0),
                                Bias::Right,
                            )
                            .row;
                        let line_offset_from_top = display_row - scroll_position_row as u32;
                        // if double click is made without alt, open the corresponding excerp
                        editor.open_excerpts_common(
                            Some(JumpData::MultiBufferRow {
                                row: MultiBufferRow(multi_buffer_row),
                                line_offset_from_top,
                            }),
                            false,
                            window,
                            cx,
                        );
                        return;
                    }
                }
            }
        }

        let point_for_position = position_map.point_for_position(event.position);
        let position = point_for_position.previous_valid;
        if modifiers == COLUMNAR_SELECTION_MODIFIERS {
            editor.select(
                SelectPhase::BeginColumnar {
                    position,
                    reset: false,
                    goal_column: point_for_position.exact_unclipped.column(),
                },
                window,
                cx,
            );
        } else if modifiers.shift && !modifiers.control && !modifiers.alt && !modifiers.secondary()
        {
            editor.select(
                SelectPhase::Extend {
                    position,
                    click_count,
                },
                window,
                cx,
            );
        } else {
            let multi_cursor_setting = EditorSettings::get_global(cx).multi_cursor_modifier;
            let multi_cursor_modifier = match multi_cursor_setting {
                MultiCursorModifier::Alt => modifiers.alt,
                MultiCursorModifier::CmdOrCtrl => modifiers.secondary(),
            };
            editor.select(
                SelectPhase::Begin {
                    position,
                    add: multi_cursor_modifier,
                    click_count,
                },
                window,
                cx,
            );
        }
        cx.stop_propagation();

        if !is_singleton {
            let display_row = (((event.position - gutter_hitbox.bounds.origin).y
                + position_map.scroll_pixel_position.y)
                / position_map.line_height) as u32;
            let multi_buffer_row = position_map
                .snapshot
                .display_point_to_point(DisplayPoint::new(DisplayRow(display_row), 0), Bias::Right)
                .row;
            if line_numbers
                .get(&MultiBufferRow(multi_buffer_row))
                .and_then(|line_number| line_number.hitbox.as_ref())
                .is_some_and(|hitbox| hitbox.contains(&event.position))
            {
                let scroll_position_row =
                    position_map.scroll_pixel_position.y / position_map.line_height;
                let line_offset_from_top = display_row - scroll_position_row as u32;

                editor.open_excerpts_common(
                    Some(JumpData::MultiBufferRow {
                        row: MultiBufferRow(multi_buffer_row),
                        line_offset_from_top,
                    }),
                    modifiers.alt,
                    window,
                    cx,
                );
                cx.stop_propagation();
            }
        }
    }

    fn mouse_right_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if position_map.gutter_hitbox.is_hovered(window) {
            let gutter_right_padding = editor.gutter_dimensions.right_padding;
            let hitbox = &position_map.gutter_hitbox;

            if event.position.x <= hitbox.bounds.right() - gutter_right_padding {
                let point_for_position = position_map.point_for_position(event.position);
                editor.set_breakpoint_context_menu(
                    point_for_position.previous_valid.row(),
                    None,
                    event.position,
                    window,
                    cx,
                );
            }
            return;
        }

        if !position_map.text_hitbox.is_hovered(window) {
            return;
        }

        let point_for_position = position_map.point_for_position(event.position);
        mouse_context_menu::deploy_context_menu(
            editor,
            Some(event.position),
            point_for_position.previous_valid,
            window,
            cx,
        );
        cx.stop_propagation();
    }

    fn mouse_middle_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !position_map.text_hitbox.is_hovered(window) || window.default_prevented() {
            return;
        }

        let point_for_position = position_map.point_for_position(event.position);
        let position = point_for_position.previous_valid;

        editor.select(
            SelectPhase::BeginColumnar {
                position,
                reset: true,
                goal_column: point_for_position.exact_unclipped.column(),
            },
            window,
            cx,
        );
    }

    fn mouse_up(
        editor: &mut Editor,
        event: &MouseUpEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let text_hitbox = &position_map.text_hitbox;
        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        if end_selection {
            editor.select(SelectPhase::End, window, cx);
        }

        if end_selection && pending_nonempty_selections {
            cx.stop_propagation();
        } else if cfg!(any(target_os = "linux", target_os = "freebsd"))
            && event.button == MouseButton::Middle
        {
            if !text_hitbox.is_hovered(window) || editor.read_only(cx) {
                return;
            }

            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            if EditorSettings::get_global(cx).middle_click_paste {
                if let Some(text) = cx.read_from_primary().and_then(|item| item.text()) {
                    let point_for_position = position_map.point_for_position(event.position);
                    let position = point_for_position.previous_valid;

                    editor.select(
                        SelectPhase::Begin {
                            position,
                            add: false,
                            click_count: 1,
                        },
                        window,
                        cx,
                    );
                    editor.insert(&text, window, cx);
                }
                cx.stop_propagation()
            }
        }
    }

    fn click(
        editor: &mut Editor,
        event: &ClickEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let text_hitbox = &position_map.text_hitbox;
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        let multi_cursor_setting = EditorSettings::get_global(cx).multi_cursor_modifier;
        let multi_cursor_modifier = match multi_cursor_setting {
            MultiCursorModifier::Alt => event.modifiers().secondary(),
            MultiCursorModifier::CmdOrCtrl => event.modifiers().alt,
        };

        if !pending_nonempty_selections && multi_cursor_modifier && text_hitbox.is_hovered(window) {
            let point = position_map.point_for_position(event.up.position);
            editor.handle_click_hovered_link(point, event.modifiers(), window, cx);

            cx.stop_propagation();
        }
    }

    fn mouse_dragged(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !editor.has_pending_selection() {
            return;
        }

        let text_bounds = position_map.text_hitbox.bounds;
        let point_for_position = position_map.point_for_position(event.position);
        let mut scroll_delta = gpui::Point::<f32>::default();
        let vertical_margin = position_map.line_height.min(text_bounds.size.height / 3.0);
        let top = text_bounds.origin.y + vertical_margin;
        let bottom = text_bounds.bottom_left().y - vertical_margin;
        if event.position.y < top {
            scroll_delta.y = -scale_vertical_mouse_autoscroll_delta(top - event.position.y);
        }
        if event.position.y > bottom {
            scroll_delta.y = scale_vertical_mouse_autoscroll_delta(event.position.y - bottom);
        }

        // We need horizontal width of text
        let style = editor.style.clone().unwrap_or_default();
        let font_id = window.text_system().resolve_font(&style.text.font());
        let font_size = style.text.font_size.to_pixels(window.rem_size());
        let em_width = window.text_system().em_width(font_id, font_size).unwrap();

        let scroll_margin_x = EditorSettings::get_global(cx).horizontal_scroll_margin;

        let scroll_space: Pixels = scroll_margin_x * em_width;

        let left = text_bounds.origin.x + scroll_space;
        let right = text_bounds.top_right().x - scroll_space;

        if event.position.x < left {
            scroll_delta.x = -scale_horizontal_mouse_autoscroll_delta(left - event.position.x);
        }
        if event.position.x > right {
            scroll_delta.x = scale_horizontal_mouse_autoscroll_delta(event.position.x - right);
        }

        editor.select(
            SelectPhase::Update {
                position: point_for_position.previous_valid,
                goal_column: point_for_position.exact_unclipped.column(),
                scroll_delta,
            },
            window,
            cx,
        );
    }

    fn mouse_moved(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let text_hitbox = &position_map.text_hitbox;
        let gutter_hitbox = &position_map.gutter_hitbox;
        let modifiers = event.modifiers;
        let gutter_hovered = gutter_hitbox.is_hovered(window);
        editor.set_gutter_hovered(gutter_hovered, cx);
        editor.mouse_cursor_hidden = false;

        if gutter_hovered {
            let new_point = position_map
                .point_for_position(event.position)
                .previous_valid;
            let buffer_anchor = position_map
                .snapshot
                .display_point_to_anchor(new_point, Bias::Left);

            if let Some((buffer_snapshot, file)) = position_map
                .snapshot
                .buffer_snapshot
                .buffer_for_excerpt(buffer_anchor.excerpt_id)
                .and_then(|buffer| buffer.file().map(|file| (buffer, file)))
            {
                let was_hovered = editor.gutter_breakpoint_indicator.0.is_some();
                let as_point = text::ToPoint::to_point(&buffer_anchor.text_anchor, buffer_snapshot);
                let is_visible = editor
                    .gutter_breakpoint_indicator
                    .0
                    .map_or(false, |indicator| indicator.is_active);

                let has_existing_breakpoint =
                    editor.breakpoint_store.as_ref().map_or(false, |store| {
                        store
                            .read(cx)
                            .breakpoint_at_row(&file.full_path(cx), as_point.row, cx)
                            .is_some()
                    });

                editor.gutter_breakpoint_indicator.0 = Some(PhantomBreakpointIndicator {
                    display_row: new_point.row(),
                    is_active: is_visible,
                    collides_with_existing_breakpoint: has_existing_breakpoint,
                });

                editor.gutter_breakpoint_indicator.1.get_or_insert_with(|| {
                    cx.spawn(async move |this, cx| {
                        if !was_hovered {
                            cx.background_executor()
                                .timer(Duration::from_millis(200))
                                .await;
                        }

                        this.update(cx, |this, cx| {
                            if let Some(indicator) = this.gutter_breakpoint_indicator.0.as_mut() {
                                indicator.is_active = true;
                            }

                            cx.notify();
                        })
                        .ok();
                    })
                });
            } else {
                editor.gutter_breakpoint_indicator = (None, None);
            }
        } else {
            editor.gutter_breakpoint_indicator = (None, None);
        }

        cx.notify();

        // Don't trigger hover popover if mouse is hovering over context menu
        if text_hitbox.is_hovered(window) {
            let point_for_position = position_map.point_for_position(event.position);

            editor.update_hovered_link(
                point_for_position,
                &position_map.snapshot,
                modifiers,
                window,
                cx,
            );

            if let Some(point) = point_for_position.as_valid() {
                let anchor = position_map
                    .snapshot
                    .buffer_snapshot
                    .anchor_before(point.to_offset(&position_map.snapshot, Bias::Left));
                hover_at(editor, Some(anchor), window, cx);
                Self::update_visible_cursor(editor, point, position_map, window, cx);
            } else {
                hover_at(editor, None, window, cx);
            }
        } else {
            editor.hide_hovered_link(cx);
            hover_at(editor, None, window, cx);
        }
    }

    fn update_visible_cursor(
        editor: &mut Editor,
        point: DisplayPoint,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let snapshot = &position_map.snapshot;
        let Some(hub) = editor.collaboration_hub() else {
            return;
        };
        let start = snapshot.display_snapshot.clip_point(
            DisplayPoint::new(point.row(), point.column().saturating_sub(1)),
            Bias::Left,
        );
        let end = snapshot.display_snapshot.clip_point(
            DisplayPoint::new(
                point.row(),
                (point.column() + 1).min(snapshot.line_len(point.row())),
            ),
            Bias::Right,
        );

        let range = snapshot
            .buffer_snapshot
            .anchor_at(start.to_point(&snapshot.display_snapshot), Bias::Left)
            ..snapshot
                .buffer_snapshot
                .anchor_at(end.to_point(&snapshot.display_snapshot), Bias::Right);

        let Some(selection) = snapshot.remote_selections_in_range(&range, hub, cx).next() else {
            return;
        };
        let key = crate::HoveredCursor {
            replica_id: selection.replica_id,
            selection_id: selection.selection.id,
        };
        editor.hovered_cursors.insert(
            key.clone(),
            cx.spawn_in(window, async move |editor, cx| {
                cx.background_executor().timer(CURSORS_VISIBLE_FOR).await;
                editor
                    .update(cx, |editor, cx| {
                        editor.hovered_cursors.remove(&key);
                        cx.notify();
                    })
                    .ok();
            }),
        );
        cx.notify()
    }

    fn layout_selections(
        &self,
        start_anchor: Anchor,
        end_anchor: Anchor,
        local_selections: &[Selection<Point>],
        snapshot: &EditorSnapshot,
        start_row: DisplayRow,
        end_row: DisplayRow,
        window: &mut Window,
        cx: &mut App,
    ) -> (
        Vec<(PlayerColor, Vec<SelectionLayout>)>,
        BTreeMap<DisplayRow, LineHighlightSpec>,
        Option<DisplayPoint>,
    ) {
        let mut selections: Vec<(PlayerColor, Vec<SelectionLayout>)> = Vec::new();
        let mut active_rows = BTreeMap::new();
        let mut newest_selection_head = None;
        self.editor.update(cx, |editor, cx| {
            if editor.show_local_selections {
                let mut layouts = Vec::new();
                let newest = editor.selections.newest(cx);
                for selection in local_selections.iter().cloned() {
                    let is_empty = selection.start == selection.end;
                    let is_newest = selection == newest;

                    let layout = SelectionLayout::new(
                        selection,
                        editor.selections.line_mode,
                        editor.cursor_shape,
                        &snapshot.display_snapshot,
                        is_newest,
                        editor.leader_peer_id.is_none(),
                        None,
                    );
                    if is_newest {
                        newest_selection_head = Some(layout.head);
                    }

                    for row in cmp::max(layout.active_rows.start.0, start_row.0)
                        ..=cmp::min(layout.active_rows.end.0, end_row.0)
                    {
                        let contains_non_empty_selection = active_rows
                            .entry(DisplayRow(row))
                            .or_insert_with(LineHighlightSpec::default);
                        contains_non_empty_selection.selection |= !is_empty;
                    }
                    layouts.push(layout);
                }

                let player = editor.current_user_player_color(cx);
                selections.push((player, layouts));
            }

            if let Some(collaboration_hub) = &editor.collaboration_hub {
                // When following someone, render the local selections in their color.
                if let Some(leader_id) = editor.leader_peer_id {
                    if let Some(collaborator) = collaboration_hub.collaborators(cx).get(&leader_id)
                    {
                        if let Some(participant_index) = collaboration_hub
                            .user_participant_indices(cx)
                            .get(&collaborator.user_id)
                        {
                            if let Some((local_selection_style, _)) = selections.first_mut() {
                                *local_selection_style = cx
                                    .theme()
                                    .players()
                                    .color_for_participant(participant_index.0);
                            }
                        }
                    }
                }

                let mut remote_selections = HashMap::default();
                for selection in snapshot.remote_selections_in_range(
                    &(start_anchor..end_anchor),
                    collaboration_hub.as_ref(),
                    cx,
                ) {
                    let selection_style =
                        Self::get_participant_color(selection.participant_index, cx);

                    // Don't re-render the leader's selections, since the local selections
                    // match theirs.
                    if Some(selection.peer_id) == editor.leader_peer_id {
                        continue;
                    }
                    let key = HoveredCursor {
                        replica_id: selection.replica_id,
                        selection_id: selection.selection.id,
                    };

                    let is_shown =
                        editor.show_cursor_names || editor.hovered_cursors.contains_key(&key);

                    remote_selections
                        .entry(selection.replica_id)
                        .or_insert((selection_style, Vec::new()))
                        .1
                        .push(SelectionLayout::new(
                            selection.selection,
                            selection.line_mode,
                            selection.cursor_shape,
                            &snapshot.display_snapshot,
                            false,
                            false,
                            if is_shown { selection.user_name } else { None },
                        ));
                }

                selections.extend(remote_selections.into_values());
            } else if !editor.is_focused(window) && editor.show_cursor_when_unfocused {
                let layouts = snapshot
                    .buffer_snapshot
                    .selections_in_range(&(start_anchor..end_anchor), true)
                    .map(move |(_, line_mode, cursor_shape, selection)| {
                        SelectionLayout::new(
                            selection,
                            line_mode,
                            cursor_shape,
                            &snapshot.display_snapshot,
                            false,
                            false,
                            None,
                        )
                    })
                    .collect::<Vec<_>>();
                let player = editor.current_user_player_color(cx);
                selections.push((player, layouts));
            }
        });
        (selections, active_rows, newest_selection_head)
    }

    fn collect_cursors(
        &self,
        snapshot: &EditorSnapshot,
        cx: &mut App,
    ) -> Vec<(DisplayPoint, Hsla)> {
        let editor = self.editor.read(cx);
        let mut cursors = Vec::new();
        let mut skip_local = false;
        let mut add_cursor = |anchor: Anchor, color| {
            cursors.push((anchor.to_display_point(&snapshot.display_snapshot), color));
        };
        // Remote cursors
        if let Some(collaboration_hub) = &editor.collaboration_hub {
            for remote_selection in snapshot.remote_selections_in_range(
                &(Anchor::min()..Anchor::max()),
                collaboration_hub.deref(),
                cx,
            ) {
                let color = Self::get_participant_color(remote_selection.participant_index, cx);
                add_cursor(remote_selection.selection.head(), color.cursor);
                if Some(remote_selection.peer_id) == editor.leader_peer_id {
                    skip_local = true;
                }
            }
        }
        // Local cursors
        if !skip_local {
            let color = cx.theme().players().local().cursor;
            editor.selections.disjoint.iter().for_each(|selection| {
                add_cursor(selection.head(), color);
            });
            if let Some(ref selection) = editor.selections.pending_anchor() {
                add_cursor(selection.head(), color);
            }
        }
        cursors
    }

    fn layout_visible_cursors(
        &self,
        snapshot: &EditorSnapshot,
        selections: &[(PlayerColor, Vec<SelectionLayout>)],
        row_block_types: &HashMap<DisplayRow, bool>,
        visible_display_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_position: gpui::Point<f32>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        em_width: Pixels,
        em_advance: Pixels,
        autoscroll_containing_element: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<CursorLayout> {
        let mut autoscroll_bounds = None;
        let cursor_layouts = self.editor.update(cx, |editor, cx| {
            let mut cursors = Vec::new();

            let show_local_cursors = editor.show_local_cursors(window, cx);

            for (player_color, selections) in selections {
                for selection in selections {
                    let cursor_position = selection.head;

                    let in_range = visible_display_row_range.contains(&cursor_position.row());
                    if (selection.is_local && !show_local_cursors)
                        || !in_range
                        || row_block_types.get(&cursor_position.row()) == Some(&true)
                    {
                        continue;
                    }

                    let cursor_row_layout = &line_layouts
                        [cursor_position.row().minus(visible_display_row_range.start) as usize];
                    let cursor_column = cursor_position.column() as usize;

                    let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);
                    let mut block_width =
                        cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
                    if block_width == Pixels::ZERO {
                        block_width = em_advance;
                    }
                    let block_text = if let CursorShape::Block = selection.cursor_shape {
                        snapshot
                            .grapheme_at(cursor_position)
                            .or_else(|| {
                                if cursor_column == 0 {
                                    snapshot.placeholder_text().and_then(|s| {
                                        s.graphemes(true).next().map(|s| s.to_string().into())
                                    })
                                } else {
                                    None
                                }
                            })
                            .and_then(|text| {
                                let len = text.len();

                                let font = cursor_row_layout
                                    .font_id_for_index(cursor_column)
                                    .and_then(|cursor_font_id| {
                                        window.text_system().get_font_for_id(cursor_font_id)
                                    })
                                    .unwrap_or(self.style.text.font());

                                // Invert the text color for the block cursor. Ensure that the text
                                // color is opaque enough to be visible against the background color.
                                //
                                // 0.75 is an arbitrary threshold to determine if the background color is
                                // opaque enough to use as a text color.
                                //
                                // TODO: In the future we should ensure themes have a `text_inverse` color.
                                let color = if cx.theme().colors().editor_background.a < 0.75 {
                                    match cx.theme().appearance {
                                        Appearance::Dark => Hsla::black(),
                                        Appearance::Light => Hsla::white(),
                                    }
                                } else {
                                    cx.theme().colors().editor_background
                                };

                                window
                                    .text_system()
                                    .shape_line(
                                        text,
                                        cursor_row_layout.font_size,
                                        &[TextRun {
                                            len,
                                            font,
                                            color,
                                            background_color: None,
                                            strikethrough: None,
                                            underline: None,
                                        }],
                                    )
                                    .log_err()
                            })
                    } else {
                        None
                    };

                    let x = cursor_character_x - scroll_pixel_position.x;
                    let y = (cursor_position.row().as_f32()
                        - scroll_pixel_position.y / line_height)
                        * line_height;
                    if selection.is_newest {
                        editor.pixel_position_of_newest_cursor = Some(point(
                            text_hitbox.origin.x + x + block_width / 2.,
                            text_hitbox.origin.y + y + line_height / 2.,
                        ));

                        if autoscroll_containing_element {
                            let top = text_hitbox.origin.y
                                + (cursor_position.row().as_f32() - scroll_position.y - 3.).max(0.)
                                    * line_height;
                            let left = text_hitbox.origin.x
                                + (cursor_position.column() as f32 - scroll_position.x - 3.)
                                    .max(0.)
                                    * em_width;

                            let bottom = text_hitbox.origin.y
                                + (cursor_position.row().as_f32() - scroll_position.y + 4.)
                                    * line_height;
                            let right = text_hitbox.origin.x
                                + (cursor_position.column() as f32 - scroll_position.x + 4.)
                                    * em_width;

                            autoscroll_bounds =
                                Some(Bounds::from_corners(point(left, top), point(right, bottom)))
                        }
                    }

                    let mut cursor = CursorLayout {
                        color: player_color.cursor,
                        block_width,
                        origin: point(x, y),
                        line_height,
                        shape: selection.cursor_shape,
                        block_text,
                        cursor_name: None,
                    };
                    let cursor_name = selection.user_name.clone().map(|name| CursorName {
                        string: name,
                        color: self.style.background,
                        is_top_row: cursor_position.row().0 == 0,
                    });
                    cursor.layout(content_origin, cursor_name, window, cx);
                    cursors.push(cursor);
                }
            }

            cursors
        });

        if let Some(bounds) = autoscroll_bounds {
            window.request_autoscroll(bounds);
        }

        cursor_layouts
    }

    fn layout_scrollbars(
        &self,
        snapshot: &EditorSnapshot,
        scrollbar_layout_information: ScrollbarLayoutInformation,
        content_offset: gpui::Point<Pixels>,
        scroll_position: gpui::Point<f32>,
        non_visible_cursors: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<EditorScrollbars> {
        if !snapshot.mode.is_full() {
            return None;
        }

        // If a drag took place after we started dragging the scrollbar,
        // cancel the scrollbar drag.
        if cx.has_active_drag() {
            self.editor.update(cx, |editor, cx| {
                editor.scroll_manager.reset_scrollbar_dragging_state(cx)
            });
        }

        let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
        let show_scrollbars = self.editor.read(cx).show_scrollbars
            && match scrollbar_settings.show {
                ShowScrollbar::Auto => {
                    let editor = self.editor.read(cx);
                    let is_singleton = editor.is_singleton(cx);
                    // Git
                    (is_singleton && scrollbar_settings.git_diff && snapshot.buffer_snapshot.has_diff_hunks())
                    ||
                    // Buffer Search Results
                    (is_singleton && scrollbar_settings.search_results && editor.has_background_highlights::<BufferSearchHighlights>())
                    ||
                    // Selected Text Occurrences
                    (is_singleton && scrollbar_settings.selected_text && editor.has_background_highlights::<SelectedTextHighlight>())
                    ||
                    // Selected Symbol Occurrences
                    (is_singleton && scrollbar_settings.selected_symbol && (editor.has_background_highlights::<DocumentHighlightRead>() || editor.has_background_highlights::<DocumentHighlightWrite>()))
                    ||
                    // Diagnostics
                    (is_singleton && scrollbar_settings.diagnostics != ScrollbarDiagnostics::None && snapshot.buffer_snapshot.has_diagnostics())
                    ||
                    // Cursors out of sight
                    non_visible_cursors
                    ||
                    // Scrollmanager
                    editor.scroll_manager.scrollbars_visible()
                }
                ShowScrollbar::System => self.editor.read(cx).scroll_manager.scrollbars_visible(),
                ShowScrollbar::Always => true,
                ShowScrollbar::Never => return None,
            };

        Some(EditorScrollbars::from_scrollbar_axes(
            scrollbar_settings.axes,
            &scrollbar_layout_information,
            content_offset,
            scroll_position,
            self.style.scrollbar_width,
            show_scrollbars,
            window,
        ))
    }

    fn prepaint_crease_toggles(
        &self,
        crease_toggles: &mut [Option<AnyElement>],
        line_height: Pixels,
        gutter_dimensions: &GutterDimensions,
        gutter_settings: crate::editor_settings::Gutter,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_hitbox: &Hitbox,
        window: &mut Window,
        cx: &mut App,
    ) {
        for (ix, crease_toggle) in crease_toggles.iter_mut().enumerate() {
            if let Some(crease_toggle) = crease_toggle {
                debug_assert!(gutter_settings.folds);
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height * 0.55),
                );
                let crease_toggle_size = crease_toggle.layout_as_root(available_space, window, cx);

                let position = point(
                    gutter_dimensions.width - gutter_dimensions.right_padding,
                    ix as f32 * line_height - (scroll_pixel_position.y % line_height),
                );
                let centering_offset = point(
                    (gutter_dimensions.fold_area_width() - crease_toggle_size.width) / 2.,
                    (line_height - crease_toggle_size.height) / 2.,
                );
                let origin = gutter_hitbox.origin + position + centering_offset;
                crease_toggle.prepaint_as_root(origin, available_space, window, cx);
            }
        }
    }

    fn prepaint_expand_toggles(
        &self,
        expand_toggles: &mut [Option<(AnyElement, gpui::Point<Pixels>)>],
        window: &mut Window,
        cx: &mut App,
    ) {
        for (expand_toggle, origin) in expand_toggles.iter_mut().flatten() {
            let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
            expand_toggle.layout_as_root(available_space, window, cx);
            expand_toggle.prepaint_as_root(*origin, available_space, window, cx);
        }
    }

    fn prepaint_crease_trailers(
        &self,
        trailers: Vec<Option<AnyElement>>,
        lines: &[LineWithInvisibles],
        line_height: Pixels,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        em_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Option<CreaseTrailerLayout>> {
        trailers
            .into_iter()
            .enumerate()
            .map(|(ix, element)| {
                let mut element = element?;
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height),
                );
                let size = element.layout_as_root(available_space, window, cx);

                let line = &lines[ix];
                let padding = if line.width == Pixels::ZERO {
                    Pixels::ZERO
                } else {
                    4. * em_width
                };
                let position = point(
                    scroll_pixel_position.x + line.width + padding,
                    ix as f32 * line_height - (scroll_pixel_position.y % line_height),
                );
                let centering_offset = point(px(0.), (line_height - size.height) / 2.);
                let origin = content_origin + position + centering_offset;
                element.prepaint_as_root(origin, available_space, window, cx);
                Some(CreaseTrailerLayout {
                    element,
                    bounds: Bounds::new(origin, size),
                })
            })
            .collect()
    }

    // Folds contained in a hunk are ignored apart from shrinking visual size
    // If a fold contains any hunks then that fold line is marked as modified
    fn layout_gutter_diff_hunks(
        &self,
        line_height: Pixels,
        gutter_hitbox: &Hitbox,
        display_rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<(DisplayDiffHunk, Option<Hitbox>)> {
        let folded_buffers = self.editor.read(cx).folded_buffers(cx);
        let mut display_hunks = snapshot
            .display_diff_hunks_for_rows(display_rows, folded_buffers)
            .map(|hunk| (hunk, None))
            .collect::<Vec<_>>();
        let git_gutter_setting = ProjectSettings::get_global(cx)
            .git
            .git_gutter
            .unwrap_or_default();
        if let GitGutterSetting::TrackedFiles = git_gutter_setting {
            for (hunk, hitbox) in &mut display_hunks {
                if matches!(hunk, DisplayDiffHunk::Unfolded { .. }) {
                    let hunk_bounds =
                        Self::diff_hunk_bounds(snapshot, line_height, gutter_hitbox.bounds, hunk);
                    *hitbox = Some(window.insert_hitbox(hunk_bounds, true));
                }
            }
        }

        display_hunks
    }

    fn layout_inline_diagnostics(
        &self,
        line_layouts: &[LineWithInvisibles],
        crease_trailers: &[Option<CreaseTrailerLayout>],
        row_block_types: &HashMap<DisplayRow, bool>,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        inline_completion_popover_origin: Option<gpui::Point<Pixels>>,
        start_row: DisplayRow,
        end_row: DisplayRow,
        line_height: Pixels,
        em_width: Pixels,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> HashMap<DisplayRow, AnyElement> {
        let max_severity = ProjectSettings::get_global(cx)
            .diagnostics
            .inline
            .max_severity
            .map_or(DiagnosticSeverity::HINT, |severity| match severity {
                project_settings::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                project_settings::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
                project_settings::DiagnosticSeverity::Info => DiagnosticSeverity::INFORMATION,
                project_settings::DiagnosticSeverity::Hint => DiagnosticSeverity::HINT,
            });

        let active_diagnostics_group =
            if let ActiveDiagnostic::Group(group) = &self.editor.read(cx).active_diagnostics {
                Some(group.group_id)
            } else {
                None
            };

        let diagnostics_by_rows = self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            editor
                .inline_diagnostics
                .iter()
                .filter(|(_, diagnostic)| diagnostic.severity <= max_severity)
                .filter(|(_, diagnostic)| match active_diagnostics_group {
                    Some(active_diagnostics_group) => {
                        // Active diagnostics are all shown in the editor already, no need to display them inline
                        diagnostic.group_id != active_diagnostics_group
                    }
                    None => true,
                })
                .map(|(point, diag)| (point.to_display_point(&snapshot), diag.clone()))
                .skip_while(|(point, _)| point.row() < start_row)
                .take_while(|(point, _)| point.row() < end_row)
                .filter(|(point, _)| !row_block_types.contains_key(&point.row()))
                .fold(HashMap::default(), |mut acc, (point, diagnostic)| {
                    acc.entry(point.row())
                        .or_insert_with(Vec::new)
                        .push(diagnostic);
                    acc
                })
        });

        if diagnostics_by_rows.is_empty() {
            return HashMap::default();
        }

        let severity_to_color = |sev: &DiagnosticSeverity| match sev {
            &DiagnosticSeverity::ERROR => Color::Error,
            &DiagnosticSeverity::WARNING => Color::Warning,
            &DiagnosticSeverity::INFORMATION => Color::Info,
            &DiagnosticSeverity::HINT => Color::Hint,
            _ => Color::Error,
        };

        let padding = ProjectSettings::get_global(cx).diagnostics.inline.padding as f32 * em_width;
        let min_x = ProjectSettings::get_global(cx)
            .diagnostics
            .inline
            .min_column as f32
            * em_width;

        let mut elements = HashMap::default();
        for (row, mut diagnostics) in diagnostics_by_rows {
            diagnostics.sort_by_key(|diagnostic| {
                (
                    diagnostic.severity,
                    std::cmp::Reverse(diagnostic.is_primary),
                    diagnostic.start.row,
                    diagnostic.start.column,
                )
            });

            let Some(diagnostic_to_render) = diagnostics
                .iter()
                .find(|diagnostic| diagnostic.is_primary)
                .or_else(|| diagnostics.first())
            else {
                continue;
            };

            let pos_y = content_origin.y
                + line_height * (row.0 as f32 - scroll_pixel_position.y / line_height);

            let window_ix = row.0.saturating_sub(start_row.0) as usize;
            let pos_x = {
                let crease_trailer_layout = &crease_trailers[window_ix];
                let line_layout = &line_layouts[window_ix];

                let line_end = if let Some(crease_trailer) = crease_trailer_layout {
                    crease_trailer.bounds.right()
                } else {
                    content_origin.x - scroll_pixel_position.x + line_layout.width
                };

                let padded_line = line_end + padding;
                let min_start = content_origin.x - scroll_pixel_position.x + min_x;

                cmp::max(padded_line, min_start)
            };

            let behind_inline_completion_popover = inline_completion_popover_origin
                .as_ref()
                .map_or(false, |inline_completion_popover_origin| {
                    (pos_y..pos_y + line_height).contains(&inline_completion_popover_origin.y)
                });
            let opacity = if behind_inline_completion_popover {
                0.5
            } else {
                1.0
            };

            let mut element = h_flex()
                .id(("diagnostic", row.0))
                .h(line_height)
                .w_full()
                .px_1()
                .rounded_xs()
                .opacity(opacity)
                .bg(severity_to_color(&diagnostic_to_render.severity)
                    .color(cx)
                    .opacity(0.05))
                .text_color(severity_to_color(&diagnostic_to_render.severity).color(cx))
                .text_sm()
                .font_family(style.text.font().family)
                .child(diagnostic_to_render.message.clone())
                .into_any();

            element.prepaint_as_root(point(pos_x, pos_y), AvailableSpace::min_size(), window, cx);

            elements.insert(row, element);
        }

        elements
    }

    fn layout_inline_blame(
        &self,
        display_row: DisplayRow,
        row_info: &RowInfo,
        line_layout: &LineWithInvisibles,
        crease_trailer: Option<&CreaseTrailerLayout>,
        em_width: Pixels,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        if !self
            .editor
            .update(cx, |editor, cx| editor.render_git_blame_inline(window, cx))
        {
            return None;
        }

        let editor = self.editor.read(cx);
        let blame = editor.blame.clone()?;
        let padding = {
            const INLINE_BLAME_PADDING_EM_WIDTHS: f32 = 6.;
            const INLINE_ACCEPT_SUGGESTION_EM_WIDTHS: f32 = 14.;

            let mut padding = INLINE_BLAME_PADDING_EM_WIDTHS;

            if let Some(inline_completion) = editor.active_inline_completion.as_ref() {
                match &inline_completion.completion {
                    InlineCompletion::Edit {
                        display_mode: EditDisplayMode::TabAccept,
                        ..
                    } => padding += INLINE_ACCEPT_SUGGESTION_EM_WIDTHS,
                    _ => {}
                }
            }

            padding * em_width
        };

        let blame_entry = blame
            .update(cx, |blame, cx| {
                blame.blame_for_rows(&[*row_info], cx).next()
            })
            .flatten()?;

        let mut element = render_inline_blame_entry(blame_entry.clone(), &self.style, cx)?;

        let start_y = content_origin.y
            + line_height * (display_row.as_f32() - scroll_pixel_position.y / line_height);

        let start_x = {
            let line_end = if let Some(crease_trailer) = crease_trailer {
                crease_trailer.bounds.right()
            } else {
                content_origin.x - scroll_pixel_position.x + line_layout.width
            };

            let padded_line_end = line_end + padding;

            let min_column_in_pixels = ProjectSettings::get_global(cx)
                .git
                .inline_blame
                .and_then(|settings| settings.min_column)
                .map(|col| self.column_pixels(col as usize, window, cx))
                .unwrap_or(px(0.));
            let min_start = content_origin.x - scroll_pixel_position.x + min_column_in_pixels;

            cmp::max(padded_line_end, min_start)
        };

        let absolute_offset = point(start_x, start_y);
        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
        let bounds = Bounds::new(absolute_offset, size);

        self.layout_blame_entry_popover(
            bounds,
            blame_entry,
            blame,
            line_height,
            text_hitbox,
            window,
            cx,
        );

        element.prepaint_as_root(absolute_offset, AvailableSpace::min_size(), window, cx);

        Some(element)
    }

    fn layout_blame_entry_popover(
        &self,
        parent_bounds: Bounds<Pixels>,
        blame_entry: BlameEntry,
        blame: Entity<GitBlame>,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mouse_position = window.mouse_position();
        let mouse_over_inline_blame = parent_bounds.contains(&mouse_position);
        let mouse_over_popover = self.editor.update(cx, |editor, _| {
            editor
                .inline_blame_popover
                .as_ref()
                .and_then(|state| state.popover_bounds)
                .map_or(false, |bounds| bounds.contains(&mouse_position))
        });

        self.editor.update(cx, |editor, cx| {
            if mouse_over_inline_blame || mouse_over_popover {
                editor.show_blame_popover(&blame_entry, mouse_position, cx);
            } else {
                editor.hide_blame_popover(cx);
            }
        });

        let should_draw = self.editor.update(cx, |editor, _| {
            editor
                .inline_blame_popover
                .as_ref()
                .map_or(false, |state| state.show_task.is_none())
        });

        if should_draw {
            let maybe_element = self.editor.update(cx, |editor, cx| {
                editor
                    .workspace()
                    .map(|workspace| workspace.downgrade())
                    .zip(
                        editor
                            .inline_blame_popover
                            .as_ref()
                            .map(|p| p.popover_state.clone()),
                    )
                    .and_then(|(workspace, popover_state)| {
                        render_blame_entry_popover(
                            blame_entry,
                            popover_state.scroll_handle,
                            popover_state.commit_message,
                            popover_state.markdown,
                            workspace,
                            &blame,
                            window,
                            cx,
                        )
                    })
            });

            if let Some(mut element) = maybe_element {
                let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                let origin = self.editor.update(cx, |editor, _| {
                    let target_point = editor
                        .inline_blame_popover
                        .as_ref()
                        .map_or(mouse_position, |state| state.position);

                    let overall_height = size.height + HOVER_POPOVER_GAP;
                    let popover_origin = if target_point.y > overall_height {
                        point(target_point.x, target_point.y - size.height)
                    } else {
                        point(
                            target_point.x,
                            target_point.y + line_height + HOVER_POPOVER_GAP,
                        )
                    };

                    let horizontal_offset = (text_hitbox.top_right().x
                        - POPOVER_RIGHT_OFFSET
                        - (popover_origin.x + size.width))
                        .min(Pixels::ZERO);

                    point(popover_origin.x + horizontal_offset, popover_origin.y)
                });

                let popover_bounds = Bounds::new(origin, size);
                self.editor.update(cx, |editor, _| {
                    if let Some(state) = &mut editor.inline_blame_popover {
                        state.popover_bounds = Some(popover_bounds);
                    }
                });

                window.defer_draw(element, origin, 2);
            }
        }
    }

    fn layout_blame_entries(
        &self,
        buffer_rows: &[RowInfo],
        em_width: Pixels,
        scroll_position: gpui::Point<f32>,
        line_height: Pixels,
        gutter_hitbox: &Hitbox,
        max_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Vec<AnyElement>> {
        if !self
            .editor
            .update(cx, |editor, cx| editor.render_git_blame_gutter(cx))
        {
            return None;
        }

        let blame = self.editor.read(cx).blame.clone()?;
        let workspace = self.editor.read(cx).workspace()?;
        let blamed_rows: Vec<_> = blame.update(cx, |blame, cx| {
            blame.blame_for_rows(buffer_rows, cx).collect()
        });

        let width = if let Some(max_width) = max_width {
            AvailableSpace::Definite(max_width)
        } else {
            AvailableSpace::MaxContent
        };
        let scroll_top = scroll_position.y * line_height;
        let start_x = em_width;

        let mut last_used_color: Option<(PlayerColor, Oid)> = None;
        let blame_renderer = cx.global::<GlobalBlameRenderer>().0.clone();

        let shaped_lines = blamed_rows
            .into_iter()
            .enumerate()
            .flat_map(|(ix, blame_entry)| {
                let mut element = render_blame_entry(
                    ix,
                    &blame,
                    blame_entry?,
                    &self.style,
                    &mut last_used_color,
                    self.editor.clone(),
                    workspace.clone(),
                    blame_renderer.clone(),
                    cx,
                )?;

                let start_y = ix as f32 * line_height - (scroll_top % line_height);
                let absolute_offset = gutter_hitbox.origin + point(start_x, start_y);

                element.prepaint_as_root(
                    absolute_offset,
                    size(width, AvailableSpace::MinContent),
                    window,
                    cx,
                );

                Some(element)
            })
            .collect();

        Some(shaped_lines)
    }

    fn layout_indent_guides(
        &self,
        content_origin: gpui::Point<Pixels>,
        text_origin: gpui::Point<Pixels>,
        visible_buffer_range: Range<MultiBufferRow>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        snapshot: &DisplaySnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Vec<IndentGuideLayout>> {
        let indent_guides = self.editor.update(cx, |editor, cx| {
            editor.indent_guides(visible_buffer_range, snapshot, cx)
        })?;

        let active_indent_guide_indices = self.editor.update(cx, |editor, cx| {
            editor
                .find_active_indent_guide_indices(&indent_guides, snapshot, window, cx)
                .unwrap_or_default()
        });

        Some(
            indent_guides
                .into_iter()
                .enumerate()
                .filter_map(|(i, indent_guide)| {
                    let single_indent_width =
                        self.column_pixels(indent_guide.tab_size as usize, window, cx);
                    let total_width = single_indent_width * indent_guide.depth as f32;
                    let start_x = content_origin.x + total_width - scroll_pixel_position.x;
                    if start_x >= text_origin.x {
                        let (offset_y, length) = Self::calculate_indent_guide_bounds(
                            indent_guide.start_row..indent_guide.end_row,
                            line_height,
                            snapshot,
                        );

                        let start_y = content_origin.y + offset_y - scroll_pixel_position.y;

                        Some(IndentGuideLayout {
                            origin: point(start_x, start_y),
                            length,
                            single_indent_width,
                            depth: indent_guide.depth,
                            active: active_indent_guide_indices.contains(&i),
                            settings: indent_guide.settings,
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    fn calculate_indent_guide_bounds(
        row_range: Range<MultiBufferRow>,
        line_height: Pixels,
        snapshot: &DisplaySnapshot,
    ) -> (gpui::Pixels, gpui::Pixels) {
        let start_point = Point::new(row_range.start.0, 0);
        let end_point = Point::new(row_range.end.0, 0);

        let row_range = start_point.to_display_point(snapshot).row()
            ..end_point.to_display_point(snapshot).row();

        let mut prev_line = start_point;
        prev_line.row = prev_line.row.saturating_sub(1);
        let prev_line = prev_line.to_display_point(snapshot).row();

        let mut cons_line = end_point;
        cons_line.row += 1;
        let cons_line = cons_line.to_display_point(snapshot).row();

        let mut offset_y = row_range.start.0 as f32 * line_height;
        let mut length = (cons_line.0.saturating_sub(row_range.start.0)) as f32 * line_height;

        // If we are at the end of the buffer, ensure that the indent guide extends to the end of the line.
        if row_range.end == cons_line {
            length += line_height;
        }

        // If there is a block (e.g. diagnostic) in between the start of the indent guide and the line above,
        // we want to extend the indent guide to the start of the block.
        let mut block_height = 0;
        let mut block_offset = 0;
        let mut found_excerpt_header = false;
        for (_, block) in snapshot.blocks_in_range(prev_line..row_range.start) {
            if matches!(block, Block::ExcerptBoundary { .. }) {
                found_excerpt_header = true;
                break;
            }
            block_offset += block.height();
            block_height += block.height();
        }
        if !found_excerpt_header {
            offset_y -= block_offset as f32 * line_height;
            length += block_height as f32 * line_height;
        }

        // If there is a block (e.g. diagnostic) at the end of an multibuffer excerpt,
        // we want to ensure that the indent guide stops before the excerpt header.
        let mut block_height = 0;
        let mut found_excerpt_header = false;
        for (_, block) in snapshot.blocks_in_range(row_range.end..cons_line) {
            if matches!(block, Block::ExcerptBoundary { .. }) {
                found_excerpt_header = true;
            }
            block_height += block.height();
        }
        if found_excerpt_header {
            length -= block_height as f32 * line_height;
        }

        (offset_y, length)
    }

    fn layout_breakpoints(
        &self,
        line_height: Pixels,
        range: Range<DisplayRow>,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        snapshot: &EditorSnapshot,
        breakpoints: HashMap<DisplayRow, (Anchor, Breakpoint)>,
        row_infos: &[RowInfo],
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        self.editor.update(cx, |editor, cx| {
            breakpoints
                .into_iter()
                .filter_map(|(display_row, (text_anchor, bp))| {
                    if row_infos
                        .get((display_row.0.saturating_sub(range.start.0)) as usize)
                        .is_some_and(|row_info| {
                            row_info.expand_info.is_some()
                                || row_info
                                    .diff_status
                                    .is_some_and(|status| status.is_deleted())
                        })
                    {
                        return None;
                    }

                    if range.start > display_row || range.end < display_row {
                        return None;
                    }

                    let row =
                        MultiBufferRow(DisplayPoint::new(display_row, 0).to_point(&snapshot).row);
                    if snapshot.is_line_folded(row) {
                        return None;
                    }

                    let button = editor.render_breakpoint(text_anchor, display_row, &bp, cx);

                    let button = prepaint_gutter_button(
                        button,
                        display_row,
                        line_height,
                        gutter_dimensions,
                        scroll_pixel_position,
                        gutter_hitbox,
                        display_hunks,
                        window,
                        cx,
                    );
                    Some(button)
                })
                .collect_vec()
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_run_indicators(
        &self,
        line_height: Pixels,
        range: Range<DisplayRow>,
        row_infos: &[RowInfo],
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        snapshot: &EditorSnapshot,
        breakpoints: &mut HashMap<DisplayRow, (Anchor, Breakpoint)>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        self.editor.update(cx, |editor, cx| {
            let active_task_indicator_row =
                if let Some(crate::CodeContextMenu::CodeActions(CodeActionsMenu {
                    deployed_from_indicator,
                    actions,
                    ..
                })) = editor.context_menu.borrow().as_ref()
                {
                    actions
                        .tasks()
                        .map(|tasks| tasks.position.to_display_point(snapshot).row())
                        .or(*deployed_from_indicator)
                } else {
                    None
                };

            let offset_range_start =
                snapshot.display_point_to_point(DisplayPoint::new(range.start, 0), Bias::Left);

            let offset_range_end =
                snapshot.display_point_to_point(DisplayPoint::new(range.end, 0), Bias::Right);

            editor
                .tasks
                .iter()
                .filter_map(|(_, tasks)| {
                    let multibuffer_point = tasks.offset.to_point(&snapshot.buffer_snapshot);
                    if multibuffer_point < offset_range_start
                        || multibuffer_point > offset_range_end
                    {
                        return None;
                    }
                    let multibuffer_row = MultiBufferRow(multibuffer_point.row);
                    let buffer_folded = snapshot
                        .buffer_snapshot
                        .buffer_line_for_row(multibuffer_row)
                        .map(|(buffer_snapshot, _)| buffer_snapshot.remote_id())
                        .map(|buffer_id| editor.is_buffer_folded(buffer_id, cx))
                        .unwrap_or(false);
                    if buffer_folded {
                        return None;
                    }

                    if snapshot.is_line_folded(multibuffer_row) {
                        // Skip folded indicators, unless it's the starting line of a fold.
                        if multibuffer_row
                            .0
                            .checked_sub(1)
                            .map_or(false, |previous_row| {
                                snapshot.is_line_folded(MultiBufferRow(previous_row))
                            })
                        {
                            return None;
                        }
                    }

                    let display_row = multibuffer_point.to_display_point(snapshot).row();
                    if row_infos
                        .get((display_row - range.start).0 as usize)
                        .is_some_and(|row_info| row_info.expand_info.is_some())
                    {
                        return None;
                    }

                    let button = editor.render_run_indicator(
                        &self.style,
                        Some(display_row) == active_task_indicator_row,
                        display_row,
                        breakpoints.remove(&display_row),
                        cx,
                    );

                    let button = prepaint_gutter_button(
                        button,
                        display_row,
                        line_height,
                        gutter_dimensions,
                        scroll_pixel_position,
                        gutter_hitbox,
                        display_hunks,
                        window,
                        cx,
                    );
                    Some(button)
                })
                .collect_vec()
        })
    }

    fn layout_expand_toggles(
        &self,
        gutter_hitbox: &Hitbox,
        gutter_dimensions: GutterDimensions,
        em_width: Pixels,
        line_height: Pixels,
        scroll_position: gpui::Point<f32>,
        buffer_rows: &[RowInfo],
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Option<(AnyElement, gpui::Point<Pixels>)>> {
        if self.editor.read(cx).disable_expand_excerpt_buttons {
            return vec![];
        }

        let editor_font_size = self.style.text.font_size.to_pixels(window.rem_size()) * 1.2;

        let scroll_top = scroll_position.y * line_height;

        let max_line_number_length = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
            .widest_line_number()
            .ilog10()
            + 1;

        let elements = buffer_rows
            .into_iter()
            .enumerate()
            .map(|(ix, row_info)| {
                let ExpandInfo {
                    excerpt_id,
                    direction,
                } = row_info.expand_info?;

                let icon_name = match direction {
                    ExpandExcerptDirection::Up => IconName::ExpandUp,
                    ExpandExcerptDirection::Down => IconName::ExpandDown,
                    ExpandExcerptDirection::UpAndDown => IconName::ExpandVertical,
                };

                let git_gutter_width = Self::gutter_strip_width(line_height);
                let available_width = gutter_dimensions.left_padding - git_gutter_width;

                let editor = self.editor.clone();
                let is_wide = max_line_number_length >= MIN_LINE_NUMBER_DIGITS
                    && row_info
                        .buffer_row
                        .is_some_and(|row| (row + 1).ilog10() + 1 == max_line_number_length)
                    || gutter_dimensions.right_padding == px(0.);

                let width = if is_wide {
                    available_width - px(2.)
                } else {
                    available_width + em_width - px(2.)
                };

                let toggle = IconButton::new(("expand", ix), icon_name)
                    .icon_color(Color::Custom(cx.theme().colors().editor_line_number))
                    .selected_icon_color(Color::Custom(cx.theme().colors().editor_foreground))
                    .icon_size(IconSize::Custom(rems(editor_font_size / window.rem_size())))
                    .width(width.into())
                    .on_click(move |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.expand_excerpt(excerpt_id, direction, window, cx);
                        });
                    })
                    .tooltip(Tooltip::for_action_title(
                        "Expand Excerpt",
                        &crate::actions::ExpandExcerpts::default(),
                    ))
                    .into_any_element();

                let position = point(
                    git_gutter_width + px(1.),
                    ix as f32 * line_height - (scroll_top % line_height) + px(1.),
                );
                let origin = gutter_hitbox.origin + position;

                Some((toggle, origin))
            })
            .collect();

        elements
    }

    fn layout_code_actions_indicator(
        &self,
        line_height: Pixels,
        newest_selection_head: DisplayPoint,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        breakpoint_points: &mut HashMap<DisplayRow, (Anchor, Breakpoint)>,
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let mut active = false;
        let mut button = None;
        let row = newest_selection_head.row();
        self.editor.update(cx, |editor, cx| {
            if let Some(crate::CodeContextMenu::CodeActions(CodeActionsMenu {
                deployed_from_indicator,
                ..
            })) = editor.context_menu.borrow().as_ref()
            {
                active = deployed_from_indicator.map_or(true, |indicator_row| indicator_row == row);
            };

            let breakpoint = breakpoint_points.get(&row);
            button = editor.render_code_actions_indicator(&self.style, row, active, breakpoint, cx);
        });

        let button = button?;
        breakpoint_points.remove(&row);

        let button = prepaint_gutter_button(
            button,
            row,
            line_height,
            gutter_dimensions,
            scroll_pixel_position,
            gutter_hitbox,
            display_hunks,
            window,
            cx,
        );

        Some(button)
    }

    fn get_participant_color(participant_index: Option<ParticipantIndex>, cx: &App) -> PlayerColor {
        if let Some(index) = participant_index {
            cx.theme().players().color_for_participant(index.0)
        } else {
            cx.theme().players().absent()
        }
    }

    fn calculate_relative_line_numbers(
        &self,
        snapshot: &EditorSnapshot,
        rows: &Range<DisplayRow>,
        relative_to: Option<DisplayRow>,
    ) -> HashMap<DisplayRow, DisplayRowDelta> {
        let mut relative_rows: HashMap<DisplayRow, DisplayRowDelta> = Default::default();
        let Some(relative_to) = relative_to else {
            return relative_rows;
        };

        let start = rows.start.min(relative_to);
        let end = rows.end.max(relative_to);

        let buffer_rows = snapshot
            .row_infos(start)
            .take(1 + end.minus(start) as usize)
            .collect::<Vec<_>>();

        let head_idx = relative_to.minus(start);
        let mut delta = 1;
        let mut i = head_idx + 1;
        while i < buffer_rows.len() as u32 {
            if buffer_rows[i as usize].buffer_row.is_some() {
                if rows.contains(&DisplayRow(i + start.0)) {
                    relative_rows.insert(DisplayRow(i + start.0), delta);
                }
                delta += 1;
            }
            i += 1;
        }
        delta = 1;
        i = head_idx.min(buffer_rows.len() as u32 - 1);
        while i > 0 && buffer_rows[i as usize].buffer_row.is_none() {
            i -= 1;
        }

        while i > 0 {
            i -= 1;
            if buffer_rows[i as usize].buffer_row.is_some() {
                if rows.contains(&DisplayRow(i + start.0)) {
                    relative_rows.insert(DisplayRow(i + start.0), delta);
                }
                delta += 1;
            }
        }

        relative_rows
    }

    fn layout_line_numbers(
        &self,
        gutter_hitbox: Option<&Hitbox>,
        gutter_dimensions: GutterDimensions,
        line_height: Pixels,
        scroll_position: gpui::Point<f32>,
        rows: Range<DisplayRow>,
        buffer_rows: &[RowInfo],
        active_rows: &BTreeMap<DisplayRow, LineHighlightSpec>,
        newest_selection_head: Option<DisplayPoint>,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Arc<HashMap<MultiBufferRow, LineNumberLayout>> {
        let include_line_numbers = snapshot.show_line_numbers.unwrap_or_else(|| {
            EditorSettings::get_global(cx).gutter.line_numbers && snapshot.mode.is_full()
        });
        if !include_line_numbers {
            return Arc::default();
        }

        let (newest_selection_head, is_relative) = self.editor.update(cx, |editor, cx| {
            let newest_selection_head = newest_selection_head.unwrap_or_else(|| {
                let newest = editor.selections.newest::<Point>(cx);
                SelectionLayout::new(
                    newest,
                    editor.selections.line_mode,
                    editor.cursor_shape,
                    &snapshot.display_snapshot,
                    true,
                    true,
                    None,
                )
                .head
            });
            let is_relative = editor.should_use_relative_line_numbers(cx);
            (newest_selection_head, is_relative)
        });

        let relative_to = if is_relative {
            Some(newest_selection_head.row())
        } else {
            None
        };
        let relative_rows = self.calculate_relative_line_numbers(snapshot, &rows, relative_to);
        let mut line_number = String::new();
        let line_numbers = buffer_rows
            .into_iter()
            .enumerate()
            .flat_map(|(ix, row_info)| {
                let display_row = DisplayRow(rows.start.0 + ix as u32);
                line_number.clear();
                let non_relative_number = row_info.buffer_row? + 1;
                let number = relative_rows
                    .get(&display_row)
                    .unwrap_or(&non_relative_number);
                write!(&mut line_number, "{number}").unwrap();
                if row_info
                    .diff_status
                    .is_some_and(|status| status.is_deleted())
                {
                    return None;
                }

                let color = active_rows
                    .get(&display_row)
                    .map(|spec| {
                        if spec.breakpoint {
                            cx.theme().colors().debugger_accent
                        } else {
                            cx.theme().colors().editor_active_line_number
                        }
                    })
                    .unwrap_or_else(|| cx.theme().colors().editor_line_number);
                let shaped_line = self
                    .shape_line_number(SharedString::from(&line_number), color, window)
                    .log_err()?;
                let scroll_top = scroll_position.y * line_height;
                let line_origin = gutter_hitbox.map(|hitbox| {
                    hitbox.origin
                        + point(
                            hitbox.size.width - shaped_line.width - gutter_dimensions.right_padding,
                            ix as f32 * line_height - (scroll_top % line_height),
                        )
                });

                #[cfg(not(test))]
                let hitbox = line_origin.map(|line_origin| {
                    window.insert_hitbox(
                        Bounds::new(line_origin, size(shaped_line.width, line_height)),
                        false,
                    )
                });
                #[cfg(test)]
                let hitbox = {
                    let _ = line_origin;
                    None
                };

                let multi_buffer_row = DisplayPoint::new(display_row, 0).to_point(snapshot).row;
                let multi_buffer_row = MultiBufferRow(multi_buffer_row);
                let line_number = LineNumberLayout {
                    shaped_line,
                    hitbox,
                };
                Some((multi_buffer_row, line_number))
            })
            .collect();
        Arc::new(line_numbers)
    }

    fn layout_crease_toggles(
        &self,
        rows: Range<DisplayRow>,
        row_infos: &[RowInfo],
        active_rows: &BTreeMap<DisplayRow, LineHighlightSpec>,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Option<AnyElement>> {
        let include_fold_statuses = EditorSettings::get_global(cx).gutter.folds
            && snapshot.mode.is_full()
            && self.editor.read(cx).is_singleton(cx);
        if include_fold_statuses {
            row_infos
                .into_iter()
                .enumerate()
                .map(|(ix, info)| {
                    if info.expand_info.is_some() {
                        return None;
                    }
                    let row = info.multibuffer_row?;
                    let display_row = DisplayRow(rows.start.0 + ix as u32);
                    let active = active_rows.contains_key(&display_row);

                    snapshot.render_crease_toggle(row, active, self.editor.clone(), window, cx)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn layout_crease_trailers(
        &self,
        buffer_rows: impl IntoIterator<Item = RowInfo>,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Option<AnyElement>> {
        buffer_rows
            .into_iter()
            .map(|row_info| {
                if row_info.expand_info.is_some() {
                    return None;
                }
                if let Some(row) = row_info.multibuffer_row {
                    snapshot.render_crease_trailer(row, window, cx)
                } else {
                    None
                }
            })
            .collect()
    }

    fn layout_lines(
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        editor_width: Pixels,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<LineWithInvisibles> {
        if rows.start >= rows.end {
            return Vec::new();
        }

        // Show the placeholder when the editor is empty
        if snapshot.is_empty() {
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let placeholder_color = cx.theme().colors().text_placeholder;
            let placeholder_text = snapshot.placeholder_text();

            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start.0 as usize)
                .chain(iter::repeat(""))
                .take(rows.len());
            placeholder_lines
                .filter_map(move |line| {
                    let run = TextRun {
                        len: line.len(),
                        font: style.text.font(),
                        color: placeholder_color,
                        background_color: None,
                        underline: Default::default(),
                        strikethrough: None,
                    };
                    window
                        .text_system()
                        .shape_line(line.to_string().into(), font_size, &[run])
                        .log_err()
                })
                .map(|line| LineWithInvisibles {
                    width: line.width,
                    len: line.len,
                    fragments: smallvec![LineFragment::Text(line)],
                    invisibles: Vec::new(),
                    font_size,
                })
                .collect()
        } else {
            let chunks = snapshot.highlighted_chunks(rows.clone(), true, style);
            LineWithInvisibles::from_chunks(
                chunks,
                &style,
                MAX_LINE_LEN,
                rows.len(),
                snapshot.mode,
                editor_width,
                is_row_soft_wrapped,
                window,
                cx,
            )
        }
    }

    fn prepaint_lines(
        &self,
        start_row: DisplayRow,
        line_layouts: &mut [LineWithInvisibles],
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        content_origin: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> SmallVec<[AnyElement; 1]> {
        let mut line_elements = SmallVec::new();
        for (ix, line) in line_layouts.iter_mut().enumerate() {
            let row = start_row + DisplayRow(ix as u32);
            line.prepaint(
                line_height,
                scroll_pixel_position,
                row,
                content_origin,
                &mut line_elements,
                window,
                cx,
            );
        }
        line_elements
    }

    fn render_block(
        &self,
        block: &Block,
        available_width: AvailableSpace,
        block_id: BlockId,
        block_row_start: DisplayRow,
        snapshot: &EditorSnapshot,
        text_x: Pixels,
        rows: &Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        gutter_dimensions: &GutterDimensions,
        line_height: Pixels,
        em_width: Pixels,
        text_hitbox: &Hitbox,
        editor_width: Pixels,
        scroll_width: &mut Pixels,
        resized_blocks: &mut HashMap<CustomBlockId, u32>,
        row_block_types: &mut HashMap<DisplayRow, bool>,
        selections: &[Selection<Point>],
        selected_buffer_ids: &Vec<BufferId>,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        sticky_header_excerpt_id: Option<ExcerptId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, Size<Pixels>, DisplayRow, Pixels)> {
        let mut x_position = None;
        let mut element = match block {
            Block::Custom(custom) => {
                let block_start = custom.start().to_point(&snapshot.buffer_snapshot);
                let block_end = custom.end().to_point(&snapshot.buffer_snapshot);
                if block.place_near() && snapshot.is_line_folded(MultiBufferRow(block_start.row)) {
                    return None;
                }
                let align_to = block_start.to_display_point(snapshot);
                let x_and_width = |layout: &LineWithInvisibles| {
                    Some((
                        text_x + layout.x_for_index(align_to.column() as usize),
                        text_x + layout.width,
                    ))
                };
                let line_ix = align_to.row().0.checked_sub(rows.start.0);
                x_position =
                    if let Some(layout) = line_ix.and_then(|ix| line_layouts.get(ix as usize)) {
                        x_and_width(&layout)
                    } else {
                        x_and_width(&layout_line(
                            align_to.row(),
                            snapshot,
                            &self.style,
                            editor_width,
                            is_row_soft_wrapped,
                            window,
                            cx,
                        ))
                    };

                let anchor_x = x_position.unwrap().0;

                let selected = selections
                    .binary_search_by(|selection| {
                        if selection.end <= block_start {
                            Ordering::Less
                        } else if selection.start >= block_end {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    })
                    .is_ok();

                div()
                    .size_full()
                    .child(custom.render(&mut BlockContext {
                        window,
                        app: cx,
                        anchor_x,
                        gutter_dimensions,
                        line_height,
                        em_width,
                        block_id,
                        selected,
                        max_width: text_hitbox.size.width.max(*scroll_width),
                        editor_style: &self.style,
                    }))
                    .into_any()
            }

            Block::FoldedBuffer {
                first_excerpt,
                height,
                ..
            } => {
                let selected = selected_buffer_ids.contains(&first_excerpt.buffer_id);
                let result = v_flex().id(block_id).w_full();

                let jump_data = header_jump_data(snapshot, block_row_start, *height, first_excerpt);
                result
                    .child(self.render_buffer_header(
                        first_excerpt,
                        true,
                        selected,
                        false,
                        jump_data,
                        window,
                        cx,
                    ))
                    .into_any_element()
            }

            Block::ExcerptBoundary {
                excerpt,
                height,
                starts_new_buffer,
                ..
            } => {
                let color = cx.theme().colors().clone();
                let mut result = v_flex().id(block_id).w_full();

                let jump_data = header_jump_data(snapshot, block_row_start, *height, excerpt);

                if *starts_new_buffer {
                    if sticky_header_excerpt_id != Some(excerpt.id) {
                        let selected = selected_buffer_ids.contains(&excerpt.buffer_id);

                        result = result.child(self.render_buffer_header(
                            excerpt, false, selected, false, jump_data, window, cx,
                        ));
                    } else {
                        result =
                            result.child(div().h(FILE_HEADER_HEIGHT as f32 * window.line_height()));
                    }
                } else {
                    result = result.child(
                        h_flex().relative().child(
                            div()
                                .top(line_height / 2.)
                                .absolute()
                                .w_full()
                                .h_px()
                                .bg(color.border_variant),
                        ),
                    );
                };

                result.into_any()
            }
        };

        // Discover the element's content height, then round up to the nearest multiple of line height.
        let preliminary_size = element.layout_as_root(
            size(available_width, AvailableSpace::MinContent),
            window,
            cx,
        );
        let quantized_height = (preliminary_size.height / line_height).ceil() * line_height;
        let final_size = if preliminary_size.height == quantized_height {
            preliminary_size
        } else {
            element.layout_as_root(size(available_width, quantized_height.into()), window, cx)
        };
        let mut element_height_in_lines = ((final_size.height / line_height).ceil() as u32).max(1);

        let mut row = block_row_start;
        let mut x_offset = px(0.);
        let mut is_block = true;

        if let BlockId::Custom(custom_block_id) = block_id {
            if block.has_height() {
                if block.place_near() {
                    if let Some((x_target, line_width)) = x_position {
                        let margin = em_width * 2;
                        if line_width + final_size.width + margin
                            < editor_width + gutter_dimensions.full_width()
                            && !row_block_types.contains_key(&(row - 1))
                            && element_height_in_lines == 1
                        {
                            x_offset = line_width + margin;
                            row = row - 1;
                            is_block = false;
                            element_height_in_lines = 0;
                            row_block_types.insert(row, is_block);
                        } else {
                            let max_offset =
                                editor_width + gutter_dimensions.full_width() - final_size.width;
                            let min_offset = (x_target + em_width - final_size.width)
                                .max(gutter_dimensions.full_width());
                            x_offset = x_target.min(max_offset).max(min_offset);
                        }
                    }
                };
                if element_height_in_lines != block.height() {
                    resized_blocks.insert(custom_block_id, element_height_in_lines);
                }
            }
        }
        for i in 0..element_height_in_lines {
            row_block_types.insert(row + i, is_block);
        }

        Some((element, final_size, row, x_offset))
    }

    fn render_buffer_header(
        &self,
        for_excerpt: &ExcerptInfo,
        is_folded: bool,
        is_selected: bool,
        is_sticky: bool,
        jump_data: JumpData,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let editor = self.editor.read(cx);
        let file_status = editor
            .buffer
            .read(cx)
            .all_diff_hunks_expanded()
            .then(|| {
                editor
                    .project
                    .as_ref()?
                    .read(cx)
                    .status_for_buffer_id(for_excerpt.buffer_id, cx)
            })
            .flatten();

        let include_root = editor
            .project
            .as_ref()
            .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
            .unwrap_or_default();
        let can_open_excerpts = Editor::can_open_excerpts_in_file(for_excerpt.buffer.file());
        let path = for_excerpt.buffer.resolve_file_path(cx, include_root);
        let filename = path
            .as_ref()
            .and_then(|path| Some(path.file_name()?.to_string_lossy().to_string()));
        let parent_path = path.as_ref().and_then(|path| {
            Some(path.parent()?.to_string_lossy().to_string() + std::path::MAIN_SEPARATOR_STR)
        });
        let focus_handle = editor.focus_handle(cx);
        let colors = cx.theme().colors();

        div()
            .p_1()
            .w_full()
            .h(FILE_HEADER_HEIGHT as f32 * window.line_height())
            .child(
                h_flex()
                    .size_full()
                    .gap_2()
                    .flex_basis(Length::Definite(DefiniteLength::Fraction(0.667)))
                    .pl_0p5()
                    .pr_5()
                    .rounded_sm()
                    .when(is_sticky, |el| el.shadow_md())
                    .border_1()
                    .map(|div| {
                        let border_color = if is_selected
                            && is_folded
                            && focus_handle.contains_focused(window, cx)
                        {
                            colors.border_focused
                        } else {
                            colors.border
                        };
                        div.border_color(border_color)
                    })
                    .bg(colors.editor_subheader_background)
                    .hover(|style| style.bg(colors.element_hover))
                    .map(|header| {
                        let editor = self.editor.clone();
                        let buffer_id = for_excerpt.buffer_id;
                        let toggle_chevron_icon =
                            FileIcons::get_chevron_icon(!is_folded, cx).map(Icon::from_path);
                        header.child(
                            div()
                                .hover(|style| style.bg(colors.element_selected))
                                .rounded_xs()
                                .child(
                                    ButtonLike::new("toggle-buffer-fold")
                                        .style(ui::ButtonStyle::Transparent)
                                        .height(px(28.).into())
                                        .width(px(28.).into())
                                        .children(toggle_chevron_icon)
                                        .tooltip({
                                            let focus_handle = focus_handle.clone();
                                            move |window, cx| {
                                                Tooltip::for_action_in(
                                                    "Toggle Excerpt Fold",
                                                    &ToggleFold,
                                                    &focus_handle,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .on_click(move |_, _, cx| {
                                            if is_folded {
                                                editor.update(cx, |editor, cx| {
                                                    editor.unfold_buffer(buffer_id, cx);
                                                });
                                            } else {
                                                editor.update(cx, |editor, cx| {
                                                    editor.fold_buffer(buffer_id, cx);
                                                });
                                            }
                                        }),
                                ),
                        )
                    })
                    .children(
                        editor
                            .addons
                            .values()
                            .filter_map(|addon| {
                                addon.render_buffer_header_controls(for_excerpt, window, cx)
                            })
                            .take(1),
                    )
                    .child(
                        h_flex()
                            .cursor_pointer()
                            .id("path header block")
                            .size_full()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new(
                                            filename
                                                .map(SharedString::from)
                                                .unwrap_or_else(|| "untitled".into()),
                                        )
                                        .single_line()
                                        .when_some(
                                            file_status,
                                            |el, status| {
                                                el.color(if status.is_conflicted() {
                                                    Color::Conflict
                                                } else if status.is_modified() {
                                                    Color::Modified
                                                } else if status.is_deleted() {
                                                    Color::Disabled
                                                } else {
                                                    Color::Created
                                                })
                                                .when(status.is_deleted(), |el| el.strikethrough())
                                            },
                                        ),
                                    )
                                    .when_some(parent_path, |then, path| {
                                        then.child(div().child(path).text_color(
                                            if file_status.is_some_and(FileStatus::is_deleted) {
                                                colors.text_disabled
                                            } else {
                                                colors.text_muted
                                            },
                                        ))
                                    }),
                            )
                            .when(can_open_excerpts && is_selected && path.is_some(), |el| {
                                el.child(
                                    h_flex()
                                        .id("jump-to-file-button")
                                        .gap_2p5()
                                        .child(Label::new("Jump To File"))
                                        .children(
                                            KeyBinding::for_action_in(
                                                &OpenExcerpts,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|binding| binding.into_any_element()),
                                        ),
                                )
                            })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(window.listener_for(&self.editor, {
                                move |editor, e: &ClickEvent, window, cx| {
                                    editor.open_excerpts_common(
                                        Some(jump_data.clone()),
                                        e.down.modifiers.secondary(),
                                        window,
                                        cx,
                                    );
                                }
                            })),
                    ),
            )
    }

    fn render_blocks(
        &self,
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        editor_width: Pixels,
        scroll_width: &mut Pixels,
        gutter_dimensions: &GutterDimensions,
        em_width: Pixels,
        text_x: Pixels,
        line_height: Pixels,
        line_layouts: &mut [LineWithInvisibles],
        selections: &[Selection<Point>],
        selected_buffer_ids: &Vec<BufferId>,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        sticky_header_excerpt_id: Option<ExcerptId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(Vec<BlockLayout>, HashMap<DisplayRow, bool>), HashMap<CustomBlockId, u32>> {
        let (fixed_blocks, non_fixed_blocks) = snapshot
            .blocks_in_range(rows.clone())
            .partition::<Vec<_>, _>(|(_, block)| block.style() == BlockStyle::Fixed);

        let mut focused_block = self
            .editor
            .update(cx, |editor, _| editor.take_focused_block());
        let mut fixed_block_max_width = Pixels::ZERO;
        let mut blocks = Vec::new();
        let mut resized_blocks = HashMap::default();
        let mut row_block_types = HashMap::default();

        for (row, block) in fixed_blocks {
            let block_id = block.id();

            if focused_block.as_ref().map_or(false, |b| b.id == block_id) {
                focused_block = None;
            }

            if let Some((element, element_size, row, x_offset)) = self.render_block(
                block,
                AvailableSpace::MinContent,
                block_id,
                row,
                snapshot,
                text_x,
                &rows,
                line_layouts,
                gutter_dimensions,
                line_height,
                em_width,
                text_hitbox,
                editor_width,
                scroll_width,
                &mut resized_blocks,
                &mut row_block_types,
                selections,
                selected_buffer_ids,
                is_row_soft_wrapped,
                sticky_header_excerpt_id,
                window,
                cx,
            ) {
                fixed_block_max_width = fixed_block_max_width.max(element_size.width + em_width);
                blocks.push(BlockLayout {
                    id: block_id,
                    x_offset,
                    row: Some(row),
                    element,
                    available_space: size(AvailableSpace::MinContent, element_size.height.into()),
                    style: BlockStyle::Fixed,
                    overlaps_gutter: true,
                    is_buffer_header: block.is_buffer_header(),
                });
            }
        }

        for (row, block) in non_fixed_blocks {
            let style = block.style();
            let width = match (style, block.place_near()) {
                (_, true) => AvailableSpace::MinContent,
                (BlockStyle::Sticky, _) => hitbox.size.width.into(),
                (BlockStyle::Flex, _) => hitbox
                    .size
                    .width
                    .max(fixed_block_max_width)
                    .max(gutter_dimensions.width + *scroll_width)
                    .into(),
                (BlockStyle::Fixed, _) => unreachable!(),
            };
            let block_id = block.id();

            if focused_block.as_ref().map_or(false, |b| b.id == block_id) {
                focused_block = None;
            }

            if let Some((element, element_size, row, x_offset)) = self.render_block(
                block,
                width,
                block_id,
                row,
                snapshot,
                text_x,
                &rows,
                line_layouts,
                gutter_dimensions,
                line_height,
                em_width,
                text_hitbox,
                editor_width,
                scroll_width,
                &mut resized_blocks,
                &mut row_block_types,
                selections,
                selected_buffer_ids,
                is_row_soft_wrapped,
                sticky_header_excerpt_id,
                window,
                cx,
            ) {
                blocks.push(BlockLayout {
                    id: block_id,
                    x_offset,
                    row: Some(row),
                    element,
                    available_space: size(width, element_size.height.into()),
                    style,
                    overlaps_gutter: !block.place_near(),
                    is_buffer_header: block.is_buffer_header(),
                });
            }
        }

        if let Some(focused_block) = focused_block {
            if let Some(focus_handle) = focused_block.focus_handle.upgrade() {
                if focus_handle.is_focused(window) {
                    if let Some(block) = snapshot.block_for_id(focused_block.id) {
                        let style = block.style();
                        let width = match style {
                            BlockStyle::Fixed => AvailableSpace::MinContent,
                            BlockStyle::Flex => AvailableSpace::Definite(
                                hitbox
                                    .size
                                    .width
                                    .max(fixed_block_max_width)
                                    .max(gutter_dimensions.width + *scroll_width),
                            ),
                            BlockStyle::Sticky => AvailableSpace::Definite(hitbox.size.width),
                        };

                        if let Some((element, element_size, _, x_offset)) = self.render_block(
                            &block,
                            width,
                            focused_block.id,
                            rows.end,
                            snapshot,
                            text_x,
                            &rows,
                            line_layouts,
                            gutter_dimensions,
                            line_height,
                            em_width,
                            text_hitbox,
                            editor_width,
                            scroll_width,
                            &mut resized_blocks,
                            &mut row_block_types,
                            selections,
                            selected_buffer_ids,
                            is_row_soft_wrapped,
                            sticky_header_excerpt_id,
                            window,
                            cx,
                        ) {
                            blocks.push(BlockLayout {
                                id: block.id(),
                                x_offset,
                                row: None,
                                element,
                                available_space: size(width, element_size.height.into()),
                                style,
                                overlaps_gutter: true,
                                is_buffer_header: block.is_buffer_header(),
                            });
                        }
                    }
                }
            }
        }

        if resized_blocks.is_empty() {
            *scroll_width = (*scroll_width).max(fixed_block_max_width - gutter_dimensions.width);
            Ok((blocks, row_block_types))
        } else {
            Err(resized_blocks)
        }
    }

    fn layout_blocks(
        &self,
        blocks: &mut Vec<BlockLayout>,
        hitbox: &Hitbox,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        for block in blocks {
            let mut origin = if let Some(row) = block.row {
                hitbox.origin
                    + point(
                        block.x_offset,
                        row.as_f32() * line_height - scroll_pixel_position.y,
                    )
            } else {
                // Position the block outside the visible area
                hitbox.origin + point(Pixels::ZERO, hitbox.size.height)
            };

            if !matches!(block.style, BlockStyle::Sticky) {
                origin += point(-scroll_pixel_position.x, Pixels::ZERO);
            }

            let focus_handle =
                block
                    .element
                    .prepaint_as_root(origin, block.available_space, window, cx);

            if let Some(focus_handle) = focus_handle {
                self.editor.update(cx, |editor, _cx| {
                    editor.set_focused_block(FocusedBlock {
                        id: block.id,
                        focus_handle: focus_handle.downgrade(),
                    });
                });
            }
        }
    }

    fn layout_sticky_buffer_header(
        &self,
        StickyHeaderExcerpt { excerpt }: StickyHeaderExcerpt<'_>,
        scroll_position: f32,
        line_height: Pixels,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        selected_buffer_ids: &Vec<BufferId>,
        blocks: &[BlockLayout],
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let jump_data = header_jump_data(
            snapshot,
            DisplayRow(scroll_position as u32),
            FILE_HEADER_HEIGHT + MULTI_BUFFER_EXCERPT_HEADER_HEIGHT,
            excerpt,
        );

        let editor_bg_color = cx.theme().colors().editor_background;

        let selected = selected_buffer_ids.contains(&excerpt.buffer_id);

        let mut header = v_flex()
            .relative()
            .child(
                div()
                    .w(hitbox.bounds.size.width)
                    .h(FILE_HEADER_HEIGHT as f32 * line_height)
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(editor_bg_color.opacity(0.), 0.),
                        linear_color_stop(editor_bg_color, 0.6),
                    ))
                    .absolute()
                    .top_0(),
            )
            .child(
                self.render_buffer_header(excerpt, false, selected, true, jump_data, window, cx)
                    .into_any_element(),
            )
            .into_any_element();

        let mut origin = hitbox.origin;
        // Move floating header up to avoid colliding with the next buffer header.
        for block in blocks.iter() {
            if !block.is_buffer_header {
                continue;
            }

            let Some(display_row) = block.row.filter(|row| row.0 > scroll_position as u32) else {
                continue;
            };

            let max_row = display_row.0.saturating_sub(FILE_HEADER_HEIGHT);
            let offset = scroll_position - max_row as f32;

            if offset > 0.0 {
                origin.y -= offset * line_height;
            }
            break;
        }

        let size = size(
            AvailableSpace::Definite(hitbox.size.width),
            AvailableSpace::MinContent,
        );

        header.prepaint_as_root(origin, size, window, cx);

        header
    }

    fn layout_cursor_popovers(
        &self,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        start_row: DisplayRow,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_layouts: &[LineWithInvisibles],
        cursor: DisplayPoint,
        cursor_point: Point,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut min_menu_height = Pixels::ZERO;
        let mut max_menu_height = Pixels::ZERO;
        let mut height_above_menu = Pixels::ZERO;
        let height_below_menu = Pixels::ZERO;
        let mut edit_prediction_popover_visible = false;
        let mut context_menu_visible = false;
        let context_menu_placement;

        {
            let editor = self.editor.read(cx);
            if editor
                .edit_prediction_visible_in_cursor_popover(editor.has_active_inline_completion())
            {
                height_above_menu +=
                    editor.edit_prediction_cursor_popover_height() + POPOVER_Y_PADDING;
                edit_prediction_popover_visible = true;
            }

            if editor.context_menu_visible() {
                if let Some(crate::ContextMenuOrigin::Cursor) = editor.context_menu_origin() {
                    let (min_height_in_lines, max_height_in_lines) = editor
                        .context_menu_options
                        .as_ref()
                        .map_or((3, 12), |options| {
                            (options.min_entries_visible, options.max_entries_visible)
                        });

                    min_menu_height += line_height * min_height_in_lines as f32 + POPOVER_Y_PADDING;
                    max_menu_height += line_height * max_height_in_lines as f32 + POPOVER_Y_PADDING;
                    context_menu_visible = true;
                }
            }
            context_menu_placement = editor
                .context_menu_options
                .as_ref()
                .and_then(|options| options.placement.clone());
        }

        let visible = edit_prediction_popover_visible || context_menu_visible;
        if !visible {
            return;
        }

        let cursor_row_layout = &line_layouts[cursor.row().minus(start_row) as usize];
        let target_position = content_origin
            + gpui::Point {
                x: cmp::max(
                    px(0.),
                    cursor_row_layout.x_for_index(cursor.column() as usize)
                        - scroll_pixel_position.x,
                ),
                y: cmp::max(
                    px(0.),
                    cursor.row().next_row().as_f32() * line_height - scroll_pixel_position.y,
                ),
            };

        let viewport_bounds =
            Bounds::new(Default::default(), window.viewport_size()).extend(Edges {
                right: -Self::SCROLLBAR_WIDTH - MENU_GAP,
                ..Default::default()
            });

        let min_height = height_above_menu + min_menu_height + height_below_menu;
        let max_height = height_above_menu + max_menu_height + height_below_menu;
        let Some((laid_out_popovers, y_flipped)) = self.layout_popovers_above_or_below_line(
            target_position,
            line_height,
            min_height,
            max_height,
            context_menu_placement,
            text_hitbox,
            viewport_bounds,
            window,
            cx,
            |height, max_width_for_stable_x, y_flipped, window, cx| {
                // First layout the menu to get its size - others can be at least this wide.
                let context_menu = if context_menu_visible {
                    let menu_height = if y_flipped {
                        height - height_below_menu
                    } else {
                        height - height_above_menu
                    };
                    let mut element = self
                        .render_context_menu(line_height, menu_height, window, cx)
                        .expect("Visible context menu should always render.");
                    let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                    Some((CursorPopoverType::CodeContextMenu, element, size))
                } else {
                    None
                };
                let min_width = context_menu
                    .as_ref()
                    .map_or(px(0.), |(_, _, size)| size.width);
                let max_width = max_width_for_stable_x.max(
                    context_menu
                        .as_ref()
                        .map_or(px(0.), |(_, _, size)| size.width),
                );

                let edit_prediction = if edit_prediction_popover_visible {
                    self.editor.update(cx, move |editor, cx| {
                        let accept_binding = editor.accept_edit_prediction_keybind(window, cx);
                        let mut element = editor.render_edit_prediction_cursor_popover(
                            min_width,
                            max_width,
                            cursor_point,
                            style,
                            accept_binding.keystroke(),
                            window,
                            cx,
                        )?;
                        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                        Some((CursorPopoverType::EditPrediction, element, size))
                    })
                } else {
                    None
                };
                vec![edit_prediction, context_menu]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            },
        ) else {
            return;
        };

        let Some((menu_ix, (_, menu_bounds))) = laid_out_popovers
            .iter()
            .find_position(|(x, _)| matches!(x, CursorPopoverType::CodeContextMenu))
        else {
            return;
        };
        let last_ix = laid_out_popovers.len() - 1;
        let menu_is_last = menu_ix == last_ix;
        let first_popover_bounds = laid_out_popovers[0].1;
        let last_popover_bounds = laid_out_popovers[last_ix].1;

        // Bounds to layout the aside around. When y_flipped, the aside goes either above or to the
        // right, and otherwise it goes below or to the right.
        let mut target_bounds = Bounds::from_corners(
            first_popover_bounds.origin,
            last_popover_bounds.bottom_right(),
        );
        target_bounds.size.width = menu_bounds.size.width;

        // Like `target_bounds`, but with the max height it could occupy. Choosing an aside position
        // based on this is preferred for layout stability.
        let mut max_target_bounds = target_bounds;
        max_target_bounds.size.height = max_height;
        if y_flipped {
            max_target_bounds.origin.y -= max_height - target_bounds.size.height;
        }

        // Add spacing around `target_bounds` and `max_target_bounds`.
        let mut extend_amount = Edges::all(MENU_GAP);
        if y_flipped {
            extend_amount.bottom = line_height;
        } else {
            extend_amount.top = line_height;
        }
        let target_bounds = target_bounds.extend(extend_amount);
        let max_target_bounds = max_target_bounds.extend(extend_amount);

        let must_place_above_or_below =
            if y_flipped && !menu_is_last && menu_bounds.size.height < max_menu_height {
                laid_out_popovers[menu_ix + 1..]
                    .iter()
                    .any(|(_, popover_bounds)| popover_bounds.size.width > menu_bounds.size.width)
            } else {
                false
            };

        self.layout_context_menu_aside(
            y_flipped,
            *menu_bounds,
            target_bounds,
            max_target_bounds,
            max_menu_height,
            must_place_above_or_below,
            text_hitbox,
            viewport_bounds,
            window,
            cx,
        );
    }

    fn layout_gutter_menu(
        &self,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_overshoot: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) {
        let editor = self.editor.read(cx);
        if !editor.context_menu_visible() {
            return;
        }
        let Some(crate::ContextMenuOrigin::GutterIndicator(gutter_row)) =
            editor.context_menu_origin()
        else {
            return;
        };
        // Context menu was spawned via a click on a gutter. Ensure it's a bit closer to the
        // indicator than just a plain first column of the text field.
        let target_position = content_origin
            + gpui::Point {
                x: -gutter_overshoot,
                y: gutter_row.next_row().as_f32() * line_height - scroll_pixel_position.y,
            };

        let (min_height_in_lines, max_height_in_lines) = editor
            .context_menu_options
            .as_ref()
            .map_or((3, 12), |options| {
                (options.min_entries_visible, options.max_entries_visible)
            });

        let min_height = line_height * min_height_in_lines as f32 + POPOVER_Y_PADDING;
        let max_height = line_height * max_height_in_lines as f32 + POPOVER_Y_PADDING;
        let viewport_bounds =
            Bounds::new(Default::default(), window.viewport_size()).extend(Edges {
                right: -Self::SCROLLBAR_WIDTH - MENU_GAP,
                ..Default::default()
            });
        self.layout_popovers_above_or_below_line(
            target_position,
            line_height,
            min_height,
            max_height,
            editor
                .context_menu_options
                .as_ref()
                .and_then(|options| options.placement.clone()),
            text_hitbox,
            viewport_bounds,
            window,
            cx,
            move |height, _max_width_for_stable_x, _, window, cx| {
                let mut element = self
                    .render_context_menu(line_height, height, window, cx)
                    .expect("Visible context menu should always render.");
                let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                vec![(CursorPopoverType::CodeContextMenu, element, size)]
            },
        );
    }

    fn layout_popovers_above_or_below_line(
        &self,
        target_position: gpui::Point<Pixels>,
        line_height: Pixels,
        min_height: Pixels,
        max_height: Pixels,
        placement: Option<ContextMenuPlacement>,
        text_hitbox: &Hitbox,
        viewport_bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
        make_sized_popovers: impl FnOnce(
            Pixels,
            Pixels,
            bool,
            &mut Window,
            &mut App,
        ) -> Vec<(CursorPopoverType, AnyElement, Size<Pixels>)>,
    ) -> Option<(Vec<(CursorPopoverType, Bounds<Pixels>)>, bool)> {
        let text_style = TextStyleRefinement {
            line_height: Some(DefiniteLength::Fraction(
                BufferLineHeight::Comfortable.value(),
            )),
            ..Default::default()
        };
        window.with_text_style(Some(text_style), |window| {
            // If the max height won't fit below and there is more space above, put it above the line.
            let bottom_y_when_flipped = target_position.y - line_height;
            let available_above = bottom_y_when_flipped - text_hitbox.top();
            let available_below = text_hitbox.bottom() - target_position.y;
            let y_overflows_below = max_height > available_below;
            let mut y_flipped = match placement {
                Some(ContextMenuPlacement::Above) => true,
                Some(ContextMenuPlacement::Below) => false,
                None => y_overflows_below && available_above > available_below,
            };
            let mut height = cmp::min(
                max_height,
                if y_flipped {
                    available_above
                } else {
                    available_below
                },
            );

            // If the min height doesn't fit within text bounds, instead fit within the window.
            if height < min_height {
                let available_above = bottom_y_when_flipped;
                let available_below = viewport_bounds.bottom() - target_position.y;
                let (y_flipped_override, height_override) = match placement {
                    Some(ContextMenuPlacement::Above) => {
                        (true, cmp::min(available_above, min_height))
                    }
                    Some(ContextMenuPlacement::Below) => {
                        (false, cmp::min(available_below, min_height))
                    }
                    None => {
                        if available_below > min_height {
                            (false, min_height)
                        } else if available_above > min_height {
                            (true, min_height)
                        } else if available_above > available_below {
                            (true, available_above)
                        } else {
                            (false, available_below)
                        }
                    }
                };
                y_flipped = y_flipped_override;
                height = height_override;
            }

            let max_width_for_stable_x = viewport_bounds.right() - target_position.x;

            // TODO: Use viewport_bounds.width as a max width so that it doesn't get clipped on the left
            // for very narrow windows.
            let popovers =
                make_sized_popovers(height, max_width_for_stable_x, y_flipped, window, cx);
            if popovers.is_empty() {
                return None;
            }

            let max_width = popovers
                .iter()
                .map(|(_, _, size)| size.width)
                .max()
                .unwrap_or_default();

            let mut current_position = gpui::Point {
                // Snap the right edge of the list to the right edge of the window if its horizontal bounds
                // overflow. Include space for the scrollbar.
                x: target_position
                    .x
                    .min((viewport_bounds.right() - max_width).max(Pixels::ZERO)),
                y: if y_flipped {
                    bottom_y_when_flipped
                } else {
                    target_position.y
                },
            };

            let mut laid_out_popovers = popovers
                .into_iter()
                .map(|(popover_type, element, size)| {
                    if y_flipped {
                        current_position.y -= size.height;
                    }
                    let position = current_position;
                    window.defer_draw(element, current_position, 1);
                    if !y_flipped {
                        current_position.y += size.height + MENU_GAP;
                    } else {
                        current_position.y -= MENU_GAP;
                    }
                    (popover_type, Bounds::new(position, size))
                })
                .collect::<Vec<_>>();

            if y_flipped {
                laid_out_popovers.reverse();
            }

            Some((laid_out_popovers, y_flipped))
        })
    }

    fn layout_context_menu_aside(
        &self,
        y_flipped: bool,
        menu_bounds: Bounds<Pixels>,
        target_bounds: Bounds<Pixels>,
        max_target_bounds: Bounds<Pixels>,
        max_height: Pixels,
        must_place_above_or_below: bool,
        text_hitbox: &Hitbox,
        viewport_bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let available_within_viewport = target_bounds.space_within(&viewport_bounds);
        let positioned_aside = if available_within_viewport.right >= MENU_ASIDE_MIN_WIDTH
            && !must_place_above_or_below
        {
            let max_width = cmp::min(
                available_within_viewport.right - px(1.),
                MENU_ASIDE_MAX_WIDTH,
            );
            let Some(mut aside) = self.render_context_menu_aside(
                size(max_width, max_height - POPOVER_Y_PADDING),
                window,
                cx,
            ) else {
                return;
            };
            aside.layout_as_root(AvailableSpace::min_size(), window, cx);
            let right_position = point(target_bounds.right(), menu_bounds.origin.y);
            Some((aside, right_position))
        } else {
            let max_size = size(
                // TODO(mgsloan): Once the menu is bounded by viewport width the bound on viewport
                // won't be needed here.
                cmp::min(
                    cmp::max(menu_bounds.size.width - px(2.), MENU_ASIDE_MIN_WIDTH),
                    viewport_bounds.right(),
                ),
                cmp::min(
                    max_height,
                    cmp::max(
                        available_within_viewport.top,
                        available_within_viewport.bottom,
                    ),
                ) - POPOVER_Y_PADDING,
            );
            let Some(mut aside) = self.render_context_menu_aside(max_size, window, cx) else {
                return;
            };
            let actual_size = aside.layout_as_root(AvailableSpace::min_size(), window, cx);

            let top_position = point(
                menu_bounds.origin.x,
                target_bounds.top() - actual_size.height,
            );
            let bottom_position = point(menu_bounds.origin.x, target_bounds.bottom());

            let fit_within = |available: Edges<Pixels>, wanted: Size<Pixels>| {
                // Prefer to fit on the same side of the line as the menu, then on the other side of
                // the line.
                if !y_flipped && wanted.height < available.bottom {
                    Some(bottom_position)
                } else if !y_flipped && wanted.height < available.top {
                    Some(top_position)
                } else if y_flipped && wanted.height < available.top {
                    Some(top_position)
                } else if y_flipped && wanted.height < available.bottom {
                    Some(bottom_position)
                } else {
                    None
                }
            };

            // Prefer choosing a direction using max sizes rather than actual size for stability.
            let available_within_text = max_target_bounds.space_within(&text_hitbox.bounds);
            let wanted = size(MENU_ASIDE_MAX_WIDTH, max_height);
            let aside_position = fit_within(available_within_text, wanted)
                // Fallback: fit max size in window.
                .or_else(|| fit_within(max_target_bounds.space_within(&viewport_bounds), wanted))
                // Fallback: fit actual size in window.
                .or_else(|| fit_within(available_within_viewport, actual_size));

            aside_position.map(|position| (aside, position))
        };

        // Skip drawing if it doesn't fit anywhere.
        if let Some((aside, position)) = positioned_aside {
            window.defer_draw(aside, position, 2);
        }
    }

    fn render_context_menu(
        &self,
        line_height: Pixels,
        height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let max_height_in_lines = ((height - POPOVER_Y_PADDING) / line_height).floor() as u32;
        self.editor.update(cx, |editor, cx| {
            editor.render_context_menu(&self.style, max_height_in_lines, window, cx)
        })
    }

    fn render_context_menu_aside(
        &self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        if max_size.width < px(100.) || max_size.height < px(12.) {
            None
        } else {
            self.editor.update(cx, |editor, cx| {
                editor.render_context_menu_aside(max_size, window, cx)
            })
        }
    }

    fn layout_mouse_context_menu(
        &self,
        editor_snapshot: &EditorSnapshot,
        visible_range: Range<DisplayRow>,
        content_origin: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let position = self.editor.update(cx, |editor, _cx| {
            let visible_start_point = editor.display_to_pixel_point(
                DisplayPoint::new(visible_range.start, 0),
                editor_snapshot,
                window,
            )?;
            let visible_end_point = editor.display_to_pixel_point(
                DisplayPoint::new(visible_range.end, 0),
                editor_snapshot,
                window,
            )?;

            let mouse_context_menu = editor.mouse_context_menu.as_ref()?;
            let (source_display_point, position) = match mouse_context_menu.position {
                MenuPosition::PinnedToScreen(point) => (None, point),
                MenuPosition::PinnedToEditor { source, offset } => {
                    let source_display_point = source.to_display_point(editor_snapshot);
                    let source_point = editor.to_pixel_point(source, editor_snapshot, window)?;
                    let position = content_origin + source_point + offset;
                    (Some(source_display_point), position)
                }
            };

            let source_included = source_display_point.map_or(true, |source_display_point| {
                visible_range
                    .to_inclusive()
                    .contains(&source_display_point.row())
            });
            let position_included =
                visible_start_point.y <= position.y && position.y <= visible_end_point.y;
            if !source_included && !position_included {
                None
            } else {
                Some(position)
            }
        })?;

        let text_style = TextStyleRefinement {
            line_height: Some(DefiniteLength::Fraction(
                BufferLineHeight::Comfortable.value(),
            )),
            ..Default::default()
        };
        window.with_text_style(Some(text_style), |window| {
            let mut element = self.editor.update(cx, |editor, _| {
                let mouse_context_menu = editor.mouse_context_menu.as_ref()?;
                let context_menu = mouse_context_menu.context_menu.clone();

                Some(
                    deferred(
                        anchored()
                            .position(position)
                            .child(context_menu)
                            .anchor(Corner::TopLeft)
                            .snap_to_window_with_margin(px(8.)),
                    )
                    .with_priority(1)
                    .into_any(),
                )
            })?;

            element.prepaint_as_root(position, AvailableSpace::min_size(), window, cx);
            Some(element)
        })
    }

    fn layout_hover_popovers(
        &self,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        visible_display_row_range: Range<DisplayRow>,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        em_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) {
        struct MeasuredHoverPopover {
            element: AnyElement,
            size: Size<Pixels>,
            horizontal_offset: Pixels,
        }

        let max_size = size(
            (120. * em_width) // Default size
                .min(hitbox.size.width / 2.) // Shrink to half of the editor width
                .max(MIN_POPOVER_CHARACTER_WIDTH * em_width), // Apply minimum width of 20 characters
            (16. * line_height) // Default size
                .min(hitbox.size.height / 2.) // Shrink to half of the editor height
                .max(MIN_POPOVER_LINE_HEIGHT * line_height), // Apply minimum height of 4 lines
        );

        let hover_popovers = self.editor.update(cx, |editor, cx| {
            editor.hover_state.render(
                snapshot,
                visible_display_row_range.clone(),
                max_size,
                window,
                cx,
            )
        });
        let Some((position, hover_popovers)) = hover_popovers else {
            return;
        };

        // This is safe because we check on layout whether the required row is available
        let hovered_row_layout =
            &line_layouts[position.row().minus(visible_display_row_range.start) as usize];

        // Compute Hovered Point
        let x =
            hovered_row_layout.x_for_index(position.column() as usize) - scroll_pixel_position.x;
        let y = position.row().as_f32() * line_height - scroll_pixel_position.y;
        let hovered_point = content_origin + point(x, y);

        let mut overall_height = Pixels::ZERO;
        let mut measured_hover_popovers = Vec::new();
        for mut hover_popover in hover_popovers {
            let size = hover_popover.layout_as_root(AvailableSpace::min_size(), window, cx);
            let horizontal_offset =
                (text_hitbox.top_right().x - POPOVER_RIGHT_OFFSET - (hovered_point.x + size.width))
                    .min(Pixels::ZERO);

            overall_height += HOVER_POPOVER_GAP + size.height;

            measured_hover_popovers.push(MeasuredHoverPopover {
                element: hover_popover,
                size,
                horizontal_offset,
            });
        }
        overall_height += HOVER_POPOVER_GAP;

        fn draw_occluder(
            width: Pixels,
            origin: gpui::Point<Pixels>,
            window: &mut Window,
            cx: &mut App,
        ) {
            let mut occlusion = div()
                .size_full()
                .occlude()
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .into_any_element();
            occlusion.layout_as_root(size(width, HOVER_POPOVER_GAP).into(), window, cx);
            window.defer_draw(occlusion, origin, 2);
        }

        if hovered_point.y > overall_height {
            // There is enough space above. Render popovers above the hovered point
            let mut current_y = hovered_point.y;
            for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                let size = popover.size;
                let popover_origin = point(
                    hovered_point.x + popover.horizontal_offset,
                    current_y - size.height,
                );

                window.defer_draw(popover.element, popover_origin, 2);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y - HOVER_POPOVER_GAP);
                    draw_occluder(size.width, origin, window, cx);
                }

                current_y = popover_origin.y - HOVER_POPOVER_GAP;
            }
        } else {
            // There is not enough space above. Render popovers below the hovered point
            let mut current_y = hovered_point.y + line_height;
            for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                let size = popover.size;
                let popover_origin = point(hovered_point.x + popover.horizontal_offset, current_y);

                window.defer_draw(popover.element, popover_origin, 2);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y + size.height);
                    draw_occluder(size.width, origin, window, cx);
                }

                current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
            }
        }
    }

    fn layout_diff_hunk_controls(
        &self,
        row_range: Range<DisplayRow>,
        row_infos: &[RowInfo],
        text_hitbox: &Hitbox,
        position_map: &PositionMap,
        newest_cursor_position: Option<DisplayPoint>,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        highlighted_rows: &BTreeMap<DisplayRow, LineHighlight>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        let render_diff_hunk_controls = editor.read(cx).render_diff_hunk_controls.clone();
        let point_for_position = position_map.point_for_position(window.mouse_position());

        let mut controls = vec![];

        let active_positions = [
            Some(point_for_position.previous_valid),
            newest_cursor_position,
        ];

        for (hunk, _) in display_hunks {
            if let DisplayDiffHunk::Unfolded {
                display_row_range,
                multi_buffer_range,
                status,
                is_created_file,
                ..
            } = &hunk
            {
                if display_row_range.start < row_range.start
                    || display_row_range.start >= row_range.end
                {
                    continue;
                }
                if highlighted_rows
                    .get(&display_row_range.start)
                    .and_then(|highlight| highlight.type_id)
                    .is_some_and(|type_id| {
                        [
                            TypeId::of::<ConflictsOuter>(),
                            TypeId::of::<ConflictsOursMarker>(),
                            TypeId::of::<ConflictsOurs>(),
                            TypeId::of::<ConflictsTheirs>(),
                            TypeId::of::<ConflictsTheirsMarker>(),
                        ]
                        .contains(&type_id)
                    })
                {
                    continue;
                }
                let row_ix = (display_row_range.start - row_range.start).0 as usize;
                if row_infos[row_ix].diff_status.is_none() {
                    continue;
                }
                if row_infos[row_ix]
                    .diff_status
                    .is_some_and(|status| status.is_added())
                    && !status.is_added()
                {
                    continue;
                }
                if active_positions
                    .iter()
                    .any(|p| p.map_or(false, |p| display_row_range.contains(&p.row())))
                {
                    let y = display_row_range.start.as_f32() * line_height
                        + text_hitbox.bounds.top()
                        - scroll_pixel_position.y;

                    let mut element = render_diff_hunk_controls(
                        display_row_range.start.0,
                        status,
                        multi_buffer_range.clone(),
                        *is_created_file,
                        line_height,
                        &editor,
                        window,
                        cx,
                    );
                    let size =
                        element.layout_as_root(size(px(100.0), line_height).into(), window, cx);

                    let x = text_hitbox.bounds.right()
                        - self.style.scrollbar_width
                        - px(10.)
                        - size.width;

                    window.with_absolute_element_offset(gpui::Point::new(x, y), |window| {
                        element.prepaint(window, cx)
                    });
                    controls.push(element);
                }
            }
        }

        controls
    }

    fn layout_signature_help(
        &self,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        newest_selection_head: Option<DisplayPoint>,
        start_row: DisplayRow,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        em_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !self.editor.focus_handle(cx).is_focused(window) {
            return;
        }
        let Some(newest_selection_head) = newest_selection_head else {
            return;
        };

        let max_size = size(
            (120. * em_width) // Default size
                .min(hitbox.size.width / 2.) // Shrink to half of the editor width
                .max(MIN_POPOVER_CHARACTER_WIDTH * em_width), // Apply minimum width of 20 characters
            (16. * line_height) // Default size
                .min(hitbox.size.height / 2.) // Shrink to half of the editor height
                .max(MIN_POPOVER_LINE_HEIGHT * line_height), // Apply minimum height of 4 lines
        );

        let maybe_element = self.editor.update(cx, |editor, cx| {
            if let Some(popover) = editor.signature_help_state.popover_mut() {
                let element = popover.render(max_size, cx);
                Some(element)
            } else {
                None
            }
        });
        let Some(mut element) = maybe_element else {
            return;
        };

        let selection_row = newest_selection_head.row();
        let Some(cursor_row_layout) = (selection_row >= start_row)
            .then(|| line_layouts.get(selection_row.minus(start_row) as usize))
            .flatten()
        else {
            return;
        };

        let target_x = cursor_row_layout.x_for_index(newest_selection_head.column() as usize)
            - scroll_pixel_position.x;
        let target_y = selection_row.as_f32() * line_height - scroll_pixel_position.y;
        let target_point = content_origin + point(target_x, target_y);

        let actual_size = element.layout_as_root(max_size.into(), window, cx);
        let overall_height = actual_size.height + HOVER_POPOVER_GAP;

        let popover_origin = if target_point.y > overall_height {
            point(target_point.x, target_point.y - actual_size.height)
        } else {
            point(
                target_point.x,
                target_point.y + line_height + HOVER_POPOVER_GAP,
            )
        };

        let horizontal_offset = (text_hitbox.top_right().x
            - POPOVER_RIGHT_OFFSET
            - (popover_origin.x + actual_size.width))
            .min(Pixels::ZERO);
        let final_origin = point(popover_origin.x + horizontal_offset, popover_origin.y);

        window.defer_draw(element, final_origin, 2);
    }

    fn paint_background(&self, layout: &EditorLayout, window: &mut Window, cx: &mut App) {
        window.paint_layer(layout.hitbox.bounds, |window| {
            let scroll_top = layout.position_map.snapshot.scroll_position().y;
            let gutter_bg = cx.theme().colors().editor_gutter_background;
            window.paint_quad(fill(layout.gutter_hitbox.bounds, gutter_bg));
            window.paint_quad(fill(
                layout.position_map.text_hitbox.bounds,
                self.style.background,
            ));

            if let EditorMode::Full {
                show_active_line_background,
                ..
            } = layout.mode
            {
                let mut active_rows = layout.active_rows.iter().peekable();
                while let Some((start_row, contains_non_empty_selection)) = active_rows.next() {
                    let mut end_row = start_row.0;
                    while active_rows
                        .peek()
                        .map_or(false, |(active_row, has_selection)| {
                            active_row.0 == end_row + 1
                                && has_selection.selection == contains_non_empty_selection.selection
                        })
                    {
                        active_rows.next().unwrap();
                        end_row += 1;
                    }

                    if show_active_line_background && !contains_non_empty_selection.selection {
                        let highlight_h_range =
                            match layout.position_map.snapshot.current_line_highlight {
                                CurrentLineHighlight::Gutter => Some(Range {
                                    start: layout.hitbox.left(),
                                    end: layout.gutter_hitbox.right(),
                                }),
                                CurrentLineHighlight::Line => Some(Range {
                                    start: layout.position_map.text_hitbox.bounds.left(),
                                    end: layout.position_map.text_hitbox.bounds.right(),
                                }),
                                CurrentLineHighlight::All => Some(Range {
                                    start: layout.hitbox.left(),
                                    end: layout.hitbox.right(),
                                }),
                                CurrentLineHighlight::None => None,
                            };
                        if let Some(range) = highlight_h_range {
                            let active_line_bg = cx.theme().colors().editor_active_line_background;
                            let bounds = Bounds {
                                origin: point(
                                    range.start,
                                    layout.hitbox.origin.y
                                        + (start_row.as_f32() - scroll_top)
                                            * layout.position_map.line_height,
                                ),
                                size: size(
                                    range.end - range.start,
                                    layout.position_map.line_height
                                        * (end_row - start_row.0 + 1) as f32,
                                ),
                            };
                            window.paint_quad(fill(bounds, active_line_bg));
                        }
                    }
                }

                let mut paint_highlight = |highlight_row_start: DisplayRow,
                                           highlight_row_end: DisplayRow,
                                           highlight: crate::LineHighlight,
                                           edges| {
                    let mut origin_x = layout.hitbox.left();
                    let mut width = layout.hitbox.size.width;
                    if !highlight.include_gutter {
                        origin_x += layout.gutter_hitbox.size.width;
                        width -= layout.gutter_hitbox.size.width;
                    }

                    let origin = point(
                        origin_x,
                        layout.hitbox.origin.y
                            + (highlight_row_start.as_f32() - scroll_top)
                                * layout.position_map.line_height,
                    );
                    let size = size(
                        width,
                        layout.position_map.line_height
                            * highlight_row_end.next_row().minus(highlight_row_start) as f32,
                    );
                    let mut quad = fill(Bounds { origin, size }, highlight.background);
                    if let Some(border_color) = highlight.border {
                        quad.border_color = border_color;
                        quad.border_widths = edges
                    }
                    window.paint_quad(quad);
                };

                let mut current_paint: Option<(LineHighlight, Range<DisplayRow>, Edges<Pixels>)> =
                    None;
                for (&new_row, &new_background) in &layout.highlighted_rows {
                    match &mut current_paint {
                        &mut Some((current_background, ref mut current_range, mut edges)) => {
                            let new_range_started = current_background != new_background
                                || current_range.end.next_row() != new_row;
                            if new_range_started {
                                if current_range.end.next_row() == new_row {
                                    edges.bottom = px(0.);
                                };
                                paint_highlight(
                                    current_range.start,
                                    current_range.end,
                                    current_background,
                                    edges,
                                );
                                let edges = Edges {
                                    top: if current_range.end.next_row() != new_row {
                                        px(1.)
                                    } else {
                                        px(0.)
                                    },
                                    bottom: px(1.),
                                    ..Default::default()
                                };
                                current_paint = Some((new_background, new_row..new_row, edges));
                                continue;
                            } else {
                                current_range.end = current_range.end.next_row();
                            }
                        }
                        None => {
                            let edges = Edges {
                                top: px(1.),
                                bottom: px(1.),
                                ..Default::default()
                            };
                            current_paint = Some((new_background, new_row..new_row, edges))
                        }
                    };
                }
                if let Some((color, range, edges)) = current_paint {
                    paint_highlight(range.start, range.end, color, edges);
                }

                let scroll_left =
                    layout.position_map.snapshot.scroll_position().x * layout.position_map.em_width;

                for (wrap_position, active) in layout.wrap_guides.iter() {
                    let x = (layout.position_map.text_hitbox.origin.x
                        + *wrap_position
                        + layout.position_map.em_width / 2.)
                        - scroll_left;

                    let show_scrollbars = layout
                        .scrollbars_layout
                        .as_ref()
                        .map_or(false, |layout| layout.visible);

                    if x < layout.position_map.text_hitbox.origin.x
                        || (show_scrollbars && x > self.scrollbar_left(&layout.hitbox.bounds))
                    {
                        continue;
                    }

                    let color = if *active {
                        cx.theme().colors().editor_active_wrap_guide
                    } else {
                        cx.theme().colors().editor_wrap_guide
                    };
                    window.paint_quad(fill(
                        Bounds {
                            origin: point(x, layout.position_map.text_hitbox.origin.y),
                            size: size(px(1.), layout.position_map.text_hitbox.size.height),
                        },
                        color,
                    ));
                }
            }
        })
    }

    fn paint_indent_guides(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(indent_guides) = &layout.indent_guides else {
            return;
        };

        let faded_color = |color: Hsla, alpha: f32| {
            let mut faded = color;
            faded.a = alpha;
            faded
        };

        for indent_guide in indent_guides {
            let indent_accent_colors = cx.theme().accents().color_for_index(indent_guide.depth);
            let settings = indent_guide.settings;

            // TODO fixed for now, expose them through themes later
            const INDENT_AWARE_ALPHA: f32 = 0.2;
            const INDENT_AWARE_ACTIVE_ALPHA: f32 = 0.4;
            const INDENT_AWARE_BACKGROUND_ALPHA: f32 = 0.1;
            const INDENT_AWARE_BACKGROUND_ACTIVE_ALPHA: f32 = 0.2;

            let line_color = match (settings.coloring, indent_guide.active) {
                (IndentGuideColoring::Disabled, _) => None,
                (IndentGuideColoring::Fixed, false) => {
                    Some(cx.theme().colors().editor_indent_guide)
                }
                (IndentGuideColoring::Fixed, true) => {
                    Some(cx.theme().colors().editor_indent_guide_active)
                }
                (IndentGuideColoring::IndentAware, false) => {
                    Some(faded_color(indent_accent_colors, INDENT_AWARE_ALPHA))
                }
                (IndentGuideColoring::IndentAware, true) => {
                    Some(faded_color(indent_accent_colors, INDENT_AWARE_ACTIVE_ALPHA))
                }
            };

            let background_color = match (settings.background_coloring, indent_guide.active) {
                (IndentGuideBackgroundColoring::Disabled, _) => None,
                (IndentGuideBackgroundColoring::IndentAware, false) => Some(faded_color(
                    indent_accent_colors,
                    INDENT_AWARE_BACKGROUND_ALPHA,
                )),
                (IndentGuideBackgroundColoring::IndentAware, true) => Some(faded_color(
                    indent_accent_colors,
                    INDENT_AWARE_BACKGROUND_ACTIVE_ALPHA,
                )),
            };

            let requested_line_width = if indent_guide.active {
                settings.active_line_width
            } else {
                settings.line_width
            }
            .clamp(1, 10);
            let mut line_indicator_width = 0.;
            if let Some(color) = line_color {
                window.paint_quad(fill(
                    Bounds {
                        origin: indent_guide.origin,
                        size: size(px(requested_line_width as f32), indent_guide.length),
                    },
                    color,
                ));
                line_indicator_width = requested_line_width as f32;
            }

            if let Some(color) = background_color {
                let width = indent_guide.single_indent_width - px(line_indicator_width);
                window.paint_quad(fill(
                    Bounds {
                        origin: point(
                            indent_guide.origin.x + px(line_indicator_width),
                            indent_guide.origin.y,
                        ),
                        size: size(width, indent_guide.length),
                    },
                    color,
                ));
            }
        }
    }

    fn paint_line_numbers(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        let is_singleton = self.editor.read(cx).is_singleton(cx);

        let line_height = layout.position_map.line_height;
        window.set_cursor_style(CursorStyle::Arrow, Some(&layout.gutter_hitbox));

        for LineNumberLayout {
            shaped_line,
            hitbox,
        } in layout.line_numbers.values()
        {
            let Some(hitbox) = hitbox else {
                continue;
            };

            let Some(()) = (if !is_singleton && hitbox.is_hovered(window) {
                let color = cx.theme().colors().editor_hover_line_number;

                let Some(line) = self
                    .shape_line_number(shaped_line.text.clone(), color, window)
                    .log_err()
                else {
                    continue;
                };

                line.paint(hitbox.origin, line_height, window, cx).log_err()
            } else {
                shaped_line
                    .paint(hitbox.origin, line_height, window, cx)
                    .log_err()
            }) else {
                continue;
            };

            // In singleton buffers, we select corresponding lines on the line number click, so use | -like cursor.
            // In multi buffers, we open file at the line number clicked, so use a pointing hand cursor.
            if is_singleton {
                window.set_cursor_style(CursorStyle::IBeam, Some(&hitbox));
            } else {
                window.set_cursor_style(CursorStyle::PointingHand, Some(&hitbox));
            }
        }
    }

    fn paint_gutter_diff_hunks(layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        if layout.display_hunks.is_empty() {
            return;
        }

        let line_height = layout.position_map.line_height;
        window.paint_layer(layout.gutter_hitbox.bounds, |window| {
            for (hunk, hitbox) in &layout.display_hunks {
                let hunk_to_paint = match hunk {
                    DisplayDiffHunk::Folded { .. } => {
                        let hunk_bounds = Self::diff_hunk_bounds(
                            &layout.position_map.snapshot,
                            line_height,
                            layout.gutter_hitbox.bounds,
                            &hunk,
                        );
                        Some((
                            hunk_bounds,
                            cx.theme().colors().version_control_modified,
                            Corners::all(px(0.)),
                            DiffHunkStatus::modified_none(),
                        ))
                    }
                    DisplayDiffHunk::Unfolded {
                        status,
                        display_row_range,
                        ..
                    } => hitbox.as_ref().map(|hunk_hitbox| match status.kind {
                        DiffHunkStatusKind::Added => (
                            hunk_hitbox.bounds,
                            cx.theme().colors().version_control_added,
                            Corners::all(px(0.)),
                            *status,
                        ),
                        DiffHunkStatusKind::Modified => (
                            hunk_hitbox.bounds,
                            cx.theme().colors().version_control_modified,
                            Corners::all(px(0.)),
                            *status,
                        ),
                        DiffHunkStatusKind::Deleted if !display_row_range.is_empty() => (
                            hunk_hitbox.bounds,
                            cx.theme().colors().version_control_deleted,
                            Corners::all(px(0.)),
                            *status,
                        ),
                        DiffHunkStatusKind::Deleted => (
                            Bounds::new(
                                point(
                                    hunk_hitbox.origin.x - hunk_hitbox.size.width,
                                    hunk_hitbox.origin.y,
                                ),
                                size(hunk_hitbox.size.width * 2., hunk_hitbox.size.height),
                            ),
                            cx.theme().colors().version_control_deleted,
                            Corners::all(1. * line_height),
                            *status,
                        ),
                    }),
                };

                if let Some((hunk_bounds, background_color, corner_radii, status)) = hunk_to_paint {
                    // Flatten the background color with the editor color to prevent
                    // elements below transparent hunks from showing through
                    let flattened_background_color = cx
                        .theme()
                        .colors()
                        .editor_background
                        .blend(background_color);

                    if !Self::diff_hunk_hollow(status, cx) {
                        window.paint_quad(quad(
                            hunk_bounds,
                            corner_radii,
                            flattened_background_color,
                            Edges::default(),
                            transparent_black(),
                            BorderStyle::default(),
                        ));
                    } else {
                        let flattened_unstaged_background_color = cx
                            .theme()
                            .colors()
                            .editor_background
                            .blend(background_color.opacity(0.3));

                        window.paint_quad(quad(
                            hunk_bounds,
                            corner_radii,
                            flattened_unstaged_background_color,
                            Edges::all(Pixels(1.0)),
                            flattened_background_color,
                            BorderStyle::Solid,
                        ));
                    }
                }
            }
        });
    }

    fn gutter_strip_width(line_height: Pixels) -> Pixels {
        (0.275 * line_height).floor()
    }

    fn diff_hunk_bounds(
        snapshot: &EditorSnapshot,
        line_height: Pixels,
        gutter_bounds: Bounds<Pixels>,
        hunk: &DisplayDiffHunk,
    ) -> Bounds<Pixels> {
        let scroll_position = snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;
        let gutter_strip_width = Self::gutter_strip_width(line_height);

        match hunk {
            DisplayDiffHunk::Folded { display_row, .. } => {
                let start_y = display_row.as_f32() * line_height - scroll_top;
                let end_y = start_y + line_height;
                let highlight_origin = gutter_bounds.origin + point(px(0.), start_y);
                let highlight_size = size(gutter_strip_width, end_y - start_y);
                Bounds::new(highlight_origin, highlight_size)
            }
            DisplayDiffHunk::Unfolded {
                display_row_range,
                status,
                ..
            } => {
                if status.is_deleted() && display_row_range.is_empty() {
                    let row = display_row_range.start;

                    let offset = line_height / 2.;
                    let start_y = row.as_f32() * line_height - offset - scroll_top;
                    let end_y = start_y + line_height;

                    let width = (0.35 * line_height).floor();
                    let highlight_origin = gutter_bounds.origin + point(px(0.), start_y);
                    let highlight_size = size(width, end_y - start_y);
                    Bounds::new(highlight_origin, highlight_size)
                } else {
                    let start_row = display_row_range.start;
                    let end_row = display_row_range.end;
                    // If we're in a multibuffer, row range span might include an
                    // excerpt header, so if we were to draw the marker straight away,
                    // the hunk might include the rows of that header.
                    // Making the range inclusive doesn't quite cut it, as we rely on the exclusivity for the soft wrap.
                    // Instead, we simply check whether the range we're dealing with includes
                    // any excerpt headers and if so, we stop painting the diff hunk on the first row of that header.
                    let end_row_in_current_excerpt = snapshot
                        .blocks_in_range(start_row..end_row)
                        .find_map(|(start_row, block)| {
                            if matches!(block, Block::ExcerptBoundary { .. }) {
                                Some(start_row)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(end_row);

                    let start_y = start_row.as_f32() * line_height - scroll_top;
                    let end_y = end_row_in_current_excerpt.as_f32() * line_height - scroll_top;

                    let highlight_origin = gutter_bounds.origin + point(px(0.), start_y);
                    let highlight_size = size(gutter_strip_width, end_y - start_y);
                    Bounds::new(highlight_origin, highlight_size)
                }
            }
        }
    }

    fn paint_gutter_indicators(
        &self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.paint_layer(layout.gutter_hitbox.bounds, |window| {
            window.with_element_namespace("crease_toggles", |window| {
                for crease_toggle in layout.crease_toggles.iter_mut().flatten() {
                    crease_toggle.paint(window, cx);
                }
            });

            window.with_element_namespace("expand_toggles", |window| {
                for (expand_toggle, _) in layout.expand_toggles.iter_mut().flatten() {
                    expand_toggle.paint(window, cx);
                }
            });

            for breakpoint in layout.breakpoints.iter_mut() {
                breakpoint.paint(window, cx);
            }

            for test_indicator in layout.test_indicators.iter_mut() {
                test_indicator.paint(window, cx);
            }

            if let Some(indicator) = layout.code_actions_indicator.as_mut() {
                indicator.paint(window, cx);
            }
        });
    }

    fn paint_gutter_highlights(
        &self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        for (_, hunk_hitbox) in &layout.display_hunks {
            if let Some(hunk_hitbox) = hunk_hitbox {
                if !self
                    .editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .all_diff_hunks_expanded()
                {
                    window.set_cursor_style(CursorStyle::PointingHand, Some(hunk_hitbox));
                }
            }
        }

        let show_git_gutter = layout
            .position_map
            .snapshot
            .show_git_diff_gutter
            .unwrap_or_else(|| {
                matches!(
                    ProjectSettings::get_global(cx).git.git_gutter,
                    Some(GitGutterSetting::TrackedFiles)
                )
            });
        if show_git_gutter {
            Self::paint_gutter_diff_hunks(layout, window, cx)
        }

        let highlight_width = 0.275 * layout.position_map.line_height;
        let highlight_corner_radii = Corners::all(0.05 * layout.position_map.line_height);
        window.paint_layer(layout.gutter_hitbox.bounds, |window| {
            for (range, color) in &layout.highlighted_gutter_ranges {
                let start_row = if range.start.row() < layout.visible_display_row_range.start {
                    layout.visible_display_row_range.start - DisplayRow(1)
                } else {
                    range.start.row()
                };
                let end_row = if range.end.row() > layout.visible_display_row_range.end {
                    layout.visible_display_row_range.end + DisplayRow(1)
                } else {
                    range.end.row()
                };

                let start_y = layout.gutter_hitbox.top()
                    + start_row.0 as f32 * layout.position_map.line_height
                    - layout.position_map.scroll_pixel_position.y;
                let end_y = layout.gutter_hitbox.top()
                    + (end_row.0 + 1) as f32 * layout.position_map.line_height
                    - layout.position_map.scroll_pixel_position.y;
                let bounds = Bounds::from_corners(
                    point(layout.gutter_hitbox.left(), start_y),
                    point(layout.gutter_hitbox.left() + highlight_width, end_y),
                );
                window.paint_quad(fill(bounds, *color).corner_radii(highlight_corner_radii));
            }
        });
    }

    fn paint_blamed_display_rows(
        &self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(blamed_display_rows) = layout.blamed_display_rows.take() else {
            return;
        };

        window.paint_layer(layout.gutter_hitbox.bounds, |window| {
            for mut blame_element in blamed_display_rows.into_iter() {
                blame_element.paint(window, cx);
            }
        })
    }

    fn paint_text(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        window.with_content_mask(
            Some(ContentMask {
                bounds: layout.position_map.text_hitbox.bounds,
            }),
            |window| {
                let editor = self.editor.read(cx);
                if editor.mouse_cursor_hidden {
                    window.set_cursor_style(CursorStyle::None, None);
                } else if editor
                    .hovered_link_state
                    .as_ref()
                    .is_some_and(|hovered_link_state| !hovered_link_state.links.is_empty())
                {
                    window.set_cursor_style(
                        CursorStyle::PointingHand,
                        Some(&layout.position_map.text_hitbox),
                    );
                } else {
                    window.set_cursor_style(
                        CursorStyle::IBeam,
                        Some(&layout.position_map.text_hitbox),
                    );
                };

                self.paint_lines_background(layout, window, cx);
                let invisible_display_ranges = self.paint_highlights(layout, window);
                self.paint_lines(&invisible_display_ranges, layout, window, cx);
                self.paint_redactions(layout, window);
                self.paint_cursors(layout, window, cx);
                self.paint_inline_diagnostics(layout, window, cx);
                self.paint_inline_blame(layout, window, cx);
                self.paint_diff_hunk_controls(layout, window, cx);
                window.with_element_namespace("crease_trailers", |window| {
                    for trailer in layout.crease_trailers.iter_mut().flatten() {
                        trailer.element.paint(window, cx);
                    }
                });
            },
        )
    }

    fn paint_highlights(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
    ) -> SmallVec<[Range<DisplayPoint>; 32]> {
        window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
            let mut invisible_display_ranges = SmallVec::<[Range<DisplayPoint>; 32]>::new();
            let line_end_overshoot = 0.15 * layout.position_map.line_height;
            for (range, color) in &layout.highlighted_ranges {
                self.paint_highlighted_range(
                    range.clone(),
                    *color,
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    window,
                );
            }

            let corner_radius = 0.15 * layout.position_map.line_height;

            for (player_color, selections) in &layout.selections {
                for selection in selections.iter() {
                    self.paint_highlighted_range(
                        selection.range.clone(),
                        player_color.selection,
                        corner_radius,
                        corner_radius * 2.,
                        layout,
                        window,
                    );

                    if selection.is_local && !selection.range.is_empty() {
                        invisible_display_ranges.push(selection.range.clone());
                    }
                }
            }
            invisible_display_ranges
        })
    }

    fn paint_lines(
        &mut self,
        invisible_display_ranges: &[Range<DisplayPoint>],
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let whitespace_setting = self
            .editor
            .read(cx)
            .buffer
            .read(cx)
            .language_settings(cx)
            .show_whitespaces;

        for (ix, line_with_invisibles) in layout.position_map.line_layouts.iter().enumerate() {
            let row = DisplayRow(layout.visible_display_row_range.start.0 + ix as u32);
            line_with_invisibles.draw(
                layout,
                row,
                layout.content_origin,
                whitespace_setting,
                invisible_display_ranges,
                window,
                cx,
            )
        }

        for line_element in &mut layout.line_elements {
            line_element.paint(window, cx);
        }
    }

    fn paint_lines_background(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        for (ix, line_with_invisibles) in layout.position_map.line_layouts.iter().enumerate() {
            let row = DisplayRow(layout.visible_display_row_range.start.0 + ix as u32);
            line_with_invisibles.draw_background(layout, row, layout.content_origin, window, cx);
        }
    }

    fn paint_redactions(&mut self, layout: &EditorLayout, window: &mut Window) {
        if layout.redacted_ranges.is_empty() {
            return;
        }

        let line_end_overshoot = layout.line_end_overshoot();

        // A softer than perfect black
        let redaction_color = gpui::rgb(0x0e1111);

        window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
            for range in layout.redacted_ranges.iter() {
                self.paint_highlighted_range(
                    range.clone(),
                    redaction_color.into(),
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    window,
                );
            }
        });
    }

    fn paint_cursors(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        for cursor in &mut layout.visible_cursors {
            cursor.paint(layout.content_origin, window, cx);
        }
    }

    fn paint_scrollbars(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        let Some(scrollbars_layout) = &layout.scrollbars_layout else {
            return;
        };

        for (scrollbar_layout, axis) in scrollbars_layout.iter_scrollbars() {
            let hitbox = &scrollbar_layout.hitbox;
            let thumb_bounds = scrollbar_layout.thumb_bounds();

            if scrollbars_layout.visible {
                let scrollbar_edges = match axis {
                    ScrollbarAxis::Horizontal => Edges {
                        top: Pixels::ZERO,
                        right: Pixels::ZERO,
                        bottom: Pixels::ZERO,
                        left: Pixels::ZERO,
                    },
                    ScrollbarAxis::Vertical => Edges {
                        top: Pixels::ZERO,
                        right: Pixels::ZERO,
                        bottom: Pixels::ZERO,
                        left: ScrollbarLayout::BORDER_WIDTH,
                    },
                };

                window.paint_layer(hitbox.bounds, |window| {
                    window.paint_quad(quad(
                        hitbox.bounds,
                        Corners::default(),
                        cx.theme().colors().scrollbar_track_background,
                        scrollbar_edges,
                        cx.theme().colors().scrollbar_track_border,
                        BorderStyle::Solid,
                    ));

                    if axis == ScrollbarAxis::Vertical {
                        let fast_markers =
                            self.collect_fast_scrollbar_markers(layout, &scrollbar_layout, cx);
                        // Refresh slow scrollbar markers in the background. Below, we
                        // paint whatever markers have already been computed.
                        self.refresh_slow_scrollbar_markers(layout, &scrollbar_layout, window, cx);

                        let markers = self.editor.read(cx).scrollbar_marker_state.markers.clone();
                        for marker in markers.iter().chain(&fast_markers) {
                            let mut marker = marker.clone();
                            marker.bounds.origin += hitbox.origin;
                            window.paint_quad(marker);
                        }
                    }

                    window.paint_quad(quad(
                        thumb_bounds,
                        Corners::default(),
                        cx.theme().colors().scrollbar_thumb_background,
                        scrollbar_edges,
                        cx.theme().colors().scrollbar_thumb_border,
                        BorderStyle::Solid,
                    ));
                })
            }
            window.set_cursor_style(CursorStyle::Arrow, Some(&hitbox));
        }

        window.on_mouse_event({
            let editor = self.editor.clone();
            let scrollbars_layout = scrollbars_layout.clone();

            let mut mouse_position = window.mouse_position();
            move |event: &MouseMoveEvent, phase, window, cx| {
                if phase == DispatchPhase::Capture {
                    return;
                }

                editor.update(cx, |editor, cx| {
                    if let Some((scrollbar_layout, axis)) = event
                        .pressed_button
                        .filter(|button| *button == MouseButton::Left)
                        .and(editor.scroll_manager.dragging_scrollbar_axis())
                        .and_then(|axis| {
                            scrollbars_layout
                                .iter_scrollbars()
                                .find(|(_, a)| *a == axis)
                        })
                    {
                        let ScrollbarLayout {
                            hitbox,
                            text_unit_size,
                            ..
                        } = scrollbar_layout;

                        let old_position = mouse_position.along(axis);
                        let new_position = event.position.along(axis);
                        if (hitbox.origin.along(axis)..hitbox.bottom_right().along(axis))
                            .contains(&old_position)
                        {
                            let position = editor.scroll_position(cx).apply_along(axis, |p| {
                                (p + (new_position - old_position) / *text_unit_size).max(0.)
                            });
                            editor.set_scroll_position(position, window, cx);
                        }
                        cx.stop_propagation();
                    } else {
                        editor.scroll_manager.reset_scrollbar_dragging_state(cx);
                    }

                    if scrollbars_layout.get_hovered_axis(window).is_some() {
                        editor.scroll_manager.show_scrollbars(window, cx);
                    }

                    mouse_position = event.position;
                })
            }
        });

        if self.editor.read(cx).scroll_manager.any_scrollbar_dragged() {
            window.on_mouse_event({
                let editor = self.editor.clone();
                move |_: &MouseUpEvent, phase, _, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        editor.scroll_manager.reset_scrollbar_dragging_state(cx);
                        cx.stop_propagation();
                    });
                }
            });
        } else {
            window.on_mouse_event({
                let editor = self.editor.clone();
                let scrollbars_layout = scrollbars_layout.clone();

                move |event: &MouseDownEvent, phase, window, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }
                    let Some((scrollbar_layout, axis)) = scrollbars_layout.get_hovered_axis(window)
                    else {
                        return;
                    };

                    let ScrollbarLayout {
                        hitbox,
                        visible_range,
                        text_unit_size,
                        ..
                    } = scrollbar_layout;

                    let thumb_bounds = scrollbar_layout.thumb_bounds();

                    editor.update(cx, |editor, cx| {
                        editor.scroll_manager.set_dragged_scrollbar_axis(axis, cx);

                        let event_position = event.position.along(axis);

                        if event_position < thumb_bounds.origin.along(axis)
                            || thumb_bounds.bottom_right().along(axis) < event_position
                        {
                            let center_position = ((event_position - hitbox.origin.along(axis))
                                / *text_unit_size)
                                .round() as u32;
                            let start_position = center_position.saturating_sub(
                                (visible_range.end - visible_range.start) as u32 / 2,
                            );

                            let position = editor
                                .scroll_position(cx)
                                .apply_along(axis, |_| start_position as f32);

                            editor.set_scroll_position(position, window, cx);
                        } else {
                            editor.scroll_manager.show_scrollbars(window, cx);
                        }

                        cx.stop_propagation();
                    });
                }
            });
        }
    }

    fn collect_fast_scrollbar_markers(
        &self,
        layout: &EditorLayout,
        scrollbar_layout: &ScrollbarLayout,
        cx: &mut App,
    ) -> Vec<PaintQuad> {
        const LIMIT: usize = 100;
        if !EditorSettings::get_global(cx).scrollbar.cursors || layout.cursors.len() > LIMIT {
            return vec![];
        }
        let cursor_ranges = layout
            .cursors
            .iter()
            .map(|(point, color)| ColoredRange {
                start: point.row(),
                end: point.row(),
                color: *color,
            })
            .collect_vec();
        scrollbar_layout.marker_quads_for_ranges(cursor_ranges, None)
    }

    fn refresh_slow_scrollbar_markers(
        &self,
        layout: &EditorLayout,
        scrollbar_layout: &ScrollbarLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.editor.update(cx, |editor, cx| {
            if !editor.is_singleton(cx)
                || !editor
                    .scrollbar_marker_state
                    .should_refresh(scrollbar_layout.hitbox.size)
            {
                return;
            }

            let scrollbar_layout = scrollbar_layout.clone();
            let background_highlights = editor.background_highlights.clone();
            let snapshot = layout.position_map.snapshot.clone();
            let theme = cx.theme().clone();
            let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;

            editor.scrollbar_marker_state.dirty = false;
            editor.scrollbar_marker_state.pending_refresh =
                Some(cx.spawn_in(window, async move |editor, cx| {
                    let scrollbar_size = scrollbar_layout.hitbox.size;
                    let scrollbar_markers = cx
                        .background_spawn(async move {
                            let max_point = snapshot.display_snapshot.buffer_snapshot.max_point();
                            let mut marker_quads = Vec::new();
                            if scrollbar_settings.git_diff {
                                let marker_row_ranges =
                                    snapshot.buffer_snapshot.diff_hunks().map(|hunk| {
                                        let start_display_row =
                                            MultiBufferPoint::new(hunk.row_range.start.0, 0)
                                                .to_display_point(&snapshot.display_snapshot)
                                                .row();
                                        let mut end_display_row =
                                            MultiBufferPoint::new(hunk.row_range.end.0, 0)
                                                .to_display_point(&snapshot.display_snapshot)
                                                .row();
                                        if end_display_row != start_display_row {
                                            end_display_row.0 -= 1;
                                        }
                                        let color = match &hunk.status().kind {
                                            DiffHunkStatusKind::Added => {
                                                theme.colors().version_control_added
                                            }
                                            DiffHunkStatusKind::Modified => {
                                                theme.colors().version_control_modified
                                            }
                                            DiffHunkStatusKind::Deleted => {
                                                theme.colors().version_control_deleted
                                            }
                                        };
                                        ColoredRange {
                                            start: start_display_row,
                                            end: end_display_row,
                                            color,
                                        }
                                    });

                                marker_quads.extend(
                                    scrollbar_layout
                                        .marker_quads_for_ranges(marker_row_ranges, Some(0)),
                                );
                            }

                            for (background_highlight_id, (_, background_ranges)) in
                                background_highlights.iter()
                            {
                                let is_search_highlights = *background_highlight_id
                                    == TypeId::of::<BufferSearchHighlights>();
                                let is_text_highlights = *background_highlight_id
                                    == TypeId::of::<SelectedTextHighlight>();
                                let is_symbol_occurrences = *background_highlight_id
                                    == TypeId::of::<DocumentHighlightRead>()
                                    || *background_highlight_id
                                        == TypeId::of::<DocumentHighlightWrite>();
                                if (is_search_highlights && scrollbar_settings.search_results)
                                    || (is_text_highlights && scrollbar_settings.selected_text)
                                    || (is_symbol_occurrences && scrollbar_settings.selected_symbol)
                                {
                                    let mut color = theme.status().info;
                                    if is_symbol_occurrences {
                                        color.fade_out(0.5);
                                    }
                                    let marker_row_ranges = background_ranges.iter().map(|range| {
                                        let display_start = range
                                            .start
                                            .to_display_point(&snapshot.display_snapshot);
                                        let display_end =
                                            range.end.to_display_point(&snapshot.display_snapshot);
                                        ColoredRange {
                                            start: display_start.row(),
                                            end: display_end.row(),
                                            color,
                                        }
                                    });
                                    marker_quads.extend(
                                        scrollbar_layout
                                            .marker_quads_for_ranges(marker_row_ranges, Some(1)),
                                    );
                                }
                            }

                            if scrollbar_settings.diagnostics != ScrollbarDiagnostics::None {
                                let diagnostics = snapshot
                                    .buffer_snapshot
                                    .diagnostics_in_range::<Point>(Point::zero()..max_point)
                                    // Don't show diagnostics the user doesn't care about
                                    .filter(|diagnostic| {
                                        match (
                                            scrollbar_settings.diagnostics,
                                            diagnostic.diagnostic.severity,
                                        ) {
                                            (ScrollbarDiagnostics::All, _) => true,
                                            (
                                                ScrollbarDiagnostics::Error,
                                                DiagnosticSeverity::ERROR,
                                            ) => true,
                                            (
                                                ScrollbarDiagnostics::Warning,
                                                DiagnosticSeverity::ERROR
                                                | DiagnosticSeverity::WARNING,
                                            ) => true,
                                            (
                                                ScrollbarDiagnostics::Information,
                                                DiagnosticSeverity::ERROR
                                                | DiagnosticSeverity::WARNING
                                                | DiagnosticSeverity::INFORMATION,
                                            ) => true,
                                            (_, _) => false,
                                        }
                                    })
                                    // We want to sort by severity, in order to paint the most severe diagnostics last.
                                    .sorted_by_key(|diagnostic| {
                                        std::cmp::Reverse(diagnostic.diagnostic.severity)
                                    });

                                let marker_row_ranges = diagnostics.into_iter().map(|diagnostic| {
                                    let start_display = diagnostic
                                        .range
                                        .start
                                        .to_display_point(&snapshot.display_snapshot);
                                    let end_display = diagnostic
                                        .range
                                        .end
                                        .to_display_point(&snapshot.display_snapshot);
                                    let color = match diagnostic.diagnostic.severity {
                                        DiagnosticSeverity::ERROR => theme.status().error,
                                        DiagnosticSeverity::WARNING => theme.status().warning,
                                        DiagnosticSeverity::INFORMATION => theme.status().info,
                                        _ => theme.status().hint,
                                    };
                                    ColoredRange {
                                        start: start_display.row(),
                                        end: end_display.row(),
                                        color,
                                    }
                                });
                                marker_quads.extend(
                                    scrollbar_layout
                                        .marker_quads_for_ranges(marker_row_ranges, Some(2)),
                                );
                            }

                            Arc::from(marker_quads)
                        })
                        .await;

                    editor.update(cx, |editor, cx| {
                        editor.scrollbar_marker_state.markers = scrollbar_markers;
                        editor.scrollbar_marker_state.scrollbar_size = scrollbar_size;
                        editor.scrollbar_marker_state.pending_refresh = None;
                        cx.notify();
                    })?;

                    Ok(())
                }));
        });
    }

    fn paint_highlighted_range(
        &self,
        range: Range<DisplayPoint>,
        color: Hsla,
        corner_radius: Pixels,
        line_end_overshoot: Pixels,
        layout: &EditorLayout,
        window: &mut Window,
    ) {
        let start_row = layout.visible_display_row_range.start;
        let end_row = layout.visible_display_row_range.end;
        if range.start != range.end {
            let row_range = if range.end.column() == 0 {
                cmp::max(range.start.row(), start_row)..cmp::min(range.end.row(), end_row)
            } else {
                cmp::max(range.start.row(), start_row)
                    ..cmp::min(range.end.row().next_row(), end_row)
            };

            let highlighted_range = HighlightedRange {
                color,
                line_height: layout.position_map.line_height,
                corner_radius,
                start_y: layout.content_origin.y
                    + row_range.start.as_f32() * layout.position_map.line_height
                    - layout.position_map.scroll_pixel_position.y,
                lines: row_range
                    .iter_rows()
                    .map(|row| {
                        let line_layout =
                            &layout.position_map.line_layouts[row.minus(start_row) as usize];
                        HighlightedRangeLine {
                            start_x: if row == range.start.row() {
                                layout.content_origin.x
                                    + line_layout.x_for_index(range.start.column() as usize)
                                    - layout.position_map.scroll_pixel_position.x
                            } else {
                                layout.content_origin.x
                                    - layout.position_map.scroll_pixel_position.x
                            },
                            end_x: if row == range.end.row() {
                                layout.content_origin.x
                                    + line_layout.x_for_index(range.end.column() as usize)
                                    - layout.position_map.scroll_pixel_position.x
                            } else {
                                layout.content_origin.x + line_layout.width + line_end_overshoot
                                    - layout.position_map.scroll_pixel_position.x
                            },
                        }
                    })
                    .collect(),
            };

            highlighted_range.paint(layout.position_map.text_hitbox.bounds, window);
        }
    }

    fn paint_inline_diagnostics(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        for mut inline_diagnostic in layout.inline_diagnostics.drain() {
            inline_diagnostic.1.paint(window, cx);
        }
    }

    fn paint_inline_blame(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        if let Some(mut inline_blame) = layout.inline_blame.take() {
            window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
                inline_blame.paint(window, cx);
            })
        }
    }

    fn paint_diff_hunk_controls(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        for mut diff_hunk_control in layout.diff_hunk_controls.drain(..) {
            diff_hunk_control.paint(window, cx);
        }
    }

    fn paint_blocks(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        for mut block in layout.blocks.drain(..) {
            if block.overlaps_gutter {
                block.element.paint(window, cx);
            } else {
                let mut bounds = layout.hitbox.bounds;
                bounds.origin.x += layout.gutter_hitbox.bounds.size.width;
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    block.element.paint(window, cx);
                })
            }
        }
    }

    fn paint_inline_completion_popover(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(inline_completion_popover) = layout.inline_completion_popover.as_mut() {
            inline_completion_popover.paint(window, cx);
        }
    }

    fn paint_mouse_context_menu(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(mouse_context_menu) = layout.mouse_context_menu.as_mut() {
            mouse_context_menu.paint(window, cx);
        }
    }

    fn paint_scroll_wheel_listener(
        &mut self,
        layout: &EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let hitbox = layout.hitbox.clone();
            let mut delta = ScrollDelta::default();

            // Set a minimum scroll_sensitivity of 0.01 to make sure the user doesn't
            // accidentally turn off their scrolling.
            let scroll_sensitivity = EditorSettings::get_global(cx).scroll_sensitivity.max(0.01);

            move |event: &ScrollWheelEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                    delta = delta.coalesce(event.delta);
                    editor.update(cx, |editor, cx| {
                        let position_map: &PositionMap = &position_map;

                        let line_height = position_map.line_height;
                        let max_glyph_width = position_map.em_width;
                        let (delta, axis) = match delta {
                            gpui::ScrollDelta::Pixels(mut pixels) => {
                                //Trackpad
                                let axis = position_map.snapshot.ongoing_scroll.filter(&mut pixels);
                                (pixels, axis)
                            }

                            gpui::ScrollDelta::Lines(lines) => {
                                //Not trackpad
                                let pixels =
                                    point(lines.x * max_glyph_width, lines.y * line_height);
                                (pixels, None)
                            }
                        };

                        let current_scroll_position = position_map.snapshot.scroll_position();
                        let x = (current_scroll_position.x * max_glyph_width
                            - (delta.x * scroll_sensitivity))
                            / max_glyph_width;
                        let y = (current_scroll_position.y * line_height
                            - (delta.y * scroll_sensitivity))
                            / line_height;
                        let mut scroll_position =
                            point(x, y).clamp(&point(0., 0.), &position_map.scroll_max);
                        let forbid_vertical_scroll = editor.scroll_manager.forbid_vertical_scroll();
                        if forbid_vertical_scroll {
                            scroll_position.y = current_scroll_position.y;
                        }

                        if scroll_position != current_scroll_position {
                            editor.scroll(scroll_position, axis, window, cx);
                            cx.stop_propagation();
                        } else if y < 0. {
                            // Due to clamping, we may fail to detect cases of overscroll to the top;
                            // We want the scroll manager to get an update in such cases and detect the change of direction
                            // on the next frame.
                            cx.notify();
                        }
                    });
                }
            }
        });
    }

    fn paint_mouse_listeners(&mut self, layout: &EditorLayout, window: &mut Window, cx: &mut App) {
        if !self.editor.read(cx).disable_scrolling {
            self.paint_scroll_wheel_listener(layout, window, cx);
        }

        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let diff_hunk_range =
                layout
                    .display_hunks
                    .iter()
                    .find_map(|(hunk, hunk_hitbox)| match hunk {
                        DisplayDiffHunk::Folded { .. } => None,
                        DisplayDiffHunk::Unfolded {
                            multi_buffer_range, ..
                        } => {
                            if hunk_hitbox
                                .as_ref()
                                .map(|hitbox| hitbox.is_hovered(window))
                                .unwrap_or(false)
                            {
                                Some(multi_buffer_range.clone())
                            } else {
                                None
                            }
                        }
                    });
            let line_numbers = layout.line_numbers.clone();

            move |event: &MouseDownEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    match event.button {
                        MouseButton::Left => editor.update(cx, |editor, cx| {
                            let pending_mouse_down = editor
                                .pending_mouse_down
                                .get_or_insert_with(Default::default)
                                .clone();

                            *pending_mouse_down.borrow_mut() = Some(event.clone());

                            Self::mouse_left_down(
                                editor,
                                event,
                                diff_hunk_range.clone(),
                                &position_map,
                                line_numbers.as_ref(),
                                window,
                                cx,
                            );
                        }),
                        MouseButton::Right => editor.update(cx, |editor, cx| {
                            Self::mouse_right_down(editor, event, &position_map, window, cx);
                        }),
                        MouseButton::Middle => editor.update(cx, |editor, cx| {
                            Self::mouse_middle_down(editor, event, &position_map, window, cx);
                        }),
                        _ => {}
                    };
                }
            }
        });

        window.on_mouse_event({
            let editor = self.editor.clone();
            let position_map = layout.position_map.clone();

            move |event: &MouseUpEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        Self::mouse_up(editor, event, &position_map, window, cx)
                    });
                }
            }
        });

        window.on_mouse_event({
            let editor = self.editor.clone();
            let position_map = layout.position_map.clone();
            let mut captured_mouse_down = None;

            move |event: &MouseUpEvent, phase, window, cx| match phase {
                // Clear the pending mouse down during the capture phase,
                // so that it happens even if another event handler stops
                // propagation.
                DispatchPhase::Capture => editor.update(cx, |editor, _cx| {
                    let pending_mouse_down = editor
                        .pending_mouse_down
                        .get_or_insert_with(Default::default)
                        .clone();

                    let mut pending_mouse_down = pending_mouse_down.borrow_mut();
                    if pending_mouse_down.is_some() && position_map.text_hitbox.is_hovered(window) {
                        captured_mouse_down = pending_mouse_down.take();
                        window.refresh();
                    }
                }),
                // Fire click handlers during the bubble phase.
                DispatchPhase::Bubble => editor.update(cx, |editor, cx| {
                    if let Some(mouse_down) = captured_mouse_down.take() {
                        let event = ClickEvent {
                            down: mouse_down,
                            up: event.clone(),
                        };
                        Self::click(editor, &event, &position_map, window, cx);
                    }
                }),
            }
        });

        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();

            move |event: &MouseMoveEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        if editor.hover_state.focused(window, cx) {
                            return;
                        }
                        if event.pressed_button == Some(MouseButton::Left)
                            || event.pressed_button == Some(MouseButton::Middle)
                        {
                            Self::mouse_dragged(editor, event, &position_map, window, cx)
                        }

                        Self::mouse_moved(editor, event, &position_map, window, cx)
                    });
                }
            }
        });
    }

    fn scrollbar_left(&self, bounds: &Bounds<Pixels>) -> Pixels {
        bounds.top_right().x - self.style.scrollbar_width
    }

    fn column_pixels(&self, column: usize, window: &mut Window, _: &mut App) -> Pixels {
        let style = &self.style;
        let font_size = style.text.font_size.to_pixels(window.rem_size());
        let layout = window
            .text_system()
            .shape_line(
                SharedString::from(" ".repeat(column)),
                font_size,
                &[TextRun {
                    len: column,
                    font: style.text.font(),
                    color: Hsla::default(),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
            )
            .unwrap();

        layout.width
    }

    fn max_line_number_width(
        &self,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Pixels {
        let digit_count = snapshot.widest_line_number().ilog10() + 1;
        self.column_pixels(digit_count as usize, window, cx)
    }

    fn shape_line_number(
        &self,
        text: SharedString,
        color: Hsla,
        window: &mut Window,
    ) -> anyhow::Result<ShapedLine> {
        let run = TextRun {
            len: text.len(),
            font: self.style.text.font(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        window.text_system().shape_line(
            text,
            self.style.text.font_size.to_pixels(window.rem_size()),
            &[run],
        )
    }

    fn diff_hunk_hollow(status: DiffHunkStatus, cx: &mut App) -> bool {
        let unstaged = status.has_secondary_hunk();
        let unstaged_hollow = ProjectSettings::get_global(cx)
            .git
            .hunk_style
            .map_or(false, |style| {
                matches!(style, GitHunkStyleSetting::UnstagedHollow)
            });

        unstaged == unstaged_hollow
    }
}

fn header_jump_data(
    snapshot: &EditorSnapshot,
    block_row_start: DisplayRow,
    height: u32,
    for_excerpt: &ExcerptInfo,
) -> JumpData {
    let range = &for_excerpt.range;
    let buffer = &for_excerpt.buffer;
    let jump_anchor = range.primary.start;

    let excerpt_start = range.context.start;
    let jump_position = language::ToPoint::to_point(&jump_anchor, buffer);
    let rows_from_excerpt_start = if jump_anchor == excerpt_start {
        0
    } else {
        let excerpt_start_point = language::ToPoint::to_point(&excerpt_start, buffer);
        jump_position.row.saturating_sub(excerpt_start_point.row)
    };

    let line_offset_from_top = (block_row_start.0 + height + rows_from_excerpt_start)
        .saturating_sub(
            snapshot
                .scroll_anchor
                .scroll_position(&snapshot.display_snapshot)
                .y as u32,
        );

    JumpData::MultiBufferPoint {
        excerpt_id: for_excerpt.id,
        anchor: jump_anchor,
        position: jump_position,
        line_offset_from_top,
    }
}

pub struct AcceptEditPredictionBinding(pub(crate) Option<gpui::KeyBinding>);

impl AcceptEditPredictionBinding {
    pub fn keystroke(&self) -> Option<&Keystroke> {
        if let Some(binding) = self.0.as_ref() {
            match &binding.keystrokes() {
                [keystroke] => Some(keystroke),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn prepaint_gutter_button(
    button: IconButton,
    row: DisplayRow,
    line_height: Pixels,
    gutter_dimensions: &GutterDimensions,
    scroll_pixel_position: gpui::Point<Pixels>,
    gutter_hitbox: &Hitbox,
    display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let mut button = button.into_any_element();

    let available_space = size(
        AvailableSpace::MinContent,
        AvailableSpace::Definite(line_height),
    );
    let indicator_size = button.layout_as_root(available_space, window, cx);

    let blame_width = gutter_dimensions.git_blame_entries_width;
    let gutter_width = display_hunks
        .binary_search_by(|(hunk, _)| match hunk {
            DisplayDiffHunk::Folded { display_row } => display_row.cmp(&row),
            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => {
                if display_row_range.end <= row {
                    Ordering::Less
                } else if display_row_range.start > row {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        })
        .ok()
        .and_then(|ix| Some(display_hunks[ix].1.as_ref()?.size.width));
    let left_offset = blame_width.max(gutter_width).unwrap_or_default();

    let mut x = left_offset;
    let available_width = gutter_dimensions.margin + gutter_dimensions.left_padding
        - indicator_size.width
        - left_offset;
    x += available_width / 2.;

    let mut y = row.as_f32() * line_height - scroll_pixel_position.y;
    y += (line_height - indicator_size.height) / 2.;

    button.prepaint_as_root(
        gutter_hitbox.origin + point(x, y),
        available_space,
        window,
        cx,
    );
    button
}

fn render_inline_blame_entry(
    blame_entry: BlameEntry,
    style: &EditorStyle,
    cx: &mut App,
) -> Option<AnyElement> {
    let renderer = cx.global::<GlobalBlameRenderer>().0.clone();
    renderer.render_inline_blame_entry(&style.text, blame_entry, cx)
}

fn render_blame_entry_popover(
    blame_entry: BlameEntry,
    scroll_handle: ScrollHandle,
    commit_message: Option<ParsedCommitMessage>,
    markdown: Entity<Markdown>,
    workspace: WeakEntity<Workspace>,
    blame: &Entity<GitBlame>,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    let renderer = cx.global::<GlobalBlameRenderer>().0.clone();
    let blame = blame.read(cx);
    let repository = blame.repository(cx)?.clone();
    renderer.render_blame_entry_popover(
        blame_entry,
        scroll_handle,
        commit_message,
        markdown,
        repository,
        workspace,
        window,
        cx,
    )
}

fn render_blame_entry(
    ix: usize,
    blame: &Entity<GitBlame>,
    blame_entry: BlameEntry,
    style: &EditorStyle,
    last_used_color: &mut Option<(PlayerColor, Oid)>,
    editor: Entity<Editor>,
    workspace: Entity<Workspace>,
    renderer: Arc<dyn BlameRenderer>,
    cx: &mut App,
) -> Option<AnyElement> {
    let mut sha_color = cx
        .theme()
        .players()
        .color_for_participant(blame_entry.sha.into());

    // If the last color we used is the same as the one we get for this line, but
    // the commit SHAs are different, then we try again to get a different color.
    match *last_used_color {
        Some((color, sha)) if sha != blame_entry.sha && color.cursor == sha_color.cursor => {
            let index: u32 = blame_entry.sha.into();
            sha_color = cx.theme().players().color_for_participant(index + 1);
        }
        _ => {}
    };
    last_used_color.replace((sha_color, blame_entry.sha));

    let blame = blame.read(cx);
    let details = blame.details_for_entry(&blame_entry);
    let repository = blame.repository(cx)?;
    renderer.render_blame_entry(
        &style.text,
        blame_entry,
        details,
        repository,
        workspace.downgrade(),
        editor,
        ix,
        sha_color.cursor,
        cx,
    )
}

#[derive(Debug)]
pub(crate) struct LineWithInvisibles {
    fragments: SmallVec<[LineFragment; 1]>,
    invisibles: Vec<Invisible>,
    len: usize,
    pub(crate) width: Pixels,
    font_size: Pixels,
}

#[allow(clippy::large_enum_variant)]
enum LineFragment {
    Text(ShapedLine),
    Element {
        id: FoldId,
        element: Option<AnyElement>,
        size: Size<Pixels>,
        len: usize,
    },
}

impl fmt::Debug for LineFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineFragment::Text(shaped_line) => f.debug_tuple("Text").field(shaped_line).finish(),
            LineFragment::Element { size, len, .. } => f
                .debug_struct("Element")
                .field("size", size)
                .field("len", len)
                .finish(),
        }
    }
}

impl LineWithInvisibles {
    fn from_chunks<'a>(
        chunks: impl Iterator<Item = HighlightedChunk<'a>>,
        editor_style: &EditorStyle,
        max_line_len: usize,
        max_line_count: usize,
        editor_mode: EditorMode,
        text_width: Pixels,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Self> {
        let text_style = &editor_style.text;
        let mut layouts = Vec::with_capacity(max_line_count);
        let mut fragments: SmallVec<[LineFragment; 1]> = SmallVec::new();
        let mut line = String::new();
        let mut invisibles = Vec::new();
        let mut width = Pixels::ZERO;
        let mut len = 0;
        let mut styles = Vec::new();
        let mut non_whitespace_added = false;
        let mut row = 0;
        let mut line_exceeded_max_len = false;
        let font_size = text_style.font_size.to_pixels(window.rem_size());

        let ellipsis = SharedString::from("⋯");

        for highlighted_chunk in chunks.chain([HighlightedChunk {
            text: "\n",
            style: None,
            is_tab: false,
            replacement: None,
        }]) {
            if let Some(replacement) = highlighted_chunk.replacement {
                if !line.is_empty() {
                    let shaped_line = window
                        .text_system()
                        .shape_line(line.clone().into(), font_size, &styles)
                        .unwrap();
                    width += shaped_line.width;
                    len += shaped_line.len;
                    fragments.push(LineFragment::Text(shaped_line));
                    line.clear();
                    styles.clear();
                }

                match replacement {
                    ChunkReplacement::Renderer(renderer) => {
                        let available_width = if renderer.constrain_width {
                            let chunk = if highlighted_chunk.text == ellipsis.as_ref() {
                                ellipsis.clone()
                            } else {
                                SharedString::from(Arc::from(highlighted_chunk.text))
                            };
                            let shaped_line = window
                                .text_system()
                                .shape_line(
                                    chunk,
                                    font_size,
                                    &[text_style.to_run(highlighted_chunk.text.len())],
                                )
                                .unwrap();
                            AvailableSpace::Definite(shaped_line.width)
                        } else {
                            AvailableSpace::MinContent
                        };

                        let mut element = (renderer.render)(&mut ChunkRendererContext {
                            context: cx,
                            window,
                            max_width: text_width,
                        });
                        let line_height = text_style.line_height_in_pixels(window.rem_size());
                        let size = element.layout_as_root(
                            size(available_width, AvailableSpace::Definite(line_height)),
                            window,
                            cx,
                        );

                        width += size.width;
                        len += highlighted_chunk.text.len();
                        fragments.push(LineFragment::Element {
                            id: renderer.id,
                            element: Some(element),
                            size,
                            len: highlighted_chunk.text.len(),
                        });
                    }
                    ChunkReplacement::Str(x) => {
                        let text_style = if let Some(style) = highlighted_chunk.style {
                            Cow::Owned(text_style.clone().highlight(style))
                        } else {
                            Cow::Borrowed(text_style)
                        };

                        let run = TextRun {
                            len: x.len(),
                            font: text_style.font(),
                            color: text_style.color,
                            background_color: text_style.background_color,
                            underline: text_style.underline,
                            strikethrough: text_style.strikethrough,
                        };
                        let line_layout = window
                            .text_system()
                            .shape_line(x, font_size, &[run])
                            .unwrap()
                            .with_len(highlighted_chunk.text.len());

                        width += line_layout.width;
                        len += highlighted_chunk.text.len();
                        fragments.push(LineFragment::Text(line_layout))
                    }
                }
            } else {
                for (ix, mut line_chunk) in highlighted_chunk.text.split('\n').enumerate() {
                    if ix > 0 {
                        let shaped_line = window
                            .text_system()
                            .shape_line(line.clone().into(), font_size, &styles)
                            .unwrap();
                        width += shaped_line.width;
                        len += shaped_line.len;
                        fragments.push(LineFragment::Text(shaped_line));
                        layouts.push(Self {
                            width: mem::take(&mut width),
                            len: mem::take(&mut len),
                            fragments: mem::take(&mut fragments),
                            invisibles: std::mem::take(&mut invisibles),
                            font_size,
                        });

                        line.clear();
                        styles.clear();
                        row += 1;
                        line_exceeded_max_len = false;
                        non_whitespace_added = false;
                        if row == max_line_count {
                            return layouts;
                        }
                    }

                    if !line_chunk.is_empty() && !line_exceeded_max_len {
                        let text_style = if let Some(style) = highlighted_chunk.style {
                            Cow::Owned(text_style.clone().highlight(style))
                        } else {
                            Cow::Borrowed(text_style)
                        };

                        if line.len() + line_chunk.len() > max_line_len {
                            let mut chunk_len = max_line_len - line.len();
                            while !line_chunk.is_char_boundary(chunk_len) {
                                chunk_len -= 1;
                            }
                            line_chunk = &line_chunk[..chunk_len];
                            line_exceeded_max_len = true;
                        }

                        styles.push(TextRun {
                            len: line_chunk.len(),
                            font: text_style.font(),
                            color: text_style.color,
                            background_color: text_style.background_color,
                            underline: text_style.underline,
                            strikethrough: text_style.strikethrough,
                        });

                        if editor_mode.is_full() {
                            // Line wrap pads its contents with fake whitespaces,
                            // avoid printing them
                            let is_soft_wrapped = is_row_soft_wrapped(row);
                            if highlighted_chunk.is_tab {
                                if non_whitespace_added || !is_soft_wrapped {
                                    invisibles.push(Invisible::Tab {
                                        line_start_offset: line.len(),
                                        line_end_offset: line.len() + line_chunk.len(),
                                    });
                                }
                            } else {
                                invisibles.extend(line_chunk.char_indices().filter_map(
                                    |(index, c)| {
                                        let is_whitespace = c.is_whitespace();
                                        non_whitespace_added |= !is_whitespace;
                                        if is_whitespace
                                            && (non_whitespace_added || !is_soft_wrapped)
                                        {
                                            Some(Invisible::Whitespace {
                                                line_offset: line.len() + index,
                                            })
                                        } else {
                                            None
                                        }
                                    },
                                ))
                            }
                        }

                        line.push_str(line_chunk);
                    }
                }
            }
        }

        layouts
    }

    fn prepaint(
        &mut self,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        row: DisplayRow,
        content_origin: gpui::Point<Pixels>,
        line_elements: &mut SmallVec<[AnyElement; 1]>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_y = line_height * (row.as_f32() - scroll_pixel_position.y / line_height);
        let mut fragment_origin = content_origin + gpui::point(-scroll_pixel_position.x, line_y);
        for fragment in &mut self.fragments {
            match fragment {
                LineFragment::Text(line) => {
                    fragment_origin.x += line.width;
                }
                LineFragment::Element { element, size, .. } => {
                    let mut element = element
                        .take()
                        .expect("you can't prepaint LineWithInvisibles twice");

                    // Center the element vertically within the line.
                    let mut element_origin = fragment_origin;
                    element_origin.y += (line_height - size.height) / 2.;
                    element.prepaint_at(element_origin, window, cx);
                    line_elements.push(element);

                    fragment_origin.x += size.width;
                }
            }
        }
    }

    fn draw(
        &self,
        layout: &EditorLayout,
        row: DisplayRow,
        content_origin: gpui::Point<Pixels>,
        whitespace_setting: ShowWhitespaceSetting,
        selection_ranges: &[Range<DisplayPoint>],
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_height = layout.position_map.line_height;
        let line_y = line_height
            * (row.as_f32() - layout.position_map.scroll_pixel_position.y / line_height);

        let mut fragment_origin =
            content_origin + gpui::point(-layout.position_map.scroll_pixel_position.x, line_y);

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(line) => {
                    line.paint(fragment_origin, line_height, window, cx)
                        .log_err();
                    fragment_origin.x += line.width;
                }
                LineFragment::Element { size, .. } => {
                    fragment_origin.x += size.width;
                }
            }
        }

        self.draw_invisibles(
            selection_ranges,
            layout,
            content_origin,
            line_y,
            row,
            line_height,
            whitespace_setting,
            window,
            cx,
        );
    }

    fn draw_background(
        &self,
        layout: &EditorLayout,
        row: DisplayRow,
        content_origin: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_height = layout.position_map.line_height;
        let line_y = line_height
            * (row.as_f32() - layout.position_map.scroll_pixel_position.y / line_height);

        let mut fragment_origin =
            content_origin + gpui::point(-layout.position_map.scroll_pixel_position.x, line_y);

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(line) => {
                    line.paint_background(fragment_origin, line_height, window, cx)
                        .log_err();
                    fragment_origin.x += line.width;
                }
                LineFragment::Element { size, .. } => {
                    fragment_origin.x += size.width;
                }
            }
        }
    }

    fn draw_invisibles(
        &self,
        selection_ranges: &[Range<DisplayPoint>],
        layout: &EditorLayout,
        content_origin: gpui::Point<Pixels>,
        line_y: Pixels,
        row: DisplayRow,
        line_height: Pixels,
        whitespace_setting: ShowWhitespaceSetting,
        window: &mut Window,
        cx: &mut App,
    ) {
        let extract_whitespace_info = |invisible: &Invisible| {
            let (token_offset, token_end_offset, invisible_symbol) = match invisible {
                Invisible::Tab {
                    line_start_offset,
                    line_end_offset,
                } => (*line_start_offset, *line_end_offset, &layout.tab_invisible),
                Invisible::Whitespace { line_offset } => {
                    (*line_offset, line_offset + 1, &layout.space_invisible)
                }
            };

            let x_offset = self.x_for_index(token_offset);
            let invisible_offset =
                (layout.position_map.em_width - invisible_symbol.width).max(Pixels::ZERO) / 2.0;
            let origin = content_origin
                + gpui::point(
                    x_offset + invisible_offset - layout.position_map.scroll_pixel_position.x,
                    line_y,
                );

            (
                [token_offset, token_end_offset],
                Box::new(move |window: &mut Window, cx: &mut App| {
                    invisible_symbol
                        .paint(origin, line_height, window, cx)
                        .log_err();
                }),
            )
        };

        let invisible_iter = self.invisibles.iter().map(extract_whitespace_info);
        match whitespace_setting {
            ShowWhitespaceSetting::None => (),
            ShowWhitespaceSetting::All => invisible_iter.for_each(|(_, paint)| paint(window, cx)),
            ShowWhitespaceSetting::Selection => invisible_iter.for_each(|([start, _], paint)| {
                let invisible_point = DisplayPoint::new(row, start as u32);
                if !selection_ranges
                    .iter()
                    .any(|region| region.start <= invisible_point && invisible_point < region.end)
                {
                    return;
                }

                paint(window, cx);
            }),

            // For a whitespace to be on a boundary, any of the following conditions need to be met:
            // - It is a tab
            // - It is adjacent to an edge (start or end)
            // - It is adjacent to a whitespace (left or right)
            ShowWhitespaceSetting::Boundary => {
                // We'll need to keep track of the last invisible we've seen and then check if we are adjacent to it for some of
                // the above cases.
                // Note: We zip in the original `invisibles` to check for tab equality
                let mut last_seen: Option<(bool, usize, Box<dyn Fn(&mut Window, &mut App)>)> = None;
                for (([start, end], paint), invisible) in
                    invisible_iter.zip_eq(self.invisibles.iter())
                {
                    let should_render = match (&last_seen, invisible) {
                        (_, Invisible::Tab { .. }) => true,
                        (Some((_, last_end, _)), _) => *last_end == start,
                        _ => false,
                    };

                    if should_render || start == 0 || end == self.len {
                        paint(window, cx);

                        // Since we are scanning from the left, we will skip over the first available whitespace that is part
                        // of a boundary between non-whitespace segments, so we correct by manually redrawing it if needed.
                        if let Some((should_render_last, last_end, paint_last)) = last_seen {
                            // Note that we need to make sure that the last one is actually adjacent
                            if !should_render_last && last_end == start {
                                paint_last(window, cx);
                            }
                        }
                    }

                    // Manually render anything within a selection
                    let invisible_point = DisplayPoint::new(row, start as u32);
                    if selection_ranges.iter().any(|region| {
                        region.start <= invisible_point && invisible_point < region.end
                    }) {
                        paint(window, cx);
                    }

                    last_seen = Some((should_render, end, paint));
                }
            }
        }
    }

    pub fn x_for_index(&self, index: usize) -> Pixels {
        let mut fragment_start_x = Pixels::ZERO;
        let mut fragment_start_index = 0;

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(shaped_line) => {
                    let fragment_end_index = fragment_start_index + shaped_line.len;
                    if index < fragment_end_index {
                        return fragment_start_x
                            + shaped_line.x_for_index(index - fragment_start_index);
                    }
                    fragment_start_x += shaped_line.width;
                    fragment_start_index = fragment_end_index;
                }
                LineFragment::Element { len, size, .. } => {
                    let fragment_end_index = fragment_start_index + len;
                    if index < fragment_end_index {
                        return fragment_start_x;
                    }
                    fragment_start_x += size.width;
                    fragment_start_index = fragment_end_index;
                }
            }
        }

        fragment_start_x
    }

    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        let mut fragment_start_x = Pixels::ZERO;
        let mut fragment_start_index = 0;

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(shaped_line) => {
                    let fragment_end_x = fragment_start_x + shaped_line.width;
                    if x < fragment_end_x {
                        return Some(
                            fragment_start_index + shaped_line.index_for_x(x - fragment_start_x)?,
                        );
                    }
                    fragment_start_x = fragment_end_x;
                    fragment_start_index += shaped_line.len;
                }
                LineFragment::Element { len, size, .. } => {
                    let fragment_end_x = fragment_start_x + size.width;
                    if x < fragment_end_x {
                        return Some(fragment_start_index);
                    }
                    fragment_start_index += len;
                    fragment_start_x = fragment_end_x;
                }
            }
        }

        None
    }

    pub fn font_id_for_index(&self, index: usize) -> Option<FontId> {
        let mut fragment_start_index = 0;

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(shaped_line) => {
                    let fragment_end_index = fragment_start_index + shaped_line.len;
                    if index < fragment_end_index {
                        return shaped_line.font_id_for_index(index - fragment_start_index);
                    }
                    fragment_start_index = fragment_end_index;
                }
                LineFragment::Element { len, .. } => {
                    let fragment_end_index = fragment_start_index + len;
                    if index < fragment_end_index {
                        return None;
                    }
                    fragment_start_index = fragment_end_index;
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Invisible {
    /// A tab character
    ///
    /// A tab character is internally represented by spaces (configured by the user's tab width)
    /// aligned to the nearest column, so it's necessary to store the start and end offset for
    /// adjacency checks.
    Tab {
        line_start_offset: usize,
        line_end_offset: usize,
    },
    Whitespace {
        line_offset: usize,
    },
}

impl EditorElement {
    /// Returns the rem size to use when rendering the [`EditorElement`].
    ///
    /// This allows UI elements to scale based on the `buffer_font_size`.
    fn rem_size(&self, cx: &mut App) -> Option<Pixels> {
        match self.editor.read(cx).mode {
            EditorMode::Full {
                scale_ui_elements_with_buffer_font_size,
                ..
            } => {
                if !scale_ui_elements_with_buffer_font_size {
                    return None;
                }
                let buffer_font_size = self.style.text.font_size;
                match buffer_font_size {
                    AbsoluteLength::Pixels(pixels) => {
                        let rem_size_scale = {
                            // Our default UI font size is 14px on a 16px base scale.
                            // This means the default UI font size is 0.875rems.
                            let default_font_size_scale = 14. / ui::BASE_REM_SIZE_IN_PX;

                            // We then determine the delta between a single rem and the default font
                            // size scale.
                            let default_font_size_delta = 1. - default_font_size_scale;

                            // Finally, we add this delta to 1rem to get the scale factor that
                            // should be used to scale up the UI.
                            1. + default_font_size_delta
                        };

                        Some(pixels * rem_size_scale)
                    }
                    AbsoluteLength::Rems(rems) => {
                        Some(rems.to_pixels(ui::BASE_REM_SIZE_IN_PX.into()))
                    }
                }
            }
            // We currently use single-line and auto-height editors in UI contexts,
            // so we don't want to scale everything with the buffer font size, as it
            // ends up looking off.
            EditorMode::SingleLine { .. } | EditorMode::AutoHeight { .. } => None,
        }
    }
}

impl Element for EditorElement {
    type RequestLayoutState = ();
    type PrepaintState = EditorLayout;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let rem_size = self.rem_size(cx);
        window.with_rem_size(rem_size, |window| {
            self.editor.update(cx, |editor, cx| {
                editor.set_style(self.style.clone(), window, cx);

                let layout_id = match editor.mode {
                    EditorMode::SingleLine { auto_width } => {
                        let rem_size = window.rem_size();

                        let height = self.style.text.line_height_in_pixels(rem_size);
                        if auto_width {
                            let editor_handle = cx.entity().clone();
                            let style = self.style.clone();
                            window.request_measured_layout(
                                Style::default(),
                                move |_, _, window, cx| {
                                    let editor_snapshot = editor_handle
                                        .update(cx, |editor, cx| editor.snapshot(window, cx));
                                    let line = Self::layout_lines(
                                        DisplayRow(0)..DisplayRow(1),
                                        &editor_snapshot,
                                        &style,
                                        px(f32::MAX),
                                        |_| false, // Single lines never soft wrap
                                        window,
                                        cx,
                                    )
                                    .pop()
                                    .unwrap();

                                    let font_id =
                                        window.text_system().resolve_font(&style.text.font());
                                    let font_size =
                                        style.text.font_size.to_pixels(window.rem_size());
                                    let em_width =
                                        window.text_system().em_width(font_id, font_size).unwrap();

                                    size(line.width + em_width, height)
                                },
                            )
                        } else {
                            let mut style = Style::default();
                            style.size.height = height.into();
                            style.size.width = relative(1.).into();
                            window.request_layout(style, None, cx)
                        }
                    }
                    EditorMode::AutoHeight { max_lines } => {
                        let editor_handle = cx.entity().clone();
                        let max_line_number_width =
                            self.max_line_number_width(&editor.snapshot(window, cx), window, cx);
                        window.request_measured_layout(
                            Style::default(),
                            move |known_dimensions, available_space, window, cx| {
                                editor_handle
                                    .update(cx, |editor, cx| {
                                        compute_auto_height_layout(
                                            editor,
                                            max_lines,
                                            max_line_number_width,
                                            known_dimensions,
                                            available_space.width,
                                            window,
                                            cx,
                                        )
                                    })
                                    .unwrap_or_default()
                            },
                        )
                    }
                    EditorMode::Full {
                        sized_by_content, ..
                    } => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        if sized_by_content {
                            let snapshot = editor.snapshot(window, cx);
                            let line_height =
                                self.style.text.line_height_in_pixels(window.rem_size());
                            let scroll_height =
                                (snapshot.max_point().row().next_row().0 as f32) * line_height;
                            style.size.height = scroll_height.into();
                        } else {
                            style.size.height = relative(1.).into();
                        }
                        window.request_layout(style, None, cx)
                    }
                };

                (layout_id, ())
            })
        })
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };
        let focus_handle = self.editor.focus_handle(cx);
        window.set_view_id(self.editor.entity_id());
        window.set_focus_handle(&focus_handle, cx);

        let rem_size = self.rem_size(cx);
        window.with_rem_size(rem_size, |window| {
            window.with_text_style(Some(text_style), |window| {
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    let (mut snapshot, is_read_only) = self.editor.update(cx, |editor, cx| {
                        (editor.snapshot(window, cx), editor.read_only(cx))
                    });
                    let style = self.style.clone();

                    let font_id = window.text_system().resolve_font(&style.text.font());
                    let font_size = style.text.font_size.to_pixels(window.rem_size());
                    let line_height = style.text.line_height_in_pixels(window.rem_size());
                    let em_width = window.text_system().em_width(font_id, font_size).unwrap();
                    let em_advance = window.text_system().em_advance(font_id, font_size).unwrap();

                    let glyph_grid_cell = size(em_width, line_height);

                    let gutter_dimensions = snapshot
                        .gutter_dimensions(
                            font_id,
                            font_size,
                            self.max_line_number_width(&snapshot, window, cx),
                            cx,
                        )
                        .unwrap_or_default();
                    let text_width = bounds.size.width - gutter_dimensions.width;

                    let editor_width =
                        text_width - gutter_dimensions.margin - em_width - style.scrollbar_width;

                    snapshot = self.editor.update(cx, |editor, cx| {
                        editor.last_bounds = Some(bounds);
                        editor.gutter_dimensions = gutter_dimensions;
                        editor.set_visible_line_count(bounds.size.height / line_height, window, cx);

                        if matches!(editor.mode, EditorMode::AutoHeight { .. }) {
                            snapshot
                        } else {
                            let wrap_width = match editor.soft_wrap_mode(cx) {
                                SoftWrap::GitDiff => None,
                                SoftWrap::None => Some((MAX_LINE_LEN / 2) as f32 * em_advance),
                                SoftWrap::EditorWidth => Some(editor_width),
                                SoftWrap::Column(column) => Some(column as f32 * em_advance),
                                SoftWrap::Bounded(column) => {
                                    Some(editor_width.min(column as f32 * em_advance))
                                }
                            };

                            if editor.set_wrap_width(wrap_width.map(|w| w.ceil()), cx) {
                                editor.snapshot(window, cx)
                            } else {
                                snapshot
                            }
                        }
                    });

                    let wrap_guides = self
                        .editor
                        .read(cx)
                        .wrap_guides(cx)
                        .iter()
                        .map(|(guide, active)| (self.column_pixels(*guide, window, cx), *active))
                        .collect::<SmallVec<[_; 2]>>();

                    let hitbox = window.insert_hitbox(bounds, false);
                    let gutter_hitbox =
                        window.insert_hitbox(gutter_bounds(bounds, gutter_dimensions), false);
                    let text_hitbox = window.insert_hitbox(
                        Bounds {
                            origin: gutter_hitbox.top_right(),
                            size: size(text_width, bounds.size.height),
                        },
                        false,
                    );

                    // Offset the content_bounds from the text_bounds by the gutter margin (which
                    // is roughly half a character wide) to make hit testing work more like how we want.
                    let content_offset = point(gutter_dimensions.margin, Pixels::ZERO);
                    let content_origin = text_hitbox.origin + content_offset;

                    let editor_text_bounds =
                        Bounds::from_corners(content_origin, bounds.bottom_right());

                    let height_in_lines = editor_text_bounds.size.height / line_height;

                    let max_row = snapshot.max_point().row().as_f32();

                    // The max scroll position for the top of the window
                    let max_scroll_top = if matches!(
                        snapshot.mode,
                        EditorMode::AutoHeight { .. } | EditorMode::SingleLine { .. }
                    ) {
                        (max_row - height_in_lines + 1.).max(0.)
                    } else {
                        let settings = EditorSettings::get_global(cx);
                        match settings.scroll_beyond_last_line {
                            ScrollBeyondLastLine::OnePage => max_row,
                            ScrollBeyondLastLine::Off => (max_row - height_in_lines + 1.).max(0.),
                            ScrollBeyondLastLine::VerticalScrollMargin => {
                                (max_row - height_in_lines + 1. + settings.vertical_scroll_margin)
                                    .max(0.)
                            }
                        }
                    };

                    // TODO: Autoscrolling for both axes
                    let mut autoscroll_request = None;
                    let mut autoscroll_containing_element = false;
                    let mut autoscroll_horizontally = false;
                    self.editor.update(cx, |editor, cx| {
                        autoscroll_request = editor.autoscroll_request();
                        autoscroll_containing_element =
                            autoscroll_request.is_some() || editor.has_pending_selection();
                        // TODO: Is this horizontal or vertical?!
                        autoscroll_horizontally = editor.autoscroll_vertically(
                            bounds,
                            line_height,
                            max_scroll_top,
                            window,
                            cx,
                        );
                        snapshot = editor.snapshot(window, cx);
                    });

                    let mut scroll_position = snapshot.scroll_position();
                    // The scroll position is a fractional point, the whole number of which represents
                    // the top of the window in terms of display rows.
                    let start_row = DisplayRow(scroll_position.y as u32);
                    let max_row = snapshot.max_point().row();
                    let end_row = cmp::min(
                        (scroll_position.y + height_in_lines).ceil() as u32,
                        max_row.next_row().0,
                    );
                    let end_row = DisplayRow(end_row);

                    let row_infos = snapshot
                        .row_infos(start_row)
                        .take((start_row..end_row).len())
                        .collect::<Vec<RowInfo>>();
                    let is_row_soft_wrapped = |row: usize| {
                        row_infos
                            .get(row)
                            .map_or(true, |info| info.buffer_row.is_none())
                    };

                    let start_anchor = if start_row == Default::default() {
                        Anchor::min()
                    } else {
                        snapshot.buffer_snapshot.anchor_before(
                            DisplayPoint::new(start_row, 0).to_offset(&snapshot, Bias::Left),
                        )
                    };
                    let end_anchor = if end_row > max_row {
                        Anchor::max()
                    } else {
                        snapshot.buffer_snapshot.anchor_before(
                            DisplayPoint::new(end_row, 0).to_offset(&snapshot, Bias::Right),
                        )
                    };

                    let mut highlighted_rows = self
                        .editor
                        .update(cx, |editor, cx| editor.highlighted_display_rows(window, cx));

                    let is_light = cx.theme().appearance().is_light();

                    for (ix, row_info) in row_infos.iter().enumerate() {
                        let Some(diff_status) = row_info.diff_status else {
                            continue;
                        };

                        let background_color = match diff_status.kind {
                            DiffHunkStatusKind::Added => cx.theme().colors().version_control_added,
                            DiffHunkStatusKind::Deleted => {
                                cx.theme().colors().version_control_deleted
                            }
                            DiffHunkStatusKind::Modified => {
                                debug_panic!("modified diff status for row info");
                                continue;
                            }
                        };

                        let hunk_opacity = if is_light { 0.16 } else { 0.12 };

                        let hollow_highlight = LineHighlight {
                            background: (background_color.opacity(if is_light {
                                0.08
                            } else {
                                0.06
                            }))
                            .into(),
                            border: Some(if is_light {
                                background_color.opacity(0.48)
                            } else {
                                background_color.opacity(0.36)
                            }),
                            include_gutter: true,
                            type_id: None,
                        };

                        let filled_highlight = LineHighlight {
                            background: solid_background(background_color.opacity(hunk_opacity)),
                            border: None,
                            include_gutter: true,
                            type_id: None,
                        };

                        let background = if Self::diff_hunk_hollow(diff_status, cx) {
                            hollow_highlight
                        } else {
                            filled_highlight
                        };

                        highlighted_rows
                            .entry(start_row + DisplayRow(ix as u32))
                            .or_insert(background);
                    }

                    let highlighted_ranges = self.editor.read(cx).background_highlights_in_range(
                        start_anchor..end_anchor,
                        &snapshot.display_snapshot,
                        cx.theme().colors(),
                    );
                    let highlighted_gutter_ranges =
                        self.editor.read(cx).gutter_highlights_in_range(
                            start_anchor..end_anchor,
                            &snapshot.display_snapshot,
                            cx,
                        );

                    let redacted_ranges = self.editor.read(cx).redacted_ranges(
                        start_anchor..end_anchor,
                        &snapshot.display_snapshot,
                        cx,
                    );

                    let (local_selections, selected_buffer_ids): (
                        Vec<Selection<Point>>,
                        Vec<BufferId>,
                    ) = self.editor.update(cx, |editor, cx| {
                        let all_selections = editor.selections.all::<Point>(cx);
                        let selected_buffer_ids = if editor.is_singleton(cx) {
                            Vec::new()
                        } else {
                            let mut selected_buffer_ids = Vec::with_capacity(all_selections.len());

                            for selection in all_selections {
                                for buffer_id in snapshot
                                    .buffer_snapshot
                                    .buffer_ids_for_range(selection.range())
                                {
                                    if selected_buffer_ids.last() != Some(&buffer_id) {
                                        selected_buffer_ids.push(buffer_id);
                                    }
                                }
                            }

                            selected_buffer_ids
                        };

                        let mut selections = editor
                            .selections
                            .disjoint_in_range(start_anchor..end_anchor, cx);
                        selections.extend(editor.selections.pending(cx));

                        (selections, selected_buffer_ids)
                    });

                    let (selections, mut active_rows, newest_selection_head) = self
                        .layout_selections(
                            start_anchor,
                            end_anchor,
                            &local_selections,
                            &snapshot,
                            start_row,
                            end_row,
                            window,
                            cx,
                        );
                    let mut breakpoint_rows = self.editor.update(cx, |editor, cx| {
                        editor.active_breakpoints(start_row..end_row, window, cx)
                    });
                    if cx.has_flag::<DebuggerFeatureFlag>() {
                        for display_row in breakpoint_rows.keys() {
                            active_rows.entry(*display_row).or_default().breakpoint = true;
                        }
                    }

                    let line_numbers = self.layout_line_numbers(
                        Some(&gutter_hitbox),
                        gutter_dimensions,
                        line_height,
                        scroll_position,
                        start_row..end_row,
                        &row_infos,
                        &active_rows,
                        newest_selection_head,
                        &snapshot,
                        window,
                        cx,
                    );

                    // We add the gutter breakpoint indicator to breakpoint_rows after painting
                    // line numbers so we don't paint a line number debug accent color if a user
                    // has their mouse over that line when a breakpoint isn't there
                    if cx.has_flag::<DebuggerFeatureFlag>() {
                        self.editor.update(cx, |editor, cx| {
                            if let Some(phantom_breakpoint) = &mut editor
                                .gutter_breakpoint_indicator
                                .0
                                .filter(|phantom_breakpoint| phantom_breakpoint.is_active)
                            {
                                // Is there a non-phantom breakpoint on this line?
                                phantom_breakpoint.collides_with_existing_breakpoint = true;
                                breakpoint_rows
                                    .entry(phantom_breakpoint.display_row)
                                    .or_insert_with(|| {
                                        let position = snapshot.display_point_to_anchor(
                                            DisplayPoint::new(phantom_breakpoint.display_row, 0),
                                            Bias::Right,
                                        );
                                        let breakpoint = Breakpoint::new_standard();
                                        phantom_breakpoint.collides_with_existing_breakpoint =
                                            false;
                                        (position, breakpoint)
                                    });
                            }
                        })
                    }

                    let mut expand_toggles =
                        window.with_element_namespace("expand_toggles", |window| {
                            self.layout_expand_toggles(
                                &gutter_hitbox,
                                gutter_dimensions,
                                em_width,
                                line_height,
                                scroll_position,
                                &row_infos,
                                window,
                                cx,
                            )
                        });

                    let mut crease_toggles =
                        window.with_element_namespace("crease_toggles", |window| {
                            self.layout_crease_toggles(
                                start_row..end_row,
                                &row_infos,
                                &active_rows,
                                &snapshot,
                                window,
                                cx,
                            )
                        });
                    let crease_trailers =
                        window.with_element_namespace("crease_trailers", |window| {
                            self.layout_crease_trailers(
                                row_infos.iter().copied(),
                                &snapshot,
                                window,
                                cx,
                            )
                        });

                    let display_hunks = self.layout_gutter_diff_hunks(
                        line_height,
                        &gutter_hitbox,
                        start_row..end_row,
                        &snapshot,
                        window,
                        cx,
                    );

                    let mut line_layouts = Self::layout_lines(
                        start_row..end_row,
                        &snapshot,
                        &self.style,
                        editor_width,
                        is_row_soft_wrapped,
                        window,
                        cx,
                    );
                    let new_fold_widths = line_layouts
                        .iter()
                        .flat_map(|layout| &layout.fragments)
                        .filter_map(|fragment| {
                            if let LineFragment::Element { id, size, .. } = fragment {
                                Some((*id, size.width))
                            } else {
                                None
                            }
                        });
                    if self.editor.update(cx, |editor, cx| {
                        editor.update_fold_widths(new_fold_widths, cx)
                    }) {
                        // If the fold widths have changed, we need to prepaint
                        // the element again to account for any changes in
                        // wrapping.
                        return self.prepaint(None, bounds, &mut (), window, cx);
                    }

                    let longest_line_blame_width = self
                        .editor
                        .update(cx, |editor, cx| {
                            if !editor.show_git_blame_inline {
                                return None;
                            }
                            let blame = editor.blame.as_ref()?;
                            let blame_entry = blame
                                .update(cx, |blame, cx| {
                                    let row_infos =
                                        snapshot.row_infos(snapshot.longest_row()).next()?;
                                    blame.blame_for_rows(&[row_infos], cx).next()
                                })
                                .flatten()?;
                            let mut element = render_inline_blame_entry(blame_entry, &style, cx)?;
                            let inline_blame_padding = INLINE_BLAME_PADDING_EM_WIDTHS * em_advance;
                            Some(
                                element
                                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                                    .width
                                    + inline_blame_padding,
                            )
                        })
                        .unwrap_or(Pixels::ZERO);

                    let longest_line_width = layout_line(
                        snapshot.longest_row(),
                        &snapshot,
                        &style,
                        editor_width,
                        is_row_soft_wrapped,
                        window,
                        cx,
                    )
                    .width;

                    let scrollbar_layout_information = ScrollbarLayoutInformation::new(
                        text_hitbox.bounds,
                        glyph_grid_cell,
                        size(longest_line_width, max_row.as_f32() * line_height),
                        longest_line_blame_width,
                        style.scrollbar_width,
                        editor_width,
                        EditorSettings::get_global(cx),
                    );

                    let mut scroll_width = scrollbar_layout_information.scroll_range.width;

                    let sticky_header_excerpt = if snapshot.buffer_snapshot.show_headers() {
                        snapshot.sticky_header_excerpt(scroll_position.y)
                    } else {
                        None
                    };
                    let sticky_header_excerpt_id =
                        sticky_header_excerpt.as_ref().map(|top| top.excerpt.id);

                    let blocks = window.with_element_namespace("blocks", |window| {
                        self.render_blocks(
                            start_row..end_row,
                            &snapshot,
                            &hitbox,
                            &text_hitbox,
                            editor_width,
                            &mut scroll_width,
                            &gutter_dimensions,
                            em_width,
                            gutter_dimensions.full_width(),
                            line_height,
                            &mut line_layouts,
                            &local_selections,
                            &selected_buffer_ids,
                            is_row_soft_wrapped,
                            sticky_header_excerpt_id,
                            window,
                            cx,
                        )
                    });
                    let (mut blocks, row_block_types) = match blocks {
                        Ok(blocks) => blocks,
                        Err(resized_blocks) => {
                            self.editor.update(cx, |editor, cx| {
                                editor.resize_blocks(resized_blocks, autoscroll_request, cx)
                            });
                            return self.prepaint(None, bounds, &mut (), window, cx);
                        }
                    };

                    let sticky_buffer_header = sticky_header_excerpt.map(|sticky_header_excerpt| {
                        window.with_element_namespace("blocks", |window| {
                            self.layout_sticky_buffer_header(
                                sticky_header_excerpt,
                                scroll_position.y,
                                line_height,
                                &snapshot,
                                &hitbox,
                                &selected_buffer_ids,
                                &blocks,
                                window,
                                cx,
                            )
                        })
                    });

                    let start_buffer_row =
                        MultiBufferRow(start_anchor.to_point(&snapshot.buffer_snapshot).row);
                    let end_buffer_row =
                        MultiBufferRow(end_anchor.to_point(&snapshot.buffer_snapshot).row);

                    let scroll_max = point(
                        ((scroll_width - editor_text_bounds.size.width) / em_width).max(0.0),
                        max_scroll_top,
                    );

                    self.editor.update(cx, |editor, cx| {
                        let clamped = editor.scroll_manager.clamp_scroll_left(scroll_max.x);

                        let autoscrolled = if autoscroll_horizontally {
                            editor.autoscroll_horizontally(
                                start_row,
                                editor_width - (glyph_grid_cell.width / 2.0)
                                    + style.scrollbar_width,
                                scroll_width,
                                em_width,
                                &line_layouts,
                                cx,
                            )
                        } else {
                            false
                        };

                        if clamped || autoscrolled {
                            snapshot = editor.snapshot(window, cx);
                            scroll_position = snapshot.scroll_position();
                        }
                    });

                    let scroll_pixel_position = point(
                        scroll_position.x * em_width,
                        scroll_position.y * line_height,
                    );

                    let indent_guides = self.layout_indent_guides(
                        content_origin,
                        text_hitbox.origin,
                        start_buffer_row..end_buffer_row,
                        scroll_pixel_position,
                        line_height,
                        &snapshot,
                        window,
                        cx,
                    );

                    let crease_trailers =
                        window.with_element_namespace("crease_trailers", |window| {
                            self.prepaint_crease_trailers(
                                crease_trailers,
                                &line_layouts,
                                line_height,
                                content_origin,
                                scroll_pixel_position,
                                em_width,
                                window,
                                cx,
                            )
                        });

                    let (inline_completion_popover, inline_completion_popover_origin) = self
                        .editor
                        .update(cx, |editor, cx| {
                            editor.render_edit_prediction_popover(
                                &text_hitbox.bounds,
                                content_origin,
                                &snapshot,
                                start_row..end_row,
                                scroll_position.y,
                                scroll_position.y + height_in_lines,
                                &line_layouts,
                                line_height,
                                scroll_pixel_position,
                                newest_selection_head,
                                editor_width,
                                &style,
                                window,
                                cx,
                            )
                        })
                        .unzip();

                    let mut inline_diagnostics = self.layout_inline_diagnostics(
                        &line_layouts,
                        &crease_trailers,
                        &row_block_types,
                        content_origin,
                        scroll_pixel_position,
                        inline_completion_popover_origin,
                        start_row,
                        end_row,
                        line_height,
                        em_width,
                        &style,
                        window,
                        cx,
                    );

                    let mut inline_blame = None;
                    if let Some(newest_selection_head) = newest_selection_head {
                        let display_row = newest_selection_head.row();
                        if (start_row..end_row).contains(&display_row)
                            && !row_block_types.contains_key(&display_row)
                        {
                            let line_ix = display_row.minus(start_row) as usize;
                            let row_info = &row_infos[line_ix];
                            let line_layout = &line_layouts[line_ix];
                            let crease_trailer_layout = crease_trailers[line_ix].as_ref();
                            inline_blame = self.layout_inline_blame(
                                display_row,
                                row_info,
                                line_layout,
                                crease_trailer_layout,
                                em_width,
                                content_origin,
                                scroll_pixel_position,
                                line_height,
                                &text_hitbox,
                                window,
                                cx,
                            );
                            if inline_blame.is_some() {
                                // Blame overrides inline diagnostics
                                inline_diagnostics.remove(&display_row);
                            }
                        }
                    }

                    let blamed_display_rows = self.layout_blame_entries(
                        &row_infos,
                        em_width,
                        scroll_position,
                        line_height,
                        &gutter_hitbox,
                        gutter_dimensions.git_blame_entries_width,
                        window,
                        cx,
                    );

                    self.editor.update(cx, |editor, cx| {
                        let clamped = editor.scroll_manager.clamp_scroll_left(scroll_max.x);

                        let autoscrolled = if autoscroll_horizontally {
                            editor.autoscroll_horizontally(
                                start_row,
                                editor_width - (glyph_grid_cell.width / 2.0)
                                    + style.scrollbar_width,
                                scroll_width,
                                em_width,
                                &line_layouts,
                                cx,
                            )
                        } else {
                            false
                        };

                        if clamped || autoscrolled {
                            snapshot = editor.snapshot(window, cx);
                            scroll_position = snapshot.scroll_position();
                        }
                    });

                    let line_elements = self.prepaint_lines(
                        start_row,
                        &mut line_layouts,
                        line_height,
                        scroll_pixel_position,
                        content_origin,
                        window,
                        cx,
                    );

                    window.with_element_namespace("blocks", |window| {
                        self.layout_blocks(
                            &mut blocks,
                            &hitbox,
                            line_height,
                            scroll_pixel_position,
                            window,
                            cx,
                        );
                    });

                    let cursors = self.collect_cursors(&snapshot, cx);
                    let visible_row_range = start_row..end_row;
                    let non_visible_cursors = cursors
                        .iter()
                        .any(|c| !visible_row_range.contains(&c.0.row()));

                    let visible_cursors = self.layout_visible_cursors(
                        &snapshot,
                        &selections,
                        &row_block_types,
                        start_row..end_row,
                        &line_layouts,
                        &text_hitbox,
                        content_origin,
                        scroll_position,
                        scroll_pixel_position,
                        line_height,
                        em_width,
                        em_advance,
                        autoscroll_containing_element,
                        window,
                        cx,
                    );

                    let scrollbars_layout = self.layout_scrollbars(
                        &snapshot,
                        scrollbar_layout_information,
                        content_offset,
                        scroll_position,
                        non_visible_cursors,
                        window,
                        cx,
                    );

                    let gutter_settings = EditorSettings::get_global(cx).gutter;

                    let mut code_actions_indicator = None;
                    if let Some(newest_selection_head) = newest_selection_head {
                        let newest_selection_point =
                            newest_selection_head.to_point(&snapshot.display_snapshot);

                        if (start_row..end_row).contains(&newest_selection_head.row()) {
                            self.layout_cursor_popovers(
                                line_height,
                                &text_hitbox,
                                content_origin,
                                start_row,
                                scroll_pixel_position,
                                &line_layouts,
                                newest_selection_head,
                                newest_selection_point,
                                &style,
                                window,
                                cx,
                            );

                            let show_code_actions = snapshot
                                .show_code_actions
                                .unwrap_or(gutter_settings.code_actions);
                            if show_code_actions {
                                let newest_selection_point =
                                    newest_selection_head.to_point(&snapshot.display_snapshot);
                                if !snapshot
                                    .is_line_folded(MultiBufferRow(newest_selection_point.row))
                                {
                                    let buffer = snapshot.buffer_snapshot.buffer_line_for_row(
                                        MultiBufferRow(newest_selection_point.row),
                                    );
                                    if let Some((buffer, range)) = buffer {
                                        let buffer_id = buffer.remote_id();
                                        let row = range.start.row;
                                        let has_test_indicator = self
                                            .editor
                                            .read(cx)
                                            .tasks
                                            .contains_key(&(buffer_id, row));

                                        let has_expand_indicator = row_infos
                                            .get(
                                                (newest_selection_head.row() - start_row).0
                                                    as usize,
                                            )
                                            .is_some_and(|row_info| row_info.expand_info.is_some());

                                        if !has_test_indicator && !has_expand_indicator {
                                            code_actions_indicator = self
                                                .layout_code_actions_indicator(
                                                    line_height,
                                                    newest_selection_head,
                                                    scroll_pixel_position,
                                                    &gutter_dimensions,
                                                    &gutter_hitbox,
                                                    &mut breakpoint_rows,
                                                    &display_hunks,
                                                    window,
                                                    cx,
                                                );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    self.layout_gutter_menu(
                        line_height,
                        &text_hitbox,
                        content_origin,
                        scroll_pixel_position,
                        gutter_dimensions.width - gutter_dimensions.left_padding,
                        window,
                        cx,
                    );

                    let test_indicators = if gutter_settings.runnables {
                        self.layout_run_indicators(
                            line_height,
                            start_row..end_row,
                            &row_infos,
                            scroll_pixel_position,
                            &gutter_dimensions,
                            &gutter_hitbox,
                            &display_hunks,
                            &snapshot,
                            &mut breakpoint_rows,
                            window,
                            cx,
                        )
                    } else {
                        Vec::new()
                    };

                    let show_breakpoints = snapshot
                        .show_breakpoints
                        .unwrap_or(gutter_settings.breakpoints);
                    let breakpoints = if cx.has_flag::<DebuggerFeatureFlag>() && show_breakpoints {
                        self.layout_breakpoints(
                            line_height,
                            start_row..end_row,
                            scroll_pixel_position,
                            &gutter_dimensions,
                            &gutter_hitbox,
                            &display_hunks,
                            &snapshot,
                            breakpoint_rows,
                            &row_infos,
                            window,
                            cx,
                        )
                    } else {
                        vec![]
                    };

                    self.layout_signature_help(
                        &hitbox,
                        &text_hitbox,
                        content_origin,
                        scroll_pixel_position,
                        newest_selection_head,
                        start_row,
                        &line_layouts,
                        line_height,
                        em_width,
                        window,
                        cx,
                    );

                    if !cx.has_active_drag() {
                        self.layout_hover_popovers(
                            &snapshot,
                            &hitbox,
                            &text_hitbox,
                            start_row..end_row,
                            content_origin,
                            scroll_pixel_position,
                            &line_layouts,
                            line_height,
                            em_width,
                            window,
                            cx,
                        );
                    }

                    let mouse_context_menu = self.layout_mouse_context_menu(
                        &snapshot,
                        start_row..end_row,
                        content_origin,
                        window,
                        cx,
                    );

                    window.with_element_namespace("crease_toggles", |window| {
                        self.prepaint_crease_toggles(
                            &mut crease_toggles,
                            line_height,
                            &gutter_dimensions,
                            gutter_settings,
                            scroll_pixel_position,
                            &gutter_hitbox,
                            window,
                            cx,
                        )
                    });

                    window.with_element_namespace("expand_toggles", |window| {
                        self.prepaint_expand_toggles(&mut expand_toggles, window, cx)
                    });

                    let invisible_symbol_font_size = font_size / 2.;
                    let tab_invisible = window
                        .text_system()
                        .shape_line(
                            "→".into(),
                            invisible_symbol_font_size,
                            &[TextRun {
                                len: "→".len(),
                                font: self.style.text.font(),
                                color: cx.theme().colors().editor_invisible,
                                background_color: None,
                                underline: None,
                                strikethrough: None,
                            }],
                        )
                        .unwrap();
                    let space_invisible = window
                        .text_system()
                        .shape_line(
                            "•".into(),
                            invisible_symbol_font_size,
                            &[TextRun {
                                len: "•".len(),
                                font: self.style.text.font(),
                                color: cx.theme().colors().editor_invisible,
                                background_color: None,
                                underline: None,
                                strikethrough: None,
                            }],
                        )
                        .unwrap();

                    let mode = snapshot.mode;

                    let position_map = Rc::new(PositionMap {
                        size: bounds.size,
                        visible_row_range,
                        scroll_pixel_position,
                        scroll_max,
                        line_layouts,
                        line_height,
                        em_width,
                        em_advance,
                        snapshot,
                        gutter_hitbox: gutter_hitbox.clone(),
                        text_hitbox: text_hitbox.clone(),
                    });

                    self.editor.update(cx, |editor, _| {
                        editor.last_position_map = Some(position_map.clone())
                    });

                    let diff_hunk_controls = if is_read_only {
                        vec![]
                    } else {
                        self.layout_diff_hunk_controls(
                            start_row..end_row,
                            &row_infos,
                            &text_hitbox,
                            &position_map,
                            newest_selection_head,
                            line_height,
                            scroll_pixel_position,
                            &display_hunks,
                            &highlighted_rows,
                            self.editor.clone(),
                            window,
                            cx,
                        )
                    };

                    EditorLayout {
                        mode,
                        position_map,
                        visible_display_row_range: start_row..end_row,
                        wrap_guides,
                        indent_guides,
                        hitbox,
                        gutter_hitbox,
                        display_hunks,
                        content_origin,
                        scrollbars_layout,
                        active_rows,
                        highlighted_rows,
                        highlighted_ranges,
                        highlighted_gutter_ranges,
                        redacted_ranges,
                        line_elements,
                        line_numbers,
                        blamed_display_rows,
                        inline_diagnostics,
                        inline_blame,
                        blocks,
                        cursors,
                        visible_cursors,
                        selections,
                        inline_completion_popover,
                        diff_hunk_controls,
                        mouse_context_menu,
                        test_indicators,
                        breakpoints,
                        code_actions_indicator,
                        crease_toggles,
                        crease_trailers,
                        tab_invisible,
                        space_invisible,
                        sticky_buffer_header,
                        expand_toggles,
                    }
                })
            })
        })
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        bounds: Bounds<gpui::Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.editor.focus_handle(cx);
        let key_context = self
            .editor
            .update(cx, |editor, cx| editor.key_context(window, cx));

        window.set_key_context(key_context);
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );
        self.register_actions(window, cx);
        self.register_key_listeners(window, cx, layout);

        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };
        let rem_size = self.rem_size(cx);
        window.with_rem_size(rem_size, |window| {
            window.with_text_style(Some(text_style), |window| {
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    self.paint_mouse_listeners(layout, window, cx);
                    self.paint_background(layout, window, cx);
                    self.paint_indent_guides(layout, window, cx);

                    if layout.gutter_hitbox.size.width > Pixels::ZERO {
                        self.paint_blamed_display_rows(layout, window, cx);
                        self.paint_line_numbers(layout, window, cx);
                    }

                    self.paint_text(layout, window, cx);

                    if layout.gutter_hitbox.size.width > Pixels::ZERO {
                        self.paint_gutter_highlights(layout, window, cx);
                        self.paint_gutter_indicators(layout, window, cx);
                    }

                    if !layout.blocks.is_empty() {
                        window.with_element_namespace("blocks", |window| {
                            self.paint_blocks(layout, window, cx);
                        });
                    }

                    window.with_element_namespace("blocks", |window| {
                        if let Some(mut sticky_header) = layout.sticky_buffer_header.take() {
                            sticky_header.paint(window, cx)
                        }
                    });

                    self.paint_scrollbars(layout, window, cx);
                    self.paint_inline_completion_popover(layout, window, cx);
                    self.paint_mouse_context_menu(layout, window, cx);
                });
            })
        })
    }
}

pub(super) fn gutter_bounds(
    editor_bounds: Bounds<Pixels>,
    gutter_dimensions: GutterDimensions,
) -> Bounds<Pixels> {
    Bounds {
        origin: editor_bounds.origin,
        size: size(gutter_dimensions.width, editor_bounds.size.height),
    }
}

/// Holds information required for layouting the editor scrollbars.
struct ScrollbarLayoutInformation {
    /// The bounds of the editor area (excluding the content offset).
    editor_bounds: Bounds<Pixels>,
    /// The available range to scroll within the document.
    scroll_range: Size<Pixels>,
    /// The space available for one glyph in the editor.
    glyph_grid_cell: Size<Pixels>,
}

impl ScrollbarLayoutInformation {
    pub fn new(
        editor_bounds: Bounds<Pixels>,
        glyph_grid_cell: Size<Pixels>,
        document_size: Size<Pixels>,
        longest_line_blame_width: Pixels,
        scrollbar_width: Pixels,
        editor_width: Pixels,
        settings: &EditorSettings,
    ) -> Self {
        let vertical_overscroll = match settings.scroll_beyond_last_line {
            ScrollBeyondLastLine::OnePage => editor_bounds.size.height,
            ScrollBeyondLastLine::Off => glyph_grid_cell.height,
            ScrollBeyondLastLine::VerticalScrollMargin => {
                (1.0 + settings.vertical_scroll_margin) * glyph_grid_cell.height
            }
        };

        let right_margin = if document_size.width + longest_line_blame_width >= editor_width {
            glyph_grid_cell.width + scrollbar_width
        } else {
            px(0.0)
        };

        let overscroll = size(right_margin + longest_line_blame_width, vertical_overscroll);

        let scroll_range = document_size + overscroll;

        ScrollbarLayoutInformation {
            editor_bounds,
            scroll_range,
            glyph_grid_cell,
        }
    }
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct EditorLayout {
    position_map: Rc<PositionMap>,
    hitbox: Hitbox,
    gutter_hitbox: Hitbox,
    content_origin: gpui::Point<Pixels>,
    scrollbars_layout: Option<EditorScrollbars>,
    mode: EditorMode,
    wrap_guides: SmallVec<[(Pixels, bool); 2]>,
    indent_guides: Option<Vec<IndentGuideLayout>>,
    visible_display_row_range: Range<DisplayRow>,
    active_rows: BTreeMap<DisplayRow, LineHighlightSpec>,
    highlighted_rows: BTreeMap<DisplayRow, LineHighlight>,
    line_elements: SmallVec<[AnyElement; 1]>,
    line_numbers: Arc<HashMap<MultiBufferRow, LineNumberLayout>>,
    display_hunks: Vec<(DisplayDiffHunk, Option<Hitbox>)>,
    blamed_display_rows: Option<Vec<AnyElement>>,
    inline_diagnostics: HashMap<DisplayRow, AnyElement>,
    inline_blame: Option<AnyElement>,
    blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    highlighted_gutter_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    redacted_ranges: Vec<Range<DisplayPoint>>,
    cursors: Vec<(DisplayPoint, Hsla)>,
    visible_cursors: Vec<CursorLayout>,
    selections: Vec<(PlayerColor, Vec<SelectionLayout>)>,
    code_actions_indicator: Option<AnyElement>,
    test_indicators: Vec<AnyElement>,
    breakpoints: Vec<AnyElement>,
    crease_toggles: Vec<Option<AnyElement>>,
    expand_toggles: Vec<Option<(AnyElement, gpui::Point<Pixels>)>>,
    diff_hunk_controls: Vec<AnyElement>,
    crease_trailers: Vec<Option<CreaseTrailerLayout>>,
    inline_completion_popover: Option<AnyElement>,
    mouse_context_menu: Option<AnyElement>,
    tab_invisible: ShapedLine,
    space_invisible: ShapedLine,
    sticky_buffer_header: Option<AnyElement>,
}

impl EditorLayout {
    fn line_end_overshoot(&self) -> Pixels {
        0.15 * self.position_map.line_height
    }
}

struct LineNumberLayout {
    shaped_line: ShapedLine,
    hitbox: Option<Hitbox>,
}

struct ColoredRange<T> {
    start: T,
    end: T,
    color: Hsla,
}

impl Along for ScrollbarAxes {
    type Unit = bool;

    fn along(&self, axis: ScrollbarAxis) -> Self::Unit {
        match axis {
            ScrollbarAxis::Horizontal => self.horizontal,
            ScrollbarAxis::Vertical => self.vertical,
        }
    }

    fn apply_along(&self, axis: ScrollbarAxis, f: impl FnOnce(Self::Unit) -> Self::Unit) -> Self {
        match axis {
            ScrollbarAxis::Horizontal => ScrollbarAxes {
                horizontal: f(self.horizontal),
                vertical: self.vertical,
            },
            ScrollbarAxis::Vertical => ScrollbarAxes {
                horizontal: self.horizontal,
                vertical: f(self.vertical),
            },
        }
    }
}

#[derive(Clone)]
struct EditorScrollbars {
    pub vertical: Option<ScrollbarLayout>,
    pub horizontal: Option<ScrollbarLayout>,
    pub visible: bool,
}

impl EditorScrollbars {
    pub fn from_scrollbar_axes(
        settings_visibility: ScrollbarAxes,
        layout_information: &ScrollbarLayoutInformation,
        content_offset: gpui::Point<Pixels>,
        scroll_position: gpui::Point<f32>,
        scrollbar_width: Pixels,
        show_scrollbars: bool,
        window: &mut Window,
    ) -> Self {
        let ScrollbarLayoutInformation {
            editor_bounds,
            scroll_range,
            glyph_grid_cell,
        } = layout_information;

        let scrollbar_bounds_for = |axis: ScrollbarAxis| match axis {
            ScrollbarAxis::Horizontal => Bounds::from_corner_and_size(
                Corner::BottomLeft,
                editor_bounds.bottom_left(),
                size(
                    if settings_visibility.vertical {
                        editor_bounds.size.width - scrollbar_width
                    } else {
                        editor_bounds.size.width
                    },
                    scrollbar_width,
                ),
            ),
            ScrollbarAxis::Vertical => Bounds::from_corner_and_size(
                Corner::TopRight,
                editor_bounds.top_right(),
                size(scrollbar_width, editor_bounds.size.height),
            ),
        };

        let mut create_scrollbar_layout = |axis| {
            settings_visibility
                .along(axis)
                .then(|| {
                    (
                        editor_bounds.size.along(axis) - content_offset.along(axis),
                        scroll_range.along(axis),
                    )
                })
                .filter(|(editor_content_size, scroll_range)| {
                    // The scrollbar should only be rendered if the content does
                    // not entirely fit into the editor
                    // However, this only applies to the horizontal scrollbar, as information about the
                    // vertical scrollbar layout is always needed for scrollbar diagnostics.
                    axis != ScrollbarAxis::Horizontal || editor_content_size < scroll_range
                })
                .map(|(editor_content_size, scroll_range)| {
                    ScrollbarLayout::new(
                        window.insert_hitbox(scrollbar_bounds_for(axis), false),
                        editor_content_size,
                        scroll_range,
                        glyph_grid_cell.along(axis),
                        content_offset.along(axis),
                        scroll_position.along(axis),
                        axis,
                    )
                })
        };

        Self {
            vertical: create_scrollbar_layout(ScrollbarAxis::Vertical),
            horizontal: create_scrollbar_layout(ScrollbarAxis::Horizontal),
            visible: show_scrollbars,
        }
    }

    pub fn iter_scrollbars(&self) -> impl Iterator<Item = (&ScrollbarLayout, ScrollbarAxis)> + '_ {
        [
            (&self.vertical, ScrollbarAxis::Vertical),
            (&self.horizontal, ScrollbarAxis::Horizontal),
        ]
        .into_iter()
        .filter_map(|(scrollbar, axis)| scrollbar.as_ref().map(|s| (s, axis)))
    }

    /// Returns the currently hovered scrollbar axis, if any.
    pub fn get_hovered_axis(&self, window: &Window) -> Option<(&ScrollbarLayout, ScrollbarAxis)> {
        self.iter_scrollbars()
            .find(|s| s.0.hitbox.is_hovered(window))
    }
}

#[derive(Clone)]
struct ScrollbarLayout {
    hitbox: Hitbox,
    visible_range: Range<f32>,
    text_unit_size: Pixels,
    content_offset: Pixels,
    thumb_size: Pixels,
    axis: ScrollbarAxis,
}

impl ScrollbarLayout {
    const BORDER_WIDTH: Pixels = px(1.0);
    const LINE_MARKER_HEIGHT: Pixels = px(2.0);
    const MIN_MARKER_HEIGHT: Pixels = px(5.0);
    const MIN_THUMB_SIZE: Pixels = px(25.0);

    fn new(
        scrollbar_track_hitbox: Hitbox,
        editor_content_size: Pixels,
        scroll_range: Pixels,
        glyph_space: Pixels,
        content_offset: Pixels,
        scroll_position: f32,
        axis: ScrollbarAxis,
    ) -> Self {
        let track_bounds = scrollbar_track_hitbox.bounds;
        // The length of the track available to the scrollbar thumb. We deliberately
        // exclude the content size here so that the thumb aligns with the content.
        let track_length = track_bounds.size.along(axis) - content_offset;

        let text_units_per_page = editor_content_size / glyph_space;
        let visible_range = scroll_position..scroll_position + text_units_per_page;
        let total_text_units = scroll_range / glyph_space;

        let thumb_percentage = text_units_per_page / total_text_units;
        let thumb_size = (track_length * thumb_percentage)
            .max(ScrollbarLayout::MIN_THUMB_SIZE)
            .min(track_length);
        let text_unit_size =
            (track_length - thumb_size) / (total_text_units - text_units_per_page).max(0.);

        ScrollbarLayout {
            hitbox: scrollbar_track_hitbox,
            visible_range,
            text_unit_size,
            content_offset,
            thumb_size,
            axis,
        }
    }

    fn thumb_bounds(&self) -> Bounds<Pixels> {
        let scrollbar_track = &self.hitbox.bounds;
        Bounds::new(
            scrollbar_track
                .origin
                .apply_along(self.axis, |origin| self.thumb_origin(origin)),
            scrollbar_track
                .size
                .apply_along(self.axis, |_| self.thumb_size),
        )
    }

    fn thumb_origin(&self, origin: Pixels) -> Pixels {
        origin + self.content_offset + self.visible_range.start * self.text_unit_size
    }

    fn marker_quads_for_ranges(
        &self,
        row_ranges: impl IntoIterator<Item = ColoredRange<DisplayRow>>,
        column: Option<usize>,
    ) -> Vec<PaintQuad> {
        struct MinMax {
            min: Pixels,
            max: Pixels,
        }
        let (x_range, height_limit) = if let Some(column) = column {
            let column_width = px(((self.hitbox.size.width - Self::BORDER_WIDTH).0 / 3.0).floor());
            let start = Self::BORDER_WIDTH + (column as f32 * column_width);
            let end = start + column_width;
            (
                Range { start, end },
                MinMax {
                    min: Self::MIN_MARKER_HEIGHT,
                    max: px(f32::MAX),
                },
            )
        } else {
            (
                Range {
                    start: Self::BORDER_WIDTH,
                    end: self.hitbox.size.width,
                },
                MinMax {
                    min: Self::LINE_MARKER_HEIGHT,
                    max: Self::LINE_MARKER_HEIGHT,
                },
            )
        };

        let row_to_y = |row: DisplayRow| row.as_f32() * self.text_unit_size;
        let mut pixel_ranges = row_ranges
            .into_iter()
            .map(|range| {
                let start_y = row_to_y(range.start);
                let end_y = row_to_y(range.end)
                    + self
                        .text_unit_size
                        .max(height_limit.min)
                        .min(height_limit.max);
                ColoredRange {
                    start: start_y,
                    end: end_y,
                    color: range.color,
                }
            })
            .peekable();

        let mut quads = Vec::new();
        while let Some(mut pixel_range) = pixel_ranges.next() {
            while let Some(next_pixel_range) = pixel_ranges.peek() {
                if pixel_range.end >= next_pixel_range.start - px(1.0)
                    && pixel_range.color == next_pixel_range.color
                {
                    pixel_range.end = next_pixel_range.end.max(pixel_range.end);
                    pixel_ranges.next();
                } else {
                    break;
                }
            }

            let bounds = Bounds::from_corners(
                point(x_range.start, pixel_range.start),
                point(x_range.end, pixel_range.end),
            );
            quads.push(quad(
                bounds,
                Corners::default(),
                pixel_range.color,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));
        }

        quads
    }
}

struct CreaseTrailerLayout {
    element: AnyElement,
    bounds: Bounds<Pixels>,
}

pub(crate) struct PositionMap {
    pub size: Size<Pixels>,
    pub line_height: Pixels,
    pub scroll_pixel_position: gpui::Point<Pixels>,
    pub scroll_max: gpui::Point<f32>,
    pub em_width: Pixels,
    pub em_advance: Pixels,
    pub visible_row_range: Range<DisplayRow>,
    pub line_layouts: Vec<LineWithInvisibles>,
    pub snapshot: EditorSnapshot,
    pub text_hitbox: Hitbox,
    pub gutter_hitbox: Hitbox,
}

#[derive(Debug, Copy, Clone)]
pub struct PointForPosition {
    pub previous_valid: DisplayPoint,
    pub next_valid: DisplayPoint,
    pub exact_unclipped: DisplayPoint,
    pub column_overshoot_after_line_end: u32,
}

impl PointForPosition {
    pub fn as_valid(&self) -> Option<DisplayPoint> {
        if self.previous_valid == self.exact_unclipped && self.next_valid == self.exact_unclipped {
            Some(self.previous_valid)
        } else {
            None
        }
    }
}

impl PositionMap {
    pub(crate) fn point_for_position(&self, position: gpui::Point<Pixels>) -> PointForPosition {
        let text_bounds = self.text_hitbox.bounds;
        let scroll_position = self.snapshot.scroll_position();
        let position = position - text_bounds.origin;
        let y = position.y.max(px(0.)).min(self.size.height);
        let x = position.x + (scroll_position.x * self.em_width);
        let row = ((y / self.line_height) + scroll_position.y) as u32;

        let (column, x_overshoot_after_line_end) = if let Some(line) = self
            .line_layouts
            .get(row as usize - scroll_position.y as usize)
        {
            if let Some(ix) = line.index_for_x(x) {
                (ix as u32, px(0.))
            } else {
                (line.len as u32, px(0.).max(x - line.width))
            }
        } else {
            (0, x)
        };

        let mut exact_unclipped = DisplayPoint::new(DisplayRow(row), column);
        let previous_valid = self.snapshot.clip_point(exact_unclipped, Bias::Left);
        let next_valid = self.snapshot.clip_point(exact_unclipped, Bias::Right);

        let column_overshoot_after_line_end = (x_overshoot_after_line_end / self.em_advance) as u32;
        *exact_unclipped.column_mut() += column_overshoot_after_line_end;
        PointForPosition {
            previous_valid,
            next_valid,
            exact_unclipped,
            column_overshoot_after_line_end,
        }
    }
}

struct BlockLayout {
    id: BlockId,
    x_offset: Pixels,
    row: Option<DisplayRow>,
    element: AnyElement,
    available_space: Size<AvailableSpace>,
    style: BlockStyle,
    overlaps_gutter: bool,
    is_buffer_header: bool,
}

pub fn layout_line(
    row: DisplayRow,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    text_width: Pixels,
    is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
    window: &mut Window,
    cx: &mut App,
) -> LineWithInvisibles {
    let chunks = snapshot.highlighted_chunks(row..row + DisplayRow(1), true, style);
    LineWithInvisibles::from_chunks(
        chunks,
        &style,
        MAX_LINE_LEN,
        1,
        snapshot.mode,
        text_width,
        is_row_soft_wrapped,
        window,
        cx,
    )
    .pop()
    .unwrap()
}

#[derive(Debug)]
pub struct IndentGuideLayout {
    origin: gpui::Point<Pixels>,
    length: Pixels,
    single_indent_width: Pixels,
    depth: u32,
    active: bool,
    settings: IndentGuideSettings,
}

pub struct CursorLayout {
    origin: gpui::Point<Pixels>,
    block_width: Pixels,
    line_height: Pixels,
    color: Hsla,
    shape: CursorShape,
    block_text: Option<ShapedLine>,
    cursor_name: Option<AnyElement>,
}

#[derive(Debug)]
pub struct CursorName {
    string: SharedString,
    color: Hsla,
    is_top_row: bool,
}

impl CursorLayout {
    pub fn new(
        origin: gpui::Point<Pixels>,
        block_width: Pixels,
        line_height: Pixels,
        color: Hsla,
        shape: CursorShape,
        block_text: Option<ShapedLine>,
    ) -> CursorLayout {
        CursorLayout {
            origin,
            block_width,
            line_height,
            color,
            shape,
            block_text,
            cursor_name: None,
        }
    }

    pub fn bounding_rect(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin + origin,
            size: size(self.block_width, self.line_height),
        }
    }

    fn bounds(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        match self.shape {
            CursorShape::Bar => Bounds {
                origin: self.origin + origin,
                size: size(px(2.0), self.line_height),
            },
            CursorShape::Block | CursorShape::Hollow => Bounds {
                origin: self.origin + origin,
                size: size(self.block_width, self.line_height),
            },
            CursorShape::Underline => Bounds {
                origin: self.origin
                    + origin
                    + gpui::Point::new(Pixels::ZERO, self.line_height - px(2.0)),
                size: size(self.block_width, px(2.0)),
            },
        }
    }

    pub fn layout(
        &mut self,
        origin: gpui::Point<Pixels>,
        cursor_name: Option<CursorName>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(cursor_name) = cursor_name {
            let bounds = self.bounds(origin);
            let text_size = self.line_height / 1.5;

            let name_origin = if cursor_name.is_top_row {
                point(bounds.right() - px(1.), bounds.top())
            } else {
                match self.shape {
                    CursorShape::Bar => point(
                        bounds.right() - px(2.),
                        bounds.top() - text_size / 2. - px(1.),
                    ),
                    _ => point(
                        bounds.right() - px(1.),
                        bounds.top() - text_size / 2. - px(1.),
                    ),
                }
            };
            let mut name_element = div()
                .bg(self.color)
                .text_size(text_size)
                .px_0p5()
                .line_height(text_size + px(2.))
                .text_color(cursor_name.color)
                .child(cursor_name.string.clone())
                .into_any_element();

            name_element.prepaint_as_root(name_origin, AvailableSpace::min_size(), window, cx);

            self.cursor_name = Some(name_element);
        }
    }

    pub fn paint(&mut self, origin: gpui::Point<Pixels>, window: &mut Window, cx: &mut App) {
        let bounds = self.bounds(origin);

        //Draw background or border quad
        let cursor = if matches!(self.shape, CursorShape::Hollow) {
            outline(bounds, self.color, BorderStyle::Solid)
        } else {
            fill(bounds, self.color)
        };

        if let Some(name) = &mut self.cursor_name {
            name.paint(window, cx);
        }

        window.paint_quad(cursor);

        if let Some(block_text) = &self.block_text {
            block_text
                .paint(self.origin + origin, self.line_height, window, cx)
                .log_err();
        }
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }
}

#[derive(Debug)]
pub struct HighlightedRange {
    pub start_y: Pixels,
    pub line_height: Pixels,
    pub lines: Vec<HighlightedRangeLine>,
    pub color: Hsla,
    pub corner_radius: Pixels,
}

#[derive(Debug)]
pub struct HighlightedRangeLine {
    pub start_x: Pixels,
    pub end_x: Pixels,
}

impl HighlightedRange {
    pub fn paint(&self, bounds: Bounds<Pixels>, window: &mut Window) {
        if self.lines.len() >= 2 && self.lines[0].start_x > self.lines[1].end_x {
            self.paint_lines(self.start_y, &self.lines[0..1], bounds, window);
            self.paint_lines(
                self.start_y + self.line_height,
                &self.lines[1..],
                bounds,
                window,
            );
        } else {
            self.paint_lines(self.start_y, &self.lines, bounds, window);
        }
    }

    fn paint_lines(
        &self,
        start_y: Pixels,
        lines: &[HighlightedRangeLine],
        _bounds: Bounds<Pixels>,
        window: &mut Window,
    ) {
        if lines.is_empty() {
            return;
        }

        let first_line = lines.first().unwrap();
        let last_line = lines.last().unwrap();

        let first_top_left = point(first_line.start_x, start_y);
        let first_top_right = point(first_line.end_x, start_y);

        let curve_height = point(Pixels::ZERO, self.corner_radius);
        let curve_width = |start_x: Pixels, end_x: Pixels| {
            let max = (end_x - start_x) / 2.;
            let width = if max < self.corner_radius {
                max
            } else {
                self.corner_radius
            };

            point(width, Pixels::ZERO)
        };

        let top_curve_width = curve_width(first_line.start_x, first_line.end_x);
        let mut builder = gpui::PathBuilder::fill();
        builder.move_to(first_top_right - top_curve_width);
        builder.curve_to(first_top_right + curve_height, first_top_right);

        let mut iter = lines.iter().enumerate().peekable();
        while let Some((ix, line)) = iter.next() {
            let bottom_right = point(line.end_x, start_y + (ix + 1) as f32 * self.line_height);

            if let Some((_, next_line)) = iter.peek() {
                let next_top_right = point(next_line.end_x, bottom_right.y);

                match next_top_right.x.partial_cmp(&bottom_right.x).unwrap() {
                    Ordering::Equal => {
                        builder.line_to(bottom_right);
                    }
                    Ordering::Less => {
                        let curve_width = curve_width(next_top_right.x, bottom_right.x);
                        builder.line_to(bottom_right - curve_height);
                        if self.corner_radius > Pixels::ZERO {
                            builder.curve_to(bottom_right - curve_width, bottom_right);
                        }
                        builder.line_to(next_top_right + curve_width);
                        if self.corner_radius > Pixels::ZERO {
                            builder.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                    Ordering::Greater => {
                        let curve_width = curve_width(bottom_right.x, next_top_right.x);
                        builder.line_to(bottom_right - curve_height);
                        if self.corner_radius > Pixels::ZERO {
                            builder.curve_to(bottom_right + curve_width, bottom_right);
                        }
                        builder.line_to(next_top_right - curve_width);
                        if self.corner_radius > Pixels::ZERO {
                            builder.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                }
            } else {
                let curve_width = curve_width(line.start_x, line.end_x);
                builder.line_to(bottom_right - curve_height);
                if self.corner_radius > Pixels::ZERO {
                    builder.curve_to(bottom_right - curve_width, bottom_right);
                }

                let bottom_left = point(line.start_x, bottom_right.y);
                builder.line_to(bottom_left + curve_width);
                if self.corner_radius > Pixels::ZERO {
                    builder.curve_to(bottom_left - curve_height, bottom_left);
                }
            }
        }

        if first_line.start_x > last_line.start_x {
            let curve_width = curve_width(last_line.start_x, first_line.start_x);
            let second_top_left = point(last_line.start_x, start_y + self.line_height);
            builder.line_to(second_top_left + curve_height);
            if self.corner_radius > Pixels::ZERO {
                builder.curve_to(second_top_left + curve_width, second_top_left);
            }
            let first_bottom_left = point(first_line.start_x, second_top_left.y);
            builder.line_to(first_bottom_left - curve_width);
            if self.corner_radius > Pixels::ZERO {
                builder.curve_to(first_bottom_left - curve_height, first_bottom_left);
            }
        }

        builder.line_to(first_top_left + curve_height);
        if self.corner_radius > Pixels::ZERO {
            builder.curve_to(first_top_left + top_curve_width, first_top_left);
        }
        builder.line_to(first_top_right - top_curve_width);

        if let Ok(path) = builder.build() {
            window.paint_path(path, self.color);
        }
    }
}

enum CursorPopoverType {
    CodeContextMenu,
    EditPrediction,
}

pub fn scale_vertical_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.2) / 100.0).min(px(3.0)).into()
}

fn scale_horizontal_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.2) / 300.0).into()
}

pub fn register_action<T: Action>(
    editor: &Entity<Editor>,
    window: &mut Window,
    listener: impl Fn(&mut Editor, &T, &mut Window, &mut Context<Editor>) + 'static,
) {
    let editor = editor.clone();
    window.on_action(TypeId::of::<T>(), move |action, phase, window, cx| {
        let action = action.downcast_ref().unwrap();
        if phase == DispatchPhase::Bubble {
            editor.update(cx, |editor, cx| {
                listener(editor, action, window, cx);
            })
        }
    })
}

fn compute_auto_height_layout(
    editor: &mut Editor,
    max_lines: usize,
    max_line_number_width: Pixels,
    known_dimensions: Size<Option<Pixels>>,
    available_width: AvailableSpace,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Option<Size<Pixels>> {
    let width = known_dimensions.width.or({
        if let AvailableSpace::Definite(available_width) = available_width {
            Some(available_width)
        } else {
            None
        }
    })?;
    if let Some(height) = known_dimensions.height {
        return Some(size(width, height));
    }

    let style = editor.style.as_ref().unwrap();
    let font_id = window.text_system().resolve_font(&style.text.font());
    let font_size = style.text.font_size.to_pixels(window.rem_size());
    let line_height = style.text.line_height_in_pixels(window.rem_size());
    let em_width = window.text_system().em_width(font_id, font_size).unwrap();

    let mut snapshot = editor.snapshot(window, cx);
    let gutter_dimensions = snapshot
        .gutter_dimensions(font_id, font_size, max_line_number_width, cx)
        .unwrap_or_default();

    editor.gutter_dimensions = gutter_dimensions;
    let text_width = width - gutter_dimensions.width;
    let overscroll = size(em_width, px(0.));

    let editor_width = text_width - gutter_dimensions.margin - overscroll.width - em_width;
    if editor.set_wrap_width(Some(editor_width), cx) {
        snapshot = editor.snapshot(window, cx);
    }

    let scroll_height = (snapshot.max_point().row().next_row().0 as f32) * line_height;
    let height = scroll_height
        .max(line_height)
        .min(line_height * max_lines as f32);

    Some(size(width, height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Editor, MultiBuffer,
        display_map::{BlockPlacement, BlockProperties},
        editor_tests::{init_test, update_test_language_settings},
    };
    use gpui::{TestAppContext, VisualTestContext};
    use language::language_settings;
    use log::info;
    use std::num::NonZeroU32;
    use util::test::sample_text;

    #[gpui::test]
    fn test_shape_line_numbers(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let line_height = window
            .update(cx, |_, window, _| {
                style.text.line_height_in_pixels(window.rem_size())
            })
            .unwrap();
        let element = EditorElement::new(&editor, style);
        let snapshot = window
            .update(cx, |editor, window, cx| editor.snapshot(window, cx))
            .unwrap();

        let layouts = cx
            .update_window(*window, |_, window, cx| {
                element.layout_line_numbers(
                    None,
                    GutterDimensions {
                        left_padding: Pixels::ZERO,
                        right_padding: Pixels::ZERO,
                        width: px(30.0),
                        margin: Pixels::ZERO,
                        git_blame_entries_width: None,
                    },
                    line_height,
                    gpui::Point::default(),
                    DisplayRow(0)..DisplayRow(6),
                    &(0..6)
                        .map(|row| RowInfo {
                            buffer_row: Some(row),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    &BTreeMap::default(),
                    Some(DisplayPoint::new(DisplayRow(0), 0)),
                    &snapshot,
                    window,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                element.calculate_relative_line_numbers(
                    &snapshot,
                    &(DisplayRow(0)..DisplayRow(6)),
                    Some(DisplayRow(3)),
                )
            })
            .unwrap();
        assert_eq!(relative_rows[&DisplayRow(0)], 3);
        assert_eq!(relative_rows[&DisplayRow(1)], 2);
        assert_eq!(relative_rows[&DisplayRow(2)], 1);
        // current line has no relative number
        assert_eq!(relative_rows[&DisplayRow(4)], 1);
        assert_eq!(relative_rows[&DisplayRow(5)], 2);

        // works if cursor is before screen
        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                element.calculate_relative_line_numbers(
                    &snapshot,
                    &(DisplayRow(3)..DisplayRow(6)),
                    Some(DisplayRow(1)),
                )
            })
            .unwrap();
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&DisplayRow(3)], 2);
        assert_eq!(relative_rows[&DisplayRow(4)], 3);
        assert_eq!(relative_rows[&DisplayRow(5)], 4);

        // works if cursor is after screen
        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                element.calculate_relative_line_numbers(
                    &snapshot,
                    &(DisplayRow(0)..DisplayRow(3)),
                    Some(DisplayRow(6)),
                )
            })
            .unwrap();
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&DisplayRow(0)], 5);
        assert_eq!(relative_rows[&DisplayRow(1)], 4);
        assert_eq!(relative_rows[&DisplayRow(2)], 3);
    }

    #[gpui::test]
    async fn test_vim_visual_selections(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&(sample_text(6, 6, 'a') + "\n"), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.read(cx).style().unwrap().clone());

        window
            .update(cx, |editor, window, cx| {
                editor.cursor_shape = CursorShape::Block;
                editor.change_selections(None, window, cx, |s| {
                    s.select_ranges([
                        Point::new(0, 0)..Point::new(1, 0),
                        Point::new(3, 2)..Point::new(3, 3),
                        Point::new(5, 6)..Point::new(6, 0),
                    ]);
                });
            })
            .unwrap();

        let (_, state) = cx.draw(
            point(px(500.), px(500.)),
            size(px(500.), px(500.)),
            |_, _| EditorElement::new(&editor, style),
        );

        assert_eq!(state.selections.len(), 1);
        let local_selections = &state.selections[0].1;
        assert_eq!(local_selections.len(), 3);
        // moves cursor back one line
        assert_eq!(
            local_selections[0].head,
            DisplayPoint::new(DisplayRow(0), 6)
        );
        assert_eq!(
            local_selections[0].range,
            DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(1), 0)
        );

        // moves cursor back one column
        assert_eq!(
            local_selections[1].range,
            DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(3), 3)
        );
        assert_eq!(
            local_selections[1].head,
            DisplayPoint::new(DisplayRow(3), 2)
        );

        // leaves cursor on the max point
        assert_eq!(
            local_selections[2].range,
            DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(6), 0)
        );
        assert_eq!(
            local_selections[2].head,
            DisplayPoint::new(DisplayRow(6), 0)
        );

        // active lines does not include 1 (even though the range of the selection does)
        assert_eq!(
            state.active_rows.keys().cloned().collect::<Vec<_>>(),
            vec![DisplayRow(0), DisplayRow(3), DisplayRow(5), DisplayRow(6)]
        );
    }

    #[gpui::test]
    fn test_layout_with_placeholder_text_and_blocks(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.read(cx).style().unwrap().clone());
        window
            .update(cx, |editor, window, cx| {
                editor.set_placeholder_text("hello", cx);
                editor.insert_blocks(
                    [BlockProperties {
                        style: BlockStyle::Fixed,
                        placement: BlockPlacement::Above(Anchor::min()),
                        height: Some(3),
                        render: Arc::new(|cx| div().h(3. * cx.window.line_height()).into_any()),
                        priority: 0,
                    }],
                    None,
                    cx,
                );

                // Blur the editor so that it displays placeholder text.
                window.blur();
            })
            .unwrap();

        let (_, state) = cx.draw(
            point(px(500.), px(500.)),
            size(px(500.), px(500.)),
            |_, _| EditorElement::new(&editor, style),
        );
        assert_eq!(state.position_map.line_layouts.len(), 4);
        assert_eq!(state.line_numbers.len(), 1);
        assert_eq!(
            state
                .line_numbers
                .get(&MultiBufferRow(0))
                .map(|line_number| line_number.shaped_line.text.as_ref()),
            Some("1")
        );
    }

    #[gpui::test]
    fn test_all_invisibles_drawing(cx: &mut TestAppContext) {
        const TAB_SIZE: u32 = 4;

        let input_text = "\t \t|\t| a b";
        let expected_invisibles = vec![
            Invisible::Tab {
                line_start_offset: 0,
                line_end_offset: TAB_SIZE as usize,
            },
            Invisible::Whitespace {
                line_offset: TAB_SIZE as usize,
            },
            Invisible::Tab {
                line_start_offset: TAB_SIZE as usize + 1,
                line_end_offset: TAB_SIZE as usize * 2,
            },
            Invisible::Tab {
                line_start_offset: TAB_SIZE as usize * 2 + 1,
                line_end_offset: TAB_SIZE as usize * 3,
            },
            Invisible::Whitespace {
                line_offset: TAB_SIZE as usize * 3 + 1,
            },
            Invisible::Whitespace {
                line_offset: TAB_SIZE as usize * 3 + 3,
            },
        ];
        assert_eq!(
            expected_invisibles.len(),
            input_text
                .chars()
                .filter(|initial_char| initial_char.is_whitespace())
                .count(),
            "Hardcoded expected invisibles differ from the actual ones in '{input_text}'"
        );

        for show_line_numbers in [true, false] {
            init_test(cx, |s| {
                s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
                s.defaults.tab_size = NonZeroU32::new(TAB_SIZE);
            });

            let actual_invisibles = collect_invisibles_from_new_editor(
                cx,
                EditorMode::full(),
                input_text,
                px(500.0),
                show_line_numbers,
            );

            assert_eq!(expected_invisibles, actual_invisibles);
        }
    }

    #[gpui::test]
    fn test_invisibles_dont_appear_in_certain_editors(cx: &mut TestAppContext) {
        init_test(cx, |s| {
            s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
            s.defaults.tab_size = NonZeroU32::new(4);
        });

        for editor_mode_without_invisibles in [
            EditorMode::SingleLine { auto_width: false },
            EditorMode::AutoHeight { max_lines: 100 },
        ] {
            for show_line_numbers in [true, false] {
                let invisibles = collect_invisibles_from_new_editor(
                    cx,
                    editor_mode_without_invisibles,
                    "\t\t\t| | a b",
                    px(500.0),
                    show_line_numbers,
                );
                assert!(
                    invisibles.is_empty(),
                    "For editor mode {editor_mode_without_invisibles:?} no invisibles was expected but got {invisibles:?}"
                );
            }
        }
    }

    #[gpui::test]
    fn test_wrapped_invisibles_drawing(cx: &mut TestAppContext) {
        let tab_size = 4;
        let input_text = "a\tbcd     ".repeat(9);
        let repeated_invisibles = [
            Invisible::Tab {
                line_start_offset: 1,
                line_end_offset: tab_size as usize,
            },
            Invisible::Whitespace {
                line_offset: tab_size as usize + 3,
            },
            Invisible::Whitespace {
                line_offset: tab_size as usize + 4,
            },
            Invisible::Whitespace {
                line_offset: tab_size as usize + 5,
            },
            Invisible::Whitespace {
                line_offset: tab_size as usize + 6,
            },
            Invisible::Whitespace {
                line_offset: tab_size as usize + 7,
            },
        ];
        let expected_invisibles = std::iter::once(repeated_invisibles)
            .cycle()
            .take(9)
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(
            expected_invisibles.len(),
            input_text
                .chars()
                .filter(|initial_char| initial_char.is_whitespace())
                .count(),
            "Hardcoded expected invisibles differ from the actual ones in '{input_text}'"
        );
        info!("Expected invisibles: {expected_invisibles:?}");

        init_test(cx, |_| {});

        // Put the same string with repeating whitespace pattern into editors of various size,
        // take deliberately small steps during resizing, to put all whitespace kinds near the wrap point.
        let resize_step = 10.0;
        let mut editor_width = 200.0;
        while editor_width <= 1000.0 {
            for show_line_numbers in [true, false] {
                update_test_language_settings(cx, |s| {
                    s.defaults.tab_size = NonZeroU32::new(tab_size);
                    s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
                    s.defaults.preferred_line_length = Some(editor_width as u32);
                    s.defaults.soft_wrap = Some(language_settings::SoftWrap::PreferredLineLength);
                });

                let actual_invisibles = collect_invisibles_from_new_editor(
                    cx,
                    EditorMode::full(),
                    &input_text,
                    px(editor_width),
                    show_line_numbers,
                );

                // Whatever the editor size is, ensure it has the same invisible kinds in the same order
                // (no good guarantees about the offsets: wrapping could trigger padding and its tests should check the offsets).
                let mut i = 0;
                for (actual_index, actual_invisible) in actual_invisibles.iter().enumerate() {
                    i = actual_index;
                    match expected_invisibles.get(i) {
                        Some(expected_invisible) => match (expected_invisible, actual_invisible) {
                            (Invisible::Whitespace { .. }, Invisible::Whitespace { .. })
                            | (Invisible::Tab { .. }, Invisible::Tab { .. }) => {}
                            _ => {
                                panic!(
                                    "At index {i}, expected invisible {expected_invisible:?} does not match actual {actual_invisible:?} by kind. Actual invisibles: {actual_invisibles:?}"
                                )
                            }
                        },
                        None => {
                            panic!("Unexpected extra invisible {actual_invisible:?} at index {i}")
                        }
                    }
                }
                let missing_expected_invisibles = &expected_invisibles[i + 1..];
                assert!(
                    missing_expected_invisibles.is_empty(),
                    "Missing expected invisibles after index {i}: {missing_expected_invisibles:?}"
                );

                editor_width += resize_step;
            }
        }
    }

    fn collect_invisibles_from_new_editor(
        cx: &mut TestAppContext,
        editor_mode: EditorMode,
        input_text: &str,
        editor_width: Pixels,
        show_line_numbers: bool,
    ) -> Vec<Invisible> {
        info!(
            "Creating editor with mode {editor_mode:?}, width {}px and text '{input_text}'",
            editor_width.0
        );
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(input_text, cx);
            Editor::new(editor_mode, buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();

        let style = cx.update(|_, cx| editor.read(cx).style().unwrap().clone());
        window
            .update(cx, |editor, _, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_wrap_width(Some(editor_width), cx);
                editor.set_show_line_numbers(show_line_numbers, cx);
            })
            .unwrap();
        let (_, state) = cx.draw(
            point(px(500.), px(500.)),
            size(px(500.), px(500.)),
            |_, _| EditorElement::new(&editor, style),
        );
        state
            .position_map
            .line_layouts
            .iter()
            .flat_map(|line_with_invisibles| &line_with_invisibles.invisibles)
            .cloned()
            .collect()
    }
}
