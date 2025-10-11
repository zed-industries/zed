use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
use editor::{Editor, EditorEvent, EditorMode, RowHighlightOptions};
use gpui::{
    App, Background, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Hsla, IntoElement, PathBuilder, Pixels, Point as GpuiPoint, Render,
    Subscription, Window, canvas, div, point, prelude::*, px, size,
};
use language::{Buffer, Point};
use multi_buffer::Anchor;
use multi_buffer::MultiBuffer;
use std::path::PathBuf;
use std::sync::Arc;
use theme::ActiveTheme;

use crate::connector::{ConnectorCurve, ConnectorKind};
use crate::connector_builder::build_connector_curves;
use crate::imara::{ImaraBlockOperation, ImaraDiffAnalysis, compute_imara_diff_default};

const DIFF_HIGHLIGHT_ALPHA: f32 = 0.5;

fn get_diff_colors(cx: &Context<DiffViewer>) -> (Hsla, Hsla, Hsla) {
    let theme = cx.theme();
    let mut deleted_bg = theme.status().deleted_background;
    deleted_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut created_bg = theme.status().created_background;
    created_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut modified_bg = theme.status().modified_background;
    modified_bg.a = DIFF_HIGHLIGHT_ALPHA;
    (deleted_bg, created_bg, modified_bg)
}

#[derive(Clone, Copy, Debug)]
enum PendingScroll {
    LeftToRight { source_rows: f32 },
    RightToLeft { source_rows: f32 },
}

fn count_lines(content: &str) -> usize {
    if content.is_empty() {
        1
    } else {
        content.split('\n').count().max(1)
    }
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

struct DiffAdditionHighlight;
struct DiffDeletionHighlight;
struct DiffModificationHighlight;

pub struct DiffViewer {
    left_editor: Entity<Editor>,
    right_editor: Entity<Editor>,
    left_buffer: Entity<Buffer>,
    right_buffer: Entity<Buffer>,
    left_multibuffer: Entity<MultiBuffer>,
    right_multibuffer: Entity<MultiBuffer>,
    focus_handle: FocusHandle,

    diff_analysis: Option<ImaraDiffAnalysis>,
    connector_curves: Vec<ConnectorCurve>,
    line_height: f32,
    left_scroll_offset: f32,
    right_scroll_offset: f32,
    needs_scroll_reset: bool,
    is_syncing_scroll: bool,
    left_total_lines: usize,
    right_total_lines: usize,
    left_visible_lines: f32,
    right_visible_lines: f32,
    left_scroll_rows: f32,
    right_scroll_rows: f32,
    pending_scroll: Option<PendingScroll>,
    _subscriptions: Vec<Subscription>,
    left_crushed_blocks: Vec<editor::display_map::CustomBlockId>,
    right_crushed_blocks: Vec<editor::display_map::CustomBlockId>,
}

impl EventEmitter<()> for DiffViewer {}

impl DiffViewer {
    fn map_left_line_to_right(&self, left_line: f32) -> f32 {
        if left_line >= self.left_total_lines as f32 {
            self.right_scroll_rows
        } else {
            left_line
        }
    }

    fn map_right_line_to_left(&self, right_line: f32) -> f32 {
        if right_line >= self.right_total_lines as f32 {
            self.left_scroll_rows
        } else {
            right_line
        }
    }

    fn request_sync_from_left(&mut self, source_rows: f32, cx: &mut Context<Self>) {
        self.pending_scroll = Some(PendingScroll::LeftToRight { source_rows });
        cx.notify();
    }

    fn request_sync_from_right(&mut self, source_rows: f32, cx: &mut Context<Self>) {
        self.pending_scroll = Some(PendingScroll::RightToLeft { source_rows });
        cx.notify();
    }

    fn left_line_to_anchor(&self, line: u32, cx: &Context<Self>) -> Anchor {
        let snapshot = self.left_multibuffer.read(cx).snapshot(cx);
        snapshot.anchor_before(Point::new(line, 0))
    }

    fn right_line_to_anchor(&self, line: u32, cx: &Context<Self>) -> Anchor {
        let snapshot = self.right_multibuffer.read(cx).snapshot(cx);
        snapshot.anchor_before(Point::new(line, 0))
    }

    fn create_crushed_block_properties(
        &self,
        anchor: Anchor,
        color: Hsla,
        _cx: &Context<Self>,
    ) -> BlockProperties<Anchor> {
        BlockProperties {
            placement: BlockPlacement::Replace(anchor..=anchor),
            height: Some(2),
            style: BlockStyle::Fixed,
            render: Arc::new(move |_| div().absolute().w_full().h(px(2.0)).bg(color).into_any()),
            priority: 0,
        }
    }

    pub fn new(
        _left_path: Option<PathBuf>,
        _right_path: Option<PathBuf>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let left_content = String::new();
        let right_content = String::new();

        let left_buffer = cx.new(|cx| Buffer::local(&left_content, cx));
        let right_buffer = cx.new(|cx| Buffer::local(&right_content, cx));

        let left_multibuffer = cx.new(|cx| MultiBuffer::singleton(left_buffer.clone(), cx));
        let right_multibuffer = cx.new(|cx| MultiBuffer::singleton(right_buffer.clone(), cx));

        let left_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: false,
                },
                left_multibuffer.clone(),
                None,
                window,
                cx,
            );
            editor.set_read_only(true);
            editor.set_show_gutter(true, cx);
            editor.set_vertical_scrollbar_on_left(true, cx);
            editor
        });

        let right_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: false,
                },
                right_multibuffer.clone(),
                None,
                window,
                cx,
            );
            editor.set_read_only(true);
            editor.set_show_gutter(true, cx);
            editor.set_show_scrollbars(true, cx);
            editor
        });

        let viewport_height = 600.0;

        let line_height = left_editor
            .read(cx)
            .style()
            .map(|style| f32::from(style.text.line_height_in_pixels(window.rem_size())))
            .unwrap_or(22.0);

        let default_visible_lines = viewport_height / line_height;

        let viewer = Self {
            left_editor,
            right_editor,
            left_buffer,
            right_buffer,
            left_multibuffer,
            right_multibuffer,
            focus_handle: cx.focus_handle(),
            diff_analysis: None,
            connector_curves: Vec::new(),
            line_height,
            left_scroll_offset: 0.0,
            right_scroll_offset: 0.0,
            needs_scroll_reset: false,
            is_syncing_scroll: false,
            left_total_lines: 1,
            right_total_lines: 1,
            left_visible_lines: default_visible_lines,
            right_visible_lines: default_visible_lines,
            left_scroll_rows: 0.0,
            right_scroll_rows: 0.0,
            pending_scroll: None,
            _subscriptions: Vec::new(),
            left_crushed_blocks: Vec::new(),
            right_crushed_blocks: Vec::new(),
        };

        viewer
    }

    pub fn initialize(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let left_subscription = cx.subscribe(
            &self.left_editor,
            |this: &mut DiffViewer, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::ScrollPositionChanged {
                    autoscroll: _,
                    local: _,
                } = event
                {
                    if this.is_syncing_scroll {
                        return;
                    }

                    let rows = this
                        .left_editor
                        .update(cx, |editor, cx| editor.scroll_position(cx).y);

                    if (rows as f32 - this.left_scroll_rows).abs() > f32::EPSILON {
                        this.left_scroll_rows = rows as f32;
                        this.left_scroll_offset = (rows as f32) * this.line_height;
                        this.request_sync_from_left(rows as f32, cx);
                    }
                }
            },
        );

        let right_subscription = cx.subscribe(
            &self.right_editor,
            |this: &mut DiffViewer, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::ScrollPositionChanged {
                    autoscroll: _,
                    local: _,
                } = event
                {
                    if this.is_syncing_scroll {
                        return;
                    }

                    let rows = this
                        .right_editor
                        .update(cx, |editor, cx| editor.scroll_position(cx).y);

                    if (rows as f32 - this.right_scroll_rows).abs() > f32::EPSILON {
                        this.right_scroll_rows = rows as f32;
                        this.right_scroll_offset = (rows as f32) * this.line_height;
                        this.request_sync_from_right(rows as f32, cx);
                    }
                }
            },
        );

        self._subscriptions.push(left_subscription);
        self._subscriptions.push(right_subscription);
    }

    pub fn set_language_from_source_buffers(
        &mut self,
        left_source_buffer: Option<&Entity<Buffer>>,
        right_source_buffer: Option<&Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(left_source) = left_source_buffer {
            let language = left_source.read(cx).language().cloned();
            self.left_buffer.update(cx, |buffer, cx| {
                buffer.set_language(language, cx);
            });
        }

        if let Some(right_source) = right_source_buffer {
            let language = right_source.read(cx).language().cloned();
            self.right_buffer.update(cx, |buffer, cx| {
                buffer.set_language(language, cx);
            });
        }
    }

    pub fn update_content(
        &mut self,
        left_content: String,
        right_content: String,
        cx: &mut Context<Self>,
    ) {
        self.left_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), left_content.clone())], None, cx);
        });

        self.right_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), right_content.clone())], None, cx);
        });

        self.left_total_lines = count_lines(&left_content);
        self.right_total_lines = count_lines(&right_content);

        let analysis = compute_imara_diff_default(&left_content, &right_content);

        self.diff_analysis = Some(analysis.clone());

        self.connector_curves = build_connector_curves(&analysis);

        self.apply_diff_highlights(&analysis, cx);

        self.pending_scroll = None;
        self.needs_scroll_reset = true;
        self.left_scroll_offset = 0.0;
        self.right_scroll_offset = 0.0;
        self.left_scroll_rows = 0.0;
        self.right_scroll_rows = 0.0;

        cx.notify();
    }

    fn apply_diff_highlights(&mut self, analysis: &ImaraDiffAnalysis, cx: &mut Context<Self>) {
        let (deleted_bg, created_bg, modified_bg) = get_diff_colors(cx);

        self.left_editor.update(cx, |editor, _cx| {
            editor.clear_row_highlights::<DiffDeletionHighlight>();
            editor.clear_row_highlights::<DiffModificationHighlight>();
        });

        self.right_editor.update(cx, |editor, _cx| {
            editor.clear_row_highlights::<DiffAdditionHighlight>();
            editor.clear_row_highlights::<DiffModificationHighlight>();
        });

        for block in &analysis.blocks {
            match block.operation {
                ImaraBlockOperation::Delete => {
                    if !block.left_range.is_empty() {
                        self.left_editor.update(cx, |editor, cx| {
                            let start_row = block.left_range.start as u32;
                            let end_row = block
                                .left_range
                                .end
                                .saturating_sub(1)
                                .max(block.left_range.start)
                                as u32;

                            let buffer = editor.buffer().read(cx);
                            let snapshot = buffer.snapshot(cx);

                            let actual_end_row = end_row.min(snapshot.max_row().0);
                            let start_anchor = snapshot.anchor_before(Point::new(start_row, 0));
                            let end_anchor =
                                snapshot.anchor_before(Point::new(actual_end_row + 1, 0));

                            editor.highlight_rows::<DiffDeletionHighlight>(
                                start_anchor..end_anchor,
                                deleted_bg,
                                RowHighlightOptions {
                                    autoscroll: false,
                                    include_gutter: true,
                                },
                                cx,
                            );
                        });
                    }
                }
                ImaraBlockOperation::Insert => {
                    if !block.right_range.is_empty() {
                        self.right_editor.update(cx, |editor, cx| {
                            let start_row = block.right_range.start as u32;
                            let end_row = block
                                .right_range
                                .end
                                .saturating_sub(1)
                                .max(block.right_range.start)
                                as u32;

                            let buffer = editor.buffer().read(cx);
                            let snapshot = buffer.snapshot(cx);

                            let actual_end_row = end_row.min(snapshot.max_row().0);
                            let start_anchor = snapshot.anchor_before(Point::new(start_row, 0));
                            let end_anchor =
                                snapshot.anchor_before(Point::new(actual_end_row + 1, 0));

                            editor.highlight_rows::<DiffAdditionHighlight>(
                                start_anchor..end_anchor,
                                created_bg,
                                RowHighlightOptions {
                                    autoscroll: false,
                                    include_gutter: true,
                                },
                                cx,
                            );
                        });
                    }
                }
                ImaraBlockOperation::Modify => {
                    if !block.left_range.is_empty() {
                        self.left_editor.update(cx, |editor, cx| {
                            let start_row = block.left_range.start as u32;
                            let end_row = block
                                .left_range
                                .end
                                .saturating_sub(1)
                                .max(block.left_range.start)
                                as u32;

                            let buffer = editor.buffer().read(cx);
                            let snapshot = buffer.snapshot(cx);

                            let actual_end_row = end_row.min(snapshot.max_row().0);
                            let start_anchor = snapshot.anchor_before(Point::new(start_row, 0));
                            let end_anchor =
                                snapshot.anchor_before(Point::new(actual_end_row + 1, 0));

                            editor.highlight_rows::<DiffModificationHighlight>(
                                start_anchor..end_anchor,
                                modified_bg,
                                RowHighlightOptions {
                                    autoscroll: false,
                                    include_gutter: true,
                                },
                                cx,
                            );
                        });
                    }

                    if !block.right_range.is_empty() {
                        self.right_editor.update(cx, |editor, cx| {
                            let start_row = block.right_range.start as u32;
                            let end_row = block
                                .right_range
                                .end
                                .saturating_sub(1)
                                .max(block.right_range.start)
                                as u32;

                            let buffer = editor.buffer().read(cx);
                            let snapshot = buffer.snapshot(cx);

                            let actual_end_row = end_row.min(snapshot.max_row().0);
                            let start_anchor = snapshot.anchor_before(Point::new(start_row, 0));
                            let end_anchor =
                                snapshot.anchor_before(Point::new(actual_end_row + 1, 0));

                            editor.highlight_rows::<DiffModificationHighlight>(
                                start_anchor..end_anchor,
                                modified_bg,
                                RowHighlightOptions {
                                    autoscroll: false,
                                    include_gutter: true,
                                },
                                cx,
                            );
                        });
                    }
                }
            }
        }
    }

    fn render_left_crushed_blocks(&self, cx: &Context<Self>) -> impl IntoElement {
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
                    } else {
                        if right_len < left_len {
                            deleted_lines_above += left_len - right_len;
                        }
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
                            builder.line_to(point(px(f32::from(bounds.origin.x)), px(clipped_bottom)));
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


    fn render_right_crushed_blocks(&self, cx: &Context<Self>) -> impl IntoElement {
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
                    } else {
                        if left_len < right_len {
                            inserted_lines_above += right_len - left_len;
                        }
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
                            builder.line_to(point(px(f32::from(bounds.origin.x)), px(clipped_bottom)));
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

    fn render_connectors(&self, cx: &Context<Self>) -> impl IntoElement {
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

    fn update_crushed_blocks(&mut self, cx: &mut Context<Self>) {
        let (deleted_bg, created_bg, _modified_bg) = get_diff_colors(cx);

        if !self.left_crushed_blocks.is_empty() {
            self.left_editor.update(cx, |editor, cx| {
                editor.remove_blocks(
                    self.left_crushed_blocks.clone().into_iter().collect(),
                    None,
                    cx,
                );
            });
            self.left_crushed_blocks.clear();
        }

        if !self.right_crushed_blocks.is_empty() {
            self.right_editor.update(cx, |editor, cx| {
                editor.remove_blocks(
                    self.right_crushed_blocks.clone().into_iter().collect(),
                    None,
                    cx,
                );
            });
            self.right_crushed_blocks.clear();
        }

        let mut left_crushed_positions = Vec::new();
        let mut right_crushed_positions = Vec::new();

        for curve in &self.connector_curves {
            if curve.left_crushed {
                left_crushed_positions.push(curve.focus_line);
            }
            if curve.right_crushed {
                right_crushed_positions.push(curve.focus_line);
            }
        }

        for line in left_crushed_positions {
            let anchor = self.left_line_to_anchor(line as u32, cx);
            let block_props = self.create_crushed_block_properties(anchor, created_bg, cx);
            let block_ids = self.left_editor.update(cx, |editor, cx| {
                editor.insert_blocks([block_props], None, cx)
            });
            self.left_crushed_blocks.extend(block_ids);
        }

        for line in right_crushed_positions {
            let anchor = self.right_line_to_anchor(line as u32, cx);
            let block_props = self.create_crushed_block_properties(anchor, deleted_bg, cx);
            let block_ids = self.right_editor.update(cx, |editor, cx| {
                editor.insert_blocks([block_props], None, cx)
            });
            self.right_crushed_blocks.extend(block_ids);
        }
    }
}

impl Focusable for DiffViewer {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DiffViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(visible) = self.left_editor.read(cx).visible_line_count() {
            self.left_visible_lines = visible as f32;
        }

        if let Some(visible) = self.right_editor.read(cx).visible_line_count() {
            self.right_visible_lines = visible as f32;
        }

        if self.needs_scroll_reset {
            self.needs_scroll_reset = false;
            self.is_syncing_scroll = true;

            self.left_editor.update(cx, |editor, cx| {
                editor.set_scroll_position(gpui::Point::new(0.0, 0.0), window, cx);
                editor.change_selections(editor::SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([0..0]);
                });
            });

            self.right_editor.update(cx, |editor, cx| {
                editor.set_scroll_position(gpui::Point::new(0.0, 0.0), window, cx);
                editor.change_selections(editor::SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([0..0]);
                });
            });

            self.is_syncing_scroll = false;
            self.left_scroll_offset = 0.0;
            self.right_scroll_offset = 0.0;
            self.left_scroll_rows = 0.0;
            self.right_scroll_rows = 0.0;
            self.pending_scroll = None;
        }

        if let Some(pending) = self.pending_scroll.take() {
            match pending {
                PendingScroll::LeftToRight { source_rows } => {
                    let target_rows = self.map_left_line_to_right(source_rows);

                    if target_rows >= 0.0
                        && target_rows < self.right_total_lines as f32
                        && (target_rows - self.right_scroll_rows).abs() > f32::EPSILON
                    {
                        self.is_syncing_scroll = true;
                        self.right_scroll_rows = target_rows;
                        self.right_scroll_offset = target_rows * self.line_height;
                        self.right_editor.update(cx, |editor, cx| {
                            editor.set_scroll_position(
                                gpui::Point::new(0.0, target_rows as f64),
                                window,
                                cx,
                            );
                        });
                        self.is_syncing_scroll = false;
                    }
                }
                PendingScroll::RightToLeft { source_rows } => {
                    let target_rows = self.map_right_line_to_left(source_rows);

                    if target_rows >= 0.0
                        && target_rows < self.left_total_lines as f32
                        && (target_rows - self.left_scroll_rows).abs() > f32::EPSILON
                    {
                        self.is_syncing_scroll = true;
                        self.left_scroll_rows = target_rows;
                        self.left_scroll_offset = target_rows * self.line_height;
                        self.left_editor.update(cx, |editor, cx| {
                            editor.set_scroll_position(
                                gpui::Point::new(0.0, target_rows as f64),
                                window,
                                cx,
                            );
                        });
                        self.is_syncing_scroll = false;
                    }
                }
            }
        }

        self.update_crushed_blocks(cx);

        div()
            .flex()
            .size_full()
            .bg(cx.theme().colors().background)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .h_8()
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .text_sm()
                                    .text_color(cx.theme().colors().text)
                                    .bg(cx.theme().colors().surface_background)
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .child("Original (HEAD)"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .bg(cx.theme().colors().editor_background)
                                    .child(
                                        div()
                                            .flex_1()
                                            .relative()
                                            .child(self.left_editor.clone())
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top_0()
                                                    .left_0()
                                            .right_0()
                                            .bottom_0()
                                                    .child(self.render_left_crushed_blocks(cx)),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w(px(45.))
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .h_8()
                                    .bg(cx.theme().colors().surface_background)
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .bg(cx.theme().colors().surface_background)
                                    .child(self.render_connectors(cx)),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .h_8()
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .text_sm()
                                    .text_color(cx.theme().colors().text)
                                    .bg(cx.theme().colors().surface_background)
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .child("Modified (Working)"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .bg(cx.theme().colors().editor_background)
                                    .relative()
                                    .child(self.right_editor.clone())
                                    .child(
                                        div()
                                            .absolute()
                                            .top_0()
                                            .left_0()
                                            .right_0()
                                            .bottom_0()
                                            .child(self.render_right_crushed_blocks(cx)),
                                    ),
                            ),
                    ),
            )
    }
}
