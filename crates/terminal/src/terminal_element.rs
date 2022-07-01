use alacritty_terminal::{
    ansi::Color as AnsiColor,
    grid::{GridIterator, Indexed},
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
    fonts::{HighlightStyle, TextStyle, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    text_layout::{Line, RunStyle},
    Event, FontCache, MouseRegion, PaintContext, Quad, SizeConstraint, WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use std::{iter, rc::Rc};
use theme::TerminalStyle;

use crate::{Input, ScrollTerminal, Terminal};

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

///Represents a span of cells in a single line in the terminal's grid.
///This is used for drawing background rectangles
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct RectSpan {
    start: i32,
    end: i32,
    line: usize,
    color: Color,
}

///A background color span
impl RectSpan {
    ///Creates a new LineSpan. `start` must be <= `end`.
    ///If `start` == `end`, then this span is considered to be over a
    /// single cell
    fn new(start: i32, end: i32, line: usize, color: Color) -> RectSpan {
        debug_assert!(start <= end);
        RectSpan {
            start,
            end,
            line,
            color,
        }
    }
}

///Helper types so I don't mix these two up
struct CellWidth(f32);
struct LineHeight(f32);

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    lines: Vec<Line>,
    line_height: LineHeight,
    em_width: CellWidth,
    cursor: Option<(Vector2F, Color, Option<Line>)>,
    cur_size: SizeInfo,
    background_color: Color,
    background_rects: Vec<(RectF, Color)>, //Vec index == Line index for the LineSpan
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

        // let cursor_char = term.grid().cursor_cell().c.to_string();

        let cursor_text = {
            let grid = term.grid();
            let cursor_point = grid.cursor.point;
            grid[cursor_point.line][cursor_point.column].c.to_string()
        };

        let content = term.renderable_content();

        //And we're off! Begin layouting
        let (chunks, line_count) = build_chunks(content.display_iter, &terminal_theme);

        let shaped_lines = layout_highlighted_chunks(
            chunks
                .iter()
                .map(|(text, style, _)| (text.as_str(), *style)),
            &text_style,
            cx.text_layout_cache,
            cx.font_cache(),
            usize::MAX,
            line_count,
        );

        let backgrounds = chunks
            .iter()
            .filter(|(_, _, line_span)| line_span != &RectSpan::default())
            .map(|(_, _, line_span)| *line_span)
            .collect();
        let background_rects = make_background_rects(backgrounds, &shaped_lines, &line_height);

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
        let cursor = get_cursor_position(
            content.cursor.point,
            &shaped_lines,
            content.display_offset,
            &line_height,
        )
        .map(|cursor_rect| (cursor_rect, terminal_theme.cursor, Some(block_text)));

        (
            constraint.max,
            LayoutState {
                lines: shaped_lines,
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
        cx.scene.push_layer(Some(visible_bounds));

        //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
        cx.scene.push_mouse_region(MouseRegion {
            view_id: self.view.id(),
            mouse_down: Some(Rc::new(|_, cx| cx.focus_parent_view())),
            bounds: visible_bounds,
            ..Default::default()
        });

        let origin = bounds.origin() + vec2f(layout.em_width.0, 0.);

        //Start us off with a nice simple background color
        cx.scene.push_layer(Some(visible_bounds));
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
        cx.scene.pop_layer();

        //Draw text
        cx.scene.push_layer(Some(visible_bounds));
        let mut line_origin = origin.clone();
        for line in &layout.lines {
            let boundaries = RectF::new(line_origin, vec2f(bounds.width(), layout.line_height.0));
            if boundaries.intersects(visible_bounds) {
                line.paint(line_origin, visible_bounds, layout.line_height.0, cx);
            }
            line_origin.set_y(boundaries.max_y());
        }
        cx.scene.pop_layer();

        //Draw cursor
        cx.scene.push_layer(Some(visible_bounds));
        if let Some((c, color, block_text)) = &layout.cursor {
            let editor_cursor = Cursor::new(
                origin + *c,
                layout.em_width.0,
                layout.line_height.0,
                *color,
                CursorShape::Block,
                block_text.clone(), //TODO fix this
            );

            editor_cursor.paint(cx);
        }
        cx.scene.pop_layer();

        #[cfg(debug_assertions)]
        if DEBUG_GRID {
            draw_debug_grid(bounds, layout, cx);
        }

        cx.scene.pop_layer();
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

///In a single pass, this function generates the background and foreground color info for every item in the grid.
pub(crate) fn build_chunks(
    grid_iterator: GridIterator<Cell>,
    theme: &TerminalStyle,
) -> (Vec<(String, Option<HighlightStyle>, RectSpan)>, usize) {
    let mut line_count: usize = 0;
    //Every `group_by()` -> `into_iter()` pair needs to be seperated by a local variable so
    //rust knows where to put everything.
    //Start by grouping by lines
    let lines = grid_iterator.group_by(|i| i.point.line.0);
    let result = lines
        .into_iter()
        .map(|(_, line)| {
            line_count += 1;
            let mut col_index = 0;

            //Then group by style
            let chunks = line.group_by(|i| cell_style(&i, theme));
            chunks
                .into_iter()
                .map(|(style, fragment)| {
                    //And assemble the styled fragment into it's background and foreground information
                    let str_fragment = fragment.map(|indexed| indexed.c).collect::<String>();
                    let start = col_index;
                    let end = start + str_fragment.len() as i32;
                    col_index = end;
                    (
                        str_fragment,
                        Some(style.0),
                        RectSpan::new(start, end, line_count - 1, style.1), //Line count -> Line index
                    )
                })
                //Add a \n to the end, as we're using text layouting rather than grid layouts
                .chain(iter::once(("\n".to_string(), None, Default::default())))
                .collect::<Vec<(String, Option<HighlightStyle>, RectSpan)>>()
        })
        .flatten()
        //We have a Vec<Vec<>> (Vec of lines of styled chunks), flatten to just Vec<> (the styled chunks)
        .collect::<Vec<(String, Option<HighlightStyle>, RectSpan)>>();
    (result, line_count)
}

///Convert a RectSpan in terms of character offsets, into RectFs of exact offsets
fn make_background_rects(
    backgrounds: Vec<RectSpan>,
    shaped_lines: &Vec<Line>,
    line_height: &LineHeight,
) -> Vec<(RectF, Color)> {
    backgrounds
        .into_iter()
        .map(|line_span| {
            //This should always be safe, as the shaped lines and backgrounds where derived
            //At the same time earlier
            let line = shaped_lines
                .get(line_span.line)
                .expect("Background line_num did not correspond to a line number");
            let x = line.x_for_index(line_span.start as usize);
            let width = line.x_for_index(line_span.end as usize) - x;
            (
                RectF::new(
                    vec2f(x, line_span.line as f32 * line_height.0),
                    vec2f(width, line_height.0),
                ),
                line_span.color,
            )
        })
        .collect::<Vec<(RectF, Color)>>()
}

///Create the rectangle for a cursor, exactly positioned according to the text
fn get_cursor_position(
    cursor_point: Point,
    shaped_lines: &Vec<Line>,
    display_offset: usize,
    line_height: &LineHeight,
) -> Option<Vector2F> {
    let cursor_line = cursor_point.line.0 as usize + display_offset;
    shaped_lines.get(cursor_line).map(|layout_line| {
        let cursor_x = layout_line.x_for_index(cursor_point.column.0);
        vec2f(cursor_x, cursor_line as f32 * line_height.0)
    })
}

///Convert the Alacritty cell styles to GPUI text styles and background color
fn cell_style(indexed: &Indexed<&Cell>, style: &TerminalStyle) -> (HighlightStyle, Color) {
    let flags = indexed.cell.flags;
    let fg = Some(convert_color(&indexed.cell.fg, style));
    let bg = convert_color(&indexed.cell.bg, style);

    let underline = flags.contains(Flags::UNDERLINE).then(|| Underline {
        color: fg,
        squiggly: false,
        thickness: OrderedFloat(1.),
    });

    (
        HighlightStyle {
            color: fg,
            underline,
            ..Default::default()
        },
        bg,
    )
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
