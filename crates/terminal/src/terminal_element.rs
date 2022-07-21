pub mod terminal_layout_context;

use alacritty_terminal::{
    ansi::{Color::Named, NamedColor},
    event::WindowSize,
    grid::{Dimensions, GridIterator, Indexed, Scroll},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    selection::SelectionRange,
    term::cell::{Cell, Flags},
};
use editor::{Cursor, CursorShape, HighlightedRange, HighlightedRangeLine};
use gpui::{
    color::Color,
    elements::*,
    fonts::{TextStyle, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    text_layout::{Line, RunStyle},
    Event, FontCache, KeyDownEvent, MouseButton, MouseButtonEvent, MouseMovedEvent, MouseRegion,
    PaintContext, Quad, ScrollWheelEvent, SizeConstraint, TextLayoutCache, WeakModelHandle,
    WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use theme::TerminalStyle;
use util::ResultExt;

use std::{cmp::min, ops::Range};
use std::{fmt::Debug, ops::Sub};

use crate::{color_translation::convert_color, connection::Terminal, TerminalView};

use self::terminal_layout_context::TerminalLayoutData;

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalEl {
    connection: WeakModelHandle<Terminal>,
    view: WeakViewHandle<TerminalView>,
    modal: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalDimensions {
    pub cell_width: f32,
    pub line_height: f32,
    pub height: f32,
    pub width: f32,
}

impl TerminalDimensions {
    pub fn new(line_height: f32, cell_width: f32, size: Vector2F) -> Self {
        TerminalDimensions {
            cell_width,
            line_height,
            width: size.x(),
            height: size.y(),
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }
}

//TODO look at what TermSize is
impl Into<WindowSize> for TerminalDimensions {
    fn into(self) -> WindowSize {
        WindowSize {
            num_lines: self.num_lines() as u16,
            num_cols: self.num_columns() as u16,
            cell_width: self.cell_width() as u16,
            cell_height: self.line_height() as u16,
        }
    }
}

impl Dimensions for TerminalDimensions {
    fn total_lines(&self) -> usize {
        self.num_lines() //TODO: Check that this is fine. This is supposed to be for the back buffer...
    }

    fn screen_lines(&self) -> usize {
        self.num_lines()
    }

    fn columns(&self) -> usize {
        self.num_columns()
    }
}

#[derive(Clone, Debug, Default)]
struct LayoutCell {
    point: Point<i32, i32>,
    text: Line,
}

impl LayoutCell {
    fn new(point: Point<i32, i32>, text: Line) -> LayoutCell {
        LayoutCell { point, text }
    }

    fn paint(
        &self,
        origin: Vector2F,
        layout: &LayoutState,
        visible_bounds: RectF,
        cx: &mut PaintContext,
    ) {
        let pos = point_to_absolute(origin, self.point, layout);
        self.text
            .paint(pos, visible_bounds, layout.size.line_height, cx);
    }
}

#[derive(Clone, Debug, Default)]
struct LayoutRect {
    point: Point<i32, i32>,
    num_of_cells: usize,
    color: Color,
}

impl LayoutRect {
    fn new(point: Point<i32, i32>, num_of_cells: usize, color: Color) -> LayoutRect {
        LayoutRect {
            point,
            num_of_cells,
            color,
        }
    }

    fn extend(&self) -> Self {
        LayoutRect {
            point: self.point,
            num_of_cells: self.num_of_cells + 1,
            color: self.color,
        }
    }

    fn paint(&self, origin: Vector2F, layout: &LayoutState, cx: &mut PaintContext) {
        let position = point_to_absolute(origin, self.point, layout);

        let size = vec2f(
            (layout.size.cell_width.ceil() * self.num_of_cells as f32).ceil(),
            layout.size.line_height,
        );

        cx.scene.push_quad(Quad {
            bounds: RectF::new(position, size),
            background: Some(self.color),
            border: Default::default(),
            corner_radius: 0.,
        })
    }
}

fn point_to_absolute(origin: Vector2F, point: Point<i32, i32>, layout: &LayoutState) -> Vector2F {
    vec2f(
        (origin.x() + point.column as f32 * layout.size.cell_width).floor(),
        origin.y() + point.line as f32 * layout.size.line_height,
    )
}

#[derive(Clone, Debug, Default)]
struct RelativeHighlightedRange {
    line_index: usize,
    range: Range<usize>,
}

impl RelativeHighlightedRange {
    fn new(line_index: usize, range: Range<usize>) -> Self {
        RelativeHighlightedRange { line_index, range }
    }

    fn to_highlighted_range_line(
        &self,
        origin: Vector2F,
        layout: &LayoutState,
    ) -> HighlightedRangeLine {
        let start_x = origin.x() + self.range.start as f32 * layout.size.cell_width;
        let end_x =
            origin.x() + self.range.end as f32 * layout.size.cell_width + layout.size.cell_width;

        return HighlightedRangeLine { start_x, end_x };
    }
}

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    highlights: Vec<RelativeHighlightedRange>,
    cursor: Option<Cursor>,
    background_color: Color,
    selection_color: Color,
    size: TerminalDimensions,
}

impl TerminalEl {
    pub fn new(
        view: WeakViewHandle<TerminalView>,
        connection: WeakModelHandle<Terminal>,
        modal: bool,
    ) -> TerminalEl {
        TerminalEl {
            view,
            connection,
            modal,
        }
    }

    fn attach_mouse_handlers(
        &self,
        origin: Vector2F,
        view_id: usize,
        visible_bounds: RectF,
        cur_size: TerminalDimensions,
        cx: &mut PaintContext,
    ) {
        let mouse_down_connection = self.connection.clone();
        let click_connection = self.connection.clone();
        let drag_connection = self.connection.clone();
        cx.scene.push_mouse_region(
            MouseRegion::new(view_id, None, visible_bounds)
                .on_down(
                    MouseButton::Left,
                    move |MouseButtonEvent { position, .. }, cx| {
                        if let Some(conn_handle) = mouse_down_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                let (point, side) = mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    terminal.get_display_offset(),
                                );

                                terminal.mouse_down(point, side);

                                cx.notify();
                            })
                        }
                    },
                )
                .on_click(
                    MouseButton::Left,
                    move |MouseButtonEvent {
                              position,
                              click_count,
                              ..
                          },
                          cx| {
                        cx.focus_parent_view();
                        if let Some(conn_handle) = click_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                let (point, side) = mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    terminal.get_display_offset(),
                                );

                                terminal.click(point, side, click_count);

                                cx.notify();
                            });
                        }
                    },
                )
                .on_drag(
                    MouseButton::Left,
                    move |_, MouseMovedEvent { position, .. }, cx| {
                        if let Some(conn_handle) = drag_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                let (point, side) = mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    terminal.get_display_offset(),
                                );

                                terminal.drag(point, side);

                                cx.notify()
                            });
                        }
                    },
                ),
        );
    }
}

impl Element for TerminalEl {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let layout =
            TerminalLayoutData::new(cx.global::<Settings>(), &cx.font_cache(), constraint.max);

        let terminal = self.connection.upgrade(cx).unwrap().read(cx);

        let (cursor, cells, rects, highlights) =
            terminal.render_lock(Some(layout.size.clone()), |content, cursor_text| {
                let (cells, rects, highlights) = layout_grid(
                    content.display_iter,
                    &layout.text_style,
                    layout.terminal_theme,
                    cx.text_layout_cache,
                    self.modal,
                    content.selection,
                );

                //Layout cursor
                let cursor = layout_cursor(
                    cursor_text,
                    cx.text_layout_cache,
                    &layout,
                    content.cursor.point,
                    content.display_offset,
                    constraint,
                );

                (cursor, cells, rects, highlights)
            });

        //Select background color
        let background_color = if self.modal {
            layout.terminal_theme.colors.modal_background
        } else {
            layout.terminal_theme.colors.background
        };

        //Done!
        (
            constraint.max,
            LayoutState {
                cells,
                cursor,
                background_color,
                selection_color: layout.selection_color,
                size: layout.size,
                rects,
                highlights,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        /*
         * For paint, I want to change how mouse events are handled:
         * - Refactor the mouse handlers to push the grid cell actions into the connection
         *   - But keep the conversion from GPUI coordinates to grid cells in the Terminal element
         * - Switch from directly painting things, to calling 'paint' on items produced by layout
         */

        //Setup element stuff
        let clip_bounds = Some(visible_bounds);

        cx.paint_layer(clip_bounds, |cx| {
            let origin = bounds.origin() + vec2f(layout.size.cell_width, 0.);

            //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
            self.attach_mouse_handlers(origin, self.view.id(), visible_bounds, layout.size, cx);

            cx.paint_layer(clip_bounds, |cx| {
                //Start with a background color
                cx.scene.push_quad(Quad {
                    bounds: RectF::new(bounds.origin(), bounds.size()),
                    background: Some(layout.background_color),
                    border: Default::default(),
                    corner_radius: 0.,
                });

                for rect in &layout.rects {
                    rect.paint(origin, &layout, cx)
                }
            });

            //Draw Selection
            cx.paint_layer(clip_bounds, |cx| {
                let start_y = layout.highlights.get(0).map(|highlight| {
                    origin.y() + highlight.line_index as f32 * layout.size.line_height
                });

                if let Some(y) = start_y {
                    let range_lines = layout
                        .highlights
                        .iter()
                        .map(|relative_highlight| {
                            relative_highlight.to_highlighted_range_line(origin, layout)
                        })
                        .collect::<Vec<HighlightedRangeLine>>();

                    let hr = HighlightedRange {
                        start_y: y, //Need to change this
                        line_height: layout.size.line_height,
                        lines: range_lines,
                        color: layout.selection_color,
                        //Copied from editor. TODO: move to theme or something
                        corner_radius: 0.15 * layout.size.line_height,
                    };
                    hr.paint(bounds, cx.scene);
                }
            });

            //Draw the text cells
            cx.paint_layer(clip_bounds, |cx| {
                for cell in &layout.cells {
                    cell.paint(origin, layout, visible_bounds, cx);
                }
            });

            //Draw cursor
            if let Some(cursor) = &layout.cursor {
                cx.paint_layer(clip_bounds, |cx| {
                    cursor.paint(origin, cx);
                })
            }
        });
    }

    fn dispatch_event(
        &mut self,
        event: &gpui::Event,
        _bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        cx: &mut gpui::EventContext,
    ) -> bool {
        match event {
            Event::ScrollWheel(ScrollWheelEvent {
                delta, position, ..
            }) => visible_bounds
                .contains_point(*position)
                .then(|| {
                    let vertical_scroll =
                        (delta.y() / layout.size.line_height) * ALACRITTY_SCROLL_MULTIPLIER;

                    self.connection.upgrade(cx.app).map(|terminal| {
                        terminal
                            .read(cx.app)
                            .scroll(Scroll::Delta(vertical_scroll.round() as i32));
                    });
                })
                .is_some(),
            Event::KeyDown(KeyDownEvent { keystroke, .. }) => {
                if !cx.is_parent_view_focused() {
                    return false;
                }

                //TODO Talk to keith about how to catch events emitted from an element.
                if let Some(view) = self.view.upgrade(cx.app) {
                    view.update(cx.app, |view, cx| view.clear_bel(cx))
                }

                self.connection
                    .upgrade(cx.app)
                    .map(|model_handle| model_handle.read(cx.app))
                    .map(|term| term.try_keystroke(keystroke))
                    .unwrap_or(false)
            }
            _ => false,
        }
    }

    fn metadata(&self) -> Option<&dyn std::any::Any> {
        None
    }

    fn debug(
        &self,
        _bounds: gpui::geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _cx: &gpui::DebugContext,
    ) -> gpui::serde_json::Value {
        json!({
            "type": "TerminalElement",
        })
    }
}

///TODO: Fix cursor rendering with alacritty fork
fn layout_cursor(
    cursor_text: char,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutData,
    cursor_point: Point,
    display_offset: usize,
    constraint: SizeConstraint,
) -> Option<Cursor> {
    let cursor_text = layout_cursor_text(cursor_text, cursor_point, text_layout_cache, tcx);
    get_cursor_shape(
        cursor_point.line.0 as usize,
        cursor_point.column.0 as usize,
        display_offset,
        tcx.size.line_height,
        tcx.size.cell_width,
        (constraint.max.y() / tcx.size.line_height) as usize, //TODO
        &cursor_text,
    )
    .map(move |(cursor_position, block_width)| {
        let block_width = if block_width != 0.0 {
            block_width
        } else {
            tcx.size.cell_width
        };

        Cursor::new(
            cursor_position,
            block_width,
            tcx.size.line_height,
            tcx.terminal_theme.colors.cursor,
            CursorShape::Block,
            Some(cursor_text.clone()),
        )
    })
}

fn layout_cursor_text(
    cursor_text: char,
    _cursor_point: Point,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutData,
) -> Line {
    let cursor_text = cursor_text.to_string();
    text_layout_cache.layout_str(
        &cursor_text,
        tcx.text_style.font_size,
        &[(
            cursor_text.len(),
            RunStyle {
                font_id: tcx.text_style.font_id,
                color: tcx.terminal_theme.colors.background,
                underline: Default::default(),
            },
        )],
    )
}

pub fn mouse_to_cell_data(
    pos: Vector2F,
    origin: Vector2F,
    cur_size: TerminalDimensions,
    display_offset: usize,
) -> (Point, alacritty_terminal::index::Direction) {
    let pos = pos.sub(origin);
    let point = {
        let col = pos.x() / cur_size.cell_width; //TODO: underflow...
        let col = min(GridCol(col as usize), cur_size.last_column());

        let line = pos.y() / cur_size.line_height;
        let line = min(line as i32, cur_size.bottommost_line().0);

        Point::new(GridLine(line - display_offset as i32), col)
    };

    //Copied (with modifications) from alacritty/src/input.rs > Processor::cell_side()
    let side = {
        let x = pos.0.x() as usize;
        let cell_x = x.saturating_sub(cur_size.cell_width as usize) % cur_size.cell_width as usize;
        let half_cell_width = (cur_size.cell_width / 2.0) as usize;

        let additional_padding =
            (cur_size.width() - cur_size.cell_width * 2.) % cur_size.cell_width;
        let end_of_grid = cur_size.width() - cur_size.cell_width - additional_padding;
        //Width: Pixels or columns?
        if cell_x > half_cell_width
                // Edge case when mouse leaves the window.
                || x as f32 >= end_of_grid
        {
            Side::Right
        } else {
            Side::Left
        }
    };

    (point, side)
}

fn layout_grid(
    grid: GridIterator<Cell>,
    text_style: &TextStyle,
    terminal_theme: &TerminalStyle,
    text_layout_cache: &TextLayoutCache,
    modal: bool,
    selection_range: Option<SelectionRange>,
) -> (
    Vec<LayoutCell>,
    Vec<LayoutRect>,
    Vec<RelativeHighlightedRange>,
) {
    let mut cells = vec![];
    let mut rects = vec![];
    let mut highlight_ranges = vec![];

    let mut cur_rect: Option<LayoutRect> = None;
    let mut cur_alac_color = None;
    let mut highlighted_range = None;

    let linegroups = grid.group_by(|i| i.point.line);
    for (line_index, (_, line)) in linegroups.into_iter().enumerate() {
        for (x_index, cell) in line.enumerate() {
            //Increase selection range
            {
                if selection_range
                    .map(|range| range.contains(cell.point))
                    .unwrap_or(false)
                {
                    let mut range = highlighted_range.take().unwrap_or(x_index..x_index);
                    range.end = range.end.max(x_index);
                    highlighted_range = Some(range);
                }
            }

            //Expand background rect range
            {
                if matches!(cell.bg, Named(NamedColor::Background)) {
                    //Continue to next cell, resetting variables if nescessary
                    cur_alac_color = None;
                    if let Some(rect) = cur_rect {
                        rects.push(rect);
                        cur_rect = None
                    }
                } else {
                    match cur_alac_color {
                        Some(cur_color) => {
                            if cell.bg == cur_color {
                                cur_rect = cur_rect.take().map(|rect| rect.extend());
                            } else {
                                cur_alac_color = Some(cell.bg);
                                if let Some(_) = cur_rect {
                                    rects.push(cur_rect.take().unwrap());
                                }
                                cur_rect = Some(LayoutRect::new(
                                    Point::new(line_index as i32, cell.point.column.0 as i32),
                                    1,
                                    convert_color(&cell.bg, &terminal_theme.colors, modal),
                                ));
                            }
                        }
                        None => {
                            cur_alac_color = Some(cell.bg);
                            cur_rect = Some(LayoutRect::new(
                                Point::new(line_index as i32, cell.point.column.0 as i32),
                                1,
                                convert_color(&cell.bg, &terminal_theme.colors, modal),
                            ));
                        }
                    }
                }
            }

            //Layout current cell text
            {
                let cell_text = &cell.c.to_string();
                if cell_text != " " {
                    let cell_style = cell_style(&cell, terminal_theme, text_style, modal);

                    let layout_cell = text_layout_cache.layout_str(
                        cell_text,
                        text_style.font_size,
                        &[(cell_text.len(), cell_style)],
                    );

                    cells.push(LayoutCell::new(
                        Point::new(line_index as i32, cell.point.column.0 as i32),
                        layout_cell,
                    ))
                }
            };
        }

        if highlighted_range.is_some() {
            highlight_ranges.push(RelativeHighlightedRange::new(
                line_index,
                highlighted_range.take().unwrap(),
            ))
        }

        if cur_rect.is_some() {
            rects.push(cur_rect.take().unwrap());
        }
    }

    (cells, rects, highlight_ranges)
}

// Compute the cursor position and expected block width, may return a zero width if x_for_index returns
// the same position for sequential indexes. Use em_width instead
//TODO: This function is messy, too many arguments and too many ifs. Simplify.
fn get_cursor_shape(
    line: usize,
    line_index: usize,
    display_offset: usize,
    line_height: f32,
    cell_width: f32,
    total_lines: usize,
    text_fragment: &Line,
) -> Option<(Vector2F, f32)> {
    let cursor_line = line + display_offset;
    if cursor_line <= total_lines {
        let cursor_width = if text_fragment.width() == 0. {
            cell_width
        } else {
            text_fragment.width()
        };

        Some((
            vec2f(
                line_index as f32 * cell_width,
                cursor_line as f32 * line_height,
            ),
            cursor_width,
        ))
    } else {
        None
    }
}

///Convert the Alacritty cell styles to GPUI text styles and background color
fn cell_style(
    indexed: &Indexed<&Cell>,
    style: &TerminalStyle,
    text_style: &TextStyle,
    modal: bool,
) -> RunStyle {
    let flags = indexed.cell.flags;
    let fg = convert_color(&indexed.cell.fg, &style.colors, modal);

    let underline = flags
        .contains(Flags::UNDERLINE)
        .then(|| Underline {
            color: Some(fg),
            squiggly: false,
            thickness: OrderedFloat(1.),
        })
        .unwrap_or_default();

    RunStyle {
        color: fg,
        font_id: text_style.font_id,
        underline,
    }
}

mod test {

    #[test]
    fn test_mouse_to_selection() {
        let term_width = 100.;
        let term_height = 200.;
        let cell_width = 10.;
        let line_height = 20.;
        let mouse_pos_x = 100.; //Window relative
        let mouse_pos_y = 100.; //Window relative
        let origin_x = 10.;
        let origin_y = 20.;

        let cur_size = crate::terminal_element::TerminalDimensions::new(
            line_height,
            cell_width,
            gpui::geometry::vector::vec2f(term_width, term_height),
        );

        let mouse_pos = gpui::geometry::vector::vec2f(mouse_pos_x, mouse_pos_y);
        let origin = gpui::geometry::vector::vec2f(origin_x, origin_y); //Position of terminal window, 1 'cell' in
        let (point, _) =
            crate::terminal_element::mouse_to_cell_data(mouse_pos, origin, cur_size, 0);
        assert_eq!(
            point,
            alacritty_terminal::index::Point::new(
                alacritty_terminal::index::Line(((mouse_pos_y - origin_y) / line_height) as i32),
                alacritty_terminal::index::Column(((mouse_pos_x - origin_x) / cell_width) as usize),
            )
        );
    }
}
