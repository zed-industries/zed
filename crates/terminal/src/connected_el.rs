use alacritty_terminal::{
    ansi::{Color as AnsiColor, Color::Named, NamedColor},
    grid::{Dimensions, Scroll},
    index::{Column as GridCol, Line as GridLine, Point, Side},
    selection::SelectionRange,
    term::cell::{Cell, Flags},
};
use editor::{Cursor, CursorShape, HighlightedRange, HighlightedRangeLine};
use gpui::{
    color::Color,
    elements::*,
    fonts::{Properties, Style::Italic, TextStyle, Underline, Weight},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    text_layout::{Line, RunStyle},
    Event, FontCache, KeyDownEvent, MouseButton, MouseButtonEvent, MouseMovedEvent, MouseRegion,
    PaintContext, Quad, ScrollWheelEvent, TextLayoutCache, WeakModelHandle, WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::{Settings, TerminalBlink};
use theme::TerminalStyle;
use util::ResultExt;

use std::{
    cmp::min,
    mem,
    ops::{Deref, Range},
};
use std::{fmt::Debug, ops::Sub};

use crate::{
    connected_view::{ConnectedView, DeployContextMenu},
    mappings::colors::convert_color,
    Terminal, TerminalSize,
};

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
pub const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    highlights: Vec<RelativeHighlightedRange>,
    cursor: Option<Cursor>,
    background_color: Color,
    selection_color: Color,
    size: TerminalSize,
    display_offset: usize,
}

#[derive(Debug)]
struct IndexedCell {
    point: Point,
    cell: Cell,
}

impl Deref for IndexedCell {
    type Target = Cell;

    #[inline]
    fn deref(&self) -> &Cell {
        &self.cell
    }
}

///Helper struct for converting data between alacritty's cursor points, and displayed cursor points
struct DisplayCursor {
    line: i32,
    col: usize,
}

impl DisplayCursor {
    fn from(cursor_point: Point, display_offset: usize) -> Self {
        Self {
            line: cursor_point.line.0 + display_offset as i32,
            col: cursor_point.column.0,
        }
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
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
        let pos = {
            let point = self.point;
            vec2f(
                (origin.x() + point.column as f32 * layout.size.cell_width).floor(),
                origin.y() + point.line as f32 * layout.size.line_height,
            )
        };

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
        let position = {
            let point = self.point;
            vec2f(
                (origin.x() + point.column as f32 * layout.size.cell_width).floor(),
                origin.y() + point.line as f32 * layout.size.line_height,
            )
        };
        let size = vec2f(
            (layout.size.cell_width * self.num_of_cells as f32).ceil(),
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

        HighlightedRangeLine { start_x, end_x }
    }
}

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalEl {
    terminal: WeakModelHandle<Terminal>,
    view: WeakViewHandle<ConnectedView>,
    modal: bool,
    focused: bool,
    blink_state: bool,
}

impl TerminalEl {
    pub fn new(
        view: WeakViewHandle<ConnectedView>,
        terminal: WeakModelHandle<Terminal>,
        modal: bool,
        focused: bool,
        blink_state: bool,
    ) -> TerminalEl {
        TerminalEl {
            view,
            terminal,
            modal,
            focused,
            blink_state,
        }
    }

    fn layout_grid(
        grid: Vec<IndexedCell>,
        text_style: &TextStyle,
        terminal_theme: &TerminalStyle,
        text_layout_cache: &TextLayoutCache,
        font_cache: &FontCache,
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

        let linegroups = grid.into_iter().group_by(|i| i.point.line);
        for (line_index, (_, line)) in linegroups.into_iter().enumerate() {
            for (x_index, cell) in line.enumerate() {
                let mut fg = cell.fg;
                let mut bg = cell.bg;
                if cell.flags.contains(Flags::INVERSE) {
                    mem::swap(&mut fg, &mut bg);
                }

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
                    if matches!(bg, Named(NamedColor::Background)) {
                        //Continue to next cell, resetting variables if nescessary
                        cur_alac_color = None;
                        if let Some(rect) = cur_rect {
                            rects.push(rect);
                            cur_rect = None
                        }
                    } else {
                        match cur_alac_color {
                            Some(cur_color) => {
                                if bg == cur_color {
                                    cur_rect = cur_rect.take().map(|rect| rect.extend());
                                } else {
                                    cur_alac_color = Some(bg);
                                    if cur_rect.is_some() {
                                        rects.push(cur_rect.take().unwrap());
                                    }
                                    cur_rect = Some(LayoutRect::new(
                                        Point::new(line_index as i32, cell.point.column.0 as i32),
                                        1,
                                        convert_color(&bg, &terminal_theme.colors, modal),
                                    ));
                                }
                            }
                            None => {
                                cur_alac_color = Some(bg);
                                cur_rect = Some(LayoutRect::new(
                                    Point::new(line_index as i32, cell.point.column.0 as i32),
                                    1,
                                    convert_color(&bg, &terminal_theme.colors, modal),
                                ));
                            }
                        }
                    }
                }

                //Layout current cell text
                {
                    let cell_text = &cell.c.to_string();
                    if cell_text != " " {
                        let cell_style = TerminalEl::cell_style(
                            &cell,
                            fg,
                            terminal_theme,
                            text_style,
                            font_cache,
                            modal,
                        );

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
    fn shape_cursor(
        cursor_point: DisplayCursor,
        size: TerminalSize,
        text_fragment: &Line,
    ) -> Option<(Vector2F, f32)> {
        if cursor_point.line() < size.total_lines() as i32 {
            let cursor_width = if text_fragment.width() == 0. {
                size.cell_width()
            } else {
                text_fragment.width()
            };

            //Cursor should always surround as much of the text as possible,
            //hence when on pixel boundaries round the origin down and the width up
            Some((
                vec2f(
                    (cursor_point.col() as f32 * size.cell_width()).floor(),
                    (cursor_point.line() as f32 * size.line_height()).floor(),
                ),
                cursor_width.ceil(),
            ))
        } else {
            None
        }
    }

    ///Convert the Alacritty cell styles to GPUI text styles and background color
    fn cell_style(
        indexed: &IndexedCell,
        fg: AnsiColor,
        style: &TerminalStyle,
        text_style: &TextStyle,
        font_cache: &FontCache,
        modal: bool,
    ) -> RunStyle {
        let flags = indexed.cell.flags;
        let fg = convert_color(&fg, &style.colors, modal);

        let underline = flags
            .intersects(Flags::ALL_UNDERLINES)
            .then(|| Underline {
                color: Some(fg),
                squiggly: flags.contains(Flags::UNDERCURL),
                thickness: OrderedFloat(1.),
            })
            .unwrap_or_default();

        let mut properties = Properties::new();
        if indexed
            .flags
            .intersects(Flags::BOLD | Flags::BOLD_ITALIC | Flags::DIM_BOLD)
        {
            properties = *properties.weight(Weight::BOLD);
        }
        if indexed.flags.intersects(Flags::ITALIC | Flags::BOLD_ITALIC) {
            properties = *properties.style(Italic);
        }

        let font_id = font_cache
            .select_font(text_style.font_family_id, &properties)
            .unwrap_or(text_style.font_id);

        RunStyle {
            color: fg,
            font_id,
            underline,
        }
    }

    fn attach_mouse_handlers(
        &self,
        origin: Vector2F,
        view_id: usize,
        visible_bounds: RectF,
        cur_size: TerminalSize,
        display_offset: usize,
        cx: &mut PaintContext,
    ) {
        let mouse_down_connection = self.terminal;
        let click_connection = self.terminal;
        let drag_connection = self.terminal;
        cx.scene.push_mouse_region(
            MouseRegion::new(view_id, None, visible_bounds)
                .on_down(
                    MouseButton::Left,
                    move |MouseButtonEvent { position, .. }, cx| {
                        if let Some(conn_handle) = mouse_down_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                let (point, side) = TerminalEl::mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    display_offset,
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
                                let (point, side) = TerminalEl::mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    display_offset,
                                );

                                terminal.click(point, side, click_count);

                                cx.notify();
                            });
                        }
                    },
                )
                .on_click(
                    MouseButton::Right,
                    move |MouseButtonEvent { position, .. }, cx| {
                        cx.dispatch_action(DeployContextMenu { position });
                    },
                )
                .on_drag(
                    MouseButton::Left,
                    move |_, MouseMovedEvent { position, .. }, cx| {
                        if let Some(conn_handle) = drag_connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                let (point, side) = TerminalEl::mouse_to_cell_data(
                                    position,
                                    origin,
                                    cur_size,
                                    display_offset,
                                );

                                terminal.drag(point, side);

                                cx.notify()
                            });
                        }
                    },
                ),
        );
    }

    ///Configures a text style from the current settings.
    pub fn make_text_style(font_cache: &FontCache, settings: &Settings) -> TextStyle {
        // Pull the font family from settings properly overriding
        let family_id = settings
            .terminal_overrides
            .font_family
            .as_ref()
            .or(settings.terminal_defaults.font_family.as_ref())
            .and_then(|family_name| font_cache.load_family(&[family_name]).log_err())
            .unwrap_or(settings.buffer_font_family);

        let font_size = settings
            .terminal_overrides
            .font_size
            .or(settings.terminal_defaults.font_size)
            .unwrap_or(settings.buffer_font_size);

        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();

        TextStyle {
            color: settings.theme.editor.text_color,
            font_family_id: family_id,
            font_family_name: font_cache.family_name(family_id).unwrap(),
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        }
    }

    pub fn mouse_to_cell_data(
        pos: Vector2F,
        origin: Vector2F,
        cur_size: TerminalSize,
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
            let cell_x =
                x.saturating_sub(cur_size.cell_width as usize) % cur_size.cell_width as usize;
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

    pub fn should_show_cursor(
        settings: Option<TerminalBlink>,
        blinking_on: bool,
        focused: bool,
        blink_show: bool,
    ) -> bool {
        if !focused {
            true
        } else {
            match settings {
                Some(setting) => match setting {
                TerminalBlink::Never => true,
                TerminalBlink::On | TerminalBlink::Off if blinking_on => blink_show,
                TerminalBlink::On | TerminalBlink::Off /*if !blinking_on */ => true,
                TerminalBlink::Always => focused && blink_show,
            },
                None => {
                    if blinking_on {
                        blink_show
                    } else {
                        false
                    }
                }
            }
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
        let settings = cx.global::<Settings>();
        let blink_settings = settings.terminal_overrides.blinking.clone();
        let font_cache = cx.font_cache();

        //Setup layout information
        let terminal_theme = settings.theme.terminal.clone(); //TODO: Try to minimize this clone.
        let text_style = TerminalEl::make_text_style(font_cache, settings);
        let selection_color = settings.theme.editor.selection.selection;
        let dimensions = {
            let line_height = font_cache.line_height(text_style.font_size);
            let cell_width = font_cache.em_advance(text_style.font_id, text_style.font_size);
            TerminalSize::new(line_height, cell_width, constraint.max)
        };

        let background_color = if self.modal {
            terminal_theme.colors.modal_background
        } else {
            terminal_theme.colors.background
        };

        let (cells, selection, cursor, display_offset, cursor_text, blink_mode) = self
            .terminal
            .upgrade(cx)
            .unwrap()
            .update(cx.app, |terminal, mcx| {
                terminal.set_size(dimensions);
                terminal.render_lock(mcx, |content, cursor_text, blink_mode| {
                    let mut cells = vec![];
                    cells.extend(
                        content
                            .display_iter
                            //TODO: Add this once there's a way to retain empty lines
                            // .filter(|ic| {
                            //     !ic.flags.contains(Flags::HIDDEN)
                            //         && !(ic.bg == Named(NamedColor::Background)
                            //             && ic.c == ' '
                            //             && !ic.flags.contains(Flags::INVERSE))
                            // })
                            .map(|ic| IndexedCell {
                                point: ic.point,
                                cell: ic.cell.clone(),
                            }),
                    );

                    (
                        cells,
                        content.selection,
                        content.cursor,
                        content.display_offset,
                        cursor_text,
                        blink_mode,
                    )
                })
            });

        let (cells, rects, highlights) = TerminalEl::layout_grid(
            cells,
            &text_style,
            &terminal_theme,
            cx.text_layout_cache,
            cx.font_cache(),
            self.modal,
            selection,
        );

        //Layout cursor
        let cursor = {
            if !TerminalEl::should_show_cursor(
                blink_settings,
                blink_mode,
                self.focused,
                self.blink_state,
            ) {
                None
            } else {
                let cursor_point = DisplayCursor::from(cursor.point, display_offset);
                let cursor_text = {
                    let str_trxt = cursor_text.to_string();

                    let color = if self.focused {
                        terminal_theme.colors.background
                    } else {
                        terminal_theme.colors.foreground
                    };

                    cx.text_layout_cache.layout_str(
                        &str_trxt,
                        text_style.font_size,
                        &[(
                            str_trxt.len(),
                            RunStyle {
                                font_id: text_style.font_id,
                                color,
                                underline: Default::default(),
                            },
                        )],
                    )
                };

                TerminalEl::shape_cursor(cursor_point, dimensions, &cursor_text).map(
                    move |(cursor_position, block_width)| {
                        let (shape, color) = if self.focused {
                            (CursorShape::Block, terminal_theme.colors.cursor)
                        } else {
                            (CursorShape::Hollow, terminal_theme.colors.foreground)
                        };

                        Cursor::new(
                            cursor_position,
                            block_width,
                            dimensions.line_height,
                            color,
                            shape,
                            Some(cursor_text),
                        )
                    },
                )
            }
        };

        //Done!
        (
            constraint.max,
            LayoutState {
                cells,
                cursor,
                background_color,
                selection_color,
                size: dimensions,
                rects,
                highlights,
                display_offset,
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
            let origin = bounds.origin() + vec2f(layout.size.cell_width, 0.);

            //Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
            self.attach_mouse_handlers(
                origin,
                self.view.id(),
                visible_bounds,
                layout.size,
                layout.display_offset,
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

                for rect in &layout.rects {
                    rect.paint(origin, layout, cx)
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

                    if let Some(terminal) = self.terminal.upgrade(cx.app) {
                        terminal.update(cx.app, |term, _| {
                            term.scroll(Scroll::Delta(vertical_scroll.round() as i32))
                        });
                    }

                    cx.notify();
                })
                .is_some(),
            Event::KeyDown(KeyDownEvent { keystroke, .. }) => {
                if !cx.is_parent_view_focused() {
                    return false;
                }

                //TODO Talk to keith about how to catch events emitted from an element.
                if let Some(view) = self.view.upgrade(cx.app) {
                    view.update(cx.app, |view, cx| {
                        view.clear_bel(cx);
                        view.pause_cursor_blinking(cx);
                    })
                }

                self.terminal
                    .upgrade(cx.app)
                    .map(|model_handle| {
                        model_handle.update(cx.app, |term, _| term.try_keystroke(keystroke))
                    })
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

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        bounds: RectF,
        _: RectF,
        layout: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::MeasurementContext,
    ) -> Option<RectF> {
        // Use the same origin that's passed to `Cursor::paint` in the paint
        // method bove.
        let mut origin = bounds.origin() + vec2f(layout.size.cell_width, 0.);

        // TODO - Why is it necessary to move downward one line to get correct
        // positioning? I would think that we'd want the same rect that is
        // painted for the cursor.
        origin += vec2f(0., layout.size.line_height);

        Some(layout.cursor.as_ref()?.bounding_rect(origin))
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

        let cur_size = crate::connected_el::TerminalSize::new(
            line_height,
            cell_width,
            gpui::geometry::vector::vec2f(term_width, term_height),
        );

        let mouse_pos = gpui::geometry::vector::vec2f(mouse_pos_x, mouse_pos_y);
        let origin = gpui::geometry::vector::vec2f(origin_x, origin_y); //Position of terminal window, 1 'cell' in
        let (point, _) =
            crate::connected_el::TerminalEl::mouse_to_cell_data(mouse_pos, origin, cur_size, 0);
        assert_eq!(
            point,
            alacritty_terminal::index::Point::new(
                alacritty_terminal::index::Line(((mouse_pos_y - origin_y) / line_height) as i32),
                alacritty_terminal::index::Column(((mouse_pos_x - origin_x) / cell_width) as usize),
            )
        );
    }
}
