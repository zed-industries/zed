use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
use gpui::{
    Background, Context, Hsla, IntoElement, PathBuilder,
    canvas, div, point, prelude::*, px,
};
use multi_buffer::Anchor;
use std::sync::Arc;

use crate::connector::ConnectorCurve;
use crate::constants::{CRUSHED_BLOCK_HEIGHT, CRUSHED_THICKNESS};
use crate::viewer::DiffViewer;

use super::colors::get_diff_colors;

impl DiffViewer {
    pub fn create_crushed_block_properties(
        &self,
        anchor: Anchor,
        color: Hsla,
    ) -> BlockProperties<Anchor> {
        BlockProperties {
            placement: BlockPlacement::Replace(anchor..=anchor),
            height: Some(CRUSHED_BLOCK_HEIGHT as u32),
            style: BlockStyle::Fixed,
            render: Arc::new(move |_| div().absolute().w_full().h(px(CRUSHED_BLOCK_HEIGHT)).bg(color).into_any()),
            priority: 0,
        }
    }

    pub fn render_left_crushed_blocks(&self, cx: &Context<Self>) -> impl IntoElement {
        // Clone necessary for canvas closures which must own their data across frames
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
                let crushed_thickness = CRUSHED_THICKNESS;
                let minimal_block_height = CRUSHED_BLOCK_HEIGHT;
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
        // Clone necessary for canvas closures which must own their data across frames
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

                let crushed_thickness = CRUSHED_THICKNESS;
                let minimal_block_height = CRUSHED_BLOCK_HEIGHT;
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
}

