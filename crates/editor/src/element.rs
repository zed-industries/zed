use super::{
    display_map::{BlockContext, ToDisplayPoint},
    Anchor, DisplayPoint, Editor, EditorMode, EditorSnapshot, SelectPhase, SoftWrap, ToPoint,
    MAX_LINE_LEN,
};
use crate::{
    display_map::{BlockStyle, DisplaySnapshot, FoldStatus, TransformBlock},
    git::{diff_hunk_to_display, DisplayDiffHunk},
    hover_popover::{
        hide_hover, hover_at, HOVER_POPOVER_GAP, MIN_POPOVER_CHARACTER_WIDTH,
        MIN_POPOVER_LINE_HEIGHT,
    },
    link_go_to_definition::{
        go_to_fetched_definition, go_to_fetched_type_definition, update_go_to_definition_link,
    },
    mouse_context_menu, EditorStyle, GutterHover, UnfoldAt,
};
use clock::ReplicaId;
use collections::{BTreeMap, HashMap};
use git::diff::DiffHunkStatus;
use gpui::{
    color::Color,
    elements::*,
    fonts::{HighlightStyle, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
        PathBuilder,
    },
    json::{self, ToJson},
    platform::{CursorStyle, Modifiers, MouseButton, MouseButtonEvent, MouseMovedEvent},
    text_layout::{self, Line, RunStyle, TextLayoutCache},
    AnyElement, Axis, Border, CursorRegion, Element, EventContext, MouseRegion, Quad, SceneBuilder,
    SizeConstraint, ViewContext, WindowContext,
};
use itertools::Itertools;
use json::json;
use language::{Bias, CursorShape, DiagnosticSeverity, OffsetUtf16, Selection};
use project::ProjectPath;
use settings::{GitGutter, Settings};
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    iter,
    ops::Range,
    sync::Arc,
};
use workspace::item::Item;

enum FoldMarkers {}

struct SelectionLayout {
    head: DisplayPoint,
    cursor_shape: CursorShape,
    range: Range<DisplayPoint>,
}

impl SelectionLayout {
    fn new<T: ToPoint + ToDisplayPoint + Clone>(
        selection: Selection<T>,
        line_mode: bool,
        cursor_shape: CursorShape,
        map: &DisplaySnapshot,
    ) -> Self {
        if line_mode {
            let selection = selection.map(|p| p.to_point(&map.buffer_snapshot));
            let point_range = map.expand_to_line(selection.range());
            Self {
                head: selection.head().to_display_point(map),
                cursor_shape,
                range: point_range.start.to_display_point(map)
                    ..point_range.end.to_display_point(map),
            }
        } else {
            let selection = selection.map(|p| p.to_display_point(map));
            Self {
                head: selection.head(),
                cursor_shape,
                range: selection.range(),
            }
        }
    }
}

#[derive(Clone)]
pub struct EditorElement {
    style: Arc<EditorStyle>,
}

impl EditorElement {
    pub fn new(style: EditorStyle) -> Self {
        Self {
            style: Arc::new(style),
        }
    }

    fn attach_mouse_handlers(
        scene: &mut SceneBuilder,
        position_map: &Arc<PositionMap>,
        has_popovers: bool,
        visible_bounds: RectF,
        text_bounds: RectF,
        gutter_bounds: RectF,
        bounds: RectF,
        cx: &mut ViewContext<Editor>,
    ) {
        enum EditorElementMouseHandlers {}
        scene.push_mouse_region(
            MouseRegion::new::<EditorElementMouseHandlers>(
                cx.view_id(),
                cx.view_id(),
                visible_bounds,
            )
            .on_down(MouseButton::Left, {
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::mouse_down(
                        editor,
                        event.platform_event,
                        position_map.as_ref(),
                        text_bounds,
                        gutter_bounds,
                        cx,
                    ) {
                        cx.propagate_event();
                    }
                }
            })
            .on_down(MouseButton::Right, {
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::mouse_right_down(
                        editor,
                        event.position,
                        position_map.as_ref(),
                        text_bounds,
                        cx,
                    ) {
                        cx.propagate_event();
                    }
                }
            })
            .on_up(MouseButton::Left, {
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::mouse_up(
                        editor,
                        event.position,
                        event.cmd,
                        event.shift,
                        position_map.as_ref(),
                        text_bounds,
                        cx,
                    ) {
                        cx.propagate_event()
                    }
                }
            })
            .on_drag(MouseButton::Left, {
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::mouse_dragged(
                        editor,
                        event.platform_event,
                        position_map.as_ref(),
                        text_bounds,
                        cx,
                    ) {
                        cx.propagate_event()
                    }
                }
            })
            .on_move({
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::mouse_moved(
                        editor,
                        event.platform_event,
                        &position_map,
                        text_bounds,
                        cx,
                    ) {
                        cx.propagate_event()
                    }
                }
            })
            .on_move_out(move |_, editor: &mut Editor, cx| {
                if has_popovers {
                    hide_hover(editor, cx);
                }
            })
            .on_scroll({
                let position_map = position_map.clone();
                move |event, editor, cx| {
                    if !Self::scroll(
                        editor,
                        event.position,
                        *event.delta.raw(),
                        event.delta.precise(),
                        &position_map,
                        bounds,
                        cx,
                    ) {
                        cx.propagate_event()
                    }
                }
            }),
        );

        enum GutterHandlers {}
        scene.push_mouse_region(
            MouseRegion::new::<GutterHandlers>(cx.view_id(), cx.view_id() + 1, gutter_bounds)
                .on_hover(|hover, _: &mut Editor, cx| {
                    cx.dispatch_action(GutterHover {
                        hovered: hover.started,
                    })
                }),
        )
    }

    fn mouse_down(
        editor: &mut Editor,
        MouseButtonEvent {
            position,
            modifiers:
                Modifiers {
                    shift,
                    ctrl,
                    alt,
                    cmd,
                    ..
                },
            mut click_count,
            ..
        }: MouseButtonEvent,
        position_map: &PositionMap,
        text_bounds: RectF,
        gutter_bounds: RectF,
        cx: &mut EventContext<Editor>,
    ) -> bool {
        if gutter_bounds.contains_point(position) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !text_bounds.contains_point(position) {
            return false;
        }

        let (position, target_position) = position_map.point_for_position(text_bounds, position);

        if shift && alt {
            editor.select(
                SelectPhase::BeginColumnar {
                    position,
                    goal_column: target_position.column(),
                },
                cx,
            );
        } else if shift && !ctrl && !alt && !cmd {
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
                    add: alt,
                    click_count,
                },
                cx,
            );
        }

        true
    }

    fn mouse_right_down(
        editor: &mut Editor,
        position: Vector2F,
        position_map: &PositionMap,
        text_bounds: RectF,
        cx: &mut EventContext<Editor>,
    ) -> bool {
        if !text_bounds.contains_point(position) {
            return false;
        }

        let (point, _) = position_map.point_for_position(text_bounds, position);
        mouse_context_menu::deploy_context_menu(editor, position, point, cx);
        true
    }

    fn mouse_up(
        editor: &mut Editor,
        position: Vector2F,
        cmd: bool,
        shift: bool,
        position_map: &PositionMap,
        text_bounds: RectF,
        cx: &mut EventContext<Editor>,
    ) -> bool {
        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        if end_selection {
            editor.select(SelectPhase::End, cx);
        }

        if !pending_nonempty_selections && cmd && text_bounds.contains_point(position) {
            let (point, target_point) = position_map.point_for_position(text_bounds, position);

            if point == target_point {
                if shift {
                    go_to_fetched_type_definition(editor, point, cx);
                } else {
                    go_to_fetched_definition(editor, point, cx);
                }

                return true;
            }
        }

        end_selection
    }

    fn mouse_dragged(
        editor: &mut Editor,
        MouseMovedEvent {
            modifiers: Modifiers { cmd, shift, .. },
            position,
            ..
        }: MouseMovedEvent,
        position_map: &PositionMap,
        text_bounds: RectF,
        cx: &mut EventContext<Editor>,
    ) -> bool {
        // This will be handled more correctly once https://github.com/zed-industries/zed/issues/1218 is completed
        // Don't trigger hover popover if mouse is hovering over context menu
        let point = if text_bounds.contains_point(position) {
            let (point, target_point) = position_map.point_for_position(text_bounds, position);
            if point == target_point {
                Some(point)
            } else {
                None
            }
        } else {
            None
        };

        update_go_to_definition_link(editor, point, cmd, shift, cx);

        if editor.has_pending_selection() {
            let mut scroll_delta = Vector2F::zero();

            let vertical_margin = position_map.line_height.min(text_bounds.height() / 3.0);
            let top = text_bounds.origin_y() + vertical_margin;
            let bottom = text_bounds.lower_left().y() - vertical_margin;
            if position.y() < top {
                scroll_delta.set_y(-scale_vertical_mouse_autoscroll_delta(top - position.y()))
            }
            if position.y() > bottom {
                scroll_delta.set_y(scale_vertical_mouse_autoscroll_delta(position.y() - bottom))
            }

            let horizontal_margin = position_map.line_height.min(text_bounds.width() / 3.0);
            let left = text_bounds.origin_x() + horizontal_margin;
            let right = text_bounds.upper_right().x() - horizontal_margin;
            if position.x() < left {
                scroll_delta.set_x(-scale_horizontal_mouse_autoscroll_delta(
                    left - position.x(),
                ))
            }
            if position.x() > right {
                scroll_delta.set_x(scale_horizontal_mouse_autoscroll_delta(
                    position.x() - right,
                ))
            }

            let (position, target_position) =
                position_map.point_for_position(text_bounds, position);

            editor.select(
                SelectPhase::Update {
                    position,
                    goal_column: target_position.column(),
                    scroll_position: (position_map.snapshot.scroll_position() + scroll_delta)
                        .clamp(Vector2F::zero(), position_map.scroll_max),
                },
                cx,
            );
            hover_at(editor, point, cx);
            true
        } else {
            hover_at(editor, point, cx);
            false
        }
    }

    fn mouse_moved(
        editor: &mut Editor,
        MouseMovedEvent {
            modifiers: Modifiers { shift, cmd, .. },
            position,
            ..
        }: MouseMovedEvent,
        position_map: &PositionMap,
        text_bounds: RectF,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        // This will be handled more correctly once https://github.com/zed-industries/zed/issues/1218 is completed
        // Don't trigger hover popover if mouse is hovering over context menu
        let point = position_to_display_point(position, text_bounds, position_map);

        update_go_to_definition_link(editor, point, cmd, shift, cx);
        hover_at(editor, point, cx);

        true
    }

    fn scroll(
        editor: &mut Editor,
        position: Vector2F,
        mut delta: Vector2F,
        precise: bool,
        position_map: &PositionMap,
        bounds: RectF,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if !bounds.contains_point(position) {
            return false;
        }

        let line_height = position_map.line_height;
        let max_glyph_width = position_map.em_width;

        let axis = if precise {
            //Trackpad
            position_map.snapshot.ongoing_scroll.filter(&mut delta)
        } else {
            //Not trackpad
            delta *= vec2f(max_glyph_width, line_height);
            None //Resets ongoing scroll
        };

        let scroll_position = position_map.snapshot.scroll_position();
        let x = (scroll_position.x() * max_glyph_width - delta.x()) / max_glyph_width;
        let y = (scroll_position.y() * line_height - delta.y()) / line_height;
        let scroll_position = vec2f(x, y).clamp(Vector2F::zero(), position_map.scroll_max);
        editor.scroll(scroll_position, axis, cx);

        true
    }

    fn paint_background(
        &self,
        scene: &mut SceneBuilder,
        gutter_bounds: RectF,
        text_bounds: RectF,
        layout: &LayoutState,
    ) {
        let bounds = gutter_bounds.union_rect(text_bounds);
        let scroll_top =
            layout.position_map.snapshot.scroll_position().y() * layout.position_map.line_height;
        scene.push_quad(Quad {
            bounds: gutter_bounds,
            background: Some(self.style.gutter_background),
            border: Border::new(0., Color::transparent_black()),
            corner_radius: 0.,
        });
        scene.push_quad(Quad {
            bounds: text_bounds,
            background: Some(self.style.background),
            border: Border::new(0., Color::transparent_black()),
            corner_radius: 0.,
        });

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
                    let origin = vec2f(
                        bounds.origin_x(),
                        bounds.origin_y() + (layout.position_map.line_height * *start_row as f32)
                            - scroll_top,
                    );
                    let size = vec2f(
                        bounds.width(),
                        layout.position_map.line_height * (end_row - start_row + 1) as f32,
                    );
                    scene.push_quad(Quad {
                        bounds: RectF::new(origin, size),
                        background: Some(self.style.active_line_background),
                        border: Border::default(),
                        corner_radius: 0.,
                    });
                }
            }

            if let Some(highlighted_rows) = &layout.highlighted_rows {
                let origin = vec2f(
                    bounds.origin_x(),
                    bounds.origin_y()
                        + (layout.position_map.line_height * highlighted_rows.start as f32)
                        - scroll_top,
                );
                let size = vec2f(
                    bounds.width(),
                    layout.position_map.line_height * highlighted_rows.len() as f32,
                );
                scene.push_quad(Quad {
                    bounds: RectF::new(origin, size),
                    background: Some(self.style.highlighted_line_background),
                    border: Border::default(),
                    corner_radius: 0.,
                });
            }
        }
    }

    fn paint_gutter(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) {
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y() * line_height;

        let show_gutter = matches!(
            &cx.global::<Settings>()
                .git_overrides
                .git_gutter
                .unwrap_or_default(),
            GitGutter::TrackedFiles
        );

        if show_gutter {
            Self::paint_diff_hunks(scene, bounds, layout, cx);
        }

        for (ix, line) in layout.line_number_layouts.iter().enumerate() {
            if let Some(line) = line {
                let line_origin = bounds.origin()
                    + vec2f(
                        bounds.width() - line.width() - layout.gutter_padding,
                        ix as f32 * line_height - (scroll_top % line_height),
                    );

                line.paint(scene, line_origin, visible_bounds, line_height, cx);
            }
        }

        for (ix, fold_indicator) in layout.fold_indicators.iter_mut().enumerate() {
            if let Some(indicator) = fold_indicator.as_mut() {
                let position = vec2f(
                    bounds.width() - layout.gutter_padding,
                    ix as f32 * line_height - (scroll_top % line_height),
                );
                let centering_offset = vec2f(
                    (layout.gutter_padding + layout.gutter_margin - indicator.size().x()) / 2.,
                    (line_height - indicator.size().y()) / 2.,
                );

                let indicator_origin = bounds.origin() + position + centering_offset;

                indicator.paint(scene, indicator_origin, visible_bounds, editor, cx);
            }
        }

        if let Some((row, indicator)) = layout.code_actions_indicator.as_mut() {
            let mut x = 0.;
            let mut y = *row as f32 * line_height - scroll_top;
            x += ((layout.gutter_padding + layout.gutter_margin) - indicator.size().x()) / 2.;
            y += (line_height - indicator.size().y()) / 2.;
            indicator.paint(
                scene,
                bounds.origin() + vec2f(x, y),
                visible_bounds,
                editor,
                cx,
            );
        }
    }

    fn paint_diff_hunks(
        scene: &mut SceneBuilder,
        bounds: RectF,
        layout: &mut LayoutState,
        cx: &mut ViewContext<Editor>,
    ) {
        let diff_style = &cx.global::<Settings>().theme.editor.diff.clone();
        let line_height = layout.position_map.line_height;

        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_top = scroll_position.y() * line_height;

        for hunk in &layout.display_hunks {
            let (display_row_range, status) = match hunk {
                //TODO: This rendering is entirely a horrible hack
                &DisplayDiffHunk::Folded { display_row: row } => {
                    let start_y = row as f32 * line_height - scroll_top;
                    let end_y = start_y + line_height;

                    let width = diff_style.removed_width_em * line_height;
                    let highlight_origin = bounds.origin() + vec2f(-width, start_y);
                    let highlight_size = vec2f(width * 2., end_y - start_y);
                    let highlight_bounds = RectF::new(highlight_origin, highlight_size);

                    scene.push_quad(Quad {
                        bounds: highlight_bounds,
                        background: Some(diff_style.modified),
                        border: Border::new(0., Color::transparent_black()),
                        corner_radius: 1. * line_height,
                    });

                    continue;
                }

                DisplayDiffHunk::Unfolded {
                    display_row_range,
                    status,
                } => (display_row_range, status),
            };

            let color = match status {
                DiffHunkStatus::Added => diff_style.inserted,
                DiffHunkStatus::Modified => diff_style.modified,

                //TODO: This rendering is entirely a horrible hack
                DiffHunkStatus::Removed => {
                    let row = *display_row_range.start();

                    let offset = line_height / 2.;
                    let start_y = row as f32 * line_height - offset - scroll_top;
                    let end_y = start_y + line_height;

                    let width = diff_style.removed_width_em * line_height;
                    let highlight_origin = bounds.origin() + vec2f(-width, start_y);
                    let highlight_size = vec2f(width * 2., end_y - start_y);
                    let highlight_bounds = RectF::new(highlight_origin, highlight_size);

                    scene.push_quad(Quad {
                        bounds: highlight_bounds,
                        background: Some(diff_style.deleted),
                        border: Border::new(0., Color::transparent_black()),
                        corner_radius: 1. * line_height,
                    });

                    continue;
                }
            };

            let start_row = *display_row_range.start();
            let end_row = *display_row_range.end();

            let start_y = start_row as f32 * line_height - scroll_top;
            let end_y = end_row as f32 * line_height - scroll_top + line_height;

            let width = diff_style.width_em * line_height;
            let highlight_origin = bounds.origin() + vec2f(-width, start_y);
            let highlight_size = vec2f(width * 2., end_y - start_y);
            let highlight_bounds = RectF::new(highlight_origin, highlight_size);

            scene.push_quad(Quad {
                bounds: highlight_bounds,
                background: Some(color),
                border: Border::new(0., Color::transparent_black()),
                corner_radius: diff_style.corner_radius * line_height,
            });
        }
    }

    fn paint_text(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) {
        let style = &self.style;
        let local_replica_id = editor.replica_id(cx);
        let scroll_position = layout.position_map.snapshot.scroll_position();
        let start_row = layout.visible_display_row_range.start;
        let scroll_top = scroll_position.y() * layout.position_map.line_height;
        let max_glyph_width = layout.position_map.em_width;
        let scroll_left = scroll_position.x() * max_glyph_width;
        let content_origin = bounds.origin() + vec2f(layout.gutter_margin, 0.);
        let line_end_overshoot = 0.15 * layout.position_map.line_height;

        scene.push_layer(Some(bounds));

        scene.push_cursor_region(CursorRegion {
            bounds,
            style: if !editor.link_go_to_definition_state.definitions.is_empty() {
                CursorStyle::PointingHand
            } else {
                CursorStyle::IBeam
            },
        });

        let fold_corner_radius =
            self.style.folds.ellipses.corner_radius_factor * layout.position_map.line_height;
        for (id, range, color) in layout.fold_ranges.iter() {
            self.paint_highlighted_range(
                scene,
                range.clone(),
                *color,
                fold_corner_radius,
                fold_corner_radius * 2.,
                layout,
                content_origin,
                scroll_top,
                scroll_left,
                bounds,
            );

            for bound in range_to_bounds(
                &range,
                content_origin,
                scroll_left,
                scroll_top,
                &layout.visible_display_row_range,
                line_end_overshoot,
                &layout.position_map,
            ) {
                scene.push_cursor_region(CursorRegion {
                    bounds: bound,
                    style: CursorStyle::PointingHand,
                });

                let display_row = range.start.row();

                let buffer_row = DisplayPoint::new(display_row, 0)
                    .to_point(&layout.position_map.snapshot.display_snapshot)
                    .row;

                scene.push_mouse_region(
                    MouseRegion::new::<FoldMarkers>(cx.view_id(), *id as usize, bound)
                        .on_click(MouseButton::Left, move |_, _: &mut Editor, cx| {
                            cx.dispatch_action(UnfoldAt { buffer_row })
                        })
                        .with_notify_on_hover(true)
                        .with_notify_on_click(true),
                )
            }
        }

        for (range, color) in &layout.highlighted_ranges {
            self.paint_highlighted_range(
                scene,
                range.clone(),
                *color,
                0.,
                line_end_overshoot,
                layout,
                content_origin,
                scroll_top,
                scroll_left,
                bounds,
            );
        }

        let mut cursors = SmallVec::<[Cursor; 32]>::new();
        let corner_radius = 0.15 * layout.position_map.line_height;

        for (replica_id, selections) in &layout.selections {
            let selection_style = style.replica_selection_style(*replica_id);

            for selection in selections {
                self.paint_highlighted_range(
                    scene,
                    selection.range.clone(),
                    selection_style.selection,
                    corner_radius,
                    corner_radius * 2.,
                    layout,
                    content_origin,
                    scroll_top,
                    scroll_left,
                    bounds,
                );

                if editor.show_local_cursors(cx) || *replica_id != local_replica_id {
                    let cursor_position = selection.head;
                    if layout
                        .visible_display_row_range
                        .contains(&cursor_position.row())
                    {
                        let cursor_row_layout = &layout.position_map.line_layouts
                            [(cursor_position.row() - start_row) as usize];
                        let cursor_column = cursor_position.column() as usize;

                        let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);
                        let mut block_width =
                            cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
                        if block_width == 0.0 {
                            block_width = layout.position_map.em_width;
                        }
                        let block_text = if let CursorShape::Block = selection.cursor_shape {
                            layout
                                .position_map
                                .snapshot
                                .chars_at(cursor_position)
                                .next()
                                .and_then(|(character, _)| {
                                    let font_id =
                                        cursor_row_layout.font_for_index(cursor_column)?;
                                    let text = character.to_string();

                                    Some(cx.text_layout_cache().layout_str(
                                        &text,
                                        cursor_row_layout.font_size(),
                                        &[(
                                            text.len(),
                                            RunStyle {
                                                font_id,
                                                color: style.background,
                                                underline: Default::default(),
                                            },
                                        )],
                                    ))
                                })
                        } else {
                            None
                        };

                        let x = cursor_character_x - scroll_left;
                        let y = cursor_position.row() as f32 * layout.position_map.line_height
                            - scroll_top;
                        cursors.push(Cursor {
                            color: selection_style.cursor,
                            block_width,
                            origin: vec2f(x, y),
                            line_height: layout.position_map.line_height,
                            shape: selection.cursor_shape,
                            block_text,
                        });
                    }
                }
            }
        }

        if let Some(visible_text_bounds) = bounds.intersection(visible_bounds) {
            // Draw glyphs
            for (ix, line) in layout.position_map.line_layouts.iter().enumerate() {
                let row = start_row + ix as u32;
                line.paint(
                    scene,
                    content_origin
                        + vec2f(
                            -scroll_left,
                            row as f32 * layout.position_map.line_height - scroll_top,
                        ),
                    visible_text_bounds,
                    layout.position_map.line_height,
                    cx,
                );
            }
        }

        scene.paint_layer(Some(bounds), |scene| {
            for cursor in cursors {
                cursor.paint(scene, content_origin, cx);
            }
        });

        if let Some((position, context_menu)) = layout.context_menu.as_mut() {
            scene.push_stacking_context(None, None);
            let cursor_row_layout =
                &layout.position_map.line_layouts[(position.row() - start_row) as usize];
            let x = cursor_row_layout.x_for_index(position.column() as usize) - scroll_left;
            let y = (position.row() + 1) as f32 * layout.position_map.line_height - scroll_top;
            let mut list_origin = content_origin + vec2f(x, y);
            let list_width = context_menu.size().x();
            let list_height = context_menu.size().y();

            // Snap the right edge of the list to the right edge of the window if
            // its horizontal bounds overflow.
            if list_origin.x() + list_width > cx.window_size().x() {
                list_origin.set_x((cx.window_size().x() - list_width).max(0.));
            }

            if list_origin.y() + list_height > bounds.max_y() {
                list_origin.set_y(list_origin.y() - layout.position_map.line_height - list_height);
            }

            context_menu.paint(
                scene,
                list_origin,
                RectF::from_points(Vector2F::zero(), vec2f(f32::MAX, f32::MAX)), // Let content bleed outside of editor
                editor,
                cx,
            );

            scene.pop_stacking_context();
        }

        if let Some((position, hover_popovers)) = layout.hover_popovers.as_mut() {
            scene.push_stacking_context(None, None);

            // This is safe because we check on layout whether the required row is available
            let hovered_row_layout =
                &layout.position_map.line_layouts[(position.row() - start_row) as usize];

            // Minimum required size: Take the first popover, and add 1.5 times the minimum popover
            // height. This is the size we will use to decide whether to render popovers above or below
            // the hovered line.
            let first_size = hover_popovers[0].size();
            let height_to_reserve = first_size.y()
                + 1.5 * MIN_POPOVER_LINE_HEIGHT as f32 * layout.position_map.line_height;

            // Compute Hovered Point
            let x = hovered_row_layout.x_for_index(position.column() as usize) - scroll_left;
            let y = position.row() as f32 * layout.position_map.line_height - scroll_top;
            let hovered_point = content_origin + vec2f(x, y);

            if hovered_point.y() - height_to_reserve > 0.0 {
                // There is enough space above. Render popovers above the hovered point
                let mut current_y = hovered_point.y();
                for hover_popover in hover_popovers {
                    let size = hover_popover.size();
                    let mut popover_origin = vec2f(hovered_point.x(), current_y - size.y());

                    let x_out_of_bounds = bounds.max_x() - (popover_origin.x() + size.x());
                    if x_out_of_bounds < 0.0 {
                        popover_origin.set_x(popover_origin.x() + x_out_of_bounds);
                    }

                    hover_popover.paint(
                        scene,
                        popover_origin,
                        RectF::from_points(Vector2F::zero(), vec2f(f32::MAX, f32::MAX)), // Let content bleed outside of editor
                        editor,
                        cx,
                    );

                    current_y = popover_origin.y() - HOVER_POPOVER_GAP;
                }
            } else {
                // There is not enough space above. Render popovers below the hovered point
                let mut current_y = hovered_point.y() + layout.position_map.line_height;
                for hover_popover in hover_popovers {
                    let size = hover_popover.size();
                    let mut popover_origin = vec2f(hovered_point.x(), current_y);

                    let x_out_of_bounds = bounds.max_x() - (popover_origin.x() + size.x());
                    if x_out_of_bounds < 0.0 {
                        popover_origin.set_x(popover_origin.x() + x_out_of_bounds);
                    }

                    hover_popover.paint(
                        scene,
                        popover_origin,
                        RectF::from_points(Vector2F::zero(), vec2f(f32::MAX, f32::MAX)), // Let content bleed outside of editor
                        editor,
                        cx,
                    );

                    current_y = popover_origin.y() + size.y() + HOVER_POPOVER_GAP;
                }
            }

            scene.pop_stacking_context();
        }

        scene.pop_layer();
    }

    fn paint_scrollbar(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        layout: &mut LayoutState,
        cx: &mut ViewContext<Editor>,
    ) {
        enum ScrollbarMouseHandlers {}
        if layout.mode != EditorMode::Full {
            return;
        }

        let style = &self.style.theme.scrollbar;

        let top = bounds.min_y();
        let bottom = bounds.max_y();
        let right = bounds.max_x();
        let left = right - style.width;
        let row_range = &layout.scrollbar_row_range;
        let max_row = layout.max_row as f32 + (row_range.end - row_range.start);

        let mut height = bounds.height();
        let mut first_row_y_offset = 0.0;

        // Impose a minimum height on the scrollbar thumb
        let min_thumb_height =
            style.min_height_factor * cx.font_cache.line_height(self.style.text.font_size);
        let thumb_height = (row_range.end - row_range.start) * height / max_row;
        if thumb_height < min_thumb_height {
            first_row_y_offset = (min_thumb_height - thumb_height) / 2.0;
            height -= min_thumb_height - thumb_height;
        }

        let y_for_row = |row: f32| -> f32 { top + first_row_y_offset + row * height / max_row };

        let thumb_top = y_for_row(row_range.start) - first_row_y_offset;
        let thumb_bottom = y_for_row(row_range.end) + first_row_y_offset;
        let track_bounds = RectF::from_points(vec2f(left, top), vec2f(right, bottom));
        let thumb_bounds = RectF::from_points(vec2f(left, thumb_top), vec2f(right, thumb_bottom));

        if layout.show_scrollbars {
            scene.push_quad(Quad {
                bounds: track_bounds,
                border: style.track.border,
                background: style.track.background_color,
                ..Default::default()
            });
            scene.push_quad(Quad {
                bounds: thumb_bounds,
                border: style.thumb.border,
                background: style.thumb.background_color,
                corner_radius: style.thumb.corner_radius,
            });
        }

        scene.push_cursor_region(CursorRegion {
            bounds: track_bounds,
            style: CursorStyle::Arrow,
        });
        scene.push_mouse_region(
            MouseRegion::new::<ScrollbarMouseHandlers>(cx.view_id(), cx.view_id(), track_bounds)
                .on_move(move |_, editor: &mut Editor, cx| {
                    editor.scroll_manager.show_scrollbar(cx);
                })
                .on_down(MouseButton::Left, {
                    let row_range = row_range.clone();
                    move |event, editor: &mut Editor, cx| {
                        let y = event.position.y();
                        if y < thumb_top || thumb_bottom < y {
                            let center_row = ((y - top) * max_row as f32 / height).round() as u32;
                            let top_row = center_row
                                .saturating_sub((row_range.end - row_range.start) as u32 / 2);
                            let mut position = editor.scroll_position(cx);
                            position.set_y(top_row as f32);
                            editor.set_scroll_position(position, cx);
                        } else {
                            editor.scroll_manager.show_scrollbar(cx);
                        }
                    }
                })
                .on_drag(MouseButton::Left, {
                    move |event, editor: &mut Editor, cx| {
                        let y = event.prev_mouse_position.y();
                        let new_y = event.position.y();
                        if thumb_top < y && y < thumb_bottom {
                            let mut position = editor.scroll_position(cx);
                            position.set_y(position.y() + (new_y - y) * (max_row as f32) / height);
                            if position.y() < 0.0 {
                                position.set_y(0.);
                            }
                            editor.set_scroll_position(position, cx);
                        }
                    }
                }),
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_highlighted_range(
        &self,
        scene: &mut SceneBuilder,
        range: Range<DisplayPoint>,
        color: Color,
        corner_radius: f32,
        line_end_overshoot: f32,
        layout: &LayoutState,
        content_origin: Vector2F,
        scroll_top: f32,
        scroll_left: f32,
        bounds: RectF,
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
                start_y: content_origin.y()
                    + row_range.start as f32 * layout.position_map.line_height
                    - scroll_top,
                lines: row_range
                    .into_iter()
                    .map(|row| {
                        let line_layout =
                            &layout.position_map.line_layouts[(row - start_row) as usize];
                        HighlightedRangeLine {
                            start_x: if row == range.start.row() {
                                content_origin.x()
                                    + line_layout.x_for_index(range.start.column() as usize)
                                    - scroll_left
                            } else {
                                content_origin.x() - scroll_left
                            },
                            end_x: if row == range.end.row() {
                                content_origin.x()
                                    + line_layout.x_for_index(range.end.column() as usize)
                                    - scroll_left
                            } else {
                                content_origin.x() + line_layout.width() + line_end_overshoot
                                    - scroll_left
                            },
                        }
                    })
                    .collect(),
            };

            highlighted_range.paint(bounds, scene);
        }
    }

    fn paint_blocks(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) {
        let scroll_position = layout.position_map.snapshot.scroll_position();
        let scroll_left = scroll_position.x() * layout.position_map.em_width;
        let scroll_top = scroll_position.y() * layout.position_map.line_height;

        for block in &mut layout.blocks {
            let mut origin = bounds.origin()
                + vec2f(
                    0.,
                    block.row as f32 * layout.position_map.line_height - scroll_top,
                );
            if !matches!(block.style, BlockStyle::Sticky) {
                origin += vec2f(-scroll_left, 0.);
            }
            block
                .element
                .paint(scene, origin, visible_bounds, editor, cx);
        }
    }

    fn max_line_number_width(&self, snapshot: &EditorSnapshot, cx: &ViewContext<Editor>) -> f32 {
        let digit_count = (snapshot.max_buffer_row() as f32).log10().floor() as usize + 1;
        let style = &self.style;

        cx.text_layout_cache()
            .layout_str(
                "1".repeat(digit_count).as_str(),
                style.text.font_size,
                &[(
                    digit_count,
                    RunStyle {
                        font_id: style.text.font_id,
                        color: Color::black(),
                        underline: Default::default(),
                    },
                )],
            )
            .width()
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
            .git_diff_hunks_in_range(buffer_start_row..buffer_end_row, false)
            .map(|hunk| diff_hunk_to_display(hunk, snapshot))
            .dedup()
            .collect()
    }

    fn layout_line_numbers(
        &self,
        rows: Range<u32>,
        active_rows: &BTreeMap<u32, bool>,
        is_singleton: bool,
        snapshot: &EditorSnapshot,
        cx: &ViewContext<Editor>,
    ) -> (
        Vec<Option<text_layout::Line>>,
        Vec<Option<(FoldStatus, BufferRow, bool)>>,
    ) {
        let style = &self.style;
        let include_line_numbers = snapshot.mode == EditorMode::Full;
        let mut line_number_layouts = Vec::with_capacity(rows.len());
        let mut fold_statuses = Vec::with_capacity(rows.len());
        let mut line_number = String::new();
        for (ix, row) in snapshot
            .buffer_rows(rows.start)
            .take((rows.end - rows.start) as usize)
            .enumerate()
        {
            let display_row = rows.start + ix as u32;
            let (active, color) = if active_rows.contains_key(&display_row) {
                (true, style.line_number_active)
            } else {
                (false, style.line_number)
            };
            if let Some(buffer_row) = row {
                if include_line_numbers {
                    line_number.clear();
                    write!(&mut line_number, "{}", buffer_row + 1).unwrap();
                    line_number_layouts.push(Some(cx.text_layout_cache().layout_str(
                        &line_number,
                        style.text.font_size,
                        &[(
                            line_number.len(),
                            RunStyle {
                                font_id: style.text.font_id,
                                color,
                                underline: Default::default(),
                            },
                        )],
                    )));
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
                line_number_layouts.push(None);
            }
        }

        (line_number_layouts, fold_statuses)
    }

    fn layout_lines(
        &mut self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        cx: &ViewContext<Editor>,
    ) -> Vec<text_layout::Line> {
        if rows.start >= rows.end {
            return Vec::new();
        }

        // When the editor is empty and unfocused, then show the placeholder.
        if snapshot.is_empty() {
            let placeholder_style = self
                .style
                .placeholder_text
                .as_ref()
                .unwrap_or(&self.style.text);
            let placeholder_text = snapshot.placeholder_text();
            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start as usize)
                .chain(iter::repeat(""))
                .take(rows.len());
            placeholder_lines
                .map(|line| {
                    cx.text_layout_cache().layout_str(
                        line,
                        placeholder_style.font_size,
                        &[(
                            line.len(),
                            RunStyle {
                                font_id: placeholder_style.font_id,
                                color: placeholder_style.color,
                                underline: Default::default(),
                            },
                        )],
                    )
                })
                .collect()
        } else {
            let style = &self.style;
            let chunks = snapshot
                .chunks(rows.clone(), true, Some(style.theme.suggestion))
                .map(|chunk| {
                    let mut highlight_style = chunk
                        .syntax_highlight_id
                        .and_then(|id| id.style(&style.syntax));

                    if let Some(chunk_highlight) = chunk.highlight_style {
                        if let Some(highlight_style) = highlight_style.as_mut() {
                            highlight_style.highlight(chunk_highlight);
                        } else {
                            highlight_style = Some(chunk_highlight);
                        }
                    }

                    let mut diagnostic_highlight = HighlightStyle::default();

                    if chunk.is_unnecessary {
                        diagnostic_highlight.fade_out = Some(style.unnecessary_code_fade);
                    }

                    if let Some(severity) = chunk.diagnostic_severity {
                        // Omit underlines for HINT/INFO diagnostics on 'unnecessary' code.
                        if severity <= DiagnosticSeverity::WARNING || !chunk.is_unnecessary {
                            let diagnostic_style = super::diagnostic_style(severity, true, style);
                            diagnostic_highlight.underline = Some(Underline {
                                color: Some(diagnostic_style.message.text.color),
                                thickness: 1.0.into(),
                                squiggly: true,
                            });
                        }
                    }

                    if let Some(highlight_style) = highlight_style.as_mut() {
                        highlight_style.highlight(diagnostic_highlight);
                    } else {
                        highlight_style = Some(diagnostic_highlight);
                    }

                    (chunk.text, highlight_style)
                });
            layout_highlighted_chunks(
                chunks,
                &style.text,
                cx.text_layout_cache(),
                cx.font_cache(),
                MAX_LINE_LEN,
                rows.len() as usize,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_blocks(
        &mut self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        editor_width: f32,
        scroll_width: f32,
        gutter_padding: f32,
        gutter_width: f32,
        em_width: f32,
        text_x: f32,
        line_height: f32,
        style: &EditorStyle,
        line_layouts: &[text_layout::Line],
        include_root: bool,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> (f32, Vec<BlockLayout>) {
        let tooltip_style = cx.global::<Settings>().theme.tooltip.clone();
        let scroll_x = snapshot.scroll_anchor.offset.x();
        let (fixed_blocks, non_fixed_blocks) = snapshot
            .blocks_in_range(rows.clone())
            .partition::<Vec<_>, _>(|(_, block)| match block {
                TransformBlock::ExcerptHeader { .. } => false,
                TransformBlock::Custom(block) => block.style() == BlockStyle::Fixed,
            });
        let mut render_block = |block: &TransformBlock, width: f32| {
            let mut element = match block {
                TransformBlock::Custom(block) => {
                    let align_to = block
                        .position()
                        .to_point(&snapshot.buffer_snapshot)
                        .to_display_point(snapshot);
                    let anchor_x = text_x
                        + if rows.contains(&align_to.row()) {
                            line_layouts[(align_to.row() - rows.start) as usize]
                                .x_for_index(align_to.column() as usize)
                        } else {
                            layout_line(align_to.row(), snapshot, style, cx.text_layout_cache())
                                .x_for_index(align_to.column() as usize)
                        };

                    block.render(&mut BlockContext {
                        view_context: cx,
                        anchor_x,
                        gutter_padding,
                        line_height,
                        scroll_x,
                        gutter_width,
                        em_width,
                    })
                }
                TransformBlock::ExcerptHeader {
                    id,
                    buffer,
                    range,
                    starts_new_buffer,
                    ..
                } => {
                    let id = *id;
                    let jump_icon = project::File::from_dyn(buffer.file()).map(|file| {
                        let jump_path = ProjectPath {
                            worktree_id: file.worktree_id(cx),
                            path: file.path.clone(),
                        };
                        let jump_anchor = range
                            .primary
                            .as_ref()
                            .map_or(range.context.start, |primary| primary.start);
                        let jump_position = language::ToPoint::to_point(&jump_anchor, buffer);

                        enum JumpIcon {}
                        MouseEventHandler::<JumpIcon, _>::new(id.into(), cx, |state, _| {
                            let style = style.jump_icon.style_for(state, false);
                            Svg::new("icons/arrow_up_right_8.svg")
                                .with_color(style.color)
                                .constrained()
                                .with_width(style.icon_width)
                                .aligned()
                                .contained()
                                .with_style(style.container)
                                .constrained()
                                .with_width(style.button_width)
                                .with_height(style.button_width)
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, move |_, editor, cx| {
                            if let Some(workspace) = editor
                                .workspace
                                .as_ref()
                                .and_then(|(workspace, _)| workspace.upgrade(cx))
                            {
                                workspace.update(cx, |workspace, cx| {
                                    Editor::jump(
                                        workspace,
                                        jump_path.clone(),
                                        jump_position,
                                        jump_anchor,
                                        cx,
                                    );
                                });
                            }
                        })
                        .with_tooltip::<JumpIcon>(
                            id.into(),
                            "Jump to Buffer".to_string(),
                            Some(Box::new(crate::OpenExcerpts)),
                            tooltip_style.clone(),
                            cx,
                        )
                        .aligned()
                        .flex_float()
                    });

                    if *starts_new_buffer {
                        let style = &self.style.diagnostic_path_header;
                        let font_size =
                            (style.text_scale_factor * self.style.text.font_size).round();

                        let path = buffer.resolve_file_path(cx, include_root);
                        let mut filename = None;
                        let mut parent_path = None;
                        // Can't use .and_then() because `.file_name()` and `.parent()` return references :(
                        if let Some(path) = path {
                            filename = path.file_name().map(|f| f.to_string_lossy().to_string());
                            parent_path =
                                path.parent().map(|p| p.to_string_lossy().to_string() + "/");
                        }

                        Flex::row()
                            .with_child(
                                Label::new(
                                    filename.unwrap_or_else(|| "untitled".to_string()),
                                    style.filename.text.clone().with_font_size(font_size),
                                )
                                .contained()
                                .with_style(style.filename.container)
                                .aligned(),
                            )
                            .with_children(parent_path.map(|path| {
                                Label::new(path, style.path.text.clone().with_font_size(font_size))
                                    .contained()
                                    .with_style(style.path.container)
                                    .aligned()
                            }))
                            .with_children(jump_icon)
                            .contained()
                            .with_style(style.container)
                            .with_padding_left(gutter_padding)
                            .with_padding_right(gutter_padding)
                            .expanded()
                            .into_any_named("path header block")
                    } else {
                        let text_style = self.style.text.clone();
                        Flex::row()
                            .with_child(Label::new("⋯", text_style))
                            .with_children(jump_icon)
                            .contained()
                            .with_padding_left(gutter_padding)
                            .with_padding_right(gutter_padding)
                            .expanded()
                            .into_any_named("collapsed context")
                    }
                }
            };

            element.layout(
                SizeConstraint {
                    min: Vector2F::zero(),
                    max: vec2f(width, block.height() as f32 * line_height),
                },
                editor,
                cx,
            );
            element
        };

        let mut fixed_block_max_width = 0f32;
        let mut blocks = Vec::new();
        for (row, block) in fixed_blocks {
            let element = render_block(block, f32::INFINITY);
            fixed_block_max_width = fixed_block_max_width.max(element.size().x() + em_width);
            blocks.push(BlockLayout {
                row,
                element,
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
                    .max(gutter_width + scroll_width),
                BlockStyle::Fixed => unreachable!(),
            };
            let element = render_block(block, width);
            blocks.push(BlockLayout {
                row,
                element,
                style,
            });
        }
        (
            scroll_width.max(fixed_block_max_width - gutter_width),
            blocks,
        )
    }
}

impl Element<Editor> for EditorElement {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        if size.x().is_infinite() {
            unimplemented!("we don't yet handle an infinite width constraint on buffer elements");
        }

        let snapshot = editor.snapshot(cx);
        let style = self.style.clone();
        let line_height = style.text.line_height(cx.font_cache());

        let gutter_padding;
        let gutter_width;
        let gutter_margin;
        if snapshot.mode == EditorMode::Full {
            gutter_padding = style.text.em_width(cx.font_cache()) * style.gutter_padding_factor;
            gutter_width = self.max_line_number_width(&snapshot, cx) + gutter_padding * 2.0;
            gutter_margin = -style.text.descent(cx.font_cache());
        } else {
            gutter_padding = 0.0;
            gutter_width = 0.0;
            gutter_margin = 0.0;
        };

        let text_width = size.x() - gutter_width;
        let em_width = style.text.em_width(cx.font_cache());
        let em_advance = style.text.em_advance(cx.font_cache());
        let overscroll = vec2f(em_width, 0.);
        let snapshot = {
            editor.set_visible_line_count(size.y() / line_height);

            let editor_width = text_width - gutter_margin - overscroll.x() - em_width;
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

        let scroll_height = (snapshot.max_point().row() + 1) as f32 * line_height;
        if let EditorMode::AutoHeight { max_lines } = snapshot.mode {
            size.set_y(
                scroll_height
                    .min(constraint.max_along(Axis::Vertical))
                    .max(constraint.min_along(Axis::Vertical))
                    .min(line_height * max_lines as f32),
            )
        } else if let EditorMode::SingleLine = snapshot.mode {
            size.set_y(
                line_height
                    .min(constraint.max_along(Axis::Vertical))
                    .max(constraint.min_along(Axis::Vertical)),
            )
        } else if size.y().is_infinite() {
            size.set_y(scroll_height);
        }
        let gutter_size = vec2f(gutter_width, size.y());
        let text_size = vec2f(text_width, size.y());

        let autoscroll_horizontally = editor.autoscroll_vertically(size.y(), line_height, cx);
        let mut snapshot = editor.snapshot(cx);

        let scroll_position = snapshot.scroll_position();
        // The scroll position is a fractional point, the whole number of which represents
        // the top of the window in terms of display rows.
        let start_row = scroll_position.y() as u32;
        let height_in_lines = size.y() / line_height;
        let max_row = snapshot.max_point().row();

        // Add 1 to ensure selections bleed off screen
        let end_row = 1 + cmp::min(
            (scroll_position.y() + height_in_lines).ceil() as u32,
            max_row,
        );

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

        let mut selections: Vec<(ReplicaId, Vec<SelectionLayout>)> = Vec::new();
        let mut active_rows = BTreeMap::new();
        let mut fold_ranges = Vec::new();
        let is_singleton = editor.is_singleton(cx);

        let highlighted_rows = editor.highlighted_rows();
        let theme = cx.global::<Settings>().theme.as_ref();
        let highlighted_ranges = editor.background_highlights_in_range(
            start_anchor..end_anchor,
            &snapshot.display_snapshot,
            theme,
        );

        fold_ranges.extend(
            snapshot
                .folds_in_range(start_anchor..end_anchor)
                .map(|anchor| {
                    let start = anchor.start.to_point(&snapshot.buffer_snapshot);
                    (
                        start.row,
                        start.to_display_point(&snapshot.display_snapshot)
                            ..anchor.end.to_display_point(&snapshot),
                    )
                }),
        );

        let mut remote_selections = HashMap::default();
        for (replica_id, line_mode, cursor_shape, selection) in snapshot
            .buffer_snapshot
            .remote_selections_in_range(&(start_anchor..end_anchor))
        {
            // The local selections match the leader's selections.
            if Some(replica_id) == editor.leader_replica_id {
                continue;
            }
            remote_selections
                .entry(replica_id)
                .or_insert(Vec::new())
                .push(SelectionLayout::new(
                    selection,
                    line_mode,
                    cursor_shape,
                    &snapshot.display_snapshot,
                ));
        }
        selections.extend(remote_selections);

        if editor.show_local_selections {
            let mut local_selections = editor
                .selections
                .disjoint_in_range(start_anchor..end_anchor, cx);
            local_selections.extend(editor.selections.pending(cx));
            for selection in &local_selections {
                let is_empty = selection.start == selection.end;
                let selection_start = snapshot.prev_line_boundary(selection.start).1;
                let selection_end = snapshot.next_line_boundary(selection.end).1;
                for row in cmp::max(selection_start.row(), start_row)
                    ..=cmp::min(selection_end.row(), end_row)
                {
                    let contains_non_empty_selection = active_rows.entry(row).or_insert(!is_empty);
                    *contains_non_empty_selection |= !is_empty;
                }
            }

            // Render the local selections in the leader's color when following.
            let local_replica_id = editor
                .leader_replica_id
                .unwrap_or_else(|| editor.replica_id(cx));

            selections.push((
                local_replica_id,
                local_selections
                    .into_iter()
                    .map(|selection| {
                        SelectionLayout::new(
                            selection,
                            editor.selections.line_mode,
                            editor.cursor_shape,
                            &snapshot.display_snapshot,
                        )
                    })
                    .collect(),
            ));
        }

        let show_scrollbars = editor.scroll_manager.scrollbars_visible();
        let include_root = editor
            .project
            .as_ref()
            .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
            .unwrap_or_default();

        let fold_ranges: Vec<(BufferRow, Range<DisplayPoint>, Color)> = fold_ranges
            .into_iter()
            .map(|(id, fold)| {
                let color = self
                    .style
                    .folds
                    .ellipses
                    .background
                    .style_for(&mut cx.mouse_state::<FoldMarkers>(id as usize), false)
                    .color;

                (id, fold, color)
            })
            .collect();

        let (line_number_layouts, fold_statuses) = self.layout_line_numbers(
            start_row..end_row,
            &active_rows,
            is_singleton,
            &snapshot,
            cx,
        );

        let display_hunks = self.layout_git_gutters(start_row..end_row, &snapshot);

        let scrollbar_row_range = scroll_position.y()..(scroll_position.y() + height_in_lines);

        let mut max_visible_line_width = 0.0;
        let line_layouts = self.layout_lines(start_row..end_row, &snapshot, cx);
        for line in &line_layouts {
            if line.width() > max_visible_line_width {
                max_visible_line_width = line.width();
            }
        }

        let style = self.style.clone();
        let longest_line_width = layout_line(
            snapshot.longest_row(),
            &snapshot,
            &style,
            cx.text_layout_cache(),
        )
        .width();
        let scroll_width = longest_line_width.max(max_visible_line_width) + overscroll.x();
        let em_width = style.text.em_width(cx.font_cache());
        let (scroll_width, blocks) = self.layout_blocks(
            start_row..end_row,
            &snapshot,
            size.x(),
            scroll_width,
            gutter_padding,
            gutter_width,
            em_width,
            gutter_width + gutter_margin,
            line_height,
            &style,
            &line_layouts,
            include_root,
            editor,
            cx,
        );

        let scroll_max = vec2f(
            ((scroll_width - text_size.x()) / em_width).max(0.0),
            max_row as f32,
        );

        let clamped = editor.scroll_manager.clamp_scroll_left(scroll_max.x());

        let autoscrolled = if autoscroll_horizontally {
            editor.autoscroll_horizontally(
                start_row,
                text_size.x(),
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

        let newest_selection_head = editor
            .selections
            .newest::<usize>(cx)
            .head()
            .to_display_point(&snapshot);
        let style = editor.style(cx);

        let mut context_menu = None;
        let mut code_actions_indicator = None;
        if (start_row..end_row).contains(&newest_selection_head.row()) {
            if editor.context_menu_visible() {
                context_menu = editor.render_context_menu(newest_selection_head, style.clone(), cx);
            }

            let active = matches!(
                editor.context_menu,
                Some(crate::ContextMenu::CodeActions(_))
            );

            code_actions_indicator = editor
                .render_code_actions_indicator(&style, active, cx)
                .map(|indicator| (newest_selection_head.row(), indicator));
        }

        let visible_rows = start_row..start_row + line_layouts.len() as u32;
        let mut hover = editor
            .hover_state
            .render(&snapshot, &style, visible_rows, cx);
        let mode = editor.mode;

        let mut fold_indicators = editor.render_fold_indicators(
            fold_statuses,
            &style,
            editor.gutter_hovered,
            line_height,
            gutter_margin,
            cx,
        );

        if let Some((_, context_menu)) = context_menu.as_mut() {
            context_menu.layout(
                SizeConstraint {
                    min: Vector2F::zero(),
                    max: vec2f(
                        cx.window_size().x() * 0.7,
                        (12. * line_height).min((size.y() - line_height) / 2.),
                    ),
                },
                editor,
                cx,
            );
        }

        if let Some((_, indicator)) = code_actions_indicator.as_mut() {
            indicator.layout(
                SizeConstraint::strict_along(
                    Axis::Vertical,
                    line_height * style.code_actions.vertical_scale,
                ),
                editor,
                cx,
            );
        }

        for fold_indicator in fold_indicators.iter_mut() {
            if let Some(indicator) = fold_indicator.as_mut() {
                indicator.layout(
                    SizeConstraint::strict_along(
                        Axis::Vertical,
                        line_height * style.code_actions.vertical_scale,
                    ),
                    editor,
                    cx,
                );
            }
        }

        if let Some((_, hover_popovers)) = hover.as_mut() {
            for hover_popover in hover_popovers.iter_mut() {
                hover_popover.layout(
                    SizeConstraint {
                        min: Vector2F::zero(),
                        max: vec2f(
                            (120. * em_width) // Default size
                                .min(size.x() / 2.) // Shrink to half of the editor width
                                .max(MIN_POPOVER_CHARACTER_WIDTH * em_width), // Apply minimum width of 20 characters
                            (16. * line_height) // Default size
                                .min(size.y() / 2.) // Shrink to half of the editor height
                                .max(MIN_POPOVER_LINE_HEIGHT * line_height), // Apply minimum height of 4 lines
                        ),
                    },
                    editor,
                    cx,
                );
            }
        }

        (
            size,
            LayoutState {
                mode,
                position_map: Arc::new(PositionMap {
                    size,
                    scroll_max,
                    line_layouts,
                    line_height,
                    em_width,
                    em_advance,
                    snapshot,
                }),
                visible_display_row_range: start_row..end_row,
                gutter_size,
                gutter_padding,
                text_size,
                scrollbar_row_range,
                show_scrollbars,
                max_row,
                gutter_margin,
                active_rows,
                highlighted_rows,
                highlighted_ranges,
                fold_ranges,
                line_number_layouts,
                display_hunks,
                blocks,
                selections,
                context_menu,
                code_actions_indicator,
                fold_indicators,
                hover_popovers: hover,
            },
        )
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();
        scene.push_layer(Some(visible_bounds));

        let gutter_bounds = RectF::new(bounds.origin(), layout.gutter_size);
        let text_bounds = RectF::new(
            bounds.origin() + vec2f(layout.gutter_size.x(), 0.0),
            layout.text_size,
        );

        Self::attach_mouse_handlers(
            scene,
            &layout.position_map,
            layout.hover_popovers.is_some(),
            visible_bounds,
            text_bounds,
            gutter_bounds,
            bounds,
            cx,
        );

        self.paint_background(scene, gutter_bounds, text_bounds, layout);
        if layout.gutter_size.x() > 0. {
            self.paint_gutter(scene, gutter_bounds, visible_bounds, layout, editor, cx);
        }
        self.paint_text(scene, text_bounds, visible_bounds, layout, editor, cx);

        scene.push_layer(Some(bounds));
        if !layout.blocks.is_empty() {
            self.paint_blocks(scene, bounds, visible_bounds, layout, editor, cx);
        }
        self.paint_scrollbar(scene, bounds, layout, cx);
        scene.pop_layer();

        scene.pop_layer();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        _: RectF,
        layout: &Self::LayoutState,
        _: &Self::PaintState,
        _: &Editor,
        _: &ViewContext<Editor>,
    ) -> Option<RectF> {
        let text_bounds = RectF::new(
            bounds.origin() + vec2f(layout.gutter_size.x(), 0.0),
            layout.text_size,
        );
        let content_origin = text_bounds.origin() + vec2f(layout.gutter_margin, 0.);
        let scroll_position = layout.position_map.snapshot.scroll_position();
        let start_row = scroll_position.y() as u32;
        let scroll_top = scroll_position.y() * layout.position_map.line_height;
        let scroll_left = scroll_position.x() * layout.position_map.em_width;

        let range_start = OffsetUtf16(range_utf16.start)
            .to_display_point(&layout.position_map.snapshot.display_snapshot);
        if range_start.row() < start_row {
            return None;
        }

        let line = layout
            .position_map
            .line_layouts
            .get((range_start.row() - start_row) as usize)?;
        let range_start_x = line.x_for_index(range_start.column() as usize);
        let range_start_y = range_start.row() as f32 * layout.position_map.line_height;
        Some(RectF::new(
            content_origin
                + vec2f(
                    range_start_x,
                    range_start_y + layout.position_map.line_height,
                )
                - vec2f(scroll_left, scroll_top),
            vec2f(
                layout.position_map.em_width,
                layout.position_map.line_height,
            ),
        ))
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &Editor,
        _: &ViewContext<Editor>,
    ) -> json::Value {
        json!({
            "type": "BufferElement",
            "bounds": bounds.to_json()
        })
    }
}

type BufferRow = u32;

pub struct LayoutState {
    position_map: Arc<PositionMap>,
    gutter_size: Vector2F,
    gutter_padding: f32,
    gutter_margin: f32,
    text_size: Vector2F,
    mode: EditorMode,
    visible_display_row_range: Range<u32>,
    active_rows: BTreeMap<u32, bool>,
    highlighted_rows: Option<Range<u32>>,
    line_number_layouts: Vec<Option<text_layout::Line>>,
    display_hunks: Vec<DisplayDiffHunk>,
    blocks: Vec<BlockLayout>,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Color)>,
    fold_ranges: Vec<(BufferRow, Range<DisplayPoint>, Color)>,
    selections: Vec<(ReplicaId, Vec<SelectionLayout>)>,
    scrollbar_row_range: Range<f32>,
    show_scrollbars: bool,
    max_row: u32,
    context_menu: Option<(DisplayPoint, AnyElement<Editor>)>,
    code_actions_indicator: Option<(u32, AnyElement<Editor>)>,
    hover_popovers: Option<(DisplayPoint, Vec<AnyElement<Editor>>)>,
    fold_indicators: Vec<Option<AnyElement<Editor>>>,
}

pub struct PositionMap {
    size: Vector2F,
    line_height: f32,
    scroll_max: Vector2F,
    em_width: f32,
    em_advance: f32,
    line_layouts: Vec<text_layout::Line>,
    snapshot: EditorSnapshot,
}

impl PositionMap {
    /// Returns two display points:
    /// 1. The nearest *valid* position in the editor
    /// 2. An unclipped, potentially *invalid* position that maps directly to
    ///    the given pixel position.
    fn point_for_position(
        &self,
        text_bounds: RectF,
        position: Vector2F,
    ) -> (DisplayPoint, DisplayPoint) {
        let scroll_position = self.snapshot.scroll_position();
        let position = position - text_bounds.origin();
        let y = position.y().max(0.0).min(self.size.y());
        let x = position.x() + (scroll_position.x() * self.em_width);
        let row = (y / self.line_height + scroll_position.y()) as u32;
        let (column, x_overshoot) = if let Some(line) = self
            .line_layouts
            .get(row as usize - scroll_position.y() as usize)
        {
            if let Some(ix) = line.index_for_x(x) {
                (ix as u32, 0.0)
            } else {
                (line.len() as u32, 0f32.max(x - line.width()))
            }
        } else {
            (0, x)
        };

        let mut target_point = DisplayPoint::new(row, column);
        let point = self.snapshot.clip_point(target_point, Bias::Left);
        *target_point.column_mut() += (x_overshoot / self.em_advance) as u32;

        (point, target_point)
    }
}

struct BlockLayout {
    row: u32,
    element: AnyElement<Editor>,
    style: BlockStyle,
}

fn layout_line(
    row: u32,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    layout_cache: &TextLayoutCache,
) -> text_layout::Line {
    let mut line = snapshot.line(row);

    if line.len() > MAX_LINE_LEN {
        let mut len = MAX_LINE_LEN;
        while !line.is_char_boundary(len) {
            len -= 1;
        }

        line.truncate(len);
    }

    layout_cache.layout_str(
        &line,
        style.text.font_size,
        &[(
            snapshot.line_len(row) as usize,
            RunStyle {
                font_id: style.text.font_id,
                color: Color::black(),
                underline: Default::default(),
            },
        )],
    )
}

#[derive(Debug)]
pub struct Cursor {
    origin: Vector2F,
    block_width: f32,
    line_height: f32,
    color: Color,
    shape: CursorShape,
    block_text: Option<Line>,
}

impl Cursor {
    pub fn new(
        origin: Vector2F,
        block_width: f32,
        line_height: f32,
        color: Color,
        shape: CursorShape,
        block_text: Option<Line>,
    ) -> Cursor {
        Cursor {
            origin,
            block_width,
            line_height,
            color,
            shape,
            block_text,
        }
    }

    pub fn bounding_rect(&self, origin: Vector2F) -> RectF {
        RectF::new(
            self.origin + origin,
            vec2f(self.block_width, self.line_height),
        )
    }

    pub fn paint(&self, scene: &mut SceneBuilder, origin: Vector2F, cx: &mut WindowContext) {
        let bounds = match self.shape {
            CursorShape::Bar => RectF::new(self.origin + origin, vec2f(2.0, self.line_height)),
            CursorShape::Block | CursorShape::Hollow => RectF::new(
                self.origin + origin,
                vec2f(self.block_width, self.line_height),
            ),
            CursorShape::Underscore => RectF::new(
                self.origin + origin + Vector2F::new(0.0, self.line_height - 2.0),
                vec2f(self.block_width, 2.0),
            ),
        };

        //Draw background or border quad
        if matches!(self.shape, CursorShape::Hollow) {
            scene.push_quad(Quad {
                bounds,
                background: None,
                border: Border::all(1., self.color),
                corner_radius: 0.,
            });
        } else {
            scene.push_quad(Quad {
                bounds,
                background: Some(self.color),
                border: Default::default(),
                corner_radius: 0.,
            });
        }

        if let Some(block_text) = &self.block_text {
            block_text.paint(scene, self.origin + origin, bounds, self.line_height, cx);
        }
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }
}

#[derive(Debug)]
pub struct HighlightedRange {
    pub start_y: f32,
    pub line_height: f32,
    pub lines: Vec<HighlightedRangeLine>,
    pub color: Color,
    pub corner_radius: f32,
}

#[derive(Debug)]
pub struct HighlightedRangeLine {
    pub start_x: f32,
    pub end_x: f32,
}

impl HighlightedRange {
    pub fn paint(&self, bounds: RectF, scene: &mut SceneBuilder) {
        if self.lines.len() >= 2 && self.lines[0].start_x > self.lines[1].end_x {
            self.paint_lines(self.start_y, &self.lines[0..1], bounds, scene);
            self.paint_lines(
                self.start_y + self.line_height,
                &self.lines[1..],
                bounds,
                scene,
            );
        } else {
            self.paint_lines(self.start_y, &self.lines, bounds, scene);
        }
    }

    fn paint_lines(
        &self,
        start_y: f32,
        lines: &[HighlightedRangeLine],
        bounds: RectF,
        scene: &mut SceneBuilder,
    ) {
        if lines.is_empty() {
            return;
        }

        let mut path = PathBuilder::new();
        let first_line = lines.first().unwrap();
        let last_line = lines.last().unwrap();

        let first_top_left = vec2f(first_line.start_x, start_y);
        let first_top_right = vec2f(first_line.end_x, start_y);

        let curve_height = vec2f(0., self.corner_radius);
        let curve_width = |start_x: f32, end_x: f32| {
            let max = (end_x - start_x) / 2.;
            let width = if max < self.corner_radius {
                max
            } else {
                self.corner_radius
            };

            vec2f(width, 0.)
        };

        let top_curve_width = curve_width(first_line.start_x, first_line.end_x);
        path.reset(first_top_right - top_curve_width);
        path.curve_to(first_top_right + curve_height, first_top_right);

        let mut iter = lines.iter().enumerate().peekable();
        while let Some((ix, line)) = iter.next() {
            let bottom_right = vec2f(line.end_x, start_y + (ix + 1) as f32 * self.line_height);

            if let Some((_, next_line)) = iter.peek() {
                let next_top_right = vec2f(next_line.end_x, bottom_right.y());

                match next_top_right.x().partial_cmp(&bottom_right.x()).unwrap() {
                    Ordering::Equal => {
                        path.line_to(bottom_right);
                    }
                    Ordering::Less => {
                        let curve_width = curve_width(next_top_right.x(), bottom_right.x());
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > 0. {
                            path.curve_to(bottom_right - curve_width, bottom_right);
                        }
                        path.line_to(next_top_right + curve_width);
                        if self.corner_radius > 0. {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                    Ordering::Greater => {
                        let curve_width = curve_width(bottom_right.x(), next_top_right.x());
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > 0. {
                            path.curve_to(bottom_right + curve_width, bottom_right);
                        }
                        path.line_to(next_top_right - curve_width);
                        if self.corner_radius > 0. {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                }
            } else {
                let curve_width = curve_width(line.start_x, line.end_x);
                path.line_to(bottom_right - curve_height);
                if self.corner_radius > 0. {
                    path.curve_to(bottom_right - curve_width, bottom_right);
                }

                let bottom_left = vec2f(line.start_x, bottom_right.y());
                path.line_to(bottom_left + curve_width);
                if self.corner_radius > 0. {
                    path.curve_to(bottom_left - curve_height, bottom_left);
                }
            }
        }

        if first_line.start_x > last_line.start_x {
            let curve_width = curve_width(last_line.start_x, first_line.start_x);
            let second_top_left = vec2f(last_line.start_x, start_y + self.line_height);
            path.line_to(second_top_left + curve_height);
            if self.corner_radius > 0. {
                path.curve_to(second_top_left + curve_width, second_top_left);
            }
            let first_bottom_left = vec2f(first_line.start_x, second_top_left.y());
            path.line_to(first_bottom_left - curve_width);
            if self.corner_radius > 0. {
                path.curve_to(first_bottom_left - curve_height, first_bottom_left);
            }
        }

        path.line_to(first_top_left + curve_height);
        if self.corner_radius > 0. {
            path.curve_to(first_top_left + top_curve_width, first_top_left);
        }
        path.line_to(first_top_right - top_curve_width);

        scene.push_path(path.build(self.color, Some(bounds)));
    }
}

pub fn position_to_display_point(
    position: Vector2F,
    text_bounds: RectF,
    position_map: &PositionMap,
) -> Option<DisplayPoint> {
    if text_bounds.contains_point(position) {
        let (point, target_point) = position_map.point_for_position(text_bounds, position);
        if point == target_point {
            Some(point)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn range_to_bounds(
    range: &Range<DisplayPoint>,
    content_origin: Vector2F,
    scroll_left: f32,
    scroll_top: f32,
    visible_row_range: &Range<u32>,
    line_end_overshoot: f32,
    position_map: &PositionMap,
) -> impl Iterator<Item = RectF> {
    let mut bounds: SmallVec<[RectF; 1]> = SmallVec::new();

    if range.start == range.end {
        return bounds.into_iter();
    }

    let start_row = visible_row_range.start;
    let end_row = visible_row_range.end;

    let row_range = if range.end.column() == 0 {
        cmp::max(range.start.row(), start_row)..cmp::min(range.end.row(), end_row)
    } else {
        cmp::max(range.start.row(), start_row)..cmp::min(range.end.row() + 1, end_row)
    };

    let first_y =
        content_origin.y() + row_range.start as f32 * position_map.line_height - scroll_top;

    for (idx, row) in row_range.enumerate() {
        let line_layout = &position_map.line_layouts[(row - start_row) as usize];

        let start_x = if row == range.start.row() {
            content_origin.x() + line_layout.x_for_index(range.start.column() as usize)
                - scroll_left
        } else {
            content_origin.x() - scroll_left
        };

        let end_x = if row == range.end.row() {
            content_origin.x() + line_layout.x_for_index(range.end.column() as usize) - scroll_left
        } else {
            content_origin.x() + line_layout.width() + line_end_overshoot - scroll_left
        };

        bounds.push(RectF::from_points(
            vec2f(start_x, first_y + position_map.line_height * idx as f32),
            vec2f(end_x, first_y + position_map.line_height * (idx + 1) as f32),
        ))
    }

    bounds.into_iter()
}

pub fn scale_vertical_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.5) / 100.0
}

fn scale_horizontal_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.2) / 300.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{BlockDisposition, BlockProperties},
        Editor, MultiBuffer,
    };
    use gpui::TestAppContext;
    use settings::Settings;
    use std::sync::Arc;
    use util::test::sample_text;

    #[gpui::test]
    fn test_layout_line_numbers(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let (_, editor) = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
            Editor::new(EditorMode::Full, buffer, None, None, cx)
        });
        let element = EditorElement::new(editor.read_with(cx, |editor, cx| editor.style(cx)));

        let layouts = editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            element
                .layout_line_numbers(0..6, &Default::default(), false, &snapshot, cx)
                .0
        });
        assert_eq!(layouts.len(), 6);
    }

    #[gpui::test]
    fn test_layout_with_placeholder_text_and_blocks(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::test(cx)));
        let (_, editor) = cx.add_window(|cx| {
            let buffer = MultiBuffer::build_simple("", cx);
            Editor::new(EditorMode::Full, buffer, None, None, cx)
        });

        editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("hello", cx);
            editor.insert_blocks(
                [BlockProperties {
                    style: BlockStyle::Fixed,
                    disposition: BlockDisposition::Above,
                    height: 3,
                    position: Anchor::min(),
                    render: Arc::new(|_| Empty::new().into_any()),
                }],
                cx,
            );

            // Blur the editor so that it displays placeholder text.
            cx.blur();
        });

        let mut element = EditorElement::new(editor.read_with(cx, |editor, cx| editor.style(cx)));
        let (size, mut state) = editor.update(cx, |editor, cx| {
            element.layout(
                SizeConstraint::new(vec2f(500., 500.), vec2f(500., 500.)),
                editor,
                cx,
            )
        });

        assert_eq!(state.position_map.line_layouts.len(), 4);
        assert_eq!(
            state
                .line_number_layouts
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>(),
            &[false, false, false, true]
        );

        // Don't panic.
        let mut scene = SceneBuilder::new(1.0);
        let bounds = RectF::new(Default::default(), size);
        editor.update(cx, |editor, cx| {
            element.paint(&mut scene, bounds, bounds, &mut state, editor, cx);
        });
    }
}
