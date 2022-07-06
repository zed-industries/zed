use alacritty_terminal::{
    ansi::Color as AnsiColor,
    grid::{Dimensions, GridIterator, Indexed},
    index::Point,
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
    Event, FontCache, KeyDownEvent, MouseRegion, PaintContext, Quad, ScrollWheelEvent,
    SizeConstraint, TextLayoutCache, WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use std::rc::Rc;
use theme::TerminalStyle;

use crate::{gpui_func_tools::paint_layer, Input, ScrollTerminal, Terminal};

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
            cx.scene.push_mouse_region(MouseRegion {
                view_id: self.view.id(),
                mouse_down: Some(Rc::new(|_, cx| cx.focus_parent_view())),
                bounds: visible_bounds,
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
            Event::ScrollWheel(ScrollWheelEvent {
                delta, position, ..
            }) => visible_bounds
                .contains_point(*position)
                .then(|| {
                    let vertical_scroll =
                        (delta.y() / layout.line_height.0) * ALACRITTY_SCROLL_MULTIPLIER;
                    cx.dispatch_action(ScrollTerminal(vertical_scroll.round() as i32));
                })
                .is_some(),
            Event::KeyDown(KeyDownEvent {
                input: Some(input), ..
            }) => cx
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

///Converts a 2, 8, or 24 bit color ANSI color to the GPUI equivalent
fn convert_color(alac_color: &AnsiColor, style: &TerminalStyle) -> Color {
    match alac_color {
        //Named and theme defined colors
        alacritty_terminal::ansi::Color::Named(n) => match n {
            alacritty_terminal::ansi::NamedColor::Black => style.black,
            alacritty_terminal::ansi::NamedColor::Red => style.red,
            alacritty_terminal::ansi::NamedColor::Green => style.green,
            alacritty_terminal::ansi::NamedColor::Yellow => style.yellow,
            alacritty_terminal::ansi::NamedColor::Blue => style.blue,
            alacritty_terminal::ansi::NamedColor::Magenta => style.magenta,
            alacritty_terminal::ansi::NamedColor::Cyan => style.cyan,
            alacritty_terminal::ansi::NamedColor::White => style.white,
            alacritty_terminal::ansi::NamedColor::BrightBlack => style.bright_black,
            alacritty_terminal::ansi::NamedColor::BrightRed => style.bright_red,
            alacritty_terminal::ansi::NamedColor::BrightGreen => style.bright_green,
            alacritty_terminal::ansi::NamedColor::BrightYellow => style.bright_yellow,
            alacritty_terminal::ansi::NamedColor::BrightBlue => style.bright_blue,
            alacritty_terminal::ansi::NamedColor::BrightMagenta => style.bright_magenta,
            alacritty_terminal::ansi::NamedColor::BrightCyan => style.bright_cyan,
            alacritty_terminal::ansi::NamedColor::BrightWhite => style.bright_white,
            alacritty_terminal::ansi::NamedColor::Foreground => style.foreground,
            alacritty_terminal::ansi::NamedColor::Background => style.background,
            alacritty_terminal::ansi::NamedColor::Cursor => style.cursor,
            alacritty_terminal::ansi::NamedColor::DimBlack => style.dim_black,
            alacritty_terminal::ansi::NamedColor::DimRed => style.dim_red,
            alacritty_terminal::ansi::NamedColor::DimGreen => style.dim_green,
            alacritty_terminal::ansi::NamedColor::DimYellow => style.dim_yellow,
            alacritty_terminal::ansi::NamedColor::DimBlue => style.dim_blue,
            alacritty_terminal::ansi::NamedColor::DimMagenta => style.dim_magenta,
            alacritty_terminal::ansi::NamedColor::DimCyan => style.dim_cyan,
            alacritty_terminal::ansi::NamedColor::DimWhite => style.dim_white,
            alacritty_terminal::ansi::NamedColor::BrightForeground => style.bright_foreground,
            alacritty_terminal::ansi::NamedColor::DimForeground => style.dim_foreground,
        },
        //'True' colors
        alacritty_terminal::ansi::Color::Spec(rgb) => Color::new(rgb.r, rgb.g, rgb.b, u8::MAX),
        //8 bit, indexed colors
        alacritty_terminal::ansi::Color::Indexed(i) => get_color_at_index(i, style),
    }
}

///Converts an 8 bit ANSI color to it's GPUI equivalent.
pub fn get_color_at_index(index: &u8, style: &TerminalStyle) -> Color {
    match index {
        //0-15 are the same as the named colors above
        0 => style.black,
        1 => style.red,
        2 => style.green,
        3 => style.yellow,
        4 => style.blue,
        5 => style.magenta,
        6 => style.cyan,
        7 => style.white,
        8 => style.bright_black,
        9 => style.bright_red,
        10 => style.bright_green,
        11 => style.bright_yellow,
        12 => style.bright_blue,
        13 => style.bright_magenta,
        14 => style.bright_cyan,
        15 => style.bright_white,
        //16-231 are mapped to their RGB colors on a 0-5 range per channel
        16..=231 => {
            let (r, g, b) = rgb_for_index(index); //Split the index into it's ANSI-RGB components
            let step = (u8::MAX as f32 / 5.).floor() as u8; //Split the RGB range into 5 chunks, with floor so no overflow
            Color::new(r * step, g * step, b * step, u8::MAX) //Map the ANSI-RGB components to an RGB color
        }
        //232-255 are a 24 step grayscale from black to white
        232..=255 => {
            let i = index - 232; //Align index to 0..24
            let step = (u8::MAX as f32 / 24.).floor() as u8; //Split the RGB grayscale values into 24 chunks
            Color::new(i * step, i * step, i * step, u8::MAX) //Map the ANSI-grayscale components to the RGB-grayscale
        }
    }
}

///Generates the rgb channels in [0, 5] for a given index into the 6x6x6 ANSI color cube
///See: [8 bit ansi color](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
///
///Wikipedia gives a formula for calculating the index for a given color:
///
///index = 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5)
///
///This function does the reverse, calculating the r, g, and b components from a given index.
fn rgb_for_index(i: &u8) -> (u8, u8, u8) {
    debug_assert!(i >= &16 && i <= &231);
    let i = i - 16;
    let r = (i - (i % 36)) / 36;
    let g = ((i % 36) - (i % 6)) / 6;
    let b = (i % 36) % 6;
    (r, g, b)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_rgb_for_index() {
        //Test every possible value in the color cube
        for i in 16..=231 {
            let (r, g, b) = crate::terminal_element::rgb_for_index(&(i as u8));
            assert_eq!(i, 16 + 36 * r + 6 * g + b);
        }
    }
}
