use gpui::{App, Bounds, Hsla, IntoElement, Pixels, Point, Styled, Window, canvas, px};

use crate::commit_data::LineType;

pub const BRANCH_COLORS: &[Hsla; 34] = &[
    Hsla {
        h: 200.0 / 360.0,
        s: 0.9,
        l: 0.55,
        a: 1.0,
    }, // Cyan
    Hsla {
        h: 320.0 / 360.0,
        s: 0.9,
        l: 0.55,
        a: 1.0,
    }, // Magenta/Pink
    Hsla {
        h: 45.0 / 360.0,
        s: 0.95,
        l: 0.50,
        a: 1.0,
    }, // Orange
    Hsla {
        h: 120.0 / 360.0,
        s: 0.8,
        l: 0.45,
        a: 1.0,
    }, // Green
    Hsla {
        h: 270.0 / 360.0,
        s: 0.8,
        l: 0.60,
        a: 1.0,
    }, // Purple
    Hsla {
        h: 0.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Red
    Hsla {
        h: 180.0 / 360.0,
        s: 0.8,
        l: 0.45,
        a: 1.0,
    }, // Teal
    Hsla {
        h: 60.0 / 360.0,
        s: 0.9,
        l: 0.50,
        a: 1.0,
    }, // Yellow
    Hsla {
        h: 210.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Blue
    Hsla {
        h: 340.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Rose
    Hsla {
        h: 90.0 / 360.0,
        s: 0.75,
        l: 0.50,
        a: 1.0,
    }, // Lime
    Hsla {
        h: 240.0 / 360.0,
        s: 0.75,
        l: 0.60,
        a: 1.0,
    }, // Indigo
    Hsla {
        h: 30.0 / 360.0,
        s: 0.90,
        l: 0.50,
        a: 1.0,
    }, // Orange-Red
    Hsla {
        h: 160.0 / 360.0,
        s: 0.75,
        l: 0.45,
        a: 1.0,
    }, // Sea Green
    Hsla {
        h: 290.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Violet
    Hsla {
        h: 15.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Coral
    Hsla {
        h: 175.0 / 360.0,
        s: 0.70,
        l: 0.50,
        a: 1.0,
    }, // Aqua
    Hsla {
        h: 300.0 / 360.0,
        s: 0.65,
        l: 0.55,
        a: 1.0,
    }, // Orchid
    Hsla {
        h: 75.0 / 360.0,
        s: 0.80,
        l: 0.45,
        a: 1.0,
    }, // Yellow-Green
    Hsla {
        h: 225.0 / 360.0,
        s: 0.75,
        l: 0.55,
        a: 1.0,
    }, // Slate Blue
    Hsla {
        h: 350.0 / 360.0,
        s: 0.80,
        l: 0.50,
        a: 1.0,
    }, // Crimson
    Hsla {
        h: 140.0 / 360.0,
        s: 0.70,
        l: 0.50,
        a: 1.0,
    }, // Spring Green
    Hsla {
        h: 255.0 / 360.0,
        s: 0.65,
        l: 0.60,
        a: 1.0,
    }, // Periwinkle
    Hsla {
        h: 20.0 / 360.0,
        s: 0.85,
        l: 0.50,
        a: 1.0,
    }, // Burnt Orange
    Hsla {
        h: 190.0 / 360.0,
        s: 0.75,
        l: 0.50,
        a: 1.0,
    }, // Steel Blue
    Hsla {
        h: 330.0 / 360.0,
        s: 0.75,
        l: 0.55,
        a: 1.0,
    }, // Hot Pink
    Hsla {
        h: 100.0 / 360.0,
        s: 0.65,
        l: 0.50,
        a: 1.0,
    }, // Olive Green
    Hsla {
        h: 265.0 / 360.0,
        s: 0.60,
        l: 0.55,
        a: 1.0,
    }, // Lavender
    Hsla {
        h: 5.0 / 360.0,
        s: 0.80,
        l: 0.55,
        a: 1.0,
    }, // Tomato
    Hsla {
        h: 150.0 / 360.0,
        s: 0.65,
        l: 0.50,
        a: 1.0,
    }, // Medium Sea Green
    Hsla {
        h: 280.0 / 360.0,
        s: 0.55,
        l: 0.55,
        a: 1.0,
    }, // Medium Purple
    Hsla {
        h: 35.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Gold
    Hsla {
        h: 195.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Light Blue
    Hsla {
        h: 310.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Medium Violet
];

pub fn render_graph_continuation(
    lines: Vec<crate::commit_data::GraphLine>,
    graph_width: Pixels,
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, _cx: &mut App| {
            let lane_width = px(16.0);
            let left_padding = px(12.0);
            let y_top = bounds.origin.y;
            let y_bottom = bounds.origin.y + bounds.size.height;
            let line_width = px(1.5);

            for line in &lines {
                // Only draw straight continuation lines, not branch-outs (those are drawn in the cell)
                let (lane, color_idx) = match line.line_type {
                    LineType::Straight if !line.ends_at_commit => (line.from_lane, line.color_idx),
                    _ => continue,
                };

                let color = BRANCH_COLORS[color_idx % BRANCH_COLORS.len()];
                let x =
                    bounds.origin.x + left_padding + lane_width * lane as f32 + lane_width / 2.0;

                draw_straight_line(window, x, y_top, x, y_bottom, line_width, color);
            }
        },
    )
    .w(graph_width)
    .h_full()
}

pub fn render_graph_cell(
    lane: usize,
    lines: Vec<crate::commit_data::GraphLine>,
    commit_color_idx: usize,
    row_height: Pixels,
    graph_width: Pixels,
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, _cx: &mut App| {
            let lane_width = px(16.0);
            let left_padding = px(12.0);
            let y_top = bounds.origin.y;
            let y_center = bounds.origin.y + row_height / 2.0;
            let y_bottom = bounds.origin.y + row_height;
            let line_width = px(1.5);

            for line in &lines {
                let color = BRANCH_COLORS[line.color_idx % BRANCH_COLORS.len()];
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
            let commit_color = BRANCH_COLORS[commit_color_idx % BRANCH_COLORS.len()];
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

pub enum BadgeType {
    CurrentBranch(String, bool), // name, has_origin
    LocalBranch(String, bool),   // name, has_origin
    RemoteBranch(String),        // full name like "origin/dev"
    Tag(String),
}

pub fn parse_refs_to_badges(refs: &[String]) -> Vec<BadgeType> {
    use std::collections::HashSet;

    let mut result = Vec::new();
    let mut local_branches: HashSet<String> = HashSet::new();
    let mut remote_branches: HashSet<String> = HashSet::new();
    let mut current_branch: Option<String> = None;

    for ref_name in refs {
        if ref_name.starts_with("HEAD -> ") {
            if let Some(branch) = ref_name.strip_prefix("HEAD -> ") {
                current_branch = Some(branch.to_string());
                local_branches.insert(branch.to_string());
            }
        } else if let Some(tag) = ref_name.strip_prefix("tag: ") {
            result.push(BadgeType::Tag(tag.to_string()));
        } else if let Some(remote) = ref_name.strip_prefix("origin/") {
            if remote != "HEAD" {
                remote_branches.insert(remote.to_string());
            }
        } else if !ref_name.contains("HEAD") {
            local_branches.insert(ref_name.clone());
        }
    }

    let mut branch_badges = Vec::new();
    if let Some(ref current) = current_branch {
        let has_origin = remote_branches.contains(current);
        branch_badges.push(BadgeType::CurrentBranch(current.clone(), has_origin));
        remote_branches.remove(current);
        local_branches.remove(current);
    }

    let mut local_sorted: Vec<_> = local_branches.iter().cloned().collect();
    local_sorted.sort();
    for branch in local_sorted {
        let has_origin = remote_branches.contains(&branch);
        branch_badges.push(BadgeType::LocalBranch(branch.clone(), has_origin));
        remote_branches.remove(&branch);
    }

    let mut remote_sorted: Vec<_> = remote_branches.iter().cloned().collect();
    remote_sorted.sort();
    for branch in remote_sorted {
        branch_badges.push(BadgeType::RemoteBranch(format!("origin/{}", branch)));
    }

    let tags: Vec<_> = result.into_iter().collect();
    branch_badges.extend(tags);
    branch_badges
}
