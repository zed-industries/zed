use super::data_table::{
    ResizableColumnsState,
    table_row::{IntoTableRow as _, TableRow},
};
use crate::{
    ActiveTheme as _, AnyElement, App, Context, Div, FluentBuilder as _, InteractiveElement,
    IntoElement, ParentElement, Pixels, StatefulInteractiveElement, Styled, Window, div, h_flex,
    px,
};
use gpui::{
    AbsoluteLength, AppContext as _, Bounds, DefiniteLength, DragMoveEvent, Empty, Entity,
    EntityId, Length, Stateful, WeakEntity,
};
use std::rc::Rc;

pub(crate) const RESIZE_COLUMN_WIDTH: f32 = 8.0;
pub(crate) const RESIZE_DIVIDER_WIDTH: f32 = 1.0;

/// Drag payload for column resize handles.
/// Includes the `EntityId` of the owning column state so that
/// `on_drag_move` handlers on unrelated tables ignore the event.
#[derive(Debug)]
pub(crate) struct DraggedColumn {
    pub(crate) col_idx: usize,
    pub(crate) state_id: EntityId,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TableResizeBehavior {
    None,
    Resizable,
    MinSize(f32),
}

impl TableResizeBehavior {
    pub fn is_resizable(&self) -> bool {
        *self != TableResizeBehavior::None
    }

    pub fn min_size(&self) -> Option<f32> {
        match self {
            TableResizeBehavior::None => None,
            TableResizeBehavior::Resizable => Some(0.05),
            TableResizeBehavior::MinSize(min_size) => Some(*min_size),
        }
    }
}

#[derive(Clone)]
pub(crate) enum ColumnsStateRef {
    Redistributable(WeakEntity<RedistributableColumnsState>),
    Resizable(WeakEntity<ResizableColumnsState>),
}

#[derive(Clone)]
pub struct HeaderResizeInfo {
    pub(crate) columns_state: ColumnsStateRef,
    pub resize_behavior: TableRow<TableResizeBehavior>,
}

impl HeaderResizeInfo {
    pub fn from_redistributable(
        columns_state: &Entity<RedistributableColumnsState>,
        cx: &App,
    ) -> Self {
        let resize_behavior = columns_state.read(cx).resize_behavior().clone();
        Self {
            columns_state: ColumnsStateRef::Redistributable(columns_state.downgrade()),
            resize_behavior,
        }
    }

    pub fn from_resizable(columns_state: &Entity<ResizableColumnsState>, cx: &App) -> Self {
        let resize_behavior = columns_state.read(cx).resize_behavior().clone();
        Self {
            columns_state: ColumnsStateRef::Resizable(columns_state.downgrade()),
            resize_behavior,
        }
    }

    pub fn reset_column(&self, col_idx: usize, window: &mut Window, cx: &mut App) {
        match &self.columns_state {
            ColumnsStateRef::Redistributable(weak) => {
                weak.update(cx, |state, cx| {
                    state.reset_column_to_initial_width(col_idx, window);
                    cx.notify();
                })
                .ok();
            }
            ColumnsStateRef::Resizable(weak) => {
                weak.update(cx, |state, cx| {
                    state.reset_column_to_initial_width(col_idx);
                    cx.notify();
                })
                .ok();
            }
        }
    }
}

pub struct RedistributableColumnsState {
    pub(crate) initial_widths: TableRow<DefiniteLength>,
    pub(crate) committed_widths: TableRow<DefiniteLength>,
    pub(crate) preview_widths: TableRow<DefiniteLength>,
    pub(crate) resize_behavior: TableRow<TableResizeBehavior>,
    pub(crate) cached_container_width: Pixels,
}

impl RedistributableColumnsState {
    pub fn new(
        cols: usize,
        initial_widths: Vec<impl Into<DefiniteLength>>,
        resize_behavior: Vec<TableResizeBehavior>,
    ) -> Self {
        let widths: TableRow<DefiniteLength> = initial_widths
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into_table_row(cols);
        Self {
            initial_widths: widths.clone(),
            committed_widths: widths.clone(),
            preview_widths: widths,
            resize_behavior: resize_behavior.into_table_row(cols),
            cached_container_width: Default::default(),
        }
    }

    pub fn cols(&self) -> usize {
        self.committed_widths.cols()
    }

    pub fn initial_widths(&self) -> &TableRow<DefiniteLength> {
        &self.initial_widths
    }

    pub fn preview_widths(&self) -> &TableRow<DefiniteLength> {
        &self.preview_widths
    }

    pub fn resize_behavior(&self) -> &TableRow<TableResizeBehavior> {
        &self.resize_behavior
    }

    pub fn widths_to_render(&self) -> TableRow<Length> {
        self.preview_widths.map_cloned(Length::Definite)
    }

    pub fn preview_fractions(&self, rem_size: Pixels) -> TableRow<f32> {
        if self.cached_container_width > px(0.) {
            self.preview_widths
                .map_ref(|length| Self::get_fraction(length, self.cached_container_width, rem_size))
        } else {
            self.preview_widths.map_ref(|length| match length {
                DefiniteLength::Fraction(fraction) => *fraction,
                DefiniteLength::Absolute(_) => 0.0,
            })
        }
    }

    pub fn preview_column_width(&self, column_index: usize, window: &Window) -> Option<Pixels> {
        let width = self.preview_widths().as_slice().get(column_index)?;
        match width {
            DefiniteLength::Fraction(fraction) if self.cached_container_width > px(0.) => {
                Some(self.cached_container_width * *fraction)
            }
            DefiniteLength::Fraction(_) => None,
            DefiniteLength::Absolute(AbsoluteLength::Pixels(pixels)) => Some(*pixels),
            DefiniteLength::Absolute(AbsoluteLength::Rems(rems_width)) => {
                Some(rems_width.to_pixels(window.rem_size()))
            }
        }
    }

    pub fn cached_container_width(&self) -> Pixels {
        self.cached_container_width
    }

    pub fn set_cached_container_width(&mut self, width: Pixels) {
        self.cached_container_width = width;
    }

    pub fn commit_preview(&mut self) {
        self.committed_widths = self.preview_widths.clone();
    }

    pub fn reset_column_to_initial_width(&mut self, column_index: usize, window: &Window) {
        let bounds_width = self.cached_container_width;
        if bounds_width <= px(0.) {
            return;
        }

        let rem_size = window.rem_size();
        let initial_sizes = self
            .initial_widths
            .map_ref(|length| Self::get_fraction(length, bounds_width, rem_size));
        let widths = self
            .committed_widths
            .map_ref(|length| Self::get_fraction(length, bounds_width, rem_size));

        let updated_widths =
            Self::reset_to_initial_size(column_index, widths, initial_sizes, &self.resize_behavior);
        self.committed_widths = updated_widths.map(DefiniteLength::Fraction);
        self.preview_widths = self.committed_widths.clone();
    }

    fn get_fraction(length: &DefiniteLength, bounds_width: Pixels, rem_size: Pixels) -> f32 {
        match length {
            DefiniteLength::Absolute(AbsoluteLength::Pixels(pixels)) => *pixels / bounds_width,
            DefiniteLength::Absolute(AbsoluteLength::Rems(rems_width)) => {
                rems_width.to_pixels(rem_size) / bounds_width
            }
            DefiniteLength::Fraction(fraction) => *fraction,
        }
    }

    pub(crate) fn reset_to_initial_size(
        col_idx: usize,
        mut widths: TableRow<f32>,
        initial_sizes: TableRow<f32>,
        resize_behavior: &TableRow<TableResizeBehavior>,
    ) -> TableRow<f32> {
        let diff = initial_sizes[col_idx] - widths[col_idx];

        let left_diff =
            initial_sizes[..col_idx].iter().sum::<f32>() - widths[..col_idx].iter().sum::<f32>();
        let right_diff = initial_sizes[col_idx + 1..].iter().sum::<f32>()
            - widths[col_idx + 1..].iter().sum::<f32>();

        let go_left_first = if diff < 0.0 {
            left_diff > right_diff
        } else {
            left_diff < right_diff
        };

        if !go_left_first {
            let diff_remaining =
                Self::propagate_resize_diff(diff, col_idx, &mut widths, resize_behavior, 1);

            if diff_remaining != 0.0 && col_idx > 0 {
                Self::propagate_resize_diff(
                    diff_remaining,
                    col_idx,
                    &mut widths,
                    resize_behavior,
                    -1,
                );
            }
        } else {
            let diff_remaining =
                Self::propagate_resize_diff(diff, col_idx, &mut widths, resize_behavior, -1);

            if diff_remaining != 0.0 {
                Self::propagate_resize_diff(
                    diff_remaining,
                    col_idx,
                    &mut widths,
                    resize_behavior,
                    1,
                );
            }
        }

        widths
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedColumn>,
        hidden: Option<&TableRow<bool>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;
        let bounds_width = bounds.right() - bounds.left();
        if bounds_width <= px(0.) {
            return;
        }

        let rem_size = window.rem_size();
        let col_idx = drag_event.drag(cx).col_idx;

        let divider_width = Self::get_fraction(
            &DefiniteLength::Absolute(AbsoluteLength::Pixels(px(RESIZE_DIVIDER_WIDTH))),
            bounds_width,
            rem_size,
        );

        let widths = self
            .committed_widths
            .map_ref(|length| Self::get_fraction(length, bounds_width, rem_size));

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;

        let widths = Self::compute_drag_preview(
            widths,
            &self.resize_behavior,
            hidden,
            col_idx,
            drag_fraction,
            divider_width,
        );

        self.preview_widths = widths.map(DefiniteLength::Fraction);
    }

    /// Computes the preview column fractions produced by dragging the divider after `col_idx`
    /// to `drag_fraction` (the cursor's x position expressed as a fraction of the container
    /// width). `divider_width` is the resize-divider width as a fraction of the container.
    ///
    /// The on-screen layout only contains the visible columns, with the hidden columns' width
    /// budget redistributed across them (see [`redistribute_hidden_widths`]), so the geometry
    /// here is done in that visible/redistributed space: the raw `widths` are compacted to the
    /// visible columns and scaled to match the rendered layout, the drag is applied there (which
    /// also makes neighbor propagation skip hidden columns), and the result is mapped back to
    /// the raw widths, leaving hidden columns untouched.
    ///
    /// Extracted as a pure function so the drag math can be unit tested, mirroring the
    /// `drag_column_handle` / `propagate_resize_diff` helpers.
    pub(crate) fn compute_drag_preview(
        mut widths: TableRow<f32>,
        resize_behavior: &TableRow<TableResizeBehavior>,
        hidden: Option<&TableRow<bool>>,
        col_idx: usize,
        drag_fraction: f32,
        divider_width: f32,
    ) -> TableRow<f32> {
        let visible_cols: Vec<usize> = (0..widths.cols())
            .filter(|idx| !is_column_hidden(hidden, *idx))
            .collect();

        // Dividers are only rendered after visible columns, so a hidden `col_idx` should be
        // impossible; bail out rather than resizing the wrong column.
        let Some(divider_position) = visible_cols.iter().position(|&idx| idx == col_idx) else {
            return widths;
        };

        let total_sum: f32 = widths.as_slice().iter().sum();
        let visible_sum: f32 = visible_cols.iter().map(|&idx| widths[idx]).sum();
        // The drag only moves width between visible columns, so `visible_sum` (and therefore
        // this scale) is the same before and after the drag, making the mapping back exact.
        let scale = if visible_sum > 0.0 {
            total_sum / visible_sum
        } else {
            1.0
        };

        let mut rendered_widths = TableRow::from_vec(
            visible_cols
                .iter()
                .map(|&idx| widths[idx] * scale)
                .collect(),
            visible_cols.len(),
        );
        let rendered_behavior = TableRow::from_vec(
            visible_cols
                .iter()
                .map(|&idx| resize_behavior[idx])
                .collect(),
            visible_cols.len(),
        );

        let mut col_position = 0.0;
        for length in rendered_widths[0..=divider_position].iter() {
            col_position += length + divider_width;
        }

        let mut total_length_ratio = col_position;
        for length in rendered_widths[divider_position + 1..].iter() {
            total_length_ratio += length;
        }
        let cols = rendered_behavior.cols();
        total_length_ratio += (cols - 1 - divider_position) as f32 * divider_width;

        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - col_position - divider_width / 2.0;

        Self::drag_column_handle(
            diff,
            divider_position,
            &mut rendered_widths,
            &rendered_behavior,
        );

        for (visible_position, &idx) in visible_cols.iter().enumerate() {
            widths[idx] = rendered_widths[visible_position] / scale;
        }

        widths
    }

    pub(crate) fn drag_column_handle(
        diff: f32,
        col_idx: usize,
        widths: &mut TableRow<f32>,
        resize_behavior: &TableRow<TableResizeBehavior>,
    ) {
        if diff > 0.0 {
            Self::propagate_resize_diff(diff, col_idx, widths, resize_behavior, 1);
        } else {
            Self::propagate_resize_diff(-diff, col_idx + 1, widths, resize_behavior, -1);
        }
    }

    pub(crate) fn propagate_resize_diff(
        diff: f32,
        col_idx: usize,
        widths: &mut TableRow<f32>,
        resize_behavior: &TableRow<TableResizeBehavior>,
        direction: i8,
    ) -> f32 {
        let mut diff_remaining = diff;
        if resize_behavior[col_idx].min_size().is_none() {
            return diff;
        }

        let step_right;
        let step_left;
        if direction < 0 {
            step_right = 0;
            step_left = 1;
        } else {
            step_right = 1;
            step_left = 0;
        }
        if col_idx == 0 && direction < 0 {
            return diff;
        }
        let mut curr_column = col_idx + step_right - step_left;

        while diff_remaining != 0.0 && curr_column < widths.cols() {
            let Some(min_size) = resize_behavior[curr_column].min_size() else {
                if curr_column == 0 {
                    break;
                }
                curr_column -= step_left;
                curr_column += step_right;
                continue;
            };

            let curr_width = widths[curr_column] - diff_remaining;
            widths[curr_column] = curr_width;

            if min_size > curr_width {
                diff_remaining = min_size - curr_width;
                widths[curr_column] = min_size;
            } else {
                diff_remaining = 0.0;
                break;
            }
            if curr_column == 0 {
                break;
            }
            curr_column -= step_left;
            curr_column += step_right;
        }
        widths[col_idx] = widths[col_idx] + (diff - diff_remaining);

        diff_remaining
    }
}

/// Returns `true` when the column at `idx` is hidden by `hidden`.
pub fn is_column_hidden(hidden: Option<&TableRow<bool>>, idx: usize) -> bool {
    hidden
        .and_then(|mask| mask.get(idx).copied())
        .unwrap_or(false)
}

/// Redistributes the fractional width budget of hidden columns across the visible columns so the
/// visible columns fill the container instead of leaving a gap. Hidden columns keep their stored
/// width (they are never rendered, so the value is not shown) and `Absolute` widths are left
/// untouched. Returns the widths unchanged when no column is hidden.
pub fn redistribute_hidden_widths(
    widths: &TableRow<Length>,
    hidden: Option<&TableRow<bool>>,
) -> TableRow<Length> {
    if !(0..widths.cols()).any(|idx| is_column_hidden(hidden, idx)) {
        return widths.clone();
    }

    let mut total_fraction_sum = 0.0;
    let mut visible_fraction_sum = 0.0;
    for (idx, width) in widths.as_slice().iter().enumerate() {
        if let Length::Definite(DefiniteLength::Fraction(fraction)) = width {
            total_fraction_sum += *fraction;
            if !is_column_hidden(hidden, idx) {
                visible_fraction_sum += *fraction;
            }
        }
    }
    let scale = if visible_fraction_sum > 0.0 {
        total_fraction_sum / visible_fraction_sum
    } else {
        1.0
    };

    let scaled: Vec<Length> = widths
        .as_slice()
        .iter()
        .enumerate()
        .map(|(idx, width)| match width {
            Length::Definite(DefiniteLength::Fraction(fraction))
                if !is_column_hidden(hidden, idx) =>
            {
                Length::Definite(DefiniteLength::Fraction(fraction * scale))
            }
            other => *other,
        })
        .collect();
    TableRow::from_vec(scaled, widths.cols())
}

/// Fraction-valued counterpart of [`redistribute_hidden_widths`].
pub fn redistribute_hidden_fractions(
    fractions: &TableRow<f32>,
    hidden: Option<&TableRow<bool>>,
) -> TableRow<f32> {
    if !(0..fractions.cols()).any(|idx| is_column_hidden(hidden, idx)) {
        return fractions.clone();
    }

    let total_sum: f32 = fractions.as_slice().iter().sum();
    let visible_sum: f32 = fractions
        .as_slice()
        .iter()
        .enumerate()
        .filter(|(idx, _)| !is_column_hidden(hidden, *idx))
        .map(|(_, fraction)| *fraction)
        .sum();
    let scale = if visible_sum > 0.0 {
        total_sum / visible_sum
    } else {
        1.0
    };

    let scaled: Vec<f32> = fractions
        .as_slice()
        .iter()
        .enumerate()
        .map(|(idx, fraction)| {
            if is_column_hidden(hidden, idx) {
                *fraction
            } else {
                fraction * scale
            }
        })
        .collect();
    TableRow::from_vec(scaled, fractions.cols())
}

pub fn bind_redistributable_columns(
    container: Div,
    columns_state: Entity<RedistributableColumnsState>,
    hidden: Option<TableRow<bool>>,
) -> Div {
    container
        .on_drag_move::<DraggedColumn>({
            let columns_state = columns_state.clone();
            move |event, window, cx| {
                if event.drag(cx).state_id != columns_state.entity_id() {
                    return;
                }
                columns_state.update(cx, |columns, cx| {
                    columns.on_drag_move(event, hidden.as_ref(), window, cx);
                });
            }
        })
        .on_children_prepainted({
            let columns_state = columns_state.clone();
            move |bounds, _, cx| {
                if let Some(width) = child_bounds_width(&bounds) {
                    columns_state.update(cx, |columns, _| {
                        columns.set_cached_container_width(width);
                    });
                }
            }
        })
        .on_drop::<DraggedColumn>(move |_, _, cx| {
            columns_state.update(cx, |columns, _| {
                columns.commit_preview();
            });
        })
}

pub fn render_redistributable_columns_resize_handles(
    columns_state: &Entity<RedistributableColumnsState>,
    hidden: Option<&TableRow<bool>>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (column_widths, resize_behavior) = {
        let state = columns_state.read(cx);
        (
            redistribute_hidden_widths(&state.widths_to_render(), hidden),
            state.resize_behavior().clone(),
        )
    };

    // Only the visible columns participate in the layout; filtered columns are skipped entirely
    // (no spacer, no divider) so we don't draw a stray resize line where a hidden column was.
    let visible_cols: Vec<usize> = (0..column_widths.cols())
        .filter(|idx| !is_column_hidden(hidden, *idx))
        .collect();

    let mut children: Vec<AnyElement> = Vec::with_capacity(visible_cols.len() * 2);
    for (position, &col_idx) in visible_cols.iter().enumerate() {
        children.push(resize_spacer(column_widths[col_idx]).into_any_element());

        // A divider is rendered after every visible column except the last, mirroring the
        // original `intersperse` behavior but in terms of visible columns.
        let is_last_visible = position + 1 == visible_cols.len();
        if is_last_visible {
            continue;
        }

        let columns_state = columns_state.clone();
        let divider = div().id(col_idx).relative().top_0();
        let entity_id = columns_state.entity_id();
        let on_reset: Rc<dyn Fn(&mut Window, &mut App)> = {
            let columns_state = columns_state.clone();
            Rc::new(move |window, cx| {
                columns_state.update(cx, |columns, cx| {
                    columns.reset_column_to_initial_width(col_idx, window);
                    cx.notify();
                });
            })
        };
        let on_drag_end: Option<Rc<dyn Fn(&mut App)>> = {
            Some(Rc::new(move |cx| {
                columns_state.update(cx, |state, _| state.commit_preview());
            }))
        };
        children.push(render_column_resize_divider(
            divider,
            col_idx,
            resize_behavior[col_idx].is_resizable(),
            entity_id,
            on_reset,
            on_drag_end,
            window,
            cx,
        ));
    }

    h_flex()
        .id("resize-handles")
        .absolute()
        .inset_0()
        .w_full()
        .children(children)
        .into_any_element()
}

/// Builds a single column resize divider with an interactive drag handle.
///
/// The caller provides:
/// - `divider`: a pre-positioned divider element (with absolute or relative positioning)
/// - `col_idx`: which column this divider is for
/// - `is_resizable`: whether the column supports resizing
/// - `entity_id`: the `EntityId` of the owning column state (for the drag payload)
/// - `on_reset`: called on double-click to reset the column to its initial width
/// - `on_drag_end`: called when the drag ends (e.g. to commit preview widths)
pub(crate) fn render_column_resize_divider(
    divider: Stateful<Div>,
    col_idx: usize,
    is_resizable: bool,
    entity_id: EntityId,
    on_reset: Rc<dyn Fn(&mut Window, &mut App)>,
    on_drag_end: Option<Rc<dyn Fn(&mut App)>>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    window.with_id(col_idx, |window| {
        let mut resize_divider = divider.w(px(RESIZE_DIVIDER_WIDTH)).h_full().bg(cx
            .theme()
            .colors()
            .border
            .opacity(0.8));

        let mut resize_handle = div()
            .id("column-resize-handle")
            .absolute()
            .left_neg_0p5()
            .w(px(RESIZE_COLUMN_WIDTH))
            .h_full();

        if is_resizable {
            let is_highlighted = window.use_state(cx, |_window, _cx| false);

            resize_divider = resize_divider.when(*is_highlighted.read(cx), |div| {
                div.bg(cx.theme().colors().border_focused)
            });

            resize_handle = resize_handle
                .on_hover({
                    let is_highlighted = is_highlighted.clone();
                    move |&was_hovered, _, cx| is_highlighted.write(cx, was_hovered)
                })
                .cursor_col_resize()
                .on_click(move |event, window, cx| {
                    if event.click_count() >= 2 {
                        on_reset(window, cx);
                    }
                    cx.stop_propagation();
                })
                .on_drag(
                    DraggedColumn {
                        col_idx,
                        state_id: entity_id,
                    },
                    {
                        let is_highlighted = is_highlighted.clone();
                        move |_, _offset, _window, cx| {
                            is_highlighted.write(cx, true);
                            cx.new(|_cx| Empty)
                        }
                    },
                )
                .on_drop::<DraggedColumn>(move |_, _, cx| {
                    is_highlighted.write(cx, false);
                    if let Some(on_drag_end) = &on_drag_end {
                        on_drag_end(cx);
                    }
                });
        }

        resize_divider.child(resize_handle).into_any_element()
    })
}

fn resize_spacer(width: Length) -> Div {
    div().w(width).h_full()
}

fn child_bounds_width(bounds: &[Bounds<Pixels>]) -> Option<Pixels> {
    let first_bounds = bounds.first()?;
    let mut left = first_bounds.left();
    let mut right = first_bounds.right();

    for bound in bounds.iter().skip(1) {
        left = left.min(bound.left());
        right = right.max(bound.right());
    }

    Some(right - left)
}
