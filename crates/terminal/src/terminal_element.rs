use alacritty_terminal::{
    grid::{Dimensions, GridIterator, Indexed},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    term::{
        cell::{Cell, Flags},
        SizeInfo,
    },
};
use editor::{Cursor, CursorShape};
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
use std::{cmp::min, rc::Rc};
use theme::TerminalStyle;

use crate::{
    color_translation::convert_color, gpui_func_tools::paint_layer, Input, ScrollTerminal, Terminal,
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

///Helper types so I don't mix these two up
struct CellWidth(f32);
struct LineHeight(f32);

#[derive(Clone, Debug, Default)]
struct LayoutCell {
    point: Point<i32, i32>,
    text: Line,
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
    cells: Vec<(Point<i32, i32>, Line)>,
    background_rects: Vec<(RectF, Color)>, //Vec index == Line index for the LineSpan
    line_height: LineHeight,
    em_width: CellWidth,
    cursor: Option<Cursor>,
    background_color: Color,
    cur_size: SizeInfo,
    display_offset: usize,
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
        let terminal_theme = &(cx.global::<Settings>()).theme.terminal;
        let term = view_handle.read(cx).term.lock();

        let grid = term.grid();
        let cursor_point = grid.cursor.point;
        let cursor_text = grid[cursor_point.line][cursor_point.column].c.to_string();

        let content = term.renderable_content();

        let layout_cells = layout_cells(
            content.display_iter,
            &text_style,
            terminal_theme,
            cx.text_layout_cache,
        );

        let cells = layout_cells
            .iter()
            .map(|c| (c.point, c.text.clone()))
            .collect::<Vec<(Point<i32, i32>, Line)>>();
        let background_rects = layout_cells
            .iter()
            .map(|cell| {
                (
                    RectF::new(
                        vec2f(
                            cell.point.column as f32 * cell_width.0,
                            cell.point.line as f32 * line_height.0,
                        ),
                        vec2f(cell_width.0, line_height.0),
                    ),
                    cell.background_color,
                )
            })
            .collect::<Vec<(RectF, Color)>>();

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

        (
            constraint.max,
            LayoutState {
                cells,
                line_height,
                em_width: cell_width,
                cursor,
                cur_size,
                background_rects,
                background_color: terminal_theme.background,
                display_offset: content.display_offset,
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
            cx.scene.push_mouse_region(MouseRegion {
                view_id: self.view.id(),
                mouse_down: Some(Rc::new(move |pos, cx| {
                    let point = grid_cell(pos, cur_size, display_offset);
                    let side = cell_side(cur_size, pos.x() as usize);

                    //One problem is we need a terminal
                    //Second problem is that we need # of clicks
                    //Third problem is that dragging reports deltas, and we need locations.
                    //Fourth (minor) is need to render the selection

                    // if single_click {
                    //   terminal.selection = Some(Selection::new(SelectionType::Simple, point, side))
                    // } else if double_click {
                    //   terminal.selection = Some(Selection::new(SelectionType::Semantic, point, side))
                    // } else if triple_click {
                    //   terminal.selection = Some(Selection::new(SelectionType::Lines, point, side))
                    // }

                    cx.focus_parent_view()
                })),
                bounds: visible_bounds,
                drag: Some(Rc::new(|delta, cx| {
                    //Calculate new point from delta
                    //terminal.selection.update(point, side)
                })),
                ..Default::default()
            });

            let origin = bounds.origin() + vec2f(layout.em_width.0, 0.);

            paint_layer(cx, clip_bounds, |cx| {
                //Start with a background color
                cx.scene.push_quad(Quad {
                    bounds: RectF::new(bounds.origin(), bounds.size()),
                    background: Some(layout.background_color),
                    border: Default::default(),
                    corner_radius: 0.,
                });

                //Draw cell backgrounds
                for background_rect in &layout.background_rects {
                    let new_origin = origin + background_rect.0.origin();
                    cx.scene.push_quad(Quad {
                        bounds: RectF::new(new_origin, background_rect.0.size()),
                        background: Some(background_rect.1),
                        border: Default::default(),
                        corner_radius: 0.,
                    })
                }
            });

            //Draw text
            paint_layer(cx, clip_bounds, |cx| {
                for (point, cell) in &layout.cells {
                    let cell_origin = vec2f(
                        origin.x() + point.column as f32 * layout.em_width.0,
                        origin.y() + point.line as f32 * layout.line_height.0,
                    );
                    cell.paint(cell_origin, visible_bounds, layout.line_height.0, cx);
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

/*
Mouse moved -> WindowEvent::CursorMoved
mouse press -> WindowEvent::MouseInput
update_selection_scrolling


copy_selection
start_selection
toggle_selection
update_selection
clear_selection
 */

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

fn layout_cells(
    grid: GridIterator<Cell>,
    text_style: &TextStyle,
    terminal_theme: &TerminalStyle,
    text_layout_cache: &TextLayoutCache,
) -> Vec<LayoutCell> {
    let mut line_count: i32 = 0;
    let lines = grid.group_by(|i| i.point.line);
    lines
        .into_iter()
        .map(|(_, line)| {
            line_count += 1;
            line.map(|indexed_cell| {
                let cell_text = &indexed_cell.c.to_string();

                let cell_style = cell_style(&indexed_cell, terminal_theme, text_style);

                let layout_cell = text_layout_cache.layout_str(
                    cell_text,
                    text_style.font_size,
                    &[(cell_text.len(), cell_style)],
                );
                LayoutCell::new(
                    Point::new(line_count - 1, indexed_cell.point.column.0 as i32),
                    layout_cell,
                    convert_color(&indexed_cell.bg, terminal_theme),
                )
            })
            .collect::<Vec<LayoutCell>>()
        })
        .flatten()
        .collect::<Vec<LayoutCell>>()
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
fn cell_side(cur_size: SizeInfo, x: usize) -> Side {
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
fn grid_cell(pos: Vector2F, cur_size: SizeInfo, display_offset: usize) -> Point {
    let col = pos.x() - cur_size.cell_width() / cur_size.cell_width(); //TODO: underflow...
    let col = min(GridCol(col as usize), cur_size.last_column());

    let line = pos.y() - cur_size.padding_y() / cur_size.cell_height();
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
