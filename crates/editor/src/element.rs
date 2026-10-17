mod header;
mod mouse;
mod prepared_text;

pub(crate) use prepared_text::{LineChunk, PreparedRendererConstraint, PreparedRowText};

#[cfg(test)]
pub(crate) use header::StickyHeader;
pub use header::file_status_label_color;
pub(crate) use header::{header_jump_data, render_buffer_header};

use crate::{
    BUFFER_HEADER_PADDING, BlockId, ChunkRenderer, ChunkRendererContext, ChunkReplacement,
    CodeActionSource, ConflictsOurs, ConflictsOursMarker, ConflictsOuter, ConflictsTheirs,
    ConflictsTheirsMarker, ContextMenuPlacement, CursorShape, CustomBlockId, DisplayDiffHunk,
    DisplayPoint, DisplayRow, EditDisplayMode, EditPrediction, Editor, EditorMode, EditorSettings,
    EditorSnapshot, EditorStyle, FILE_HEADER_HEIGHT, FocusedBlock, GutterDimensions, HalfPageDown,
    HalfPageUp, HandleInput, HoveredCursor, InlayHintRefreshReason, LineDown, LineHighlight,
    LineUp, MAX_LINE_LEN, MINIMAP_FONT_SIZE, PageDown, PageUp, Point, RendererWidthClass, RowExt,
    RowRangeExt, Selection, SelectionDragState, SizingBehavior, SoftWrap, ToPoint,
    code_context_menus::{CodeActionsMenu, MENU_ASIDE_MAX_WIDTH, MENU_ASIDE_MIN_WIDTH, MENU_GAP},
    column_pixels,
    cursor_animation::{CursorViewport, LogicalCursorPosition, animated_corners_overlap_target},
    display_map::{
        Block, BlockContext, BlockStyle, ChunkRendererId, DisplaySnapshot, EditorMargins,
        HighlightKey, HighlightedChunk, LongRow, PendingRowGeometry, RendererWidthsKey, RowMetrics,
        ToDisplayPoint,
    },
    editor_settings::{
        CurrentLineHighlight, DocumentColorsRenderMode, GitGutterWidth, Minimap, MinimapThumb,
        MinimapThumbBorder, ScrollBeyondLastLine, ScrollbarAxes, ScrollbarDiagnostics, ShowMinimap,
    },
    git::blame::{BlameRenderer, GitBlame, GlobalBlameRenderer},
    hover_popover::{
        self, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT,
        POPOVER_RIGHT_OFFSET,
    },
    inlay_hint_settings,
    scroll::{
        ActiveScrollbarState, Autoscroll, ScrollOffset, ScrollPixelOffset, ScrollbarThumbState,
        scroll_amount::ScrollAmount,
    },
};
use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
use collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use feature_flags::{DiffReviewFeatureFlag, FeatureFlagAppExt as _};
use git::{Oid, blame::BlameEntry, commit::ParsedCommitMessage};
use gpui::{
    Action, Along, AnyElement, App, AppContext, AvailableSpace, Axis as ScrollbarAxis, Background,
    BorderStyle, Bounds, ClipboardItem, ContentMask, Context, Corners, CursorStyle, DispatchPhase,
    Edges, Element, ElementInputHandler, Entity, Focusable as _, Font, FontId, FontWeight,
    GlobalElementId, Hitbox, HitboxBehavior, Hsla, InteractiveElement, IntoElement, IsZero,
    LineLayout, ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PaintQuad, ParentElement, Pixels, ScaledPixels, ScrollHandle, ShapedLine, ShapedRun,
    SharedString, Size, StatefulInteractiveElement, Style, Styled, StyledText, TaskExt, TextAlign,
    TextRun, TextStyle, TextStyleRefinement, TextSystem, TextUnderline, UnderlineStyle, WeakEntity,
    Window, div, fill, outline, pattern_slash, point, px, quad, relative, size, solid_background,
    transparent_black, underline_y_offset,
};
use itertools::Itertools;
use language::{
    Diagnostic, HighlightedText, IndentGuideSettings, LanguageAwareStyling,
    language_settings::ShowWhitespaceSetting,
};
use markdown::Markdown;
use multi_buffer::{
    Anchor, ExcerptRange, ExpandExcerptDirection, ExpandInfo, MultiBufferOffset, MultiBufferPoint,
    MultiBufferRow, MultiBufferSnapshot, RowInfo, ToOffset,
};

use project::{
    debugger::breakpoint_store::{Breakpoint, BreakpointSessionState},
    project_settings::{InlineBlameLocation, ProjectSettings},
};
use settings::{
    GitGutterSetting, GitHunkStyleSetting, IndentGuideBackgroundColoring, IndentGuideColoring,
    Settings,
};
use smallvec::{SmallVec, smallvec};
use std::{
    any::TypeId,
    borrow::Cow,
    cell::Cell,
    cmp::{self, Ordering},
    fmt::{self, Write},
    iter, mem,
    ops::{ControlFlow, Deref, Range},
    rc::Rc,
    slice,
    sync::Arc,
    time::{Duration, Instant},
};
use sum_tree::Bias;
use text::{BufferId, ToPoint as _};
use theme::{ActiveTheme, Appearance, PlayerColor};
use theme_settings::BufferLineHeight;
use ui::utils::ensure_minimum_contrast;
use ui::{ButtonLike, POPOVER_Y_PADDING, Tooltip, prelude::*, scrollbars::ShowScrollbar};
use unicode_segmentation::UnicodeSegmentation;
use util::{ResultExt, debug_panic};
use workspace::{
    CollaboratorId, ItemHandle, Workspace,
    item::{Item, ItemBufferKind},
};

/// Determines what kinds of highlights should be applied to a lines background.
#[derive(Clone, Copy, Default)]
struct LineHighlightSpec {
    selection: bool,
    breakpoint: bool,
    _active_stack_frame: bool,
}

enum LineNumberStyle {
    Breakpoint,
    DiffAdded,
    DiffDeleted,
    Active,
    Inactive,
}

impl LineNumberStyle {
    fn new(is_active: bool, is_breakpoint: bool, diff_status: Option<DiffHunkStatus>) -> Self {
        match (
            is_active,
            is_breakpoint,
            diff_status.map(|status| status.kind),
        ) {
            (_, true, _) => Self::Breakpoint,
            (true, _, _) => Self::Active,
            (_, _, Some(DiffHunkStatusKind::Added)) => Self::DiffAdded,
            (_, _, Some(DiffHunkStatusKind::Deleted)) => Self::DiffDeleted,
            (_, _, _) => Self::Inactive,
        }
    }

    fn color(self, colors: &theme::ThemeColors) -> Hsla {
        match self {
            Self::Breakpoint => colors.debugger_accent,
            Self::DiffAdded => colors.version_control_added,
            Self::DiffDeleted => colors.version_control_deleted,
            Self::Active => colors.editor_active_line_number,
            Self::Inactive => colors.editor_line_number,
        }
    }
}

pub(crate) struct SelectionGeometry {
    pub(crate) head: DisplayPoint,
    range: Range<DisplayPoint>,
    active_rows: Range<DisplayRow>,
}

#[derive(Debug)]
struct SelectionLayout {
    id: usize,
    head: DisplayPoint,
    cursor_shape: CursorShape,
    is_newest: bool,
    is_local: bool,
    range: Range<DisplayPoint>,
    active_rows: Range<DisplayRow>,
    user_name: Option<SharedString>,
}

struct InlineBlameLayout {
    element: AnyElement,
    bounds: Bounds<Pixels>,
    buffer_id: BufferId,
    entry: BlameEntry,
}

#[derive(Debug, Clone, Copy)]
struct PointDiagnostic {
    column: u32,
    underline: UnderlineStyle,
    severity: lsp::DiagnosticSeverity,
}

struct DiagnosticUnderline {
    underline: TextUnderline,
    bounds: Bounds<ScaledPixels>,
    span: Range<ScaledPixels>,
    severity: lsp::DiagnosticSeverity,
    is_point: bool,
}

impl DiagnosticUnderline {
    fn point(
        diagnostic: &PointDiagnostic,
        origin: gpui::Point<Pixels>,
        width: Pixels,
        window: &Window,
    ) -> Self {
        let bounds = window.underline_bounds(origin, width, &diagnostic.underline);
        Self {
            underline: TextUnderline::new(origin, width, diagnostic.underline),
            bounds,
            span: bounds.left()..bounds.right(),
            severity: diagnostic.severity,
            is_point: true,
        }
    }

    fn paint(&self, span: Range<ScaledPixels>, window: &mut Window) {
        self.underline.paint_with_exclusions(
            &[
                self.bounds.left()..span.start,
                span.end..self.bounds.right(),
            ],
            window,
        );
    }

    fn paint_all(underlines: &[Self], window: &mut Window) {
        if underlines.is_empty() {
            return;
        }
        let mask = window.content_mask().bounds;
        let scale = window.scale_factor();
        let left = ScaledPixels((f32::from(mask.left()) * scale).floor());
        let top = ScaledPixels((f32::from(mask.top()) * scale).floor());
        let right = ScaledPixels((f32::from(mask.right()) * scale).ceil()).max(left);
        let bottom = ScaledPixels((f32::from(mask.bottom()) * scale).ceil()).max(top);
        let mask = Bounds::from_corners(point(left, top), point(right, bottom));
        let mut events = SmallVec::<[_; 8]>::new();
        for (index, underline) in underlines.iter().enumerate() {
            let bounds = underline.bounds.intersect(&mask);
            let start = underline.span.start.max(bounds.left());
            let end = underline.span.end.min(bounds.right());
            if !bounds.is_empty() && start < end {
                events.push((start, true, index));
                events.push((end, false, index));
            }
        }
        events.sort_unstable();

        let mut active = BTreeSet::new();
        let mut previous = None;
        let mut visible: Option<(usize, Range<ScaledPixels>)> = None;
        for (position, is_start, index) in events {
            if let Some(start) = previous
                && start < position
                && let Some(&(_, _, winner)) = active.first()
            {
                if let Some((current, span)) = visible.as_mut()
                    && *current == winner
                    && span.end == start
                {
                    span.end = position;
                } else {
                    if let Some((current, span)) = visible.take() {
                        underlines[current].paint(span, window);
                    }
                    visible = Some((winner, start..position));
                }
            }
            let underline = &underlines[index];
            let key = (underline.severity, underline.is_point, index);
            if is_start {
                active.insert(key);
            } else {
                active.remove(&key);
            }
            previous = Some(position);
        }
        if let Some((winner, span)) = visible {
            underlines[winner].paint(span, window);
        }
    }
}

impl SelectionGeometry {
    pub(crate) fn new<T: ToPoint + ToDisplayPoint + Clone>(
        selection: Selection<T>,
        line_mode: bool,
        cursor_offset: bool,
        map: &DisplaySnapshot,
    ) -> Self {
        let buffer_snapshot = map.buffer_snapshot();
        let point_selection = selection.map(|p| p.to_point(buffer_snapshot));
        let display_selection = point_selection.map(|p| p.to_display_point(map));
        let mut range = display_selection.range();
        let mut head = display_selection.head();
        if !line_mode {
            let offset_range = point_selection.start.to_offset(buffer_snapshot)
                ..point_selection.end.to_offset(buffer_snapshot);
            if let Some(contiguous_range) =
                map.contiguous_display_point_range_for_buffer_range(offset_range)
            {
                range = contiguous_range;
                // Keep the cursor attached to the highlight boundary; the
                // anchor-bias display position may sit on the far side of a
                // boundary inlay the highlight excludes.
                head = if selection.reversed {
                    range.start
                } else {
                    range.end
                };
            }
        }
        let mut active_rows = map.prev_line_boundary(point_selection.start).1.row()
            ..map.next_line_boundary(point_selection.end).1.row();

        // vim visual line mode
        if line_mode {
            let point_range = map.expand_to_line(point_selection.range());
            range = point_range.start.to_display_point(map)..point_range.end.to_display_point(map);
        }

        // any vim visual mode (including line mode)
        if cursor_offset && !range.is_empty() && !selection.reversed {
            if head.column() > 0 {
                head = map.clip_point(DisplayPoint::new(head.row(), head.column() - 1), Bias::Left);
            } else if head.row().0 > 0 {
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
            range,
            active_rows,
        }
    }
}

impl SelectionLayout {
    fn new<T: ToPoint + ToDisplayPoint + Clone>(
        selection: Selection<T>,
        line_mode: bool,
        cursor_offset: bool,
        cursor_shape: CursorShape,
        map: &DisplaySnapshot,
        is_newest: bool,
        is_local: bool,
        user_name: Option<SharedString>,
    ) -> Self {
        let id = selection.id;
        let SelectionGeometry {
            head,
            range,
            active_rows,
        } = SelectionGeometry::new(selection, line_mode, cursor_offset, map);
        Self {
            id,
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

#[derive(Default)]
struct RenderBlocksOutput {
    // We store spacer blocks separately because they paint in a different order
    // (spacers -> indent guides -> non-spacers)
    non_spacer_blocks: Vec<BlockLayout>,
    spacer_blocks: Vec<BlockLayout>,
    row_block_types: HashMap<DisplayRow, bool>,
    resized_blocks: Option<HashMap<CustomBlockId, u32>>,
}

pub struct EditorElement {
    editor: Entity<Editor>,
    style: EditorStyle,
    split_side: Option<SplitSide>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitSide {
    Left,
    Right,
}

impl EditorElement {
    pub(crate) const SCROLLBAR_WIDTH: Pixels = ui::EDITOR_SCROLLBAR_WIDTH;

    pub fn new(editor: &Entity<Editor>, style: EditorStyle) -> Self {
        Self {
            editor: editor.clone(),
            style,
            split_side: None,
        }
    }

    pub fn set_split_side(&mut self, side: SplitSide) {
        self.split_side = Some(side);
    }

    fn register_actions(&self, window: &mut Window, cx: &mut App) {
        let editor = &self.editor;
        editor.update(cx, |editor, cx| {
            for action in editor.editor_actions.borrow().values() {
                (action)(editor, false, window, cx)
            }
        });

        crate::rust_analyzer_ext::apply_related_actions(editor, window, cx);
        crate::clangd_ext::apply_related_actions(editor, window, cx);
        crate::emmet_ext::apply_related_actions(editor, window, cx);

        if editor.read(cx).pending_inline_input().is_some() {
            register_action(editor, window, |editor, _: &menu::Confirm, window, cx| {
                editor.confirm_inline_input(window, cx);
            });
        }

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
        register_action(editor, window, Editor::blame_hover);
        register_action(editor, window, Editor::next_snippet_tabstop);
        register_action(editor, window, Editor::previous_snippet_tabstop);
        register_action(editor, window, Editor::copy);
        register_action(editor, window, Editor::copy_and_trim);
        register_action(editor, window, Editor::diff_clipboard_with_selection);
        register_action(editor, window, Editor::move_page_up);
        register_action(editor, window, Editor::move_page_down);
        register_action(editor, window, Editor::next_screen);
        register_action(editor, window, Editor::scroll_cursor_top);
        register_action(editor, window, Editor::scroll_cursor_center);
        register_action(editor, window, Editor::scroll_cursor_bottom);
        register_action(editor, window, Editor::scroll_cursor_center_top_bottom);
        register_action(editor, window, |editor, _: &LineDown, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Line(1.), window, cx)
        });
        register_action(editor, window, |editor, _: &LineUp, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Line(-1.), window, cx)
        });
        register_action(editor, window, |editor, _: &HalfPageDown, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Page(0.5), window, cx)
        });
        register_action(editor, window, |editor, _: &HalfPageUp, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Page(-0.5), window, cx)
        });
        register_action(editor, window, |editor, _: &PageDown, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Page(1.), window, cx)
        });
        register_action(editor, window, |editor, _: &PageUp, window, cx| {
            editor.scroll_screen_with_cursor_margin(&ScrollAmount::Page(-1.), window, cx)
        });
        register_action(editor, window, Editor::move_to_previous_word_start);
        register_action(editor, window, Editor::move_to_previous_subword_start);
        register_action(editor, window, Editor::move_to_next_word_end);
        register_action(editor, window, Editor::move_to_next_subword_end);
        register_action(editor, window, Editor::move_to_beginning_of_line);
        register_action(editor, window, Editor::move_to_end_of_line);
        register_action(editor, window, Editor::move_to_start_of_paragraph);
        register_action(editor, window, Editor::move_to_end_of_paragraph);
        register_action(editor, window, Editor::move_to_next_comment_paragraph);
        register_action(editor, window, Editor::move_to_previous_comment_paragraph);
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
        register_action(editor, window, Editor::insert_snippet_at_selections);
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
        register_action(editor, window, Editor::select_larger_syntax_node);
        register_action(editor, window, Editor::select_smaller_syntax_node);
        register_action(editor, window, Editor::select_next_syntax_node);
        register_action(editor, window, Editor::select_prev_syntax_node);
        register_action(
            editor,
            window,
            Editor::select_to_start_of_larger_syntax_node,
        );
        register_action(editor, window, Editor::select_to_end_of_larger_syntax_node);
        register_action(editor, window, Editor::move_to_start_of_larger_syntax_node);
        register_action(editor, window, Editor::move_to_end_of_larger_syntax_node);
        register_action(editor, window, Editor::select_enclosing_symbol);
        register_action(editor, window, Editor::move_to_enclosing_bracket);
        register_action(editor, window, Editor::select_inside_delimiters);
        register_action(editor, window, Editor::select_around_delimiters);
        register_action(editor, window, Editor::undo_selection);
        register_action(editor, window, Editor::redo_selection);
        if editor.read(cx).buffer_kind(cx) == ItemBufferKind::Multibuffer {
            register_action(editor, window, Editor::expand_excerpts);
            register_action(editor, window, Editor::expand_excerpts_up);
            register_action(editor, window, Editor::expand_excerpts_down);
        }
        register_action(editor, window, Editor::go_to_diagnostic);
        register_action(editor, window, Editor::go_to_prev_diagnostic);
        register_action(editor, window, Editor::go_to_next_hunk);
        register_action(editor, window, Editor::go_to_prev_hunk);
        register_action(editor, window, Editor::go_to_next_document_highlight);
        register_action(editor, window, Editor::go_to_prev_document_highlight);
        if editor.read(cx).lsp_data_enabled() {
            register_action(editor, window, Editor::open_definition_locations);
        }
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
        register_action(editor, window, Editor::fold_at_level_1);
        register_action(editor, window, Editor::fold_at_level_2);
        register_action(editor, window, Editor::fold_at_level_3);
        register_action(editor, window, Editor::fold_at_level_4);
        register_action(editor, window, Editor::fold_at_level_5);
        register_action(editor, window, Editor::fold_at_level_6);
        register_action(editor, window, Editor::fold_at_level_7);
        register_action(editor, window, Editor::fold_at_level_8);
        register_action(editor, window, Editor::fold_at_level_9);
        register_action(editor, window, Editor::fold_all);
        register_action(editor, window, Editor::fold_function_bodies);
        register_action(editor, window, Editor::fold_recursive);
        register_action(editor, window, Editor::toggle_fold);
        register_action(editor, window, Editor::toggle_fold_recursive);
        register_action(editor, window, Editor::toggle_fold_all);
        register_action(editor, window, Editor::unfold_lines);
        register_action(editor, window, Editor::unfold_recursive);
        register_action(editor, window, Editor::unfold_all);
        register_action(editor, window, Editor::fold_selected_ranges);
        register_action(editor, window, Editor::set_mark);
        register_action(editor, window, Editor::save_location);
        register_action(editor, window, Editor::swap_selection_ends);
        register_action(editor, window, Editor::show_completions);
        register_action(editor, window, Editor::show_word_completions);
        register_action(editor, window, Editor::toggle_code_actions);
        register_action(editor, window, Editor::open_excerpts);
        register_action(editor, window, Editor::open_excerpts_in_split);
        register_action(editor, window, Editor::toggle_soft_wrap);
        register_action(editor, window, Editor::toggle_tab_bar);
        register_action(editor, window, Editor::toggle_breadcrumb);
        register_action(editor, window, Editor::toggle_line_numbers);
        register_action(editor, window, Editor::toggle_relative_line_numbers);
        register_action(editor, window, Editor::toggle_indent_guides);
        register_action(editor, window, Editor::toggle_inline_values);
        register_action(editor, window, Editor::toggle_edit_predictions);
        if editor.read(cx).lsp_data_enabled() {
            register_action(editor, window, Editor::toggle_inlay_hints);
            if editor.read(cx).can_accept_inlay_hint(cx) {
                register_action(editor, window, Editor::accept_inlay_hint);
            }
            register_action(editor, window, Editor::toggle_code_lens_action);
            register_action(editor, window, Editor::toggle_semantic_highlights);
            register_action(editor, window, Editor::toggle_diagnostics);
        }
        if editor.read(cx).inline_diagnostics_enabled() {
            register_action(editor, window, Editor::toggle_inline_diagnostics);
        }
        if editor.read(cx).supports_minimap(cx) {
            register_action(editor, window, Editor::toggle_minimap);
        }
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
        if editor.read(cx).blame().is_some() {
            register_action(editor, window, Editor::open_git_blame_commit);
            if editor.update(cx, |editor, cx| {
                editor.blame_revision_target(window, cx).is_some()
            }) {
                register_action(editor, window, Editor::blame_revision);
            }
            if editor.update(cx, |editor, cx| {
                editor.blame_previous_revision_target(window, cx).is_some()
            }) {
                register_action(editor, window, Editor::blame_previous_revision);
            }
        }
        register_action(editor, window, Editor::toggle_selected_diff_hunks);
        register_action(editor, window, Editor::toggle_staged_selected_diff_hunks);
        register_action(editor, window, Editor::stage_and_next);
        register_action(editor, window, Editor::unstage_and_next);
        register_action(editor, window, Editor::expand_all_diff_hunks);
        register_action(editor, window, Editor::collapse_all_diff_hunks);
        register_action(editor, window, Editor::toggle_all_diff_hunks);
        register_action(editor, window, Editor::toggle_review_comments_expanded);
        register_action(editor, window, Editor::submit_diff_review_comment_action);
        register_action(editor, window, Editor::edit_review_comment);
        register_action(editor, window, Editor::delete_review_comment);
        register_action(editor, window, Editor::confirm_edit_review_comment_action);
        register_action(editor, window, Editor::cancel_edit_review_comment_action);
        register_action(editor, window, Editor::go_to_previous_change);
        register_action(editor, window, Editor::go_to_next_change);
        register_action(editor, window, Editor::go_to_prev_reference);
        register_action(editor, window, Editor::go_to_next_reference);
        register_action(editor, window, Editor::go_to_previous_symbol);
        register_action(editor, window, Editor::go_to_next_symbol);
        register_action(editor, window, Editor::restart_language_server);
        register_action(editor, window, Editor::stop_language_server);
        register_action(editor, window, Editor::show_character_palette);
        register_action(editor, window, |editor, action, window, cx| {
            if let Some(task) = editor.compose_completion(action, window, cx) {
                editor.detach_and_notify_err(task, window, cx);
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
        register_action(editor, window, Editor::signature_help_prev);
        register_action(editor, window, Editor::signature_help_next);
        register_action(editor, window, Editor::show_edit_prediction);
        register_action(editor, window, Editor::context_menu_first);
        register_action(editor, window, Editor::context_menu_prev);
        register_action(editor, window, Editor::context_menu_next);
        register_action(editor, window, Editor::context_menu_last);
        register_action(editor, window, Editor::display_cursor_names);
        register_action(editor, window, Editor::open_active_item_in_terminal);
        register_action(editor, window, Editor::spawn_nearest_task);
        register_action(editor, window, Editor::open_selections_in_multibuffer);
        register_action(editor, window, Editor::toggle_bookmark);
        register_action(editor, window, Editor::toggle_bookmark_with_label);
        register_action(editor, window, Editor::edit_bookmark);
        register_action(editor, window, Editor::go_to_next_bookmark);
        register_action(editor, window, Editor::go_to_previous_bookmark);
        register_action(editor, window, Editor::toggle_breakpoint);
        register_action(editor, window, Editor::edit_log_breakpoint);
        register_action(editor, window, Editor::enable_breakpoint);
        register_action(editor, window, Editor::disable_breakpoint);
        register_action(editor, window, Editor::toggle_read_only);
        register_action(editor, window, Editor::reload_file);

        if !editor.read(cx).read_only(cx) {
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
            register_action(editor, window, Editor::sort_lines_by_length);
            register_action(editor, window, Editor::sort_lines_case_sensitive);
            register_action(editor, window, Editor::sort_lines_case_insensitive);
            register_action(editor, window, Editor::unique_lines_case_insensitive);
            register_action(editor, window, Editor::unique_lines_case_sensitive);
            register_action(editor, window, Editor::reverse_lines);
            register_action(editor, window, Editor::shuffle_lines);
            register_action(editor, window, Editor::rotate_selections_forward);
            register_action(editor, window, Editor::rotate_selections_backward);
            register_action(editor, window, Editor::convert_indentation_to_spaces);
            register_action(editor, window, Editor::convert_indentation_to_tabs);
            register_action(editor, window, Editor::convert_to_upper_case);
            register_action(editor, window, Editor::convert_to_lower_case);
            register_action(editor, window, Editor::convert_to_title_case);
            register_action(editor, window, Editor::convert_to_snake_case);
            register_action(editor, window, Editor::convert_to_kebab_case);
            register_action(editor, window, Editor::convert_to_upper_camel_case);
            register_action(editor, window, Editor::convert_to_lower_camel_case);
            register_action(editor, window, Editor::convert_to_opposite_case);
            register_action(editor, window, Editor::convert_to_sentence_case);
            register_action(editor, window, Editor::toggle_case);
            register_action(editor, window, Editor::convert_to_rot13);
            register_action(editor, window, Editor::convert_to_rot47);
            register_action(editor, window, Editor::convert_to_base64);
            register_action(editor, window, Editor::convert_from_base64);
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
            register_action(editor, window, |editor, _: &crate::Rewrap, _, cx| {
                editor.rewrap(crate::RewrapOptions::default(), cx);
            });
            register_action(editor, window, Editor::cut);
            register_action(editor, window, Editor::kill_ring_cut);
            register_action(editor, window, Editor::kill_ring_yank);
            register_action(editor, window, Editor::paste);
            register_recovery_action(editor, window, Editor::undo);
            register_recovery_action(editor, window, Editor::redo);
            register_action(editor, window, Editor::toggle_comments);
            register_action(editor, window, Editor::toggle_block_comments);
            register_action(editor, window, Editor::toggle_markdown_block_quote);
            register_action(editor, window, Editor::unwrap_syntax_node);
            register_action(editor, window, Editor::accept_next_word_edit_prediction);
            register_action(editor, window, Editor::accept_next_line_edit_prediction);
            register_action(editor, window, Editor::accept_edit_prediction);
            register_action(editor, window, Editor::restore_file);
            register_action(editor, window, Editor::git_restore);
            register_action(editor, window, Editor::restore_and_next);
            register_action(editor, window, Editor::apply_all_diff_hunks);
            register_action(editor, window, Editor::apply_selected_diff_hunks);
            register_action(editor, window, Editor::insert_uuid_v4);
            register_action(editor, window, Editor::insert_uuid_v7);
            register_action(editor, window, Editor::align_selections);
            if editor.read(cx).enable_wrap_selections_in_tag(cx) {
                register_action(editor, window, Editor::wrap_selections_in_tag);
            }
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
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.format(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            if editor.read(cx).can_format_selections(cx) {
                register_action(editor, window, |editor, action, window, cx| {
                    if let Some(task) = editor.format_selections(action, window, cx) {
                        editor.detach_and_notify_err(task, window, cx);
                    } else {
                        cx.propagate();
                    }
                });
            }
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.organize_imports(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.confirm_completion(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.confirm_completion_replace(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.confirm_completion_insert(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.confirm_code_action(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.rename(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
            register_action(editor, window, |editor, action, window, cx| {
                if let Some(task) = editor.confirm_rename(action, window, cx) {
                    editor.detach_and_notify_err(task, window, cx);
                } else {
                    cx.propagate();
                }
            });
        }
    }

    fn register_key_listeners(&self, window: &mut Window, position_map: Option<Rc<PositionMap>>) {
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

                    if let Some(position_map) = &position_map {
                        editor.handle_modifiers_changed(event.modifiers, position_map, window, cx);
                    } else {
                        editor.update_edit_prediction_preview(&event.modifiers, window, cx);
                    }
                })
            }
        });
    }

    fn layout_selections(
        &self,
        start_anchor: Anchor,
        end_anchor: Anchor,
        visible_ranges: &[Range<Anchor>],
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

        let Some(editor_with_selections) = self.editor_with_selections(cx) else {
            return (selections, active_rows, newest_selection_head);
        };

        let visible_range = start_anchor..end_anchor;
        editor_with_selections.update(cx, |editor, cx| {
            if editor.show_local_selections {
                let mut layouts = Vec::new();
                let newest = editor.selections.newest(&editor.display_snapshot(cx));
                for selection in local_selections.iter().cloned() {
                    let is_empty = selection.start == selection.end;
                    let is_newest = selection == newest;

                    let layout = SelectionLayout::new(
                        selection,
                        editor.selections.line_mode(),
                        editor.cursor_offset_on_selection,
                        editor.cursor_shape,
                        &snapshot.display_snapshot,
                        is_newest,
                        editor.leader_id.is_none(),
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

                let mut player = editor.current_user_player_color(cx);
                if !editor.is_focused(window) {
                    const UNFOCUS_EDITOR_SELECTION_OPACITY: f32 = 0.5;
                    player.selection = player.selection.opacity(UNFOCUS_EDITOR_SELECTION_OPACITY);
                }
                selections.push((player, layouts));

                if let SelectionDragState::Dragging {
                    ref selection,
                    ref drop_cursor,
                    ref hide_drop_cursor,
                } = editor.selection_drag_state
                    && !hide_drop_cursor
                    && (drop_cursor
                        .start
                        .cmp(&selection.start, &snapshot.buffer_snapshot())
                        .eq(&Ordering::Less)
                        || drop_cursor
                            .end
                            .cmp(&selection.end, &snapshot.buffer_snapshot())
                            .eq(&Ordering::Greater))
                {
                    let drag_cursor_layout = SelectionLayout::new(
                        *drop_cursor,
                        false,
                        editor.cursor_offset_on_selection,
                        CursorShape::Bar,
                        &snapshot.display_snapshot,
                        false,
                        false,
                        None,
                    );
                    let absent_color = cx.theme().players().absent();
                    selections.push((absent_color, vec![drag_cursor_layout]));
                }
            }

            if let Some(collaboration_hub) = &editor.collaboration_hub {
                // When following someone, render the local selections in their color.
                if let Some(leader_id) = editor.leader_id {
                    match leader_id {
                        CollaboratorId::PeerId(peer_id) => {
                            if let Some(collaborator) =
                                collaboration_hub.collaborators(cx).get(&peer_id)
                                && let Some(participant_index) = collaboration_hub
                                    .user_participant_indices(cx)
                                    .get(&collaborator.user_id)
                                && let Some((local_selection_style, _)) = selections.first_mut()
                            {
                                *local_selection_style = cx
                                    .theme()
                                    .players()
                                    .color_for_participant(participant_index.0);
                            }
                        }
                        CollaboratorId::Agent => {
                            if let Some((local_selection_style, _)) = selections.first_mut() {
                                *local_selection_style = cx.theme().players().agent();
                            }
                        }
                    }
                }

                let mut remote_selections = HashMap::default();
                for selection in snapshot.remote_selections_in_ranges(
                    visible_ranges,
                    &visible_range,
                    collaboration_hub.as_ref(),
                    cx,
                ) {
                    // Don't re-render the leader's selections, since the local selections
                    // match theirs.
                    if Some(selection.collaborator_id) == editor.leader_id {
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
                        .or_insert((selection.color, Vec::new()))
                        .1
                        .push(SelectionLayout::new(
                            selection.selection,
                            selection.line_mode,
                            editor.cursor_offset_on_selection,
                            selection.cursor_shape,
                            &snapshot.display_snapshot,
                            false,
                            false,
                            if is_shown { selection.user_name } else { None },
                        ));
                }

                selections.extend(remote_selections.into_values());
            } else if !editor.is_focused(window) && editor.show_cursor_when_unfocused {
                let cursor_offset_on_selection = editor.cursor_offset_on_selection;

                let layouts = snapshot
                    .buffer_snapshot()
                    .selections_in_ranges(visible_ranges, &visible_range, true)
                    .map(move |(_, line_mode, cursor_shape, selection)| {
                        SelectionLayout::new(
                            selection,
                            line_mode,
                            cursor_offset_on_selection,
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

        #[cfg(debug_assertions)]
        Self::layout_debug_ranges(
            &mut selections,
            start_anchor..end_anchor,
            &snapshot.display_snapshot,
            cx,
        );

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
                &(Anchor::Min..Anchor::Max),
                collaboration_hub.deref(),
                cx,
            ) {
                add_cursor(
                    remote_selection.selection.head(),
                    remote_selection.color.cursor,
                );
                if Some(remote_selection.collaborator_id) == editor.leader_id {
                    skip_local = true;
                }
            }
        }
        // Local cursors
        if !skip_local {
            let color = cx.theme().players().local().cursor;
            editor
                .selections
                .disjoint_anchors()
                .iter()
                .for_each(|selection| {
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
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_height: Pixels,
        em_width: Pixels,
        em_advance: Pixels,
        autoscroll_containing_element: bool,
        redacted_ranges: &[Range<DisplayPoint>],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Vec<CursorLayout>, PendingRowGeometry> {
        let mut autoscroll_bounds = None;
        let cursor_layouts = self.editor.update(cx, |editor, cx| {
            let mut cursors = Vec::new();
            let mut handled_animation_cursors = HashSet::default();
            let mut request_animation_frame = false;

            let show_local_cursors = editor.show_local_cursors(window, cx);
            let animation_settings = EditorSettings::get_global(cx).cursor_animation;
            let animation_enabled = animation_settings.enabled && !cx.reduce_motion();
            let animation_context = animation_enabled.then(|| {
                (
                    CursorViewport::new(
                        content_origin,
                        text_hitbox.bounds,
                        scroll_position,
                        scroll_pixel_position,
                        line_height,
                        em_advance,
                    ),
                    Instant::now(),
                )
            });

            if animation_enabled {
                let newest_animation_selection_id = if editor.leader_id.is_none()
                    && cursor_shape_supports_cursor_animation(editor.cursor_shape)
                {
                    Some(editor.selections.newest_anchor().id)
                } else {
                    None
                };

                editor
                    .cursor_animations
                    .reconcile_newest_selection(newest_animation_selection_id);
            } else {
                editor.cursor_animations.clear();
            }

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

                    let alignment_offset = cursor_row_layout
                        .alignment_offset(self.style.text.text_align, text_hitbox.size.width);
                    let cursor_character_x =
                        cursor_row_layout.x_for_index(cursor_column) + alignment_offset;
                    let cursor_next_x =
                        cursor_row_layout.x_for_index(cursor_column + 1) + alignment_offset;
                    let mut block_width =
                        CursorLayout::cell_width(cursor_character_x, cursor_next_x, em_advance);
                    let mut block_text = None;
                    let mut retained_block_text = RetainedCursorText::default();
                    let x = Pixels::from(cursor_character_x - scroll_pixel_position.x);

                    let is_cursor_in_redacted_range = redacted_ranges
                        .iter()
                        .any(|range| range.start <= cursor_position && cursor_position < range.end);

                    if selection.cursor_shape == CursorShape::Block && !is_cursor_in_redacted_range
                    {
                        if let Some(row) = &cursor_row_layout.long_row {
                            CursorLayout::long_row_width(
                                &mut block_width,
                                snapshot
                                    .placeholder_display_snapshot
                                    .as_ref()
                                    .filter(|_| snapshot.is_empty())
                                    .unwrap_or(&snapshot.display_snapshot),
                                &row.row,
                                cursor_position,
                                cursor_row_layout.font_size,
                                &self.style.text,
                                window,
                                cx,
                            )?;
                            let origin = x + content_origin.x;
                            let left = window.pixel_snap(origin);
                            retained_block_text = RetainedCursorText::new(
                                cursor_row_layout.fragment_positions(),
                                cursor_row_layout.x_for_index(cursor_column),
                                origin,
                                left..left + window.pixel_snap(block_width),
                                CursorLayout::text_color(cx),
                                |font_id, font_size| {
                                    window.text_system().bounding_box(font_id, font_size)
                                },
                            );
                        } else if let Some(text) =
                            snapshot.grapheme_at(cursor_position).or_else(|| {
                                if snapshot.is_empty() {
                                    snapshot.placeholder_text().and_then(|s| {
                                        s.graphemes(true).next().map(|s| s.to_string().into())
                                    })
                                } else {
                                    None
                                }
                            })
                        {
                            block_text = Some(CursorLayout::shape_text(
                                &mut block_width,
                                text,
                                cursor_row_layout.font_id_for_index(cursor_column),
                                cursor_row_layout.font_size,
                                cursor_row_layout.text_metrics_for_index(cursor_column),
                                &self.style.text,
                                window,
                                cx,
                            ));
                        }
                    }

                    let y = ((cursor_position.row().as_f64() - scroll_position.y)
                        * ScrollPixelOffset::from(line_height))
                    .into();
                    if selection.is_newest {
                        editor.pixel_position_of_newest_cursor = Some(point(
                            text_hitbox.origin.x + x + block_width / 2.,
                            text_hitbox.origin.y + y + line_height / 2.,
                        ));

                        if autoscroll_containing_element {
                            let top = text_hitbox.origin.y
                                + ((cursor_position.row().as_f64() - scroll_position.y - 3.)
                                    .max(0.)
                                    * ScrollPixelOffset::from(line_height))
                                .into();
                            let left = text_hitbox.origin.x
                                + ((cursor_position.column() as ScrollOffset
                                    - scroll_position.x
                                    - 3.)
                                    .max(0.)
                                    * ScrollPixelOffset::from(em_width))
                                .into();

                            let bottom = text_hitbox.origin.y
                                + ((cursor_position.row().as_f64() - scroll_position.y + 4.)
                                    * ScrollPixelOffset::from(line_height))
                                .into();
                            let right = text_hitbox.origin.x
                                + ((cursor_position.column() as ScrollOffset - scroll_position.x
                                    + 4.)
                                    * ScrollPixelOffset::from(em_width))
                                .into();

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
                        retained_block_text,
                        cursor_name: None,
                        animated_corners: None,
                    };
                    let cursor_name = selection.user_name.clone().map(|name| CursorName {
                        string: name,
                        color: self.style.background,
                        is_top_row: cursor_position.row().0 == 0,
                    });
                    cursor.layout(content_origin, cursor_name, window, cx);
                    if selection.is_local
                        && cursor_shape_supports_cursor_animation(selection.cursor_shape)
                    {
                        if let Some((cursor_viewport, animation_now)) = animation_context {
                            handled_animation_cursors.insert(selection.id);
                            let target_bounds =
                                window.pixel_snap_bounds(cursor.bounds(content_origin));
                            cursor.animated_corners = editor.cursor_animations.update(
                                selection.id,
                                LogicalCursorPosition {
                                    row: cursor_position.row().0,
                                    column: cursor_position.column(),
                                },
                                target_bounds,
                                cursor_viewport,
                                animation_now,
                            );
                            request_animation_frame |= cursor.animated_corners.is_some();
                        }
                    } else if animation_enabled && selection.is_local {
                        editor.cursor_animations.remove(selection.id);
                    }
                    cursors.push(cursor);
                }
            }

            if animation_enabled {
                editor.cursor_animations.capture_newest_state();
                editor
                    .cursor_animations
                    .retain(|selection_id| handled_animation_cursors.contains(&selection_id));
                if request_animation_frame {
                    window.request_animation_frame();
                }
            }

            Ok::<_, PendingRowGeometry>(cursors)
        })?;

        if let Some(bounds) = autoscroll_bounds {
            window.request_autoscroll(bounds);
        }

        Ok(cursor_layouts)
    }

    fn layout_navigation_overlays(
        &self,
        snapshot: &EditorSnapshot,
        visible_display_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<NavigationOverlayPaintCommand> {
        let mut overlay_sets = self
            .editor
            .read(cx)
            .navigation_overlay_sets()
            .iter()
            .map(|(key, overlays)| (*key, overlays.clone()))
            .collect::<Vec<_>>();
        if overlay_sets.is_empty() {
            return Vec::new();
        }
        overlay_sets.sort_by_key(|(key, _)| *key);

        let layout_context = NavigationOverlayLayoutContext {
            display_snapshot: &snapshot.display_snapshot,
            visible_display_row_range: &visible_display_row_range,
            line_layouts,
            text_align: self.style.text.text_align,
            content_width: text_hitbox.size.width,
            content_origin,
            scroll_position,
            scroll_pixel_position,
            line_height,
            editor_font: self.style.text.font(),
            editor_font_size: self.style.text.font_size.to_pixels(window.rem_size()),
        };
        let mut navigation_overlay_paint_commands = Vec::new();

        for (_, overlays) in overlay_sets {
            for overlay in overlays.as_ref() {
                Self::layout_navigation_label(
                    overlay,
                    &layout_context,
                    window,
                    cx,
                    &mut navigation_overlay_paint_commands,
                );
            }
        }

        navigation_overlay_paint_commands
    }

    fn layout_navigation_label(
        overlay: &crate::NavigationTargetOverlay,
        context: &NavigationOverlayLayoutContext<'_>,
        window: &mut Window,
        cx: &mut App,
        paint_commands: &mut Vec<NavigationOverlayPaintCommand>,
    ) {
        let label = &overlay.label;
        let label_display_point = overlay
            .target_range
            .start
            .to_display_point(context.display_snapshot);
        let label_row = label_display_point.row();
        if !context.visible_display_row_range.contains(&label_row) {
            return;
        }

        let row_index = label_row.minus(context.visible_display_row_range.start) as usize;
        let row_layout = &context.line_layouts[row_index];
        let label_column = label_display_point.column().min(row_layout.len as u32) as usize;
        let label_x = Pixels::from(
            row_layout.x_for_index(label_column)
                + row_layout.alignment_offset(context.text_align, context.content_width)
                - context.scroll_pixel_position.x,
        ) + label.x_offset;
        let label_y = ((label_row.as_f64() - context.scroll_position.y)
            * ScrollPixelOffset::from(context.line_height))
        .into();
        let label_text_size = (context.editor_font_size * label.scale_factor.max(0.0)).max(px(1.0));
        let origin = context.content_origin + point(label_x, label_y);

        let mut element = div()
            .block_mouse_except_scroll()
            .font(context.editor_font.clone())
            .text_size(label_text_size)
            .text_color(label.text_color)
            .line_height(context.line_height)
            .child(label.text.clone())
            .into_any_element();
        element.prepaint_as_root(origin, AvailableSpace::min_size(), window, cx);

        paint_commands.push(NavigationOverlayPaintCommand::Label(
            NavigationLabelLayout { element, origin },
        ));
    }

    fn layout_scrollbars(
        &self,
        snapshot: &EditorSnapshot,
        scrollbar_layout_information: &ScrollbarLayoutInformation,
        content_offset: gpui::Point<Pixels>,
        scroll_position: gpui::Point<ScrollOffset>,
        horizontal_scroll_pending: bool,
        non_visible_cursors: bool,
        right_margin: Pixels,
        editor_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<EditorScrollbars> {
        let show_scrollbars = self.editor.read(cx).show_scrollbars;
        if (!show_scrollbars.horizontal && !show_scrollbars.vertical)
            || self.style.scrollbar_width.is_zero()
        {
            return None;
        }

        // If a drag took place after we started dragging the scrollbar,
        // cancel the scrollbar drag.
        if cx.has_active_drag() {
            self.editor.update(cx, |editor, cx| {
                editor.scroll_manager.reset_scrollbar_state(cx)
            });
        }

        let editor_settings = EditorSettings::get_global(cx);
        let scrollbar_settings = editor_settings.scrollbar;
        let show_scrollbars = match scrollbar_settings.show {
            ShowScrollbar::Auto => {
                let editor = self.editor.read(cx);
                let is_singleton = editor.buffer_kind(cx) == ItemBufferKind::Singleton;
                let supports_git_diff_markers =
                    is_singleton || editor.allow_git_diff_scrollbar_markers;
                // Git
                (supports_git_diff_markers && scrollbar_settings.git_diff && snapshot.buffer_snapshot().has_diff_hunks())
                ||
                // Buffer Search Results
                (is_singleton && scrollbar_settings.search_results && editor.has_background_highlights(HighlightKey::BufferSearchHighlights))
                ||
                // Selected Text Occurrences
                (is_singleton && scrollbar_settings.selected_text && editor.has_background_highlights(HighlightKey::SelectedTextHighlight))
                ||
                // Selected Symbol Occurrences
                (is_singleton && scrollbar_settings.selected_symbol && (editor.has_background_highlights(HighlightKey::DocumentHighlightRead) || editor.has_background_highlights(HighlightKey::DocumentHighlightWrite)))
                ||
                // Diagnostics
                (is_singleton && scrollbar_settings.diagnostics != ScrollbarDiagnostics::None && snapshot.buffer_snapshot().has_diagnostics())
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

        // The horizontal scrollbar is usually slightly offset to align nicely with
        // indent guides. However, this offset is not needed if indent guides are
        // disabled for the current editor.
        let content_offset = self
            .editor
            .read(cx)
            .show_indent_guides
            .is_none_or(|should_show| should_show)
            .then_some(content_offset)
            .unwrap_or_default();

        Some(EditorScrollbars::from_scrollbar_axes(
            ScrollbarAxes {
                horizontal: !horizontal_scroll_pending
                    && scrollbar_settings.axes.horizontal
                    && self.editor.read(cx).show_scrollbars.horizontal,
                vertical: scrollbar_settings.axes.vertical
                    && self.editor.read(cx).show_scrollbars.vertical,
            },
            scrollbar_layout_information,
            content_offset,
            scroll_position,
            self.style.scrollbar_width,
            right_margin,
            editor_width,
            show_scrollbars,
            self.editor.read(cx).scroll_manager.active_scrollbar_state(),
            window,
        ))
    }

    fn layout_minimap(
        &self,
        snapshot: &EditorSnapshot,
        minimap_width: Pixels,
        scroll_position: gpui::Point<f64>,
        scrollbar_layout_information: &ScrollbarLayoutInformation,
        scrollbar_layout: Option<&EditorScrollbars>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<MinimapLayout> {
        let minimap_editor = self.editor.read(cx).minimap().cloned()?;

        let minimap_settings = EditorSettings::get_global(cx).minimap;

        if minimap_settings.on_active_editor() {
            let active_editor = self.editor.read(cx).workspace().and_then(|ws| {
                ws.read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .and_then(|i| i.act_as::<Editor>(cx))
            });
            if active_editor.is_some_and(|e| e != self.editor) {
                return None;
            }
        }

        if !snapshot.mode.is_full()
            || minimap_width.is_zero()
            || matches!(
                minimap_settings.show,
                ShowMinimap::Auto if scrollbar_layout.is_none_or(|layout| !layout.visible)
            )
        {
            return None;
        }

        const MINIMAP_AXIS: ScrollbarAxis = ScrollbarAxis::Vertical;

        let ScrollbarLayoutInformation {
            editor_bounds,
            scroll_range,
            glyph_grid_cell,
        } = scrollbar_layout_information;

        let line_height = glyph_grid_cell.height;
        let scroll_position = scroll_position.along(MINIMAP_AXIS);

        let top_right_anchor = scrollbar_layout
            .and_then(|layout| layout.vertical.as_ref())
            .map(|vertical_scrollbar| vertical_scrollbar.hitbox.origin)
            .unwrap_or_else(|| editor_bounds.top_right());

        let thumb_state = self
            .editor
            .read_with(cx, |editor, _| editor.scroll_manager.minimap_thumb_state());

        let show_thumb = match minimap_settings.thumb {
            MinimapThumb::Always => true,
            MinimapThumb::Hover => thumb_state.is_some(),
        };

        let minimap_bounds = Bounds::from_anchor_and_size(
            gpui::Anchor::TopRight,
            top_right_anchor,
            size(minimap_width, editor_bounds.size.height),
        );
        let minimap_line_height = self.get_minimap_line_height(
            minimap_editor
                .read(cx)
                .text_style_refinement
                .as_ref()
                .and_then(|refinement| refinement.font_size)
                .unwrap_or(MINIMAP_FONT_SIZE),
            window,
            cx,
        );
        let minimap_height = minimap_bounds.size.height;

        let visible_editor_lines = (editor_bounds.size.height / line_height) as f64;
        let total_editor_lines = (scroll_range.height / line_height) as f64;
        let minimap_lines = (minimap_height / minimap_line_height) as f64;

        let minimap_scroll_top = MinimapLayout::calculate_minimap_top_offset(
            total_editor_lines,
            visible_editor_lines,
            minimap_lines,
            scroll_position,
        );

        let layout = ScrollbarLayout::for_minimap(
            window.insert_hitbox(minimap_bounds, HitboxBehavior::Normal),
            visible_editor_lines,
            total_editor_lines,
            minimap_line_height,
            scroll_position,
            minimap_scroll_top,
            show_thumb,
        )
        .with_thumb_state(thumb_state);

        let thumb_pixels_per_editor_line = MinimapLayout::thumb_pixels_per_editor_line(
            &layout,
            total_editor_lines,
            minimap_line_height,
        );

        minimap_editor.update(cx, |editor, cx| {
            editor.set_scroll_position(point(0., minimap_scroll_top), window, cx)
        });

        // Required for the drop shadow to be visible
        const PADDING_OFFSET: Pixels = px(4.);

        let mut minimap = div()
            .size_full()
            .shadow_xs()
            .px(PADDING_OFFSET)
            .child(minimap_editor)
            .into_any_element();

        let extended_bounds = minimap_bounds.extend(Edges {
            right: PADDING_OFFSET,
            left: PADDING_OFFSET,
            ..Default::default()
        });
        minimap.layout_as_root(extended_bounds.size.into(), window, cx);
        window.with_absolute_element_offset(extended_bounds.origin, |window| {
            minimap.prepaint(window, cx)
        });

        Some(MinimapLayout {
            minimap,
            thumb_layout: layout,
            thumb_border_style: minimap_settings.thumb_border,
            minimap_line_height,
            minimap_scroll_top,
            max_scroll_top: total_editor_lines,
            thumb_pixels_per_editor_line,
        })
    }

    fn get_minimap_line_height(
        &self,
        font_size: AbsoluteLength,
        window: &mut Window,
        cx: &mut App,
    ) -> Pixels {
        let rem_size = self.rem_size(cx).unwrap_or(window.rem_size());
        let mut text_style = self.style.text.clone();
        text_style.font_size = font_size;
        text_style.line_height_in_pixels(rem_size)
    }

    fn get_minimap_width(
        &self,
        minimap_settings: &Minimap,
        scrollbars_shown: bool,
        text_width: Pixels,
        em_width: Pixels,
        font_size: Pixels,
        rem_size: Pixels,
        cx: &App,
    ) -> Option<Pixels> {
        if minimap_settings.show == ShowMinimap::Auto && !scrollbars_shown {
            return None;
        }

        let minimap_font_size = self.editor.read_with(cx, |editor, cx| {
            editor.minimap().map(|minimap_editor| {
                minimap_editor
                    .read(cx)
                    .text_style_refinement
                    .as_ref()
                    .and_then(|refinement| refinement.font_size)
                    .unwrap_or(MINIMAP_FONT_SIZE)
            })
        })?;

        let minimap_em_width = em_width * (minimap_font_size.to_pixels(rem_size) / font_size);

        let minimap_width = (text_width * MinimapLayout::MINIMAP_WIDTH_PCT)
            .min(minimap_em_width * minimap_settings.max_width_columns.get() as f32);

        (minimap_width >= minimap_em_width * MinimapLayout::MINIMAP_MIN_WIDTH_COLUMNS)
            .then_some(minimap_width)
    }

    fn prepaint_crease_toggles(
        &self,
        crease_toggles: &mut [Option<AnyElement>],
        line_height: Pixels,
        gutter_dimensions: &GutterDimensions,
        gutter_settings: crate::editor_settings::Gutter,
        scroll_position: gpui::Point<ScrollOffset>,
        start_row: DisplayRow,
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

                let display_row = DisplayRow(start_row.0 + ix as u32);
                let position = point(
                    gutter_dimensions.width - gutter_dimensions.right_padding,
                    line_height * (display_row.as_f64() - scroll_position.y) as f32,
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
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        scroll_position: gpui::Point<ScrollOffset>,
        start_row: DisplayRow,
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
                let padding = if line.width == 0. {
                    Pixels::ZERO
                } else {
                    4. * em_width
                };
                let position = point(
                    Pixels::from(line.width - scroll_pixel_position.x) + padding,
                    line_height
                        * (DisplayRow(start_row.0 + ix as u32).as_f64() - scroll_position.y) as f32,
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
        scroll_position: gpui::Point<ScrollOffset>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<(DisplayDiffHunk, Option<Hitbox>)> {
        let folded_buffers = self.editor.read(cx).folded_buffers(cx);
        let mut display_hunks = snapshot
            .display_diff_hunks_for_rows(display_rows, folded_buffers)
            .map(|hunk| (hunk, None))
            .collect::<Vec<_>>();
        let git_gutter_setting = ProjectSettings::get_global(cx).git.git_gutter;
        if let GitGutterSetting::TrackedFiles = git_gutter_setting {
            for (hunk, hitbox) in &mut display_hunks {
                if matches!(hunk, DisplayDiffHunk::Unfolded { .. }) {
                    let hunk_bounds = Self::diff_hunk_bounds(
                        scroll_position,
                        line_height,
                        gutter_hitbox.bounds,
                        hunk,
                        snapshot,
                        cx,
                    );
                    *hitbox = Some(window.insert_hitbox(hunk_bounds, HitboxBehavior::BlockMouse));
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
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        edit_prediction_popover_origin: Option<gpui::Point<Pixels>>,
        start_row: DisplayRow,
        end_row: DisplayRow,
        line_height: Pixels,
        em_width: Pixels,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> HashMap<DisplayRow, AnyElement> {
        let max_severity = match self
            .editor
            .read(cx)
            .inline_diagnostics_enabled()
            .then(|| {
                ProjectSettings::get_global(cx)
                    .diagnostics
                    .inline
                    .max_severity
                    .unwrap_or_else(|| self.editor.read(cx).diagnostics_max_severity)
                    .into_lsp()
            })
            .flatten()
        {
            Some(max_severity) => max_severity,
            None => return HashMap::default(),
        };

        let active_diagnostics_group = self.editor.read(cx).active_diagnostic_group_id();

        let diagnostics_by_rows = self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            editor
                .inline_diagnostics
                .per_row(
                    &snapshot,
                    start_row..end_row,
                    max_severity,
                    // Active diagnostics are all shown in the editor already, no need to display them inline
                    active_diagnostics_group,
                )
                .into_iter()
                .filter(|(row, _)| !row_block_types.contains_key(row))
                .map(|(row, diagnostic)| (row, diagnostic.clone()))
                .collect::<Vec<_>>()
        });

        if diagnostics_by_rows.is_empty() {
            return HashMap::default();
        }

        let severity_to_color = |sev: &lsp::DiagnosticSeverity| match sev {
            &lsp::DiagnosticSeverity::ERROR => Color::Error,
            &lsp::DiagnosticSeverity::WARNING => Color::Warning,
            &lsp::DiagnosticSeverity::INFORMATION => Color::Info,
            &lsp::DiagnosticSeverity::HINT => Color::Hint,
            _ => Color::Error,
        };

        let padding = ProjectSettings::get_global(cx).diagnostics.inline.padding as f32 * em_width;
        let min_x = column_pixels(
            &self.style,
            ProjectSettings::get_global(cx)
                .diagnostics
                .inline
                .min_column as usize,
            window,
        );

        let mut elements = HashMap::default();
        for (row, diagnostic_to_render) in diagnostics_by_rows {
            let pos_y = content_origin.y + line_height * (row.0 as f64 - scroll_position.y) as f32;

            let window_ix = row.0.saturating_sub(start_row.0) as usize;
            let pos_x = {
                let crease_trailer_layout = &crease_trailers[window_ix];
                let line_layout = &line_layouts[window_ix];

                let line_end = if let Some(crease_trailer) = crease_trailer_layout {
                    crease_trailer.bounds.right()
                } else {
                    Pixels::from(
                        ScrollPixelOffset::from(content_origin.x) + line_layout.width
                            - scroll_pixel_position.x,
                    )
                };

                let padded_line = line_end + padding;
                let min_start = Pixels::from(
                    ScrollPixelOffset::from(content_origin.x + min_x) - scroll_pixel_position.x,
                );

                cmp::max(padded_line, min_start)
            };

            let behind_edit_prediction_popover = edit_prediction_popover_origin
                .as_ref()
                .is_some_and(|edit_prediction_popover_origin| {
                    (pos_y..pos_y + line_height).contains(&edit_prediction_popover_origin.y)
                });
            let opacity = if behind_edit_prediction_popover {
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
                .font(style.text.font())
                .child(diagnostic_to_render.message.clone())
                .into_any();

            element.prepaint_as_root(point(pos_x, pos_y), AvailableSpace::min_size(), window, cx);

            elements.insert(row, element);
        }

        elements
    }

    fn layout_inline_code_actions(
        &self,
        display_point: DisplayPoint,
        content_origin: gpui::Point<Pixels>,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_height: Pixels,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        // Don't show code actions in split diff view
        if self.split_side.is_some() {
            return None;
        }

        if !snapshot
            .show_code_actions
            .unwrap_or(EditorSettings::get_global(cx).inline_code_actions)
        {
            return None;
        }

        let icon_size = ui::IconSize::XSmall;
        let mut button = self.editor.update(cx, |editor, cx| {
            if !editor.has_available_code_actions_for_selection() {
                return None;
            }
            let active = editor
                .context_menu
                .borrow()
                .as_ref()
                .and_then(|menu| {
                    if let crate::CodeContextMenu::CodeActions(CodeActionsMenu {
                        deployed_from,
                        ..
                    }) = menu
                    {
                        deployed_from.as_ref()
                    } else {
                        None
                    }
                })
                .is_some_and(|source| matches!(source, CodeActionSource::Indicator(..)));
            Some(editor.render_inline_code_actions(icon_size, display_point.row(), active, cx))
        })?;

        let buffer_point = display_point.to_point(&snapshot.display_snapshot);
        let new_display_row = snapshot.display_row_for_inline_code_action(buffer_point)?;

        let start_y = content_origin.y
            + (((new_display_row.as_f64() - scroll_position.y) as f32) * line_height)
            + (line_height / 2.0)
            - (icon_size.square(window, cx) / 2.);
        let start_x = (ScrollPixelOffset::from(content_origin.x) - scroll_pixel_position.x
            + ScrollPixelOffset::from(window.rem_size() * 0.1))
        .into();

        let absolute_offset = gpui::point(start_x, start_y);
        button.layout_as_root(gpui::AvailableSpace::min_size(), window, cx);
        button.prepaint_as_root(
            absolute_offset,
            gpui::AvailableSpace::min_size(),
            window,
            cx,
        );
        Some(button)
    }

    fn layout_inline_blame(
        &self,
        display_row: DisplayRow,
        row_info: &RowInfo,
        line_layout: &LineWithInvisibles,
        crease_trailer: Option<&CreaseTrailerLayout>,
        em_width: Pixels,
        content_origin: gpui::Point<Pixels>,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InlineBlameLayout> {
        if !self
            .editor
            .update(cx, |editor, cx| editor.render_git_blame_inline(window, cx))
        {
            return None;
        }

        let editor = self.editor.read(cx);
        let blame = editor.blame.clone()?;
        let padding = {
            const INLINE_ACCEPT_SUGGESTION_EM_WIDTHS: f32 = 14.;

            let mut padding = ProjectSettings::get_global(cx).git.inline_blame.padding as f32;

            if let Some(edit_prediction) = editor.active_edit_prediction.as_ref()
                && let EditPrediction::Edit {
                    display_mode: EditDisplayMode::TabAccept,
                    ..
                } = &edit_prediction.completion
            {
                padding += INLINE_ACCEPT_SUGGESTION_EM_WIDTHS
            }

            padding * em_width
        };

        let (buffer_id, entry) = blame
            .update(cx, |blame, cx| {
                blame.blame_for_rows(&[*row_info], cx).next()
            })
            .flatten()?;

        let mut element = render_inline_blame_entry(entry.clone(), &self.style, cx)?;

        let start_y =
            content_origin.y + line_height * ((display_row.as_f64() - scroll_position.y) as f32);

        let start_x = {
            let line_end = if let Some(crease_trailer) = crease_trailer {
                crease_trailer.bounds.right()
            } else {
                Pixels::from(
                    ScrollPixelOffset::from(content_origin.x) + line_layout.width
                        - scroll_pixel_position.x,
                )
            };

            let padded_line_end = line_end + padding;

            let min_column_in_pixels = column_pixels(
                &self.style,
                ProjectSettings::get_global(cx).git.inline_blame.min_column as usize,
                window,
            );
            let min_start = Pixels::from(
                ScrollPixelOffset::from(content_origin.x + min_column_in_pixels)
                    - scroll_pixel_position.x,
            );

            cmp::max(padded_line_end, min_start)
        };

        let absolute_offset = point(start_x, start_y);
        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
        let bounds = Bounds::new(absolute_offset, size);

        element.prepaint_as_root(absolute_offset, AvailableSpace::min_size(), window, cx);

        Some(InlineBlameLayout {
            element,
            bounds,
            buffer_id,
            entry,
        })
    }

    fn layout_blame_popover(
        &self,
        editor_snapshot: &EditorSnapshot,
        text_hitbox: &Hitbox,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !self.editor.read(cx).inline_blame_popover.is_some() {
            return;
        }

        let Some(blame) = self.editor.read(cx).blame.clone() else {
            return;
        };
        let cursor_point = self
            .editor
            .read(cx)
            .selections
            .newest::<language::Point>(&editor_snapshot.display_snapshot)
            .head();

        let Some((buffer, buffer_point)) = editor_snapshot
            .buffer_snapshot()
            .point_to_buffer_point(cursor_point)
        else {
            return;
        };

        let row_info = RowInfo {
            buffer_id: Some(buffer.remote_id()),
            buffer_row: Some(buffer_point.row),
            ..Default::default()
        };

        let Some((buffer_id, blame_entry)) = blame
            .update(cx, |blame, cx| blame.blame_for_rows(&[row_info], cx).next())
            .flatten()
        else {
            return;
        };

        let Some((popover_state, target_point)) = self.editor.read_with(cx, |editor, _| {
            editor
                .inline_blame_popover
                .as_ref()
                .map(|state| (state.popover_state.clone(), state.position))
        }) else {
            return;
        };

        let workspace = self
            .editor
            .read_with(cx, |editor, _| editor.workspace().map(|w| w.downgrade()));

        let maybe_element = workspace.and_then(|workspace| {
            render_blame_entry_popover(
                blame_entry,
                popover_state.scroll_handle,
                popover_state.commit_message,
                popover_state.markdown,
                workspace,
                &blame,
                buffer_id,
                window,
                cx,
            )
        });

        if let Some(mut element) = maybe_element {
            let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
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

            let origin = point(popover_origin.x + horizontal_offset, popover_origin.y);
            let popover_bounds = Bounds::new(origin, size);

            self.editor.update(cx, |editor, _| {
                if let Some(state) = &mut editor.inline_blame_popover {
                    state.popover_bounds = Some(popover_bounds);
                }
            });

            window.defer_draw(element, origin, 2, None);
        }
    }

    fn layout_blame_entries(
        &self,
        buffer_rows: &[RowInfo],
        em_width: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        start_row: DisplayRow,
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
        let start_x = em_width;

        let mut last_used_color: Option<(Hsla, Oid)> = None;
        let blame_renderer = cx.global::<GlobalBlameRenderer>().0.clone();

        let shaped_lines = blamed_rows
            .into_iter()
            .enumerate()
            .flat_map(|(ix, blame_entry)| {
                let (buffer_id, blame_entry) = blame_entry?;
                let mut element = render_blame_entry(
                    ix,
                    &blame,
                    blame_entry,
                    &self.style,
                    &mut last_used_color,
                    self.editor.clone(),
                    workspace.clone(),
                    buffer_id,
                    &*blame_renderer,
                    window,
                    cx,
                )?;

                let start_y = line_height
                    * (DisplayRow(start_row.0 + ix as u32).as_f64() - scroll_position.y) as f32;
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
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
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
                        column_pixels(&self.style, indent_guide.tab_size as usize, window);
                    let total_width = single_indent_width * indent_guide.depth as f32;
                    let start_x = Pixels::from(
                        ScrollOffset::from(content_origin.x + total_width)
                            - scroll_pixel_position.x,
                    );
                    if start_x >= text_origin.x {
                        let (offset_y, length, display_row_range) =
                            Self::calculate_indent_guide_bounds(
                                indent_guide.start_row..indent_guide.end_row,
                                line_height,
                                snapshot,
                            );

                        let start_y = Pixels::from(
                            ScrollOffset::from(content_origin.y) + offset_y
                                - scroll_pixel_position.y,
                        );

                        Some(IndentGuideLayout {
                            origin: point(start_x, start_y),
                            length,
                            single_indent_width,
                            display_row_range,
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

    fn depth_zero_indent_guide_padding_for_row(
        indent_guides: &[IndentGuideLayout],
        row: DisplayRow,
    ) -> Pixels {
        indent_guides
            .iter()
            .find(|guide| guide.depth == 0 && guide.display_row_range.contains(&row))
            .and_then(|guide| {
                guide
                    .settings
                    .visible_line_width(guide.active)
                    .map(|width| px(width as f32 * 2.0))
            })
            .unwrap_or(px(0.0))
    }

    fn layout_wrap_guides(
        &self,
        scroll_left: ScrollPixelOffset,
        content_origin: gpui::Point<Pixels>,
        scrollbar_layout: Option<&EditorScrollbars>,
        vertical_scrollbar_width: Pixels,
        hitbox: &Hitbox,
        window: &Window,
        cx: &App,
    ) -> SmallVec<[(Pixels, bool); 2]> {
        let content_origin = content_origin.x;
        let vertical_scrollbar_width = scrollbar_layout
            .and_then(|layout| layout.visible.then_some(vertical_scrollbar_width))
            .unwrap_or_default();

        self.editor
            .read(cx)
            .wrap_guides(cx)
            .into_iter()
            .flat_map(|(guide, active)| {
                let wrap_position = column_pixels(&self.style, guide, window);
                let wrap_guide_x =
                    content_origin + Pixels::from(f64::from(wrap_position) - scroll_left);
                let display_wrap_guide = wrap_guide_x >= content_origin
                    && wrap_guide_x <= hitbox.bounds.right() - vertical_scrollbar_width;

                display_wrap_guide.then_some((wrap_guide_x, active))
            })
            .collect()
    }

    fn calculate_indent_guide_bounds(
        row_range: Range<MultiBufferRow>,
        line_height: Pixels,
        snapshot: &DisplaySnapshot,
    ) -> (f64, gpui::Pixels, Range<DisplayRow>) {
        let start_point = Point::new(row_range.start.0, 0);
        let end_point = Point::new(row_range.end.0, 0);

        let mut row_range = start_point.to_display_point(snapshot).row()
            ..end_point.to_display_point(snapshot).row();

        let mut prev_line = start_point;
        prev_line.row = prev_line.row.saturating_sub(1);
        let prev_line = prev_line.to_display_point(snapshot).row();

        let mut cons_line = end_point;
        cons_line.row += 1;
        let cons_line = cons_line.to_display_point(snapshot).row();

        let mut offset_y = row_range.start.as_f64() * f64::from(line_height);
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
            if matches!(
                block,
                Block::ExcerptBoundary { .. } | Block::BufferHeader { .. }
            ) {
                found_excerpt_header = true;
                break;
            }
            block_offset += block.height();
            block_height += block.height();
        }
        if !found_excerpt_header {
            offset_y -= block_offset as f64 * f64::from(line_height);
            length += block_height as f32 * line_height;
            row_range = DisplayRow(row_range.start.0.saturating_sub(block_offset))..row_range.end;
        }

        // If there is a block (e.g. diagnostic) at the end of an multibuffer excerpt,
        // we want to ensure that the indent guide stops before the excerpt header.
        let mut block_height = 0;
        let mut found_excerpt_header = false;
        for (_, block) in snapshot.blocks_in_range(row_range.end..cons_line) {
            if matches!(
                block,
                Block::ExcerptBoundary { .. } | Block::BufferHeader { .. }
            ) {
                found_excerpt_header = true;
            }
            block_height += block.height();
        }
        if found_excerpt_header {
            length -= block_height as f32 * line_height;
        } else {
            row_range = row_range.start..cons_line;
        }

        (offset_y, length, row_range)
    }

    fn layout_bookmarks(
        &self,
        gutter: &Gutter<'_>,
        bookmarks: &HashSet<DisplayRow>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        if self.split_side == Some(SplitSide::Left) {
            return Vec::new();
        }

        self.editor.update(cx, |editor, cx| {
            bookmarks
                .iter()
                .filter_map(|row| {
                    gutter.layout_item_skipping_folds(
                        *row,
                        |cx, _| editor.render_bookmark(*row, cx).into_any_element(),
                        window,
                        cx,
                    )
                })
                .collect_vec()
        })
    }

    fn layout_gutter_hover_button(
        &self,
        gutter: &Gutter,
        position: Anchor,
        row: DisplayRow,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        if self.split_side == Some(SplitSide::Left) {
            return None;
        }

        self.editor.update(cx, |editor, cx| {
            gutter.layout_item_skipping_folds(
                row,
                |cx, window| {
                    editor
                        .render_gutter_hover_button(position, row, window, cx)
                        .into_any_element()
                },
                window,
                cx,
            )
        })
    }

    fn layout_breakpoints(
        &self,
        gutter: &Gutter,
        breakpoints: &HashMap<DisplayRow, (Anchor, Breakpoint, Option<BreakpointSessionState>)>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        if self.split_side == Some(SplitSide::Left) {
            return Vec::new();
        }

        self.editor.update(cx, |editor, cx| {
            breakpoints
                .iter()
                .filter_map(|(row, (text_anchor, bp, state))| {
                    gutter.layout_item_skipping_folds(
                        *row,
                        |cx, _| {
                            editor
                                .render_breakpoint(*text_anchor, *row, &bp, *state, cx)
                                .into_any_element()
                        },
                        window,
                        cx,
                    )
                })
                .collect_vec()
        })
    }

    fn should_render_diff_review_button(
        &self,
        range: Range<DisplayRow>,
        row_infos: &[RowInfo],
        snapshot: &EditorSnapshot,
        cx: &App,
    ) -> Option<(DisplayRow, Option<u32>)> {
        if !cx.has_flag::<DiffReviewFeatureFlag>() {
            return None;
        }

        let show_diff_review_button = self.editor.read(cx).show_diff_review_button();
        if !show_diff_review_button {
            return None;
        }

        let indicator = self.editor.read(cx).gutter_diff_review_indicator.0?;
        if !indicator.is_active {
            return None;
        }

        let display_row = indicator
            .start
            .to_display_point(&snapshot.display_snapshot)
            .row();
        let row_index = (display_row.0.saturating_sub(range.start.0)) as usize;

        let row_info = row_infos.get(row_index);
        if row_info.is_some_and(|row_info| row_info.expand_info.is_some()) {
            return None;
        }

        let buffer_id = row_info.and_then(|info| info.buffer_id)?;

        let editor = self.editor.read(cx);
        if editor.is_buffer_folded(buffer_id, cx) {
            return None;
        }

        let buffer_row = row_info.and_then(|info| info.buffer_row);
        Some((display_row, buffer_row))
    }

    fn layout_run_indicators(
        &self,
        gutter: &Gutter,
        run_indicators: &HashSet<DisplayRow>,
        breakpoints: &HashMap<DisplayRow, (Anchor, Breakpoint, Option<BreakpointSessionState>)>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<AnyElement> {
        if self.split_side == Some(SplitSide::Left) {
            return Vec::new();
        }

        self.editor.update(cx, |editor, cx| {
            let active_task_indicator_row =
                // TODO: add edit button on the right side of each row in the context menu
                if let Some(crate::CodeContextMenu::CodeActions(CodeActionsMenu {
                    deployed_from,
                    actions,
                    ..
                })) = editor.context_menu.borrow().as_ref()
                {
                    actions
                        .tasks()
                        .map(|tasks| tasks.position.to_display_point(gutter.snapshot).row())
                        .or_else(|| match deployed_from {
                            Some(CodeActionSource::Indicator(row)) => Some(*row),
                            _ => None,
                        })
                } else {
                    None
                };

            run_indicators
                .iter()
                .filter_map(|display_row| {
                    let task_status = gutter
                        .row_infos
                        .get((display_row.0.saturating_sub(gutter.range.start.0)) as usize)
                        .and_then(|row_info| Some((row_info.buffer_id?, row_info.buffer_row?)))
                        .and_then(|(buffer_id, buffer_row)| {
                            editor.runnable_task_status(buffer_id, buffer_row)
                        });

                    gutter.layout_item(
                        *display_row,
                        |cx, _| {
                            editor
                                .render_run_indicator(
                                    &self.style,
                                    Some(*display_row) == active_task_indicator_row,
                                    breakpoints.get(&display_row).map(|(anchor, _, _)| *anchor),
                                    task_status,
                                    *display_row,
                                    cx,
                                )
                                .into_any_element()
                        },
                        window,
                        cx,
                    )
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
        scroll_position: gpui::Point<ScrollOffset>,
        start_row: DisplayRow,
        buffer_rows: &[RowInfo],
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Option<(AnyElement, gpui::Point<Pixels>)>> {
        if self.editor.read(cx).disable_expand_excerpt_buttons {
            return vec![];
        }

        let editor_font_size = self.style.text.font_size.to_pixels(window.rem_size()) * 1.2;

        let max_line_number_length = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
            .widest_line_number()
            .ilog10()
            + 1;

        let git_gutter_width = Self::gutter_strip_width(line_height, cx)
            + gutter_dimensions
                .git_blame_entries_width
                .unwrap_or_default();
        let available_width = gutter_dimensions.left_padding - git_gutter_width;

        buffer_rows
            .iter()
            .enumerate()
            .map(|(ix, row_info)| {
                let ExpandInfo {
                    direction,
                    start_anchor,
                } = row_info.expand_info?;

                let icon_name = match direction {
                    ExpandExcerptDirection::Up => IconName::ExpandUp,
                    ExpandExcerptDirection::Down => IconName::ExpandDown,
                    ExpandExcerptDirection::UpAndDown => IconName::ExpandVertical,
                };

                let editor = self.editor.clone();
                let is_wide = max_line_number_length
                    >= EditorSettings::get_global(cx).gutter.min_line_number_digits as u32
                    && row_info
                        .buffer_row
                        .is_some_and(|row| (row + 1).ilog10() + 1 == max_line_number_length)
                    || gutter_dimensions.right_padding == px(0.);

                let width = if is_wide {
                    available_width - px(5.)
                } else {
                    available_width + em_width - px(5.)
                };

                let toggle = IconButton::new(("expand", ix), icon_name)
                    .icon_color(Color::Custom(cx.theme().colors().editor_line_number))
                    .icon_size(IconSize::Custom(rems(editor_font_size / window.rem_size())))
                    .width(width)
                    .on_click(move |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.expand_excerpt(start_anchor, direction, window, cx);
                        });
                    })
                    .tooltip(Tooltip::for_action_title(
                        "Expand Excerpt",
                        &crate::actions::ExpandExcerpts::default(),
                    ))
                    .into_any_element();

                let position = point(
                    git_gutter_width + px(1.),
                    line_height
                        * (DisplayRow(start_row.0 + ix as u32).as_f64() - scroll_position.y) as f32
                        + px(1.),
                );
                let origin = gutter_hitbox.origin + position;

                Some((toggle, origin))
            })
            .collect()
    }

    fn layout_line_numbers(
        &self,
        gutter: &Gutter<'_>,
        active_rows: &BTreeMap<DisplayRow, LineHighlightSpec>,
        current_selection_head: Option<DisplayRow>,
        window: &mut Window,
        cx: &mut App,
    ) -> Arc<HashMap<MultiBufferRow, LineNumberLayout>> {
        let include_line_numbers = gutter
            .snapshot
            .show_line_numbers
            .unwrap_or_else(|| EditorSettings::get_global(cx).gutter.line_numbers);
        if !include_line_numbers {
            return Arc::default();
        }

        let relative = self.editor.read(cx).relative_line_numbers(cx);

        let relative_line_numbers_enabled = relative.enabled();
        let relative_rows = if relative_line_numbers_enabled
            && let Some(current_selection_head) = current_selection_head
        {
            gutter.snapshot.calculate_relative_line_numbers(
                &gutter.range,
                current_selection_head,
                relative.wrapped(),
            )
        } else {
            Default::default()
        };

        let mut line_number = String::new();
        let segments = gutter
            .row_infos
            .iter()
            .enumerate()
            .flat_map(|(ix, row_info)| {
                let display_row = DisplayRow(gutter.range.start.0 + ix as u32);
                line_number.clear();
                let non_relative_number = if relative.wrapped() {
                    row_info.buffer_row.or(row_info.wrapped_buffer_row)? + 1
                } else {
                    row_info.buffer_row? + 1
                };
                let relative_number = relative_rows.get(&display_row);
                if !(relative_line_numbers_enabled && relative_number.is_some())
                    && !gutter.snapshot.number_deleted_lines
                    && row_info
                        .diff_status
                        .is_some_and(|status| status.is_deleted())
                {
                    return None;
                }

                let number = relative_number.unwrap_or(&non_relative_number);
                write!(&mut line_number, "{number}").unwrap();

                let spec = active_rows.get(&display_row);
                let color = LineNumberStyle::new(
                    spec.is_some(),
                    spec.is_some_and(|spec| spec.breakpoint),
                    row_info.diff_status,
                )
                .color(cx.theme().colors());

                let shaped_line =
                    self.shape_line_number(SharedString::from(&line_number), color, window);
                let scroll_top =
                    gutter.scroll_position.y * ScrollPixelOffset::from(gutter.line_height);
                let line_origin = gutter.hitbox.origin
                    + point(
                        gutter.hitbox.size.width
                            - shaped_line.width
                            - gutter.dimensions.right_padding,
                        ix as f32 * gutter.line_height
                            - Pixels::from(
                                scroll_top % ScrollPixelOffset::from(gutter.line_height),
                            ),
                    );

                #[cfg(not(test))]
                let hitbox = Some(window.insert_hitbox(
                    Bounds::new(line_origin, size(shaped_line.width, gutter.line_height)),
                    HitboxBehavior::Normal,
                ));
                #[cfg(test)]
                let hitbox = {
                    let _ = line_origin;
                    None
                };

                let segment = LineNumberSegment {
                    shaped_line,
                    hitbox,
                };

                let buffer_row = DisplayPoint::new(display_row, 0)
                    .to_point(gutter.snapshot)
                    .row;
                let multi_buffer_row = MultiBufferRow(buffer_row);

                Some((multi_buffer_row, segment))
            });

        let mut line_numbers: HashMap<MultiBufferRow, LineNumberLayout> = HashMap::default();
        for (buffer_row, segment) in segments {
            line_numbers
                .entry(buffer_row)
                .or_insert_with(|| LineNumberLayout {
                    segments: Default::default(),
                })
                .segments
                .push(segment);
        }
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
            && snapshot.display_snapshot.companion_snapshot().is_none()
            && self.editor.read(cx).buffer_kind(cx) == ItemBufferKind::Singleton;
        if include_fold_statuses {
            row_infos
                .iter()
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

    fn bg_segments_per_row(
        rows: Range<DisplayRow>,
        selections: &[(PlayerColor, Vec<SelectionLayout>)],
        highlight_ranges: impl IntoIterator<Item = (Range<DisplayPoint>, Hsla)>,
        base_background: Hsla,
    ) -> Vec<Vec<(Range<DisplayPoint>, Hsla)>> {
        if rows.start >= rows.end {
            return Vec::new();
        }
        if !base_background.is_opaque() {
            // We don't actually know what color is behind this editor.
            return Vec::new();
        }
        let highlight_iter = highlight_ranges.into_iter();
        let selection_iter = selections.iter().flat_map(|(player_color, layouts)| {
            let color = player_color.selection;
            layouts.iter().filter_map(move |selection_layout| {
                if selection_layout.range.start != selection_layout.range.end {
                    Some((selection_layout.range.clone(), color))
                } else {
                    None
                }
            })
        });
        let mut per_row_map = vec![Vec::new(); rows.len()];
        for (range, color) in highlight_iter.chain(selection_iter) {
            let covered_rows = if range.end.column() == 0 {
                cmp::max(range.start.row(), rows.start)..cmp::min(range.end.row(), rows.end)
            } else {
                cmp::max(range.start.row(), rows.start)
                    ..cmp::min(range.end.row().next_row(), rows.end)
            };
            for row in covered_rows.iter_rows() {
                let seg_start = if row == range.start.row() {
                    range.start
                } else {
                    DisplayPoint::new(row, 0)
                };
                let seg_end = if row == range.end.row() && range.end.column() != 0 {
                    range.end
                } else {
                    DisplayPoint::new(row, u32::MAX)
                };
                let ix = row.minus(rows.start) as usize;
                debug_assert!(row >= rows.start && row < rows.end);
                debug_assert!(ix < per_row_map.len());
                per_row_map[ix].push((seg_start..seg_end, color));
            }
        }
        for row_segments in per_row_map.iter_mut() {
            if row_segments.is_empty() {
                continue;
            }
            let segments = mem::take(row_segments);
            let merged = Self::merge_overlapping_ranges(segments, base_background);
            *row_segments = merged;
        }
        per_row_map
    }

    /// Merge overlapping ranges by splitting at all range boundaries and blending colors where
    /// multiple ranges overlap. The result contains non-overlapping ranges ordered from left to right.
    ///
    /// Expects `start.row() == end.row()` for each range.
    fn merge_overlapping_ranges(
        ranges: Vec<(Range<DisplayPoint>, Hsla)>,
        base_background: Hsla,
    ) -> Vec<(Range<DisplayPoint>, Hsla)> {
        struct Boundary {
            pos: DisplayPoint,
            is_start: bool,
            index: usize,
            color: Hsla,
        }

        let mut boundaries: SmallVec<[Boundary; 16]> = SmallVec::with_capacity(ranges.len() * 2);
        for (index, (range, color)) in ranges.iter().enumerate() {
            debug_assert!(
                range.start.row() == range.end.row(),
                "expects single-row ranges"
            );
            if range.start < range.end {
                boundaries.push(Boundary {
                    pos: range.start,
                    is_start: true,
                    index,
                    color: *color,
                });
                boundaries.push(Boundary {
                    pos: range.end,
                    is_start: false,
                    index,
                    color: *color,
                });
            }
        }

        if boundaries.is_empty() {
            return Vec::new();
        }

        boundaries
            .sort_unstable_by(|a, b| a.pos.cmp(&b.pos).then_with(|| a.is_start.cmp(&b.is_start)));

        let mut processed_ranges: Vec<(Range<DisplayPoint>, Hsla)> = Vec::new();
        let mut active_ranges: SmallVec<[(usize, Hsla); 8]> = SmallVec::new();

        let mut i = 0;
        let mut start_pos = boundaries[0].pos;

        let boundaries_len = boundaries.len();
        while i < boundaries_len {
            let current_boundary_pos = boundaries[i].pos;
            if start_pos < current_boundary_pos {
                if !active_ranges.is_empty() {
                    let mut color = base_background;
                    for &(_, c) in &active_ranges {
                        color = Hsla::blend(color, c);
                    }
                    if let Some((last_range, last_color)) = processed_ranges.last_mut() {
                        if *last_color == color && last_range.end == start_pos {
                            last_range.end = current_boundary_pos;
                        } else {
                            processed_ranges.push((start_pos..current_boundary_pos, color));
                        }
                    } else {
                        processed_ranges.push((start_pos..current_boundary_pos, color));
                    }
                }
            }
            while i < boundaries_len && boundaries[i].pos == current_boundary_pos {
                let active_range = &boundaries[i];
                if active_range.is_start {
                    let idx = active_range.index;
                    let pos = active_ranges
                        .binary_search_by_key(&idx, |(i, _)| *i)
                        .unwrap_or_else(|p| p);
                    active_ranges.insert(pos, (idx, active_range.color));
                } else {
                    let idx = active_range.index;
                    if let Ok(pos) = active_ranges.binary_search_by_key(&idx, |(i, _)| *i) {
                        active_ranges.remove(pos);
                    }
                }
                i += 1;
            }
            start_pos = current_boundary_pos;
        }

        processed_ranges
    }

    fn layout_lines(
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        editor_width: Pixels,
        content_width: Pixels,
        content_origin_x: Pixels,
        paint_x: Range<ScaledPixels>,
        scroll_x: ScrollPixelOffset,
        long_rows: &[(DisplayRow, LongRow, Range<u32>)],
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        bg_segments_per_row: &[Vec<(Range<DisplayPoint>, Hsla)>],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Vec<LineWithInvisibles>, PendingRowGeometry> {
        if rows.start >= rows.end {
            return Ok(Vec::new());
        }

        // Show the placeholder when the editor is empty
        if snapshot.is_empty() {
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let placeholder_color = cx.theme().colors().text_placeholder;
            let placeholder_text = snapshot.placeholder_text();

            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or(Vec::new(), |text| text.split('\n').collect::<Vec<_>>());

            let placeholder_line_count = placeholder_lines.len();

            Ok(placeholder_lines
                .into_iter()
                .skip(rows.start.0 as usize)
                .chain(iter::repeat(""))
                .take(cmp::max(rows.len(), placeholder_line_count))
                .map(move |line| {
                    let run = TextRun {
                        len: line.len(),
                        font: style.text.font(),
                        color: placeholder_color,
                        ..Default::default()
                    };
                    let line = window.text_system().shape_line(
                        SharedString::new(line),
                        font_size,
                        &[run],
                        None,
                    );

                    LineWithInvisibles {
                        width: ScrollPixelOffset::from(line.width),
                        len: line.len,
                        fragments: smallvec![LineFragment::Text(line)],
                        invisibles: Vec::new(),
                        diagnostic_underline_severity_ranges: Vec::new(),
                        point_diagnostics: Vec::new(),
                        font_size,
                        long_row: None,
                    }
                })
                .collect())
        } else {
            let short_lines = |short_rows: Range<DisplayRow>,
                               window: &mut Window,
                               cx: &mut App|
             -> Result<Vec<LineWithInvisibles>, PendingRowGeometry> {
                let mut lines = Vec::with_capacity(short_rows.len());
                let mut start = short_rows.start;
                while start < short_rows.end {
                    let language_aware = language_aware_styling(snapshot, start);
                    let mut end = start.next_row();
                    while end < short_rows.end
                        && language_aware_styling(snapshot, end) == language_aware
                    {
                        end = end.next_row();
                    }
                    let offset = start.minus(rows.start) as usize;
                    lines.extend(LineWithInvisibles::from_chunks(
                        snapshot.highlighted_chunks(start..end, language_aware, style),
                        snapshot,
                        start,
                        style,
                        MAX_LINE_LEN,
                        end.minus(start) as usize,
                        &snapshot.mode,
                        editor_width,
                        move |index| is_row_soft_wrapped(index + offset),
                        bg_segments_per_row.get(offset..).unwrap_or_default(),
                        window,
                        cx,
                    )?);
                    start = end;
                }
                Ok(lines)
            };
            let mut lines = Vec::with_capacity(rows.len());
            let mut short_rows_start = rows.start;
            for (row, long_row, row_window) in long_rows {
                let row = *row;
                lines.extend(short_lines(short_rows_start..row, window, cx)?);
                short_rows_start = row.next_row();
                let bg_segments = bg_segments_per_row
                    .get(row.minus(rows.start) as usize)
                    .map_or(&[][..], Vec::as_slice);
                lines.push(if snapshot.mode.is_minimap() {
                    LineWithInvisibles::long_row_prefix(
                        row,
                        row_window.end,
                        snapshot,
                        style,
                        editor_width,
                        bg_segments,
                        window,
                        cx,
                    )
                } else {
                    LineWithInvisibles::long_row(
                        long_row.clone(),
                        row,
                        snapshot,
                        style,
                        row_window.clone(),
                        editor_width,
                        content_width,
                        content_origin_x,
                        paint_x.clone(),
                        scroll_x,
                        bg_segments,
                        window,
                        cx,
                    )
                }?);
            }
            lines.extend(short_lines(short_rows_start..rows.end, window, cx)?);
            Ok(lines)
        }
    }

    fn renderer_widths_key(&self, editor_width: Pixels, window: &Window) -> RendererWidthsKey {
        RendererWidthsKey {
            rem_size: window.rem_size(),
            font: window.text_style().font(),
            font_size: window.text_style().font_size.to_pixels(window.rem_size()),
            buffer_font: self.style.text.font(),
            editor_width,
            line_height: self.style.text.line_height_in_pixels(window.rem_size()),
        }
    }

    fn publish_renderer_widths<'a>(
        &self,
        lines: impl IntoIterator<Item = &'a LineWithInvisibles>,
        editor_width: Pixels,
        window: &Window,
        cx: &mut App,
    ) -> bool {
        let key = self.renderer_widths_key(editor_width, window);
        let widths = lines
            .into_iter()
            .flat_map(|line| &line.fragments)
            .filter_map(|fragment| match fragment {
                LineFragment::Element { id, size, .. } => Some((*id, size.width)),
                LineFragment::Text(_) => None,
            });
        self.editor.update(cx, |editor, cx| {
            editor.update_renderer_widths(key, widths, cx)
        })
    }

    fn measure_unmeasured_renderers(
        &self,
        snapshot: &mut EditorSnapshot,
        editor_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) {
        let key = self.renderer_widths_key(editor_width, window);
        let is_stale = self
            .editor
            .read(cx)
            .display_map
            .read(cx)
            .renderer_widths_key()
            != Some(&key);
        if is_stale
            && self.editor.update(cx, |editor, cx| {
                editor.update_renderer_widths(key.clone(), [], cx)
            })
        {
            *snapshot = self
                .editor
                .update(cx, |editor, cx| editor.snapshot(window, cx));
        }
        let scroll_top = snapshot.scroll_position().y.max(0.).floor() as u32;
        let visible_rows = self
            .editor
            .read(cx)
            .visible_line_count()
            .unwrap_or_default()
            .max(0.)
            .ceil() as u32;
        let priority_rows =
            DisplayRow(scroll_top)..DisplayRow(scroll_top.saturating_add(visible_rows));
        let display_map = self.editor.read(cx).display_map.clone();
        let font = self.style.text.font();
        let font_size = self.style.text.font_size.to_pixels(window.rem_size());
        let mut class_widths = HashMap::<RendererWidthClass, Pixels>::default();
        let mut widths = Vec::new();
        let mut budget = PrepaintWorkBudget::new(RENDERER_MEASURE_BUDGET_PER_PREPAINT);
        window.with_element_namespace("unmeasured_renderers", |window| {
            snapshot.for_each_unmeasured_renderer(priority_rows, |renderer, text| {
                if !budget.try_charge(RENDERER_VISIT_COST) {
                    return ControlFlow::Break(());
                }
                let constraint =
                    if renderer.constrain_width {
                        let ready = snapshot.prepared_renderer_constraint(renderer.id).filter(
                            |constraint| constraint.matches(&renderer, &text, &font, font_size),
                        );
                        if ready.is_some() {
                            ready
                        } else {
                            if !budget.try_charge(RENDERER_LAYOUT_COST) {
                                return ControlFlow::Break(());
                            }
                            display_map.update(cx, |map, cx| {
                                map.request_renderer_constraint(
                                    renderer.clone(),
                                    text.clone(),
                                    font.clone(),
                                    font_size,
                                    cx,
                                )
                            })
                        }
                    } else {
                        None
                    };
                let available_width = if renderer.constrain_width {
                    let Some(width) = constraint.and_then(|constraint| constraint.width()) else {
                        return ControlFlow::Continue(());
                    };
                    AvailableSpace::Definite(width)
                } else {
                    AvailableSpace::MinContent
                };
                let width_class = renderer.width_class.as_ref().filter(|class| match class {
                    RendererWidthClass::ColorSwatch => true,
                    RendererWidthClass::Placeholder { text, .. } => text.len() <= 32,
                });
                let class_width = width_class.and_then(|class| class_widths.get(class).copied());
                let width = match class_width {
                    Some(width) => width,
                    None => {
                        if !budget.try_charge(RENDERER_LAYOUT_COST) {
                            return ControlFlow::Break(());
                        }

                        let (_, size) = layout_chunk_renderer_with_width(
                            &renderer,
                            available_width,
                            &self.style.text,
                            editor_width,
                            window,
                            cx,
                        );
                        let width = size.width;
                        if let Some(class) = width_class {
                            class_widths.insert(class.clone(), width);
                        }
                        width
                    }
                };
                widths.push((renderer.id, width));
                ControlFlow::Continue(())
            });
        });
        if budget.exhausted {
            window.request_animation_frame();
        }
        if !widths.is_empty()
            && self.editor.update(cx, |editor, cx| {
                editor.update_renderer_widths(key, widths, cx)
            })
        {
            *snapshot = self
                .editor
                .update(cx, |editor, cx| editor.snapshot(window, cx));
        }
    }

    fn request_viewport_geometry(
        &self,
        snapshot: &EditorSnapshot,
        rows: Range<DisplayRow>,
        window: &Window,
        cx: &mut App,
    ) -> Vec<PendingRowGeometry> {
        let mut required = (rows.start.0..rows.end.0)
            .map(DisplayRow)
            .collect::<BTreeSet<_>>();
        if !snapshot.mode.is_minimap() {
            if self.editor.read(cx).buffer_kind(cx) == ItemBufferKind::Singleton
                && EditorSettings::get_global(cx).sticky_scroll.enabled
            {
                required.extend(
                    Self::sticky_headers(self.editor.read(cx), snapshot)
                        .into_iter()
                        .map(|header| header.sticky_row),
                );
            }
            for (_, block) in snapshot.blocks_in_range(rows) {
                if let Block::Custom(block) = block {
                    required.insert(
                        block
                            .start()
                            .to_point(snapshot.buffer_snapshot())
                            .to_display_point(snapshot)
                            .row(),
                    );
                }
            }
            if let Some(focused) = self.editor.read(cx).focused_block.as_ref()
                && focused
                    .focus_handle
                    .upgrade()
                    .is_some_and(|handle| handle.is_focused(window))
                && let Some(Block::Custom(block)) = snapshot.block_for_id(focused.id)
            {
                required.insert(
                    block
                        .start()
                        .to_point(snapshot.buffer_snapshot())
                        .to_display_point(snapshot)
                        .row(),
                );
            }
        }
        let longest_row = snapshot.longest_row();
        if snapshot.mode.is_minimap() || !snapshot.is_long_row(longest_row) {
            required.insert(longest_row);
        }
        let pending = required
            .into_iter()
            .flat_map(|row| {
                iter::once(&snapshot.display_snapshot)
                    .chain(
                        snapshot
                            .is_empty()
                            .then_some(snapshot.placeholder_display_snapshot.as_ref())
                            .flatten(),
                    )
                    .flat_map(move |snapshot| {
                        [
                            snapshot.long_row(row).err(),
                            snapshot.prepared_row_text(row).err(),
                        ]
                        .into_iter()
                        .flatten()
                    })
            })
            .collect::<Vec<_>>();
        self.submit_viewport_geometry(&pending, cx);
        pending
    }

    fn submit_viewport_geometry(&self, pending: &[PendingRowGeometry], cx: &mut App) {
        let mut requests = HashMap::default();
        for pending in pending {
            requests
                .entry(pending.display_map_id())
                .or_insert_with(Vec::new)
                .push(pending.clone());
        }
        let editor = self.editor.read(cx);
        let mut sources = vec![editor.display_map.clone()];
        sources.extend(editor.placeholder_display_map.clone());
        let owned = sources
            .iter()
            .map(|source| source.entity_id())
            .collect::<HashSet<_>>();
        let mut visited = HashSet::default();
        while let Some(source) = sources.pop() {
            if !visited.insert(source.entity_id()) {
                continue;
            }
            if let Some((companion, _)) = &source.read(cx).companion
                && let Some(companion) = companion.upgrade()
            {
                sources.push(companion);
            }
            if owned.contains(&source.entity_id()) || requests.contains_key(&source.entity_id()) {
                let pending = requests.remove(&source.entity_id()).unwrap_or_default();
                source.update(cx, |map, cx| map.request_viewport_row_geometry(pending, cx));
            }
        }
    }

    fn retain_autoscroll_request(&self, request: Option<(Autoscroll, bool)>, cx: &mut App) {
        if let Some(request) = request {
            self.editor.update(cx, |editor, _| {
                editor.scroll_manager.restore_autoscroll_request(request);
            });
        }
    }

    fn prepaint_lines(
        &self,
        start_row: DisplayRow,
        line_layouts: &mut [LineWithInvisibles],
        line_height: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        content_origin: gpui::Point<Pixels>,
        content_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> SmallVec<[AnyElement; 1]> {
        let mut line_elements = SmallVec::new();
        for (ix, line) in line_layouts.iter_mut().enumerate() {
            let row = start_row + DisplayRow(ix as u32);
            line.prepaint(
                line_height,
                scroll_position,
                scroll_pixel_position,
                row,
                content_origin,
                self.style.text.text_align,
                content_width,
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
        width: BlockWidth,
        block_id: BlockId,
        block_row_start: DisplayRow,
        snapshot: &EditorSnapshot,
        text_x: Pixels,
        rows: &Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        editor_margins: &EditorMargins,
        line_height: Pixels,
        em_width: Pixels,
        text_hitbox: &Hitbox,
        editor_width: Pixels,
        scroll_width: &mut ScrollPixelOffset,
        resized_blocks: &mut HashMap<CustomBlockId, u32>,
        row_block_types: &mut HashMap<DisplayRow, bool>,
        selected_buffer_ids: &Vec<BufferId>,
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        sticky_header_excerpt_id: Option<BufferId>,
        indent_guides: &Option<Vec<IndentGuideLayout>>,
        block_resize_offset: &mut i32,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<
        Option<(
            AnyElement,
            Size<Pixels>,
            DisplayRow,
            ScrollPixelOffset,
            AvailableSpace,
        )>,
        PendingRowGeometry,
    > {
        let mut x_position = None;
        let mut coordinate_base: ScrollPixelOffset = 0.;
        let mut available_width = width.rebased(coordinate_base);
        let mut element = match block {
            Block::Custom(custom) => {
                let block_start = custom.start().to_point(&snapshot.buffer_snapshot());
                if block.place_near() && snapshot.is_line_folded(MultiBufferRow(block_start.row)) {
                    return Ok(None);
                }
                let align_to = block_start.to_display_point(snapshot);
                let text_x = ScrollPixelOffset::from(text_x);
                let x_and_width = |layout: &LineWithInvisibles| {
                    (
                        text_x + layout.x_for_index(align_to.column() as usize),
                        text_x + layout.width,
                    )
                };
                let line_ix = align_to.row().0.checked_sub(rows.start.0);
                let custom_block_x_position =
                    if let Some(layout) = line_ix.and_then(|ix| line_layouts.get(ix as usize)) {
                        x_and_width(layout)
                    } else {
                        x_and_width(&layout_line(
                            align_to.row(),
                            snapshot,
                            &self.style,
                            editor_width,
                            0.,
                            is_row_soft_wrapped,
                            window,
                            cx,
                        )?)
                    };

                if block.style() == BlockStyle::FlexWithAnchor && !block.place_near() {
                    coordinate_base = (custom_block_x_position.0 - BLOCK_COORDINATE_MARGIN).max(0.);
                    available_width = width.rebased(coordinate_base);
                }
                let anchor_x = Pixels::from(custom_block_x_position.0 - coordinate_base);
                x_position = Some(custom_block_x_position);

                let block_range = custom.start()..custom.end();
                let selected = self.editor_with_selections(cx).is_some_and(|editor| {
                    editor
                        .read(cx)
                        .selections
                        .overlaps_range(&block_range, snapshot)
                });

                div()
                    .size_full()
                    .child(
                        custom.render(&mut BlockContext {
                            window,
                            app: cx,
                            anchor_x,
                            margins: editor_margins,
                            line_height,
                            em_width,
                            block_id,
                            height: custom.height.unwrap_or(1),
                            selected,
                            max_width: ScrollPixelOffset::from(text_hitbox.size.width)
                                .max(*scroll_width - coordinate_base),
                            editor_style: &self.style,
                            indent_guide_padding: indent_guides
                                .as_ref()
                                .map(|guides| {
                                    Self::depth_zero_indent_guide_padding_for_row(
                                        guides,
                                        block_row_start,
                                    )
                                })
                                .unwrap_or(px(0.0)),
                        }),
                    )
                    .into_any()
            }

            Block::FoldedBuffer {
                first_excerpt,
                height,
                ..
            } => {
                let mut result = v_flex().id(block_id).w_full().pr(editor_margins.right);

                if self.should_show_buffer_headers() {
                    let selected = selected_buffer_ids.contains(&first_excerpt.buffer_id());
                    let jump_data = header::header_jump_data(
                        snapshot,
                        block_row_start,
                        *height,
                        first_excerpt,
                        latest_selection_anchors,
                    );
                    result = result.child(header::render_buffer_header(
                        &self.editor,
                        first_excerpt,
                        true,
                        selected,
                        false,
                        jump_data,
                        window,
                        cx,
                    ));
                } else {
                    result =
                        result.child(div().h(FILE_HEADER_HEIGHT as f32 * window.line_height()));
                }

                result.into_any_element()
            }

            Block::ExcerptBoundary { .. } => {
                let color = cx.theme().colors().clone();
                let mut result = v_flex().id(block_id).w_full();

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

                result.into_any()
            }

            Block::BufferHeader { excerpt, height } => {
                let mut result = v_flex().id(block_id).w_full();

                if self.should_show_buffer_headers() {
                    let jump_data = header::header_jump_data(
                        snapshot,
                        block_row_start,
                        *height,
                        excerpt,
                        latest_selection_anchors,
                    );

                    if sticky_header_excerpt_id != Some(excerpt.buffer_id()) {
                        let selected = selected_buffer_ids.contains(&excerpt.buffer_id());

                        result = result.child(div().pr(editor_margins.right).child(
                            header::render_buffer_header(
                                &self.editor,
                                excerpt,
                                false,
                                selected,
                                false,
                                jump_data,
                                window,
                                cx,
                            ),
                        ));
                    } else {
                        result =
                            result.child(div().h(FILE_HEADER_HEIGHT as f32 * window.line_height()));
                    }
                } else {
                    result =
                        result.child(div().h(FILE_HEADER_HEIGHT as f32 * window.line_height()));
                }

                result.into_any()
            }

            Block::Spacer { height, .. } => {
                let indent_guide_padding = indent_guides
                    .as_ref()
                    .map(|guides| {
                        Self::depth_zero_indent_guide_padding_for_row(guides, block_row_start)
                    })
                    .unwrap_or(px(0.0));
                Self::render_spacer_block(
                    block_id,
                    *height,
                    line_height,
                    indent_guide_padding,
                    window,
                    cx,
                )
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

        let effective_row_start = block_row_start.0 as i32 + *block_resize_offset;
        debug_assert!(effective_row_start >= 0);
        let mut row = DisplayRow(effective_row_start.max(0) as u32);

        let mut x_offset: ScrollPixelOffset = 0.;
        let mut is_block = true;

        if let BlockId::Custom(custom_block_id) = block_id
            && block.has_height()
        {
            if block.place_near()
                && let Some((x_target, line_width)) = x_position
            {
                let margin = ScrollPixelOffset::from(em_width * 2);
                let block_width = ScrollPixelOffset::from(final_size.width);
                let gutter_width = ScrollPixelOffset::from(editor_margins.gutter.full_width());
                let text_right = ScrollPixelOffset::from(editor_width) + gutter_width;
                if line_width + block_width + margin < text_right
                    && !row_block_types.contains_key(&(row - 1))
                    && element_height_in_lines == 1
                {
                    // Render inline at end of line (for diagnostic blocks that fit)
                    x_offset = line_width + margin;
                    row = row - 1;
                    is_block = false;
                    element_height_in_lines = 0;
                    row_block_types.insert(row, is_block);
                } else {
                    let max_offset = text_right - block_width;
                    let min_offset = (x_target + ScrollPixelOffset::from(em_width) - block_width)
                        .max(gutter_width);
                    x_offset = x_target.min(max_offset).max(min_offset);
                }
            };
            if element_height_in_lines != block.height() {
                *block_resize_offset += element_height_in_lines as i32 - block.height() as i32;
                resized_blocks.insert(custom_block_id, element_height_in_lines);
            }
        }
        for i in 0..element_height_in_lines {
            row_block_types.insert(row + i, is_block);
        }

        Ok(Some((
            element,
            final_size,
            row,
            coordinate_base + x_offset,
            available_width,
        )))
    }

    /// The spacer pattern period must be an even factor of the line height, so
    /// that two consecutive spacer blocks can render contiguously without an
    /// obvious break in the pattern.
    ///
    /// Two consecutive spacers can appear when the other side has a diff hunk
    /// and a custom block next to each other (e.g. merge conflict buttons).
    fn spacer_pattern_period(line_height: f32, target_height: f32) -> f32 {
        let k_approx = line_height / (2.0 * target_height);
        let k_floor = (k_approx.floor() as u32).max(1);
        let k_ceil = (k_approx.ceil() as u32).max(1);

        let size_floor = line_height / (2 * k_floor) as f32;
        let size_ceil = line_height / (2 * k_ceil) as f32;

        if (size_floor - target_height).abs() <= (size_ceil - target_height).abs() {
            size_floor
        } else {
            size_ceil
        }
    }

    fn spacer_background(line_height: Pixels, window: &Window, cx: &App) -> Background {
        let target_size = 16.0;
        let scale = window.scale_factor();
        let pattern_size =
            Self::spacer_pattern_period(f32::from(line_height) * scale, target_size * scale);
        pattern_slash(
            cx.theme().colors().panel_background,
            2.0,
            pattern_size - 2.0,
        )
    }

    pub fn render_spacer_block(
        block_id: BlockId,
        block_height: u32,
        line_height: Pixels,
        indent_guide_padding: Pixels,
        window: &mut Window,
        cx: &App,
    ) -> AnyElement {
        let background = Self::spacer_background(line_height, window, cx);

        div()
            .id(block_id)
            .cursor(CursorStyle::Arrow)
            .w_full()
            .h((block_height as f32) * line_height)
            .flex()
            .flex_row()
            .child(div().flex_shrink_0().w(indent_guide_padding).h_full())
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .relative()
                    .overflow_x_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .right_0()
                            .left(-indent_guide_padding)
                            .bg(background),
                    ),
            )
            .into_any()
    }

    fn render_blocks(
        &self,
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        editor_width: Pixels,
        scroll_width: &mut ScrollPixelOffset,
        editor_margins: &EditorMargins,
        em_width: Pixels,
        text_x: Pixels,
        line_height: Pixels,
        line_layouts: &[LineWithInvisibles],
        selected_buffer_ids: &Vec<BufferId>,
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        sticky_header_excerpt_id: Option<BufferId>,
        indent_guides: &Option<Vec<IndentGuideLayout>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<RenderBlocksOutput, PendingRowGeometry> {
        let (fixed_blocks, non_fixed_blocks) = snapshot
            .blocks_in_range(rows.clone())
            .partition::<Vec<_>, _>(|(_, block)| block.style() == BlockStyle::Fixed);

        let mut focused_block = self
            .editor
            .update(cx, |editor, _| editor.take_focused_block());
        let mut fixed_block_max_width = Pixels::ZERO;
        let mut blocks = Vec::new();
        let mut spacer_blocks = Vec::new();
        let mut resized_blocks = HashMap::default();
        let mut row_block_types = HashMap::default();
        let mut block_resize_offset: i32 = 0;

        for (row, block) in fixed_blocks {
            let block_id = block.id();

            if focused_block.as_ref().is_some_and(|b| b.id == block_id) {
                focused_block = None;
            }

            if let Some((element, element_size, row, x_offset, _)) = self.render_block(
                block,
                BlockWidth::MinContent,
                block_id,
                row,
                snapshot,
                text_x,
                &rows,
                line_layouts,
                editor_margins,
                line_height,
                em_width,
                text_hitbox,
                editor_width,
                scroll_width,
                &mut resized_blocks,
                &mut row_block_types,
                selected_buffer_ids,
                latest_selection_anchors,
                is_row_soft_wrapped,
                sticky_header_excerpt_id,
                indent_guides,
                &mut block_resize_offset,
                window,
                cx,
            )? {
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

        let content_width = |scroll_width: ScrollPixelOffset, leading: Pixels| {
            BlockWidth::Definite(
                ScrollPixelOffset::from(hitbox.size.width.max(fixed_block_max_width)).max(
                    ScrollPixelOffset::from(leading + editor_margins.extended_right) + scroll_width,
                ),
            )
        };
        for (row, block) in non_fixed_blocks {
            let style = block.style();
            let width = match (style, block.place_near()) {
                (_, true) => BlockWidth::MinContent,
                (BlockStyle::Sticky, _) => {
                    BlockWidth::Definite(ScrollPixelOffset::from(hitbox.size.width))
                }
                (BlockStyle::Flex | BlockStyle::FlexWithAnchor, _) => {
                    content_width(*scroll_width, editor_margins.gutter.width)
                }
                (BlockStyle::Spacer, _) => content_width(*scroll_width, Pixels::ZERO),
                (BlockStyle::Fixed, _) => unreachable!(),
            };
            let block_id = block.id();

            if focused_block.as_ref().is_some_and(|b| b.id == block_id) {
                focused_block = None;
            }

            if let Some((element, element_size, row, x_offset, width)) = self.render_block(
                block,
                width,
                block_id,
                row,
                snapshot,
                text_x,
                &rows,
                line_layouts,
                editor_margins,
                line_height,
                em_width,
                text_hitbox,
                editor_width,
                scroll_width,
                &mut resized_blocks,
                &mut row_block_types,
                selected_buffer_ids,
                latest_selection_anchors,
                is_row_soft_wrapped,
                sticky_header_excerpt_id,
                indent_guides,
                &mut block_resize_offset,
                window,
                cx,
            )? {
                let layout = BlockLayout {
                    id: block_id,
                    x_offset,
                    row: Some(row),
                    element,
                    available_space: size(width, element_size.height.into()),
                    style,
                    overlaps_gutter: !block.place_near() && style != BlockStyle::Spacer,
                    is_buffer_header: block.is_buffer_header(),
                };
                if style == BlockStyle::Spacer {
                    spacer_blocks.push(layout);
                } else {
                    blocks.push(layout);
                }
            }
        }

        if let Some(focused_block) = focused_block
            && let Some(focus_handle) = focused_block.focus_handle.upgrade()
            && focus_handle.is_focused(window)
            && let Some(block) = snapshot.block_for_id(focused_block.id)
        {
            let style = block.style();
            let width = match style {
                BlockStyle::Fixed => BlockWidth::MinContent,
                BlockStyle::Flex | BlockStyle::FlexWithAnchor => {
                    content_width(*scroll_width, editor_margins.gutter.width)
                }
                BlockStyle::Spacer => content_width(*scroll_width, Pixels::ZERO),
                BlockStyle::Sticky => {
                    BlockWidth::Definite(ScrollPixelOffset::from(hitbox.size.width))
                }
            };

            if let Some((element, element_size, _, x_offset, width)) = self.render_block(
                &block,
                width,
                focused_block.id,
                rows.end,
                snapshot,
                text_x,
                &rows,
                line_layouts,
                editor_margins,
                line_height,
                em_width,
                text_hitbox,
                editor_width,
                scroll_width,
                &mut resized_blocks,
                &mut row_block_types,
                selected_buffer_ids,
                latest_selection_anchors,
                is_row_soft_wrapped,
                sticky_header_excerpt_id,
                indent_guides,
                &mut block_resize_offset,
                window,
                cx,
            )? {
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

        if resized_blocks.is_empty() {
            *scroll_width = (*scroll_width).max(ScrollPixelOffset::from(
                fixed_block_max_width - editor_margins.gutter.width,
            ));
        }

        Ok(RenderBlocksOutput {
            non_spacer_blocks: blocks,
            spacer_blocks,
            row_block_types,
            resized_blocks: (!resized_blocks.is_empty()).then_some(resized_blocks),
        })
    }

    fn layout_blocks(
        &self,
        blocks: &mut Vec<BlockLayout>,
        hitbox: &Hitbox,
        gutter_hitbox: &Hitbox,
        line_height: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        editor_margins: &EditorMargins,
        window: &mut Window,
        cx: &mut App,
    ) {
        for block in blocks {
            let scroll_x = if block.style == BlockStyle::Sticky {
                0.
            } else {
                scroll_pixel_position.x
            };
            let mut origin = if let Some(row) = block.row {
                hitbox.origin
                    + point(
                        Pixels::from(block.x_offset - scroll_x),
                        Pixels::from(
                            (row.as_f64() - scroll_position.y)
                                * ScrollPixelOffset::from(line_height),
                        ),
                    )
            } else {
                // Position the block outside the visible area
                hitbox.origin + point(Pixels::from(-scroll_x), hitbox.size.height)
            };

            if block.style == BlockStyle::Spacer {
                origin += point(
                    gutter_hitbox.size.width + editor_margins.gutter.margin,
                    Pixels::ZERO,
                );
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

    fn layout_cursor_popovers(
        &self,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        right_margin: Pixels,
        start_row: DisplayRow,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_layouts: &[LineWithInvisibles],
        cursor: DisplayPoint,
        cursor_point: Point,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<ContextMenuLayout> {
        let mut min_menu_height = Pixels::ZERO;
        let mut max_menu_height = Pixels::ZERO;
        let mut height_above_menu = Pixels::ZERO;
        let height_below_menu = Pixels::ZERO;
        let mut edit_prediction_popover_visible = false;
        let mut context_menu_visible = false;
        let context_menu_placement;

        {
            let editor = self.editor.read(cx);
            if editor.edit_prediction_visible_in_cursor_popover(editor.has_active_edit_prediction())
            {
                height_above_menu +=
                    editor.edit_prediction_cursor_popover_height() + POPOVER_Y_PADDING;
                edit_prediction_popover_visible = true;
            }

            if editor.context_menu_visible()
                && let Some(crate::ContextMenuOrigin::Cursor) = editor.context_menu_origin()
            {
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
            context_menu_placement = editor
                .context_menu_options
                .as_ref()
                .and_then(|options| options.placement.clone());
        }

        let visible = edit_prediction_popover_visible || context_menu_visible;
        if !visible {
            return None;
        }

        let cursor_row_layout = &line_layouts[cursor.row().minus(start_row) as usize];
        let target_position = content_origin
            + gpui::Point {
                x: cmp::max(
                    px(0.),
                    Pixels::from(
                        cursor_row_layout.x_for_index(cursor.column() as usize)
                            - scroll_pixel_position.x,
                    ),
                ),
                y: cmp::max(
                    px(0.),
                    Pixels::from(
                        cursor.row().next_row().as_f64() * ScrollPixelOffset::from(line_height)
                            - scroll_pixel_position.y,
                    ),
                ),
            };

        let viewport_bounds =
            Bounds::new(Default::default(), window.viewport_size()).extend(Edges {
                right: -right_margin - MENU_GAP,
                ..Default::default()
            });

        let min_height = height_above_menu + min_menu_height + height_below_menu;
        let max_height = height_above_menu + max_menu_height + height_below_menu;
        let (laid_out_popovers, y_flipped) = self.layout_popovers_above_or_below_line(
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
                        let mut element = editor.render_edit_prediction_cursor_popover(
                            min_width,
                            max_width,
                            cursor_point,
                            style,
                            window,
                            cx,
                        )?;
                        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                        Some((CursorPopoverType::EditPrediction, element, size))
                    })
                } else {
                    None
                };
                [edit_prediction, context_menu]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            },
        )?;

        let (menu_ix, (_, menu_bounds)) = laid_out_popovers
            .iter()
            .find_position(|(x, _)| matches!(x, CursorPopoverType::CodeContextMenu))?;
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

        let aside_bounds = self.layout_context_menu_aside(
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

        if let Some(menu_bounds) = laid_out_popovers.iter().find_map(|(popover_type, bounds)| {
            if matches!(popover_type, CursorPopoverType::CodeContextMenu) {
                Some(*bounds)
            } else {
                None
            }
        }) {
            let bounds = if let Some(aside_bounds) = aside_bounds {
                menu_bounds.union(&aside_bounds)
            } else {
                menu_bounds
            };
            return Some(ContextMenuLayout { y_flipped, bounds });
        }

        None
    }

    fn layout_gutter_menu(
        &self,
        line_height: Pixels,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        right_margin: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
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
                y: Pixels::from(
                    gutter_row.next_row().as_f64() * ScrollPixelOffset::from(line_height)
                        - scroll_pixel_position.y,
                ),
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
                right: -right_margin - MENU_GAP,
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
                    window.defer_draw(element, current_position, 1, None);
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
    ) -> Option<Bounds<Pixels>> {
        let available_within_viewport = target_bounds.space_within(&viewport_bounds);
        let positioned_aside = if available_within_viewport.right >= MENU_ASIDE_MIN_WIDTH
            && !must_place_above_or_below
        {
            let max_width = cmp::min(
                available_within_viewport.right - px(1.),
                MENU_ASIDE_MAX_WIDTH,
            );
            let mut aside = self.render_context_menu_aside(
                size(max_width, max_height - POPOVER_Y_PADDING),
                window,
                cx,
            )?;
            let size = aside.layout_as_root(AvailableSpace::min_size(), window, cx);
            let right_position = point(target_bounds.right(), menu_bounds.origin.y);
            Some((aside, right_position, size))
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
            let mut aside = self.render_context_menu_aside(max_size, window, cx)?;
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

            aside_position.map(|position| (aside, position, actual_size))
        };

        // Skip drawing if it doesn't fit anywhere.
        if let Some((aside, position, size)) = positioned_aside {
            let aside_bounds = Bounds::new(position, size);
            window.defer_draw(aside, position, 2, None);
            return Some(aside_bounds);
        }

        None
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
            editor.render_context_menu(max_height_in_lines, window, cx)
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

    fn layout_hover_popovers(
        &self,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        visible_display_row_range: Range<DisplayRow>,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        em_width: Pixels,
        context_menu_layout: Option<ContextMenuLayout>,
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

        // Don't show hover popovers when context menu is open to avoid overlap
        let has_context_menu = self.editor.read(cx).mouse_context_menu.is_some();
        if has_context_menu {
            return;
        }

        let hover_popovers = self.editor.update(cx, |editor, cx| {
            editor.hover_state.render(
                snapshot,
                visible_display_row_range.clone(),
                max_size,
                &editor.text_layout_details(window, cx),
                window,
                cx,
            )
        });
        let Some((popover_position, hover_popovers)) = hover_popovers else {
            return;
        };

        // This is safe because we check on layout whether the required row is available
        let hovered_row_layout = &line_layouts[popover_position
            .row()
            .minus(visible_display_row_range.start)
            as usize];

        // Compute Hovered Point
        let x = Pixels::from(
            hovered_row_layout.x_for_index(popover_position.column() as usize)
                - scroll_pixel_position.x,
        );
        let y = Pixels::from(
            popover_position.row().as_f64() * ScrollPixelOffset::from(line_height)
                - scroll_pixel_position.y,
        );
        let hovered_point = content_origin + point(x, y);

        let mut overall_height = Pixels::ZERO;

        let measured_hover_popovers = hover_popovers
            .into_iter()
            .with_position()
            .map(|(position, mut hover_popover)| {
                let size = hover_popover.layout_as_root(AvailableSpace::min_size(), window, cx);
                let horizontal_offset =
                    (hitbox.top_right().x - POPOVER_RIGHT_OFFSET - (hovered_point.x + size.width))
                        .min(Pixels::ZERO);
                match position {
                    itertools::Position::Middle | itertools::Position::Last => {
                        overall_height += HOVER_POPOVER_GAP
                    }
                    _ => {}
                }
                overall_height += size.height;
                MeasuredHoverPopover {
                    element: hover_popover,
                    size,
                    horizontal_offset,
                }
            })
            .collect::<Vec<_>>();

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
            window.defer_draw(occlusion, origin, 2, None);
        }

        fn place_popovers_above(
            hovered_point: gpui::Point<Pixels>,
            measured_hover_popovers: Vec<MeasuredHoverPopover>,
            window: &mut Window,
            cx: &mut App,
        ) {
            let mut current_y = hovered_point.y;
            for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                let size = popover.size;
                let popover_origin = point(
                    hovered_point.x + popover.horizontal_offset,
                    current_y - size.height,
                );

                window.defer_draw(popover.element, popover_origin, 2, None);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y - HOVER_POPOVER_GAP);
                    draw_occluder(size.width, origin, window, cx);
                }

                current_y = popover_origin.y - HOVER_POPOVER_GAP;
            }
        }

        fn place_popovers_below(
            hovered_point: gpui::Point<Pixels>,
            measured_hover_popovers: Vec<MeasuredHoverPopover>,
            line_height: Pixels,
            window: &mut Window,
            cx: &mut App,
        ) {
            let mut current_y = hovered_point.y + line_height;
            for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                let size = popover.size;
                let popover_origin = point(hovered_point.x + popover.horizontal_offset, current_y);

                window.defer_draw(popover.element, popover_origin, 2, None);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y + size.height);
                    draw_occluder(size.width, origin, window, cx);
                }

                current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
            }
        }

        let intersects_menu = |bounds: Bounds<Pixels>| -> bool {
            context_menu_layout
                .as_ref()
                .is_some_and(|menu| bounds.intersects(&menu.bounds))
        };

        let can_place_above = {
            let mut current_y = hovered_point.y;
            measured_hover_popovers.iter().all(|popover| {
                let size = popover.size;
                let popover_origin = point(
                    hovered_point.x + popover.horizontal_offset,
                    current_y - size.height,
                );
                let bounds = Bounds::new(popover_origin, size);
                current_y = popover_origin.y - HOVER_POPOVER_GAP;
                bounds.is_contained_within(hitbox) && !intersects_menu(bounds)
            })
        };

        let can_place_below = || {
            let mut current_y = hovered_point.y + line_height;
            measured_hover_popovers.iter().all(|popover| {
                let size = popover.size;
                let popover_origin = point(hovered_point.x + popover.horizontal_offset, current_y);
                let bounds = Bounds::new(popover_origin, size);
                current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
                bounds.is_contained_within(hitbox) && !intersects_menu(bounds)
            })
        };

        if can_place_above {
            // try placing above hovered point
            place_popovers_above(hovered_point, measured_hover_popovers, window, cx);
        } else if can_place_below() {
            // try placing below hovered point
            place_popovers_below(
                hovered_point,
                measured_hover_popovers,
                line_height,
                window,
                cx,
            );
        } else {
            // try to place popovers around the context menu
            let origin_surrounding_menu = context_menu_layout.as_ref().and_then(|menu| {
                let total_width = measured_hover_popovers
                    .iter()
                    .map(|p| p.size.width)
                    .max()
                    .unwrap_or(Pixels::ZERO);
                let y_for_horizontal_positioning = if menu.y_flipped {
                    menu.bounds.bottom() - overall_height
                } else {
                    menu.bounds.top()
                };
                let possible_origins = [
                    // left of context menu
                    point(
                        menu.bounds.left() - total_width - HOVER_POPOVER_GAP,
                        y_for_horizontal_positioning,
                    ),
                    // right of context menu
                    point(
                        menu.bounds.right() + HOVER_POPOVER_GAP,
                        y_for_horizontal_positioning,
                    ),
                    // top of context menu
                    point(
                        menu.bounds.left(),
                        menu.bounds.top() - overall_height - HOVER_POPOVER_GAP,
                    ),
                    // bottom of context menu
                    point(menu.bounds.left(), menu.bounds.bottom() + HOVER_POPOVER_GAP),
                ];
                possible_origins.into_iter().find(|&origin| {
                    Bounds::new(origin, size(total_width, overall_height))
                        .is_contained_within(hitbox)
                })
            });
            if let Some(origin) = origin_surrounding_menu {
                let mut current_y = origin.y;
                for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                    let size = popover.size;
                    let popover_origin = point(origin.x, current_y);

                    window.defer_draw(popover.element, popover_origin, 2, None);
                    if position != itertools::Position::Last {
                        let origin = point(popover_origin.x, popover_origin.y + size.height);
                        draw_occluder(size.width, origin, window, cx);
                    }

                    current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
                }
            } else {
                // fallback to existing above/below cursor logic
                // this might overlap menu or overflow in rare case
                if can_place_above {
                    place_popovers_above(hovered_point, measured_hover_popovers, window, cx);
                } else {
                    place_popovers_below(
                        hovered_point,
                        measured_hover_popovers,
                        line_height,
                        window,
                        cx,
                    );
                }
            }
        }
    }

    fn layout_word_diff_highlights(
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        row_infos: &[RowInfo],
        start_row: DisplayRow,
        visible_ranges: &[Range<Anchor>],
        snapshot: &EditorSnapshot,
        highlighted_ranges: &mut Vec<(Range<DisplayPoint>, Hsla)>,
        cx: &mut App,
    ) {
        let colors = cx.theme().colors();

        let buffer = snapshot.buffer_snapshot();
        let visible_ranges = visible_ranges
            .iter()
            .map(|range| range.start.to_offset(buffer)..range.end.to_offset(buffer))
            .collect::<Vec<_>>();

        // Gather the word diffs that intersect the viewport. A hunk stores the
        // word diffs for its entire range, so without this filter a large hunk
        // that is only partially scrolled into view would cost work
        // proportional to its whole size every frame.
        let mut visible_word_diffs: Vec<&Range<MultiBufferOffset>> = display_hunks
            .iter()
            .filter_map(|(hunk, _)| match hunk {
                DisplayDiffHunk::Unfolded {
                    word_diffs, status, ..
                } if status.is_modified() => Some(word_diffs),
                _ => None,
            })
            .flatten()
            .filter(|word_diff| {
                let index =
                    visible_ranges.partition_point(|visible| visible.end <= word_diff.start);
                visible_ranges
                    .get(index)
                    .is_some_and(|visible| visible.start < word_diff.end)
            })
            .collect();

        // The converter walks each display-map layer with a forward-only cursor,
        // so it must receive ranges in non-decreasing order. Word diffs are
        // disjoint, so sorting by offset yields a monotonic sequence.
        visible_word_diffs.sort_unstable_by_key(|word_diff| (word_diff.start, word_diff.end));

        let mut converter = snapshot.display_snapshot.display_point_converter();
        for word_diff in visible_word_diffs {
            for range in converter.map(word_diff.start..word_diff.end) {
                let start_row_offset = range.start.row().0.saturating_sub(start_row.0) as usize;

                let Some(diff_status) = row_infos
                    .get(start_row_offset)
                    .and_then(|row_info| row_info.diff_status)
                else {
                    continue;
                };

                let background_color = match diff_status.kind {
                    DiffHunkStatusKind::Added => colors.version_control_word_added,
                    DiffHunkStatusKind::Deleted => colors.version_control_word_deleted,
                    DiffHunkStatusKind::Modified => {
                        debug_panic!("modified diff status for row info");
                        continue;
                    }
                };

                highlighted_ranges.push((range, background_color));
            }
        }
    }

    fn layout_diff_hunk_controls(
        &self,
        row_range: Range<DisplayRow>,
        row_infos: &[RowInfo],
        text_hitbox: &Hitbox,
        newest_cursor_row: Option<DisplayRow>,
        line_height: Pixels,
        right_margin: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        sticky_header_height: Pixels,
        display_hunks: &[(DisplayDiffHunk, Option<Hitbox>)],
        highlighted_rows: &BTreeMap<DisplayRow, LineHighlight>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) -> (Vec<AnyElement>, Vec<(DisplayRow, Bounds<Pixels>)>) {
        let diff_hunk_renderer = editor.read(cx).diff_hunk_renderer();
        let hovered_diff_hunk_row = editor.read(cx).hovered_diff_hunk_row;
        let sticky_top = text_hitbox.bounds.top() + sticky_header_height;

        let mut controls = vec![];
        let mut control_bounds = vec![];

        let active_rows = [hovered_diff_hunk_row, newest_cursor_row];

        for (hunk, _) in display_hunks {
            if let DisplayDiffHunk::Unfolded {
                display_row_range,
                multi_buffer_range,
                status,
                is_created_file,
                ..
            } = &hunk
            {
                if display_row_range.start >= row_range.end {
                    // hunk is fully below the viewport
                    continue;
                }
                if display_row_range.end <= row_range.start {
                    // hunk is fully above the viewport
                    continue;
                }
                let row_ix = display_row_range.start.0.saturating_sub(row_range.start.0);
                if row_infos
                    .get(row_ix as usize)
                    .and_then(|row_info| row_info.diff_status)
                    .is_none()
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

                if active_rows
                    .iter()
                    .any(|row| row.is_some_and(|row| display_row_range.contains(&row)))
                {
                    let hunk_start_y: Pixels = (display_row_range.start.as_f64()
                        * ScrollPixelOffset::from(line_height)
                        + ScrollPixelOffset::from(text_hitbox.bounds.top())
                        - scroll_pixel_position.y)
                        .into();

                    let y: Pixels = if hunk_start_y >= sticky_top {
                        hunk_start_y
                    } else {
                        let hunk_end_y: Pixels = hunk_start_y
                            + (display_row_range.len() as f64
                                * ScrollPixelOffset::from(line_height))
                            .into();
                        let max_y = hunk_end_y - line_height;
                        sticky_top.min(max_y)
                    };

                    let mut element = diff_hunk_renderer.render_hunk_controls(
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

                    let x = text_hitbox.bounds.right() - right_margin - px(10.) - size.width;

                    if x < text_hitbox.bounds.left() {
                        continue;
                    }

                    let bounds = Bounds::new(gpui::Point::new(x, y), size);
                    control_bounds.push((display_row_range.start, bounds));

                    window.with_absolute_element_offset(gpui::Point::new(x, y), |window| {
                        element.prepaint(window, cx)
                    });
                    controls.push(element);
                }
            }
        }

        (controls, control_bounds)
    }

    fn layout_signature_help(
        &self,
        hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        newest_selection_head: Option<DisplayPoint>,
        start_row: DisplayRow,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        em_width: Pixels,
        context_menu_layout: Option<ContextMenuLayout>,
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
                let element = popover.render(max_size, window, cx);
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

        let target_x = Pixels::from(
            cursor_row_layout.x_for_index(newest_selection_head.column() as usize)
                - scroll_pixel_position.x,
        );
        let target_y = Pixels::from(
            selection_row.as_f64() * ScrollPixelOffset::from(line_height) - scroll_pixel_position.y,
        );
        let target_point = content_origin + point(target_x, target_y);

        let actual_size = element.layout_as_root(Size::<AvailableSpace>::default(), window, cx);

        let (popover_bounds_above, popover_bounds_below) = {
            let horizontal_offset = (hitbox.top_right().x
                - POPOVER_RIGHT_OFFSET
                - (target_point.x + actual_size.width))
                .min(Pixels::ZERO);
            let initial_x = target_point.x + horizontal_offset;
            (
                Bounds::new(
                    point(initial_x, target_point.y - actual_size.height),
                    actual_size,
                ),
                Bounds::new(
                    point(initial_x, target_point.y + line_height + HOVER_POPOVER_GAP),
                    actual_size,
                ),
            )
        };

        let intersects_menu = |bounds: Bounds<Pixels>| -> bool {
            context_menu_layout
                .as_ref()
                .is_some_and(|menu| bounds.intersects(&menu.bounds))
        };

        let final_origin = if popover_bounds_above.is_contained_within(hitbox)
            && !intersects_menu(popover_bounds_above)
        {
            // try placing above cursor
            popover_bounds_above.origin
        } else if popover_bounds_below.is_contained_within(hitbox)
            && !intersects_menu(popover_bounds_below)
        {
            // try placing below cursor
            popover_bounds_below.origin
        } else {
            // try surrounding context menu if exists
            let origin_surrounding_menu = context_menu_layout.as_ref().and_then(|menu| {
                let y_for_horizontal_positioning = if menu.y_flipped {
                    menu.bounds.bottom() - actual_size.height
                } else {
                    menu.bounds.top()
                };
                let possible_origins = [
                    // left of context menu
                    point(
                        menu.bounds.left() - actual_size.width - HOVER_POPOVER_GAP,
                        y_for_horizontal_positioning,
                    ),
                    // right of context menu
                    point(
                        menu.bounds.right() + HOVER_POPOVER_GAP,
                        y_for_horizontal_positioning,
                    ),
                    // top of context menu
                    point(
                        menu.bounds.left(),
                        menu.bounds.top() - actual_size.height - HOVER_POPOVER_GAP,
                    ),
                    // bottom of context menu
                    point(menu.bounds.left(), menu.bounds.bottom() + HOVER_POPOVER_GAP),
                ];
                possible_origins
                    .into_iter()
                    .find(|&origin| Bounds::new(origin, actual_size).is_contained_within(hitbox))
            });
            origin_surrounding_menu.unwrap_or_else(|| {
                // fallback to existing above/below cursor logic
                // this might overlap menu or overflow in rare case
                if popover_bounds_above.is_contained_within(hitbox) {
                    popover_bounds_above.origin
                } else {
                    popover_bounds_below.origin
                }
            })
        };

        window.defer_draw(element, final_origin, 2, None);
    }

    fn paint_background(&self, layout: &EditorLayout, window: &mut Window, cx: &mut App) {
        window.paint_layer(layout.hitbox.bounds, |window| {
            let scroll_top = layout.position_map.scroll_position.y;
            let gutter_bg = cx.theme().colors().editor_gutter_background;
            window.paint_quad(fill(layout.gutter_hitbox.bounds, gutter_bg));
            window.paint_quad(fill(
                layout.position_map.text_hitbox.bounds,
                self.style.background,
            ));

            if matches!(
                layout.mode,
                EditorMode::Full { .. } | EditorMode::Minimap { .. }
            ) {
                let show_active_line_background = match layout.mode {
                    EditorMode::Full {
                        show_active_line_background,
                        ..
                    } => show_active_line_background,
                    EditorMode::Minimap { .. } => true,
                    _ => false,
                };
                let mut active_rows = layout.active_rows.iter().peekable();
                while let Some((start_row, contains_non_empty_selection)) = active_rows.next() {
                    let mut end_row = start_row.0;
                    while active_rows
                        .peek()
                        .is_some_and(|(active_row, has_selection)| {
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
                                        + Pixels::from(
                                            (start_row.as_f64() - scroll_top)
                                                * ScrollPixelOffset::from(
                                                    layout.position_map.line_height,
                                                ),
                                        ),
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
                            + Pixels::from(
                                (highlight_row_start.as_f64() - scroll_top)
                                    * ScrollPixelOffset::from(layout.position_map.line_height),
                            ),
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

                for (guide_x, active) in layout.wrap_guides.iter() {
                    let color = if *active {
                        cx.theme().colors().editor_active_wrap_guide
                    } else {
                        cx.theme().colors().editor_wrap_guide
                    };
                    window.paint_quad(fill(
                        window.pixel_snap_bounds(Bounds {
                            origin: point(*guide_x, layout.position_map.text_hitbox.origin.y),
                            size: size(px(1.), layout.position_map.text_hitbox.size.height),
                        }),
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
            let settings = &indent_guide.settings;

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

            let mut line_indicator_width = 0.;
            if let Some(requested_line_width) = settings.visible_line_width(indent_guide.active) {
                if let Some(color) = line_color {
                    window.paint_quad(fill(
                        window.pixel_snap_bounds(Bounds {
                            origin: indent_guide.origin,
                            size: size(px(requested_line_width as f32), indent_guide.length),
                        }),
                        color,
                    ));
                    line_indicator_width = requested_line_width as f32;
                }
            }

            if let Some(color) = background_color {
                let width = indent_guide.single_indent_width - px(line_indicator_width);
                window.paint_quad(fill(
                    window.pixel_snap_bounds(Bounds {
                        origin: point(
                            indent_guide.origin.x + px(line_indicator_width),
                            indent_guide.origin.y,
                        ),
                        size: size(width, indent_guide.length),
                    }),
                    color,
                ));
            }
        }
    }

    fn paint_line_numbers(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        let is_singleton = self.editor.read(cx).buffer_kind(cx) == ItemBufferKind::Singleton;

        let line_height = layout.position_map.line_height;
        window.set_cursor_style(CursorStyle::Arrow, &layout.gutter_hitbox);

        for line_layout in layout.line_numbers.values() {
            for LineNumberSegment {
                shaped_line,
                hitbox,
            } in &line_layout.segments
            {
                let Some(hitbox) = hitbox else {
                    continue;
                };

                let Some(()) = (if !is_singleton && hitbox.is_hovered(window) {
                    let color = cx.theme().colors().editor_hover_line_number;

                    let line = self.shape_line_number(shaped_line.text.clone(), color, window);
                    line.paint(
                        hitbox.origin,
                        line_height,
                        TextAlign::Left,
                        None,
                        window,
                        cx,
                    )
                    .log_err()
                } else {
                    shaped_line
                        .paint(
                            hitbox.origin,
                            line_height,
                            TextAlign::Left,
                            None,
                            window,
                            cx,
                        )
                        .log_err()
                }) else {
                    continue;
                };

                // In singleton buffers, we select corresponding lines on the line number click, so use | -like cursor.
                // In multi buffers, we open file at the line number clicked, so use a pointing hand cursor.
                if is_singleton {
                    window.set_cursor_style(CursorStyle::IBeam, hitbox);
                } else {
                    window.set_cursor_style(CursorStyle::PointingHand, hitbox);
                }
            }
        }
    }

    fn paint_gutter_diff_hunks(
        &self,
        layout: &mut EditorLayout,
        split_side: Option<SplitSide>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if layout.display_hunks.is_empty() {
            return;
        }

        let line_height = layout.position_map.line_height;
        window.paint_layer(layout.gutter_hitbox.bounds, |window| {
            for (hunk, hitbox) in &layout.display_hunks {
                let hunk_to_paint = match hunk {
                    DisplayDiffHunk::Folded { .. } => {
                        let hunk_bounds = Self::diff_hunk_bounds(
                            layout.position_map.scroll_position,
                            line_height,
                            layout.gutter_hitbox.bounds,
                            hunk,
                            &layout.position_map.snapshot,
                            cx,
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
                    } => hitbox.as_ref().map(|hunk_hitbox| {
                        let color = match split_side {
                            Some(SplitSide::Left) => cx.theme().colors().version_control_deleted,
                            Some(SplitSide::Right) => cx.theme().colors().version_control_added,
                            None => match status.kind {
                                DiffHunkStatusKind::Added => {
                                    cx.theme().colors().version_control_added
                                }
                                DiffHunkStatusKind::Modified => {
                                    cx.theme().colors().version_control_modified
                                }
                                DiffHunkStatusKind::Deleted => {
                                    cx.theme().colors().version_control_deleted
                                }
                            },
                        };
                        match status.kind {
                            DiffHunkStatusKind::Deleted if display_row_range.is_empty() => (
                                Bounds::new(
                                    point(
                                        hunk_hitbox.origin.x - hunk_hitbox.size.width,
                                        hunk_hitbox.origin.y,
                                    ),
                                    size(hunk_hitbox.size.width * 2., hunk_hitbox.size.height),
                                ),
                                color,
                                Corners::all(1. * line_height),
                                *status,
                            ),
                            _ => (hunk_hitbox.bounds, color, Corners::all(px(0.)), *status),
                        }
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

                    if !self.diff_hunk_hollow(status, cx) {
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
                            Edges::all(px(1.0)),
                            flattened_background_color,
                            BorderStyle::Solid,
                        ));
                    }
                }
            }
        });
    }

    const DEFAULT_STRIP_WIDTH_RATIO: f32 = 0.275;
    const DELETED_MARKER_WIDTH_RATIO: f32 = 0.35 / Self::DEFAULT_STRIP_WIDTH_RATIO;
    const MIN_DELETED_MARKER_WIDTH_RATIO: f32 = 0.2;

    fn gutter_strip_width(line_height: Pixels, cx: &App) -> Pixels {
        match EditorSettings::get_global(cx).gutter.git_gutter_width {
            GitGutterWidth::Custom(width) => px(*width),
            GitGutterWidth::Default => (Self::DEFAULT_STRIP_WIDTH_RATIO * line_height).floor(),
        }
    }

    fn deleted_marker_base_width(setting: GitGutterWidth, line_height: Pixels) -> Pixels {
        match setting {
            GitGutterWidth::Custom(width) => {
                let scaled_width = px(*width * Self::DELETED_MARKER_WIDTH_RATIO);
                if scaled_width > Pixels::ZERO {
                    let default_strip_width = Self::DEFAULT_STRIP_WIDTH_RATIO * line_height;
                    let boost_factor = (1.0 - *width / f32::from(default_strip_width)).max(0.0);
                    scaled_width + line_height * Self::MIN_DELETED_MARKER_WIDTH_RATIO * boost_factor
                } else {
                    Pixels::ZERO
                }
            }
            GitGutterWidth::Default => {
                (Self::DEFAULT_STRIP_WIDTH_RATIO * line_height * Self::DELETED_MARKER_WIDTH_RATIO)
                    .floor()
            }
        }
    }

    fn diff_hunk_bounds(
        scroll_position: gpui::Point<ScrollOffset>,
        line_height: Pixels,
        gutter_bounds: Bounds<Pixels>,
        hunk: &DisplayDiffHunk,
        snapshot: &EditorSnapshot,
        cx: &App,
    ) -> Bounds<Pixels> {
        let scroll_top = scroll_position.y * ScrollPixelOffset::from(line_height);
        let gutter_strip_width = Self::gutter_strip_width(line_height, cx);

        match hunk {
            DisplayDiffHunk::Folded { display_row, .. } => {
                let start_y = (display_row.as_f64() * ScrollPixelOffset::from(line_height)
                    - scroll_top)
                    .into();
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

                    let offset = ScrollPixelOffset::from(line_height / 2.);
                    let start_y =
                        (row.as_f64() * ScrollPixelOffset::from(line_height) - offset - scroll_top)
                            .into();
                    let end_y = start_y + line_height;

                    let width = Self::deleted_marker_base_width(
                        EditorSettings::get_global(cx).gutter.git_gutter_width,
                        line_height,
                    );
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
                            if matches!(
                                block,
                                Block::ExcerptBoundary { .. } | Block::BufferHeader { .. }
                            ) {
                                Some(start_row)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(end_row);

                    let start_y = (start_row.as_f64() * ScrollPixelOffset::from(line_height)
                        - scroll_top)
                        .into();
                    let end_y = Pixels::from(
                        end_row_in_current_excerpt.as_f64() * ScrollPixelOffset::from(line_height)
                            - scroll_top,
                    );

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

            for bookmark in layout.bookmarks.iter_mut() {
                bookmark.paint(window, cx);
            }

            for breakpoint in layout.breakpoints.iter_mut() {
                breakpoint.paint(window, cx);
            }

            for test_indicator in layout.test_indicators.iter_mut() {
                test_indicator.paint(window, cx);
            }

            if let Some(diff_review_button) = layout.diff_review_button.as_mut() {
                diff_review_button.paint(window, cx);
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
            if let Some(hunk_hitbox) = hunk_hitbox
                && !self
                    .editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .all_diff_hunks_expanded()
            {
                window.set_cursor_style(CursorStyle::PointingHand, hunk_hitbox);
            }
        }

        let show_git_gutter = layout
            .position_map
            .snapshot
            .show_git_diff_gutter
            .unwrap_or_else(|| {
                matches!(
                    ProjectSettings::get_global(cx).git.git_gutter,
                    GitGutterSetting::TrackedFiles
                )
            });
        if show_git_gutter {
            self.paint_gutter_diff_hunks(layout, self.split_side, window, cx)
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
                    + Pixels::from(
                        start_row.0 as f64
                            * ScrollPixelOffset::from(layout.position_map.line_height)
                            - layout.position_map.scroll_pixel_position.y,
                    );
                let end_y = layout.gutter_hitbox.top()
                    + Pixels::from(
                        (end_row.0 + 1) as f64
                            * ScrollPixelOffset::from(layout.position_map.line_height)
                            - layout.position_map.scroll_pixel_position.y,
                    );
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

    fn populate_point_diagnostics(
        &self,
        snapshot: &EditorSnapshot,
        visible_rows: Range<DisplayRow>,
        visible_ranges: &[Range<Anchor>],
        line_layouts: &mut [LineWithInvisibles],
    ) {
        if visible_rows.is_empty() {
            return;
        }

        let display_snapshot = &snapshot.display_snapshot;
        let buffer_snapshot = snapshot.buffer_snapshot();
        let mut diagnostics_per_line = iter::repeat_with(Vec::new)
            .take(line_layouts.len())
            .collect::<Vec<_>>();

        for (point, diagnostic) in visible_ranges.iter().flat_map(|range| {
            Self::point_diagnostics_in_range(
                buffer_snapshot,
                range.start.to_point(buffer_snapshot)..range.end.to_point(buffer_snapshot),
            )
        }) {
            if display_snapshot.intersects_fold(point) {
                continue;
            }

            let Some(underline) = display_snapshot.diagnostic_underline_style(
                diagnostic.severity,
                diagnostic.underline,
                diagnostic.is_unnecessary,
                &self.style,
            ) else {
                continue;
            };

            let display_point = point.to_display_point(display_snapshot);
            if !visible_rows.contains(&display_point.row()) {
                continue;
            }

            let line_ix = display_point.row().minus(visible_rows.start) as usize;
            let Some(line_diagnostics) = diagnostics_per_line.get_mut(line_ix) else {
                continue;
            };
            line_diagnostics.push(PointDiagnostic {
                column: display_point.column(),
                underline,
                severity: diagnostic.severity,
            });
        }

        for (line_layout, diagnostics) in line_layouts.iter_mut().zip(diagnostics_per_line) {
            line_layout.set_point_diagnostics(diagnostics);
        }
    }

    fn point_diagnostic_anchor(
        snapshot: &MultiBufferSnapshot,
        excerpt: &ExcerptRange<text::Anchor>,
        text_anchor: text::Anchor,
    ) -> Option<Anchor> {
        let buffer = snapshot.buffer_for_id(text_anchor.buffer_id)?;
        if !excerpt.contains(&text_anchor, buffer) {
            return None;
        }
        let anchor = text_anchor.bias_right(buffer);
        let anchor = if excerpt.context.start.cmp(&anchor, buffer).is_gt() {
            excerpt.context.start
        } else if excerpt.context.end.cmp(&anchor, buffer).is_lt() {
            excerpt.context.end
        } else {
            anchor
        };
        snapshot.anchor_in_buffer(anchor)
    }

    fn add_point_diagnostic<'a>(
        point_diagnostics: &mut Vec<(Point, &'a Diagnostic)>,
        seen: &mut HashSet<(Point, *const Diagnostic)>,
        range: &Range<Point>,
        point: Point,
        diagnostic: &'a Diagnostic,
    ) {
        if point >= range.start
            && point <= range.end
            && seen.insert((point, std::ptr::from_ref(diagnostic)))
        {
            point_diagnostics.push((point, diagnostic));
        }
    }

    fn point_diagnostics_in_range(
        snapshot: &MultiBufferSnapshot,
        range: Range<Point>,
    ) -> Vec<(Point, &Diagnostic)> {
        let mut point_diagnostics = Vec::new();
        let mut seen = HashSet::default();

        for (buffer, buffer_range, excerpt) in snapshot.range_to_buffer_ranges(range.clone()) {
            for entry in buffer
                .diagnostic_entries_in_range(buffer_range, false)
                .filter(|entry| entry.range.start == entry.range.end)
            {
                let Some(anchor) =
                    Self::point_diagnostic_anchor(snapshot, &excerpt, entry.range.start)
                else {
                    continue;
                };
                Self::add_point_diagnostic(
                    &mut point_diagnostics,
                    &mut seen,
                    &range,
                    anchor.to_point(snapshot),
                    &entry.diagnostic,
                );
            }
        }

        for (_, _, deleted_hunk_anchor) in
            snapshot.range_to_buffer_ranges_with_deleted_hunks(range.clone())
        {
            let Some(deleted_hunk_anchor) = deleted_hunk_anchor else {
                continue;
            };
            let Some((live_anchor, buffer)) = snapshot.anchor_to_buffer_anchor(deleted_hunk_anchor)
            else {
                continue;
            };
            let Some((_, excerpt)) =
                snapshot.excerpt_containing(deleted_hunk_anchor..deleted_hunk_anchor)
            else {
                continue;
            };
            let live_point = live_anchor.to_point(buffer);

            for entry in buffer
                .diagnostic_entries_in_range(live_point..live_point, false)
                .filter(|entry| entry.range.start == entry.range.end)
            {
                if entry.range.start.to_point(buffer) != live_point {
                    continue;
                }
                let Some(anchor) =
                    Self::point_diagnostic_anchor(snapshot, &excerpt, entry.range.start)
                else {
                    continue;
                };
                Self::add_point_diagnostic(
                    &mut point_diagnostics,
                    &mut seen,
                    &range,
                    anchor.to_point(snapshot),
                    &entry.diagnostic,
                );
            }
        }

        point_diagnostics
    }

    fn paint_text(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        window.with_content_mask(
            Some(ContentMask {
                bounds: layout.position_map.text_hitbox.bounds,
            }),
            |window| {
                let editor = self.editor.read(cx);
                if let SelectionDragState::ReadyToDrag {
                    mouse_down_time, ..
                } = &editor.selection_drag_state
                {
                    let drag_and_drop_delay = Duration::from_millis(
                        EditorSettings::get_global(cx)
                            .drag_and_drop_selection
                            .delay
                            .0,
                    );
                    if mouse_down_time.elapsed() >= drag_and_drop_delay {
                        window.set_cursor_style(
                            CursorStyle::DragCopy,
                            &layout.position_map.text_hitbox,
                        );
                    }
                } else if matches!(
                    editor.selection_drag_state,
                    SelectionDragState::Dragging { .. }
                ) {
                    window
                        .set_cursor_style(CursorStyle::DragCopy, &layout.position_map.text_hitbox);
                } else if editor
                    .hovered_link_state
                    .as_ref()
                    .is_some_and(|hovered_link_state| !hovered_link_state.links.is_empty())
                {
                    window.set_cursor_style(
                        CursorStyle::PointingHand,
                        &layout.position_map.text_hitbox,
                    );
                } else if !window.modifiers().modified()
                    && let Some(hovered_command) = editor.hovered_inlay_hint_command()
                    && hovered_command.contains_glyph(
                        &layout.position_map.snapshot,
                        layout
                            .position_map
                            .inlay_hint_glyph_for_position(window.mouse_position()),
                    )
                {
                    window.set_cursor_style(
                        CursorStyle::PointingHand,
                        &layout.position_map.text_hitbox,
                    );
                } else {
                    window.set_cursor_style(CursorStyle::IBeam, &layout.position_map.text_hitbox);
                };

                self.paint_lines_background(layout, window, cx);
                let invisible_display_ranges = self.paint_highlights(layout, window, cx);
                self.paint_document_colors(layout, window);
                self.paint_lines(&invisible_display_ranges, layout, window, cx);
                self.paint_redactions(layout, window);
                self.paint_navigation_overlays(layout, window, cx);
                self.paint_cursors(layout, window, cx);
                self.paint_inline_diagnostics(layout, window, cx);
                self.paint_inline_blame(layout, window, cx);
                self.paint_inline_code_actions(layout, window, cx);
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
        cx: &mut App,
    ) -> SmallVec<[Range<DisplayPoint>; 32]> {
        window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
            let mut invisible_display_ranges = SmallVec::<[Range<DisplayPoint>; 32]>::new();
            let line_end_overshoot = 0.15 * layout.position_map.line_height;
            for (range, color) in &layout.highlighted_ranges {
                self.paint_highlighted_range(
                    range.clone(),
                    true,
                    *color,
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    window,
                );
            }

            let corner_radius = if EditorSettings::get_global(cx).rounded_selection {
                0.15 * layout.position_map.line_height
            } else {
                Pixels::ZERO
            };

            for (player_color, selections) in &layout.selections {
                for selection in selections.iter() {
                    self.paint_highlighted_range(
                        selection.range.clone(),
                        true,
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
                    true,
                    redaction_color.into(),
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    window,
                );
            }
        });
    }

    fn paint_navigation_overlays(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_element_namespace("navigation_overlays", |window| {
            for command in &mut layout.navigation_overlay_paint_commands {
                let NavigationOverlayPaintCommand::Label(label) = command;
                label.element.paint(window, cx);
            }
        });
    }

    fn paint_document_colors(&self, layout: &mut EditorLayout, window: &mut Window) {
        let Some((colors_render_mode, image_colors)) = &layout.document_colors else {
            return;
        };
        if image_colors.is_empty()
            || colors_render_mode == &DocumentColorsRenderMode::None
            || colors_render_mode == &DocumentColorsRenderMode::Inlay
        {
            return;
        }

        let line_end_overshoot = layout.line_end_overshoot();

        for (range, color) in image_colors {
            match colors_render_mode {
                DocumentColorsRenderMode::Inlay | DocumentColorsRenderMode::None => return,
                DocumentColorsRenderMode::Background => {
                    self.paint_highlighted_range(
                        range.clone(),
                        true,
                        *color,
                        Pixels::ZERO,
                        line_end_overshoot,
                        layout,
                        window,
                    );
                }
                DocumentColorsRenderMode::Border => {
                    self.paint_highlighted_range(
                        range.clone(),
                        false,
                        *color,
                        Pixels::ZERO,
                        line_end_overshoot,
                        layout,
                        window,
                    );
                }
            }
        }
    }

    fn paint_cursors(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        for cursor in &mut layout.visible_cursors {
            cursor.paint(layout.content_origin, window, cx);
        }
    }

    fn paint_scrollbars(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        let Some(scrollbars_layout) = layout.scrollbars_layout.take() else {
            return;
        };
        let any_scrollbar_dragged = self.editor.read(cx).scroll_manager.any_scrollbar_dragged();

        for (scrollbar_layout, axis) in scrollbars_layout.iter_scrollbars() {
            let hitbox = &scrollbar_layout.hitbox;
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
                            self.collect_fast_scrollbar_markers(layout, scrollbar_layout, cx);
                        // Refresh slow scrollbar markers in the background. Below, we
                        // paint whatever markers have already been computed.
                        self.refresh_slow_scrollbar_markers(layout, scrollbar_layout, window, cx);

                        let markers = self.editor.read(cx).scrollbar_marker_state.markers.clone();
                        for marker in markers.iter().chain(&fast_markers) {
                            let mut marker = marker.clone();
                            marker.bounds.origin += hitbox.origin;
                            window.paint_quad(marker);
                        }
                    }

                    if let Some(thumb_bounds) = scrollbar_layout.thumb_bounds {
                        let scrollbar_thumb_color = match scrollbar_layout.thumb_state {
                            ScrollbarThumbState::Dragging => {
                                cx.theme().colors().scrollbar_thumb_active_background
                            }
                            ScrollbarThumbState::Hovered => {
                                cx.theme().colors().scrollbar_thumb_hover_background
                            }
                            ScrollbarThumbState::Idle => {
                                cx.theme().colors().scrollbar_thumb_background
                            }
                        };
                        window.paint_quad(quad(
                            thumb_bounds,
                            Corners::default(),
                            scrollbar_thumb_color,
                            scrollbar_edges,
                            cx.theme().colors().scrollbar_thumb_border,
                            BorderStyle::Solid,
                        ));

                        if any_scrollbar_dragged {
                            window.set_window_cursor_style(CursorStyle::Arrow);
                        } else {
                            window.set_cursor_style(CursorStyle::Arrow, hitbox);
                        }
                    }
                })
            }
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
                                (p + ScrollOffset::from(
                                    (new_position - old_position) / *text_unit_size,
                                ))
                                .max(0.)
                            });
                            editor.set_scroll_position(position, window, cx);
                        }

                        editor.scroll_manager.show_scrollbars(window, cx);
                        cx.stop_propagation();
                    } else if let Some((layout, axis)) = scrollbars_layout
                        .get_hovered_axis(window)
                        .filter(|_| !event.dragging())
                    {
                        if layout.thumb_hovered(&event.position) {
                            editor
                                .scroll_manager
                                .set_hovered_scroll_thumb_axis(axis, cx);
                        } else {
                            editor.scroll_manager.reset_scrollbar_state(cx);
                        }

                        editor.scroll_manager.show_scrollbars(window, cx);
                    } else {
                        editor.scroll_manager.reset_scrollbar_state(cx);
                    }

                    mouse_position = event.position;
                })
            }
        });

        if any_scrollbar_dragged {
            window.on_mouse_event({
                let editor = self.editor.clone();
                move |_: &MouseUpEvent, phase, window, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        if let Some((_, axis)) = scrollbars_layout.get_hovered_axis(window) {
                            editor
                                .scroll_manager
                                .set_hovered_scroll_thumb_axis(axis, cx);
                        } else {
                            editor.scroll_manager.reset_scrollbar_state(cx);
                        }
                        cx.stop_propagation();
                    });
                }
            });
        } else {
            window.on_mouse_event({
                let editor = self.editor.clone();

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
                        thumb_bounds,
                        ..
                    } = scrollbar_layout;

                    let Some(thumb_bounds) = thumb_bounds else {
                        return;
                    };

                    editor.update(cx, |editor, cx| {
                        editor
                            .scroll_manager
                            .set_dragged_scroll_thumb_axis(axis, cx);

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
                                .apply_along(axis, |_| start_position as ScrollOffset);

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
            let is_singleton = editor.buffer_kind(cx) == ItemBufferKind::Singleton;
            let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
            let show_git_diff_markers = scrollbar_settings.git_diff
                && (is_singleton || editor.allow_git_diff_scrollbar_markers);
            if !is_singleton && !show_git_diff_markers {
                editor.scrollbar_marker_state.dirty = true;
                editor.scrollbar_marker_state.markers = Default::default();
                editor.scrollbar_marker_state.pending_refresh = None;
                return;
            }
            if !editor
                .scrollbar_marker_state
                .should_refresh(scrollbar_layout.hitbox.size)
            {
                return;
            }

            let scrollbar_layout = scrollbar_layout.clone();
            let background_highlights = editor.background_highlights.clone();
            let snapshot = layout.position_map.snapshot.clone();
            let theme = cx.theme().clone();

            editor.scrollbar_marker_state.dirty = false;
            editor.scrollbar_marker_state.pending_refresh =
                Some(cx.spawn_in(window, async move |editor, cx| {
                    let scrollbar_size = scrollbar_layout.hitbox.size;
                    let scrollbar_markers = cx
                        .background_spawn(async move {
                            let max_point = snapshot.display_snapshot.buffer_snapshot().max_point();
                            let mut marker_quads = Vec::new();
                            if show_git_diff_markers {
                                let marker_row_ranges =
                                    snapshot.buffer_snapshot().diff_hunks().map(|hunk| {
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
                                background_highlights.iter().filter(|_| is_singleton)
                            {
                                let is_search_highlights = *background_highlight_id
                                    == HighlightKey::BufferSearchHighlights;
                                let is_text_highlights =
                                    *background_highlight_id == HighlightKey::SelectedTextHighlight;
                                let is_symbol_occurrences = *background_highlight_id
                                    == HighlightKey::DocumentHighlightRead
                                    || *background_highlight_id
                                        == HighlightKey::DocumentHighlightWrite;
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

                            if is_singleton
                                && scrollbar_settings.diagnostics != ScrollbarDiagnostics::None
                            {
                                let diagnostics = snapshot
                                    .buffer_snapshot()
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
                                                lsp::DiagnosticSeverity::ERROR,
                                            ) => true,
                                            (
                                                ScrollbarDiagnostics::Warning,
                                                lsp::DiagnosticSeverity::ERROR
                                                | lsp::DiagnosticSeverity::WARNING,
                                            ) => true,
                                            (
                                                ScrollbarDiagnostics::Information,
                                                lsp::DiagnosticSeverity::ERROR
                                                | lsp::DiagnosticSeverity::WARNING
                                                | lsp::DiagnosticSeverity::INFORMATION,
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
                                        lsp::DiagnosticSeverity::ERROR => theme.status().error,
                                        lsp::DiagnosticSeverity::WARNING => theme.status().warning,
                                        lsp::DiagnosticSeverity::INFORMATION => theme.status().info,
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
        fill: bool,
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
                    + Pixels::from(
                        (row_range.start.as_f64() - layout.position_map.scroll_position.y)
                            * ScrollOffset::from(layout.position_map.line_height),
                    ),
                lines: row_range
                    .iter_rows()
                    .map(|row| {
                        let line_layout =
                            &layout.position_map.line_layouts[row.minus(start_row) as usize];
                        let alignment_offset =
                            line_layout.alignment_offset(layout.text_align, layout.content_width);
                        HighlightedRangeLine {
                            start_x: if row == range.start.row() {
                                layout.content_origin.x
                                    + Pixels::from(
                                        line_layout.x_for_index(range.start.column() as usize)
                                            + alignment_offset
                                            - layout.position_map.scroll_pixel_position.x,
                                    )
                            } else {
                                layout.content_origin.x
                                    + Pixels::from(
                                        alignment_offset
                                            - layout.position_map.scroll_pixel_position.x,
                                    )
                            },
                            end_x: if row == range.end.row() {
                                layout.content_origin.x
                                    + Pixels::from(
                                        line_layout.x_for_index(range.end.column() as usize)
                                            + alignment_offset
                                            - layout.position_map.scroll_pixel_position.x,
                                    )
                            } else {
                                Pixels::from(
                                    ScrollPixelOffset::from(
                                        layout.content_origin.x + line_end_overshoot,
                                    ) + alignment_offset
                                        + line_layout.width
                                        - layout.position_map.scroll_pixel_position.x,
                                )
                            },
                        }
                    })
                    .collect(),
            };

            highlighted_range.paint(fill, layout.position_map.text_hitbox.bounds, window);
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
        if let Some(mut blame_layout) = layout.inline_blame_layout.take() {
            window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
                blame_layout.element.paint(window, cx);
            })
        }
    }

    fn paint_inline_code_actions(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(mut inline_code_actions) = layout.inline_code_actions.take() {
            window.paint_layer(layout.position_map.text_hitbox.bounds, |window| {
                inline_code_actions.paint(window, cx);
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

    fn paint_minimap(&self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        if let Some(mut layout) = layout.minimap.take() {
            let minimap_hitbox = layout.thumb_layout.hitbox.clone();
            let dragging_minimap = self.editor.read(cx).scroll_manager.is_dragging_minimap();

            window.paint_layer(layout.thumb_layout.hitbox.bounds, |window| {
                window.with_element_namespace("minimap", |window| {
                    layout.minimap.paint(window, cx);
                    if let Some(thumb_bounds) = layout.thumb_layout.thumb_bounds {
                        let minimap_thumb_color = match layout.thumb_layout.thumb_state {
                            ScrollbarThumbState::Idle => {
                                cx.theme().colors().minimap_thumb_background
                            }
                            ScrollbarThumbState::Hovered => {
                                cx.theme().colors().minimap_thumb_hover_background
                            }
                            ScrollbarThumbState::Dragging => {
                                cx.theme().colors().minimap_thumb_active_background
                            }
                        };
                        let minimap_thumb_border = match layout.thumb_border_style {
                            MinimapThumbBorder::Full => Edges::all(ScrollbarLayout::BORDER_WIDTH),
                            MinimapThumbBorder::LeftOnly => Edges {
                                left: ScrollbarLayout::BORDER_WIDTH,
                                ..Default::default()
                            },
                            MinimapThumbBorder::LeftOpen => Edges {
                                right: ScrollbarLayout::BORDER_WIDTH,
                                top: ScrollbarLayout::BORDER_WIDTH,
                                bottom: ScrollbarLayout::BORDER_WIDTH,
                                ..Default::default()
                            },
                            MinimapThumbBorder::RightOpen => Edges {
                                left: ScrollbarLayout::BORDER_WIDTH,
                                top: ScrollbarLayout::BORDER_WIDTH,
                                bottom: ScrollbarLayout::BORDER_WIDTH,
                                ..Default::default()
                            },
                            MinimapThumbBorder::None => Default::default(),
                        };

                        window.paint_layer(minimap_hitbox.bounds, |window| {
                            window.paint_quad(quad(
                                thumb_bounds,
                                Corners::default(),
                                minimap_thumb_color,
                                minimap_thumb_border,
                                cx.theme().colors().minimap_thumb_border,
                                BorderStyle::Solid,
                            ));
                        });
                    }
                });
            });

            if dragging_minimap {
                window.set_window_cursor_style(CursorStyle::Arrow);
            } else {
                window.set_cursor_style(CursorStyle::Arrow, &minimap_hitbox);
            }

            let minimap_axis = ScrollbarAxis::Vertical;

            window.on_mouse_event({
                let editor = self.editor.clone();

                let minimap_hitbox = minimap_hitbox.clone();

                let mut mouse_position = window.mouse_position();

                move |event: &MouseMoveEvent, phase, window, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        if event.pressed_button == Some(MouseButton::Left)
                            && editor.scroll_manager.is_dragging_minimap()
                        {
                            let old_position = mouse_position.along(minimap_axis);
                            let new_position = event.position.along(minimap_axis);
                            if let Some(pixels_per_line) = layout.thumb_pixels_per_editor_line
                                && (minimap_hitbox.origin.along(minimap_axis)
                                    ..minimap_hitbox.bottom_right().along(minimap_axis))
                                    .contains(&old_position)
                            {
                                let position =
                                    editor.scroll_position(cx).apply_along(minimap_axis, |p| {
                                        (p + ScrollPixelOffset::from(
                                            (new_position - old_position) / pixels_per_line,
                                        ))
                                        .max(0.)
                                    });

                                editor.set_scroll_position(position, window, cx);
                            }
                            cx.stop_propagation();
                        } else if minimap_hitbox.is_hovered(window) {
                            editor.scroll_manager.set_is_hovering_minimap_thumb(
                                !event.dragging()
                                    && layout
                                        .thumb_layout
                                        .thumb_bounds
                                        .is_some_and(|bounds| bounds.contains(&event.position)),
                                cx,
                            );

                            // Stop hover events from propagating to the
                            // underlying editor if the minimap hitbox is hovered
                            if !event.dragging() {
                                cx.stop_propagation();
                            }
                        } else {
                            editor.scroll_manager.hide_minimap_thumb(cx);
                        }
                        mouse_position = event.position;
                    });
                }
            });

            if dragging_minimap {
                window.on_mouse_event({
                    let editor = self.editor.clone();
                    move |event: &MouseUpEvent, phase, window, cx| {
                        if phase == DispatchPhase::Capture {
                            return;
                        }

                        editor.update(cx, |editor, cx| {
                            if minimap_hitbox.is_hovered(window) {
                                editor.scroll_manager.set_is_hovering_minimap_thumb(
                                    layout
                                        .thumb_layout
                                        .thumb_bounds
                                        .is_some_and(|bounds| bounds.contains(&event.position)),
                                    cx,
                                );
                            } else {
                                editor.scroll_manager.hide_minimap_thumb(cx);
                            }
                            cx.stop_propagation();
                        });
                    }
                });
            } else {
                window.on_mouse_event({
                    let editor = self.editor.clone();

                    move |event: &MouseDownEvent, phase, window, cx| {
                        if phase == DispatchPhase::Capture || !minimap_hitbox.is_hovered(window) {
                            return;
                        }

                        let event_position = event.position;

                        let Some(thumb_bounds) = layout.thumb_layout.thumb_bounds else {
                            return;
                        };

                        editor.update(cx, |editor, cx| {
                            if !thumb_bounds.contains(&event_position) {
                                let click_position =
                                    event_position.relative_to(&minimap_hitbox.origin).y;

                                let top_position = (click_position
                                    - thumb_bounds.size.along(minimap_axis) / 2.0)
                                    .max(Pixels::ZERO);

                                let scroll_offset = (layout.minimap_scroll_top
                                    + ScrollPixelOffset::from(
                                        top_position / layout.minimap_line_height,
                                    ))
                                .min(layout.max_scroll_top);

                                let scroll_position = editor
                                    .scroll_position(cx)
                                    .apply_along(minimap_axis, |_| scroll_offset);
                                editor.set_scroll_position(scroll_position, window, cx);
                            }

                            editor.scroll_manager.set_is_dragging_minimap(cx);
                            cx.stop_propagation();
                        });
                    }
                });
            }
        }
    }

    fn paint_spacer_blocks(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        for mut block in layout.spacer_blocks.drain(..) {
            let mut bounds = layout.hitbox.bounds;
            bounds.origin.x += layout.gutter_hitbox.bounds.size.width;
            window.with_content_mask(Some(ContentMask { bounds }), |window| {
                block.element.paint(window, cx);
            })
        }
    }

    fn paint_non_spacer_blocks(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
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

    fn paint_edit_prediction_popover(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(edit_prediction_popover) = layout.edit_prediction_popover.as_mut() {
            edit_prediction_popover.paint(window, cx);
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

    fn shape_line_number(
        &self,
        text: SharedString,
        color: Hsla,
        window: &mut Window,
    ) -> ShapedLine {
        let run = TextRun {
            len: text.len(),
            font: self.style.text.font(),
            color,
            ..Default::default()
        };
        window.text_system().shape_line(
            text,
            self.style.text.font_size.to_pixels(window.rem_size()),
            &[run],
            None,
        )
    }

    fn diff_hunk_hollow(&self, status: DiffHunkStatus, cx: &mut App) -> bool {
        let unstaged = !self
            .editor
            .read(cx)
            .diff_hunk_renderer()
            .render_hunk_as_staged(&status, cx);
        let unstaged_hollow = matches!(
            ProjectSettings::get_global(cx).git.hunk_style,
            GitHunkStyleSetting::UnstagedHollow
        );

        unstaged == unstaged_hollow
    }

    #[cfg(debug_assertions)]
    fn layout_debug_ranges(
        selections: &mut Vec<(PlayerColor, Vec<SelectionLayout>)>,
        anchor_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        cx: &App,
    ) {
        let theme = cx.theme();
        text::debug::GlobalDebugRanges::with_locked(|debug_ranges| {
            if debug_ranges.ranges.is_empty() {
                return;
            }
            let buffer_snapshot = &display_snapshot.buffer_snapshot();
            for (excerpt_buffer_snapshot, buffer_range, _) in
                buffer_snapshot.range_to_buffer_ranges(anchor_range.start..anchor_range.end)
            {
                let buffer_range = excerpt_buffer_snapshot.anchor_after(buffer_range.start)
                    ..excerpt_buffer_snapshot.anchor_before(buffer_range.end);
                selections.extend(debug_ranges.ranges.iter().flat_map(|debug_range| {
                    debug_range.ranges.iter().filter_map(|range| {
                        let player_color = theme
                            .players()
                            .color_for_participant(debug_range.occurrence_index as u32 + 1);
                        if range.start.buffer_id != excerpt_buffer_snapshot.remote_id() {
                            return None;
                        }
                        let clipped_start = range
                            .start
                            .max(&buffer_range.start, &excerpt_buffer_snapshot);
                        let clipped_end =
                            range.end.min(&buffer_range.end, &excerpt_buffer_snapshot);
                        let range = buffer_snapshot
                            .buffer_anchor_range_to_anchor_range(*clipped_start..*clipped_end)?;
                        let start = range.start.to_display_point(display_snapshot);
                        let end = range.end.to_display_point(display_snapshot);
                        let selection_layout = SelectionLayout {
                            id: 0,
                            head: start,
                            range: start..end,
                            cursor_shape: CursorShape::Bar,
                            is_newest: false,
                            is_local: false,
                            active_rows: start.row()..end.row(),
                            user_name: Some(SharedString::from(debug_range.value.clone())),
                        };
                        Some((player_color, vec![selection_layout]))
                    })
                }));
            }
        });
    }
}

struct Gutter<'a> {
    line_height: Pixels,
    range: Range<DisplayRow>,
    scroll_position: gpui::Point<ScrollOffset>,
    dimensions: &'a GutterDimensions,
    hitbox: &'a Hitbox,
    snapshot: &'a EditorSnapshot,
    row_infos: &'a [RowInfo],
}

impl Gutter<'_> {
    fn layout_item_skipping_folds(
        &self,
        display_row: DisplayRow,
        render_item: impl Fn(&mut Context<'_, Editor>, &mut Window) -> AnyElement,
        window: &mut Window,
        cx: &mut Context<'_, Editor>,
    ) -> Option<AnyElement> {
        let row = MultiBufferRow(
            DisplayPoint::new(display_row, 0)
                .to_point(self.snapshot)
                .row,
        );
        if self.snapshot.is_line_folded(row) {
            return None;
        }

        self.layout_item(display_row, render_item, window, cx)
    }

    fn layout_item(
        &self,
        display_row: DisplayRow,
        render_item: impl Fn(&mut Context<'_, Editor>, &mut Window) -> AnyElement,
        window: &mut Window,
        cx: &mut Context<'_, Editor>,
    ) -> Option<AnyElement> {
        if !self.range.contains(&display_row) {
            return None;
        }

        if self
            .row_infos
            .get((display_row.0.saturating_sub(self.range.start.0)) as usize)
            .is_some_and(|row_info| {
                row_info.expand_info.is_some()
                    || row_info
                        .diff_status
                        .is_some_and(|status| status.is_deleted())
            })
        {
            return None;
        }

        let button = self.prepaint_button(render_item(cx, window), display_row, window, cx);
        Some(button)
    }

    fn prepaint_button(
        &self,
        mut button: AnyElement,
        row: DisplayRow,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let available_space = size(
            AvailableSpace::MinContent,
            AvailableSpace::Definite(self.line_height),
        );
        let indicator_size = button.layout_as_root(available_space, window, cx);
        let git_gutter_width = EditorElement::gutter_strip_width(self.line_height, cx)
            + self.dimensions.git_blame_entries_width.unwrap_or_default();

        let x = git_gutter_width + px(2.);

        let mut y = Pixels::from(
            (row.as_f64() - self.scroll_position.y) * ScrollPixelOffset::from(self.line_height),
        );
        y += (self.line_height - indicator_size.height) / 2.;

        button.prepaint_as_root(
            self.hitbox.origin + point(x, y),
            available_space,
            window,
            cx,
        );
        button
    }
}

pub fn render_breadcrumb_text(
    mut segments: Vec<HighlightedText>,
    breadcrumb_font: Option<Font>,
    prefix: Option<gpui::AnyElement>,
    active_item: &dyn ItemHandle,
    multibuffer_header: bool,
    window: &mut Window,
    cx: &App,
) -> gpui::AnyElement {
    const MAX_SEGMENTS: usize = 12;

    let element = h_flex().flex_grow_1().text_ui(cx);

    let prefix_end_ix = cmp::min(segments.len(), MAX_SEGMENTS / 2);
    let suffix_start_ix = cmp::max(
        prefix_end_ix,
        segments.len().saturating_sub(MAX_SEGMENTS / 2),
    );

    if suffix_start_ix > prefix_end_ix {
        segments.splice(
            prefix_end_ix..suffix_start_ix,
            Some(HighlightedText {
                text: "⋯".into(),
                highlights: vec![],
            }),
        );
    }

    let highlighted_segments = segments.into_iter().enumerate().map(|(index, segment)| {
        let mut text_style = window.text_style();
        if let Some(font) = &breadcrumb_font {
            text_style.font_family = font.family.clone();
            text_style.font_features = font.features.clone();
            text_style.font_style = font.style;
            text_style.font_weight = font.weight;
        }
        text_style.color = Color::Muted.color(cx);

        if index == 0
            && !workspace::TabBarSettings::get_global(cx).show
            && active_item.is_dirty(cx)
            && let Some(styled_element) = apply_dirty_filename_style(&segment, &text_style, cx)
        {
            return styled_element;
        }

        StyledText::new(segment.text.replace('\n', " "))
            .with_default_highlights(&text_style, segment.highlights)
            .into_any()
    });

    let breadcrumbs = Itertools::intersperse_with(highlighted_segments, || {
        Label::new("›").color(Color::Placeholder).into_any_element()
    });

    let breadcrumbs_stack = h_flex()
        .gap_1()
        .when(multibuffer_header, |this| {
            this.pl_2()
                .border_l_1()
                .border_color(cx.theme().colors().border.opacity(0.6))
        })
        .children(breadcrumbs);

    let breadcrumbs = if let Some(prefix) = prefix {
        h_flex().gap_1p5().child(prefix).child(breadcrumbs_stack)
    } else {
        breadcrumbs_stack
    };

    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    let has_project_path = active_item.project_path(cx).is_some();

    match editor {
        Some(editor) => element
            .id("breadcrumb_container")
            .when(!multibuffer_header, |this| this.overflow_x_scroll())
            .child(
                ButtonLike::new("toggle outline view")
                    .child(breadcrumbs)
                    .when(multibuffer_header, |this| {
                        this.style(ButtonStyle::Transparent)
                    })
                    .when(!multibuffer_header, |this| {
                        let focus_handle = editor.upgrade().unwrap().focus_handle(&cx);

                        this.tooltip(Tooltip::element(move |_window, cx| {
                            v_flex()
                                .gap_1()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .justify_between()
                                        .child(Label::new("Show Symbol Outline"))
                                        .child(ui::KeyBinding::for_action_in(
                                            &zed_actions::outline::ToggleOutline,
                                            &focus_handle,
                                            cx,
                                        )),
                                )
                                .when(has_project_path, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_1()
                                            .justify_between()
                                            .pt_1()
                                            .border_t_1()
                                            .border_color(cx.theme().colors().border_variant)
                                            .child(Label::new("Right-Click to Copy Path")),
                                    )
                                })
                                .into_any_element()
                        }))
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
                                if let Some((editor, callback)) = editor
                                    .upgrade()
                                    .zip(zed_actions::outline::TOGGLE_OUTLINE.get())
                                {
                                    callback(editor.to_any_view(), window, cx);
                                }
                            }
                        })
                        .when(has_project_path, |this| {
                            this.on_right_click({
                                let editor = editor.clone();
                                move |_, _, cx| {
                                    if let Some(abs_path) = editor.upgrade().and_then(|editor| {
                                        editor.update(cx, |editor, cx| {
                                            editor.target_file_abs_path(cx)
                                        })
                                    }) {
                                        if let Some(path_str) = abs_path.to_str() {
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                path_str.to_string(),
                                            ));
                                        }
                                    }
                                }
                            })
                        })
                    }),
            )
            .into_any_element(),
        None => element
            .h(rems_from_px(22_f32)) // Match the height and padding of the `ButtonLike` in the other arm.
            .pl_1()
            .child(breadcrumbs)
            .into_any_element(),
    }
}

fn apply_dirty_filename_style(
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
    cx: &App,
) -> Option<gpui::AnyElement> {
    let text = segment.text.replace('\n', " ");

    let filename_position = std::path::Path::new(segment.text.as_ref())
        .file_name()
        .and_then(|f| {
            let filename_str = f.to_string_lossy();
            segment.text.rfind(filename_str.as_ref())
        })?;

    let bold_weight = FontWeight::BOLD;
    let default_color = Color::Default.color(cx);

    if filename_position == 0 {
        let mut filename_style = text_style.clone();
        filename_style.font_weight = bold_weight;
        filename_style.color = default_color;

        return Some(
            StyledText::new(text)
                .with_default_highlights(&filename_style, [])
                .into_any(),
        );
    }

    let highlight_style = gpui::HighlightStyle {
        font_weight: Some(bold_weight),
        color: Some(default_color),
        ..Default::default()
    };

    let highlight = vec![(filename_position..text.len(), highlight_style)];
    Some(
        StyledText::new(text)
            .with_default_highlights(text_style, highlight)
            .into_any(),
    )
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
    buffer: BufferId,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    if markdown.read(cx).is_parsing() {
        return None;
    }

    let renderer = cx.global::<GlobalBlameRenderer>().0.clone();
    let blame = blame.read(cx);
    let repository = blame.repository(cx, buffer)?;
    let tag_names = blame.tag_names_for_entry(buffer, &blame_entry);
    renderer.render_blame_entry_popover(
        blame_entry,
        scroll_handle,
        commit_message,
        tag_names,
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
    last_used_color: &mut Option<(Hsla, Oid)>,
    editor: Entity<Editor>,
    workspace: Entity<Workspace>,
    buffer: BufferId,
    renderer: &dyn BlameRenderer,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    let index: u32 = blame_entry.sha.into();
    let mut sha_color = cx.theme().players().color_for_participant(index).cursor;

    // If the last color we used is the same as the one we get for this line, but
    // the commit SHAs are different, then we try again to get a different color.
    if let Some((color, sha)) = *last_used_color
        && sha != blame_entry.sha
        && color == sha_color
    {
        sha_color = cx.theme().players().color_for_participant(index + 1).cursor;
    }
    last_used_color.replace((sha_color, blame_entry.sha));

    let blame = blame.read(cx);
    let details = blame.details_for_entry(buffer, &blame_entry);
    let tag_names = blame.tag_names_for_entry(buffer, &blame_entry);
    let repository = blame.repository(cx, buffer)?;
    renderer.render_blame_entry(
        &style.text,
        blame_entry,
        details,
        tag_names,
        repository,
        workspace.downgrade(),
        editor,
        ix,
        sha_color,
        window,
        cx,
    )
}

#[derive(Debug, Default)]
pub(crate) struct LineWithInvisibles {
    fragments: SmallVec<[LineFragment; 1]>,
    invisibles: Vec<Invisible>,
    diagnostic_underline_severity_ranges: Vec<(Range<usize>, lsp::DiagnosticSeverity)>,
    point_diagnostics: Vec<PointDiagnostic>,
    len: usize,
    pub(crate) width: ScrollPixelOffset,
    font_size: Pixels,
    long_row: Option<LongRowWindow>,
}

struct LongRowWindow {
    row: LongRow,
    plan: Range<u32>,
    origin_x: ScrollPixelOffset,
    paint_x: Range<ScrollPixelOffset>,
    fragment_origins: SmallVec<[(usize, ScrollPixelOffset); 1]>,
}

impl LongRowWindow {
    fn contains_column(&self, column: usize) -> bool {
        self.plan.start as usize <= column && column <= self.plan.end as usize
    }

    fn x_for_index(&self, index: usize) -> ScrollPixelOffset {
        let Ok(column) = u32::try_from(index) else {
            return self.row.width();
        };
        self.row.x_for_column(column)
    }

    fn index_for_x(&self, x: ScrollPixelOffset) -> Option<usize> {
        if x >= self.row.width() {
            return None;
        }
        Some(self.row.column_at_x(x) as usize)
    }
}

impl fmt::Debug for LongRowWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LongRowWindow")
            .field("plan", &self.plan)
            .finish()
    }
}

enum LineFragment {
    Text(ShapedLine),
    Element {
        id: ChunkRendererId,
        element: Option<AnyElement>,
        size: Size<Pixels>,
        len: usize,
    },
}

impl fmt::Debug for LineFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineFragment::Text(line) => f.debug_tuple("Text").field(line).finish(),
            LineFragment::Element { size, len, .. } => f
                .debug_struct("Element")
                .field("size", size)
                .field("len", len)
                .finish(),
        }
    }
}

impl LineFragment {
    fn len(&self) -> usize {
        match self {
            Self::Text(line) => line.len(),
            Self::Element { len, .. } => *len,
        }
    }

    fn width(&self) -> Pixels {
        match self {
            Self::Text(line) => line.width,
            Self::Element { size, .. } => size.width,
        }
    }
}

impl LineWithInvisibles {
    fn long_row(
        long_row: LongRow,
        row: DisplayRow,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        mut row_window: Range<u32>,
        editor_width: Pixels,
        content_width: Pixels,
        content_origin_x: Pixels,
        paint_x: Range<ScaledPixels>,
        scroll_x: ScrollPixelOffset,
        bg_segments: &[(Range<DisplayPoint>, Hsla)],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Self, PendingRowGeometry> {
        let prepared = snapshot
            .prepared_row_text(row)?
            .ok_or_else(|| snapshot.pending_row(row, true))?;
        let mut line = Self {
            width: long_row.width(),
            font_size: style.text.font_size.to_pixels(window.rem_size()),
            ..Self::default()
        };
        let origin_x = -line.origin_x(style.text.text_align, content_width, scroll_x);
        let mut fragment_origins = SmallVec::new();
        let metrics = long_row.metrics();
        let min_contrast = EditorSettings::get_global(cx).minimum_contrast_for_highlights;
        let paint_x = LongRow::paint_window_bounds(
            paint_x,
            f64::from(content_origin_x) - origin_x,
            window.scale_factor(),
        );
        for columns in long_row.paint_ranges(paint_x.clone()) {
            row_window.start = row_window.start.min(columns.start);
            row_window.end = row_window.end.max(columns.end);
            let mut start = columns.start;
            for piece in long_row
                .window_layout(columns.clone(), origin_x)
                .into_iter()
                .map(Some)
                .chain([None])
            {
                let end = piece
                    .as_ref()
                    .map_or(columns.end, |piece| piece.columns.start);
                if start < end {
                    let shift = |point: DisplayPoint| {
                        DisplayPoint::new(point.row(), point.column().saturating_sub(start))
                    };
                    let shifted_bg_segments =
                        bg_segments_in(bg_segments, start as usize..end as usize)
                            .iter()
                            .map(|(range, color)| (shift(range.start)..shift(range.end), *color))
                            .collect::<Vec<_>>();
                    let part = Self::from_chunks(
                        prepared.chunks(start as usize..end as usize),
                        snapshot,
                        row,
                        style,
                        usize::MAX,
                        1,
                        &snapshot.mode,
                        editor_width,
                        |_| false,
                        slice::from_ref(&shifted_bg_segments),
                        window,
                        cx,
                    )?
                    .pop()
                    .unwrap_or_default();
                    let mut offset = start;
                    for mut fragment in part.fragments {
                        if let LineFragment::Text(line) = &mut fragment {
                            let (_, origins) = prepared
                                .styles(offset as usize..offset as usize + line.len(), line)
                                .ok_or_else(|| snapshot.pending_row(row, true))?;
                            *line = mem::take(line)
                                .with_underline_origins(long_row.x_for_column(offset), origins);
                            apply_row_metrics(line, metrics);
                        }
                        fragment_origins.push((offset as usize, long_row.x_for_column(offset)));
                        offset += fragment.len() as u32;
                        line.fragments.push(fragment);
                    }
                    line.diagnostic_underline_severity_ranges.extend(
                        part.diagnostic_underline_severity_ranges.into_iter().map(
                            |(range, severity)| {
                                (
                                    range.start + start as usize..range.end + start as usize,
                                    severity,
                                )
                            },
                        ),
                    );
                }
                if let Some(piece) = piece {
                    let columns = piece.columns.start as usize..piece.columns.end as usize;
                    let (styles, origins) = prepared
                        .styles(columns.clone(), &piece.layout)
                        .ok_or_else(|| snapshot.pending_row(row, true))?;
                    let runs = decorate_layout(
                        &styles,
                        bg_segments,
                        min_contrast,
                        columns.start,
                        &piece.layout,
                    );
                    let mut shaped = ShapedLine::new(piece.layout, piece.text, &runs)
                        .with_underline_origins(piece.x, origins);
                    apply_row_metrics(&mut shaped, metrics);
                    fragment_origins.push((columns.start, piece.x));
                    line.fragments.push(LineFragment::Text(shaped));
                    if snapshot.mode.is_full() {
                        line.invisibles.extend(prepared.invisibles(columns.clone()));
                    }
                    line.diagnostic_underline_severity_ranges
                        .extend(prepared.diagnostics(columns));
                    start = piece.columns.end;
                }
            }
        }
        line.len = long_row.len() as usize;
        line.long_row = Some(LongRowWindow {
            row: long_row,
            plan: row_window,
            origin_x,
            paint_x,
            fragment_origins,
        });
        Ok(line)
    }

    fn long_row_prefix(
        row: DisplayRow,
        end: u32,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        editor_width: Pixels,
        bg_segments: &[(Range<DisplayPoint>, Hsla)],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Self, PendingRowGeometry> {
        let prepared = snapshot
            .prepared_row_text(row)?
            .ok_or_else(|| snapshot.pending_row(row, true))?;
        Ok(Self::from_chunks(
            prepared.chunks(0..end as usize),
            snapshot,
            row,
            style,
            MAX_LINE_LEN,
            1,
            &snapshot.mode,
            editor_width,
            |_| false,
            slice::from_ref(&bg_segments.to_vec()),
            window,
            cx,
        )?
        .pop()
        .unwrap_or_default())
    }

    fn long_row_needs_relayout(
        &self,
        scroll_x: ScrollPixelOffset,
        text_align: TextAlign,
        content_origin_x: Pixels,
        paint_bounds: Range<ScaledPixels>,
        content_width: Pixels,
        scale_factor: f32,
    ) -> bool {
        let Some(window) = self.long_row.as_ref() else {
            return false;
        };
        let origin_x = -self.origin_x(text_align, content_width, scroll_x);
        let paint_x = LongRow::paint_window_bounds(
            paint_bounds,
            f64::from(content_origin_x) - origin_x,
            scale_factor,
        );
        let plan =
            window
                .row
                .visible_window(paint_x.start, TextAlign::Left, paint_x.end - paint_x.start);
        let covered = window.origin_x == origin_x
            && window.plan.start <= plan.start
            && plan.end <= window.plan.end
            && window.paint_x.start <= paint_x.start
            && paint_x.end <= window.paint_x.end;
        !covered
    }

    fn fragment_positions(
        &self,
    ) -> impl Iterator<Item = (&LineFragment, usize, ScrollPixelOffset)> + '_ {
        let mut index = 0;
        let mut x = 0.;
        self.fragments
            .iter()
            .enumerate()
            .map(move |(fragment_index, fragment)| {
                let (start, fragment_x) = self
                    .long_row
                    .as_ref()
                    .and_then(|window| window.fragment_origins.get(fragment_index).copied())
                    .unwrap_or((index, x));
                index = start + fragment.len();
                x = fragment_x + ScrollPixelOffset::from(fragment.width());
                (fragment, start, fragment_x)
            })
    }

    fn from_chunks<'a>(
        chunks: impl Iterator<Item = impl Into<LineChunk<'a>>>,
        snapshot: &DisplaySnapshot,
        start_row: DisplayRow,
        editor_style: &EditorStyle,
        max_line_len: usize,
        max_line_count: usize,
        editor_mode: &EditorMode,
        text_width: Pixels,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        bg_segments_per_row: &[Vec<(Range<DisplayPoint>, Hsla)>],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Vec<Self>, PendingRowGeometry> {
        let text_style = &editor_style.text;
        let mut layouts = Vec::with_capacity(max_line_count);
        let mut fragments: SmallVec<[LineFragment; 1]> = SmallVec::new();
        let mut line = String::new();
        // Byte offset into the logical line used to position invisible markers.
        // Unlike `line`, this is not cleared when we flush `shape_line` for
        // mid-line inlays/replacements, so marker offsets stay correct in that case.
        let mut line_byte_offset: usize = 0;
        let mut invisibles = Vec::new();
        let mut diagnostic_underline_severity_ranges = Vec::new();
        let mut width = Pixels::ZERO;
        let mut len = 0;
        let mut styles = Vec::new();
        let mut non_whitespace_added = false;
        let mut row = 0;
        let mut line_exceeded_max_len = false;
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let min_contrast = EditorSettings::get_global(cx).minimum_contrast_for_highlights;

        for chunk in chunks
            .map(Into::into)
            .chain([LineChunk::from(HighlightedChunk {
                text: "\n",
                style: None,
                diagnostic_underline_severity: None,
                is_tab: false,
                is_inlay: false,
                replacement: None,
            })])
        {
            match chunk {
                LineChunk::Replacement {
                    len: chunk_len,
                    replacement,
                    style: chunk_style,
                    diagnostic_underline_severity,
                } => {
                    if line_exceeded_max_len {
                        continue;
                    }

                    if len + line.len() + chunk_len > max_line_len {
                        line_exceeded_max_len = true;
                        continue;
                    }

                    if !line.is_empty() {
                        let segments = bg_segments_per_row.get(row).map(|v| &v[..]).unwrap_or(&[]);
                        let shaped_line = shape_text_fragment(
                            &line,
                            len,
                            &styles,
                            segments,
                            min_contrast,
                            font_size,
                            window,
                        );
                        width += shaped_line.width;
                        len += shaped_line.len;
                        fragments.push(LineFragment::Text(shaped_line));
                        line.clear();
                        styles.clear();
                    }

                    match replacement {
                        ChunkReplacement::Renderer(renderer) => {
                            if let Some(LineFragment::Element {
                                id,
                                len: element_len,
                                ..
                            }) = fragments.last_mut()
                                && *id == renderer.id
                            {
                                *element_len += chunk_len;
                                len += chunk_len;
                                line_byte_offset += chunk_len;
                                continue;
                            }
                            let (element, size) = layout_chunk_renderer(
                                &renderer,
                                snapshot,
                                start_row + DisplayRow(row as u32),
                                text_style,
                                text_width,
                                window,
                                cx,
                            )?;

                            width += size.width;
                            len += chunk_len;
                            line_byte_offset += chunk_len;
                            fragments.push(LineFragment::Element {
                                id: renderer.id,
                                element: Some(element),
                                size,
                                len: chunk_len,
                            });
                        }
                        ChunkReplacement::Str(x) => {
                            let text_style = if let Some(style) = chunk_style {
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
                                .shape_line(x, font_size, &[run], None)
                                .with_len(chunk_len);

                            if let Some(severity) = diagnostic_underline_severity {
                                diagnostic_underline_severity_ranges.push((
                                    line_byte_offset..line_byte_offset + chunk_len,
                                    severity,
                                ));
                            }
                            width += line_layout.width;
                            len += chunk_len;
                            line_byte_offset += chunk_len;
                            fragments.push(LineFragment::Text(line_layout))
                        }
                    }
                }
                LineChunk::Text(highlighted_chunk) => {
                    for (ix, mut line_chunk) in highlighted_chunk.text.split('\n').enumerate() {
                        if ix > 0 {
                            let segments =
                                bg_segments_per_row.get(row).map(|v| &v[..]).unwrap_or(&[]);
                            let shaped_line = shape_text_fragment(
                                &line,
                                len,
                                &styles,
                                segments,
                                min_contrast,
                                font_size,
                                window,
                            );
                            width += shaped_line.width;
                            len += shaped_line.len;
                            fragments.push(LineFragment::Text(shaped_line));
                            layouts.push(Self {
                                width: ScrollPixelOffset::from(mem::take(&mut width)),
                                len: mem::take(&mut len),
                                fragments: mem::take(&mut fragments),
                                invisibles: std::mem::take(&mut invisibles),
                                diagnostic_underline_severity_ranges: mem::take(
                                    &mut diagnostic_underline_severity_ranges,
                                ),
                                point_diagnostics: Vec::new(),
                                font_size,
                                long_row: None,
                            });

                            line.clear();
                            line_byte_offset = 0;
                            styles.clear();
                            row += 1;
                            line_exceeded_max_len = false;
                            non_whitespace_added = false;
                            if row == max_line_count {
                                return Ok(layouts);
                            }
                        }

                        if !line_chunk.is_empty() && !line_exceeded_max_len {
                            let text_style = if let Some(style) = highlighted_chunk.style {
                                Cow::Owned(text_style.clone().highlight(style))
                            } else {
                                Cow::Borrowed(text_style)
                            };

                            let current_line_len = len + line.len();
                            if current_line_len + line_chunk.len() > max_line_len {
                                let mut chunk_len = max_line_len - current_line_len;
                                while !line_chunk.is_char_boundary(chunk_len) {
                                    chunk_len -= 1;
                                }
                                line_chunk = &line_chunk[..chunk_len];
                                line_exceeded_max_len = true;
                            }

                            if line_chunk.is_empty() {
                                continue;
                            }

                            styles.push(TextRun {
                                len: line_chunk.len(),
                                font: text_style.font(),
                                color: text_style.color,
                                background_color: text_style.background_color,
                                underline: text_style.underline,
                                strikethrough: text_style.strikethrough,
                            });

                            if let Some(severity) = highlighted_chunk.diagnostic_underline_severity
                            {
                                diagnostic_underline_severity_ranges.push((
                                    line_byte_offset..line_byte_offset + line_chunk.len(),
                                    severity,
                                ));
                            }

                            if editor_mode.is_full() && !highlighted_chunk.is_inlay {
                                // Line wrap pads its contents with fake whitespaces,
                                // avoid printing them
                                let is_soft_wrapped = is_row_soft_wrapped(row);
                                if highlighted_chunk.is_tab {
                                    if non_whitespace_added || !is_soft_wrapped {
                                        invisibles.push(Invisible::Tab {
                                            line_start_offset: line_byte_offset,
                                            line_end_offset: line_byte_offset + line_chunk.len(),
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
                                                    line_start_offset: line_byte_offset + index,
                                                    line_end_offset: line_byte_offset
                                                        + index
                                                        + c.len_utf8(),
                                                })
                                            } else {
                                                None
                                            }
                                        },
                                    ))
                                }
                            }

                            line.push_str(line_chunk);
                            line_byte_offset += line_chunk.len();
                        }
                    }
                }
            }
        }

        Ok(layouts)
    }

    fn set_point_diagnostics(&mut self, mut point_diagnostics: Vec<PointDiagnostic>) {
        point_diagnostics.sort_by_key(|diagnostic| (diagnostic.column, diagnostic.severity));
        point_diagnostics.dedup_by_key(|diagnostic| diagnostic.column);
        self.point_diagnostics = point_diagnostics;
    }

    /// Takes text runs and non-overlapping left-to-right background ranges with color.
    /// Returns new text runs with adjusted contrast as per background ranges.
    fn split_runs_by_bg_segments(
        text_runs: &[TextRun],
        bg_segments: &[(Range<DisplayPoint>, Hsla)],
        min_contrast: f32,
        start_col_offset: usize,
    ) -> Vec<TextRun> {
        let mut output_runs: Vec<TextRun> = Vec::with_capacity(text_runs.len());
        let mut line_col = start_col_offset;
        let mut segment_ix = 0usize;

        for text_run in text_runs.iter() {
            let run_start_col = line_col;
            let run_end_col = run_start_col + text_run.len;
            while segment_ix < bg_segments.len()
                && (bg_segments[segment_ix].0.end.column() as usize) <= run_start_col
            {
                segment_ix += 1;
            }
            let mut cursor_col = run_start_col;
            let mut local_segment_ix = segment_ix;
            while local_segment_ix < bg_segments.len() {
                let (range, segment_color) = &bg_segments[local_segment_ix];
                let segment_start_col = range.start.column() as usize;
                let segment_end_col = range.end.column() as usize;
                if segment_start_col >= run_end_col {
                    break;
                }
                if segment_start_col > cursor_col {
                    let span_len = segment_start_col - cursor_col;
                    output_runs.push(TextRun {
                        len: span_len,
                        font: text_run.font.clone(),
                        color: text_run.color,
                        background_color: text_run.background_color,
                        underline: text_run.underline,
                        strikethrough: text_run.strikethrough,
                    });
                    cursor_col = segment_start_col;
                }
                let segment_slice_end_col = segment_end_col.min(run_end_col);
                if segment_slice_end_col > cursor_col {
                    let new_text_color =
                        ensure_minimum_contrast(text_run.color, *segment_color, min_contrast);
                    output_runs.push(TextRun {
                        len: segment_slice_end_col - cursor_col,
                        font: text_run.font.clone(),
                        color: new_text_color,
                        background_color: text_run.background_color,
                        underline: text_run.underline,
                        strikethrough: text_run.strikethrough,
                    });
                    cursor_col = segment_slice_end_col;
                }
                if segment_end_col >= run_end_col {
                    break;
                }
                local_segment_ix += 1;
            }
            if cursor_col < run_end_col {
                output_runs.push(TextRun {
                    len: run_end_col - cursor_col,
                    font: text_run.font.clone(),
                    color: text_run.color,
                    background_color: text_run.background_color,
                    underline: text_run.underline,
                    strikethrough: text_run.strikethrough,
                });
            }
            line_col = run_end_col;
            segment_ix = local_segment_ix;
        }
        output_runs
    }

    fn prepaint(
        &mut self,
        line_height: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        row: DisplayRow,
        content_origin: gpui::Point<Pixels>,
        text_align: TextAlign,
        content_width: Pixels,
        line_elements: &mut SmallVec<[AnyElement; 1]>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_y = f32::from(line_height) * Pixels::from(row.as_f64() - scroll_position.y);
        self.prepaint_with_custom_offset(
            line_height,
            scroll_pixel_position,
            content_origin,
            line_y,
            text_align,
            content_width,
            line_elements,
            window,
            cx,
        );
    }

    fn prepaint_with_custom_offset(
        &mut self,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        content_origin: gpui::Point<Pixels>,
        line_y: Pixels,
        text_align: TextAlign,
        content_width: Pixels,
        line_elements: &mut SmallVec<[AnyElement; 1]>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let origin_x = self.origin_x(text_align, content_width, scroll_pixel_position.x);
        let fragment_xs = self
            .fragment_positions()
            .map(|(_, _, x)| x)
            .collect::<SmallVec<[ScrollPixelOffset; 4]>>();
        for (fragment, fragment_x) in self.fragments.iter_mut().zip(fragment_xs) {
            if let LineFragment::Element { element, size, .. } = fragment {
                let mut element = element
                    .take()
                    .expect("you can't prepaint LineWithInvisibles twice");

                // Center the element vertically within the line.
                let mut element_origin =
                    content_origin + point(Pixels::from(origin_x + fragment_x), line_y);
                element_origin.y += (line_height - size.height) / 2.;
                element.prepaint_at(element_origin, window, cx);
                line_elements.push(element);
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
        self.draw_with_custom_offset(
            layout,
            row,
            content_origin,
            layout.position_map.line_height
                * (row.as_f64() - layout.position_map.scroll_position.y) as f32,
            whitespace_setting,
            selection_ranges,
            window,
            cx,
        );
    }

    fn draw_with_custom_offset(
        &self,
        layout: &EditorLayout,
        row: DisplayRow,
        content_origin: gpui::Point<Pixels>,
        line_y: Pixels,
        whitespace_setting: ShowWhitespaceSetting,
        selection_ranges: &[Range<DisplayPoint>],
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_height = layout.position_map.line_height;
        let origin_x = self.origin_x(
            layout.text_align,
            layout.content_width,
            layout.position_map.scroll_pixel_position.x,
        );
        let mut points = self
            .point_diagnostics
            .iter()
            .filter(|diagnostic| {
                let column = diagnostic.column as usize;
                column <= self.len
                    && self
                        .long_row
                        .as_ref()
                        .is_none_or(|window| window.contains_column(column))
            })
            .collect::<SmallVec<[_; 4]>>();
        points.sort_unstable_by_key(|diagnostic| diagnostic.column);
        let has_points = !points.is_empty();
        let mut points = points.into_iter().peekable();
        let mut underlines = SmallVec::<[DiagnosticUnderline; 4]>::new();
        let mut diagnostic_ranges = self.diagnostic_underline_severity_ranges.iter().peekable();
        let mut end_underline_offset = layout.point_diagnostic_underline_offset;
        let column_underline =
            |diagnostic: &PointDiagnostic, underline_offset: Pixels, window: &Window| {
                let x = origin_x + self.x_for_index(diagnostic.column as usize);
                DiagnosticUnderline::point(
                    diagnostic,
                    content_origin + point(Pixels::from(x), line_y + underline_offset),
                    layout.position_map.em_advance,
                    window,
                )
            };

        for (fragment, fragment_start, fragment_x) in self.fragment_positions() {
            while let Some(diagnostic) =
                points.next_if(|diagnostic| (diagnostic.column as usize) < fragment_start)
            {
                underlines.push(column_underline(diagnostic, end_underline_offset, window));
            }
            let fragment_origin =
                content_origin + point(Pixels::from(origin_x + fragment_x), line_y);
            let (fragment_len, fragment_width, line) = match fragment {
                LineFragment::Text(line) => {
                    if has_points {
                        let mut glyphs = line.runs.iter().flat_map(|run| &run.glyphs).peekable();
                        line.paint_with_underline_handler(
                            fragment_origin,
                            line_height,
                            TextAlign::Left,
                            None,
                            window,
                            cx,
                            |range, underline, window| {
                                let bounds = underline.bounds(window);
                                if bounds.is_empty() {
                                    return;
                                }
                                let source_range = if line.text.len() == line.len() {
                                    fragment_start + range.start..fragment_start + range.end
                                } else {
                                    fragment_start..fragment_start + line.len()
                                };
                                if source_range.is_empty() {
                                    underline.paint(window);
                                    return;
                                }
                                let mut start = source_range.start;
                                let mut span_start = bounds.left();
                                while start < source_range.end {
                                    while diagnostic_ranges
                                        .peek()
                                        .is_some_and(|(range, _)| range.end <= start)
                                    {
                                        diagnostic_ranges.next();
                                    }
                                    let (end, severity) = match diagnostic_ranges.peek().copied() {
                                        Some((range, severity)) if range.start <= start => {
                                            (range.end.min(source_range.end), Some(*severity))
                                        }
                                        Some((range, _)) => {
                                            (range.start.min(source_range.end), None)
                                        }
                                        None => (source_range.end, None),
                                    };
                                    let span_end = if end == source_range.end {
                                        bounds.right()
                                    } else {
                                        let index = end - fragment_start;
                                        while glyphs.peek().is_some_and(|glyph| glyph.index < index)
                                        {
                                            glyphs.next();
                                        }
                                        let x = fragment_origin.x
                                            + glyphs
                                                .peek()
                                                .map_or(line.width, |glyph| glyph.position.x);
                                        window
                                            .underline_bounds(
                                                point(x, underline.origin.y),
                                                Pixels::ZERO,
                                                &underline.style,
                                            )
                                            .left()
                                            .clamp(span_start, bounds.right())
                                    };
                                    if span_start < span_end {
                                        if let Some(severity) = severity {
                                            underlines.push(DiagnosticUnderline {
                                                underline,
                                                bounds,
                                                span: span_start..span_end,
                                                severity,
                                                is_point: false,
                                            });
                                        } else {
                                            underline.paint_with_exclusions(
                                                &[
                                                    bounds.left()..span_start,
                                                    span_end..bounds.right(),
                                                ],
                                                window,
                                            );
                                        }
                                    }
                                    start = end;
                                    span_start = span_end;
                                }
                            },
                        )
                        .log_err();
                    } else {
                        line.paint(
                            fragment_origin,
                            line_height,
                            TextAlign::Left,
                            None,
                            window,
                            cx,
                        )
                        .log_err();
                    }
                    (line.len(), line.width, Some(line))
                }
                LineFragment::Element { size, len, .. } => (*len, size.width, None),
            };
            let fragment_end = fragment_start + fragment_len;
            if has_points {
                let mut glyphs = line
                    .into_iter()
                    .flat_map(|line| &line.runs)
                    .flat_map(|run| &run.glyphs)
                    .peekable();
                let has_glyphs = glyphs.peek().is_some();
                let underline_offset = line
                    .filter(|_| has_glyphs)
                    .map_or(layout.point_diagnostic_underline_offset, |line| {
                        underline_y_offset(line_height, line.ascent, line.descent)
                    });
                if fragment_len > 0 || fragment_width != Pixels::ZERO || has_glyphs {
                    end_underline_offset = underline_offset;
                }
                while points
                    .peek()
                    .is_some_and(|diagnostic| (diagnostic.column as usize) < fragment_end)
                {
                    let Some(diagnostic) = points.next() else {
                        break;
                    };
                    let index = diagnostic.column as usize - fragment_start;
                    let x = if let Some(line) = line {
                        let index = if line.text.len() == line.len() {
                            index
                        } else {
                            0
                        };
                        while glyphs.peek().is_some_and(|glyph| glyph.index < index) {
                            glyphs.next();
                        }
                        glyphs.peek().map_or(line.width, |glyph| glyph.position.x)
                    } else {
                        Pixels::ZERO
                    };
                    underlines.push(DiagnosticUnderline::point(
                        diagnostic,
                        fragment_origin + point(x, underline_offset),
                        layout.position_map.em_advance,
                        window,
                    ));
                }
            }
        }
        for diagnostic in points {
            underlines.push(column_underline(diagnostic, end_underline_offset, window));
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
        DiagnosticUnderline::paint_all(&underlines, window);
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
        let line_y = line_height * (row.as_f64() - layout.position_map.scroll_position.y) as f32;

        let origin_x = self.origin_x(
            layout.text_align,
            layout.content_width,
            layout.position_map.scroll_pixel_position.x,
        );
        for (fragment, _, fragment_x) in self.fragment_positions() {
            if let LineFragment::Text(line) = fragment {
                line.paint_background(
                    content_origin + point(Pixels::from(origin_x + fragment_x), line_y),
                    line_height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .log_err();
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
        let origin_x = self.origin_x(
            layout.text_align,
            layout.content_width,
            layout.position_map.scroll_pixel_position.x,
        );
        let extract_whitespace_info = |invisible: &Invisible| {
            let (token_offset, token_end_offset, invisible_symbol) = match invisible {
                Invisible::Tab {
                    line_start_offset,
                    line_end_offset,
                } => (*line_start_offset, *line_end_offset, &layout.tab_invisible),
                Invisible::Whitespace {
                    line_start_offset,
                    line_end_offset,
                } => (
                    *line_start_offset,
                    *line_end_offset,
                    &layout.space_invisible,
                ),
            };

            let token_x = self.x_for_index(token_offset);
            // Center the marker inside the actual glyph's width so it lines up with
            // proportional fonts instead of assuming a monospace `em_width` cell.
            let glyph_width =
                Pixels::from(self.x_for_index(token_end_offset) - token_x).max(Pixels::ZERO);
            let invisible_offset = ScrollPixelOffset::from(
                (glyph_width - invisible_symbol.width).max(Pixels::ZERO) / 2.0,
            );
            let origin =
                content_origin + point(Pixels::from(origin_x + token_x + invisible_offset), line_y);

            (
                [token_offset, token_end_offset],
                Box::new(move |window: &mut Window, cx: &mut App| {
                    invisible_symbol
                        .paint(origin, line_height, TextAlign::Left, None, window, cx)
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

            ShowWhitespaceSetting::Trailing => match self
                .long_row
                .as_ref()
                .map(|long_row_window| long_row_window.row.trailing_whitespace_start())
            {
                Some(trailing_start) => invisible_iter
                    .filter(|([start, _], _)| *start >= trailing_start as usize)
                    .for_each(|(_, paint)| paint(window, cx)),
                None => {
                    let mut previous_start = self.len;
                    for ([start, end], paint) in invisible_iter.rev() {
                        if previous_start != end {
                            break;
                        }
                        previous_start = start;
                        paint(window, cx);
                    }
                }
            },

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

    pub fn x_for_index(&self, index: usize) -> ScrollPixelOffset {
        if let Some(window) = &self.long_row {
            return window.x_for_index(index);
        }
        for (fragment, fragment_start, fragment_x) in self.fragment_positions() {
            if index < fragment_start + fragment.len() {
                return match fragment {
                    LineFragment::Text(shaped_line) => {
                        fragment_x
                            + ScrollPixelOffset::from(
                                shaped_line.x_for_index(index - fragment_start),
                            )
                    }
                    LineFragment::Element { .. } => fragment_x,
                };
            }
        }
        self.width
    }

    pub fn index_for_x(&self, x: ScrollPixelOffset) -> Option<usize> {
        if let Some(window) = &self.long_row {
            return window.index_for_x(x);
        }
        for (fragment, fragment_start, fragment_x) in self.fragment_positions() {
            if x < fragment_x + ScrollPixelOffset::from(fragment.width()) {
                return Some(match fragment {
                    LineFragment::Text(shaped_line) => {
                        fragment_start
                            + shaped_line
                                .index_for_x(Pixels::from(x - fragment_x))
                                .unwrap_or(shaped_line.len)
                    }
                    LineFragment::Element { .. } => fragment_start,
                });
            }
        }
        None
    }

    pub fn font_id_for_index(&self, index: usize) -> Option<FontId> {
        if let Some(window) = &self.long_row {
            return window
                .row
                .font_id_for_column(index as u32, &(0..window.row.len()));
        }
        for (fragment, fragment_start, _) in self.fragment_positions() {
            if index < fragment_start + fragment.len() {
                return match fragment {
                    LineFragment::Text(shaped_line) => {
                        shaped_line.font_id_for_index(index.checked_sub(fragment_start)?)
                    }
                    LineFragment::Element { .. } => None,
                };
            }
        }
        None
    }

    fn text_metrics_for_index(&self, index: usize) -> Option<RowMetrics> {
        if let Some(window) = &self.long_row {
            return self.font_id_for_index(index).map(|_| window.row.metrics());
        }
        self.fragment_positions()
            .find(|(fragment, fragment_start, _)| index < fragment_start + fragment.len())
            .and_then(|(fragment, _, _)| match fragment {
                LineFragment::Text(shaped_line) => Some(RowMetrics {
                    ascent: shaped_line.ascent,
                    descent: shaped_line.descent,
                }),
                LineFragment::Element { .. } => None,
            })
    }

    pub fn alignment_offset(
        &self,
        text_align: TextAlign,
        content_width: Pixels,
    ) -> ScrollPixelOffset {
        Self::alignment_offset_for_width(self.width, text_align, content_width)
    }

    pub(crate) fn alignment_offset_for_width(
        width: ScrollPixelOffset,
        text_align: TextAlign,
        content_width: Pixels,
    ) -> ScrollPixelOffset {
        let free_width = ScrollPixelOffset::from(content_width) - width;
        match text_align {
            TextAlign::Left => 0.,
            TextAlign::Center => free_width / 2.,
            TextAlign::Right => free_width,
        }
    }

    fn origin_x(
        &self,
        text_align: TextAlign,
        content_width: Pixels,
        scroll_x: ScrollPixelOffset,
    ) -> ScrollPixelOffset {
        self.alignment_offset(text_align, content_width) - scroll_x
    }

    fn visible_anchor_ranges(
        &self,
        row: DisplayRow,
        snapshot: &EditorSnapshot,
    ) -> Vec<Range<Anchor>> {
        let buffer = snapshot.buffer_snapshot();
        let start = DisplayPoint::new(row, 0).to_offset(snapshot, Bias::Left);
        let end = DisplayPoint::new(row.next_row(), 0).to_offset(snapshot, Bias::Right);
        let long_rows = self
            .long_row
            .iter()
            .map(|window| (row, window.row.clone(), window.plan.clone()))
            .collect::<Vec<_>>();
        visible_anchor_ranges(
            snapshot,
            buffer.anchor_before(start)..buffer.anchor_after(end),
            &long_rows,
        )
    }
}

fn visible_anchor_ranges(
    snapshot: &EditorSnapshot,
    range: Range<Anchor>,
    long_rows: &[(DisplayRow, LongRow, Range<u32>)],
) -> Vec<Range<Anchor>> {
    if long_rows.is_empty() {
        return vec![range];
    }
    let buffer = snapshot.buffer_snapshot();
    let start = range.start.to_point(buffer);
    let end = range.end.to_point(buffer);
    snapshot
        .visible_buffer_ranges(start..end, long_rows)
        .into_iter()
        .map(|points| {
            let range_start = if points.start == start {
                range.start
            } else {
                buffer.anchor_before(points.start)
            };
            let range_end = if points.end == end {
                range.end
            } else {
                buffer.anchor_after(points.end)
            };
            range_start..range_end
        })
        .collect()
}

fn horizontal_scroll_anchor(
    snapshot: &EditorSnapshot,
    start_row: DisplayRow,
    line_layouts: &[LineWithInvisibles],
    newest_selection_head: Option<DisplayPoint>,
    scroll_left: ScrollPixelOffset,
) -> Option<(Anchor, ScrollPixelOffset)> {
    let left_edge = |index: usize| {
        let layout = line_layouts.get(index)?;
        layout.long_row.as_ref()?;
        let column = layout.index_for_x(scroll_left)?;
        let point = DisplayPoint::new(DisplayRow(start_row.0 + index as u32), column as u32);
        let anchor = snapshot
            .buffer_snapshot()
            .anchor_before(point.to_offset(snapshot, Bias::Left));
        let anchor_point = anchor.to_display_point(snapshot);
        let long_row = line_layouts
            .get(anchor_point.row().0.checked_sub(start_row.0)? as usize)?
            .long_row
            .as_ref()?
            .row
            .clone();
        let x = long_row.x_for_column(anchor_point.column().min(long_row.len()));
        Some((anchor, x - scroll_left))
    };
    newest_selection_head
        .and_then(|head| left_edge(head.row().0.checked_sub(start_row.0)? as usize))
        .or_else(|| (0..line_layouts.len()).find_map(left_edge))
}

fn clip_ranges_to_columns<T>(
    items: &mut Vec<T>,
    range: impl Fn(&mut T) -> &mut Range<DisplayPoint>,
    painted_columns: impl Fn(DisplayRow) -> Option<Range<u32>>,
) {
    items.retain_mut(|item| {
        let range = range(item);
        if let Some(columns) = painted_columns(range.start.row()) {
            if range.start.column() > columns.end {
                if range.start.row() == range.end.row() {
                    return false;
                }
                range.start = DisplayPoint::new(range.start.row().next_row(), 0);
            } else {
                *range.start.column_mut() = range.start.column().max(columns.start);
            }
        }
        if range.end.column() > 0
            && let Some(columns) = painted_columns(range.end.row())
        {
            if range.end.column() < columns.start {
                if range.start.row() == range.end.row() {
                    return false;
                }
                *range.end.column_mut() = 0;
            } else {
                *range.end.column_mut() = range.end.column().min(columns.end);
            }
        }
        range.start < range.end
    });
}

fn language_aware_styling(snapshot: &EditorSnapshot, row: DisplayRow) -> LanguageAwareStyling {
    snapshot
        .display_snapshot
        .language_aware_styling(row, &snapshot.syntax)
}

fn bg_segments_in(
    bg_segments: &[(Range<DisplayPoint>, Hsla)],
    columns: Range<usize>,
) -> &[(Range<DisplayPoint>, Hsla)] {
    let first =
        bg_segments.partition_point(|(range, _)| range.end.column() as usize <= columns.start);
    let len = bg_segments[first..]
        .partition_point(|(range, _)| (range.start.column() as usize) < columns.end);
    &bg_segments[first..first + len]
}

fn decorate_layout<'a>(
    styles: &'a [TextRun],
    bg_segments: &[(Range<DisplayPoint>, Hsla)],
    min_contrast: f32,
    offset: usize,
    layout: &LineLayout,
) -> Cow<'a, [TextRun]> {
    let bg_segments = bg_segments_in(bg_segments, offset..offset + layout.len);
    if bg_segments.is_empty() {
        Cow::Borrowed(styles)
    } else {
        let bg_segments = glyph_aligned_bg_segments(bg_segments, layout, offset);
        Cow::Owned(LineWithInvisibles::split_runs_by_bg_segments(
            styles,
            &bg_segments,
            min_contrast,
            offset,
        ))
    }
}

fn glyph_aligned_bg_segments(
    bg_segments: &[(Range<DisplayPoint>, Hsla)],
    line: &LineLayout,
    start_col_offset: usize,
) -> Vec<(Range<DisplayPoint>, Hsla)> {
    let mut glyph_starts = line
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.index))
        .collect::<Vec<_>>();
    glyph_starts.sort_unstable();
    let line_end = start_col_offset + line.len;
    let is_inside = |column: usize| start_col_offset < column && column < line_end;
    let snap_start = |column: usize| {
        if !is_inside(column) {
            return column;
        }
        let local = column - start_col_offset;
        let ix = glyph_starts.partition_point(|start| *start <= local);
        start_col_offset + ix.checked_sub(1).map_or(0, |ix| glyph_starts[ix])
    };
    let snap_end = |column: usize| {
        if !is_inside(column) {
            return column;
        }
        let local = column - start_col_offset;
        let ix = glyph_starts.partition_point(|start| *start < local);
        start_col_offset + glyph_starts.get(ix).copied().unwrap_or(line.len)
    };
    let mut previous_end = 0;
    let mut remaining = bg_segments;
    let mut aligned = Vec::new();
    while let Some(((range, color), rest)) = remaining.split_first() {
        let start = snap_start(range.start.column() as usize).max(previous_end);
        let end = snap_end(range.end.column() as usize);
        if start < end {
            previous_end = end;
            aligned.push((
                DisplayPoint::new(range.start.row(), start as u32)
                    ..DisplayPoint::new(range.end.row(), end as u32),
                *color,
            ));
        }
        remaining =
            &rest[rest.partition_point(|(range, _)| range.end.column() as usize <= previous_end)..];
    }
    aligned
}

fn shape_text_fragment(
    text: &str,
    offset: usize,
    styles: &[TextRun],
    bg_segments: &[(Range<DisplayPoint>, Hsla)],
    min_contrast: f32,
    font_size: Pixels,
    window: &mut Window,
) -> ShapedLine {
    let layout = window
        .text_system()
        .layout_line(text, font_size, styles, None);
    let runs = decorate_layout(styles, bg_segments, min_contrast, offset, &layout);
    ShapedLine::new(layout, SharedString::new(text), &runs)
}

fn layout_chunk_renderer(
    renderer: &ChunkRenderer,
    snapshot: &DisplaySnapshot,
    row: DisplayRow,
    text_style: &TextStyle,
    text_width: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> Result<(AnyElement, Size<Pixels>), PendingRowGeometry> {
    let available_width = if renderer.constrain_width {
        let width = snapshot
            .prepared_renderer_constraint(renderer.id)
            .filter(|constraint| {
                constraint.matches(
                    renderer,
                    constraint.text(),
                    &text_style.font(),
                    text_style.font_size.to_pixels(window.rem_size()),
                )
            })
            .and_then(|constraint| constraint.width())
            .ok_or_else(|| snapshot.pending_renderer_constraint(row, renderer.id))?;
        AvailableSpace::Definite(width)
    } else {
        AvailableSpace::MinContent
    };
    Ok(layout_chunk_renderer_with_width(
        renderer,
        available_width,
        text_style,
        text_width,
        window,
        cx,
    ))
}

fn layout_chunk_renderer_with_width(
    renderer: &ChunkRenderer,
    available_width: AvailableSpace,
    text_style: &TextStyle,
    text_width: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> (AnyElement, Size<Pixels>) {
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
    (element, size)
}

fn apply_row_metrics(shaped_line: &mut ShapedLine, metrics: RowMetrics) {
    if shaped_line.ascent == metrics.ascent && shaped_line.descent == metrics.descent {
        return;
    }
    let layout: &mut Arc<LineLayout> = shaped_line;
    if let Some(layout) = Arc::get_mut(layout) {
        layout.ascent = metrics.ascent;
        layout.descent = metrics.descent;
        return;
    }
    *layout = Arc::new(LineLayout {
        font_size: layout.font_size,
        width: layout.width,
        ascent: metrics.ascent,
        descent: metrics.descent,
        runs: layout.runs.clone(),
        len: layout.len,
        caret_positions: layout.caret_positions.clone(),
    });
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
    /// A whitespace character (ASCII space or any other Unicode whitespace).
    ///
    /// Storing both offsets correctly accounts for multi-byte whitespace characters
    /// such as U+00A0 NO-BREAK SPACE, keeping adjacency checks correct.
    Whitespace {
        line_start_offset: usize,
        line_end_offset: usize,
    },
}

impl Invisible {
    fn range(&self) -> Range<usize> {
        match *self {
            Self::Tab {
                line_start_offset,
                line_end_offset,
            }
            | Self::Whitespace {
                line_start_offset,
                line_end_offset,
            } => line_start_offset..line_end_offset,
        }
    }
}

impl EditorElement {
    /// Returns the rem size to use when rendering the [`EditorElement`].
    ///
    /// This allows UI elements to scale based on the `buffer_font_size`.
    fn rem_size(&self, cx: &mut App) -> Option<Pixels> {
        match self.editor.read(cx).mode {
            EditorMode::Full {
                scale_ui_elements_with_buffer_font_size: true,
                ..
            }
            | EditorMode::Minimap { .. } => {
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
            _ => None,
        }
    }

    fn editor_with_selections(&self, cx: &App) -> Option<Entity<Editor>> {
        if let EditorMode::Minimap { parent } = self.editor.read(cx).mode() {
            parent.upgrade()
        } else {
            Some(self.editor.clone())
        }
    }
}

#[derive(Default)]
pub struct EditorRequestLayoutState {
    // We use prepaint depth to limit the number of times prepaint is
    // called recursively. We need this so that we can update stale
    // data for e.g. block heights in block map.
    prepaint_depth: Rc<Cell<usize>>,
}

impl EditorRequestLayoutState {
    // In ideal conditions we only need one more subsequent prepaint call for resize to take effect.
    // i.e. MAX_PREPAINT_DEPTH = 2, but placing near blocks can expose more lines from below, and
    // we end up querying blocks for those lines too in subsequent renders.
    // Setting MAX_PREPAINT_DEPTH = 3, passes all tests. Just to be on the safe side we set it to 5, so
    // that subsequent shrinking does not lead to incorrect block placing.
    const MAX_PREPAINT_DEPTH: usize = 5;

    fn increment_prepaint_depth(&self) -> EditorPrepaintGuard {
        let depth = self.prepaint_depth.get();
        self.prepaint_depth.set(depth + 1);
        EditorPrepaintGuard {
            prepaint_depth: self.prepaint_depth.clone(),
        }
    }

    fn has_remaining_prepaint_depth(&self) -> bool {
        self.prepaint_depth.get() < Self::MAX_PREPAINT_DEPTH
    }
}

struct EditorPrepaintGuard {
    prepaint_depth: Rc<Cell<usize>>,
}

impl Drop for EditorPrepaintGuard {
    fn drop(&mut self) {
        let depth = self.prepaint_depth.get();
        self.prepaint_depth.set(depth.saturating_sub(1));
    }
}

impl Element for EditorElement {
    type RequestLayoutState = EditorRequestLayoutState;
    type PrepaintState = EditorPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let rem_size = self.rem_size(cx);
        window.with_rem_size(rem_size, |window| {
            self.editor.update(cx, |editor, cx| {
                editor.set_style(self.style.clone(), window, cx);

                let layout_id = match editor.mode {
                    EditorMode::SingleLine => {
                        let rem_size = window.rem_size();
                        let height = self.style.text.line_height_in_pixels(rem_size);
                        let mut style = Style::default();
                        style.size.height = height.into();
                        style.size.width = relative(1.).into();
                        window.request_layout(style, None, cx)
                    }
                    EditorMode::AutoHeight {
                        min_lines,
                        max_lines,
                    } => {
                        let editor_handle = cx.entity();
                        window.request_measured_layout(
                            Style::default(),
                            move |known_dimensions, available_space, window, cx| {
                                editor_handle
                                    .update(cx, |editor, cx| {
                                        compute_auto_height_layout(
                                            editor,
                                            min_lines,
                                            max_lines,
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
                    EditorMode::Minimap { .. } => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        style.size.height = relative(1.).into();
                        window.request_layout(style, None, cx)
                    }
                    EditorMode::Full {
                        sizing_behavior, ..
                    } => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        if sizing_behavior == SizingBehavior::SizeByContent {
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

                (layout_id, EditorRequestLayoutState::default())
            })
        })
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let _prepaint_depth_guard = request_layout.increment_prepaint_depth();
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };

        let is_minimap = self.editor.read(cx).mode.is_minimap();
        let is_singleton = self.editor.read(cx).buffer_kind(cx) == ItemBufferKind::Singleton;

        if !is_minimap {
            let focus_handle = self.editor.focus_handle(cx);
            window.set_view_id(self.editor.entity_id());
            window.set_focus_handle(&focus_handle, cx);
        }

        let rem_size = self.rem_size(cx);
        window.with_rem_size(rem_size, |window| {
            window.with_text_style(Some(text_style), |window| {
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    let (mut snapshot, is_read_only) = self.editor.update(cx, |editor, cx| {
                        let config = window.glyph_raster_config();
                        editor
                            .display_map
                            .update(cx, |map, cx| map.set_raster_config(config, cx));
                        if let Some(placeholder) = &editor.placeholder_display_map {
                            placeholder.update(cx, |map, cx| map.set_raster_config(config, cx));
                        }
                        (editor.snapshot(window, cx), editor.read_only(cx))
                    });

                    if snapshot.tab_snapshot().has_unrepresentable_rows() {
                        self.editor.update(cx, |editor, _| {
                            editor.cancel_unrepresentable_input();
                            editor.last_bounds = Some(bounds);
                            editor.last_position_map = None;
                            editor.last_input_viewport = None;
                            editor.last_right_margin = Pixels::ZERO;
                            editor.last_horizontal_scrollbar_visible = false;
                            editor.pixel_position_of_newest_cursor = None;
                            editor.gutter_dimensions = GutterDimensions::default();
                        });
                        let focus_handle = self.editor.focus_handle(cx);
                        let mut feedback = div()
                            .id(("editor-display-unavailable", self.editor.entity_id()))
                            .w(bounds.size.width)
                            .h(bounds.size.height)
                            .px_2()
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                focus_handle.focus(window, cx);
                            })
                            .child(
                                Label::new("Line too long to display — undo or reduce tab size.")
                                    .size(LabelSize::Small)
                                    .color(Color::Error)
                                    .truncate(),
                            )
                            .tooltip(Tooltip::text(
                                "A tab-expanded line exceeds 4,294,967,295 bytes.\nUndo the change or reduce the tab size to restore editing.",
                            ))
                            .into_any_element();
                        feedback.prepaint_as_root(
                            bounds.origin,
                            bounds.size.map(AvailableSpace::Definite),
                            window,
                            cx,
                        );
                        return EditorPrepaintState::Unavailable {
                            mode: snapshot.mode,
                            feedback,
                        };
                    }

                    let style = &self.style;

                    let rem_size = window.rem_size();
                    let font_id = window.text_system().resolve_font(&style.text.font());
                    let font_size = style.text.font_size.to_pixels(rem_size);
                    let line_height = style.text.line_height_in_pixels(rem_size);
                    let ascent = window.text_system().ascent(font_id, font_size);
                    let descent = window.text_system().descent(font_id, font_size).abs();
                    let point_diagnostic_underline_offset =
                        underline_y_offset(line_height, ascent, descent);
                    let em_width = window.text_system().em_width(font_id, font_size).unwrap();
                    let em_advance = window.text_system().em_advance(font_id, font_size).unwrap();
                    let em_layout_width = window.text_system().em_layout_width(font_id, font_size);
                    let glyph_grid_cell = size(em_layout_width, line_height);

                    let gutter_dimensions =
                        snapshot.gutter_dimensions(font_id, font_size, style, window, cx);
                    let text_width = bounds.size.width - gutter_dimensions.width;

                    let settings = EditorSettings::get_global(cx);
                    let scrollbars_shown = settings.scrollbar.show != ShowScrollbar::Never;
                    let vertical_scrollbar_width = (scrollbars_shown
                        && settings.scrollbar.axes.vertical
                        && self.editor.read(cx).show_scrollbars.vertical)
                        .then_some(style.scrollbar_width)
                        .unwrap_or_default();
                    let minimap_width = self
                        .get_minimap_width(
                            &settings.minimap,
                            scrollbars_shown,
                            text_width,
                            em_width,
                            font_size,
                            rem_size,
                            cx,
                        )
                        .unwrap_or_default();

                    let right_margin = minimap_width + vertical_scrollbar_width;

                    let extended_right = 2 * em_width + right_margin;
                    let editor_width = text_width - gutter_dimensions.margin - extended_right;
                    let editor_margins = EditorMargins {
                        gutter: gutter_dimensions,
                        right: right_margin,
                        extended_right,
                    };

                    snapshot = self.editor.update(cx, |editor, cx| {
                        editor.last_bounds = Some(bounds);
                        editor.gutter_dimensions = gutter_dimensions;
                        editor.set_visible_line_count(
                            (bounds.size.height / line_height) as f64,
                            window,
                            cx,
                        );
                        if editor.set_visible_column_count(f64::from(editor_width / em_advance)) {
                            editor.update_data_on_scroll(true, window, cx);
                        }

                        if matches!(
                            editor.mode,
                            EditorMode::AutoHeight { .. } | EditorMode::Minimap { .. }
                        ) {
                            snapshot
                        } else {
                            let wrap_width = calculate_wrap_width(
                                editor.soft_wrap_mode(cx),
                                editor.display_map.read(cx).masked,
                                editor_width,
                                em_layout_width,
                            );

                            if editor.set_wrap_width(wrap_width, cx) {
                                editor.snapshot(window, cx)
                            } else {
                                snapshot
                            }
                        }
                    });
                    if !is_minimap && request_layout.prepaint_depth.get() == 1 {
                        self.measure_unmeasured_renderers(&mut snapshot, editor_width, window, cx);
                    }

                    let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);
                    let gutter_hitbox = window.insert_hitbox(
                        gutter_bounds(bounds, gutter_dimensions),
                        HitboxBehavior::Normal,
                    );
                    let text_hitbox = window.insert_hitbox(
                        Bounds {
                            origin: gutter_hitbox.top_right(),
                            size: size(text_width, bounds.size.height),
                        },
                        HitboxBehavior::Normal,
                    );

                    // Offset the content_bounds from the text_bounds by the gutter margin (which
                    // is roughly half a character wide) to make hit testing work more like how we want.
                    let content_offset = point(editor_margins.gutter.margin, Pixels::ZERO);
                    let content_origin = text_hitbox.origin + content_offset;

                    let height_in_lines = f64::from(bounds.size.height / line_height);
                    let max_row = snapshot.max_point().row().as_f64();

                    // Calculate how much of the editor is clipped by parent containers (e.g., List).
                    // This allows us to only render lines that are actually visible, which is
                    // critical for performance when large content-sized editors are inside Lists.
                    let visible_bounds = window.content_mask().bounds;
                    let visible_top = bounds.top().max(visible_bounds.top());
                    let visible_bottom = bounds.bottom().min(visible_bounds.bottom());
                    let clipped_top = (visible_top - bounds.top()).max(px(0.));
                    let visible_height = (visible_bottom - visible_top).max(px(0.));
                    let clipped_top_in_lines = f64::from(clipped_top / line_height);
                    let visible_height_in_lines = f64::from(visible_height / line_height);

                    // The max scroll position for the top of the window
                    let scroll_beyond_last_line = self.editor.read(cx).scroll_beyond_last_line(cx);
                    let max_scroll_top = match scroll_beyond_last_line {
                        ScrollBeyondLastLine::OnePage => max_row,
                        ScrollBeyondLastLine::Off => (max_row - height_in_lines + 1.).max(0.),
                        ScrollBeyondLastLine::VerticalScrollMargin => {
                            let settings = EditorSettings::get_global(cx);
                            (max_row - height_in_lines + 1. + settings.vertical_scroll_margin)
                                .max(0.)
                        }
                    };

                    let (
                        autoscroll_request,
                        autoscroll_containing_element,
                        needs_horizontal_autoscroll,
                    ) = self.editor.update(cx, |editor, cx| {
                        let autoscroll_request = editor.scroll_manager.take_autoscroll_request();

                        if !is_minimap
                            && autoscroll_request.is_none()
                            && editor.scroll_manager.resolve_horizontal_anchor(
                                &snapshot,
                                em_layout_width,
                                cx,
                            )
                        {
                            snapshot = editor.snapshot(window, cx);
                        }

                        let autoscroll_containing_element =
                            autoscroll_request.is_some() || editor.has_pending_selection();

                        let (needs_horizontal_autoscroll, was_scrolled) = editor
                            .autoscroll_vertically(
                                bounds,
                                line_height,
                                max_scroll_top,
                                autoscroll_request,
                                window,
                                cx,
                            );
                        if was_scrolled.0 {
                            snapshot = editor.snapshot(window, cx);
                        }
                        (
                            autoscroll_request,
                            autoscroll_containing_element,
                            needs_horizontal_autoscroll,
                        )
                    });

                    let mut scroll_position = snapshot.scroll_position();
                    if !line_height.is_zero() {
                        scroll_position.y = window
                            .pixel_snap_f64(scroll_position.y * f64::from(line_height))
                            / f64::from(line_height);
                    }
                    // The scroll position is a fractional point, the whole number of which represents
                    // the top of the window in terms of display rows.
                    // We add clipped_top_in_lines to skip rows that are clipped by parent containers,
                    // but we don't modify scroll_position itself since the parent handles positioning.
                    let max_row = snapshot.max_point().row();
                    let start_row = cmp::min(
                        DisplayRow((scroll_position.y + clipped_top_in_lines).floor() as u32),
                        max_row,
                    );
                    let end_row = cmp::min(
                        (scroll_position.y + clipped_top_in_lines + visible_height_in_lines).ceil()
                            as u32,
                        max_row.next_row().0,
                    );
                    let end_row = DisplayRow(end_row);
                    let pointer_state = window.with_global_id(
                        ElementId::from(("editor-pointer", self.editor.entity_id().as_u64())),
                        |id, window| {
                            window.with_element_state(
                                id,
                                |state: Option<Rc<mouse::PointerState>>, _| {
                                    let state = state.unwrap_or_default();
                                    (state.clone(), state)
                                },
                            )
                        },
                    );

                    let mut viewport_geometry =
                        self.request_viewport_geometry(&snapshot, start_row..end_row, window, cx);
                    macro_rules! ready {
                        ($result:expr) => {
                            match $result {
                                Ok(value) => value,
                                Err(pending) => {
                                    self.retain_autoscroll_request(autoscroll_request, cx);
                                    let mut scroll_position = scroll_position;
                                    let scroll_pixel_x = window.pixel_snap_f64(
                                        scroll_position.x * f64::from(em_layout_width),
                                    );
                                    if !em_layout_width.is_zero() {
                                        scroll_position.x =
                                            scroll_pixel_x / f64::from(em_layout_width);
                                    }
                                    viewport_geometry.push(pending);
                                    self.submit_viewport_geometry(&viewport_geometry, cx);
                                    self.editor.update(cx, |editor, _| {
                                        editor.last_position_map = None;
                                        editor.last_input_viewport = Some(InputViewport {
                                            text_bounds: text_hitbox.bounds,
                                            content_origin,
                                            visible_row_range: start_row..end_row,
                                            scroll_pixel_position: point(
                                                scroll_pixel_x,
                                                scroll_position.y * f64::from(line_height),
                                            ),
                                            scroll_max_y: max_scroll_top,
                                            line_height,
                                            em_advance,
                                            em_layout_width,
                                        });
                                        editor.last_horizontal_scrollbar_visible = false;
                                    });
                                    let mut feedback = div()
                                        .child(Label::new("...").size(LabelSize::Small))
                                        .into_any_element();
                                    feedback.prepaint_as_root(
                                        point(content_origin.x, visible_top),
                                        AvailableSpace::min_size(),
                                        window,
                                        cx,
                                    );
                                    let mut text_layout_details =
                                        self.editor.update(cx, |editor, cx| {
                                            editor.text_layout_details(window, cx)
                                        });
                                    text_layout_details.editor_style = style.clone();
                                    return EditorPrepaintState::Pending(PendingViewport {
                                        mode: snapshot.mode.clone(),
                                        hitbox: hitbox.clone(),
                                        gutter_hitbox: gutter_hitbox.clone(),
                                        text_hitbox: text_hitbox.clone(),
                                        content_origin,
                                        line_height,
                                        em_layout_width,
                                        scroll_position,
                                        scroll_max_y: max_scroll_top,
                                        text_align: style.text.text_align,
                                        pointer_state,
                                        snapshot,
                                        text_layout_details,
                                        feedback,
                                    });
                                }
                            }
                        };
                    }
                    if let Some(pending) = viewport_geometry.first().cloned() {
                        ready!(Err::<(), _>(pending));
                    }
                    let row_infos = snapshot // note we only get the visual range
                        .row_infos(start_row)
                        .take((start_row..end_row).len())
                        .collect::<Vec<RowInfo>>();
                    let is_row_soft_wrapped = |row: usize| {
                        row_infos
                            .get(row)
                            .is_none_or(|info| info.buffer_row.is_none())
                    };

                    let start_anchor = if start_row == Default::default() {
                        Anchor::Min
                    } else {
                        snapshot.buffer_snapshot().anchor_before(
                            DisplayPoint::new(start_row, 0).to_offset(&snapshot, Bias::Left),
                        )
                    };
                    let end_anchor = if end_row > max_row {
                        Anchor::Max
                    } else {
                        snapshot.buffer_snapshot().anchor_before(
                            DisplayPoint::new(end_row, 0).to_offset(&snapshot, Bias::Right),
                        )
                    };
                    let long_row_scroll_x =
                        window.pixel_snap_f64(scroll_position.x * f64::from(em_layout_width));
                    let long_rows =
                        ready!(snapshot.long_rows_in_view(start_row..end_row, |long_row| {
                            if is_minimap {
                                0..long_row.floor_column(MAX_LINE_LEN as u32)
                            } else {
                                long_row.visible_window(
                                    long_row_scroll_x - f64::from(gutter_dimensions.margin),
                                    style.text.text_align,
                                    f64::from(text_hitbox.size.width),
                                )
                            }
                        }));
                    let visible_ranges =
                        visible_anchor_ranges(&snapshot, start_anchor..end_anchor, &long_rows);

                    let mut highlighted_rows =
                        self.editor.read(cx).highlighted_display_rows_in_range(
                            start_anchor..end_anchor,
                            start_row..end_row,
                            &snapshot.display_snapshot,
                            cx,
                        );

                    let mut highlighted_ranges = self
                        .editor_with_selections(cx)
                        .map(|editor| {
                            if editor == self.editor {
                                editor.read(cx).background_highlights_in_ranges(
                                    &visible_ranges,
                                    &snapshot.display_snapshot,
                                    cx.theme(),
                                )
                            } else {
                                editor.update(cx, |editor, cx| {
                                    let snapshot = editor.snapshot(window, cx);
                                    let start_anchor = if start_row == Default::default() {
                                        Anchor::Min
                                    } else {
                                        snapshot.buffer_snapshot().anchor_before(
                                            DisplayPoint::new(start_row, 0)
                                                .to_offset(&snapshot, Bias::Left),
                                        )
                                    };
                                    let end_anchor = if end_row > max_row {
                                        Anchor::Max
                                    } else {
                                        snapshot.buffer_snapshot().anchor_before(
                                            DisplayPoint::new(end_row, 0)
                                                .to_offset(&snapshot, Bias::Right),
                                        )
                                    };

                                    if long_rows.is_empty() {
                                        editor.background_highlights_in_range(
                                            start_anchor..end_anchor,
                                            &snapshot.display_snapshot,
                                            cx.theme(),
                                        )
                                    } else {
                                        editor.background_highlights_in_ranges(
                                            &visible_ranges,
                                            &snapshot.display_snapshot,
                                            cx.theme(),
                                        )
                                    }
                                })
                            }
                        })
                        .unwrap_or_default();

                    struct DiffHunkHighlightColors {
                        filled_background: Hsla,
                        hollow_background: Hsla,
                        hollow_border: Hsla,
                    }

                    let colors = cx.theme().colors();
                    let added_diff_hunk_colors = DiffHunkHighlightColors {
                        filled_background: colors.editor_diff_hunk_added_background,
                        hollow_background: colors.editor_diff_hunk_added_hollow_background,
                        hollow_border: colors.editor_diff_hunk_added_hollow_border,
                    };
                    let deleted_diff_hunk_colors = DiffHunkHighlightColors {
                        filled_background: colors.editor_diff_hunk_deleted_background,
                        hollow_background: colors.editor_diff_hunk_deleted_hollow_background,
                        hollow_border: colors.editor_diff_hunk_deleted_hollow_border,
                    };
                    let drag_highlight_color = colors.editor_active_line_background;
                    let drag_border_color = colors.border_focused;

                    for (ix, row_info) in row_infos.iter().enumerate() {
                        let Some(diff_status) = row_info.diff_status else {
                            continue;
                        };

                        let diff_hunk_colors = match diff_status.kind {
                            DiffHunkStatusKind::Added => &added_diff_hunk_colors,
                            DiffHunkStatusKind::Deleted => &deleted_diff_hunk_colors,
                            DiffHunkStatusKind::Modified => {
                                debug_panic!("modified diff status for row info");
                                continue;
                            }
                        };

                        let hollow_highlight = LineHighlight {
                            background: diff_hunk_colors.hollow_background.into(),
                            border: Some(diff_hunk_colors.hollow_border),
                            include_gutter: true,
                            type_id: None,
                        };

                        let filled_highlight = LineHighlight {
                            background: solid_background(diff_hunk_colors.filled_background),
                            border: None,
                            include_gutter: true,
                            type_id: None,
                        };

                        let background = if self.diff_hunk_hollow(diff_status, cx) {
                            hollow_highlight
                        } else {
                            filled_highlight
                        };

                        let base_display_point =
                            DisplayPoint::new(start_row + DisplayRow(ix as u32), 0);

                        highlighted_rows
                            .entry(base_display_point.row())
                            .or_insert(background);
                    }

                    // Add diff review drag selection highlight to text area
                    if let Some(drag_state) = &self.editor.read(cx).diff_review_drag_state {
                        let range = drag_state.row_range(&snapshot.display_snapshot);
                        let start_row = range.start().0;
                        let end_row = range.end().0;
                        let drag_highlight = LineHighlight {
                            background: solid_background(drag_highlight_color),
                            border: Some(drag_border_color),
                            include_gutter: true,
                            type_id: None,
                        };
                        for row_num in start_row..=end_row {
                            highlighted_rows
                                .entry(DisplayRow(row_num))
                                .or_insert(drag_highlight);
                        }
                    }

                    let highlighted_gutter_ranges =
                        self.editor.read(cx).gutter_highlights_in_range(
                            start_anchor..end_anchor,
                            &snapshot.display_snapshot,
                            cx,
                        );

                    let mut document_colors = self
                        .editor
                        .read(cx)
                        .colors
                        .as_ref()
                        .map(|colors| colors.editor_display_highlights(&snapshot));
                    let mut redacted_ranges = visible_ranges
                        .iter()
                        .flat_map(|visible_range| {
                            self.editor.read(cx).redacted_ranges(
                                visible_range.clone(),
                                &snapshot.display_snapshot,
                                cx,
                            )
                        })
                        .collect::<Vec<_>>();
                    redacted_ranges.dedup();

                    let (local_selections, selected_buffer_ids, latest_selection_anchors): (
                        Vec<Selection<Point>>,
                        Vec<BufferId>,
                        HashMap<BufferId, Anchor>,
                    ) = self
                        .editor_with_selections(cx)
                        .map(|editor| {
                            editor.update(cx, |editor, cx| {
                                let is_singleton =
                                    editor.buffer_kind(cx) == ItemBufferKind::Singleton;

                                // Singleton buffers only need the newest selection anchor here.
                                let selected_buffer_ids = if is_singleton {
                                    Vec::new()
                                } else {
                                    let all_selections =
                                        editor.selections.all::<Point>(&snapshot.display_snapshot);
                                    let mut selected_buffer_ids =
                                        Vec::with_capacity(all_selections.len());

                                    for selection in all_selections {
                                        for buffer_id in snapshot
                                            .buffer_snapshot()
                                            .buffer_ids_for_range(selection.range())
                                        {
                                            if selected_buffer_ids.last() != Some(&buffer_id) {
                                                selected_buffer_ids.push(buffer_id);
                                            }
                                        }
                                    }

                                    selected_buffer_ids
                                };

                                let mut selections =
                                    if long_rows.is_empty() || editor.selections.line_mode() {
                                        editor.selections.disjoint_in_row_range(
                                            start_anchor..end_anchor,
                                            &snapshot.display_snapshot,
                                        )
                                    } else {
                                        let mut selections = editor.selections.disjoint_in_ranges(
                                            &visible_ranges,
                                            &snapshot.display_snapshot,
                                        );
                                        let newest_id = editor.selections.newest_anchor().id;
                                        let newest_is_pending = editor
                                            .selections
                                            .pending_anchor()
                                            .is_some_and(|pending| pending.id == newest_id);
                                        if !newest_is_pending
                                            && !selections
                                                .iter()
                                                .any(|selection| selection.id == newest_id)
                                        {
                                            let newest = editor
                                                .selections
                                                .newest::<Point>(&snapshot.display_snapshot);
                                            let index = selections.partition_point(|selection| {
                                                selection.start < newest.start
                                            });
                                            selections.insert(index, newest);
                                        }
                                        selections
                                    };
                                selections
                                    .extend(editor.selections.pending(&snapshot.display_snapshot));

                                let latest_selection_anchors: HashMap<BufferId, Anchor> =
                                    if is_singleton {
                                        let head = editor.selections.newest_anchor().head();
                                        snapshot
                                            .buffer_snapshot()
                                            .anchor_to_buffer_anchor(head)
                                            .map(|(text_anchor, _)| (text_anchor.buffer_id, head))
                                            .into_iter()
                                            .collect()
                                    } else {
                                        let all_anchor_selections = editor
                                            .selections
                                            .all_anchors(&snapshot.display_snapshot);
                                        let mut anchors_by_buffer: HashMap<
                                            BufferId,
                                            (usize, Anchor),
                                        > = HashMap::default();
                                        for selection in all_anchor_selections.iter() {
                                            let head = selection.head();
                                            if let Some((text_anchor, _)) = snapshot
                                                .buffer_snapshot()
                                                .anchor_to_buffer_anchor(head)
                                            {
                                                anchors_by_buffer
                                                    .entry(text_anchor.buffer_id)
                                                    .and_modify(|(latest_id, latest_anchor)| {
                                                        if selection.id > *latest_id {
                                                            *latest_id = selection.id;
                                                            *latest_anchor = head;
                                                        }
                                                    })
                                                    .or_insert((selection.id, head));
                                            }
                                        }
                                        anchors_by_buffer
                                            .into_iter()
                                            .map(|(buffer_id, (_, anchor))| (buffer_id, anchor))
                                            .collect()
                                    };

                                (selections, selected_buffer_ids, latest_selection_anchors)
                            })
                        })
                        .unwrap_or_else(|| (Vec::new(), Vec::new(), HashMap::default()));

                    let (selections, mut active_rows, newest_selection_head) = self
                        .layout_selections(
                            start_anchor,
                            end_anchor,
                            &visible_ranges,
                            &local_selections,
                            &snapshot,
                            start_row,
                            end_row,
                            window,
                            cx,
                        );
                    if let Some(editor) = self.editor_with_selections(cx).filter(|editor| {
                        !long_rows.is_empty() && editor.read(cx).show_local_selections
                    }) {
                        let selections = &editor.read(cx).selections;
                        let buffer = snapshot.buffer_snapshot();
                        for (row, long_row, _) in &long_rows {
                            let row_range = buffer
                                .anchor_before(DisplayPoint::new(*row, 0).to_point(&snapshot))
                                ..buffer.anchor_after(
                                    DisplayPoint::new(*row, long_row.len()).to_point(&snapshot),
                                );
                            if let Some(has_non_empty_selection) =
                                selections.disjoint_activity_in_range(&row_range, buffer)
                            {
                                active_rows.entry(*row).or_default().selection |=
                                    has_non_empty_selection;
                            }
                        }
                    }

                    // relative rows are based on newest selection, even outside the visible area
                    let current_selection_head = self.editor.update(cx, |editor, cx| {
                        (editor.selections.count() != 0).then(|| {
                            let newest = editor
                                .selections
                                .newest::<Point>(&editor.display_snapshot(cx));

                            SelectionLayout::new(
                                newest,
                                editor.selections.line_mode(),
                                editor.cursor_offset_on_selection,
                                editor.cursor_shape,
                                &snapshot,
                                true,
                                true,
                                None,
                            )
                            .head
                            .row()
                        })
                    });

                    let run_indicator_rows = self.editor.update(cx, |editor, cx| {
                        editor.active_run_indicators(start_row..end_row, window, cx)
                    });

                    let mut breakpoint_rows = self.editor.update(cx, |editor, cx| {
                        editor.active_breakpoints(start_row..end_row, window, cx)
                    });

                    for (display_row, (_, bp, state)) in &breakpoint_rows {
                        if bp.is_enabled() && state.is_none_or(|s| s.verified) {
                            active_rows.entry(*display_row).or_default().breakpoint = true;
                        }
                    }

                    let gutter = Gutter {
                        line_height,
                        range: start_row..end_row,
                        scroll_position,
                        dimensions: &gutter_dimensions,
                        hitbox: &gutter_hitbox,
                        snapshot: &snapshot,
                        row_infos: &row_infos,
                    };

                    let line_numbers = self.layout_line_numbers(
                        &gutter,
                        &active_rows,
                        current_selection_head,
                        window,
                        cx,
                    );

                    let mut expand_toggles =
                        window.with_element_namespace("expand_toggles", |window| {
                            self.layout_expand_toggles(
                                &gutter_hitbox,
                                gutter_dimensions,
                                em_width,
                                line_height,
                                scroll_position,
                                start_row,
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
                                row_infos.iter().cloned(),
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
                        scroll_position,
                        window,
                        cx,
                    );

                    Self::layout_word_diff_highlights(
                        &display_hunks,
                        &row_infos,
                        start_row,
                        &visible_ranges,
                        &snapshot,
                        &mut highlighted_ranges,
                        cx,
                    );
                    if !long_rows.is_empty() {
                        let painted_columns = |row| {
                            let index = long_rows
                                .binary_search_by_key(&row, |(long_row_row, ..)| *long_row_row)
                                .ok()?;
                            Some(long_rows[index].2.clone())
                        };
                        clip_ranges_to_columns(
                            &mut highlighted_ranges,
                            |(range, _)| range,
                            painted_columns,
                        );
                        if let Some((_, colors)) = document_colors.as_mut() {
                            clip_ranges_to_columns(colors, |(range, _)| range, painted_columns);
                        }
                        clip_ranges_to_columns(
                            &mut redacted_ranges,
                            |range| range,
                            painted_columns,
                        );
                    }

                    let bg_segments_per_row = Self::bg_segments_per_row(
                        start_row..end_row,
                        &selections,
                        highlighted_ranges.iter().cloned().chain(
                            document_colors
                                .iter()
                                .flat_map(|(_, colors)| colors.iter().cloned()),
                        ),
                        self.style.background,
                    );

                    let paint_bounds = window
                        .cover_bounds(text_hitbox.bounds.intersect(&window.content_mask().bounds));
                    let mut line_layouts = ready!(Self::layout_lines(
                        start_row..end_row,
                        &snapshot,
                        &self.style,
                        editor_width,
                        text_hitbox.size.width,
                        content_origin.x,
                        paint_bounds.left()..paint_bounds.right(),
                        long_row_scroll_x,
                        &long_rows,
                        is_row_soft_wrapped,
                        &bg_segments_per_row,
                        window,
                        cx,
                    ));
                    let renderer_widths_changed = !is_minimap
                        && request_layout.has_remaining_prepaint_depth()
                        && self.publish_renderer_widths(&line_layouts, editor_width, window, cx);
                    if renderer_widths_changed {
                        self.retain_autoscroll_request(autoscroll_request, cx);
                        return self.prepaint(
                            None,
                            _inspector_id,
                            bounds,
                            request_layout,
                            window,
                            cx,
                        );
                    }

                    let longest_line_blame_width = self
                        .editor
                        .update(cx, |editor, cx| {
                            if !editor.show_git_blame_inline {
                                return None;
                            }
                            // Blame is only painted inline for the Inline location, so
                            // reserving scroll room for it in other locations would let
                            // the editor scroll into blank space.
                            if ProjectSettings::get_global(cx).git.inline_blame.location
                                != InlineBlameLocation::Inline
                            {
                                return None;
                            }
                            let blame = editor.blame.as_ref()?;
                            let (_, blame_entry) = blame
                                .update(cx, |blame, cx| {
                                    let row_infos =
                                        snapshot.row_infos(snapshot.longest_row()).next()?;
                                    blame.blame_for_rows(&[row_infos], cx).next()
                                })
                                .flatten()?;
                            let mut element = render_inline_blame_entry(blame_entry, style, cx)?;
                            let inline_blame_padding =
                                ProjectSettings::get_global(cx).git.inline_blame.padding as f32
                                    * em_advance;
                            Some(
                                element
                                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                                    .width
                                    + inline_blame_padding,
                            )
                        })
                        .unwrap_or(Pixels::ZERO);

                    let max_long_row_width = if is_minimap {
                        Some(0.)
                    } else {
                        let display_map = self.editor.read(cx).display_map.clone();
                        display_map.update(cx, |map, cx| {
                            map.request_max_long_row_width(&snapshot.display_snapshot, cx)
                        })
                    };
                    let horizontal_scroll_pending = max_long_row_width.is_none();
                    let longest_row = snapshot.longest_row();
                    let longest_line_width = if !is_minimap && snapshot.is_long_row(longest_row) {
                        0.
                    } else {
                        ready!(layout_line(
                            longest_row,
                            &snapshot,
                            style,
                            editor_width,
                            0.,
                            is_row_soft_wrapped,
                            window,
                            cx,
                        ))
                        .width
                    };
                    let longest_line_width = line_layouts.iter().map(|line| line.width).fold(
                        longest_line_width.max(max_long_row_width.unwrap_or(0.)),
                        ScrollPixelOffset::max,
                    );

                    let scrollbar_layout_information = ScrollbarLayoutInformation::new(
                        text_hitbox.bounds,
                        glyph_grid_cell,
                        size(
                            Pixels::from(longest_line_width),
                            Pixels::from(max_row.as_f64() * f64::from(line_height)),
                        ),
                        longest_line_blame_width,
                        EditorSettings::get_global(cx),
                        scroll_beyond_last_line,
                    );

                    let mut scroll_width =
                        longest_line_width + ScrollPixelOffset::from(longest_line_blame_width);

                    let sticky_header_excerpt = if snapshot.buffer_snapshot().show_headers() {
                        snapshot.sticky_header_excerpt(scroll_position.y)
                    } else {
                        None
                    };
                    let sticky_header_excerpt_id = sticky_header_excerpt
                        .as_ref()
                        .map(|top| top.excerpt.buffer_id());

                    let buffer = snapshot.buffer_snapshot();
                    let start_buffer_row = MultiBufferRow(start_anchor.to_point(&buffer).row);
                    let end_buffer_row = MultiBufferRow(end_anchor.to_point(&buffer).row);

                    let preliminary_scroll_pixel_position = point(
                        scroll_position.x * f64::from(em_layout_width),
                        scroll_position.y * f64::from(line_height),
                    );
                    let indent_guides = self.layout_indent_guides(
                        content_origin,
                        text_hitbox.origin,
                        start_buffer_row..end_buffer_row,
                        preliminary_scroll_pixel_position,
                        line_height,
                        &snapshot,
                        window,
                        cx,
                    );
                    let indent_guides_for_spacers = indent_guides.clone();

                    let blocks = ready!(
                        (!is_minimap)
                            .then(|| {
                                window.with_element_namespace("blocks", |window| {
                                    self.render_blocks(
                                        start_row..end_row,
                                        &snapshot,
                                        &hitbox,
                                        &text_hitbox,
                                        editor_width,
                                        &mut scroll_width,
                                        &editor_margins,
                                        em_width,
                                        gutter_dimensions.full_width(),
                                        line_height,
                                        &line_layouts,
                                        &selected_buffer_ids,
                                        &latest_selection_anchors,
                                        is_row_soft_wrapped,
                                        sticky_header_excerpt_id,
                                        &indent_guides_for_spacers,
                                        window,
                                        cx,
                                    )
                                })
                            })
                            .transpose()
                    )
                    .unwrap_or_default();
                    let RenderBlocksOutput {
                        non_spacer_blocks: mut blocks,
                        mut spacer_blocks,
                        row_block_types,
                        resized_blocks,
                    } = blocks;
                    if let Some(resized_blocks) = resized_blocks {
                        if request_layout.has_remaining_prepaint_depth() {
                            self.editor.update(cx, |editor, cx| {
                                editor.resize_blocks(
                                    resized_blocks,
                                    autoscroll_request.map(|(autoscroll, _)| autoscroll),
                                    cx,
                                )
                            });
                            return self.prepaint(
                                None,
                                _inspector_id,
                                bounds,
                                request_layout,
                                window,
                                cx,
                            );
                        } else {
                            debug_panic!(
                                "dropping block resize because prepaint depth \
                                 limit was reached"
                            );
                        }
                    }

                    let sticky_buffer_header = if self.should_show_buffer_headers() {
                        sticky_header_excerpt.map(|sticky_header_excerpt| {
                            window.with_element_namespace("blocks", |window| {
                                self.layout_sticky_buffer_header(
                                    sticky_header_excerpt,
                                    scroll_position,
                                    line_height,
                                    right_margin,
                                    &snapshot,
                                    &hitbox,
                                    &selected_buffer_ids,
                                    &blocks,
                                    &latest_selection_anchors,
                                    window,
                                    cx,
                                )
                            })
                        })
                    } else {
                        None
                    };

                    let scroll_max: gpui::Point<ScrollPixelOffset> = point(
                        ((scroll_width - ScrollPixelOffset::from(editor_width))
                            / ScrollPixelOffset::from(em_layout_width))
                        .max(0.0),
                        max_scroll_top,
                    );

                    let defer_horizontal_scroll = !is_minimap
                        && !long_rows.is_empty()
                        && !request_layout.has_remaining_prepaint_depth();
                    if defer_horizontal_scroll {
                        self.retain_autoscroll_request(autoscroll_request, cx);
                        window.request_animation_frame();
                    } else {
                        self.editor.update(cx, |editor, cx| {
                            if !horizontal_scroll_pending
                                && editor.scroll_manager.clamp_scroll_left(scroll_max.x, cx)
                            {
                                scroll_position.x = scroll_max.x.min(scroll_position.x);
                            }

                            if needs_horizontal_autoscroll.0
                                && let Some(new_scroll_position) = editor.autoscroll_horizontally(
                                    start_row,
                                    editor_width,
                                    scroll_width,
                                    em_layout_width,
                                    &line_layouts,
                                    autoscroll_request,
                                    window,
                                    cx,
                                )
                            {
                                scroll_position.x = new_scroll_position.x;
                            }
                        });
                    }

                    let scroll_pixel_x =
                        window.pixel_snap_f64(scroll_position.x * f64::from(em_layout_width));
                    if !em_layout_width.is_zero() {
                        scroll_position.x = scroll_pixel_x / f64::from(em_layout_width);
                    }

                    let scroll_pixel_position =
                        point(scroll_pixel_x, scroll_position.y * f64::from(line_height));
                    let long_rows_need_relayout = line_layouts.iter().any(|line| {
                        line.long_row_needs_relayout(
                            scroll_pixel_position.x,
                            style.text.text_align,
                            content_origin.x,
                            paint_bounds.left()..paint_bounds.right(),
                            text_hitbox.size.width,
                            window.scale_factor(),
                        )
                    });
                    let restart_for_uncovered_long_rows =
                        long_rows_need_relayout && request_layout.has_remaining_prepaint_depth();
                    let sticky_header_lines = if !restart_for_uncovered_long_rows
                        && !is_minimap
                        && is_singleton
                        && EditorSettings::get_global(cx).sticky_scroll.enabled
                    {
                        ready!(self.layout_sticky_header_lines(
                            &snapshot,
                            editor_width,
                            text_hitbox.size.width,
                            gutter_dimensions.margin,
                            content_origin.x,
                            paint_bounds.left()..paint_bounds.right(),
                            is_row_soft_wrapped,
                            scroll_pixel_position.x,
                            window,
                            cx,
                        ))
                    } else {
                        Vec::new()
                    };
                    let sticky_renderer_widths_changed = request_layout
                        .has_remaining_prepaint_depth()
                        && sticky_header_lines.iter().any(|(_, line)| {
                            line.fragments
                                .iter()
                                .any(|fragment| matches!(fragment, LineFragment::Element { .. }))
                        })
                        && self.publish_renderer_widths(
                            sticky_header_lines.iter().map(|(_, line)| line),
                            editor_width,
                            window,
                            cx,
                        );
                    if restart_for_uncovered_long_rows || sticky_renderer_widths_changed {
                        self.retain_autoscroll_request(autoscroll_request, cx);
                        return self.prepaint(
                            None,
                            _inspector_id,
                            bounds,
                            request_layout,
                            window,
                            cx,
                        );
                    }
                    debug_assert!(!long_rows_need_relayout);

                    if !is_minimap {
                        let horizontal_anchor = (style.text.text_align == TextAlign::Left)
                            .then(|| {
                                horizontal_scroll_anchor(
                                    &snapshot,
                                    start_row,
                                    &line_layouts,
                                    newest_selection_head,
                                    scroll_pixel_position.x,
                                )
                            })
                            .flatten();
                        self.editor.update(cx, |editor, cx| {
                            editor.scroll_manager.store_horizontal_anchor(
                                horizontal_anchor,
                                &snapshot,
                                cx,
                            );
                            editor.persist_scroll_position(&snapshot, cx);
                        });
                    }
                    let sticky_headers = if !sticky_header_lines.is_empty() {
                        let relative = self.editor.read(cx).relative_line_numbers(cx);
                        self.layout_sticky_headers(
                            sticky_header_lines,
                            &snapshot,
                            line_height,
                            scroll_pixel_position,
                            content_origin,
                            &gutter_dimensions,
                            &gutter_hitbox,
                            &text_hitbox,
                            relative,
                            current_selection_head,
                            window,
                            cx,
                        )
                    } else {
                        None
                    };
                    let indent_guides =
                        if scroll_pixel_position != preliminary_scroll_pixel_position {
                            self.layout_indent_guides(
                                content_origin,
                                text_hitbox.origin,
                                start_buffer_row..end_buffer_row,
                                scroll_pixel_position,
                                line_height,
                                &snapshot,
                                window,
                                cx,
                            )
                        } else {
                            indent_guides
                        };

                    let crease_trailers =
                        window.with_element_namespace("crease_trailers", |window| {
                            self.prepaint_crease_trailers(
                                crease_trailers,
                                &line_layouts,
                                line_height,
                                content_origin,
                                scroll_pixel_position,
                                scroll_position,
                                start_row,
                                em_width,
                                window,
                                cx,
                            )
                        });

                    let (edit_prediction_popover, edit_prediction_popover_origin) = self
                        .editor
                        .update(cx, |editor, cx| {
                            editor.render_edit_prediction_popover(
                                &text_hitbox.bounds,
                                content_origin,
                                right_margin,
                                &snapshot,
                                start_row..end_row,
                                scroll_position.y,
                                scroll_position.y + height_in_lines,
                                &line_layouts,
                                line_height,
                                scroll_position,
                                scroll_pixel_position,
                                newest_selection_head,
                                editor_width,
                                style,
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
                        scroll_position,
                        scroll_pixel_position,
                        edit_prediction_popover_origin,
                        start_row,
                        end_row,
                        line_height,
                        em_width,
                        style,
                        window,
                        cx,
                    );

                    let mut inline_blame_layout = None;
                    let mut inline_code_actions = None;
                    if let Some(newest_selection_head) = newest_selection_head {
                        let display_row = newest_selection_head.row();
                        if (start_row..end_row).contains(&display_row)
                            && !row_block_types.contains_key(&display_row)
                        {
                            inline_code_actions = self.layout_inline_code_actions(
                                newest_selection_head,
                                content_origin,
                                scroll_position,
                                scroll_pixel_position,
                                line_height,
                                &snapshot,
                                window,
                                cx,
                            );

                            let line_ix = display_row.minus(start_row) as usize;
                            if let (Some(row_info), Some(line_layout), Some(crease_trailer)) = (
                                row_infos.get(line_ix),
                                line_layouts.get(line_ix),
                                crease_trailers.get(line_ix),
                            ) {
                                let crease_trailer_layout = crease_trailer.as_ref();
                                if let Some(layout) = self.layout_inline_blame(
                                    display_row,
                                    row_info,
                                    line_layout,
                                    crease_trailer_layout,
                                    em_width,
                                    content_origin,
                                    scroll_position,
                                    scroll_pixel_position,
                                    line_height,
                                    window,
                                    cx,
                                ) {
                                    inline_blame_layout = Some(layout);
                                    // Blame overrides inline diagnostics
                                    inline_diagnostics.remove(&display_row);
                                }
                            } else {
                                log::error!(
                                    "bug: line_ix {} is out of bounds - row_infos.len(): {}, \
                                    line_layouts.len(): {}, \
                                    crease_trailers.len(): {}",
                                    line_ix,
                                    row_infos.len(),
                                    line_layouts.len(),
                                    crease_trailers.len(),
                                );
                            }
                        }
                    }

                    let blamed_display_rows = self.layout_blame_entries(
                        &row_infos,
                        em_width,
                        scroll_position,
                        start_row,
                        line_height,
                        &gutter_hitbox,
                        gutter_dimensions.git_blame_entries_width,
                        window,
                        cx,
                    );

                    let line_elements = self.prepaint_lines(
                        start_row,
                        &mut line_layouts,
                        line_height,
                        scroll_position,
                        scroll_pixel_position,
                        content_origin,
                        text_hitbox.size.width,
                        window,
                        cx,
                    );

                    window.with_element_namespace("blocks", |window| {
                        self.layout_blocks(
                            &mut blocks,
                            &hitbox,
                            &gutter_hitbox,
                            line_height,
                            scroll_position,
                            scroll_pixel_position,
                            &editor_margins,
                            window,
                            cx,
                        );
                        self.layout_blocks(
                            &mut spacer_blocks,
                            &hitbox,
                            &gutter_hitbox,
                            line_height,
                            scroll_position,
                            scroll_pixel_position,
                            &editor_margins,
                            window,
                            cx,
                        );
                    });

                    let cursors = self.collect_cursors(&snapshot, cx);
                    let visible_row_range = start_row..end_row;
                    let non_visible_cursors = cursors
                        .iter()
                        .any(|c| !visible_row_range.contains(&c.0.row()));

                    let visible_cursors = ready!(self.layout_visible_cursors(
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
                        &redacted_ranges,
                        window,
                        cx,
                    ));
                    let navigation_overlay_paint_commands = self.layout_navigation_overlays(
                        &snapshot,
                        start_row..end_row,
                        &line_layouts,
                        &text_hitbox,
                        content_origin,
                        scroll_position,
                        scroll_pixel_position,
                        line_height,
                        window,
                        cx,
                    );

                    let frozen_scroll_state = if self.editor.read(cx).search_results_hold.is_some()
                    {
                        let is_rewrapping =
                            self.editor.read(cx).display_map.read(cx).is_rewrapping(cx);
                        self.editor.update(cx, |editor, _| {
                            editor.frozen_scroll_range(
                                is_rewrapping,
                                scrollbar_layout_information.scroll_range,
                                editor_width,
                                scrollbar_layout_information.editor_bounds.size,
                            )
                        })
                    } else {
                        None
                    };
                    let effective_scrollbar_layout_information =
                        frozen_scroll_state.map_or(scrollbar_layout_information, |settled| {
                            ScrollbarLayoutInformation {
                                scroll_range: settled.range,
                                ..scrollbar_layout_information
                            }
                        });
                    let effective_editor_width =
                        frozen_scroll_state.map_or(editor_width, |settled| settled.editor_width);

                    let scrollbars_layout = self.layout_scrollbars(
                        &snapshot,
                        &effective_scrollbar_layout_information,
                        content_offset,
                        scroll_position,
                        horizontal_scroll_pending,
                        non_visible_cursors,
                        right_margin,
                        effective_editor_width,
                        window,
                        cx,
                    );

                    let gutter_settings = EditorSettings::get_global(cx).gutter;

                    let context_menu_layout =
                        if let Some(newest_selection_head) = newest_selection_head {
                            let newest_selection_point =
                                newest_selection_head.to_point(&snapshot.display_snapshot);
                            if (start_row..end_row).contains(&newest_selection_head.row()) {
                                self.layout_cursor_popovers(
                                    line_height,
                                    &text_hitbox,
                                    content_origin,
                                    right_margin,
                                    start_row,
                                    scroll_pixel_position,
                                    &line_layouts,
                                    newest_selection_head,
                                    newest_selection_point,
                                    style,
                                    window,
                                    cx,
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                    self.layout_gutter_menu(
                        line_height,
                        &text_hitbox,
                        content_origin,
                        right_margin,
                        scroll_pixel_position,
                        gutter_dimensions.width - gutter_dimensions.left_padding,
                        window,
                        cx,
                    );

                    let test_indicators = if gutter_settings.runnables {
                        self.layout_run_indicators(
                            &gutter,
                            &run_indicator_rows,
                            &breakpoint_rows,
                            window,
                            cx,
                        )
                    } else {
                        Vec::new()
                    };

                    let show_bookmarks =
                        snapshot.show_bookmarks.unwrap_or(gutter_settings.bookmarks);

                    let bookmark_rows = self.editor.update(cx, |editor, cx| {
                        let mut rows = editor.active_bookmarks(start_row..end_row, window, cx);
                        rows.retain(|k| !run_indicator_rows.contains(k));
                        rows.retain(|k| !breakpoint_rows.contains_key(k));
                        rows
                    });

                    let bookmarks = if show_bookmarks {
                        self.layout_bookmarks(&gutter, &bookmark_rows, window, cx)
                    } else {
                        Vec::new()
                    };

                    let show_breakpoints = snapshot
                        .show_breakpoints
                        .unwrap_or(gutter_settings.breakpoints);

                    breakpoint_rows.retain(|k, _| !run_indicator_rows.contains(k));
                    let mut breakpoints = if show_breakpoints {
                        self.layout_breakpoints(&gutter, &breakpoint_rows, window, cx)
                    } else {
                        Vec::new()
                    };

                    let gutter_hover_button = self
                        .editor
                        .read(cx)
                        .gutter_hover_button
                        .0
                        .filter(|phantom| phantom.is_active)
                        .map(|phantom| phantom.display_row);

                    if let Some(row) = gutter_hover_button
                        && !breakpoint_rows.contains_key(&row)
                        && !run_indicator_rows.contains(&row)
                        && !bookmark_rows.contains(&row)
                        && (show_bookmarks || show_breakpoints)
                    {
                        let position = snapshot
                            .display_point_to_anchor(DisplayPoint::new(row, 0), Bias::Right);
                        breakpoints.extend(
                            self.layout_gutter_hover_button(&gutter, position, row, window, cx),
                        );
                    }

                    let git_gutter_width = Self::gutter_strip_width(line_height, cx)
                        + gutter_dimensions
                            .git_blame_entries_width
                            .unwrap_or_default();
                    let available_width = gutter_dimensions.left_padding - git_gutter_width;

                    let max_line_number_length = self
                        .editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .widest_line_number()
                        .ilog10()
                        + 1;

                    let diff_review_button = self
                        .should_render_diff_review_button(
                            start_row..end_row,
                            &row_infos,
                            &snapshot,
                            cx,
                        )
                        .map(|(display_row, buffer_row)| {
                            let is_wide = max_line_number_length
                                >= EditorSettings::get_global(cx).gutter.min_line_number_digits
                                    as u32
                                && buffer_row.is_some_and(|row| {
                                    (row + 1).ilog10() + 1 == max_line_number_length
                                })
                                || gutter_dimensions.right_padding == px(0.);

                            let button_width = if is_wide {
                                available_width - px(6.)
                            } else {
                                available_width + em_width - px(6.)
                            };

                            let button = self.editor.update(cx, |editor, cx| {
                                editor
                                    .render_diff_review_button(display_row, button_width, cx)
                                    .into_any_element()
                            });
                            gutter.prepaint_button(button, display_row, window, cx)
                        });

                    self.layout_signature_help(
                        &hitbox,
                        content_origin,
                        scroll_pixel_position,
                        newest_selection_head,
                        start_row,
                        &line_layouts,
                        line_height,
                        em_width,
                        context_menu_layout,
                        window,
                        cx,
                    );

                    if !cx.has_active_drag() {
                        self.layout_hover_popovers(
                            &snapshot,
                            &hitbox,
                            start_row..end_row,
                            content_origin,
                            scroll_pixel_position,
                            &line_layouts,
                            line_height,
                            em_width,
                            context_menu_layout,
                            window,
                            cx,
                        );

                        self.layout_blame_popover(&snapshot, &hitbox, line_height, window, cx);
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
                            scroll_position,
                            start_row,
                            &gutter_hitbox,
                            window,
                            cx,
                        )
                    });

                    window.with_element_namespace("expand_toggles", |window| {
                        self.prepaint_expand_toggles(&mut expand_toggles, window, cx)
                    });

                    let wrap_guides = self.layout_wrap_guides(
                        scroll_pixel_position.x,
                        content_origin,
                        scrollbars_layout.as_ref(),
                        vertical_scrollbar_width,
                        &hitbox,
                        window,
                        cx,
                    );

                    let minimap = window.with_element_namespace("minimap", |window| {
                        self.layout_minimap(
                            &snapshot,
                            minimap_width,
                            scroll_position,
                            &effective_scrollbar_layout_information,
                            scrollbars_layout.as_ref(),
                            window,
                            cx,
                        )
                    });

                    let invisible_symbol_font_size = font_size / 2.;
                    let whitespace_map = &self
                        .editor
                        .read(cx)
                        .buffer
                        .read(cx)
                        .language_settings(cx)
                        .whitespace_map;

                    let tab_char = whitespace_map.tab.clone();
                    let tab_len = tab_char.len();
                    let tab_invisible = window.text_system().shape_line(
                        tab_char,
                        invisible_symbol_font_size,
                        &[TextRun {
                            len: tab_len,
                            font: self.style.text.font(),
                            color: cx.theme().colors().editor_invisible,
                            ..Default::default()
                        }],
                        None,
                    );

                    let space_char = whitespace_map.space.clone();
                    let space_len = space_char.len();
                    let space_invisible = window.text_system().shape_line(
                        space_char,
                        invisible_symbol_font_size,
                        &[TextRun {
                            len: space_len,
                            font: self.style.text.font(),
                            color: cx.theme().colors().editor_invisible,
                            ..Default::default()
                        }],
                        None,
                    );

                    let mode = snapshot.mode.clone();

                    let sticky_scroll_header_height = sticky_headers
                        .as_ref()
                        .and_then(|headers| headers.lines.last())
                        .map_or(Pixels::ZERO, |last| last.offset + line_height);

                    let has_sticky_buffer_header =
                        sticky_buffer_header.is_some() || sticky_header_excerpt_id.is_some();
                    let sticky_header_height = if has_sticky_buffer_header {
                        let full_height = FILE_HEADER_HEIGHT as f32 * line_height;
                        let display_row = blocks
                            .iter()
                            .filter(|block| block.is_buffer_header)
                            .find_map(|block| {
                                block.row.filter(|row| row.0 > scroll_position.y as u32)
                            });
                        let offset = match display_row {
                            Some(display_row) => {
                                let max_row = display_row.0.saturating_sub(FILE_HEADER_HEIGHT);
                                let offset = (scroll_position.y - max_row as f64).max(0.0);
                                let slide_up =
                                    Pixels::from(offset * ScrollPixelOffset::from(line_height));

                                (full_height - slide_up).max(Pixels::ZERO)
                            }
                            None => full_height,
                        };
                        let header_bottom_padding =
                            BUFFER_HEADER_PADDING.to_pixels(window.rem_size());
                        sticky_scroll_header_height + offset - header_bottom_padding
                    } else {
                        sticky_scroll_header_height
                    };

                    let (diff_hunk_controls, diff_hunk_control_bounds) =
                        if is_read_only && self.editor.read(cx).diff_hunk_renderer.is_none() {
                            (vec![], vec![])
                        } else {
                            self.layout_diff_hunk_controls(
                                start_row..end_row,
                                &row_infos,
                                &text_hitbox,
                                current_selection_head,
                                line_height,
                                right_margin,
                                scroll_pixel_position,
                                sticky_header_height,
                                &display_hunks,
                                &highlighted_rows,
                                self.editor.clone(),
                                window,
                                cx,
                            )
                        };

                    self.populate_point_diagnostics(
                        &snapshot,
                        start_row..end_row,
                        &visible_ranges,
                        &mut line_layouts,
                    );

                    let position_map = Rc::new(PositionMap {
                        size: bounds.size,
                        visible_row_range,
                        scroll_position,
                        scroll_pixel_position,
                        scroll_max,
                        horizontal_scroll_pending,
                        line_layouts,
                        line_height,
                        em_advance,
                        em_layout_width,
                        snapshot,
                        pointer_state,
                        text_align: self.style.text.text_align,
                        content_width: text_hitbox.size.width,
                        content_origin,
                        gutter_hitbox: gutter_hitbox.clone(),
                        text_hitbox: text_hitbox.clone(),
                        inline_blame_bounds: inline_blame_layout
                            .as_ref()
                            .map(|layout| (layout.bounds, layout.buffer_id, layout.entry.clone())),
                        display_hunks: display_hunks.clone(),
                        diff_hunk_control_bounds,
                    });

                    let visible_horizontal_scrollbar =
                        scrollbars_layout.as_ref().is_some_and(|scrollbars_layout| {
                            scrollbars_layout.visible && scrollbars_layout.horizontal.is_some()
                        });

                    self.editor.update(cx, |editor, _| {
                        editor.last_input_viewport = Some(InputViewport {
                            text_bounds: text_hitbox.bounds,
                            content_origin,
                            visible_row_range: position_map.visible_row_range.clone(),
                            scroll_pixel_position,
                            scroll_max_y: max_scroll_top,
                            line_height,
                            em_advance,
                            em_layout_width,
                        });
                        editor.last_position_map = Some(position_map.clone());
                        editor.last_right_margin = right_margin;
                        editor.last_horizontal_scrollbar_visible = visible_horizontal_scrollbar;
                    });

                    EditorPrepaintState::Ready(EditorLayout {
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
                        minimap,
                        active_rows,
                        highlighted_rows,
                        highlighted_ranges,
                        highlighted_gutter_ranges,
                        redacted_ranges,
                        document_colors,
                        line_elements,
                        line_numbers,
                        blamed_display_rows,
                        inline_diagnostics,
                        point_diagnostic_underline_offset,
                        inline_blame_layout,
                        inline_code_actions,
                        blocks,
                        spacer_blocks,
                        cursors,
                        visible_cursors,
                        navigation_overlay_paint_commands,
                        selections,
                        edit_prediction_popover,
                        diff_hunk_controls,
                        mouse_context_menu,
                        test_indicators,
                        bookmarks,
                        breakpoints,
                        diff_review_button,
                        crease_toggles,
                        crease_trailers,
                        tab_invisible,
                        space_invisible,
                        sticky_buffer_header,
                        sticky_headers,
                        expand_toggles,
                        text_align: self.style.text.text_align,
                        content_width: text_hitbox.size.width,
                    })
                })
            })
        })
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<gpui::Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mode = match layout {
            EditorPrepaintState::Ready(layout) => &layout.mode,
            EditorPrepaintState::Pending(layout) => &layout.mode,
            EditorPrepaintState::Unavailable { mode, feedback } => {
                if !mode.is_minimap() {
                    let key_context = self
                        .editor
                        .update(cx, |editor, cx| editor.key_context(window, cx));
                    window.set_key_context(key_context);
                    self.editor.update(cx, |editor, cx| {
                        for action in editor.editor_actions.borrow().values() {
                            (action)(editor, true, window, cx);
                        }
                    });
                    if !self.editor.read(cx).read_only(cx) {
                        register_recovery_action(&self.editor, window, Editor::undo);
                        register_recovery_action(&self.editor, window, Editor::redo);
                    }
                }
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    window.paint_quad(fill(bounds, self.style.background));
                    if !mode.is_minimap() {
                        feedback.paint(window, cx);
                    }
                });
                return;
            }
        };
        if !mode.is_minimap() {
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
            self.register_key_listeners(
                window,
                match layout {
                    EditorPrepaintState::Ready(layout) => Some(layout.position_map.clone()),
                    EditorPrepaintState::Pending(_) | EditorPrepaintState::Unavailable { .. } => {
                        None
                    }
                },
            );
        }
        let layout = match layout {
            EditorPrepaintState::Ready(layout) => layout,
            EditorPrepaintState::Unavailable { .. } => return,
            EditorPrepaintState::Pending(layout) => {
                let text_style = TextStyleRefinement {
                    font_size: Some(self.style.text.font_size),
                    line_height: Some(self.style.text.line_height),
                    ..TextStyleRefinement::default()
                };
                window.with_rem_size(Some(layout.text_layout_details.rem_size), |window| {
                    window.with_text_style(Some(text_style), |window| {
                        window.with_content_mask(Some(ContentMask { bounds }), |window| {
                            window.paint_quad(fill(bounds, self.style.background));
                            if !layout.mode.is_minimap() {
                                window.paint_quad(fill(
                                    layout.gutter_hitbox.bounds,
                                    cx.theme().colors().editor_gutter_background,
                                ));
                                layout.feedback.paint(window, cx);
                                self.paint_pending_mouse_listeners(layout, window, cx);
                            }
                        });
                    });
                });
                return;
            }
        };

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

                    // Mask the editor behind sticky scroll headers. Important
                    // for transparent backgrounds.
                    let below_sticky_headers_mask = layout
                        .sticky_headers
                        .as_ref()
                        .and_then(|h| h.lines.last())
                        .map(|last| ContentMask {
                            bounds: Bounds {
                                origin: point(
                                    bounds.origin.x,
                                    bounds.origin.y + last.offset + layout.position_map.line_height,
                                ),
                                size: size(
                                    bounds.size.width,
                                    (bounds.size.height
                                        - last.offset
                                        - layout.position_map.line_height)
                                        .max(Pixels::ZERO),
                                ),
                            },
                        });

                    window.with_content_mask(below_sticky_headers_mask, |window| {
                        self.paint_background(layout, window, cx);

                        self.paint_indent_guides(layout, window, cx);

                        if layout.gutter_hitbox.size.width > Pixels::ZERO {
                            self.paint_blamed_display_rows(layout, window, cx);
                            self.paint_line_numbers(layout, window, cx);
                        }

                        self.paint_text(layout, window, cx);

                        if !layout.spacer_blocks.is_empty() {
                            window.with_element_namespace("blocks", |window| {
                                self.paint_spacer_blocks(layout, window, cx);
                            });
                        }

                        if layout.gutter_hitbox.size.width > Pixels::ZERO {
                            self.paint_gutter_highlights(layout, window, cx);
                            self.paint_gutter_indicators(layout, window, cx);
                        }

                        if !layout.blocks.is_empty() {
                            window.with_element_namespace("blocks", |window| {
                                self.paint_non_spacer_blocks(layout, window, cx);
                            });
                        }
                    });

                    window.with_element_namespace("blocks", |window| {
                        if let Some(mut sticky_header) = layout.sticky_buffer_header.take() {
                            sticky_header.paint(window, cx)
                        }
                    });

                    self.paint_sticky_headers(layout, window, cx);
                    self.paint_minimap(layout, window, cx);
                    self.paint_scrollbars(layout, window, cx);
                    self.paint_edit_prediction_popover(layout, window, cx);
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

#[derive(Clone, Copy)]
struct ContextMenuLayout {
    y_flipped: bool,
    bounds: Bounds<Pixels>,
}

/// Holds information required for layouting the editor scrollbars.
#[derive(Clone, Copy)]
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
        settings: &EditorSettings,
        scroll_beyond_last_line: ScrollBeyondLastLine,
    ) -> Self {
        let vertical_overscroll = match scroll_beyond_last_line {
            ScrollBeyondLastLine::OnePage => editor_bounds.size.height,
            ScrollBeyondLastLine::Off => glyph_grid_cell.height,
            ScrollBeyondLastLine::VerticalScrollMargin => {
                (1.0 + settings.vertical_scroll_margin) as f32 * glyph_grid_cell.height
            }
        };

        let overscroll = size(longest_line_blame_width, vertical_overscroll);

        ScrollbarLayoutInformation {
            editor_bounds,
            scroll_range: document_size + overscroll,
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

pub enum EditorPrepaintState {
    Ready(EditorLayout),
    Pending(PendingViewport),
    Unavailable {
        mode: EditorMode,
        feedback: AnyElement,
    },
}

pub struct PendingViewport {
    mode: EditorMode,
    hitbox: Hitbox,
    gutter_hitbox: Hitbox,
    text_hitbox: Hitbox,
    content_origin: gpui::Point<Pixels>,
    line_height: Pixels,
    em_layout_width: Pixels,
    scroll_position: gpui::Point<ScrollOffset>,
    scroll_max_y: ScrollOffset,
    text_align: TextAlign,
    pointer_state: Rc<mouse::PointerState>,
    snapshot: EditorSnapshot,
    text_layout_details: crate::movement::TextLayoutDetails,
    feedback: AnyElement,
}

pub struct EditorLayout {
    position_map: Rc<PositionMap>,
    hitbox: Hitbox,
    gutter_hitbox: Hitbox,
    content_origin: gpui::Point<Pixels>,
    scrollbars_layout: Option<EditorScrollbars>,
    minimap: Option<MinimapLayout>,
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
    point_diagnostic_underline_offset: Pixels,
    inline_blame_layout: Option<InlineBlameLayout>,
    inline_code_actions: Option<AnyElement>,
    blocks: Vec<BlockLayout>,
    spacer_blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    highlighted_gutter_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    redacted_ranges: Vec<Range<DisplayPoint>>,
    cursors: Vec<(DisplayPoint, Hsla)>,
    visible_cursors: Vec<CursorLayout>,
    navigation_overlay_paint_commands: Vec<NavigationOverlayPaintCommand>,
    selections: Vec<(PlayerColor, Vec<SelectionLayout>)>,
    test_indicators: Vec<AnyElement>,
    bookmarks: Vec<AnyElement>,
    breakpoints: Vec<AnyElement>,
    diff_review_button: Option<AnyElement>,
    crease_toggles: Vec<Option<AnyElement>>,
    expand_toggles: Vec<Option<(AnyElement, gpui::Point<Pixels>)>>,
    diff_hunk_controls: Vec<AnyElement>,
    crease_trailers: Vec<Option<CreaseTrailerLayout>>,
    edit_prediction_popover: Option<AnyElement>,
    mouse_context_menu: Option<AnyElement>,
    tab_invisible: ShapedLine,
    space_invisible: ShapedLine,
    sticky_buffer_header: Option<AnyElement>,
    sticky_headers: Option<header::StickyHeaders>,
    document_colors: Option<(DocumentColorsRenderMode, Vec<(Range<DisplayPoint>, Hsla)>)>,
    text_align: TextAlign,
    content_width: Pixels,
}

impl EditorLayout {
    fn line_end_overshoot(&self) -> Pixels {
        0.15 * self.position_map.line_height
    }
}

#[derive(Debug)]
struct LineNumberSegment {
    shaped_line: ShapedLine,
    hitbox: Option<Hitbox>,
}

#[derive(Debug)]
struct LineNumberLayout {
    segments: SmallVec<[LineNumberSegment; 1]>,
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
        show_scrollbar: ScrollbarAxes,
        layout_information: &ScrollbarLayoutInformation,
        content_offset: gpui::Point<Pixels>,
        scroll_position: gpui::Point<f64>,
        scrollbar_width: Pixels,
        right_margin: Pixels,
        editor_width: Pixels,
        show_scrollbars: bool,
        scrollbar_state: Option<&ActiveScrollbarState>,
        window: &mut Window,
    ) -> Self {
        let ScrollbarLayoutInformation {
            editor_bounds,
            scroll_range,
            glyph_grid_cell,
        } = layout_information;

        let viewport_size = size(editor_width, editor_bounds.size.height);

        let scrollbar_bounds_for = |axis: ScrollbarAxis| match axis {
            ScrollbarAxis::Horizontal => Bounds::from_anchor_and_size(
                gpui::Anchor::BottomLeft,
                editor_bounds.bottom_left(),
                size(
                    // The horizontal viewport size differs from the space available for the
                    // horizontal scrollbar, so we have to manually stitch it together here.
                    editor_bounds.size.width - right_margin,
                    scrollbar_width,
                ),
            ),
            ScrollbarAxis::Vertical => Bounds::from_anchor_and_size(
                gpui::Anchor::TopRight,
                editor_bounds.top_right(),
                size(scrollbar_width, viewport_size.height),
            ),
        };

        let mut create_scrollbar_layout = |axis| {
            let viewport_size = viewport_size.along(axis);
            let scroll_range = scroll_range.along(axis);

            // We always want a vertical scrollbar track for scrollbar diagnostic visibility.
            (show_scrollbar.along(axis)
                && (axis == ScrollbarAxis::Vertical || scroll_range > viewport_size))
                .then(|| {
                    ScrollbarLayout::new(
                        window.insert_hitbox(scrollbar_bounds_for(axis), HitboxBehavior::Normal),
                        viewport_size,
                        scroll_range,
                        glyph_grid_cell.along(axis),
                        content_offset.along(axis),
                        scroll_position.along(axis),
                        show_scrollbars,
                        axis,
                    )
                    .with_thumb_state(
                        scrollbar_state.and_then(|state| state.thumb_state_for_axis(axis)),
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
    visible_range: Range<ScrollOffset>,
    text_unit_size: Pixels,
    thumb_bounds: Option<Bounds<Pixels>>,
    thumb_state: ScrollbarThumbState,
}

impl ScrollbarLayout {
    const BORDER_WIDTH: Pixels = px(1.0);
    const LINE_MARKER_HEIGHT: Pixels = px(2.0);
    const MIN_MARKER_HEIGHT: Pixels = px(5.0);
    const MIN_THUMB_SIZE: Pixels = px(25.0);

    fn new(
        scrollbar_track_hitbox: Hitbox,
        viewport_size: Pixels,
        scroll_range: Pixels,
        glyph_space: Pixels,
        content_offset: Pixels,
        scroll_position: ScrollOffset,
        show_thumb: bool,
        axis: ScrollbarAxis,
    ) -> Self {
        let track_bounds = scrollbar_track_hitbox.bounds;
        // The length of the track available to the scrollbar thumb. We deliberately
        // exclude the content size here so that the thumb aligns with the content.
        let track_length = track_bounds.size.along(axis) - content_offset;

        Self::new_with_hitbox_and_track_length(
            scrollbar_track_hitbox,
            track_length,
            viewport_size,
            scroll_range.into(),
            glyph_space,
            content_offset.into(),
            scroll_position,
            show_thumb,
            axis,
        )
    }

    fn for_minimap(
        minimap_track_hitbox: Hitbox,
        visible_lines: f64,
        total_editor_lines: f64,
        minimap_line_height: Pixels,
        scroll_position: ScrollOffset,
        minimap_scroll_top: ScrollOffset,
        show_thumb: bool,
    ) -> Self {
        // The scrollbar thumb size is calculated as
        // (visible_content/total_content) × scrollbar_track_length.
        //
        // For the minimap's thumb layout, we leverage this by setting the
        // scrollbar track length to the entire document size (using minimap line
        // height). This creates a thumb that exactly represents the editor
        // viewport scaled to minimap proportions.
        //
        // We adjust the thumb position relative to `minimap_scroll_top` to
        // accommodate for the deliberately oversized track.
        //
        // This approach ensures that the minimap thumb accurately reflects the
        // editor's current scroll position whilst nicely synchronizing the minimap
        // thumb and scrollbar thumb.
        let scroll_range = total_editor_lines * f64::from(minimap_line_height);
        let viewport_size = visible_lines * f64::from(minimap_line_height);

        let track_top_offset = -minimap_scroll_top * f64::from(minimap_line_height);

        Self::new_with_hitbox_and_track_length(
            minimap_track_hitbox,
            Pixels::from(scroll_range),
            Pixels::from(viewport_size),
            scroll_range,
            minimap_line_height,
            track_top_offset,
            scroll_position,
            show_thumb,
            ScrollbarAxis::Vertical,
        )
    }

    fn new_with_hitbox_and_track_length(
        scrollbar_track_hitbox: Hitbox,
        track_length: Pixels,
        viewport_size: Pixels,
        scroll_range: f64,
        glyph_space: Pixels,
        content_offset: ScrollOffset,
        scroll_position: ScrollOffset,
        show_thumb: bool,
        axis: ScrollbarAxis,
    ) -> Self {
        let text_units_per_page = viewport_size.to_f64() / glyph_space.to_f64();
        let visible_range = scroll_position..scroll_position + text_units_per_page;
        let total_text_units = scroll_range / glyph_space.to_f64();

        let thumb_percentage = text_units_per_page / total_text_units;
        let thumb_size = Pixels::from(ScrollOffset::from(track_length) * thumb_percentage)
            .max(ScrollbarLayout::MIN_THUMB_SIZE)
            .min(track_length);

        let text_unit_divisor = (total_text_units - text_units_per_page).max(0.);

        let content_larger_than_viewport = text_unit_divisor > 0.;

        let text_unit_size = if content_larger_than_viewport {
            Pixels::from(ScrollOffset::from(track_length - thumb_size) / text_unit_divisor)
        } else {
            glyph_space
        };

        let thumb_bounds = (show_thumb && content_larger_than_viewport).then(|| {
            Self::thumb_bounds(
                &scrollbar_track_hitbox,
                content_offset,
                visible_range.start,
                text_unit_size,
                thumb_size,
                axis,
            )
        });

        ScrollbarLayout {
            hitbox: scrollbar_track_hitbox,
            visible_range,
            text_unit_size,
            thumb_bounds,
            thumb_state: Default::default(),
        }
    }

    fn with_thumb_state(self, thumb_state: Option<ScrollbarThumbState>) -> Self {
        if let Some(thumb_state) = thumb_state {
            Self {
                thumb_state,
                ..self
            }
        } else {
            self
        }
    }

    fn thumb_bounds(
        scrollbar_track: &Hitbox,
        content_offset: f64,
        visible_range_start: f64,
        text_unit_size: Pixels,
        thumb_size: Pixels,
        axis: ScrollbarAxis,
    ) -> Bounds<Pixels> {
        let thumb_origin = scrollbar_track.origin.apply_along(axis, |origin| {
            origin
                + Pixels::from(
                    content_offset + visible_range_start * ScrollOffset::from(text_unit_size),
                )
        });
        Bounds::new(
            thumb_origin,
            scrollbar_track.size.apply_along(axis, |_| thumb_size),
        )
    }

    fn thumb_hovered(&self, position: &gpui::Point<Pixels>) -> bool {
        self.thumb_bounds
            .is_some_and(|bounds| bounds.contains(position))
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
            let column_width = ((self.hitbox.size.width - Self::BORDER_WIDTH) / 3.0).floor();
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

        let row_to_y = |row: DisplayRow| row.as_f64() as f32 * self.text_unit_size;
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

struct MinimapLayout {
    pub minimap: AnyElement,
    pub thumb_layout: ScrollbarLayout,
    pub minimap_scroll_top: ScrollOffset,
    pub minimap_line_height: Pixels,
    pub thumb_border_style: MinimapThumbBorder,
    pub max_scroll_top: ScrollOffset,
    pub thumb_pixels_per_editor_line: Option<Pixels>,
}

impl MinimapLayout {
    /// The minimum width of the minimap in columns. If the minimap is smaller than this, it will be hidden.
    const MINIMAP_MIN_WIDTH_COLUMNS: f32 = 20.;
    /// The minimap width as a percentage of the editor width.
    const MINIMAP_WIDTH_PCT: f32 = 0.15;
    /// Calculates the scroll top offset the minimap editor has to have based on the
    /// current scroll progress.
    fn calculate_minimap_top_offset(
        document_lines: f64,
        visible_editor_lines: f64,
        visible_minimap_lines: f64,
        scroll_position: f64,
    ) -> ScrollOffset {
        let non_visible_document_lines = (document_lines - visible_editor_lines).max(0.);
        if non_visible_document_lines == 0. {
            0.
        } else {
            let scroll_percentage = (scroll_position / non_visible_document_lines).clamp(0., 1.);
            scroll_percentage * (document_lines - visible_minimap_lines).max(0.)
        }
    }

    /// How far the thumb moves on screen per scrolled editor line.
    ///
    /// The thumb moves along its oversized track, but the minimap content scrolls underneath
    /// it at the same time (see [`Self::calculate_minimap_top_offset`]), so what we actually
    /// see is the difference of the two. Returns `None` if the thumb can't move at all, e.g.
    /// because the whole document fits into the editor or the thumb covers the entire minimap
    /// (extremely unlikely, but may happen for very small editors that we currently allow)
    fn thumb_pixels_per_editor_line(
        thumb_layout: &ScrollbarLayout,
        document_lines: f64,
        minimap_line_height: Pixels,
    ) -> Option<Pixels> {
        let visible_editor_lines =
            thumb_layout.visible_range.end - thumb_layout.visible_range.start;
        let visible_minimap_lines = (thumb_layout.hitbox.size.height / minimap_line_height) as f64;
        let scrollable_editor_lines = document_lines - visible_editor_lines;
        if scrollable_editor_lines <= 0. {
            return None;
        }

        let minimap_scroll_per_editor_line =
            (document_lines - visible_minimap_lines).max(0.) / scrollable_editor_lines;
        let pixels_per_line = Pixels::from(
            ScrollOffset::from(thumb_layout.text_unit_size)
                - minimap_scroll_per_editor_line * ScrollOffset::from(minimap_line_height),
        );
        (pixels_per_line > Pixels::ZERO).then_some(pixels_per_line)
    }
}

struct CreaseTrailerLayout {
    element: AnyElement,
    bounds: Bounds<Pixels>,
}

#[derive(Clone)]
pub(crate) struct InputViewport {
    pub text_bounds: Bounds<Pixels>,
    pub content_origin: gpui::Point<Pixels>,
    pub visible_row_range: Range<DisplayRow>,
    pub scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
    pub scroll_max_y: ScrollOffset,
    pub line_height: Pixels,
    pub em_advance: Pixels,
    pub em_layout_width: Pixels,
}

pub(crate) struct PositionMap {
    pub size: Size<Pixels>,
    pub line_height: Pixels,
    pub scroll_position: gpui::Point<ScrollOffset>,
    pub scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
    pub scroll_max: gpui::Point<ScrollOffset>,
    pub horizontal_scroll_pending: bool,
    pointer_state: Rc<mouse::PointerState>,
    pub em_advance: Pixels,
    pub em_layout_width: Pixels,
    pub visible_row_range: Range<DisplayRow>,
    pub line_layouts: Vec<LineWithInvisibles>,
    pub snapshot: EditorSnapshot,
    pub text_align: TextAlign,
    pub content_width: Pixels,
    pub content_origin: gpui::Point<Pixels>,
    pub text_hitbox: Hitbox,
    pub gutter_hitbox: Hitbox,
    pub inline_blame_bounds: Option<(Bounds<Pixels>, BufferId, BlameEntry)>,
    pub display_hunks: Vec<(DisplayDiffHunk, Option<Hitbox>)>,
    pub diff_hunk_control_bounds: Vec<(DisplayRow, Bounds<Pixels>)>,
}

#[derive(Debug, Copy, Clone)]
pub struct PointForPosition {
    pub previous_valid: DisplayPoint,
    pub next_valid: DisplayPoint,
    pub nearest_valid: DisplayPoint,
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

    pub fn intersects_selection(&self, selection: &Selection<DisplayPoint>) -> bool {
        let Some(valid_point) = self.as_valid() else {
            return false;
        };
        let range = selection.range();

        let candidate_row = valid_point.row();
        let candidate_col = valid_point.column();

        let start_row = range.start.row();
        let start_col = range.start.column();
        let end_row = range.end.row();
        let end_col = range.end.column();

        if candidate_row < start_row || candidate_row > end_row {
            false
        } else if start_row == end_row {
            candidate_col >= start_col && candidate_col < end_col
        } else if candidate_row == start_row {
            candidate_col >= start_col
        } else if candidate_row == end_row {
            candidate_col < end_col
        } else {
            true
        }
    }

    fn new(
        snapshot: &EditorSnapshot,
        row: DisplayRow,
        column: u32,
        column_overshoot_after_line_end: u32,
    ) -> Self {
        let mut exact_unclipped = DisplayPoint::new(row, column);
        let previous_valid = snapshot.clip_point(exact_unclipped, Bias::Left);
        let next_valid = snapshot.clip_point(exact_unclipped, Bias::Right);
        let nearest_valid = if previous_valid == next_valid {
            previous_valid
        } else {
            match snapshot.inlay_bias_at(exact_unclipped) {
                Some(Bias::Left) => next_valid,
                Some(Bias::Right) => previous_valid,
                None => previous_valid,
            }
        };
        *exact_unclipped.column_mut() = column.saturating_add(column_overshoot_after_line_end);
        Self {
            previous_valid,
            next_valid,
            nearest_valid,
            exact_unclipped,
            column_overshoot_after_line_end,
        }
    }
}

impl PositionMap {
    pub(crate) fn inlay_hint_glyph_for_position(
        &self,
        position: gpui::Point<Pixels>,
    ) -> Option<DisplayPoint> {
        let position = position - self.content_origin;
        if position.y < Pixels::ZERO {
            return None;
        }
        let row = ((position.y / self.line_height) as f64 + self.scroll_position.y) as u32;
        let line_index = row.checked_sub(self.visible_row_range.start.0)?;
        let line = self.line_layouts.get(line_index as usize)?;
        let x = ScrollPixelOffset::from(position.x)
            + self.scroll_position.x * ScrollPixelOffset::from(self.em_layout_width)
            - line.alignment_offset(self.text_align, self.content_width);
        if x < 0. {
            return None;
        }
        let glyph = DisplayPoint::new(DisplayRow(row), line.index_for_x(x)? as u32);
        self.snapshot.inlay_hint_at(glyph).map(|_| glyph)
    }

    pub(crate) fn point_for_position(&self, position: gpui::Point<Pixels>) -> PointForPosition {
        let text_bounds = self.text_hitbox.bounds;
        let scroll_position = self.scroll_position;
        let position = position - text_bounds.origin;
        let y = position.y.max(px(0.)).min(self.size.height);
        let x = ScrollPixelOffset::from(position.x)
            + scroll_position.x * ScrollPixelOffset::from(self.em_layout_width);
        let row = ((y / self.line_height) as f64 + scroll_position.y) as u32;

        let line = row
            .checked_sub(self.visible_row_range.start.0)
            .and_then(|line_index| self.line_layouts.get(line_index as usize));
        self.resolve_point_for_position(DisplayRow(row), x, line)
    }

    fn text_is_hovered(&self, window: &Window) -> bool {
        self.text_hitbox.is_hovered(window)
    }

    fn gutter_is_hovered(&self, window: &Window) -> bool {
        self.gutter_hitbox.is_hovered(window)
    }

    fn point_for_position_on_line(
        &self,
        position: gpui::Point<Pixels>,
        row: DisplayRow,
        line: &LineWithInvisibles,
    ) -> PointForPosition {
        let text_bounds = self.text_hitbox.bounds;
        let scroll_position = self.scroll_position;
        let position = position - text_bounds.origin;
        let x = ScrollPixelOffset::from(position.x)
            + scroll_position.x * ScrollPixelOffset::from(self.em_layout_width);

        self.resolve_point_for_position(row, x, Some(line))
    }

    fn resolve_point_for_position(
        &self,
        row: DisplayRow,
        x: ScrollPixelOffset,
        line: Option<&LineWithInvisibles>,
    ) -> PointForPosition {
        let (column, x_overshoot_after_line_end) = if let Some(line) = line {
            let x_relative_to_text = x - line.alignment_offset(self.text_align, self.content_width);
            if let Some(ix) = line.index_for_x(x_relative_to_text) {
                (ix as u32, 0.)
            } else {
                (line.len as u32, (x_relative_to_text - line.width).max(0.))
            }
        } else {
            (0, x)
        };

        PointForPosition::new(
            &self.snapshot,
            row,
            column,
            (x_overshoot_after_line_end / ScrollPixelOffset::from(self.em_layout_width)) as u32,
        )
    }
}

const BLOCK_COORDINATE_MARGIN: ScrollPixelOffset = 1_048_576.;

const RENDERER_MEASURE_BUDGET_PER_PREPAINT: usize = 1024;

const RENDERER_VISIT_COST: usize = 1;

const RENDERER_LAYOUT_COST: usize = 31;

struct PrepaintWorkBudget {
    remaining: usize,
    exhausted: bool,
}

impl PrepaintWorkBudget {
    fn new(units: usize) -> Self {
        Self {
            remaining: units,
            exhausted: false,
        }
    }

    fn try_charge(&mut self, cost: usize) -> bool {
        match self.remaining.checked_sub(cost) {
            Some(remaining) => {
                self.remaining = remaining;
                true
            }
            None => {
                self.exhausted = true;
                false
            }
        }
    }
}

#[derive(Clone, Copy)]
enum BlockWidth {
    MinContent,
    Definite(ScrollPixelOffset),
}

impl BlockWidth {
    fn rebased(self, base: ScrollPixelOffset) -> AvailableSpace {
        match self {
            Self::MinContent => AvailableSpace::MinContent,
            Self::Definite(width) => AvailableSpace::Definite(Pixels::from(width - base)),
        }
    }
}

pub(crate) struct BlockLayout {
    pub(crate) id: BlockId,
    pub(crate) x_offset: ScrollPixelOffset,
    pub(crate) row: Option<DisplayRow>,
    pub(crate) element: AnyElement,
    pub(crate) available_space: Size<AvailableSpace>,
    pub(crate) style: BlockStyle,
    pub(crate) overlaps_gutter: bool,
    pub(crate) is_buffer_header: bool,
}

pub fn layout_line(
    row: DisplayRow,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    text_width: Pixels,
    scroll_x: ScrollPixelOffset,
    is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
    window: &mut Window,
    cx: &mut App,
) -> Result<LineWithInvisibles, PendingRowGeometry> {
    if let Some(long_row) = snapshot.long_row(row)? {
        if snapshot.mode.is_minimap() {
            return LineWithInvisibles::long_row_prefix(
                row,
                long_row.floor_column(MAX_LINE_LEN as u32),
                snapshot,
                style,
                text_width,
                &[],
                window,
                cx,
            );
        }

        let row_window =
            long_row.visible_window(scroll_x, style.text.text_align, f64::from(text_width));
        let paint_bounds = window.cover_bounds(Bounds::new(
            gpui::Point::default(),
            size(text_width, px(1.)),
        ));
        return LineWithInvisibles::long_row(
            long_row,
            row,
            snapshot,
            style,
            row_window,
            text_width,
            text_width,
            Pixels::ZERO,
            paint_bounds.left()..paint_bounds.right(),
            scroll_x,
            &[],
            window,
            cx,
        );
    }
    let language_aware = language_aware_styling(snapshot, row);
    let chunks = snapshot.highlighted_chunks(row..row + DisplayRow(1), language_aware, style);
    Ok(LineWithInvisibles::from_chunks(
        chunks,
        snapshot,
        row,
        style,
        MAX_LINE_LEN,
        1,
        &snapshot.mode,
        text_width,
        is_row_soft_wrapped,
        &[],
        window,
        cx,
    )?
    .pop()
    .unwrap_or_default())
}

#[derive(Debug, Clone)]
pub struct IndentGuideLayout {
    origin: gpui::Point<Pixels>,
    length: Pixels,
    single_indent_width: Pixels,
    display_row_range: Range<DisplayRow>,
    depth: u32,
    active: bool,
    settings: IndentGuideSettings,
}

enum NavigationOverlayPaintCommand {
    Label(NavigationLabelLayout),
}

struct NavigationLabelLayout {
    element: AnyElement,
    #[cfg_attr(not(test), allow(dead_code))]
    origin: gpui::Point<Pixels>,
}

struct NavigationOverlayLayoutContext<'a> {
    display_snapshot: &'a DisplaySnapshot,
    visible_display_row_range: &'a Range<DisplayRow>,
    line_layouts: &'a [LineWithInvisibles],
    text_align: TextAlign,
    content_width: Pixels,
    content_origin: gpui::Point<Pixels>,
    scroll_position: gpui::Point<ScrollOffset>,
    scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
    line_height: Pixels,
    editor_font: Font,
    editor_font_size: Pixels,
}

const LABEL_LINE_HEIGHT_PADDING_PX: f32 = 2.0;

pub struct CursorLayout {
    origin: gpui::Point<Pixels>,
    block_width: Pixels,
    line_height: Pixels,
    color: Hsla,
    shape: CursorShape,
    block_text: Option<ShapedLine>,
    retained_block_text: RetainedCursorText,
    cursor_name: Option<AnyElement>,
    animated_corners: Option<[gpui::Point<Pixels>; 4]>,
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
            retained_block_text: RetainedCursorText::default(),
            cursor_name: None,
            animated_corners: None,
        }
    }

    pub fn bounding_rect(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin + origin,
            size: size(self.block_width, self.line_height),
        }
    }

    fn bounds(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        Self::shape_bounds(
            self.origin + origin,
            self.block_width,
            self.line_height,
            self.shape,
        )
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
                .line_height(text_size + px(LABEL_LINE_HEIGHT_PADDING_PX))
                .text_color(cursor_name.color)
                .child(cursor_name.string)
                .into_any_element();

            name_element.prepaint_as_root(name_origin, AvailableSpace::min_size(), window, cx);

            self.cursor_name = Some(name_element);
        }
    }

    pub fn paint(&mut self, origin: gpui::Point<Pixels>, window: &mut Window, cx: &mut App) {
        let bounds = window.pixel_snap_bounds(self.bounds(origin));

        if let Some(corners) = self.animated_corners {
            let mut builder = gpui::PathBuilder::fill();
            builder.add_polygon(&corners, true);
            if let Ok(path) = builder.build() {
                if let Some(name) = &mut self.cursor_name {
                    name.paint(window, cx);
                }
                window.paint_path(path, self.color);

                if !animated_corners_overlap_target(bounds, &corners) {
                    return;
                }
            }
        } else {
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
        }

        if !self.retained_block_text.fragments.is_empty() {
            window.with_content_mask(Some(ContentMask { bounds }), |window| {
                for (offset, text) in &self.retained_block_text.fragments {
                    text.paint(
                        self.origin + origin + point(*offset, Pixels::ZERO),
                        self.line_height,
                        TextAlign::Left,
                        None,
                        window,
                        cx,
                    )
                    .log_err();
                }
            });
        }
        if let Some(block_text) = &self.block_text {
            block_text
                .paint(
                    self.origin + origin,
                    self.line_height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .log_err();
        }
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }

    pub(crate) fn cell_width(
        x: ScrollPixelOffset,
        next_x: ScrollPixelOffset,
        em_advance: Pixels,
    ) -> Pixels {
        let width = Pixels::from(next_x - x);
        if width == Pixels::ZERO {
            em_advance
        } else {
            width
        }
    }

    pub(crate) fn shape_bounds(
        origin: gpui::Point<Pixels>,
        block_width: Pixels,
        line_height: Pixels,
        shape: CursorShape,
    ) -> Bounds<Pixels> {
        match shape {
            CursorShape::Bar => Bounds::new(origin, size(px(2.), line_height)),
            CursorShape::Block | CursorShape::Hollow => {
                Bounds::new(origin, size(block_width, line_height))
            }
            CursorShape::Underline => Bounds::new(
                origin + point(Pixels::ZERO, line_height - px(2.)),
                size(block_width, px(2.)),
            ),
        }
    }

    pub(crate) fn long_row_width(
        width: &mut Pixels,
        snapshot: &DisplaySnapshot,
        row: &LongRow,
        cursor: DisplayPoint,
        font_size: Pixels,
        style: &TextStyle,
        window: &Window,
        cx: &App,
    ) -> Result<(), PendingRowGeometry> {
        let start = row.floor_column(cursor.column());
        if start == row.len() {
            return Ok(());
        }
        let prepared = snapshot
            .prepared_row_text(cursor.row())?
            .ok_or_else(|| snapshot.pending_row(cursor.row(), true))?;
        if let Some(intrinsic) = prepared.oversized_cursor_width(start) {
            let intrinsic = intrinsic.ok_or_else(|| snapshot.pending_row(cursor.row(), false))?;
            *width = (*width).max(intrinsic);
        } else {
            let limit = start
                .saturating_add(PreparedRowText::CURSOR_TEXT_LIMIT)
                .min(row.len());
            let end = row.floor_column(limit);
            let end = if end > start { end } else { limit };
            let Some(text) = prepared.cursor_text(start..end) else {
                return Ok(());
            };
            Self::shape_text(
                width,
                text,
                row.font_id_for_column(start, &(start..end)),
                font_size,
                None,
                style,
                window,
                cx,
            );
        }
        Ok(())
    }

    pub(crate) fn text_font(
        font_id: Option<FontId>,
        style: &TextStyle,
        text_system: &TextSystem,
    ) -> Font {
        let mut font = font_id
            .and_then(|font_id| text_system.get_font_for_id(font_id))
            .unwrap_or_else(|| style.font());
        font.features = style.font_features.clone();
        font
    }

    pub(crate) fn shape_text(
        width: &mut Pixels,
        text: SharedString,
        font_id: Option<FontId>,
        font_size: Pixels,
        metrics: Option<RowMetrics>,
        style: &TextStyle,
        window: &Window,
        cx: &App,
    ) -> ShapedLine {
        let is_ascii_whitespace_only = text
            .chars()
            .all(|character| character.is_ascii_whitespace());
        let len = text.len();
        let font = Self::text_font(font_id, style, window.text_system());

        let color = Self::text_color(cx);
        let mut shaped = window.text_system().shape_line(
            text,
            font_size,
            &[TextRun {
                len,
                font,
                color,
                ..TextRun::default()
            }],
            None,
        );
        if let Some(metrics) = metrics {
            apply_row_metrics(&mut shaped, metrics);
        }
        if !is_ascii_whitespace_only {
            *width = (*width).max(shaped.width);
        }
        shaped
    }

    fn text_color(cx: &App) -> Hsla {
        // Invert the text color for the block cursor. Ensure that the text
        // color is opaque enough to be visible against the background color.
        //
        // 0.75 is an arbitrary threshold to determine if the background color is
        // opaque enough to use as a text color.
        //
        // TODO: In the future we should ensure themes have a `text_inverse` color.
        if cx.theme().colors().editor_background.a < 0.75 {
            match cx.theme().appearance {
                Appearance::Dark => Hsla::black(),
                Appearance::Light => Hsla::white(),
            }
        } else {
            cx.theme().colors().editor_background
        }
    }
}

#[derive(Default)]
struct RetainedCursorText {
    fragments: SmallVec<[(Pixels, ShapedLine); 1]>,
}

impl RetainedCursorText {
    fn new<'a>(
        fragments: impl Iterator<Item = (&'a LineFragment, usize, ScrollPixelOffset)>,
        cursor_x: ScrollPixelOffset,
        cursor_origin: Pixels,
        cursor_bounds: Range<Pixels>,
        color: Hsla,
        mut font_bounds: impl FnMut(FontId, Pixels) -> Bounds<Pixels>,
    ) -> Self {
        Self {
            fragments: fragments
                .filter_map(|(fragment, _, origin)| {
                    let LineFragment::Text(line) = fragment else {
                        return None;
                    };
                    let offset = Pixels::from(origin - cursor_x);
                    let runs = line
                        .runs
                        .iter()
                        .filter_map(|run| {
                            let bounds = font_bounds(run.font_id, line.font_size);
                            let glyphs = run
                                .glyphs
                                .iter()
                                .filter(|glyph| {
                                    let left =
                                        cursor_origin + offset + glyph.position.x + bounds.origin.x;
                                    let right = left + bounds.size.width;
                                    left <= cursor_bounds.end && right >= cursor_bounds.start
                                })
                                .cloned()
                                .collect::<Vec<_>>();
                            (!glyphs.is_empty()).then_some(ShapedRun {
                                font_id: run.font_id,
                                glyphs,
                            })
                        })
                        .collect::<Vec<_>>();
                    if runs.is_empty() {
                        return None;
                    }
                    let layout = Arc::new(LineLayout {
                        font_size: line.font_size,
                        width: line.width,
                        ascent: line.ascent,
                        descent: line.descent,
                        len: line.len(),
                        runs,
                        caret_positions: line.caret_positions.clone(),
                    });
                    Some((
                        offset,
                        ShapedLine::new(
                            layout,
                            line.text.clone(),
                            &[TextRun {
                                len: line.len(),
                                color,
                                ..TextRun::default()
                            }],
                        ),
                    ))
                })
                .collect(),
        }
    }
}

fn cursor_shape_supports_cursor_animation(shape: CursorShape) -> bool {
    matches!(shape, CursorShape::Bar | CursorShape::Block)
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
    pub fn paint(&self, fill: bool, bounds: Bounds<Pixels>, window: &mut Window) {
        if self.lines.len() >= 2 && self.lines[0].start_x > self.lines[1].end_x {
            self.paint_lines(self.start_y, &self.lines[0..1], fill, bounds, window);
            self.paint_lines(
                self.start_y + self.line_height,
                &self.lines[1..],
                fill,
                bounds,
                window,
            );
        } else {
            self.paint_lines(self.start_y, &self.lines, fill, bounds, window);
        }
    }

    fn paint_lines(
        &self,
        start_y: Pixels,
        lines: &[HighlightedRangeLine],
        fill: bool,
        bounds: Bounds<Pixels>,
        window: &mut Window,
    ) {
        if lines.is_empty() {
            return;
        }

        let margin = self.corner_radius * 2. + px(1.);
        let clamp_x = |x: Pixels| x.max(bounds.left() - margin).min(bounds.right() + margin);
        let lines = lines
            .iter()
            .map(|line| HighlightedRangeLine {
                start_x: clamp_x(line.start_x),
                end_x: clamp_x(line.end_x),
            })
            .collect::<SmallVec<[HighlightedRangeLine; 4]>>();
        let (Some(first_line), Some(last_line)) = (lines.first(), lines.last()) else {
            return;
        };

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
        let mut builder = if fill {
            gpui::PathBuilder::fill()
        } else {
            gpui::PathBuilder::stroke(px(1.))
        };
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

pub fn register_action<T: Action>(
    editor: &Entity<Editor>,
    window: &mut Window,
    listener: impl Fn(&mut Editor, &T, &mut Window, &mut Context<Editor>) + 'static,
) {
    register_action_internal(editor, false, window, listener);
}

fn register_recovery_action<T: Action>(
    editor: &Entity<Editor>,
    window: &mut Window,
    listener: impl Fn(&mut Editor, &T, &mut Window, &mut Context<Editor>) + 'static,
) {
    register_action_internal(editor, true, window, listener);
}

fn register_action_internal<T: Action>(
    editor: &Entity<Editor>,
    recovery: bool,
    window: &mut Window,
    listener: impl Fn(&mut Editor, &T, &mut Window, &mut Context<Editor>) + 'static,
) {
    let editor = editor.clone();
    let listener = Rc::new(listener);
    window.on_action(TypeId::of::<T>(), move |action, phase, window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        let Some(action) = action.downcast_ref::<T>() else {
            cx.propagate();
            return;
        };
        editor.update(cx, |editor, cx| {
            let deferred_editor = cx.weak_entity();
            let deferred_listener = listener.clone();
            if editor.defer_action_dispatch(
                action,
                recovery,
                window,
                cx,
                move |action, window, cx| {
                    if deferred_editor
                        .update(cx, |editor, cx| {
                            deferred_listener(editor, action, window, cx)
                        })
                        .is_err()
                    {
                        cx.propagate();
                    }
                },
            ) {
                return;
            }
            listener(editor, action, window, cx);
        });
    });
}

/// Shared between `prepaint` and `compute_auto_height_layout` to ensure
/// both full and auto-height editors compute wrap widths consistently.
fn calculate_wrap_width(
    soft_wrap: SoftWrap,
    masked: bool,
    editor_width: Pixels,
    em_width: Pixels,
) -> Option<Pixels> {
    let wrap_width_for = |column: u32| (column as f32 * em_width).ceil();

    match soft_wrap {
        SoftWrap::None if masked => Some(wrap_width_for(MAX_LINE_LEN as u32 / 2)),
        SoftWrap::None => None,
        SoftWrap::EditorWidth => Some(editor_width),
        SoftWrap::Bounded(column) => Some(editor_width.min(wrap_width_for(column))),
    }
}

fn compute_auto_height_layout(
    editor: &mut Editor,
    min_lines: usize,
    max_lines: Option<usize>,
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
    let gutter_dimensions = snapshot.gutter_dimensions(font_id, font_size, style, window, cx);

    editor.gutter_dimensions = gutter_dimensions;
    let text_width = width - gutter_dimensions.width;
    let overscroll = size(em_width, px(0.));

    let editor_width = text_width - gutter_dimensions.margin - overscroll.width - em_width;
    let wrap_width = calculate_wrap_width(
        editor.soft_wrap_mode(cx),
        editor.display_map.read(cx).masked,
        editor_width,
        em_width,
    )
    .unwrap_or(editor_width)
    .min(editor_width);
    if editor.set_wrap_width(Some(wrap_width), cx) {
        snapshot = editor.snapshot(window, cx);
    }

    let scroll_height = (snapshot.max_point().row().next_row().0 as f32) * line_height;

    let min_height = line_height * min_lines as f32;
    let content_height = scroll_height.max(min_height);

    let final_height = if let Some(max_lines) = max_lines {
        let max_height = line_height * max_lines as f32;
        content_height.min(max_height)
    } else {
        content_height
    };

    Some(size(width, final_height))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    use crate::test::NativeClipboardCapture;
    use crate::{
        Editor, EditorEvent, FoldPlaceholder, HighlightKey, Inlay, MoveDown, MoveUp, MultiBuffer,
        NavigationOverlayKey, NavigationOverlayLabel, NavigationTargetOverlay, ScrollAnchor,
        SelectionEffects, ToggleSoftWrap,
        display_map::{BlockPlacement, BlockProperties, DisplayMap, RowLayout},
        editor_tests::{init_test, update_test_editor_settings, update_test_language_settings},
        test::ShapingTextSystem,
    };
    use buffer_diff::BufferDiff;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    use gpui::{ClipboardReadError, ClipboardSnapshot, ClipboardSnapshotRead};
    use gpui::{
        FontStyle, GlyphId, HighlightStyle, Modifiers, PlatformInput, Render, ScrollDelta,
        ScrollWheelEvent, ShapedGlyph, ShapedRun, StrikethroughStyle, TestAppContext, TouchPhase,
        Underline, UpdateGlobal, VisualTestContext, WindowHandle, font,
    };
    use language::{
        Buffer, Capability, Diagnostic, DiagnosticEntry, DiagnosticSet, SelectionGoal,
        language_settings, tree_sitter_python,
    };
    use log::info;
    use lsp::DiagnosticSeverity;
    use multi_buffer::PathKey;
    use parking_lot::Mutex;
    use rand::{RngCore, rngs::StdRng};
    use std::{
        cell::RefCell,
        num::NonZeroU32,
        sync::atomic::{AtomicUsize, Ordering as AtomicOrdering},
    };
    use text::PointUtf16;
    use util::test::sample_text;

    gpui::actions!(element_input_test, [DeferredTypedAction, RecoveryUndo]);

    struct MinimapGeometry {
        document_lines: f64,
        visible_editor_lines: f64,
        minimap_line_height: Pixels,
        minimap_height: Pixels,
    }

    impl MinimapGeometry {
        fn visible_minimap_lines(&self) -> f64 {
            (self.minimap_height / self.minimap_line_height) as f64
        }

        fn thumb_layout(&self, scroll_position: f64) -> ScrollbarLayout {
            let bounds = Bounds::new(point(px(0.), px(0.)), size(px(100.), self.minimap_height));
            let hitbox = Hitbox {
                id: gpui::HitboxId::placeholder(),
                bounds,
                content_mask: gpui::ContentMask { bounds },
                behavior: HitboxBehavior::Normal,
            };
            let minimap_scroll_top = MinimapLayout::calculate_minimap_top_offset(
                self.document_lines,
                self.visible_editor_lines,
                self.visible_minimap_lines(),
                scroll_position,
            );
            ScrollbarLayout::for_minimap(
                hitbox,
                self.visible_editor_lines,
                self.document_lines,
                self.minimap_line_height,
                scroll_position,
                minimap_scroll_top,
                true,
            )
        }

        fn thumb_movement_for_drag(&self, scroll_position: f64, mouse_delta: Pixels) -> Pixels {
            let layout = self.thumb_layout(scroll_position);
            let pixels_per_line = self
                .thumb_pixels_per_editor_line(&layout)
                .expect("thumb should be draggable");

            let new_scroll_position =
                scroll_position + ScrollPixelOffset::from(mouse_delta / pixels_per_line);

            let old_top = layout.thumb_bounds.expect("thumb should be shown").origin.y;
            let new_top = self
                .thumb_layout(new_scroll_position)
                .thumb_bounds
                .expect("thumb should be shown")
                .origin
                .y;
            new_top - old_top
        }

        fn thumb_pixels_per_editor_line(&self, layout: &ScrollbarLayout) -> Option<Pixels> {
            MinimapLayout::thumb_pixels_per_editor_line(
                layout,
                self.document_lines,
                self.minimap_line_height,
            )
        }

        #[track_caller]
        fn assert_thumb_follows_drag(&self, scroll_position: f64) {
            let mouse_delta = px(10.);
            let movement = self.thumb_movement_for_drag(scroll_position, mouse_delta);
            assert!(
                (movement - mouse_delta).abs() < px(0.01),
                "dragging by {mouse_delta:?} from scroll position {scroll_position} moved the thumb by {movement:?}",
            );
        }
    }

    #[test]
    fn test_minimap_thumb_follows_drag_in_short_document() {
        let geometry = MinimapGeometry {
            document_lines: 100.,
            visible_editor_lines: 40.,
            minimap_line_height: px(2.),
            minimap_height: px(800.),
        };
        geometry.assert_thumb_follows_drag(0.);
        geometry.assert_thumb_follows_drag(30.);
    }

    #[test]
    fn test_minimap_thumb_follows_drag_in_long_document() {
        let geometry = MinimapGeometry {
            document_lines: 2000.,
            visible_editor_lines: 40.,
            minimap_line_height: px(2.),
            minimap_height: px(800.),
        };
        geometry.assert_thumb_follows_drag(0.);
        geometry.assert_thumb_follows_drag(500.);
    }

    #[test]
    fn test_minimap_thumb_follows_drag_with_minimum_thumb_size() {
        // 10 lines at 2px each would only be a 20px thumb, so this gets bumped up to
        // `ScrollbarLayout::MIN_THUMB_SIZE`
        let short_document = MinimapGeometry {
            document_lines: 80.,
            visible_editor_lines: 10.,
            minimap_line_height: px(2.),
            minimap_height: px(200.),
        };
        short_document.assert_thumb_follows_drag(0.);
        short_document.assert_thumb_follows_drag(30.);

        let long_document = MinimapGeometry {
            document_lines: 2000.,
            ..short_document
        };
        long_document.assert_thumb_follows_drag(0.);
        long_document.assert_thumb_follows_drag(500.);
    }

    #[test]
    fn test_minimap_thumb_not_draggable_without_room_to_move() {
        let fits_in_editor = MinimapGeometry {
            document_lines: 30.,
            visible_editor_lines: 40.,
            minimap_line_height: px(2.),
            minimap_height: px(800.),
        };
        assert_eq!(
            fits_in_editor.thumb_pixels_per_editor_line(&fits_in_editor.thumb_layout(0.)),
            None
        );

        // 10 lines at 2px each is a 20px track, so the thumb (at least 25px) already covers all of it
        let thumb_fills_track = MinimapGeometry {
            document_lines: 10.,
            visible_editor_lines: 5.,
            minimap_line_height: px(2.),
            minimap_height: px(800.),
        };
        assert_eq!(
            thumb_fills_track.thumb_pixels_per_editor_line(&thumb_fills_track.thumb_layout(0.)),
            None
        );
    }

    enum PrimaryNavigationOverlay {}

    const PRIMARY_NAVIGATION_OVERLAY_KEY: NavigationOverlayKey =
        NavigationOverlayKey::unique::<PrimaryNavigationOverlay>();

    fn navigation_overlay(
        label_text: &'static str,
        target_range: Range<Anchor>,
        covered_text_range: Option<Range<Anchor>>,
    ) -> NavigationTargetOverlay {
        NavigationTargetOverlay {
            target_range,
            label: NavigationOverlayLabel {
                text: SharedString::from(label_text),
                text_color: Hsla::black(),
                x_offset: Pixels::ZERO,
                scale_factor: 1.0,
            },
            covered_text_range,
        }
    }

    fn navigation_label_layouts(state: &EditorLayout) -> Vec<&NavigationLabelLayout> {
        state
            .navigation_overlay_paint_commands
            .iter()
            .map(|command| match command {
                NavigationOverlayPaintCommand::Label(label) => label,
            })
            .collect()
    }

    const fn placeholder_hitbox() -> Hitbox {
        use gpui::HitboxId;
        let zero_bounds = Bounds {
            origin: point(Pixels::ZERO, Pixels::ZERO),
            size: Size {
                width: Pixels::ZERO,
                height: Pixels::ZERO,
            },
        };

        Hitbox {
            id: HitboxId::placeholder(),
            bounds: zero_bounds,
            content_mask: ContentMask {
                bounds: zero_bounds,
            },
            behavior: HitboxBehavior::Normal,
        }
    }

    fn test_gutter(line_height: Pixels, snapshot: &EditorSnapshot) -> Gutter<'_> {
        const DIMENSIONS: GutterDimensions = GutterDimensions {
            left_padding: Pixels::ZERO,
            right_padding: Pixels::ZERO,
            width: px(30.0),
            margin: Pixels::ZERO,
            git_blame_entries_width: None,
        };
        const EMPTY_ROW_INFO: RowInfo = RowInfo {
            buffer_id: None,
            buffer_row: None,
            multibuffer_row: None,
            diff_status: None,
            expand_info: None,
            wrapped_buffer_row: None,
        };

        const fn row_info(row: u32) -> RowInfo {
            RowInfo {
                buffer_row: Some(row),
                ..EMPTY_ROW_INFO
            }
        }

        const ROW_INFOS: [RowInfo; 6] = [
            row_info(0),
            row_info(1),
            row_info(2),
            row_info(3),
            row_info(4),
            row_info(5),
        ];

        const HITBOX: Hitbox = placeholder_hitbox();
        Gutter {
            line_height,
            range: DisplayRow(0)..DisplayRow(6),
            scroll_position: gpui::Point::default(),
            dimensions: &DIMENSIONS,
            hitbox: &HITBOX,
            snapshot: snapshot,
            row_infos: &ROW_INFOS,
        }
    }

    #[gpui::test]
    async fn test_deferred_typed_action_preserves_remaining_handlers(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context("", cx);
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = editor.update(cx, |editor, cx| {
            let events = events.clone();
            let subscription = editor.register_action_renderer(move |_, window, cx| {
                window.on_action(TypeId::of::<DeferredTypedAction>(), {
                    let events = events.clone();
                    move |_, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            events.borrow_mut().push("visited");
                            cx.propagate();
                        }
                    }
                });
                register_action(&cx.entity(), window, {
                    let events = events.clone();
                    move |editor, _: &DeferredTypedAction, window, cx| {
                        events.borrow_mut().push("typed");
                        editor.handle_input("A", window, cx);
                        cx.propagate();
                    }
                });
                window.on_action(TypeId::of::<DeferredTypedAction>(), {
                    let editor = cx.weak_entity();
                    let events = events.clone();
                    move |_, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            editor
                                .update(cx, |editor, cx| {
                                    events.borrow_mut().push("remaining");
                                    editor.handle_input("B", window, cx);
                                })
                                .expect("remaining handler can borrow the editor");
                            cx.propagate();
                        }
                    }
                });
            });
            cx.notify();
            subscription
        });
        cx.update(|window, cx| {
            cx.on_action({
                let events = events.clone();
                move |_: &DeferredTypedAction, cx| {
                    events.borrow_mut().push("global");
                    cx.propagate();
                }
            });
            window.draw(cx).clear(cx);
        });

        let text = format!("ab\n{}", "x".repeat(2_000));
        let completion = cx.update(|window, cx| {
            let focus = editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                editor.set_text(text.clone(), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred(cx));
                editor.focus_handle.clone()
            });
            focus.dispatch_action(&DeferredTypedAction, window, cx);
            assert_eq!(*events.borrow(), ["visited"]);
            editor.update(cx, |editor, cx| {
                editor.handle_input("C", window, cx);
                assert_eq!(editor.text(cx), text);
                editor.input_completion(cx)
            })
        });
        completion
            .await
            .expect("deferred action and following input complete");
        assert_eq!(
            *events.borrow(),
            ["visited", "typed", "remaining", "global"]
        );
        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), format!("ab\nxABC{}", "x".repeat(1_999)));
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(1, 4)
            );
        });
    }

    #[gpui::test]
    async fn test_cold_action_and_queued_key_preserve_propagation(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.hard_tabs = Some(true);
            settings.defaults.tab_size = NonZeroU32::new(4);
        });
        let (editor, ref mut cx) = editor_test_context("", cx);
        let events = Rc::new(RefCell::new(Vec::new()));
        cx.update(|window, cx| {
            cx.bind_keys([
                gpui::KeyBinding::new("tab", crate::Tab, Some("Editor")),
                gpui::KeyBinding::new("tab", crate::NextSnippetTabstop, Some("Editor")),
            ]);
            cx.on_action({
                let events = events.clone();
                move |_: &MoveUp, cx| {
                    events.borrow_mut().push("up");
                    cx.propagate();
                }
            });
            editor.focus_handle(cx).focus(window, cx);
            window.draw(cx).clear(cx);
        });
        let text = "x".repeat(2_000);
        let completion = cx.update(|window, cx| {
            let focus = editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                editor.set_text(text.clone(), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::zero()..Point::zero()]);
                });
                assert_eq!(
                    editor
                        .selections
                        .newest::<Point>(&editor.display_snapshot(cx))
                        .head(),
                    Point::zero()
                );
                editor.focus_handle.clone()
            });
            focus.dispatch_action(&MoveUp, window, cx);
            editor.read_with(cx, |editor, cx| assert!(editor.input_is_deferred(cx)));
            assert_eq!(*events.borrow(), Vec::<&str>::new());
            window.dispatch_keystroke(gpui::Keystroke::parse("tab").expect("keystroke"), cx);
            editor.update(cx, |editor, cx| {
                editor.handle_input("Z", window, cx);
                assert_eq!(editor.text(cx), text);
                editor.input_completion(cx)
            })
        });
        completion.await.expect("deferred commands complete");
        assert_eq!(*events.borrow(), ["up"]);
        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), format!("\tZ{text}"));
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(0, 2)
            );
        });
    }

    // Regression test for https://github.com/zed-industries/zed/issues/48141.
    #[gpui::test]
    fn test_selection_layout_around_inlay(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let snapshot = cx.update(|cx| {
            let buffer = MultiBuffer::build_simple("abcd", cx);
            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            let display_map = cx.new(|cx| {
                DisplayMap::new(
                    buffer,
                    font("Helvetica"),
                    px(14.0),
                    None,
                    1,
                    1,
                    FoldPlaceholder::test(),
                    project::project_settings::DiagnosticSeverity::Warning,
                    cx,
                )
            });
            display_map.update(cx, |display_map, cx| {
                display_map.splice_inlays(
                    &[],
                    vec![
                        Inlay::mock_hint(
                            0,
                            buffer_snapshot.anchor_before(MultiBufferOffset(1)),
                            "hint ",
                        ),
                        Inlay::mock_hint(
                            1,
                            buffer_snapshot.anchor_after(MultiBufferOffset(3)),
                            "!!",
                        ),
                    ],
                    cx,
                );
                display_map.snapshot(cx)
            })
        });

        let layout = |range: Range<MultiBufferOffset>, reversed, cursor_offset| {
            SelectionLayout::new(
                Selection {
                    id: 0,
                    start: range.start,
                    end: range.end,
                    reversed,
                    goal: SelectionGoal::None,
                },
                false,
                cursor_offset,
                CursorShape::Bar,
                &snapshot,
                true,
                true,
                None,
            )
        };
        let display_point = |column| DisplayPoint::new(DisplayRow(0), column);

        let ending_at_inlay = layout(MultiBufferOffset(0)..MultiBufferOffset(1), false, false);
        assert_eq!(ending_at_inlay.range, display_point(0)..display_point(1));
        assert_eq!(ending_at_inlay.head, display_point(1));

        let starting_at_inlay = layout(MultiBufferOffset(1)..MultiBufferOffset(2), false, false);
        assert_eq!(starting_at_inlay.range, display_point(6)..display_point(7));
        assert_eq!(starting_at_inlay.head, display_point(7));

        let spanning_inlay = layout(MultiBufferOffset(0)..MultiBufferOffset(2), false, false);
        assert_eq!(spanning_inlay.range, display_point(0)..display_point(7));
        assert_eq!(spanning_inlay.head, display_point(7));

        let reversed = layout(MultiBufferOffset(0)..MultiBufferOffset(2), true, false);
        assert_eq!(reversed.range, display_point(0)..display_point(7));
        assert_eq!(reversed.head, display_point(0));

        // A reversed selection starting at a right-anchored hint: the
        // anchor-bias cursor position would sit before the hint, detached from
        // the highlight, so the head snaps to the highlight boundary instead.
        let reversed_at_right_anchored_inlay =
            layout(MultiBufferOffset(3)..MultiBufferOffset(4), true, false);
        assert_eq!(
            reversed_at_right_anchored_inlay.range,
            display_point(10)..display_point(11)
        );
        assert_eq!(reversed_at_right_anchored_inlay.head, display_point(10));

        // An empty selection is a typing position, so it keeps the anchor-bias
        // display position rather than snapping around boundary inlays.
        let empty = layout(MultiBufferOffset(1)..MultiBufferOffset(1), false, false);
        assert!(empty.range.is_empty());
        assert_eq!(empty.head, display_point(6));

        let vim_ending_at_inlay = layout(MultiBufferOffset(0)..MultiBufferOffset(1), false, true);
        assert_eq!(
            vim_ending_at_inlay.range,
            display_point(0)..display_point(1)
        );
        assert_eq!(vim_ending_at_inlay.head, display_point(0));

        let vim_spanning_inlay = layout(MultiBufferOffset(0)..MultiBufferOffset(2), false, true);
        assert_eq!(vim_spanning_inlay.range, display_point(0)..display_point(7));
        assert_eq!(vim_spanning_inlay.head, display_point(6));

        let vim_reversed_at_right_anchored_inlay =
            layout(MultiBufferOffset(3)..MultiBufferOffset(4), true, true);
        assert_eq!(
            vim_reversed_at_right_anchored_inlay.range,
            display_point(10)..display_point(11)
        );
        assert_eq!(vim_reversed_at_right_anchored_inlay.head, display_point(10));
    }

    #[gpui::test]
    fn test_unrepresentable_rows_show_unavailable_and_recover(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(128);
        });
        let (editor, ref mut cx) = editor_test_context("A", cx);
        let _recovery = editor.update(cx, |editor, cx| {
            let handle = cx.weak_entity();
            editor.register_recovery_action_with_context(
                |_| (),
                move |(), _: &RecoveryUndo, window, cx| {
                    handle
                        .update(cx, |editor, cx| editor.undo(&crate::Undo, window, cx))
                        .expect("editor");
                },
            )
        });
        let _ordinary = editor.update(cx, |editor, _| {
            editor.register_action(|_: &DeferredTypedAction, _, _| {
                panic!("ordinary addon action ran without geometry");
            })
        });
        let style = editor.update_in(cx, |editor, window, cx| {
            editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            editor.display_map.update(cx, |map, cx| {
                map.set_tab_column_limit(32_767, cx);
            });
            editor.focus_handle.focus(window, cx);
            editor.style(cx).clone()
        });
        draw_ready_editor(&editor, &style, cx);
        let text = format!("A{}", "\t".repeat(256));
        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.last_position_map.is_some());
            editor.transact(window, cx, |editor, _, cx| {
                editor.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(
                            MultiBufferOffset(1)..MultiBufferOffset(1),
                            text[1..].to_string(),
                        )],
                        None,
                        cx,
                    );
                });
            });
        });
        cx.update(|window, cx| {
            editor
                .focus_handle(cx)
                .dispatch_action(&DeferredTypedAction, window, cx);
            assert_eq!(editor.read(cx).text(cx), text);
        });
        for _ in 0..2 {
            assert!(matches!(
                draw_editor_frame(&editor, &style, cx),
                EditorPrepaintState::Unavailable { .. }
            ));
            editor.read_with(cx, |editor, cx| {
                assert_eq!(editor.text(cx), text);
                assert!(editor.last_position_map.is_none());
                assert!(editor.last_input_viewport.is_none());
                assert!(editor.pixel_position_of_newest_cursor.is_none());
                assert!(!editor.input_is_deferred(cx));
            });
        }
        cx.update(|window, cx| {
            let focus = editor.focus_handle(cx);
            focus.dispatch_action(&crate::Delete, window, cx);
            focus.dispatch_action(&DeferredTypedAction, window, cx);
            assert_eq!(editor.read(cx).text(cx), text);
            focus.dispatch_action(&RecoveryUndo, window, cx);
            assert_eq!(editor.read(cx).text(cx), "A");
        });
        draw_ready_editor(&editor, &style, cx);
        cx.update(|window, cx| {
            editor
                .focus_handle(cx)
                .dispatch_action(&crate::Redo, window, cx);
            assert_eq!(editor.read(cx).text(cx), text);
        });
        assert!(matches!(
            draw_editor_frame(&editor, &style, cx),
            EditorPrepaintState::Unavailable { .. }
        ));
        update_test_language_settings(cx, &|settings| {
            settings.defaults.tab_size = NonZeroU32::new(64);
        });
        let layout = draw_ready_editor(&editor, &style, cx);
        assert_eq!(layout.position_map.snapshot.line_len(DisplayRow(0)), 16_384);
        editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.text(cx), text);
            assert!(editor.last_position_map.is_some());
            assert!(editor.last_input_viewport.is_some());
        });
    }

    #[gpui::test]
    async fn test_soft_wrap_editor_width_auto_height_editor(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&"a ".to_string().repeat(100), cx);
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: None,
                },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));

        for x in 1..=100 {
            let (_, state) = cx.draw(
                Default::default(),
                size(px(200. + 0.13 * x as f32), px(500.)),
                |_, _| EditorElement::new(&editor, style.clone()),
            );
            let state = ready_layout(state);

            assert!(
                state.position_map.scroll_max.x == 0.,
                "Soft wrapped editor should have no horizontal scrolling!"
            );
        }
    }

    #[gpui::test]
    async fn test_auto_height_editor_rewraps_after_soft_wrap_toggle(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.soft_wrap = Some(language_settings::SoftWrap::EditorWidth);
        });
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&"a ".repeat(100), cx);
            Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: None,
                },
                buffer,
                None,
                window,
                cx,
            )
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));
        let display_rows = |width: f32, cx: &mut VisualTestContext| {
            let (_, state) = cx.draw(Default::default(), size(px(width), px(500.)), |_, _| {
                EditorElement::new(&editor, style.clone())
            });
            let state = ready_layout(state);
            state.position_map.snapshot.max_point().row()
        };

        let narrow_rows = display_rows(200., cx);
        editor.update_in(cx, |editor, window, cx| {
            editor.toggle_soft_wrap(&ToggleSoftWrap, window, cx)
        });
        assert_eq!(display_rows(200., cx), narrow_rows);
        assert!(display_rows(400., cx) < narrow_rows);
    }

    #[gpui::test]
    fn test_point_diagnostic_stays_in_edited_excerpt(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let point = Point::new(1, 0);
        let diagnostic_point = PointUtf16::new(1, 0);
        let buffer = point_diagnostic_buffer(
            "lead\n\nhidden\nsafe",
            [(diagnostic_point..diagnostic_point, ERROR)],
            cx,
        );
        let multi_buffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        multi_buffer.update(cx, |multi_buffer, cx| {
            multi_buffer.set_excerpts_for_path(
                PathKey::for_buffer(&buffer, cx),
                buffer.clone(),
                vec![Point::zero()..point, Point::new(3, 0)..Point::new(3, 4)],
                0,
                cx,
            );
        });
        buffer.update(cx, |buffer, cx| {
            let snapshot = buffer.snapshot();
            buffer.edit(
                [(snapshot.anchor_before(5)..snapshot.anchor_after(6), "")],
                None,
                cx,
            );
        });

        let snapshot = cx.update(|cx| multi_buffer.read(cx).snapshot(cx));
        let excerpts = snapshot.excerpts().collect::<Vec<_>>();
        let points_by_excerpt = excerpts
            .iter()
            .map(|excerpt| {
                let start = snapshot
                    .anchor_in_excerpt(excerpt.context.start)
                    .expect("excerpt start")
                    .to_point(&snapshot);
                let end = snapshot
                    .anchor_in_excerpt(excerpt.context.end)
                    .expect("excerpt end")
                    .to_point(&snapshot);
                EditorElement::point_diagnostics_in_range(&snapshot, start..end)
                    .into_iter()
                    .map(|(point, _)| point)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(points_by_excerpt, vec![vec![point], Vec::new()]);
    }

    #[gpui::test]
    fn test_point_diagnostic_painted_without_glyph_or_at_edited_eof(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        for (text, point, deletion, expected_row, expected_pixels) in [
            ("\nx", PointUtf16::new(0, 0), None, 0, 0..16),
            ("x\n", PointUtf16::new(1, 0), None, 1, 0..16),
            ("", PointUtf16::new(0, 0), None, 0, 0..16),
            ("x\n", PointUtf16::new(1, 0), Some(1..2), 0, 16..32),
        ] {
            for clip_at_line_ends in [false, true] {
                let buffer = point_diagnostic_buffer(
                    text,
                    [(point..point, WARNING), (point..point, ERROR)],
                    cx,
                );
                if let Some(deletion) = deletion.clone() {
                    buffer.update(cx, |buffer, cx| buffer.edit([(deletion, "")], None, cx));
                }
                let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let (window, editor) = point_diagnostic_editor(multi_buffer, cx);
                editor.update(cx, |editor, cx| {
                    editor.set_clip_at_line_ends(clip_at_line_ends, cx);
                });
                assert_painted_point_diagnostics(
                    window,
                    &[(expected_row, expected_pixels.clone(), ERROR)],
                    cx,
                );
            }
        }
    }

    #[gpui::test]
    fn test_point_diagnostic_painted_monospace_severity(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        for (point_severity, other_severity) in [(WARNING, ERROR), (ERROR, WARNING), (ERROR, ERROR)]
        {
            for other_range in [0..1, 0..4, 1..4, 1..1] {
                let buffer = point_diagnostic_buffer(
                    "\niabc",
                    [
                        (PointUtf16::new(0, 0)..PointUtf16::new(0, 0), point_severity),
                        (
                            PointUtf16::new(1, other_range.start)
                                ..PointUtf16::new(1, other_range.end),
                            other_severity,
                        ),
                    ],
                    cx,
                );
                buffer.update(cx, |buffer, cx| buffer.edit([(0..1, "")], None, cx));
                let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let (window, editor) = point_diagnostic_editor(multi_buffer, cx);

                for offset_content in [false, true] {
                    editor.update(cx, |editor, cx| {
                        editor.set_offset_content(offset_content, cx)
                    });
                    for (
                        scale,
                        point_width,
                        range_width,
                        full_range_width,
                        gutter_origins,
                        root_origins,
                    ) in [
                        (1., 8, 23, 31, (4, 11), [0, 0, 0, 1, 3]),
                        (1.25, 10, 29, 39, (4, 14), [0, 0, 1, 1, 4]),
                        (1.5, 12, 35, 47, (5, 17), [0, 0, 1, 1, 5]),
                        (2., 16, 47, 62, (7, 23), [0, 0, 1, 2, 6]),
                        (3., 23, 70, 94, (11, 34), [0, 1, 1, 2, 10]),
                    ] {
                        cx.simulate_window_scale_factor_change(window.into(), scale);
                        for (offset, root_start) in
                            [0., 0.2, 0.5, 0.8, 3.25].into_iter().zip(root_origins)
                        {
                            window.root(cx).expect("test view").update(cx, |view, cx| {
                                view.x_offset = px(offset);
                                cx.notify();
                            });
                            let (point_start, next_start) = if offset_content {
                                (root_start + gutter_origins.0, root_start + gutter_origins.1)
                            } else {
                                (root_start, root_start + point_width)
                            };
                            let point_pixels = point_start..point_start + point_width;
                            let other_pixels = match (other_range.start, other_range.end) {
                                (0, 1) => point_pixels.clone(),
                                (0, 4) => point_start..point_start + full_range_width,
                                (1, 1) => next_start..next_start + point_width,
                                (1, 4) => next_start..next_start + range_width,
                                _ => unreachable!(),
                            };
                            let expected = (point_start..other_pixels.end.max(point_pixels.end))
                                .filter_map(|x| {
                                    let severity = match (
                                        point_pixels.contains(&x),
                                        other_pixels.contains(&x),
                                    ) {
                                        (true, true) => point_severity.min(other_severity),
                                        (true, false) => point_severity,
                                        (false, true) => other_severity,
                                        (false, false) => return None,
                                    };
                                    Some((0, x..x + 1, severity))
                                })
                                .collect::<Vec<_>>();
                            assert_painted_point_diagnostics(window, &expected, cx);
                        }
                    }
                }
            }
        }
    }

    #[gpui::test]
    fn test_point_diagnostic_painted_tabs_multibyte_and_replacements(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(4);
            settings.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
        });

        for (text, range, ranged_pixels) in [
            ("\n\tabc", 0..1, 2..64),
            ("\néabc", 1..4, 18..65),
            ("\n😀abc", 2..5, 33..80),
            ("\n\u{00a0}abc", 1..4, 18..65),
            ("\na\u{00a0}bc", 1..3, 33..49),
        ] {
            for (point_severity, ranged_severity) in
                [(WARNING, ERROR), (ERROR, WARNING), (ERROR, ERROR)]
            {
                let buffer = point_diagnostic_buffer(
                    text,
                    [
                        (PointUtf16::new(0, 0)..PointUtf16::new(0, 0), point_severity),
                        (
                            PointUtf16::new(1, range.start)..PointUtf16::new(1, range.end),
                            ranged_severity,
                        ),
                    ],
                    cx,
                );
                buffer.update(cx, |buffer, cx| buffer.edit([(0..1, "")], None, cx));
                let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let (window, _) = point_diagnostic_editor(multi_buffer, cx);
                window.root(cx).expect("test view").update(cx, |view, cx| {
                    view.x_offset = px(0.8);
                    cx.notify();
                });
                let expected = (2..ranged_pixels.end)
                    .filter_map(|x| {
                        let severity = match (x < 18, ranged_pixels.contains(&x)) {
                            (true, true) => point_severity.min(ranged_severity),
                            (true, false) => point_severity,
                            (false, true) => ranged_severity,
                            (false, false) => return None,
                        };
                        Some((0, x..x + 1, severity))
                    })
                    .collect::<Vec<_>>();
                assert_painted_point_diagnostics(window, &expected, cx);
            }
        }
    }

    #[gpui::test]
    fn test_point_diagnostic_ignores_non_diagnostic_replacement_underline(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        for (point_severity, ranged_severity) in [
            (None, ERROR),
            (Some(WARNING), ERROR),
            (Some(ERROR), WARNING),
            (Some(ERROR), ERROR),
        ] {
            let diagnostics = iter::once((
                PointUtf16::new(1, 0)..PointUtf16::new(1, 1),
                ranged_severity,
            ))
            .chain(
                point_severity
                    .map(|severity| (PointUtf16::new(0, 0)..PointUtf16::new(0, 0), severity)),
            );
            let buffer = point_diagnostic_buffer("\n\u{00a0}abc", diagnostics, cx);
            buffer.update(cx, |buffer, cx| buffer.edit([(0..1, "")], None, cx));
            let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let (window, _) = point_diagnostic_editor(multi_buffer, cx);
            let expected = point_severity
                .map(|severity| (0, 0..16, severity))
                .into_iter()
                .collect::<Vec<_>>();
            assert_painted_point_diagnostics(window, &expected, cx);
        }
    }

    #[gpui::test]
    fn test_point_diagnostics_follow_expanded_deletions(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        for (text, base_text, point, expected_display_row) in [
            ("", "two\n", PointUtf16::new(0, 0), 1),
            ("\nthree\n", "one\n\nthree\n", PointUtf16::new(0, 0), 1),
            (
                "one\n\nthree\n",
                "one\ntwo\n\nthree\n",
                PointUtf16::new(1, 0),
                2,
            ),
            ("one\n", "one\ntwo\n", PointUtf16::new(1, 0), 2),
        ] {
            let buffer =
                point_diagnostic_buffer(text, [(point..point, WARNING), (point..point, ERROR)], cx);

            let diff = cx.new(|cx| {
                BufferDiff::new_with_base_text(base_text, &buffer.read(cx).text_snapshot(), cx)
            });
            let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.add_diff(diff, cx);
                multi_buffer.expand_diff_hunks(vec![Anchor::Min..Anchor::Max], cx);
            });
            cx.run_until_parked();

            let snapshot = cx.update(|cx| multi_buffer.read(cx).snapshot(cx));
            let expected_point = Point::new(expected_display_row, 0);
            let deleted_region_start = snapshot
                .diff_hunks_in_range(Point::zero()..snapshot.max_point())
                .find(|hunk| hunk.status.kind == DiffHunkStatusKind::Deleted)
                .map(|hunk| Point::new(hunk.row_range.start.0, 0))
                .expect("expanded deletion should have a deleted display region");
            for query_start in [Point::zero(), deleted_region_start, expected_point] {
                let point_diagnostics = EditorElement::point_diagnostics_in_range(
                    &snapshot,
                    query_start..snapshot.max_point(),
                );
                assert_eq!(
                    point_diagnostics
                        .iter()
                        .map(|(point, _)| *point)
                        .collect::<Vec<_>>(),
                    vec![expected_point; 2],
                    "text: {text:?}, base: {base_text:?}, query start: {query_start:?}",
                );
            }

            let (window, _) = point_diagnostic_editor(multi_buffer, cx);
            assert_painted_point_diagnostics(window, &[(expected_display_row, 0..16, ERROR)], cx);
        }
    }

    #[gpui::test]
    fn test_point_diagnostics_exclude_collapsed_and_clipped_ranges(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let collapsed_buffer = point_diagnostic_buffer(
            "bad ok",
            [(PointUtf16::new(0, 0)..PointUtf16::new(0, 3), ERROR)],
            cx,
        );
        collapsed_buffer.update(cx, |buffer, cx| buffer.edit([(0..4, "")], None, cx));
        let collapsed_multi_buffer = cx.new(|cx| MultiBuffer::singleton(collapsed_buffer, cx));
        let clipped_buffer = point_diagnostic_buffer(
            "bad\ngood\n",
            [(PointUtf16::new(0, 0)..PointUtf16::new(1, 0), ERROR)],
            cx,
        );
        let clipped_multi_buffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        clipped_multi_buffer.update(cx, |multi_buffer, cx| {
            multi_buffer.set_excerpt_ranges_for_path(
                multi_buffer::PathKey::sorted(0),
                clipped_buffer.clone(),
                &clipped_buffer.read(cx).snapshot(),
                vec![multi_buffer::ExcerptRange::new(
                    Point::new(1, 0)..Point::new(1, 4),
                )],
                cx,
            );
        });
        for multi_buffer in [collapsed_multi_buffer, clipped_multi_buffer] {
            let snapshot = cx.update(|cx| multi_buffer.read(cx).snapshot(cx));
            assert_eq!(
                EditorElement::point_diagnostics_in_range(
                    &snapshot,
                    Point::zero()..snapshot.max_point(),
                )
                .len(),
                0,
            );
            let (window, _) = point_diagnostic_editor(multi_buffer, cx);
            assert_painted_point_diagnostics(window, &[], cx);
        }
    }

    #[gpui::test]
    fn test_point_diagnostic_inside_fold_is_not_rendered(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.gutter.get_or_insert_default().folds = Some(false);
                });
            });
        });

        let diagnostic_point = PointUtf16::new(1, 0);
        let buffer = point_diagnostic_buffer(
            "fn f() {\n\n}\n",
            [(diagnostic_point..diagnostic_point, ERROR)],
            cx,
        );
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let (window, editor) = point_diagnostic_editor(multi_buffer, cx);
        assert_painted_point_diagnostics(window, &[(1, 0..16, ERROR)], cx);
        let visual_cx = &mut VisualTestContext::from_window(*window, cx);
        editor.update_in(visual_cx, |editor, window, cx| {
            editor.display_map.update(cx, |display_map, _| {
                display_map.fold_placeholder = FoldPlaceholder::test();
            });
            editor.fold_ranges(vec![Point::new(0, 8)..Point::new(2, 0)], false, window, cx);
        });

        assert_painted_point_diagnostics(window, &[], &mut visual_cx.cx);
    }

    #[gpui::test]
    async fn test_point_diagnostic_painted_in_sticky_header(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.gutter.get_or_insert_default().folds = Some(false);
                    settings.editor.sticky_scroll = Some(settings::StickyScrollContent {
                        enabled: Some(true),
                    });
                });
            });
        });

        let diagnostic_point = PointUtf16::new(0, 0);
        let buffer = point_diagnostic_buffer("", [(diagnostic_point..diagnostic_point, ERROR)], cx);
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(Some(languages::rust_lang()), cx);
            buffer.edit(
                [(
                    0..0,
                    indoc::indoc! {"
                        fn foo() {
                            let one = 1;
                            let two = 2;
                            let three = 3;
                        }
                    "},
                )],
                None,
                cx,
            );

            let snapshot = buffer.snapshot();
            assert_eq!(
                snapshot
                    .diagnostics_in_range::<_, Point>(Point::zero()..snapshot.max_point(), false)
                    .map(|diagnostic| diagnostic.range)
                    .collect::<Vec<_>>(),
                vec![Point::zero()..Point::zero()],
            );
        });

        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let (window, editor) = point_diagnostic_editor(multi_buffer, cx);
        cx.simulate_window_resize(window.into(), size(px(500.), px(50.)));
        let cx = &mut VisualTestContext::from_window(*window, cx);
        cx.cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            editor.refresh_sticky_headers(&snapshot.display_snapshot, cx);
        });
        cx.cx.run_until_parked();
        assert_painted_point_diagnostics(window, &[(0, 0..16, ERROR)], &mut cx.cx);
        editor.update_in(cx, |editor, window, cx| {
            editor.scroll(point(0., 1.), window, cx)
        });
        cx.cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            assert_eq!(EditorElement::sticky_headers(editor, &snapshot).len(), 1);
        });
        assert_painted_point_diagnostics(window, &[(0, 0..16, ERROR)], &mut cx.cx);
        cx.update(|_, cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.sticky_scroll = Some(settings::StickyScrollContent {
                        enabled: Some(false),
                    });
                });
            });
        });
        assert_painted_point_diagnostics(window, &[], &mut cx.cx);
    }

    #[gpui::test]
    async fn test_soft_wrap_editor_width_full_editor(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&"a ".to_string().repeat(100), cx);
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));

        for x in 1..=100 {
            let (_, state) = cx.draw(
                Default::default(),
                size(px(200. + 0.13 * x as f32), px(500.)),
                |_, _| EditorElement::new(&editor, style.clone()),
            );
            let state = ready_layout(state);

            assert!(
                state.position_map.scroll_max.x == 0.,
                "Soft wrapped editor should have no horizontal scrolling!"
            );
        }
    }

    #[gpui::test]
    async fn test_status_bar_blame_location_reserves_no_scroll_width(cx: &mut TestAppContext) {
        struct FixedWidthBlameRenderer;

        impl BlameRenderer for FixedWidthBlameRenderer {
            fn max_author_length(&self) -> usize {
                20
            }

            fn render_blame_entry(
                &self,
                _: &gpui::TextStyle,
                _: BlameEntry,
                _: Option<ParsedCommitMessage>,
                _: Vec<SharedString>,
                _: Entity<project::git_store::Repository>,
                _: WeakEntity<Workspace>,
                _: Entity<Editor>,
                _: usize,
                _: Hsla,
                _: &mut Window,
                _: &mut App,
            ) -> Option<AnyElement> {
                None
            }

            fn render_inline_blame_entry(
                &self,
                _: &gpui::TextStyle,
                _: BlameEntry,
                _: &mut App,
            ) -> Option<AnyElement> {
                Some(div().w(px(160.)).into_any_element())
            }

            fn render_blame_entry_popover(
                &self,
                _: BlameEntry,
                _: ScrollHandle,
                _: Option<ParsedCommitMessage>,
                _: Vec<SharedString>,
                _: Entity<Markdown>,
                _: Entity<project::git_store::Repository>,
                _: WeakEntity<Workspace>,
                _: &mut Window,
                _: &mut App,
            ) -> Option<AnyElement> {
                None
            }

            fn open_blame_commit(
                &self,
                _: BlameEntry,
                _: Entity<project::git_store::Repository>,
                _: WeakEntity<Workspace>,
                _: &mut Window,
                _: &mut App,
            ) {
            }
        }

        init_test(cx, |_| {});
        cx.update(|cx| crate::git::set_blame_renderer(FixedWidthBlameRenderer, cx));

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            util::path!("/my-repo"),
            serde_json::json!({
                ".git": {},
                "file.txt": "a ".repeat(100),
            }),
        )
        .await;
        fs.set_blame_for_repo(
            std::path::Path::new(util::path!("/my-repo/.git")),
            vec![(
                git::repository::repo_path("file.txt"),
                git::blame::Blame {
                    entries: vec![BlameEntry {
                        sha: "1b1b1b".parse().unwrap(),
                        range: 0..1,
                        original_line_number: 0,
                        author: None,
                        author_mail: None,
                        author_time: None,
                        author_tz: None,
                        committer_name: None,
                        committer_email: None,
                        committer_time: None,
                        committer_tz: None,
                        summary: None,
                        previous: None,
                        filename: String::new(),
                        boundary: false,
                    }],
                    ..Default::default()
                },
            )],
        );

        let project = project::Project::test(fs, [util::path!("/my-repo").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(util::path!("/my-repo/file.txt"), cx)
            })
            .await
            .unwrap();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let window = cx.add_window(|window, cx| {
            // No soft wrap: a long line legitimately scrolls horizontally, so the
            // inline blame width reservation is meaningful and observable here.
            let mut editor = Editor::new(EditorMode::full(), buffer, Some(project), window, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        cx.update(|window, cx| window.focus(&editor.read(cx).focus_handle(cx), cx));
        editor.update(cx, |editor, cx| {
            editor
                .blame()
                .expect("inline blame should be running")
                .clone()
                .update(cx, |blame, cx| blame.focus(cx))
        });
        cx.executor().run_until_parked();

        // Ensure the blame entry actually loaded, so a broken setup can't let this
        // test pass vacuously with a zero-width blame reservation on both draws.
        editor.update(cx, |editor, cx| {
            assert!(editor.show_git_blame_inline);
            let blame = editor
                .blame()
                .expect("inline blame should be running")
                .clone();
            let entry = blame.update(cx, |blame, cx| {
                blame
                    .blame_for_rows(
                        &[RowInfo {
                            buffer_row: Some(0),
                            buffer_id: Some(buffer_id),
                            ..Default::default()
                        }],
                        cx,
                    )
                    .next()
                    .flatten()
            });
            assert!(entry.is_some(), "blame entry should be available");
        });

        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));

        // Default `Inline` location: the long line plus the reserved inline blame
        // width push the horizontal scroll range past the viewport.
        let (_, state) = cx.draw(Default::default(), size(px(226.), px(500.)), |_, _| {
            EditorElement::new(&editor, style.clone())
        });
        let state = ready_layout(state);
        let scroll_max_with_inline_blame = state.position_map.scroll_max.x;
        assert!(
            scroll_max_with_inline_blame > 0.,
            "inline blame on a long line should reserve horizontal scroll room"
        );

        // Moving blame to the status bar paints nothing inline, so the reservation
        // must be dropped and the scroll range shrink accordingly.
        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .git
                        .get_or_insert_default()
                        .inline_blame
                        .get_or_insert_default()
                        .location = Some(settings::InlineBlameLocation::StatusBar);
                });
            });
        });

        let (_, state) = cx.draw(Default::default(), size(px(226.), px(500.)), |_, _| {
            EditorElement::new(&editor, style.clone())
        });
        let state = ready_layout(state);
        assert!(
            state.position_map.scroll_max.x < scroll_max_with_inline_blame,
            "Blame in the status bar should not reserve horizontal scroll room"
        );
    }

    #[gpui::test]
    async fn test_mouse_pending_frame_hit_testing(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(4);
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        let long = "x".repeat(2_000);
        for (text, column, row, align, top, expected) in [
            (
                format!("0123456789\n{long}"),
                5.75,
                0.5,
                TextAlign::Left,
                1.,
                (0, 5),
            ),
            (long.clone(), 5.75, 0.5, TextAlign::Left, 1., (0, 5)),
            (
                format!("\tabc\n{long}"),
                2.75,
                0.5,
                TextAlign::Left,
                1.,
                (0, 0),
            ),
            (
                format!("é漢z\n{long}"),
                1.75,
                0.5,
                TextAlign::Left,
                1.,
                (0, 2),
            ),
            (
                format!("é😀z\n{long}"),
                3.75,
                0.5,
                TextAlign::Left,
                1.,
                (0, 6),
            ),
            (
                format!("a\u{200b}b\n{long}"),
                2.75,
                0.5,
                TextAlign::Left,
                1.,
                (0, 4),
            ),
            (long.clone(), 0.25, 1.5, TextAlign::Left, 1., (0, 2_000)),
            (
                format!("0123456789\n{long}"),
                5.75,
                0.5,
                TextAlign::Center,
                1.,
                (0, 5),
            ),
            (
                format!("0123456789\n{long}"),
                5.75,
                0.5,
                TextAlign::Right,
                1.,
                (0, 5),
            ),
            (
                format!("aaa\n0123456789\n{long}"),
                5.75,
                1.75,
                TextAlign::Left,
                -1.5,
                (1, 5),
            ),
        ] {
            let expected = Point::new(expected.0, expected.1);
            for cold in [false, true] {
                let (editor, ref mut cx) = editor_test_context("", cx);
                cx.update(|window, cx| {
                    let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
                    style.text.text_align = align;
                    let line_height = style.text.line_height_in_pixels(window.rem_size());
                    window.replace_root(cx, |_, _| EditorLayoutTestRoot {
                        editor: editor.clone(),
                        style,
                        bounds: Bounds::new(
                            point(px(32.), line_height * top),
                            size(px(500.), px(500.)),
                        ),
                        layout: Rc::new(RefCell::new(None)),
                    });
                    editor.update(cx, |editor, cx| {
                        editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                        editor.set_text(text.clone(), window, cx);
                        editor.change_selections(
                            SelectionEffects::no_scroll(),
                            window,
                            cx,
                            |selections| {
                                selections.select_ranges([Point::zero()..Point::zero()]);
                            },
                        );
                    });
                });
                if !cold {
                    cx.run_until_parked();
                }
                let completion = cx.update(|window, cx| {
                    window.draw(cx).clear(cx);
                    assert_eq!(editor.read(cx).last_position_map.is_none(), cold);
                    let viewport = editor
                        .read(cx)
                        .last_input_viewport
                        .as_ref()
                        .expect("painted viewport");
                    let free_width =
                        viewport.text_bounds.size.width - viewport.em_layout_width * 10.;
                    let alignment = match align {
                        TextAlign::Left => Pixels::ZERO,
                        TextAlign::Center => free_width / 2.,
                        TextAlign::Right => free_width,
                    };
                    let position = viewport.text_bounds.origin
                        + point(
                            alignment + viewport.em_layout_width * column,
                            viewport.line_height * row,
                        );
                    dispatch_mouse_click(position, 1, window, cx);
                    editor.update(cx, |editor, cx| {
                        if cold {
                            let long_row =
                                text.bytes().filter(|byte| *byte == b'\n').count() as u32;
                            assert!(
                                editor
                                    .snapshot(window, cx)
                                    .long_row(DisplayRow(long_row))
                                    .is_err()
                            );
                        }
                        editor.input_completion(cx)
                    })
                });
                completion.await.expect("mouse input completed");
                assert_eq!(
                    newest_head(&editor, cx),
                    expected,
                    "cold={cold}, column={column}, row={row}"
                );
            }
        }
    }

    #[gpui::test]
    async fn test_mouse_modifier_drag_preserves_selection_in_pending_frame(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.multi_cursor_modifier = Some(settings::MultiCursorModifier::Alt);
            settings.autoscroll_on_clicks = Some(false);
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        for cold in [false, true] {
            let (editor, ref mut cx) = editor_test_context("", cx);
            cx.update(|window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                    editor.set_text("x".repeat(2_000), window, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::zero()..Point::zero()]);
                        },
                    );
                });
            });
            if !cold {
                cx.run_until_parked();
            }
            let completion = cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(editor.read(cx).last_position_map.is_none(), cold);
                let viewport = editor
                    .read(cx)
                    .last_input_viewport
                    .as_ref()
                    .expect("painted viewport");
                let start = viewport.text_bounds.origin
                    + point(viewport.em_layout_width * 1.75, viewport.line_height * 0.5);
                let end = viewport.text_bounds.origin
                    + point(viewport.em_layout_width * 4.75, viewport.line_height * 0.5);
                let modifiers = Modifiers::secondary_key();
                window.dispatch_event(
                    PlatformInput::MouseDown(MouseDownEvent {
                        position: start,
                        modifiers,
                        button: MouseButton::Left,
                        click_count: 1,
                        first_mouse: false,
                    }),
                    cx,
                );
                window.dispatch_event(
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position: end,
                        modifiers,
                        pressed_button: Some(MouseButton::Left),
                    }),
                    cx,
                );
                if !cold {
                    assert!(editor.read(cx).has_pending_nonempty_selection());
                }
                window.dispatch_event(
                    PlatformInput::MouseUp(MouseUpEvent {
                        position: end,
                        modifiers,
                        button: MouseButton::Left,
                        click_count: 1,
                    }),
                    cx,
                );
                editor.update(cx, |editor, cx| {
                    if cold {
                        assert!(editor.snapshot(window, cx).long_row(DisplayRow(0)).is_err());
                        assert!(editor.input_is_deferred(cx));
                    }
                    editor.input_completion(cx)
                })
            });
            completion.await.expect("modifier drag completed");
            editor.update(cx, |editor, cx| {
                assert_eq!(
                    editor
                        .selections
                        .all::<Point>(&editor.display_snapshot(cx))
                        .into_iter()
                        .map(|selection| selection.range())
                        .collect::<Vec<_>>(),
                    [Point::new(0, 1)..Point::new(0, 4)],
                    "cold={cold}"
                );
                assert!(!editor.has_pending_selection());
            });
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[gpui::test]
    async fn test_mouse_middle_release_reads_primary_at_execution(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.middle_click_paste = Some(true);
            settings.autoscroll_on_clicks = Some(false);
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        for (cold, receipt, prior_write, expected) in [
            (false, Some("arrival"), false, "arrival"),
            (false, None, false, ""),
            (true, Some("arrival"), false, "external"),
            (true, None, false, "external"),
            (true, Some("arrival"), true, "queued"),
            (true, None, true, "queued"),
        ] {
            let (editor, ref mut cx) = editor_test_context("", cx);
            cx.update(|window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                    editor.set_text(format!("queued{}", "x".repeat(2_000)), window, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::zero()..Point::zero()]);
                        },
                    );
                });
            });
            if !cold {
                cx.run_until_parked();
            }
            let completion = cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(editor.read(cx).last_position_map.is_none(), cold);
                let viewport = editor
                    .read(cx)
                    .last_input_viewport
                    .as_ref()
                    .expect("painted viewport");
                let position = viewport.text_bounds.origin
                    + point(viewport.em_layout_width * 8.75, viewport.line_height * 0.5);
                if cold {
                    editor.update(cx, |editor, cx| {
                        let pending = editor
                            .snapshot(window, cx)
                            .long_row(DisplayRow(0))
                            .err()
                            .expect("cold geometry");
                        editor.defer_geometry(
                            pending,
                            move |editor, window, cx| {
                                if prior_write {
                                    editor.change_selections(
                                        SelectionEffects::no_scroll(),
                                        window,
                                        cx,
                                        |selections| {
                                            selections
                                                .select_ranges([Point::zero()..Point::new(0, 6)]);
                                        },
                                    );
                                }
                            },
                            window,
                            cx,
                        );
                    });
                }
                cx.write_to_primary(ClipboardItem::new_string(String::from("down")));
                window.dispatch_event(
                    PlatformInput::MouseDown(MouseDownEvent {
                        position,
                        modifiers: Modifiers::none(),
                        button: MouseButton::Middle,
                        click_count: 1,
                        first_mouse: false,
                    }),
                    cx,
                );
                cx.write_to_primary(
                    receipt
                        .map(|text| ClipboardItem::new_string(String::from(text)))
                        .unwrap_or_else(|| ClipboardItem {
                            entries: Vec::new(),
                        }),
                );
                window.dispatch_event(
                    PlatformInput::MouseUp(MouseUpEvent {
                        position,
                        modifiers: Modifiers::none(),
                        button: MouseButton::Middle,
                        click_count: 1,
                    }),
                    cx,
                );
                cx.write_to_primary(ClipboardItem::new_string(String::from("external")));
                editor.update(cx, |editor, cx| {
                    if cold {
                        assert!(editor.snapshot(window, cx).long_row(DisplayRow(0)).is_err());
                        assert!(editor.input_is_deferred(cx));
                    }
                    editor.input_completion(cx)
                })
            });
            completion.await.expect("middle release completed");
            editor.update(cx, |editor, cx| {
                assert_eq!(
                    editor.text(cx),
                    format!("queuedxx{expected}{}", "x".repeat(1_998)),
                    "cold={cold}, receipt={receipt:?}, prior_write={prior_write}"
                );
                assert_eq!(
                    editor
                        .selections
                        .newest::<Point>(&editor.display_snapshot(cx))
                        .head(),
                    Point::new(0, 8 + expected.len() as u32)
                );
                assert_eq!(
                    cx.read_from_primary().and_then(|item| item.text()),
                    Some(String::from(if prior_write {
                        "queued"
                    } else {
                        "external"
                    })),
                );
            });
        }

        for (delayed, failed) in [(true, false), (true, true), (false, true)] {
            let text = format!("queued{}", "x".repeat(2_000));
            let (editor, ref mut cx) = editor_test_context(&text, cx);
            editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            });
            cx.run_until_parked();
            let (read, resolver) = ClipboardSnapshotRead::pending();
            let resolver = if delayed {
                Some(resolver)
            } else {
                resolver.resolve(Err(ClipboardReadError::Unavailable));
                None
            };
            let captures = Rc::new(Cell::new(0));
            let completion = cx.update(|window, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new({
                    let captures = captures.clone();
                    move |primary| {
                        assert!(primary);
                        captures.set(captures.get() + 1);
                        read.clone()
                    }
                })));
                window.draw(cx).clear(cx);
                let viewport = editor
                    .read(cx)
                    .last_input_viewport
                    .as_ref()
                    .expect("painted viewport");
                let position = viewport.text_bounds.origin
                    + point(viewport.em_layout_width * 8.75, viewport.line_height * 0.5);
                cx.write_to_primary(ClipboardItem::new_string(String::from("before")));
                window.dispatch_event(
                    PlatformInput::MouseDown(MouseDownEvent {
                        position,
                        modifiers: Modifiers::none(),
                        button: MouseButton::Middle,
                        click_count: 1,
                        first_mouse: false,
                    }),
                    cx,
                );
                assert_eq!(captures.get(), 0);
                window.dispatch_event(
                    PlatformInput::MouseUp(MouseUpEvent {
                        position,
                        modifiers: Modifiers::none(),
                        button: MouseButton::Middle,
                        click_count: 1,
                    }),
                    cx,
                );
                assert_eq!(captures.get(), 1);
                editor.update(cx, |editor, cx| editor.input_completion(cx))
            });
            cx.run_until_parked();
            editor.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), text);
                editor.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(MultiBufferOffset(0)..MultiBufferOffset(0), "QQ")],
                        None,
                        cx,
                    );
                });
                cx.write_to_primary(ClipboardItem::new_string(String::from("external")));
            });
            if let Some(resolver) = resolver {
                resolver.resolve(if failed {
                    Err(ClipboardReadError::Unavailable)
                } else {
                    Ok(Arc::new(ClipboardSnapshot {
                        item: Some(ClipboardItem::new_string(String::from("started"))),
                        revision: None,
                    }))
                });
            }
            assert_eq!(completion.await.is_err(), delayed && failed);
            assert_eq!(captures.get(), 1);
            editor.update(cx, |editor, cx| {
                assert!(
                    !editor.has_pending_selection(),
                    "delayed={delayed}, failed={failed}"
                );
                let inserted = if failed { "" } else { "started" };
                assert_eq!(
                    editor.text(cx),
                    format!("QQqueuedxx{inserted}{}", "x".repeat(1_998)),
                );
                assert_eq!(
                    editor
                        .selections
                        .newest::<Point>(&editor.display_snapshot(cx))
                        .head(),
                    Point::new(0, 10 + inserted.len() as u32),
                );
                assert_eq!(
                    cx.read_from_primary().and_then(|item| item.text()),
                    Some(String::from("external")),
                );
            });
        }
    }

    #[gpui::test]
    async fn test_mouse_deferred_ready_hit_follows_edits(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.background_executor.set_block_on_ticks(0..=0);
        for (click_count, expected) in [
            (1, Point::new(0, 7)..Point::new(0, 7)),
            (2, Point::new(0, 6)..Point::new(0, 16)),
            (3, Point::zero()..Point::new(1, 0)),
        ] {
            let text = format!("abc 0123456789 def{}x", "\n".repeat(100));
            let (editor, ref mut cx) = editor_test_context(&text, cx);
            editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            });
            cx.run_until_parked();
            let completion = cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                let position_map = editor
                    .read(cx)
                    .last_position_map
                    .clone()
                    .expect("ready frame");
                let position = position_map.text_hitbox.origin
                    + point(
                        position_map.em_layout_width * 5.75,
                        position_map.line_height * 0.5,
                    );
                editor.update(cx, |editor, cx| {
                    editor.buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            [(Point::new(100, 1)..Point::new(100, 1), "x".repeat(1_999))],
                            None,
                            cx,
                        );
                    });
                    let pending = editor
                        .snapshot(window, cx)
                        .long_row(DisplayRow(100))
                        .err()
                        .expect("cold offscreen geometry");
                    editor.defer_geometry(pending, |_, _, _| {}, window, cx);
                    assert!(editor.input_is_deferred(cx));
                });
                dispatch_mouse_click(position, click_count, window, cx);
                editor.update(cx, |editor, cx| {
                    assert_eq!(
                        editor
                            .selections
                            .newest::<Point>(&editor.display_snapshot(cx))
                            .head(),
                        Point::zero()
                    );
                    assert!(
                        editor
                            .snapshot(window, cx)
                            .long_row(DisplayRow(100))
                            .is_err()
                    );
                    editor.buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            [(MultiBufferOffset(0)..MultiBufferOffset(0), "QQ")],
                            None,
                            cx,
                        );
                    });
                    editor.input_completion(cx)
                })
            });
            completion.await.expect("deferred mouse input completed");
            editor.update(cx, |editor, cx| {
                assert_eq!(
                    editor
                        .selections
                        .newest::<Point>(&editor.display_snapshot(cx))
                        .range(),
                    expected,
                    "click_count={click_count}"
                );
            });
        }
    }

    #[gpui::test]
    async fn test_mouse_pending_row_edited_or_removed(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.background_executor.set_block_on_ticks(0..=0);
        for remove in [false, true] {
            let (editor, ref mut cx) = editor_test_context("", cx);
            let completion = cx.update(|window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
                    editor.set_text(format!("{}\nend", "x".repeat(2_000)), window, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::zero()..Point::zero()]);
                        },
                    );
                });
                window.draw(cx).clear(cx);
                assert!(editor.read(cx).last_position_map.is_none());
                let viewport = editor
                    .read(cx)
                    .last_input_viewport
                    .as_ref()
                    .expect("pending viewport");
                let position = viewport.text_bounds.origin
                    + point(viewport.em_layout_width * 5.75, viewport.line_height * 0.5);
                dispatch_mouse_click(position, 1, window, cx);
                editor.update(cx, |editor, cx| {
                    assert!(editor.input_is_deferred(cx));
                    assert_eq!(
                        editor
                            .selections
                            .newest::<Point>(&editor.display_snapshot(cx))
                            .head(),
                        Point::zero()
                    );
                    editor.buffer.update(cx, |buffer, cx| {
                        if remove {
                            buffer.edit([(Point::zero()..Point::new(1, 0), "")], None, cx);
                        } else {
                            buffer.edit([(Point::zero()..Point::zero(), "header\n")], None, cx);
                        }
                    });
                    editor.input_completion(cx)
                })
            });
            completion
                .await
                .expect("mouse input survives changed geometry");
            assert_eq!(
                newest_head(&editor, cx),
                if remove {
                    Point::new(0, 3)
                } else {
                    Point::new(1, 5)
                }
            );
        }
    }

    #[gpui::test]
    async fn test_mouse_deferred_drag_preserves_column_goal(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.background_executor.set_block_on_ticks(0..=0);
        let (editor, ref mut cx) =
            editor_test_context(&format!("abc\n0123456789{}x", "\n".repeat(99)), cx);
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx)
        });
        cx.run_until_parked();
        let completion = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            let position_map = editor
                .read(cx)
                .last_position_map
                .clone()
                .expect("ready frame");
            let start = position_map.text_hitbox.origin
                + point(
                    position_map.em_layout_width * 5.75,
                    position_map.line_height * 0.5,
                );
            let end = position_map.text_hitbox.origin
                + point(
                    position_map.em_layout_width * 8.75,
                    position_map.line_height * 1.5,
                );
            editor.update(cx, |editor, cx| {
                editor.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(Point::new(100, 1)..Point::new(100, 1), "x".repeat(1_999))],
                        None,
                        cx,
                    )
                });
                let pending = editor
                    .snapshot(window, cx)
                    .long_row(DisplayRow(100))
                    .err()
                    .expect("cold geometry");
                editor.defer_geometry(pending, |_, _, _| {}, window, cx);
            });
            window.dispatch_event(
                PlatformInput::MouseDown(MouseDownEvent {
                    position: start,
                    modifiers: Modifiers::none(),
                    button: MouseButton::Middle,
                    click_count: 1,
                    first_mouse: false,
                }),
                cx,
            );
            window.dispatch_event(
                PlatformInput::MouseMove(MouseMoveEvent {
                    position: end,
                    modifiers: Modifiers::none(),
                    pressed_button: Some(MouseButton::Middle),
                }),
                cx,
            );
            window.dispatch_event(
                PlatformInput::MouseUp(MouseUpEvent {
                    position: end,
                    modifiers: Modifiers::none(),
                    button: MouseButton::Middle,
                    click_count: 1,
                }),
                cx,
            );
            editor.update(cx, |editor, cx| {
                assert!(editor.input_is_deferred(cx));
                editor.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(MultiBufferOffset(0)..MultiBufferOffset(0), "QQ")],
                        None,
                        cx,
                    )
                });
                editor.input_completion(cx)
            })
        });
        completion.await.expect("columnar drag completed");
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(&editor.display_snapshot(cx))
                    .into_iter()
                    .map(|selection| selection.range())
                    .collect::<Vec<_>>(),
                [Point::new(1, 7)..Point::new(1, 8)]
            );
        });
    }

    #[gpui::test]
    fn test_mouse_row_layout_floor_at_large_renderer_boundary(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context("a\u{1}\u{85}b", cx);
        editor.update(cx, |editor, cx| {
            let position = editor
                .buffer
                .read(cx)
                .snapshot(cx)
                .anchor_after(Point::new(0, 1));
            editor.splice_inlays(
                &[],
                vec![Inlay::mock_repl_result(0, position, "render")],
                cx,
            );
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            let em_width = editor
                .read(cx)
                .last_position_map
                .as_ref()
                .expect("ready frame")
                .em_layout_width;
            editor.update(cx, |editor, cx| {
                let width = px(33_554_432.);
                editor.display_map.update(cx, |map, cx| {
                    let key = map
                        .renderer_widths_key()
                        .cloned()
                        .expect("measured renderer");
                    map.update_renderer_widths(
                        key,
                        [(ChunkRendererId::Inlay(crate::InlayId::ReplResult(0)), width)],
                        cx,
                    );
                });
                let snapshot = editor.snapshot(window, cx);
                let layout = snapshot
                    .layout_row(DisplayRow(0), &editor.text_layout_details(window, cx))
                    .expect("ready row");
                let advance = f64::from(em_width);
                let boundary = advance + f64::from(width);
                let expected_width = f64::from(width) + 4. * advance;
                assert_eq!(snapshot.line_len(DisplayRow(0)), 11);
                assert_eq!(layout.width(), expected_width);
                assert_eq!(layout.hit_test_width(), expected_width);
                for (column, expected) in [
                    (0, 0.),
                    (1, advance),
                    (6, advance),
                    (7, boundary),
                    (8, boundary + advance),
                    (9, boundary + 2. * advance),
                    (10, boundary + 2. * advance),
                    (11, expected_width),
                    (12, expected_width),
                ] {
                    assert_eq!(layout.x_for_index(column), expected, "column {column}");
                }
                for (x, floor, nearest) in [
                    (-0.25, Some(0), 0),
                    (advance - 0.25, Some(0), 1),
                    (advance, Some(1), 1),
                    (advance + f64::from(width) / 2., Some(1), 1),
                    (advance + f64::from(width) / 2. + 0.25, Some(1), 7),
                    (boundary - 0.25, Some(1), 7),
                    (boundary, Some(7), 7),
                    (boundary + advance / 2., Some(7), 7),
                    (boundary + advance / 2. + 0.25, Some(7), 8),
                    (boundary + advance, Some(8), 8),
                    (boundary + 2. * advance - 0.25, Some(8), 10),
                    (boundary + 2. * advance, Some(10), 10),
                    (boundary + 2.5 * advance, Some(10), 10),
                    (boundary + 2.5 * advance + 0.25, Some(10), 11),
                    (expected_width - 0.25, Some(10), 11),
                    (expected_width, None, 11),
                    (expected_width + 0.25, None, 11),
                ] {
                    assert_eq!(layout.index_for_x(x), floor, "floor at {x}");
                    assert_eq!(layout.closest_index_for_x(x), nearest, "nearest at {x}");
                }
                let RowLayout::Rendered(fragments) = layout else {
                    panic!("short row with renderer and replacements");
                };
                for column in 0..=11 {
                    let expected = match column {
                        0 | 7..=10 => Some(FontId(0)),
                        _ => None,
                    };
                    assert_eq!(
                        fragments.font_id_for_index(column),
                        expected,
                        "font at {column}"
                    );
                }
            });
        });
    }

    #[gpui::test]
    async fn test_point_for_position_clipped_rows(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let text = "aaa\nbbb";
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(text, cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let line_height = window
            .update(cx, |_, window, _| {
                style.text.line_height_in_pixels(window.rem_size())
            })
            .unwrap();

        // the first line is clipped
        let state = draw_ready_editor_at(
            &editor,
            &style,
            Bounds::new(
                point(Pixels::ZERO, Pixels::ZERO - line_height * 1.5),
                size(px(500.), px(500.)),
            ),
            cx,
        );
        editor.read_with(cx, |editor, _| {
            let viewport = editor.last_input_viewport().expect("painted viewport");
            assert_eq!(viewport.visible_row_range, DisplayRow(1)..DisplayRow(2));
            assert_eq!(viewport.scroll_pixel_position, point(0., 0.));
        });

        // click at the end of the second line
        let target_point = DisplayPoint::new(DisplayRow(1), 3);
        let click_x = state.content_origin.x
            + Pixels::from(editor.update_in(cx, |editor, window, cx| {
                editor
                    .snapshot(window, cx)
                    .x_for_display_point(target_point, &editor.text_layout_details(window, cx))
                    .expect("ready row geometry")
            }));

        let point = state
            .position_map
            .point_for_position(point(click_x, px(0.)));
        assert_eq!(point.nearest_valid, target_point);
    }

    #[gpui::test]
    fn test_point_for_position_keeps_precision_far_past_line_ends(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context(&format!("\n{}", "x".repeat(30_000)), cx);
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.text.font_size = px(1_000.).into();
        draw_editor_frame(&editor, &style, cx);
        cx.run_until_parked();
        let em_layout_width = ScrollPixelOffset::from(
            draw_editor(&editor, &style, cx)
                .position_map
                .em_layout_width,
        );

        let target_column = 28_000_u32;
        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(f64::from(target_column - 1), 0.), window, cx);
        });
        let state = draw_editor(&editor, &style, cx);
        let position_map = &state.position_map;
        let column_start_x = f64::from(target_column) * em_layout_width;
        let target_x = column_start_x - 0.25;
        assert!(
            f64::from(target_x as f32) >= column_start_x,
            "{target_x} survives a round trip through f32"
        );
        let offset = target_x - position_map.scroll_pixel_position.x;
        let point = position_map.point_for_position(point(
            position_map.text_hitbox.bounds.origin.x + Pixels::from(offset),
            px(0.),
        ));
        assert_eq!(
            point.exact_unclipped,
            DisplayPoint::new(DisplayRow(0), target_column - 1)
        );
    }

    #[gpui::test]
    fn test_long_rows_paint_and_hit_test_at_row_geometry(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let long_row = "ab\u{0}漢\té ".repeat(1_000);
        let (editor, ref mut cx) = editor_test_context(&format!("short\n{long_row}\nshort"), cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(1, 0)..Point::new(1, 0)])
            });
            editor.set_scroll_position(point(3_000., 0.), window, cx);
        });
        cx.run_until_parked();

        let state = draw_editor(&editor, &style, cx);
        let (snapshot, details) = editor.update_in(cx, |editor, window, cx| {
            (
                editor.snapshot(window, cx),
                editor.text_layout_details(window, cx),
            )
        });
        let row = DisplayRow(1);
        let row_x = |column: usize| {
            snapshot
                .x_for_display_point(DisplayPoint::new(row, column as u32), &details)
                .expect("ready row geometry")
        };
        let line = &state.position_map.line_layouts[row.0 as usize];
        let window_columns = line
            .long_row
            .as_ref()
            .map(|window| window.plan.start as usize..window.plan.end as usize)
            .expect("the long row is laid out as a window");
        assert!(window_columns.start > 0);
        assert!(window_columns.end < line.len);
        assert!(state.active_rows.contains_key(&row));

        for (fragment, fragment_start, fragment_x) in line.fragment_positions() {
            let LineFragment::Text(shaped) = fragment else {
                continue;
            };
            if shaped.text.len() != shaped.len() {
                continue;
            }
            for glyph in shaped.runs.iter().flat_map(|run| &run.glyphs) {
                let painted_x = fragment_x + f64::from(glyph.position.x);
                let expected_x = row_x(fragment_start + glyph.index);
                assert!(
                    (painted_x - expected_x).abs() < 0.01,
                    "glyph at column {}: {painted_x} != {expected_x}",
                    fragment_start + glyph.index
                );
            }
        }

        let scroll_x = state.position_map.scroll_pixel_position.x;
        let line_text = snapshot.line(row);
        for (column, _) in line_text.match_indices('a') {
            let x = row_x(column);
            assert!((line.x_for_index(column) - x).abs() < 0.01);
            if !window_columns.contains(&column) {
                continue;
            }
            let click = state.content_origin
                + point(
                    Pixels::from(x - scroll_x),
                    state.position_map.line_height * 1.5,
                );
            let position = state.position_map.point_for_position(click);
            assert_eq!(
                position.previous_valid,
                DisplayPoint::new(row, column as u32)
            );
        }
        for (column, _) in line_text.match_indices('漢') {
            let after_han = row_x(column + 3);
            assert!(after_han > row_x(column));
            let interior_xs = [1, 2, 3].map(|offset| line.x_for_index(column + offset));
            assert_eq!(interior_xs, [interior_xs[2]; 3]);
            assert!((interior_xs[2] - after_han).abs() < 0.01);
        }
        cx.simulate_resize(size(px(300.), px(500.)));
        for scale in [1_f32, 1.25, 1.5, 2., 3.] {
            cx.simulate_scale_factor_change(scale);
            let state = draw_ready_editor_at(
                &editor,
                &style,
                Bounds::new(point(px(3.25), px(0.)), size(px(500.), px(500.))),
                cx,
            );
            let line = &state.position_map.line_layouts[1];
            let retained = line.long_row.as_ref().expect("retained row");
            let origin = f64::from(state.content_origin.x)
                + line.origin_x(
                    state.text_align,
                    state.content_width,
                    state.position_map.scroll_pixel_position.x,
                );
            let left = f64::from(
                (f32::from(state.position_map.text_hitbox.bounds.left()) * scale).floor(),
            ) / f64::from(scale)
                - origin;
            let right = f64::from((300. * scale).ceil()) / f64::from(scale) - origin;
            assert_eq!(retained.paint_x, left..right, "scale {scale}");
            assert!(!line.long_row_needs_relayout(
                state.position_map.scroll_pixel_position.x,
                state.text_align,
                state.content_origin.x,
                ScaledPixels(
                    (f32::from(state.position_map.text_hitbox.bounds.left()) * scale).floor()
                )..ScaledPixels((300. * scale).ceil()),
                state.content_width,
                scale,
            ));
        }
    }

    #[gpui::test]
    fn test_long_row_decorations_skip_horizontally_hidden_columns(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let empty_range = |column| PointUtf16::new(0, column)..PointUtf16::new(0, column);
        let buffer = point_diagnostic_buffer(
            &format!("{}\nshort", "x".repeat(10_000)),
            [0, 5_010, 9_000]
                .into_iter()
                .map(|column| (empty_range(column), ERROR)),
            cx,
        );
        let window = cx.add_window(|window, cx| {
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([
                    Point::new(0, 0)..Point::new(0, 5),
                    Point::new(1, 0)..Point::new(1, 0),
                ])
            });
            editor.set_scroll_position(point(5_000., 0.), window, cx);
        });
        cx.run_until_parked();

        let state = draw_editor(&editor, &style, cx);
        let line = &state.position_map.line_layouts[0];
        let window = line.long_row.as_ref().expect("a windowed long row");
        assert!(!window.contains_column(0));
        assert_eq!(
            state
                .active_rows
                .get(&DisplayRow(0))
                .map(|spec| spec.selection),
            Some(true)
        );
        assert_eq!(
            line.point_diagnostics
                .iter()
                .map(|diagnostic| diagnostic.column)
                .collect::<Vec<_>>(),
            [5_010]
        );
    }

    #[gpui::test]
    fn test_long_row_window_keeps_clipped_tab_bounds(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(128);
        });
        let state = draw_single_row_editor(&"\t".repeat(9), 120., cx);
        let line = &state.position_map.line_layouts[0];
        let window = line.long_row.as_ref().expect("a windowed long row");
        let columns = window.plan.clone();
        assert!(
            (1..128).contains(&columns.start) && (129..=256).contains(&columns.end),
            "{window:?}"
        );
        assert_eq!(
            line.invisibles,
            [
                Invisible::Tab {
                    line_start_offset: 0,
                    line_end_offset: 128,
                },
                Invisible::Tab {
                    line_start_offset: 128,
                    line_end_offset: 256,
                },
            ]
        );
        for plan in [columns.clone(), columns.start..columns.start] {
            let start = plan.start as usize;
            let end = plan.end as usize;
            let window = LongRowWindow {
                row: window.row.clone(),
                plan,
                origin_x: window.origin_x,
                paint_x: window.paint_x.clone(),
                fragment_origins: SmallVec::new(),
            };
            assert_eq!(
                [start - 1, start, end, end + 1].map(|column| window.contains_column(column)),
                [false, true, true, false],
            );
        }
    }

    #[gpui::test]
    fn test_offscreen_long_row_pending_horizontal_scroll(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.scroll_sensitivity = Some(1.);
            settings.scrollbar.get_or_insert_default().show = Some(settings::ShowScrollbar::Always);
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        let text = format!(
            "{}{}{}",
            "x".repeat(128),
            "\n".repeat(100),
            "x".repeat(2_048)
        );
        let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
            let buffer = MultiBuffer::build_simple(&text, cx);
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            editor.set_scroll_position(point(1_024., 0.), window, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).expect("test editor");
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let pending = draw_editor(&editor, &style, cx);
        let position_map = &pending.position_map;
        assert!(position_map.horizontal_scroll_pending);
        assert!(position_map.visible_row_range.end <= DisplayRow(100));
        assert!(position_map.scroll_max.x > 0.);
        assert!(position_map.scroll_max.x < 1_024.);
        assert!(!editor.read_with(cx, |editor, _| editor.last_horizontal_scrollbar_visible));
        assert_eq!(
            editor.update(cx, |editor, cx| editor.scroll_position(cx)),
            point(1_024., 0.)
        );
        assert_eq!(
            position_map.scroll_pixel_position.x,
            cx.update(|window, _| {
                window.pixel_snap_f64(1_024. * f64::from(position_map.em_layout_width))
            })
        );

        for (delta, expected_x) in [(2_048., 0.), (-4_096., 4_096.)] {
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                let position_map = editor
                    .read(cx)
                    .last_position_map
                    .as_ref()
                    .expect("painted editor");
                assert!(position_map.horizontal_scroll_pending);
                let dispatch = window.dispatch_event(
                    PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: position_map.text_hitbox.bounds.center(),
                        delta: ScrollDelta::Lines(point(delta, 1_000.)),
                        ..ScrollWheelEvent::default()
                    }),
                    cx,
                );
                assert!(!dispatch.propagate);
                assert_eq!(
                    editor.update(cx, |editor, cx| editor.scroll_position(cx)),
                    point(expected_x, 0.)
                );
            });
            let pending = draw_editor(&editor, &style, cx);
            assert!(pending.position_map.horizontal_scroll_pending);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.scroll_position(cx)),
                point(expected_x, 0.)
            );
        }

        cx.run_until_parked();
        let resolved = draw_editor(&editor, &style, cx);
        let position_map = &resolved.position_map;
        assert!(!position_map.horizontal_scroll_pending);
        assert!(position_map.visible_row_range.end <= DisplayRow(100));
        let exact_width = 2_048. * f64::from(position_map.em_layout_width);

        let (expected_max, expected_pixel_x) = cx.update(|window, cx| {
            let font_id = window.text_system().resolve_font(&style.text.font());
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let em_width = window
                .text_system()
                .em_width(font_id, font_size)
                .expect("font width");
            let editor = editor.read(cx);
            let viewport_width = position_map.text_hitbox.size.width
                - editor.gutter_dimensions.margin
                - (2 * em_width + editor.last_right_margin);
            let expected_max =
                (exact_width - f64::from(viewport_width)) / f64::from(position_map.em_layout_width);
            (
                expected_max,
                window.pixel_snap_f64(expected_max * f64::from(position_map.em_layout_width)),
            )
        });
        assert_eq!(position_map.scroll_max.x, expected_max);
        assert_eq!(position_map.scroll_pixel_position.x, expected_pixel_x);
        assert_eq!(
            editor.update(cx, |editor, cx| editor.scroll_position(cx)),
            point(expected_max, 0.)
        );
        assert!(editor.read_with(cx, |editor, _| editor.last_horizontal_scrollbar_visible));

        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.set_scroll_position(point(0., 10.), window, cx);
            });
            window.draw(cx).clear(cx);
            let position_map = editor
                .read(cx)
                .last_position_map
                .as_ref()
                .expect("painted editor");
            let position = position_map.text_hitbox.bounds.center();
            let line_height = position_map.line_height;
            let glyph_width = position_map.em_layout_width;
            assert_eq!(
                editor.update(cx, |editor, cx| editor.scroll_position(cx)),
                point(0., 10.)
            );
            for (delta, touch_phase, expected) in [
                (
                    ScrollDelta::Lines(point(0., -1.)),
                    TouchPhase::Moved,
                    point(0., 11.),
                ),
                (
                    ScrollDelta::Lines(point(0., -1.)),
                    TouchPhase::Moved,
                    point(0., 12.),
                ),
                (
                    ScrollDelta::Pixels(point(-line_height / 4., -line_height / 2.)),
                    TouchPhase::Started,
                    point(0., 12.5),
                ),
                (
                    ScrollDelta::Pixels(point(-line_height / 4., -line_height / 2.)),
                    TouchPhase::Moved,
                    point(0., 13.),
                ),
                (
                    ScrollDelta::Pixels(point(line_height / 4., line_height / 2.)),
                    TouchPhase::Moved,
                    point(0., 12.5),
                ),
                (
                    ScrollDelta::Pixels(point(-glyph_width, -glyph_width / 4.)),
                    TouchPhase::Started,
                    point(1., 12.5),
                ),
                (
                    ScrollDelta::Pixels(point(-glyph_width, -glyph_width / 4.)),
                    TouchPhase::Moved,
                    point(2., 12.5),
                ),
                (
                    ScrollDelta::Lines(point(1., 1.)),
                    TouchPhase::Moved,
                    point(1., 11.5),
                ),
            ] {
                window.dispatch_event(
                    PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position,
                        delta,
                        touch_phase,
                        ..ScrollWheelEvent::default()
                    }),
                    cx,
                );
                assert_eq!(
                    editor.update(cx, |editor, cx| editor.scroll_position(cx)),
                    expected,
                    "{delta:?}, {touch_phase:?}"
                );
            }
        });
    }

    #[gpui::test]
    fn test_autoscroll_after_long_row_insertion(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
            let buffer = MultiBuffer::build_simple(&"x".repeat(2_000), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        draw_editor(&editor, &style, cx);

        let inserted = "漢".repeat(40_000);
        editor.update_in(cx, |editor, window, cx| {
            editor.insert(&inserted, window, cx)
        });
        let state = draw_ready_editor(&editor, &style, cx);
        let line = &state.position_map.line_layouts[0];
        let position_map = &state.position_map;
        let cursor_x = position_map.content_origin.x
            + Pixels::from(line.x_for_index(inserted.len()) - position_map.scroll_pixel_position.x);
        let bounds = position_map.text_hitbox.bounds;
        assert!(
            bounds.left() - px(1.) <= cursor_x && cursor_x <= bounds.right(),
            "{cursor_x:?} outside {bounds:?}"
        );
    }

    #[gpui::test]
    fn test_autoscroll_survives_renderer_width_restart(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
            let buffer = MultiBuffer::build_simple(
                &format!("{}{}", "x\n".repeat(332), "x".repeat(2_000)),
                cx,
            );
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        draw_editor(&editor, &style, cx);
        editor.update_in(cx, |editor, window, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let inlays = (300..=332)
                .map(|row| {
                    Inlay::mock_repl_result(
                        row as usize,
                        buffer.anchor_after(Point::new(row, 1)),
                        "result",
                    )
                })
                .collect();
            editor.splice_inlays(&[], inlays, cx);
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::newest()),
                window,
                cx,
                |selections| {
                    selections.select_ranges([Point::new(332, 1_800)..Point::new(332, 1_800)])
                },
            );
        });
        let state = draw_ready_editor(&editor, &style, cx);
        let position_map = &state.position_map;
        assert!(position_map.scroll_pixel_position.x > 0.);
        let row = DisplayRow(332);
        let line =
            &position_map.line_layouts[row.minus(position_map.visible_row_range.start) as usize];
        let cursor_x = position_map.content_origin.x
            + Pixels::from(line.x_for_index(1_806) - position_map.scroll_pixel_position.x);
        assert!(cursor_x >= position_map.text_hitbox.bounds.left());
        assert!(cursor_x <= position_map.text_hitbox.bounds.right());
    }

    #[gpui::test]
    fn test_final_prepaint_keeps_long_row_viewport_atomic(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let text = "x".repeat(10_000);
        let buffer = point_diagnostic_buffer(
            &text,
            [10, 9_010].map(|column| {
                let point = PointUtf16::new(0, column);
                (point..point, ERROR)
            }),
            cx,
        );
        let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::None, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        draw_editor(&editor, &style, cx);

        let prepaint = cx.new(|_| EditorPrepaintTestState::default());
        let layout = Rc::new(RefCell::new(None));
        cx.draw(
            point(px(0.), px(0.)),
            size(px(500.), px(500.)),
            |window, cx| {
                editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let ranges = [8..12, 9_008..9_012].map(|range| {
                        buffer.anchor_before(Point::new(0, range.start))
                            ..buffer.anchor_after(Point::new(0, range.end))
                    });
                    editor.highlight_background(
                        HighlightKey::Editor,
                        &ranges,
                        |_, _| Hsla::red(),
                        cx,
                    );
                    editor.insert_blocks(
                        [BlockProperties {
                            style: BlockStyle::Sticky,
                            placement: BlockPlacement::Below(buffer.anchor_after(Point::new(0, 0))),
                            height: Some(1),
                            render: Arc::new({
                                let prepaint = prepaint.clone();
                                move |cx| {
                                    let stored_height = cx.height;
                                    let height = (stored_height + 1).min(5);
                                    prepaint.update(cx.app, |prepaint, _| {
                                        if let Some(depth) = &prepaint.depth {
                                            prepaint.block_renders.push((
                                                depth.get(),
                                                stored_height,
                                                height,
                                            ));
                                        }
                                    });
                                    div().h(cx.line_height * height as f32).into_any()
                                }
                            }),
                            priority: 0,
                        }],
                        None,
                        cx,
                    );
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::new(0, 9_010)..Point::new(0, 9_010)]);
                        },
                    );
                });
                gpui::ViewElement::new(EditorLayoutTestView {
                    element: EditorElement::new(&editor, style.clone()),
                    layout: layout.clone(),
                    prepaint: Some(prepaint.clone()),
                })
            },
        );
        let deferred = ready_layout(layout.borrow_mut().take().expect("editor was painted"));
        let deferred_scroll_x = prepaint.read_with(cx, |prepaint, _| {
            assert_eq!(
                prepaint.block_renders,
                [(1, 1, 2), (2, 2, 3), (3, 3, 4), (4, 4, 5), (5, 5, 5)],
                "prepaint depth, stored block height, rendered block height",
            );
            prepaint.scroll_x.expect("editor was prepainted")
        });
        let scrolled = draw_editor(&editor, &style, cx);
        let full_line = cx.update(|window, _| {
            window.text_system().layout_line(
                &text,
                deferred.position_map.line_layouts[0].font_size,
                &[style.text.to_run(text.len())],
                None,
            )
        });
        for (state, expected_column) in [(&deferred, 10), (&scrolled, 9_010)] {
            assert!(state.position_map.line_layouts[0].long_row.is_some());
            let columns = [10, 9_010]
                .into_iter()
                .filter(|column| {
                    let x = state.position_map.content_origin.x
                        + Pixels::from(
                            f64::from(full_line.x_for_index(*column as usize))
                                - state.position_map.scroll_pixel_position.x,
                        );
                    let bounds = state.position_map.text_hitbox.bounds;
                    x >= bounds.left() && x < bounds.right()
                })
                .collect::<Vec<_>>();
            assert_eq!(columns, [expected_column]);
            assert_eq!(
                state.position_map.line_layouts[0]
                    .point_diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.column)
                    .collect::<Vec<_>>(),
                [expected_column],
            );
            assert_eq!(
                state.highlighted_ranges,
                vec![(
                    DisplayPoint::new(DisplayRow(0), expected_column - 2)
                        ..DisplayPoint::new(DisplayRow(0), expected_column + 2),
                    Hsla::red(),
                )],
            );
        }
        assert_eq!(deferred.position_map.scroll_position.x, 0.);
        assert_eq!(deferred_scroll_x, 0.);
        assert!(scrolled.position_map.scroll_position.x > 0.);
    }

    #[gpui::test]
    fn test_vertical_moves_into_long_rows(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let characters = 20_000;
        let source = "x".repeat(characters * 5 / 2);
        let destination = format!("{}{}", "𝒜".repeat(characters), "x".repeat(characters));
        let (editor, ref mut cx) = editor_test_context(&format!("{source}\n{destination}"), cx);
        editor.update_in(cx, |editor, window, cx| {
            let source_end = Point::new(0, source.len() as u32);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([source_end..source_end])
            });
            editor.move_down(&MoveDown, window, cx);
        });
        cx.run_until_parked();
        let column_at_source_end = (characters * "𝒜".len() + characters / 2) as u32;
        assert_eq!(
            newest_head(&editor, cx),
            Point::new(1, column_at_source_end)
        );
    }

    #[gpui::test]
    fn test_vertical_moves_from_long_rows() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        let source = format!("{}{}", "x".repeat(70_000), "w".repeat(30_000));
        let destination = "x".repeat(100_000);
        let (editor, ref mut cx) = editor_test_context(&format!("\n{destination}"), &mut cx);
        cx.run_until_parked();
        editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(0)..MultiBufferOffset(0), source.as_str())],
                    None,
                    cx,
                );
            });
        });
        let source_end = Point::new(0, source.len() as u32);
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([source_end..source_end])
            });
            editor.move_down(&MoveDown, window, cx);
        });
        cx.run_until_parked();
        let source_end_in_x_advances = 70_000 + 30_000 * 3 / 4;
        assert_eq!(
            newest_head(&editor, cx),
            Point::new(1, source_end_in_x_advances)
        );

        editor.update_in(cx, |editor, window, cx| editor.move_up(&MoveUp, window, cx));
        cx.run_until_parked();
        assert_eq!(newest_head(&editor, cx), source_end);
    }

    #[gpui::test]
    fn test_cloned_editor_keeps_horizontal_scroll_on_text(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context(&"é".repeat(200_000), cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(100_000., 0.), window, cx);
        });
        let original = draw_editor(&editor, &style, cx);
        let clone = editor.update_in(cx, |editor, window, cx| {
            cx.new(|cx| editor.clone(window, cx))
        });
        let cloned = draw_ready_editor(&clone, &style, cx);

        assert_eq!(left_edge_point(&cloned), left_edge_point(&original));
    }

    #[gpui::test]
    fn test_render_scroll_corrections_keep_remote_provenance(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context(&"é".repeat(200_000), cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        let scroll_events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|_, cx| {
            let scroll_events = scroll_events.clone();
            cx.subscribe(&editor, move |_, event: &EditorEvent, _| {
                if let EditorEvent::ScrollPositionChanged { local, .. } = event {
                    scroll_events.borrow_mut().push(*local);
                }
            })
        });

        editor.update_in(cx, |editor, window, cx| {
            let anchor = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_before(Point::zero());
            editor.set_scroll_anchor_remote(
                ScrollAnchor {
                    anchor,
                    offset: point(100_000.3, 0.),
                },
                window,
                cx,
            );
        });
        for _ in 0..3 {
            draw_editor_frame(&editor, &style, cx);
            cx.run_until_parked();
        }
        let remote_events = scroll_events.take();
        assert!(!remote_events.is_empty());
        assert_eq!(remote_events, vec![false; remote_events.len()]);

        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(50_000.3, 0.), window, cx);
        });
        draw_editor(&editor, &style, cx);
        let local_events = scroll_events.take();
        assert!(!local_events.is_empty());
        assert_eq!(local_events, vec![true; local_events.len()]);
    }

    #[gpui::test]
    async fn test_persisted_horizontal_scroll_follows_edits_and_restores(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let workspace_id = cx
            .update(|cx| workspace::WorkspaceDb::global(cx))
            .next_id()
            .await
            .unwrap();
        let editor_db = cx.update(|cx| crate::persistence::EditorDb::global(cx));
        let text = "é".repeat(200_000);
        let (editor, ref mut cx) = editor_test_context(&text, cx);
        let item_id = editor.entity_id().as_u64() as workspace::ItemId;
        editor_db
            .save_serialized_editor(
                item_id,
                workspace_id,
                crate::persistence::SerializedEditor::default(),
            )
            .await
            .unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            editor.workspace = Some((WeakEntity::new_invalid(), Some(workspace_id)));
            editor.set_scroll_position(point(100_000., 0.), window, cx);
        });
        let scrolled = draw_editor(&editor, &style, cx);

        editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(0)..MultiBufferOffset(0), "y".repeat(100))],
                    None,
                    cx,
                )
            })
        });
        let edited = draw_ready_editor(&editor, &style, cx);
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();

        let live_point = left_edge_point(&edited).expect("a visible left edge");
        assert_eq!(
            left_edge_point(&scrolled).map(|point| point.column + 100),
            Some(live_point.column)
        );
        let (_, _, _, row, column, _) = editor_db
            .get_scroll_position(item_id, workspace_id)
            .unwrap()
            .expect("a persisted scroll position");
        assert_eq!(row.zip(column), Some((live_point.row, live_point.column)));

        let edited_text = format!("{}{text}", "y".repeat(100));
        let (restored_editor, ref mut cx) = editor_test_context(&edited_text, cx);
        restored_editor.update_in(cx, |editor, window, cx| {
            editor.read_scroll_position_from_db(item_id, workspace_id, window, cx);
        });
        let restored = draw_ready_editor(&restored_editor, &style, cx);

        let position_map = &restored.position_map;
        let saved_x = position_map
            .snapshot
            .long_row(DisplayRow(0))
            .expect("ready row geometry")
            .expect("a long row")
            .x_for_column(live_point.column);
        assert!(
            (position_map.scroll_pixel_position.x - saved_x).abs() <= 0.5,
            "{} != {saved_x}",
            position_map.scroll_pixel_position.x
        );
    }

    #[gpui::test]
    fn test_anchored_block_coordinates_at_far_horizontal_scroll(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let column = 3_999_000;
        let (editor, ref mut cx) = editor_test_context(&"x".repeat(4_000_000), cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let anchored_x = Arc::new(Mutex::new(None::<Pixels>));
        let unanchored_x = Arc::new(Mutex::new(None::<(Pixels, Pixels)>));
        editor.update_in(cx, |editor, window, cx| {
            let anchor = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_after(MultiBufferOffset(column));
            let render_anchored_x = anchored_x.clone();
            let render_unanchored_x = unanchored_x.clone();
            editor.insert_blocks(
                [
                    BlockProperties {
                        style: BlockStyle::FlexWithAnchor,
                        placement: BlockPlacement::Below(anchor),
                        height: Some(1),
                        render: Arc::new(move |cx| {
                            let anchored_x = render_anchored_x.clone();
                            div()
                                .pl(cx.anchor_x)
                                .child(
                                    gpui::canvas(
                                        move |bounds, _, _| {
                                            *anchored_x.lock() = Some(bounds.left())
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .size(px(4.)),
                                )
                                .into_any()
                        }),
                        priority: 0,
                    },
                    BlockProperties {
                        style: BlockStyle::Flex,
                        placement: BlockPlacement::Below(anchor),
                        height: Some(1),
                        render: Arc::new(move |cx| {
                            let unanchored_x = render_unanchored_x.clone();
                            let gutter_width = cx.margins.gutter.full_width();
                            div()
                                .w_full()
                                .pl(gutter_width)
                                .child(
                                    gpui::canvas(
                                        move |bounds, _, _| {
                                            *unanchored_x.lock() =
                                                Some((bounds.left(), gutter_width))
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .w_full()
                                    .h(cx.line_height),
                                )
                                .into_any()
                        }),
                        priority: 1,
                    },
                ],
                None,
                cx,
            );
            editor.set_scroll_position(point(column as ScrollOffset - 10.3, 0.), window, cx);
        });

        let state = draw_ready_editor(&editor, &style, cx);
        let position_map = &state.position_map;
        let expected_x = position_map.content_origin.x
            + Pixels::from(
                position_map.line_layouts[0].x_for_index(column)
                    - position_map.scroll_pixel_position.x,
            );
        let line = &position_map.line_layouts[0];
        let glyph_x = painted_glyphs(line)
            .into_iter()
            .find_map(|(_, _, start, x, _)| (start == column).then_some(x))
            .expect("the anchor glyph was retained");
        assert!(position_map.scroll_pixel_position.x > 16_777_216.);
        assert_eq!(
            glyph_x - position_map.scroll_pixel_position.x,
            line.x_for_index(column) - position_map.scroll_pixel_position.x
        );
        let anchored_x = anchored_x.lock().expect("the anchored block was painted");
        assert!(
            (anchored_x - expected_x).abs() <= px(0.5),
            "{anchored_x:?} != {expected_x:?}"
        );
        let pixel_point = editor
            .update_in(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                editor.display_to_pixel_point(
                    DisplayPoint::new(DisplayRow(0), column as u32),
                    &snapshot,
                    window,
                    cx,
                )
            })
            .expect("the anchor row is visible");
        let pixel_x = position_map.content_origin.x + pixel_point.x;
        assert!(
            (pixel_x - expected_x).abs() <= px(0.01),
            "{pixel_x:?} != {expected_x:?}"
        );
        let (unanchored_x, gutter_width) = unanchored_x
            .lock()
            .expect("the unanchored block was painted");
        assert_eq!(
            unanchored_x,
            state.hitbox.origin.x
                + Pixels::from(-position_map.scroll_pixel_position.x)
                + gutter_width,
        );
    }

    #[gpui::test]
    fn test_blocks_see_selections_outside_the_painted_columns(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) =
            editor_test_context(&format!("{}\nx\n", "x".repeat(100_000)), cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let selected = Arc::new(Mutex::new(None::<bool>));
        editor.update_in(cx, |editor, window, cx| {
            let anchor = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_after(Point::new(0, 90_001));
            let render_selected = selected.clone();
            editor.insert_blocks(
                [BlockProperties {
                    style: BlockStyle::Sticky,
                    placement: BlockPlacement::Above(anchor),
                    height: Some(1),
                    render: Arc::new(move |cx| {
                        *render_selected.lock() = Some(cx.selected);
                        div().into_any()
                    }),
                    priority: 0,
                }],
                None,
                cx,
            );
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([
                    Point::new(0, 90_000)..Point::new(0, 90_002),
                    Point::new(1, 0)..Point::new(1, 0),
                ])
            });
        });

        draw_ready_editor(&editor, &style, cx);
        assert_eq!(*selected.lock(), Some(true));
    }

    #[gpui::test]
    fn test_renderer_inlays_are_measured_once(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for row_len in [2_000, 100] {
            let offscreen_row = 300;
            let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
                let row = "x".repeat(row_len);
                let buffer = MultiBuffer::build_simple(
                    &format!("{row}\n").repeat(offscreen_row as usize + 1),
                    cx,
                );
                Editor::new(EditorMode::full(), buffer, None, window, cx)
            });
            let cx = &mut VisualTestContext::from_window(*window, cx);
            let editor = window.root(cx).unwrap();
            let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            let inlay_start = 60;
            let inlay_text = "r".repeat(200);
            editor.update(cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).snapshot(cx);
                let inlays = [0, offscreen_row]
                    .into_iter()
                    .enumerate()
                    .map(|(id, row)| {
                        let position = buffer.anchor_after(Point::new(row, inlay_start as u32));
                        Inlay::mock_repl_result(id, position, &inlay_text)
                    })
                    .collect();
                editor.splice_inlays(&[], inlays, cx);
            });
            cx.run_until_parked();
            draw_editor_frame(&editor, &style, cx);
            cx.run_until_parked();
            editor.update_in(cx, |editor, window, cx| {
                editor.set_scroll_position(point(inlay_start as f64 + 2., 0.), window, cx);
            });
            let state = draw_editor(&editor, &style, cx);

            let line = &state.position_map.line_layouts[0];
            let elements = line
                .fragments
                .iter()
                .filter_map(|fragment| match fragment {
                    LineFragment::Element { len, size, .. } => Some((*len, size.width)),
                    LineFragment::Text(_) => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(elements.len(), 1, "row of {row_len}: {elements:?}");
            let (element_len, element_width) = elements[0];
            assert_eq!(element_len, inlay_text.len());
            let element_left = state.content_origin.x
                + Pixels::from(
                    line.x_for_index(inlay_start) - state.position_map.scroll_pixel_position.x,
                );
            let bounds = state.position_map.text_hitbox.bounds;
            assert!(element_left < bounds.left());
            assert!(element_left + element_width > bounds.right());
            assert_eq!(
                line.x_for_index(inlay_start + inlay_text.len()) - line.x_for_index(inlay_start),
                f64::from(element_width),
            );

            let (snapshot, details) = editor.update_in(cx, |editor, window, cx| {
                (
                    editor.snapshot(window, cx),
                    editor.text_layout_details(window, cx),
                )
            });
            assert!(
                state.position_map.line_layouts.len() < offscreen_row as usize,
                "row {offscreen_row} must stay unpainted"
            );
            let geometry_lease = editor.update(cx, |editor, cx| {
                editor.display_map.update(cx, |map, cx| {
                    let lease = map.retain_row_geometry();
                    if let Err(pending) = snapshot.long_row(DisplayRow(offscreen_row)) {
                        map.request_row_geometry(pending, cx);
                    }
                    lease
                })
            });
            cx.run_until_parked();
            for row in [0, offscreen_row] {
                let row_x = |column: usize| {
                    snapshot
                        .x_for_display_point(
                            DisplayPoint::new(DisplayRow(row), column as u32),
                            &details,
                        )
                        .expect("ready row geometry")
                };
                let inlay_width = row_x(inlay_start + inlay_text.len()) - row_x(inlay_start);
                assert_eq!(
                    inlay_width,
                    f64::from(element_width),
                    "row {row} of {row_len}"
                );
                for column in inlay_start..inlay_start + inlay_text.len() {
                    assert_eq!(
                        row_x(column),
                        line.x_for_index(column),
                        "row {row}, column {column}"
                    );
                    if row_len == 100 {
                        assert_eq!(row_x(column), row_x(inlay_start));
                    }
                }
                if row_len == 100 {
                    let layout = snapshot
                        .layout_row(DisplayRow(row), &details)
                        .expect("ready row geometry");
                    assert_eq!(
                        layout.closest_index_for_x(row_x(inlay_start) + inlay_width / 4.),
                        inlay_start,
                    );
                    for fraction in [0.25, 0.75] {
                        assert_eq!(
                            layout.index_for_x(row_x(inlay_start) + inlay_width * fraction),
                            Some(inlay_start)
                        );
                    }
                    assert_eq!(
                        layout.index_for_x(row_x(inlay_start + inlay_text.len())),
                        Some(inlay_start + inlay_text.len())
                    );
                }
            }
            drop(geometry_lease);

            if row_len == 100 {
                editor.update_in(cx, |editor, window, cx| {
                    window.focus(&editor.focus_handle(cx), cx);
                    editor.set_cursor_shape(CursorShape::Block, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            let cursor = Point::new(0, inlay_start as u32);
                            selections.select_ranges([cursor..cursor]);
                        },
                    );
                    editor.set_scroll_position(point(inlay_start as f64 - 1., 0.), window, cx);
                });
                let state = draw_editor(&editor, &style, cx);
                assert_eq!(
                    state
                        .visible_cursors
                        .iter()
                        .map(|cursor| cursor.block_width)
                        .collect::<Vec<_>>(),
                    [state.position_map.em_advance],
                );
                editor.update_in(cx, |editor, window, cx| {
                    let snapshot = editor.display_snapshot(cx);
                    let details = editor.text_layout_details(window, cx);
                    let selection = editor.selections.newest_display(&snapshot);
                    assert_eq!(
                        selection.head(),
                        DisplayPoint::new(DisplayRow(0), inlay_start as u32)
                    );
                    assert_eq!(
                        editor
                            .painted_viewport(&snapshot, window, cx)
                            .expect("painted editor")
                            .clip_selection(
                                &snapshot,
                                &selection,
                                selection.head(),
                                &details,
                                window,
                                cx
                            )
                            .expect("ready row geometry"),
                        Some(selection.head()),
                    );
                });
            }
        }
    }

    #[gpui::test]
    fn test_offscreen_renderer_widths_follow_layout_dimensions(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let text = format!("{}{}", "x\n".repeat(300), "x".repeat(2_000));
        let (editor, ref mut cx) = editor_test_context(&text, cx);
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            let editor_handle = cx.entity().downgrade();
            editor.display_map.update(cx, |map, _| {
                map.fold_placeholder = FoldPlaceholder {
                    render: Arc::new(move |_, _, _| {
                        DimensionDependentFold(editor_handle.clone()).into_any_element()
                    }),
                    constrain_width: false,
                    ..FoldPlaceholder::test()
                };
            });
            editor.fold_ranges(
                vec![Point::new(300, 1)..Point::new(300, 2)],
                false,
                window,
                cx,
            );
        });

        for (width, line_height, expected) in [
            (500., 20., 270.),
            (800., 20., 420.),
            (800., 40., 440.),
            (500., 40., 290.),
            (500., 20., 270.),
        ] {
            style.text.line_height = px(line_height).into();
            let state = draw_ready_editor_at(
                &editor,
                &style,
                Bounds::new(point(px(0.), px(0.)), size(px(width), px(500.))),
                cx,
            );
            assert!(state.position_map.visible_row_range.end <= DisplayRow(300));
            let geometry_lease = editor.update(cx, |editor, cx| {
                editor.display_map.update(cx, |map, cx| {
                    let lease = map.retain_row_geometry();
                    if let Err(pending) = state.position_map.snapshot.long_row(DisplayRow(300)) {
                        map.request_row_geometry(pending, cx);
                    }
                    lease
                })
            });
            cx.run_until_parked();
            let row = state
                .position_map
                .snapshot
                .long_row(DisplayRow(300))
                .expect("ready row geometry")
                .expect("an offscreen long row");
            assert_eq!(
                row.x_for_column(1 + "⋯".len() as u32) - row.x_for_column(1),
                expected,
                "editor width {width}, line height {line_height}",
            );
            drop(geometry_lease);
        }
    }

    #[gpui::test]
    fn test_offscreen_renderers_are_measured_within_a_per_frame_budget(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        assert_offscreen_renderers_measured_per_frame(
            RENDERER_MEASURE_BUDGET_PER_PREPAINT / (RENDERER_VISIT_COST + RENDERER_LAYOUT_COST),
            |id, position| Inlay::mock_repl_result(id, position, "r"),
            cx,
        );
    }

    #[gpui::test]
    fn test_offscreen_renderers_sharing_a_width_class_are_measured_within_a_per_frame_budget(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        assert_offscreen_renderers_measured_per_frame(
            (RENDERER_MEASURE_BUDGET_PER_PREPAINT - RENDERER_LAYOUT_COST) / RENDERER_VISIT_COST,
            |id, position| Inlay::color(id, position, gpui::rgba(0xff0000ff)),
            cx,
        );
    }

    #[gpui::test]
    async fn test_sticky_long_rows_use_measured_renderer_widths(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.sticky_scroll = Some(settings::StickyScrollContent {
                enabled: Some(true),
            });
        });
        let header = format!("fn foo() {{ // {}", "x".repeat(1_100));
        let text = format!("{header}\n{}}}\n", "    let one = 1;\n".repeat(40));
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx);
            buffer.set_language(Some(languages::rust_lang()), cx);
            buffer
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let window = cx.add_window(|window, cx| {
            Editor::new(EditorMode::full(), multi_buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            editor.refresh_sticky_headers(&snapshot.display_snapshot, cx);
        });
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(0., 10.), window, cx);
        });
        cx.run_until_parked();
        let inlay_start = 3;
        let inlay_text = "rrrr";
        editor.update(cx, |editor, cx| {
            let position = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_after(MultiBufferOffset(inlay_start));
            editor.splice_inlays(
                &[],
                vec![Inlay::mock_repl_result(0, position, inlay_text)],
                cx,
            );
        });
        cx.run_until_parked();

        let state = draw_ready_editor(&editor, &style, cx);
        let line = &state
            .sticky_headers
            .as_ref()
            .expect("a sticky header")
            .lines[0]
            .line;
        assert!(line.long_row.is_some());
        let element_width = line
            .fragments
            .iter()
            .find_map(|fragment| match fragment {
                LineFragment::Element { size, .. } => Some(size.width),
                LineFragment::Text(_) => None,
            })
            .expect("a rendered inlay");
        let inlay_width =
            line.x_for_index(inlay_start + inlay_text.len()) - line.x_for_index(inlay_start);
        assert!(
            (inlay_width - f64::from(element_width)).abs() < 0.01,
            "{inlay_width} != {element_width:?}"
        );
    }

    #[test]
    fn test_long_rows_paint_kerning_ligatures_and_styles_at_row_geometry() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |settings| {
            settings.defaults.show_wrap_guides = Some(true);
            settings.defaults.wrap_guides = Some(vec![40]);
        });
        update_test_editor_settings(&mut cx, &|settings| {
            settings.minimum_contrast_for_highlights = Some(settings::MinimumContrast(106.));
            settings.scrollbar.get_or_insert_default().show = Some(settings::ShowScrollbar::Always);
        });
        let invisible_column = 30;
        let styled_invisible_count = 5;
        let styled_invisible_len = '\u{a0}'.len_utf8();
        let styled_invisibles = 40..40 + styled_invisible_count * styled_invisible_len;
        let text = format!(
            "AVA fi {}\u{200b}{}{}{}",
            "x".repeat(invisible_column - 7),
            "x".repeat(styled_invisibles.start - invisible_column - '\u{200b}'.len_utf8()),
            "\u{a0}".repeat(styled_invisible_count),
            "x".repeat(1_100)
        );
        let bold_columns = 10..20;
        let italic_columns = 20..30;
        let (editor, ref mut cx) = editor_test_context(&text, &mut cx);
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.scrollbar_width = EditorElement::SCROLLBAR_WIDTH;
        editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let bold_ranges = [bold_columns.clone(), styled_invisibles.clone()]
                .into_iter()
                .map(|columns| {
                    buffer.anchor_after(MultiBufferOffset(columns.start))
                        ..buffer.anchor_before(MultiBufferOffset(columns.end))
                })
                .collect();
            editor.highlight_text(
                HighlightKey::Editor,
                bold_ranges,
                HighlightStyle {
                    font_weight: Some(FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
                cx,
            );
            let range = buffer.anchor_after(MultiBufferOffset(italic_columns.start))
                ..buffer.anchor_before(MultiBufferOffset(italic_columns.end));
            editor.highlight_text(
                HighlightKey::SelectedTextHighlight,
                vec![range],
                HighlightStyle {
                    font_style: Some(FontStyle::Italic),
                    ..HighlightStyle::default()
                },
                cx,
            );
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([MultiBufferOffset(5)..MultiBufferOffset(6)])
            });
        });
        cx.run_until_parked();
        let state = draw_editor(&editor, &style, cx);
        let (snapshot, details) = editor.update_in(cx, |editor, window, cx| {
            (
                editor.snapshot(window, cx),
                editor.text_layout_details(window, cx),
            )
        });
        let row_x = |column: usize| {
            snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(0), column as u32), &details)
                .expect("ready row geometry")
        };
        let advance = f64::from(style.text.font_size.to_pixels(px(16.)))
            * f64::from(ShapingTextSystem::ADVANCE);

        let line = &state.position_map.line_layouts[0];
        assert!(line.long_row.is_some());
        let text_metrics = line
            .fragments
            .iter()
            .filter_map(|fragment| match fragment {
                LineFragment::Text(shaped) => Some((shaped.ascent, shaped.descent)),
                LineFragment::Element { .. } => None,
            })
            .collect::<Vec<_>>();
        let row_metrics = snapshot
            .long_row(DisplayRow(0))
            .expect("ready row geometry")
            .expect("a long row")
            .metrics();
        assert_eq!(
            text_metrics,
            vec![(row_metrics.ascent, row_metrics.descent); text_metrics.len()]
        );
        let mut shaped = ShapedLine::default();
        let layout_pointer = Arc::as_ptr(&shaped);
        apply_row_metrics(&mut shaped, row_metrics);
        assert_eq!(
            (Arc::as_ptr(&shaped), shaped.ascent, shaped.descent),
            (layout_pointer, row_metrics.ascent, row_metrics.descent)
        );
        let painted = painted_glyphs(line)
            .into_iter()
            .map(|(font_id, _, column, x, _)| (column, x, font_id))
            .collect::<Vec<_>>();
        let kerning = f64::from(ShapingTextSystem::KERNING);
        assert_eq!(
            painted
                .iter()
                .filter(|(column, ..)| *column < 7)
                .map(|(column, x, _)| (*column, *x))
                .collect::<Vec<_>>(),
            [
                (0, 0.),
                (1, advance - kerning),
                (2, 2. * advance - kerning),
                (3, 3. * advance - kerning),
                (4, 4. * advance - kerning),
                (6, 5.5 * advance - kerning),
            ]
        );
        assert_eq!(
            [row_x(5) - row_x(4), row_x(6) - row_x(5)].map(|width| (width * 100.).round() / 100.),
            [0.75 * advance, 0.75 * advance]
        );
        let bold_extra = f64::from(ShapingTextSystem::BOLD_EXTRA);
        let bold_advance = advance + bold_extra;
        for (columns, font) in [
            (bold_columns.clone(), ShapingTextSystem::BOLD),
            (italic_columns, ShapingTextSystem::ITALIC),
        ] {
            assert_eq!(
                painted
                    .iter()
                    .filter(|(column, ..)| columns.contains(column))
                    .copied()
                    .collect::<Vec<_>>(),
                columns
                    .map(|column| {
                        (
                            column,
                            (column as f64 - 0.5) * advance - kerning
                                + column
                                    .saturating_sub(bold_columns.start)
                                    .min(bold_columns.len())
                                    as f64
                                    * bold_extra,
                            font,
                        )
                    })
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(
            painted
                .iter()
                .filter(|(column, ..)| *column == invisible_column)
                .count(),
            1
        );

        let invisible_byte_overhead = '\u{200b}'.len_utf8() - 1;
        let first_invisible_x =
            (styled_invisibles.start as f64 - invisible_byte_overhead as f64 - 0.5) * advance
                - kerning
                + bold_columns.len() as f64 * bold_extra;
        let expected_invisibles = (0..styled_invisible_count)
            .map(|index| {
                (
                    styled_invisibles.start + index * styled_invisible_len,
                    first_invisible_x + index as f64 * bold_advance,
                    bold_advance,
                )
            })
            .collect::<Vec<_>>();
        for &(column, x, width) in &expected_invisibles {
            assert_eq!(
                [row_x(column), row_x(column + styled_invisible_len)],
                [x, x + width],
            );
        }
        let bounds = state.position_map.text_hitbox.bounds;
        let clipped_scroll_x = first_invisible_x
            + bold_advance / 2.
            + f64::from(state.content_origin.x - bounds.left());
        for scroll_x in [0., clipped_scroll_x] {
            editor.update_in(cx, |editor, window, cx| {
                editor.set_scroll_position(
                    point(scroll_x / f64::from(state.position_map.em_layout_width), 0.),
                    window,
                    cx,
                );
            });
            let state = draw_editor(&editor, &style, cx);
            let line = &state.position_map.line_layouts[0];
            assert_eq!(
                line.fragment_positions()
                    .filter(|(_, start, _)| styled_invisibles.contains(start))
                    .map(|(fragment, start, x)| (start, x, f64::from(fragment.width())))
                    .collect::<Vec<_>>(),
                expected_invisibles,
            );
            if scroll_x > 0. {
                let first_left = state.content_origin.x
                    + Pixels::from(first_invisible_x - state.position_map.scroll_pixel_position.x);
                assert!(first_left < bounds.left());
                assert!(first_left + Pixels::from(bold_advance) > bounds.left());
            }
        }

        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(0., 0.), window, cx);
            window.focus(&editor.focus_handle(cx), cx);
            editor.set_cursor_shape(CursorShape::Block, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                let cursor = MultiBufferOffset(bold_columns.start + 1);
                selections.select_ranges([cursor..cursor])
            });
        });
        let state = draw_editor(&editor, &style, cx);
        let block_text_metrics = state
            .visible_cursors
            .iter()
            .filter_map(|cursor| {
                cursor.block_text.as_ref().or_else(|| {
                    cursor
                        .retained_block_text
                        .fragments
                        .first()
                        .map(|(_, text)| text)
                })
            })
            .map(|text| (text.ascent, text.descent))
            .collect::<Vec<_>>();
        assert_eq!(
            block_text_metrics,
            [(row_metrics.ascent, row_metrics.descent)]
        );

        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(10., 0.), window, cx);
        });
        let scrolled = draw_editor(&editor, &style, cx);
        assert_eq!(
            scrolled.wrap_guides.as_slice(),
            &[(
                scrolled.content_origin.x + Pixels::from(30. * advance),
                false
            )]
        );
        cx.draw(point(px(0.), px(0.)), size(px(500.), px(500.)), |_, _| {
            gpui::canvas(
                move |bounds, window, cx| {
                    EditorElement::new(&editor, style).prepaint(
                        None,
                        None,
                        bounds,
                        &mut EditorRequestLayoutState::default(),
                        window,
                        cx,
                    )
                },
                |_, layout, _, _| {
                    let layout = ready_layout(layout);
                    let scrollbar = layout
                        .scrollbars_layout
                        .as_ref()
                        .and_then(|scrollbars| scrollbars.horizontal.as_ref())
                        .expect("horizontal scrollbar");
                    let thumb = scrollbar.thumb_bounds.expect("visible thumb");
                    let remaining_travel = (layout.position_map.scroll_max.x
                        - layout.position_map.scroll_position.x)
                        * f64::from(scrollbar.text_unit_size);
                    let end = thumb.right() + Pixels::from(remaining_travel);
                    assert!((end - scrollbar.hitbox.bounds.right()).abs() <= px(1.));
                },
            )
            .size_full()
        });
    }

    #[test]
    fn test_short_rows_paint_at_movement_geometry() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        update_test_editor_settings(&mut cx, &|settings| {
            settings.minimum_contrast_for_highlights = Some(settings::MinimumContrast(106.));
        });
        let syntax = theme::SyntaxTheme::new_test_styles([(
            "keyword",
            HighlightStyle {
                font_weight: Some(FontWeight::BOLD),
                ..HighlightStyle::default()
            },
        )]);
        let language = languages::rust_lang();
        language.set_theme(&syntax);
        let text = "fn fi() {}";
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx);
            buffer.set_language(Some(language), cx);
            buffer
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let window = cx.add_window(|window, cx| {
            Editor::new(EditorMode::full(), multi_buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, &mut cx);
        let editor = window.root(cx).unwrap();
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.syntax = Arc::new(syntax);
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([MultiBufferOffset(3)..MultiBufferOffset(4)])
            });
        });
        cx.run_until_parked();
        let state = draw_editor(&editor, &style, cx);
        let (snapshot, details) = editor.update_in(cx, |editor, window, cx| {
            (
                editor.snapshot(window, cx),
                editor.text_layout_details(window, cx),
            )
        });

        let details = crate::movement::TextLayoutDetails {
            editor_style: style.clone(),
            ..details
        };
        let line = &state.position_map.line_layouts[0];
        assert!(line.long_row.is_none());
        let painted = painted_glyphs(line)
            .into_iter()
            .map(|(font_id, _, column, x, _)| (column, x, font_id))
            .collect::<Vec<_>>();
        assert_eq!(
            painted
                .iter()
                .filter(|(column, ..)| *column < 2)
                .map(|(.., font_id)| *font_id)
                .collect::<Vec<_>>(),
            [ShapingTextSystem::BOLD; 2]
        );
        assert_eq!(
            painted
                .iter()
                .filter(|(column, ..)| (3..5).contains(column))
                .map(|(column, ..)| *column)
                .collect::<Vec<_>>(),
            [3]
        );
        for (column, x, _) in &painted {
            let expected = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(0), *column as u32), &details)
                .expect("ready row geometry");
            assert!(
                (x - expected).abs() < 0.01,
                "glyph at column {column}: {x} != {expected}"
            );
        }
    }

    #[test]
    fn test_highlights_are_clipped_to_painted_long_row_columns() {
        let point = |row, column| DisplayPoint::new(DisplayRow(row), column);
        let color = Hsla::default();
        let mut ranges = vec![
            (point(0, 10)..point(0, 20), color),
            (point(0, 500)..point(0, 900), color),
            (point(0, 1_500)..point(0, 1_600), color),
            (point(0, 1_500)..point(2, 5), color),
            (point(0, 5)..point(1, 0), color),
            (point(1, 3)..point(2, 50), color),
        ];
        clip_ranges_to_columns(
            &mut ranges,
            |(range, _)| range,
            |row| (row == DisplayRow(0)).then_some(400..1_000),
        );
        assert_eq!(
            ranges,
            [
                (point(0, 500)..point(0, 900), color),
                (point(1, 0)..point(2, 5), color),
                (point(0, 400)..point(1, 0), color),
                (point(1, 3)..point(2, 50), color),
            ]
        );
    }

    #[gpui::test]
    fn test_navigation_overlay_covered_text_highlights_are_replaced(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple("overlay replacement", cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let editor = window.root(cx).unwrap();

        editor.update(cx, |editor, cx| {
            let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let target_start = buffer_snapshot.anchor_after(Point::new(0, 0));
            let target_end = buffer_snapshot.anchor_after(Point::new(0, 7));
            let covered_text_end = buffer_snapshot.anchor_after(Point::new(0, 2));

            editor.set_navigation_overlays(
                PRIMARY_NAVIGATION_OVERLAY_KEY,
                vec![navigation_overlay(
                    "ov",
                    target_start..target_end,
                    Some(target_start..covered_text_end),
                )],
                cx,
            );
            assert!(
                editor
                    .text_highlights(
                        HighlightKey::NavigationOverlay(PRIMARY_NAVIGATION_OVERLAY_KEY),
                        cx,
                    )
                    .is_some()
            );

            editor.set_navigation_overlays(
                PRIMARY_NAVIGATION_OVERLAY_KEY,
                vec![navigation_overlay("ov", target_start..target_end, None)],
                cx,
            );
            assert!(
                editor
                    .text_highlights(
                        HighlightKey::NavigationOverlay(PRIMARY_NAVIGATION_OVERLAY_KEY),
                        cx,
                    )
                    .is_none()
            );
        });
    }

    #[gpui::test]
    async fn test_navigation_overlay_repositions_when_editor_width_changes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        let text = "jump target overlay ".repeat(16);
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&text, cx);
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
            editor
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();

        editor.update(cx, |editor, cx| {
            let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let target_start = buffer_snapshot.anchor_after(Point::new(0, 30));
            let target_end = buffer_snapshot.anchor_after(Point::new(0, 40));

            editor.set_navigation_overlays(
                PRIMARY_NAVIGATION_OVERLAY_KEY,
                vec![navigation_overlay("jj", target_start..target_end, None)],
                cx,
            );
        });

        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));
        let (_, wide_state) = cx.draw(Default::default(), size(px(520.), px(260.)), |_, _| {
            EditorElement::new(&editor, style.clone())
        });
        let wide_state = ready_layout(wide_state);
        let (_, narrow_state) = cx.draw(Default::default(), size(px(140.), px(260.)), |_, _| {
            EditorElement::new(&editor, style.clone())
        });
        let narrow_state = ready_layout(narrow_state);

        let wide_label_layouts = navigation_label_layouts(&wide_state);
        let narrow_label_layouts = navigation_label_layouts(&narrow_state);

        assert_eq!(wide_label_layouts.len(), 1);
        assert_eq!(narrow_label_layouts.len(), 1);

        let wide_label_origin = wide_label_layouts[0].origin;
        let narrow_label_origin = narrow_label_layouts[0].origin;

        assert!(
            narrow_label_origin.y > wide_label_origin.y,
            "expected inline label to move to a later wrapped row when the editor narrows"
        );
        assert!(
            narrow_label_origin.x < wide_label_origin.x,
            "expected inline label to recompute its horizontal position for the wrapped row"
        );
    }

    #[gpui::test]
    fn test_layout_line_numbers(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
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
                    &test_gutter(line_height, &snapshot),
                    &BTreeMap::default(),
                    Some(DisplayRow(0)),
                    window,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(0)..DisplayRow(6)),
                    DisplayRow(3),
                    false,
                )
            })
            .unwrap();
        assert_eq!(relative_rows[&DisplayRow(0)], 3);
        assert_eq!(relative_rows[&DisplayRow(1)], 2);
        assert_eq!(relative_rows[&DisplayRow(2)], 1);
        // current line has no relative number
        assert!(!relative_rows.contains_key(&DisplayRow(3)));
        assert_eq!(relative_rows[&DisplayRow(4)], 1);
        assert_eq!(relative_rows[&DisplayRow(5)], 2);

        // works if cursor is before screen
        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(3)..DisplayRow(6)),
                    DisplayRow(1),
                    false,
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
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(0)..DisplayRow(3)),
                    DisplayRow(6),
                    false,
                )
            })
            .unwrap();
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&DisplayRow(0)], 5);
        assert_eq!(relative_rows[&DisplayRow(1)], 4);
        assert_eq!(relative_rows[&DisplayRow(2)], 3);

        let gutter = Gutter {
            row_infos: &(0..6)
                .map(|row| RowInfo {
                    buffer_row: Some(row),
                    diff_status: (row == DELETED_LINE).then(|| {
                        DiffHunkStatus::deleted(
                            buffer_diff::DiffHunkSecondaryStatus::NoSecondaryHunk,
                        )
                    }),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..test_gutter(line_height, &snapshot)
        };

        const DELETED_LINE: u32 = 3;
        let layouts = cx
            .update_window(*window, |_, window, cx| {
                element.layout_line_numbers(
                    &gutter,
                    &BTreeMap::default(),
                    Some(DisplayRow(0)),
                    window,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 5,);
        assert!(
            layouts.get(&MultiBufferRow(DELETED_LINE)).is_none(),
            "Deleted line should not have a line number"
        );
    }

    #[gpui::test]
    async fn test_layout_line_numbers_with_folded_lines(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let python_lang = languages::language("python", tree_sitter_python::LANGUAGE.into());

        let window = cx.add_window(|window, cx| {
            let buffer = cx.new(|cx| {
                Buffer::local(
                    indoc::indoc! {"
                        fn test() -> int {
                            return 2;
                        }

                        fn another_test() -> int {
                            # This is a very peculiar method that is hard to grasp.
                            return 4;
                        }
                    "},
                    cx,
                )
                .with_language(python_lang, cx)
            });

            let buffer = MultiBuffer::build_from_buffer(buffer, cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let line_height = window
            .update(cx, |_, window, _| {
                style.text.line_height_in_pixels(window.rem_size())
            })
            .unwrap();
        let element = EditorElement::new(&editor, style);
        let snapshot = window
            .update(cx, |editor, window, cx| {
                editor.fold_at(MultiBufferRow(0), window, cx);
                editor.snapshot(window, cx)
            })
            .unwrap();

        let layouts = cx
            .update_window(*window, |_, window, cx| {
                element.layout_line_numbers(
                    &test_gutter(line_height, &snapshot),
                    &BTreeMap::default(),
                    Some(DisplayRow(3)),
                    window,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(0)..DisplayRow(6)),
                    DisplayRow(3),
                    false,
                )
            })
            .unwrap();
        assert_eq!(relative_rows[&DisplayRow(0)], 3);
        assert_eq!(relative_rows[&DisplayRow(1)], 2);
        assert_eq!(relative_rows[&DisplayRow(2)], 1);
        // current line has no relative number
        assert!(!relative_rows.contains_key(&DisplayRow(3)));
        assert_eq!(relative_rows[&DisplayRow(4)], 1);
        assert_eq!(relative_rows[&DisplayRow(5)], 2);
    }

    #[gpui::test]
    fn test_layout_line_numbers_wrapping(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        update_test_language_settings(cx, &|s| {
            s.defaults.preferred_line_length = Some(5_u32);
            s.defaults.soft_wrap = Some(language_settings::SoftWrap::Bounded);
        });

        let editor = window.root(cx).unwrap();
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
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
                    &test_gutter(line_height, &snapshot),
                    &BTreeMap::default(),
                    Some(DisplayRow(0)),
                    window,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 3);

        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(0)..DisplayRow(6)),
                    DisplayRow(3),
                    true,
                )
            })
            .unwrap();

        assert_eq!(relative_rows[&DisplayRow(0)], 3);
        assert_eq!(relative_rows[&DisplayRow(1)], 2);
        assert_eq!(relative_rows[&DisplayRow(2)], 1);
        // current line has no relative number
        assert!(!relative_rows.contains_key(&DisplayRow(3)));
        assert_eq!(relative_rows[&DisplayRow(4)], 1);
        assert_eq!(relative_rows[&DisplayRow(5)], 2);

        let layouts = cx
            .update_window(*window, |_, window, cx| {
                element.layout_line_numbers(
                    &Gutter {
                        row_infos: &(0..6)
                            .map(|row| RowInfo {
                                buffer_row: Some(row),
                                diff_status: Some(DiffHunkStatus::deleted(
                                    buffer_diff::DiffHunkSecondaryStatus::NoSecondaryHunk,
                                )),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>(),
                        ..test_gutter(line_height, &snapshot)
                    },
                    &BTreeMap::from_iter([(DisplayRow(0), LineHighlightSpec::default())]),
                    Some(DisplayRow(0)),
                    window,
                    cx,
                )
            })
            .unwrap();
        assert!(
            layouts.is_empty(),
            "Deleted lines should have no line number"
        );

        let relative_rows = window
            .update(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot.calculate_relative_line_numbers(
                    &(DisplayRow(0)..DisplayRow(6)),
                    DisplayRow(3),
                    true,
                )
            })
            .unwrap();

        // Deleted lines should still have relative numbers
        assert_eq!(relative_rows[&DisplayRow(0)], 3);
        assert_eq!(relative_rows[&DisplayRow(1)], 2);
        assert_eq!(relative_rows[&DisplayRow(2)], 1);
        // current line, even if deleted, has no relative number
        assert!(!relative_rows.contains_key(&DisplayRow(3)));
        assert_eq!(relative_rows[&DisplayRow(4)], 1);
        assert_eq!(relative_rows[&DisplayRow(5)], 2);
    }

    #[gpui::test]
    async fn test_relative_line_numbers_after_scrolling_wrapped_line(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });

        update_test_language_settings(
            cx,
            &|settings: &mut settings::AllLanguageSettingsContent| {
                settings.defaults.soft_wrap = Some(language_settings::SoftWrap::Bounded);
                settings.defaults.preferred_line_length = Some(10);
            },
        );

        window
            .update(cx, |editor, _window, cx| {
                let text = format!("{}\nshort line", "a".repeat(100));
                editor.buffer.update(cx, |buffer, cx| {
                    buffer.edit([(Point::default()..Point::default(), text)], None, cx);
                });
            })
            .unwrap();
        cx.run_until_parked();

        window
            .update(cx, |editor, _window, cx| {
                let snapshot = editor.snapshot(_window, cx);

                let line_1_display_row = Point::new(1, 0).to_display_point(&snapshot).row();
                assert!(
                    line_1_display_row.0 > 1,
                    "Line 0 should wrap into multiple rows"
                );

                let start_row = DisplayRow(1);
                let relative_rows = snapshot.calculate_relative_line_numbers(
                    &(start_row..line_1_display_row.next_row()),
                    line_1_display_row,
                    false,
                );

                // If the bug exists, line_1_display_row would have a non-zero relative number
                // and would be included in the map. It should be 0 (and thus omitted).
                assert!(!relative_rows.contains_key(&line_1_display_row));
            })
            .unwrap();
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
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));

        window
            .update(cx, |editor, window, cx| {
                editor.cursor_offset_on_selection = true;
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
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
        let state = ready_layout(state);

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

        // displays the trailing newline cursor on the preceding row
        assert_eq!(
            local_selections[2].range,
            DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(6), 0)
        );
        assert_eq!(
            local_selections[2].head,
            DisplayPoint::new(DisplayRow(5), 6)
        );

        // active lines does not include 1 (even though the range of the selection does)
        assert_eq!(
            state.active_rows.keys().cloned().collect::<Vec<_>>(),
            vec![DisplayRow(0), DisplayRow(3), DisplayRow(5)]
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
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));
        window
            .update(cx, |editor, window, cx| {
                editor.set_placeholder_text("hello", window, cx);
                editor.insert_blocks(
                    [BlockProperties {
                        style: BlockStyle::Fixed,
                        placement: BlockPlacement::Above(Anchor::Min),
                        height: Some(3),
                        render: Arc::new(|cx| div().h(3. * cx.window.line_height()).into_any()),
                        priority: 0,
                    }],
                    None,
                    cx,
                );

                // Blur the editor so that it displays placeholder text.
                window.blur(cx);
            })
            .unwrap();

        let (_, state) = cx.draw(
            point(px(500.), px(500.)),
            size(px(500.), px(500.)),
            |_, _| EditorElement::new(&editor, style),
        );
        let state = ready_layout(state);
        assert_eq!(state.position_map.line_layouts.len(), 4);
        assert_eq!(state.line_numbers.len(), 1);
        assert_eq!(
            state
                .line_numbers
                .get(&MultiBufferRow(0))
                .map(|line_number| line_number
                    .segments
                    .first()
                    .unwrap()
                    .shaped_line
                    .text
                    .as_ref()),
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
                line_start_offset: TAB_SIZE as usize,
                line_end_offset: TAB_SIZE as usize + 1,
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
                line_start_offset: TAB_SIZE as usize * 3 + 1,
                line_end_offset: TAB_SIZE as usize * 3 + 2,
            },
            Invisible::Whitespace {
                line_start_offset: TAB_SIZE as usize * 3 + 3,
                line_end_offset: TAB_SIZE as usize * 3 + 4,
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
    fn test_multibyte_whitespace_uses_utf8_byte_offsets(cx: &mut TestAppContext) {
        init_test(cx, |s| {
            s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
        });

        // Regression test for #49186. NBSP (U+00A0) is rendered via the invisible
        // character `replacement` pipeline, which flushes the internal `line`
        // scratch buffer mid-line. Any whitespace invisible that follows must use
        // the absolute byte offset within the logical line (here: byte 4 for the
        // trailing ASCII space), not an offset relative to the post-flush buffer.
        let actual_invisibles = collect_invisibles_from_new_editor(
            cx,
            EditorMode::full(),
            "a\u{00A0}b ",
            px(500.0),
            false,
        );

        assert_eq!(
            actual_invisibles,
            vec![Invisible::Whitespace {
                line_start_offset: 4,
                line_end_offset: 5,
            }]
        );
    }

    #[gpui::test]
    fn test_replacement_chunks_are_clipped_to_max_line_len(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|_, cx| editor.update(cx, |editor, cx| editor.style(cx).clone()));
        let editor_mode = EditorMode::full();
        let max_line_len = "\u{00a0}abcdef".len();

        window
            .update(cx, |editor, window, cx| {
                let chunks = std::iter::once(HighlightedChunk {
                    text: "\u{00a0}",
                    style: None,
                    diagnostic_underline_severity: None,
                    is_tab: false,
                    is_inlay: false,
                    replacement: Some(ChunkReplacement::Str("\u{2007}".into())),
                })
                .chain(std::iter::once(HighlightedChunk {
                    text: "abcdefghi",
                    style: None,
                    diagnostic_underline_severity: None,
                    is_tab: false,
                    is_inlay: false,
                    replacement: None,
                }))
                .chain(
                    std::iter::repeat_with(|| HighlightedChunk {
                        text: "\u{00a0}",
                        style: None,
                        diagnostic_underline_severity: None,
                        is_tab: false,
                        is_inlay: false,
                        replacement: Some(ChunkReplacement::Str("\u{2007}".into())),
                    })
                    .take(8),
                );

                let snapshot = editor.display_snapshot(cx);
                let layouts = LineWithInvisibles::from_chunks(
                    chunks,
                    &snapshot,
                    DisplayRow(0),
                    &style,
                    max_line_len,
                    1,
                    &editor_mode,
                    px(500.),
                    |_| false,
                    &[],
                    window,
                    cx,
                )
                .expect("text replacements are ready");

                assert_eq!(layouts.len(), 1);
                assert_eq!(layouts[0].len, max_line_len);
                assert!(layouts[0].fragments.len() <= max_line_len);
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_invisibles_dont_appear_in_certain_editors(cx: &mut TestAppContext) {
        init_test(cx, |s| {
            s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
            s.defaults.tab_size = NonZeroU32::new(4);
        });

        for editor_mode_without_invisibles in [
            EditorMode::SingleLine,
            EditorMode::AutoHeight {
                min_lines: 1,
                max_lines: Some(100),
            },
        ] {
            for show_line_numbers in [true, false] {
                let invisibles = collect_invisibles_from_new_editor(
                    cx,
                    editor_mode_without_invisibles.clone(),
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
                line_start_offset: tab_size as usize + 3,
                line_end_offset: tab_size as usize + 4,
            },
            Invisible::Whitespace {
                line_start_offset: tab_size as usize + 4,
                line_end_offset: tab_size as usize + 5,
            },
            Invisible::Whitespace {
                line_start_offset: tab_size as usize + 5,
                line_end_offset: tab_size as usize + 6,
            },
            Invisible::Whitespace {
                line_start_offset: tab_size as usize + 6,
                line_end_offset: tab_size as usize + 7,
            },
            Invisible::Whitespace {
                line_start_offset: tab_size as usize + 7,
                line_end_offset: tab_size as usize + 8,
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
                update_test_language_settings(cx, &|s| {
                    s.defaults.tab_size = NonZeroU32::new(tab_size);
                    s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
                    s.defaults.preferred_line_length = Some(editor_width as u32);
                    s.defaults.soft_wrap = Some(language_settings::SoftWrap::Bounded);
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
            f32::from(editor_width)
        );
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(input_text, cx);
            Editor::new(editor_mode, buffer, None, window, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();

        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
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
        let state = ready_layout(state);
        state
            .position_map
            .line_layouts
            .iter()
            .flat_map(|line_with_invisibles| &line_with_invisibles.invisibles)
            .cloned()
            .collect()
    }

    #[gpui::test]
    fn test_merge_overlapping_ranges() {
        let base_bg = Hsla::white();
        let color1 = Hsla {
            h: 0.0,
            s: 0.5,
            l: 0.5,
            a: 0.5,
        };
        let color2 = Hsla {
            h: 120.0,
            s: 0.5,
            l: 0.5,
            a: 0.5,
        };

        let display_point = |col| DisplayPoint::new(DisplayRow(0), col);
        let cols = |v: &Vec<(Range<DisplayPoint>, Hsla)>| -> Vec<(u32, u32)> {
            v.iter()
                .map(|(r, _)| (r.start.column(), r.end.column()))
                .collect()
        };

        // Test overlapping ranges blend colors
        let overlapping = vec![
            (display_point(5)..display_point(15), color1),
            (display_point(10)..display_point(20), color2),
        ];
        let result = EditorElement::merge_overlapping_ranges(overlapping, base_bg);
        assert_eq!(cols(&result), vec![(5, 10), (10, 15), (15, 20)]);

        // Test middle segment should have blended color
        let blended = Hsla::blend(Hsla::blend(base_bg, color1), color2);
        assert_eq!(result[1].1, blended);

        // Test adjacent same-color ranges merge
        let adjacent_same = vec![
            (display_point(5)..display_point(10), color1),
            (display_point(10)..display_point(15), color1),
        ];
        let result = EditorElement::merge_overlapping_ranges(adjacent_same, base_bg);
        assert_eq!(cols(&result), vec![(5, 15)]);

        // Test contained range splits
        let contained = vec![
            (display_point(5)..display_point(20), color1),
            (display_point(10)..display_point(15), color2),
        ];
        let result = EditorElement::merge_overlapping_ranges(contained, base_bg);
        assert_eq!(cols(&result), vec![(5, 10), (10, 15), (15, 20)]);

        // Test multiple overlaps split at every boundary
        let color3 = Hsla {
            h: 240.0,
            s: 0.5,
            l: 0.5,
            a: 0.5,
        };
        let complex = vec![
            (display_point(5)..display_point(12), color1),
            (display_point(8)..display_point(16), color2),
            (display_point(10)..display_point(14), color3),
        ];
        let result = EditorElement::merge_overlapping_ranges(complex, base_bg);
        assert_eq!(
            cols(&result),
            vec![(5, 8), (8, 10), (10, 12), (12, 14), (14, 16)]
        );
    }

    #[gpui::test]
    fn test_bg_segments_per_row() {
        let base_bg = Hsla::white();

        // Case A: selection spans three display rows: row 1 [5, end), full row 2, row 3 [0, 7)
        {
            let selection_color = Hsla {
                h: 200.0,
                s: 0.5,
                l: 0.5,
                a: 0.5,
            };
            let player_color = PlayerColor {
                cursor: selection_color,
                background: selection_color,
                selection: selection_color,
            };

            let spanning_selection = SelectionLayout {
                id: 0,
                head: DisplayPoint::new(DisplayRow(3), 7),
                cursor_shape: CursorShape::Bar,
                is_newest: true,
                is_local: true,
                range: DisplayPoint::new(DisplayRow(1), 5)..DisplayPoint::new(DisplayRow(3), 7),
                active_rows: DisplayRow(1)..DisplayRow(4),
                user_name: None,
            };

            let selections = vec![(player_color, vec![spanning_selection])];
            let result = EditorElement::bg_segments_per_row(
                DisplayRow(0)..DisplayRow(5),
                &selections,
                [].into_iter(),
                base_bg,
            );

            assert_eq!(result.len(), 5);
            assert!(result[0].is_empty());
            assert_eq!(result[1].len(), 1);
            assert_eq!(result[2].len(), 1);
            assert_eq!(result[3].len(), 1);
            assert!(result[4].is_empty());

            assert_eq!(result[1][0].0.start, DisplayPoint::new(DisplayRow(1), 5));
            assert_eq!(result[1][0].0.end.row(), DisplayRow(1));
            assert_eq!(result[1][0].0.end.column(), u32::MAX);
            assert_eq!(result[2][0].0.start, DisplayPoint::new(DisplayRow(2), 0));
            assert_eq!(result[2][0].0.end.row(), DisplayRow(2));
            assert_eq!(result[2][0].0.end.column(), u32::MAX);
            assert_eq!(result[3][0].0.start, DisplayPoint::new(DisplayRow(3), 0));
            assert_eq!(result[3][0].0.end, DisplayPoint::new(DisplayRow(3), 7));
        }

        // Case B: selection ends exactly at the start of row 3, excluding row 3
        {
            let selection_color = Hsla {
                h: 120.0,
                s: 0.5,
                l: 0.5,
                a: 0.5,
            };
            let player_color = PlayerColor {
                cursor: selection_color,
                background: selection_color,
                selection: selection_color,
            };

            let selection = SelectionLayout {
                id: 0,
                head: DisplayPoint::new(DisplayRow(2), 0),
                cursor_shape: CursorShape::Bar,
                is_newest: true,
                is_local: true,
                range: DisplayPoint::new(DisplayRow(1), 5)..DisplayPoint::new(DisplayRow(3), 0),
                active_rows: DisplayRow(1)..DisplayRow(3),
                user_name: None,
            };

            let selections = vec![(player_color, vec![selection])];
            let result = EditorElement::bg_segments_per_row(
                DisplayRow(0)..DisplayRow(4),
                &selections,
                [].into_iter(),
                base_bg,
            );

            assert_eq!(result.len(), 4);
            assert!(result[0].is_empty());
            assert_eq!(result[1].len(), 1);
            assert_eq!(result[2].len(), 1);
            assert!(result[3].is_empty());

            assert_eq!(result[1][0].0.start, DisplayPoint::new(DisplayRow(1), 5));
            assert_eq!(result[1][0].0.end.row(), DisplayRow(1));
            assert_eq!(result[1][0].0.end.column(), u32::MAX);
            assert_eq!(result[2][0].0.start, DisplayPoint::new(DisplayRow(2), 0));
            assert_eq!(result[2][0].0.end.row(), DisplayRow(2));
            assert_eq!(result[2][0].0.end.column(), u32::MAX);
        }
    }

    #[cfg(test)]
    fn generate_test_run(len: usize, color: Hsla) -> TextRun {
        TextRun {
            len,
            color,
            ..Default::default()
        }
    }

    #[gpui::test]
    fn test_split_runs_by_bg_segments(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let dx = |start: u32, end: u32| {
            DisplayPoint::new(DisplayRow(0), start)..DisplayPoint::new(DisplayRow(0), end)
        };

        let text_color = Hsla {
            h: 210.0,
            s: 0.1,
            l: 0.4,
            a: 1.0,
        };
        let bg_1 = Hsla {
            h: 30.0,
            s: 0.6,
            l: 0.8,
            a: 1.0,
        };
        let bg_2 = Hsla {
            h: 200.0,
            s: 0.6,
            l: 0.2,
            a: 1.0,
        };
        let min_contrast = 45.0;
        let adjusted_bg1 = ensure_minimum_contrast(text_color, bg_1, min_contrast);
        let adjusted_bg2 = ensure_minimum_contrast(text_color, bg_2, min_contrast);

        // Case A: single run; disjoint segments inside the run
        {
            let runs = vec![generate_test_run(20, text_color)];
            let segs = vec![(dx(5, 10), bg_1), (dx(12, 16), bg_2)];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 0);
            // Expected slices: [0,5) [5,10) [10,12) [12,16) [16,20)
            assert_eq!(
                out.iter().map(|r| r.len).collect::<Vec<_>>(),
                vec![5, 5, 2, 4, 4]
            );
            assert_eq!(out[0].color, text_color);
            assert_eq!(out[1].color, adjusted_bg1);
            assert_eq!(out[2].color, text_color);
            assert_eq!(out[3].color, adjusted_bg2);
            assert_eq!(out[4].color, text_color);
        }

        // Case B: multiple runs; segment extends to end of line (u32::MAX)
        {
            let runs = vec![
                generate_test_run(8, text_color),
                generate_test_run(7, text_color),
            ];
            let segs = vec![(dx(6, u32::MAX), bg_1)];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 0);
            // Expected slices across runs: [0,6) [6,8) | [0,7)
            assert_eq!(out.iter().map(|r| r.len).collect::<Vec<_>>(), vec![6, 2, 7]);
            assert_eq!(out[0].color, text_color);
            assert_eq!(out[1].color, adjusted_bg1);
            assert_eq!(out[2].color, adjusted_bg1);
        }

        // Case C: multi-byte characters
        {
            // for text: "Hello 🌍 世界!"
            let runs = vec![
                generate_test_run(5, text_color), // "Hello"
                generate_test_run(6, text_color), // " 🌍 "
                generate_test_run(6, text_color), // "世界"
                generate_test_run(1, text_color), // "!"
            ];
            // selecting "🌍 世"
            let segs = vec![(dx(6, 14), bg_1)];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 0);
            // "Hello" | " " | "🌍 " | "世" | "界" | "!"
            assert_eq!(
                out.iter().map(|r| r.len).collect::<Vec<_>>(),
                vec![5, 1, 5, 3, 3, 1]
            );
            assert_eq!(out[0].color, text_color); // "Hello"
            assert_eq!(out[2].color, adjusted_bg1); // "🌍 "
            assert_eq!(out[3].color, adjusted_bg1); // "世"
            assert_eq!(out[4].color, text_color); // "界"
            assert_eq!(out[5].color, text_color); // "!"
        }

        // Case D: split multiple consecutive text runs with segments
        {
            let segs = vec![
                (dx(2, 4), bg_1),   // selecting "cd"
                (dx(4, 8), bg_2),   // selecting "efgh"
                (dx(9, 11), bg_1),  // selecting "jk"
                (dx(12, 16), bg_2), // selecting "mnop"
                (dx(18, 19), bg_1), // selecting "s"
            ];

            // for text: "abcdef"
            let runs = vec![
                generate_test_run(2, text_color), // ab
                generate_test_run(4, text_color), // cdef
            ];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 0);
            // new splits "ab", "cd", "ef"
            assert_eq!(out.iter().map(|r| r.len).collect::<Vec<_>>(), vec![2, 2, 2]);
            assert_eq!(out[0].color, text_color);
            assert_eq!(out[1].color, adjusted_bg1);
            assert_eq!(out[2].color, adjusted_bg2);

            // for text: "ghijklmn"
            let runs = vec![
                generate_test_run(3, text_color), // ghi
                generate_test_run(2, text_color), // jk
                generate_test_run(3, text_color), // lmn
            ];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 6); // 2 + 4 from first run
            // new splits "gh", "i", "jk", "l", "mn"
            assert_eq!(
                out.iter().map(|r| r.len).collect::<Vec<_>>(),
                vec![2, 1, 2, 1, 2]
            );
            assert_eq!(out[0].color, adjusted_bg2);
            assert_eq!(out[1].color, text_color);
            assert_eq!(out[2].color, adjusted_bg1);
            assert_eq!(out[3].color, text_color);
            assert_eq!(out[4].color, adjusted_bg2);

            // for text: "opqrs"
            let runs = vec![
                generate_test_run(1, text_color), // o
                generate_test_run(4, text_color), // pqrs
            ];
            let out = LineWithInvisibles::split_runs_by_bg_segments(&runs, &segs, min_contrast, 14); // 6 + 3 + 2 + 3 from first two runs
            // new splits "o", "p", "qr", "s"
            assert_eq!(
                out.iter().map(|r| r.len).collect::<Vec<_>>(),
                vec![1, 1, 2, 1]
            );
            assert_eq!(out[0].color, adjusted_bg2);
            assert_eq!(out[1].color, adjusted_bg2);
            assert_eq!(out[2].color, text_color);
            assert_eq!(out[3].color, adjusted_bg1);
        }
    }

    #[test]
    fn test_bg_segments_cover_whole_ligatures() {
        let glyph = |index: usize| gpui::ShapedGlyph {
            id: gpui::GlyphId(0),
            position: point(px(index as f32), px(0.)),
            index,
            is_emoji: false,
        };
        let xfix = LineLayout {
            font_size: px(10.),
            width: px(4.),
            ascent: px(8.),
            descent: px(2.),
            runs: vec![gpui::ShapedRun {
                font_id: FontId(0),
                glyphs: vec![glyph(0), glyph(1), glyph(3)],
            }],
            len: 4,
            caret_positions: Vec::new(),
        };
        let segment = |columns: Range<u32>, color: Hsla| {
            (
                DisplayPoint::new(DisplayRow(0), columns.start)
                    ..DisplayPoint::new(DisplayRow(0), columns.end),
                color,
            )
        };
        let (red, blue) = (gpui::red(), gpui::blue());
        let aligned = |segments: &[(Range<DisplayPoint>, Hsla)]| {
            glyph_aligned_bg_segments(segments, &xfix, 10)
        };

        assert_eq!(aligned(&[segment(12..13, red)]), vec![segment(11..13, red)]);
        assert_eq!(
            aligned(&[segment(5..12, red), segment(12..20, blue)]),
            vec![segment(5..13, red), segment(13..20, blue)]
        );
        assert_eq!(
            aligned(&[segment(10..11, red), segment(13..14, blue)]),
            vec![segment(10..11, red), segment(13..14, blue)]
        );
        let ignored = LineLayout {
            runs: vec![gpui::ShapedRun {
                font_id: FontId(0),
                glyphs: vec![glyph(0)],
            }],
            len: 262_144,
            ..xfix
        };
        let segments = (0..131_072)
            .map(|index| {
                segment(
                    2 * index..2 * index + 1,
                    if index % 2 == 0 { red } else { blue },
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            glyph_aligned_bg_segments(&segments, &ignored, 0),
            [segment(0..262_144, red)]
        );
    }

    #[test]
    fn test_long_row_ligatures_paint_each_character_once() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        let text = format!("x{}!", "w".repeat(1_500));
        let (editor, ref mut cx) = editor_test_context(&text, &mut cx);
        cx.simulate_resize(size(px(20_000.), px(500.)));
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.text.font_weight = FontWeight::NORMAL;
        style.text.color = Hsla::black();
        style.show_underlines = true;
        let advance = editor.update_in(cx, |_, window, _| {
            f64::from(style.text.font_size.to_pixels(window.rem_size()))
                * f64::from(ShapingTextSystem::ADVANCE)
        });
        cx.run_until_parked();
        for (highlight, width) in [
            (HighlightStyle::default(), 1_127.),
            (
                HighlightStyle {
                    background_color: Some(Hsla::red()),
                    ..HighlightStyle::default()
                },
                1_127.,
            ),
            (
                HighlightStyle {
                    color: Some(Hsla::red()),
                    ..HighlightStyle::default()
                },
                1_127.5,
            ),
            (
                HighlightStyle {
                    underline: Some(UnderlineStyle {
                        thickness: px(1.),
                        color: None,
                        wavy: false,
                    }),
                    ..HighlightStyle::default()
                },
                1_127.5,
            ),
            (
                HighlightStyle {
                    strikethrough: Some(StrikethroughStyle {
                        thickness: px(1.),
                        color: None,
                    }),
                    ..HighlightStyle::default()
                },
                1_127.5,
            ),
        ] {
            editor.update(cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).snapshot(cx);
                editor.highlight_text(
                    HighlightKey::Editor,
                    vec![
                        buffer.anchor_after(MultiBufferOffset(2))
                            ..buffer.anchor_before(MultiBufferOffset(3)),
                    ],
                    highlight,
                    cx,
                );
            });
            let state = draw_ready_editor_at(
                &editor,
                &style,
                Bounds::new(point(px(0.), px(0.)), size(px(20_000.), px(500.))),
                cx,
            );
            let line = &state.position_map.line_layouts[0];
            assert!(line.long_row.is_some());
            assert_eq!(line.width, width * advance, "{highlight:?}");
            let mut covered_end = 0;
            let mut misplaced = Vec::new();
            for (_, glyph, start, _, _) in painted_glyphs(line) {
                if start != covered_end {
                    misplaced.push((start, covered_end));
                }
                covered_end = start + glyph.0 as usize;
            }
            assert_eq!(misplaced, Vec::new());
            assert_eq!(covered_end, text.len());
        }
    }

    #[test]
    fn test_styled_long_rows_match_unwindowed_shaping() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        update_test_editor_settings(&mut cx, &|settings| {
            settings.sticky_scroll = Some(settings::StickyScrollContent {
                enabled: Some(true),
            });
        });
        for (text, styled, oversized) in [
            (
                format!(
                    "a{}",
                    ShapingTextSystem::PRE_BASE_VOWEL.to_string().repeat(1_365)
                ),
                vec![997..1_060],
                true,
            ),
            (
                format!("fn foo({}: u8) {{", "漢".repeat(1_500)),
                (0..1_500)
                    .step_by(5)
                    .map(|index| 7 + 3 * index..7 + 3 * (index + 2))
                    .collect(),
                false,
            ),
        ] {
            let document = if oversized {
                text.clone()
            } else {
                format!("{text}\n{}}}\n", "    let one = 1;\n".repeat(40))
            };
            let window = cx.open_window(size(px(500.), px(500.)), |window, cx| {
                let buffer = MultiBuffer::build_simple(&document, cx);
                if !oversized {
                    buffer
                        .read(cx)
                        .as_singleton()
                        .expect("a singleton buffer")
                        .update(cx, |buffer, cx| {
                            buffer.set_language(Some(languages::rust_lang()), cx);
                        });
                }
                Editor::new(EditorMode::full(), buffer, None, window, cx)
            });
            let cx = &mut VisualTestContext::from_window(*window, &mut cx);
            let editor = window.root(cx).unwrap();
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.syntax = Arc::new(theme::SyntaxTheme::new(Vec::new()));

            cx.run_until_parked();
            let bold = HighlightStyle {
                font_weight: Some(FontWeight::BOLD),
                ..HighlightStyle::default()
            };
            editor.update(cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).snapshot(cx);
                let ranges = styled
                    .iter()
                    .map(|columns| {
                        buffer.anchor_after(MultiBufferOffset(columns.start))
                            ..buffer.anchor_before(MultiBufferOffset(columns.end))
                    })
                    .collect();
                editor.highlight_text(HighlightKey::Editor, ranges, bold, cx);
            });
            let expected = editor.update_in(cx, |_, window, _| {
                let mut runs = Vec::new();
                let mut start = 0;
                for columns in &styled {
                    if start < columns.start {
                        runs.push(style.text.to_run(columns.start - start));
                    }
                    runs.push(style.text.clone().highlight(bold).to_run(columns.len()));
                    start = columns.end;
                }
                runs.push(style.text.to_run(text.len() - start));
                window.text_system().layout_line(
                    &text,
                    style.text.font_size.to_pixels(window.rem_size()),
                    &runs,
                    None,
                )
            });
            let glyph_bounds = editor.update_in(cx, |_, window, _| {
                expected
                    .runs
                    .iter()
                    .map(|run| {
                        (
                            run.font_id,
                            window
                                .text_system()
                                .bounding_box(run.font_id, expected.font_size),
                        )
                    })
                    .collect::<HashMap<_, _>>()
            });
            let scrolls = if oversized {
                &[
                    (TextAlign::Left, 0., 0.),
                    (TextAlign::Left, 330., 0.),
                    (TextAlign::Left, 350., 0.),
                    (TextAlign::Center, 0., 0.),
                    (TextAlign::Right, 0., 0.),
                    (TextAlign::Left, 1_300., 0.),
                    (TextAlign::Left, 0., 0.),
                ][..]
            } else {
                &[
                    (TextAlign::Left, 0., 0.),
                    (TextAlign::Left, 700., 0.),
                    (TextAlign::Center, 0., 0.),
                    (TextAlign::Right, 0., 0.),
                    (TextAlign::Left, 0., 10.),
                    (TextAlign::Left, 700., 10.),
                    (TextAlign::Center, 0., 10.),
                    (TextAlign::Right, 0., 10.),
                ][..]
            };
            for &(alignment, scroll, scroll_y) in scrolls {
                style.text.text_align = alignment;
                editor.update_in(cx, |editor, window, cx| {
                    editor.set_scroll_position(point(scroll, scroll_y), window, cx);
                });
                let state = draw_ready_editor(&editor, &style, cx);
                let line = if scroll_y == 0. {
                    &state.position_map.line_layouts[0]
                } else {
                    state
                        .sticky_headers
                        .as_ref()
                        .expect("a sticky header")
                        .lines[0]
                        .line
                        .as_ref()
                };
                let viewport_origin = state.position_map.scroll_pixel_position.x
                    + match alignment {
                        TextAlign::Left => 0.,
                        TextAlign::Center => {
                            (f64::from(expected.width) - f64::from(state.content_width)) / 2.
                        }
                        TextAlign::Right => {
                            f64::from(expected.width) - f64::from(state.content_width)
                        }
                    };
                if oversized {
                    assert_eq!(
                        line.fragment_positions()
                            .filter(|(_, _, origin)| *origin == viewport_origin)
                            .count(),
                        1,
                        "alignment {alignment:?}, scroll ({scroll}, {scroll_y})"
                    );
                }
                assert!(line.long_row.is_some());
                let expected_glyphs = expected
                    .runs
                    .iter()
                    .flat_map(|run| {
                        run.glyphs.iter().map(move |glyph| {
                            (
                                run.font_id,
                                glyph.id,
                                glyph.index,
                                Pixels::from(f64::from(glyph.position.x) - viewport_origin),
                                glyph.position.y,
                            )
                        })
                    })
                    .collect::<Vec<_>>();
                let painted = painted_glyphs(line);
                assert!(!expected_glyphs.is_empty());
                if !oversized {
                    for (_, _, column, x, _) in &painted {
                        assert!((line.x_for_index(*column) - x).abs() < 0.01);
                    }
                }
                let painted = painted
                    .into_iter()
                    .map(|(font, glyph, column, x, y)| {
                        (font, glyph, column, Pixels::from(x - viewport_origin), y)
                    })
                    .collect::<Vec<_>>();
                let mut expected_order = expected_glyphs.iter();
                for glyph in &painted {
                    assert_eq!(
                        expected_order.find(|expected| expected.2 == glyph.2),
                        Some(glyph),
                        "alignment {alignment:?}, scroll ({scroll}, {scroll_y})"
                    );
                }
                let visible = |glyph: &&(FontId, gpui::GlyphId, usize, Pixels, Pixels)| {
                    let x = state.content_origin.x + glyph.3;
                    let bounds = state.position_map.text_hitbox.bounds;
                    let glyph_bounds = glyph_bounds[&glyph.0];
                    x + glyph_bounds.left() < bounds.right()
                        && x + glyph_bounds.right() > bounds.left()
                };
                assert_eq!(
                    painted.iter().filter(visible).collect::<Vec<_>>(),
                    expected_glyphs.iter().filter(visible).collect::<Vec<_>>(),
                    "alignment {alignment:?}, scroll ({scroll}, {scroll_y})"
                );
            }
        }
    }

    #[test]
    fn test_spacer_pattern_period() {
        // line height is smaller than target height, so we just return half the line height
        assert_eq!(EditorElement::spacer_pattern_period(10.0, 20.0), 5.0);

        // line height is exactly half the target height, perfect match
        assert_eq!(EditorElement::spacer_pattern_period(20.0, 10.0), 10.0);

        // line height is close to half the target height
        assert_eq!(EditorElement::spacer_pattern_period(20.0, 9.0), 10.0);

        // line height is close to 1/4 the target height
        assert_eq!(EditorElement::spacer_pattern_period(20.0, 4.8), 5.0);
    }

    #[test]
    fn cursor_animation_supports_bar_and_block_shapes() {
        assert!(cursor_shape_supports_cursor_animation(CursorShape::Bar));
        assert!(cursor_shape_supports_cursor_animation(CursorShape::Block));
        assert!(!cursor_shape_supports_cursor_animation(
            CursorShape::Underline
        ));
        assert!(!cursor_shape_supports_cursor_animation(CursorShape::Hollow));
    }

    #[gpui::test(iterations = 100)]
    fn test_random_spacer_pattern_period(mut rng: StdRng) {
        let line_height = rng.next_u32() as f32;
        let target_height = rng.next_u32() as f32;

        let result = EditorElement::spacer_pattern_period(line_height, target_height);

        let k = line_height / result;
        assert!(k - k.round() < 0.0000001); // approximately integer
        assert!((k.round() as u32).is_multiple_of(2));
    }

    #[test]
    fn test_calculate_wrap_width() {
        let editor_width = px(800.0);
        let em_width = px(8.0);

        assert_eq!(
            calculate_wrap_width(SoftWrap::None, false, editor_width, em_width),
            None,
        );
        assert_eq!(
            calculate_wrap_width(SoftWrap::None, true, editor_width, em_width),
            Some(px((MAX_LINE_LEN as f32 / 2.0 * 8.0).ceil())),
        );

        assert_eq!(
            calculate_wrap_width(SoftWrap::EditorWidth, false, editor_width, em_width),
            Some(px(800.0)),
        );

        assert_eq!(
            calculate_wrap_width(SoftWrap::Bounded(72), false, editor_width, em_width),
            Some(px((72.0 * 8.0_f32).ceil())),
        );
        assert_eq!(
            calculate_wrap_width(SoftWrap::Bounded(200), false, px(400.0), em_width),
            Some(px(400.0)),
        );
    }

    #[test]
    fn test_deleted_marker_base_width() {
        use settings::PixelSetting;

        assert_eq!(
            EditorElement::deleted_marker_base_width(GitGutterWidth::Default, px(22.0)),
            px(7.0),
        );

        let boosted = EditorElement::deleted_marker_base_width(
            GitGutterWidth::Custom(PixelSetting(6.0)),
            px(22.0),
        );
        assert!(
            boosted > px(6.0),
            "boosted={boosted:?} must exceed the raw custom width so the deleted pill stays visible"
        );

        for line_height in [22.0, 40.0] {
            let widths = [1.0, 2.0, 3.0, 6.0].map(|width| {
                EditorElement::deleted_marker_base_width(
                    GitGutterWidth::Custom(PixelSetting(width)),
                    px(line_height),
                )
            });
            assert!(
                widths.windows(2).all(|pair| pair[0] < pair[1]),
                "widths={widths:?} must grow with the custom setting"
            );
            assert!(
                widths[0] > px(line_height / 8.0),
                "widths={widths:?} must stay above the vanishing width for line_height={line_height}"
            );
        }

        assert_eq!(
            EditorElement::deleted_marker_base_width(
                GitGutterWidth::Custom(PixelSetting(0.275 * 40.0)),
                px(40.0),
            ),
            px(14.0),
        );

        assert_eq!(
            EditorElement::deleted_marker_base_width(
                GitGutterWidth::Custom(PixelSetting(0.0)),
                px(22.0),
            ),
            px(0.0),
        );
    }

    #[test]
    fn test_point_diagnostic_admission_preserves_covered_column() {
        let mut line = LineWithInvisibles {
            fragments: SmallVec::new(),
            invisibles: Vec::new(),
            diagnostic_underline_severity_ranges: vec![(0..1, ERROR)],
            point_diagnostics: Vec::new(),
            len: 1,
            width: 3.25,
            font_size: px(13.),
            long_row: None,
        };
        let underline = point_diagnostic_test_style(WARNING);
        let point_diagnostic = |column, severity| PointDiagnostic {
            column,
            underline,
            severity,
        };
        line.set_point_diagnostics(vec![
            point_diagnostic(2, WARNING),
            point_diagnostic(0, WARNING),
            point_diagnostic(2, ERROR),
            point_diagnostic(2, WARNING),
        ]);
        assert_eq!(
            line.point_diagnostics
                .iter()
                .map(|point| (point.column, point.severity, point.underline))
                .collect::<Vec<_>>(),
            [(0, WARNING, underline), (2, ERROR, underline)]
        );
    }

    #[gpui::test]
    fn test_point_diagnostic_paint_all_ownership_and_phase(cx: &mut TestAppContext) {
        let device_bounds = |span: &Range<i32>| {
            Bounds::from_corners(point(span.start as f32, 8.), point(span.end as f32, 14.))
                .map(ScaledPixels)
        };
        let clipped =
            |underline: &Underline| underline.bounds.intersect(&underline.content_mask.bounds);
        for (inputs, expected) in [
            (
                vec![(0., 11.349, WARNING, true), (0., 3.25, ERROR, false)],
                vec![(0..6, 0..6, ERROR), (6..23, 0..23, WARNING)],
            ),
            (
                vec![(0., 11.349, ERROR, true), (0., 3.25, WARNING, false)],
                vec![(0..23, 0..23, ERROR)],
            ),
            (
                vec![
                    (2.888_183_8, 10.829_102, ERROR, true),
                    (2.888_183_8, 20.959_962, WARNING, false),
                ],
                vec![(6..28, 6..28, ERROR), (28..48, 6..48, WARNING)],
            ),
            (
                vec![
                    (0., 3., WARNING, true),
                    (3., 2., ERROR, false),
                    (7., 2., WARNING, false),
                ],
                vec![
                    (0..6, 0..6, WARNING),
                    (6..10, 6..10, ERROR),
                    (14..18, 14..18, WARNING),
                ],
            ),
            (
                vec![
                    (0., 12., WARNING, true),
                    (2., 8., WARNING, false),
                    (4., 4., ERROR, true),
                ],
                vec![
                    (0..4, 0..24, WARNING),
                    (4..8, 4..20, WARNING),
                    (8..16, 8..16, ERROR),
                    (16..20, 4..20, WARNING),
                    (20..24, 0..24, WARNING),
                ],
            ),
            (
                vec![(-2., 42., WARNING, true), (-1., 3., ERROR, false)],
                vec![(0..4, -2..4, ERROR), (4..64, -4..80, WARNING)],
            ),
        ] {
            for reversed in [false, true] {
                let mut inputs = inputs.clone();
                if reversed {
                    inputs.reverse();
                }
                let window = cx.open_window(size(px(32.), px(16.)), |_, _| {
                    PointDiagnosticPaintTestView(inputs)
                });
                cx.simulate_window_scale_factor_change(window.into(), 2.);
                let mut painted = cx
                    .update_window(window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.painted_underlines()
                    })
                    .expect("completed diagnostic canvas");
                painted.sort_by_key(|underline| clipped(underline).left());
                assert_eq!(painted.len(), expected.len());
                for (underline, (visible, original, severity)) in painted.iter().zip(&expected) {
                    let style = point_diagnostic_test_style(*severity);
                    assert_eq!(clipped(underline), device_bounds(visible));
                    assert_eq!(underline.bounds, device_bounds(original));
                    assert_eq!(Some(underline.color), style.color);
                    assert_eq!(underline.thickness, ScaledPixels(2.));
                    assert_eq!(underline.wavy, true.into());
                }
            }
        }
    }

    #[test]
    fn test_prepared_styles_preserve_reordered_source_indices() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        let text = format!("{}किx", "x".repeat(2_048));
        let (editor, ref mut cx) = editor_test_context(&text, &mut cx);
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let foreground = Hsla::red();
        let background = Hsla::green();
        let highlighted_background = Hsla::blue();
        style.text.color = foreground;
        style.text.background_color = Some(background);
        style.text.underline = None;
        style.text.strikethrough = None;
        editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let range = buffer.anchor_after(MultiBufferOffset(2_051))
                ..buffer.anchor_before(MultiBufferOffset(2_054));
            editor.highlight_text(
                HighlightKey::Editor,
                vec![range],
                HighlightStyle {
                    background_color: Some(highlighted_background),
                    ..HighlightStyle::default()
                },
                cx,
            );
        });
        let state = draw_ready_editor(&editor, &style, cx);
        let prepared = state
            .position_map
            .snapshot
            .prepared_row_text(DisplayRow(0))
            .expect("ready text")
            .expect("long row");
        let expected = TextRun {
            len: 3,
            font: style.text.font(),
            color: foreground,
            background_color: Some(background),
            ..TextRun::default()
        };
        for start in [0, 2_048] {
            let layout = LineLayout {
                len: text.len() - start,
                runs: vec![ShapedRun {
                    font_id: FontId(0),
                    glyphs: (start..2_048)
                        .map(|column| column - start)
                        .chain([3, 0, 6].map(|column| 2_048 + column - start))
                        .enumerate()
                        .map(|(position, index)| ShapedGlyph {
                            id: GlyphId(0),
                            position: point(px(position as f32), Pixels::ZERO),
                            index,
                            is_emoji: false,
                        })
                        .collect(),
                }],
                ..LineLayout::default()
            };
            let lookups = prepared.paint_style_lookups();
            let (runs, _) = prepared
                .styles(start..text.len(), &layout)
                .expect("prepared paint runs");
            assert_eq!(
                runs,
                [
                    TextRun {
                        len: 2_051 - start,
                        ..expected.clone()
                    },
                    TextRun {
                        background_color: Some(highlighted_background),
                        ..expected.clone()
                    },
                    TextRun {
                        len: 1,
                        ..expected.clone()
                    },
                ],
            );
            assert_eq!(prepared.paint_style_lookups() - lookups, 3);
        }
    }

    #[test]
    fn test_long_row_paint_work_culls_advancing_glyphs() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        let mut counts = Vec::new();
        for marks in [4_096, 32_768] {
            let text = format!(
                "{}a{}{}",
                "x".repeat(2_048),
                "\u{301}".repeat(marks),
                "b".repeat(2_048)
            );
            let (editor, ref mut cx) = editor_test_context(&text, &mut cx);
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.text.font_size = px(16.).into();
            style.text.font_weight = FontWeight::NORMAL;
            let state = draw_ready_editor(&editor, &style, cx);
            let snapshot = &state.position_map.snapshot;
            let prepared = snapshot
                .prepared_row_text(DisplayRow(0))
                .expect("ready text")
                .expect("long row");
            let lookups = prepared.paint_style_lookups();
            let scroll = (2_048 + marks / 2) as f64 * 8. + 0.25;
            cx.update(|window, cx| {
                let line = layout_line(
                    DisplayRow(0),
                    snapshot,
                    &style,
                    px(100.),
                    scroll,
                    |_| false,
                    window,
                    cx,
                )
                .expect("ready geometry");
                let font = window.text_system().resolve_font(&style.text.font());
                let bounds = window.text_system().bounding_box(font, px(16.));
                let left = f64::from(bounds.origin.x)
                    .min(-f64::from(bounds.size.width) / 2.)
                    .min(0.);
                let right = f64::from(bounds.origin.x + bounds.size.width).max(8.);
                let expected = text
                    .char_indices()
                    .enumerate()
                    .filter_map(|(index, (column, _))| {
                        let x = index as f64 * 8.;
                        (x + left <= scroll + 100. && x + right >= scroll).then_some((column, x))
                    })
                    .collect::<Vec<_>>();
                let actual = line
                    .fragment_positions()
                    .flat_map(|(fragment, start, x)| {
                        let LineFragment::Text(shaped) = fragment else {
                            panic!("text only");
                        };
                        shaped.runs.iter().flat_map(move |run| {
                            run.glyphs.iter().map(move |glyph| {
                                (start + glyph.index, x + f64::from(glyph.position.x))
                            })
                        })
                    })
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
                let bytes = line
                    .fragments
                    .iter()
                    .map(|fragment| match fragment {
                        LineFragment::Text(shaped) => shaped.text.len(),
                        LineFragment::Element { .. } => 0,
                    })
                    .sum::<usize>();
                assert_eq!(bytes, 2 * actual.len());
                assert_eq!(prepared.paint_style_lookups() - lookups, 1);
                let scale = window.scale_factor();
                for (content_origin, width, expected) in
                    [(0., 100., false), (0., 200., true), (4., 100., true)]
                {
                    assert_eq!(
                        line.long_row_needs_relayout(
                            scroll,
                            TextAlign::Left,
                            px(content_origin),
                            ScaledPixels(0.)..ScaledPixels(width * scale),
                            px(width),
                            scale,
                        ),
                        expected
                    );
                }
                assert!(line.long_row.as_ref().expect("logical window").plan.len() >= 2 * marks);
                let row = snapshot
                    .long_row(DisplayRow(0))
                    .expect("ready geometry")
                    .expect("long row");
                assert_eq!(line.x_for_index(2_049), row.x_for_column(2_049));
                counts.push(actual.len());
            });
        }
        assert_eq!(counts[0], counts[1]);
    }

    #[test]
    fn test_long_row_block_cursor_intrinsic_widths() {
        let mut cx = shaping_test_context();
        init_test(&mut cx, |_| {});
        for (grapheme, expected_width, expected_input_bytes) in [
            (String::from("A"), 8., 1_024),
            (format!("a{}", "\u{034f}".repeat(511)), 4_608., 1_024),
            (format!("a{}", "\u{093f}".repeat(341)), 3_078., 1_024),
            (format!("a{}", "\u{034f}".repeat(512)), 4_617., 0),
        ] {
            let text = format!("xxxxxxxxx{grapheme}V{}", "x".repeat(2_000));
            let (editor, ref mut cx) = editor_test_context(&text, &mut cx);
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.text.font_size = px(16.).into();
            style.text.font_weight = FontWeight::NORMAL;
            editor.update_in(cx, |editor, window, cx| {
                window.focus(&editor.focus_handle(cx), cx);
                editor.set_cursor_shape(CursorShape::Block, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 9)..Point::new(0, 9)]);
                });
                if grapheme.len() > 1 {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    editor.highlight_text(
                        HighlightKey::Editor,
                        vec![
                            buffer.anchor_after(Point::new(0, 9))
                                ..buffer.anchor_before(Point::new(0, 10)),
                        ],
                        HighlightStyle {
                            font_weight: Some(FontWeight::BOLD),
                            ..HighlightStyle::default()
                        },
                        cx,
                    );
                }
            });
            let state = draw_ready_editor(&editor, &style, cx);
            assert_eq!(
                state
                    .visible_cursors
                    .iter()
                    .map(|cursor| cursor.block_width)
                    .collect::<Vec<_>>(),
                [px(expected_width)],
            );
            let cursor = state.visible_cursors.first().expect("painted cursor");
            assert!(cursor.block_text.is_none());
            assert!(!cursor.retained_block_text.fragments.is_empty());
            let snapshot = &state.position_map.snapshot;
            let row = snapshot
                .long_row(DisplayRow(0))
                .expect("ready geometry")
                .expect("long row");
            let prepared = snapshot
                .prepared_row_text(DisplayRow(0))
                .expect("ready text")
                .expect("prepared text");
            let input_bytes = prepared.cursor_text_input_bytes();
            cx.update(|window, cx| {
                let mut width = Pixels::ZERO;
                CursorLayout::long_row_width(
                    &mut width,
                    snapshot,
                    &row,
                    DisplayPoint::new(DisplayRow(0), 9),
                    px(16.),
                    &style.text,
                    window,
                    cx,
                )
                .expect("prepared cursor width");
                assert_eq!(width, px(expected_width));
                assert_eq!(
                    prepared.cursor_text_input_bytes() - input_bytes,
                    expected_input_bytes,
                );
            });
        }
    }

    #[gpui::test]
    async fn test_prepared_zero_advance_text_and_retained_cursor(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context("", cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let cluster_len = 2_000_001;
        let prepared = cx
            .update(|_, cx| {
                cx.background_spawn(async move {
                    let marks = "\u{034f}".repeat(1_000_000);
                    let chunks = [
                        ("a", false, false),
                        (marks.as_str(), false, false),
                        ("b ", false, false),
                        ("   ", true, false),
                        ("  ", true, false),
                        ("\u{200b}", false, false),
                        ("\u{3000}", false, false),
                        (" ", false, true),
                    ];
                    let prepared =
                        PreparedRowText::new(chunks.into_iter().map(|(text, is_tab, is_inlay)| {
                            HighlightedChunk {
                                text,
                                style: (text == "\u{200b}").then_some(HighlightStyle {
                                    background_color: Some(style.status.hint_background),
                                    underline: Some(UnderlineStyle {
                                        color: Some(style.status.hint),
                                        thickness: px(1.),
                                        wavy: false,
                                    }),
                                    ..HighlightStyle::default()
                                }),
                                diagnostic_underline_severity: None,
                                is_tab,
                                is_inlay,
                                replacement: (text == "\u{200b}").then(|| {
                                    ChunkReplacement::Str(SharedString::new_static("\u{2007}"))
                                }),
                            }
                        }));
                    let text = prepared.prepare_text_slice(0..cluster_len + 1);
                    (prepared, text)
                })
            })
            .await;
        let (prepared, text) = prepared;
        assert_eq!(text.len(), 2_000_002);
        assert_eq!(prepared.oversized_cursor_width(0), Some(None));
        assert_eq!(
            prepared.oversized_cursor_width(cluster_len as u32 - 1),
            Some(None)
        );
        assert_eq!(prepared.oversized_cursor_width(cluster_len as u32), None);
        assert_eq!(prepared.cursor_text_input_bytes(), 0);
        let chunks = prepared.chunks(0..usize::MAX).collect::<Vec<_>>();
        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks[0].len(), cluster_len + 2);
        assert_eq!(prepared.chunks(1..1).count(), 0);
        assert_eq!(
            prepared
                .invisibles(cluster_len + 3..cluster_len + 3)
                .count(),
            0
        );
        assert_eq!(
            chunks
                .iter()
                .enumerate()
                .filter_map(|(index, chunk)| {
                    match chunk {
                        LineChunk::Replacement {
                            replacement: ChunkReplacement::Str(text),
                            ..
                        } => Some((index, text.as_str())),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>(),
            [(2, "\u{2007}")],
        );
        assert_eq!(
            prepared.invisibles(0..usize::MAX).collect::<Vec<_>>(),
            [
                Invisible::Whitespace {
                    line_start_offset: cluster_len + 1,
                    line_end_offset: cluster_len + 2
                },
                Invisible::Tab {
                    line_start_offset: cluster_len + 2,
                    line_end_offset: cluster_len + 5
                },
                Invisible::Tab {
                    line_start_offset: cluster_len + 5,
                    line_end_offset: cluster_len + 7
                },
                Invisible::Whitespace {
                    line_start_offset: cluster_len + 10,
                    line_end_offset: cluster_len + 13
                },
            ],
        );
        assert_eq!(
            prepared
                .invisibles(cluster_len + 4..cluster_len + 6)
                .collect::<Vec<_>>(),
            [
                Invisible::Tab {
                    line_start_offset: cluster_len + 2,
                    line_end_offset: cluster_len + 5,
                },
                Invisible::Tab {
                    line_start_offset: cluster_len + 5,
                    line_end_offset: cluster_len + 7,
                },
            ],
        );
        let LineChunk::Text(marks) = &chunks[0] else {
            panic!("retained marks must be text");
        };
        let LineChunk::Text(retained_marks) = prepared
            .chunks(1..cluster_len)
            .next()
            .expect("retained marks")
        else {
            panic!("retained marks must be text");
        };
        assert_eq!(retained_marks.text.as_ptr(), marks.text[1..].as_ptr());
        let layout = Arc::new(LineLayout {
            font_size: px(16.),
            width: px(16.5),
            ascent: px(12.),
            descent: px(4.),
            len: text.len(),
            caret_positions: Vec::new(),
            runs: vec![
                ShapedRun {
                    font_id: FontId(1),
                    glyphs: vec![ShapedGlyph {
                        id: GlyphId(7),
                        index: 0,
                        position: point(px(0.), px(0.)),
                        is_emoji: false,
                    }],
                },
                ShapedRun {
                    font_id: FontId(9),
                    glyphs: vec![
                        ShapedGlyph {
                            id: GlyphId(8),
                            index: cluster_len,
                            position: point(px(8.25), px(0.)),
                            is_emoji: false,
                        },
                        ShapedGlyph {
                            id: GlyphId(9),
                            index: cluster_len,
                            position: point(px(1_000.), px(0.)),
                            is_emoji: false,
                        },
                    ],
                },
            ],
        });
        let fragment = LineFragment::Text(ShapedLine::new(layout, text.clone(), &[]));
        for (origin, cursor, offset) in [
            (0., 8.25, -8.25),
            (1_099_511_627_776., 1_099_511_627_784.25, -8.25),
            (f64::MAX, f64::MAX, 0.),
        ] {
            let visits = Cell::new(0);
            let mut font_visits = Vec::new();
            let retained = RetainedCursorText::new(
                iter::once((&fragment, 0, origin)).inspect(|_| visits.set(visits.get() + 1)),
                cursor,
                px(32.),
                px(32.)..px(40.25),
                Hsla::black(),
                |font_id, font_size| {
                    font_visits.push((font_id, font_size));
                    Bounds::new(point(px(-1.), px(0.)), size(px(10.), px(16.)))
                },
            );
            assert_eq!(visits.get(), 1);
            assert_eq!(font_visits, [(FontId(1), px(16.)), (FontId(9), px(16.))]);
            assert_eq!(retained.fragments.len(), 1);
            let (x, retained) = &retained.fragments[0];
            assert_eq!(*x, px(offset));
            assert_eq!(retained.len(), 2_000_002);
            assert_eq!(retained.ascent, px(12.));
            assert_eq!(retained.descent, px(4.));
            assert_eq!(retained.text.as_ptr(), text.as_ptr());
            assert_eq!(
                retained
                    .runs
                    .iter()
                    .flat_map(|run| run.glyphs.iter().map(move |glyph| (
                        run.font_id,
                        glyph.id,
                        glyph.index,
                        glyph.position.x
                    )))
                    .collect::<Vec<_>>(),
                [
                    (FontId(1), GlyphId(7), 0, px(0.)),
                    (FontId(9), GlyphId(8), cluster_len, px(8.25))
                ],
            );
        }

        let mut shaping_cx = shaping_test_context();
        init_test(&mut shaping_cx, |_| {});
        let grapheme = format!("a{}", "\u{0301}".repeat(500_000));
        let source = format!("{}{grapheme}V", "x".repeat(2_048));
        let (editor, ref mut cx) = editor_test_context(&source, &mut shaping_cx);
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        let font_size = px(17.3);
        style.text.font_size = AbsoluteLength::from(font_size);
        style.text.font_weight = FontWeight::NORMAL;
        editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            editor.highlight_text(
                HighlightKey::Editor,
                vec![
                    buffer.anchor_after(Point::new(0, 2_048))
                        ..buffer.anchor_before(Point::new(0, 2_049)),
                ],
                HighlightStyle {
                    font_weight: Some(FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
                cx,
            );
        });
        let state = draw_ready_editor(&editor, &style, cx);
        let snapshot = &state.position_map.snapshot;
        let row = snapshot
            .long_row(DisplayRow(0))
            .expect("ready geometry")
            .expect("long row");
        let prepared = snapshot
            .prepared_row_text(DisplayRow(0))
            .expect("ready text")
            .expect("prepared text");
        let expected_width = Pixels::from(500_001. * f64::from(font_size * 0.5 + px(1.)));
        assert_eq!(
            prepared.oversized_cursor_width(2_048),
            Some(Some(expected_width))
        );
        let input_bytes = prepared.cursor_text_input_bytes();
        cx.update(|window, cx| {
            let mut width = Pixels::ZERO;
            CursorLayout::long_row_width(
                &mut width,
                snapshot,
                &row,
                DisplayPoint::new(DisplayRow(0), 2_048),
                font_size,
                &style.text,
                window,
                cx,
            )
            .expect("prepared cursor width");
            assert_eq!(width, expected_width);
        });
        assert_eq!(prepared.cursor_text_input_bytes(), input_bytes);
    }

    #[gpui::test]
    async fn test_renderer_constraint_pending_and_exact_width(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (editor, ref mut cx) = editor_test_context("", cx);
        let display_map = editor.read_with(cx, |editor, _| editor.display_map.clone());
        let renders = Arc::new(AtomicUsize::new(0));
        let mut renderer = ChunkRenderer {
            id: ChunkRendererId::Inlay(crate::InlayId::Hint(0)),
            render: Arc::new({
                let renders = renders.clone();
                move |_| {
                    renders.fetch_add(1, AtomicOrdering::Relaxed);
                    div().w(relative(0.5)).h(px(10.)).into_any_element()
                }
            }),
            constrain_width: true,
            measured_width: None,
            width_class: None,
        };
        let text = SharedString::from("x".repeat(1_000_000));
        let prepared = PreparedRowText::new(
            [
                ("a", None),
                (
                    text.as_ref(),
                    Some(ChunkReplacement::Renderer(renderer.clone())),
                ),
                ("\u{0301} b", None),
            ]
            .into_iter()
            .map(|(text, replacement)| HighlightedChunk {
                text,
                style: None,
                diagnostic_underline_severity: None,
                is_tab: false,
                is_inlay: false,
                replacement,
            }),
        );
        assert_eq!(prepared.stored_text_len(), 5);
        assert_eq!(prepared.cursor_text(0..u32::MAX).as_deref(), Some("a"));
        assert_eq!(prepared.cursor_text(1..2), None);
        assert_eq!(
            prepared.cursor_text(1_000_001..u32::MAX).as_deref(),
            Some("\u{0301}")
        );
        assert_eq!(
            prepared
                .chunks(0..usize::MAX)
                .map(|chunk| chunk.len())
                .collect::<Vec<_>>(),
            [1, 1_000_000, 4],
        );
        assert_eq!(
            prepared
                .chunks(100..200)
                .map(|chunk| chunk.len())
                .collect::<Vec<_>>(),
            [100]
        );
        assert_eq!(
            prepared.invisibles(0..usize::MAX).collect::<Vec<_>>(),
            [Invisible::Whitespace {
                line_start_offset: 1_000_003,
                line_end_offset: 1_000_004,
            }]
        );
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.text.font_size = px(10.).into();
        draw_ready_editor(&editor, &style, cx);
        let style = style.text;
        let pending = cx.update(|window, cx| {
            display_map.update(cx, |map, cx| {
                assert!(
                    map.request_renderer_constraint(
                        renderer.clone(),
                        text.clone(),
                        style.font(),
                        px(10.),
                        cx,
                    )
                    .is_none()
                );
            });
            let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
            assert!(!snapshot.is_long_row(DisplayRow(0)));
            let pending = layout_chunk_renderer(
                &renderer,
                &snapshot,
                DisplayRow(0),
                &style,
                px(800.),
                window,
                cx,
            )
            .err()
            .expect("pending renderer constraint");
            assert_eq!(renders.load(AtomicOrdering::Relaxed), 0);
            pending
        });
        pending.wait().await;
        for measured_width in [None, Some(px(3_000_000.))] {
            renderer.measured_width = measured_width;
            cx.draw(
                point(Pixels::ZERO, Pixels::ZERO),
                size(px(800.), px(500.)),
                |window, cx| {
                    let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
                    let (_, size) = layout_chunk_renderer(
                        &renderer,
                        &snapshot,
                        DisplayRow(0),
                        &style,
                        px(800.),
                        window,
                        cx,
                    )
                    .expect("prepared exact constraint");
                    assert_eq!(size.width, px(3_000_000.));
                    div()
                },
            );
        }
        let changed = SharedString::from("y".repeat(1_000_000));
        let pending = cx.update(|window, cx| {
            let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
            let changed_style = TextStyle {
                font_size: px(20.).into(),
                ..style.clone()
            };
            assert!(
                layout_chunk_renderer(
                    &renderer,
                    &snapshot,
                    DisplayRow(0),
                    &changed_style,
                    px(800.),
                    window,
                    cx,
                )
                .is_err()
            );
            display_map.update(cx, |map, cx| {
                assert!(
                    map.request_renderer_constraint(
                        renderer.clone(),
                        changed,
                        style.font(),
                        px(10.),
                        cx,
                    )
                    .is_none()
                );
                let snapshot = map.snapshot(cx);
                assert!(snapshot.prepared_renderer_constraint(renderer.id).is_none());
                snapshot.pending_renderer_constraint(DisplayRow(0), renderer.id)
            })
        });
        pending.wait().await;
        assert_eq!(renders.load(AtomicOrdering::Relaxed), 2);
    }

    const ERROR: DiagnosticSeverity = DiagnosticSeverity::ERROR;
    const WARNING: DiagnosticSeverity = DiagnosticSeverity::WARNING;

    struct PointDiagnosticPaintTestView(Vec<(f32, f32, DiagnosticSeverity, bool)>);

    impl Render for PointDiagnosticPaintTestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let inputs = self.0.clone();
            gpui::canvas(
                |_, _, _| {},
                move |_, _, window, _| {
                    let underlines = inputs
                        .into_iter()
                        .map(|(x, width, severity, is_point)| {
                            let diagnostic = PointDiagnostic {
                                column: 0,
                                underline: point_diagnostic_test_style(severity),
                                severity,
                            };
                            let mut underline = DiagnosticUnderline::point(
                                &diagnostic,
                                point(px(x), px(4.)),
                                px(width),
                                window,
                            );
                            underline.is_point = is_point;
                            underline
                        })
                        .collect::<Vec<_>>();
                    DiagnosticUnderline::paint_all(&underlines, window);
                },
            )
            .size_full()
        }
    }

    fn point_diagnostic_test_style(severity: DiagnosticSeverity) -> UnderlineStyle {
        let hue = if severity == ERROR { 0. } else { 0.15 };
        UnderlineStyle {
            thickness: px(1.),
            color: Some(gpui::hsla(hue, 1., 0.5, 1.)),
            wavy: true,
        }
    }

    struct PointDiagnosticTestView {
        editor: Entity<Editor>,
        style: EditorStyle,
        x_offset: Pixels,
    }

    impl Render for PointDiagnosticTestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .pl(self.x_offset)
                .child(EditorElement::new(&self.editor, self.style.clone()))
        }
    }

    fn point_diagnostic_buffer(
        text: &str,
        diagnostics: impl IntoIterator<Item = (Range<PointUtf16>, DiagnosticSeverity)>,
        cx: &mut impl AppContext,
    ) -> Entity<Buffer> {
        cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx);
            let diagnostics = DiagnosticSet::new(
                diagnostics.into_iter().map(|(range, severity)| {
                    DiagnosticEntry::new(
                        range,
                        Diagnostic {
                            severity,
                            underline: true,
                            ..Diagnostic::default()
                        },
                    )
                }),
                &buffer.snapshot(),
            );
            buffer.update_diagnostics(lsp::LanguageServerId(0), diagnostics, cx);
            buffer
        })
    }

    fn point_diagnostic_editor(
        buffer: Entity<MultiBuffer>,
        cx: &mut TestAppContext,
    ) -> (WindowHandle<PointDiagnosticTestView>, Entity<Editor>) {
        let window = cx.open_window(size(px(500.), px(200.)), |window, cx| {
            let editor = cx.new(|cx| {
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.show_gutter = false;
                editor.offset_content = false;
                editor.set_read_only(true);
                editor
            });
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.text.font_size = px(13.).into();
            style.text.line_height = px(26.).into();
            PointDiagnosticTestView {
                editor,
                style,
                x_offset: Pixels::ZERO,
            }
        });
        let editor = window
            .read_with(cx, |view, _| view.editor.clone())
            .expect("test editor");
        (window, editor)
    }

    #[track_caller]
    fn assert_painted_point_diagnostics(
        window: WindowHandle<PointDiagnosticTestView>,
        expected: &[(u32, Range<i32>, DiagnosticSeverity)],
        cx: &mut TestAppContext,
    ) {
        let (error, warning, x_offset) = window
            .read_with(cx, |view, _| {
                (
                    view.style.status.error,
                    view.style.status.warning,
                    view.x_offset,
                )
            })
            .expect("diagnostic colors");
        let (underlines, scale) = cx
            .update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                (window.painted_underlines(), window.scale_factor())
            })
            .expect("completed diagnostic scene");
        let mut actual = Vec::new();
        for underline in underlines {
            let severity = if underline.color == error {
                ERROR
            } else if underline.color == warning {
                WARNING
            } else {
                continue;
            };
            let bounds = underline.bounds.intersect(&underline.content_mask.bounds);
            if bounds.is_empty() {
                continue;
            }
            assert_eq!(
                [bounds.left().0.fract(), bounds.right().0.fract()],
                [0., 0.]
            );
            let row = (bounds.top().0 / (26. * scale)).floor() as u32;
            actual.extend(
                (bounds.left().0 as i32..bounds.right().0 as i32).map(|x| (row, x, severity)),
            );
        }
        let mut expected = expected
            .iter()
            .flat_map(|(row, range, severity)| range.clone().map(|x| (*row, x, *severity)))
            .collect::<Vec<_>>();
        actual.sort_by_key(|(row, x, _)| (*row, *x));
        expected.sort_by_key(|(row, x, _)| (*row, *x));

        assert_eq!(actual, expected, "scale: {scale}, x offset: {x_offset:?}");
    }

    pub(crate) fn assert_long_row_paints(
        editor: &Entity<Editor>,
        style: &EditorStyle,
        expected: &[(usize, char)],
        cx: &mut VisualTestContext,
    ) {
        let state = draw_ready_editor(editor, style, cx);
        let line = &state.position_map.line_layouts[0];
        assert!(line.long_row.is_some());
        let mut actual = Vec::new();
        for (fragment, start, _) in line.fragment_positions() {
            let LineFragment::Text(shaped) = fragment else {
                panic!("unexpected non-text fragment");
            };
            actual.extend(shaped.runs.iter().flat_map(|run| &run.glyphs).map(|glyph| {
                (
                    start + glyph.index,
                    shaped.text[glyph.index..]
                        .chars()
                        .next()
                        .expect("a glyph character"),
                )
            }));
        }
        assert_eq!(actual, expected);
        assert!(line.width <= f64::from(state.position_map.text_hitbox.bounds.size.width));
    }

    #[derive(Default)]
    struct EditorPrepaintTestState {
        depth: Option<Rc<Cell<usize>>>,
        block_renders: Vec<(usize, u32, u32)>,
        scroll_x: Option<ScrollOffset>,
    }

    struct EditorLayoutTestRoot {
        editor: Entity<Editor>,
        style: EditorStyle,
        bounds: Bounds<Pixels>,
        layout: Rc<RefCell<Option<EditorPrepaintState>>>,
    }

    impl Render for EditorLayoutTestRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .absolute()
                    .left(self.bounds.origin.x)
                    .top(self.bounds.origin.y)
                    .w(self.bounds.size.width)
                    .h(self.bounds.size.height)
                    .child(gpui::ViewElement::new(EditorLayoutTestView {
                        element: EditorElement::new(&self.editor, self.style.clone()),
                        layout: self.layout.clone(),
                        prepaint: None,
                    })),
            )
        }
    }

    struct EditorLayoutTestView {
        element: EditorElement,
        layout: Rc<RefCell<Option<EditorPrepaintState>>>,
        prepaint: Option<Entity<EditorPrepaintTestState>>,
    }

    impl gpui::View for EditorLayoutTestView {
        fn entity_id(&self) -> Option<gpui::EntityId> {
            Some(self.element.editor.entity_id())
        }

        fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
            self
        }
    }

    impl IntoElement for EditorLayoutTestView {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl Element for EditorLayoutTestView {
        type RequestLayoutState = EditorRequestLayoutState;
        type PrepaintState = Option<EditorPrepaintState>;

        fn id(&self) -> Option<ElementId> {
            self.element.id()
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            self.element.source_location()
        }

        fn request_layout(
            &mut self,
            id: Option<&GlobalElementId>,
            inspector_id: Option<&gpui::InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (gpui::LayoutId, Self::RequestLayoutState) {
            self.element.request_layout(id, inspector_id, window, cx)
        }

        fn prepaint(
            &mut self,
            id: Option<&GlobalElementId>,
            inspector_id: Option<&gpui::InspectorElementId>,
            bounds: Bounds<Pixels>,
            request_layout: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut App,
        ) -> Self::PrepaintState {
            if let Some(prepaint) = &self.prepaint {
                assert_eq!(request_layout.prepaint_depth.get(), 0);
                prepaint.update(cx, |prepaint, _| {
                    prepaint.depth = Some(request_layout.prepaint_depth.clone());
                });
            }
            let layout =
                self.element
                    .prepaint(id, inspector_id, bounds, request_layout, window, cx);
            if let Some(prepaint) = &self.prepaint {
                assert_eq!(request_layout.prepaint_depth.get(), 0);
                let scroll_x = self
                    .element
                    .editor
                    .update(cx, |editor, cx| editor.scroll_position(cx).x);
                prepaint.update(cx, |prepaint, _| {
                    prepaint.depth = None;
                    prepaint.scroll_x = Some(scroll_x);
                });
            }
            Some(layout)
        }

        fn paint(
            &mut self,
            id: Option<&GlobalElementId>,
            inspector_id: Option<&gpui::InspectorElementId>,
            bounds: Bounds<Pixels>,
            request_layout: &mut Self::RequestLayoutState,
            prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
            let mut layout = prepaint.take().expect("editor was prepainted");
            self.element.paint(
                id,
                inspector_id,
                bounds,
                request_layout,
                &mut layout,
                window,
                cx,
            );
            *self.layout.borrow_mut() = Some(layout);
        }
    }

    #[derive(gpui::IntoElement)]
    struct DimensionDependentFold(WeakEntity<Editor>);

    impl gpui::RenderOnce for DimensionDependentFold {
        fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
            let editor = self.0.upgrade().expect("the editor is alive");
            let width = editor
                .read(cx)
                .last_bounds
                .expect("editor bounds")
                .size
                .width;
            div()
                .w(width / 2. + window.line_height())
                .h(window.line_height())
        }
    }

    fn editor_test_context(
        text: &str,
        cx: &mut TestAppContext,
    ) -> (Entity<Editor>, VisualTestContext) {
        let window = cx.add_window(|window, cx| {
            let buffer = MultiBuffer::build_simple(text, cx);
            Editor::new(EditorMode::full(), buffer, None, window, cx)
        });
        let mut cx = VisualTestContext::from_window(*window, cx);
        let editor = window.root(&mut cx).unwrap();
        (editor, cx)
    }

    fn shaping_test_context() -> TestAppContext {
        TestAppContext::build_with_text_system(
            gpui::TestDispatcher::new(0),
            None,
            Arc::new(ShapingTextSystem(gpui::NoopTextSystem)),
        )
    }

    fn painted_glyphs(
        line: &LineWithInvisibles,
    ) -> Vec<(FontId, gpui::GlyphId, usize, f64, Pixels)> {
        line.fragment_positions()
            .flat_map(|(fragment, start, x)| {
                let runs = match fragment {
                    LineFragment::Text(line) => line.runs.as_slice(),
                    LineFragment::Element { .. } => &[],
                };
                runs.iter().flat_map(move |run| {
                    run.glyphs.iter().map(move |glyph| {
                        (
                            run.font_id,
                            glyph.id,
                            start + glyph.index,
                            x + f64::from(glyph.position.x),
                            glyph.position.y,
                        )
                    })
                })
            })
            .collect()
    }

    fn ready_layout(state: EditorPrepaintState) -> EditorLayout {
        match state {
            EditorPrepaintState::Ready(layout) => layout,
            EditorPrepaintState::Pending(_) => panic!("editor geometry is pending"),
            EditorPrepaintState::Unavailable { .. } => panic!("editor geometry is unavailable"),
        }
    }

    fn draw_editor(
        editor: &Entity<Editor>,
        style: &EditorStyle,
        cx: &mut VisualTestContext,
    ) -> EditorLayout {
        ready_layout(draw_editor_frame(editor, style, cx))
    }

    fn draw_ready_editor(
        editor: &Entity<Editor>,
        style: &EditorStyle,
        cx: &mut VisualTestContext,
    ) -> EditorLayout {
        draw_ready_editor_at(
            editor,
            style,
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(500.))),
            cx,
        )
    }

    fn draw_ready_editor_at(
        editor: &Entity<Editor>,
        style: &EditorStyle,
        bounds: Bounds<Pixels>,
        cx: &mut VisualTestContext,
    ) -> EditorLayout {
        let layout = Rc::new(RefCell::new(None));
        cx.update(|window, cx| {
            window.replace_root(cx, |_, _| EditorLayoutTestRoot {
                editor: editor.clone(),
                style: style.clone(),
                bounds,
                layout: layout.clone(),
            });
        });
        cx.run_until_parked();
        let state = layout.borrow_mut().take().expect("editor was painted");
        ready_layout(state)
    }

    fn draw_editor_frame(
        editor: &Entity<Editor>,
        style: &EditorStyle,
        cx: &mut VisualTestContext,
    ) -> EditorPrepaintState {
        let layout = Rc::new(RefCell::new(None));
        cx.draw(point(px(0.), px(0.)), size(px(500.), px(500.)), |_, _| {
            gpui::ViewElement::new(EditorLayoutTestView {
                element: EditorElement::new(editor, style.clone()),
                layout: layout.clone(),
                prepaint: None,
            })
        });
        let layout = layout.borrow_mut().take().expect("editor was painted");
        layout
    }

    fn left_edge_point(state: &EditorLayout) -> Option<Point> {
        let position_map = &state.position_map;
        let column =
            position_map.line_layouts[0].index_for_x(position_map.scroll_pixel_position.x)?;
        Some(
            position_map.snapshot.display_point_to_point(
                DisplayPoint::new(DisplayRow(0), column as u32),
                Bias::Left,
            ),
        )
    }

    fn assert_offscreen_renderers_measured_per_frame(
        per_frame: usize,
        new_inlay: impl Fn(usize, Anchor) -> Inlay,
        cx: &mut TestAppContext,
    ) {
        let first_inlay_row = 300;
        let inlay_count = per_frame + 8;
        let (editor, ref mut cx) =
            editor_test_context(&"xx\n".repeat(first_inlay_row + inlay_count), cx);
        cx.run_until_parked();
        editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let inlays = (0..inlay_count)
                .map(|id| {
                    let row = (first_inlay_row + id) as u32;
                    new_inlay(id, buffer.anchor_after(Point::new(row, 1)))
                })
                .collect();
            editor.splice_inlays(&[], inlays, cx);
        });
        let unmeasured = |cx: &mut VisualTestContext| {
            let snapshot = editor.update_in(cx, |editor, window, cx| editor.snapshot(window, cx));
            let mut count = 0;
            snapshot.for_each_unmeasured_renderer(DisplayRow(0)..DisplayRow(0), |_, _| {
                count += 1;
                ControlFlow::Continue(())
            });
            count
        };

        let remaining = inlay_count - per_frame;
        assert_eq!(unmeasured(cx), remaining);
        cx.run_until_parked();
        assert_eq!(unmeasured(cx), remaining);
        cx.update(|window, cx| window.simulate_next_frame(cx));
        cx.run_until_parked();
        assert_eq!(unmeasured(cx), 0);
    }

    fn draw_single_row_editor(
        text: &str,
        scroll_columns: f64,
        cx: &mut TestAppContext,
    ) -> EditorLayout {
        let (editor, ref mut cx) = editor_test_context(text, cx);
        let style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        editor.update_in(cx, |editor, window, cx| {
            editor.set_scroll_position(point(scroll_columns, 0.), window, cx);
        });
        cx.run_until_parked();
        draw_editor(&editor, &style, cx)
    }

    fn dispatch_mouse_click(
        position: gpui::Point<Pixels>,
        click_count: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        let dispatch = window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                position,
                modifiers: Modifiers::none(),
                button: MouseButton::Left,
                click_count,
                first_mouse: false,
            }),
            cx,
        );
        assert!(!dispatch.propagate);
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                position,
                modifiers: Modifiers::none(),
                button: MouseButton::Left,
                click_count,
            }),
            cx,
        );
    }

    fn newest_head(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> Point {
        editor.update(cx, |editor, cx| {
            editor
                .selections
                .newest::<Point>(&editor.display_snapshot(cx))
                .head()
        })
    }
}
