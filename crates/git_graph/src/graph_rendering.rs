use std::collections::BTreeMap;

use gpui::{
    App, Bounds, Context, Hsla, IntoElement, PathBuilder, Pixels, Point, Styled, Window, canvas,
    point, px,
};
use theme::AccentColors;
use ui::ActiveTheme as _;

use crate::{
    GitGraph,
    graph::{CommitLineSegment, CurveKind},
};

const LANE_WIDTH: Pixels = px(16.0);
const LINE_WIDTH: Pixels = px(1.5);
const COMMIT_CIRCLE_RADIUS: Pixels = px(4.5);
const COMMIT_CIRCLE_STROKE_WIDTH: Pixels = px(1.5);

pub fn accent_colors_count(accents: &AccentColors) -> usize {
    accents.0.len()
}

fn lane_center_x(
    bounds: Bounds<Pixels>,
    left_padding: Pixels,
    lane: f32,
    horizontal_scroll_offset: Pixels,
) -> Pixels {
    bounds.origin.x + left_padding + lane * LANE_WIDTH + LANE_WIDTH / 2.0 - horizontal_scroll_offset
}

fn to_row_center(
    to_row: usize,
    row_height: Pixels,
    scroll_offset: Pixels,
    bounds: Bounds<Pixels>,
) -> Pixels {
    bounds.origin.y + to_row as f32 * row_height + row_height / 2.0 - scroll_offset
}

struct LineRenderingParams {
    bounds: Bounds<Pixels>,
    left_padding: Pixels,
    horizontal_scroll_offset: Pixels,
    vertical_scroll_offset: Pixels,
    row_height: Pixels,
    first_visible_row: usize,
}

pub fn render_graph(graph: &GitGraph, cx: &mut Context<GitGraph>) -> impl IntoElement {
    let row_height = graph.row_height;
    let table_state = graph.table_interaction_state.read(cx);
    let viewport_height = table_state
        .scroll_handle
        .0
        .borrow()
        .last_item_size
        .map(|size| size.item.height)
        .unwrap_or(px(600.0));
    let loaded_commit_count = graph.graph.commits.len();

    let content_height = row_height * loaded_commit_count;
    let max_scroll = (content_height - viewport_height).max(px(0.));
    let scroll_offset_y = (-table_state.scroll_offset().y).clamp(px(0.), max_scroll);

    let first_visible_row = (scroll_offset_y / row_height).floor() as usize;
    let vertical_scroll_offset = scroll_offset_y - (first_visible_row as f32 * row_height);
    let horizontal_scroll_offset = graph.horizontal_scroll_offset;

    let left_padding = px(12.0);
    let max_lanes = graph.graph.max_lanes.max(1);
    let graph_width = LANE_WIDTH * max_lanes as f32 + left_padding * 2.0;
    let last_visible_row = first_visible_row + (viewport_height / row_height).ceil() as usize + 1;

    let viewport_range = first_visible_row.min(loaded_commit_count.saturating_sub(1))
        ..(last_visible_row).min(loaded_commit_count);
    let rows = graph.graph.commits[viewport_range.clone()].to_vec();
    let commit_lines: Vec<_> = graph
        .graph
        .lines
        .iter()
        .filter(|line| {
            line.full_interval.start <= viewport_range.end
                && line.full_interval.end >= viewport_range.start
        })
        .cloned()
        .collect();

    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, cx: &mut App| {
            window.paint_layer(bounds, |window| {
                let accent_colors = cx.theme().accents();

                for (row_idx, row) in rows.into_iter().enumerate() {
                    let row_color = accent_colors.color_for_index(row.color_idx as u32);
                    let row_y_center =
                        bounds.origin.y + row_idx as f32 * row_height + row_height / 2.0
                            - vertical_scroll_offset;

                    let commit_x = lane_center_x(
                        bounds,
                        left_padding,
                        row.lane as f32,
                        horizontal_scroll_offset,
                    );

                    draw_commit_circle(commit_x, row_y_center, row_color, window);
                }

                let params = LineRenderingParams {
                    bounds,
                    left_padding,
                    horizontal_scroll_offset,
                    vertical_scroll_offset,
                    row_height,
                    first_visible_row,
                };

                let mut paths_by_color: BTreeMap<usize, PathBuilder> = BTreeMap::default();

                for line in commit_lines {
                    let Some((start_segment_idx, start_column)) =
                        line.get_first_visible_segment_idx(first_visible_row)
                    else {
                        continue;
                    };

                    let builder = paths_by_color
                        .entry(line.color_idx)
                        .or_insert_with(|| PathBuilder::stroke(LINE_WIDTH));

                    add_line_segments_to_path(
                        builder,
                        &line.segments[start_segment_idx..],
                        start_column,
                        line.full_interval.start,
                        &params,
                    );
                }

                for (color_idx, builder) in paths_by_color {
                    let line_color = accent_colors.color_for_index(color_idx as u32);
                    if let Ok(path) = builder.build() {
                        window.paint_layer(bounds, |window| {
                            window.paint_path(path, line_color);
                        });
                    }
                }
            })
        },
    )
    .w(graph_width)
    .h_full()
}

fn add_line_segments_to_path(
    builder: &mut PathBuilder,
    segments: &[CommitLineSegment],
    start_column: usize,
    line_start_row: usize,
    params: &LineRenderingParams,
) {
    let line_x = lane_center_x(
        params.bounds,
        params.left_padding,
        start_column as f32,
        params.horizontal_scroll_offset,
    );

    let start_row = line_start_row as i32 - params.first_visible_row as i32;
    let from_y =
        params.bounds.origin.y + start_row as f32 * params.row_height + params.row_height / 2.0
            - params.vertical_scroll_offset
            + COMMIT_CIRCLE_RADIUS;

    let mut current_row = from_y;
    let mut current_column = line_x;

    builder.move_to(point(line_x, from_y));

    for (segment_idx, segment) in segments.iter().enumerate() {
        let is_last = segment_idx + 1 == segments.len();

        match segment {
            CommitLineSegment::Straight { to_row } => {
                let mut dest_row = to_row_center(
                    to_row - params.first_visible_row,
                    params.row_height,
                    params.vertical_scroll_offset,
                    params.bounds,
                );
                if is_last {
                    dest_row -= COMMIT_CIRCLE_RADIUS;
                }

                let dest_point = point(current_column, dest_row);

                current_row = dest_point.y;
                builder.line_to(dest_point);
                builder.move_to(dest_point);
            }
            CommitLineSegment::Curve {
                to_column,
                on_row,
                curve_kind,
            } => {
                let (new_row, new_column) = draw_curve_segment(
                    builder,
                    current_row,
                    current_column,
                    *to_column,
                    *on_row,
                    curve_kind,
                    is_last,
                    params,
                );
                current_row = new_row;
                current_column = new_column;
            }
        }
    }
}

fn draw_curve_segment(
    builder: &mut PathBuilder,
    current_row: Pixels,
    current_column: Pixels,
    to_column: usize,
    on_row: usize,
    curve_kind: &CurveKind,
    is_last: bool,
    params: &LineRenderingParams,
) -> (Pixels, Pixels) {
    let mut to_column = lane_center_x(
        params.bounds,
        params.left_padding,
        to_column as f32,
        params.horizontal_scroll_offset,
    );

    let mut to_row = to_row_center(
        on_row - params.first_visible_row,
        params.row_height,
        params.vertical_scroll_offset,
        params.bounds,
    );

    let going_right = to_column > current_column;
    let column_shift = if going_right {
        COMMIT_CIRCLE_RADIUS + COMMIT_CIRCLE_STROKE_WIDTH
    } else {
        -COMMIT_CIRCLE_RADIUS - COMMIT_CIRCLE_STROKE_WIDTH
    };

    let control: Point<Pixels> = match curve_kind {
        CurveKind::Checkout => {
            if is_last {
                to_column -= column_shift;
            }
            builder.move_to(point(current_column, current_row));
            point(current_column, to_row)
        }
        CurveKind::Merge => {
            if is_last {
                to_row -= COMMIT_CIRCLE_RADIUS;
            }
            builder.move_to(point(
                current_column + column_shift,
                current_row - COMMIT_CIRCLE_RADIUS,
            ));
            point(to_column, current_row)
        }
    };

    match curve_kind {
        CurveKind::Checkout if (to_row - current_row).abs() > params.row_height => {
            let start_curve = point(current_column, current_row + params.row_height);
            builder.line_to(start_curve);
            builder.move_to(start_curve);
        }
        CurveKind::Merge if (to_column - current_column).abs() > LANE_WIDTH => {
            let column_shift = if going_right { LANE_WIDTH } else { -LANE_WIDTH };

            let start_curve = point(
                current_column + column_shift,
                current_row - COMMIT_CIRCLE_RADIUS,
            );

            builder.line_to(start_curve);
            builder.move_to(start_curve);
        }
        _ => {}
    };

    builder.curve_to(point(to_column, to_row), control);
    builder.move_to(point(to_column, to_row));

    (to_row, to_column)
}

fn draw_commit_circle(center_x: Pixels, center_y: Pixels, color: Hsla, window: &mut Window) {
    let radius = COMMIT_CIRCLE_RADIUS;
    let stroke_width = COMMIT_CIRCLE_STROKE_WIDTH;

    let mut builder = PathBuilder::stroke(stroke_width);

    builder.move_to(point(center_x + radius, center_y));

    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x - radius, center_y),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x + radius, center_y),
    );
    builder.close();

    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}
