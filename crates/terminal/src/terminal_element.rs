use alacritty_terminal::{
    ansi::{Color as AnsiColor, Color::Named, CursorShape as AlacCursorShape, NamedColor},
    grid::Dimensions,
    index::Point,
    selection::SelectionRange,
    term::{
        cell::{Cell, Flags},
        TermMode,
    },
};
use editor::{Cursor, CursorShape, HighlightedRange, HighlightedRangeLine};
use gpui::{
    color::Color,
    fonts::{Properties, Style::Italic, TextStyle, Underline, Weight},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    serde_json::json,
    text_layout::{Line, RunStyle},
    Element, Event, EventContext, FontCache, KeyDownEvent, ModelContext, MouseButton, MouseRegion,
    PaintContext, Quad, TextLayoutCache, WeakModelHandle, WeakViewHandle,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use settings::Settings;
use theme::TerminalStyle;
use util::ResultExt;

use std::fmt::Debug;
use std::{
    mem,
    ops::{Deref, Range},
};

use crate::{
    mappings::colors::convert_color,
    terminal_view::{DeployContextMenu, TerminalView},
    Terminal, TerminalSize,
};

///The information generated during layout that is nescessary for painting
pub struct LayoutState {
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    selections: Vec<RelativeHighlightedRange>,
    cursor: Option<Cursor>,
    background_color: Color,
    selection_color: Color,
    size: TerminalSize,
    mode: TermMode,
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
pub struct TerminalElement {
    terminal: WeakModelHandle<Terminal>,
    view: WeakViewHandle<TerminalView>,
    modal: bool,
    focused: bool,
    cursor_visible: bool,
}

impl TerminalElement {
    pub fn new(
        view: WeakViewHandle<TerminalView>,
        terminal: WeakModelHandle<Terminal>,
        modal: bool,
        focused: bool,
        cursor_visible: bool,
    ) -> TerminalElement {
        TerminalElement {
            view,
            terminal,
            modal,
            focused,
            cursor_visible,
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
                        let cell_style = TerminalElement::cell_style(
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

    fn generic_button_handler<E>(
        connection: WeakModelHandle<Terminal>,
        origin: Vector2F,
        f: impl Fn(&mut Terminal, Vector2F, E, &mut ModelContext<Terminal>),
    ) -> impl Fn(E, &mut EventContext) {
        move |event, cx| {
            cx.focus_parent_view();
            if let Some(conn_handle) = connection.upgrade(cx.app) {
                conn_handle.update(cx.app, |terminal, cx| {
                    f(terminal, origin, event, cx);

                    cx.notify();
                })
            }
        }
    }

    fn attach_mouse_handlers(
        &self,
        origin: Vector2F,
        view_id: usize,
        visible_bounds: RectF,
        mode: TermMode,
        cx: &mut PaintContext,
    ) {
        let connection = self.terminal;

        let mut region = MouseRegion::new(view_id, None, visible_bounds);

        // Terminal Emulator controlled behavior:
        region = region
            // Start selections
            .on_down(
                MouseButton::Left,
                TerminalElement::generic_button_handler(
                    connection,
                    origin,
                    move |terminal, origin, e, _cx| {
                        terminal.mouse_down(&e, origin);
                    },
                ),
            )
            // Update drag selections
            .on_drag(MouseButton::Left, move |event, cx| {
                if cx.is_parent_view_focused() {
                    if let Some(conn_handle) = connection.upgrade(cx.app) {
                        conn_handle.update(cx.app, |terminal, cx| {
                            terminal.mouse_drag(event, origin);
                            cx.notify();
                        })
                    }
                }
            })
            // Copy on up behavior
            .on_up(
                MouseButton::Left,
                TerminalElement::generic_button_handler(
                    connection,
                    origin,
                    move |terminal, origin, e, _cx| {
                        terminal.mouse_up(&e, origin);
                    },
                ),
            )
            // Handle click based selections
            .on_click(
                MouseButton::Left,
                TerminalElement::generic_button_handler(
                    connection,
                    origin,
                    move |terminal, origin, e, _cx| {
                        terminal.left_click(&e, origin);
                    },
                ),
            )
            // Context menu
            .on_click(MouseButton::Right, move |e, cx| {
                let mouse_mode = if let Some(conn_handle) = connection.upgrade(cx.app) {
                    conn_handle.update(cx.app, |terminal, _cx| terminal.mouse_mode(e.shift))
                } else {
                    // If we can't get the model handle, probably can't deploy the context menu
                    true
                };
                if !mouse_mode {
                    cx.dispatch_action(DeployContextMenu {
                        position: e.position,
                    });
                }
            });

        // Mouse mode handlers:
        // All mouse modes need the extra click handlers
        if mode.intersects(TermMode::MOUSE_MODE) {
            region = region
                .on_down(
                    MouseButton::Right,
                    TerminalElement::generic_button_handler(
                        connection,
                        origin,
                        move |terminal, origin, e, _cx| {
                            terminal.mouse_down(&e, origin);
                        },
                    ),
                )
                .on_down(
                    MouseButton::Middle,
                    TerminalElement::generic_button_handler(
                        connection,
                        origin,
                        move |terminal, origin, e, _cx| {
                            terminal.mouse_down(&e, origin);
                        },
                    ),
                )
                .on_up(
                    MouseButton::Right,
                    TerminalElement::generic_button_handler(
                        connection,
                        origin,
                        move |terminal, origin, e, _cx| {
                            terminal.mouse_up(&e, origin);
                        },
                    ),
                )
                .on_up(
                    MouseButton::Middle,
                    TerminalElement::generic_button_handler(
                        connection,
                        origin,
                        move |terminal, origin, e, _cx| {
                            terminal.mouse_up(&e, origin);
                        },
                    ),
                )
        }
        //Mouse move manages both dragging and motion events
        if mode.intersects(TermMode::MOUSE_DRAG | TermMode::MOUSE_MOTION) {
            region = region
                //TODO: This does not fire on right-mouse-down-move events.
                .on_move(move |event, cx| {
                    if cx.is_parent_view_focused() {
                        if let Some(conn_handle) = connection.upgrade(cx.app) {
                            conn_handle.update(cx.app, |terminal, cx| {
                                terminal.mouse_move(&event, origin);
                                cx.notify();
                            })
                        }
                    }
                })
        }

        cx.scene.push_mouse_region(region);
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
}

impl Element for TerminalElement {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let settings = cx.global::<Settings>();
        let font_cache = cx.font_cache();

        //Setup layout information
        let terminal_theme = settings.theme.terminal.clone(); //TODO: Try to minimize this clone.
        let text_style = TerminalElement::make_text_style(font_cache, settings);
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

        let (cells, selection, cursor, display_offset, cursor_text, searcher, mode) = self
            .terminal
            .upgrade(cx)
            .unwrap()
            .update(cx.app, |terminal, mcx| {
                terminal.set_size(dimensions);
                terminal.render_lock(mcx, |content, cursor_text, searcher| {
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
                        searcher,
                        content.mode,
                    )
                })
            });

        let (cells, rects, selections) = TerminalElement::layout_grid(
            cells,
            &text_style,
            &terminal_theme,
            cx.text_layout_cache,
            cx.font_cache(),
            self.modal,
            selection,
        );

        //Layout cursor. Rectangle is used for IME, so we should lay it out even
        //if we don't end up showing it.
        let cursor = if let AlacCursorShape::Hidden = cursor.shape {
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

            TerminalElement::shape_cursor(cursor_point, dimensions, &cursor_text).map(
                move |(cursor_position, block_width)| {
                    let shape = match cursor.shape {
                        AlacCursorShape::Block if !self.focused => CursorShape::Hollow,
                        AlacCursorShape::Block => CursorShape::Block,
                        AlacCursorShape::Underline => CursorShape::Underscore,
                        AlacCursorShape::Beam => CursorShape::Bar,
                        AlacCursorShape::HollowBlock => CursorShape::Hollow,
                        //This case is handled in the if wrapping the whole cursor layout
                        AlacCursorShape::Hidden => unreachable!(),
                    };

                    Cursor::new(
                        cursor_position,
                        block_width,
                        dimensions.line_height,
                        terminal_theme.colors.cursor,
                        shape,
                        Some(cursor_text),
                    )
                },
            )
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
                selections,
                mode,
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
            self.attach_mouse_handlers(origin, self.view.id(), visible_bounds, layout.mode, cx);

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
                let start_y = layout.selections.get(0).map(|highlight| {
                    origin.y() + highlight.line_index as f32 * layout.size.line_height
                });

                if let Some(y) = start_y {
                    let range_lines = layout
                        .selections
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
            if self.cursor_visible {
                if let Some(cursor) = &layout.cursor {
                    cx.paint_layer(clip_bounds, |cx| {
                        cursor.paint(origin, cx);
                    })
                }
            }
        });
    }

    fn dispatch_event(
        &mut self,
        event: &gpui::Event,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        cx: &mut gpui::EventContext,
    ) -> bool {
        match event {
            Event::ScrollWheel(e) => visible_bounds
                .contains_point(e.position)
                .then(|| {
                    let origin = bounds.origin() + vec2f(layout.size.cell_width, 0.);

                    if let Some(terminal) = self.terminal.upgrade(cx.app) {
                        terminal.update(cx.app, |term, _| term.scroll_wheel(e, origin));
                        cx.notify();
                    }
                })
                .is_some(),
            Event::KeyDown(KeyDownEvent { keystroke, .. }) => {
                if !cx.is_parent_view_focused() {
                    return false;
                }

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
