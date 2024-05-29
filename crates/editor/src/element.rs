use crate::{
    blame_entry_tooltip::{blame_entry_relative_timestamp, BlameEntryTooltip},
    display_map::{
        BlockContext, BlockStyle, DisplaySnapshot, HighlightedChunk, ToDisplayPoint, TransformBlock,
    },
    editor_settings::{
        CurrentLineHighlight, DoubleClickInMultibuffer, MultiCursorModifier, ShowScrollbar,
    },
    git::{
        blame::{CommitDetails, GitBlame},
        diff_hunk_to_display, DisplayDiffHunk,
    },
    hover_popover::{
        self, hover_at, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT,
    },
    hunk_status,
    items::BufferSearchHighlights,
    mouse_context_menu::{self, MouseContextMenu},
    scroll::scroll_amount::ScrollAmount,
    CodeActionsMenu, CursorShape, DisplayPoint, DisplayRow, DocumentHighlightRead,
    DocumentHighlightWrite, Editor, EditorMode, EditorSettings, EditorSnapshot, EditorStyle,
    ExpandExcerpts, GutterDimensions, HalfPageDown, HalfPageUp, HoveredCursor, HunkToExpand,
    LineDown, LineUp, OpenExcerpts, PageDown, PageUp, Point, RowExt, RowRangeExt, SelectPhase,
    Selection, SoftWrap, ToPoint, CURSORS_VISIBLE_FOR, MAX_LINE_LEN,
};
use client::ParticipantIndex;
use collections::{BTreeMap, HashMap};
use git::{blame::BlameEntry, diff::DiffHunkStatus, Oid};
use gpui::{
    anchored, deferred, div, fill, outline, point, px, quad, relative, size, svg,
    transparent_black, Action, AnchorCorner, AnyElement, AvailableSpace, Bounds, ClipboardItem,
    ContentMask, Corners, CursorStyle, DispatchPhase, Edges, Element, ElementInputHandler, Entity,
    FontId, GlobalElementId, Hitbox, Hsla, InteractiveElement, IntoElement, Length,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad,
    ParentElement, Pixels, ScrollDelta, ScrollWheelEvent, ShapedLine, SharedString, Size,
    StatefulInteractiveElement, Style, Styled, TextRun, TextStyle, TextStyleRefinement, View,
    ViewContext, WeakView, WindowContext,
};
use itertools::Itertools;
use language::language_settings::{
    IndentGuideBackgroundColoring, IndentGuideColoring, ShowWhitespaceSetting,
};
use lsp::DiagnosticSeverity;
use multi_buffer::{Anchor, MultiBufferPoint, MultiBufferRow};
use project::{
    project_settings::{GitGutterSetting, ProjectSettings},
    ProjectPath,
};
use settings::Settings;
use smallvec::{smallvec, SmallVec};
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering},
    fmt::{self, Write},
    iter, mem,
    ops::{Deref, Range},
    sync::Arc,
};
use sum_tree::Bias;
use theme::{ActiveTheme, PlayerColor};
use ui::prelude::*;
use ui::{h_flex, ButtonLike, ButtonStyle, ContextMenu, Tooltip};
use util::ResultExt;
use workspace::{item::Item, Workspace};

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
    editor: View<Editor>,
    style: EditorStyle,
}

type DisplayRowDelta = u32;

impl EditorElement {
    pub(crate) const SCROLLBAR_WIDTH: Pixels = px(13.);

    pub fn new(editor: &View<Editor>, style: EditorStyle) -> Self {
        Self {
            editor: editor.clone(),
            style,
        }
    }

    fn register_actions(&self, cx: &mut WindowContext) {
        let view = &self.editor;
        view.update(cx, |editor, cx| {
            for action in editor.editor_actions.iter() {
                (action)(cx)
            }
        });

        crate::rust_analyzer_ext::apply_related_actions(view, cx);
        register_action(view, cx, Editor::move_left);
        register_action(view, cx, Editor::move_right);
        register_action(view, cx, Editor::move_down);
        register_action(view, cx, Editor::move_down_by_lines);
        register_action(view, cx, Editor::select_down_by_lines);
        register_action(view, cx, Editor::move_up);
        register_action(view, cx, Editor::move_up_by_lines);
        register_action(view, cx, Editor::select_up_by_lines);
        register_action(view, cx, Editor::cancel);
        register_action(view, cx, Editor::newline);
        register_action(view, cx, Editor::newline_above);
        register_action(view, cx, Editor::newline_below);
        register_action(view, cx, Editor::backspace);
        register_action(view, cx, Editor::delete);
        register_action(view, cx, Editor::tab);
        register_action(view, cx, Editor::tab_prev);
        register_action(view, cx, Editor::indent);
        register_action(view, cx, Editor::outdent);
        register_action(view, cx, Editor::delete_line);
        register_action(view, cx, Editor::join_lines);
        register_action(view, cx, Editor::sort_lines_case_sensitive);
        register_action(view, cx, Editor::sort_lines_case_insensitive);
        register_action(view, cx, Editor::reverse_lines);
        register_action(view, cx, Editor::shuffle_lines);
        register_action(view, cx, Editor::convert_to_upper_case);
        register_action(view, cx, Editor::convert_to_lower_case);
        register_action(view, cx, Editor::convert_to_title_case);
        register_action(view, cx, Editor::convert_to_snake_case);
        register_action(view, cx, Editor::convert_to_kebab_case);
        register_action(view, cx, Editor::convert_to_upper_camel_case);
        register_action(view, cx, Editor::convert_to_lower_camel_case);
        register_action(view, cx, Editor::convert_to_opposite_case);
        register_action(view, cx, Editor::delete_to_previous_word_start);
        register_action(view, cx, Editor::delete_to_previous_subword_start);
        register_action(view, cx, Editor::delete_to_next_word_end);
        register_action(view, cx, Editor::delete_to_next_subword_end);
        register_action(view, cx, Editor::delete_to_beginning_of_line);
        register_action(view, cx, Editor::delete_to_end_of_line);
        register_action(view, cx, Editor::cut_to_end_of_line);
        register_action(view, cx, Editor::duplicate_line_up);
        register_action(view, cx, Editor::duplicate_line_down);
        register_action(view, cx, Editor::move_line_up);
        register_action(view, cx, Editor::move_line_down);
        register_action(view, cx, Editor::transpose);
        register_action(view, cx, Editor::cut);
        register_action(view, cx, Editor::copy);
        register_action(view, cx, Editor::paste);
        register_action(view, cx, Editor::undo);
        register_action(view, cx, Editor::redo);
        register_action(view, cx, Editor::move_page_up);
        register_action(view, cx, Editor::move_page_down);
        register_action(view, cx, Editor::next_screen);
        register_action(view, cx, Editor::scroll_cursor_top);
        register_action(view, cx, Editor::scroll_cursor_center);
        register_action(view, cx, Editor::scroll_cursor_bottom);
        register_action(view, cx, |editor, _: &LineDown, cx| {
            editor.scroll_screen(&ScrollAmount::Line(1.), cx)
        });
        register_action(view, cx, |editor, _: &LineUp, cx| {
            editor.scroll_screen(&ScrollAmount::Line(-1.), cx)
        });
        register_action(view, cx, |editor, _: &HalfPageDown, cx| {
            editor.scroll_screen(&ScrollAmount::Page(0.5), cx)
        });
        register_action(view, cx, |editor, _: &HalfPageUp, cx| {
            editor.scroll_screen(&ScrollAmount::Page(-0.5), cx)
        });
        register_action(view, cx, |editor, _: &PageDown, cx| {
            editor.scroll_screen(&ScrollAmount::Page(1.), cx)
        });
        register_action(view, cx, |editor, _: &PageUp, cx| {
            editor.scroll_screen(&ScrollAmount::Page(-1.), cx)
        });
        register_action(view, cx, Editor::move_to_previous_word_start);
        register_action(view, cx, Editor::move_to_previous_subword_start);
        register_action(view, cx, Editor::move_to_next_word_end);
        register_action(view, cx, Editor::move_to_next_subword_end);
        register_action(view, cx, Editor::move_to_beginning_of_line);
        register_action(view, cx, Editor::move_to_end_of_line);
        register_action(view, cx, Editor::move_to_start_of_paragraph);
        register_action(view, cx, Editor::move_to_end_of_paragraph);
        register_action(view, cx, Editor::move_to_beginning);
        register_action(view, cx, Editor::move_to_end);
        register_action(view, cx, Editor::select_up);
        register_action(view, cx, Editor::select_down);
        register_action(view, cx, Editor::select_left);
        register_action(view, cx, Editor::select_right);
        register_action(view, cx, Editor::select_to_previous_word_start);
        register_action(view, cx, Editor::select_to_previous_subword_start);
        register_action(view, cx, Editor::select_to_next_word_end);
        register_action(view, cx, Editor::select_to_next_subword_end);
        register_action(view, cx, Editor::select_to_beginning_of_line);
        register_action(view, cx, Editor::select_to_end_of_line);
        register_action(view, cx, Editor::select_to_start_of_paragraph);
        register_action(view, cx, Editor::select_to_end_of_paragraph);
        register_action(view, cx, Editor::select_to_beginning);
        register_action(view, cx, Editor::select_to_end);
        register_action(view, cx, Editor::select_all);
        register_action(view, cx, |editor, action, cx| {
            editor.select_all_matches(action, cx).log_err();
        });
        register_action(view, cx, Editor::select_line);
        register_action(view, cx, Editor::split_selection_into_lines);
        register_action(view, cx, Editor::add_selection_above);
        register_action(view, cx, Editor::add_selection_below);
        register_action(view, cx, |editor, action, cx| {
            editor.select_next(action, cx).log_err();
        });
        register_action(view, cx, |editor, action, cx| {
            editor.select_previous(action, cx).log_err();
        });
        register_action(view, cx, Editor::toggle_comments);
        register_action(view, cx, Editor::select_larger_syntax_node);
        register_action(view, cx, Editor::select_smaller_syntax_node);
        register_action(view, cx, Editor::move_to_enclosing_bracket);
        register_action(view, cx, Editor::undo_selection);
        register_action(view, cx, Editor::redo_selection);
        if !view.read(cx).is_singleton(cx) {
            register_action(view, cx, Editor::expand_excerpts);
            register_action(view, cx, Editor::expand_excerpts_up);
            register_action(view, cx, Editor::expand_excerpts_down);
        }
        register_action(view, cx, Editor::go_to_diagnostic);
        register_action(view, cx, Editor::go_to_prev_diagnostic);
        register_action(view, cx, Editor::go_to_hunk);
        register_action(view, cx, Editor::go_to_prev_hunk);
        register_action(view, cx, |editor, a, cx| {
            editor.go_to_definition(a, cx).detach_and_log_err(cx);
        });
        register_action(view, cx, |editor, a, cx| {
            editor.go_to_definition_split(a, cx).detach_and_log_err(cx);
        });
        register_action(view, cx, |editor, a, cx| {
            editor.go_to_implementation(a, cx).detach_and_log_err(cx);
        });
        register_action(view, cx, |editor, a, cx| {
            editor
                .go_to_implementation_split(a, cx)
                .detach_and_log_err(cx);
        });
        register_action(view, cx, |editor, a, cx| {
            editor.go_to_type_definition(a, cx).detach_and_log_err(cx);
        });
        register_action(view, cx, |editor, a, cx| {
            editor
                .go_to_type_definition_split(a, cx)
                .detach_and_log_err(cx);
        });
        register_action(view, cx, Editor::open_url);
        register_action(view, cx, Editor::fold);
        register_action(view, cx, Editor::fold_at);
        register_action(view, cx, Editor::unfold_lines);
        register_action(view, cx, Editor::unfold_at);
        register_action(view, cx, Editor::fold_selected_ranges);
        register_action(view, cx, Editor::show_completions);
        register_action(view, cx, Editor::toggle_code_actions);
        register_action(view, cx, Editor::open_excerpts);
        register_action(view, cx, Editor::open_excerpts_in_split);
        register_action(view, cx, Editor::toggle_soft_wrap);
        register_action(view, cx, Editor::toggle_line_numbers);
        register_action(view, cx, Editor::toggle_indent_guides);
        register_action(view, cx, Editor::toggle_inlay_hints);
        register_action(view, cx, hover_popover::hover);
        register_action(view, cx, Editor::reveal_in_finder);
        register_action(view, cx, Editor::copy_path);
        register_action(view, cx, Editor::copy_relative_path);
        register_action(view, cx, Editor::copy_highlight_json);
        register_action(view, cx, Editor::copy_permalink_to_line);
        register_action(view, cx, Editor::open_permalink_to_line);
        register_action(view, cx, Editor::toggle_git_blame);
        register_action(view, cx, Editor::toggle_git_blame_inline);
        register_action(view, cx, Editor::toggle_hunk_diff);
        register_action(view, cx, Editor::expand_all_hunk_diffs);
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.format(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, Editor::restart_language_server);
        register_action(view, cx, Editor::show_character_palette);
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.confirm_completion(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.confirm_code_action(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.rename(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.confirm_rename(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, |editor, action, cx| {
            if let Some(task) = editor.find_all_references(action, cx) {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
        register_action(view, cx, Editor::next_inline_completion);
        register_action(view, cx, Editor::previous_inline_completion);
        register_action(view, cx, Editor::show_inline_completion);
        register_action(view, cx, Editor::context_menu_first);
        register_action(view, cx, Editor::context_menu_prev);
        register_action(view, cx, Editor::context_menu_next);
        register_action(view, cx, Editor::context_menu_last);
        register_action(view, cx, Editor::display_cursor_names);
        register_action(view, cx, Editor::unique_lines_case_insensitive);
        register_action(view, cx, Editor::unique_lines_case_sensitive);
        register_action(view, cx, Editor::accept_partial_inline_completion);
        register_action(view, cx, Editor::accept_inline_completion);
        register_action(view, cx, Editor::revert_selected_hunks);
        register_action(view, cx, Editor::open_active_item_in_terminal)
    }

    fn register_key_listeners(&self, cx: &mut WindowContext, layout: &EditorLayout) {
        let position_map = layout.position_map.clone();
        cx.on_key_event({
            let editor = self.editor.clone();
            let text_hitbox = layout.text_hitbox.clone();
            move |event: &ModifiersChangedEvent, phase, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }

                editor.update(cx, |editor, cx| {
                    Self::modifiers_changed(editor, event, &position_map, &text_hitbox, cx)
                })
            }
        });
    }

    fn modifiers_changed(
        editor: &mut Editor,
        event: &ModifiersChangedEvent,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        let mouse_position = cx.mouse_position();
        if !text_hitbox.is_hovered(cx) {
            return;
        }

        editor.update_hovered_link(
            position_map.point_for_position(text_hitbox.bounds, mouse_position),
            &position_map.snapshot,
            event.modifiers,
            cx,
        )
    }

    fn mouse_left_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        hovered_hunk: Option<&HunkToExpand>,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        gutter_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        if cx.default_prevented() {
            return;
        }

        let mut click_count = event.click_count;
        let mut modifiers = event.modifiers;

        if let Some(hovered_hunk) = hovered_hunk {
            editor.expand_diff_hunk(None, hovered_hunk, cx);
            cx.notify();
            return;
        } else if gutter_hitbox.is_hovered(cx) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !text_hitbox.is_hovered(cx) {
            return;
        }

        if click_count == 2 && !editor.buffer().read(cx).is_singleton() {
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
                        // if double click is made without alt, open the corresponding excerp
                        editor.open_excerpts(&OpenExcerpts, cx);
                        return;
                    }
                }
            }
        }

        let point_for_position =
            position_map.point_for_position(text_hitbox.bounds, event.position);
        let position = point_for_position.previous_valid;
        if modifiers.shift && modifiers.alt {
            editor.select(
                SelectPhase::BeginColumnar {
                    position,
                    reset: false,
                    goal_column: point_for_position.exact_unclipped.column(),
                },
                cx,
            );
        } else if modifiers.shift && !modifiers.control && !modifiers.alt && !modifiers.secondary()
        {
            editor.select(
                SelectPhase::Extend {
                    position,
                    click_count,
                },
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
                cx,
            );
        }

        cx.stop_propagation();
    }

    fn mouse_right_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        if !text_hitbox.is_hovered(cx) {
            return;
        }
        let point_for_position =
            position_map.point_for_position(text_hitbox.bounds, event.position);
        mouse_context_menu::deploy_context_menu(
            editor,
            event.position,
            point_for_position.previous_valid,
            cx,
        );
        cx.stop_propagation();
    }

    fn mouse_middle_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        if !text_hitbox.is_hovered(cx) || cx.default_prevented() {
            return;
        }

        let point_for_position =
            position_map.point_for_position(text_hitbox.bounds, event.position);
        let position = point_for_position.previous_valid;

        editor.select(
            SelectPhase::BeginColumnar {
                position,
                reset: true,
                goal_column: point_for_position.exact_unclipped.column(),
            },
            cx,
        );
    }

    fn mouse_up(
        editor: &mut Editor,
        event: &MouseUpEvent,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        if end_selection {
            editor.select(SelectPhase::End, cx);
        }

        let multi_cursor_setting = EditorSettings::get_global(cx).multi_cursor_modifier;
        let multi_cursor_modifier = match multi_cursor_setting {
            MultiCursorModifier::Alt => event.modifiers.secondary(),
            MultiCursorModifier::CmdOrCtrl => event.modifiers.alt,
        };

        if !pending_nonempty_selections && multi_cursor_modifier && text_hitbox.is_hovered(cx) {
            let point = position_map.point_for_position(text_hitbox.bounds, event.position);
            editor.handle_click_hovered_link(point, event.modifiers, cx);

            cx.stop_propagation();
        } else if end_selection {
            cx.stop_propagation();
        } else if cfg!(target_os = "linux") && event.button == MouseButton::Middle {
            if !text_hitbox.is_hovered(cx) || editor.read_only(cx) {
                return;
            }

            #[cfg(target_os = "linux")]
            if let Some(item) = cx.read_from_clipboard() {
                let point_for_position =
                    position_map.point_for_position(text_hitbox.bounds, event.position);
                let position = point_for_position.previous_valid;

                editor.select(
                    SelectPhase::Begin {
                        position,
                        add: false,
                        click_count: 1,
                    },
                    cx,
                );
                editor.insert(item.text(), cx);
            }
            cx.stop_propagation()
        }
    }

    fn mouse_dragged(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        text_bounds: Bounds<Pixels>,
        cx: &mut ViewContext<Editor>,
    ) {
        if !editor.has_pending_selection() {
            return;
        }

        let point_for_position = position_map.point_for_position(text_bounds, event.position);
        let mut scroll_delta = gpui::Point::<f32>::default();
        let vertical_margin = position_map.line_height.min(text_bounds.size.height / 3.0);
        let top = text_bounds.origin.y + vertical_margin;
        let bottom = text_bounds.lower_left().y - vertical_margin;
        if event.position.y < top {
            scroll_delta.y = -scale_vertical_mouse_autoscroll_delta(top - event.position.y);
        }
        if event.position.y > bottom {
            scroll_delta.y = scale_vertical_mouse_autoscroll_delta(event.position.y - bottom);
        }

        let horizontal_margin = position_map.line_height.min(text_bounds.size.width / 3.0);
        let left = text_bounds.origin.x + horizontal_margin;
        let right = text_bounds.upper_right().x - horizontal_margin;
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
            cx,
        );
    }

    fn mouse_moved(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        text_hitbox: &Hitbox,
        gutter_hitbox: &Hitbox,
        cx: &mut ViewContext<Editor>,
    ) {
        let modifiers = event.modifiers;
        let gutter_hovered = gutter_hitbox.is_hovered(cx);
        editor.set_gutter_hovered(gutter_hovered, cx);

        // Don't trigger hover popover if mouse is hovering over context menu
        if text_hitbox.is_hovered(cx) {
            let point_for_position =
                position_map.point_for_position(text_hitbox.bounds, event.position);

            editor.update_hovered_link(point_for_position, &position_map.snapshot, modifiers, cx);

            if let Some(point) = point_for_position.as_valid() {
                let anchor = position_map
                    .snapshot
                    .buffer_snapshot
                    .anchor_before(point.to_offset(&position_map.snapshot, Bias::Left));
                hover_at(editor, Some(anchor), cx);
                Self::update_visible_cursor(editor, point, position_map, cx);
            } else {
                hover_at(editor, None, cx);
            }
        } else {
            editor.hide_hovered_link(cx);
            hover_at(editor, None, cx);
            if gutter_hovered {
                cx.stop_propagation();
            }
        }
    }

    fn update_visible_cursor(
        editor: &mut Editor,
        point: DisplayPoint,
        position_map: &PositionMap,
        cx: &mut ViewContext<Editor>,
    ) {
        let snapshot = &position_map.snapshot;
        let Some(hub) = editor.collaboration_hub() else {
            return;
        };
        let range = DisplayPoint::new(point.row(), point.column().saturating_sub(1))
            ..DisplayPoint::new(
                point.row(),
                (point.column() + 1).min(snapshot.line_len(point.row())),
            );

        let range = snapshot
            .buffer_snapshot
            .anchor_at(range.start.to_point(&snapshot.display_snapshot), Bias::Left)
            ..snapshot
                .buffer_snapshot
                .anchor_at(range.end.to_point(&snapshot.display_snapshot), Bias::Right);

        let Some(selection) = snapshot.remote_selections_in_range(&range, hub, cx).next() else {
            return;
        };
        let key = crate::HoveredCursor {
            replica_id: selection.replica_id,
            selection_id: selection.selection.id,
        };
        editor.hovered_cursors.insert(
            key.clone(),
            cx.spawn(|editor, mut cx| async move {
                cx.background_executor().timer(CURSORS_VISIBLE_FOR).await;
                editor
                    .update(&mut cx, |editor, cx| {
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
        snapshot: &EditorSnapshot,
        start_row: DisplayRow,
        end_row: DisplayRow,
        cx: &mut WindowContext,
    ) -> (
        Vec<(PlayerColor, Vec<SelectionLayout>)>,
        BTreeMap<DisplayRow, bool>,
        Option<DisplayPoint>,
    ) {
        let mut selections: Vec<(PlayerColor, Vec<SelectionLayout>)> = Vec::new();
        let mut active_rows = BTreeMap::new();
        let mut newest_selection_head = None;
        let editor = self.editor.read(cx);

        if editor.show_local_selections {
            let mut local_selections: Vec<Selection<Point>> = editor
                .selections
                .disjoint_in_range(start_anchor..end_anchor, cx);
            local_selections.extend(editor.selections.pending(cx));
            let mut layouts = Vec::new();
            let newest = editor.selections.newest(cx);
            for selection in local_selections.drain(..) {
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
                    let contains_non_empty_selection =
                        active_rows.entry(DisplayRow(row)).or_insert(!is_empty);
                    *contains_non_empty_selection |= !is_empty;
                }
                layouts.push(layout);
            }

            let player = if editor.read_only(cx) {
                cx.theme().players().read_only()
            } else {
                self.style.local_player
            };

            selections.push((player, layouts));
        }

        if let Some(collaboration_hub) = &editor.collaboration_hub {
            // When following someone, render the local selections in their color.
            if let Some(leader_id) = editor.leader_peer_id {
                if let Some(collaborator) = collaboration_hub.collaborators(cx).get(&leader_id) {
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
                let selection_style = Self::get_participant_color(selection.participant_index, cx);

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
        }
        (selections, active_rows, newest_selection_head)
    }

    fn collect_cursors(
        &self,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
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

    #[allow(clippy::too_many_arguments)]
    fn layout_visible_cursors(
        &self,
        snapshot: &EditorSnapshot,
        selections: &[(PlayerColor, Vec<SelectionLayout>)],
        visible_display_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_position: gpui::Point<f32>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        em_width: Pixels,
        autoscroll_containing_element: bool,
        cx: &mut WindowContext,
    ) -> Vec<CursorLayout> {
        let mut autoscroll_bounds = None;
        let cursor_layouts = self.editor.update(cx, |editor, cx| {
            let mut cursors = Vec::new();
            for (player_color, selections) in selections {
                for selection in selections {
                    let cursor_position = selection.head;

                    let in_range = visible_display_row_range.contains(&cursor_position.row());
                    if (selection.is_local && !editor.show_local_cursors(cx)) || !in_range {
                        continue;
                    }

                    let cursor_row_layout = &line_layouts
                        [cursor_position.row().minus(visible_display_row_range.start) as usize];
                    let cursor_column = cursor_position.column() as usize;

                    let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);
                    let mut block_width =
                        cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
                    if block_width == Pixels::ZERO {
                        block_width = em_width;
                    }
                    let block_text = if let CursorShape::Block = selection.cursor_shape {
                        snapshot.display_chars_at(cursor_position).next().and_then(
                            |(character, _)| {
                                let text = if character == '\n' {
                                    SharedString::from(" ")
                                } else {
                                    SharedString::from(character.to_string())
                                };
                                let len = text.len();

                                let font = cursor_row_layout
                                    .font_id_for_index(cursor_column)
                                    .and_then(|cursor_font_id| {
                                        cx.text_system().get_font_for_id(cursor_font_id)
                                    })
                                    .unwrap_or(self.style.text.font());

                                cx.text_system()
                                    .shape_line(
                                        text,
                                        cursor_row_layout.font_size,
                                        &[TextRun {
                                            len,
                                            font,
                                            color: self.style.background,
                                            background_color: None,
                                            strikethrough: None,
                                            underline: None,
                                        }],
                                    )
                                    .log_err()
                            },
                        )
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
                    cursor.layout(content_origin, cursor_name, cx);
                    cursors.push(cursor);
                }
            }
            cursors
        });

        if let Some(bounds) = autoscroll_bounds {
            cx.request_autoscroll(bounds);
        }

        cursor_layouts
    }

    fn layout_scrollbar(
        &self,
        snapshot: &EditorSnapshot,
        bounds: Bounds<Pixels>,
        scroll_position: gpui::Point<f32>,
        rows_per_page: f32,
        non_visible_cursors: bool,
        cx: &mut WindowContext,
    ) -> Option<ScrollbarLayout> {
        let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
        let show_scrollbars = match scrollbar_settings.show {
            ShowScrollbar::Auto => {
                let editor = self.editor.read(cx);
                let is_singleton = editor.is_singleton(cx);
                // Git
                (is_singleton && scrollbar_settings.git_diff && snapshot.buffer_snapshot.has_git_diffs())
                    ||
                    // Buffer Search Results
                    (is_singleton && scrollbar_settings.search_results && editor.has_background_highlights::<BufferSearchHighlights>())
                    ||
                    // Selected Symbol Occurrences
                    (is_singleton && scrollbar_settings.selected_symbol && (editor.has_background_highlights::<DocumentHighlightRead>() || editor.has_background_highlights::<DocumentHighlightWrite>()))
                    ||
                    // Diagnostics
                    (is_singleton && scrollbar_settings.diagnostics && snapshot.buffer_snapshot.has_diagnostics())
                    ||
                    // Cursors out of sight
                    non_visible_cursors
                    ||
                    // Scrollmanager
                    editor.scroll_manager.scrollbars_visible()
            }
            ShowScrollbar::System => self.editor.read(cx).scroll_manager.scrollbars_visible(),
            ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        };
        if snapshot.mode != EditorMode::Full {
            return None;
        }

        let visible_row_range = scroll_position.y..scroll_position.y + rows_per_page;

        // If a drag took place after we started dragging the scrollbar,
        // cancel the scrollbar drag.
        if cx.has_active_drag() {
            self.editor.update(cx, |editor, cx| {
                editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
            });
        }

        let track_bounds = Bounds::from_corners(
            point(self.scrollbar_left(&bounds), bounds.origin.y),
            point(bounds.lower_right().x, bounds.lower_left().y),
        );

        let height = bounds.size.height;
        let total_rows = snapshot.max_point().row().as_f32() + rows_per_page;
        let px_per_row = height / total_rows;
        let thumb_height = (rows_per_page * px_per_row).max(ScrollbarLayout::MIN_THUMB_HEIGHT);
        let row_height = (height - thumb_height) / snapshot.max_point().row().as_f32();

        Some(ScrollbarLayout {
            hitbox: cx.insert_hitbox(track_bounds, false),
            visible_row_range,
            row_height,
            visible: show_scrollbars,
            thumb_height,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn prepaint_gutter_fold_toggles(
        &self,
        toggles: &mut [Option<AnyElement>],
        line_height: Pixels,
        gutter_dimensions: &GutterDimensions,
        gutter_settings: crate::editor_settings::Gutter,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_hitbox: &Hitbox,
        cx: &mut WindowContext,
    ) {
        for (ix, fold_indicator) in toggles.iter_mut().enumerate() {
            if let Some(fold_indicator) = fold_indicator {
                debug_assert!(gutter_settings.folds);
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height * 0.55),
                );
                let fold_indicator_size = fold_indicator.layout_as_root(available_space, cx);

                let position = point(
                    gutter_dimensions.width - gutter_dimensions.right_padding,
                    ix as f32 * line_height - (scroll_pixel_position.y % line_height),
                );
                let centering_offset = point(
                    (gutter_dimensions.right_padding + gutter_dimensions.margin
                        - fold_indicator_size.width)
                        / 2.,
                    (line_height - fold_indicator_size.height) / 2.,
                );
                let origin = gutter_hitbox.origin + position + centering_offset;
                fold_indicator.prepaint_as_root(origin, available_space, cx);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn prepaint_flap_trailers(
        &self,
        trailers: Vec<Option<AnyElement>>,
        lines: &[LineWithInvisibles],
        line_height: Pixels,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        em_width: Pixels,
        cx: &mut WindowContext,
    ) -> Vec<Option<FlapTrailerLayout>> {
        trailers
            .into_iter()
            .enumerate()
            .map(|(ix, element)| {
                let mut element = element?;
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height),
                );
                let size = element.layout_as_root(available_space, cx);

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
                element.prepaint_as_root(origin, available_space, cx);
                Some(FlapTrailerLayout {
                    element,
                    bounds: Bounds::new(origin, size),
                })
            })
            .collect()
    }

    // Folds contained in a hunk are ignored apart from shrinking visual size
    // If a fold contains any hunks then that fold line is marked as modified
    fn layout_git_gutters(
        &self,
        line_height: Pixels,
        gutter_hitbox: &Hitbox,
        display_rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<(DisplayDiffHunk, Option<Hitbox>)> {
        let buffer_snapshot = &snapshot.buffer_snapshot;

        let buffer_start_row = MultiBufferRow(
            DisplayPoint::new(display_rows.start, 0)
                .to_point(snapshot)
                .row,
        );
        let buffer_end_row = MultiBufferRow(
            DisplayPoint::new(display_rows.end, 0)
                .to_point(snapshot)
                .row,
        );

        let expanded_hunk_display_rows = self.editor.update(cx, |editor, _| {
            editor
                .expanded_hunks
                .hunks(false)
                .map(|expanded_hunk| {
                    let start_row = expanded_hunk
                        .hunk_range
                        .start
                        .to_display_point(snapshot)
                        .row();
                    let end_row = expanded_hunk
                        .hunk_range
                        .end
                        .to_display_point(snapshot)
                        .row();
                    (start_row, end_row)
                })
                .collect::<HashMap<_, _>>()
        });

        buffer_snapshot
            .git_diff_hunks_in_range(buffer_start_row..buffer_end_row)
            .map(|hunk| diff_hunk_to_display(&hunk, snapshot))
            .dedup()
            .map(|hunk| {
                let hitbox = if let DisplayDiffHunk::Unfolded {
                    display_row_range, ..
                } = &hunk
                {
                    let was_expanded = expanded_hunk_display_rows
                        .get(&display_row_range.start)
                        .map(|expanded_end_row| expanded_end_row == &display_row_range.end)
                        .unwrap_or(false);
                    if was_expanded {
                        None
                    } else {
                        let hunk_bounds = Self::diff_hunk_bounds(
                            &snapshot,
                            line_height,
                            gutter_hitbox.bounds,
                            &hunk,
                        );
                        Some(cx.insert_hitbox(hunk_bounds, true))
                    }
                } else {
                    None
                };
                (hunk, hitbox)
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_inline_blame(
        &self,
        display_row: DisplayRow,
        display_snapshot: &DisplaySnapshot,
        line_layout: &LineWithInvisibles,
        flap_trailer: Option<&FlapTrailerLayout>,
        em_width: Pixels,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        if !self
            .editor
            .update(cx, |editor, cx| editor.render_git_blame_inline(cx))
        {
            return None;
        }

        let workspace = self
            .editor
            .read(cx)
            .workspace
            .as_ref()
            .map(|(w, _)| w.clone());

        let display_point = DisplayPoint::new(display_row, 0);
        let buffer_row = MultiBufferRow(display_point.to_point(display_snapshot).row);

        let blame = self.editor.read(cx).blame.clone()?;
        let blame_entry = blame
            .update(cx, |blame, cx| {
                blame.blame_for_rows([Some(buffer_row)], cx).next()
            })
            .flatten()?;

        let mut element =
            render_inline_blame_entry(&blame, blame_entry, &self.style, workspace, cx);

        let start_y = content_origin.y
            + line_height * (display_row.as_f32() - scroll_pixel_position.y / line_height);

        let start_x = {
            const INLINE_BLAME_PADDING_EM_WIDTHS: f32 = 6.;

            let line_end = if let Some(flap_trailer) = flap_trailer {
                flap_trailer.bounds.right()
            } else {
                content_origin.x - scroll_pixel_position.x + line_layout.width
            };
            let padded_line_end = line_end + em_width * INLINE_BLAME_PADDING_EM_WIDTHS;

            let min_column_in_pixels = ProjectSettings::get_global(cx)
                .git
                .inline_blame
                .and_then(|settings| settings.min_column)
                .map(|col| self.column_pixels(col as usize, cx))
                .unwrap_or(px(0.));
            let min_start = content_origin.x - scroll_pixel_position.x + min_column_in_pixels;

            cmp::max(padded_line_end, min_start)
        };

        let absolute_offset = point(start_x, start_y);
        let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);

        element.prepaint_as_root(absolute_offset, available_space, cx);

        Some(element)
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_blame_entries(
        &self,
        buffer_rows: impl Iterator<Item = Option<MultiBufferRow>>,
        em_width: Pixels,
        scroll_position: gpui::Point<f32>,
        line_height: Pixels,
        gutter_hitbox: &Hitbox,
        max_width: Option<Pixels>,
        cx: &mut WindowContext,
    ) -> Option<Vec<AnyElement>> {
        if !self
            .editor
            .update(cx, |editor, cx| editor.render_git_blame_gutter(cx))
        {
            return None;
        }

        let blame = self.editor.read(cx).blame.clone()?;
        let blamed_rows: Vec<_> = blame.update(cx, |blame, cx| {
            blame.blame_for_rows(buffer_rows, cx).collect()
        });

        let width = if let Some(max_width) = max_width {
            AvailableSpace::Definite(max_width)
        } else {
            AvailableSpace::MaxContent
        };
        let scroll_top = scroll_position.y * line_height;
        let start_x = em_width * 1;

        let mut last_used_color: Option<(PlayerColor, Oid)> = None;

        let shaped_lines = blamed_rows
            .into_iter()
            .enumerate()
            .flat_map(|(ix, blame_entry)| {
                if let Some(blame_entry) = blame_entry {
                    let mut element = render_blame_entry(
                        ix,
                        &blame,
                        blame_entry,
                        &self.style,
                        &mut last_used_color,
                        self.editor.clone(),
                        cx,
                    );

                    let start_y = ix as f32 * line_height - (scroll_top % line_height);
                    let absolute_offset = gutter_hitbox.origin + point(start_x, start_y);

                    element.prepaint_as_root(
                        absolute_offset,
                        size(width, AvailableSpace::MinContent),
                        cx,
                    );

                    Some(element)
                } else {
                    None
                }
            })
            .collect();

        Some(shaped_lines)
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_indent_guides(
        &self,
        content_origin: gpui::Point<Pixels>,
        text_origin: gpui::Point<Pixels>,
        visible_buffer_range: Range<MultiBufferRow>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        snapshot: &DisplaySnapshot,
        cx: &mut WindowContext,
    ) -> Option<Vec<IndentGuideLayout>> {
        let indent_guides = self.editor.update(cx, |editor, cx| {
            editor.indent_guides(visible_buffer_range, snapshot, cx)
        })?;

        let active_indent_guide_indices = self.editor.update(cx, |editor, cx| {
            editor
                .find_active_indent_guide_indices(&indent_guides, snapshot, cx)
                .unwrap_or_default()
        });

        Some(
            indent_guides
                .into_iter()
                .enumerate()
                .filter_map(|(i, indent_guide)| {
                    let single_indent_width =
                        self.column_pixels(indent_guide.tab_size as usize, cx);
                    let total_width = single_indent_width * indent_guide.depth as f32;
                    let start_x = content_origin.x + total_width - scroll_pixel_position.x;
                    if start_x >= text_origin.x {
                        let (offset_y, length) = Self::calculate_indent_guide_bounds(
                            indent_guide.multibuffer_row_range.clone(),
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
            if matches!(block, TransformBlock::ExcerptHeader { .. }) {
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
            if matches!(block, TransformBlock::ExcerptHeader { .. }) {
                found_excerpt_header = true;
            }
            block_height += block.height();
        }
        if found_excerpt_header {
            length -= block_height as f32 * line_height;
        }

        (offset_y, length)
    }

    fn layout_run_indicators(
        &self,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<AnyElement> {
        self.editor.update(cx, |editor, cx| {
            let active_task_indicator_row =
                if let Some(crate::ContextMenu::CodeActions(CodeActionsMenu {
                    deployed_from_indicator,
                    actions,
                    ..
                })) = editor.context_menu.read().as_ref()
                {
                    actions
                        .tasks
                        .as_ref()
                        .map(|tasks| tasks.position.to_display_point(snapshot).row())
                        .or_else(|| *deployed_from_indicator)
                } else {
                    None
                };
            editor
                .tasks
                .iter()
                .filter_map(|(_, tasks)| {
                    let multibuffer_point = tasks.offset.0.to_point(&snapshot.buffer_snapshot);
                    let multibuffer_row = MultiBufferRow(multibuffer_point.row);
                    if snapshot.is_line_folded(multibuffer_row) {
                        return None;
                    }
                    let display_row = multibuffer_point.to_display_point(snapshot).row();
                    let button = editor.render_run_indicator(
                        &self.style,
                        Some(display_row) == active_task_indicator_row,
                        display_row,
                        cx,
                    );

                    let button = prepaint_gutter_button(
                        button,
                        display_row,
                        line_height,
                        gutter_dimensions,
                        scroll_pixel_position,
                        gutter_hitbox,
                        cx,
                    );
                    Some(button)
                })
                .collect_vec()
        })
    }

    fn layout_code_actions_indicator(
        &self,
        line_height: Pixels,
        newest_selection_head: DisplayPoint,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        let mut active = false;
        let mut button = None;
        let row = newest_selection_head.row();
        self.editor.update(cx, |editor, cx| {
            if let Some(crate::ContextMenu::CodeActions(CodeActionsMenu {
                deployed_from_indicator,
                ..
            })) = editor.context_menu.read().as_ref()
            {
                active = deployed_from_indicator.map_or(true, |indicator_row| indicator_row == row);
            };
            button = editor.render_code_actions_indicator(&self.style, row, active, cx);
        });

        let button = prepaint_gutter_button(
            button?,
            row,
            line_height,
            gutter_dimensions,
            scroll_pixel_position,
            gutter_hitbox,
            cx,
        );

        Some(button)
    }

    fn get_participant_color(
        participant_index: Option<ParticipantIndex>,
        cx: &WindowContext,
    ) -> PlayerColor {
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
            .buffer_rows(start)
            .take(1 + end.minus(start) as usize)
            .collect::<Vec<_>>();

        let head_idx = relative_to.minus(start);
        let mut delta = 1;
        let mut i = head_idx + 1;
        while i < buffer_rows.len() as u32 {
            if buffer_rows[i as usize].is_some() {
                if rows.contains(&DisplayRow(i + start.0)) {
                    relative_rows.insert(DisplayRow(i + start.0), delta);
                }
                delta += 1;
            }
            i += 1;
        }
        delta = 1;
        i = head_idx.min(buffer_rows.len() as u32 - 1);
        while i > 0 && buffer_rows[i as usize].is_none() {
            i -= 1;
        }

        while i > 0 {
            i -= 1;
            if buffer_rows[i as usize].is_some() {
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
        rows: Range<DisplayRow>,
        buffer_rows: impl Iterator<Item = Option<MultiBufferRow>>,
        active_rows: &BTreeMap<DisplayRow, bool>,
        newest_selection_head: Option<DisplayPoint>,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<Option<ShapedLine>> {
        let include_line_numbers = snapshot.show_line_numbers.unwrap_or_else(|| {
            EditorSettings::get_global(cx).gutter.line_numbers && snapshot.mode == EditorMode::Full
        });
        if !include_line_numbers {
            return Vec::new();
        }

        let editor = self.editor.read(cx);
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
        let font_size = self.style.text.font_size.to_pixels(cx.rem_size());

        let is_relative = EditorSettings::get_global(cx).relative_line_numbers;
        let relative_to = if is_relative {
            Some(newest_selection_head.row())
        } else {
            None
        };
        let relative_rows = self.calculate_relative_line_numbers(snapshot, &rows, relative_to);
        let mut line_number = String::new();
        buffer_rows
            .into_iter()
            .enumerate()
            .map(|(ix, multibuffer_row)| {
                let multibuffer_row = multibuffer_row?;
                let display_row = DisplayRow(rows.start.0 + ix as u32);
                let color = if active_rows.contains_key(&display_row) {
                    cx.theme().colors().editor_active_line_number
                } else {
                    cx.theme().colors().editor_line_number
                };
                line_number.clear();
                let default_number = multibuffer_row.0 + 1;
                let number = relative_rows
                    .get(&DisplayRow(ix as u32 + rows.start.0))
                    .unwrap_or(&default_number);
                write!(&mut line_number, "{number}").unwrap();
                let run = TextRun {
                    len: line_number.len(),
                    font: self.style.text.font(),
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let shaped_line = cx
                    .text_system()
                    .shape_line(line_number.clone().into(), font_size, &[run])
                    .unwrap();
                Some(shaped_line)
            })
            .collect()
    }

    fn layout_gutter_fold_toggles(
        &self,
        rows: Range<DisplayRow>,
        buffer_rows: impl IntoIterator<Item = Option<MultiBufferRow>>,
        active_rows: &BTreeMap<DisplayRow, bool>,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<Option<AnyElement>> {
        let include_fold_statuses = EditorSettings::get_global(cx).gutter.folds
            && snapshot.mode == EditorMode::Full
            && self.editor.read(cx).is_singleton(cx);
        if include_fold_statuses {
            buffer_rows
                .into_iter()
                .enumerate()
                .map(|(ix, row)| {
                    if let Some(multibuffer_row) = row {
                        let display_row = DisplayRow(rows.start.0 + ix as u32);
                        let active = active_rows.contains_key(&display_row);
                        snapshot.render_fold_toggle(
                            multibuffer_row,
                            active,
                            self.editor.clone(),
                            cx,
                        )
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn layout_flap_trailers(
        &self,
        buffer_rows: impl IntoIterator<Item = Option<MultiBufferRow>>,
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<Option<AnyElement>> {
        buffer_rows
            .into_iter()
            .map(|row| {
                if let Some(multibuffer_row) = row {
                    snapshot.render_flap_trailer(multibuffer_row, cx)
                } else {
                    None
                }
            })
            .collect()
    }

    fn layout_lines(
        &self,
        rows: Range<DisplayRow>,
        line_number_layouts: &[Option<ShapedLine>],
        snapshot: &EditorSnapshot,
        cx: &mut WindowContext,
    ) -> Vec<LineWithInvisibles> {
        if rows.start >= rows.end {
            return Vec::new();
        }

        // Show the placeholder when the editor is empty
        if snapshot.is_empty() {
            let font_size = self.style.text.font_size.to_pixels(cx.rem_size());
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
                        font: self.style.text.font(),
                        color: placeholder_color,
                        background_color: None,
                        underline: Default::default(),
                        strikethrough: None,
                    };
                    cx.text_system()
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
            let chunks = snapshot.highlighted_chunks(rows.clone(), true, &self.style);
            LineWithInvisibles::from_chunks(
                chunks,
                &self.style.text,
                MAX_LINE_LEN,
                rows.len(),
                line_number_layouts,
                snapshot.mode,
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
        cx: &mut WindowContext,
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
                cx,
            );
        }
        line_elements
    }

    #[allow(clippy::too_many_arguments)]
    fn build_blocks(
        &self,
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        scroll_width: &mut Pixels,
        gutter_dimensions: &GutterDimensions,
        em_width: Pixels,
        text_x: Pixels,
        line_height: Pixels,
        line_layouts: &[LineWithInvisibles],
        cx: &mut WindowContext,
    ) -> Vec<BlockLayout> {
        let mut block_id = 0;
        let (fixed_blocks, non_fixed_blocks) = snapshot
            .blocks_in_range(rows.clone())
            .partition::<Vec<_>, _>(|(_, block)| match block {
                TransformBlock::ExcerptHeader { .. } => false,
                TransformBlock::Custom(block) => block.style() == BlockStyle::Fixed,
                TransformBlock::ExcerptFooter { .. } => false,
            });

        let render_block = |block: &TransformBlock,
                            available_space: Size<AvailableSpace>,
                            block_id: usize,
                            block_row_start: DisplayRow,
                            cx: &mut WindowContext| {
            let mut element = match block {
                TransformBlock::Custom(block) => {
                    let align_to = block
                        .position()
                        .to_point(&snapshot.buffer_snapshot)
                        .to_display_point(snapshot);
                    let anchor_x = text_x
                        + if rows.contains(&align_to.row()) {
                            line_layouts[align_to.row().minus(rows.start) as usize]
                                .x_for_index(align_to.column() as usize)
                        } else {
                            layout_line(align_to.row(), snapshot, &self.style, cx)
                                .x_for_index(align_to.column() as usize)
                        };

                    block.render(&mut BlockContext {
                        context: cx,
                        anchor_x,
                        gutter_dimensions,
                        line_height,
                        em_width,
                        block_id,
                        max_width: text_hitbox.size.width.max(*scroll_width),
                        editor_style: &self.style,
                    })
                }

                TransformBlock::ExcerptHeader {
                    buffer,
                    range,
                    starts_new_buffer,
                    height,
                    id,
                    show_excerpt_controls,
                    ..
                } => {
                    let include_root = self
                        .editor
                        .read(cx)
                        .project
                        .as_ref()
                        .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
                        .unwrap_or_default();

                    #[derive(Clone)]
                    struct JumpData {
                        position: Point,
                        anchor: text::Anchor,
                        path: ProjectPath,
                        line_offset_from_top: u32,
                    }

                    let jump_data = project::File::from_dyn(buffer.file()).map(|file| {
                        let jump_path = ProjectPath {
                            worktree_id: file.worktree_id(cx),
                            path: file.path.clone(),
                        };
                        let jump_anchor = range
                            .primary
                            .as_ref()
                            .map_or(range.context.start, |primary| primary.start);

                        let excerpt_start = range.context.start;
                        let jump_position = language::ToPoint::to_point(&jump_anchor, buffer);
                        let offset_from_excerpt_start = if jump_anchor == excerpt_start {
                            0
                        } else {
                            let excerpt_start_row =
                                language::ToPoint::to_point(&jump_anchor, buffer).row;
                            jump_position.row - excerpt_start_row
                        };

                        let line_offset_from_top =
                            block_row_start.0 + *height as u32 + offset_from_excerpt_start
                                - snapshot
                                    .scroll_anchor
                                    .scroll_position(&snapshot.display_snapshot)
                                    .y as u32;

                        JumpData {
                            position: jump_position,
                            anchor: jump_anchor,
                            path: jump_path,
                            line_offset_from_top,
                        }
                    });

                    let icon_offset = gutter_dimensions.width
                        - (gutter_dimensions.left_padding + gutter_dimensions.margin);

                    let element = if *starts_new_buffer {
                        let path = buffer.resolve_file_path(cx, include_root);
                        let mut filename = None;
                        let mut parent_path = None;
                        // Can't use .and_then() because `.file_name()` and `.parent()` return references :(
                        if let Some(path) = path {
                            filename = path.file_name().map(|f| f.to_string_lossy().to_string());
                            parent_path = path
                                .parent()
                                .map(|p| SharedString::from(p.to_string_lossy().to_string() + "/"));
                        }

                        let header_padding = px(6.0);

                        v_flex()
                            .id(("path excerpt header", block_id))
                            .size_full()
                            .p(header_padding)
                            .child(
                                h_flex()
                                    .flex_basis(Length::Definite(DefiniteLength::Fraction(0.667)))
                                    .id("path header block")
                                    .pl(gpui::px(12.))
                                    .pr(gpui::px(8.))
                                    .rounded_md()
                                    .shadow_md()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .bg(cx.theme().colors().editor_subheader_background)
                                    .justify_between()
                                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                                    .child(
                                        h_flex().gap_3().child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    filename
                                                        .map(SharedString::from)
                                                        .unwrap_or_else(|| "untitled".into()),
                                                )
                                                .when_some(parent_path, |then, path| {
                                                    then.child(
                                                        div().child(path).text_color(
                                                            cx.theme().colors().text_muted,
                                                        ),
                                                    )
                                                }),
                                        ),
                                    )
                                    .when_some(jump_data.clone(), |this, jump_data| {
                                        this.cursor_pointer()
                                            .tooltip(|cx| {
                                                Tooltip::for_action(
                                                    "Jump to File",
                                                    &OpenExcerpts,
                                                    cx,
                                                )
                                            })
                                            .on_mouse_down(MouseButton::Left, |_, cx| {
                                                cx.stop_propagation()
                                            })
                                            .on_click(cx.listener_for(&self.editor, {
                                                move |editor, _, cx| {
                                                    editor.jump(
                                                        jump_data.path.clone(),
                                                        jump_data.position,
                                                        jump_data.anchor,
                                                        jump_data.line_offset_from_top,
                                                        cx,
                                                    );
                                                }
                                            }))
                                    }),
                            )
                            .children(show_excerpt_controls.then(|| {
                                h_flex()
                                    .flex_basis(Length::Definite(DefiniteLength::Fraction(0.333)))
                                    .pt_1()
                                    .justify_end()
                                    .flex_none()
                                    .w(icon_offset - header_padding)
                                    .child(
                                        ButtonLike::new("expand-icon")
                                            .style(ButtonStyle::Transparent)
                                            .child(
                                                svg()
                                                    .path(IconName::ArrowUpFromLine.path())
                                                    .size(IconSize::XSmall.rems())
                                                    .text_color(
                                                        cx.theme().colors().editor_line_number,
                                                    )
                                                    .group("")
                                                    .hover(|style| {
                                                        style.text_color(
                                                            cx.theme()
                                                                .colors()
                                                                .editor_active_line_number,
                                                        )
                                                    }),
                                            )
                                            .on_click(cx.listener_for(&self.editor, {
                                                let id = *id;
                                                move |editor, _, cx| {
                                                    editor.expand_excerpt(
                                                        id,
                                                        multi_buffer::ExpandExcerptDirection::Up,
                                                        cx,
                                                    );
                                                }
                                            }))
                                            .tooltip({
                                                move |cx| {
                                                    Tooltip::for_action(
                                                        "Expand Excerpt",
                                                        &ExpandExcerpts { lines: 0 },
                                                        cx,
                                                    )
                                                }
                                            }),
                                    )
                            }))
                    } else {
                        v_flex()
                            .id(("excerpt header", block_id))
                            .size_full()
                            .child(
                                div()
                                    .flex()
                                    .v_flex()
                                    .justify_start()
                                    .id("jump to collapsed context")
                                    .w(relative(1.0))
                                    .h_full()
                                    .child(
                                        div()
                                            .h_px()
                                            .w_full()
                                            .bg(cx.theme().colors().border_variant)
                                            .group_hover("excerpt-jump-action", |style| {
                                                style.bg(cx.theme().colors().border)
                                            }),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .justify_end()
                                    .flex_none()
                                    .w(icon_offset)
                                    .h_full()
                                    .child(
                                        show_excerpt_controls.then(|| {
                                            ButtonLike::new("expand-icon")
                                                .style(ButtonStyle::Transparent)
                                                .child(
                                                    svg()
                                                        .path(IconName::ArrowUpFromLine.path())
                                                        .size(IconSize::XSmall.rems())
                                                        .text_color(
                                                            cx.theme().colors().editor_line_number,
                                                        )
                                                        .group("")
                                                        .hover(|style| {
                                                            style.text_color(
                                                                cx.theme()
                                                                    .colors()
                                                                    .editor_active_line_number,
                                                            )
                                                        }),
                                                )
                                                .on_click(cx.listener_for(&self.editor, {
                                                    let id = *id;
                                                    move |editor, _, cx| {
                                                        editor.expand_excerpt(
                                                            id,
                                                            multi_buffer::ExpandExcerptDirection::Up,
                                                            cx,
                                                        );
                                                    }
                                                }))
                                                .tooltip({
                                                    move |cx| {
                                                        Tooltip::for_action(
                                                            "Expand Excerpt",
                                                            &ExpandExcerpts { lines: 0 },
                                                            cx,
                                                        )
                                                    }
                                                })
                                        }).unwrap_or_else(|| {
                                            ButtonLike::new("jump-icon")
                                                .style(ButtonStyle::Transparent)
                                                .child(
                                                    svg()
                                                        .path(IconName::ArrowUpRight.path())
                                                        .size(IconSize::XSmall.rems())
                                                        .text_color(
                                                            cx.theme().colors().border_variant,
                                                        )
                                                        .group("excerpt-jump-action")
                                                        .group_hover("excerpt-jump-action", |style| {
                                                            style.text_color(
                                                                cx.theme().colors().border

                                                            )
                                                        })
                                                )
                                                .when_some(jump_data.clone(), |this, jump_data| {
                                                    this.on_click(cx.listener_for(&self.editor, {
                                                        let path = jump_data.path.clone();
                                                        move |editor, _, cx| {
                                                            cx.stop_propagation();

                                                            editor.jump(
                                                                path.clone(),
                                                                jump_data.position,
                                                                jump_data.anchor,
                                                                jump_data.line_offset_from_top,
                                                                cx,
                                                            );
                                                        }
                                                    }))
                                                    .tooltip(move |cx| {
                                                        Tooltip::for_action(
                                                            format!(
                                                                "Jump to {}:L{}",
                                                                jump_data.path.path.display(),
                                                                jump_data.position.row + 1
                                                            ),
                                                            &OpenExcerpts,
                                                            cx,
                                                        )
                                                    })
                                                })
                                        })

                                    ),
                            )
                            .group("excerpt-jump-action")
                            .cursor_pointer()
                            .when_some(jump_data.clone(), |this, jump_data| {
                                this.on_click(cx.listener_for(&self.editor, {
                                    let path = jump_data.path.clone();
                                    move |editor, _, cx| {
                                        cx.stop_propagation();

                                        editor.jump(
                                            path.clone(),
                                            jump_data.position,
                                            jump_data.anchor,
                                            jump_data.line_offset_from_top,
                                            cx,
                                        );
                                    }
                                }))
                                .tooltip(move |cx| {
                                    Tooltip::for_action(
                                        format!(
                                            "Jump to {}:L{}",
                                            jump_data.path.path.display(),
                                            jump_data.position.row + 1
                                        ),
                                        &OpenExcerpts,
                                        cx,
                                    )
                                })
                            })
                    };
                    element.into_any()
                }

                TransformBlock::ExcerptFooter { id, .. } => {
                    let element = v_flex().id(("excerpt footer", block_id)).size_full().child(
                        h_flex()
                            .justify_end()
                            .flex_none()
                            .w(gutter_dimensions.width
                                - (gutter_dimensions.left_padding + gutter_dimensions.margin))
                            .h_full()
                            .child(
                                ButtonLike::new("expand-icon")
                                    .style(ButtonStyle::Transparent)
                                    .child(
                                        svg()
                                            .path(IconName::ArrowDownFromLine.path())
                                            .size(IconSize::XSmall.rems())
                                            .text_color(cx.theme().colors().editor_line_number)
                                            .group("")
                                            .hover(|style| {
                                                style.text_color(
                                                    cx.theme().colors().editor_active_line_number,
                                                )
                                            }),
                                    )
                                    .on_click(cx.listener_for(&self.editor, {
                                        let id = *id;
                                        move |editor, _, cx| {
                                            editor.expand_excerpt(
                                                id,
                                                multi_buffer::ExpandExcerptDirection::Down,
                                                cx,
                                            );
                                        }
                                    }))
                                    .tooltip({
                                        move |cx| {
                                            Tooltip::for_action(
                                                "Expand Excerpt",
                                                &ExpandExcerpts { lines: 0 },
                                                cx,
                                            )
                                        }
                                    }),
                            ),
                    );
                    element.into_any()
                }
            };

            let size = element.layout_as_root(available_space, cx);
            (element, size)
        };

        let mut fixed_block_max_width = Pixels::ZERO;
        let mut blocks = Vec::new();
        for (row, block) in fixed_blocks {
            let available_space = size(
                AvailableSpace::MinContent,
                AvailableSpace::Definite(block.height() as f32 * line_height),
            );
            let (element, element_size) = render_block(block, available_space, block_id, row, cx);
            block_id += 1;
            fixed_block_max_width = fixed_block_max_width.max(element_size.width + em_width);
            blocks.push(BlockLayout {
                row,
                element,
                available_space,
                style: BlockStyle::Fixed,
            });
        }
        for (row, block) in non_fixed_blocks {
            let style = match block {
                TransformBlock::Custom(block) => block.style(),
                TransformBlock::ExcerptHeader { .. } => BlockStyle::Sticky,
                TransformBlock::ExcerptFooter { .. } => BlockStyle::Sticky,
            };
            let width = match style {
                BlockStyle::Sticky => hitbox.size.width,
                BlockStyle::Flex => hitbox
                    .size
                    .width
                    .max(fixed_block_max_width)
                    .max(gutter_dimensions.width + *scroll_width),
                BlockStyle::Fixed => unreachable!(),
            };
            let available_space = size(
                AvailableSpace::Definite(width),
                AvailableSpace::Definite(block.height() as f32 * line_height),
            );
            let (element, _) = render_block(block, available_space, block_id, row, cx);
            block_id += 1;
            blocks.push(BlockLayout {
                row,
                element,
                available_space,
                style,
            });
        }

        *scroll_width = (*scroll_width).max(fixed_block_max_width - gutter_dimensions.width);
        blocks
    }

    fn layout_blocks(
        &self,
        blocks: &mut Vec<BlockLayout>,
        hitbox: &Hitbox,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<Pixels>,
        cx: &mut WindowContext,
    ) {
        for block in blocks {
            let mut origin = hitbox.origin
                + point(
                    Pixels::ZERO,
                    block.row.as_f32() * line_height - scroll_pixel_position.y,
                );
            if !matches!(block.style, BlockStyle::Sticky) {
                origin += point(-scroll_pixel_position.x, Pixels::ZERO);
            }
            block
                .element
                .prepaint_as_root(origin, block.available_space, cx);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_context_menu(
        &self,
        line_height: Pixels,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        start_row: DisplayRow,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_layouts: &[LineWithInvisibles],
        newest_selection_head: DisplayPoint,
        gutter_overshoot: Pixels,
        cx: &mut WindowContext,
    ) -> bool {
        let max_height = cmp::min(
            12. * line_height,
            cmp::max(3. * line_height, (hitbox.size.height - line_height) / 2.),
        );
        let Some((position, mut context_menu)) = self.editor.update(cx, |editor, cx| {
            if editor.context_menu_visible() {
                editor.render_context_menu(newest_selection_head, &self.style, max_height, cx)
            } else {
                None
            }
        }) else {
            return false;
        };

        let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
        let context_menu_size = context_menu.layout_as_root(available_space, cx);

        let (x, y) = match position {
            crate::ContextMenuOrigin::EditorPoint(point) => {
                let cursor_row_layout = &line_layouts[point.row().minus(start_row) as usize];
                let x = cursor_row_layout.x_for_index(point.column() as usize)
                    - scroll_pixel_position.x;
                let y = point.row().next_row().as_f32() * line_height - scroll_pixel_position.y;
                (x, y)
            }
            crate::ContextMenuOrigin::GutterIndicator(row) => {
                // Context menu was spawned via a click on a gutter. Ensure it's a bit closer to the indicator than just a plain first column of the
                // text field.
                let x = -gutter_overshoot;
                let y = row.next_row().as_f32() * line_height - scroll_pixel_position.y;
                (x, y)
            }
        };

        let mut list_origin = content_origin + point(x, y);
        let list_width = context_menu_size.width;
        let list_height = context_menu_size.height;

        // Snap the right edge of the list to the right edge of the window if
        // its horizontal bounds overflow.
        if list_origin.x + list_width > cx.viewport_size().width {
            list_origin.x = (cx.viewport_size().width - list_width).max(Pixels::ZERO);
        }

        if list_origin.y + list_height > text_hitbox.lower_right().y {
            list_origin.y -= line_height + list_height;
        }

        cx.defer_draw(context_menu, list_origin, 1);
        true
    }

    fn layout_mouse_context_menu(&self, cx: &mut WindowContext) -> Option<AnyElement> {
        let mouse_context_menu = self.editor.read(cx).mouse_context_menu.as_ref()?;
        let mut element = deferred(
            anchored()
                .position(mouse_context_menu.position)
                .child(mouse_context_menu.context_menu.clone())
                .anchor(AnchorCorner::TopLeft)
                .snap_to_window(),
        )
        .with_priority(1)
        .into_any();

        element.prepaint_as_root(gpui::Point::default(), AvailableSpace::min_size(), cx);
        Some(element)
    }

    #[allow(clippy::too_many_arguments)]
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
        cx: &mut WindowContext,
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
                &snapshot,
                &self.style,
                visible_display_row_range.clone(),
                max_size,
                editor.workspace.as_ref().map(|(w, _)| w.clone()),
                cx,
            )
        });
        let Some((position, hover_popovers)) = hover_popovers else {
            return;
        };

        let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);

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
            let size = hover_popover.layout_as_root(available_space, cx);
            let horizontal_offset =
                (text_hitbox.upper_right().x - (hovered_point.x + size.width)).min(Pixels::ZERO);

            overall_height += HOVER_POPOVER_GAP + size.height;

            measured_hover_popovers.push(MeasuredHoverPopover {
                element: hover_popover,
                size,
                horizontal_offset,
            });
        }
        overall_height += HOVER_POPOVER_GAP;

        fn draw_occluder(width: Pixels, origin: gpui::Point<Pixels>, cx: &mut WindowContext) {
            let mut occlusion = div()
                .size_full()
                .occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .into_any_element();
            occlusion.layout_as_root(size(width, HOVER_POPOVER_GAP).into(), cx);
            cx.defer_draw(occlusion, origin, 2);
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

                cx.defer_draw(popover.element, popover_origin, 2);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y - HOVER_POPOVER_GAP);
                    draw_occluder(size.width, origin, cx);
                }

                current_y = popover_origin.y - HOVER_POPOVER_GAP;
            }
        } else {
            // There is not enough space above. Render popovers below the hovered point
            let mut current_y = hovered_point.y + line_height;
            for (position, popover) in measured_hover_popovers.into_iter().with_position() {
                let size = popover.size;
                let popover_origin = point(hovered_point.x + popover.horizontal_offset, current_y);

                cx.defer_draw(popover.element, popover_origin, 2);
                if position != itertools::Position::Last {
                    let origin = point(popover_origin.x, popover_origin.y + size.height);
                    draw_occluder(size.width, origin, cx);
                }

                current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
            }
        }
    }

    fn paint_background(&self, layout: &EditorLayout, cx: &mut WindowContext) {
        cx.paint_layer(layout.hitbox.bounds, |cx| {
            let scroll_top = layout.position_map.snapshot.scroll_position().y;
            let gutter_bg = cx.theme().colors().editor_gutter_background;
            cx.paint_quad(fill(layout.gutter_hitbox.bounds, gutter_bg));
            cx.paint_quad(fill(layout.text_hitbox.bounds, self.style.background));

            if let EditorMode::Full = layout.mode {
                let mut active_rows = layout.active_rows.iter().peekable();
                while let Some((start_row, contains_non_empty_selection)) = active_rows.next() {
                    let mut end_row = start_row.0;
                    while active_rows
                        .peek()
                        .map_or(false, |(active_row, has_selection)| {
                            active_row.0 == end_row + 1
                                && *has_selection == contains_non_empty_selection
                        })
                    {
                        active_rows.next().unwrap();
                        end_row += 1;
                    }

                    if !contains_non_empty_selection {
                        let highlight_h_range =
                            match layout.position_map.snapshot.current_line_highlight {
                                CurrentLineHighlight::Gutter => Some(Range {
                                    start: layout.hitbox.left(),
                                    end: layout.gutter_hitbox.right(),
                                }),
                                CurrentLineHighlight::Line => Some(Range {
                                    start: layout.text_hitbox.bounds.left(),
                                    end: layout.text_hitbox.bounds.right(),
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
                            cx.paint_quad(fill(bounds, active_line_bg));
                        }
                    }
                }

                let mut paint_highlight =
                    |highlight_row_start: DisplayRow, highlight_row_end: DisplayRow, color| {
                        let origin = point(
                            layout.hitbox.origin.x,
                            layout.hitbox.origin.y
                                + (highlight_row_start.as_f32() - scroll_top)
                                    * layout.position_map.line_height,
                        );
                        let size = size(
                            layout.hitbox.size.width,
                            layout.position_map.line_height
                                * highlight_row_end.next_row().minus(highlight_row_start) as f32,
                        );
                        cx.paint_quad(fill(Bounds { origin, size }, color));
                    };

                let mut current_paint: Option<(Hsla, Range<DisplayRow>)> = None;
                for (&new_row, &new_color) in &layout.highlighted_rows {
                    match &mut current_paint {
                        Some((current_color, current_range)) => {
                            let current_color = *current_color;
                            let new_range_started = current_color != new_color
                                || current_range.end.next_row() != new_row;
                            if new_range_started {
                                paint_highlight(
                                    current_range.start,
                                    current_range.end,
                                    current_color,
                                );
                                current_paint = Some((new_color, new_row..new_row));
                                continue;
                            } else {
                                current_range.end = current_range.end.next_row();
                            }
                        }
                        None => current_paint = Some((new_color, new_row..new_row)),
                    };
                }
                if let Some((color, range)) = current_paint {
                    paint_highlight(range.start, range.end, color);
                }

                let scroll_left =
                    layout.position_map.snapshot.scroll_position().x * layout.position_map.em_width;

                for (wrap_position, active) in layout.wrap_guides.iter() {
                    let x = (layout.text_hitbox.origin.x
                        + *wrap_position
                        + layout.position_map.em_width / 2.)
                        - scroll_left;

                    let show_scrollbars = layout
                        .scrollbar_layout
                        .as_ref()
                        .map_or(false, |scrollbar| scrollbar.visible);
                    if x < layout.text_hitbox.origin.x
                        || (show_scrollbars && x > self.scrollbar_left(&layout.hitbox.bounds))
                    {
                        continue;
                    }

                    let color = if *active {
                        cx.theme().colors().editor_active_wrap_guide
                    } else {
                        cx.theme().colors().editor_wrap_guide
                    };
                    cx.paint_quad(fill(
                        Bounds {
                            origin: point(x, layout.text_hitbox.origin.y),
                            size: size(px(1.), layout.text_hitbox.size.height),
                        },
                        color,
                    ));
                }
            }
        })
    }

    fn paint_indent_guides(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        let Some(indent_guides) = &layout.indent_guides else {
            return;
        };

        let settings = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .settings_at(0, cx)
            .indent_guides;

        let faded_color = |color: Hsla, alpha: f32| {
            let mut faded = color;
            faded.a = alpha;
            faded
        };

        for indent_guide in indent_guides {
            let indent_accent_colors = cx.theme().accents().color_for_index(indent_guide.depth);

            // TODO fixed for now, expose them through themes later
            const INDENT_AWARE_ALPHA: f32 = 0.2;
            const INDENT_AWARE_ACTIVE_ALPHA: f32 = 0.4;
            const INDENT_AWARE_BACKGROUND_ALPHA: f32 = 0.1;
            const INDENT_AWARE_BACKGROUND_ACTIVE_ALPHA: f32 = 0.2;

            let line_color = match (&settings.coloring, indent_guide.active) {
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

            let background_color = match (&settings.background_coloring, indent_guide.active) {
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

            let requested_line_width = settings.line_width.clamp(1, 10);
            let mut line_indicator_width = 0.;
            if let Some(color) = line_color {
                cx.paint_quad(fill(
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
                cx.paint_quad(fill(
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

    fn paint_gutter(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        cx.set_cursor_style(CursorStyle::Arrow, &layout.gutter_hitbox);
        for (_, hunk_hitbox) in &layout.display_hunks {
            if let Some(hunk_hitbox) = hunk_hitbox {
                cx.set_cursor_style(CursorStyle::PointingHand, hunk_hitbox);
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
            Self::paint_diff_hunks(layout.gutter_hitbox.bounds, layout, cx)
        }

        if layout.blamed_display_rows.is_some() {
            self.paint_blamed_display_rows(layout, cx);
        }

        for (ix, line) in layout.line_numbers.iter().enumerate() {
            if let Some(line) = line {
                let line_origin = layout.gutter_hitbox.origin
                    + point(
                        layout.gutter_hitbox.size.width
                            - line.width
                            - layout.gutter_dimensions.right_padding,
                        ix as f32 * line_height - (scroll_top % line_height),
                    );

                line.paint(line_origin, line_height, cx).log_err();
            }
        }

        cx.paint_layer(layout.gutter_hitbox.bounds, |cx| {
            cx.with_element_namespace("gutter_fold_toggles", |cx| {
                for fold_indicator in layout.gutter_fold_toggles.iter_mut().flatten() {
                    fold_indicator.paint(cx);
                }
            });

            for test_indicators in layout.test_indicators.iter_mut() {
                test_indicators.paint(cx);
            }

            if let Some(indicator) = layout.code_actions_indicator.as_mut() {
                indicator.paint(cx);
            }
        });
    }

    fn paint_diff_hunks(
        gutter_bounds: Bounds<Pixels>,
        layout: &EditorLayout,
        cx: &mut WindowContext,
    ) {
        if layout.display_hunks.is_empty() {
            return;
        }

        let line_height = layout.position_map.line_height;
        cx.paint_layer(layout.gutter_hitbox.bounds, |cx| {
            for (hunk, hitbox) in &layout.display_hunks {
                let hunk_to_paint = match hunk {
                    DisplayDiffHunk::Folded { .. } => {
                        let hunk_bounds = Self::diff_hunk_bounds(
                            &layout.position_map.snapshot,
                            line_height,
                            gutter_bounds,
                            &hunk,
                        );
                        Some((
                            hunk_bounds,
                            cx.theme().status().modified,
                            Corners::all(1. * line_height),
                        ))
                    }
                    DisplayDiffHunk::Unfolded { status, .. } => {
                        hitbox.as_ref().map(|hunk_hitbox| match status {
                            DiffHunkStatus::Added => (
                                hunk_hitbox.bounds,
                                cx.theme().status().created,
                                Corners::all(0.05 * line_height),
                            ),
                            DiffHunkStatus::Modified => (
                                hunk_hitbox.bounds,
                                cx.theme().status().modified,
                                Corners::all(0.05 * line_height),
                            ),
                            DiffHunkStatus::Removed => (
                                Bounds::new(
                                    point(
                                        hunk_hitbox.origin.x - hunk_hitbox.size.width,
                                        hunk_hitbox.origin.y,
                                    ),
                                    size(hunk_hitbox.size.width * px(2.), hunk_hitbox.size.height),
                                ),
                                cx.theme().status().deleted,
                                Corners::all(1. * line_height),
                            ),
                        })
                    }
                };

                if let Some((hunk_bounds, background_color, corner_radii)) = hunk_to_paint {
                    cx.paint_quad(quad(
                        hunk_bounds,
                        corner_radii,
                        background_color,
                        Edges::default(),
                        transparent_black(),
                    ));
                }
            }
        });
    }

    fn diff_hunk_bounds(
        snapshot: &EditorSnapshot,
        line_height: Pixels,
        bounds: Bounds<Pixels>,
        hunk: &DisplayDiffHunk,
    ) -> Bounds<Pixels> {
        let scroll_position = snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        match hunk {
            DisplayDiffHunk::Folded { display_row, .. } => {
                let start_y = display_row.as_f32() * line_height - scroll_top;
                let end_y = start_y + line_height;

                let width = 0.275 * line_height;
                let highlight_origin = bounds.origin + point(px(0.), start_y);
                let highlight_size = size(width, end_y - start_y);
                Bounds::new(highlight_origin, highlight_size)
            }
            DisplayDiffHunk::Unfolded {
                display_row_range,
                status,
                ..
            } => match status {
                DiffHunkStatus::Added | DiffHunkStatus::Modified => {
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
                            if matches!(block, TransformBlock::ExcerptHeader { .. }) {
                                Some(start_row)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(end_row);

                    let start_y = start_row.as_f32() * line_height - scroll_top;
                    let end_y = end_row_in_current_excerpt.as_f32() * line_height - scroll_top;

                    let width = 0.275 * line_height;
                    let highlight_origin = bounds.origin + point(px(0.), start_y);
                    let highlight_size = size(width, end_y - start_y);
                    Bounds::new(highlight_origin, highlight_size)
                }
                DiffHunkStatus::Removed => {
                    let row = display_row_range.start;

                    let offset = line_height / 2.;
                    let start_y = row.as_f32() * line_height - offset - scroll_top;
                    let end_y = start_y + line_height;

                    let width = 0.35 * line_height;
                    let highlight_origin = bounds.origin + point(px(0.), start_y);
                    let highlight_size = size(width, end_y - start_y);
                    Bounds::new(highlight_origin, highlight_size)
                }
            },
        }
    }

    fn paint_blamed_display_rows(&self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        let Some(blamed_display_rows) = layout.blamed_display_rows.take() else {
            return;
        };

        cx.paint_layer(layout.gutter_hitbox.bounds, |cx| {
            for mut blame_element in blamed_display_rows.into_iter() {
                blame_element.paint(cx);
            }
        })
    }

    fn paint_text(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        cx.with_content_mask(
            Some(ContentMask {
                bounds: layout.text_hitbox.bounds,
            }),
            |cx| {
                let cursor_style = if self
                    .editor
                    .read(cx)
                    .hovered_link_state
                    .as_ref()
                    .is_some_and(|hovered_link_state| !hovered_link_state.links.is_empty())
                {
                    CursorStyle::PointingHand
                } else {
                    CursorStyle::IBeam
                };
                cx.set_cursor_style(cursor_style, &layout.text_hitbox);

                let invisible_display_ranges = self.paint_highlights(layout, cx);
                self.paint_lines(&invisible_display_ranges, layout, cx);
                self.paint_redactions(layout, cx);
                self.paint_cursors(layout, cx);
                self.paint_inline_blame(layout, cx);
                cx.with_element_namespace("flap_trailers", |cx| {
                    for trailer in layout.flap_trailers.iter_mut().flatten() {
                        trailer.element.paint(cx);
                    }
                });
            },
        )
    }

    fn paint_highlights(
        &mut self,
        layout: &mut EditorLayout,
        cx: &mut WindowContext,
    ) -> SmallVec<[Range<DisplayPoint>; 32]> {
        cx.paint_layer(layout.text_hitbox.bounds, |cx| {
            let mut invisible_display_ranges = SmallVec::<[Range<DisplayPoint>; 32]>::new();
            let line_end_overshoot = 0.15 * layout.position_map.line_height;
            for (range, color) in &layout.highlighted_ranges {
                self.paint_highlighted_range(
                    range.clone(),
                    *color,
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    cx,
                );
            }

            let corner_radius = 0.15 * layout.position_map.line_height;

            for (player_color, selections) in &layout.selections {
                for selection in selections.into_iter() {
                    self.paint_highlighted_range(
                        selection.range.clone(),
                        player_color.selection,
                        corner_radius,
                        corner_radius * 2.,
                        layout,
                        cx,
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
        cx: &mut WindowContext,
    ) {
        let whitespace_setting = self
            .editor
            .read(cx)
            .buffer
            .read(cx)
            .settings_at(0, cx)
            .show_whitespaces;

        for (ix, line_with_invisibles) in layout.position_map.line_layouts.iter().enumerate() {
            let row = DisplayRow(layout.visible_display_row_range.start.0 + ix as u32);
            line_with_invisibles.draw(
                layout,
                row,
                layout.content_origin,
                whitespace_setting,
                invisible_display_ranges,
                cx,
            )
        }

        for line_element in &mut layout.line_elements {
            line_element.paint(cx);
        }
    }

    fn paint_redactions(&mut self, layout: &EditorLayout, cx: &mut WindowContext) {
        if layout.redacted_ranges.is_empty() {
            return;
        }

        let line_end_overshoot = layout.line_end_overshoot();

        // A softer than perfect black
        let redaction_color = gpui::rgb(0x0e1111);

        cx.paint_layer(layout.text_hitbox.bounds, |cx| {
            for range in layout.redacted_ranges.iter() {
                self.paint_highlighted_range(
                    range.clone(),
                    redaction_color.into(),
                    Pixels::ZERO,
                    line_end_overshoot,
                    layout,
                    cx,
                );
            }
        });
    }

    fn paint_cursors(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        for cursor in &mut layout.visible_cursors {
            cursor.paint(layout.content_origin, cx);
        }
    }

    fn paint_scrollbar(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        let Some(scrollbar_layout) = layout.scrollbar_layout.as_ref() else {
            return;
        };

        let thumb_bounds = scrollbar_layout.thumb_bounds();
        if scrollbar_layout.visible {
            cx.paint_layer(scrollbar_layout.hitbox.bounds, |cx| {
                cx.paint_quad(quad(
                    scrollbar_layout.hitbox.bounds,
                    Corners::default(),
                    cx.theme().colors().scrollbar_track_background,
                    Edges {
                        top: Pixels::ZERO,
                        right: Pixels::ZERO,
                        bottom: Pixels::ZERO,
                        left: ScrollbarLayout::BORDER_WIDTH,
                    },
                    cx.theme().colors().scrollbar_track_border,
                ));

                let fast_markers =
                    self.collect_fast_scrollbar_markers(layout, scrollbar_layout, cx);
                // Refresh slow scrollbar markers in the background. Below, we paint whatever markers have already been computed.
                self.refresh_slow_scrollbar_markers(layout, scrollbar_layout, cx);

                let markers = self.editor.read(cx).scrollbar_marker_state.markers.clone();
                for marker in markers.iter().chain(&fast_markers) {
                    let mut marker = marker.clone();
                    marker.bounds.origin += scrollbar_layout.hitbox.origin;
                    cx.paint_quad(marker);
                }

                cx.paint_quad(quad(
                    thumb_bounds,
                    Corners::default(),
                    cx.theme().colors().scrollbar_thumb_background,
                    Edges {
                        top: Pixels::ZERO,
                        right: Pixels::ZERO,
                        bottom: Pixels::ZERO,
                        left: ScrollbarLayout::BORDER_WIDTH,
                    },
                    cx.theme().colors().scrollbar_thumb_border,
                ));
            });
        }

        cx.set_cursor_style(CursorStyle::Arrow, &scrollbar_layout.hitbox);

        let row_height = scrollbar_layout.row_height;
        let row_range = scrollbar_layout.visible_row_range.clone();

        cx.on_mouse_event({
            let editor = self.editor.clone();
            let hitbox = scrollbar_layout.hitbox.clone();
            let mut mouse_position = cx.mouse_position();
            move |event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Capture {
                    return;
                }

                editor.update(cx, |editor, cx| {
                    if event.pressed_button == Some(MouseButton::Left)
                        && editor.scroll_manager.is_dragging_scrollbar()
                    {
                        let y = mouse_position.y;
                        let new_y = event.position.y;
                        if (hitbox.top()..hitbox.bottom()).contains(&y) {
                            let mut position = editor.scroll_position(cx);
                            position.y += (new_y - y) / row_height;
                            if position.y < 0.0 {
                                position.y = 0.0;
                            }
                            editor.set_scroll_position(position, cx);
                        }

                        cx.stop_propagation();
                    } else {
                        editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
                        if hitbox.is_hovered(cx) {
                            editor.scroll_manager.show_scrollbar(cx);
                        }
                    }
                    mouse_position = event.position;
                })
            }
        });

        if self.editor.read(cx).scroll_manager.is_dragging_scrollbar() {
            cx.on_mouse_event({
                let editor = self.editor.clone();
                move |_: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
                        cx.stop_propagation();
                    });
                }
            });
        } else {
            cx.on_mouse_event({
                let editor = self.editor.clone();
                let hitbox = scrollbar_layout.hitbox.clone();
                move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Capture || !hitbox.is_hovered(cx) {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        editor.scroll_manager.set_is_dragging_scrollbar(true, cx);

                        let y = event.position.y;
                        if y < thumb_bounds.top() || thumb_bounds.bottom() < y {
                            let center_row = ((y - hitbox.top()) / row_height).round() as u32;
                            let top_row = center_row
                                .saturating_sub((row_range.end - row_range.start) as u32 / 2);
                            let mut position = editor.scroll_position(cx);
                            position.y = top_row as f32;
                            editor.set_scroll_position(position, cx);
                        } else {
                            editor.scroll_manager.show_scrollbar(cx);
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
        cx: &mut WindowContext,
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
        cx: &mut WindowContext,
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
                Some(cx.spawn(|editor, mut cx| async move {
                    let scrollbar_size = scrollbar_layout.hitbox.size;
                    let scrollbar_markers = cx
                        .background_executor()
                        .spawn(async move {
                            let max_point = snapshot.display_snapshot.buffer_snapshot.max_point();
                            let mut marker_quads = Vec::new();
                            if scrollbar_settings.git_diff {
                                let marker_row_ranges = snapshot
                                    .buffer_snapshot
                                    .git_diff_hunks_in_range(
                                        MultiBufferRow::MIN..MultiBufferRow::MAX,
                                    )
                                    .map(|hunk| {
                                        let start_display_row =
                                            MultiBufferPoint::new(hunk.associated_range.start.0, 0)
                                                .to_display_point(&snapshot.display_snapshot)
                                                .row();
                                        let mut end_display_row =
                                            MultiBufferPoint::new(hunk.associated_range.end.0, 0)
                                                .to_display_point(&snapshot.display_snapshot)
                                                .row();
                                        if end_display_row != start_display_row {
                                            end_display_row.0 -= 1;
                                        }
                                        let color = match hunk_status(&hunk) {
                                            DiffHunkStatus::Added => theme.status().created,
                                            DiffHunkStatus::Modified => theme.status().modified,
                                            DiffHunkStatus::Removed => theme.status().deleted,
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
                                let is_symbol_occurrences = *background_highlight_id
                                    == TypeId::of::<DocumentHighlightRead>()
                                    || *background_highlight_id
                                        == TypeId::of::<DocumentHighlightWrite>();
                                if (is_search_highlights && scrollbar_settings.search_results)
                                    || (is_symbol_occurrences && scrollbar_settings.selected_symbol)
                                {
                                    let mut color = theme.status().info;
                                    if is_symbol_occurrences {
                                        color.fade_out(0.5);
                                    }
                                    let marker_row_ranges =
                                        background_ranges.into_iter().map(|range| {
                                            let display_start = range
                                                .start
                                                .to_display_point(&snapshot.display_snapshot);
                                            let display_end = range
                                                .end
                                                .to_display_point(&snapshot.display_snapshot);
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

                            if scrollbar_settings.diagnostics {
                                let diagnostics = snapshot
                                    .buffer_snapshot
                                    .diagnostics_in_range::<_, Point>(
                                        Point::zero()..max_point,
                                        false,
                                    )
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

                    editor.update(&mut cx, |editor, cx| {
                        editor.scrollbar_marker_state.markers = scrollbar_markers;
                        editor.scrollbar_marker_state.scrollbar_size = scrollbar_size;
                        editor.scrollbar_marker_state.pending_refresh = None;
                        cx.notify();
                    })?;

                    Ok(())
                }));
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_highlighted_range(
        &self,
        range: Range<DisplayPoint>,
        color: Hsla,
        corner_radius: Pixels,
        line_end_overshoot: Pixels,
        layout: &EditorLayout,
        cx: &mut WindowContext,
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

            highlighted_range.paint(layout.text_hitbox.bounds, cx);
        }
    }

    fn paint_inline_blame(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        if let Some(mut inline_blame) = layout.inline_blame.take() {
            cx.paint_layer(layout.text_hitbox.bounds, |cx| {
                inline_blame.paint(cx);
            })
        }
    }

    fn paint_blocks(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        for mut block in layout.blocks.drain(..) {
            block.element.paint(cx);
        }
    }

    fn paint_mouse_context_menu(&mut self, layout: &mut EditorLayout, cx: &mut WindowContext) {
        if let Some(mouse_context_menu) = layout.mouse_context_menu.as_mut() {
            mouse_context_menu.paint(cx);
        }
    }

    fn paint_scroll_wheel_listener(&mut self, layout: &EditorLayout, cx: &mut WindowContext) {
        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let hitbox = layout.hitbox.clone();
            let mut delta = ScrollDelta::default();

            // Set a minimum scroll_sensitivity of 0.01 to make sure the user doesn't
            // accidentally turn off their scrolling.
            let scroll_sensitivity = EditorSettings::get_global(cx).scroll_sensitivity.max(0.01);

            move |event: &ScrollWheelEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && hitbox.is_hovered(cx) {
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
                            if scroll_position == current_scroll_position {
                                return;
                            }
                        }
                        editor.scroll(scroll_position, axis, cx);
                        cx.stop_propagation();
                    });
                }
            }
        });
    }

    fn paint_mouse_listeners(
        &mut self,
        layout: &EditorLayout,
        hovered_hunk: Option<HunkToExpand>,
        cx: &mut WindowContext,
    ) {
        self.paint_scroll_wheel_listener(layout, cx);

        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let text_hitbox = layout.text_hitbox.clone();
            let gutter_hitbox = layout.gutter_hitbox.clone();

            move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    match event.button {
                        MouseButton::Left => editor.update(cx, |editor, cx| {
                            Self::mouse_left_down(
                                editor,
                                event,
                                hovered_hunk.as_ref(),
                                &position_map,
                                &text_hitbox,
                                &gutter_hitbox,
                                cx,
                            );
                        }),
                        MouseButton::Right => editor.update(cx, |editor, cx| {
                            Self::mouse_right_down(editor, event, &position_map, &text_hitbox, cx);
                        }),
                        MouseButton::Middle => editor.update(cx, |editor, cx| {
                            Self::mouse_middle_down(editor, event, &position_map, &text_hitbox, cx);
                        }),
                        _ => {}
                    };
                }
            }
        });

        cx.on_mouse_event({
            let editor = self.editor.clone();
            let position_map = layout.position_map.clone();
            let text_hitbox = layout.text_hitbox.clone();

            move |event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        Self::mouse_up(editor, event, &position_map, &text_hitbox, cx)
                    });
                }
            }
        });
        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let text_hitbox = layout.text_hitbox.clone();
            let gutter_hitbox = layout.gutter_hitbox.clone();

            move |event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        if event.pressed_button == Some(MouseButton::Left)
                            || event.pressed_button == Some(MouseButton::Middle)
                        {
                            Self::mouse_dragged(
                                editor,
                                event,
                                &position_map,
                                text_hitbox.bounds,
                                cx,
                            )
                        }

                        Self::mouse_moved(
                            editor,
                            event,
                            &position_map,
                            &text_hitbox,
                            &gutter_hitbox,
                            cx,
                        )
                    });
                }
            }
        });
    }

    fn scrollbar_left(&self, bounds: &Bounds<Pixels>) -> Pixels {
        bounds.upper_right().x - self.style.scrollbar_width
    }

    fn column_pixels(&self, column: usize, cx: &WindowContext) -> Pixels {
        let style = &self.style;
        let font_size = style.text.font_size.to_pixels(cx.rem_size());
        let layout = cx
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

    fn max_line_number_width(&self, snapshot: &EditorSnapshot, cx: &WindowContext) -> Pixels {
        let digit_count = snapshot
            .max_buffer_row()
            .next_row()
            .as_f32()
            .log10()
            .floor() as usize
            + 1;
        self.column_pixels(digit_count, cx)
    }
}

fn prepaint_gutter_button(
    button: IconButton,
    row: DisplayRow,
    line_height: Pixels,
    gutter_dimensions: &GutterDimensions,
    scroll_pixel_position: gpui::Point<Pixels>,
    gutter_hitbox: &Hitbox,
    cx: &mut WindowContext<'_>,
) -> AnyElement {
    let mut button = button.into_any_element();
    let available_space = size(
        AvailableSpace::MinContent,
        AvailableSpace::Definite(line_height),
    );
    let indicator_size = button.layout_as_root(available_space, cx);

    let blame_width = gutter_dimensions
        .git_blame_entries_width
        .unwrap_or(Pixels::ZERO);

    let mut x = blame_width;
    let available_width = gutter_dimensions.margin + gutter_dimensions.left_padding
        - indicator_size.width
        - blame_width;
    x += available_width / 2.;

    let mut y = row.as_f32() * line_height - scroll_pixel_position.y;
    y += (line_height - indicator_size.height) / 2.;

    button.prepaint_as_root(gutter_hitbox.origin + point(x, y), available_space, cx);
    button
}

fn render_inline_blame_entry(
    blame: &gpui::Model<GitBlame>,
    blame_entry: BlameEntry,
    style: &EditorStyle,
    workspace: Option<WeakView<Workspace>>,
    cx: &mut WindowContext<'_>,
) -> AnyElement {
    let relative_timestamp = blame_entry_relative_timestamp(&blame_entry, cx);

    let author = blame_entry.author.as_deref().unwrap_or_default();
    let text = format!("{}, {}", author, relative_timestamp);

    let details = blame.read(cx).details_for_entry(&blame_entry);

    let tooltip = cx.new_view(|_| BlameEntryTooltip::new(blame_entry, details, style, workspace));

    h_flex()
        .id("inline-blame")
        .w_full()
        .font_family(style.text.font().family)
        .text_color(cx.theme().status().hint)
        .line_height(style.text.line_height)
        .child(Icon::new(IconName::FileGit).color(Color::Hint))
        .child(text)
        .gap_2()
        .hoverable_tooltip(move |_| tooltip.clone().into())
        .into_any()
}

fn render_blame_entry(
    ix: usize,
    blame: &gpui::Model<GitBlame>,
    blame_entry: BlameEntry,
    style: &EditorStyle,
    last_used_color: &mut Option<(PlayerColor, Oid)>,
    editor: View<Editor>,
    cx: &mut WindowContext<'_>,
) -> AnyElement {
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

    let relative_timestamp = blame_entry_relative_timestamp(&blame_entry, cx);

    let short_commit_id = blame_entry.sha.display_short();

    let author_name = blame_entry.author.as_deref().unwrap_or("<no name>");
    let name = util::truncate_and_trailoff(author_name, 20);

    let details = blame.read(cx).details_for_entry(&blame_entry);

    let workspace = editor.read(cx).workspace.as_ref().map(|(w, _)| w.clone());

    let tooltip = cx.new_view(|_| {
        BlameEntryTooltip::new(blame_entry.clone(), details.clone(), style, workspace)
    });

    h_flex()
        .w_full()
        .font_family(style.text.font().family)
        .line_height(style.text.line_height)
        .id(("blame", ix))
        .children([
            div()
                .text_color(sha_color.cursor)
                .child(short_commit_id)
                .mr_2(),
            div()
                .w_full()
                .h_flex()
                .justify_between()
                .text_color(cx.theme().status().hint)
                .child(name)
                .child(relative_timestamp),
        ])
        .on_mouse_down(MouseButton::Right, {
            let blame_entry = blame_entry.clone();
            let details = details.clone();
            move |event, cx| {
                deploy_blame_entry_context_menu(
                    &blame_entry,
                    details.as_ref(),
                    editor.clone(),
                    event.position,
                    cx,
                );
            }
        })
        .hover(|style| style.bg(cx.theme().colors().element_hover))
        .when_some(
            details.and_then(|details| details.permalink),
            |this, url| {
                let url = url.clone();
                this.cursor_pointer().on_click(move |_, cx| {
                    cx.stop_propagation();
                    cx.open_url(url.as_str())
                })
            },
        )
        .hoverable_tooltip(move |_| tooltip.clone().into())
        .into_any()
}

fn deploy_blame_entry_context_menu(
    blame_entry: &BlameEntry,
    details: Option<&CommitDetails>,
    editor: View<Editor>,
    position: gpui::Point<Pixels>,
    cx: &mut WindowContext<'_>,
) {
    let context_menu = ContextMenu::build(cx, move |this, _| {
        let sha = format!("{}", blame_entry.sha);
        this.entry("Copy commit SHA", None, move |cx| {
            cx.write_to_clipboard(ClipboardItem::new(sha.clone()));
        })
        .when_some(
            details.and_then(|details| details.permalink.clone()),
            |this, url| this.entry("Open permalink", None, move |cx| cx.open_url(url.as_str())),
        )
    });

    editor.update(cx, move |editor, cx| {
        editor.mouse_context_menu = Some(MouseContextMenu::new(position, context_menu, cx));
        cx.notify();
    });
}

#[derive(Debug)]
pub(crate) struct LineWithInvisibles {
    fragments: SmallVec<[LineFragment; 1]>,
    invisibles: Vec<Invisible>,
    len: usize,
    width: Pixels,
    font_size: Pixels,
}

#[allow(clippy::large_enum_variant)]
enum LineFragment {
    Text(ShapedLine),
    Element {
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
        text_style: &TextStyle,
        max_line_len: usize,
        max_line_count: usize,
        line_number_layouts: &[Option<ShapedLine>],
        editor_mode: EditorMode,
        cx: &mut WindowContext,
    ) -> Vec<Self> {
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
        let font_size = text_style.font_size.to_pixels(cx.rem_size());

        let ellipsis = SharedString::from("⋯");

        for highlighted_chunk in chunks.chain([HighlightedChunk {
            text: "\n",
            style: None,
            is_tab: false,
            renderer: None,
        }]) {
            if let Some(renderer) = highlighted_chunk.renderer {
                if !line.is_empty() {
                    let shaped_line = cx
                        .text_system()
                        .shape_line(line.clone().into(), font_size, &styles)
                        .unwrap();
                    width += shaped_line.width;
                    len += shaped_line.len;
                    fragments.push(LineFragment::Text(shaped_line));
                    line.clear();
                    styles.clear();
                }

                let available_width = if renderer.constrain_width {
                    let chunk = if highlighted_chunk.text == ellipsis.as_ref() {
                        ellipsis.clone()
                    } else {
                        SharedString::from(Arc::from(highlighted_chunk.text))
                    };
                    let shaped_line = cx
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

                let mut element = (renderer.render)(cx);
                let line_height = text_style.line_height_in_pixels(cx.rem_size());
                let size = element.layout_as_root(
                    size(available_width, AvailableSpace::Definite(line_height)),
                    cx,
                );

                width += size.width;
                len += highlighted_chunk.text.len();
                fragments.push(LineFragment::Element {
                    element: Some(element),
                    size,
                    len: highlighted_chunk.text.len(),
                });
            } else {
                for (ix, mut line_chunk) in highlighted_chunk.text.split('\n').enumerate() {
                    if ix > 0 {
                        let shaped_line = cx
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

                        if editor_mode == EditorMode::Full {
                            // Line wrap pads its contents with fake whitespaces,
                            // avoid printing them
                            let inside_wrapped_string = line_number_layouts
                                .get(row)
                                .and_then(|layout| layout.as_ref())
                                .is_none();
                            if highlighted_chunk.is_tab {
                                if non_whitespace_added || !inside_wrapped_string {
                                    invisibles.push(Invisible::Tab {
                                        line_start_offset: line.len(),
                                    });
                                }
                            } else {
                                invisibles.extend(
                                    line_chunk
                                        .bytes()
                                        .enumerate()
                                        .filter(|(_, line_byte)| {
                                            let is_whitespace =
                                                (*line_byte as char).is_whitespace();
                                            non_whitespace_added |= !is_whitespace;
                                            is_whitespace
                                                && (non_whitespace_added || !inside_wrapped_string)
                                        })
                                        .map(|(whitespace_index, _)| Invisible::Whitespace {
                                            line_offset: line.len() + whitespace_index,
                                        }),
                                )
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
        cx: &mut WindowContext,
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
                    element.prepaint_at(element_origin, cx);
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
        cx: &mut WindowContext,
    ) {
        let line_height = layout.position_map.line_height;
        let line_y = line_height
            * (row.as_f32() - layout.position_map.scroll_pixel_position.y / line_height);

        let mut fragment_origin =
            content_origin + gpui::point(-layout.position_map.scroll_pixel_position.x, line_y);

        for fragment in &self.fragments {
            match fragment {
                LineFragment::Text(line) => {
                    line.paint(fragment_origin, line_height, cx).log_err();
                    fragment_origin.x += line.width;
                }
                LineFragment::Element { size, .. } => {
                    fragment_origin.x += size.width;
                }
            }
        }

        self.draw_invisibles(
            &selection_ranges,
            layout,
            content_origin,
            line_y,
            row,
            line_height,
            whitespace_setting,
            cx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_invisibles(
        &self,
        selection_ranges: &[Range<DisplayPoint>],
        layout: &EditorLayout,
        content_origin: gpui::Point<Pixels>,
        line_y: Pixels,
        row: DisplayRow,
        line_height: Pixels,
        whitespace_setting: ShowWhitespaceSetting,
        cx: &mut WindowContext,
    ) {
        let allowed_invisibles_regions = match whitespace_setting {
            ShowWhitespaceSetting::None => return,
            ShowWhitespaceSetting::Selection => Some(selection_ranges),
            ShowWhitespaceSetting::All => None,
        };

        for invisible in &self.invisibles {
            let (&token_offset, invisible_symbol) = match invisible {
                Invisible::Tab { line_start_offset } => (line_start_offset, &layout.tab_invisible),
                Invisible::Whitespace { line_offset } => (line_offset, &layout.space_invisible),
            };

            let x_offset = self.x_for_index(token_offset);
            let invisible_offset =
                (layout.position_map.em_width - invisible_symbol.width).max(Pixels::ZERO) / 2.0;
            let origin = content_origin
                + gpui::point(
                    x_offset + invisible_offset - layout.position_map.scroll_pixel_position.x,
                    line_y,
                );

            if let Some(allowed_regions) = allowed_invisibles_regions {
                let invisible_point = DisplayPoint::new(row, token_offset as u32);
                if !allowed_regions
                    .iter()
                    .any(|region| region.start <= invisible_point && invisible_point < region.end)
                {
                    continue;
                }
            }
            invisible_symbol.paint(origin, line_height, cx).log_err();
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
    Tab { line_start_offset: usize },
    Whitespace { line_offset: usize },
}

impl EditorElement {
    /// Returns the rem size to use when rendering the [`EditorElement`].
    ///
    /// This allows UI elements to scale based on the `buffer_font_size`.
    fn rem_size(&self, cx: &WindowContext) -> Option<Pixels> {
        match self.editor.read(cx).mode {
            EditorMode::Full => {
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
            EditorMode::SingleLine | EditorMode::AutoHeight { .. } => None,
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
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, ()) {
        let rem_size = self.rem_size(cx);
        cx.with_rem_size(rem_size, |cx| {
            self.editor.update(cx, |editor, cx| {
                editor.set_style(self.style.clone(), cx);

                let layout_id = match editor.mode {
                    EditorMode::SingleLine => {
                        let rem_size = cx.rem_size();
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        style.size.height = self.style.text.line_height_in_pixels(rem_size).into();
                        cx.request_layout(style, None)
                    }
                    EditorMode::AutoHeight { max_lines } => {
                        let editor_handle = cx.view().clone();
                        let max_line_number_width =
                            self.max_line_number_width(&editor.snapshot(cx), cx);
                        cx.request_measured_layout(
                            Style::default(),
                            move |known_dimensions, available_space, cx| {
                                editor_handle
                                    .update(cx, |editor, cx| {
                                        compute_auto_height_layout(
                                            editor,
                                            max_lines,
                                            max_line_number_width,
                                            known_dimensions,
                                            available_space.width,
                                            cx,
                                        )
                                    })
                                    .unwrap_or_default()
                            },
                        )
                    }
                    EditorMode::Full => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        style.size.height = relative(1.).into();
                        cx.request_layout(style, None)
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
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };
        cx.set_view_id(self.editor.entity_id());

        let rem_size = self.rem_size(cx);
        cx.with_rem_size(rem_size, |cx| {
            cx.with_text_style(Some(text_style), |cx| {
                cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                    let mut snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));
                    let style = self.style.clone();

                    let font_id = cx.text_system().resolve_font(&style.text.font());
                    let font_size = style.text.font_size.to_pixels(cx.rem_size());
                    let line_height = style.text.line_height_in_pixels(cx.rem_size());
                    let em_width = cx
                        .text_system()
                        .typographic_bounds(font_id, font_size, 'm')
                        .unwrap()
                        .size
                        .width;
                    let em_advance = cx
                        .text_system()
                        .advance(font_id, font_size, 'm')
                        .unwrap()
                        .width;

                    let gutter_dimensions = snapshot.gutter_dimensions(
                        font_id,
                        font_size,
                        em_width,
                        self.max_line_number_width(&snapshot, cx),
                        cx,
                    );
                    let text_width = bounds.size.width - gutter_dimensions.width;

                    let right_margin = if snapshot.mode == EditorMode::Full {
                        EditorElement::SCROLLBAR_WIDTH
                    } else {
                        px(0.)
                    };
                    let overscroll = size(em_width + right_margin, px(0.));

                    snapshot = self.editor.update(cx, |editor, cx| {
                        editor.last_bounds = Some(bounds);
                        editor.gutter_dimensions = gutter_dimensions;
                        editor.set_visible_line_count(bounds.size.height / line_height, cx);

                        let editor_width =
                            text_width - gutter_dimensions.margin - overscroll.width - em_width;
                        let wrap_width = match editor.soft_wrap_mode(cx) {
                            SoftWrap::None => None,
                            SoftWrap::PreferLine => Some((MAX_LINE_LEN / 2) as f32 * em_advance),
                            SoftWrap::EditorWidth => Some(editor_width),
                            SoftWrap::Column(column) => {
                                Some(editor_width.min(column as f32 * em_advance))
                            }
                        };

                        if editor.set_wrap_width(wrap_width, cx) {
                            editor.snapshot(cx)
                        } else {
                            snapshot
                        }
                    });

                    let wrap_guides = self
                        .editor
                        .read(cx)
                        .wrap_guides(cx)
                        .iter()
                        .map(|(guide, active)| (self.column_pixels(*guide, cx), *active))
                        .collect::<SmallVec<[_; 2]>>();

                    let hitbox = cx.insert_hitbox(bounds, false);
                    let gutter_hitbox = cx.insert_hitbox(
                        Bounds {
                            origin: bounds.origin,
                            size: size(gutter_dimensions.width, bounds.size.height),
                        },
                        false,
                    );
                    let text_hitbox = cx.insert_hitbox(
                        Bounds {
                            origin: gutter_hitbox.upper_right(),
                            size: size(text_width, bounds.size.height),
                        },
                        false,
                    );
                    // Offset the content_bounds from the text_bounds by the gutter margin (which
                    // is roughly half a character wide) to make hit testing work more like how we want.
                    let content_origin =
                        text_hitbox.origin + point(gutter_dimensions.margin, Pixels::ZERO);

                    let mut autoscroll_containing_element = false;
                    let mut autoscroll_horizontally = false;
                    self.editor.update(cx, |editor, cx| {
                        autoscroll_containing_element =
                            editor.autoscroll_requested() || editor.has_pending_selection();
                        autoscroll_horizontally =
                            editor.autoscroll_vertically(bounds, line_height, cx);
                        snapshot = editor.snapshot(cx);
                    });

                    let mut scroll_position = snapshot.scroll_position();
                    // The scroll position is a fractional point, the whole number of which represents
                    // the top of the window in terms of display rows.
                    let start_row = DisplayRow(scroll_position.y as u32);
                    let height_in_lines = bounds.size.height / line_height;
                    let max_row = snapshot.max_point().row();
                    let end_row = cmp::min(
                        (scroll_position.y + height_in_lines).ceil() as u32,
                        max_row.next_row().0,
                    );
                    let end_row = DisplayRow(end_row);

                    let buffer_rows = snapshot
                        .buffer_rows(start_row)
                        .take((start_row..end_row).len())
                        .collect::<Vec<_>>();

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

                    let highlighted_rows = self
                        .editor
                        .update(cx, |editor, cx| editor.highlighted_display_rows(cx));
                    let highlighted_ranges = self.editor.read(cx).background_highlights_in_range(
                        start_anchor..end_anchor,
                        &snapshot.display_snapshot,
                        cx.theme().colors(),
                    );

                    let redacted_ranges = self.editor.read(cx).redacted_ranges(
                        start_anchor..end_anchor,
                        &snapshot.display_snapshot,
                        cx,
                    );

                    let (selections, active_rows, newest_selection_head) = self.layout_selections(
                        start_anchor,
                        end_anchor,
                        &snapshot,
                        start_row,
                        end_row,
                        cx,
                    );

                    let line_numbers = self.layout_line_numbers(
                        start_row..end_row,
                        buffer_rows.iter().copied(),
                        &active_rows,
                        newest_selection_head,
                        &snapshot,
                        cx,
                    );

                    let mut gutter_fold_toggles =
                        cx.with_element_namespace("gutter_fold_toggles", |cx| {
                            self.layout_gutter_fold_toggles(
                                start_row..end_row,
                                buffer_rows.iter().copied(),
                                &active_rows,
                                &snapshot,
                                cx,
                            )
                        });
                    let flap_trailers = cx.with_element_namespace("flap_trailers", |cx| {
                        self.layout_flap_trailers(buffer_rows.iter().copied(), &snapshot, cx)
                    });

                    let display_hunks = self.layout_git_gutters(
                        line_height,
                        &gutter_hitbox,
                        start_row..end_row,
                        &snapshot,
                        cx,
                    );

                    let mut max_visible_line_width = Pixels::ZERO;
                    let mut line_layouts =
                        self.layout_lines(start_row..end_row, &line_numbers, &snapshot, cx);
                    for line_with_invisibles in &line_layouts {
                        if line_with_invisibles.width > max_visible_line_width {
                            max_visible_line_width = line_with_invisibles.width;
                        }
                    }

                    let longest_line_width =
                        layout_line(snapshot.longest_row(), &snapshot, &style, cx).width;
                    let mut scroll_width =
                        longest_line_width.max(max_visible_line_width) + overscroll.width;

                    let mut blocks = cx.with_element_namespace("blocks", |cx| {
                        self.build_blocks(
                            start_row..end_row,
                            &snapshot,
                            &hitbox,
                            &text_hitbox,
                            &mut scroll_width,
                            &gutter_dimensions,
                            em_width,
                            gutter_dimensions.width + gutter_dimensions.margin,
                            line_height,
                            &line_layouts,
                            cx,
                        )
                    });

                    let scroll_pixel_position = point(
                        scroll_position.x * em_width,
                        scroll_position.y * line_height,
                    );

                    let start_buffer_row =
                        MultiBufferRow(start_anchor.to_point(&snapshot.buffer_snapshot).row);
                    let end_buffer_row =
                        MultiBufferRow(end_anchor.to_point(&snapshot.buffer_snapshot).row);

                    let indent_guides = self.layout_indent_guides(
                        content_origin,
                        text_hitbox.origin,
                        start_buffer_row..end_buffer_row,
                        scroll_pixel_position,
                        line_height,
                        &snapshot,
                        cx,
                    );

                    let flap_trailers = cx.with_element_namespace("flap_trailers", |cx| {
                        self.prepaint_flap_trailers(
                            flap_trailers,
                            &line_layouts,
                            line_height,
                            content_origin,
                            scroll_pixel_position,
                            em_width,
                            cx,
                        )
                    });

                    let mut inline_blame = None;
                    if let Some(newest_selection_head) = newest_selection_head {
                        let display_row = newest_selection_head.row();
                        if (start_row..end_row).contains(&display_row) {
                            let line_ix = display_row.minus(start_row) as usize;
                            let line_layout = &line_layouts[line_ix];
                            let flap_trailer_layout = flap_trailers[line_ix].as_ref();
                            inline_blame = self.layout_inline_blame(
                                display_row,
                                &snapshot.display_snapshot,
                                line_layout,
                                flap_trailer_layout,
                                em_width,
                                content_origin,
                                scroll_pixel_position,
                                line_height,
                                cx,
                            );
                        }
                    }

                    let blamed_display_rows = self.layout_blame_entries(
                        buffer_rows.into_iter(),
                        em_width,
                        scroll_position,
                        line_height,
                        &gutter_hitbox,
                        gutter_dimensions.git_blame_entries_width,
                        cx,
                    );

                    let scroll_max = point(
                        ((scroll_width - text_hitbox.size.width) / em_width).max(0.0),
                        max_row.as_f32(),
                    );

                    self.editor.update(cx, |editor, cx| {
                        let clamped = editor.scroll_manager.clamp_scroll_left(scroll_max.x);

                        let autoscrolled = if autoscroll_horizontally {
                            editor.autoscroll_horizontally(
                                start_row,
                                text_hitbox.size.width,
                                scroll_width,
                                em_width,
                                &line_layouts,
                                cx,
                            )
                        } else {
                            false
                        };

                        if clamped || autoscrolled {
                            snapshot = editor.snapshot(cx);
                            scroll_position = snapshot.scroll_position();
                        }
                    });

                    let line_elements = self.prepaint_lines(
                        start_row,
                        &mut line_layouts,
                        line_height,
                        scroll_pixel_position,
                        content_origin,
                        cx,
                    );

                    cx.with_element_namespace("blocks", |cx| {
                        self.layout_blocks(
                            &mut blocks,
                            &hitbox,
                            line_height,
                            scroll_pixel_position,
                            cx,
                        );
                    });

                    let cursors = self.collect_cursors(&snapshot, cx);
                    let visible_row_range = start_row..end_row;
                    let non_visible_cursors = cursors
                        .iter()
                        .any(move |c| !visible_row_range.contains(&c.0.row()));

                    let visible_cursors = self.layout_visible_cursors(
                        &snapshot,
                        &selections,
                        start_row..end_row,
                        &line_layouts,
                        &text_hitbox,
                        content_origin,
                        scroll_position,
                        scroll_pixel_position,
                        line_height,
                        em_width,
                        autoscroll_containing_element,
                        cx,
                    );

                    let scrollbar_layout = self.layout_scrollbar(
                        &snapshot,
                        bounds,
                        scroll_position,
                        height_in_lines,
                        non_visible_cursors,
                        cx,
                    );

                    let gutter_settings = EditorSettings::get_global(cx).gutter;

                    let mut _context_menu_visible = false;
                    let mut code_actions_indicator = None;
                    if let Some(newest_selection_head) = newest_selection_head {
                        if (start_row..end_row).contains(&newest_selection_head.row()) {
                            _context_menu_visible = self.layout_context_menu(
                                line_height,
                                &hitbox,
                                &text_hitbox,
                                content_origin,
                                start_row,
                                scroll_pixel_position,
                                &line_layouts,
                                newest_selection_head,
                                gutter_dimensions.width - gutter_dimensions.left_padding,
                                cx,
                            );

                            let show_code_actions = snapshot
                                .show_code_actions
                                .unwrap_or_else(|| gutter_settings.code_actions);
                            if show_code_actions {
                                let newest_selection_point =
                                    newest_selection_head.to_point(&snapshot.display_snapshot);
                                let buffer = snapshot.buffer_snapshot.buffer_line_for_row(
                                    MultiBufferRow(newest_selection_point.row),
                                );
                                if let Some((buffer, range)) = buffer {
                                    let buffer_id = buffer.remote_id();
                                    let row = range.start.row;
                                    let has_test_indicator =
                                        self.editor.read(cx).tasks.contains_key(&(buffer_id, row));

                                    if !has_test_indicator {
                                        code_actions_indicator = self
                                            .layout_code_actions_indicator(
                                                line_height,
                                                newest_selection_head,
                                                scroll_pixel_position,
                                                &gutter_dimensions,
                                                &gutter_hitbox,
                                                cx,
                                            );
                                    }
                                }
                            }
                        }
                    }

                    let test_indicators = self.layout_run_indicators(
                        line_height,
                        scroll_pixel_position,
                        &gutter_dimensions,
                        &gutter_hitbox,
                        &snapshot,
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
                            cx,
                        );
                    }

                    let mouse_context_menu = self.layout_mouse_context_menu(cx);

                    cx.with_element_namespace("gutter_fold_toggles", |cx| {
                        self.prepaint_gutter_fold_toggles(
                            &mut gutter_fold_toggles,
                            line_height,
                            &gutter_dimensions,
                            gutter_settings,
                            scroll_pixel_position,
                            &gutter_hitbox,
                            cx,
                        )
                    });

                    let invisible_symbol_font_size = font_size / 2.;
                    let tab_invisible = cx
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
                    let space_invisible = cx
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

                    EditorLayout {
                        mode: snapshot.mode,
                        position_map: Arc::new(PositionMap {
                            size: bounds.size,
                            scroll_pixel_position,
                            scroll_max,
                            line_layouts,
                            line_height,
                            em_width,
                            em_advance,
                            snapshot,
                        }),
                        visible_display_row_range: start_row..end_row,
                        wrap_guides,
                        indent_guides,
                        hitbox,
                        text_hitbox,
                        gutter_hitbox,
                        gutter_dimensions,
                        content_origin,
                        scrollbar_layout,
                        active_rows,
                        highlighted_rows,
                        highlighted_ranges,
                        redacted_ranges,
                        line_elements,
                        line_numbers,
                        display_hunks,
                        blamed_display_rows,
                        inline_blame,
                        blocks,
                        cursors,
                        visible_cursors,
                        selections,
                        mouse_context_menu,
                        test_indicators,
                        code_actions_indicator,
                        gutter_fold_toggles,
                        flap_trailers,
                        tab_invisible,
                        space_invisible,
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
        cx: &mut WindowContext,
    ) {
        let focus_handle = self.editor.focus_handle(cx);
        let key_context = self.editor.read(cx).key_context(cx);
        cx.set_focus_handle(&focus_handle);
        cx.set_key_context(key_context);
        cx.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
        );
        self.register_actions(cx);
        self.register_key_listeners(cx, layout);

        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };
        let mouse_position = cx.mouse_position();
        let hovered_hunk = layout
            .display_hunks
            .iter()
            .find_map(|(hunk, hunk_hitbox)| match hunk {
                DisplayDiffHunk::Folded { .. } => None,
                DisplayDiffHunk::Unfolded {
                    diff_base_byte_range,
                    multi_buffer_range,
                    status,
                    ..
                } => {
                    if hunk_hitbox
                        .as_ref()
                        .map(|hitbox| hitbox.contains(&mouse_position))
                        .unwrap_or(false)
                    {
                        Some(HunkToExpand {
                            status: *status,
                            multi_buffer_range: multi_buffer_range.clone(),
                            diff_base_byte_range: diff_base_byte_range.clone(),
                        })
                    } else {
                        None
                    }
                }
            });
        let rem_size = self.rem_size(cx);
        cx.with_rem_size(rem_size, |cx| {
            cx.with_text_style(Some(text_style), |cx| {
                cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                    self.paint_mouse_listeners(layout, hovered_hunk, cx);
                    self.paint_background(layout, cx);
                    self.paint_indent_guides(layout, cx);
                    if layout.gutter_hitbox.size.width > Pixels::ZERO {
                        self.paint_gutter(layout, cx)
                    }

                    self.paint_text(layout, cx);

                    if !layout.blocks.is_empty() {
                        cx.with_element_namespace("blocks", |cx| {
                            self.paint_blocks(layout, cx);
                        });
                    }

                    self.paint_scrollbar(layout, cx);
                    self.paint_mouse_context_menu(layout, cx);
                });
            })
        })
    }
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct EditorLayout {
    position_map: Arc<PositionMap>,
    hitbox: Hitbox,
    text_hitbox: Hitbox,
    gutter_hitbox: Hitbox,
    gutter_dimensions: GutterDimensions,
    content_origin: gpui::Point<Pixels>,
    scrollbar_layout: Option<ScrollbarLayout>,
    mode: EditorMode,
    wrap_guides: SmallVec<[(Pixels, bool); 2]>,
    indent_guides: Option<Vec<IndentGuideLayout>>,
    visible_display_row_range: Range<DisplayRow>,
    active_rows: BTreeMap<DisplayRow, bool>,
    highlighted_rows: BTreeMap<DisplayRow, Hsla>,
    line_elements: SmallVec<[AnyElement; 1]>,
    line_numbers: Vec<Option<ShapedLine>>,
    display_hunks: Vec<(DisplayDiffHunk, Option<Hitbox>)>,
    blamed_display_rows: Option<Vec<AnyElement>>,
    inline_blame: Option<AnyElement>,
    blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    redacted_ranges: Vec<Range<DisplayPoint>>,
    cursors: Vec<(DisplayPoint, Hsla)>,
    visible_cursors: Vec<CursorLayout>,
    selections: Vec<(PlayerColor, Vec<SelectionLayout>)>,
    code_actions_indicator: Option<AnyElement>,
    test_indicators: Vec<AnyElement>,
    gutter_fold_toggles: Vec<Option<AnyElement>>,
    flap_trailers: Vec<Option<FlapTrailerLayout>>,
    mouse_context_menu: Option<AnyElement>,
    tab_invisible: ShapedLine,
    space_invisible: ShapedLine,
}

impl EditorLayout {
    fn line_end_overshoot(&self) -> Pixels {
        0.15 * self.position_map.line_height
    }
}

struct ColoredRange<T> {
    start: T,
    end: T,
    color: Hsla,
}

#[derive(Clone)]
struct ScrollbarLayout {
    hitbox: Hitbox,
    visible_row_range: Range<f32>,
    visible: bool,
    row_height: Pixels,
    thumb_height: Pixels,
}

impl ScrollbarLayout {
    const BORDER_WIDTH: Pixels = px(1.0);
    const LINE_MARKER_HEIGHT: Pixels = px(2.0);
    const MIN_MARKER_HEIGHT: Pixels = px(5.0);
    const MIN_THUMB_HEIGHT: Pixels = px(20.0);

    fn thumb_bounds(&self) -> Bounds<Pixels> {
        let thumb_top = self.y_for_row(self.visible_row_range.start);
        let thumb_bottom = thumb_top + self.thumb_height;
        Bounds::from_corners(
            point(self.hitbox.left(), thumb_top),
            point(self.hitbox.right(), thumb_bottom),
        )
    }

    fn y_for_row(&self, row: f32) -> Pixels {
        self.hitbox.top() + row * self.row_height
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

        let row_to_y = |row: DisplayRow| row.as_f32() * self.row_height;
        let mut pixel_ranges = row_ranges
            .into_iter()
            .map(|range| {
                let start_y = row_to_y(range.start);
                let end_y = row_to_y(range.end)
                    + self.row_height.max(height_limit.min).min(height_limit.max);
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
            ));
        }

        quads
    }
}

struct FlapTrailerLayout {
    element: AnyElement,
    bounds: Bounds<Pixels>,
}

struct PositionMap {
    size: Size<Pixels>,
    line_height: Pixels,
    scroll_pixel_position: gpui::Point<Pixels>,
    scroll_max: gpui::Point<f32>,
    em_width: Pixels,
    em_advance: Pixels,
    line_layouts: Vec<LineWithInvisibles>,
    snapshot: EditorSnapshot,
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
    fn point_for_position(
        &self,
        text_bounds: Bounds<Pixels>,
        position: gpui::Point<Pixels>,
    ) -> PointForPosition {
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
    row: DisplayRow,
    element: AnyElement,
    available_space: Size<AvailableSpace>,
    style: BlockStyle,
}

fn layout_line(
    row: DisplayRow,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    cx: &mut WindowContext,
) -> LineWithInvisibles {
    let chunks = snapshot.highlighted_chunks(row..row + DisplayRow(1), true, style);
    LineWithInvisibles::from_chunks(chunks, &style.text, MAX_LINE_LEN, 1, &[], snapshot.mode, cx)
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
            CursorShape::Underscore => Bounds {
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
        cx: &mut WindowContext,
    ) {
        if let Some(cursor_name) = cursor_name {
            let bounds = self.bounds(origin);
            let text_size = self.line_height / 1.5;

            let name_origin = if cursor_name.is_top_row {
                point(bounds.right() - px(1.), bounds.top())
            } else {
                point(bounds.left(), bounds.top() - text_size / 2. - px(1.))
            };
            let mut name_element = div()
                .bg(self.color)
                .text_size(text_size)
                .px_0p5()
                .line_height(text_size + px(2.))
                .text_color(cursor_name.color)
                .child(cursor_name.string.clone())
                .into_any_element();

            name_element.prepaint_as_root(
                name_origin,
                size(AvailableSpace::MinContent, AvailableSpace::MinContent),
                cx,
            );

            self.cursor_name = Some(name_element);
        }
    }

    pub fn paint(&mut self, origin: gpui::Point<Pixels>, cx: &mut WindowContext) {
        let bounds = self.bounds(origin);

        //Draw background or border quad
        let cursor = if matches!(self.shape, CursorShape::Hollow) {
            outline(bounds, self.color)
        } else {
            fill(bounds, self.color)
        };

        if let Some(name) = &mut self.cursor_name {
            name.paint(cx);
        }

        cx.paint_quad(cursor);

        if let Some(block_text) = &self.block_text {
            block_text
                .paint(self.origin + origin, self.line_height, cx)
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
    pub fn paint(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) {
        if self.lines.len() >= 2 && self.lines[0].start_x > self.lines[1].end_x {
            self.paint_lines(self.start_y, &self.lines[0..1], bounds, cx);
            self.paint_lines(
                self.start_y + self.line_height,
                &self.lines[1..],
                bounds,
                cx,
            );
        } else {
            self.paint_lines(self.start_y, &self.lines, bounds, cx);
        }
    }

    fn paint_lines(
        &self,
        start_y: Pixels,
        lines: &[HighlightedRangeLine],
        _bounds: Bounds<Pixels>,
        cx: &mut WindowContext,
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
        let mut path = gpui::Path::new(first_top_right - top_curve_width);
        path.curve_to(first_top_right + curve_height, first_top_right);

        let mut iter = lines.iter().enumerate().peekable();
        while let Some((ix, line)) = iter.next() {
            let bottom_right = point(line.end_x, start_y + (ix + 1) as f32 * self.line_height);

            if let Some((_, next_line)) = iter.peek() {
                let next_top_right = point(next_line.end_x, bottom_right.y);

                match next_top_right.x.partial_cmp(&bottom_right.x).unwrap() {
                    Ordering::Equal => {
                        path.line_to(bottom_right);
                    }
                    Ordering::Less => {
                        let curve_width = curve_width(next_top_right.x, bottom_right.x);
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > Pixels::ZERO {
                            path.curve_to(bottom_right - curve_width, bottom_right);
                        }
                        path.line_to(next_top_right + curve_width);
                        if self.corner_radius > Pixels::ZERO {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                    Ordering::Greater => {
                        let curve_width = curve_width(bottom_right.x, next_top_right.x);
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > Pixels::ZERO {
                            path.curve_to(bottom_right + curve_width, bottom_right);
                        }
                        path.line_to(next_top_right - curve_width);
                        if self.corner_radius > Pixels::ZERO {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                }
            } else {
                let curve_width = curve_width(line.start_x, line.end_x);
                path.line_to(bottom_right - curve_height);
                if self.corner_radius > Pixels::ZERO {
                    path.curve_to(bottom_right - curve_width, bottom_right);
                }

                let bottom_left = point(line.start_x, bottom_right.y);
                path.line_to(bottom_left + curve_width);
                if self.corner_radius > Pixels::ZERO {
                    path.curve_to(bottom_left - curve_height, bottom_left);
                }
            }
        }

        if first_line.start_x > last_line.start_x {
            let curve_width = curve_width(last_line.start_x, first_line.start_x);
            let second_top_left = point(last_line.start_x, start_y + self.line_height);
            path.line_to(second_top_left + curve_height);
            if self.corner_radius > Pixels::ZERO {
                path.curve_to(second_top_left + curve_width, second_top_left);
            }
            let first_bottom_left = point(first_line.start_x, second_top_left.y);
            path.line_to(first_bottom_left - curve_width);
            if self.corner_radius > Pixels::ZERO {
                path.curve_to(first_bottom_left - curve_height, first_bottom_left);
            }
        }

        path.line_to(first_top_left + curve_height);
        if self.corner_radius > Pixels::ZERO {
            path.curve_to(first_top_left + top_curve_width, first_top_left);
        }
        path.line_to(first_top_right - top_curve_width);

        cx.paint_path(path, self.color);
    }
}

pub fn scale_vertical_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.5) / 100.0).into()
}

fn scale_horizontal_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.2) / 300.0).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{BlockDisposition, BlockProperties},
        editor_tests::{init_test, update_test_language_settings},
        Editor, MultiBuffer,
    };
    use gpui::{TestAppContext, VisualTestContext};
    use language::language_settings;
    use log::info;
    use std::num::NonZeroU32;
    use ui::Context;
    use util::test::sample_text;

    #[gpui::test]
    fn test_shape_line_numbers(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::Full, buffer, None, true, cx)
        });

        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let element = EditorElement::new(&editor, style);
        let snapshot = window.update(cx, |editor, cx| editor.snapshot(cx)).unwrap();

        let layouts = cx
            .update_window(*window, |_, cx| {
                element.layout_line_numbers(
                    DisplayRow(0)..DisplayRow(6),
                    (0..6).map(MultiBufferRow).map(Some),
                    &Default::default(),
                    Some(DisplayPoint::new(DisplayRow(0), 0)),
                    &snapshot,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows = window
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
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
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
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
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
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

        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&(sample_text(6, 6, 'a') + "\n"), cx);
            Editor::new(EditorMode::Full, buffer, None, true, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());

        window
            .update(cx, |editor, cx| {
                editor.cursor_shape = CursorShape::Block;
                editor.change_selections(None, cx, |s| {
                    s.select_ranges([
                        Point::new(0, 0)..Point::new(1, 0),
                        Point::new(3, 2)..Point::new(3, 3),
                        Point::new(5, 6)..Point::new(6, 0),
                    ]);
                });
            })
            .unwrap();

        let (_, state) = cx.draw(point(px(500.), px(500.)), size(px(500.), px(500.)), |_| {
            EditorElement::new(&editor, style)
        });

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

        // multi-buffer support
        // in DisplayPoint coordinates, this is what we're dealing with:
        //  0: [[file
        //  1:   header
        //  2:   section]]
        //  3: aaaaaa
        //  4: bbbbbb
        //  5: cccccc
        //  6:
        //  7: [[footer]]
        //  8: [[header]]
        //  9: ffffff
        // 10: gggggg
        // 11: hhhhhh
        // 12:
        // 13: [[footer]]
        // 14: [[file
        // 15:   header
        // 16:   section]]
        // 17: bbbbbb
        // 18: cccccc
        // 19: dddddd
        // 20: [[footer]]
        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_multi(
                [
                    (
                        &(sample_text(8, 6, 'a') + "\n"),
                        vec![
                            Point::new(0, 0)..Point::new(3, 0),
                            Point::new(4, 0)..Point::new(7, 0),
                        ],
                    ),
                    (
                        &(sample_text(8, 6, 'a') + "\n"),
                        vec![Point::new(1, 0)..Point::new(3, 0)],
                    ),
                ],
                cx,
            );
            Editor::new(EditorMode::Full, buffer, None, true, cx)
        });
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let _state = window.update(cx, |editor, cx| {
            editor.cursor_shape = CursorShape::Block;
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(7), 0),
                    DisplayPoint::new(DisplayRow(10), 0)..DisplayPoint::new(DisplayRow(13), 0),
                ]);
            });
        });

        let (_, state) = cx.draw(point(px(500.), px(500.)), size(px(500.), px(500.)), |_| {
            EditorElement::new(&editor, style)
        });
        assert_eq!(state.selections.len(), 1);
        let local_selections = &state.selections[0].1;
        assert_eq!(local_selections.len(), 2);

        // moves cursor on excerpt boundary back a line
        // and doesn't allow selection to bleed through
        assert_eq!(
            local_selections[0].range,
            DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(7), 0)
        );
        assert_eq!(
            local_selections[0].head,
            DisplayPoint::new(DisplayRow(6), 0)
        );
        // moves cursor on buffer boundary back two lines
        // and doesn't allow selection to bleed through
        assert_eq!(
            local_selections[1].range,
            DisplayPoint::new(DisplayRow(10), 0)..DisplayPoint::new(DisplayRow(13), 0)
        );
        assert_eq!(
            local_selections[1].head,
            DisplayPoint::new(DisplayRow(12), 0)
        );
    }

    #[gpui::test]
    fn test_layout_with_placeholder_text_and_blocks(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::Full, buffer, None, true, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        window
            .update(cx, |editor, cx| {
                editor.set_placeholder_text("hello", cx);
                editor.insert_blocks(
                    [BlockProperties {
                        style: BlockStyle::Fixed,
                        disposition: BlockDisposition::Above,
                        height: 3,
                        position: Anchor::min(),
                        render: Box::new(|_| div().into_any()),
                    }],
                    None,
                    cx,
                );

                // Blur the editor so that it displays placeholder text.
                cx.blur();
            })
            .unwrap();

        let (_, state) = cx.draw(point(px(500.), px(500.)), size(px(500.), px(500.)), |_| {
            EditorElement::new(&editor, style)
        });
        assert_eq!(state.position_map.line_layouts.len(), 4);
        assert_eq!(
            state
                .line_numbers
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>(),
            &[false, false, false, true]
        );
    }

    #[gpui::test]
    fn test_all_invisibles_drawing(cx: &mut TestAppContext) {
        const TAB_SIZE: u32 = 4;

        let input_text = "\t \t|\t| a b";
        let expected_invisibles = vec![
            Invisible::Tab {
                line_start_offset: 0,
            },
            Invisible::Whitespace {
                line_offset: TAB_SIZE as usize,
            },
            Invisible::Tab {
                line_start_offset: TAB_SIZE as usize + 1,
            },
            Invisible::Tab {
                line_start_offset: TAB_SIZE as usize * 2 + 1,
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

        init_test(cx, |s| {
            s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
            s.defaults.tab_size = NonZeroU32::new(TAB_SIZE);
        });

        let actual_invisibles =
            collect_invisibles_from_new_editor(cx, EditorMode::Full, &input_text, px(500.0));

        assert_eq!(expected_invisibles, actual_invisibles);
    }

    #[gpui::test]
    fn test_invisibles_dont_appear_in_certain_editors(cx: &mut TestAppContext) {
        init_test(cx, |s| {
            s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
            s.defaults.tab_size = NonZeroU32::new(4);
        });

        for editor_mode_without_invisibles in [
            EditorMode::SingleLine,
            EditorMode::AutoHeight { max_lines: 100 },
        ] {
            let invisibles = collect_invisibles_from_new_editor(
                cx,
                editor_mode_without_invisibles,
                "\t\t\t| | a b",
                px(500.0),
            );
            assert!(invisibles.is_empty(),
                    "For editor mode {editor_mode_without_invisibles:?} no invisibles was expected but got {invisibles:?}");
        }
    }

    #[gpui::test]
    fn test_wrapped_invisibles_drawing(cx: &mut TestAppContext) {
        let tab_size = 4;
        let input_text = "a\tbcd   ".repeat(9);
        let repeated_invisibles = [
            Invisible::Tab {
                line_start_offset: 1,
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
            update_test_language_settings(cx, |s| {
                s.defaults.tab_size = NonZeroU32::new(tab_size);
                s.defaults.show_whitespaces = Some(ShowWhitespaceSetting::All);
                s.defaults.preferred_line_length = Some(editor_width as u32);
                s.defaults.soft_wrap = Some(language_settings::SoftWrap::PreferredLineLength);
            });

            let actual_invisibles = collect_invisibles_from_new_editor(
                cx,
                EditorMode::Full,
                &input_text,
                px(editor_width),
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
                            panic!("At index {i}, expected invisible {expected_invisible:?} does not match actual {actual_invisible:?} by kind. Actual invisibles: {actual_invisibles:?}")
                        }
                    },
                    None => panic!("Unexpected extra invisible {actual_invisible:?} at index {i}"),
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

    fn collect_invisibles_from_new_editor(
        cx: &mut TestAppContext,
        editor_mode: EditorMode,
        input_text: &str,
        editor_width: Pixels,
    ) -> Vec<Invisible> {
        info!(
            "Creating editor with mode {editor_mode:?}, width {}px and text '{input_text}'",
            editor_width.0
        );
        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&input_text, cx);
            Editor::new(editor_mode, buffer, None, true, cx)
        });
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        window
            .update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_wrap_width(Some(editor_width), cx);
            })
            .unwrap();
        let (_, state) = cx.draw(point(px(500.), px(500.)), size(px(500.), px(500.)), |_| {
            EditorElement::new(&editor, style)
        });
        state
            .position_map
            .line_layouts
            .iter()
            .flat_map(|line_with_invisibles| &line_with_invisibles.invisibles)
            .cloned()
            .collect()
    }
}

pub fn register_action<T: Action>(
    view: &View<Editor>,
    cx: &mut WindowContext,
    listener: impl Fn(&mut Editor, &T, &mut ViewContext<Editor>) + 'static,
) {
    let view = view.clone();
    cx.on_action(TypeId::of::<T>(), move |action, phase, cx| {
        let action = action.downcast_ref().unwrap();
        if phase == DispatchPhase::Bubble {
            view.update(cx, |editor, cx| {
                listener(editor, action, cx);
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
    cx: &mut ViewContext<Editor>,
) -> Option<Size<Pixels>> {
    let width = known_dimensions.width.or_else(|| {
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
    let font_id = cx.text_system().resolve_font(&style.text.font());
    let font_size = style.text.font_size.to_pixels(cx.rem_size());
    let line_height = style.text.line_height_in_pixels(cx.rem_size());
    let em_width = cx
        .text_system()
        .typographic_bounds(font_id, font_size, 'm')
        .unwrap()
        .size
        .width;

    let mut snapshot = editor.snapshot(cx);
    let gutter_dimensions =
        snapshot.gutter_dimensions(font_id, font_size, em_width, max_line_number_width, cx);

    editor.gutter_dimensions = gutter_dimensions;
    let text_width = width - gutter_dimensions.width;
    let overscroll = size(em_width, px(0.));

    let editor_width = text_width - gutter_dimensions.margin - overscroll.width - em_width;
    if editor.set_wrap_width(Some(editor_width), cx) {
        snapshot = editor.snapshot(cx);
    }

    let scroll_height = Pixels::from(snapshot.max_point().row().next_row().0) * line_height;
    let height = scroll_height
        .max(line_height)
        .min(line_height * max_lines as f32);

    Some(size(width, height))
}
