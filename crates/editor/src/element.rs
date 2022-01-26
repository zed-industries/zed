use super::{
    display_map::{BlockContext, ToDisplayPoint},
    Anchor, DisplayPoint, Editor, EditorMode, EditorSettings, EditorSnapshot, EditorStyle, Input,
    Scroll, Select, SelectPhase, SoftWrap, ToPoint, MAX_LINE_LEN,
};
use clock::ReplicaId;
use collections::{BTreeMap, HashMap};
use gpui::{
    color::Color,
    elements::layout_highlighted_chunks,
    fonts::{HighlightStyle, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
        PathBuilder,
    },
    json::{self, ToJson},
    keymap::Keystroke,
    text_layout::{self, RunStyle, TextLayoutCache},
    AppContext, Axis, Border, Element, ElementBox, Event, EventContext, LayoutContext,
    MutableAppContext, PaintContext, Quad, Scene, SizeConstraint, ViewContext, WeakViewHandle,
};
use json::json;
use language::Bias;
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    ops::Range,
};

pub struct EditorElement {
    view: WeakViewHandle<Editor>,
    settings: EditorSettings,
}

impl EditorElement {
    pub fn new(view: WeakViewHandle<Editor>, settings: EditorSettings) -> Self {
        Self { view, settings }
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

    fn key_down(&self, chars: &str, keystroke: &Keystroke, cx: &mut EventContext) -> bool {
        let view = self.view.upgrade(cx.app).unwrap();

        if view.is_focused(cx.app) {
            if chars.is_empty() {
                false
            } else {
                if chars.chars().any(|c| c.is_control()) || keystroke.cmd || keystroke.ctrl {
                    false
                } else {
                    cx.dispatch_action(Input(chars.to_string()));
                    true
                }
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
        let style = &self.settings.style;
        cx.scene.push_quad(Quad {
            bounds: gutter_bounds,
            background: Some(style.gutter_background),
            border: Border::new(0., Color::transparent_black()),
            corner_radius: 0.,
        });
        cx.scene.push_quad(Quad {
            bounds: text_bounds,
            background: Some(style.background),
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
                        background: Some(style.active_line_background),
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
                    background: Some(style.highlighted_line_background),
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
        layout: &LayoutState,
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
    }

    fn paint_text(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &LayoutState,
        cx: &mut PaintContext,
    ) {
        let view = self.view(cx.app);
        let style = &self.settings.style;
        let local_replica_id = view.replica_id(cx);
        let scroll_position = layout.snapshot.scroll_position();
        let start_row = scroll_position.y() as u32;
        let scroll_top = scroll_position.y() * layout.line_height;
        let end_row = ((scroll_top + bounds.height()) / layout.line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen
        let max_glyph_width = layout.em_width;
        let scroll_left = scroll_position.x() * max_glyph_width;

        cx.scene.push_layer(Some(bounds));

        // Draw selections
        let corner_radius = 2.5;
        let mut cursors = SmallVec::<[Cursor; 32]>::new();

        let content_origin = bounds.origin() + layout.text_offset;

        for (replica_id, selections) in &layout.selections {
            let style = style.replica_selection_style(*replica_id);

            for selection in selections {
                if selection.start != selection.end {
                    let row_range = if selection.end.column() == 0 {
                        cmp::max(selection.start.row(), start_row)
                            ..cmp::min(selection.end.row(), end_row)
                    } else {
                        cmp::max(selection.start.row(), start_row)
                            ..cmp::min(selection.end.row() + 1, end_row)
                    };

                    let selection = Selection {
                        color: style.selection,
                        line_height: layout.line_height,
                        start_y: content_origin.y() + row_range.start as f32 * layout.line_height
                            - scroll_top,
                        lines: row_range
                            .into_iter()
                            .map(|row| {
                                let line_layout = &layout.line_layouts[(row - start_row) as usize];
                                SelectionLine {
                                    start_x: if row == selection.start.row() {
                                        content_origin.x()
                                            + line_layout
                                                .x_for_index(selection.start.column() as usize)
                                            - scroll_left
                                    } else {
                                        content_origin.x() - scroll_left
                                    },
                                    end_x: if row == selection.end.row() {
                                        content_origin.x()
                                            + line_layout
                                                .x_for_index(selection.end.column() as usize)
                                            - scroll_left
                                    } else {
                                        content_origin.x()
                                            + line_layout.width()
                                            + corner_radius * 2.0
                                            - scroll_left
                                    },
                                }
                            })
                            .collect(),
                    };

                    selection.paint(bounds, cx.scene);
                }

                if view.show_local_cursors() || *replica_id != local_replica_id {
                    let cursor_position = selection.head();
                    if (start_row..end_row).contains(&cursor_position.row()) {
                        let cursor_row_layout =
                            &layout.line_layouts[(cursor_position.row() - start_row) as usize];
                        let x = cursor_row_layout.x_for_index(cursor_position.column() as usize)
                            - scroll_left;
                        let y = cursor_position.row() as f32 * layout.line_height - scroll_top;
                        cursors.push(Cursor {
                            color: style.cursor,
                            origin: content_origin + vec2f(x, y),
                            line_height: layout.line_height,
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

        cx.scene.pop_layer();
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
        let style = &self.settings.style;

        cx.text_layout_cache
            .layout_str(
                "1".repeat(digit_count).as_str(),
                style.text.font_size,
                &[(
                    digit_count,
                    RunStyle {
                        font_id: style.text.font_id,
                        color: Color::black(),
                        underline: None,
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
        let style = &self.settings.style;
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
                                underline: None,
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
        mut rows: Range<u32>,
        snapshot: &mut EditorSnapshot,
        cx: &LayoutContext,
    ) -> Vec<text_layout::Line> {
        rows.end = cmp::min(rows.end, snapshot.max_point().row() + 1);
        if rows.start >= rows.end {
            return Vec::new();
        }

        // When the editor is empty and unfocused, then show the placeholder.
        if snapshot.is_empty() && !snapshot.is_focused() {
            let placeholder_style = self.settings.style.placeholder_text();
            let placeholder_text = snapshot.placeholder_text();
            let placeholder_lines = placeholder_text
                .as_ref()
                .map_or("", AsRef::as_ref)
                .split('\n')
                .skip(rows.start as usize)
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
                                underline: None,
                            },
                        )],
                    )
                })
                .collect();
        } else {
            let style = &self.settings.style;
            let chunks = snapshot
                .chunks(rows.clone(), Some(&style.syntax))
                .map(|chunk| {
                    let highlight = if let Some(severity) = chunk.diagnostic {
                        let diagnostic_style = super::diagnostic_style(severity, true, style);
                        let underline = Some(Underline {
                            color: diagnostic_style.message.text.color,
                            thickness: 1.0.into(),
                            squiggly: true,
                        });
                        if let Some(mut highlight) = chunk.highlight_style {
                            highlight.underline = underline;
                            Some(highlight)
                        } else {
                            Some(HighlightStyle {
                                underline,
                                color: style.text.color,
                                font_properties: style.text.font_properties,
                            })
                        }
                    } else {
                        chunk.highlight_style
                    };
                    (chunk.text, highlight)
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
        snapshot
            .blocks_in_range(rows.clone())
            .map(|(start_row, block)| {
                let anchor_row = block
                    .position()
                    .to_point(&snapshot.buffer_snapshot)
                    .to_display_point(snapshot)
                    .row();

                let anchor_x = text_x
                    + if rows.contains(&anchor_row) {
                        line_layouts[(anchor_row - rows.start) as usize]
                            .x_for_index(block.column() as usize)
                    } else {
                        layout_line(anchor_row, snapshot, style, cx.text_layout_cache)
                            .x_for_index(block.column() as usize)
                    };

                let mut element = block.render(&BlockContext {
                    cx,
                    anchor_x,
                    gutter_padding,
                    line_height,
                    scroll_x: snapshot.scroll_position.x(),
                    gutter_width,
                    em_width,
                });
                element.layout(
                    SizeConstraint {
                        min: Vector2F::zero(),
                        max: vec2f(width, block.height() as f32 * line_height),
                    },
                    cx,
                );
                (start_row, element)
            })
            .collect()
    }
}

impl Element for EditorElement {
    type LayoutState = Option<LayoutState>;
    type PaintState = Option<PaintState>;

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
        let style = self.settings.style.clone();
        let line_height = style.text.line_height(cx.font_cache);

        let gutter_padding;
        let gutter_width;
        if snapshot.mode == EditorMode::Full {
            gutter_padding = style.text.em_width(cx.font_cache) * style.gutter_padding_factor;
            gutter_width = self.max_line_number_width(&snapshot, cx) + gutter_padding * 2.0;
        } else {
            gutter_padding = 0.0;
            gutter_width = 0.0
        };

        let text_width = size.x() - gutter_width;
        let text_offset = vec2f(-style.text.descent(cx.font_cache), 0.);
        let em_width = style.text.em_width(cx.font_cache);
        let em_advance = style.text.em_advance(cx.font_cache);
        let overscroll = vec2f(em_width, 0.);
        let wrap_width = match self.settings.soft_wrap {
            SoftWrap::None => None,
            SoftWrap::EditorWidth => Some(text_width - text_offset.x() - overscroll.x() - em_width),
            SoftWrap::Column(column) => Some(column as f32 * em_advance),
        };
        let snapshot = self.update_view(cx.app, |view, cx| {
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
        let end_row = ((scroll_top + size.y()) / line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen

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

        let mut selections = HashMap::default();
        let mut active_rows = BTreeMap::new();
        let mut highlighted_rows = None;
        self.update_view(cx.app, |view, cx| {
            highlighted_rows = view.highlighted_rows();
            let display_map = view.display_map.update(cx, |map, cx| map.snapshot(cx));

            let local_selections = view
                .local_selections_in_range(start_anchor.clone()..end_anchor.clone(), &display_map);
            for selection in &local_selections {
                let is_empty = selection.start == selection.end;
                let selection_start = snapshot.prev_line_boundary(selection.start).1;
                let selection_end = snapshot.next_line_boundary(selection.end).1;
                for row in cmp::max(selection_start.row(), start_row)
                    ..=cmp::min(selection_end.row(), end_row)
                {
                    let contains_non_empty_selection = active_rows.entry(row).or_insert(!is_empty);
                    *contains_non_empty_selection |= !is_empty;
                }
            }
            selections.insert(
                view.replica_id(cx),
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
            );

            for (replica_id, selection) in display_map
                .buffer_snapshot
                .remote_selections_in_range(&(start_anchor..end_anchor))
            {
                selections
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
        });

        let line_number_layouts =
            self.layout_line_numbers(start_row..end_row, &active_rows, &snapshot, cx);

        let mut max_visible_line_width = 0.0;
        let line_layouts = self.layout_lines(start_row..end_row, &mut snapshot, cx);
        for line in &line_layouts {
            if line.width() > max_visible_line_width {
                max_visible_line_width = line.width();
            }
        }

        let style = self.settings.style.clone();
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
        });

        let blocks = self.layout_blocks(
            start_row..end_row,
            &snapshot,
            size.x().max(scroll_width + gutter_width),
            gutter_padding,
            gutter_width,
            em_width,
            gutter_width + text_offset.x(),
            line_height,
            &style,
            &line_layouts,
            cx,
        );

        (
            size,
            Some(LayoutState {
                size,
                scroll_max,
                gutter_size,
                gutter_padding,
                text_size,
                text_offset,
                snapshot,
                active_rows,
                highlighted_rows,
                line_layouts,
                line_number_layouts,
                blocks,
                line_height,
                em_width,
                em_advance,
                selections,
            }),
        )
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let layout = layout.as_mut()?;
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

        Some(PaintState {
            bounds,
            gutter_bounds,
            text_bounds,
        })
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if let (Some(layout), Some(paint)) = (layout, paint) {
            match event {
                Event::LeftMouseDown {
                    position,
                    alt,
                    shift,
                    click_count,
                    ..
                } => self.mouse_down(*position, *alt, *shift, *click_count, layout, paint, cx),
                Event::LeftMouseUp { position } => self.mouse_up(*position, cx),
                Event::LeftMouseDragged { position } => {
                    self.mouse_dragged(*position, layout, paint, cx)
                }
                Event::ScrollWheel {
                    position,
                    delta,
                    precise,
                } => self.scroll(*position, *delta, *precise, layout, paint, cx),
                Event::KeyDown {
                    chars, keystroke, ..
                } => self.key_down(chars, keystroke, cx),
                _ => false,
            }
        } else {
            false
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
    selections: HashMap<ReplicaId, Vec<text::Selection<DisplayPoint>>>,
    text_offset: Vector2F,
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
                underline: None,
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

struct Cursor {
    origin: Vector2F,
    line_height: f32,
    color: Color,
}

impl Cursor {
    fn paint(&self, cx: &mut PaintContext) {
        cx.scene.push_quad(Quad {
            bounds: RectF::new(self.origin, vec2f(2.0, self.line_height)),
            background: Some(self.color),
            border: Border::new(0., Color::black()),
            corner_radius: 0.,
        });
    }
}

#[derive(Debug)]
struct Selection {
    start_y: f32,
    line_height: f32,
    lines: Vec<SelectionLine>,
    color: Color,
}

#[derive(Debug)]
struct SelectionLine {
    start_x: f32,
    end_x: f32,
}

impl Selection {
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

    fn paint_lines(&self, start_y: f32, lines: &[SelectionLine], bounds: RectF, scene: &mut Scene) {
        if lines.is_empty() {
            return;
        }

        let mut path = PathBuilder::new();
        let corner_radius = 0.15 * self.line_height;
        let first_line = lines.first().unwrap();
        let last_line = lines.last().unwrap();

        let first_top_left = vec2f(first_line.start_x, start_y);
        let first_top_right = vec2f(first_line.end_x, start_y);

        let curve_height = vec2f(0., corner_radius);
        let curve_width = |start_x: f32, end_x: f32| {
            let max = (end_x - start_x) / 2.;
            let width = if max < corner_radius {
                max
            } else {
                corner_radius
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
                        path.curve_to(bottom_right - curve_width, bottom_right);
                        path.line_to(next_top_right + curve_width);
                        path.curve_to(next_top_right + curve_height, next_top_right);
                    }
                    Ordering::Greater => {
                        let curve_width = curve_width(bottom_right.x(), next_top_right.x());
                        path.line_to(bottom_right - curve_height);
                        path.curve_to(bottom_right + curve_width, bottom_right);
                        path.line_to(next_top_right - curve_width);
                        path.curve_to(next_top_right + curve_height, next_top_right);
                    }
                }
            } else {
                let curve_width = curve_width(line.start_x, line.end_x);
                path.line_to(bottom_right - curve_height);
                path.curve_to(bottom_right - curve_width, bottom_right);

                let bottom_left = vec2f(line.start_x, bottom_right.y());
                path.line_to(bottom_left + curve_width);
                path.curve_to(bottom_left - curve_height, bottom_left);
            }
        }

        if first_line.start_x > last_line.start_x {
            let curve_width = curve_width(last_line.start_x, first_line.start_x);
            let second_top_left = vec2f(last_line.start_x, start_y + self.line_height);
            path.line_to(second_top_left + curve_height);
            path.curve_to(second_top_left + curve_width, second_top_left);
            let first_bottom_left = vec2f(first_line.start_x, second_top_left.y());
            path.line_to(first_bottom_left - curve_width);
            path.curve_to(first_bottom_left - curve_height, first_bottom_left);
        }

        path.line_to(first_top_left + curve_height);
        path.curve_to(first_top_left + top_curve_width, first_top_left);
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
    use super::*;
    use crate::{Editor, EditorSettings, MultiBuffer};
    use std::sync::Arc;
    use util::test::sample_text;

    #[gpui::test]
    fn test_layout_line_numbers(cx: &mut gpui::MutableAppContext) {
        let settings = EditorSettings::test(cx);
        let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
        let (window_id, editor) = cx.add_window(Default::default(), |cx| {
            Editor::for_buffer(
                buffer,
                {
                    let settings = settings.clone();
                    Arc::new(move |_| settings.clone())
                },
                cx,
            )
        });
        let element = EditorElement::new(editor.downgrade(), settings);

        let layouts = editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let mut presenter = cx.build_presenter(window_id, 30.);
            let mut layout_cx = presenter.build_layout_context(false, cx);
            element.layout_line_numbers(0..6, &Default::default(), &snapshot, &mut layout_cx)
        });
        assert_eq!(layouts.len(), 6);
    }
}
