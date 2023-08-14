use editor::{Cursor, HighlightedRange, HighlightedRangeLine};
use gpui::{
    color::Color,
    elements::{Empty, Overlay},
    fonts::{HighlightStyle, Properties, Style::Italic, TextStyle, Underline, Weight},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::{CursorStyle, MouseButton},
    serde_json::json,
    text_layout::{Line, RunStyle},
    AnyElement, Element, EventContext, FontCache, LayoutContext, ModelContext, MouseRegion,
    PaintContext, Quad, SceneBuilder, SizeConstraint, TextLayoutCache, ViewContext,
    WeakModelHandle,
};
use itertools::Itertools;
use language::CursorShape;
use ordered_float::OrderedFloat;
use terminal::{
    alacritty_terminal::{
        ansi::{Color as AnsiColor, Color::Named, CursorShape as AlacCursorShape, NamedColor},
        grid::Dimensions,
        index::Point,
        term::{cell::Flags, TermMode},
    },
    mappings::colors::convert_color,
    IndexedCell, Terminal, TerminalContent, TerminalSettings, TerminalSize,
};
use theme::{TerminalStyle, ThemeSettings};
use util::ResultExt;

use std::{fmt::Debug, ops::RangeInclusive};
use std::{mem, ops::Range};

use crate::TerminalView;

///The information generated during layout that is necessary for painting
pub struct LayoutState {
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    relative_highlighted_ranges: Vec<(RangeInclusive<Point>, Color)>,
    cursor: Option<Cursor>,
    background_color: Color,
    size: TerminalSize,
    mode: TermMode,
    display_offset: usize,
    hyperlink_tooltip: Option<AnyElement<TerminalView>>,
    gutter: f32,
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
        scene: &mut SceneBuilder,
        origin: Vector2F,
        layout: &LayoutState,
        visible_bounds: RectF,
        _view: &mut TerminalView,
        cx: &mut ViewContext<TerminalView>,
    ) {
        let pos = {
            let point = self.point;
            vec2f(
                (origin.x() + point.column as f32 * layout.size.cell_width).floor(),
                origin.y() + point.line as f32 * layout.size.line_height,
            )
        };

        self.text
            .paint(scene, pos, visible_bounds, layout.size.line_height, cx);
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

    fn paint(
        &self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        layout: &LayoutState,
        _view: &mut TerminalView,
        _cx: &mut ViewContext<TerminalView>,
    ) {
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

        scene.push_quad(Quad {
            bounds: RectF::new(position, size),
            background: Some(self.color),
            border: Default::default(),
            corner_radii: Default::default(),
        })
    }
}

///The GPUI element that paints the terminal.
///We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalElement {
    terminal: WeakModelHandle<Terminal>,
    focused: bool,
    cursor_visible: bool,
    can_navigate_to_selected_word: bool,
}

impl TerminalElement {
    pub fn new(
        terminal: WeakModelHandle<Terminal>,
        focused: bool,
        cursor_visible: bool,
        can_navigate_to_selected_word: bool,
    ) -> TerminalElement {
        TerminalElement {
            terminal,
            focused,
            cursor_visible,
            can_navigate_to_selected_word,
        }
    }

    //Vec<Range<Point>> -> Clip out the parts of the ranges

    fn layout_grid(
        grid: &Vec<IndexedCell>,
        text_style: &TextStyle,
        terminal_theme: &TerminalStyle,
        text_layout_cache: &TextLayoutCache,
        font_cache: &FontCache,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<Point>)>,
    ) -> (Vec<LayoutCell>, Vec<LayoutRect>) {
        let mut cells = vec![];
        let mut rects = vec![];

        let mut cur_rect: Option<LayoutRect> = None;
        let mut cur_alac_color = None;

        let linegroups = grid.into_iter().group_by(|i| i.point.line);
        for (line_index, (_, line)) in linegroups.into_iter().enumerate() {
            for cell in line {
                let mut fg = cell.fg;
                let mut bg = cell.bg;
                if cell.flags.contains(Flags::INVERSE) {
                    mem::swap(&mut fg, &mut bg);
                }

                //Expand background rect range
                {
                    if matches!(bg, Named(NamedColor::Background)) {
                        //Continue to next cell, resetting variables if necessary
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
                                        convert_color(&bg, &terminal_theme),
                                    ));
                                }
                            }
                            None => {
                                cur_alac_color = Some(bg);
                                cur_rect = Some(LayoutRect::new(
                                    Point::new(line_index as i32, cell.point.column.0 as i32),
                                    1,
                                    convert_color(&bg, &terminal_theme),
                                ));
                            }
                        }
                    }
                }

                //Layout current cell text
                {
                    let cell_text = &cell.c.to_string();
                    if !is_blank(&cell) {
                        let cell_style = TerminalElement::cell_style(
                            &cell,
                            fg,
                            terminal_theme,
                            text_style,
                            font_cache,
                            hyperlink,
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
                    };
                }
            }

            if cur_rect.is_some() {
                rects.push(cur_rect.take().unwrap());
            }
        }
        (cells, rects)
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
        fg: terminal::alacritty_terminal::ansi::Color,
        style: &TerminalStyle,
        text_style: &TextStyle,
        font_cache: &FontCache,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<Point>)>,
    ) -> RunStyle {
        let flags = indexed.cell.flags;
        let fg = convert_color(&fg, &style);

        let mut underline = flags
            .intersects(Flags::ALL_UNDERLINES)
            .then(|| Underline {
                color: Some(fg),
                squiggly: flags.contains(Flags::UNDERCURL),
                thickness: OrderedFloat(1.),
            })
            .unwrap_or_default();

        if indexed.cell.hyperlink().is_some() {
            if underline.thickness == OrderedFloat(0.) {
                underline.thickness = OrderedFloat(1.);
            }
        }

        let mut properties = Properties::new();
        if indexed.flags.intersects(Flags::BOLD | Flags::DIM_BOLD) {
            properties = *properties.weight(Weight::BOLD);
        }
        if indexed.flags.intersects(Flags::ITALIC) {
            properties = *properties.style(Italic);
        }

        let font_id = font_cache
            .select_font(text_style.font_family_id, &properties)
            .unwrap_or(text_style.font_id);

        let mut result = RunStyle {
            color: fg,
            font_id,
            underline,
        };

        if let Some((style, range)) = hyperlink {
            if range.contains(&indexed.point) {
                if let Some(underline) = style.underline {
                    result.underline = underline;
                }

                if let Some(color) = style.color {
                    result.color = color;
                }
            }
        }

        result
    }

    fn generic_button_handler<E>(
        connection: WeakModelHandle<Terminal>,
        origin: Vector2F,
        f: impl Fn(&mut Terminal, Vector2F, E, &mut ModelContext<Terminal>),
    ) -> impl Fn(E, &mut TerminalView, &mut EventContext<TerminalView>) {
        move |event, _: &mut TerminalView, cx| {
            cx.focus_parent();
            if let Some(conn_handle) = connection.upgrade(cx) {
                conn_handle.update(cx, |terminal, cx| {
                    f(terminal, origin, event, cx);

                    cx.notify();
                })
            }
        }
    }

    fn attach_mouse_handlers(
        &self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        mode: TermMode,
        cx: &mut ViewContext<TerminalView>,
    ) {
        let connection = self.terminal;

        let mut region = MouseRegion::new::<Self>(cx.view_id(), 0, visible_bounds);

        // Terminal Emulator controlled behavior:
        region = region
            // Start selections
            .on_down(MouseButton::Left, move |event, v: &mut TerminalView, cx| {
                cx.focus_parent();
                v.context_menu.update(cx, |menu, _cx| menu.delay_cancel());
                if let Some(conn_handle) = connection.upgrade(cx) {
                    conn_handle.update(cx, |terminal, cx| {
                        terminal.mouse_down(&event, origin);

                        cx.notify();
                    })
                }
            })
            // Update drag selections
            .on_drag(MouseButton::Left, move |event, _: &mut TerminalView, cx| {
                if event.end {
                    return;
                }

                if cx.is_self_focused() {
                    if let Some(conn_handle) = connection.upgrade(cx) {
                        conn_handle.update(cx, |terminal, cx| {
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
                    move |terminal, origin, e, cx| {
                        terminal.mouse_up(&e, origin, cx);
                    },
                ),
            )
            // Context menu
            .on_click(
                MouseButton::Right,
                move |event, view: &mut TerminalView, cx| {
                    let mouse_mode = if let Some(conn_handle) = connection.upgrade(cx) {
                        conn_handle.update(cx, |terminal, _cx| terminal.mouse_mode(event.shift))
                    } else {
                        // If we can't get the model handle, probably can't deploy the context menu
                        true
                    };
                    if !mouse_mode {
                        view.deploy_context_menu(event.position, cx);
                    }
                },
            )
            .on_move(move |event, _: &mut TerminalView, cx| {
                if cx.is_self_focused() {
                    if let Some(conn_handle) = connection.upgrade(cx) {
                        conn_handle.update(cx, |terminal, cx| {
                            terminal.mouse_move(&event, origin);
                            cx.notify();
                        })
                    }
                }
            })
            .on_scroll(move |event, _: &mut TerminalView, cx| {
                if let Some(conn_handle) = connection.upgrade(cx) {
                    conn_handle.update(cx, |terminal, cx| {
                        terminal.scroll_wheel(event, origin);
                        cx.notify();
                    })
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
                        move |terminal, origin, e, cx| {
                            terminal.mouse_up(&e, origin, cx);
                        },
                    ),
                )
                .on_up(
                    MouseButton::Middle,
                    TerminalElement::generic_button_handler(
                        connection,
                        origin,
                        move |terminal, origin, e, cx| {
                            terminal.mouse_up(&e, origin, cx);
                        },
                    ),
                )
        }

        scene.push_mouse_region(region);
    }
}

impl Element<TerminalView> for TerminalElement {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut TerminalView,
        cx: &mut LayoutContext<TerminalView>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let settings = settings::get::<ThemeSettings>(cx);
        let terminal_settings = settings::get::<TerminalSettings>(cx);

        //Setup layout information
        let terminal_theme = settings.theme.terminal.clone(); //TODO: Try to minimize this clone.
        let link_style = settings.theme.editor.link_definition;
        let tooltip_style = settings.theme.tooltip.clone();

        let font_cache = cx.font_cache();
        let font_size = terminal_settings
            .font_size(cx)
            .unwrap_or(settings.buffer_font_size(cx));
        let font_family_name = terminal_settings
            .font_family
            .as_ref()
            .unwrap_or(&settings.buffer_font_family_name);
        let font_features = terminal_settings
            .font_features
            .as_ref()
            .unwrap_or(&settings.buffer_font_features);
        let family_id = font_cache
            .load_family(&[font_family_name], &font_features)
            .log_err()
            .unwrap_or(settings.buffer_font_family);
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();

        let text_style = TextStyle {
            color: settings.theme.editor.text_color,
            font_family_id: family_id,
            font_family_name: font_cache.family_name(family_id).unwrap(),
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
            soft_wrap: false,
        };
        let selection_color = settings.theme.editor.selection.selection;
        let match_color = settings.theme.search.match_background;
        let gutter;
        let dimensions = {
            let line_height = text_style.font_size * terminal_settings.line_height.value();
            let cell_width = font_cache.em_advance(text_style.font_id, text_style.font_size);
            gutter = cell_width;

            let size = constraint.max - vec2f(gutter, 0.);
            TerminalSize::new(line_height, cell_width, size)
        };

        let search_matches = if let Some(terminal_model) = self.terminal.upgrade(cx) {
            terminal_model.read(cx).matches.clone()
        } else {
            Default::default()
        };

        let background_color = terminal_theme.background;
        let terminal_handle = self.terminal.upgrade(cx).unwrap();

        let last_hovered_word = terminal_handle.update(cx, |terminal, cx| {
            terminal.set_size(dimensions);
            terminal.try_sync(cx);
            if self.can_navigate_to_selected_word && terminal.can_navigate_to_selected_word() {
                terminal.last_content.last_hovered_word.clone()
            } else {
                None
            }
        });

        let hyperlink_tooltip = last_hovered_word.clone().map(|hovered_word| {
            let mut tooltip = Overlay::new(
                Empty::new()
                    .contained()
                    .constrained()
                    .with_width(dimensions.width())
                    .with_height(dimensions.height())
                    .with_tooltip::<TerminalElement>(
                        hovered_word.id,
                        hovered_word.word,
                        None,
                        tooltip_style,
                        cx,
                    ),
            )
            .with_position_mode(gpui::elements::OverlayPositionMode::Local)
            .into_any();

            tooltip.layout(
                SizeConstraint::new(Vector2F::zero(), cx.window_size()),
                view,
                cx,
            );
            tooltip
        });

        let TerminalContent {
            cells,
            mode,
            display_offset,
            cursor_char,
            selection,
            cursor,
            ..
        } = { &terminal_handle.read(cx).last_content };

        // searches, highlights to a single range representations
        let mut relative_highlighted_ranges = Vec::new();
        for search_match in search_matches {
            relative_highlighted_ranges.push((search_match, match_color))
        }
        if let Some(selection) = selection {
            relative_highlighted_ranges.push((selection.start..=selection.end, selection_color));
        }

        // then have that representation be converted to the appropriate highlight data structure

        let (cells, rects) = TerminalElement::layout_grid(
            cells,
            &text_style,
            &terminal_theme,
            cx.text_layout_cache(),
            cx.font_cache(),
            last_hovered_word
                .as_ref()
                .map(|last_hovered_word| (link_style, &last_hovered_word.word_match)),
        );

        //Layout cursor. Rectangle is used for IME, so we should lay it out even
        //if we don't end up showing it.
        let cursor = if let AlacCursorShape::Hidden = cursor.shape {
            None
        } else {
            let cursor_point = DisplayCursor::from(cursor.point, *display_offset);
            let cursor_text = {
                let str_trxt = cursor_char.to_string();

                let color = if self.focused {
                    terminal_theme.background
                } else {
                    terminal_theme.foreground
                };

                cx.text_layout_cache().layout_str(
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

            let focused = self.focused;
            TerminalElement::shape_cursor(cursor_point, dimensions, &cursor_text).map(
                move |(cursor_position, block_width)| {
                    let (shape, text) = match cursor.shape {
                        AlacCursorShape::Block if !focused => (CursorShape::Hollow, None),
                        AlacCursorShape::Block => (CursorShape::Block, Some(cursor_text)),
                        AlacCursorShape::Underline => (CursorShape::Underscore, None),
                        AlacCursorShape::Beam => (CursorShape::Bar, None),
                        AlacCursorShape::HollowBlock => (CursorShape::Hollow, None),
                        //This case is handled in the if wrapping the whole cursor layout
                        AlacCursorShape::Hidden => unreachable!(),
                    };

                    Cursor::new(
                        cursor_position,
                        block_width,
                        dimensions.line_height,
                        terminal_theme.cursor,
                        shape,
                        text,
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
                size: dimensions,
                rects,
                relative_highlighted_ranges,
                mode: *mode,
                display_offset: *display_offset,
                hyperlink_tooltip,
                gutter,
            },
        )
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        view: &mut TerminalView,
        cx: &mut PaintContext<TerminalView>,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        //Setup element stuff
        let clip_bounds = Some(visible_bounds);

        scene.paint_layer(clip_bounds, |scene| {
            let origin = bounds.origin() + vec2f(layout.gutter, 0.);

            // Elements are ephemeral, only at paint time do we know what could be clicked by a mouse
            self.attach_mouse_handlers(scene, origin, visible_bounds, layout.mode, cx);

            scene.push_cursor_region(gpui::CursorRegion {
                bounds,
                style: if layout.hyperlink_tooltip.is_some() {
                    CursorStyle::PointingHand
                } else {
                    CursorStyle::IBeam
                },
            });

            scene.paint_layer(clip_bounds, |scene| {
                //Start with a background color
                scene.push_quad(Quad {
                    bounds: RectF::new(bounds.origin(), bounds.size()),
                    background: Some(layout.background_color),
                    border: Default::default(),
                    corner_radii: Default::default(),
                });

                for rect in &layout.rects {
                    rect.paint(scene, origin, layout, view, cx)
                }
            });

            //Draw Highlighted Backgrounds
            scene.paint_layer(clip_bounds, |scene| {
                for (relative_highlighted_range, color) in layout.relative_highlighted_ranges.iter()
                {
                    if let Some((start_y, highlighted_range_lines)) =
                        to_highlighted_range_lines(relative_highlighted_range, layout, origin)
                    {
                        let hr = HighlightedRange {
                            start_y, //Need to change this
                            line_height: layout.size.line_height,
                            lines: highlighted_range_lines,
                            color: color.clone(),
                            //Copied from editor. TODO: move to theme or something
                            corner_radius: 0.15 * layout.size.line_height,
                        };
                        hr.paint(bounds, scene);
                    }
                }
            });

            //Draw the text cells
            scene.paint_layer(clip_bounds, |scene| {
                for cell in &layout.cells {
                    cell.paint(scene, origin, layout, visible_bounds, view, cx);
                }
            });

            //Draw cursor
            if self.cursor_visible {
                if let Some(cursor) = &layout.cursor {
                    scene.paint_layer(clip_bounds, |scene| {
                        cursor.paint(scene, origin, cx);
                    })
                }
            }

            if let Some(element) = &mut layout.hyperlink_tooltip {
                element.paint(scene, origin, visible_bounds, view, cx)
            }
        });
    }

    fn metadata(&self) -> Option<&dyn std::any::Any> {
        None
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &TerminalView,
        _: &gpui::ViewContext<TerminalView>,
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
        _: &TerminalView,
        _: &gpui::ViewContext<TerminalView>,
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

fn is_blank(cell: &IndexedCell) -> bool {
    if cell.c != ' ' {
        return false;
    }

    if cell.bg != AnsiColor::Named(NamedColor::Background) {
        return false;
    }

    if cell.hyperlink().is_some() {
        return false;
    }

    if cell
        .flags
        .intersects(Flags::ALL_UNDERLINES | Flags::INVERSE | Flags::STRIKEOUT)
    {
        return false;
    }

    return true;
}

fn to_highlighted_range_lines(
    range: &RangeInclusive<Point>,
    layout: &LayoutState,
    origin: Vector2F,
) -> Option<(f32, Vec<HighlightedRangeLine>)> {
    // Step 1. Normalize the points to be viewport relative.
    // When display_offset = 1, here's how the grid is arranged:
    //-2,0 -2,1...
    //--- Viewport top
    //-1,0 -1,1...
    //--------- Terminal Top
    // 0,0  0,1...
    // 1,0  1,1...
    //--- Viewport Bottom
    // 2,0  2,1...
    //--------- Terminal Bottom

    // Normalize to viewport relative, from terminal relative.
    // lines are i32s, which are negative above the top left corner of the terminal
    // If the user has scrolled, we use the display_offset to tell us which offset
    // of the grid data we should be looking at. But for the rendering step, we don't
    // want negatives. We want things relative to the 'viewport' (the area of the grid
    // which is currently shown according to the display offset)
    let unclamped_start = Point::new(
        range.start().line + layout.display_offset,
        range.start().column,
    );
    let unclamped_end = Point::new(range.end().line + layout.display_offset, range.end().column);

    // Step 2. Clamp range to viewport, and return None if it doesn't overlap
    if unclamped_end.line.0 < 0 || unclamped_start.line.0 > layout.size.num_lines() as i32 {
        return None;
    }

    let clamped_start_line = unclamped_start.line.0.max(0) as usize;
    let clamped_end_line = unclamped_end.line.0.min(layout.size.num_lines() as i32) as usize;
    //Convert the start of the range to pixels
    let start_y = origin.y() + clamped_start_line as f32 * layout.size.line_height;

    // Step 3. Expand ranges that cross lines into a collection of single-line ranges.
    //  (also convert to pixels)
    let mut highlighted_range_lines = Vec::new();
    for line in clamped_start_line..=clamped_end_line {
        let mut line_start = 0;
        let mut line_end = layout.size.columns();

        if line == clamped_start_line {
            line_start = unclamped_start.column.0 as usize;
        }
        if line == clamped_end_line {
            line_end = unclamped_end.column.0 as usize + 1; //+1 for inclusive
        }

        highlighted_range_lines.push(HighlightedRangeLine {
            start_x: origin.x() + line_start as f32 * layout.size.cell_width,
            end_x: origin.x() + line_end as f32 * layout.size.cell_width,
        });
    }

    Some((start_y, highlighted_range_lines))
}
