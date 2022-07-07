use alacritty_terminal::{
    grid::{Dimensions, GridIterator, Indexed},
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
    Event, FontCache, MouseRegion, PaintContext, Quad, SizeConstraint, TextLayoutCache,
    WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use std::{cmp::min, ops::Range, rc::Rc, sync::Arc};
use std::{fmt::Debug, ops::Sub};
use theme::TerminalStyle;

use crate::{
    color_translation::convert_color, gpui_func_tools::paint_layer, Input, ScrollTerminal,
    Terminal, ZedListener,
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
pub struct TerminalEl {
    view: WeakViewHandle<Terminal>,
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
    display_offset: usize,
    terminal: Arc<FairMutex<Term<ZedListener>>>,
    selection_color: Color,
}

impl TerminalEl {
    pub fn new(view: WeakViewHandle<Terminal>) -> TerminalEl {
        TerminalEl { view }
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
        let view_handle = self.view.upgrade(cx).unwrap();

        //Tell the view our new size. Requires a mutable borrow of cx and the view
        let cur_size = make_new_size(constraint, &cell_width, &line_height);
        //Note that set_size locks and mutates the terminal.
        //TODO: Would be nice to lock once for the whole of layout
        view_handle.update(cx.app, |view, _cx| view.set_size(cur_size));

        //Now that we're done with the mutable portion, grab the immutable settings and view again
        let (selection_color, terminal_theme) = {
            let theme = &(cx.global::<Settings>()).theme;
            (theme.editor.selection.selection, &theme.terminal)
        };
        let terminal_mutex = view_handle.read(cx).term.clone();

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
            content.selection,
        );

        let block_text = cx.text_layout_cache.layout_str(
            &cursor_text,
            text_style.font_size,
            &[(
                cursor_text.len(),
                RunStyle {
                    font_id: text_style.font_id,
                    color: terminal_theme.background,
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
                terminal_theme.cursor,
                CursorShape::Block,
                Some(block_text.clone()),
            )
        });
        let display_offset = content.display_offset;
        drop(term);

        (
            constraint.max,
            LayoutState {
                layout_lines,
                line_height,
                em_width: cell_width,
                cursor,
                cur_size,
                background_color: terminal_theme.background,
                display_offset,
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

        paint_layer(cx, clip_bounds, |cx| {
            //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse

            /*
            To set a selection,
            set the selection variable on the terminal

            CLICK:
            Get the grid point associated with this mouse click
            And the side????? - TODO - algorithm for calculating this in Processor::cell_side
            On single left click -> Clear selection, start empty selection
            On double left click -> start semantic selection
            On double triple click -> start line selection

            MOUSE MOVED:
            Find the new cell the mouse is over
            Update the selection by calling terminal.selection.update()
            */
            let cur_size = layout.cur_size.clone();
            let display_offset = layout.display_offset.clone();
            let terminal_mutex = layout.terminal.clone();
            let origin = bounds.origin() + vec2f(layout.em_width.0, 0.);

            //TODO: Better way of doing this?
            let mutex1 = terminal_mutex.clone();
            let mutex2 = terminal_mutex.clone();

            cx.scene.push_mouse_region(MouseRegion {
                view_id: self.view.id(),
                click: Some(Rc::new(move |pos, click_count, cx| {
                    let (point, side) = mouse_to_cell_data(pos, origin, cur_size, display_offset);

                    let selection_type = match click_count {
                        0 => return, //This is a release
                        1 => Some(SelectionType::Simple),
                        2 => Some(SelectionType::Semantic),
                        3 => Some(SelectionType::Lines),
                        _ => None,
                    };

                    let selection = selection_type
                        .map(|selection_type| Selection::new(selection_type, point, side));

                    let mut term = mutex1.lock();
                    term.selection = selection;
                    cx.focus_parent_view()
                })),
                bounds: visible_bounds,
                drag: Some(Rc::new(move |_delta, pos, cx| {
                    let (point, side) = mouse_to_cell_data(pos, origin, cur_size, display_offset);

                    let mut term = mutex2.lock();
                    if let Some(mut selection) = term.selection.take() {
                        selection.update(point, side);
                        term.selection = Some(selection);
                    } else {
                        term.selection = Some(Selection::new(SelectionType::Simple, point, side));
                    }
                    cx.notify();
                })),
                ..Default::default()
            });

            paint_layer(cx, clip_bounds, |cx| {
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
                            origin.x() + layout_cell.point.column as f32 * layout.em_width.0,
                            origin.y() + layout_cell.point.line as f32 * layout.line_height.0,
                        );
                        let size = vec2f(layout.em_width.0, layout.line_height.0);

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
            paint_layer(cx, clip_bounds, |cx| {
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
                                //TODO: Why -1? I know switch from count to index... but where...
                                + line.cells[range.end - 1].point.column as f32 * layout.em_width.0
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

            //Draw text
            paint_layer(cx, clip_bounds, |cx| {
                for layout_line in &layout.layout_lines {
                    for layout_cell in &layout_line.cells {
                        let point = layout_cell.point;

                        //Don't actually know the start_x for a line, until here:
                        let cell_origin = vec2f(
                            origin.x() + point.column as f32 * layout.em_width.0,
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
                paint_layer(cx, clip_bounds, |cx| {
                    cursor.paint(origin, cx);
                })
            }

            #[cfg(debug_assertions)]
            if DEBUG_GRID {
                paint_layer(cx, clip_bounds, |cx| {
                    draw_debug_grid(bounds, layout, cx);
                });
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
            Event::ScrollWheel {
                delta, position, ..
            } => visible_bounds
                .contains_point(*position)
                .then(|| {
                    let vertical_scroll =
                        (delta.y() / layout.line_height.0) * ALACRITTY_SCROLL_MULTIPLIER;
                    cx.dispatch_action(ScrollTerminal(vertical_scroll.round() as i32));
                })
                .is_some(),
            Event::KeyDown {
                input: Some(input), ..
            } => cx
                .is_parent_view_focused()
                .then(|| {
                    cx.dispatch_action(Input(input.to_string()));
                })
                .is_some(),
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

fn mouse_to_cell_data(
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
    TextStyle {
        color: settings.theme.editor.text_color,
        font_family_id: settings.buffer_font_family,
        font_family_name: font_cache.family_name(settings.buffer_font_family).unwrap(),
        font_id: font_cache
            .select_font(settings.buffer_font_family, &Default::default())
            .unwrap(),
        font_size: settings.buffer_font_size,
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

//Let's say that calculating the display is correct, that means that either calculating the highlight ranges is incorrect
//OR calculating the click ranges is incorrect

fn layout_lines(
    grid: GridIterator<Cell>,
    text_style: &TextStyle,
    terminal_theme: &TerminalStyle,
    text_layout_cache: &TextLayoutCache,
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
                        let mut range = highlighted_range.take().unwrap_or(x_index..x_index + 1);
                        range.end = range.end.max(x_index + 1);
                        highlighted_range = Some(range);
                    }

                    let cell_text = &indexed_cell.c.to_string();

                    let cell_style = cell_style(&indexed_cell, terminal_theme, text_style);

                    //This is where we might be able to get better performance
                    let layout_cell = text_layout_cache.layout_str(
                        cell_text,
                        text_style.font_size,
                        &[(cell_text.len(), cell_style)],
                    );

                    LayoutCell::new(
                        Point::new(line_index as i32, indexed_cell.point.column.0 as i32),
                        layout_cell,
                        convert_color(&indexed_cell.bg, terminal_theme),
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
fn cell_style(indexed: &Indexed<&Cell>, style: &TerminalStyle, text_style: &TextStyle) -> RunStyle {
    let flags = indexed.cell.flags;
    let fg = convert_color(&indexed.cell.fg, style);

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
    let line = min(line as usize, cur_size.bottommost_line().0 as usize);

    Point::new(GridLine((line - display_offset) as i32), col)
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
