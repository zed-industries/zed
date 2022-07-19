mod terminal_layout_context;

use alacritty_terminal::{
    ansi::{Color::Named, NamedColor},
    grid::{Dimensions, GridIterator, Indexed, Scroll},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    selection::SelectionRange,
    term::{
        cell::{Cell, Flags},
        SizeInfo,
    },
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

use crate::{color_translation::convert_color, connection::TerminalConnection, TerminalView};

use self::terminal_layout_context::TerminalLayoutTheme;

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalEl {
    connection: WeakModelHandle<TerminalConnection>,
    view: WeakViewHandle<TerminalView>,
    modal: bool,
}

///New type pattern so I don't mix these two up
pub struct CellWidth(f32);
pub struct LineHeight(f32);

///New type pattern to ensure that we use adjusted mouse positions throughout the code base, rather than
struct PaneRelativePos(Vector2F);

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
            .paint(pos, visible_bounds, layout.line_height.0, cx);
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
            (layout.em_width.0.ceil() * self.num_of_cells as f32).ceil(),
            layout.line_height.0,
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
        (origin.x() + point.column as f32 * layout.em_width.0).floor(),
        origin.y() + point.line as f32 * layout.line_height.0,
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
        let start_x = origin.x() + self.range.start as f32 * layout.em_width.0;
        let end_x = origin.x() + self.range.end as f32 * layout.em_width.0 + layout.em_width.0;

        return HighlightedRangeLine { start_x, end_x };
    }
}

///Functionally the constructor for the PaneRelativePos type, mutates the mouse_position
fn relative_pos(mouse_position: Vector2F, origin: Vector2F) -> PaneRelativePos {
    PaneRelativePos(mouse_position.sub(origin))
}

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    highlights: Vec<RelativeHighlightedRange>,
    line_height: LineHeight,
    em_width: CellWidth,
    cursor: Option<Cursor>,
    background_color: Color,
    selection_color: Color,
    cur_size: SizeInfo,
}

impl TerminalEl {
    pub fn new(
        view: WeakViewHandle<TerminalView>,
        connection: WeakModelHandle<TerminalConnection>,
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
        cur_size: SizeInfo,
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
                            conn_handle.update(cx.app, |connection, cx| {
                                connection.get_terminal().map(|terminal| {
                                    let (point, side) = mouse_to_cell_data(
                                        position,
                                        origin,
                                        cur_size,
                                        terminal.get_display_offset(),
                                    );

                                    terminal.mouse_down(point, side);

                                    cx.notify();
                                });
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
                            conn_handle.update(cx.app, |connection, cx| {
                                connection.get_terminal().map(|terminal| {
                                    let (point, side) = mouse_to_cell_data(
                                        position,
                                        origin,
                                        cur_size,
                                        terminal.get_display_offset(),
                                    );

                                    terminal.click(point, side, click_count);

                                    cx.notify();
                                })
                            });
                        }
                    },
                )
                .on_drag(
                    MouseButton::Left,
                    move |_, MouseMovedEvent { position, .. }, cx| {
                        if let Some(conn_handle) = drag_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |connection, cx| {
                                connection.get_terminal().map(|terminal| {
                                    let (point, side) = mouse_to_cell_data(
                                        position,
                                        origin,
                                        cur_size,
                                        terminal.get_display_offset(),
                                    );

                                    terminal.drag(point, side);

                                    cx.notify()
                                });
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
        let tcx = TerminalLayoutTheme::new(cx.global::<Settings>(), &cx.font_cache());

        //This locks the terminal, so resize it first.
        //Layout grid cells
        let cur_size = make_new_size(constraint, &tcx.cell_width, &tcx.line_height);

        let terminal = self
            .connection
            .upgrade(cx)
            .unwrap()
            .read(cx)
            .get_terminal()
            .unwrap();

        let (cursor, cells, rects, highlights) = terminal.render_lock(Some(cur_size), |content| {
            let (cells, rects, highlights) = layout_grid(
                content.display_iter,
                &tcx.text_style,
                tcx.terminal_theme,
                cx.text_layout_cache,
                self.modal,
                content.selection,
            );

            //Layout cursor
            let cursor = layout_cursor(
                // grid,
                cx.text_layout_cache,
                &tcx,
                content.cursor.point,
                content.display_offset,
                constraint,
            );

            (cursor, cells, rects, highlights)
        });

        //Select background color
        let background_color = if self.modal {
            tcx.terminal_theme.colors.modal_background
        } else {
            tcx.terminal_theme.colors.background
        };

        //Done!
        (
            constraint.max,
            LayoutState {
                cells,
                line_height: tcx.line_height,
                em_width: tcx.cell_width,
                cursor,
                background_color,
                selection_color: tcx.selection_color,
                cur_size,
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
            let origin = bounds.origin() + vec2f(layout.em_width.0, 0.);

            //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
            self.attach_mouse_handlers(origin, self.view.id(), visible_bounds, layout.cur_size, cx);

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
                    origin.y() + highlight.line_index as f32 * layout.line_height.0
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
                        line_height: layout.line_height.0,
                        lines: range_lines,
                        color: layout.selection_color,
                        //Copied from editor. TODO: move to theme or something
                        corner_radius: 0.15 * layout.line_height.0,
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
                        (delta.y() / layout.line_height.0) * ALACRITTY_SCROLL_MULTIPLIER;

                    self.connection
                        .upgrade(cx.app)
                        .and_then(|handle| handle.read(cx.app).get_terminal())
                        .map(|terminal| {
                            terminal.scroll(Scroll::Delta(vertical_scroll.round() as i32));
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
                    .and_then(|model_handle| model_handle.read(cx.app).get_terminal())
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
    // grid: &Grid<Cell>,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutTheme,
    cursor_point: Point,
    display_offset: usize,
    constraint: SizeConstraint,
) -> Option<Cursor> {
    let cursor_text = layout_cursor_text(/*grid,*/ cursor_point, text_layout_cache, tcx);
    get_cursor_shape(
        cursor_point.line.0 as usize,
        cursor_point.column.0 as usize,
        display_offset,
        &tcx.line_height,
        &tcx.cell_width,
        (constraint.max.y() / &tcx.line_height.0) as usize, //TODO
        &cursor_text,
    )
    .map(move |(cursor_position, block_width)| {
        let block_width = if block_width != 0.0 {
            block_width
        } else {
            tcx.cell_width.0
        };

        Cursor::new(
            cursor_position,
            block_width,
            tcx.line_height.0,
            tcx.terminal_theme.colors.cursor,
            CursorShape::Block,
            Some(cursor_text.clone()),
        )
    })
}

fn layout_cursor_text(
    // grid: &Grid<Cell>,
    _cursor_point: Point,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutTheme,
) -> Line {
    let cursor_text = " "; //grid[cursor_point.line][cursor_point.column].c.to_string();

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
    cur_size: SizeInfo,
    display_offset: usize,
) -> (Point, alacritty_terminal::index::Direction) {
    let relative_pos = relative_pos(pos, origin);
    let point = grid_cell(&relative_pos, cur_size, display_offset);
    let side = cell_side(&relative_pos, cur_size);
    (point, side)
}

///Configures a size info object from the given information.
fn make_new_size(
    constraint: SizeConstraint,
    cell_width: &CellWidth,
    line_height: &LineHeight,
) -> SizeInfo {
    SizeInfo::new(
        constraint.max.x() - cell_width.0,
        constraint.max.y(),
        cell_width.0,
        line_height.0,
        0.,
        0.,
        false,
    )
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
    line_height: &LineHeight,
    cell_width: &CellWidth,
    total_lines: usize,
    text_fragment: &Line,
) -> Option<(Vector2F, f32)> {
    let cursor_line = line + display_offset;
    if cursor_line <= total_lines {
        let cursor_width = if text_fragment.width() == 0. {
            cell_width.0
        } else {
            text_fragment.width()
        };

        Some((
            vec2f(
                line_index as f32 * cell_width.0,
                cursor_line as f32 * line_height.0,
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

// fn attach_mouse_handlers(
//     origin: Vector2F,
//     cur_size: SizeInfo,
//     view_id: usize,
//     terminal_mutex: &Arc<FairMutex<Term<ZedListener>>>,
//     visible_bounds: RectF,
//     cx: &mut PaintContext,
// ) {
//     let click_mutex = terminal_mutex.clone();
//     let drag_mutex = terminal_mutex.clone();
//     let mouse_down_mutex = terminal_mutex.clone();

//     cx.scene.push_mouse_region(
//         MouseRegion::new(view_id, None, visible_bounds)
//             .on_down(
//                 MouseButton::Left,
//                 move |MouseButtonEvent { position, .. }, _| {
//                     let mut term = mouse_down_mutex.lock();

//                     let (point, side) = mouse_to_cell_data(
//                         position,
//                         origin,
//                         cur_size,
//                         term.renderable_content().display_offset,
//                     );
//                     term.selection = Some(Selection::new(SelectionType::Simple, point, side))
//                 },
//             )
//             .on_click(
//                 MouseButton::Left,
//                 move |MouseButtonEvent {
//                           position,
//                           click_count,
//                           ..
//                       },
//                       cx| {
//                     let mut term = click_mutex.lock();

//                     let (point, side) = mouse_to_cell_data(
//                         position,
//                         origin,
//                         cur_size,
//                         term.renderable_content().display_offset,
//                     );

//                     let selection_type = match click_count {
//                         0 => return, //This is a release
//                         1 => Some(SelectionType::Simple),
//                         2 => Some(SelectionType::Semantic),
//                         3 => Some(SelectionType::Lines),
//                         _ => None,
//                     };

//                     let selection = selection_type
//                         .map(|selection_type| Selection::new(selection_type, point, side));

//                     term.selection = selection;
//                     cx.focus_parent_view();
//                     cx.notify();
//                 },
//             )
//             .on_drag(
//                 MouseButton::Left,
//                 move |_, MouseMovedEvent { position, .. }, cx| {
//                     let mut term = drag_mutex.lock();

//                     let (point, side) = mouse_to_cell_data(
//                         position,
//                         origin,
//                         cur_size,
//                         term.renderable_content().display_offset,
//                     );

//                     if let Some(mut selection) = term.selection.take() {
//                         selection.update(point, side);
//                         term.selection = Some(selection);
//                     }

//                     cx.notify();
//                 },
//             ),
//     );
// }

///Copied (with modifications) from alacritty/src/input.rs > Processor::cell_side()
fn cell_side(pos: &PaneRelativePos, cur_size: SizeInfo) -> Side {
    let x = pos.0.x() as usize;
    let cell_x = x.saturating_sub(cur_size.cell_width() as usize) % cur_size.cell_width() as usize;
    let half_cell_width = (cur_size.cell_width() / 2.0) as usize;

    let additional_padding =
        (cur_size.width() - cur_size.cell_width() * 2.) % cur_size.cell_width();
    let end_of_grid = cur_size.width() - cur_size.cell_width() - additional_padding;

    if cell_x > half_cell_width
            // Edge case when mouse leaves the window.
            || x as f32 >= end_of_grid
    {
        Side::Right
    } else {
        Side::Left
    }
}

///Copied (with modifications) from alacritty/src/event.rs > Mouse::point()
///Position is a pane-relative position. That means the top left corner of the mouse
///Region should be (0,0)
fn grid_cell(pos: &PaneRelativePos, cur_size: SizeInfo, display_offset: usize) -> Point {
    let pos = pos.0;
    let col = pos.x() / cur_size.cell_width(); //TODO: underflow...
    let col = min(GridCol(col as usize), cur_size.last_column());

    let line = pos.y() / cur_size.cell_height();
    let line = min(line as i32, cur_size.bottommost_line().0);

    //when clicking, need to ADD to get to the top left cell
    //e.g. total_lines - viewport_height, THEN subtract display offset
    //0 -> total_lines - viewport_height - display_offset + mouse_line

    Point::new(GridLine(line - display_offset as i32), col)
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

        let cur_size = alacritty_terminal::term::SizeInfo::new(
            term_width,
            term_height,
            cell_width,
            line_height,
            0.,
            0.,
            false,
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
