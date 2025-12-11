use gpui::{App, Bounds, Hsla, IntoElement, Pixels, Point, Styled, Window, canvas, px};
use theme::AccentColors;
use ui::ActiveTheme as _;

use crate::{
    GitGraph,
    graph::{GraphLine, LineType},
};

pub fn accent_colors_count(accents: &AccentColors) -> usize {
    accents.0.len()
}

const LANE_WIDTH: Pixels = px(16.0);
const LINE_WIDTH: Pixels = px(1.5);

pub fn render_graph(graph: &GitGraph) -> impl IntoElement {
    let top_row = graph.list_state.logical_scroll_top();
    let row_height = graph.row_height;
    let scroll_offset = top_row.offset_in_item;
    let first_visible_row = top_row.item_ix;
    let graph_width = px(16.0) * (4 as f32) + px(24.0);
    let loaded_commit_count = graph.graph.commits.len();

    // todo! Figure out how we can avoid over allocating this data
    let rows = graph.graph.commits[first_visible_row.min(loaded_commit_count.saturating_sub(1))
        ..(first_visible_row + 50).min(loaded_commit_count)]
        .to_vec();

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
                    let _row_x_coordinate =
                        bounds.origin.x + row.lane * LANE_WIDTH + LANE_WIDTH / 2.0;

                    for line in row.lines.iter() {
                        let line_color = accent_colors.color_for_index(line.color_idx as u32);

                        let from_x = bounds.origin.x
                            + line.from_lane * LANE_WIDTH
                            + LANE_WIDTH / 2.0
                            + left_padding;
                        let to_x = bounds.origin.x
                            + line.to_lane * LANE_WIDTH
                            + LANE_WIDTH / 2.0
                            + left_padding;

                        match line.line_type {
                            LineType::Straight => {
                                let start_y = if line.continues_from_above {
                                    row_y_center - row_height / 2.0
                                } else {
                                    row_y_center
                                };
                                let end_y = if line.ends_at_commit {
                                    row_y_center
                                } else {
                                    row_y_center + row_height / 2.0
                                };

                                draw_straight_line(
                                    window, from_x, start_y, from_x, end_y, LINE_WIDTH, line_color,
                                );
                            }
                            LineType::MergeDown | LineType::BranchOut => {
                                draw_s_curve(
                                    window,
                                    from_x,
                                    row_y_center,
                                    to_x,
                                    row_y_center + row_height / 2.0,
                                    LINE_WIDTH,
                                    line_color,
                                );
                            }
                        }
                    }

                    let commit_x = bounds.origin.x
                        + left_padding
                        + LANE_WIDTH * row.lane as f32
                        + LANE_WIDTH / 2.0;
                    let dot_radius = px(4.5);
                    let stroke_width = px(1.5);

                    // Draw colored outline only (hollow/transparent circle)
                    draw_circle_outline(
                        window,
                        commit_x,
                        row_y_center,
                        dot_radius,
                        stroke_width,
                        row_color,
                    );
                }
            })
        },
    )
    .w(graph_width)
    .h_full()
}

pub fn render_graph_cell(
    lane: usize,
    lines: Vec<GraphLine>,
    commit_color_idx: usize,
    row_height: Pixels,
    graph_width: Pixels,
    accent_colors: AccentColors,
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, _cx: &mut App| {
            let accent_colors = &accent_colors;
            let lane_width = px(16.0);
            let left_padding = px(12.0);
            let y_top = bounds.origin.y;
            let y_center = bounds.origin.y + row_height / 2.0;
            let y_bottom = bounds.origin.y + row_height;
            let line_width = px(1.5);

            for line in &lines {
                let color = accent_colors.color_for_index(line.color_idx as u32);
                let from_x = bounds.origin.x
                    + left_padding
                    + lane_width * line.from_lane as f32
                    + lane_width / 2.0;
                let to_x = bounds.origin.x
                    + left_padding
                    + lane_width * line.to_lane as f32
                    + lane_width / 2.0;

                match line.line_type {
                    LineType::Straight => {
                        let start_y = if line.continues_from_above {
                            y_top
                        } else {
                            y_center
                        };
                        let end_y = if line.ends_at_commit {
                            y_center
                        } else {
                            y_bottom
                        };

                        draw_straight_line(
                            window, from_x, start_y, from_x, end_y, line_width, color,
                        );
                    }
                    LineType::MergeDown | LineType::BranchOut => {
                        draw_s_curve(window, from_x, y_center, to_x, y_bottom, line_width, color);
                    }
                }
            }

            let commit_x =
                bounds.origin.x + left_padding + lane_width * lane as f32 + lane_width / 2.0;
            let commit_color = accent_colors.color_for_index(commit_color_idx as u32);
            let dot_radius = px(4.5);
            let stroke_width = px(1.5);

            // Draw colored outline only (hollow/transparent circle)
            draw_circle_outline(
                window,
                commit_x,
                y_center,
                dot_radius,
                stroke_width,
                commit_color,
            );
        },
    )
    .w(graph_width)
    .h(row_height)
}

fn draw_circle_outline(
    window: &mut Window,
    center_x: Pixels,
    center_y: Pixels,
    radius: Pixels,
    stroke_width: Pixels,
    color: Hsla,
) {
    // Draw a circle outline using path segments
    let segments = 32;
    let outer_radius = radius;
    let inner_radius = radius - stroke_width;

    let mut outer_points = Vec::with_capacity(segments);
    let mut inner_points = Vec::with_capacity(segments);

    for i in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        outer_points.push(Point::new(
            center_x + px(f32::from(outer_radius) * cos_a),
            center_y + px(f32::from(outer_radius) * sin_a),
        ));
        inner_points.push(Point::new(
            center_x + px(f32::from(inner_radius) * cos_a),
            center_y + px(f32::from(inner_radius) * sin_a),
        ));
    }

    // Create path: outer circle clockwise, then inner circle counter-clockwise
    let mut path = gpui::Path::new(outer_points[0]);
    for point in outer_points.iter().skip(1) {
        path.line_to(*point);
    }
    path.line_to(outer_points[0]); // Close outer circle

    // Connect to inner circle and trace it in reverse
    path.line_to(inner_points[0]);
    for point in inner_points.iter().rev() {
        path.line_to(*point);
    }

    window.paint_path(path, color);
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
