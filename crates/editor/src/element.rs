use super::{
    display_map::{BlockContext, ToDisplayPoint},
    Anchor, DisplayPoint, Editor, EditorMode, EditorSnapshot, Input, Scroll, Select, SelectPhase,
    SoftWrap, ToPoint, MAX_LINE_LEN,
};
use crate::{display_map::TransformBlock, EditorStyle};
use clock::ReplicaId;
use collections::{BTreeMap, HashMap};
use gpui::{
    color::Color,
    elements::*,
    fonts::{HighlightStyle, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
        PathBuilder,
    },
    json::{self, ToJson},
    platform::CursorStyle,
    text_layout::{self, Line, RunStyle, TextLayoutCache},
    AppContext, Axis, Border, Element, ElementBox, Event, EventContext, LayoutContext,
    MutableAppContext, PaintContext, Quad, Scene, SizeConstraint, ViewContext, WeakViewHandle,
};
use json::json;
use language::{Bias, DiagnosticSeverity};
use settings::Settings;
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    iter,
    ops::Range,
};

pub struct EditorElement {
    view: WeakViewHandle<Editor>,
    style: EditorStyle,
    cursor_shape: CursorShape,
}

impl EditorElement {
    pub fn new(
        view: WeakViewHandle<Editor>,
        style: EditorStyle,
        cursor_shape: CursorShape,
    ) -> Self {
        Self {
            view,
            style,
            cursor_shape,
        }
    }

    fn view<'a>(&self, cx: &'a AppContext) -> &'a Editor {
        self.view.upgrade(cx).unwrap().read(cx)
    }

    fn update_view<F, T>(&self, cx: &mut MutableAppContext, f: F) -> T
    where
        F: FnOnce(&mut Editor, &mut ViewContext<Editor>) -> T,
    {
        self.view.upgrade(cx).unwrap().update(cx, f)
    }

    fn snapshot(&self, cx: &mut MutableAppContext) -> EditorSnapshot {
        self.update_view(cx, |view, cx| view.snapshot(cx))
    }

    fn mouse_down(
        &self,
        position: Vector2F,
        alt: bool,
        shift: bool,
        mut click_count: usize,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if paint.gutter_bounds.contains_point(position) {
            click_count = 3; // Simulate triple-click when clicking the gutter to select lines
        } else if !paint.text_bounds.contains_point(position) {
            return false;
        }

        let snapshot = self.snapshot(cx.app);
        let (position, overshoot) = paint.point_for_position(&snapshot, layout, position);

        if shift && alt {
            cx.dispatch_action(Select(SelectPhase::BeginColumnar {
                position,
                overshoot,
            }));
        } else if shift {
            cx.dispatch_action(Select(SelectPhase::Extend {
                position,
                click_count,
            }));
        } else {
            cx.dispatch_action(Select(SelectPhase::Begin {
                position,
                add: alt,
                click_count,
            }));
        }

        true
    }

    fn mouse_up(&self, _position: Vector2F, cx: &mut EventContext) -> bool {
        if self.view(cx.app.as_ref()).is_selecting() {
            cx.dispatch_action(Select(SelectPhase::End));
            true
        } else {
            false
        }
    }

    fn mouse_dragged(
        &self,
        position: Vector2F,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        cx: &mut EventContext,
    ) -> bool {
        let view = self.view(cx.app.as_ref());

        if view.is_selecting() {
            let rect = paint.text_bounds;
            let mut scroll_delta = Vector2F::zero();

            let vertical_margin = layout.line_height.min(rect.height() / 3.0);
            let top = rect.origin_y() + vertical_margin;
            let bottom = rect.lower_left().y() - vertical_margin;
            if position.y() < top {
                scroll_delta.set_y(-scale_vertical_mouse_autoscroll_delta(top - position.y()))
            }
            if position.y() > bottom {
                scroll_delta.set_y(scale_vertical_mouse_autoscroll_delta(position.y() - bottom))
            }

            let horizontal_margin = layout.line_height.min(rect.width() / 3.0);
            let left = rect.origin_x() + horizontal_margin;
            let right = rect.upper_right().x() - horizontal_margin;
            if position.x() < left {
                scroll_delta.set_x(-scale_horizontal_mouse_autoscroll_delta(
                    left - position.x(),
                ))
            }
            if position.x() > right {
                scroll_delta.set_x(scale_horizontal_mouse_autoscroll_delta(
                    position.x() - right,
                ))
            }

            let snapshot = self.snapshot(cx.app);
            let (position, overshoot) = paint.point_for_position(&snapshot, layout, position);

            cx.dispatch_action(Select(SelectPhase::Update {
                position,
                overshoot,
                scroll_position: (snapshot.scroll_position() + scroll_delta)
                    .clamp(Vector2F::zero(), layout.scroll_max),
            }));
            true
        } else {
            false
        }
    }

    fn key_down(&self, input: Option<&str>, cx: &mut EventContext) -> bool {
        let view = self.view.upgrade(cx.app).unwrap();

        if view.is_focused(cx.app) {
            if let Some(input) = input {
                cx.dispatch_action(Input(input.to_string()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn scroll(
        &self,
        position: Vector2F,
        mut delta: Vector2F,
        precise: bool,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if !paint.bounds.contains_point(position) {
            return false;
        }

        let snapshot = self.snapshot(cx.app);
        let max_glyph_width = layout.em_width;
        if !precise {
            delta *= vec2f(max_glyph_width, layout.line_height);
        }

        let scroll_position = snapshot.scroll_position();
        let x = (scroll_position.x() * max_glyph_width - delta.x()) / max_glyph_width;
        let y = (scroll_position.y() * layout.line_height - delta.y()) / layout.line_height;
        let scroll_position = vec2f(x, y).clamp(Vector2F::zero(), layout.scroll_max);

        cx.dispatch_action(Scroll(scroll_position));

        true
    }

    fn paint_background(
        &self,
        gutter_bounds: RectF,
        text_bounds: RectF,
        layout: &LayoutState,
        cx: &mut PaintContext,
    ) {
        let bounds = gutter_bounds.union_rect(text_bounds);
        let scroll_top = layout.snapshot.scroll_position().y() * layout.line_height;
        let editor = self.view(cx.app);
        cx.scene.push_quad(Quad {
            bounds: gutter_bounds,
            background: Some(self.style.gutter_background),
            border: Border::new(0., Color::transparent_black()),
            corner_radius: 0.,
        });
        cx.scene.push_quad(Quad {
            bounds: text_bounds,
            background: Some(self.style.background),
            border: Border::new(0., Color::transparent_black()),
            corner_radius: 0.,
        });

        if let EditorMode::Full = editor.mode {
            let mut active_rows = layout.active_rows.iter().peekable();
            while let Some((start_row, contains_non_empty_selection)) = active_rows.next() {
                let mut end_row = *start_row;
                while active_rows.peek().map_or(false, |r| {
                    *r.0 == end_row + 1 && r.1 == contains_non_empty_selection
                }) {
                    active_rows.next().unwrap();
                    end_row += 1;
                }

                if !contains_non_empty_selection {
                    let origin = vec2f(
                        bounds.origin_x(),
                        bounds.origin_y() + (layout.line_height * *start_row as f32) - scroll_top,
                    );
                    let size = vec2f(
                        bounds.width(),
                        layout.line_height * (end_row - start_row + 1) as f32,
                    );
                    cx.scene.push_quad(Quad {
                        bounds: RectF::new(origin, size),
                        background: Some(self.style.active_line_background),
                        border: Border::default(),
                        corner_radius: 0.,
                    });
                }
            }

            if let Some(highlighted_rows) = &layout.highlighted_rows {
                let origin = vec2f(
                    bounds.origin_x(),
                    bounds.origin_y() + (layout.line_height * highlighted_rows.start as f32)
                        - scroll_top,
                );
                let size = vec2f(
                    bounds.width(),
                    layout.line_height * highlighted_rows.len() as f32,
                );
                cx.scene.push_quad(Quad {
                    bounds: RectF::new(origin, size),
                    background: Some(self.style.highlighted_line_background),
                    border: Border::default(),
                    corner_radius: 0.,
                });
            }
        }
    }

    fn paint_gutter(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        cx: &mut PaintContext,
    ) {
        let scroll_top = layout.snapshot.scroll_position().y() * layout.line_height;
        for (ix, line) in layout.line_number_layouts.iter().enumerate() {
            if let Some(line) = line {
                let line_origin = bounds.origin()
                    + vec2f(
                        bounds.width() - line.width() - layout.gutter_padding,
                        ix as f32 * layout.line_height - (scroll_top % layout.line_height),
                    );
                line.paint(line_origin, visible_bounds, layout.line_height, cx);
            }
        }

        if let Some((row, indicator)) = layout.code_actions_indicator.as_mut() {
            let mut x = bounds.width() - layout.gutter_padding;
            let mut y = *row as f32 * layout.line_height - scroll_top;
            x += ((layout.gutter_padding + layout.gutter_margin) - indicator.size().x()) / 2.;
            y += (layout.line_height - indicator.size().y()) / 2.;
            indicator.paint(bounds.origin() + vec2f(x, y), visible_bounds, cx);
        }
    }

    fn paint_text(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        cx: &mut PaintContext,
    ) {
        let view = self.view(cx.app);
        let style = &self.style;
        let local_replica_id = view.replica_id(cx);
        let scroll_position = layout.snapshot.scroll_position();
        let start_row = scroll_position.y() as u32;
        let scroll_top = scroll_position.y() * layout.line_height;
        let end_row = ((scroll_top + bounds.height()) / layout.line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen
        let max_glyph_width = layout.em_width;
        let scroll_left = scroll_position.x() * max_glyph_width;
        let content_origin = bounds.origin() + vec2f(layout.gutter_margin, 0.);

        cx.scene.push_layer(Some(bounds));
        cx.scene.push_cursor_style(bounds, CursorStyle::IBeam);

        for (range, color) in &layout.highlighted_ranges {
            self.paint_highlighted_range(
                range.clone(),
                start_row,
                end_row,
                *color,
                0.,
                0.15 * layout.line_height,
                layout,
                content_origin,
                scroll_top,
                scroll_left,
                bounds,
                cx,
            );
        }

        let mut cursors = SmallVec::<[Cursor; 32]>::new();
        for (replica_id, selections) in &layout.selections {
            let selection_style = style.replica_selection_style(*replica_id);
            let corner_radius = 0.15 * layout.line_height;

            for selection in selections {
                self.paint_highlighted_range(
                    selection.start..selection.end,
                    start_row,
                    end_row,
                    selection_style.selection,
                    corner_radius,
                    corner_radius * 2.,
                    layout,
                    content_origin,
                    scroll_top,
                    scroll_left,
                    bounds,
                    cx,
                );

                if view.show_local_cursors() || *replica_id != local_replica_id {
                    let cursor_position = selection.head();
                    if (start_row..end_row).contains(&cursor_position.row()) {
                        let cursor_row_layout =
                            &layout.line_layouts[(cursor_position.row() - start_row) as usize];
                        let cursor_column = cursor_position.column() as usize;

                        let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);
                        let mut block_width =
                            cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
                        if block_width == 0.0 {
                            block_width = layout.em_width;
                        }

                        let block_text =
                            if matches!(self.cursor_shape, CursorShape::Block) {
                                layout.snapshot.chars_at(cursor_position).next().and_then(
                                    |character| {
                                        let font_id =
                                            cursor_row_layout.font_for_index(cursor_column)?;
                                        let text = character.to_string();

                                        Some(cx.text_layout_cache.layout_str(
                                            &text,
                                            cursor_row_layout.font_size(),
                                            &[(
                                                text.len(),
                                                RunStyle {
                                                    font_id,
                                                    color: style.background,
                                                    underline: Default::default(),
                                                },
                                            )],
                                        ))
                                    },
                                )
                            } else {
                                None
                            };

                        let x = cursor_character_x - scroll_left;
                        let y = cursor_position.row() as f32 * layout.line_height - scroll_top;
                        cursors.push(Cursor {
                            color: selection_style.cursor,
                            block_width,
                            origin: content_origin + vec2f(x, y),
                            line_height: layout.line_height,
                            shape: self.cursor_shape,
                            block_text,
                        });
                    }
                }
            }
        }

        if let Some(visible_text_bounds) = bounds.intersection(visible_bounds) {
            // Draw glyphs
            for (ix, line) in layout.line_layouts.iter().enumerate() {
                let row = start_row + ix as u32;
                line.paint(
                    content_origin
                        + vec2f(-scroll_left, row as f32 * layout.line_height - scroll_top),
                    visible_text_bounds,
                    layout.line_height,
                    cx,
                );
            }
        }

        cx.scene.push_layer(Some(bounds));
        for cursor in cursors {
            cursor.paint(cx);
        }
        cx.scene.pop_layer();

        if let Some((position, context_menu)) = layout.context_menu.as_mut() {
            cx.scene.push_stacking_context(None);

            let cursor_row_layout = &layout.line_layouts[(position.row() - start_row) as usize];
            let x = cursor_row_layout.x_for_index(position.column() as usize) - scroll_left;
            let y = (position.row() + 1) as f32 * layout.line_height - scroll_top;
            let mut list_origin = content_origin + vec2f(x, y);
            let list_height = context_menu.size().y();

            if list_origin.y() + list_height > bounds.lower_left().y() {
                list_origin.set_y(list_origin.y() - layout.line_height - list_height);
            }

            context_menu.paint(
                list_origin,
                RectF::from_points(Vector2F::zero(), vec2f(f32::MAX, f32::MAX)), // Let content bleed outside of editor
                cx,
            );

            cx.scene.pop_stacking_context();
        }

        cx.scene.pop_layer();
    }

    fn paint_highlighted_range(
        &self,
        range: Range<DisplayPoint>,
        start_row: u32,
        end_row: u32,
        color: Color,
        corner_radius: f32,
        line_end_overshoot: f32,
        layout: &LayoutState,
        content_origin: Vector2F,
        scroll_top: f32,
        scroll_left: f32,
        bounds: RectF,
        cx: &mut PaintContext,
    ) {
        if range.start != range.end {
            let row_range = if range.end.column() == 0 {
                cmp::max(range.start.row(), start_row)..cmp::min(range.end.row(), end_row)
            } else {
                cmp::max(range.start.row(), start_row)..cmp::min(range.end.row() + 1, end_row)
            };

            let highlighted_range = HighlightedRange {
                color,
                line_height: layout.line_height,
                corner_radius,
                start_y: content_origin.y() + row_range.start as f32 * layout.line_height
                    - scroll_top,
                lines: row_range
                    .into_iter()
                    .map(|row| {
                        let line_layout = &layout.line_layouts[(row - start_row) as usize];
                        HighlightedRangeLine {
                            start_x: if row == range.start.row() {
                                content_origin.x()
                                    + line_layout.x_for_index(range.start.column() as usize)
                                    - scroll_left
                            } else {
                                content_origin.x() - scroll_left
                            },
                            end_x: if row == range.end.row() {
                                content_origin.x()
                                    + line_layout.x_for_index(range.end.column() as usize)
                                    - scroll_left
                            } else {
                                content_origin.x() + line_layout.width() + line_end_overshoot
                                    - scroll_left
                            },
                        }
                    })
                    .collect(),
            };

            highlighted_range.paint(bounds, cx.scene);
        }
    }

    fn paint_blocks(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut LayoutState,
        cx: &mut PaintContext,
    ) {
        let scroll_position = layout.snapshot.scroll_position();
        let scroll_left = scroll_position.x() * layout.em_width;
        let scroll_top = scroll_position.y() * layout.line_height;

        for (row, element) in &mut layout.blocks {
            let origin = bounds.origin()
                + vec2f(-scroll_left, *row as f32 * layout.line_height - scroll_top);
            element.paint(origin, visible_bounds, cx);
        }
    }

    fn max_line_number_width(&self, snapshot: &EditorSnapshot, cx: &LayoutContext) -> f32 {
        let digit_count = (snapshot.max_buffer_row() as f32).log10().floor() as usize + 1;
        let style = &self.style;

        cx.text_layout_cache
            .layout_str(
                "1".repeat(digit_count).as_str(),
                style.text.font_size,
                &[(
                    digit_count,
                    RunStyle {
                        font_id: style.text.font_id,
                        color: Color::black(),
                        underline: Default::default(),
                    },
                )],
            )
            .width()
    }

    fn layout_line_numbers(
        &self,
        rows: Range<u32>,
        active_rows: &BTreeMap<u32, bool>,
        snapshot: &EditorSnapshot,
        cx: &LayoutContext,
    ) -> Vec<Option<text_layout::Line>> {
        let style = &self.style;
        let include_line_numbers = snapshot.mode == EditorMode::Full;
        let mut line_number_layouts = Vec::with_capacity(rows.len());
        let mut line_number = String::new();
        for (ix, row) in snapshot
            .buffer_rows(rows.start)
            .take((rows.end - rows.start) as usize)
            .enumerate()
        {
            let display_row = rows.start + ix as u32;
            let color = if active_rows.contains_key(&display_row) {
                style.line_number_active
            } else {
                style.line_number
            };
            if let Some(buffer_row) = row {
                if include_line_numbers {
                    line_number.clear();
                    write!(&mut line_number, "{}", buffer_row + 1).unwrap();
                    line_number_layouts.push(Some(cx.text_layout_cache.layout_str(
                        &line_number,
                        style.text.font_size,
                        &[(
                            line_number.len(),
                            RunStyle {
                                font_id: style.text.font_id,
                                color,
                                underline: Default::default(),
                            },
                        )],
                    )));
                }
            } else {
                line_number_layouts.push(None);
            }
        }

        line_number_layouts
    }

    fn layout_lines(
        &mut self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        cx: &LayoutContext,
    ) -> Vec<text_layout::Line> {
        if rows.start >= rows.end {
            return Vec::new();
        }

        // When the editor is empty and unfocused, then show the placeholder.
        if snapshot.is_empty() && !snapshot.is_focused() {
            let placeholder_style = self
                .style
                .placeholder_text
                .as_ref()
                .unwrap_or_else(|| &self.style.text);
            let placeholder_text = snapshot.placeholder_text();
            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start as usize)
                .chain(iter::repeat(""))
                .take(rows.len());
            return placeholder_lines
                .map(|line| {
                    cx.text_layout_cache.layout_str(
                        line,
                        placeholder_style.font_size,
                        &[(
                            line.len(),
                            RunStyle {
                                font_id: placeholder_style.font_id,
                                color: placeholder_style.color,
                                underline: Default::default(),
                            },
                        )],
                    )
                })
                .collect();
        } else {
            let style = &self.style;
            let chunks = snapshot.chunks(rows.clone(), true).map(|chunk| {
                let mut highlight_style = chunk
                    .syntax_highlight_id
                    .and_then(|id| id.style(&style.syntax));

                if let Some(chunk_highlight) = chunk.highlight_style {
                    if let Some(highlight_style) = highlight_style.as_mut() {
                        highlight_style.highlight(chunk_highlight);
                    } else {
                        highlight_style = Some(chunk_highlight);
                    }
                }

                let mut diagnostic_highlight = HighlightStyle::default();

                if chunk.is_unnecessary {
                    diagnostic_highlight.fade_out = Some(style.unnecessary_code_fade);
                }

                if let Some(severity) = chunk.diagnostic_severity {
                    // Omit underlines for HINT/INFO diagnostics on 'unnecessary' code.
                    if severity <= DiagnosticSeverity::WARNING || !chunk.is_unnecessary {
                        let diagnostic_style = super::diagnostic_style(severity, true, style);
                        diagnostic_highlight.underline = Some(Underline {
                            color: Some(diagnostic_style.message.text.color),
                            thickness: 1.0.into(),
                            squiggly: true,
                        });
                    }
                }

                if let Some(highlight_style) = highlight_style.as_mut() {
                    highlight_style.highlight(diagnostic_highlight);
                } else {
                    highlight_style = Some(diagnostic_highlight);
                }

                (chunk.text, highlight_style)
            });
            layout_highlighted_chunks(
                chunks,
                &style.text,
                &cx.text_layout_cache,
                &cx.font_cache,
                MAX_LINE_LEN,
                rows.len() as usize,
            )
        }
    }

    fn layout_blocks(
        &mut self,
        rows: Range<u32>,
        snapshot: &EditorSnapshot,
        width: f32,
        gutter_padding: f32,
        gutter_width: f32,
        em_width: f32,
        text_x: f32,
        line_height: f32,
        style: &EditorStyle,
        line_layouts: &[text_layout::Line],
        cx: &mut LayoutContext,
    ) -> Vec<(u32, ElementBox)> {
        let scroll_x = snapshot.scroll_position.x();
        snapshot
            .blocks_in_range(rows.clone())
            .map(|(block_row, block)| {
                let mut element = match block {
                    TransformBlock::Custom(block) => {
                        let align_to = block
                            .position()
                            .to_point(&snapshot.buffer_snapshot)
                            .to_display_point(snapshot);
                        let anchor_x = text_x
                            + if rows.contains(&align_to.row()) {
                                line_layouts[(align_to.row() - rows.start) as usize]
                                    .x_for_index(align_to.column() as usize)
                            } else {
                                layout_line(align_to.row(), snapshot, style, cx.text_layout_cache)
                                    .x_for_index(align_to.column() as usize)
                            };

                        block.render(&BlockContext {
                            cx,
                            anchor_x,
                            gutter_padding,
                            line_height,
                            scroll_x,
                            gutter_width,
                            em_width,
                        })
                    }
                    TransformBlock::ExcerptHeader {
                        buffer,
                        starts_new_buffer,
                        ..
                    } => {
                        if *starts_new_buffer {
                            let style = &self.style.diagnostic_path_header;
                            let font_size =
                                (style.text_scale_factor * self.style.text.font_size).round();

                            let mut filename = None;
                            let mut parent_path = None;
                            if let Some(path) = buffer.path() {
                                filename =
                                    path.file_name().map(|f| f.to_string_lossy().to_string());
                                parent_path =
                                    path.parent().map(|p| p.to_string_lossy().to_string() + "/");
                            }

                            Flex::row()
                                .with_child(
                                    Label::new(
                                        filename.unwrap_or_else(|| "untitled".to_string()),
                                        style.filename.text.clone().with_font_size(font_size),
                                    )
                                    .contained()
                                    .with_style(style.filename.container)
                                    .boxed(),
                                )
                                .with_children(parent_path.map(|path| {
                                    Label::new(
                                        path,
                                        style.path.text.clone().with_font_size(font_size),
                                    )
                                    .contained()
                                    .with_style(style.path.container)
                                    .boxed()
                                }))
                                .aligned()
                                .left()
                                .contained()
                                .with_style(style.container)
                                .with_padding_left(gutter_padding + scroll_x * em_width)
                                .expanded()
                                .named("path header block")
                        } else {
                            let text_style = self.style.text.clone();
                            Label::new("…".to_string(), text_style)
                                .contained()
                                .with_padding_left(gutter_padding + scroll_x * em_width)
                                .named("collapsed context")
                        }
                    }
                };

                element.layout(
                    SizeConstraint {
                        min: Vector2F::zero(),
                        max: vec2f(width, block.height() as f32 * line_height),
                    },
                    cx,
                );
                (block_row, element)
            })
            .collect()
    }
}

impl Element for EditorElement {
    type LayoutState = LayoutState;
    type PaintState = PaintState;

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        if size.x().is_infinite() {
            unimplemented!("we don't yet handle an infinite width constraint on buffer elements");
        }

        let snapshot = self.snapshot(cx.app);
        let style = self.style.clone();
        let line_height = style.text.line_height(cx.font_cache);

        let gutter_padding;
        let gutter_width;
        let gutter_margin;
        if snapshot.mode == EditorMode::Full {
            gutter_padding = style.text.em_width(cx.font_cache) * style.gutter_padding_factor;
            gutter_width = self.max_line_number_width(&snapshot, cx) + gutter_padding * 2.0;
            gutter_margin = -style.text.descent(cx.font_cache);
        } else {
            gutter_padding = 0.0;
            gutter_width = 0.0;
            gutter_margin = 0.0;
        };

        let text_width = size.x() - gutter_width;
        let em_width = style.text.em_width(cx.font_cache);
        let em_advance = style.text.em_advance(cx.font_cache);
        let overscroll = vec2f(em_width, 0.);
        let snapshot = self.update_view(cx.app, |view, cx| {
            let wrap_width = match view.soft_wrap_mode(cx) {
                SoftWrap::None => Some((MAX_LINE_LEN / 2) as f32 * em_advance),
                SoftWrap::EditorWidth => {
                    Some(text_width - gutter_margin - overscroll.x() - em_width)
                }
                SoftWrap::Column(column) => Some(column as f32 * em_advance),
            };

            if view.set_wrap_width(wrap_width, cx) {
                view.snapshot(cx)
            } else {
                snapshot
            }
        });

        let scroll_height = (snapshot.max_point().row() + 1) as f32 * line_height;
        if let EditorMode::AutoHeight { max_lines } = snapshot.mode {
            size.set_y(
                scroll_height
                    .min(constraint.max_along(Axis::Vertical))
                    .max(constraint.min_along(Axis::Vertical))
                    .min(line_height * max_lines as f32),
            )
        } else if let EditorMode::SingleLine = snapshot.mode {
            size.set_y(
                line_height
                    .min(constraint.max_along(Axis::Vertical))
                    .max(constraint.min_along(Axis::Vertical)),
            )
        } else if size.y().is_infinite() {
            size.set_y(scroll_height);
        }
        let gutter_size = vec2f(gutter_width, size.y());
        let text_size = vec2f(text_width, size.y());

        let (autoscroll_horizontally, mut snapshot) = self.update_view(cx.app, |view, cx| {
            let autoscroll_horizontally = view.autoscroll_vertically(size.y(), line_height, cx);
            let snapshot = view.snapshot(cx);
            (autoscroll_horizontally, snapshot)
        });

        let scroll_position = snapshot.scroll_position();
        let start_row = scroll_position.y() as u32;
        let scroll_top = scroll_position.y() * line_height;

        // Add 1 to ensure selections bleed off screen
        let end_row = 1 + cmp::min(
            ((scroll_top + size.y()) / line_height).ceil() as u32,
            snapshot.max_point().row(),
        );

        let start_anchor = if start_row == 0 {
            Anchor::min()
        } else {
            snapshot
                .buffer_snapshot
                .anchor_before(DisplayPoint::new(start_row, 0).to_offset(&snapshot, Bias::Left))
        };
        let end_anchor = if end_row > snapshot.max_point().row() {
            Anchor::max()
        } else {
            snapshot
                .buffer_snapshot
                .anchor_before(DisplayPoint::new(end_row, 0).to_offset(&snapshot, Bias::Right))
        };

        let mut selections = Vec::new();
        let mut active_rows = BTreeMap::new();
        let mut highlighted_rows = None;
        let mut highlighted_ranges = Vec::new();
        self.update_view(cx.app, |view, cx| {
            let display_map = view.display_map.update(cx, |map, cx| map.snapshot(cx));

            highlighted_rows = view.highlighted_rows();
            let theme = cx.global::<Settings>().theme.as_ref();
            highlighted_ranges = view.background_highlights_in_range(
                start_anchor.clone()..end_anchor.clone(),
                &display_map,
                theme,
            );

            let mut remote_selections = HashMap::default();
            for (replica_id, selection) in display_map
                .buffer_snapshot
                .remote_selections_in_range(&(start_anchor.clone()..end_anchor.clone()))
            {
                // The local selections match the leader's selections.
                if Some(replica_id) == view.leader_replica_id {
                    continue;
                }

                remote_selections
                    .entry(replica_id)
                    .or_insert(Vec::new())
                    .push(crate::Selection {
                        id: selection.id,
                        goal: selection.goal,
                        reversed: selection.reversed,
                        start: selection.start.to_display_point(&display_map),
                        end: selection.end.to_display_point(&display_map),
                    });
            }
            selections.extend(remote_selections);

            if view.show_local_selections {
                let local_selections = view
                    .selections
                    .interleaved_in_range(start_anchor..end_anchor, cx);
                for selection in &local_selections {
                    let is_empty = selection.start == selection.end;
                    let selection_start = snapshot.prev_line_boundary(selection.start).1;
                    let selection_end = snapshot.next_line_boundary(selection.end).1;
                    for row in cmp::max(selection_start.row(), start_row)
                        ..=cmp::min(selection_end.row(), end_row)
                    {
                        let contains_non_empty_selection =
                            active_rows.entry(row).or_insert(!is_empty);
                        *contains_non_empty_selection |= !is_empty;
                    }
                }

                // Render the local selections in the leader's color when following.
                let local_replica_id = view.leader_replica_id.unwrap_or(view.replica_id(cx));

                selections.push((
                    local_replica_id,
                    local_selections
                        .into_iter()
                        .map(|selection| crate::Selection {
                            id: selection.id,
                            goal: selection.goal,
                            reversed: selection.reversed,
                            start: selection.start.to_display_point(&display_map),
                            end: selection.end.to_display_point(&display_map),
                        })
                        .collect(),
                ));
            }
        });

        let line_number_layouts =
            self.layout_line_numbers(start_row..end_row, &active_rows, &snapshot, cx);

        let mut max_visible_line_width = 0.0;
        let line_layouts = self.layout_lines(start_row..end_row, &snapshot, cx);
        for line in &line_layouts {
            if line.width() > max_visible_line_width {
                max_visible_line_width = line.width();
            }
        }

        let style = self.style.clone();
        let longest_line_width = layout_line(
            snapshot.longest_row(),
            &snapshot,
            &style,
            cx.text_layout_cache,
        )
        .width();
        let scroll_width = longest_line_width.max(max_visible_line_width) + overscroll.x();
        let em_width = style.text.em_width(cx.font_cache);
        let max_row = snapshot.max_point().row();
        let scroll_max = vec2f(
            ((scroll_width - text_size.x()) / em_width).max(0.0),
            max_row.saturating_sub(1) as f32,
        );

        let mut context_menu = None;
        let mut code_actions_indicator = None;
        self.update_view(cx.app, |view, cx| {
            let clamped = view.clamp_scroll_left(scroll_max.x());
            let autoscrolled;
            if autoscroll_horizontally {
                autoscrolled = view.autoscroll_horizontally(
                    start_row,
                    text_size.x(),
                    scroll_width,
                    em_width,
                    &line_layouts,
                    cx,
                );
            } else {
                autoscrolled = false;
            }

            if clamped || autoscrolled {
                snapshot = view.snapshot(cx);
            }

            let newest_selection_head = view
                .selections
                .newest::<usize>(cx)
                .head()
                .to_display_point(&snapshot);

            if (start_row..end_row).contains(&newest_selection_head.row()) {
                let style = view.style(cx);
                if view.context_menu_visible() {
                    context_menu =
                        view.render_context_menu(newest_selection_head, style.clone(), cx);
                }

                code_actions_indicator = view
                    .render_code_actions_indicator(&style, cx)
                    .map(|indicator| (newest_selection_head.row(), indicator));
            }
        });

        if let Some((_, context_menu)) = context_menu.as_mut() {
            context_menu.layout(
                SizeConstraint {
                    min: Vector2F::zero(),
                    max: vec2f(
                        f32::INFINITY,
                        (12. * line_height).min((size.y() - line_height) / 2.),
                    ),
                },
                cx,
            );
        }

        if let Some((_, indicator)) = code_actions_indicator.as_mut() {
            indicator.layout(
                SizeConstraint::strict_along(Axis::Vertical, line_height * 0.618),
                cx,
            );
        }

        let blocks = self.layout_blocks(
            start_row..end_row,
            &snapshot,
            size.x().max(scroll_width + gutter_width),
            gutter_padding,
            gutter_width,
            em_width,
            gutter_width + gutter_margin,
            line_height,
            &style,
            &line_layouts,
            cx,
        );

        (
            size,
            LayoutState {
                size,
                scroll_max,
                gutter_size,
                gutter_padding,
                text_size,
                gutter_margin,
                snapshot,
                active_rows,
                highlighted_rows,
                highlighted_ranges,
                line_layouts,
                line_number_layouts,
                blocks,
                line_height,
                em_width,
                em_advance,
                selections,
                context_menu,
                code_actions_indicator,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        cx.scene.push_layer(Some(bounds));

        let gutter_bounds = RectF::new(bounds.origin(), layout.gutter_size);
        let text_bounds = RectF::new(
            bounds.origin() + vec2f(layout.gutter_size.x(), 0.0),
            layout.text_size,
        );

        self.paint_background(gutter_bounds, text_bounds, layout, cx);
        if layout.gutter_size.x() > 0. {
            self.paint_gutter(gutter_bounds, visible_bounds, layout, cx);
        }
        self.paint_text(text_bounds, visible_bounds, layout, cx);

        if !layout.blocks.is_empty() {
            cx.scene.push_layer(Some(bounds));
            self.paint_blocks(bounds, visible_bounds, layout, cx);
            cx.scene.pop_layer();
        }

        cx.scene.pop_layer();

        PaintState {
            bounds,
            gutter_bounds,
            text_bounds,
        }
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if let Some((_, context_menu)) = &mut layout.context_menu {
            if context_menu.dispatch_event(event, cx) {
                return true;
            }
        }

        if let Some((_, indicator)) = &mut layout.code_actions_indicator {
            if indicator.dispatch_event(event, cx) {
                return true;
            }
        }

        for (_, block) in &mut layout.blocks {
            if block.dispatch_event(event, cx) {
                return true;
            }
        }

        match event {
            Event::LeftMouseDown {
                position,
                alt,
                shift,
                click_count,
                ..
            } => self.mouse_down(*position, *alt, *shift, *click_count, layout, paint, cx),
            Event::LeftMouseUp { position, .. } => self.mouse_up(*position, cx),
            Event::LeftMouseDragged { position } => {
                self.mouse_dragged(*position, layout, paint, cx)
            }
            Event::ScrollWheel {
                position,
                delta,
                precise,
            } => self.scroll(*position, *delta, *precise, layout, paint, cx),
            Event::KeyDown { input, .. } => self.key_down(input.as_deref(), cx),
            _ => false,
        }
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::DebugContext,
    ) -> json::Value {
        json!({
            "type": "BufferElement",
            "bounds": bounds.to_json()
        })
    }
}

pub struct LayoutState {
    size: Vector2F,
    scroll_max: Vector2F,
    gutter_size: Vector2F,
    gutter_padding: f32,
    gutter_margin: f32,
    text_size: Vector2F,
    snapshot: EditorSnapshot,
    active_rows: BTreeMap<u32, bool>,
    highlighted_rows: Option<Range<u32>>,
    line_layouts: Vec<text_layout::Line>,
    line_number_layouts: Vec<Option<text_layout::Line>>,
    blocks: Vec<(u32, ElementBox)>,
    line_height: f32,
    em_width: f32,
    em_advance: f32,
    highlighted_ranges: Vec<(Range<DisplayPoint>, Color)>,
    selections: Vec<(ReplicaId, Vec<text::Selection<DisplayPoint>>)>,
    context_menu: Option<(DisplayPoint, ElementBox)>,
    code_actions_indicator: Option<(u32, ElementBox)>,
}

fn layout_line(
    row: u32,
    snapshot: &EditorSnapshot,
    style: &EditorStyle,
    layout_cache: &TextLayoutCache,
) -> text_layout::Line {
    let mut line = snapshot.line(row);

    if line.len() > MAX_LINE_LEN {
        let mut len = MAX_LINE_LEN;
        while !line.is_char_boundary(len) {
            len -= 1;
        }

        line.truncate(len);
    }

    layout_cache.layout_str(
        &line,
        style.text.font_size,
        &[(
            snapshot.line_len(row) as usize,
            RunStyle {
                font_id: style.text.font_id,
                color: Color::black(),
                underline: Default::default(),
            },
        )],
    )
}

pub struct PaintState {
    bounds: RectF,
    gutter_bounds: RectF,
    text_bounds: RectF,
}

impl PaintState {
    fn point_for_position(
        &self,
        snapshot: &EditorSnapshot,
        layout: &LayoutState,
        position: Vector2F,
    ) -> (DisplayPoint, u32) {
        let scroll_position = snapshot.scroll_position();
        let position = position - self.text_bounds.origin();
        let y = position.y().max(0.0).min(layout.size.y());
        let row = ((y / layout.line_height) + scroll_position.y()) as u32;
        let row = cmp::min(row, snapshot.max_point().row());
        let line = &layout.line_layouts[(row - scroll_position.y() as u32) as usize];
        let x = position.x() + (scroll_position.x() * layout.em_width);

        let column = if x >= 0.0 {
            line.index_for_x(x)
                .map(|ix| ix as u32)
                .unwrap_or_else(|| snapshot.line_len(row))
        } else {
            0
        };
        let overshoot = (0f32.max(x - line.width()) / layout.em_advance) as u32;

        (DisplayPoint::new(row, column), overshoot)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CursorShape {
    Bar,
    Block,
    Underscore,
}

impl Default for CursorShape {
    fn default() -> Self {
        CursorShape::Bar
    }
}

struct Cursor {
    origin: Vector2F,
    block_width: f32,
    line_height: f32,
    color: Color,
    shape: CursorShape,
    block_text: Option<Line>,
}

impl Cursor {
    fn paint(&self, cx: &mut PaintContext) {
        let bounds = match self.shape {
            CursorShape::Bar => RectF::new(self.origin, vec2f(2.0, self.line_height)),
            CursorShape::Block => {
                RectF::new(self.origin, vec2f(self.block_width, self.line_height))
            }
            CursorShape::Underscore => RectF::new(
                self.origin + Vector2F::new(0.0, self.line_height - 2.0),
                vec2f(self.block_width, 2.0),
            ),
        };

        cx.scene.push_quad(Quad {
            bounds,
            background: Some(self.color),
            border: Border::new(0., Color::black()),
            corner_radius: 0.,
        });

        if let Some(block_text) = &self.block_text {
            block_text.paint(self.origin, bounds, self.line_height, cx);
        }
    }
}

#[derive(Debug)]
struct HighlightedRange {
    start_y: f32,
    line_height: f32,
    lines: Vec<HighlightedRangeLine>,
    color: Color,
    corner_radius: f32,
}

#[derive(Debug)]
struct HighlightedRangeLine {
    start_x: f32,
    end_x: f32,
}

impl HighlightedRange {
    fn paint(&self, bounds: RectF, scene: &mut Scene) {
        if self.lines.len() >= 2 && self.lines[0].start_x > self.lines[1].end_x {
            self.paint_lines(self.start_y, &self.lines[0..1], bounds, scene);
            self.paint_lines(
                self.start_y + self.line_height,
                &self.lines[1..],
                bounds,
                scene,
            );
        } else {
            self.paint_lines(self.start_y, &self.lines, bounds, scene);
        }
    }

    fn paint_lines(
        &self,
        start_y: f32,
        lines: &[HighlightedRangeLine],
        bounds: RectF,
        scene: &mut Scene,
    ) {
        if lines.is_empty() {
            return;
        }

        let mut path = PathBuilder::new();
        let first_line = lines.first().unwrap();
        let last_line = lines.last().unwrap();

        let first_top_left = vec2f(first_line.start_x, start_y);
        let first_top_right = vec2f(first_line.end_x, start_y);

        let curve_height = vec2f(0., self.corner_radius);
        let curve_width = |start_x: f32, end_x: f32| {
            let max = (end_x - start_x) / 2.;
            let width = if max < self.corner_radius {
                max
            } else {
                self.corner_radius
            };

            vec2f(width, 0.)
        };

        let top_curve_width = curve_width(first_line.start_x, first_line.end_x);
        path.reset(first_top_right - top_curve_width);
        path.curve_to(first_top_right + curve_height, first_top_right);

        let mut iter = lines.iter().enumerate().peekable();
        while let Some((ix, line)) = iter.next() {
            let bottom_right = vec2f(line.end_x, start_y + (ix + 1) as f32 * self.line_height);

            if let Some((_, next_line)) = iter.peek() {
                let next_top_right = vec2f(next_line.end_x, bottom_right.y());

                match next_top_right.x().partial_cmp(&bottom_right.x()).unwrap() {
                    Ordering::Equal => {
                        path.line_to(bottom_right);
                    }
                    Ordering::Less => {
                        let curve_width = curve_width(next_top_right.x(), bottom_right.x());
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > 0. {
                            path.curve_to(bottom_right - curve_width, bottom_right);
                        }
                        path.line_to(next_top_right + curve_width);
                        if self.corner_radius > 0. {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                    Ordering::Greater => {
                        let curve_width = curve_width(bottom_right.x(), next_top_right.x());
                        path.line_to(bottom_right - curve_height);
                        if self.corner_radius > 0. {
                            path.curve_to(bottom_right + curve_width, bottom_right);
                        }
                        path.line_to(next_top_right - curve_width);
                        if self.corner_radius > 0. {
                            path.curve_to(next_top_right + curve_height, next_top_right);
                        }
                    }
                }
            } else {
                let curve_width = curve_width(line.start_x, line.end_x);
                path.line_to(bottom_right - curve_height);
                if self.corner_radius > 0. {
                    path.curve_to(bottom_right - curve_width, bottom_right);
                }

                let bottom_left = vec2f(line.start_x, bottom_right.y());
                path.line_to(bottom_left + curve_width);
                if self.corner_radius > 0. {
                    path.curve_to(bottom_left - curve_height, bottom_left);
                }
            }
        }

        if first_line.start_x > last_line.start_x {
            let curve_width = curve_width(last_line.start_x, first_line.start_x);
            let second_top_left = vec2f(last_line.start_x, start_y + self.line_height);
            path.line_to(second_top_left + curve_height);
            if self.corner_radius > 0. {
                path.curve_to(second_top_left + curve_width, second_top_left);
            }
            let first_bottom_left = vec2f(first_line.start_x, second_top_left.y());
            path.line_to(first_bottom_left - curve_width);
            if self.corner_radius > 0. {
                path.curve_to(first_bottom_left - curve_height, first_bottom_left);
            }
        }

        path.line_to(first_top_left + curve_height);
        if self.corner_radius > 0. {
            path.curve_to(first_top_left + top_curve_width, first_top_left);
        }
        path.line_to(first_top_right - top_curve_width);

        scene.push_path(path.build(self.color, Some(bounds)));
    }
}

fn scale_vertical_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.5) / 100.0
}

fn scale_horizontal_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.2) / 300.0
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        display_map::{BlockDisposition, BlockProperties},
        Editor, MultiBuffer,
    };
    use settings::Settings;
    use util::test::sample_text;

    #[gpui::test]
    fn test_layout_line_numbers(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
        let (window_id, editor) = cx.add_window(Default::default(), |cx| {
            Editor::new(EditorMode::Full, buffer, None, None, cx)
        });
        let element = EditorElement::new(
            editor.downgrade(),
            editor.read(cx).style(cx),
            CursorShape::Bar,
        );

        let layouts = editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let mut presenter = cx.build_presenter(window_id, 30.);
            let mut layout_cx = presenter.build_layout_context(false, cx);
            element.layout_line_numbers(0..6, &Default::default(), &snapshot, &mut layout_cx)
        });
        assert_eq!(layouts.len(), 6);
    }

    #[gpui::test]
    fn test_layout_with_placeholder_text_and_blocks(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("", cx);
        let (window_id, editor) = cx.add_window(Default::default(), |cx| {
            Editor::new(EditorMode::Full, buffer, None, None, cx)
        });

        editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("hello", cx);
            editor.insert_blocks(
                [BlockProperties {
                    disposition: BlockDisposition::Above,
                    height: 3,
                    position: Anchor::min(),
                    render: Arc::new(|_| Empty::new().boxed()),
                }],
                cx,
            );

            // Blur the editor so that it displays placeholder text.
            cx.blur();
        });

        let mut element = EditorElement::new(
            editor.downgrade(),
            editor.read(cx).style(cx),
            CursorShape::Bar,
        );

        let mut scene = Scene::new(1.0);
        let mut presenter = cx.build_presenter(window_id, 30.);
        let mut layout_cx = presenter.build_layout_context(false, cx);
        let (size, mut state) = element.layout(
            SizeConstraint::new(vec2f(500., 500.), vec2f(500., 500.)),
            &mut layout_cx,
        );

        assert_eq!(state.line_layouts.len(), 4);
        assert_eq!(
            state
                .line_number_layouts
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>(),
            &[false, false, false, true]
        );

        // Don't panic.
        let bounds = RectF::new(Default::default(), size);
        let mut paint_cx = presenter.build_paint_context(&mut scene, cx);
        element.paint(bounds, bounds, &mut state, &mut paint_cx);
    }
}
