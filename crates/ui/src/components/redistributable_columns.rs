use std::rc::Rc;

use gpui::{
    AbsoluteLength, AppContext as _, Bounds, DefiniteLength, DragMoveEvent, Empty, Entity, Length,
    WeakEntity,
};
use itertools::intersperse_with;

use super::data_table::table_row::{IntoTableRow as _, TableRow};
use crate::{
    ActiveTheme as _, AnyElement, App, Context, Div, FluentBuilder as _, InteractiveElement,
    IntoElement, ParentElement, Pixels, StatefulInteractiveElement, Styled, Window, div, h_flex,
    px,
};

const RESIZE_COLUMN_WIDTH: f32 = 8.0;
const RESIZE_DIVIDER_WIDTH: f32 = 1.0;

#[derive(Debug)]
struct DraggedColumn(usize);

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
pub struct HeaderResizeInfo {
    pub columns_state: WeakEntity<RedistributableColumnsState>,
    pub resize_behavior: TableRow<TableResizeBehavior>,
}

impl HeaderResizeInfo {
    pub fn from_state(columns_state: &Entity<RedistributableColumnsState>, cx: &App) -> Self {
        let resize_behavior = columns_state.read(cx).resize_behavior().clone();
        Self {
            columns_state: columns_state.downgrade(),
            resize_behavior,
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;
        let bounds_width = bounds.right() - bounds.left();
        if bounds_width <= px(0.) {
            return;
        }

        let mut col_position = 0.0;
        let rem_size = window.rem_size();
        let col_idx = drag_event.drag(cx).0;

        let divider_width = Self::get_fraction(
            &DefiniteLength::Absolute(AbsoluteLength::Pixels(px(RESIZE_DIVIDER_WIDTH))),
            bounds_width,
            rem_size,
        );

        let mut widths = self
            .committed_widths
            .map_ref(|length| Self::get_fraction(length, bounds_width, rem_size));

        for length in widths[0..=col_idx].iter() {
            col_position += length + divider_width;
        }

        let mut total_length_ratio = col_position;
        for length in widths[col_idx + 1..].iter() {
            total_length_ratio += length;
        }
        let cols = self.resize_behavior.cols();
        total_length_ratio += (cols - 1 - col_idx) as f32 * divider_width;

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;
        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - col_position - divider_width / 2.0;

        Self::drag_column_handle(diff, col_idx, &mut widths, &self.resize_behavior);

        self.preview_widths = widths.map(DefiniteLength::Fraction);
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

pub fn bind_redistributable_columns(
    container: Div,
    columns_state: Entity<RedistributableColumnsState>,
) -> Div {
    container
        .on_drag_move::<DraggedColumn>({
            let columns_state = columns_state.clone();
            move |event, window, cx| {
                columns_state.update(cx, |columns, cx| {
                    columns.on_drag_move(event, window, cx);
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
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (column_widths, resize_behavior) = {
        let state = columns_state.read(cx);
        (state.widths_to_render(), state.resize_behavior().clone())
    };

    let mut column_ix = 0;
    let resize_behavior = Rc::new(resize_behavior);
    let dividers = intersperse_with(
        column_widths
            .as_slice()
            .iter()
            .copied()
            .map(|width| resize_spacer(width).into_any_element()),
        || {
            let current_column_ix = column_ix;
            let resize_behavior = Rc::clone(&resize_behavior);
            let columns_state = columns_state.clone();
            column_ix += 1;

            window.with_id(current_column_ix, |window| {
                let mut resize_divider = div()
                    .id(current_column_ix)
                    .relative()
                    .top_0()
                    .w(px(RESIZE_DIVIDER_WIDTH))
                    .h_full()
                    .bg(cx.theme().colors().border.opacity(0.8));

                let mut resize_handle = div()
                    .id("column-resize-handle")
                    .absolute()
                    .left_neg_0p5()
                    .w(px(RESIZE_COLUMN_WIDTH))
                    .h_full();

                if resize_behavior[current_column_ix].is_resizable() {
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
                        .on_click({
                            let columns_state = columns_state.clone();
                            move |event, window, cx| {
                                if event.click_count() >= 2 {
                                    columns_state.update(cx, |columns, _| {
                                        columns.reset_column_to_initial_width(
                                            current_column_ix,
                                            window,
                                        );
                                    });
                                }

                                cx.stop_propagation();
                            }
                        })
                        .on_drag(DraggedColumn(current_column_ix), {
                            let is_highlighted = is_highlighted.clone();
                            move |_, _offset, _window, cx| {
                                is_highlighted.write(cx, true);
                                cx.new(|_cx| Empty)
                            }
                        })
                        .on_drop::<DraggedColumn>(move |_, _, cx| {
                            is_highlighted.write(cx, false);
                            columns_state.update(cx, |state, _| {
                                state.commit_preview();
                            });
                        });
                }

                resize_divider.child(resize_handle).into_any_element()
            })
        },
    );

    h_flex()
        .id("resize-handles")
        .absolute()
        .inset_0()
        .w_full()
        .children(dividers)
        .into_any_element()
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
