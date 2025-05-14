use editor::{CursorLayout, HighlightedRange, HighlightedRangeLine};
use gpui::{
    AnyElement, App, AvailableSpace, Bounds, ContentMask, Context, DispatchPhase, Element,
    ElementId, Entity, FocusHandle, Font, FontStyle, FontWeight, GlobalElementId, HighlightStyle,
    Hitbox, Hsla, InputHandler, InteractiveElement, Interactivity, IntoElement, LayoutId,
    ModifiersChangedEvent, MouseButton, MouseMoveEvent, Pixels, Point, ShapedLine,
    StatefulInteractiveElement, StrikethroughStyle, Styled, TextRun, TextStyle, UTF16Selection,
    UnderlineStyle, WeakEntity, WhiteSpace, Window, WindowTextSystem, div, fill, point, px,
    relative, size,
};
use itertools::Itertools;
use language::CursorShape;
use settings::Settings;
use terminal::{
    IndexedCell, Terminal, TerminalBounds, TerminalContent,
    alacritty_terminal::{
        grid::Dimensions,
        index::Point as AlacPoint,
        term::{TermMode, cell::Flags},
        vte::ansi::{
            Color::{self as AnsiColor, Named},
            CursorShape as AlacCursorShape, NamedColor,
        },
    },
    terminal_settings::TerminalSettings,
};
use theme::{ActiveTheme, Theme, ThemeSettings};
use ui::{ParentElement, Tooltip};
use util::ResultExt;
use workspace::Workspace;

use std::mem;
use std::{fmt::Debug, ops::RangeInclusive, rc::Rc};

use crate::{BlockContext, BlockProperties, TerminalView};

/// The information generated during layout that is necessary for painting.
pub struct LayoutState {
    hitbox: Hitbox,
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    relative_highlighted_ranges: Vec<(RangeInclusive<AlacPoint>, Hsla)>,
    cursor: Option<CursorLayout>,
    background_color: Hsla,
    dimensions: TerminalBounds,
    mode: TermMode,
    display_offset: usize,
    hyperlink_tooltip: Option<AnyElement>,
    gutter: Pixels,
    block_below_cursor_element: Option<AnyElement>,
    base_text_style: TextStyle,
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
pub struct LayoutCell {
    pub point: AlacPoint<i32, i32>,
    text: gpui::ShapedLine,
}

impl LayoutCell {
    fn new(point: AlacPoint<i32, i32>, text: gpui::ShapedLine) -> LayoutCell {
        LayoutCell { point, text }
    }

    pub fn paint(
        &self,
        origin: Point<Pixels>,
        dimensions: &TerminalBounds,
        _visible_bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let pos = {
            let point = self.point;

            Point::new(
                (origin.x + point.column as f32 * dimensions.cell_width).floor(),
                origin.y + point.line as f32 * dimensions.line_height,
            )
        };

        self.text
            .paint(pos, dimensions.line_height, window, cx)
            .ok();
    }
}

#[derive(Clone, Debug, Default)]
pub struct LayoutRect {
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

    pub fn paint(&self, origin: Point<Pixels>, dimensions: &TerminalBounds, window: &mut Window) {
        let position = {
            let alac_point = self.point;
            point(
                (origin.x + alac_point.column as f32 * dimensions.cell_width).floor(),
                origin.y + alac_point.line as f32 * dimensions.line_height,
            )
        };
        let size = point(
            (dimensions.cell_width * self.num_of_cells as f32).ceil(),
            dimensions.line_height,
        )
        .into();

        window.paint_quad(fill(Bounds::new(position, size), self.color));
    }
}

/// The GPUI element that paints the terminal.
/// We need to keep a reference to the model for mouse events, do we need it for any other terminal stuff, or can we move that to connection?
pub struct TerminalElement {
    terminal: Entity<Terminal>,
    terminal_view: Entity<TerminalView>,
    workspace: WeakEntity<Workspace>,
    focus: FocusHandle,
    focused: bool,
    cursor_visible: bool,
    interactivity: Interactivity,
    block_below_cursor: Option<Rc<BlockProperties>>,
}

impl InteractiveElement for TerminalElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl StatefulInteractiveElement for TerminalElement {}

impl TerminalElement {
    pub fn new(
        terminal: Entity<Terminal>,
        terminal_view: Entity<TerminalView>,
        workspace: WeakEntity<Workspace>,
        focus: FocusHandle,
        focused: bool,
        cursor_visible: bool,
        block_below_cursor: Option<Rc<BlockProperties>>,
    ) -> TerminalElement {
        TerminalElement {
            terminal,
            terminal_view,
            workspace,
            focused,
            focus: focus.clone(),
            cursor_visible,
            block_below_cursor,
            interactivity: Default::default(),
        }
        .track_focus(&focus)
        .element
    }

    //Vec<Range<AlacPoint>> -> Clip out the parts of the ranges

    pub fn layout_grid(
        grid: impl Iterator<Item = IndexedCell>,
        text_style: &TextStyle,
        // terminal_theme: &TerminalStyle,
        text_system: &WindowTextSystem,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
        window: &Window,
        cx: &App,
    ) -> (Vec<LayoutCell>, Vec<LayoutRect>) {
        let theme = cx.theme();
        let mut cells = vec![];
        let mut rects = vec![];

        let mut cur_rect: Option<LayoutRect> = None;
        let mut cur_alac_color = None;

        let linegroups = grid.into_iter().chunk_by(|i| i.point.line);
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
                                    convert_color(&bg, theme),
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
                                text_style.font_size.to_pixels(window.rem_size()),
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
        size: TerminalBounds,
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
        let mut fg = convert_color(&fg, colors);

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
            text_style.font_weight
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
        connection: Entity<Terminal>,
        focus_handle: FocusHandle,
        f: impl Fn(&mut Terminal, &E, &mut Context<Terminal>),
    ) -> impl Fn(&E, &mut Window, &mut App) {
        move |event, window, cx| {
            window.focus(&focus_handle);
            connection.update(cx, |terminal, cx| {
                f(terminal, event, cx);

                cx.notify();
            })
        }
    }

    fn register_mouse_listeners(&mut self, mode: TermMode, hitbox: &Hitbox, window: &mut Window) {
        let focus = self.focus.clone();
        let terminal = self.terminal.clone();
        let terminal_view = self.terminal_view.clone();

        self.interactivity.on_mouse_down(MouseButton::Left, {
            let terminal = terminal.clone();
            let focus = focus.clone();
            let terminal_view = terminal_view.clone();

            move |e, window, cx| {
                window.focus(&focus);

                let scroll_top = terminal_view.read(cx).scroll_top;
                terminal.update(cx, |terminal, cx| {
                    let mut adjusted_event = e.clone();
                    if scroll_top > Pixels::ZERO {
                        adjusted_event.position.y += scroll_top;
                    }
                    terminal.mouse_down(&adjusted_event, cx);
                    cx.notify();
                })
            }
        });

        window.on_mouse_event({
            let terminal = self.terminal.clone();
            let hitbox = hitbox.clone();
            let focus = focus.clone();
            let terminal_view = terminal_view.clone();
            move |e: &MouseMoveEvent, phase, window, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }

                if e.pressed_button.is_some() && !cx.has_active_drag() && focus.is_focused(window) {
                    let hovered = hitbox.is_hovered(window);

                    let scroll_top = terminal_view.read(cx).scroll_top;
                    terminal.update(cx, |terminal, cx| {
                        if terminal.selection_started() || hovered {
                            let mut adjusted_event = e.clone();
                            if scroll_top > Pixels::ZERO {
                                adjusted_event.position.y += scroll_top;
                            }
                            terminal.mouse_drag(&adjusted_event, hitbox.bounds, cx);
                            cx.notify();
                        }
                    })
                }

                if hitbox.is_hovered(window) {
                    terminal.update(cx, |terminal, cx| {
                        terminal.mouse_move(e, cx);
                    })
                }
            }
        });

        self.interactivity.on_mouse_up(
            MouseButton::Left,
            TerminalElement::generic_button_handler(
                terminal.clone(),
                focus.clone(),
                move |terminal, e, cx| {
                    terminal.mouse_up(e, cx);
                },
            ),
        );
        self.interactivity.on_mouse_down(
            MouseButton::Middle,
            TerminalElement::generic_button_handler(
                terminal.clone(),
                focus.clone(),
                move |terminal, e, cx| {
                    terminal.mouse_down(e, cx);
                },
            ),
        );
        self.interactivity.on_scroll_wheel({
            let terminal_view = self.terminal_view.downgrade();
            move |e, _, cx| {
                terminal_view
                    .update(cx, |terminal_view, cx| {
                        terminal_view.scroll_wheel(e, cx);
                        cx.notify();
                    })
                    .ok();
            }
        });

        // Mouse mode handlers:
        // All mouse modes need the extra click handlers
        if mode.intersects(TermMode::MOUSE_MODE) {
            self.interactivity.on_mouse_down(
                MouseButton::Right,
                TerminalElement::generic_button_handler(
                    terminal.clone(),
                    focus.clone(),
                    move |terminal, e, cx| {
                        terminal.mouse_down(e, cx);
                    },
                ),
            );
            self.interactivity.on_mouse_up(
                MouseButton::Right,
                TerminalElement::generic_button_handler(
                    terminal.clone(),
                    focus.clone(),
                    move |terminal, e, cx| {
                        terminal.mouse_up(e, cx);
                    },
                ),
            );
            self.interactivity.on_mouse_up(
                MouseButton::Middle,
                TerminalElement::generic_button_handler(terminal, focus, move |terminal, e, cx| {
                    terminal.mouse_up(e, cx);
                }),
            );
        }
    }

    fn rem_size(&self, cx: &mut App) -> Option<Pixels> {
        let settings = ThemeSettings::get_global(cx).clone();
        let buffer_font_size = settings.buffer_font_size(cx);
        let rem_size_scale = {
            // Our default UI font size is 14px on a 16px base scale.
            // This means the default UI font size is 0.875rems.
            let default_font_size_scale = 14. / ui::BASE_REM_SIZE_IN_PX;

            // We then determine the delta between a single rem and the default font
            // size scale.
            let default_font_size_delta = 1. - default_font_size_scale;

            // Finally, we add this delta to 1rem to get the scale factor that
            // should be used to scale up the UI.
            1. + default_font_size_delta
        };

        Some(buffer_font_size * rem_size_scale)
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
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id =
            self.interactivity
                .request_layout(global_id, window, cx, |mut style, window, cx| {
                    style.size.width = relative(1.).into();
                    style.size.height = relative(1.).into();
                    // style.overflow = point(Overflow::Hidden, Overflow::Hidden);

                    window.request_layout(style, None, cx)
                });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let rem_size = self.rem_size(cx);
        self.interactivity.prepaint(
            global_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_, _, hitbox, window, cx| {
                let hitbox = hitbox.unwrap();
                let settings = ThemeSettings::get_global(cx).clone();

                let buffer_font_size = settings.buffer_font_size(cx);

                let terminal_settings = TerminalSettings::get_global(cx);

                let font_family = terminal_settings
                    .font_family
                    .as_ref()
                    .unwrap_or(&settings.buffer_font.family)
                    .clone();

                let font_fallbacks = terminal_settings
                    .font_fallbacks
                    .as_ref()
                    .or(settings.buffer_font.fallbacks.as_ref())
                    .cloned();

                let font_features = terminal_settings
                    .font_features
                    .as_ref()
                    .unwrap_or(&settings.buffer_font.features)
                    .clone();

                let font_weight = terminal_settings.font_weight.unwrap_or_default();

                let line_height = terminal_settings.line_height.value();
                let font_size = terminal_settings.font_size;

                let font_size =
                    font_size.map_or(buffer_font_size, |size| theme::adjusted_font_size(size, cx));

                let theme = cx.theme().clone();

                let link_style = HighlightStyle {
                    color: Some(theme.colors().link_text_hover),
                    font_weight: Some(font_weight),
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
                    font_fallbacks,
                    font_size: font_size.into(),
                    font_style: FontStyle::Normal,
                    line_height: line_height.into(),
                    background_color: Some(theme.colors().terminal_ansi_background),
                    white_space: WhiteSpace::Normal,
                    // These are going to be overridden per-cell
                    color: theme.colors().terminal_foreground,
                    ..Default::default()
                };

                let text_system = cx.text_system();
                let player_color = theme.players().local();
                let match_color = theme.colors().search_match_background;
                let gutter;
                let dimensions = {
                    let rem_size = window.rem_size();
                    let font_pixels = text_style.font_size.to_pixels(rem_size);
                    // TODO: line_height should be an f32 not an AbsoluteLength.
                    let line_height = font_pixels * line_height.to_pixels(rem_size).0;
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

                    let mut origin = bounds.origin;
                    origin.x += gutter;

                    TerminalBounds::new(line_height, cell_width, Bounds { origin, size })
                };

                let search_matches = self.terminal.read(cx).matches.clone();

                let background_color = theme.colors().terminal_background;

                let (last_hovered_word, hover_target) = self.terminal.update(cx, |terminal, cx| {
                    terminal.set_size(dimensions);
                    terminal.sync(window, cx);

                    if window.modifiers().secondary()
                        && bounds.contains(&window.mouse_position())
                        && self.terminal_view.read(cx).hover_target_tooltip.is_some()
                    {
                        let hover_target = self.terminal_view.read(cx).hover_target_tooltip.clone();
                        let last_hovered_word = terminal.last_content.last_hovered_word.clone();
                        (last_hovered_word, hover_target)
                    } else {
                        (None, None)
                    }
                });

                let scroll_top = self.terminal_view.read(cx).scroll_top;
                let hyperlink_tooltip = hover_target.as_ref().map(|hover_target| {
                    let offset = bounds.origin + point(gutter, px(0.)) - point(px(0.), scroll_top);
                    let mut element = div()
                        .size_full()
                        .id("terminal-element")
                        .tooltip(Tooltip::text(hover_target.clone()))
                        .into_any_element();
                    element.prepaint_as_root(offset, bounds.size.into(), window, cx);
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
                let mode = *mode;
                let display_offset = *display_offset;

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
                    cells.iter().cloned(),
                    &text_style,
                    window.text_system(),
                    last_hovered_word
                        .as_ref()
                        .map(|last_hovered_word| (link_style, &last_hovered_word.word_match)),
                    window,
                    cx,
                );

                // Layout cursor. Rectangle is used for IME, so we should lay it out even
                // if we don't end up showing it.
                let cursor = if let AlacCursorShape::Hidden = cursor.shape {
                    None
                } else {
                    let cursor_point = DisplayCursor::from(cursor.point, display_offset);
                    let cursor_text = {
                        let str_trxt = cursor_char.to_string();
                        let len = str_trxt.len();
                        window
                            .text_system()
                            .shape_line(
                                str_trxt.into(),
                                text_style.font_size.to_pixels(window.rem_size()),
                                &[TextRun {
                                    len,
                                    font: text_style.font(),
                                    color: theme.colors().terminal_ansi_background,
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
                                AlacCursorShape::Underline => (CursorShape::Underline, None),
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

                let block_below_cursor_element = if let Some(block) = &self.block_below_cursor {
                    let terminal = self.terminal.read(cx);
                    if terminal.last_content.display_offset == 0 {
                        let target_line = terminal.last_content.cursor.point.line.0 + 1;
                        let render = &block.render;
                        let mut block_cx = BlockContext {
                            window,
                            context: cx,
                            dimensions,
                        };
                        let element = render(&mut block_cx);
                        let mut element = div().occlude().child(element).into_any_element();
                        let available_space = size(
                            AvailableSpace::Definite(dimensions.width() + gutter),
                            AvailableSpace::Definite(
                                block.height as f32 * dimensions.line_height(),
                            ),
                        );
                        let origin = bounds.origin
                            + point(px(0.), target_line as f32 * dimensions.line_height())
                            - point(px(0.), scroll_top);
                        window.with_rem_size(rem_size, |window| {
                            element.prepaint_as_root(origin, available_space, window, cx);
                        });
                        Some(element)
                    } else {
                        None
                    }
                } else {
                    None
                };

                LayoutState {
                    hitbox,
                    cells,
                    cursor,
                    background_color,
                    dimensions,
                    rects,
                    relative_highlighted_ranges,
                    mode,
                    display_offset,
                    hyperlink_tooltip,
                    gutter,
                    block_below_cursor_element,
                    base_text_style: text_style,
                }
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let scroll_top = self.terminal_view.read(cx).scroll_top;

            window.paint_quad(fill(bounds, layout.background_color));
            let origin =
                bounds.origin + Point::new(layout.gutter, px(0.)) - Point::new(px(0.), scroll_top);

            let marked_text_cloned: Option<String> = {
                let ime_state = self.terminal_view.read(cx);
                ime_state.marked_text.clone()
            };

            let terminal_input_handler = TerminalInputHandler {
                terminal: self.terminal.clone(),
                terminal_view: self.terminal_view.clone(),
                cursor_bounds: layout
                    .cursor
                    .as_ref()
                    .map(|cursor| cursor.bounding_rect(origin)),
                workspace: self.workspace.clone(),
            };

            self.register_mouse_listeners(layout.mode, &layout.hitbox, window);
            if window.modifiers().secondary()
                && bounds.contains(&window.mouse_position())
                && self.terminal_view.read(cx).hover_target_tooltip.is_some()
            {
                window.set_cursor_style(gpui::CursorStyle::PointingHand, Some(&layout.hitbox));
            } else {
                window.set_cursor_style(gpui::CursorStyle::IBeam, Some(&layout.hitbox));
            }

            let original_cursor = layout.cursor.take();
            let hyperlink_tooltip = layout.hyperlink_tooltip.take();
            let block_below_cursor_element = layout.block_below_cursor_element.take();
            self.interactivity.paint(
                global_id,
                bounds,
                Some(&layout.hitbox),
                window,
                cx,
                |_, window, cx| {
                    window.handle_input(&self.focus, terminal_input_handler, cx);

                    window.on_key_event({
                        let this = self.terminal.clone();
                        move |event: &ModifiersChangedEvent, phase, window, cx| {
                            if phase != DispatchPhase::Bubble {
                                return;
                            }

                            this.update(cx, |term, cx| {
                                term.try_modifiers_change(&event.modifiers, window, cx)
                            });
                        }
                    });

                    for rect in &layout.rects {
                        rect.paint(origin, &layout.dimensions, window);
                    }

                    for (relative_highlighted_range, color) in
                        layout.relative_highlighted_ranges.iter()
                    {
                        if let Some((start_y, highlighted_range_lines)) =
                            to_highlighted_range_lines(relative_highlighted_range, layout, origin)
                        {
                            let hr = HighlightedRange {
                                start_y,
                                line_height: layout.dimensions.line_height,
                                lines: highlighted_range_lines,
                                color: *color,
                                corner_radius: 0.15 * layout.dimensions.line_height,
                            };
                            hr.paint(bounds, window);
                        }
                    }

                    for cell in &layout.cells {
                        cell.paint(origin, &layout.dimensions, bounds, window, cx);
                    }

                    if let Some(text_to_mark) = &marked_text_cloned {
                        if !text_to_mark.is_empty() {
                            if let Some(cursor_layout) = &original_cursor {
                                let ime_position = cursor_layout.bounding_rect(origin).origin;
                                let mut ime_style = layout.base_text_style.clone();
                                ime_style.underline = Some(UnderlineStyle {
                                    color: Some(ime_style.color),
                                    thickness: px(1.0),
                                    wavy: false,
                                });

                                let shaped_line = window
                                    .text_system()
                                    .shape_line(
                                        text_to_mark.clone().into(),
                                        ime_style.font_size.to_pixels(window.rem_size()),
                                        &[TextRun {
                                            len: text_to_mark.len(),
                                            font: ime_style.font(),
                                            color: ime_style.color,
                                            background_color: None,
                                            underline: ime_style.underline,
                                            strikethrough: None,
                                        }],
                                    )
                                    .unwrap();
                                shaped_line
                                    .paint(ime_position, layout.dimensions.line_height, window, cx)
                                    .log_err();
                            }
                        }
                    }

                    if self.cursor_visible && marked_text_cloned.is_none() {
                        if let Some(mut cursor) = original_cursor {
                            cursor.paint(origin, window, cx);
                        }
                    }

                    if let Some(mut element) = block_below_cursor_element {
                        element.paint(window, cx);
                    }

                    if let Some(mut element) = hyperlink_tooltip {
                        element.paint(window, cx);
                    }
                },
            );
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
    terminal: Entity<Terminal>,
    terminal_view: Entity<TerminalView>,
    workspace: WeakEntity<Workspace>,
    cursor_bounds: Option<Bounds<Pixels>>,
}

impl InputHandler for TerminalInputHandler {
    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _: &mut Window,
        cx: &mut App,
    ) -> Option<UTF16Selection> {
        if self
            .terminal
            .read(cx)
            .last_content
            .mode
            .contains(TermMode::ALT_SCREEN)
        {
            None
        } else {
            Some(UTF16Selection {
                range: 0..0,
                reversed: false,
            })
        }
    }

    fn marked_text_range(
        &mut self,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<std::ops::Range<usize>> {
        self.terminal_view.read(cx).marked_text_range()
    }

    fn text_for_range(
        &mut self,
        _: std::ops::Range<usize>,
        _: &mut Option<std::ops::Range<usize>>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<String> {
        None
    }

    fn replace_text_in_range(
        &mut self,
        _replacement_range: Option<std::ops::Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.terminal_view.update(cx, |view, view_cx| {
            view.clear_marked_text(view_cx);
            view.commit_text(text, view_cx);
        });

        self.workspace
            .update(cx, |this, cx| {
                window.invalidate_character_coordinates();
                let project = this.project().read(cx);
                let telemetry = project.client().telemetry().clone();
                telemetry.log_edit_event("terminal", project.is_via_ssh());
            })
            .ok();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_marked_range: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(range) = new_marked_range {
            self.terminal_view.update(cx, |view, view_cx| {
                view.set_marked_text(new_text.to_string(), range, view_cx);
            });
        }
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut App) {
        self.terminal_view.update(cx, |view, view_cx| {
            view.clear_marked_text(view_cx);
        });
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>> {
        let term_bounds = self.terminal_view.read(cx).terminal_bounds(cx);

        let mut bounds = self.cursor_bounds?;
        let offset_x = term_bounds.cell_width * range_utf16.start as f32;
        bounds.origin.x += offset_x;

        Some(bounds)
    }

    fn apple_press_and_hold_enabled(&mut self) -> bool {
        false
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<usize> {
        None
    }
}

pub fn is_blank(cell: &IndexedCell) -> bool {
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

    true
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
pub fn convert_color(fg: &terminal::alacritty_terminal::vte::ansi::Color, theme: &Theme) -> Hsla {
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
            NamedColor::Foreground => colors.terminal_foreground,
            NamedColor::Background => colors.terminal_ansi_background,
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
