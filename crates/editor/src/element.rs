use crate::{
    display_map::{
        BlockContext, BlockStyle, DisplaySnapshot, FoldStatus, HighlightedChunk, ToDisplayPoint,
        TransformBlock,
    },
    editor_settings::{DoubleClickInMultibuffer, MultiCursorModifier, ShowScrollbar},
    git::{blame::GitBlame, diff_hunk_to_display, DisplayDiffHunk},
    hover_popover::{
        self, hover_at, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT,
    },
    items::BufferSearchHighlights,
    mouse_context_menu::{self, MouseContextMenu},
    scroll::scroll_amount::ScrollAmount,
    CursorShape, DisplayPoint, DocumentHighlightRead, DocumentHighlightWrite, Editor, EditorMode,
    EditorSettings, EditorSnapshot, EditorStyle, GutterDimensions, HalfPageDown, HalfPageUp,
    HoveredCursor, LineDown, LineUp, OpenExcerpts, PageDown, PageUp, Point, SelectPhase, Selection,
    SoftWrap, ToPoint, CURSORS_VISIBLE_FOR, MAX_LINE_LEN,
};
use anyhow::Result;
use collections::{BTreeMap, HashMap};
use git::{blame::BlameEntry, diff::DiffHunkStatus, Oid};
use gpui::{
    anchored, deferred, div, fill, outline, point, px, quad, relative, size, svg,
    transparent_black, Action, AnchorCorner, AnyElement, AnyView, AvailableSpace, Bounds,
    ClipboardItem, ContentMask, Corners, CursorStyle, DispatchPhase, Edges, Element,
    ElementContext, ElementInputHandler, Entity, Hitbox, Hsla, InteractiveElement, IntoElement,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, Pixels, ScrollDelta, ScrollWheelEvent, ShapedLine, SharedString, Size, Stateful,
    StatefulInteractiveElement, Style, Styled, TextRun, TextStyle, TextStyleRefinement, View,
    ViewContext, WindowContext,
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
    iter, mem,
    ops::Range,
    sync::Arc,
};
use sum_tree::Bias;
use theme::{ActiveTheme, PlayerColor};
use ui::{h_flex, ButtonLike, ButtonStyle, ContextMenu, Tooltip};
use ui::{prelude::*, tooltip_container};
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
        register_action(view, cx, Editor::toggle_inlay_hints);
        register_action(view, cx, hover_popover::hover);
        register_action(view, cx, Editor::reveal_in_finder);
        register_action(view, cx, Editor::copy_path);
        register_action(view, cx, Editor::copy_relative_path);
        register_action(view, cx, Editor::copy_highlight_json);
        register_action(view, cx, Editor::copy_permalink_to_line);
        register_action(view, cx, Editor::open_permalink_to_line);
        register_action(view, cx, Editor::toggle_git_blame);
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
        register_action(view, cx, Editor::revert_selected_hunks);
    }

    fn register_key_listeners(&self, cx: &mut ElementContext, layout: &EditorLayout) {
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

        if gutter_hitbox.is_hovered(cx) {
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
            let multi_cursor_setting = EditorSettings::get_global(cx).multi_cursor_modifier;
            let multi_cursor_modifier = match multi_cursor_setting {
                MultiCursorModifier::Alt => modifiers.alt,
                MultiCursorModifier::Cmd => modifiers.command,
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
            MultiCursorModifier::Alt => event.modifiers.command,
            MultiCursorModifier::Cmd => event.modifiers.alt,
        };

        if !pending_nonempty_selections && multi_cursor_modifier && text_hitbox.is_hovered(cx) {
            let point = position_map.point_for_position(text_hitbox.bounds, event.position);
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
                hover_at(editor, Some(point), cx);
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
        start_row: u32,
        end_row: u32,
        cx: &mut ElementContext,
    ) -> (
        Vec<(PlayerColor, Vec<SelectionLayout>)>,
        BTreeMap<u32, bool>,
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

    #[allow(clippy::too_many_arguments)]
    fn layout_folds(
        &self,
        snapshot: &EditorSnapshot,
        content_origin: gpui::Point<Pixels>,
        visible_anchor_range: Range<Anchor>,
        visible_display_row_range: Range<u32>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        line_layouts: &[LineWithInvisibles],
        cx: &mut ElementContext,
    ) -> Vec<FoldLayout> {
        snapshot
            .folds_in_range(visible_anchor_range.clone())
            .filter_map(|fold| {
                let fold_range = fold.range.clone();
                let display_range = fold.range.start.to_display_point(&snapshot)
                    ..fold.range.end.to_display_point(&snapshot);
                debug_assert_eq!(display_range.start.row(), display_range.end.row());
                let row = display_range.start.row();
                debug_assert!(row < visible_display_row_range.end);
                let line_layout = line_layouts
                    .get((row - visible_display_row_range.start) as usize)
                    .map(|l| &l.line)?;

                let start_x = content_origin.x
                    + line_layout.x_for_index(display_range.start.column() as usize)
                    - scroll_pixel_position.x;
                let start_y = content_origin.y + row as f32 * line_height - scroll_pixel_position.y;
                let end_x = content_origin.x
                    + line_layout.x_for_index(display_range.end.column() as usize)
                    - scroll_pixel_position.x;

                let fold_bounds = Bounds {
                    origin: point(start_x, start_y),
                    size: size(end_x - start_x, line_height),
                };

                let mut hover_element = div()
                    .id(fold.id)
                    .size_full()
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                    .on_click(
                        cx.listener_for(&self.editor, move |editor: &mut Editor, _, cx| {
                            editor.unfold_ranges(
                                [fold_range.start..fold_range.end],
                                true,
                                false,
                                cx,
                            );
                            cx.stop_propagation();
                        }),
                    )
                    .into_any();
                hover_element.layout(fold_bounds.origin, fold_bounds.size.into(), cx);
                Some(FoldLayout {
                    display_range,
                    hover_element,
                })
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_cursors(
        &self,
        snapshot: &EditorSnapshot,
        selections: &[(PlayerColor, Vec<SelectionLayout>)],
        visible_display_row_range: Range<u32>,
        line_layouts: &[LineWithInvisibles],
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_height: Pixels,
        em_width: Pixels,
        cx: &mut ElementContext,
    ) -> Vec<CursorLayout> {
        self.editor.update(cx, |editor, cx| {
            let mut cursors = Vec::new();
            for (player_color, selections) in selections {
                for selection in selections {
                    let cursor_position = selection.head;
                    if (selection.is_local && !editor.show_local_cursors(cx))
                        || !visible_display_row_range.contains(&cursor_position.row())
                    {
                        continue;
                    }

                    let cursor_row_layout = &line_layouts
                        [(cursor_position.row() - visible_display_row_range.start) as usize]
                        .line;
                    let cursor_column = cursor_position.column() as usize;

                    let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);
                    let mut block_width =
                        cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
                    if block_width == Pixels::ZERO {
                        block_width = em_width;
                    }
                    let block_text = if let CursorShape::Block = selection.cursor_shape {
                        snapshot
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

                    let x = cursor_character_x - scroll_pixel_position.x;
                    let y = (cursor_position.row() as f32 - scroll_pixel_position.y / line_height)
                        * line_height;
                    if selection.is_newest {
                        editor.pixel_position_of_newest_cursor = Some(point(
                            text_hitbox.origin.x + x + block_width / 2.,
                            text_hitbox.origin.y + y + line_height / 2.,
                        ))
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
                        is_top_row: cursor_position.row() == 0,
                    });
                    cx.with_element_context(|cx| cursor.layout(content_origin, cursor_name, cx));
                    cursors.push(cursor);
                }
            }
            cursors
        })
    }

    fn layout_scrollbar(
        &self,
        snapshot: &EditorSnapshot,
        bounds: Bounds<Pixels>,
        scroll_position: gpui::Point<f32>,
        line_height: Pixels,
        height_in_lines: f32,
        cx: &mut ElementContext,
    ) -> Option<ScrollbarLayout> {
        let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
        let show_scrollbars = match scrollbar_settings.show {
            ShowScrollbar::Auto => {
                let editor = self.editor.read(cx);
                let is_singleton = editor.is_singleton(cx);
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
            ShowScrollbar::System => self.editor.read(cx).scroll_manager.scrollbars_visible(),
            ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        };
        if snapshot.mode != EditorMode::Full {
            return None;
        }

        let visible_row_range = scroll_position.y..scroll_position.y + height_in_lines;

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

        let scroll_height = snapshot.max_point().row() as f32 + height_in_lines;
        let mut height = bounds.size.height;
        let mut first_row_y_offset = px(0.0);

        // Impose a minimum height on the scrollbar thumb
        let row_height = height / scroll_height;
        let min_thumb_height = line_height;
        let thumb_height = height_in_lines * row_height;
        if thumb_height < min_thumb_height {
            first_row_y_offset = (min_thumb_height - thumb_height) / 2.0;
            height -= min_thumb_height - thumb_height;
        }

        Some(ScrollbarLayout {
            hitbox: cx.insert_hitbox(track_bounds, false),
            visible_row_range,
            height,
            scroll_height,
            first_row_y_offset,
            row_height,
            visible: show_scrollbars,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_gutter_fold_indicators(
        &self,
        fold_statuses: Vec<Option<(FoldStatus, u32, bool)>>,
        line_height: Pixels,
        gutter_dimensions: &GutterDimensions,
        gutter_settings: crate::editor_settings::Gutter,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_hitbox: &Hitbox,
        cx: &mut ElementContext,
    ) -> Vec<Option<AnyElement>> {
        let mut indicators = self.editor.update(cx, |editor, cx| {
            editor.render_fold_indicators(
                fold_statuses,
                &self.style,
                editor.gutter_hovered,
                line_height,
                gutter_dimensions.margin,
                cx,
            )
        });

        for (ix, fold_indicator) in indicators.iter_mut().enumerate() {
            if let Some(fold_indicator) = fold_indicator {
                debug_assert!(gutter_settings.folds);
                let available_space = size(
                    AvailableSpace::MinContent,
                    AvailableSpace::Definite(line_height * 0.55),
                );
                let fold_indicator_size = fold_indicator.measure(available_space, cx);

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
                fold_indicator.layout(origin, available_space, cx);
            }
        }

        indicators
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

    #[allow(clippy::too_many_arguments)]
    fn layout_blame_entries(
        &self,
        buffer_rows: impl Iterator<Item = Option<u32>>,
        em_width: Pixels,
        scroll_position: gpui::Point<f32>,
        line_height: Pixels,
        gutter_hitbox: &Hitbox,
        max_width: Option<Pixels>,
        cx: &mut ElementContext,
    ) -> Option<Vec<AnyElement>> {
        let Some(blame) = self.editor.read(cx).blame.as_ref().cloned() else {
            return None;
        };

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
        let text_style = &self.style.text;

        let shaped_lines = blamed_rows
            .into_iter()
            .enumerate()
            .flat_map(|(ix, blame_entry)| {
                if let Some(blame_entry) = blame_entry {
                    let mut element = render_blame_entry(
                        ix,
                        &blame,
                        blame_entry,
                        text_style,
                        &mut last_used_color,
                        self.editor.clone(),
                        cx,
                    );

                    let start_y = ix as f32 * line_height - (scroll_top % line_height);
                    let absolute_offset = gutter_hitbox.origin + point(start_x, start_y);

                    element.layout(absolute_offset, size(width, AvailableSpace::MinContent), cx);

                    Some(element)
                } else {
                    None
                }
            })
            .collect();

        Some(shaped_lines)
    }

    fn layout_code_actions_indicator(
        &self,
        line_height: Pixels,
        newest_selection_head: DisplayPoint,
        scroll_pixel_position: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        cx: &mut ElementContext,
    ) -> Option<AnyElement> {
        let mut active = false;
        let mut button = None;
        self.editor.update(cx, |editor, cx| {
            active = matches!(
                editor.context_menu.read().as_ref(),
                Some(crate::ContextMenu::CodeActions(_))
            );
            button = editor.render_code_actions_indicator(&self.style, active, cx);
        });

        let mut button = button?.into_any_element();
        let available_space = size(
            AvailableSpace::MinContent,
            AvailableSpace::Definite(line_height),
        );
        let indicator_size = button.measure(available_space, cx);

        let blame_width = gutter_dimensions
            .git_blame_entries_width
            .unwrap_or(Pixels::ZERO);

        let mut x = blame_width;
        let available_width = gutter_dimensions.margin + gutter_dimensions.left_padding
            - indicator_size.width
            - blame_width;
        x += available_width / 2.;

        let mut y = newest_selection_head.row() as f32 * line_height - scroll_pixel_position.y;
        y += (line_height - indicator_size.height) / 2.;

        button.layout(gutter_hitbox.origin + point(x, y), available_space, cx);
        Some(button)
    }

    fn calculate_relative_line_numbers(
        &self,
        buffer_rows: Vec<Option<u32>>,
        rows: &Range<u32>,
        relative_to: Option<u32>,
    ) -> HashMap<u32, u32> {
        let mut relative_rows: HashMap<u32, u32> = Default::default();
        let Some(relative_to) = relative_to else {
            return relative_rows;
        };

        let start = rows.start.min(relative_to);

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

    fn layout_line_numbers(
        &self,
        rows: Range<u32>,
        buffer_rows: impl Iterator<Item = Option<u32>>,
        active_rows: &BTreeMap<u32, bool>,
        newest_selection_head: Option<DisplayPoint>,
        snapshot: &EditorSnapshot,
        cx: &ElementContext,
    ) -> (
        Vec<Option<ShapedLine>>,
        Vec<Option<(FoldStatus, BufferRow, bool)>>,
    ) {
        let editor = self.editor.read(cx);
        let is_singleton = editor.is_singleton(cx);
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

        let buffer_rows = buffer_rows.collect::<Vec<_>>();
        let relative_rows =
            self.calculate_relative_line_numbers(buffer_rows.clone(), &rows, relative_to);

        for (ix, row) in buffer_rows.into_iter().enumerate() {
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
        cx: &ElementContext,
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
                rows.len(),
                line_number_layouts,
                snapshot.mode,
                cx,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn build_blocks(
        &self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        scroll_width: &mut Pixels,
        gutter_dimensions: &GutterDimensions,
        em_width: Pixels,
        text_x: Pixels,
        line_height: Pixels,
        line_layouts: &[LineWithInvisibles],
        cx: &mut ElementContext,
    ) -> Vec<BlockLayout> {
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
                            block_row_start: u32,
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
                            layout_line(align_to.row(), snapshot, &self.style, cx)
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
                        max_width: text_hitbox.size.width.max(*scroll_width),
                        editor_style: &self.style,
                    })
                }

                TransformBlock::ExcerptHeader {
                    buffer,
                    range,
                    starts_new_buffer,
                    height,
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
                            block_row_start + *height as u32 + offset_from_excerpt_start
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
                    } else {
                        v_flex()
                            .id(("collapsed context", block_id))
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
                                    .w(
                                        gutter_dimensions.width - (gutter_dimensions.left_padding), // + gutter_dimensions.right_padding)
                                    )
                                    .h_full()
                                    .child(
                                        ButtonLike::new("jump-icon")
                                            .style(ButtonStyle::Transparent)
                                            .child(
                                                svg()
                                                    .path(IconName::ArrowUpRight.path())
                                                    .size(IconSize::XSmall.rems())
                                                    .text_color(cx.theme().colors().border)
                                                    .group_hover("excerpt-jump-action", |style| {
                                                        style.text_color(
                                                            cx.theme().colors().editor_line_number,
                                                        )
                                                    }),
                                            )
                                            .when_some(jump_data.clone(), |this, jump_data| {
                                                this.on_click(cx.listener_for(&self.editor, {
                                                    let path = jump_data.path.clone();
                                                    move |editor, _, cx| {
                                                        editor.jump(
                                                            path.clone(),
                                                            jump_data.position,
                                                            jump_data.anchor,
                                                            jump_data.line_offset_from_top,
                                                            cx,
                                                        );
                                                    }
                                                }))
                                                .tooltip({
                                                    move |cx| {
                                                        Tooltip::for_action(
                                                            format!(
                                                                "Jump to {}:L{}",
                                                                jump_data.path.path.display(),
                                                                jump_data.position.row + 1
                                                            ),
                                                            &OpenExcerpts,
                                                            cx,
                                                        )
                                                    }
                                                })
                                            }),
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
        cx: &mut ElementContext,
    ) {
        for block in blocks {
            let mut origin = hitbox.origin
                + point(
                    Pixels::ZERO,
                    block.row as f32 * line_height - scroll_pixel_position.y,
                );
            if !matches!(block.style, BlockStyle::Sticky) {
                origin += point(-scroll_pixel_position.x, Pixels::ZERO);
            }
            block.element.layout(origin, block.available_space, cx);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_context_menu(
        &self,
        line_height: Pixels,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        content_origin: gpui::Point<Pixels>,
        start_row: u32,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_layouts: &[LineWithInvisibles],
        newest_selection_head: DisplayPoint,
        cx: &mut ElementContext,
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
        let context_menu_size = context_menu.measure(available_space, cx);

        let cursor_row_layout = &line_layouts[(position.row() - start_row) as usize].line;
        let x = cursor_row_layout.x_for_index(position.column() as usize) - scroll_pixel_position.x;
        let y = (position.row() + 1) as f32 * line_height - scroll_pixel_position.y;
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

    fn layout_mouse_context_menu(&self, cx: &mut ElementContext) -> Option<AnyElement> {
        let mouse_context_menu = self.editor.read(cx).mouse_context_menu.as_ref()?;
        let mut element = deferred(
            anchored()
                .position(mouse_context_menu.position)
                .child(mouse_context_menu.context_menu.clone())
                .anchor(AnchorCorner::TopLeft)
                .snap_to_window(),
        )
        .into_any();

        element.layout(gpui::Point::default(), AvailableSpace::min_size(), cx);
        Some(element)
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_hover_popovers(
        &self,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        visible_display_row_range: Range<u32>,
        content_origin: gpui::Point<Pixels>,
        scroll_pixel_position: gpui::Point<Pixels>,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        em_width: Pixels,
        cx: &mut ElementContext,
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
            &line_layouts[(position.row() - visible_display_row_range.start) as usize].line;

        // Compute Hovered Point
        let x =
            hovered_row_layout.x_for_index(position.column() as usize) - scroll_pixel_position.x;
        let y = position.row() as f32 * line_height - scroll_pixel_position.y;
        let hovered_point = content_origin + point(x, y);

        let mut overall_height = Pixels::ZERO;
        let mut measured_hover_popovers = Vec::new();
        for mut hover_popover in hover_popovers {
            let size = hover_popover.measure(available_space, cx);
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

        fn draw_occluder(width: Pixels, origin: gpui::Point<Pixels>, cx: &mut ElementContext) {
            let mut occlusion = div()
                .size_full()
                .occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .into_any_element();
            occlusion.measure(size(width, HOVER_POPOVER_GAP).into(), cx);
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

    fn paint_background(&self, layout: &EditorLayout, cx: &mut ElementContext) {
        cx.paint_layer(layout.hitbox.bounds, |cx| {
            let scroll_top = layout.position_map.snapshot.scroll_position().y;
            let gutter_bg = cx.theme().colors().editor_gutter_background;
            cx.paint_quad(fill(layout.gutter_hitbox.bounds, gutter_bg));
            cx.paint_quad(fill(layout.text_hitbox.bounds, self.style.background));

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
                            layout.hitbox.origin.x,
                            layout.hitbox.origin.y
                                + (*start_row as f32 - scroll_top)
                                    * layout.position_map.line_height,
                        );
                        let size = size(
                            layout.hitbox.size.width,
                            layout.position_map.line_height * (end_row - start_row + 1) as f32,
                        );
                        let active_line_bg = cx.theme().colors().editor_active_line_background;
                        cx.paint_quad(fill(Bounds { origin, size }, active_line_bg));
                    }
                }

                let mut paint_highlight =
                    |highlight_row_start: u32, highlight_row_end: u32, color| {
                        let origin = point(
                            layout.hitbox.origin.x,
                            layout.hitbox.origin.y
                                + (highlight_row_start as f32 - scroll_top)
                                    * layout.position_map.line_height,
                        );
                        let size = size(
                            layout.hitbox.size.width,
                            layout.position_map.line_height
                                * (highlight_row_end + 1 - highlight_row_start) as f32,
                        );
                        cx.paint_quad(fill(Bounds { origin, size }, color));
                    };

                let mut last_row = None;
                let mut highlight_row_start = 0u32;
                let mut highlight_row_end = 0u32;
                for (&row, &color) in &layout.highlighted_rows {
                    let paint = last_row.map_or(false, |(last_row, last_color)| {
                        last_color != color || last_row + 1 < row
                    });

                    if paint {
                        let paint_range_is_unfinished = highlight_row_end == 0;
                        if paint_range_is_unfinished {
                            highlight_row_end = row;
                            last_row = None;
                        }
                        paint_highlight(highlight_row_start, highlight_row_end, color);
                        highlight_row_start = 0;
                        highlight_row_end = 0;
                        if !paint_range_is_unfinished {
                            highlight_row_start = row;
                            last_row = Some((row, color));
                        }
                    } else {
                        if last_row.is_none() {
                            highlight_row_start = row;
                        } else {
                            highlight_row_end = row;
                        }
                        last_row = Some((row, color));
                    }
                }
                if let Some((row, hsla)) = last_row {
                    highlight_row_end = row;
                    paint_highlight(highlight_row_start, highlight_row_end, hsla);
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

    fn paint_gutter(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        cx.set_cursor_style(CursorStyle::Arrow, &layout.gutter_hitbox);

        let show_git_gutter = matches!(
            ProjectSettings::get_global(cx).git.git_gutter,
            Some(GitGutterSetting::TrackedFiles)
        );

        if show_git_gutter {
            Self::paint_diff_hunks(layout, cx);
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
            cx.with_element_id(Some("gutter_fold_indicators"), |cx| {
                for fold_indicator in layout.fold_indicators.iter_mut().flatten() {
                    fold_indicator.paint(cx);
                }
            });

            if let Some(indicator) = layout.code_actions_indicator.as_mut() {
                indicator.paint(cx);
            }
        })
    }

    fn paint_diff_hunks(layout: &EditorLayout, cx: &mut ElementContext) {
        if layout.display_hunks.is_empty() {
            return;
        }

        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y * line_height;

        cx.paint_layer(layout.gutter_hitbox.bounds, |cx| {
            for hunk in &layout.display_hunks {
                let (display_row_range, status) = match hunk {
                    //TODO: This rendering is entirely a horrible hack
                    &DisplayDiffHunk::Folded { display_row: row } => {
                        let start_y = row as f32 * line_height - scroll_top;
                        let end_y = start_y + line_height;

                        let width = 0.275 * line_height;
                        let highlight_origin = layout.gutter_hitbox.origin + point(-width, start_y);
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
                        let highlight_origin = layout.gutter_hitbox.origin + point(-width, start_y);
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
                let highlight_origin = layout.gutter_hitbox.origin + point(-width, start_y);
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
        })
    }

    fn paint_blamed_display_rows(&self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        let Some(blamed_display_rows) = layout.blamed_display_rows.take() else {
            return;
        };

        cx.paint_layer(layout.gutter_hitbox.bounds, |cx| {
            for mut blame_element in blamed_display_rows.into_iter() {
                blame_element.paint(cx);
            }
        })
    }

    fn paint_text(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
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

                cx.with_element_id(Some("folds"), |cx| self.paint_folds(layout, cx));
                let invisible_display_ranges = self.paint_highlights(layout, cx);
                self.paint_lines(&invisible_display_ranges, layout, cx);
                self.paint_redactions(layout, cx);
                self.paint_cursors(layout, cx);
            },
        )
    }

    fn paint_highlights(
        &mut self,
        layout: &mut EditorLayout,
        cx: &mut ElementContext,
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
        layout: &EditorLayout,
        cx: &mut ElementContext,
    ) {
        let whitespace_setting = self
            .editor
            .read(cx)
            .buffer
            .read(cx)
            .settings_at(0, cx)
            .show_whitespaces;

        for (ix, line_with_invisibles) in layout.position_map.line_layouts.iter().enumerate() {
            let row = layout.visible_display_row_range.start + ix as u32;
            line_with_invisibles.draw(
                layout,
                row,
                layout.content_origin,
                whitespace_setting,
                invisible_display_ranges,
                cx,
            )
        }
    }

    fn paint_redactions(&mut self, layout: &EditorLayout, cx: &mut ElementContext) {
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

    fn paint_cursors(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        for cursor in &mut layout.cursors {
            cursor.paint(layout.content_origin, cx);
        }
    }

    fn paint_scrollbar(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
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
                let scrollbar_settings = EditorSettings::get_global(cx).scrollbar;
                let is_singleton = self.editor.read(cx).is_singleton(cx);
                let left = scrollbar_layout.hitbox.left();
                let right = scrollbar_layout.hitbox.right();
                let column_width =
                    px(((right - left - ScrollbarLayout::BORDER_WIDTH).0 / 3.0).floor());
                if is_singleton && scrollbar_settings.selections {
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
                    let left_x = left + ScrollbarLayout::BORDER_WIDTH + column_width;
                    let right_x = left_x + column_width;
                    for range in background_ranges {
                        let (start_y, end_y) =
                            scrollbar_layout.ys_for_marker(range.start().row(), range.end().row());
                        let bounds =
                            Bounds::from_corners(point(left_x, start_y), point(right_x, end_y));
                        cx.paint_quad(quad(
                            bounds,
                            Corners::default(),
                            cx.theme().status().info,
                            Edges::default(),
                            cx.theme().colors().scrollbar_thumb_border,
                        ));
                    }
                }

                if is_singleton && scrollbar_settings.symbols_selections {
                    let selection_ranges = self.editor.read(cx).background_highlights_in_range(
                        Anchor::min()..Anchor::max(),
                        &layout.position_map.snapshot,
                        cx.theme().colors(),
                    );
                    let left_x = left + ScrollbarLayout::BORDER_WIDTH + column_width;
                    let right_x = left_x + column_width;
                    for hunk in selection_ranges {
                        let start_display = Point::new(hunk.0.start.row(), 0)
                            .to_display_point(&layout.position_map.snapshot.display_snapshot);
                        let end_display = Point::new(hunk.0.end.row(), 0)
                            .to_display_point(&layout.position_map.snapshot.display_snapshot);
                        let (start_y, end_y) =
                            scrollbar_layout.ys_for_marker(start_display.row(), end_display.row());
                        let bounds =
                            Bounds::from_corners(point(left_x, start_y), point(right_x, end_y));
                        cx.paint_quad(quad(
                            bounds,
                            Corners::default(),
                            cx.theme().status().info,
                            Edges::default(),
                            cx.theme().colors().scrollbar_thumb_border,
                        ));
                    }
                }

                if is_singleton && scrollbar_settings.git_diff {
                    let left_x = left + ScrollbarLayout::BORDER_WIDTH;
                    let right_x = left_x + column_width;
                    for hunk in layout
                        .position_map
                        .snapshot
                        .buffer_snapshot
                        .git_diff_hunks_in_range(0..layout.max_row)
                    {
                        let start_display_row = Point::new(hunk.associated_range.start, 0)
                            .to_display_point(&layout.position_map.snapshot.display_snapshot)
                            .row();
                        let mut end_display_row = Point::new(hunk.associated_range.end, 0)
                            .to_display_point(&layout.position_map.snapshot.display_snapshot)
                            .row();
                        if end_display_row != start_display_row {
                            end_display_row -= 1;
                        }
                        let (start_y, end_y) =
                            scrollbar_layout.ys_for_marker(start_display_row, end_display_row);
                        let bounds =
                            Bounds::from_corners(point(left_x, start_y), point(right_x, end_y));
                        let color = match hunk.status() {
                            DiffHunkStatus::Added => cx.theme().status().created,
                            DiffHunkStatus::Modified => cx.theme().status().modified,
                            DiffHunkStatus::Removed => cx.theme().status().deleted,
                        };
                        cx.paint_quad(quad(
                            bounds,
                            Corners::default(),
                            color,
                            Edges::default(),
                            cx.theme().colors().scrollbar_thumb_border,
                        ));
                    }
                }

                if is_singleton && scrollbar_settings.diagnostics {
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
                        .sorted_by_key(|diagnostic| {
                            std::cmp::Reverse(diagnostic.diagnostic.severity)
                        });

                    let left_x = left + ScrollbarLayout::BORDER_WIDTH + 2.0 * column_width;
                    for diagnostic in diagnostics {
                        let start_display = diagnostic
                            .range
                            .start
                            .to_display_point(&layout.position_map.snapshot.display_snapshot);
                        let end_display = diagnostic
                            .range
                            .end
                            .to_display_point(&layout.position_map.snapshot.display_snapshot);
                        let (start_y, end_y) =
                            scrollbar_layout.ys_for_marker(start_display.row(), end_display.row());
                        let bounds =
                            Bounds::from_corners(point(left_x, start_y), point(right, end_y));
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
                            Edges::default(),
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
                        right: Pixels::ZERO,
                        bottom: Pixels::ZERO,
                        left: ScrollbarLayout::BORDER_WIDTH,
                    },
                    cx.theme().colors().scrollbar_thumb_border,
                ));
            });
        }

        cx.set_cursor_style(CursorStyle::Arrow, &scrollbar_layout.hitbox);

        let scroll_height = scrollbar_layout.scroll_height;
        let height = scrollbar_layout.height;
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
                            position.y += (new_y - y) * scroll_height / height;
                            if position.y < 0.0 {
                                position.y = 0.0;
                            }
                            editor.set_scroll_position(position, cx);
                        }

                        mouse_position = event.position;
                        cx.stop_propagation();
                    } else {
                        editor.scroll_manager.set_is_dragging_scrollbar(false, cx);
                        if hitbox.is_hovered(cx) {
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
                let hitbox = scrollbar_layout.hitbox.clone();
                move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Capture || !hitbox.is_hovered(cx) {
                        return;
                    }

                    editor.update(cx, |editor, cx| {
                        editor.scroll_manager.set_is_dragging_scrollbar(true, cx);

                        let y = event.position.y;
                        if y < thumb_bounds.top() || thumb_bounds.bottom() < y {
                            let center_row =
                                ((y - hitbox.top()) * scroll_height / height).round() as u32;
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

    #[allow(clippy::too_many_arguments)]
    fn paint_highlighted_range(
        &self,
        range: Range<DisplayPoint>,
        color: Hsla,
        corner_radius: Pixels,
        line_end_overshoot: Pixels,
        layout: &EditorLayout,
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
                start_y: layout.content_origin.y
                    + row_range.start as f32 * layout.position_map.line_height
                    - layout.position_map.scroll_pixel_position.y,
                lines: row_range
                    .into_iter()
                    .map(|row| {
                        let line_layout =
                            &layout.position_map.line_layouts[(row - start_row) as usize].line;
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

    fn paint_folds(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        if layout.folds.is_empty() {
            return;
        }

        cx.paint_layer(layout.text_hitbox.bounds, |cx| {
            let fold_corner_radius = 0.15 * layout.position_map.line_height;
            for mut fold in mem::take(&mut layout.folds) {
                fold.hover_element.paint(cx);

                let hover_element = fold.hover_element.downcast_mut::<Stateful<Div>>().unwrap();
                let fold_background = if hover_element.interactivity().active.unwrap() {
                    cx.theme().colors().ghost_element_active
                } else if hover_element.interactivity().hovered.unwrap() {
                    cx.theme().colors().ghost_element_hover
                } else {
                    cx.theme().colors().ghost_element_background
                };

                self.paint_highlighted_range(
                    fold.display_range.clone(),
                    fold_background,
                    fold_corner_radius,
                    fold_corner_radius * 2.,
                    layout,
                    cx,
                );
            }
        })
    }

    fn paint_blocks(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        for mut block in layout.blocks.drain(..) {
            block.element.paint(cx);
        }
    }

    fn paint_mouse_context_menu(&mut self, layout: &mut EditorLayout, cx: &mut ElementContext) {
        if let Some(mouse_context_menu) = layout.mouse_context_menu.as_mut() {
            mouse_context_menu.paint(cx);
        }
    }

    fn paint_scroll_wheel_listener(&mut self, layout: &EditorLayout, cx: &mut ElementContext) {
        cx.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let hitbox = layout.hitbox.clone();
            let mut delta = ScrollDelta::default();

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

                        let scroll_position = position_map.snapshot.scroll_position();
                        let x = (scroll_position.x * max_glyph_width - delta.x) / max_glyph_width;
                        let y = (scroll_position.y * line_height - delta.y) / line_height;
                        let scroll_position =
                            point(x, y).clamp(&point(0., 0.), &position_map.scroll_max);
                        editor.scroll(scroll_position, axis, cx);
                        cx.stop_propagation();
                    });
                }
            }
        });
    }

    fn paint_mouse_listeners(&mut self, layout: &EditorLayout, cx: &mut ElementContext) {
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
                                &position_map,
                                &text_hitbox,
                                &gutter_hitbox,
                                cx,
                            );
                        }),
                        MouseButton::Right => editor.update(cx, |editor, cx| {
                            Self::mouse_right_down(editor, event, &position_map, &text_hitbox, cx);
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
                        if event.pressed_button == Some(MouseButton::Left) {
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
        let digit_count = (snapshot.max_buffer_row() as f32 + 1.).log10().floor() as usize + 1;
        self.column_pixels(digit_count, cx)
    }
}

fn render_blame_entry(
    ix: usize,
    blame: &gpui::Model<GitBlame>,
    blame_entry: BlameEntry,
    text_style: &TextStyle,
    last_used_color: &mut Option<(PlayerColor, Oid)>,
    editor: View<Editor>,
    cx: &mut ElementContext<'_>,
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

    let relative_timestamp = match blame_entry.author_offset_date_time() {
        Ok(timestamp) => time_format::format_localized_timestamp(
            timestamp,
            time::OffsetDateTime::now_utc(),
            cx.local_timezone(),
            time_format::TimestampFormat::Relative,
        ),
        Err(_) => "Error parsing date".to_string(),
    };

    let pretty_commit_id = format!("{}", blame_entry.sha);
    let short_commit_id = pretty_commit_id.clone().chars().take(6).collect::<String>();

    let name = blame_entry.author.as_deref().unwrap_or("<no name>");
    let name = if name.len() > 20 {
        format!("{}...", &name[..16])
    } else {
        name.to_string()
    };

    let permalink = blame.read(cx).permalink_for_entry(&blame_entry);
    let commit_message = blame.read(cx).message_for_entry(&blame_entry);

    h_flex()
        .font(text_style.font().family)
        .line_height(text_style.line_height)
        .id(("blame", ix))
        .children([
            div()
                .text_color(sha_color.cursor)
                .child(short_commit_id)
                .mr_2(),
            div()
                .text_color(cx.theme().status().hint)
                .child(format!("{:20} {: >14}", name, relative_timestamp)),
        ])
        .on_mouse_down(MouseButton::Right, {
            let blame_entry = blame_entry.clone();
            move |event, cx| {
                deploy_blame_entry_context_menu(&blame_entry, editor.clone(), event.position, cx);
            }
        })
        .hover(|style| style.bg(cx.theme().colors().element_hover))
        .when_some(permalink, |this, url| {
            let url = url.clone();
            this.cursor_pointer().on_click(move |_, cx| {
                cx.stop_propagation();
                cx.open_url(url.as_str())
            })
        })
        .tooltip(move |cx| {
            BlameEntryTooltip::new(
                sha_color.cursor,
                commit_message.clone(),
                blame_entry.clone(),
                cx,
            )
        })
        .into_any()
}

fn deploy_blame_entry_context_menu(
    blame_entry: &BlameEntry,
    editor: View<Editor>,
    position: gpui::Point<Pixels>,
    cx: &mut WindowContext<'_>,
) {
    let context_menu = ContextMenu::build(cx, move |this, _| {
        let sha = format!("{}", blame_entry.sha);
        this.entry("Copy commit SHA", None, move |cx| {
            cx.write_to_clipboard(ClipboardItem::new(sha.clone()));
        })
    });

    editor.update(cx, move |editor, cx| {
        editor.mouse_context_menu = Some(MouseContextMenu::new(position, context_menu, cx));
        cx.notify();
    });
}

struct BlameEntryTooltip {
    color: Hsla,
    commit_message: Option<String>,
    blame_entry: BlameEntry,
}

impl BlameEntryTooltip {
    fn new(
        color: Hsla,
        commit_message: Option<String>,
        blame_entry: BlameEntry,
        cx: &mut WindowContext,
    ) -> AnyView {
        cx.new_view(|_cx| Self {
            color,
            commit_message,
            blame_entry,
        })
        .into()
    }
}

impl Render for BlameEntryTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let author = self
            .blame_entry
            .author
            .clone()
            .unwrap_or("<no name>".to_string());
        let author_email = self.blame_entry.author_mail.clone().unwrap_or_default();
        let absolute_timestamp = match self.blame_entry.author_offset_date_time() {
            Ok(timestamp) => time_format::format_localized_timestamp(
                timestamp,
                time::OffsetDateTime::now_utc(),
                cx.local_timezone(),
                time_format::TimestampFormat::Absolute,
            ),
            Err(_) => "Error parsing date".to_string(),
        };

        let message = match &self.commit_message {
            Some(message) => util::truncate_lines_and_trailoff(message, 15),
            None => self.blame_entry.summary.clone().unwrap_or_default(),
        };

        let pretty_commit_id = format!("{}", self.blame_entry.sha);

        tooltip_container(cx, move |this, cx| {
            this.occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .child(
                            h_flex()
                                .child(
                                    div()
                                        .text_color(cx.theme().colors().text_muted)
                                        .child("Commit")
                                        .pr_2(),
                                )
                                .child(
                                    div().text_color(self.color).child(pretty_commit_id.clone()),
                                ),
                        )
                        .child(
                            div()
                                .child(format!(
                                    "{} {} - {}",
                                    author, author_email, absolute_timestamp
                                ))
                                .text_color(cx.theme().colors().text_muted),
                        )
                        .child(div().child(message)),
                )
        })
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
                        invisibles: std::mem::take(&mut invisibles),
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
        layout: &EditorLayout,
        row: u32,
        content_origin: gpui::Point<Pixels>,
        whitespace_setting: ShowWhitespaceSetting,
        selection_ranges: &[Range<DisplayPoint>],
        cx: &mut ElementContext,
    ) {
        let line_height = layout.position_map.line_height;
        let line_y =
            line_height * (row as f32 - layout.position_map.scroll_pixel_position.y / line_height);

        self.line
            .paint(
                content_origin + gpui::point(-layout.position_map.scroll_pixel_position.x, line_y),
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

    #[allow(clippy::too_many_arguments)]
    fn draw_invisibles(
        &self,
        selection_ranges: &[Range<DisplayPoint>],
        layout: &EditorLayout,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Invisible {
    Tab { line_start_offset: usize },
    Whitespace { line_offset: usize },
}

impl Element for EditorElement {
    type BeforeLayout = ();
    type AfterLayout = EditorLayout;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (gpui::LayoutId, ()) {
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
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout {
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };
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
                let overscroll = size(em_width, px(0.));

                snapshot = self.editor.update(cx, |editor, cx| {
                    editor.gutter_width = gutter_dimensions.width;
                    editor.set_visible_line_count(bounds.size.height / line_height, cx);

                    let editor_width =
                        text_width - gutter_dimensions.margin - overscroll.width - em_width;
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

                let autoscroll_horizontally = self.editor.update(cx, |editor, cx| {
                    let autoscroll_horizontally =
                        editor.autoscroll_vertically(bounds.size.height, line_height, cx);
                    snapshot = editor.snapshot(cx);
                    autoscroll_horizontally
                });

                let mut scroll_position = snapshot.scroll_position();
                // The scroll position is a fractional point, the whole number of which represents
                // the top of the window in terms of display rows.
                let start_row = scroll_position.y as u32;
                let height_in_lines = bounds.size.height / line_height;
                let max_row = snapshot.max_point().row();

                // Add 1 to ensure selections bleed off screen
                let end_row =
                    1 + cmp::min((scroll_position.y + height_in_lines).ceil() as u32, max_row);

                let buffer_rows = snapshot
                    .buffer_rows(start_row)
                    .take((start_row..end_row).len());

                let start_anchor = if start_row == 0 {
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

                let (line_numbers, fold_statuses) = self.layout_line_numbers(
                    start_row..end_row,
                    buffer_rows.clone(),
                    &active_rows,
                    newest_selection_head,
                    &snapshot,
                    cx,
                );

                let display_hunks = self.layout_git_gutters(start_row..end_row, &snapshot);

                let blamed_display_rows = self.layout_blame_entries(
                    buffer_rows,
                    em_width,
                    scroll_position,
                    line_height,
                    &gutter_hitbox,
                    gutter_dimensions.git_blame_entries_width,
                    cx,
                );

                let mut max_visible_line_width = Pixels::ZERO;
                let line_layouts =
                    self.layout_lines(start_row..end_row, &line_numbers, &snapshot, cx);
                for line_with_invisibles in &line_layouts {
                    if line_with_invisibles.line.width > max_visible_line_width {
                        max_visible_line_width = line_with_invisibles.line.width;
                    }
                }

                let longest_line_width = layout_line(snapshot.longest_row(), &snapshot, &style, cx)
                    .unwrap()
                    .width;
                let mut scroll_width =
                    longest_line_width.max(max_visible_line_width) + overscroll.width;
                let mut blocks = self.build_blocks(
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
                );

                let scroll_max = point(
                    ((scroll_width - text_hitbox.size.width) / em_width).max(0.0),
                    max_row as f32,
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

                let scroll_pixel_position = point(
                    scroll_position.x * em_width,
                    scroll_position.y * line_height,
                );

                cx.with_element_id(Some("blocks"), |cx| {
                    self.layout_blocks(
                        &mut blocks,
                        &hitbox,
                        line_height,
                        scroll_pixel_position,
                        cx,
                    );
                });

                let cursors = self.layout_cursors(
                    &snapshot,
                    &selections,
                    start_row..end_row,
                    &line_layouts,
                    &text_hitbox,
                    content_origin,
                    scroll_pixel_position,
                    line_height,
                    em_width,
                    cx,
                );

                let scrollbar_layout = self.layout_scrollbar(
                    &snapshot,
                    bounds,
                    scroll_position,
                    line_height,
                    height_in_lines,
                    cx,
                );

                let folds = cx.with_element_id(Some("folds"), |cx| {
                    self.layout_folds(
                        &snapshot,
                        content_origin,
                        start_anchor..end_anchor,
                        start_row..end_row,
                        scroll_pixel_position,
                        line_height,
                        &line_layouts,
                        cx,
                    )
                });

                let gutter_settings = EditorSettings::get_global(cx).gutter;

                let mut context_menu_visible = false;
                let mut code_actions_indicator = None;
                if let Some(newest_selection_head) = newest_selection_head {
                    if (start_row..end_row).contains(&newest_selection_head.row()) {
                        context_menu_visible = self.layout_context_menu(
                            line_height,
                            &hitbox,
                            &text_hitbox,
                            content_origin,
                            start_row,
                            scroll_pixel_position,
                            &line_layouts,
                            newest_selection_head,
                            cx,
                        );
                        if gutter_settings.code_actions {
                            code_actions_indicator = self.layout_code_actions_indicator(
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

                if !context_menu_visible && !cx.has_active_drag() {
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

                let fold_indicators = if gutter_settings.folds {
                    cx.with_element_id(Some("gutter_fold_indicators"), |cx| {
                        self.layout_gutter_fold_indicators(
                            fold_statuses,
                            line_height,
                            &gutter_dimensions,
                            gutter_settings,
                            scroll_pixel_position,
                            &gutter_hitbox,
                            cx,
                        )
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
                    hitbox,
                    text_hitbox,
                    gutter_hitbox,
                    gutter_dimensions,
                    content_origin,
                    scrollbar_layout,
                    max_row,
                    active_rows,
                    highlighted_rows,
                    highlighted_ranges,
                    redacted_ranges,
                    line_numbers,
                    display_hunks,
                    blamed_display_rows,
                    folds,
                    blocks,
                    cursors,
                    selections,
                    mouse_context_menu,
                    code_actions_indicator,
                    fold_indicators,
                    tab_invisible,
                    space_invisible,
                }
            })
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<gpui::Pixels>,
        _: &mut Self::BeforeLayout,
        layout: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        let focus_handle = self.editor.focus_handle(cx);
        let key_context = self.editor.read(cx).key_context(cx);
        cx.set_focus_handle(&focus_handle);
        cx.set_key_context(key_context);
        cx.set_view_id(self.editor.entity_id());
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
        cx.with_text_style(Some(text_style), |cx| {
            cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                self.paint_mouse_listeners(layout, cx);

                self.paint_background(layout, cx);
                if layout.gutter_hitbox.size.width > Pixels::ZERO {
                    self.paint_gutter(layout, cx);
                }
                self.paint_text(layout, cx);

                if !layout.blocks.is_empty() {
                    cx.with_element_id(Some("blocks"), |cx| {
                        self.paint_blocks(layout, cx);
                    });
                }

                self.paint_scrollbar(layout, cx);
                self.paint_mouse_context_menu(layout, cx);
            });
        })
    }
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

type BufferRow = u32;

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
    visible_display_row_range: Range<u32>,
    active_rows: BTreeMap<u32, bool>,
    highlighted_rows: BTreeMap<u32, Hsla>,
    line_numbers: Vec<Option<ShapedLine>>,
    display_hunks: Vec<DisplayDiffHunk>,
    blamed_display_rows: Option<Vec<AnyElement>>,
    folds: Vec<FoldLayout>,
    blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Hsla)>,
    redacted_ranges: Vec<Range<DisplayPoint>>,
    cursors: Vec<CursorLayout>,
    selections: Vec<(PlayerColor, Vec<SelectionLayout>)>,
    max_row: u32,
    code_actions_indicator: Option<AnyElement>,
    fold_indicators: Vec<Option<AnyElement>>,
    mouse_context_menu: Option<AnyElement>,
    tab_invisible: ShapedLine,
    space_invisible: ShapedLine,
}

impl EditorLayout {
    fn line_end_overshoot(&self) -> Pixels {
        0.15 * self.position_map.line_height
    }
}

struct ScrollbarLayout {
    hitbox: Hitbox,
    visible_row_range: Range<f32>,
    visible: bool,
    height: Pixels,
    scroll_height: f32,
    first_row_y_offset: Pixels,
    row_height: Pixels,
}

impl ScrollbarLayout {
    const BORDER_WIDTH: Pixels = px(1.0);
    const MIN_MARKER_HEIGHT: Pixels = px(2.0);

    fn thumb_bounds(&self) -> Bounds<Pixels> {
        let thumb_top = self.y_for_row(self.visible_row_range.start) - self.first_row_y_offset;
        let thumb_bottom = self.y_for_row(self.visible_row_range.end) + self.first_row_y_offset;
        Bounds::from_corners(
            point(self.hitbox.left(), thumb_top),
            point(self.hitbox.right(), thumb_bottom),
        )
    }

    fn y_for_row(&self, row: f32) -> Pixels {
        self.hitbox.top() + self.first_row_y_offset + row * self.row_height
    }

    fn ys_for_marker(&self, start_row: u32, end_row: u32) -> (Pixels, Pixels) {
        let start_y = self.y_for_row(start_row as f32);
        let mut end_y = self.y_for_row((end_row + 1) as f32);
        if end_y - start_y < Self::MIN_MARKER_HEIGHT {
            end_y = start_y + Self::MIN_MARKER_HEIGHT;
        }
        (start_y, end_y)
    }
}

struct FoldLayout {
    display_range: Range<DisplayPoint>,
    hover_element: AnyElement,
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
            .map(|LineWithInvisibles { line, .. }| line)
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
        cx: &mut ElementContext,
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

            name_element.layout(
                name_origin,
                size(AvailableSpace::MinContent, AvailableSpace::MinContent),
                cx,
            );

            self.cursor_name = Some(name_element);
        }
    }

    pub fn paint(&mut self, origin: gpui::Point<Pixels>, cx: &mut ElementContext) {
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
        let snapshot = window.update(cx, |editor, cx| editor.snapshot(cx)).unwrap();

        let layouts = cx
            .update_window(*window, |_, cx| {
                cx.with_element_context(|cx| {
                    element
                        .layout_line_numbers(
                            0..6,
                            (0..6).map(Some),
                            &Default::default(),
                            Some(DisplayPoint::new(0, 0)),
                            &snapshot,
                            cx,
                        )
                        .0
                })
            })
            .unwrap();
        assert_eq!(layouts.len(), 6);

        let relative_rows =
            element.calculate_relative_line_numbers((0..6).map(Some).collect(), &(0..6), Some(3));
        assert_eq!(relative_rows[&0], 3);
        assert_eq!(relative_rows[&1], 2);
        assert_eq!(relative_rows[&2], 1);
        // current line has no relative number
        assert_eq!(relative_rows[&4], 1);
        assert_eq!(relative_rows[&5], 2);

        // works if cursor is before screen
        let relative_rows =
            element.calculate_relative_line_numbers((0..6).map(Some).collect(), &(3..6), Some(1));
        assert_eq!(relative_rows.len(), 3);
        assert_eq!(relative_rows[&3], 2);
        assert_eq!(relative_rows[&4], 3);
        assert_eq!(relative_rows[&5], 4);

        // works if cursor is after screen
        let relative_rows =
            element.calculate_relative_line_numbers((0..6).map(Some).collect(), &(0..3), Some(6));
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
            .update_window(window.into(), |_view, cx| {
                cx.with_element_context(|cx| {
                    element.after_layout(
                        Bounds {
                            origin: point(px(500.), px(500.)),
                            size: size(px(500.), px(500.)),
                        },
                        &mut (),
                        cx,
                    )
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
            .update_window(window.into(), |_view, cx| {
                cx.with_element_context(|cx| {
                    element.after_layout(
                        Bounds {
                            origin: point(px(500.), px(500.)),
                            size: size(px(500.), px(500.)),
                        },
                        &mut (),
                        cx,
                    )
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
            .update_window(window.into(), |_view, cx| {
                cx.with_element_context(|cx| {
                    element.after_layout(
                        Bounds {
                            origin: point(px(500.), px(500.)),
                            size: size(px(500.), px(500.)),
                        },
                        &mut (),
                        cx,
                    )
                })
            })
            .unwrap();

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
                    element.after_layout(
                        Bounds {
                            origin: point(px(500.), px(500.)),
                            size: size(px(500.), px(500.)),
                        },
                        &mut (),
                        cx,
                    )
                })
            })
            .unwrap();

        layout_state
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
