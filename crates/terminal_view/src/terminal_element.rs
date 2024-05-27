use editor::{CursorLayout, HighlightedRange, HighlightedRangeLine};
use gpui::{
    div, fill, point, px, relative, AnyElement, Bounds, DispatchPhase, Element, ElementId,
    FocusHandle, Font, FontStyle, FontWeight, GlobalElementId, HighlightStyle, Hitbox, Hsla,
    InputHandler, InteractiveElement, Interactivity, IntoElement, LayoutId, Model, ModelContext,
    ModifiersChangedEvent, MouseButton, MouseMoveEvent, Pixels, Point, ShapedLine,
    StatefulInteractiveElement, StrikethroughStyle, Styled, TextRun, TextStyle, UnderlineStyle,
    WeakView, WhiteSpace, WindowContext, WindowTextSystem,
};
use itertools::Itertools;
use language::CursorShape;
use settings::Settings;
use terminal::{
    alacritty_terminal::{
        grid::Dimensions,
        index::Point as AlacPoint,
        term::{cell::Flags, TermMode},
        vte::ansi::{
            Color::{self as AnsiColor, Named},
            CursorShape as AlacCursorShape, NamedColor,
        },
    },
    terminal_settings::TerminalSettings,
    HoveredWord, IndexedCell, Terminal, TerminalContent, TerminalSize,
};
use theme::{ActiveTheme, Theme, ThemeSettings};
use ui::Tooltip;
use workspace::Workspace;

use std::mem;
use std::{fmt::Debug, ops::RangeInclusive};

/// The information generated during layout that is necessary for painting.
pub struct LayoutState {
    hitbox: Hitbox,
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    relative_highlighted_ranges: Vec<(RangeInclusive<AlacPoint>, Hsla)>,
    cursor: Option<CursorLayout>,
    background_color: Hsla,
    dimensions: TerminalSize,
    mode: TermMode,
    display_offset: usize,
    hyperlink_tooltip: Option<AnyElement>,
    gutter: Pixels,
    last_hovered_word: Option<HoveredWord>,
}

/// Helper struct for converting data between Alacritty's cursor points, and displayed cursor points.
struct DisplayCursor {
    line: i32,
    col: usize,
}

impl DisplayCursor {
    fn from(cursor_point: AlacPoint, display_offset: usize) -> Self {
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

#[derive(Debug, Default)]
struct LayoutCell {
    point: AlacPoint<i32, i32>,
    text: gpui::ShapedLine,
}

impl LayoutCell {
    fn new(point: AlacPoint<i32, i32>, text: gpui::ShapedLine) -> LayoutCell {
        LayoutCell { point, text }
    }

    fn paint(
        &self,
        origin: Point<Pixels>,
        layout: &LayoutState,
        _visible_bounds: Bounds<Pixels>,
        cx: &mut WindowContext,
    ) {
        let pos = {
            let point = self.point;

            Point::new(
                (origin.x + point.column as f32 * layout.dimensions.cell_width).floor(),
                origin.y + point.line as f32 * layout.dimensions.line_height,
            )
        };

        self.text.paint(pos, layout.dimensions.line_height, cx).ok();
    }
}

#[derive(Clone, Debug, Default)]
struct LayoutRect {
    point: AlacPoint<i32, i32>,
    num_of_cells: usize,
    color: Hsla,
}

impl LayoutRect {
    fn new(point: AlacPoint<i32, i32>, num_of_cells: usize, color: Hsla) -> LayoutRect {
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

    fn paint(&self, origin: Point<Pixels>, layout: &LayoutState, cx: &mut WindowContext) {
        let position = {
            let alac_point = self.point;
            point(
                (origin.x + alac_point.column as f32 * layout.dimensions.cell_width).floor(),
                origin.y + alac_point.line as f32 * layout.dimensions.line_height,
            )
        };
        let size = point(
            (layout.dimensions.cell_width * self.num_of_cells as f32).ceil(),
            layout.dimensions.line_height,
        )
        .into();

        cx.paint_quad(fill(Bounds::new(position, size), self.color));
    }
}

/// The GPUI element that paints the terminal.
/// We need to keep a reference to the view for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalElement {
    terminal: Model<Terminal>,
    workspace: WeakView<Workspace>,
    focus: FocusHandle,
    focused: bool,
    cursor_visible: bool,
    can_navigate_to_selected_word: bool,
    interactivity: Interactivity,
}

impl InteractiveElement for TerminalElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl StatefulInteractiveElement for TerminalElement {}

impl TerminalElement {
    pub fn new(
        terminal: Model<Terminal>,
        workspace: WeakView<Workspace>,
        focus: FocusHandle,
        focused: bool,
        cursor_visible: bool,
        can_navigate_to_selected_word: bool,
    ) -> TerminalElement {
        TerminalElement {
            terminal,
            workspace,
            focused,
            focus: focus.clone(),
            cursor_visible,
            can_navigate_to_selected_word,
            interactivity: Default::default(),
        }
        .track_focus(&focus)
        .element
    }

    //Vec<Range<AlacPoint>> -> Clip out the parts of the ranges

    fn layout_grid(
        grid: &Vec<IndexedCell>,
        text_style: &TextStyle,
        // terminal_theme: &TerminalStyle,
        text_system: &WindowTextSystem,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
        cx: &WindowContext<'_>,
    ) -> (Vec<LayoutCell>, Vec<LayoutRect>) {
        let theme = cx.theme();
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
                                    // `cur_rect` can be None if it was moved to the `rects` vec after wrapping around
                                    // from one line to the next. The variables are all set correctly but there is no current
                                    // rect, so we create one if necessary.
                                    cur_rect = cur_rect.map_or_else(
                                        || {
                                            Some(LayoutRect::new(
                                                AlacPoint::new(
                                                    line_index as i32,
                                                    cell.point.column.0 as i32,
                                                ),
                                                1,
                                                convert_color(&bg, theme),
                                            ))
                                        },
                                        |rect| Some(rect.extend()),
                                    );
                                } else {
                                    cur_alac_color = Some(bg);
                                    if cur_rect.is_some() {
                                        rects.push(cur_rect.take().unwrap());
                                    }
                                    cur_rect = Some(LayoutRect::new(
                                        AlacPoint::new(
                                            line_index as i32,
                                            cell.point.column.0 as i32,
                                        ),
                                        1,
                                        convert_color(&bg, theme),
                                    ));
                                }
                            }
                            None => {
                                cur_alac_color = Some(bg);
                                cur_rect = Some(LayoutRect::new(
                                    AlacPoint::new(line_index as i32, cell.point.column.0 as i32),
                                    1,
                                    convert_color(&bg, &theme),
                                ));
                            }
                        }
                    }
                }

                //Layout current cell text
                {
                    if !is_blank(&cell) {
                        let cell_text = cell.c.to_string();
                        let cell_style =
                            TerminalElement::cell_style(&cell, fg, theme, text_style, hyperlink);

                        let layout_cell = text_system
                            .shape_line(
                                cell_text.into(),
                                text_style.font_size.to_pixels(cx.rem_size()),
                                &[cell_style],
                            )
                            .unwrap();

                        cells.push(LayoutCell::new(
                            AlacPoint::new(line_index as i32, cell.point.column.0 as i32),
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

    /// Computes the cursor position and expected block width, may return a zero width if x_for_index returns
    /// the same position for sequential indexes. Use em_width instead
    fn shape_cursor(
        cursor_point: DisplayCursor,
        size: TerminalSize,
        text_fragment: &ShapedLine,
    ) -> Option<(Point<Pixels>, Pixels)> {
        if cursor_point.line() < size.total_lines() as i32 {
            let cursor_width = if text_fragment.width == Pixels::ZERO {
                size.cell_width()
            } else {
                text_fragment.width
            };

            // Cursor should always surround as much of the text as possible,
            // hence when on pixel boundaries round the origin down and the width up
            Some((
                point(
                    (cursor_point.col() as f32 * size.cell_width()).floor(),
                    (cursor_point.line() as f32 * size.line_height()).floor(),
                ),
                cursor_width.ceil(),
            ))
        } else {
            None
        }
    }

    /// Converts the Alacritty cell styles to GPUI text styles and background color.
    fn cell_style(
        indexed: &IndexedCell,
        fg: terminal::alacritty_terminal::vte::ansi::Color,
        // bg: terminal::alacritty_terminal::ansi::Color,
        colors: &Theme,
        text_style: &TextStyle,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
    ) -> TextRun {
        let flags = indexed.cell.flags;
        let mut fg = convert_color(&fg, &colors);

        // Ghostty uses (175/255) as the multiplier (~0.69), Alacritty uses 0.66, Kitty
        // uses 0.75. We're using 0.7 because it's pretty well in the middle of that.
        if flags.intersects(Flags::DIM) {
            fg.a *= 0.7;
        }

        let underline = (flags.intersects(Flags::ALL_UNDERLINES)
            || indexed.cell.hyperlink().is_some())
        .then(|| UnderlineStyle {
            color: Some(fg),
            thickness: Pixels::from(1.0),
            wavy: flags.contains(Flags::UNDERCURL),
        });

        let strikethrough = flags
            .intersects(Flags::STRIKEOUT)
            .then(|| StrikethroughStyle {
                color: Some(fg),
                thickness: Pixels::from(1.0),
            });

        let weight = if flags.intersects(Flags::BOLD) {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };

        let style = if flags.intersects(Flags::ITALIC) {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        };

        let mut result = TextRun {
            len: indexed.c.len_utf8(),
            color: fg,
            background_color: None,
            font: Font {
                weight,
                style,
                ..text_style.font()
            },
            underline,
            strikethrough,
        };

        if let Some((style, range)) = hyperlink {
            if range.contains(&indexed.point) {
                if let Some(underline) = style.underline {
                    result.underline = Some(underline);
                }

                if let Some(color) = style.color {
                    result.color = color;
                }
            }
        }

        result
    }

    fn generic_button_handler<E>(
        connection: Model<Terminal>,
        origin: Point<Pixels>,
        focus_handle: FocusHandle,
        f: impl Fn(&mut Terminal, Point<Pixels>, &E, &mut ModelContext<Terminal>),
    ) -> impl Fn(&E, &mut WindowContext) {
        move |event, cx| {
            cx.focus(&focus_handle);
            connection.update(cx, |terminal, cx| {
                f(terminal, origin, event, cx);

                cx.notify();
            })
        }
    }

    fn register_mouse_listeners(
        &mut self,
        origin: Point<Pixels>,
        mode: TermMode,
        hitbox: &Hitbox,
        cx: &mut WindowContext,
    ) {
        let focus = self.focus.clone();
        let terminal = self.terminal.clone();

        self.interactivity.on_mouse_down(MouseButton::Left, {
            let terminal = terminal.clone();
            let focus = focus.clone();
            move |e, cx| {
                cx.focus(&focus);
                terminal.update(cx, |terminal, cx| {
                    terminal.mouse_down(&e, origin, cx);
                    cx.notify();
                })
            }
        });

        cx.on_mouse_event({
            let focus = self.focus.clone();
            let terminal = self.terminal.clone();
            let hitbox = hitbox.clone();
            move |e: &MouseMoveEvent, phase, cx| {
                if phase != DispatchPhase::Bubble || !focus.is_focused(cx) {
                    return;
                }

                if e.pressed_button.is_some() && !cx.has_active_drag() {
                    let hovered = hitbox.is_hovered(cx);
                    terminal.update(cx, |terminal, cx| {
                        if terminal.selection_started() {
                            terminal.mouse_drag(e, origin, hitbox.bounds);
                            cx.notify();
                        } else {
                            if hovered {
                                terminal.mouse_drag(e, origin, hitbox.bounds);
                                cx.notify();
                            }
                        }
                    })
                }

                if hitbox.is_hovered(cx) {
                    terminal.update(cx, |terminal, cx| {
                        terminal.mouse_move(&e, origin);
                        cx.notify();
                    })
                }
            }
        });

        self.interactivity.on_mouse_up(
            MouseButton::Left,
            TerminalElement::generic_button_handler(
                terminal.clone(),
                origin,
                focus.clone(),
                move |terminal, origin, e, cx| {
                    terminal.mouse_up(&e, origin, cx);
                },
            ),
        );
        self.interactivity.on_mouse_down(
            MouseButton::Middle,
            TerminalElement::generic_button_handler(
                terminal.clone(),
                origin,
                focus.clone(),
                move |terminal, origin, e, cx| {
                    terminal.mouse_down(&e, origin, cx);
                },
            ),
        );
        self.interactivity.on_scroll_wheel({
            let terminal = terminal.clone();
            move |e, cx| {
                terminal.update(cx, |terminal, cx| {
                    terminal.scroll_wheel(e, origin);
                    cx.notify();
                })
            }
        });

        // Mouse mode handlers:
        // All mouse modes need the extra click handlers
        if mode.intersects(TermMode::MOUSE_MODE) {
            self.interactivity.on_mouse_down(
                MouseButton::Right,
                TerminalElement::generic_button_handler(
                    terminal.clone(),
                    origin,
                    focus.clone(),
                    move |terminal, origin, e, cx| {
                        terminal.mouse_down(&e, origin, cx);
                    },
                ),
            );
            self.interactivity.on_mouse_up(
                MouseButton::Right,
                TerminalElement::generic_button_handler(
                    terminal.clone(),
                    origin,
                    focus.clone(),
                    move |terminal, origin, e, cx| {
                        terminal.mouse_up(&e, origin, cx);
                    },
                ),
            );
            self.interactivity.on_mouse_up(
                MouseButton::Middle,
                TerminalElement::generic_button_handler(
                    terminal,
                    origin,
                    focus,
                    move |terminal, origin, e, cx| {
                        terminal.mouse_up(&e, origin, cx);
                    },
                ),
            );
        }
    }
}

impl Element for TerminalElement {
    type RequestLayoutState = ();
    type PrepaintState = LayoutState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.interactivity.occlude_mouse();
        let layout_id = self
            .interactivity
            .request_layout(global_id, cx, |mut style, cx| {
                style.size.width = relative(1.).into();
                style.size.height = relative(1.).into();
                let layout_id = cx.request_layout(style, None);

                layout_id
            });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        self.interactivity
            .prepaint(global_id, bounds, bounds.size, cx, |_, _, hitbox, cx| {
                let hitbox = hitbox.unwrap();
                let settings = ThemeSettings::get_global(cx).clone();

                let buffer_font_size = settings.buffer_font_size(cx);

                let terminal_settings = TerminalSettings::get_global(cx);
                let font_family = terminal_settings
                    .font_family
                    .as_ref()
                    .map(|string| string.clone().into())
                    .unwrap_or(settings.buffer_font.family);

                let font_features = terminal_settings
                    .font_features
                    .clone()
                    .unwrap_or(settings.buffer_font.features.clone());

                let font_weight = terminal_settings.font_weight.unwrap_or_default();

                let line_height = terminal_settings.line_height.value();
                let font_size = terminal_settings.font_size;

                let font_size =
                    font_size.map_or(buffer_font_size, |size| theme::adjusted_font_size(size, cx));

                let theme = cx.theme().clone();

                let link_style = HighlightStyle {
                    color: Some(theme.colors().link_text_hover),
                    font_weight: None,
                    font_style: None,
                    background_color: None,
                    underline: Some(UnderlineStyle {
                        thickness: px(1.0),
                        color: Some(theme.colors().link_text_hover),
                        wavy: false,
                    }),
                    strikethrough: None,
                    fade_out: None,
                };

                let text_style = TextStyle {
                    font_family,
                    font_features,
                    font_weight,
                    font_size: font_size.into(),
                    font_style: FontStyle::Normal,
                    line_height: line_height.into(),
                    background_color: None,
                    white_space: WhiteSpace::Normal,
                    // These are going to be overridden per-cell
                    underline: None,
                    strikethrough: None,
                    color: theme.colors().text,
                };

                let text_system = cx.text_system();
                let player_color = theme.players().local();
                let match_color = theme.colors().search_match_background;
                let gutter;
                let dimensions = {
                    let rem_size = cx.rem_size();
                    let font_pixels = text_style.font_size.to_pixels(rem_size);
                    let line_height = font_pixels * line_height.to_pixels(rem_size);
                    let font_id = cx.text_system().resolve_font(&text_style.font());

                    let cell_width = text_system
                        .advance(font_id, font_pixels, 'm')
                        .unwrap()
                        .width;
                    gutter = cell_width;

                    let mut size = bounds.size;
                    size.width -= gutter;

                    // https://github.com/zed-industries/zed/issues/2750
                    // if the terminal is one column wide, rendering 🦀
                    // causes alacritty to misbehave.
                    if size.width < cell_width * 2.0 {
                        size.width = cell_width * 2.0;
                    }

                    TerminalSize::new(line_height, cell_width, size)
                };

                let search_matches = self.terminal.read(cx).matches.clone();

                let background_color = theme.colors().terminal_background;

                let last_hovered_word = self.terminal.update(cx, |terminal, cx| {
                    terminal.set_size(dimensions);
                    terminal.sync(cx);
                    if self.can_navigate_to_selected_word
                        && terminal.can_navigate_to_selected_word()
                    {
                        terminal.last_content.last_hovered_word.clone()
                    } else {
                        None
                    }
                });

                let hyperlink_tooltip = last_hovered_word.clone().map(|hovered_word| {
                    let offset = bounds.origin + Point::new(gutter, px(0.));
                    let mut element = div()
                        .size_full()
                        .id("terminal-element")
                        .tooltip(move |cx| Tooltip::text(hovered_word.word.clone(), cx))
                        .into_any_element();
                    element.prepaint_as_root(offset, bounds.size.into(), cx);
                    element
                });

                let TerminalContent {
                    cells,
                    mode,
                    display_offset,
                    cursor_char,
                    selection,
                    cursor,
                    ..
                } = &self.terminal.read(cx).last_content;

                // searches, highlights to a single range representations
                let mut relative_highlighted_ranges = Vec::new();
                for search_match in search_matches {
                    relative_highlighted_ranges.push((search_match, match_color))
                }
                if let Some(selection) = selection {
                    relative_highlighted_ranges
                        .push((selection.start..=selection.end, player_color.selection));
                }

                // then have that representation be converted to the appropriate highlight data structure

                let (cells, rects) = TerminalElement::layout_grid(
                    cells,
                    &text_style,
                    &cx.text_system(),
                    last_hovered_word
                        .as_ref()
                        .map(|last_hovered_word| (link_style, &last_hovered_word.word_match)),
                    cx,
                );

                // Layout cursor. Rectangle is used for IME, so we should lay it out even
                // if we don't end up showing it.
                let cursor = if let AlacCursorShape::Hidden = cursor.shape {
                    None
                } else {
                    let cursor_point = DisplayCursor::from(cursor.point, *display_offset);
                    let cursor_text = {
                        let str_trxt = cursor_char.to_string();
                        let len = str_trxt.len();
                        cx.text_system()
                            .shape_line(
                                str_trxt.into(),
                                text_style.font_size.to_pixels(cx.rem_size()),
                                &[TextRun {
                                    len,
                                    font: text_style.font(),
                                    color: theme.colors().terminal_background,
                                    background_color: None,
                                    underline: Default::default(),
                                    strikethrough: None,
                                }],
                            )
                            .unwrap()
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

                            CursorLayout::new(
                                cursor_position,
                                block_width,
                                dimensions.line_height,
                                theme.players().local().cursor,
                                shape,
                                text,
                            )
                        },
                    )
                };

                LayoutState {
                    hitbox,
                    cells,
                    cursor,
                    background_color,
                    dimensions,
                    rects,
                    relative_highlighted_ranges,
                    mode: *mode,
                    display_offset: *display_offset,
                    hyperlink_tooltip,
                    gutter,
                    last_hovered_word,
                }
            })
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        cx: &mut WindowContext<'_>,
    ) {
        cx.paint_quad(fill(bounds, layout.background_color));
        let origin = bounds.origin + Point::new(layout.gutter, px(0.));

        let terminal_input_handler = TerminalInputHandler {
            terminal: self.terminal.clone(),
            cursor_bounds: layout
                .cursor
                .as_ref()
                .map(|cursor| cursor.bounding_rect(origin)),
            workspace: self.workspace.clone(),
        };

        self.register_mouse_listeners(origin, layout.mode, &layout.hitbox, cx);
        if self.can_navigate_to_selected_word && layout.last_hovered_word.is_some() {
            cx.set_cursor_style(gpui::CursorStyle::PointingHand, &layout.hitbox);
        } else {
            cx.set_cursor_style(gpui::CursorStyle::IBeam, &layout.hitbox);
        }

        let cursor = layout.cursor.take();
        let hyperlink_tooltip = layout.hyperlink_tooltip.take();
        self.interactivity
            .paint(global_id, bounds, Some(&layout.hitbox), cx, |_, cx| {
                cx.handle_input(&self.focus, terminal_input_handler);

                cx.on_key_event({
                    let this = self.terminal.clone();
                    move |event: &ModifiersChangedEvent, phase, cx| {
                        if phase != DispatchPhase::Bubble {
                            return;
                        }

                        let handled =
                            this.update(cx, |term, _| term.try_modifiers_change(&event.modifiers));

                        if handled {
                            cx.refresh();
                        }
                    }
                });

                for rect in &layout.rects {
                    rect.paint(origin, &layout, cx);
                }

                for (relative_highlighted_range, color) in layout.relative_highlighted_ranges.iter()
                {
                    if let Some((start_y, highlighted_range_lines)) =
                        to_highlighted_range_lines(relative_highlighted_range, &layout, origin)
                    {
                        let hr = HighlightedRange {
                            start_y, //Need to change this
                            line_height: layout.dimensions.line_height,
                            lines: highlighted_range_lines,
                            color: *color,
                            //Copied from editor. TODO: move to theme or something
                            corner_radius: 0.15 * layout.dimensions.line_height,
                        };
                        hr.paint(bounds, cx);
                    }
                }

                for cell in &layout.cells {
                    cell.paint(origin, &layout, bounds, cx);
                }

                if self.cursor_visible {
                    if let Some(mut cursor) = cursor {
                        cursor.paint(origin, cx);
                    }
                }

                if let Some(mut element) = hyperlink_tooltip {
                    element.paint(cx);
                }
            });
    }
}

impl IntoElement for TerminalElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct TerminalInputHandler {
    terminal: Model<Terminal>,
    workspace: WeakView<Workspace>,
    cursor_bounds: Option<Bounds<Pixels>>,
}

impl InputHandler for TerminalInputHandler {
    fn selected_text_range(&mut self, cx: &mut WindowContext) -> Option<std::ops::Range<usize>> {
        if self
            .terminal
            .read(cx)
            .last_content
            .mode
            .contains(TermMode::ALT_SCREEN)
        {
            None
        } else {
            Some(0..0)
        }
    }

    fn marked_text_range(&mut self, _: &mut WindowContext) -> Option<std::ops::Range<usize>> {
        None
    }

    fn text_for_range(
        &mut self,
        _: std::ops::Range<usize>,
        _: &mut WindowContext,
    ) -> Option<String> {
        None
    }

    fn replace_text_in_range(
        &mut self,
        _replacement_range: Option<std::ops::Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    ) {
        self.terminal.update(cx, |terminal, _| {
            terminal.input(text.into());
        });

        self.workspace
            .update(cx, |this, cx| {
                let telemetry = this.project().read(cx).client().telemetry().clone();
                telemetry.log_edit_event("terminal");
            })
            .ok();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        _new_text: &str,
        _new_selected_range: Option<std::ops::Range<usize>>,
        _: &mut WindowContext,
    ) {
    }

    fn unmark_text(&mut self, _: &mut WindowContext) {}

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        _: &mut WindowContext,
    ) -> Option<Bounds<Pixels>> {
        self.cursor_bounds
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
    range: &RangeInclusive<AlacPoint>,
    layout: &LayoutState,
    origin: Point<Pixels>,
) -> Option<(Pixels, Vec<HighlightedRangeLine>)> {
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
    let unclamped_start = AlacPoint::new(
        range.start().line + layout.display_offset,
        range.start().column,
    );
    let unclamped_end =
        AlacPoint::new(range.end().line + layout.display_offset, range.end().column);

    // Step 2. Clamp range to viewport, and return None if it doesn't overlap
    if unclamped_end.line.0 < 0 || unclamped_start.line.0 > layout.dimensions.num_lines() as i32 {
        return None;
    }

    let clamped_start_line = unclamped_start.line.0.max(0) as usize;
    let clamped_end_line = unclamped_end
        .line
        .0
        .min(layout.dimensions.num_lines() as i32) as usize;
    //Convert the start of the range to pixels
    let start_y = origin.y + clamped_start_line as f32 * layout.dimensions.line_height;

    // Step 3. Expand ranges that cross lines into a collection of single-line ranges.
    //  (also convert to pixels)
    let mut highlighted_range_lines = Vec::new();
    for line in clamped_start_line..=clamped_end_line {
        let mut line_start = 0;
        let mut line_end = layout.dimensions.columns();

        if line == clamped_start_line {
            line_start = unclamped_start.column.0;
        }
        if line == clamped_end_line {
            line_end = unclamped_end.column.0 + 1; // +1 for inclusive
        }

        highlighted_range_lines.push(HighlightedRangeLine {
            start_x: origin.x + line_start as f32 * layout.dimensions.cell_width,
            end_x: origin.x + line_end as f32 * layout.dimensions.cell_width,
        });
    }

    Some((start_y, highlighted_range_lines))
}

/// Converts a 2, 8, or 24 bit color ANSI color to the GPUI equivalent.
fn convert_color(fg: &terminal::alacritty_terminal::vte::ansi::Color, theme: &Theme) -> Hsla {
    let colors = theme.colors();
    match fg {
        // Named and theme defined colors
        terminal::alacritty_terminal::vte::ansi::Color::Named(n) => match n {
            NamedColor::Black => colors.terminal_ansi_black,
            NamedColor::Red => colors.terminal_ansi_red,
            NamedColor::Green => colors.terminal_ansi_green,
            NamedColor::Yellow => colors.terminal_ansi_yellow,
            NamedColor::Blue => colors.terminal_ansi_blue,
            NamedColor::Magenta => colors.terminal_ansi_magenta,
            NamedColor::Cyan => colors.terminal_ansi_cyan,
            NamedColor::White => colors.terminal_ansi_white,
            NamedColor::BrightBlack => colors.terminal_ansi_bright_black,
            NamedColor::BrightRed => colors.terminal_ansi_bright_red,
            NamedColor::BrightGreen => colors.terminal_ansi_bright_green,
            NamedColor::BrightYellow => colors.terminal_ansi_bright_yellow,
            NamedColor::BrightBlue => colors.terminal_ansi_bright_blue,
            NamedColor::BrightMagenta => colors.terminal_ansi_bright_magenta,
            NamedColor::BrightCyan => colors.terminal_ansi_bright_cyan,
            NamedColor::BrightWhite => colors.terminal_ansi_bright_white,
            NamedColor::Foreground => colors.text,
            NamedColor::Background => colors.background,
            NamedColor::Cursor => theme.players().local().cursor,
            NamedColor::DimBlack => colors.terminal_ansi_dim_black,
            NamedColor::DimRed => colors.terminal_ansi_dim_red,
            NamedColor::DimGreen => colors.terminal_ansi_dim_green,
            NamedColor::DimYellow => colors.terminal_ansi_dim_yellow,
            NamedColor::DimBlue => colors.terminal_ansi_dim_blue,
            NamedColor::DimMagenta => colors.terminal_ansi_dim_magenta,
            NamedColor::DimCyan => colors.terminal_ansi_dim_cyan,
            NamedColor::DimWhite => colors.terminal_ansi_dim_white,
            NamedColor::BrightForeground => colors.terminal_bright_foreground,
            NamedColor::DimForeground => colors.terminal_dim_foreground,
        },
        // 'True' colors
        terminal::alacritty_terminal::vte::ansi::Color::Spec(rgb) => {
            terminal::rgba_color(rgb.r, rgb.g, rgb.b)
        }
        // 8 bit, indexed colors
        terminal::alacritty_terminal::vte::ansi::Color::Indexed(i) => {
            terminal::get_color_at_index(*i as usize, theme)
        }
    }
}
