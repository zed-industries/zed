use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
use gpui::{
    Background, Context, Hsla, IntoElement, MouseMoveEvent, PathBuilder, Pixels, Point as GpuiPoint, Window,
    canvas, div, point, prelude::*, px, size,
};
use multi_buffer::Anchor;
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{ButtonCommon, ButtonSize, Clickable, IconButton, IconName, IconSize};

use crate::connector::{ConnectorCurve, ConnectorKind};
use crate::viewer::DiffViewer;

const DIFF_HIGHLIGHT_ALPHA: f32 = 0.5;

pub fn get_diff_colors<T>(cx: &Context<T>) -> (Hsla, Hsla, Hsla) {
    let theme = cx.theme();
    let mut deleted_bg = theme.status().deleted_background;
    deleted_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut created_bg = theme.status().created_background;
    created_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut modified_bg = theme.status().modified_background;
    modified_bg.a = DIFF_HIGHLIGHT_ALPHA;
    (deleted_bg, created_bg, modified_bg)
}

fn cubic_bezier(
    p0: GpuiPoint<Pixels>,
    p1: GpuiPoint<Pixels>,
    p2: GpuiPoint<Pixels>,
    p3: GpuiPoint<Pixels>,
    t: f32,
) -> GpuiPoint<Pixels> {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    point(
        px(uuu * f32::from(p0.x)
            + 3.0 * uu * t * f32::from(p1.x)
            + 3.0 * u * tt * f32::from(p2.x)
            + ttt * f32::from(p3.x)),
        px(uuu * f32::from(p0.y)
            + 3.0 * uu * t * f32::from(p1.y)
            + 3.0 * u * tt * f32::from(p2.y)
            + ttt * f32::from(p3.y)),
    )
}

impl DiffViewer {
    pub fn create_crushed_block_properties(
        &self,
        anchor: Anchor,
        color: Hsla,
    ) -> BlockProperties<Anchor> {
        BlockProperties {
            placement: BlockPlacement::Replace(anchor..=anchor),
            height: Some(2),
            style: BlockStyle::Fixed,
            render: Arc::new(move |_| div().absolute().w_full().h(px(2.0)).bg(color).into_any()),
            priority: 0,
        }
    }

    pub fn render_left_crushed_blocks(&self, cx: &Context<Self>) -> impl IntoElement {
        let curves = self.connector_curves.clone();
        let left_editor = self.left_editor.clone();
        let fallback_line_height = self.line_height;

        let (_deleted_bg, created_bg, _modified_bg) = get_diff_colors(cx);

        #[derive(Clone)]
        struct LeftCrushedCanvasData {
            curves: Vec<ConnectorCurve>,
            line_height: f32,
            left_scroll_pixels: f32,
            left_top_origin: f32,
            created_color: Hsla,
        }

        canvas(
            move |bounds, window, cx| {
                let (left_line_height, left_scroll_pixels, left_bounds) =
                    left_editor.update(cx, |editor, cx| {
                        let line_height = editor
                            .style()
                            .map(|style| {
                                f32::from(style.text.line_height_in_pixels(window.rem_size()))
                            })
                            .unwrap_or(fallback_line_height);

                        let scroll_rows = editor.scroll_position(cx).y;
                        let scroll_pixels = (scroll_rows as f32) * line_height;
                        let bounds = editor.last_bounds().cloned();

                        (line_height, scroll_pixels, bounds)
                    });

                let left_top_origin = left_bounds
                    .as_ref()
                    .map(|b| f32::from(b.origin.y))
                    .unwrap_or(f32::from(bounds.origin.y));

                LeftCrushedCanvasData {
                    curves,
                    line_height: left_line_height,
                    left_scroll_pixels,
                    left_top_origin,
                    created_color: created_bg,
                }
            },
            move |bounds, data, window, _cx| {
                if data.curves.is_empty() {
                    return;
                }

                let _header_height = data.left_top_origin - f32::from(bounds.origin.y);
                let crushed_thickness = 2.0;
                let minimal_block_height = 2.0;
                let mut deleted_lines_above = 0usize;

                for curve in &data.curves {
                    let left_len = curve.left_end.saturating_sub(curve.left_start) + 1;
                    let right_len = curve.right_end.saturating_sub(curve.right_start) + 1;

                    if curve.left_crushed {
                    } else if curve.right_crushed {
                        deleted_lines_above += left_len;
                    } else if right_len < left_len {
                        deleted_lines_above += left_len - right_len;
                    }

                    if curve.left_crushed {
                        let left_offset_rows = deleted_lines_above as f32;
                        let left_row = curve.focus_line as f32 + left_offset_rows;
                        let left_y = (left_row * data.line_height) - data.left_scroll_pixels;
                        let left_bottom = left_y + minimal_block_height;

                        let left_absolute_top = data.left_top_origin + left_y;
                        let left_absolute_bottom = data.left_top_origin + left_bottom;

                        let y_center = (left_absolute_top + left_absolute_bottom) * 0.5;
                        let top = y_center - crushed_thickness / 2.0;
                        let bottom = top + crushed_thickness;

                        if bottom > data.left_top_origin {
                            let clipped_top = top.max(data.left_top_origin);
                            let clipped_bottom = bottom.max(data.left_top_origin);

                            let mut builder = PathBuilder::fill();
                            builder.move_to(point(px(f32::from(bounds.origin.x)), px(clipped_top)));
                            builder.line_to(point(
                                px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                                px(clipped_top),
                            ));
                            builder.line_to(point(
                                px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                                px(clipped_bottom),
                            ));
                            builder
                                .line_to(point(px(f32::from(bounds.origin.x)), px(clipped_bottom)));
                            builder.close();

                            if let Ok(path) = builder.build() {
                                let background: Background = data.created_color.into();
                                window.paint_path(path, background);
                            }
                        }
                    }
                }
            },
        )
        .size_full()
    }

    pub fn render_right_crushed_blocks(&self, cx: &Context<Self>) -> impl IntoElement {
        let curves = self.connector_curves.clone();
        let right_editor = self.right_editor.clone();
        let fallback_line_height = self.line_height;

        let (deleted_bg, _created_bg, _modified_bg) = get_diff_colors(cx);

        #[derive(Clone)]
        struct RightCrushedCanvasData {
            curves: Vec<ConnectorCurve>,
            line_height: f32,
            right_scroll_pixels: f32,
            right_top_origin: f32,
            deleted_color: Hsla,
        }

        canvas(
            move |bounds, window, cx| {
                let (right_line_height, right_scroll_pixels, right_bounds) =
                    right_editor.update(cx, |editor, cx| {
                        let line_height = editor
                            .style()
                            .map(|style| {
                                f32::from(style.text.line_height_in_pixels(window.rem_size()))
                            })
                            .unwrap_or(fallback_line_height);

                        let scroll_rows = editor.scroll_position(cx).y;
                        let scroll_pixels = (scroll_rows as f32) * line_height;
                        let bounds = editor.last_bounds().cloned();

                        (line_height, scroll_pixels, bounds)
                    });

                let right_top_origin = right_bounds
                    .as_ref()
                    .map(|b| f32::from(b.origin.y))
                    .unwrap_or(f32::from(bounds.origin.y));

                RightCrushedCanvasData {
                    curves,
                    line_height: right_line_height,
                    right_scroll_pixels,
                    right_top_origin,
                    deleted_color: deleted_bg,
                }
            },
            move |bounds, data, window, _cx| {
                if data.curves.is_empty() {
                    return;
                }

                let crushed_thickness = 2.0;
                let minimal_block_height = 2.0;
                let mut inserted_lines_above = 0usize;

                for curve in &data.curves {
                    let left_len = curve.left_end.saturating_sub(curve.left_start) + 1;
                    let right_len = curve.right_end.saturating_sub(curve.right_start) + 1;

                    if curve.left_crushed {
                        inserted_lines_above += right_len;
                    } else if curve.right_crushed {
                    } else if left_len < right_len {
                        inserted_lines_above += right_len - left_len;
                    }

                    if curve.right_crushed {
                        let right_offset_rows = inserted_lines_above as f32;
                        let right_row = curve.focus_line as f32 + right_offset_rows;
                        let right_y = (right_row * data.line_height) - data.right_scroll_pixels;
                        let right_bottom = right_y + minimal_block_height;

                        let right_absolute_top = data.right_top_origin + right_y;
                        let right_absolute_bottom = data.right_top_origin + right_bottom;

                        let y_center = (right_absolute_top + right_absolute_bottom) * 0.5;
                        let top = y_center - crushed_thickness / 2.0;
                        let bottom = top + crushed_thickness;

                        if bottom > data.right_top_origin {
                            let clipped_top = top.max(data.right_top_origin);
                            let clipped_bottom = bottom.max(data.right_top_origin);

                            let mut builder = PathBuilder::fill();
                            builder.move_to(point(px(f32::from(bounds.origin.x)), px(clipped_top)));
                            builder.line_to(point(
                                px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                                px(clipped_top),
                            ));
                            builder.line_to(point(
                                px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                                px(clipped_bottom),
                            ));
                            builder
                                .line_to(point(px(f32::from(bounds.origin.x)), px(clipped_bottom)));
                            builder.close();

                            if let Ok(path) = builder.build() {
                                let background: Background = data.deleted_color.into();
                                window.paint_path(path, background);
                            }
                        }
                    }
                }
            },
        )
        .size_full()
    }

    pub fn render_connectors(&self, cx: &Context<Self>) -> impl IntoElement {
        let curves = self.connector_curves.clone();
        let left_editor = self.left_editor.clone();
        let right_editor = self.right_editor.clone();
        let fallback_line_height = self.line_height;

        let (deleted_bg, created_bg, modified_bg) = get_diff_colors(cx);

        #[derive(Clone)]
        struct ConnectorCanvasData {
            curves: Vec<ConnectorCurve>,
            line_height: f32,
            left_scroll_pixels: f32,
            right_scroll_pixels: f32,
            left_top_origin: f32,
            right_top_origin: f32,
            left_bounds: Option<gpui::Bounds<Pixels>>,
            right_bounds: Option<gpui::Bounds<Pixels>>,
            created_bg: Hsla,
            deleted_bg: Hsla,
            modified_bg: Hsla,
        }

        canvas(
            move |bounds, window, cx| {
                let (left_line_height, left_scroll_pixels, left_bounds) =
                    left_editor.update(cx, |editor, cx| {
                        let line_height = editor
                            .style()
                            .map(|style| {
                                f32::from(style.text.line_height_in_pixels(window.rem_size()))
                            })
                            .unwrap_or(fallback_line_height);

                        let scroll_rows = editor.scroll_position(cx).y;
                        let scroll_pixels = (scroll_rows as f32) * line_height;
                        let bounds = editor.last_bounds().cloned();

                        (line_height, scroll_pixels, bounds)
                    });

                let (_right_line_height, right_scroll_pixels, right_bounds) =
                    right_editor.update(cx, |editor, cx| {
                        let line_height = editor
                            .style()
                            .map(|style| {
                                f32::from(style.text.line_height_in_pixels(window.rem_size()))
                            })
                            .unwrap_or(fallback_line_height);

                        let scroll_rows = editor.scroll_position(cx).y;
                        let scroll_pixels = (scroll_rows as f32) * line_height;
                        let bounds = editor.last_bounds().cloned();

                        (line_height, scroll_pixels, bounds)
                    });

                let line_height = left_line_height;
                let left_top_origin = left_bounds
                    .as_ref()
                    .map(|b| f32::from(b.origin.y))
                    .unwrap_or(f32::from(bounds.origin.y));
                let right_top_origin = right_bounds
                    .as_ref()
                    .map(|b| f32::from(b.origin.y))
                    .unwrap_or(f32::from(bounds.origin.y));

                ConnectorCanvasData {
                    curves,
                    line_height,
                    left_scroll_pixels,
                    right_scroll_pixels,
                    left_top_origin,
                    right_top_origin,
                    left_bounds,
                    right_bounds,
                    created_bg,
                    deleted_bg,
                    modified_bg,
                }
            },
            move |bounds, data, window, _cx| {
                if data.curves.is_empty() {
                    return;
                }

                let gutter_width = f32::from(bounds.size.width);

                let header_height = data.left_top_origin - f32::from(bounds.origin.y);
                let viewport_top = header_height;
                let viewport_bottom = f32::from(bounds.size.height);

                let left_offset = data.left_top_origin - f32::from(bounds.origin.y);
                let right_offset = data.right_top_origin - f32::from(bounds.origin.y);

                let minimal_block_height = 2.0;
                let mut inserted_lines_above = 0usize;
                let mut deleted_lines_above = 0usize;

                for curve in &data.curves {
                    let is_left_empty = curve.left_crushed;
                    let is_right_empty = curve.right_crushed;

                    let left_offset_rows = if is_left_empty {
                        deleted_lines_above as f32
                    } else {
                        0.0
                    };

                    let right_offset_rows = if is_right_empty {
                        inserted_lines_above as f32
                    } else {
                        0.0
                    };

                    let left_len = curve.left_end.saturating_sub(curve.left_start) + 1;
                    let right_len = curve.right_end.saturating_sub(curve.right_start) + 1;

                    if curve.left_crushed {
                        inserted_lines_above += right_len;
                    } else if curve.right_crushed {
                        deleted_lines_above += left_len;
                    } else {
                        if left_len < right_len {
                            inserted_lines_above += right_len - left_len;
                        } else if right_len < left_len {
                            deleted_lines_above += left_len - right_len;
                        }
                    }

                    let left_row = if is_left_empty {
                        curve.focus_line as f32 + left_offset_rows
                    } else {
                        curve.left_start as f32
                    };

                    let right_row = if is_right_empty {
                        curve.focus_line as f32 + right_offset_rows
                    } else {
                        curve.right_start as f32
                    };

                    let left_y = (left_row * data.line_height) - data.left_scroll_pixels;
                    let right_y = (right_row * data.line_height) - data.right_scroll_pixels;

                    let left_bottom = if is_left_empty {
                        left_y + minimal_block_height
                    } else {
                        ((curve.left_end as f32 + 1.0) * data.line_height - data.left_scroll_pixels)
                            .max(left_y + minimal_block_height)
                    };

                    let right_bottom = if is_right_empty {
                        right_y + minimal_block_height
                    } else {
                        ((curve.right_end as f32 + 1.0) * data.line_height
                            - data.right_scroll_pixels)
                            .max(right_y + minimal_block_height)
                    };

                    let left_top = left_y;
                    let right_top = right_y;

                    let left_absolute_top = data.left_top_origin + left_top;
                    let left_absolute_bottom = data.left_top_origin + left_bottom;
                    let right_absolute_top = data.right_top_origin + right_top;
                    let right_absolute_bottom = data.right_top_origin + right_bottom;

                    let adjusted_left_top = left_top + left_offset;
                    let adjusted_left_bottom = left_bottom + left_offset;
                    let adjusted_right_top = right_top + right_offset;
                    let adjusted_right_bottom = right_bottom + right_offset;

                    let connector_height = (adjusted_left_bottom - adjusted_left_top)
                        .max(adjusted_right_bottom - adjusted_right_top);
                    let base_control_offset = gutter_width * 0.25;
                    let reference_line_height = data.line_height.max(1.0);
                    let control_offset = if connector_height < reference_line_height * 2.0 {
                        base_control_offset
                            * (connector_height / (reference_line_height * 2.0)).max(0.3)
                    } else {
                        base_control_offset
                    };

                    let connector_top = adjusted_left_top.min(adjusted_right_top);
                    let connector_bottom = adjusted_left_bottom.max(adjusted_right_bottom);

                    let base_color = match curve.kind {
                        ConnectorKind::Insert => data.created_bg,
                        ConnectorKind::Delete => data.deleted_bg,
                        ConnectorKind::Modify => data.modified_bg,
                    };

                    let is_visible =
                        connector_bottom >= viewport_top && connector_top <= viewport_bottom;

                    if is_visible {
                        Self::draw_crushed_indicator(
                            window,
                            &bounds,
                            data.left_bounds.as_ref(),
                            data.right_bounds.as_ref(),
                            is_left_empty,
                            is_right_empty,
                            left_absolute_top,
                            right_absolute_top,
                            left_absolute_bottom,
                            right_absolute_bottom,
                            gutter_width,
                            base_color,
                        );
                    }

                    let thickness_multiplier = match curve.kind {
                        ConnectorKind::Modify => {
                            let line_count = ((curve.left_end - curve.left_start)
                                .max(curve.right_end - curve.right_start))
                                as u32;
                            if line_count > 5 {
                                1.3
                            } else if line_count > 1 {
                                1.15
                            } else {
                                1.0
                            }
                        }
                        _ => 1.0,
                    };

                    let _clipped_left_top = adjusted_left_top.max(header_height);
                    let _clipped_right_top = adjusted_right_top.max(header_height);

                    let has_left_visible = adjusted_left_bottom > header_height
                        && adjusted_left_top < adjusted_left_bottom;
                    let has_right_visible = adjusted_right_bottom > header_height
                        && adjusted_right_top < adjusted_right_bottom;

                    if is_visible && (has_left_visible || has_right_visible) {
                        Self::draw_connector_ribbon(
                            window,
                            &bounds,
                            adjusted_left_top,
                            adjusted_left_bottom,
                            adjusted_right_top,
                            adjusted_right_bottom,
                            control_offset,
                            base_color,
                            thickness_multiplier,
                            header_height,
                        );
                    }
                }
            },
        )
        .size_full()
    }

    fn draw_crushed_indicator(
        window: &mut Window,
        gutter_bounds: &gpui::Bounds<Pixels>,
        _left_bounds: Option<&gpui::Bounds<Pixels>>,
        _right_bounds: Option<&gpui::Bounds<Pixels>>,
        left_crushed: bool,
        right_crushed: bool,
        left_top: f32,
        right_top: f32,
        left_bottom: f32,
        right_bottom: f32,
        _gutter_width: f32,
        color: gpui::Hsla,
    ) {
        let crushed_thickness = 2.0;

        if left_crushed && right_crushed {
            let y_center = ((left_top + left_bottom) + (right_top + right_bottom)) * 0.25;
            let top = f32::from(gutter_bounds.origin.y) + y_center - crushed_thickness / 2.0;
            let bottom = top + crushed_thickness;
            let left = f32::from(gutter_bounds.origin.x);
            let right = f32::from(gutter_bounds.origin.x) + f32::from(gutter_bounds.size.width);
            let mut builder = PathBuilder::fill();
            builder.move_to(point(px(left), px(top)));
            builder.line_to(point(px(right), px(top)));
            builder.line_to(point(px(right), px(bottom)));
            builder.line_to(point(px(left), px(bottom)));
            builder.close();

            if let Ok(path) = builder.build() {
                let background: Background = color.into();
                window.paint_path(path, background);
            }
        }
    }

    fn draw_connector_ribbon(
        window: &mut Window,
        bounds: &gpui::Bounds<Pixels>,
        left_top: f32,
        left_bottom: f32,
        right_top: f32,
        right_bottom: f32,
        control_offset: f32,
        color: gpui::Hsla,
        thickness_multiplier: f32,
        header_height: f32,
    ) {
        let _base_thickness = 6.0 * thickness_multiplier;
        let segments = 48;

        let mut builder = PathBuilder::fill();

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let top_point = cubic_bezier(
                point(
                    px(f32::from(bounds.origin.x)),
                    px(f32::from(bounds.origin.y) + left_top),
                ),
                point(
                    px(f32::from(bounds.origin.x) + control_offset),
                    px(f32::from(bounds.origin.y) + left_top),
                ),
                point(
                    px(f32::from(bounds.origin.x) + f32::from(bounds.size.width) - control_offset),
                    px(f32::from(bounds.origin.y) + right_top),
                ),
                point(
                    px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                    px(f32::from(bounds.origin.y) + right_top),
                ),
                t,
            );
            if i == 0 {
                builder.move_to(top_point);
            } else {
                builder.line_to(top_point);
            }
        }

        for i in (0..=segments).rev() {
            let t = i as f32 / segments as f32;
            let bottom_point = cubic_bezier(
                point(
                    px(f32::from(bounds.origin.x)),
                    px(f32::from(bounds.origin.y) + left_bottom),
                ),
                point(
                    px(f32::from(bounds.origin.x) + control_offset),
                    px(f32::from(bounds.origin.y) + left_bottom),
                ),
                point(
                    px(f32::from(bounds.origin.x) + f32::from(bounds.size.width) - control_offset),
                    px(f32::from(bounds.origin.y) + right_bottom),
                ),
                point(
                    px(f32::from(bounds.origin.x) + f32::from(bounds.size.width)),
                    px(f32::from(bounds.origin.y) + right_bottom),
                ),
                t,
            );
            builder.line_to(bottom_point);
        }

        if let Ok(path) = builder.build() {
            let clip_top = f32::from(bounds.origin.y) + header_height;
            let clip_bounds = gpui::Bounds {
                origin: point(px(f32::from(bounds.origin.x)), px(clip_top)),
                size: size(
                    bounds.size.width,
                    px(f32::from(bounds.size.height) - header_height),
                ),
            };

            window.with_content_mask(
                Some(gpui::ContentMask {
                    bounds: clip_bounds,
                }),
                |window| {
                    let background: Background = color.into();
                    window.paint_path(path, background);
                },
            );
        }
    }

    pub fn render_left_editor_revert_buttons(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<impl IntoElement> {
        let mut buttons = Vec::new();
        let mut deleted_lines_above = 0usize;

        let rem_size = window.rem_size();
        let icon_height = IconSize::Small.rems().to_pixels(rem_size);
        let button_height = ButtonSize::Compact.rems().to_pixels(rem_size);

        let (current_line_height, current_scroll_pixels) =
            self.left_editor.update(cx, |editor, cx| {
                let line_height = editor
                    .style()
                    .map(|style| f32::from(style.text.line_height_in_pixels(window.rem_size())))
                    .unwrap_or(self.line_height);
                let scroll_rows = editor.scroll_position(cx).y;
                let scroll_pixels = (scroll_rows as f32) * line_height;
                (line_height, scroll_pixels)
            });

        for (index, curve) in self.connector_curves.iter().enumerate() {
            let left_len = curve.left_end.saturating_sub(curve.left_start) + 1;
            let right_len = curve.right_end.saturating_sub(curve.right_start) + 1;

            if curve.right_crushed {
                deleted_lines_above += left_len;
            } else if !curve.left_crushed && right_len < left_len {
                deleted_lines_above += left_len - right_len;
            }

            if !matches!(curve.kind, ConnectorKind::Modify | ConnectorKind::Delete) {
                continue;
            }

            let is_left_empty = curve.left_crushed;
            let left_offset_rows = if is_left_empty {
                deleted_lines_above as f32
            } else {
                0.0
            };

            let left_row = if is_left_empty {
                curve.focus_line as f32 + left_offset_rows
            } else {
                curve.left_start as f32
            };

            let left_y = (left_row * current_line_height) - current_scroll_pixels;

            let minimal_block_height = 2.0;
            let left_bottom = if is_left_empty {
                left_y + minimal_block_height
            } else {
                ((curve.left_end as f32 + 1.0) * current_line_height - current_scroll_pixels)
                    .max(left_y + minimal_block_height)
            };

            let block_height = left_bottom - left_y;
            let block_center_y = left_y + block_height / 2.0;

            let container_height = block_height
                .max(button_height.into())
                .max(icon_height.into());
            let container_top = block_center_y - container_height / 2.0;

            if container_top + container_height > 0.0 {
                let button = div()
                    .absolute()
                    .right(px(8.0))
                    .top(px(container_top))
                    .h(px(container_height))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        IconButton::new(("revert-btn", index), IconName::ArrowRight)
                            .icon_size(IconSize::Small)
                            .size(ButtonSize::Compact)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.handle_revert_block(index, cx);
                            })),
                    );

                buttons.push(button);
            }
        }

        buttons
    }

    pub fn handle_mouse_move(&mut self, _event: &MouseMoveEvent, _cx: &mut gpui::Context<Self>) {
        // Currently unused, but kept for future hover effects
    }
}

