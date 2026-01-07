use gpui::{
    App, Bounds, Hsla, IntoElement, PathBuilder, PathStyle, Pixels, Point, StrokeOptions, Styled,
    Window, canvas, point, px,
};
use project::debugger::dap_store::DapAdapterDelegate;
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

fn lane_center_x(bounds: Bounds<Pixels>, left_padding: Pixels, lane: f32) -> Pixels {
    bounds.origin.x + left_padding + lane * LANE_WIDTH + LANE_WIDTH / 2.0
}

fn to_row_center(
    to_row: usize,
    row_height: Pixels,
    scroll_offset: Pixels,
    bounds: Bounds<Pixels>,
) -> Pixels {
    bounds.origin.y + to_row as f32 * row_height + row_height / 2.0 - scroll_offset
}

pub fn render_graph(graph: &GitGraph) -> impl IntoElement {
    let top_row = graph.list_state.logical_scroll_top();
    let row_height = graph.row_height;
    let scroll_offset = top_row.offset_in_item;
    let first_visible_row = top_row.item_ix;
    // this goes one row over to draw the lines off the screen correctly
    let last_visible_row = first_visible_row
        + (graph.list_state.viewport_bounds().size.height / row_height).ceil() as usize;
    let graph_width = px(16.0) * (4 as f32) + px(24.0);
    let loaded_commit_count = graph.graph.commits.len();

    let viewport_range = first_visible_row.min(loaded_commit_count.saturating_sub(1))
        ..(last_visible_row).min(loaded_commit_count);
    // todo! Figure out how we can avoid over allocating this data
    let rows = graph.graph.commits[viewport_range.clone()].to_vec();
    let commit_lines: Vec<_> = graph
        .graph
        .lines
        .iter()
        .filter(|line| {
            (line.full_interval.start >= viewport_range.start
                && line.full_interval.start <= viewport_range.end)
                || (line.full_interval.end >= viewport_range.start
                    && line.full_interval.end <= viewport_range.end)
        })
        .cloned()
        .collect();

    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, cx: &mut App| {
            window.paint_layer(bounds, |window| {
                let left_padding = px(12.0);
                let accent_colors = cx.theme().accents();

                for (row_idx, row) in rows.into_iter().enumerate() {
                    let row_color = accent_colors.color_for_index(row.color_idx as u32);
                    let row_y_center =
                        bounds.origin.y + row_idx as f32 * row_height + row_height / 2.0
                            - scroll_offset;

                    let commit_x = lane_center_x(bounds, left_padding, row.lane as f32);

                    draw_commit_circle(commit_x, row_y_center, row_color, window);
                }

                for line in commit_lines {
                    let Some((start_segment_idx, start_column)) =
                        line.get_first_visible_segment_idx(first_visible_row)
                    else {
                        continue;
                    };

                    let line_color = accent_colors.color_for_index(line.color_idx as u32);
                    // dbg!(&line);
                    let line_x = lane_center_x(bounds, left_padding, start_column as f32);

                    let start_row = line.full_interval.start as i32 - first_visible_row as i32;

                    let from_y = bounds.origin.y + start_row as f32 * row_height + row_height / 2.0
                        - scroll_offset
                        + COMMIT_CIRCLE_RADIUS;

                    let mut current_row = from_y;
                    let mut current_column = line_x;

                    let mut builder = PathBuilder::stroke(LINE_WIDTH);
                    builder.move_to(point(line_x, from_y));

                    let segments = &line.segments[start_segment_idx..];

                    for (segment_idx, segment) in segments.iter().enumerate() {
                        let is_last = segment_idx + 1 == segments.len();

                        match segment {
                            CommitLineSegment::Straight { to_row } => {
                                let mut dest_row = to_row_center(
                                    to_row - first_visible_row,
                                    row_height,
                                    scroll_offset,
                                    bounds,
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
                                let mut to_column =
                                    lane_center_x(bounds, left_padding, *to_column as f32);

                                let mut to_row = to_row_center(
                                    // todo! subtract with overflow here
                                    *on_row - first_visible_row,
                                    row_height,
                                    scroll_offset,
                                    bounds,
                                );

                                // This means that this branch was a checkout
                                let going_right = to_column > current_column;
                                let column_shift = if going_right {
                                    COMMIT_CIRCLE_RADIUS + COMMIT_CIRCLE_STROKE_WIDTH
                                } else {
                                    -COMMIT_CIRCLE_RADIUS - COMMIT_CIRCLE_STROKE_WIDTH
                                };

                                let control = match curve_kind {
                                    CurveKind::Checkout => {
                                        if is_last {
                                            to_column -= column_shift;
                                        }
                                        builder.move_to(point(
                                            current_column,
                                            current_row - COMMIT_CIRCLE_RADIUS, // - COMMIT_CIRCLE_STROKE_WIDTH,
                                        ));
                                        point(to_column, to_row)
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

                                if (to_column - current_column).abs() > LANE_WIDTH {
                                    let column_shift =
                                        if going_right { LANE_WIDTH } else { -LANE_WIDTH };

                                    // todo! we should only subtract the commit circle radius if this is the first segment

                                    let start_curve = match curve_kind {
                                        CurveKind::Checkout => point(
                                            current_column,
                                            to_row - (row_height / 2.0).into(),
                                        ),
                                        CurveKind::Merge => point(
                                            current_column + column_shift,
                                            current_row - COMMIT_CIRCLE_RADIUS,
                                        ),
                                    };

                                    builder.line_to(start_curve);
                                    builder.move_to(start_curve);
                                }

                                builder.curve_to(point(to_column, to_row), control);
                                current_row = to_row;
                                current_column = to_column;
                                builder.move_to(point(current_column, current_row));
                            }
                        }
                    }

                    builder.close();
                    if let Ok(path) = builder.build() {
                        window.paint_path(path, line_color);
                    }
                }
            })
        },
    )
    .w(graph_width)
    .h_full()
}

fn draw_commit_circle(center_x: Pixels, center_y: Pixels, color: Hsla, window: &mut Window) {
    let radius = COMMIT_CIRCLE_RADIUS;
    let stroke_width = COMMIT_CIRCLE_STROKE_WIDTH;

    let mut builder = PathBuilder::stroke(stroke_width);

    // Start at the rightmost point of the circle
    builder.move_to(point(center_x + radius, center_y));

    // Draw the circle using two arc_to calls (top half, then bottom half)
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
