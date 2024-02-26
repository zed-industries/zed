use crate::{
    display_map::{
        BlockContext, BlockStyle, DisplaySnapshot, FoldStatus, HighlightedChunk, ToDisplayPoint,
        TransformBlock,
    },
    editor_settings::ShowScrollbar,
    git::{diff_hunk_to_display, DisplayDiffHunk},
    hover_popover::{
        self, hover_at, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT,
    },
    items::BufferSearchHighlights,
    mouse_context_menu,
    scroll::scroll_amount::ScrollAmount,
    CursorShape, DisplayPoint, DocumentHighlightRead, DocumentHighlightWrite, Editor, EditorMode,
    EditorSettings, EditorSnapshot, EditorStyle, GutterDimensions, HalfPageDown, HalfPageUp,
    HoveredCursor, LineDown, LineUp, OpenExcerpts, PageDown, PageUp, Point, SelectPhase, Selection,
    SoftWrap, ToPoint, CURSORS_VISIBLE_FOR, MAX_LINE_LEN,
};
use anyhow::Result;
use collections::{BTreeMap, HashMap};
use git::diff::DiffHunkStatus;
use gpui::{
    div, fill, outline, overlay, point, px, quad, relative, size, transparent_black, Action,
    AnchorCorner, AnyElement, AvailableSpace, Bounds, ContentMask, Corners, CursorStyle,
    DispatchPhase, Edges, Element, ElementInputHandler, Entity, Hsla, InteractiveBounds,
    InteractiveElement, IntoElement, ModifiersChangedEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, ScrollDelta, ScrollWheelEvent, ShapedLine,
    SharedString, Size, StackingOrder, StatefulInteractiveElement, Style, Styled, TextRun,
    TextStyle, View, ViewContext, WindowContext,
};
use itertools::Itertools;
use language::language_settings::ShowWhitespaceSetting;
use lsp::DiagnosticSeverity;
use multi_buffer::Anchor;
use project::{
    project_settings::{GitGutterSetting, ProjectSettings},
    ProjectPath,
};
use settings::Settings;
use smallvec::SmallVec;
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering},
    fmt::Write,
    iter,
    ops::Range,
    sync::Arc,
};
use sum_tree::Bias;
use theme::{ActiveTheme, PlayerColor};
use ui::prelude::*;
use ui::{h_flex, ButtonLike, ButtonStyle, IconButton, Tooltip};
use util::ResultExt;
use workspace::item::Item;

struct SelectionLayout {
    head: DisplayPoint,
    cursor_shape: CursorShape,
    is_newest: bool,
    is_local: bool,
    range: Range<DisplayPoint>,
    active_rows: Range<u32>,
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
        if cursor_shape == CursorShape::Block && !range.is_empty() && !selection.reversed {
            if head.column() > 0 {
                head = map.clip_point(DisplayPoint::new(head.row(), head.column() - 1), Bias::Left)
            } else if head.row() > 0 && head != map.max_point() {
                head = map.clip_point(
                    DisplayPoint::new(head.row() - 1, map.line_len(head.row() - 1)),
                    Bias::Left,
                );
                // updating range.end is a no-op unless you're cursor is
                // on the newline containing a multi-buffer divider
                // in which case the clip_point may have moved the head up
                // an additional row.
                range.end = DisplayPoint::new(head.row() + 1, 0);
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

impl EditorElement {
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
        register_action(view, cx, Editor::delete_to_previous_word_start);
        register_action(view, cx, Editor::delete_to_previous_subword_start);
        register_action(view, cx, Editor::delete_to_next_word_end);
        register_action(view, cx, Editor::delete_to_next_subword_end);
        register_action(view, cx, Editor::delete_to_beginning_of_line);
        register_action(view, cx, Editor::delete_to_end_of_line);
        register_action(view, cx, Editor::cut_to_end_of_line);
        register_action(view, cx, Editor::duplicate_line);
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
        register_action(view, cx, Editor::go_to_diagnostic);
        register_action(view, cx, Editor::go_to_prev_diagnostic);
        register_action(view, cx, Editor::go_to_hunk);
        register_action(view, cx, Editor::go_to_prev_hunk);
        register_action(view, cx, Editor::go_to_definition);
        register_action(view, cx, Editor::go_to_definition_split);
        register_action(view, cx, Editor::go_to_implementation);
        register_action(view, cx, Editor::go_to_implementation_split);
        register_action(view, cx, Editor::go_to_type_definition);
        register_action(view, cx, Editor::go_to_type_definition_split);
        register_action(view, cx, Editor::open_url);
        register_action(view, cx, Editor::fold);
        register_action(view, cx, Editor::fold_at);
        register_action(view, cx, Editor::unfold_lines);
        register_action(view, cx, Editor::unfold_at);
        register_action(view, cx, Editor::fold_selected_ranges);
        register_action(view, cx, Editor::show_completions);
        register_action(view, cx, Editor::toggle_code_actions);
        register_action(view, cx, Editor::open_excerpts);
        register_action(view, cx, Editor::toggle_soft_wrap);
        register_action(view, cx, Editor::toggle_inlay_hints);
        register_action(view, cx, hover_popover::hover);
        register_action(view, cx, Editor::reveal_in_finder);
        register_action(view, cx, Editor::copy_path);
        register_action(view, cx, Editor::copy_relative_path);
        register_action(view, cx, Editor::copy_highlight_json);
        register_action(view, cx, Editor::copy_permalink_to_line);
        register_action(view, cx, Editor::open_permalink_to_line);
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
        register_action(view, cx, Editor::next_copilot_suggestion);
        register_action(view, cx, Editor::previous_copilot_suggestion);
        register_action(view, cx, Editor::copilot_suggest);
        register_action(view, cx, Editor::context_menu_first);
        register_action(view, cx, Editor::context_menu_prev);
        register_action(view, cx, Editor::context_menu_next);
        register_action(view, cx, Editor::context_menu_last);
        register_action(view, cx, Editor::display_cursor_names);
        register_action(view, cx, Editor::unique_lines_case_insensitive);
        register_action(view, cx, Editor::unique_lines_case_sensitive);
    }

    fn register_key_listeners(
        &self,
        cx: &mut ElementContext,
        text_bounds: Bounds<Pixels>,
        layout: &LayoutState,
    ) {
        let position_map = layout.position_map.clone();
        let stacking_order = cx.stacking_order().clone();
        cx.on_key_event({
            let editor = self.editor.clone();
            move |event: &ModifiersChangedEvent, phase, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }

                editor.update(cx, |editor, cx| {
                    Self::modifiers_changed(
                        editor,
                        event,
                        &position_map,
                        text_bounds,
                        &stacking_order,
                        cx,
                    )
                })
            }
        });
    }

    fn modifiers_changed(
        editor: &mut Editor,
        event: &ModifiersChangedEvent,
        position_map: &PositionMap,
        text_bounds: Bounds<Pixels>,
        stacking_order: &StackingOrder,
        cx: &mut ViewContext<Editor>,
    ) {
        let mouse_position = cx.mouse_position();
        if !text_bounds.contains(&mouse_position)
            || !cx.was_top_layer(&mouse_position, stacking_order)
        {
            return;
        }

        editor.update_hovered_link(
            position_map.point_for_position(text_bounds, mouse_position),
            &position_map.snapshot,
            event.modifiers,
            cx,
        )
    }

    fn mouse_left_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
        position_map: &PositionMap,
        text_bounds: Bounds<Pixels>,
        gutter_bounds: Bounds<Pixels>,
        stacking_order: &StackingOrder,
        cx: &mut ViewContext<Editor>,
    ) {
        let mut click_count = event.click_count;
        let modifiers = event.modifiers;

        if cx.default_prevented() {
            return;
        } else if gutter_bounds.contains(&event.position) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !text_bounds.contains(&event.position) {
            return;
        }
        if !cx.was_top_layer(&event.position, stacking_order) {
            return;
        }

        let point_for_position = position_map.point_for_position(text_bounds, event.position);
        let position = point_for_position.previous_valid;
        if modifiers.shift && modifiers.alt {
            editor.select(
                SelectPhase::BeginColumnar {
                    position,
                    goal_column: point_for_position.exact_unclipped.column(),
                },
                cx,
            );
        } else if modifiers.shift && !modifiers.control && !modifiers.alt && !modifiers.command {
            editor.select(
                SelectPhase::Extend {
                    position,
                    click_count,
                },
                cx,
            );
        } else {
            editor.select(
                SelectPhase::Begin {
                    position,
                    add: modifiers.alt,
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
        text_bounds: Bounds<Pixels>,
        cx: &mut ViewContext<Editor>,
    ) {
        if !text_bounds.contains(&event.position) {
            return;
        }
        let point_for_position = position_map.point_for_position(text_bounds, event.position);
        mouse_context_menu::deploy_context_menu(
            editor,
            event.position,
            point_for_position.previous_valid,
            cx,
        );
        cx.stop_propagation();
    }

    fn mouse_up(
        editor: &mut Editor,
        event: &MouseUpEvent,
        position_map: &PositionMap,
        text_bounds: Bounds<Pixels>,
        interactive_bounds: &InteractiveBounds,
        stacking_order: &StackingOrder,
        cx: &mut ViewContext<Editor>,
    ) {
        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        if end_selection {
            editor.select(SelectPhase::End, cx);
        }

        if interactive_bounds.visibly_contains(&event.position, cx)
            && !pending_nonempty_selections
            && event.modifiers.command
            && text_bounds.contains(&event.position)
            && cx.was_top_layer(&event.position, stacking_order)
        {
            let point = position_map.point_for_position(text_bounds, event.position);
            editor.handle_click_hovered_link(point, event.modifiers, cx);

            cx.stop_propagation();
        } else if end_selection {
            cx.stop_propagation();
        }
    }

    fn mouse_dragged(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        text_bounds: Bounds<Pixels>,
        _gutter_bounds: Bounds<Pixels>,
        _stacking_order: &StackingOrder,
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
        text_bounds: Bounds<Pixels>,
        gutter_bounds: Bounds<Pixels>,
        stacking_order: &StackingOrder,
        cx: &mut ViewContext<Editor>,
    ) {
        let modifiers = event.modifiers;
        let text_hovered = text_bounds.contains(&event.position);
        let gutter_hovered = gutter_bounds.contains(&event.position);
        let was_top = cx.was_top_layer(&event.position, stacking_order);

        editor.set_gutter_hovered(gutter_hovered, cx);

        // Don't trigger hover popover if mouse is hovering over context menu
        if text_hovered && was_top {
            let point_for_position = position_map.point_for_position(text_bounds, event.position);

            editor.update_hovered_link(point_for_position, &position_map.snapshot, modifiers, cx);

            if let Some(point) = point_for_position.as_valid() {
                hover_at(editor, Some(point), cx);
                Self::update_visible_cursor(editor, point, position_map, cx);
            }
        } else {
            editor.hide_hovered_link(cx);
            hover_at(editor, None, cx);
            if gutter_hovered && was_top {
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

    fn paint_background(
        &self,
        gutter_bounds: Bounds<Pixels>,
        text_bounds: Bounds<Pixels>,
        layout: &LayoutState,
        cx: &mut ElementContext,
    ) {
        let bounds = gutter_bounds.union(&text_bounds);
        let scroll_top =
            layout.position_map.snapshot.scroll_position().y * layout.position_map.line_height;
        let gutter_bg = cx.theme().colors().editor_gutter_background;
        cx.paint_quad(fill(gutter_bounds, gutter_bg));
        cx.paint_quad(fill(text_bounds, self.style.background));

        if let EditorMode::Full = layout.mode {
            let mut active_rows = layout.active_rows.iter().peekable();
            while let Some((start_row, contains_non_empty_selection)) = active_rows.next() {
                let mut end_row = *start_row;
                while active_rows.peek().map_or(false, |r| {
                    *r.0 == end_row + 1 && r.1 == contains_non_empty_selection
                }) {
                    active_rows.next().unwrap();
                    end_row += 1;
                }

                if !contains_non_empty_selection {
                    let origin = point(
                        bounds.origin.x,
                        bounds.origin.y + (layout.position_map.line_height * *start_row as f32)
                            - scroll_top,
                    );
                    let size = size(
                        bounds.size.width,
                        layout.position_map.line_height * (end_row - start_row + 1) as f32,
                    );
                    let active_line_bg = cx.theme().colors().editor_active_line_background;
                    cx.paint_quad(fill(Bounds { origin, size }, active_line_bg));
                }
            }

            if let Some(highlighted_rows) = &layout.highlighted_rows {
                let origin = point(
                    bounds.origin.x,
                    bounds.origin.y
                        + (layout.position_map.line_height * highlighted_rows.start as f32)
                        - scroll_top,
                );
                let size = size(
                    bounds.size.width,
                    layout.position_map.line_height * highlighted_rows.len() as f32,
                );
                let highlighted_line_bg = cx.theme().colors().editor_highlighted_line_background;
                cx.paint_quad(fill(Bounds { origin, size }, highlighted_line_bg));
            }

            let scroll_left =
                layout.position_map.snapshot.scroll_position().x * layout.position_map.em_width;

            for (wrap_position, active) in layout.wrap_guides.iter() {
                let x = (text_bounds.origin.x + *wrap_position + layout.position_map.em_width / 2.)
                    - scroll_left;

                if x < text_bounds.origin.x
                    || (layout.show_scrollbars && x > self.scrollbar_left(&bounds))
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
                        origin: point(x, text_bounds.origin.y),
                        size: size(px(1.), text_bounds.size.height),
                    },
                    color,
                ));
            }
        }
    }

    fn paint_gutter(
        &mut self,
        bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        let show_git_gutter = matches!(
            ProjectSettings::get_global(cx).git.git_gutter,
            Some(GitGutterSetting::TrackedFiles)
        );

        if show_git_gutter {
            Self::paint_diff_hunks(bounds, layout, cx);
        }

        let gutter_settings = EditorSettings::get_global(cx).gutter;

        for (ix, line) in layout.line_numbers.iter().enumerate() {
            if let Some(line) = line {
                let line_origin = bounds.origin
                    + point(
                        bounds.size.width - line.width - layout.gutter_dimensions.right_padding,
                        ix as f32 * line_height - (scroll_top % line_height),
                    );

                line.paint(line_origin, line_height, cx).log_err();
            }
        }

        cx.with_z_index(1, |cx| {
            for (ix, fold_indicator) in layout.fold_indicators.drain(..).enumerate() {
                if let Some(fold_indicator) = fold_indicator {
                    debug_assert!(gutter_settings.folds);
                    let mut fold_indicator = fold_indicator.into_any_element();
                    let available_space = size(
                        AvailableSpace::MinContent,
                        AvailableSpace::Definite(line_height * 0.55),
                    );
                    let fold_indicator_size = fold_indicator.measure(available_space, cx);

                    let position = point(
                        bounds.size.width - layout.gutter_dimensions.right_padding,
                        ix as f32 * line_height - (scroll_top % line_height),
                    );
                    let centering_offset = point(
                        (layout.gutter_dimensions.right_padding + layout.gutter_dimensions.margin
                            - fold_indicator_size.width)
                            / 2.,
                        (line_height - fold_indicator_size.height) / 2.,
                    );
                    let origin = bounds.origin + position + centering_offset;
                    fold_indicator.draw(origin, available_space, cx);
                }
            }

            if let Some(indicator) = layout.code_actions_indicator.take() {
                debug_assert!(gutter_settings.code_actions);
                let mut button = indicator.button.into_any_element();
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height),
                );
                let indicator_size = button.measure(available_space, cx);

                let mut x = Pixels::ZERO;
                let mut y = indicator.row as f32 * line_height - scroll_top;
                // Center indicator.
                x += (layout.gutter_dimensions.margin + layout.gutter_dimensions.left_padding
                    - indicator_size.width)
                    / 2.;
                y += (line_height - indicator_size.height) / 2.;

                button.draw(bounds.origin + point(x, y), available_space, cx);
            }
        });
    }

    fn paint_diff_hunks(bounds: Bounds<Pixels>, layout: &LayoutState, cx: &mut ElementContext) {
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        for hunk in &layout.display_hunks {
            let (display_row_range, status) = match hunk {
                //TODO: This rendering is entirely a horrible hack
                &DisplayDiffHunk::Folded { display_row: row } => {
                    let start_y = row as f32 * line_height - scroll_top;
                    let end_y = start_y + line_height;

                    let width = 0.275 * line_height;
                    let highlight_origin = bounds.origin + point(-width, start_y);
                    let highlight_size = size(width * 2., end_y - start_y);
                    let highlight_bounds = Bounds::new(highlight_origin, highlight_size);
                    cx.paint_quad(quad(
                        highlight_bounds,
                        Corners::all(1. * line_height),
                        cx.theme().status().modified,
                        Edges::default(),
                        transparent_black(),
                    ));

                    continue;
                }

                DisplayDiffHunk::Unfolded {
                    display_row_range,
                    status,
                } => (display_row_range, status),
            };

            let color = match status {
                DiffHunkStatus::Added => cx.theme().status().created,
                DiffHunkStatus::Modified => cx.theme().status().modified,

                //TODO: This rendering is entirely a horrible hack
                DiffHunkStatus::Removed => {
                    let row = display_row_range.start;

                    let offset = line_height / 2.;
                    let start_y = row as f32 * line_height - offset - scroll_top;
                    let end_y = start_y + line_height;

                    let width = 0.275 * line_height;
                    let highlight_origin = bounds.origin + point(-width, start_y);
                    let highlight_size = size(width * 2., end_y - start_y);
                    let highlight_bounds = Bounds::new(highlight_origin, highlight_size);
                    cx.paint_quad(quad(
                        highlight_bounds,
                        Corners::all(1. * line_height),
                        cx.theme().status().deleted,
                        Edges::default(),
                        transparent_black(),
                    ));

                    continue;
                }
            };

            let start_row = display_row_range.start;
            let end_row = display_row_range.end;
            // If we're in a multibuffer, row range span might include an
            // excerpt header, so if we were to draw the marker straight away,
            // the hunk might include the rows of that header.
            // Making the range inclusive doesn't quite cut it, as we rely on the exclusivity for the soft wrap.
            // Instead, we simply check whether the range we're dealing with includes
            // any excerpt headers and if so, we stop painting the diff hunk on the first row of that header.
            let end_row_in_current_excerpt = layout
                .position_map
                .snapshot
                .blocks_in_range(start_row..end_row)
                .find_map(|(start_row, block)| {
                    if matches!(block, TransformBlock::ExcerptHeader { .. }) {
                        Some(start_row)
                    } else {
                        None
                    }
                })
                .unwrap_or(end_row);

            let start_y = start_row as f32 * line_height - scroll_top;
            let end_y = end_row_in_current_excerpt as f32 * line_height - scroll_top;

            let width = 0.275 * line_height;
            let highlight_origin = bounds.origin + point(-width, start_y);
            let highlight_size = size(width * 2., end_y - start_y);
            let highlight_bounds = Bounds::new(highlight_origin, highlight_size);
            cx.paint_quad(quad(
                highlight_bounds,
                Corners::all(0.05 * line_height),
                color,
                Edges::default(),
                transparent_black(),
            ));
        }
    }

    fn paint_text(
        &mut self,
        text_bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let start_row = layout.visible_display_row_range.start;
        // Offset the content_bounds from the text_bounds by the gutter margin (which is roughly half a character wide) to make hit testing work more like how we want.
        let content_origin =
            text_bounds.origin + point(layout.gutter_dimensions.margin, Pixels::ZERO);
        let line_end_overshoot = 0.15 * layout.position_map.line_height;
        let whitespace_setting = self
            .editor
            .read(cx)
            .buffer
            .read(cx)
            .settings_at(0, cx)
            .show_whitespaces;

        cx.with_content_mask(
            Some(ContentMask {
                bounds: text_bounds,
            }),
            |cx| {
                let interactive_text_bounds = InteractiveBounds {
                    bounds: text_bounds,
                    stacking_order: cx.stacking_order().clone(),
                };
                if interactive_text_bounds.visibly_contains(&cx.mouse_position(), cx) {
                    if self
                        .editor
                        .read(cx)
                        .hovered_link_state
                        .as_ref()
                        .is_some_and(|hovered_link_state| !hovered_link_state.links.is_empty())
                    {
                        cx.set_cursor_style(CursorStyle::PointingHand);
                    } else {
                        cx.set_cursor_style(CursorStyle::IBeam);
                    }
                }

                let fold_corner_radius = 0.15 * layout.position_map.line_height;
                cx.with_element_id(Some("folds"), |cx| {
                    let snapshot = &layout.position_map.snapshot;

                    for fold in snapshot.folds_in_range(layout.visible_anchor_range.clone()) {
                        let fold_range = fold.range.clone();
                        let display_range = fold.range.start.to_display_point(&snapshot)
                            ..fold.range.end.to_display_point(&snapshot);
                        debug_assert_eq!(display_range.start.row(), display_range.end.row());
                        let row = display_range.start.row();
                        debug_assert!(row < layout.visible_display_row_range.end);
                        let Some(line_layout) = &layout
                            .position_map
                            .line_layouts
                            .get((row - layout.visible_display_row_range.start) as usize)
                            .map(|l| &l.line)
                        else {
                            continue;
                        };

                        let start_x = content_origin.x
                            + line_layout.x_for_index(display_range.start.column() as usize)
                            - layout.position_map.scroll_position.x;
                        let start_y = content_origin.y
                            + row as f32 * layout.position_map.line_height
                            - layout.position_map.scroll_position.y;
                        let end_x = content_origin.x
                            + line_layout.x_for_index(display_range.end.column() as usize)
                            - layout.position_map.scroll_position.x;

                        let fold_bounds = Bounds {
                            origin: point(start_x, start_y),
                            size: size(end_x - start_x, layout.position_map.line_height),
                        };

                        let fold_background = cx.with_z_index(1, |cx| {
                            div()
                                .id(fold.id)
                                .size_full()
                                .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                                .on_click(cx.listener_for(
                                    &self.editor,
                                    move |editor: &mut Editor, _, cx| {
                                        editor.unfold_ranges(
                                            [fold_range.start..fold_range.end],
                                            true,
                                            false,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    },
                                ))
                                .draw_and_update_state(
                                    fold_bounds.origin,
                                    fold_bounds.size,
                                    cx,
                                    |fold_element_state, cx| {
                                        if fold_element_state.is_active() {
                                            cx.theme().colors().ghost_element_active
                                        } else if fold_bounds.contains(&cx.mouse_position()) {
                                            cx.theme().colors().ghost_element_hover
                                        } else {
                                            cx.theme().colors().ghost_element_background
                                        }
                                    },
                                )
                        });

                        self.paint_highlighted_range(
                            display_range.clone(),
                            fold_background,
                            fold_corner_radius,
                            fold_corner_radius * 2.,
                            layout,
                            content_origin,
                            text_bounds,
                            cx,
                        );
                    }
                });

                for (range, color) in &layout.highlighted_ranges {
                    self.paint_highlighted_range(
                        range.clone(),
                        *color,
                        Pixels::ZERO,
                        line_end_overshoot,
                        layout,
                        content_origin,
                        text_bounds,
                        cx,
                    );
                }

                let mut cursors = SmallVec::<[Cursor; 32]>::new();
                let corner_radius = 0.15 * layout.position_map.line_height;
                let mut invisible_display_ranges = SmallVec::<[Range<DisplayPoint>; 32]>::new();

                for (participant_ix, (player_color, selections)) in
                    layout.selections.iter().enumerate()
                {
                    for selection in selections.into_iter() {
                        self.paint_highlighted_range(
                            selection.range.clone(),
                            player_color.selection,
                            corner_radius,
                            corner_radius * 2.,
                            layout,
                            content_origin,
                            text_bounds,
                            cx,
                        );

                        if selection.is_local && !selection.range.is_empty() {
                            invisible_display_ranges.push(selection.range.clone());
                        }

                        if !selection.is_local || self.editor.read(cx).show_local_cursors(cx) {
                            let cursor_position = selection.head;
                            if layout
                                .visible_display_row_range
                                .contains(&cursor_position.row())
                            {
                                let cursor_row_layout = &layout.position_map.line_layouts
                                    [(cursor_position.row() - start_row) as usize]
                                    .line;
                                let cursor_column = cursor_position.column() as usize;

                                let cursor_character_x =
                                    cursor_row_layout.x_for_index(cursor_column);
                                let mut block_width = cursor_row_layout
                                    .x_for_index(cursor_column + 1)
                                    - cursor_character_x;
                                if block_width == Pixels::ZERO {
                                    block_width = layout.position_map.em_width;
                                }
                                let block_text = if let CursorShape::Block = selection.cursor_shape
                                {
                                    layout
                                        .position_map
                                        .snapshot
                                        .chars_at(cursor_position)
                                        .next()
                                        .and_then(|(character, _)| {
                                            let text = if character == '\n' {
                                                SharedString::from(" ")
                                            } else {
                                                SharedString::from(character.to_string())
                                            };
                                            let len = text.len();
                                            cx.text_system()
                                                .shape_line(
                                                    text,
                                                    cursor_row_layout.font_size,
                                                    &[TextRun {
                                                        len,
                                                        font: self.style.text.font(),
                                                        color: self.style.background,
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

                                let x = cursor_character_x - layout.position_map.scroll_position.x;
                                let y = cursor_position.row() as f32
                                    * layout.position_map.line_height
                                    - layout.position_map.scroll_position.y;
                                if selection.is_newest {
                                    self.editor.update(cx, |editor, _| {
                                        editor.pixel_position_of_newest_cursor = Some(point(
                                            text_bounds.origin.x + x + block_width / 2.,
                                            text_bounds.origin.y
                                                + y
                                                + layout.position_map.line_height / 2.,
                                        ))
                                    });
                                }

                                cursors.push(Cursor {
                                    color: player_color.cursor,
                                    block_width,
                                    origin: point(x, y),
                                    line_height: layout.position_map.line_height,
                                    shape: selection.cursor_shape,
                                    block_text,
                                    cursor_name: selection.user_name.clone().map(|name| {
                                        CursorName {
                                            string: name,
                                            color: self.style.background,
                                            is_top_row: cursor_position.row() == 0,
                                            z_index: (participant_ix % 256).try_into().unwrap(),
                                        }
                                    }),
                                });
                            }
                        }
                    }
                }

                for (ix, line_with_invisibles) in
                    layout.position_map.line_layouts.iter().enumerate()
                {
                    let row = start_row + ix as u32;
                    line_with_invisibles.draw(
                        layout,
                        row,
                        content_origin,
                        whitespace_setting,
                        &invisible_display_ranges,
                        cx,
                    )
                }

                cx.with_z_index(0, |cx| self.paint_redactions(text_bounds, &layout, cx));

                cx.with_z_index(1, |cx| {
                    for cursor in cursors {
                        cursor.paint(content_origin, cx);
                    }
                });
            },
        )
    }

    fn paint_redactions(
        &mut self,
        text_bounds: Bounds<Pixels>,
        layout: &LayoutState,
        cx: &mut ElementContext,
    ) {
        let content_origin =
            text_bounds.origin + point(layout.gutter_dimensions.margin, Pixels::ZERO);
        let line_end_overshoot = layout.line_end_overshoot();

        // A softer than perfect black
        let redaction_color = gpui::rgb(0x0e1111);

        for range in layout.redacted_ranges.iter() {
            self.paint_highlighted_range(
                range.clone(),
                redaction_color.into(),
                Pixels::ZERO,
                line_end_overshoot,
                layout,
                content_origin,
                text_bounds,
                cx,
            );
        }
    }

    fn paint_overlays(
        &mut self,
        text_bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let content_origin =
            text_bounds.origin + point(layout.gutter_dimensions.margin, Pixels::ZERO);
        let start_row = layout.visible_display_row_range.start;
        if let Some((position, mut context_menu)) = layout.context_menu.take() {
            let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
            let context_menu_size = context_menu.measure(available_space, cx);

            let cursor_row_layout =
                &layout.position_map.line_layouts[(position.row() - start_row) as usize].line;
            let x = cursor_row_layout.x_for_index(position.column() as usize)
                - layout.position_map.scroll_position.x;
            let y = (position.row() + 1) as f32 * layout.position_map.line_height
                - layout.position_map.scroll_position.y;
            let mut list_origin = content_origin + point(x, y);
            let list_width = context_menu_size.width;
            let list_height = context_menu_size.height;

            // Snap the right edge of the list to the right edge of the window if
            // its horizontal bounds overflow.
            if list_origin.x + list_width > cx.viewport_size().width {
                list_origin.x = (cx.viewport_size().width - list_width).max(Pixels::ZERO);
            }

            if list_origin.y + list_height > text_bounds.lower_right().y {
                list_origin.y -= layout.position_map.line_height + list_height;
            }

            cx.break_content_mask(|cx| context_menu.draw(list_origin, available_space, cx));
        }

        if let Some((position, mut hover_popovers)) = layout.hover_popovers.take() {
            let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);

            // This is safe because we check on layout whether the required row is available
            let hovered_row_layout =
                &layout.position_map.line_layouts[(position.row() - start_row) as usize].line;

            // Minimum required size: Take the first popover, and add 1.5 times the minimum popover
            // height. This is the size we will use to decide whether to render popovers above or below
            // the hovered line.
            let first_size = hover_popovers[0].measure(available_space, cx);
            let height_to_reserve =
                first_size.height + 1.5 * MIN_POPOVER_LINE_HEIGHT * layout.position_map.line_height;

            // Compute Hovered Point
            let x = hovered_row_layout.x_for_index(position.column() as usize)
                - layout.position_map.scroll_position.x;
            let y = position.row() as f32 * layout.position_map.line_height
                - layout.position_map.scroll_position.y;
            let hovered_point = content_origin + point(x, y);

            if hovered_point.y - height_to_reserve > Pixels::ZERO {
                // There is enough space above. Render popovers above the hovered point
                let mut current_y = hovered_point.y;
                for mut hover_popover in hover_popovers {
                    let size = hover_popover.measure(available_space, cx);
                    let mut popover_origin = point(hovered_point.x, current_y - size.height);

                    let x_out_of_bounds =
                        text_bounds.upper_right().x - (popover_origin.x + size.width);
                    if x_out_of_bounds < Pixels::ZERO {
                        popover_origin.x = popover_origin.x + x_out_of_bounds;
                    }

                    if cx.was_top_layer(&popover_origin, cx.stacking_order()) {
                        cx.break_content_mask(|cx| {
                            hover_popover.draw(popover_origin, available_space, cx)
                        });
                    }

                    current_y = popover_origin.y - HOVER_POPOVER_GAP;
                }
            } else {
                // There is not enough space above. Render popovers below the hovered point
                let mut current_y = hovered_point.y + layout.position_map.line_height;
                for mut hover_popover in hover_popovers {
                    let size = hover_popover.measure(available_space, cx);
                    let mut popover_origin = point(hovered_point.x, current_y);

                    let x_out_of_bounds =
                        text_bounds.upper_right().x - (popover_origin.x + size.width);
                    if x_out_of_bounds < Pixels::ZERO {
                        popover_origin.x = popover_origin.x + x_out_of_bounds;
                    }

                    hover_popover.draw(popover_origin, available_space, cx);

                    current_y = popover_origin.y + size.height + HOVER_POPOVER_GAP;
                }
            }
        }

        if let Some(mouse_context_menu) = self.editor.read(cx).mouse_context_menu.as_ref() {
            let element = overlay()
                .position(mouse_context_menu.position)
                .child(mouse_context_menu.context_menu.clone())
                .anchor(AnchorCorner::TopLeft)
                .snap_to_window();
            element.into_any().draw(
                gpui::Point::default(),
                size(AvailableSpace::MinContent, AvailableSpace::MinContent),
                cx,
            );
        }
    }

    fn scrollbar_left(&self, bounds: &Bounds<Pixels>) -> Pixels {
        bounds.upper_right().x - self.style.scrollbar_width
    }

    fn paint_scrollbar(
        &mut self,
        bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        if layout.mode != EditorMode::Full {
            return;
        }

        // If a drag took place after we started dragging the scrollbar,
        // cancel the scrollbar drag.
        if cx.has_active_drag() {
            self.editor.update(cx, |editor, cx| {
                editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
            });
        }

        let top = bounds.origin.y;
        let bottom = bounds.lower_left().y;
        let right = bounds.lower_right().x;
        let left = self.scrollbar_left(&bounds);
        let row_range = layout.scrollbar_row_range.clone();
        let max_row = layout.max_row as f32 + (row_range.end - row_range.start);

        let mut height = bounds.size.height;
        let mut first_row_y_offset = px(0.0);

        // Impose a minimum height on the scrollbar thumb
        let row_height = height / max_row;
        let min_thumb_height = layout.position_map.line_height;
        let thumb_height = (row_range.end - row_range.start) * row_height;
        if thumb_height < min_thumb_height {
            first_row_y_offset = (min_thumb_height - thumb_height) / 2.0;
            height -= min_thumb_height - thumb_height;
        }

        let y_for_row = |row: f32| -> Pixels { top + first_row_y_offset + row * row_height };

        let thumb_top = y_for_row(row_range.start) - first_row_y_offset;
        let thumb_bottom = y_for_row(row_range.end) + first_row_y_offset;
        let track_bounds = Bounds::from_corners(point(left, top), point(right, bottom));
        let thumb_bounds = Bounds::from_corners(point(left, thumb_top), point(right, thumb_bottom));

        if layout.show_scrollbars {
            cx.paint_quad(quad(
                track_bounds,
                Corners::default(),
                cx.theme().colors().scrollbar_track_background,
                Edges {
                    top: Pixels::ZERO,
                    right: Pixels::ZERO,
                    bottom: Pixels::ZERO,
                    left: px(1.),
                },
                cx.theme().colors().scrollbar_track_border,
            ));
            let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
            if layout.is_singleton && scrollbar_settings.selections {
                let start_anchor = Anchor::min();
                let end_anchor = Anchor::max();
                let background_ranges = self
                    .editor
                    .read(cx)
                    .background_highlight_row_ranges::<BufferSearchHighlights>(
                        start_anchor..end_anchor,
                        &layout.position_map.snapshot,
                        50000,
                    );
                for range in background_ranges {
                    let start_y = y_for_row(range.start().row() as f32);
                    let mut end_y = y_for_row(range.end().row() as f32);
                    if end_y - start_y < px(1.) {
                        end_y = start_y + px(1.);
                    }
                    let bounds = Bounds::from_corners(point(left, start_y), point(right, end_y));
                    cx.paint_quad(quad(
                        bounds,
                        Corners::default(),
                        cx.theme().status().info,
                        Edges {
                            top: Pixels::ZERO,
                            right: px(1.),
                            bottom: Pixels::ZERO,
                            left: px(1.),
                        },
                        cx.theme().colors().scrollbar_thumb_border,
                    ));
                }
            }

            if layout.is_singleton && scrollbar_settings.symbols_selections {
                let selection_ranges = self.editor.read(cx).background_highlights_in_range(
                    Anchor::min()..Anchor::max(),
                    &layout.position_map.snapshot,
                    cx.theme().colors(),
                );
                for hunk in selection_ranges {
                    let start_display = Point::new(hunk.0.start.row(), 0)
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let end_display = Point::new(hunk.0.end.row(), 0)
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let start_y = y_for_row(start_display.row() as f32);
                    let mut end_y = if hunk.0.start == hunk.0.end {
                        y_for_row((end_display.row() + 1) as f32)
                    } else {
                        y_for_row((end_display.row()) as f32)
                    };

                    if end_y - start_y < px(1.) {
                        end_y = start_y + px(1.);
                    }
                    let bounds = Bounds::from_corners(point(left, start_y), point(right, end_y));

                    cx.paint_quad(quad(
                        bounds,
                        Corners::default(),
                        cx.theme().status().info,
                        Edges {
                            top: Pixels::ZERO,
                            right: px(1.),
                            bottom: Pixels::ZERO,
                            left: px(1.),
                        },
                        cx.theme().colors().scrollbar_thumb_border,
                    ));
                }
            }

            if layout.is_singleton && scrollbar_settings.git_diff {
                for hunk in layout
                    .position_map
                    .snapshot
                    .buffer_snapshot
                    .git_diff_hunks_in_range(0..(max_row.floor() as u32))
                {
                    let start_display = Point::new(hunk.buffer_range.start, 0)
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let end_display = Point::new(hunk.buffer_range.end, 0)
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let start_y = y_for_row(start_display.row() as f32);
                    let mut end_y = if hunk.buffer_range.start == hunk.buffer_range.end {
                        y_for_row((end_display.row() + 1) as f32)
                    } else {
                        y_for_row((end_display.row()) as f32)
                    };

                    if end_y - start_y < px(1.) {
                        end_y = start_y + px(1.);
                    }
                    let bounds = Bounds::from_corners(point(left, start_y), point(right, end_y));

                    let color = match hunk.status() {
                        DiffHunkStatus::Added => cx.theme().status().created,
                        DiffHunkStatus::Modified => cx.theme().status().modified,
                        DiffHunkStatus::Removed => cx.theme().status().deleted,
                    };
                    cx.paint_quad(quad(
                        bounds,
                        Corners::default(),
                        color,
                        Edges {
                            top: Pixels::ZERO,
                            right: px(1.),
                            bottom: Pixels::ZERO,
                            left: px(1.),
                        },
                        cx.theme().colors().scrollbar_thumb_border,
                    ));
                }
            }

            if layout.is_singleton && scrollbar_settings.diagnostics {
                let max_point = layout
                    .position_map
                    .snapshot
                    .display_snapshot
                    .buffer_snapshot
                    .max_point();

                let diagnostics = layout
                    .position_map
                    .snapshot
                    .buffer_snapshot
                    .diagnostics_in_range::<_, Point>(Point::zero()..max_point, false)
                    // We want to sort by severity, in order to paint the most severe diagnostics last.
                    .sorted_by_key(|diagnostic| std::cmp::Reverse(diagnostic.diagnostic.severity));

                for diagnostic in diagnostics {
                    let start_display = diagnostic
                        .range
                        .start
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let end_display = diagnostic
                        .range
                        .end
                        .to_display_point(&layout.position_map.snapshot.display_snapshot);
                    let start_y = y_for_row(start_display.row() as f32);
                    let mut end_y = if diagnostic.range.start == diagnostic.range.end {
                        y_for_row((end_display.row() + 1) as f32)
                    } else {
                        y_for_row((end_display.row()) as f32)
                    };

                    if end_y - start_y < px(1.) {
                        end_y = start_y + px(1.);
                    }
                    let bounds = Bounds::from_corners(point(left, start_y), point(right, end_y));

                    let color = match diagnostic.diagnostic.severity {
                        DiagnosticSeverity::ERROR => cx.theme().status().error,
                        DiagnosticSeverity::WARNING => cx.theme().status().warning,
                        DiagnosticSeverity::INFORMATION => cx.theme().status().info,
                        _ => cx.theme().status().hint,
                    };
                    cx.paint_quad(quad(
                        bounds,
                        Corners::default(),
                        color,
                        Edges {
                            top: Pixels::ZERO,
                            right: px(1.),
                            bottom: Pixels::ZERO,
                            left: px(1.),
                        },
                        cx.theme().colors().scrollbar_thumb_border,
                    ));
                }
            }

            cx.paint_quad(quad(
                thumb_bounds,
                Corners::default(),
                cx.theme().colors().scrollbar_thumb_background,
                Edges {
                    top: Pixels::ZERO,
                    right: px(1.),
                    bottom: Pixels::ZERO,
                    left: px(1.),
                },
                cx.theme().colors().scrollbar_thumb_border,
            ));
        }

        let interactive_track_bounds = InteractiveBounds {
            bounds: track_bounds,
            stacking_order: cx.stacking_order().clone(),
        };
        let mut mouse_position = cx.mouse_position();
        if interactive_track_bounds.visibly_contains(&mouse_position, cx) {
            cx.set_cursor_style(CursorStyle::Arrow);
        }

        cx.on_mouse_event({
            let editor = self.editor.clone();
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
                        if (track_bounds.top()..track_bounds.bottom()).contains(&y) {
                            let mut position = editor.scroll_position(cx);
                            position.y += (new_y - y) * (max_row as f32) / height;
                            if position.y < 0.0 {
                                position.y = 0.0;
                            }
                            editor.set_scroll_position(position, cx);
                        }

                        mouse_position = event.position;
                        cx.stop_propagation();
                    } else {
                        editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
                        if interactive_track_bounds.visibly_contains(&event.position, cx) {
                            editor.scroll_manager.show_scrollbar(cx);
                        }
                    }
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
                move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Capture {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        if track_bounds.contains(&event.position) {
                            editor.scroll_manager.set_is_dragging_scrollbar(true, cx);

                            let y = event.position.y;
                            if y < thumb_top || thumb_bottom < y {
                                let center_row =
                                    ((y - top) * max_row as f32 / height).round() as u32;
                                let top_row = center_row
                                    .saturating_sub((row_range.end - row_range.start) as u32 / 2);
                                let mut position = editor.scroll_position(cx);
                                position.y = top_row as f32;
                                editor.set_scroll_position(position, cx);
                            } else {
                                editor.scroll_manager.show_scrollbar(cx);
                            }

                            cx.stop_propagation();
                        }
                    });
                }
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_highlighted_range(
        &self,
        range: Range<DisplayPoint>,
        color: Hsla,
        corner_radius: Pixels,
        line_end_overshoot: Pixels,
        layout: &LayoutState,
        content_origin: gpui::Point<Pixels>,
        bounds: Bounds<Pixels>,
        cx: &mut ElementContext,
    ) {
        let start_row = layout.visible_display_row_range.start;
        let end_row = layout.visible_display_row_range.end;
        if range.start != range.end {
            let row_range = if range.end.column() == 0 {
                cmp::max(range.start.row(), start_row)..cmp::min(range.end.row(), end_row)
            } else {
                cmp::max(range.start.row(), start_row)..cmp::min(range.end.row() + 1, end_row)
            };

            let highlighted_range = HighlightedRange {
                color,
                line_height: layout.position_map.line_height,
                corner_radius,
                start_y: content_origin.y
                    + row_range.start as f32 * layout.position_map.line_height
                    - layout.position_map.scroll_position.y,
                lines: row_range
                    .into_iter()
                    .map(|row| {
                        let line_layout =
                            &layout.position_map.line_layouts[(row - start_row) as usize].line;
                        HighlightedRangeLine {
                            start_x: if row == range.start.row() {
                                content_origin.x
                                    + line_layout.x_for_index(range.start.column() as usize)
                                    - layout.position_map.scroll_position.x
                            } else {
                                content_origin.x - layout.position_map.scroll_position.x
                            },
                            end_x: if row == range.end.row() {
                                content_origin.x
                                    + line_layout.x_for_index(range.end.column() as usize)
                                    - layout.position_map.scroll_position.x
                            } else {
                                content_origin.x + line_layout.width + line_end_overshoot
                                    - layout.position_map.scroll_position.x
                            },
                        }
                    })
                    .collect(),
            };

            highlighted_range.paint(bounds, cx);
        }
    }

    fn paint_blocks(
        &mut self,
        bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_left = scroll_position.x * layout.position_map.em_width;
        let scroll_top = scroll_position.y * layout.position_map.line_height;

        for mut block in layout.blocks.drain(..) {
            let mut origin = bounds.origin
                + point(
                    Pixels::ZERO,
                    block.row as f32 * layout.position_map.line_height - scroll_top,
                );
            if !matches!(block.style, BlockStyle::Sticky) {
                origin += point(-scroll_left, Pixels::ZERO);
            }
            block.element.draw(origin, block.available_space, cx);
        }
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
        let digit_count = (snapshot.max_buffer_row() as f32 + 1.).log10().floor() as usize + 1;
        self.column_pixels(digit_count, cx)
    }

    //Folds contained in a hunk are ignored apart from shrinking visual size
    //If a fold contains any hunks then that fold line is marked as modified
    fn layout_git_gutters(
        &self,
        display_rows: Range<u32>,
        snapshot: &EditorSnapshot,
    ) -> Vec<DisplayDiffHunk> {
        let buffer_snapshot = &snapshot.buffer_snapshot;

        let buffer_start_row = DisplayPoint::new(display_rows.start, 0)
            .to_point(snapshot)
            .row;
        let buffer_end_row = DisplayPoint::new(display_rows.end, 0)
            .to_point(snapshot)
            .row;

        buffer_snapshot
            .git_diff_hunks_in_range(buffer_start_row..buffer_end_row)
            .map(|hunk| diff_hunk_to_display(hunk, snapshot))
            .dedup()
            .collect()
    }

    fn calculate_relative_line_numbers(
        &self,
        snapshot: &EditorSnapshot,
        rows: &Range<u32>,
        relative_to: Option<u32>,
    ) -> HashMap<u32, u32> {
        let mut relative_rows: HashMap<u32, u32> = Default::default();
        let Some(relative_to) = relative_to else {
            return relative_rows;
        };

        let start = rows.start.min(relative_to);
        let end = rows.end.max(relative_to);

        let buffer_rows = snapshot
            .buffer_rows(start)
            .take(1 + (end - start) as usize)
            .collect::<Vec<_>>();

        let head_idx = relative_to - start;
        let mut delta = 1;
        let mut i = head_idx + 1;
        while i < buffer_rows.len() as u32 {
            if buffer_rows[i as usize].is_some() {
                if rows.contains(&(i + start)) {
                    relative_rows.insert(i + start, delta);
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
                if rows.contains(&(i + start)) {
                    relative_rows.insert(i + start, delta);
                }
                delta += 1;
            }
        }

        relative_rows
    }

    fn shape_line_numbers(
        &self,
        rows: Range<u32>,
        active_rows: &BTreeMap<u32, bool>,
        newest_selection_head: DisplayPoint,
        is_singleton: bool,
        snapshot: &EditorSnapshot,
        cx: &ViewContext<Editor>,
    ) -> (
        Vec<Option<ShapedLine>>,
        Vec<Option<(FoldStatus, BufferRow, bool)>>,
    ) {
        let font_size = self.style.text.font_size.to_pixels(cx.rem_size());
        let include_line_numbers =
            EditorSettings::get_global(cx).gutter.line_numbers && snapshot.mode == EditorMode::Full;
        let include_fold_statuses =
            EditorSettings::get_global(cx).gutter.folds && snapshot.mode == EditorMode::Full;
        let mut shaped_line_numbers = Vec::with_capacity(rows.len());
        let mut fold_statuses = Vec::with_capacity(rows.len());
        let mut line_number = String::new();
        let is_relative = EditorSettings::get_global(cx).relative_line_numbers;
        let relative_to = if is_relative {
            Some(newest_selection_head.row())
        } else {
            None
        };

        let relative_rows = self.calculate_relative_line_numbers(&snapshot, &rows, relative_to);

        for (ix, row) in snapshot
            .buffer_rows(rows.start)
            .take((rows.end - rows.start) as usize)
            .enumerate()
        {
            let display_row = rows.start + ix as u32;
            let (active, color) = if active_rows.contains_key(&display_row) {
                (true, cx.theme().colors().editor_active_line_number)
            } else {
                (false, cx.theme().colors().editor_line_number)
            };
            if let Some(buffer_row) = row {
                if include_line_numbers {
                    line_number.clear();
                    let default_number = buffer_row + 1;
                    let number = relative_rows
                        .get(&(ix as u32 + rows.start))
                        .unwrap_or(&default_number);
                    write!(&mut line_number, "{}", number).unwrap();
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
                    shaped_line_numbers.push(Some(shaped_line));
                }
                if include_fold_statuses {
                    fold_statuses.push(
                        is_singleton
                            .then(|| {
                                snapshot
                                    .fold_for_line(buffer_row)
                                    .map(|fold_status| (fold_status, buffer_row, active))
                            })
                            .flatten(),
                    )
                }
            } else {
                fold_statuses.push(None);
                shaped_line_numbers.push(None);
            }
        }

        (shaped_line_numbers, fold_statuses)
    }

    fn layout_lines(
        &self,
        rows: Range<u32>,
        line_number_layouts: &[Option<ShapedLine>],
        snapshot: &EditorSnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> Vec<LineWithInvisibles> {
        if rows.start >= rows.end {
            return Vec::new();
        }

        // Show the placeholder when the editor is empty
        if snapshot.is_empty() {
            let font_size = self.style.text.font_size.to_pixels(cx.rem_size());
            let placeholder_color = cx.theme().colors().text_placeholder;
            let placeholder_text = snapshot.placeholder_text(cx);

            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start as usize)
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
                    line,
                    invisibles: Vec::new(),
                })
                .collect()
        } else {
            let chunks = snapshot.highlighted_chunks(rows.clone(), true, &self.style);
            LineWithInvisibles::from_chunks(
                chunks,
                &self.style.text,
                MAX_LINE_LEN,
                rows.len() as usize,
                line_number_layouts,
                snapshot.mode,
                cx,
            )
        }
    }

    fn compute_layout(&mut self, bounds: Bounds<Pixels>, cx: &mut ElementContext) -> LayoutState {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
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

            editor.gutter_width = gutter_dimensions.width;

            let text_width = bounds.size.width - gutter_dimensions.width;
            let overscroll = size(em_width, px(0.));
            let _snapshot = {
                editor.set_visible_line_count((bounds.size.height / line_height).into(), cx);

                let editor_width = text_width - gutter_dimensions.margin - overscroll.width - em_width;
                let wrap_width = match editor.soft_wrap_mode(cx) {
                    SoftWrap::None => (MAX_LINE_LEN / 2) as f32 * em_advance,
                    SoftWrap::EditorWidth => editor_width,
                    SoftWrap::Column(column) => editor_width.min(column as f32 * em_advance),
                };

                if editor.set_wrap_width(Some(wrap_width), cx) {
                    editor.snapshot(cx)
                } else {
                    snapshot
                }
            };

            let wrap_guides = editor
                .wrap_guides(cx)
                .iter()
                .map(|(guide, active)| (self.column_pixels(*guide, cx), *active))
                .collect::<SmallVec<[_; 2]>>();

            let gutter_size = size(gutter_dimensions.width, bounds.size.height);
            let text_size = size(text_width, bounds.size.height);

            let autoscroll_horizontally =
                editor.autoscroll_vertically(bounds.size.height, line_height, cx);
            let mut snapshot = editor.snapshot(cx);

            let scroll_position = snapshot.scroll_position();
            // The scroll position is a fractional point, the whole number of which represents
            // the top of the window in terms of display rows.
            let start_row = scroll_position.y as u32;
            let height_in_lines = f32::from(bounds.size.height / line_height);
            let max_row = snapshot.max_point().row();

            // Add 1 to ensure selections bleed off screen
            let end_row = 1 + cmp::min((scroll_position.y + height_in_lines).ceil() as u32, max_row);

            let start_anchor = if start_row == 0 {
                Anchor::min()
            } else {
                snapshot
                    .buffer_snapshot
                    .anchor_before(DisplayPoint::new(start_row, 0).to_offset(&snapshot, Bias::Left))
            };
            let end_anchor = if end_row > max_row {
                Anchor::max()
            } else {
                snapshot
                    .buffer_snapshot
                    .anchor_before(DisplayPoint::new(end_row, 0).to_offset(&snapshot, Bias::Right))
            };

            let mut selections: Vec<(PlayerColor, Vec<SelectionLayout>)> = Vec::new();
            let mut active_rows = BTreeMap::new();
            let is_singleton = editor.is_singleton(cx);

            let highlighted_rows = editor.highlighted_rows();
            let highlighted_ranges = editor.background_highlights_in_range(
                start_anchor..end_anchor,
                &snapshot.display_snapshot,
                cx.theme().colors(),
            );

            let redacted_ranges = editor.redacted_ranges(start_anchor..end_anchor, &snapshot.display_snapshot, cx);

            let mut newest_selection_head = None;

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
                        true,
                        None,
                    );
                    if is_newest {
                        newest_selection_head = Some(layout.head);
                    }

                    for row in cmp::max(layout.active_rows.start, start_row)
                        ..=cmp::min(layout.active_rows.end, end_row)
                    {
                        let contains_non_empty_selection = active_rows.entry(row).or_insert(!is_empty);
                        *contains_non_empty_selection |= !is_empty;
                    }
                    layouts.push(layout);
                }

                let player = if editor.read_only(cx) {
                    cx.theme().players().read_only()
                } else {
                    style.local_player
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
                    let selection_style = if let Some(participant_index) = selection.participant_index {
                        cx.theme()
                            .players()
                            .color_for_participant(participant_index.0)
                    } else {
                        cx.theme().players().absent()
                    };

                    // Don't re-render the leader's selections, since the local selections
                    // match theirs.
                    if Some(selection.peer_id) == editor.leader_peer_id {
                        continue;
                    }
                    let key = HoveredCursor{replica_id: selection.replica_id, selection_id: selection.selection.id};

                    let is_shown = editor.show_cursor_names || editor.hovered_cursors.contains_key(&key);

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
                            if is_shown {
                                selection.user_name
                            } else {
                                None
                            },
                        ));
                }

                selections.extend(remote_selections.into_values());
            }

            let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
            let show_scrollbars = match scrollbar_settings.show {
                ShowScrollbar::Auto => {
                    // Git
                    (is_singleton && scrollbar_settings.git_diff && snapshot.buffer_snapshot.has_git_diffs())
                    ||
                    // Selections
                    (is_singleton && scrollbar_settings.selections && editor.has_background_highlights::<BufferSearchHighlights>())
                    ||
                    // Symbols Selections
                    (is_singleton && scrollbar_settings.symbols_selections && (editor.has_background_highlights::<DocumentHighlightRead>() || editor.has_background_highlights::<DocumentHighlightWrite>()))
                    ||
                    // Diagnostics
                    (is_singleton && scrollbar_settings.diagnostics && snapshot.buffer_snapshot.has_diagnostics())
                    ||
                    // Scrollmanager
                    editor.scroll_manager.scrollbars_visible()
                }
                ShowScrollbar::System => editor.scroll_manager.scrollbars_visible(),
                ShowScrollbar::Always => true,
                ShowScrollbar::Never => false,
            };

            let head_for_relative = newest_selection_head.unwrap_or_else(|| {
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

            let (line_numbers, fold_statuses) = self.shape_line_numbers(
                start_row..end_row,
                &active_rows,
                head_for_relative,
                is_singleton,
                &snapshot,
                cx,
            );

            let display_hunks = self.layout_git_gutters(start_row..end_row, &snapshot);

            let scrollbar_row_range = scroll_position.y..(scroll_position.y + height_in_lines);

            let mut max_visible_line_width = Pixels::ZERO;
            let line_layouts = self.layout_lines(start_row..end_row, &line_numbers, &snapshot, cx);
            for line_with_invisibles in &line_layouts {
                if line_with_invisibles.line.width > max_visible_line_width {
                    max_visible_line_width = line_with_invisibles.line.width;
                }
            }

            let longest_line_width = layout_line(snapshot.longest_row(), &snapshot, &style, cx)
                .unwrap()
                .width;
            let scroll_width = longest_line_width.max(max_visible_line_width) + overscroll.width;

            let (scroll_width, blocks) = cx.with_element_context(|cx| {
             cx.with_element_id(Some("editor_blocks"), |cx| {
                self.layout_blocks(
                    start_row..end_row,
                    &snapshot,
                    bounds.size.width,
                    scroll_width,
                    text_width,
                    &gutter_dimensions,
                    em_width,
                    gutter_dimensions.width + gutter_dimensions.margin,
                    line_height,
                    &style,
                    &line_layouts,
                    editor,
                    cx,
                )
            })
            });

            let scroll_max = point(
                f32::from((scroll_width - text_size.width) / em_width).max(0.0),
                max_row as f32,
            );

            let clamped = editor.scroll_manager.clamp_scroll_left(scroll_max.x);

            let autoscrolled = if autoscroll_horizontally {
                editor.autoscroll_horizontally(
                    start_row,
                    text_size.width,
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
            }

            let gutter_settings = EditorSettings::get_global(cx).gutter;

            let mut context_menu = None;
            let mut code_actions_indicator = None;
            if let Some(newest_selection_head) = newest_selection_head {
                if (start_row..end_row).contains(&newest_selection_head.row()) {
                    if editor.context_menu_visible() {
                        let max_height = cmp::min(
                            12. * line_height,
                            cmp::max(
                                3. * line_height,
                                (bounds.size.height - line_height) / 2.,
                            )
                        );
                        context_menu =
                            editor.render_context_menu(newest_selection_head, &self.style, max_height, cx);
                    }

                    let active = matches!(
                        editor.context_menu.read().as_ref(),
                        Some(crate::ContextMenu::CodeActions(_))
                    );

                    if gutter_settings.code_actions {
                        code_actions_indicator = editor
                            .render_code_actions_indicator(&style, active, cx)
                            .map(|element| CodeActionsIndicator {
                                row: newest_selection_head.row(),
                                button: element,
                            });
                    }
                }
            }

            let visible_rows = start_row..start_row + line_layouts.len() as u32;
            let max_size = size(
                (120. * em_width) // Default size
                    .min(bounds.size.width / 2.) // Shrink to half of the editor width
                    .max(MIN_POPOVER_CHARACTER_WIDTH * em_width), // Apply minimum width of 20 characters
                (16. * line_height) // Default size
                    .min(bounds.size.height / 2.) // Shrink to half of the editor height
                    .max(MIN_POPOVER_LINE_HEIGHT * line_height), // Apply minimum height of 4 lines
            );

            let hover = if context_menu.is_some() {
                None
            } else {
                editor.hover_state.render(
                    &snapshot,
                    &style,
                    visible_rows,
                    max_size,
                    editor.workspace.as_ref().map(|(w, _)| w.clone()),
                    cx,
                )
            };

            let editor_view = cx.view().clone();
            let fold_indicators = if gutter_settings.folds {
                cx.with_element_context(|cx| {
                    cx.with_element_id(Some("gutter_fold_indicators"), |_cx| {
                        editor.render_fold_indicators(
                            fold_statuses,
                            &style,
                            editor.gutter_hovered,
                            line_height,
                            gutter_dimensions.margin,
                            editor_view,
                        )
                    })
                })
            } else {
                Vec::new()
            };

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

            LayoutState {
                mode: snapshot.mode,
                position_map: Arc::new(PositionMap {
                    size: bounds.size,
                    scroll_position: point(
                        scroll_position.x * em_width,
                        scroll_position.y * line_height,
                    ),
                    scroll_max,
                    line_layouts,
                    line_height,
                    em_width,
                    em_advance,
                    snapshot,
                }),
                visible_anchor_range: start_anchor..end_anchor,
                visible_display_row_range: start_row..end_row,
                wrap_guides,
                gutter_size,
                gutter_dimensions,
                text_size,
                scrollbar_row_range,
                show_scrollbars,
                is_singleton,
                max_row,
                active_rows,
                highlighted_rows,
                highlighted_ranges,
                redacted_ranges,
                line_numbers,
                display_hunks,
                blocks,
                selections,
                context_menu,
                code_actions_indicator,
                fold_indicators,
                tab_invisible,
                space_invisible,
                hover_popovers: hover,
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_blocks(
        &self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        editor_width: Pixels,
        scroll_width: Pixels,
        text_width: Pixels,
        gutter_dimensions: &GutterDimensions,
        em_width: Pixels,
        text_x: Pixels,
        line_height: Pixels,
        style: &EditorStyle,
        line_layouts: &[LineWithInvisibles],
        editor: &mut Editor,
        cx: &mut ElementContext,
    ) -> (Pixels, Vec<BlockLayout>) {
        let mut block_id = 0;
        let (fixed_blocks, non_fixed_blocks) = snapshot
            .blocks_in_range(rows.clone())
            .partition::<Vec<_>, _>(|(_, block)| match block {
                TransformBlock::ExcerptHeader { .. } => false,
                TransformBlock::Custom(block) => block.style() == BlockStyle::Fixed,
            });

        let render_block = |block: &TransformBlock,
                            available_space: Size<AvailableSpace>,
                            block_id: usize,
                            editor: &mut Editor,
                            cx: &mut ElementContext| {
            let mut element = match block {
                TransformBlock::Custom(block) => {
                    let align_to = block
                        .position()
                        .to_point(&snapshot.buffer_snapshot)
                        .to_display_point(snapshot);
                    let anchor_x = text_x
                        + if rows.contains(&align_to.row()) {
                            line_layouts[(align_to.row() - rows.start) as usize]
                                .line
                                .x_for_index(align_to.column() as usize)
                        } else {
                            layout_line(align_to.row(), snapshot, style, cx)
                                .unwrap()
                                .x_for_index(align_to.column() as usize)
                        };

                    block.render(&mut BlockContext {
                        context: cx,
                        anchor_x,
                        gutter_dimensions,
                        line_height,
                        em_width,
                        block_id,
                        max_width: scroll_width.max(text_width),
                        editor_style: &self.style,
                    })
                }

                TransformBlock::ExcerptHeader {
                    buffer,
                    range,
                    starts_new_buffer,
                    ..
                } => {
                    let include_root = editor
                        .project
                        .as_ref()
                        .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
                        .unwrap_or_default();

                    let jump_handler = project::File::from_dyn(buffer.file()).map(|file| {
                        let jump_path = ProjectPath {
                            worktree_id: file.worktree_id(cx),
                            path: file.path.clone(),
                        };
                        let jump_anchor = range
                            .primary
                            .as_ref()
                            .map_or(range.context.start, |primary| primary.start);
                        let jump_position = language::ToPoint::to_point(&jump_anchor, buffer);

                        cx.listener_for(&self.editor, move |editor, _, cx| {
                            editor.jump(jump_path.clone(), jump_position, jump_anchor, cx);
                        })
                    });

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

                        v_flex()
                            .id(("path header container", block_id))
                            .size_full()
                            .justify_center()
                            .p(gpui::px(6.))
                            .child(
                                h_flex()
                                    .id("path header block")
                                    .size_full()
                                    .pl(gpui::px(12.))
                                    .pr(gpui::px(8.))
                                    .rounded_md()
                                    .shadow_md()
                                    .border()
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
                                    .when_some(jump_handler, |this, jump_handler| {
                                        this.cursor_pointer()
                                            .tooltip(|cx| {
                                                Tooltip::for_action(
                                                    "Jump to Buffer",
                                                    &OpenExcerpts,
                                                    cx,
                                                )
                                            })
                                            .on_mouse_down(MouseButton::Left, |_, cx| {
                                                cx.stop_propagation()
                                            })
                                            .on_click(jump_handler)
                                    }),
                            )
                    } else {
                        h_flex()
                            .id(("collapsed context", block_id))
                            .size_full()
                            .gap(gutter_dimensions.left_padding + gutter_dimensions.right_padding)
                            .child(
                                h_flex()
                                    .justify_end()
                                    .flex_none()
                                    .w(gutter_dimensions.width
                                        - (gutter_dimensions.left_padding
                                            + gutter_dimensions.right_padding))
                                    .h_full()
                                    .text_buffer(cx)
                                    .text_color(cx.theme().colors().editor_line_number)
                                    .child("..."),
                            )
                            .child(
                                ButtonLike::new("jump to collapsed context")
                                    .style(ButtonStyle::Transparent)
                                    .full_width()
                                    .child(
                                        div()
                                            .h_px()
                                            .w_full()
                                            .bg(cx.theme().colors().border_variant)
                                            .group_hover("", |style| {
                                                style.bg(cx.theme().colors().border)
                                            }),
                                    )
                                    .when_some(jump_handler, |this, jump_handler| {
                                        this.on_click(jump_handler).tooltip(|cx| {
                                            Tooltip::for_action("Jump to Buffer", &OpenExcerpts, cx)
                                        })
                                    }),
                            )
                    };
                    element.into_any()
                }
            };

            let size = element.measure(available_space, cx);
            (element, size)
        };

        let mut fixed_block_max_width = Pixels::ZERO;
        let mut blocks = Vec::new();
        for (row, block) in fixed_blocks {
            let available_space = size(
                AvailableSpace::MinContent,
                AvailableSpace::Definite(block.height() as f32 * line_height),
            );
            let (element, element_size) =
                render_block(block, available_space, block_id, editor, cx);
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
            };
            let width = match style {
                BlockStyle::Sticky => editor_width,
                BlockStyle::Flex => editor_width
                    .max(fixed_block_max_width)
                    .max(gutter_dimensions.width + scroll_width),
                BlockStyle::Fixed => unreachable!(),
            };
            let available_space = size(
                AvailableSpace::Definite(width),
                AvailableSpace::Definite(block.height() as f32 * line_height),
            );
            let (element, _) = render_block(block, available_space, block_id, editor, cx);
            block_id += 1;
            blocks.push(BlockLayout {
                row,
                element,
                available_space,
                style,
            });
        }
        (
            scroll_width.max(fixed_block_max_width - gutter_dimensions.width),
            blocks,
        )
    }

    fn paint_scroll_wheel_listener(
        &mut self,
        interactive_bounds: &InteractiveBounds,
        layout: &LayoutState,
        cx: &mut ElementContext,
    ) {
        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let interactive_bounds = interactive_bounds.clone();
            let mut delta = ScrollDelta::default();

            move |event: &ScrollWheelEvent, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && interactive_bounds.visibly_contains(&event.position, cx)
                {
                    delta = delta.coalesce(event.delta);
                    editor.update(cx, |editor, cx| {
                        let position = event.position;
                        let position_map: &PositionMap = &position_map;
                        let bounds = &interactive_bounds;
                        if !bounds.visibly_contains(&position, cx) {
                            return;
                        }

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

                        let scroll_position = position_map.snapshot.scroll_position();
                        let x = f32::from(
                            (scroll_position.x * max_glyph_width - delta.x) / max_glyph_width,
                        );
                        let y =
                            f32::from((scroll_position.y * line_height - delta.y) / line_height);
                        let scroll_position =
                            point(x, y).clamp(&point(0., 0.), &position_map.scroll_max);
                        editor.scroll(scroll_position, axis, cx);
                        cx.stop_propagation();
                    });
                }
            }
        });
    }

    fn paint_mouse_listeners(
        &mut self,
        bounds: Bounds<Pixels>,
        gutter_bounds: Bounds<Pixels>,
        text_bounds: Bounds<Pixels>,
        layout: &LayoutState,
        cx: &mut ElementContext,
    ) {
        let interactive_bounds = InteractiveBounds {
            bounds: bounds.intersect(&cx.content_mask().bounds),
            stacking_order: cx.stacking_order().clone(),
        };

        self.paint_scroll_wheel_listener(&interactive_bounds, layout, cx);

        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let stacking_order = cx.stacking_order().clone();
            let interactive_bounds = interactive_bounds.clone();

            move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && interactive_bounds.visibly_contains(&event.position, cx)
                {
                    match event.button {
                        MouseButton::Left => editor.update(cx, |editor, cx| {
                            Self::mouse_left_down(
                                editor,
                                event,
                                &position_map,
                                text_bounds,
                                gutter_bounds,
                                &stacking_order,
                                cx,
                            );
                        }),
                        MouseButton::Right => editor.update(cx, |editor, cx| {
                            Self::mouse_right_down(editor, event, &position_map, text_bounds, cx);
                        }),
                        _ => {}
                    };
                }
            }
        });

        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let stacking_order = cx.stacking_order().clone();
            let interactive_bounds = interactive_bounds.clone();

            move |event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        Self::mouse_up(
                            editor,
                            event,
                            &position_map,
                            text_bounds,
                            &interactive_bounds,
                            &stacking_order,
                            cx,
                        )
                    });
                }
            }
        });
        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let stacking_order = cx.stacking_order().clone();

            move |event: &MouseMoveEvent, phase, cx| {
                // if editor.has_pending_selection() && event.pressed_button == Some(MouseButton::Left) {

                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        if event.pressed_button == Some(MouseButton::Left) {
                            Self::mouse_dragged(
                                editor,
                                event,
                                &position_map,
                                text_bounds,
                                gutter_bounds,
                                &stacking_order,
                                cx,
                            )
                        }

                        if interactive_bounds.visibly_contains(&event.position, cx) {
                            Self::mouse_moved(
                                editor,
                                event,
                                &position_map,
                                text_bounds,
                                gutter_bounds,
                                &stacking_order,
                                cx,
                            )
                        }
                    });
                }
            }
        });
    }
}

#[derive(Debug)]
pub(crate) struct LineWithInvisibles {
    pub line: ShapedLine,
    invisibles: Vec<Invisible>,
}

impl LineWithInvisibles {
    fn from_chunks<'a>(
        chunks: impl Iterator<Item = HighlightedChunk<'a>>,
        text_style: &TextStyle,
        max_line_len: usize,
        max_line_count: usize,
        line_number_layouts: &[Option<ShapedLine>],
        editor_mode: EditorMode,
        cx: &WindowContext,
    ) -> Vec<Self> {
        let mut layouts = Vec::with_capacity(max_line_count);
        let mut line = String::new();
        let mut invisibles = Vec::new();
        let mut styles = Vec::new();
        let mut non_whitespace_added = false;
        let mut row = 0;
        let mut line_exceeded_max_len = false;
        let font_size = text_style.font_size.to_pixels(cx.rem_size());

        for highlighted_chunk in chunks.chain([HighlightedChunk {
            chunk: "\n",
            style: None,
            is_tab: false,
        }]) {
            for (ix, mut line_chunk) in highlighted_chunk.chunk.split('\n').enumerate() {
                if ix > 0 {
                    let shaped_line = cx
                        .text_system()
                        .shape_line(line.clone().into(), font_size, &styles)
                        .unwrap();
                    layouts.push(Self {
                        line: shaped_line,
                        invisibles: invisibles.drain(..).collect(),
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
                                    .chars()
                                    .enumerate()
                                    .filter(|(_, line_char)| {
                                        let is_whitespace = line_char.is_whitespace();
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

        layouts
    }

    fn draw(
        &self,
        layout: &LayoutState,
        row: u32,
        content_origin: gpui::Point<Pixels>,
        whitespace_setting: ShowWhitespaceSetting,
        selection_ranges: &[Range<DisplayPoint>],
        cx: &mut ElementContext,
    ) {
        let line_height = layout.position_map.line_height;
        let line_y = line_height * row as f32 - layout.position_map.scroll_position.y;

        self.line
            .paint(
                content_origin + gpui::point(-layout.position_map.scroll_position.x, line_y),
                line_height,
                cx,
            )
            .log_err();

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

    fn draw_invisibles(
        &self,
        selection_ranges: &[Range<DisplayPoint>],
        layout: &LayoutState,
        content_origin: gpui::Point<Pixels>,
        line_y: Pixels,
        row: u32,
        line_height: Pixels,
        whitespace_setting: ShowWhitespaceSetting,
        cx: &mut ElementContext,
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

            let x_offset = self.line.x_for_index(token_offset);
            let invisible_offset =
                (layout.position_map.em_width - invisible_symbol.width).max(Pixels::ZERO) / 2.0;
            let origin = content_origin
                + gpui::point(
                    x_offset + invisible_offset - layout.position_map.scroll_position.x,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Invisible {
    Tab { line_start_offset: usize },
    Whitespace { line_offset: usize },
}

impl Element for EditorElement {
    type State = ();

    fn request_layout(
        &mut self,
        _element_state: Option<Self::State>,
        cx: &mut gpui::ElementContext,
    ) -> (gpui::LayoutId, Self::State) {
        cx.with_view_id(self.editor.entity_id(), |cx| {
            self.editor.update(cx, |editor, cx| {
                editor.set_style(self.style.clone(), cx);

                let layout_id = match editor.mode {
                    EditorMode::SingleLine => {
                        let rem_size = cx.rem_size();
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        style.size.height = self.style.text.line_height_in_pixels(rem_size).into();
                        cx.with_element_context(|cx| cx.request_layout(&style, None))
                    }
                    EditorMode::AutoHeight { max_lines } => {
                        let editor_handle = cx.view().clone();
                        let max_line_number_width =
                            self.max_line_number_width(&editor.snapshot(cx), cx);
                        cx.with_element_context(|cx| {
                            cx.request_measured_layout(
                                Style::default(),
                                move |known_dimensions, _, cx| {
                                    editor_handle
                                        .update(cx, |editor, cx| {
                                            compute_auto_height_layout(
                                                editor,
                                                max_lines,
                                                max_line_number_width,
                                                known_dimensions,
                                                cx,
                                            )
                                        })
                                        .unwrap_or_default()
                                },
                            )
                        })
                    }
                    EditorMode::Full => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        style.size.height = relative(1.).into();
                        cx.with_element_context(|cx| cx.request_layout(&style, None))
                    }
                };

                (layout_id, ())
            })
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<gpui::Pixels>,
        _element_state: &mut Self::State,
        cx: &mut gpui::ElementContext,
    ) {
        let editor = self.editor.clone();

        cx.paint_view(self.editor.entity_id(), |cx| {
            cx.with_text_style(
                Some(gpui::TextStyleRefinement {
                    font_size: Some(self.style.text.font_size),
                    line_height: Some(self.style.text.line_height),
                    ..Default::default()
                }),
                |cx| {
                    let mut layout = self.compute_layout(bounds, cx);
                    let gutter_bounds = Bounds {
                        origin: bounds.origin,
                        size: layout.gutter_size,
                    };
                    let text_bounds = Bounds {
                        origin: gutter_bounds.upper_right(),
                        size: layout.text_size,
                    };

                    let focus_handle = editor.focus_handle(cx);
                    let key_context = self.editor.read(cx).key_context(cx);
                    cx.with_key_dispatch(Some(key_context), Some(focus_handle.clone()), |_, cx| {
                        self.register_actions(cx);

                        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                            self.register_key_listeners(cx, text_bounds, &layout);
                            cx.handle_input(
                                &focus_handle,
                                ElementInputHandler::new(bounds, self.editor.clone()),
                            );

                            self.paint_background(gutter_bounds, text_bounds, &layout, cx);
                            if layout.gutter_size.width > Pixels::ZERO {
                                self.paint_gutter(gutter_bounds, &mut layout, cx);
                            }
                            self.paint_text(text_bounds, &mut layout, cx);

                            cx.with_z_index(0, |cx| {
                                self.paint_mouse_listeners(
                                    bounds,
                                    gutter_bounds,
                                    text_bounds,
                                    &layout,
                                    cx,
                                );
                            });
                            if !layout.blocks.is_empty() {
                                cx.with_z_index(0, |cx| {
                                    cx.with_element_id(Some("editor_blocks"), |cx| {
                                        self.paint_blocks(bounds, &mut layout, cx);
                                    });
                                })
                            }

                            cx.with_z_index(1, |cx| {
                                self.paint_overlays(text_bounds, &mut layout, cx);
                            });

                            cx.with_z_index(2, |cx| self.paint_scrollbar(bounds, &mut layout, cx));
                        });
                    })
                },
            )
        })
    }
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn element_id(&self) -> Option<gpui::ElementId> {
        self.editor.element_id()
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

type BufferRow = u32;

pub struct LayoutState {
    position_map: Arc<PositionMap>,
    gutter_size: Size<Pixels>,
    gutter_dimensions: GutterDimensions,
    text_size: gpui::Size<Pixels>,
    mode: EditorMode,
    wrap_guides: SmallVec<[(Pixels, bool); 2]>,
    visible_anchor_range: Range<Anchor>,
    visible_display_row_range: Range<u32>,
    active_rows: BTreeMap<u32, bool>,
    highlighted_rows: Option<Range<u32>>,
    line_numbers: Vec<Option<ShapedLine>>,
    display_hunks: Vec<DisplayDiffHunk>,
    blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    redacted_ranges: Vec<Range<DisplayPoint>>,
    selections: Vec<(PlayerColor, Vec<SelectionLayout>)>,
    scrollbar_row_range: Range<f32>,
    show_scrollbars: bool,
    is_singleton: bool,
    max_row: u32,
    context_menu: Option<(DisplayPoint, AnyElement)>,
    code_actions_indicator: Option<CodeActionsIndicator>,
    hover_popovers: Option<(DisplayPoint, Vec<AnyElement>)>,
    fold_indicators: Vec<Option<IconButton>>,
    tab_invisible: ShapedLine,
    space_invisible: ShapedLine,
}

impl LayoutState {
    fn line_end_overshoot(&self) -> Pixels {
        0.15 * self.position_map.line_height
    }
}

struct CodeActionsIndicator {
    row: u32,
    button: IconButton,
}

struct PositionMap {
    size: Size<Pixels>,
    line_height: Pixels,
    scroll_position: gpui::Point<Pixels>,
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
        let row = (f32::from(y / self.line_height) + scroll_position.y) as u32;

        let (column, x_overshoot_after_line_end) = if let Some(line) = self
            .line_layouts
            .get(row as usize - scroll_position.y as usize)
            .map(|&LineWithInvisibles { ref line, .. }| line)
        {
            if let Some(ix) = line.index_for_x(x) {
                (ix as u32, px(0.))
            } else {
                (line.len as u32, px(0.).max(x - line.width))
            }
        } else {
            (0, x)
        };

        let mut exact_unclipped = DisplayPoint::new(row, column);
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
    row: u32,
    element: AnyElement,
    available_space: Size<AvailableSpace>,
    style: BlockStyle,
}

fn layout_line(
    row: u32,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    cx: &WindowContext,
) -> Result<ShapedLine> {
    let mut line = snapshot.line(row);

    if line.len() > MAX_LINE_LEN {
        let mut len = MAX_LINE_LEN;
        while !line.is_char_boundary(len) {
            len -= 1;
        }

        line.truncate(len);
    }

    cx.text_system().shape_line(
        line.into(),
        style.text.font_size.to_pixels(cx.rem_size()),
        &[TextRun {
            len: snapshot.line_len(row) as usize,
            font: style.text.font(),
            color: Hsla::default(),
            background_color: None,
            underline: None,
            strikethrough: None,
        }],
    )
}

#[derive(Debug)]
pub struct Cursor {
    origin: gpui::Point<Pixels>,
    block_width: Pixels,
    line_height: Pixels,
    color: Hsla,
    shape: CursorShape,
    block_text: Option<ShapedLine>,
    cursor_name: Option<CursorName>,
}

#[derive(Debug)]
pub struct CursorName {
    string: SharedString,
    color: Hsla,
    is_top_row: bool,
    z_index: u16,
}

impl Cursor {
    pub fn new(
        origin: gpui::Point<Pixels>,
        block_width: Pixels,
        line_height: Pixels,
        color: Hsla,
        shape: CursorShape,
        block_text: Option<ShapedLine>,
        cursor_name: Option<CursorName>,
    ) -> Cursor {
        Cursor {
            origin,
            block_width,
            line_height,
            color,
            shape,
            block_text,
            cursor_name,
        }
    }

    pub fn bounding_rect(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin + origin,
            size: size(self.block_width, self.line_height),
        }
    }

    pub fn paint(&self, origin: gpui::Point<Pixels>, cx: &mut ElementContext) {
        let bounds = match self.shape {
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
        };

        //Draw background or border quad
        let cursor = if matches!(self.shape, CursorShape::Hollow) {
            outline(bounds, self.color)
        } else {
            fill(bounds, self.color)
        };

        if let Some(name) = &self.cursor_name {
            let text_size = self.line_height / 1.5;

            let name_origin = if name.is_top_row {
                point(bounds.right() - px(1.), bounds.top())
            } else {
                point(bounds.left(), bounds.top() - text_size / 2. - px(1.))
            };
            cx.with_z_index(name.z_index, |cx| {
                div()
                    .bg(self.color)
                    .text_size(text_size)
                    .px_0p5()
                    .line_height(text_size + px(2.))
                    .text_color(name.color)
                    .child(name.string.clone())
                    .into_any_element()
                    .draw(
                        name_origin,
                        size(AvailableSpace::MinContent, AvailableSpace::MinContent),
                        cx,
                    )
            })
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
    pub fn paint(&self, bounds: Bounds<Pixels>, cx: &mut ElementContext) {
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
        cx: &mut ElementContext,
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
    use gpui::TestAppContext;
    use language::language_settings;
    use log::info;
    use std::{num::NonZeroU32, sync::Arc};
    use util::test::sample_text;

    #[gpui::test]
    fn test_shape_line_numbers(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::Full, buffer, None, cx)
        });

        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let element = EditorElement::new(&editor, style);

        let layouts = window
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
                element
                    .shape_line_numbers(
                        0..6,
                        &Default::default(),
                        DisplayPoint::new(0, 0),
                        false,
                        &snapshot,
                        cx,
                    )
                    .0
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows = window
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
                element.calculate_relative_line_numbers(&snapshot, &(0..6), Some(3))
            })
            .unwrap();
        assert_eq!(relative_rows[&0], 3);
        assert_eq!(relative_rows[&1], 2);
        assert_eq!(relative_rows[&2], 1);
        // current line has no relative number
        assert_eq!(relative_rows[&4], 1);
        assert_eq!(relative_rows[&5], 2);

        // works if cursor is before screen
        let relative_rows = window
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);

                element.calculate_relative_line_numbers(&snapshot, &(3..6), Some(1))
            })
            .unwrap();
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&3], 2);
        assert_eq!(relative_rows[&4], 3);
        assert_eq!(relative_rows[&5], 4);

        // works if cursor is after screen
        let relative_rows = window
            .update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);

                element.calculate_relative_line_numbers(&snapshot, &(0..3), Some(6))
            })
            .unwrap();
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&0], 5);
        assert_eq!(relative_rows[&1], 4);
        assert_eq!(relative_rows[&2], 3);
    }

    #[gpui::test]
    async fn test_vim_visual_selections(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&(sample_text(6, 6, 'a') + "\n"), cx);
            Editor::new(EditorMode::Full, buffer, None, cx)
        });
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let mut element = EditorElement::new(&editor, style);

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
        let state = cx
            .update_window(window.into(), |view, cx| {
                cx.with_element_context(|cx| {
                    cx.with_view_id(view.entity_id(), |cx| {
                        element.compute_layout(
                            Bounds {
                                origin: point(px(500.), px(500.)),
                                size: size(px(500.), px(500.)),
                            },
                            cx,
                        )
                    })
                })
            })
            .unwrap();

        assert_eq!(state.selections.len(), 1);
        let local_selections = &state.selections[0].1;
        assert_eq!(local_selections.len(), 3);
        // moves cursor back one line
        assert_eq!(local_selections[0].head, DisplayPoint::new(0, 6));
        assert_eq!(
            local_selections[0].range,
            DisplayPoint::new(0, 0)..DisplayPoint::new(1, 0)
        );

        // moves cursor back one column
        assert_eq!(
            local_selections[1].range,
            DisplayPoint::new(3, 2)..DisplayPoint::new(3, 3)
        );
        assert_eq!(local_selections[1].head, DisplayPoint::new(3, 2));

        // leaves cursor on the max point
        assert_eq!(
            local_selections[2].range,
            DisplayPoint::new(5, 6)..DisplayPoint::new(6, 0)
        );
        assert_eq!(local_selections[2].head, DisplayPoint::new(6, 0));

        // active lines does not include 1 (even though the range of the selection does)
        assert_eq!(
            state.active_rows.keys().cloned().collect::<Vec<u32>>(),
            vec![0, 3, 5, 6]
        );

        // multi-buffer support
        // in DisplayPoint coordinates, this is what we're dealing with:
        //  0: [[file
        //  1:   header]]
        //  2: aaaaaa
        //  3: bbbbbb
        //  4: cccccc
        //  5:
        //  6: ...
        //  7: ffffff
        //  8: gggggg
        //  9: hhhhhh
        // 10:
        // 11: [[file
        // 12:   header]]
        // 13: bbbbbb
        // 14: cccccc
        // 15: dddddd
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
            Editor::new(EditorMode::Full, buffer, None, cx)
        });
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let mut element = EditorElement::new(&editor, style);
        let _state = window.update(cx, |editor, cx| {
            editor.cursor_shape = CursorShape::Block;
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(4, 0)..DisplayPoint::new(7, 0),
                    DisplayPoint::new(10, 0)..DisplayPoint::new(13, 0),
                ]);
            });
        });

        let state = cx
            .update_window(window.into(), |view, cx| {
                cx.with_element_context(|cx| {
                    cx.with_view_id(view.entity_id(), |cx| {
                        element.compute_layout(
                            Bounds {
                                origin: point(px(500.), px(500.)),
                                size: size(px(500.), px(500.)),
                            },
                            cx,
                        )
                    })
                })
            })
            .unwrap();
        assert_eq!(state.selections.len(), 1);
        let local_selections = &state.selections[0].1;
        assert_eq!(local_selections.len(), 2);

        // moves cursor on excerpt boundary back a line
        // and doesn't allow selection to bleed through
        assert_eq!(
            local_selections[0].range,
            DisplayPoint::new(4, 0)..DisplayPoint::new(6, 0)
        );
        assert_eq!(local_selections[0].head, DisplayPoint::new(5, 0));
        // moves cursor on buffer boundary back two lines
        // and doesn't allow selection to bleed through
        assert_eq!(
            local_selections[1].range,
            DisplayPoint::new(10, 0)..DisplayPoint::new(11, 0)
        );
        assert_eq!(local_selections[1].head, DisplayPoint::new(10, 0));
    }

    #[gpui::test]
    fn test_layout_with_placeholder_text_and_blocks(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let window = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::Full, buffer, None, cx)
        });
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
                        render: Arc::new(|_| div().into_any()),
                    }],
                    None,
                    cx,
                );

                // Blur the editor so that it displays placeholder text.
                cx.blur();
            })
            .unwrap();

        let mut element = EditorElement::new(&editor, style);
        let state = cx
            .update_window(window.into(), |view, cx| {
                cx.with_element_context(|cx| {
                    cx.with_view_id(view.entity_id(), |cx| {
                        element.compute_layout(
                            Bounds {
                                origin: point(px(500.), px(500.)),
                                size: size(px(500.), px(500.)),
                            },
                            cx,
                        )
                    })
                })
            })
            .unwrap();
        let size = state.position_map.size;

        assert_eq!(state.position_map.line_layouts.len(), 4);
        assert_eq!(
            state
                .line_numbers
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>(),
            &[false, false, false, true]
        );

        // Don't panic.
        let bounds = Bounds::<Pixels>::new(Default::default(), size);
        cx.update_window(window.into(), |_, cx| {
            cx.with_element_context(|cx| element.paint(bounds, &mut (), cx))
        })
        .unwrap()
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
            Editor::new(editor_mode, buffer, None, cx)
        });
        let editor = window.root(cx).unwrap();
        let style = cx.update(|cx| editor.read(cx).style().unwrap().clone());
        let mut element = EditorElement::new(&editor, style);
        window
            .update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_wrap_width(Some(editor_width), cx);
            })
            .unwrap();
        let layout_state = cx
            .update_window(window.into(), |_, cx| {
                cx.with_element_context(|cx| {
                    element.compute_layout(
                        Bounds {
                            origin: point(px(500.), px(500.)),
                            size: size(px(500.), px(500.)),
                        },
                        cx,
                    )
                })
            })
            .unwrap();

        layout_state
            .position_map
            .line_layouts
            .iter()
            .map(|line_with_invisibles| &line_with_invisibles.invisibles)
            .flatten()
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
    cx: &mut ViewContext<Editor>,
) -> Option<Size<Pixels>> {
    let width = known_dimensions.width?;
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

    editor.gutter_width = gutter_dimensions.width;
    let text_width = width - gutter_dimensions.width;
    let overscroll = size(em_width, px(0.));

    let editor_width = text_width - gutter_dimensions.margin - overscroll.width - em_width;
    if editor.set_wrap_width(Some(editor_width), cx) {
        snapshot = editor.snapshot(cx);
    }

    let scroll_height = Pixels::from(snapshot.max_point().row() + 1) * line_height;
    let height = scroll_height
        .max(line_height)
        .min(line_height * max_lines as f32);

    Some(size(width, height))
}
