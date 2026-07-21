use std::ops::Range;
use std::time::{Duration, Instant};

use collections::HashMap;
use feature_flags::{DiffReviewFeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    AnyElement, App, AvailableSpace, ClickEvent, Context, DefiniteLength, DispatchPhase, Element,
    MouseButton, MouseClickEvent, MouseDownEvent, MouseMoveEvent, MousePressureEvent, MouseUpEvent,
    ParentElement, Pixels, PressureStage, ScrollDelta, ScrollWheelEvent, TextStyleRefinement,
    Window, anchored, deferred, point, px,
};
use multi_buffer::MultiBufferRow;
use project::DisableAiSettings;
use settings::Settings;
use sum_tree::Bias;
use text::SelectionGoal;
use theme_settings::BufferLineHeight;
use util::{RangeExt, debug_panic, post_inc};

use super::{EditorElement, EditorLayout, LineNumberLayout, PositionMap, SplitSide};
use crate::{
    CURSORS_VISIBLE_FOR, ColumnarMode, DisplayDiffHunk, DisplayPoint, DisplayRow, Editor,
    EditorSettings, EditorSnapshot, GutterHoverButton, HoveredCursor, JumpData,
    PhantomDiffReviewIndicator, SelectPhase, Selection, SelectionDragState,
    display_map::ToDisplayPoint, editor_settings::DoubleClickInMultibuffer,
    hover_popover::hover_at, mouse_context_menu, scroll::ScrollPixelOffset,
};

impl EditorElement {
    pub(crate) fn mouse_moved(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        split_side: Option<SplitSide>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let text_hitbox = &position_map.text_hitbox;
        let gutter_hitbox = &position_map.gutter_hitbox;
        let modifiers = event.modifiers;
        let text_hovered = text_hitbox.is_hovered(window);
        let gutter_hovered = gutter_hitbox.is_hovered(window);
        editor.set_gutter_hovered(gutter_hovered, cx);

        let point_for_position = position_map.point_for_position(event.position);
        let valid_point = point_for_position.nearest_valid;

        // Update diff review drag state if we're dragging
        if editor.diff_review_drag_state.is_some() {
            editor.update_diff_review_drag(valid_point.row(), window, cx);
        }

        let hovered_diff_control = position_map
            .diff_hunk_control_bounds
            .iter()
            .find(|(_, bounds)| bounds.contains(&event.position))
            .map(|(row, _)| *row);

        let hovered_diff_hunk_row = if let Some(control_row) = hovered_diff_control {
            Some(control_row)
        } else if text_hovered {
            let current_row = valid_point.row();
            position_map.display_hunks.iter().find_map(|(hunk, _)| {
                if let DisplayDiffHunk::Unfolded {
                    display_row_range, ..
                } = hunk
                {
                    if display_row_range.contains(&current_row) {
                        Some(display_row_range.start)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        } else {
            None
        };

        if hovered_diff_hunk_row != editor.hovered_diff_hunk_row {
            editor.hovered_diff_hunk_row = hovered_diff_hunk_row;
            cx.notify();
        }

        if text_hovered
            && let Some((bounds, buffer_id, blame_entry)) = &position_map.inline_blame_bounds
        {
            let mouse_over_inline_blame = bounds.contains(&event.position);
            let mouse_over_popover = editor
                .inline_blame_popover
                .as_ref()
                .and_then(|state| state.popover_bounds)
                .is_some_and(|bounds| bounds.contains(&event.position));
            let keyboard_grace = editor
                .inline_blame_popover
                .as_ref()
                .is_some_and(|state| state.keyboard_grace);

            if mouse_over_inline_blame || mouse_over_popover {
                editor.show_blame_popover(*buffer_id, blame_entry, event.position, false, cx);
            } else if !keyboard_grace {
                editor.hide_blame_popover(false, cx);
            }
        } else {
            let keyboard_grace = editor
                .inline_blame_popover
                .as_ref()
                .is_some_and(|state| state.keyboard_grace);
            if !keyboard_grace {
                editor.hide_blame_popover(false, cx);
            }
        }

        // Handle diff review indicator when gutter is hovered in diff mode with AI enabled
        let show_diff_review = editor.show_diff_review_button()
            && cx.has_flag::<DiffReviewFeatureFlag>()
            && !DisableAiSettings::is_ai_disabled_for_buffer(
                editor.buffer.read(cx).as_singleton().as_ref(),
                cx,
            );

        let diff_review_indicator = if gutter_hovered && show_diff_review {
            let is_visible = editor
                .gutter_diff_review_indicator
                .0
                .is_some_and(|indicator| indicator.is_active);

            if !is_visible {
                editor
                    .gutter_diff_review_indicator
                    .1
                    .get_or_insert_with(|| {
                        cx.spawn(async move |this, cx| {
                            cx.background_executor()
                                .timer(Duration::from_millis(200))
                                .await;

                            this.update(cx, |this, cx| {
                                if let Some(indicator) =
                                    this.gutter_diff_review_indicator.0.as_mut()
                                {
                                    indicator.is_active = true;
                                    cx.notify();
                                }
                            })
                            .ok();
                        })
                    });
            }

            let anchor = position_map
                .snapshot
                .display_point_to_anchor(valid_point, Bias::Left);
            Some(PhantomDiffReviewIndicator {
                start: anchor,
                end: anchor,
                is_active: is_visible,
            })
        } else {
            editor.gutter_diff_review_indicator.1 = None;
            None
        };

        if diff_review_indicator != editor.gutter_diff_review_indicator.0 {
            editor.gutter_diff_review_indicator.0 = diff_review_indicator;
            cx.notify();
        }

        // Don't show breakpoint indicator when diff review indicator is active on this row
        let is_on_diff_review_button_row = diff_review_indicator.is_some_and(|indicator| {
            let start_row = indicator
                .start
                .to_display_point(&position_map.snapshot.display_snapshot)
                .row();
            indicator.is_active && start_row == valid_point.row()
        });

        let gutter_hover_button = if gutter_hovered
            && !is_on_diff_review_button_row
            && split_side != Some(SplitSide::Left)
        {
            let buffer_anchor = position_map
                .snapshot
                .display_point_to_anchor(valid_point, Bias::Left);

            if position_map
                .snapshot
                .buffer_snapshot()
                .anchor_to_buffer_anchor(buffer_anchor)
                .is_some()
            {
                let is_visible = editor
                    .gutter_hover_button
                    .0
                    .is_some_and(|indicator| indicator.is_active);

                if !is_visible {
                    editor.gutter_hover_button.1.get_or_insert_with(|| {
                        cx.spawn(async move |this, cx| {
                            cx.background_executor()
                                .timer(Duration::from_millis(200))
                                .await;

                            this.update(cx, |this, cx| {
                                if let Some(indicator) = this.gutter_hover_button.0.as_mut() {
                                    indicator.is_active = true;
                                    cx.notify();
                                }
                            })
                            .ok();
                        })
                    });
                }

                Some(GutterHoverButton {
                    display_row: valid_point.row(),
                    is_active: is_visible,
                })
            } else {
                editor.gutter_hover_button.1 = None;
                None
            }
        } else if editor.has_mouse_context_menu() {
            editor.gutter_hover_button.1 = None;
            editor.gutter_hover_button.0
        } else {
            editor.gutter_hover_button.1 = None;
            None
        };

        if &gutter_hover_button != &editor.gutter_hover_button.0 {
            editor.gutter_hover_button.0 = gutter_hover_button;
            cx.notify();
        }

        // Don't trigger hover popover if mouse is hovering over context menu
        if text_hovered {
            editor.update_hovered_link(
                point_for_position,
                Some(event.position),
                &position_map.snapshot,
                modifiers,
                window,
                cx,
            );

            if let Some(point) = point_for_position.as_valid() {
                let anchor = position_map
                    .snapshot
                    .buffer_snapshot()
                    .anchor_before(point.to_offset(&position_map.snapshot, Bias::Left));
                hover_at(editor, Some(anchor), Some(event.position), window, cx);
                Self::update_visible_cursor(editor, point, position_map, window, cx);
            } else {
                editor.update_inlay_link_and_hover_points(
                    &position_map.snapshot,
                    point_for_position,
                    Some(event.position),
                    modifiers.secondary(),
                    modifiers.shift,
                    window,
                    cx,
                );
            }
        } else {
            editor.hide_hovered_link(cx);
            hover_at(editor, None, Some(event.position), window, cx);
        }
    }

    pub(super) fn layout_mouse_context_menu(
        &self,
        editor_snapshot: &EditorSnapshot,
        visible_range: Range<DisplayRow>,
        content_origin: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let position = self.editor.update(cx, |editor, cx| {
            let visible_start_point = editor.display_to_pixel_point(
                DisplayPoint::new(visible_range.start, 0),
                editor_snapshot,
                window,
                cx,
            )?;
            let visible_end_point = editor.display_to_pixel_point(
                DisplayPoint::new(visible_range.end, 0),
                editor_snapshot,
                window,
                cx,
            )?;

            let mouse_context_menu = editor.mouse_context_menu.as_ref()?;
            let (source_display_point, position) = match mouse_context_menu.position {
                mouse_context_menu::MenuPosition::PinnedToScreen(point) => (None, point),
                mouse_context_menu::MenuPosition::PinnedToEditor { source, offset } => {
                    let source_display_point = source.to_display_point(editor_snapshot);
                    let source_point =
                        editor.to_pixel_point(source, editor_snapshot, window, cx)?;
                    let position = content_origin + source_point + offset;
                    (Some(source_display_point), position)
                }
            };

            let source_included = source_display_point.is_none_or(|source_display_point| {
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
            let mut element = self.editor.read_with(cx, |editor, _| {
                let mouse_context_menu = editor.mouse_context_menu.as_ref()?;
                let context_menu = mouse_context_menu.context_menu.clone();

                Some(
                    deferred(
                        anchored()
                            .position(position)
                            .child(context_menu)
                            .anchor(gpui::Anchor::TopLeft)
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

    pub(super) fn paint_mouse_listeners(
        &mut self,
        layout: &EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        if layout.mode.is_minimap() {
            return;
        }

        self.paint_scroll_wheel_listener(layout, window, cx);

        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
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
                        let event = ClickEvent::Mouse(MouseClickEvent {
                            down: mouse_down,
                            up: event.clone(),
                        });
                        Self::click(editor, &event, &position_map, window, cx);
                    }
                }),
            }
        });

        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();

            move |event: &MousePressureEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    editor.update(cx, |editor, cx| {
                        Self::pressure_click(editor, &event, &position_map, window, cx);
                    })
                }
            }
        });

        window.on_mouse_event({
            let position_map = layout.position_map.clone();
            let editor = self.editor.clone();
            let split_side = self.split_side;

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

                        Self::mouse_moved(editor, event, &position_map, split_side, window, cx)
                    });
                }
            }
        });
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
            let base_scroll_sensitivity =
                EditorSettings::get_global(cx).scroll_sensitivity.max(0.01);

            // Use a minimum fast_scroll_sensitivity for same reason above
            let fast_scroll_sensitivity = EditorSettings::get_global(cx)
                .fast_scroll_sensitivity
                .max(0.01);

            move |event: &ScrollWheelEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
                    delta = delta.coalesce(event.delta);

                    if event.modifiers.secondary()
                        && editor.read(cx).enable_mouse_wheel_zoom
                        && EditorSettings::get_global(cx).mouse_wheel_zoom
                    {
                        let delta_y = match event.delta {
                            ScrollDelta::Pixels(pixels) => pixels.y.into(),
                            ScrollDelta::Lines(lines) => lines.y,
                        };

                        if delta_y > 0.0 {
                            theme_settings::increase_buffer_font_size(cx);
                        } else if delta_y < 0.0 {
                            theme_settings::decrease_buffer_font_size(cx);
                        }

                        cx.stop_propagation();
                    } else {
                        let scroll_sensitivity = {
                            if event.modifiers.alt {
                                fast_scroll_sensitivity
                            } else {
                                base_scroll_sensitivity
                            }
                        };

                        editor.update(cx, |editor, cx| {
                            let line_height = position_map.line_height;
                            let glyph_width = position_map.em_layout_width;
                            let (delta, axis) = match delta {
                                gpui::ScrollDelta::Pixels(mut pixels) => {
                                    //Trackpad
                                    let axis =
                                        position_map.snapshot.ongoing_scroll.filter(&mut pixels);
                                    (pixels, axis)
                                }

                                gpui::ScrollDelta::Lines(lines) => {
                                    //Not trackpad
                                    let pixels =
                                        point(lines.x * glyph_width, lines.y * line_height);
                                    (pixels, None)
                                }
                            };

                            let current_scroll_position = position_map.snapshot.scroll_position();
                            let x = (current_scroll_position.x
                                * ScrollPixelOffset::from(glyph_width)
                                - ScrollPixelOffset::from(delta.x * scroll_sensitivity))
                                / ScrollPixelOffset::from(glyph_width);
                            let y = (current_scroll_position.y
                                * ScrollPixelOffset::from(line_height)
                                - ScrollPixelOffset::from(delta.y * scroll_sensitivity))
                                / ScrollPixelOffset::from(line_height);
                            let mut scroll_position =
                                point(x, y).clamp(&point(0., 0.), &position_map.scroll_max);
                            let forbid_vertical_scroll =
                                editor.scroll_manager.forbid_vertical_scroll();
                            if forbid_vertical_scroll {
                                scroll_position.y = current_scroll_position.y;
                            }

                            if scroll_position != current_scroll_position {
                                editor.scroll(scroll_position, axis, window, cx);
                                cx.stop_propagation();
                            } else if y < 0. && !forbid_vertical_scroll {
                                // Due to clamping, we may fail to detect cases of overscroll to the top;
                                // We want the scroll manager to get an update in such cases and detect the change of direction
                                // on the next frame.
                                if editor.scroll_manager.should_notify_top_overscroll(axis) {
                                    cx.notify();
                                }
                            } else {
                                editor.scroll_manager.reset_top_overscroll_notification();
                            }
                        });
                    }
                }
            }
        });
    }

    fn mouse_left_down(
        editor: &mut Editor,
        event: &MouseDownEvent,
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
        let point_for_position = position_map.point_for_position(event.position);
        let mut click_count = event.click_count;
        let mut modifiers = event.modifiers;

        if let Some(hovered_hunk) =
            position_map
                .display_hunks
                .iter()
                .find_map(|(hunk, hunk_hitbox)| match hunk {
                    DisplayDiffHunk::Folded { .. } => None,
                    DisplayDiffHunk::Unfolded {
                        multi_buffer_range, ..
                    } => hunk_hitbox
                        .as_ref()
                        .is_some_and(|hitbox| hitbox.is_hovered(window))
                        .then(|| multi_buffer_range.clone()),
                })
        {
            editor.toggle_single_diff_hunk(hovered_hunk, cx);
            cx.notify();
            return;
        } else if gutter_hitbox.is_hovered(window) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !text_hitbox.is_hovered(window) {
            return;
        }

        if EditorSettings::get_global(cx)
            .drag_and_drop_selection
            .enabled
            && click_count == 1
            && !modifiers.shift
        {
            let newest_anchor = editor.selections.newest_anchor();
            let snapshot = editor.snapshot(window, cx);
            let selection = newest_anchor.map(|anchor| anchor.to_display_point(&snapshot));
            if point_for_position.intersects_selection(&selection) {
                editor.selection_drag_state = SelectionDragState::ReadyToDrag {
                    selection: newest_anchor.clone(),
                    click_position: event.position,
                    mouse_down_time: Instant::now(),
                };
                cx.stop_propagation();
                return;
            }
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
                        let scroll_position_row = position_map.scroll_position.y;
                        let display_row = (((event.position - gutter_hitbox.bounds.origin).y
                            / position_map.line_height)
                            as f64
                            + position_map.scroll_position.y)
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

        if !is_singleton {
            let display_row = (ScrollPixelOffset::from(
                (event.position - gutter_hitbox.bounds.origin).y / position_map.line_height,
            ) + position_map.scroll_position.y) as u32;
            let multi_buffer_row = position_map
                .snapshot
                .display_point_to_point(DisplayPoint::new(DisplayRow(display_row), 0), Bias::Right)
                .row;
            if line_numbers
                .get(&MultiBufferRow(multi_buffer_row))
                .is_some_and(|line_layout| {
                    line_layout.segments.iter().any(|segment| {
                        segment
                            .hitbox
                            .as_ref()
                            .is_some_and(|hitbox| hitbox.contains(&event.position))
                    })
                })
            {
                let line_offset_from_top = display_row - position_map.scroll_position.y as u32;

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
                return;
            }
        }

        let position = point_for_position.nearest_valid;
        if let Some(mode) = Editor::columnar_selection_mode(&modifiers, cx) {
            editor.select(
                SelectPhase::BeginColumnar {
                    position,
                    reset: match mode {
                        ColumnarMode::FromMouse => true,
                        ColumnarMode::FromSelection => false,
                    },
                    mode,
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
            editor.select(
                SelectPhase::Begin {
                    position,
                    add: Editor::is_alt_pressed(&modifiers, cx),
                    click_count,
                },
                window,
                cx,
            );
        }
        cx.stop_propagation();
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

            if event.position.x <= hitbox.bounds.right() - gutter_right_padding
                // Don't show the gutter_context_menu in collab notes
                && editor.project.is_some()
            {
                let point_for_position = position_map.point_for_position(event.position);
                editor.set_gutter_context_menu(
                    point_for_position.nearest_valid.row(),
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
            point_for_position.nearest_valid,
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
        let position = point_for_position.nearest_valid;

        editor.select(
            SelectPhase::BeginColumnar {
                position,
                reset: true,
                mode: ColumnarMode::FromMouse,
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
        // Handle diff review drag completion
        if editor.diff_review_drag_state.is_some() {
            editor.end_diff_review_drag(window, cx);
            cx.stop_propagation();
            return;
        }

        let text_hitbox = &position_map.text_hitbox;
        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();
        let point_for_position = position_map.point_for_position(event.position);

        match editor.selection_drag_state {
            SelectionDragState::ReadyToDrag {
                selection: _,
                ref click_position,
                mouse_down_time: _,
            } => {
                if event.position == *click_position {
                    editor.select(
                        SelectPhase::Begin {
                            position: point_for_position.nearest_valid,
                            add: false,
                            click_count: 1, // ready to drag state only occurs on click count 1
                        },
                        window,
                        cx,
                    );
                    editor.selection_drag_state = SelectionDragState::None;
                    cx.stop_propagation();
                    return;
                } else {
                    debug_panic!("drag state can never be in ready state after drag")
                }
            }
            SelectionDragState::Dragging { ref selection, .. } => {
                let snapshot = editor.snapshot(window, cx);
                let selection_display = selection.map(|anchor| anchor.to_display_point(&snapshot));
                if !point_for_position.intersects_selection(&selection_display)
                    && text_hitbox.is_hovered(window)
                {
                    let is_cut = !(cfg!(target_os = "macos") && event.modifiers.alt
                        || cfg!(not(target_os = "macos")) && event.modifiers.control);
                    editor.move_selection_on_drop(
                        &selection.clone(),
                        point_for_position.nearest_valid,
                        is_cut,
                        window,
                        cx,
                    );
                }
                editor.selection_drag_state = SelectionDragState::None;
                cx.stop_propagation();
                cx.notify();
                return;
            }
            _ => {}
        }

        if end_selection {
            editor.select(SelectPhase::End, window, cx);
        }

        if end_selection && pending_nonempty_selections {
            cx.stop_propagation();
        } else if cfg!(any(target_os = "linux", target_os = "freebsd"))
            && event.button == MouseButton::Middle
        {
            #[allow(
                clippy::collapsible_if,
                clippy::needless_return,
                reason = "The cfg-block below makes this a false positive"
            )]
            if !text_hitbox.is_hovered(window) || editor.read_only(cx) {
                return;
            }

            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            if EditorSettings::get_global(cx).middle_click_paste {
                if let Some(text) = cx.read_from_primary().and_then(|item| item.text()) {
                    let point_for_position = position_map.point_for_position(event.position);
                    let position = point_for_position.nearest_valid;

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

        let hovered_link_modifier = Editor::is_cmd_or_ctrl_pressed(&event.modifiers(), cx);
        let mouse_down_hovered_link_modifier = if let ClickEvent::Mouse(mouse_event) = event {
            Editor::is_cmd_or_ctrl_pressed(&mouse_event.down.modifiers, cx)
        } else {
            true
        };

        if let Some(mouse_position) = event.mouse_position()
            && !pending_nonempty_selections
            && hovered_link_modifier
            && mouse_down_hovered_link_modifier
            && text_hitbox.is_hovered(window)
            && !matches!(
                editor.selection_drag_state,
                SelectionDragState::Dragging { .. }
            )
        {
            let point = position_map.point_for_position(mouse_position);
            editor.handle_click_hovered_link(point, event.modifiers(), window, cx);
            editor.selection_drag_state = SelectionDragState::None;

            cx.stop_propagation();
        }
    }

    fn pressure_click(
        editor: &mut Editor,
        event: &MousePressureEvent,
        position_map: &PositionMap,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let text_hitbox = &position_map.text_hitbox;
        let force_click_possible =
            matches!(editor.prev_pressure_stage, Some(PressureStage::Normal))
                && event.stage == PressureStage::Force;

        editor.prev_pressure_stage = Some(event.stage);

        if force_click_possible && text_hitbox.is_hovered(window) {
            let point = position_map.point_for_position(event.position);
            editor.handle_click_hovered_link(point, event.modifiers, window, cx);
            editor.selection_drag_state = SelectionDragState::None;
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
        if !editor.has_pending_selection()
            && matches!(editor.selection_drag_state, SelectionDragState::None)
        {
            return;
        }

        let point_for_position = position_map.point_for_position(event.position);
        let text_hitbox = &position_map.text_hitbox;

        let scroll_delta = {
            let text_bounds = text_hitbox.bounds;
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
            let em_width = window
                .text_system()
                .em_width(font_id, font_size)
                .unwrap_or(font_size);

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
            scroll_delta
        };

        if !editor.has_pending_selection() {
            let drop_anchor = position_map
                .snapshot
                .display_point_to_anchor(point_for_position.nearest_valid, Bias::Left);
            match editor.selection_drag_state {
                SelectionDragState::Dragging {
                    ref mut drop_cursor,
                    ref mut hide_drop_cursor,
                    ..
                } => {
                    drop_cursor.start = drop_anchor;
                    drop_cursor.end = drop_anchor;
                    *hide_drop_cursor = !text_hitbox.is_hovered(window);
                    editor.apply_scroll_delta(scroll_delta, window, cx);
                    cx.notify();
                }
                SelectionDragState::ReadyToDrag {
                    ref selection,
                    ref click_position,
                    ref mouse_down_time,
                } => {
                    let drag_and_drop_delay = Duration::from_millis(
                        EditorSettings::get_global(cx)
                            .drag_and_drop_selection
                            .delay
                            .0,
                    );
                    if mouse_down_time.elapsed() >= drag_and_drop_delay {
                        let drop_cursor = Selection {
                            id: post_inc(&mut editor.selections.next_selection_id()),
                            start: drop_anchor,
                            end: drop_anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        };
                        editor.selection_drag_state = SelectionDragState::Dragging {
                            selection: selection.clone(),
                            drop_cursor,
                            hide_drop_cursor: false,
                        };
                        editor.apply_scroll_delta(scroll_delta, window, cx);
                        cx.notify();
                    } else {
                        let click_point = position_map.point_for_position(*click_position);
                        editor.selection_drag_state = SelectionDragState::None;
                        editor.select(
                            SelectPhase::Begin {
                                position: click_point.nearest_valid,
                                add: false,
                                click_count: 1,
                            },
                            window,
                            cx,
                        );
                        editor.select(
                            SelectPhase::Update {
                                position: point_for_position.nearest_valid,
                                goal_column: point_for_position.exact_unclipped.column(),
                                scroll_delta,
                            },
                            window,
                            cx,
                        );
                    }
                }
                _ => {}
            }
        } else {
            editor.select(
                SelectPhase::Update {
                    position: point_for_position.nearest_valid,
                    goal_column: point_for_position.exact_unclipped.column(),
                    scroll_delta,
                },
                window,
                cx,
            );
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
            .buffer_snapshot()
            .anchor_before(start.to_point(&snapshot.display_snapshot))
            ..snapshot
                .buffer_snapshot()
                .anchor_after(end.to_point(&snapshot.display_snapshot));

        let Some(selection) = snapshot.remote_selections_in_range(&range, hub, cx).next() else {
            return;
        };
        let key = HoveredCursor {
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
}

fn scale_vertical_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.2) / 100.0).min(px(3.0)).into()
}

fn scale_horizontal_mouse_autoscroll_delta(delta: Pixels) -> f32 {
    (delta.pow(1.2) / 300.0).into()
}
