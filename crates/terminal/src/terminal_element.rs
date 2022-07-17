mod terminal_layout_context;

use alacritty_terminal::{
    grid::{Dimensions, GridIterator, Indexed, Scroll},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    selection::{Selection, SelectionRange, SelectionType},
    term::{
        cell::{Cell, Flags},
        SizeInfo,
    },
    Grid,
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
    Event, FontCache, KeyDownEvent, MouseRegion, PaintContext, Quad, ScrollWheelEvent,
    SizeConstraint, TextLayoutCache, WeakModelHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use theme::TerminalStyle;
use util::ResultExt;

use std::{cmp::min, ops::Range, rc::Rc};
use std::{fmt::Debug, ops::Sub};

use crate::{color_translation::convert_color, connection::TerminalConnection};

use self::terminal_layout_context::TerminalLayoutContext;

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalEl {
    connection: WeakModelHandle<TerminalConnection>,
    view_id: usize,
    modal: bool,
}

///New type pattern so I don't mix these two up
pub struct CellWidth(f32);
pub struct LineHeight(f32);

struct LayoutLine {
    cells: Vec<LayoutCell>,
    highlighted_range: Option<Range<usize>>,
}

///New type pattern to ensure that we use adjusted mouse positions throughout the code base, rather than
struct PaneRelativePos(Vector2F);

///Functionally the constructor for the PaneRelativePos type, mutates the mouse_position
fn relative_pos(mouse_position: Vector2F, origin: Vector2F) -> PaneRelativePos {
    PaneRelativePos(mouse_position.sub(origin)) //Avoid the extra allocation by mutating
}

#[derive(Clone, Debug, Default)]
struct LayoutCell {
    point: Point<i32, i32>,
    text: Line, //NOTE TO SELF THIS IS BAD PERFORMANCE RN!
    background_color: Color,
}

impl LayoutCell {
    fn new(point: Point<i32, i32>, text: Line, background_color: Color) -> LayoutCell {
        LayoutCell {
            point,
            text,
            background_color,
        }
    }
}

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    layout_lines: Vec<LayoutLine>,
    line_height: LineHeight,
    em_width: CellWidth,
    cursor: Option<Cursor>,
    background_color: Color,
    selection_color: Color,
}

impl TerminalEl {
    pub fn new(
        view_id: usize,
        connection: WeakModelHandle<TerminalConnection>,
        modal: bool,
    ) -> TerminalEl {
        TerminalEl {
            view_id,
            connection,
            modal,
        }
    }

    fn attach_mouse_handlers(
        &self,
        origin: Vector2F,
        view_id: usize,
        visible_bounds: RectF,
        cx: &mut PaintContext,
    ) {
        let mouse_down_connection = self.connection.clone();
        let click_connection = self.connection.clone();
        let drag_connection = self.connection.clone();
        cx.scene.push_mouse_region(MouseRegion {
            view_id,
            mouse_down: Some(Rc::new(move |pos, cx| {
                if let Some(conn_handle) = mouse_down_connection.upgrade(cx.app) {
                    conn_handle.update(cx.app, |conn, _cx| {
                        let mut term = conn.term.lock();
                        let (point, side) = mouse_to_cell_data(
                            pos,
                            origin,
                            conn.cur_size,
                            term.renderable_content().display_offset,
                        );
                        term.selection = Some(Selection::new(SelectionType::Simple, point, side))
                    });
                }
            })),
            click: Some(Rc::new(move |pos, click_count, cx| {
                cx.focus_parent_view();
                if let Some(conn_handle) = click_connection.upgrade(cx.app) {
                    conn_handle.update(cx.app, |conn, cx| {
                        let mut term = conn.term.lock();

                        let (point, side) = mouse_to_cell_data(
                            pos,
                            origin,
                            conn.cur_size,
                            term.renderable_content().display_offset,
                        );

                        let selection_type = match click_count {
                            0 => return, //This is a release
                            1 => Some(SelectionType::Simple),
                            2 => Some(SelectionType::Semantic),
                            3 => Some(SelectionType::Lines),
                            _ => None,
                        };

                        let selection = selection_type
                            .map(|selection_type| Selection::new(selection_type, point, side));

                        term.selection = selection;

                        cx.notify();
                    });
                }
            })),
            bounds: visible_bounds,
            drag: Some(Rc::new(move |_delta, pos, cx| {
                if let Some(conn_handle) = drag_connection.upgrade(cx.app) {
                    conn_handle.update(cx.app, |conn, cx| {
                        let mut term = conn.term.lock();

                        let (point, side) = mouse_to_cell_data(
                            pos,
                            origin,
                            conn.cur_size,
                            term.renderable_content().display_offset,
                        );

                        if let Some(mut selection) = term.selection.take() {
                            selection.update(point, side);
                            term.selection = Some(selection);
                        }

                        cx.notify()
                    });
                }
            })),
            ..Default::default()
        });
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
        let tcx = TerminalLayoutContext::new(cx.global::<Settings>(), &cx.font_cache());

        let term = {
            let connection = self.connection.upgrade(cx).unwrap().read(cx);
            //This locks the terminal, so resize it first.
            connection.set_size(make_new_size(constraint, &tcx.cell_width, &tcx.line_height));
            connection.term.lock()
        };

        let content = term.renderable_content();

        /*
        * TODO for layouts:
        * - Refactor this whole process to produce 'text cells', 'background rects', and 'selections' which know
        *   how to paint themselves
        * - Rather than doing everything per cell, map each cell into a tuple and then unzip the streams
        * - For efficiency:
        *  - filter out all background colored background rects
        *  - filter out all text cells which just contain ' '
        *  - Smoosh together rectangles on same line

        */
        //Layout grid cells
        let layout_lines = layout_lines(
            content.display_iter,
            &tcx.text_style,
            tcx.terminal_theme,
            cx.text_layout_cache,
            self.modal,
            content.selection,
        );

        //Layout cursor
        let cursor = layout_cursor(
            term.grid(),
            cx.text_layout_cache,
            &tcx,
            content.cursor.point,
            content.display_offset,
            constraint,
        );

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
                layout_lines,
                line_height: tcx.line_height,
                em_width: tcx.cell_width,
                cursor,
                background_color,
                selection_color: tcx.selection_color,
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
            self.attach_mouse_handlers(origin, self.view_id, visible_bounds, cx);

            cx.paint_layer(clip_bounds, |cx| {
                //Start with a background color
                cx.scene.push_quad(Quad {
                    bounds: RectF::new(bounds.origin(), bounds.size()),
                    background: Some(layout.background_color),
                    border: Default::default(),
                    corner_radius: 0.,
                });

                //Draw cell backgrounds
                for layout_line in &layout.layout_lines {
                    for layout_cell in &layout_line.cells {
                        let position = vec2f(
                            (origin.x() + layout_cell.point.column as f32 * layout.em_width.0)
                                .floor(),
                            origin.y() + layout_cell.point.line as f32 * layout.line_height.0,
                        );
                        let size = vec2f(layout.em_width.0.ceil(), layout.line_height.0);

                        cx.scene.push_quad(Quad {
                            bounds: RectF::new(position, size),
                            background: Some(layout_cell.background_color),
                            border: Default::default(),
                            corner_radius: 0.,
                        })
                    }
                }
            });

            //Draw Selection
            cx.paint_layer(clip_bounds, |cx| {
                let mut highlight_y = None;
                let highlight_lines = layout
                    .layout_lines
                    .iter()
                    .filter_map(|line| {
                        if let Some(range) = &line.highlighted_range {
                            if let None = highlight_y {
                                highlight_y = Some(
                                    origin.y()
                                        + line.cells[0].point.line as f32 * layout.line_height.0,
                                );
                            }
                            let start_x = origin.x()
                                + line.cells[range.start].point.column as f32 * layout.em_width.0;
                            let end_x = origin.x()
                                + line.cells[range.end].point.column as f32 * layout.em_width.0
                                + layout.em_width.0;

                            return Some(HighlightedRangeLine { start_x, end_x });
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<HighlightedRangeLine>>();

                if let Some(y) = highlight_y {
                    let hr = HighlightedRange {
                        start_y: y, //Need to change this
                        line_height: layout.line_height.0,
                        lines: highlight_lines,
                        color: layout.selection_color,
                        //Copied from editor. TODO: move to theme or something
                        corner_radius: 0.15 * layout.line_height.0,
                    };
                    hr.paint(bounds, cx.scene);
                }
            });

            cx.paint_layer(clip_bounds, |cx| {
                for layout_line in &layout.layout_lines {
                    for layout_cell in &layout_line.cells {
                        let point = layout_cell.point;

                        //Don't actually know the start_x for a line, until here:
                        let cell_origin = vec2f(
                            (origin.x() + point.column as f32 * layout.em_width.0).floor(),
                            origin.y() + point.line as f32 * layout.line_height.0,
                        );

                        layout_cell.text.paint(
                            cell_origin,
                            visible_bounds,
                            layout.line_height.0,
                            cx,
                        );
                    }
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

                    if let Some(connection) = self.connection.upgrade(cx.app) {
                        connection.update(cx.app, |connection, _| {
                            connection
                                .term
                                .lock()
                                .scroll_display(Scroll::Delta(vertical_scroll.round() as i32));
                        })
                    }
                })
                .is_some(),
            Event::KeyDown(KeyDownEvent { keystroke, .. }) => {
                if !cx.is_parent_view_focused() {
                    return false;
                }

                self.connection
                    .upgrade(cx.app)
                    .map(|connection| {
                        connection
                            .update(cx.app, |connection, _| connection.try_keystroke(keystroke))
                    })
                    .unwrap_or(false)
            }
            _ => false,
        }
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

fn layout_cursor(
    grid: &Grid<Cell>,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutContext,
    cursor_point: Point,
    display_offset: usize,
    constraint: SizeConstraint,
) -> Option<Cursor> {
    let cursor_text = layout_cursor_text(grid, text_layout_cache, tcx);
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
    grid: &Grid<Cell>,
    text_layout_cache: &TextLayoutCache,
    tcx: &TerminalLayoutContext,
) -> Line {
    let cursor_point = grid.cursor.point;
    let cursor_text = grid[cursor_point.line][cursor_point.column].c.to_string();

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

fn layout_lines(
    grid: GridIterator<Cell>,
    text_style: &TextStyle,
    terminal_theme: &TerminalStyle,
    text_layout_cache: &TextLayoutCache,
    modal: bool,
    selection_range: Option<SelectionRange>,
) -> Vec<LayoutLine> {
    let lines = grid.group_by(|i| i.point.line);
    lines
        .into_iter()
        .enumerate()
        .map(|(line_index, (_, line))| {
            let mut highlighted_range = None;
            let cells = line
                .enumerate()
                .map(|(x_index, indexed_cell)| {
                    if selection_range
                        .map(|range| range.contains(indexed_cell.point))
                        .unwrap_or(false)
                    {
                        let mut range = highlighted_range.take().unwrap_or(x_index..x_index);
                        range.end = range.end.max(x_index);
                        highlighted_range = Some(range);
                    }

                    let cell_text = &indexed_cell.c.to_string();

                    let cell_style = cell_style(&indexed_cell, terminal_theme, text_style, modal);

                    //This is where we might be able to get better performance
                    let layout_cell = text_layout_cache.layout_str(
                        cell_text,
                        text_style.font_size,
                        &[(cell_text.len(), cell_style)],
                    );

                    LayoutCell::new(
                        Point::new(line_index as i32, indexed_cell.point.column.0 as i32),
                        layout_cell,
                        convert_color(&indexed_cell.bg, &terminal_theme.colors, modal),
                    )
                })
                .collect::<Vec<LayoutCell>>();

            LayoutLine {
                cells,
                highlighted_range,
            }
        })
        .collect::<Vec<LayoutLine>>()
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

    #[test]
    fn test_mouse_to_selection_off_edge() {
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
