use std::cell::Cell;
use std::ops::{Deref, Range};
use std::rc::Rc;
use std::time::{Duration, Instant};

use collections::HashMap;
use feature_flags::{DiffReviewFeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    AnyElement, App, AvailableSpace, ClickEvent, Context, DispatchPhase, Element, Hitbox,
    MouseButton, MouseClickEvent, MouseDownEvent, MouseMoveEvent, MousePressureEvent, MouseUpEvent,
    ParentElement, Pixels, PressureStage, ScrollDelta, ScrollWheelEvent, TextAlign, Window,
    anchored, deferred, point, px,
};
use multi_buffer::{Anchor, MultiBufferRow};
use project::DisableAiSettings;
use settings::Settings;
use smallvec::SmallVec;
use sum_tree::Bias;
use text::SelectionGoal;
use util::{RangeExt, debug_panic};

use super::{
    EditorElement, EditorLayout, LineNumberLayout, LineWithInvisibles, PendingViewport,
    PointForPosition, PositionMap, SplitSide,
};
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use crate::clipboard::CapturedClipboardAction;
use crate::{
    CURSORS_VISIBLE_FOR, ColumnarMode, DisplayDiffHunk, DisplayPoint, DisplayRow, Editor,
    EditorSettings, EditorSnapshot, GutterHoverButton, HoveredCursor, JumpData,
    PhantomDiffReviewIndicator, RowExt, SelectPhase, Selection, SelectionDragState,
    display_map::{PendingRowGeometry, ToDisplayPoint},
    editor_settings::DoubleClickInMultibuffer,
    hover_popover::hover_at,
    inlays::inlay_hints::HintTarget,
    mouse_context_menu,
    movement::TextLayoutDetails,
    scroll::ScrollPixelOffset,
};

#[derive(Default)]
pub(super) struct PointerState {
    button: Cell<Option<MouseButton>>,
    down: Cell<Option<(gpui::Point<Pixels>, Anchor)>>,
}

#[derive(Clone, Copy)]
struct PointerPoint {
    anchor: Anchor,
    row_offset: i64,
    column_offset: i64,
}

impl PointerPoint {
    fn new(point: DisplayPoint, bias: Bias, snapshot: &EditorSnapshot) -> Self {
        let anchor = snapshot.display_point_to_anchor(snapshot.clip_point(point, bias), bias);
        let anchored = anchor.to_display_point(snapshot);
        let row_offset = i64::from(point.row().0) - i64::from(anchored.row().0);
        Self {
            anchor,
            row_offset,
            column_offset: i64::from(point.column())
                - if row_offset == 0 {
                    i64::from(anchored.column())
                } else {
                    0
                },
        }
    }

    fn resolve(&self, snapshot: &EditorSnapshot) -> DisplayPoint {
        let anchored = self.anchor.to_display_point(snapshot);
        let row = i64::from(anchored.row().0) + self.row_offset;
        let column = self.column_offset
            + if self.row_offset == 0 {
                i64::from(anchored.column())
            } else {
                0
            };
        DisplayPoint::new(
            DisplayRow(row.clamp(0, i64::from(u32::MAX)) as u32),
            column.clamp(0, i64::from(u32::MAX)) as u32,
        )
    }
}

struct PointerHit {
    previous_valid: PointerPoint,
    next_valid: PointerPoint,
    nearest_valid: PointerPoint,
    exact_unclipped: PointerPoint,
    column_overshoot_after_line_end: u32,
}

impl PointerHit {
    fn new(point: PointForPosition, snapshot: &EditorSnapshot) -> Self {
        Self {
            previous_valid: PointerPoint::new(point.previous_valid, Bias::Left, snapshot),
            next_valid: PointerPoint::new(point.next_valid, Bias::Left, snapshot),
            nearest_valid: PointerPoint::new(point.nearest_valid, Bias::Left, snapshot),
            exact_unclipped: PointerPoint::new(point.exact_unclipped, Bias::Left, snapshot),
            column_overshoot_after_line_end: point.column_overshoot_after_line_end,
        }
    }

    fn resolve(&self, snapshot: &EditorSnapshot) -> PointForPosition {
        PointForPosition {
            previous_valid: snapshot.clip_point(self.previous_valid.resolve(snapshot), Bias::Left),
            next_valid: snapshot.clip_point(self.next_valid.resolve(snapshot), Bias::Right),
            nearest_valid: snapshot.clip_point(self.nearest_valid.resolve(snapshot), Bias::Left),
            exact_unclipped: self.exact_unclipped.resolve(snapshot),
            column_overshoot_after_line_end: self.column_overshoot_after_line_end,
        }
    }
}

#[derive(Clone)]
struct MouseFrame {
    text_hitbox: Hitbox,
    gutter_hitbox: Hitbox,
    line_height: Pixels,
    scroll_position: gpui::Point<ScrollPixelOffset>,
    pointer_state: Rc<PointerState>,
}

struct MouseGeometry {
    details: TextLayoutDetails,
    em_layout_width: Pixels,
    text_align: TextAlign,
    content_width: Pixels,
}

struct MouseHitTest {
    frame: MouseFrame,
    snapshot: EditorSnapshot,
    point: PointForPosition,
    points: SmallVec<[(gpui::Point<Pixels>, PointForPosition, Option<DisplayPoint>); 3]>,
    hovered: (bool, bool),
    received_at: Instant,
    replayed: bool,
    line_number_hit: bool,
}

impl Deref for MouseHitTest {
    type Target = MouseFrame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl MouseHitTest {
    fn point_for_position(&self, position: gpui::Point<Pixels>) -> Option<PointForPosition> {
        self.points
            .iter()
            .find(|(target, _, _)| *target == position)
            .map(|(_, point, _)| *point)
    }

    fn inlay_hint_glyph_for_position(&self, position: gpui::Point<Pixels>) -> Option<DisplayPoint> {
        self.points
            .iter()
            .find(|(target, _, _)| *target == position)?
            .2
    }

    fn text_is_hovered(&self, _: &Window) -> bool {
        self.hovered.0
    }

    fn gutter_is_hovered(&self, _: &Window) -> bool {
        self.hovered.1
    }
}

enum MouseTarget {
    Ready {
        hit: PointerHit,
        hint: Option<PointerPoint>,
    },
    Pending {
        row: PointerPoint,
        x: ScrollPixelOffset,
        hint: Option<(PointerPoint, ScrollPixelOffset)>,
    },
}

struct MouseInput {
    event: DeferredMouseEvent,
    frame: MouseFrame,
    targets: SmallVec<[(gpui::Point<Pixels>, MouseTarget); 3]>,
    geometry: Option<Rc<MouseGeometry>>,
    original_row: DisplayRow,
    hovered: (bool, bool),
    received_at: Instant,
    replayed: bool,
}

#[derive(Clone)]
struct ColdMouseFrame {
    frame: MouseFrame,
    snapshot: EditorSnapshot,
    geometry: Rc<MouseGeometry>,
    content_origin: gpui::Point<Pixels>,
}

impl Deref for ColdMouseFrame {
    type Target = MouseFrame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl ColdMouseFrame {
    fn dispatch(
        &self,
        event: DeferredMouseEvent,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let mut original_row = DisplayRow(0);
        let targets = MouseInput::positions(&event, editor)
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                let offset = position - self.text_hitbox.origin;
                let y = offset.y.max(Pixels::ZERO).min(self.text_hitbox.size.height);
                let row =
                    DisplayRow((f64::from(y / self.line_height) + self.scroll_position.y) as u32);
                if index == 0 {
                    original_row = row;
                }
                let scroll_x = self.scroll_position.x * f64::from(self.geometry.em_layout_width);
                let hint_offset = position - self.content_origin;
                let hint = (hint_offset.y >= Pixels::ZERO).then(|| {
                    let row = DisplayRow(
                        (f64::from(hint_offset.y / self.line_height) + self.scroll_position.y)
                            as u32,
                    );
                    (
                        PointerPoint::new(row.as_display_point(), Bias::Right, &self.snapshot),
                        f64::from(hint_offset.x) + scroll_x,
                    )
                });
                (
                    position,
                    MouseTarget::Pending {
                        row: PointerPoint::new(row.as_display_point(), Bias::Right, &self.snapshot),
                        x: f64::from(offset.x) + scroll_x,
                        hint,
                    },
                )
            })
            .collect();
        let hovered = (
            self.text_hitbox.is_hovered(window),
            self.gutter_hitbox.is_hovered(window),
        );
        let mut input = MouseInput {
            event,
            frame: self.frame.clone(),
            targets,
            geometry: Some(self.geometry.clone()),
            original_row,
            hovered,
            received_at: Instant::now(),
            replayed: false,
        };
        match input.resolve_targets(&self.snapshot) {
            Ok(()) => input.dispatch(editor, window, cx),
            Err(pending) => {
                input.replayed = true;
                editor.defer_geometry(
                    pending,
                    move |editor, window, cx| input.replay(editor, window, cx),
                    window,
                    cx,
                );
            }
        }
    }
}

enum DeferredMouseEvent {
    Down(MouseDownEvent, bool),
    Up {
        event: MouseUpEvent,
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        primary: Option<CapturedClipboardAction>,
    },
    Move(MouseMoveEvent),
    Click(ClickEvent),
    Pressure(MousePressureEvent),
}

impl DeferredMouseEvent {
    fn up(event: &MouseUpEvent, _editor: &Editor, _cx: &App) -> Self {
        Self::Up {
            event: event.clone(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            primary: (event.button == MouseButton::Middle).then(|| _editor.capture_primary(_cx)),
        }
    }

    fn position(&self) -> Option<gpui::Point<Pixels>> {
        match self {
            Self::Down(event, _) => Some(event.position),
            Self::Up { event, .. } => Some(event.position),
            Self::Move(event) => Some(event.position),
            Self::Click(event) => event.mouse_position(),
            Self::Pressure(event) => Some(event.position),
        }
    }
}

impl MouseInput {
    fn positions(
        event: &DeferredMouseEvent,
        editor: &Editor,
    ) -> SmallVec<[gpui::Point<Pixels>; 3]> {
        let mut positions = event.position().into_iter().collect::<SmallVec<[_; 3]>>();
        if let DeferredMouseEvent::Click(ClickEvent::Mouse(event)) = event {
            positions.push(event.down.position);
        }
        if let SelectionDragState::ReadyToDrag { click_position, .. } = editor.selection_drag_state
        {
            positions.push(click_position);
        }
        positions.dedup();
        positions
    }

    fn dispatch(mut self, editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        if editor.input_is_deferred() {
            self.replayed = true;
            editor.defer_input(
                move |editor, window, cx| self.replay(editor, window, cx),
                window,
                cx,
            );
        } else {
            self.replay(editor, window, cx);
        }
    }

    fn resolve_targets(&mut self, snapshot: &EditorSnapshot) -> Result<(), PendingRowGeometry> {
        let Some(geometry) = &self.geometry else {
            return Ok(());
        };
        for (_, target) in &mut self.targets {
            let MouseTarget::Pending { row, x, hint } = target else {
                continue;
            };
            let row = row.resolve(snapshot).row();
            let (column, overshoot) = if row > snapshot.max_point().row() {
                (0, *x)
            } else {
                let layout = snapshot.layout_row(row, &geometry.details)?;
                let width = layout.hit_test_width();
                let x = *x
                    - LineWithInvisibles::alignment_offset_for_width(
                        width,
                        geometry.text_align,
                        geometry.content_width,
                    );
                match layout.index_for_x(x) {
                    Some(column) => (column as u32, 0.),
                    None => (snapshot.line_len(row), (x - width).max(0.)),
                }
            };
            let point = PointForPosition::new(
                snapshot,
                row,
                column,
                (overshoot / f64::from(geometry.em_layout_width)) as u32,
            );
            let hint = if let Some((row, x)) = hint {
                let row = row.resolve(snapshot).row();
                if row > snapshot.max_point().row() {
                    None
                } else {
                    let layout = snapshot.layout_row(row, &geometry.details)?;
                    let x = *x
                        - LineWithInvisibles::alignment_offset_for_width(
                            layout.hit_test_width(),
                            geometry.text_align,
                            geometry.content_width,
                        );
                    (x >= 0.)
                        .then(|| layout.index_for_x(x))
                        .flatten()
                        .map(|column| DisplayPoint::new(row, column as u32))
                        .filter(|point| snapshot.inlay_hint_at(*point).is_some())
                        .map(|point| PointerPoint::new(point, Bias::Left, snapshot))
                }
            } else {
                None
            };
            *target = MouseTarget::Ready {
                hit: PointerHit::new(point, snapshot),
                hint,
            };
        }
        Ok(())
    }

    fn replay(mut self, editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        editor.bind_input_window(window);
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        if matches!(&self.event, DeferredMouseEvent::Up { event, .. } if event.button == MouseButton::Middle)
            && editor.clipboard_input_is_deferred()
        {
            let editor_handle = cx.weak_entity();
            editor.defer_clipboard_input(
                move |window, cx| {
                    editor_handle
                        .update(cx, |editor, cx| self.replay(editor, window, cx))
                        .ok();
                },
                window,
                cx,
            );
            return;
        }
        let snapshot = editor.snapshot(window, cx);
        if let Err(pending) = self.resolve_targets(&snapshot) {
            self.replayed = true;
            editor.defer_geometry(
                pending,
                move |editor, window, cx| self.replay(editor, window, cx),
                window,
                cx,
            );
            return;
        }
        let points = self
            .targets
            .into_iter()
            .filter_map(|(position, target)| {
                let MouseTarget::Ready { hit, hint } = target else {
                    return None;
                };
                Some((
                    position,
                    hit.resolve(&snapshot),
                    hint.map(|hint| hint.resolve(&snapshot)),
                ))
            })
            .collect::<SmallVec<[_; 3]>>();
        let Some((_, point, _)) = points.first() else {
            return;
        };
        let row = point.nearest_valid.row();
        self.frame.scroll_position.y +=
            point.exact_unclipped.row().as_f64() - self.original_row.as_f64();
        let position_map = MouseHitTest {
            frame: self.frame,
            snapshot,
            point: *point,
            points,
            hovered: self.hovered,
            received_at: self.received_at,
            replayed: self.replayed,
            line_number_hit: matches!(&self.event, DeferredMouseEvent::Down(_, true)),
        };
        match self.event {
            DeferredMouseEvent::Down(event, _) => match event.button {
                MouseButton::Left => {
                    EditorElement::mouse_left_down(editor, &event, &position_map, window, cx)
                }
                MouseButton::Right => {
                    EditorElement::mouse_right_down(editor, &event, &position_map, window, cx)
                }
                MouseButton::Middle => {
                    EditorElement::mouse_middle_down(editor, &position_map, window, cx)
                }
                _ => {}
            },
            DeferredMouseEvent::Up {
                event,
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                primary,
            } => EditorElement::mouse_up(
                editor,
                &event,
                &position_map,
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                primary,
                window,
                cx,
            ),
            DeferredMouseEvent::Move(event) => {
                EditorElement::mouse_dragged(editor, &event, &position_map, window, cx);
                if position_map.replayed && editor.diff_review_drag_state.is_some() {
                    editor.update_diff_review_drag(row, window, cx);
                }
            }
            DeferredMouseEvent::Click(event) => {
                EditorElement::click(editor, &event, &position_map, window, cx)
            }
            DeferredMouseEvent::Pressure(event) => {
                EditorElement::pressure_click(editor, &event, &position_map, window, cx)
            }
        }
    }
}

impl EditorElement {
    pub(super) fn paint_pending_mouse_listeners(
        &mut self,
        layout: &PendingViewport,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.paint_scroll_wheel_listener(
            layout.hitbox.clone(),
            layout.line_height,
            layout.em_layout_width,
            point(0., layout.scroll_max_y),
            true,
            window,
            cx,
        );
        let frame = ColdMouseFrame {
            frame: MouseFrame {
                text_hitbox: layout.text_hitbox.clone(),
                gutter_hitbox: layout.gutter_hitbox.clone(),
                line_height: layout.line_height,
                scroll_position: layout.scroll_position,
                pointer_state: layout.pointer_state.clone(),
            },
            snapshot: layout.snapshot.clone(),
            geometry: Rc::new(MouseGeometry {
                details: layout.text_layout_details.clone(),
                em_layout_width: layout.em_layout_width,
                text_align: layout.text_align,
                content_width: layout.text_hitbox.size.width,
            }),
            content_origin: layout.content_origin,
        };
        window.on_mouse_event({
            let editor = self.editor.clone();
            let frame = frame.clone();
            move |event: &MouseDownEvent, phase, window, cx| {
                if phase != DispatchPhase::Bubble
                    || window.default_prevented()
                    || !(frame.text_hitbox.is_hovered(window)
                        || frame.gutter_hitbox.is_hovered(window))
                {
                    return;
                }
                editor.update(cx, |editor, cx| {
                    frame.pointer_state.button.set(Some(event.button));
                    if event.button == MouseButton::Left {
                        *editor
                            .pending_mouse_down
                            .get_or_insert_with(Default::default)
                            .borrow_mut() = Some(event.clone());
                    }
                    if !editor.focus_handle.is_focused(window) {
                        editor.last_focused_descendant = None;
                        window.focus(&editor.focus_handle, cx);
                    }
                    frame.dispatch(
                        DeferredMouseEvent::Down(event.clone(), false),
                        editor,
                        window,
                        cx,
                    );
                });
                cx.stop_propagation();
            }
        });
        window.on_mouse_event({
            let editor = self.editor.clone();
            let frame = frame.clone();
            let mut mouse_down = None;
            let mut mouse_up = None;
            move |event: &MouseUpEvent, phase, window, cx| {
                if phase == DispatchPhase::Capture {
                    editor.update(cx, |editor, cx| {
                        let handle_up = frame.pointer_state.button.take().is_some()
                            || editor.has_pending_selection()
                            || !matches!(editor.selection_drag_state, SelectionDragState::None)
                            || editor.diff_review_drag_state.is_some();
                        mouse_down = editor
                            .pending_mouse_down
                            .as_ref()
                            .and_then(|pending| pending.borrow_mut().take());
                        mouse_up = (handle_up || mouse_down.is_some())
                            .then(|| DeferredMouseEvent::up(event, editor, cx));
                    });
                } else if let Some(mouse_up) = mouse_up.take() {
                    editor.update(cx, |editor, cx| {
                        if let Some(down) = mouse_down.take()
                            && frame.text_hitbox.is_hovered(window)
                        {
                            frame.dispatch(
                                DeferredMouseEvent::Click(ClickEvent::Mouse(MouseClickEvent {
                                    down,
                                    up: event.clone(),
                                })),
                                editor,
                                window,
                                cx,
                            );
                        }
                        frame.dispatch(mouse_up, editor, window, cx);
                    });
                }
            }
        });
        window.on_mouse_event({
            let editor = self.editor.clone();
            let frame = frame.clone();
            move |event: &MouseMoveEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble
                    && matches!(
                        event.pressed_button,
                        Some(MouseButton::Left | MouseButton::Middle)
                    )
                {
                    editor.update(cx, |editor, cx| {
                        if frame.pointer_state.button.get().is_some()
                            || editor.has_pending_selection()
                            || !matches!(editor.selection_drag_state, SelectionDragState::None)
                            || editor.diff_review_drag_state.is_some()
                        {
                            frame.dispatch(
                                DeferredMouseEvent::Move(event.clone()),
                                editor,
                                window,
                                cx,
                            );
                        }
                    });
                }
            }
        });
        window.on_mouse_event({
            let editor = self.editor.clone();
            move |event: &MousePressureEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && frame.text_hitbox.is_hovered(window) {
                    editor.update(cx, |editor, cx| {
                        frame.dispatch(
                            DeferredMouseEvent::Pressure(event.clone()),
                            editor,
                            window,
                            cx,
                        );
                    });
                }
            }
        });
    }

    fn dispatch_mouse_input(
        editor: &mut Editor,
        mut event: DeferredMouseEvent,
        position_map: &PositionMap,
        line_numbers: Option<&HashMap<MultiBufferRow, LineNumberLayout>>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> bool {
        let Some(position) = event.position() else {
            return false;
        };
        let point = position_map.point_for_position(position);
        if let DeferredMouseEvent::Down(event, line_number_hit) = &mut event {
            if event.button == MouseButton::Left || event.button == MouseButton::Middle {
                if window.default_prevented() {
                    return false;
                }
                if event.button == MouseButton::Left {
                    if let Some(hovered_hunk) =
                        position_map
                            .display_hunks
                            .iter()
                            .find_map(|(hunk, hitbox)| match hunk {
                                DisplayDiffHunk::Unfolded {
                                    multi_buffer_range, ..
                                } => hitbox
                                    .as_ref()
                                    .is_some_and(|hitbox| hitbox.is_hovered(window))
                                    .then(|| multi_buffer_range.clone()),
                                DisplayDiffHunk::Folded { .. } => None,
                            })
                    {
                        let deferred_hunk = hovered_hunk.clone();
                        if !editor.defer_input(
                            move |editor, _, cx| {
                                editor.toggle_single_diff_hunk(deferred_hunk, cx);
                                cx.notify();
                            },
                            window,
                            cx,
                        ) {
                            editor.toggle_single_diff_hunk(hovered_hunk, cx);
                            cx.notify();
                        }
                        return false;
                    }
                    let row = position_map
                        .snapshot
                        .display_point_to_point(
                            point.exact_unclipped.row().as_display_point(),
                            Bias::Right,
                        )
                        .row;
                    *line_number_hit = line_numbers
                        .and_then(|lines| lines.get(&MultiBufferRow(row)))
                        .is_some_and(|line| {
                            line.segments.iter().any(|segment| {
                                segment
                                    .hitbox
                                    .as_ref()
                                    .is_some_and(|hitbox| hitbox.contains(&position))
                            })
                        });
                }
                if !position_map.text_is_hovered(window)
                    && !(event.button == MouseButton::Left
                        && position_map.gutter_is_hovered(window))
                {
                    return false;
                }
                position_map.pointer_state.button.set(Some(event.button));
                if editor.input_is_deferred() {
                    if !editor.focus_handle.is_focused(window) {
                        editor.last_focused_descendant = None;
                        window.focus(&editor.focus_handle, cx);
                    }
                    if event.button == MouseButton::Left {
                        cx.stop_propagation();
                    }
                }
            }
        }
        let targets = MouseInput::positions(&event, editor)
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                let point = if index == 0 {
                    point
                } else {
                    position_map.point_for_position(position)
                };
                (
                    position,
                    MouseTarget::Ready {
                        hit: PointerHit::new(point, &position_map.snapshot),
                        hint: position_map
                            .inlay_hint_glyph_for_position(position)
                            .map(|point| {
                                PointerPoint::new(point, Bias::Left, &position_map.snapshot)
                            }),
                    },
                )
            })
            .collect();
        let deferred = editor.input_is_deferred();
        MouseInput {
            event,
            frame: MouseFrame {
                text_hitbox: position_map.text_hitbox.clone(),
                gutter_hitbox: position_map.gutter_hitbox.clone(),
                line_height: position_map.line_height,
                scroll_position: position_map.scroll_position,
                pointer_state: position_map.pointer_state.clone(),
            },
            targets,
            geometry: None,
            original_row: point.exact_unclipped.row(),
            hovered: (
                position_map.text_is_hovered(window),
                position_map.gutter_is_hovered(window),
            ),
            received_at: Instant::now(),
            replayed: false,
        }
        .dispatch(editor, window, cx);
        deferred
    }

    pub(crate) fn mouse_moved(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &PositionMap,
        split_side: Option<SplitSide>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let modifiers = event.modifiers;
        let text_hovered = position_map.text_is_hovered(window);
        let gutter_hovered = position_map.gutter_is_hovered(window);
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

            // Breakpoints and bookmarks are keyed by absolute file path, so the
            // gutter button would be a no-op for buffers without a worktree file
            // (e.g. untitled buffers). Hide it there.
            if position_map
                .snapshot
                .buffer_snapshot()
                .anchor_to_buffer_anchor(buffer_anchor)
                .is_some_and(|(_, buffer_snapshot)| {
                    project::File::from_dyn(buffer_snapshot.file()).is_some()
                })
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
            let hint_glyph = position_map.inlay_hint_glyph_for_position(event.position);
            editor.update_hovered_link(
                point_for_position,
                hint_glyph,
                Some(event.position),
                &position_map.snapshot,
                modifiers,
                window,
                cx,
            );

            if hint_glyph.is_none()
                && let Some(point) = point_for_position.as_valid()
            {
                let anchor = position_map
                    .snapshot
                    .buffer_snapshot()
                    .anchor_before(point.to_offset(&position_map.snapshot, Bias::Left));
                hover_at(editor, Some(anchor), Some(event.position), window, cx);
                Self::update_visible_cursor(editor, point, position_map, window, cx);
            } else {
                editor.update_inlay_link_and_hover_points(
                    &position_map.snapshot,
                    hint_glyph,
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

        self.paint_scroll_wheel_listener(
            layout.hitbox.clone(),
            layout.position_map.line_height,
            layout.position_map.em_layout_width,
            layout.position_map.scroll_max,
            layout.position_map.horizontal_scroll_pending,
            window,
            cx,
        );

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

                            Self::dispatch_mouse_input(
                                editor,
                                DeferredMouseEvent::Down(event.clone(), false),
                                &position_map,
                                Some(line_numbers.as_ref()),
                                window,
                                cx,
                            );
                        }),
                        MouseButton::Right => editor.update(cx, |editor, cx| {
                            Self::dispatch_mouse_input(
                                editor,
                                DeferredMouseEvent::Down(event.clone(), false),
                                &position_map,
                                None,
                                window,
                                cx,
                            );
                        }),
                        MouseButton::Middle => editor.update(cx, |editor, cx| {
                            Self::dispatch_mouse_input(
                                editor,
                                DeferredMouseEvent::Down(event.clone(), false),
                                &position_map,
                                None,
                                window,
                                cx,
                            );
                        }),
                        _ => {}
                    };
                }
            }
        });

        window.on_mouse_event({
            let editor = self.editor.clone();
            let position_map = layout.position_map.clone();
            let mut mouse_up = None;

            move |event: &MouseUpEvent, phase, window, cx| {
                if phase == DispatchPhase::Capture {
                    mouse_up = Some(DeferredMouseEvent::up(event, editor.read(cx), cx));
                } else if let Some(mouse_up) = mouse_up.take() {
                    editor.update(cx, |editor, cx| {
                        Self::dispatch_mouse_input(
                            editor,
                            mouse_up,
                            &position_map,
                            None,
                            window,
                            cx,
                        );
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
                DispatchPhase::Capture => editor.update(cx, |editor, _| {
                    position_map.pointer_state.button.set(None);
                    let pending_mouse_down = editor
                        .pending_mouse_down
                        .get_or_insert_with(Default::default)
                        .clone();

                    let mut pending_mouse_down = pending_mouse_down.borrow_mut();
                    if pending_mouse_down.is_some() && position_map.text_is_hovered(window) {
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
                        Self::dispatch_mouse_input(
                            editor,
                            DeferredMouseEvent::Click(event),
                            &position_map,
                            None,
                            window,
                            cx,
                        );
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
                        Self::dispatch_mouse_input(
                            editor,
                            DeferredMouseEvent::Pressure(event.clone()),
                            &position_map,
                            None,
                            window,
                            cx,
                        );
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
                            if Self::dispatch_mouse_input(
                                editor,
                                DeferredMouseEvent::Move(event.clone()),
                                &position_map,
                                None,
                                window,
                                cx,
                            ) {
                                return;
                            }
                        }

                        Self::mouse_moved(editor, event, &position_map, split_side, window, cx)
                    });
                }
            }
        });
    }

    fn paint_scroll_wheel_listener(
        &mut self,
        hitbox: Hitbox,
        line_height: Pixels,
        glyph_width: Pixels,
        scroll_max: gpui::Point<ScrollPixelOffset>,
        horizontal_scroll_pending: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.on_mouse_event({
            let editor = self.editor.clone();

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
                            let delta = match event.delta {
                                gpui::ScrollDelta::Pixels(mut pixels) => {
                                    //Trackpad
                                    editor
                                        .scroll_manager
                                        .filter_scroll_delta(&mut pixels, event.touch_phase);
                                    pixels
                                }

                                gpui::ScrollDelta::Lines(lines) => {
                                    //Not trackpad
                                    point(lines.x * glyph_width, lines.y * line_height)
                                }
                            };

                            let current_scroll_position = editor.scroll_position(cx);
                            let x = (current_scroll_position.x
                                * ScrollPixelOffset::from(glyph_width)
                                - ScrollPixelOffset::from(delta.x * scroll_sensitivity))
                                / ScrollPixelOffset::from(glyph_width);
                            let y = (current_scroll_position.y
                                * ScrollPixelOffset::from(line_height)
                                - ScrollPixelOffset::from(delta.y * scroll_sensitivity))
                                / ScrollPixelOffset::from(line_height);
                            let mut scroll_position = point(
                                if horizontal_scroll_pending {
                                    x.max(0.)
                                } else {
                                    x.clamp(0., scroll_max.x)
                                },
                                y.clamp(0., scroll_max.y),
                            );
                            let forbid_vertical_scroll =
                                editor.scroll_manager.forbid_vertical_scroll();
                            if forbid_vertical_scroll {
                                scroll_position.y = current_scroll_position.y;
                            }

                            if scroll_position != current_scroll_position {
                                editor.scroll(scroll_position, window, cx);
                                cx.stop_propagation();
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
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let gutter_hitbox = &position_map.gutter_hitbox;
        let point_for_position = position_map.point;
        let mut click_count = event.click_count;
        let mut modifiers = event.modifiers;

        if position_map.gutter_is_hovered(window) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !position_map.text_is_hovered(window) {
            return;
        }

        let line_number_hit = position_map.line_number_hit;

        let hint_glyph = position_map
            .text_is_hovered(window)
            .then(|| position_map.inlay_hint_glyph_for_position(event.position))
            .flatten();
        if !event.modifiers.modified()
            && editor
                .hovered_inlay_hint_command()
                .is_some_and(|command| command.contains_glyph(&position_map.snapshot, hint_glyph))
        {
            cx.stop_propagation();
            return;
        }

        let is_singleton = editor.buffer().read(cx).is_singleton();
        let double_click_selects = is_singleton
            || EditorSettings::get_global(cx).double_click_in_multibuffer
                == DoubleClickInMultibuffer::Select;

        if click_count == 2
            && double_click_selects
            && !modifiers.modified()
            && let Some((hint, offset_in_hint)) =
                hint_glyph.and_then(|glyph| position_map.snapshot.inlay_hint_at(glyph))
            && let Some(buffer_id) = hint
                .position
                .raw_text_anchor()
                .map(|anchor| anchor.buffer_id)
        {
            let resolved = editor.is_inlay_hint_resolved(buffer_id, hint.id, cx);
            let accepting = editor.apply_inlay_hint_text_edits(
                [HintTarget {
                    buffer_id,
                    hint_id: hint.id,
                    position: hint.position,
                }]
                .into_iter(),
                Some(offset_in_hint),
                window,
                cx,
            );
            if accepting && resolved {
                cx.stop_propagation();
                return;
            }
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
                let anchor = position_map
                    .snapshot
                    .display_point_to_anchor(point_for_position.nearest_valid, Bias::Left);
                position_map
                    .pointer_state
                    .down
                    .set(Some((event.position, anchor)));
                editor.selection_drag_state = SelectionDragState::ReadyToDrag {
                    selection: *newest_anchor,
                    click_position: event.position,
                    mouse_down_time: position_map.received_at,
                };
                cx.stop_propagation();
                return;
            }
        }

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
            if line_number_hit {
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
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if position_map.gutter_is_hovered(window) {
            let gutter_right_padding = editor.gutter_dimensions.right_padding;
            let hitbox = &position_map.gutter_hitbox;

            if event.position.x <= hitbox.bounds.right() - gutter_right_padding
                // Don't show the gutter_context_menu in collab notes
                && editor.project.is_some()
            {
                let point_for_position = position_map.point;
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

        if !position_map.text_is_hovered(window) {
            return;
        }

        let point_for_position = position_map.point;
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
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !position_map.text_is_hovered(window) {
            return;
        }

        let point_for_position = position_map.point;
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
        position_map: &MouseHitTest,
        #[cfg(any(target_os = "linux", target_os = "freebsd"))] primary: Option<
            CapturedClipboardAction,
        >,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        let primary = primary
            .and_then(
                |captured| match editor.read_captured_primary(&captured, cx) {
                    Ok(item) => item,
                    Err(error) => {
                        editor.notify_clipboard_error(error, cx);
                        None
                    }
                },
            )
            .and_then(|item| item.text());

        // Handle diff review drag completion
        if editor.diff_review_drag_state.is_some() {
            editor.end_diff_review_drag(window, cx);
            cx.stop_propagation();
            return;
        }

        let end_selection = editor.has_pending_selection();
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();
        let point_for_position = position_map.point;

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
                    && position_map.text_is_hovered(window)
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
            if !position_map.text_is_hovered(window) || editor.read_only(cx) {
                return;
            }

            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            if EditorSettings::get_global(cx).middle_click_paste {
                if let Some(text) = primary {
                    let point_for_position = position_map.point;
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
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let pending_nonempty_selections = editor.has_pending_nonempty_selection();

        let hovered_link_modifier = Editor::is_cmd_or_ctrl_pressed(&event.modifiers(), cx);
        let mouse_down_hovered_link_modifier = if let ClickEvent::Mouse(mouse_event) = event {
            Editor::is_cmd_or_ctrl_pressed(&mouse_event.down.modifiers, cx)
        } else {
            true
        };

        if let Some(mouse_position) = event.mouse_position()
            && !pending_nonempty_selections
            && position_map.text_is_hovered(window)
            && !matches!(
                editor.selection_drag_state,
                SelectionDragState::Dragging { .. }
            )
        {
            let point = position_map.point;
            if let ClickEvent::Mouse(mouse_event) = event
                && mouse_event.up.click_count == 1
                && !mouse_event.down.modifiers.modified()
                && !mouse_event.up.modifiers.modified()
                && editor.activate_hovered_inlay_hint_command(
                    &position_map.snapshot,
                    position_map.inlay_hint_glyph_for_position(mouse_event.down.position),
                    position_map.inlay_hint_glyph_for_position(mouse_position),
                    cx,
                )
            {
                editor.selection_drag_state = SelectionDragState::None;
                cx.stop_propagation();
                return;
            }

            if hovered_link_modifier && mouse_down_hovered_link_modifier {
                editor.handle_click_hovered_link(point, event.modifiers(), window, cx);
                editor.selection_drag_state = SelectionDragState::None;
                cx.stop_propagation();
            }
        }
    }

    fn pressure_click(
        editor: &mut Editor,
        event: &MousePressureEvent,
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let force_click_possible =
            matches!(editor.prev_pressure_stage, Some(PressureStage::Normal))
                && event.stage == PressureStage::Force;

        editor.prev_pressure_stage = Some(event.stage);

        if force_click_possible && position_map.text_is_hovered(window) {
            let point = position_map.point;
            editor.handle_click_hovered_link(point, event.modifiers, window, cx);
            editor.selection_drag_state = SelectionDragState::None;
            cx.stop_propagation();
        }
    }

    fn mouse_dragged(
        editor: &mut Editor,
        event: &MouseMoveEvent,
        position_map: &MouseHitTest,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !position_map.replayed && editor.has_autoscroll_request()
            || !editor.has_pending_selection()
                && matches!(editor.selection_drag_state, SelectionDragState::None)
        {
            return;
        }

        let point_for_position = position_map.point;
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
                    *hide_drop_cursor = !position_map.text_is_hovered(window);
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
                    if position_map
                        .received_at
                        .saturating_duration_since(*mouse_down_time)
                        >= drag_and_drop_delay
                    {
                        let drop_cursor = Selection {
                            id: editor.selections.next_selection_id(),
                            start: drop_anchor,
                            end: drop_anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        };
                        editor.selection_drag_state = SelectionDragState::Dragging {
                            selection: *selection,
                            drop_cursor,
                            hide_drop_cursor: false,
                        };
                        editor.apply_scroll_delta(scroll_delta, window, cx);
                        cx.notify();
                    } else {
                        let Some(click_point) = position_map
                            .pointer_state
                            .down
                            .get()
                            .filter(|(position, _)| position == click_position)
                            .map(|(_, anchor)| anchor.to_display_point(&position_map.snapshot))
                            .or_else(|| {
                                position_map
                                    .point_for_position(*click_position)
                                    .map(|point| point.nearest_valid)
                            })
                        else {
                            return;
                        };
                        editor.selection_drag_state = SelectionDragState::None;
                        editor.select(
                            SelectPhase::Begin {
                                position: click_point,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        SelectionEffects, editor_tests::init_test, scroll::Autoscroll,
        test::editor_test_context::EditorTestContext,
    };
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    async fn test_mouse_drag_preserves_pending_sticky_header_autoscroll(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        let line_height = cx.update_editor(|editor, window, cx| {
            editor
                .style(cx)
                .text
                .line_height_in_pixels(window.rem_size())
        });

        let buffer = indoc::indoc! {"
                ˇfn foo() {
                    let abc = 123;
                }
                struct Bar;
                impl Bar {
                    fn new() -> Self {
                        Self
                    }
                }
                fn baz() {
                }
            "};
        cx.set_state(&buffer);

        let text_origin_x = cx.update_editor(|editor, _, _| {
            editor
                .last_position_map
                .as_ref()
                .unwrap()
                .text_hitbox
                .bounds
                .origin
                .x
        });

        cx.update_editor(|editor, window, cx| {
            editor.scroll(gpui::Point { x: 0., y: 5.5 }, window, cx);
        });
        cx.run_until_parked();

        let mouse_drag_position = gpui::Point {
            x: text_origin_x,
            y: 2.25 * line_height,
        };
        cx.update_editor(|editor, window, cx| {
            let position_map = editor.last_position_map.as_ref().unwrap().clone();
            let anchor = editor
                .snapshot(window, cx)
                .display_snapshot
                .display_point_to_anchor(DisplayPoint::new(DisplayRow(5), 0), Bias::Left);

            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::top_relative(1.0)),
                window,
                cx,
                |selections| {
                    selections.clear_disjoint();
                    selections
                        .set_pending_anchor_range(anchor..anchor, crate::SelectMode::Character);
                },
            );
            assert!(editor.has_autoscroll_request());

            EditorElement::dispatch_mouse_input(
                editor,
                DeferredMouseEvent::Move(MouseMoveEvent {
                    position: mouse_drag_position,
                    modifiers: Modifiers::none(),
                    pressed_button: Some(MouseButton::Left),
                }),
                &position_map,
                None,
                window,
                cx,
            );
            assert!(editor.has_autoscroll_request());
        });
    }
}
