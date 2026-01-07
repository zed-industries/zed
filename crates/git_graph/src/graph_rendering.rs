use gpui::{
    App, Bounds, Hsla, IntoElement, PathBuilder, Pixels, Point, Styled, Window, canvas, point, px,
};
use theme::AccentColors;
use ui::{ActiveTheme as _, div};

use crate::{GitGraph, graph::LineType};

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
                    let line_color = accent_colors.color_for_index(line.color_idx as u32);
                    let line_x = lane_center_x(bounds, left_padding, line.child_column as f32);

                    let start_row = line.full_interval.start as i32 - first_visible_row as i32;
                    let end_row = line.full_interval.end as i32 - first_visible_row as i32;

                    let from_y = bounds.origin.y + start_row as f32 * row_height + row_height / 2.0
                        - scroll_offset
                        + COMMIT_CIRCLE_RADIUS;
                    let to_y = bounds.origin.y + end_row as f32 * row_height + row_height / 2.0
                        - scroll_offset
                        - COMMIT_CIRCLE_RADIUS;

                    draw_straight_line(
                        window, line_x, from_y, line_x, to_y, LINE_WIDTH, line_color,
                    );
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

fn draw_straight_line(
    window: &mut Window,
    from_x: Pixels,
    from_y: Pixels,
    to_x: Pixels,
    to_y: Pixels,
    line_width: Pixels,
    color: Hsla,
) {
    let half_width = line_width / 2.0;

    // Create a path as a thin rectangle for anti-aliased rendering
    let mut path = gpui::Path::new(Point::new(from_x - half_width, from_y));
    path.line_to(Point::new(from_x + half_width, from_y));
    path.line_to(Point::new(to_x + half_width, to_y));
    path.line_to(Point::new(to_x - half_width, to_y));
    window.paint_path(path, color);
}

fn draw_s_curve(
    window: &mut Window,
    from_x: Pixels,
    from_y: Pixels,
    to_x: Pixels,
    to_y: Pixels,
    line_width: Pixels,
    color: Hsla,
) {
    if from_x == to_x {
        draw_straight_line(window, from_x, from_y, to_x, to_y, line_width, color);
        return;
    }

    let segments = 12;
    let half_width = f32::from(line_width / 2.0);
    let mid_y = (from_y + to_y) / 2.0;

    let mut left_points = Vec::with_capacity(segments + 1);
    let mut right_points = Vec::with_capacity(segments + 1);

    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let (x, y) = cubic_bezier(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t);

        let (dx, dy) =
            cubic_bezier_derivative(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t);
        let dx_f = f32::from(dx);
        let dy_f = f32::from(dy);
        let len = (dx_f * dx_f + dy_f * dy_f).sqrt();

        let (nx, ny) = if len > 0.001 {
            (-dy_f / len * half_width, dx_f / len * half_width)
        } else {
            (half_width, 0.0)
        };

        left_points.push(Point::new(x - px(nx), y - px(ny)));
        right_points.push(Point::new(x + px(nx), y + px(ny)));
    }

    let mut path = gpui::Path::new(left_points[0]);
    for point in left_points.iter().skip(1) {
        path.line_to(*point);
    }
    for point in right_points.iter().rev() {
        path.line_to(*point);
    }
    window.paint_path(path, color);
}

fn cubic_bezier(
    p0x: Pixels,
    p0y: Pixels,
    p1x: Pixels,
    p1y: Pixels,
    p2x: Pixels,
    p2y: Pixels,
    p3x: Pixels,
    p3y: Pixels,
    t: f32,
) -> (Pixels, Pixels) {
    let inv_t = 1.0 - t;
    let inv_t2 = inv_t * inv_t;
    let inv_t3 = inv_t2 * inv_t;
    let t2 = t * t;
    let t3 = t2 * t;

    let x = inv_t3 * p0x + 3.0 * inv_t2 * t * p1x + 3.0 * inv_t * t2 * p2x + t3 * p3x;
    let y = inv_t3 * p0y + 3.0 * inv_t2 * t * p1y + 3.0 * inv_t * t2 * p2y + t3 * p3y;
    (x, y)
}

fn cubic_bezier_derivative(
    p0x: Pixels,
    p0y: Pixels,
    p1x: Pixels,
    p1y: Pixels,
    p2x: Pixels,
    p2y: Pixels,
    p3x: Pixels,
    p3y: Pixels,
    t: f32,
) -> (Pixels, Pixels) {
    let inv_t = 1.0 - t;
    let inv_t2 = inv_t * inv_t;
    let t2 = t * t;

    let dx = 3.0 * inv_t2 * (p1x - p0x) + 6.0 * inv_t * t * (p2x - p1x) + 3.0 * t2 * (p3x - p2x);
    let dy = 3.0 * inv_t2 * (p1y - p0y) + 6.0 * inv_t * t * (p2y - p1y) + 3.0 * t2 * (p3y - p2y);
    (dx, dy)
}
