use super::{BufferView, DisplayPoint, SelectAction};
use gpui::{
    color::{ColorF, ColorU},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
        PathBuilder,
    },
    text_layout::{self, TextLayoutCache},
    AfterLayoutContext, AppContext, Border, Element, Event, EventContext, FontCache, LayoutContext,
    PaintContext, Quad, Scene, SizeConstraint, ViewHandle,
};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::{
    cmp::{self},
    sync::Arc,
};

pub struct BufferElement {
    view: ViewHandle<BufferView>,
}

impl BufferElement {
    pub fn new(view: ViewHandle<BufferView>) -> Self {
        Self { view }
    }

    fn mouse_down(
        &self,
        position: Vector2F,
        cmd: bool,
        layout: &mut LayoutState,
        paint: &mut PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        if paint.text_bounds.contains_point(position) {
            let view = self.view.as_ref(ctx.app);
            let position =
                paint.point_for_position(view, layout, position, ctx.font_cache, ctx.app);
            ctx.dispatch_action("buffer:select", SelectAction::Begin { position, add: cmd });
            true
        } else {
            false
        }
    }

    fn mouse_up(&self, _position: Vector2F, ctx: &mut EventContext) -> bool {
        if self.view.as_ref(ctx.app).is_selecting() {
            ctx.dispatch_action("buffer:select", SelectAction::End);
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
        ctx: &mut EventContext,
    ) -> bool {
        let view = self.view.as_ref(ctx.app);

        if view.is_selecting() {
            let rect = paint.text_bounds;
            let mut scroll_delta = Vector2F::zero();

            let vertical_margin = view.line_height(ctx.font_cache).min(rect.height() / 3.0);
            let top = rect.origin_y() + vertical_margin;
            let bottom = rect.lower_left().y() - vertical_margin;
            if position.y() < top {
                scroll_delta.set_y(-scale_vertical_mouse_autoscroll_delta(top - position.y()))
            }
            if position.y() > bottom {
                scroll_delta.set_y(scale_vertical_mouse_autoscroll_delta(position.y() - bottom))
            }

            let horizontal_margin = view.line_height(ctx.font_cache).min(rect.width() / 3.0);
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

            ctx.dispatch_action(
                "buffer:select",
                SelectAction::Update {
                    position: paint.point_for_position(
                        view,
                        layout,
                        position,
                        ctx.font_cache,
                        ctx.app,
                    ),
                    scroll_position: (view.scroll_position() + scroll_delta).clamp(
                        Vector2F::zero(),
                        layout.scroll_max(view, ctx.font_cache, ctx.text_layout_cache, ctx.app),
                    ),
                },
            );
            true
        } else {
            false
        }
    }

    fn key_down(&self, chars: &str, ctx: &mut EventContext) -> bool {
        if self.view.is_focused(ctx.app) {
            if chars.is_empty() {
                false
            } else {
                if chars.chars().any(|c| c.is_control()) {
                    false
                } else {
                    ctx.dispatch_action("buffer:insert", chars.to_string());
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
        ctx: &mut EventContext,
    ) -> bool {
        if !paint.bounds.contains_point(position) {
            return false;
        }

        let view = self.view.as_ref(ctx.app);
        let font_cache = &ctx.font_cache;
        let layout_cache = &ctx.text_layout_cache;
        let max_glyph_width = view.em_width(font_cache);
        let line_height = view.line_height(font_cache);
        if !precise {
            delta *= vec2f(max_glyph_width, line_height);
        }

        let x = (view.scroll_position().x() * max_glyph_width - delta.x()) / max_glyph_width;
        let y = (view.scroll_position().y() * line_height - delta.y()) / line_height;
        let scroll_position = vec2f(x, y).clamp(
            Vector2F::zero(),
            layout.scroll_max(view, font_cache, layout_cache, ctx.app),
        );

        ctx.dispatch_action("buffer:scroll", scroll_position);

        true
    }

    fn paint_gutter(&mut self, rect: RectF, layout: &LayoutState, ctx: &mut PaintContext) {
        let view = self.view.as_ref(ctx.app);
        let line_height = view.line_height(ctx.font_cache);
        let scroll_top = view.scroll_position().y() * line_height;

        ctx.scene.push_layer(Some(rect));
        ctx.scene.push_quad(Quad {
            bounds: rect,
            background: Some(ColorU::white()),
            border: Border::new(0., ColorU::transparent_black()),
            corner_radius: 0.,
        });

        for (ix, line) in layout.line_number_layouts.iter().enumerate() {
            let line_origin = rect.origin()
                + vec2f(
                    rect.width() - line.width - layout.gutter_padding,
                    ix as f32 * line_height - (scroll_top % line_height),
                );
            line.paint(
                line_origin,
                RectF::new(vec2f(0., 0.), vec2f(line.width, line_height)),
                &[(0..line.len, ColorU::black())],
                ctx,
            );
        }

        ctx.scene.pop_layer();
    }

    fn paint_text(&mut self, bounds: RectF, layout: &LayoutState, ctx: &mut PaintContext) {
        let view = self.view.as_ref(ctx.app);
        let line_height = view.line_height(ctx.font_cache);
        let descent = view.font_descent(ctx.font_cache);
        let start_row = view.scroll_position().y() as u32;
        let scroll_top = view.scroll_position().y() * line_height;
        let end_row = ((scroll_top + bounds.height()) / line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen
        let max_glyph_width = view.em_width(ctx.font_cache);
        let scroll_left = view.scroll_position().x() * max_glyph_width;

        ctx.scene.push_layer(Some(bounds));
        ctx.scene.push_quad(Quad {
            bounds,
            background: Some(ColorU::white()),
            border: Border::new(0., ColorU::transparent_black()),
            corner_radius: 0.,
        });

        // Draw selections
        let corner_radius = 2.5;
        let mut cursors = SmallVec::<[Cursor; 32]>::new();

        let content_origin = bounds.origin() + vec2f(-descent, 0.0);

        for selection in view.selections_in_range(
            DisplayPoint::new(start_row, 0)..DisplayPoint::new(end_row, 0),
            ctx.app,
        ) {
            if selection.start != selection.end {
                let range_start = cmp::min(selection.start, selection.end);
                let range_end = cmp::max(selection.start, selection.end);
                let row_range = if range_end.column() == 0 {
                    cmp::max(range_start.row(), start_row)..cmp::min(range_end.row(), end_row)
                } else {
                    cmp::max(range_start.row(), start_row)..cmp::min(range_end.row() + 1, end_row)
                };

                let selection = Selection {
                    line_height,
                    start_y: content_origin.y() + row_range.start as f32 * line_height - scroll_top,
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
                                    content_origin.x() + line_layout.width + corner_radius * 2.0
                                        - scroll_left
                                },
                            }
                        })
                        .collect(),
                };

                selection.paint(bounds, ctx.scene);
            }

            if view.cursors_visible() {
                let cursor_position = selection.end;
                if (start_row..end_row).contains(&cursor_position.row()) {
                    let cursor_row_layout =
                        &layout.line_layouts[(selection.end.row() - start_row) as usize];
                    let x = cursor_row_layout.x_for_index(selection.end.column() as usize)
                        - scroll_left;
                    let y = selection.end.row() as f32 * line_height - scroll_top;
                    cursors.push(Cursor {
                        origin: content_origin + vec2f(x, y),
                        line_height,
                    });
                }
            }
        }

        // Draw glyphs
        for (ix, line) in layout.line_layouts.iter().enumerate() {
            let row = start_row + ix as u32;
            line.paint(
                content_origin + vec2f(-scroll_left, row as f32 * line_height - scroll_top),
                RectF::new(vec2f(scroll_left, 0.), vec2f(bounds.width(), line_height)),
                &[(0..line.len, ColorU::black())],
                ctx,
            );
        }

        ctx.scene.push_layer(Some(bounds));
        for cursor in cursors {
            cursor.paint(ctx);
        }
        ctx.scene.pop_layer();

        ctx.scene.pop_layer();
    }
}

impl Element for BufferElement {
    type LayoutState = Option<LayoutState>;
    type PaintState = Option<PaintState>;

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let app = ctx.app;
        let mut size = constraint.max;
        if size.y().is_infinite() {
            let view = self.view.as_ref(app);
            size.set_y((view.max_point(app).row() + 1) as f32 * view.line_height(ctx.font_cache));
        }
        if size.x().is_infinite() {
            unimplemented!("we don't yet handle an infinite width constraint on buffer elements");
        }

        let view = self.view.as_ref(app);
        let font_cache = &ctx.font_cache;
        let layout_cache = &ctx.text_layout_cache;
        let line_height = view.line_height(font_cache);

        let gutter_padding;
        let gutter_width;
        if view.is_gutter_visible() {
            gutter_padding = view.em_width(ctx.font_cache);
            match view.max_line_number_width(ctx.font_cache, ctx.text_layout_cache, app) {
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

        let gutter_size = vec2f(gutter_width, size.y());
        let text_size = size - vec2f(gutter_width, 0.0);

        let autoscroll_horizontally = view.autoscroll_vertically(size.y(), line_height, app);

        let line_number_layouts = if view.is_gutter_visible() {
            match view.layout_line_numbers(size.y(), ctx.font_cache, ctx.text_layout_cache, app) {
                Err(error) => {
                    log::error!("error laying out line numbers: {}", error);
                    return (size, None);
                }
                Ok(layouts) => layouts,
            }
        } else {
            Vec::new()
        };

        let start_row = view.scroll_position().y() as u32;
        let scroll_top = view.scroll_position().y() * line_height;
        let end_row = ((scroll_top + size.y()) / line_height).ceil() as u32 + 1; // Add 1 to ensure selections bleed off screen

        let mut max_visible_line_width = 0.0;
        let line_layouts =
            match view.layout_lines(start_row..end_row, font_cache, layout_cache, app) {
                Err(error) => {
                    log::error!("error laying out lines: {}", error);
                    return (size, None);
                }
                Ok(layouts) => {
                    for line in &layouts {
                        if line.width > max_visible_line_width {
                            max_visible_line_width = line.width;
                        }
                    }

                    layouts
                }
            };

        (
            size,
            Some(LayoutState {
                size,
                gutter_size,
                gutter_padding,
                text_size,
                line_layouts,
                line_number_layouts,
                max_visible_line_width,
                autoscroll_horizontally,
            }),
        )
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        layout: &mut Option<LayoutState>,
        ctx: &mut AfterLayoutContext,
    ) {
        if let Some(layout) = layout {
            let app = ctx.app.downgrade();

            let view = self.view.as_ref(app);
            view.clamp_scroll_left(
                layout
                    .scroll_max(view, ctx.font_cache, ctx.text_layout_cache, app)
                    .x(),
            );

            if layout.autoscroll_horizontally {
                view.autoscroll_horizontally(
                    view.scroll_position().y() as u32,
                    layout.text_size.x(),
                    layout.scroll_width(view, ctx.font_cache, ctx.text_layout_cache, app),
                    view.em_width(ctx.font_cache),
                    &layout.line_layouts,
                    app,
                );
            }
        }
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        if let Some(layout) = layout {
            let gutter_bounds = RectF::new(bounds.origin(), layout.gutter_size);
            let text_bounds = RectF::new(
                bounds.origin() + vec2f(layout.gutter_size.x(), 0.0),
                layout.text_size,
            );

            if self.view.as_ref(ctx.app).is_gutter_visible() {
                self.paint_gutter(gutter_bounds, layout, ctx);
            }
            self.paint_text(text_bounds, layout, ctx);

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
        ctx: &mut EventContext,
    ) -> bool {
        if let (Some(layout), Some(paint)) = (layout, paint) {
            match event {
                Event::LeftMouseDown { position, cmd } => {
                    self.mouse_down(*position, *cmd, layout, paint, ctx)
                }
                Event::LeftMouseUp { position } => self.mouse_up(*position, ctx),
                Event::LeftMouseDragged { position } => {
                    self.mouse_dragged(*position, layout, paint, ctx)
                }
                Event::ScrollWheel {
                    position,
                    delta,
                    precise,
                } => self.scroll(*position, *delta, *precise, layout, paint, ctx),
                Event::KeyDown { chars, .. } => self.key_down(chars, ctx),
            }
        } else {
            false
        }
    }
}

pub struct LayoutState {
    size: Vector2F,
    gutter_size: Vector2F,
    gutter_padding: f32,
    text_size: Vector2F,
    line_layouts: Vec<Arc<text_layout::Line>>,
    line_number_layouts: Vec<Arc<text_layout::Line>>,
    max_visible_line_width: f32,
    autoscroll_horizontally: bool,
}

impl LayoutState {
    fn scroll_width(
        &self,
        view: &BufferView,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        app: &AppContext,
    ) -> f32 {
        let row = view.rightmost_point(app).row();
        let longest_line_width = view
            .layout_line(row, font_cache, layout_cache, app)
            .unwrap()
            .width;
        longest_line_width.max(self.max_visible_line_width) + view.em_width(font_cache)
    }

    fn scroll_max(
        &self,
        view: &BufferView,
        font_cache: &FontCache,
        layout_cache: &TextLayoutCache,
        app: &AppContext,
    ) -> Vector2F {
        vec2f(
            ((self.scroll_width(view, font_cache, layout_cache, app) - self.text_size.x())
                / view.em_width(font_cache))
            .max(0.0),
            view.max_point(app).row().saturating_sub(1) as f32,
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
        view: &BufferView,
        layout: &LayoutState,
        position: Vector2F,
        font_cache: &FontCache,
        app: &AppContext,
    ) -> DisplayPoint {
        let scroll_position = view.scroll_position();
        let position = position - self.text_bounds.origin();
        let y = position.y().max(0.0).min(layout.size.y());
        let row = ((y / view.line_height(font_cache)) + scroll_position.y()) as u32;
        let row = cmp::min(row, view.max_point(app).row());
        let line = &layout.line_layouts[(row - scroll_position.y() as u32) as usize];
        let x = position.x() + (scroll_position.x() * view.em_width(font_cache));

        let column = if x >= 0.0 {
            line.index_for_x(x)
                .map(|ix| ix as u32)
                .unwrap_or(view.line_len(row, app).unwrap())
        } else {
            0
        };

        DisplayPoint::new(row, column)
    }
}

struct Cursor {
    origin: Vector2F,
    line_height: f32,
}

impl Cursor {
    fn paint(&self, ctx: &mut PaintContext) {
        ctx.scene.push_quad(Quad {
            bounds: RectF::new(self.origin, vec2f(2.0, self.line_height)),
            background: Some(ColorU::black()),
            border: Border::new(0., ColorU::black()),
            corner_radius: 0.,
        });
    }
}

#[derive(Debug)]
struct Selection {
    start_y: f32,
    line_height: f32,
    lines: Vec<SelectionLine>,
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

        scene.push_path(path.build(ColorF::new(0.639, 0.839, 1.0, 1.0).to_u8(), Some(bounds)));
    }
}

fn scale_vertical_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.5) / 100.0
}

fn scale_horizontal_mouse_autoscroll_delta(delta: f32) -> f32 {
    delta.powf(1.2) / 300.0
}
