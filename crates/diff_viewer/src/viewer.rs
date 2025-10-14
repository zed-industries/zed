use editor::{Editor, EditorEvent, EditorMode};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render, Subscription,
    Window, div, prelude::*, px,
};
use language::{Buffer, Point};
use multi_buffer::Anchor;
use multi_buffer::MultiBuffer;
use std::path::PathBuf;

use crate::connector::ConnectorCurve;
use crate::connector_builder::{DiffBlock, build_connector_curves};

#[derive(Clone, Copy, Debug)]
pub enum PendingScroll {
    LeftToRight { source_rows: f32 },
    RightToLeft { source_rows: f32 },
}

pub struct DiffViewer {
    pub left_editor: Entity<Editor>,
    pub right_editor: Entity<Editor>,
    pub left_buffer: Entity<Buffer>,
    pub right_buffer: Entity<Buffer>,
    pub left_multibuffer: Entity<MultiBuffer>,
    pub right_multibuffer: Entity<MultiBuffer>,
    pub focus_handle: FocusHandle,

    pub diff_blocks: Vec<DiffBlock>,
    pub connector_curves: Vec<ConnectorCurve>,
    pub line_height: f32,
    pub left_scroll_offset: f32,
    pub right_scroll_offset: f32,
    pub needs_scroll_reset: bool,
    pub is_syncing_scroll: bool,
    pub left_total_lines: usize,
    pub right_total_lines: usize,
    pub left_visible_lines: f32,
    pub right_visible_lines: f32,
    pub left_scroll_rows: f32,
    pub right_scroll_rows: f32,
    pub pending_scroll: Option<PendingScroll>,
    pub _subscriptions: Vec<Subscription>,
    pub left_crushed_blocks: Vec<editor::display_map::CustomBlockId>,
    pub right_crushed_blocks: Vec<editor::display_map::CustomBlockId>,
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

    pub fn left_line_to_anchor(&self, line: u32, cx: &Context<Self>) -> Anchor {
        let snapshot = self.left_multibuffer.read(cx).snapshot(cx);
        snapshot.anchor_before(Point::new(line, 0))
    }

    pub fn right_line_to_anchor(&self, line: u32, cx: &Context<Self>) -> Anchor {
        let snapshot = self.right_multibuffer.read(cx).snapshot(cx);
        snapshot.anchor_before(Point::new(line, 0))
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
            editor.set_read_only(false);
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

        Self {
            left_editor,
            right_editor,
            left_buffer,
            right_buffer,
            left_multibuffer,
            right_multibuffer,
            focus_handle: cx.focus_handle(),
            diff_blocks: Vec::new(),
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
        }
    }

    pub fn initialize(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let left_subscription = cx.subscribe(
            &self.left_editor,
            |this: &mut DiffViewer, _editor, event: &EditorEvent, cx| match event {
                EditorEvent::ScrollPositionChanged {
                    autoscroll: _,
                    local: _,
                } => {
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
                EditorEvent::BufferEdited | EditorEvent::Edited { .. } => {
                    this.refresh_diff_on_content_change(cx);
                }
                _ => {}
            },
        );

        let right_subscription = cx.subscribe(
            &self.right_editor,
            |this: &mut DiffViewer, _editor, event: &EditorEvent, cx| match event {
                EditorEvent::ScrollPositionChanged {
                    autoscroll: _,
                    local: _,
                } => {
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
                EditorEvent::BufferEdited | EditorEvent::Edited { .. } => {
                    this.refresh_diff_on_content_change(cx);
                }
                _ => {}
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

        use crate::diff_operations::count_lines;
        self.left_total_lines = count_lines(&left_content);
        self.right_total_lines = count_lines(&right_content);

        self.diff_blocks = self.extract_diff_blocks(cx);
        self.connector_curves = build_connector_curves(&self.diff_blocks);
        self.apply_diff_highlights(cx);

        self.pending_scroll = None;
        self.needs_scroll_reset = true;
        self.left_scroll_offset = 0.0;
        self.right_scroll_offset = 0.0;
        self.left_scroll_rows = 0.0;
        self.right_scroll_rows = 0.0;

        cx.notify();
    }

    pub fn set_right_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        left_content: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.left_buffer.update(cx, |buf, cx| {
            buf.edit([(0..buf.len(), left_content.clone())], None, cx);
        });

        self.right_buffer = buffer.clone();
        self.right_multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        self.right_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: false,
                },
                self.right_multibuffer.clone(),
                None,
                window,
                cx,
            );
            editor.set_read_only(false);
            editor.set_show_gutter(true, cx);
            editor.set_show_scrollbars(true, cx);
            editor
        });

        let right_subscription = cx.subscribe(
            &self.right_editor,
            |this: &mut DiffViewer, _editor, event: &EditorEvent, cx| match event {
                EditorEvent::ScrollPositionChanged {
                    autoscroll: _,
                    local: _,
                } => {
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
                EditorEvent::BufferEdited | EditorEvent::Edited { .. } => {
                    this.refresh_diff_on_content_change(cx);
                }
                _ => {}
            },
        );

        self._subscriptions.truncate(1);
        self._subscriptions.push(right_subscription);

        let right_content = buffer.read(cx).text();
        use crate::diff_operations::count_lines;
        self.left_total_lines = count_lines(&left_content);
        self.right_total_lines = count_lines(&right_content);

        self.diff_blocks = self.extract_diff_blocks(cx);
        self.connector_curves = build_connector_curves(&self.diff_blocks);
        self.apply_diff_highlights(cx);

        self.pending_scroll = None;
        self.needs_scroll_reset = true;
        self.left_scroll_offset = 0.0;
        self.right_scroll_offset = 0.0;
        self.left_scroll_rows = 0.0;
        self.right_scroll_rows = 0.0;

        cx.notify();
    }
}

impl Focusable for DiffViewer {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DiffViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use theme::ActiveTheme;

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
                                                    .on_mouse_move(cx.listener(
                                                        |this, event, _, cx| {
                                                            this.handle_mouse_move(event, cx);
                                                        },
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top_0()
                                                    .left_0()
                                                    .right_0()
                                                    .bottom_0()
                                                    .child(self.render_left_crushed_blocks(cx)),
                                            )
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top_0()
                                                    .left_0()
                                                    .right_0()
                                                    .bottom_0()
                                                    .children(
                                                        self.render_left_editor_revert_buttons(
                                                            window, cx,
                                                        ),
                                                    ),
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
                                    .relative()
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
