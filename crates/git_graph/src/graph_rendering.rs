use gpui::{
    App, Bounds, Hsla, IntoElement, Pixels, Point, SharedString, Styled, Window, canvas,
    hsla, px,
};
use time::{OffsetDateTime, UtcOffset};
use ui::prelude::*;

use crate::commit_data::LineType;

pub const BRANCH_COLORS: &[Hsla] = &[
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

pub fn format_timestamp(timestamp: i64) -> String {
    let Ok(datetime) = OffsetDateTime::from_unix_timestamp(timestamp) else {
        return "Unknown".to_string();
    };

    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_datetime = datetime.to_offset(local_offset);

    let format = time::format_description::parse("[day] [month repr:short] [year] [hour]:[minute]")
        .unwrap_or_default();
    local_datetime.format(&format).unwrap_or_default()
}

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
            let line_width = px(2.0);

            let mut lanes_to_draw: std::collections::HashMap<usize, usize> =
                std::collections::HashMap::new();

            for line in &lines {
                match line.line_type {
                    LineType::Straight => {
                        if !line.ends_at_commit {
                            lanes_to_draw
                                .entry(line.from_lane)
                                .or_insert(line.color_idx);
                        }
                    }
                    LineType::BranchOut => {
                        lanes_to_draw.entry(line.to_lane).or_insert(line.color_idx);
                    }
                    _ => {}
                }
            }

            for (lane, color_idx) in &lanes_to_draw {
                let color = BRANCH_COLORS[*color_idx % BRANCH_COLORS.len()];
                let x =
                    bounds.origin.x + left_padding + lane_width * *lane as f32 + lane_width / 2.0;

                window.paint_quad(gpui::fill(
                    Bounds::new(
                        Point::new(x - line_width / 2.0, y_top),
                        gpui::size(line_width, y_bottom - y_top),
                    ),
                    color,
                ));
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
            let line_width = px(2.0);

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

                        window.paint_quad(gpui::fill(
                            Bounds::new(
                                Point::new(from_x - line_width / 2.0, start_y),
                                gpui::size(line_width, end_y - start_y),
                            ),
                            color,
                        ));
                    }
                    LineType::MergeDown | LineType::BranchOut => {
                        draw_s_curve(window, from_x, y_center, to_x, y_bottom, line_width, color);
                    }
                }
            }

            let commit_x =
                bounds.origin.x + left_padding + lane_width * lane as f32 + lane_width / 2.0;
            let commit_color = BRANCH_COLORS[commit_color_idx % BRANCH_COLORS.len()];
            let dot_radius = px(4.0);

            window.paint_quad(
                gpui::fill(
                    Bounds::centered_at(
                        Point::new(commit_x, y_center),
                        gpui::size(dot_radius * 2.0, dot_radius * 2.0),
                    ),
                    commit_color,
                )
                .corner_radii(dot_radius),
            );
        },
    )
    .w(graph_width)
    .h(row_height)
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
    let segments = 20;
    let half_width = f32::from(line_width / 2.0);

    let mid_y = (from_y + to_y) / 2.0;

    for i in 0..segments {
        let t0 = i as f32 / segments as f32;
        let t1 = (i + 1) as f32 / segments as f32;

        let (x0, y0) = cubic_bezier(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t0);
        let (x1, y1) = cubic_bezier(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t1);

        let dx = f32::from(x1 - x0);
        let dy = f32::from(y1 - y0);
        let len = (dx * dx + dy * dy).sqrt();

        if len > 0.01 {
            let nx = -dy / len * half_width;
            let ny = dx / len * half_width;

            let mut path = gpui::Path::new(Point::new(x0 - px(nx), y0 - px(ny)));
            path.line_to(Point::new(x0 + px(nx), y0 + px(ny)));
            path.line_to(Point::new(x1 + px(nx), y1 + px(ny)));
            path.line_to(Point::new(x1 - px(nx), y1 - px(ny)));
            window.paint_path(path, color);
        }
    }
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

enum RefType {
    Head,
    LocalBranch { is_current: bool },
    RemoteBranch,
    Tag,
}

struct RefBadge {
    display_name: SharedString,
    ref_type: RefType,
}

fn parse_refs(refs: &[String]) -> Vec<RefBadge> {
    let mut badges = Vec::new();
    let mut has_head = false;

    for ref_name in refs {
        if ref_name.starts_with("HEAD -> ") {
            has_head = true;
            if let Some(branch) = ref_name.strip_prefix("HEAD -> ") {
                badges.push(RefBadge {
                    display_name: branch.to_string().into(),
                    ref_type: RefType::LocalBranch { is_current: true },
                });
            }
        } else if ref_name == "HEAD" {
            has_head = true;
        } else if let Some(tag) = ref_name.strip_prefix("tag: ") {
            badges.push(RefBadge {
                display_name: tag.to_string().into(),
                ref_type: RefType::Tag,
            });
        } else if ref_name.starts_with("origin/") {
            if ref_name != "origin/HEAD" {
                badges.push(RefBadge {
                    display_name: ref_name.clone().into(),
                    ref_type: RefType::RemoteBranch,
                });
            }
        } else if !ref_name.contains("HEAD") {
            badges.push(RefBadge {
                display_name: ref_name.clone().into(),
                ref_type: RefType::LocalBranch { is_current: false },
            });
        }
    }

    if has_head && badges.is_empty() {
        badges.insert(
            0,
            RefBadge {
                display_name: "HEAD".into(),
                ref_type: RefType::Head,
            },
        );
    }

    badges
}

pub fn render_ref_badges(refs: &[String]) -> impl IntoElement {
    let badges = parse_refs(refs);

    h_flex()
        .gap_1()
        .children(badges.into_iter().take(4).map(|badge| {
            let (bg_color, text_color, icon) = match badge.ref_type {
                RefType::Head => (
                    hsla(180.0 / 360.0, 0.7, 0.35, 1.0),
                    hsla(0.0, 0.0, 1.0, 1.0),
                    IconName::ArrowUpRight,
                ),
                RefType::LocalBranch { is_current } => {
                    if is_current {
                        (
                            hsla(145.0 / 360.0, 0.65, 0.35, 1.0),
                            hsla(0.0, 0.0, 1.0, 1.0),
                            IconName::GitBranch,
                        )
                    } else {
                        (
                            hsla(145.0 / 360.0, 0.5, 0.4, 1.0),
                            hsla(0.0, 0.0, 1.0, 1.0),
                            IconName::GitBranch,
                        )
                    }
                }
                RefType::RemoteBranch => (
                    hsla(210.0 / 360.0, 0.6, 0.45, 1.0),
                    hsla(0.0, 0.0, 1.0, 1.0),
                    IconName::Server,
                ),
                RefType::Tag => (
                    hsla(35.0 / 360.0, 0.8, 0.45, 1.0),
                    hsla(0.0, 0.0, 1.0, 1.0),
                    IconName::Hash,
                ),
            };

            h_flex()
                .gap_1()
                .px_1p5()
                .py_0p5()
                .rounded_sm()
                .bg(bg_color)
                .child(
                    Icon::new(icon)
                        .size(IconSize::XSmall)
                        .color(Color::Custom(text_color)),
                )
                .child(
                    Label::new(badge.display_name)
                        .size(LabelSize::Small)
                        .color(Color::Custom(text_color)),
                )
        }))
}
