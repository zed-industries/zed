use alacritty_terminal::{
    grid::{Dimensions, GridIterator, Indexed, Scroll},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    selection::{Selection, SelectionRange, SelectionType},
    sync::FairMutex,
    term::{
        cell::{Cell, Flags},
        SizeInfo,
    },
    Term,
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
    SizeConstraint, TextLayoutCache, WeakModelHandle, WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use theme::TerminalStyle;
use util::ResultExt;

use std::{cmp::min, ops::Range, rc::Rc, sync::Arc};
use std::{fmt::Debug, ops::Sub};

use crate::{
    color_translation::convert_color,
    connection::{TerminalConnection, ZedListener},
    Terminal,
};

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

///Used to display the grid as passed to Alacritty and the TTY.
///Useful for debugging inconsistencies between behavior and display
#[cfg(debug_assertions)]
const DEBUG_GRID: bool = false;

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalEl {
    connection: WeakModelHandle<TerminalConnection>,
    view: WeakViewHandle<Terminal>,
    modal: bool,
}

///New type pattern so I don't mix these two up
struct CellWidth(f32);
struct LineHeight(f32);

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
    cur_size: SizeInfo,
    terminal: Arc<FairMutex<Term<ZedListener>>>,
    selection_color: Color,
}

impl TerminalEl {
    pub fn new(
        view: WeakViewHandle<Terminal>,
        connection: WeakModelHandle<TerminalConnection>,
        modal: bool,
    ) -> TerminalEl {
        TerminalEl {
            view,
            connection,
            modal,
        }
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
        //Settings immutably borrows cx here for the settings and font cache
        //and we need to modify the cx to resize the terminal. So instead of
        //storing Settings or the font_cache(), we toss them ASAP and then reborrow later
        let text_style = make_text_style(cx.font_cache(), cx.global::<Settings>());
        let line_height = LineHeight(cx.font_cache().line_height(text_style.font_size));
        let cell_width = CellWidth(
            cx.font_cache()
                .em_advance(text_style.font_id, text_style.font_size),
        );
        let connection_handle = self.connection.upgrade(cx).unwrap();

        //Tell the view our new size. Requires a mutable borrow of cx and the view
        let cur_size = make_new_size(constraint, &cell_width, &line_height);
        //Note that set_size locks and mutates the terminal.
        connection_handle.update(cx.app, |connection, _| connection.set_size(cur_size));

        let (selection_color, terminal_theme) = {
            let theme = &(cx.global::<Settings>()).theme;
            (theme.editor.selection.selection, &theme.terminal)
        };

        let terminal_mutex = connection_handle.read(cx).term.clone();
        let term = terminal_mutex.lock();
        let grid = term.grid();
        let cursor_point = grid.cursor.point;
        let cursor_text = grid[cursor_point.line][cursor_point.column].c.to_string();

        let content = term.renderable_content();

        let layout_lines = layout_lines(
            content.display_iter,
            &text_style,
            terminal_theme,
            cx.text_layout_cache,
            self.modal,
            content.selection,
        );

        let block_text = cx.text_layout_cache.layout_str(
            &cursor_text,
            text_style.font_size,
            &[(
                cursor_text.len(),
                RunStyle {
                    font_id: text_style.font_id,
                    color: terminal_theme.colors.background,
                    underline: Default::default(),
                },
            )],
        );

        let cursor = get_cursor_shape(
            content.cursor.point.line.0 as usize,
            content.cursor.point.column.0 as usize,
            content.display_offset,
            &line_height,
            &cell_width,
            cur_size.total_lines(),
            &block_text,
        )
        .map(move |(cursor_position, block_width)| {
            let block_width = if block_width != 0.0 {
                block_width
            } else {
                cell_width.0
            };

            Cursor::new(
                cursor_position,
                block_width,
                line_height.0,
                terminal_theme.colors.cursor,
                CursorShape::Block,
                Some(block_text.clone()),
            )
        });
        drop(term);

        let background_color = if self.modal {
            terminal_theme.colors.modal_background
        } else {
            terminal_theme.colors.background
        };

        (
            constraint.max,
            LayoutState {
                layout_lines,
                line_height,
                em_width: cell_width,
                cursor,
                cur_size,
                background_color,
                terminal: terminal_mutex,
                selection_color,
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
        //Setup element stuff
        let clip_bounds = Some(visible_bounds);

        cx.paint_layer(clip_bounds, |cx| {
            let cur_size = layout.cur_size.clone();
            let origin = bounds.origin() + vec2f(layout.em_width.0, 0.);

            //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
            attach_mouse_handlers(
                origin,
                cur_size,
                self.view.id(),
                &layout.terminal,
                visible_bounds,
                cx,
            );

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

            #[cfg(debug_assertions)]
            if DEBUG_GRID {
                cx.paint_layer(clip_bounds, |cx| {
                    draw_debug_grid(bounds, layout, cx);
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

                //TODO Talk to keith about how to catch events emitted from an element.
                if let Some(view) = self.view.upgrade(cx.app) {
                    view.update(cx.app, |view, cx| view.clear_bel(cx))
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

///Configures a text style from the current settings.
fn make_text_style(font_cache: &FontCache, settings: &Settings) -> TextStyle {
    // Pull the font family from settings properly overriding
    let family_id = settings
        .terminal_overrides
        .font_family
        .as_ref()
        .and_then(|family_name| font_cache.load_family(&[family_name]).log_err())
        .or_else(|| {
            settings
                .terminal_defaults
                .font_family
                .as_ref()
                .and_then(|family_name| font_cache.load_family(&[family_name]).log_err())
        })
        .unwrap_or(settings.buffer_font_family);

    TextStyle {
        color: settings.theme.editor.text_color,
        font_family_id: family_id,
        font_family_name: font_cache.family_name(family_id).unwrap(),
        font_id: font_cache
            .select_font(family_id, &Default::default())
            .unwrap(),
        font_size: settings
            .terminal_overrides
            .font_size
            .or(settings.terminal_defaults.font_size)
            .unwrap_or(settings.buffer_font_size),
        font_properties: Default::default(),
        underline: Default::default(),
    }
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

fn attach_mouse_handlers(
    origin: Vector2F,
    cur_size: SizeInfo,
    view_id: usize,
    terminal_mutex: &Arc<FairMutex<Term<ZedListener>>>,
    visible_bounds: RectF,
    cx: &mut PaintContext,
) {
    let click_mutex = terminal_mutex.clone();
    let drag_mutex = terminal_mutex.clone();
    let mouse_down_mutex = terminal_mutex.clone();

    cx.scene.push_mouse_region(MouseRegion {
        view_id,
        mouse_down: Some(Rc::new(move |pos, _| {
            let mut term = mouse_down_mutex.lock();
            let (point, side) = mouse_to_cell_data(
                pos,
                origin,
                cur_size,
                term.renderable_content().display_offset,
            );
            term.selection = Some(Selection::new(SelectionType::Simple, point, side))
        })),
        click: Some(Rc::new(move |pos, click_count, cx| {
            let mut term = click_mutex.lock();

            let (point, side) = mouse_to_cell_data(
                pos,
                origin,
                cur_size,
                term.renderable_content().display_offset,
            );

            let selection_type = match click_count {
                0 => return, //This is a release
                1 => Some(SelectionType::Simple),
                2 => Some(SelectionType::Semantic),
                3 => Some(SelectionType::Lines),
                _ => None,
            };

            let selection =
                selection_type.map(|selection_type| Selection::new(selection_type, point, side));

            term.selection = selection;
            cx.focus_parent_view();
            cx.notify();
        })),
        bounds: visible_bounds,
        drag: Some(Rc::new(move |_delta, pos, cx| {
            let mut term = drag_mutex.lock();

            let (point, side) = mouse_to_cell_data(
                pos,
                origin,
                cur_size,
                term.renderable_content().display_offset,
            );

            if let Some(mut selection) = term.selection.take() {
                selection.update(point, side);
                term.selection = Some(selection);
            }

            cx.notify();
        })),
        ..Default::default()
    });
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

///Draws the grid as Alacritty sees it. Useful for checking if there is an inconsistency between
///Display and conceptual grid.
#[cfg(debug_assertions)]
fn draw_debug_grid(bounds: RectF, layout: &mut LayoutState, cx: &mut PaintContext) {
    let width = layout.cur_size.width();
    let height = layout.cur_size.height();
    //Alacritty uses 'as usize', so shall we.
    for col in 0..(width / layout.em_width.0).round() as usize {
        cx.scene.push_quad(Quad {
            bounds: RectF::new(
                bounds.origin() + vec2f((col + 1) as f32 * layout.em_width.0, 0.),
                vec2f(1., height),
            ),
            background: Some(Color::green()),
            border: Default::default(),
            corner_radius: 0.,
        });
    }
    for row in 0..((height / layout.line_height.0) + 1.0).round() as usize {
        cx.scene.push_quad(Quad {
            bounds: RectF::new(
                bounds.origin() + vec2f(layout.em_width.0, row as f32 * layout.line_height.0),
                vec2f(width, 1.),
            ),
            background: Some(Color::green()),
            border: Default::default(),
            corner_radius: 0.,
        });
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
