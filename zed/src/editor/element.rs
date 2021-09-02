use super::{DisplayPoint, Editor, EditorMode, Insert, Scroll, Select, SelectPhase, Snapshot};
use crate::{theme::EditorStyle, time::ReplicaId};
use gpui::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
        PathBuilder,
    },
    json::{self, ToJson},
    text_layout::{self, TextLayoutCache},
    AppContext, Axis, Border, Element, Event, EventContext, FontCache, LayoutContext,
    MutableAppContext, PaintContext, Quad, Scene, SizeConstraint, ViewContext, WeakViewHandle,
};
use json::json;
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    collections::{BTreeMap, HashMap},
    ops::Range,
};

pub struct EditorElement {
    view: WeakViewHandle<Editor>,
    style: EditorStyle,
}

impl EditorElement {
    pub fn new(view: WeakViewHandle<Editor>, style: EditorStyle) -> Self {
        Self { view, style }
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

    fn snapshot(&self, cx: &mut MutableAppContext) -> Snapshot {
        self.update_view(cx, |view, cx| view.snapshot(cx))
    }

    fn mouse_down(
        &self,
        position: Vector2F,
        cmd: bool,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if paint.text_bounds.contains_point(position) {
            let snapshot = self.snapshot(cx.app);
            let position = paint.point_for_position(&snapshot, layout, position);
            cx.dispatch_action(Select(SelectPhase::Begin { position, add: cmd }));
            true
        } else {
            false
        }
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

            let font_cache = cx.font_cache.clone();
            let text_layout_cache = cx.text_layout_cache.clone();
            let snapshot = self.snapshot(cx.app);
            let position = paint.point_for_position(&snapshot, layout, position);

            cx.dispatch_action(Select(SelectPhase::Update {
                position,
                scroll_position: (snapshot.scroll_position() + scroll_delta).clamp(
                    Vector2F::zero(),
                    layout.scroll_max(&font_cache, &text_layout_cache),
                ),
            }));
            true
        } else {
            false
        }
    }

    fn key_down(&self, chars: &str, cx: &mut EventContext) -> bool {
        let view = self.view.upgrade(cx.app).unwrap();

        if view.is_focused(cx.app) {
            if chars.is_empty() {
                false
            } else {
                if chars.chars().any(|c| c.is_control()) {
                    false
                } else {
                    cx.dispatch_action(Insert(chars.to_string()));
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
        let font_cache = &cx.font_cache;
        let layout_cache = &cx.text_layout_cache;
        let max_glyph_width = layout.em_width;
        if !precise {
            delta *= vec2f(max_glyph_width, layout.line_height);
        }

        let scroll_position = snapshot.scroll_position();
        let x = (scroll_position.x() * max_glyph_width - delta.x()) / max_glyph_width;
        let y = (scroll_position.y() * layout.line_height - delta.y()) / layout.line_height;
        let scroll_position = vec2f(x, y).clamp(
            Vector2F::zero(),
            layout.scroll_max(font_cache, layout_cache),
        );

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
        let settings = self.view(cx.app).settings.borrow();
        let theme = &settings.theme.editor;
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
            let style_ix = *replica_id as usize % (theme.guest_selections.len() + 1);
            let style = if style_ix == 0 {
                &theme.selection
            } else {
                &theme.guest_selections[style_ix - 1]
            };

            for selection in selections {
                if selection.start != selection.end {
                    let range_start = cmp::min(selection.start, selection.end);
                    let range_end = cmp::max(selection.start, selection.end);
                    let row_range = if range_end.column() == 0 {
                        cmp::max(range_start.row(), start_row)..cmp::min(range_end.row(), end_row)
                    } else {
                        cmp::max(range_start.row(), start_row)
                            ..cmp::min(range_end.row() + 1, end_row)
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
                                    start_x: if row == range_start.row() {
                                        content_origin.x()
                                            + line_layout.x_for_index(range_start.column() as usize)
                                            - scroll_left
                                    } else {
                                        content_origin.x() - scroll_left
                                    },
                                    end_x: if row == range_end.row() {
                                        content_origin.x()
                                            + line_layout.x_for_index(range_end.column() as usize)
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

                if view.cursors_visible() {
                    let cursor_position = selection.end;
                    if (start_row..end_row).contains(&cursor_position.row()) {
                        let cursor_row_layout =
                            &layout.line_layouts[(selection.end.row() - start_row) as usize];
                        let x = cursor_row_layout.x_for_index(selection.end.column() as usize)
                            - scroll_left;
                        let y = selection.end.row() as f32 * layout.line_height - scroll_top;
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

        let font_cache = &cx.font_cache;
        let layout_cache = &cx.text_layout_cache;
        let snapshot = self.snapshot(cx.app);
        let line_height = snapshot.line_height(font_cache);

        let gutter_padding;
        let gutter_width;
        if snapshot.gutter_visible {
            gutter_padding = snapshot.em_width(cx.font_cache);
            match snapshot.max_line_number_width(cx.font_cache, cx.text_layout_cache) {
                Err(error) => {
                    log::error!("error computing max line number width: {}", error);
                    return (size, None);
                }
                Ok(width) => gutter_width = width + gutter_padding * 2.0,
            }
        } else {
            gutter_padding = 0.0;
            gutter_width = 0.0
        };

        let text_width = size.x() - gutter_width;
        let text_offset = vec2f(-snapshot.font_descent(cx.font_cache), 0.);
        let em_width = snapshot.em_width(font_cache);
        let overscroll = vec2f(em_width, 0.);
        let wrap_width = text_width - text_offset.x() - overscroll.x() - em_width;
        let snapshot = self.update_view(cx.app, |view, cx| {
            if view.set_wrap_width(wrap_width, cx) {
                view.snapshot(cx)
            } else {
                snapshot
            }
        });

        let scroll_height = (snapshot.max_point().row() + 1) as f32 * line_height;
        if snapshot.auto_height {
            size.set_y(
                scroll_height
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
        let end_row = ((scroll_top + size.y()) / line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen

        let mut selections = HashMap::new();
        let mut active_rows = BTreeMap::new();
        self.update_view(cx.app, |view, cx| {
            let replica_id = view.replica_id(cx);
            for selection_set_id in view.active_selection_sets(cx).collect::<Vec<_>>() {
                let mut set = Vec::new();
                for selection in view.selections_in_range(
                    selection_set_id,
                    DisplayPoint::new(start_row, 0)..DisplayPoint::new(end_row, 0),
                    cx,
                ) {
                    set.push(selection.clone());
                    if selection_set_id.replica_id == replica_id {
                        let is_empty = selection.start == selection.end;
                        let mut selection_start;
                        let mut selection_end;
                        if selection.start < selection.end {
                            selection_start = selection.start;
                            selection_end = selection.end;
                        } else {
                            selection_start = selection.end;
                            selection_end = selection.start;
                        };
                        selection_start = snapshot.prev_row_boundary(selection_start).0;
                        selection_end = snapshot.next_row_boundary(selection_end).0;
                        for row in cmp::max(selection_start.row(), start_row)
                            ..=cmp::min(selection_end.row(), end_row)
                        {
                            let contains_non_empty_selection =
                                active_rows.entry(row).or_insert(!is_empty);
                            *contains_non_empty_selection |= !is_empty;
                        }
                    }
                }

                selections.insert(selection_set_id.replica_id, set);
            }
        });

        let line_number_layouts = if snapshot.gutter_visible {
            let settings = self
                .view
                .upgrade(cx.app)
                .unwrap()
                .read(cx.app)
                .settings
                .borrow();
            match snapshot.layout_line_numbers(
                start_row..end_row,
                &active_rows,
                cx.font_cache,
                cx.text_layout_cache,
                &settings.theme,
            ) {
                Err(error) => {
                    log::error!("error laying out line numbers: {}", error);
                    return (size, None);
                }
                Ok(layouts) => layouts,
            }
        } else {
            Vec::new()
        };

        let mut max_visible_line_width = 0.0;
        let line_layouts = match snapshot.layout_lines(
            start_row..end_row,
            &self.style,
            font_cache,
            layout_cache,
        ) {
            Err(error) => {
                log::error!("error laying out lines: {}", error);
                return (size, None);
            }
            Ok(layouts) => {
                for line in &layouts {
                    if line.width() > max_visible_line_width {
                        max_visible_line_width = line.width();
                    }
                }

                layouts
            }
        };

        let mut layout = LayoutState {
            size,
            gutter_size,
            gutter_padding,
            text_size,
            overscroll,
            text_offset,
            snapshot,
            active_rows,
            line_layouts,
            line_number_layouts,
            line_height,
            em_width,
            selections,
            max_visible_line_width,
        };

        self.update_view(cx.app, |view, cx| {
            let clamped = view.clamp_scroll_left(layout.scroll_max(font_cache, layout_cache).x());
            let autoscrolled;
            if autoscroll_horizontally {
                autoscrolled = view.autoscroll_horizontally(
                    start_row,
                    layout.text_size.x(),
                    layout.scroll_width(font_cache, layout_cache),
                    layout.snapshot.em_width(font_cache),
                    &layout.line_layouts,
                    cx,
                );
            } else {
                autoscrolled = false;
            }

            if clamped || autoscrolled {
                layout.snapshot = view.snapshot(cx);
            }
        });

        (size, Some(layout))
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        if let Some(layout) = layout {
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

            cx.scene.pop_layer();

            Some(PaintState {
                bounds,
                text_bounds,
            })
        } else {
            None
        }
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
                Event::LeftMouseDown { position, cmd } => {
                    self.mouse_down(*position, *cmd, layout, paint, cx)
                }
                Event::LeftMouseUp { position } => self.mouse_up(*position, cx),
                Event::LeftMouseDragged { position } => {
                    self.mouse_dragged(*position, layout, paint, cx)
                }
                Event::ScrollWheel {
                    position,
                    delta,
                    precise,
                } => self.scroll(*position, *delta, *precise, layout, paint, cx),
                Event::KeyDown { chars, .. } => self.key_down(chars, cx),
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
    gutter_size: Vector2F,
    gutter_padding: f32,
    text_size: Vector2F,
    snapshot: Snapshot,
    active_rows: BTreeMap<u32, bool>,
    line_layouts: Vec<text_layout::Line>,
    line_number_layouts: Vec<Option<text_layout::Line>>,
    line_height: f32,
    em_width: f32,
    selections: HashMap<ReplicaId, Vec<Range<DisplayPoint>>>,
    overscroll: Vector2F,
    text_offset: Vector2F,
    max_visible_line_width: f32,
}

impl LayoutState {
    fn scroll_width(&self, font_cache: &FontCache, layout_cache: &TextLayoutCache) -> f32 {
        let row = self.snapshot.longest_row();
        let longest_line_width = self
            .snapshot
            .layout_line(row, font_cache, layout_cache)
            .unwrap()
            .width();
        longest_line_width.max(self.max_visible_line_width) + self.overscroll.x()
    }

    fn scroll_max(&self, font_cache: &FontCache, layout_cache: &TextLayoutCache) -> Vector2F {
        let text_width = self.text_size.x();
        let scroll_width = self.scroll_width(font_cache, layout_cache);
        let em_width = self.snapshot.em_width(font_cache);
        let max_row = self.snapshot.max_point().row();

        vec2f(
            ((scroll_width - text_width) / em_width).max(0.0),
            max_row.saturating_sub(1) as f32,
        )
    }
}

pub struct PaintState {
    bounds: RectF,
    text_bounds: RectF,
}

impl PaintState {
    fn point_for_position(
        &self,
        snapshot: &Snapshot,
        layout: &LayoutState,
        position: Vector2F,
    ) -> DisplayPoint {
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
                .unwrap_or(snapshot.line_len(row))
        } else {
            0
        };

        DisplayPoint::new(row, column)
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
