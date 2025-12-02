use editor::{CursorLayout, EditorSettings, HighlightedRange, HighlightedRangeLine};
use gpui::{
    AbsoluteLength, AnyElement, App, AvailableSpace, Bounds, ContentMask, Context, DispatchPhase,
    Element, ElementId, Entity, FocusHandle, Font, FontFeatures, FontStyle, FontWeight,
    GlobalElementId, HighlightStyle, Hitbox, Hsla, InputHandler, InteractiveElement, Interactivity,
    IntoElement, LayoutId, Length, ModifiersChangedEvent, MouseButton, MouseMoveEvent, Pixels,
    Point, ShapedLine, StatefulInteractiveElement, StrikethroughStyle, Styled, TextRun, TextStyle,
    UTF16Selection, UnderlineStyle, WeakEntity, WhiteSpace, Window, div, fill, point, px, relative,
    size,
};
use itertools::Itertools;
use language::CursorShape;
use settings::Settings;
use std::time::Instant;
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
use ui::utils::ensure_minimum_contrast;
use ui::{ParentElement, Tooltip};
use util::ResultExt;
use workspace::Workspace;

use std::mem;
use std::{fmt::Debug, ops::RangeInclusive, rc::Rc};

use crate::{BlockContext, BlockProperties, ContentMode, TerminalMode, TerminalView};

/// The information generated during layout that is necessary for painting.
pub struct LayoutState {
    hitbox: Hitbox,
    batched_text_runs: Vec<BatchedTextRun>,
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
    content_mode: ContentMode,
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

/// A batched text run that combines multiple adjacent cells with the same style
#[derive(Debug)]
pub struct BatchedTextRun {
    pub start_point: AlacPoint<i32, i32>,
    pub text: String,
    pub cell_count: usize,
    pub style: TextRun,
    pub font_size: AbsoluteLength,
}

impl BatchedTextRun {
    fn new_from_char(
        start_point: AlacPoint<i32, i32>,
        c: char,
        style: TextRun,
        font_size: AbsoluteLength,
    ) -> Self {
        let mut text = String::with_capacity(100); // Pre-allocate for typical line length
        text.push(c);
        BatchedTextRun {
            start_point,
            text,
            cell_count: 1,
            style,
            font_size,
        }
    }

    fn can_append(&self, other_style: &TextRun) -> bool {
        self.style.font == other_style.font
            && self.style.color == other_style.color
            && self.style.background_color == other_style.background_color
            && self.style.underline == other_style.underline
            && self.style.strikethrough == other_style.strikethrough
    }

    fn append_char(&mut self, c: char) {
        self.append_char_internal(c, true);
    }

    fn append_zero_width_chars(&mut self, chars: &[char]) {
        for &c in chars {
            self.append_char_internal(c, false);
        }
    }

    fn append_char_internal(&mut self, c: char, counts_cell: bool) {
        self.text.push(c);
        if counts_cell {
            self.cell_count += 1;
        }
        self.style.len += c.len_utf8();
    }

    pub fn paint(
        &self,
        origin: Point<Pixels>,
        dimensions: &TerminalBounds,
        window: &mut Window,
        cx: &mut App,
    ) {
        let pos = Point::new(
            origin.x + self.start_point.column as f32 * dimensions.cell_width,
            origin.y + self.start_point.line as f32 * dimensions.line_height,
        );

        let _ = window
            .text_system()
            .shape_line(
                self.text.clone().into(),
                self.font_size.to_pixels(window.rem_size()),
                std::slice::from_ref(&self.style),
                Some(dimensions.cell_width),
            )
            .paint(pos, dimensions.line_height, window, cx);
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

/// Represents a rectangular region with a specific background color
#[derive(Debug, Clone)]
struct BackgroundRegion {
    start_line: i32,
    start_col: i32,
    end_line: i32,
    end_col: i32,
    color: Hsla,
}

impl BackgroundRegion {
    fn new(line: i32, col: i32, color: Hsla) -> Self {
        BackgroundRegion {
            start_line: line,
            start_col: col,
            end_line: line,
            end_col: col,
            color,
        }
    }

    /// Check if this region can be merged with another region
    fn can_merge_with(&self, other: &BackgroundRegion) -> bool {
        if self.color != other.color {
            return false;
        }

        // Check if regions are adjacent horizontally
        if self.start_line == other.start_line && self.end_line == other.end_line {
            return self.end_col + 1 == other.start_col || other.end_col + 1 == self.start_col;
        }

        // Check if regions are adjacent vertically with same column span
        if self.start_col == other.start_col && self.end_col == other.end_col {
            return self.end_line + 1 == other.start_line || other.end_line + 1 == self.start_line;
        }

        false
    }

    /// Merge this region with another region
    fn merge_with(&mut self, other: &BackgroundRegion) {
        self.start_line = self.start_line.min(other.start_line);
        self.start_col = self.start_col.min(other.start_col);
        self.end_line = self.end_line.max(other.end_line);
        self.end_col = self.end_col.max(other.end_col);
    }
}

/// Merge background regions to minimize the number of rectangles
fn merge_background_regions(regions: Vec<BackgroundRegion>) -> Vec<BackgroundRegion> {
    if regions.is_empty() {
        return regions;
    }

    let mut merged = regions;
    let mut changed = true;

    // Keep merging until no more merges are possible
    while changed {
        changed = false;
        let mut i = 0;

        while i < merged.len() {
            let mut j = i + 1;
            while j < merged.len() {
                if merged[i].can_merge_with(&merged[j]) {
                    let other = merged.remove(j);
                    merged[i].merge_with(&other);
                    changed = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    merged
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
    mode: TerminalMode,
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
        mode: TerminalMode,
    ) -> TerminalElement {
        TerminalElement {
            terminal,
            terminal_view,
            workspace,
            focused,
            focus: focus.clone(),
            cursor_visible,
            block_below_cursor,
            mode,
            interactivity: Default::default(),
        }
        .track_focus(&focus)
    }

    //Vec<Range<AlacPoint>> -> Clip out the parts of the ranges

    pub fn layout_grid(
        grid: impl Iterator<Item = IndexedCell>,
        start_line_offset: i32,
        text_style: &TextStyle,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
        minimum_contrast: f32,
        cx: &App,
    ) -> (Vec<LayoutRect>, Vec<BatchedTextRun>) {
        let start_time = Instant::now();
        let theme = cx.theme();

        // Pre-allocate with estimated capacity to reduce reallocations
        let estimated_cells = grid.size_hint().0;
        let estimated_runs = estimated_cells / 10; // Estimate ~10 cells per run
        let estimated_regions = estimated_cells / 20; // Estimate ~20 cells per background region

        let mut batched_runs = Vec::with_capacity(estimated_runs);
        let mut cell_count = 0;

        // Collect background regions for efficient merging
        let mut background_regions: Vec<BackgroundRegion> = Vec::with_capacity(estimated_regions);
        let mut current_batch: Option<BatchedTextRun> = None;

        // First pass: collect all cells and their backgrounds
        let linegroups = grid.into_iter().chunk_by(|i| i.point.line);
        for (line_index, (_, line)) in linegroups.into_iter().enumerate() {
            let alac_line = start_line_offset + line_index as i32;

            // Flush any existing batch at line boundaries
            if let Some(batch) = current_batch.take() {
                batched_runs.push(batch);
            }

            let mut previous_cell_had_extras = false;

            for cell in line {
                let mut fg = cell.fg;
                let mut bg = cell.bg;
                if cell.flags.contains(Flags::INVERSE) {
                    mem::swap(&mut fg, &mut bg);
                }

                // Collect background regions (skip default background)
                if !matches!(bg, Named(NamedColor::Background)) {
                    let color = convert_color(&bg, theme);
                    let col = cell.point.column.0 as i32;

                    // Try to extend the last region if it's on the same line with the same color
                    if let Some(last_region) = background_regions.last_mut() {
                        if last_region.color == color
                            && last_region.start_line == alac_line
                            && last_region.end_line == alac_line
                            && last_region.end_col + 1 == col
                        {
                            last_region.end_col = col;
                        } else {
                            background_regions.push(BackgroundRegion::new(alac_line, col, color));
                        }
                    } else {
                        background_regions.push(BackgroundRegion::new(alac_line, col, color));
                    }
                }
                // Skip wide character spacers - they're just placeholders for the second cell of wide characters
                if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                    continue;
                }

                // Skip spaces that follow cells with extras (emoji variation sequences)
                if cell.c == ' ' && previous_cell_had_extras {
                    previous_cell_had_extras = false;
                    continue;
                }
                // Update tracking for next iteration
                previous_cell_had_extras =
                    matches!(cell.zerowidth(), Some(chars) if !chars.is_empty());

                //Layout current cell text
                {
                    if !is_blank(&cell) {
                        cell_count += 1;
                        let cell_style = TerminalElement::cell_style(
                            &cell,
                            fg,
                            bg,
                            theme,
                            text_style,
                            hyperlink,
                            minimum_contrast,
                        );

                        let cell_point = AlacPoint::new(alac_line, cell.point.column.0 as i32);
                        let zero_width_chars = cell.zerowidth();

                        // Try to batch with existing run
                        if let Some(ref mut batch) = current_batch {
                            if batch.can_append(&cell_style)
                                && batch.start_point.line == cell_point.line
                                && batch.start_point.column + batch.cell_count as i32
                                    == cell_point.column
                            {
                                batch.append_char(cell.c);
                                if let Some(chars) = zero_width_chars {
                                    batch.append_zero_width_chars(chars);
                                }
                            } else {
                                // Flush current batch and start new one
                                let old_batch = current_batch.take().unwrap();
                                batched_runs.push(old_batch);
                                let mut new_batch = BatchedTextRun::new_from_char(
                                    cell_point,
                                    cell.c,
                                    cell_style,
                                    text_style.font_size,
                                );
                                if let Some(chars) = zero_width_chars {
                                    new_batch.append_zero_width_chars(chars);
                                }
                                current_batch = Some(new_batch);
                            }
                        } else {
                            // Start new batch
                            let mut new_batch = BatchedTextRun::new_from_char(
                                cell_point,
                                cell.c,
                                cell_style,
                                text_style.font_size,
                            );
                            if let Some(chars) = zero_width_chars {
                                new_batch.append_zero_width_chars(chars);
                            }
                            current_batch = Some(new_batch);
                        }
                    };
                }
            }
        }

        // Flush any remaining batch
        if let Some(batch) = current_batch {
            batched_runs.push(batch);
        }

        // Second pass: merge background regions and convert to layout rects
        let region_count = background_regions.len();
        let merged_regions = merge_background_regions(background_regions);
        let mut rects = Vec::with_capacity(merged_regions.len() * 2); // Estimate 2 rects per merged region

        // Convert merged regions to layout rects
        // Since LayoutRect only supports single-line rectangles, we need to split multi-line regions
        for region in merged_regions {
            for line in region.start_line..=region.end_line {
                rects.push(LayoutRect::new(
                    AlacPoint::new(line, region.start_col),
                    (region.end_col - region.start_col + 1) as usize,
                    region.color,
                ));
            }
        }

        let layout_time = start_time.elapsed();
        log::debug!(
            "Terminal layout_grid: {} cells processed, {} batched runs created, {} rects (from {} merged regions), layout took {:?}",
            cell_count,
            batched_runs.len(),
            rects.len(),
            region_count,
            layout_time
        );

        (rects, batched_runs)
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

    /// Checks if a character is a decorative block/box-like character that should
    /// preserve its exact colors without contrast adjustment.
    ///
    /// This specifically targets characters used as visual connectors, separators,
    /// and borders where color matching with adjacent backgrounds is critical.
    /// Regular icons (git, folders, etc.) are excluded as they need to remain readable.
    ///
    /// Fixes https://github.com/zed-industries/zed/issues/34234
    fn is_decorative_character(ch: char) -> bool {
        matches!(
            ch as u32,
            // Unicode Box Drawing and Block Elements
            0x2500..=0x257F // Box Drawing (└ ┐ ─ │ etc.)
            | 0x2580..=0x259F // Block Elements (▀ ▄ █ ░ ▒ ▓ etc.)
            | 0x25A0..=0x25FF // Geometric Shapes (■ ▶ ● etc. - includes triangular/circular separators)

            // Private Use Area - Powerline separator symbols only
            | 0xE0B0..=0xE0B7 // Powerline separators: triangles (E0B0-E0B3) and half circles (E0B4-E0B7)
            | 0xE0B8..=0xE0BF // Powerline separators: corner triangles
            | 0xE0C0..=0xE0CA // Powerline separators: flames (E0C0-E0C3), pixelated (E0C4-E0C7), and ice (E0C8 & E0CA)
            | 0xE0CC..=0xE0D1 // Powerline separators: honeycombs (E0CC-E0CD) and lego (E0CE-E0D1)
            | 0xE0D2..=0xE0D7 // Powerline separators: trapezoid (E0D2 & E0D4) and inverted triangles (E0D6-E0D7)
        )
    }

    /// Converts the Alacritty cell styles to GPUI text styles and background color.
    fn cell_style(
        indexed: &IndexedCell,
        fg: terminal::alacritty_terminal::vte::ansi::Color,
        bg: terminal::alacritty_terminal::vte::ansi::Color,
        colors: &Theme,
        text_style: &TextStyle,
        hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
        minimum_contrast: f32,
    ) -> TextRun {
        let flags = indexed.cell.flags;
        let mut fg = convert_color(&fg, colors);
        let bg = convert_color(&bg, colors);

        // Only apply contrast adjustment to non-decorative characters
        if !Self::is_decorative_character(indexed.c) {
            fg = ensure_minimum_contrast(fg, bg, minimum_contrast);
        }

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

        if let Some((style, range)) = hyperlink
            && range.contains(&indexed.point)
        {
            if let Some(underline) = style.underline {
                result.underline = Some(underline);
            }

            if let Some(color) = style.color {
                result.color = color;
            }
        }

        result
    }

    fn generic_button_handler<E>(
        connection: Entity<Terminal>,
        focus_handle: FocusHandle,
        steal_focus: bool,
        f: impl Fn(&mut Terminal, &E, &mut Context<Terminal>),
    ) -> impl Fn(&E, &mut Window, &mut App) {
        move |event, window, cx| {
            if steal_focus {
                window.focus(&focus_handle);
            } else if !focus_handle.is_focused(window) {
                return;
            }
            connection.update(cx, |terminal, cx| {
                f(terminal, event, cx);

                cx.notify();
            })
        }
    }

    fn register_mouse_listeners(
        &mut self,
        mode: TermMode,
        hitbox: &Hitbox,
        content_mode: &ContentMode,
        window: &mut Window,
    ) {
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
            let terminal_view = terminal_view;
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
                false,
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
                true,
                move |terminal, e, cx| {
                    terminal.mouse_down(e, cx);
                },
            ),
        );

        if content_mode.is_scrollable() {
            self.interactivity.on_scroll_wheel({
                let terminal_view = self.terminal_view.downgrade();
                move |e, window, cx| {
                    terminal_view
                        .update(cx, |terminal_view, cx| {
                            if matches!(terminal_view.mode, TerminalMode::Standalone)
                                || terminal_view.focus_handle.is_focused(window)
                            {
                                terminal_view.scroll_wheel(e, cx);
                                cx.notify();
                            }
                        })
                        .ok();
                }
            });
        }

        // Mouse mode handlers:
        // All mouse modes need the extra click handlers
        if mode.intersects(TermMode::MOUSE_MODE) {
            self.interactivity.on_mouse_down(
                MouseButton::Right,
                TerminalElement::generic_button_handler(
                    terminal.clone(),
                    focus.clone(),
                    true,
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
                    false,
                    move |terminal, e, cx| {
                        terminal.mouse_up(e, cx);
                    },
                ),
            );
            self.interactivity.on_mouse_up(
                MouseButton::Middle,
                TerminalElement::generic_button_handler(
                    terminal,
                    focus,
                    false,
                    move |terminal, e, cx| {
                        terminal.mouse_up(e, cx);
                    },
                ),
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

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let height: Length = match self.terminal_view.read(cx).content_mode(window, cx) {
            ContentMode::Inline {
                displayed_lines,
                total_lines: _,
            } => {
                let rem_size = window.rem_size();
                let line_height = f32::from(window.text_style().font_size.to_pixels(rem_size))
                    * TerminalSettings::get_global(cx)
                        .line_height
                        .value()
                        .to_pixels(rem_size);
                (displayed_lines * line_height).into()
            }
            ContentMode::Scrollable => {
                if let TerminalMode::Embedded { .. } = &self.mode {
                    let term = self.terminal.read(cx);
                    if !term.scrolled_to_top() && !term.scrolled_to_bottom() && self.focused {
                        self.interactivity.occlude_mouse();
                    }
                }

                relative(1.).into()
            }
        };

        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |mut style, window, cx| {
                style.size.width = relative(1.).into();
                style.size.height = height;

                window.request_layout(style, None, cx)
            },
        );
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let rem_size = self.rem_size(cx);
        self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_, _, hitbox, window, cx| {
                let hitbox = hitbox.unwrap();
                let settings = ThemeSettings::get_global(cx).clone();

                let buffer_font_size = settings.buffer_font_size(cx);

                let terminal_settings = TerminalSettings::get_global(cx);
                let minimum_contrast = terminal_settings.minimum_contrast;

                let font_family = terminal_settings.font_family.as_ref().map_or_else(
                    || settings.buffer_font.family.clone(),
                    |font_family| font_family.0.clone().into(),
                );

                let font_fallbacks = terminal_settings
                    .font_fallbacks
                    .as_ref()
                    .or(settings.buffer_font.fallbacks.as_ref())
                    .cloned();

                let font_features = terminal_settings
                    .font_features
                    .as_ref()
                    .unwrap_or(&FontFeatures::disable_ligatures())
                    .clone();

                let font_weight = terminal_settings.font_weight.unwrap_or_default();

                let line_height = terminal_settings.line_height.value();

                let font_size = match &self.mode {
                    TerminalMode::Embedded { .. } => {
                        window.text_style().font_size.to_pixels(window.rem_size())
                    }
                    TerminalMode::Standalone => terminal_settings
                        .font_size
                        .map_or(buffer_font_size, |size| theme::adjusted_font_size(size, cx)),
                };

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
                let (dimensions, line_height_px) = {
                    let rem_size = window.rem_size();
                    let font_pixels = text_style.font_size.to_pixels(rem_size);
                    // TODO: line_height should be an f32 not an AbsoluteLength.
                    let line_height = f32::from(font_pixels) * line_height.to_pixels(rem_size);
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

                    (
                        TerminalBounds::new(line_height, cell_width, Bounds { origin, size }),
                        line_height,
                    )
                };

                let search_matches = self.terminal.read(cx).matches.clone();

                let background_color = theme.colors().terminal_background;

                let (last_hovered_word, hover_tooltip) =
                    self.terminal.update(cx, |terminal, cx| {
                        terminal.set_size(dimensions);
                        terminal.sync(window, cx);

                        if window.modifiers().secondary()
                            && bounds.contains(&window.mouse_position())
                            && self.terminal_view.read(cx).hover.is_some()
                        {
                            let registered_hover = self.terminal_view.read(cx).hover.as_ref();
                            if terminal.last_content.last_hovered_word.as_ref()
                                == registered_hover.map(|hover| &hover.hovered_word)
                            {
                                (
                                    terminal.last_content.last_hovered_word.clone(),
                                    registered_hover.map(|hover| hover.tooltip.clone()),
                                )
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    });

                let scroll_top = self.terminal_view.read(cx).scroll_top;
                let hyperlink_tooltip = hover_tooltip.map(|hover_tooltip| {
                    let offset = bounds.origin + point(gutter, px(0.)) - point(px(0.), scroll_top);
                    let mut element = div()
                        .size_full()
                        .id("terminal-element")
                        .tooltip(Tooltip::text(hover_tooltip))
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

                let content_mode = self.terminal_view.read(cx).content_mode(window, cx);
                let (rects, batched_text_runs) = match content_mode {
                    ContentMode::Scrollable => {
                        // In scrollable mode, the terminal already provides cells
                        // that are correctly positioned for the current viewport
                        // based on its display_offset. We don't need additional filtering.
                        TerminalElement::layout_grid(
                            cells.iter().cloned(),
                            0,
                            &text_style,
                            last_hovered_word.as_ref().map(|last_hovered_word| {
                                (link_style, &last_hovered_word.word_match)
                            }),
                            minimum_contrast,
                            cx,
                        )
                    }
                    ContentMode::Inline { .. } => {
                        let intersection = window.content_mask().bounds.intersect(&bounds);
                        let start_row = (intersection.top() - bounds.top()) / line_height_px;
                        let end_row = start_row + intersection.size.height / line_height_px;
                        let line_range = (start_row as i32)..=(end_row as i32);

                        TerminalElement::layout_grid(
                            cells
                                .iter()
                                .skip_while(|i| &i.point.line < line_range.start())
                                .take_while(|i| &i.point.line <= line_range.end())
                                .cloned(),
                            *line_range.start(),
                            &text_style,
                            last_hovered_word.as_ref().map(|last_hovered_word| {
                                (link_style, &last_hovered_word.word_match)
                            }),
                            minimum_contrast,
                            cx,
                        )
                    }
                };

                // Layout cursor. Rectangle is used for IME, so we should lay it out even
                // if we don't end up showing it.
                let cursor = if let AlacCursorShape::Hidden = cursor.shape {
                    None
                } else {
                    let cursor_point = DisplayCursor::from(cursor.point, display_offset);
                    let cursor_text = {
                        let str_trxt = cursor_char.to_string();
                        let len = str_trxt.len();
                        window.text_system().shape_line(
                            str_trxt.into(),
                            text_style.font_size.to_pixels(window.rem_size()),
                            &[TextRun {
                                len,
                                font: text_style.font(),
                                color: theme.colors().terminal_ansi_background,
                                ..Default::default()
                            }],
                            None,
                        )
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
                    batched_text_runs,
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
                    content_mode,
                }
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let paint_start = Instant::now();
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let scroll_top = self.terminal_view.read(cx).scroll_top;

            window.paint_quad(fill(bounds, layout.background_color));
            let origin =
                bounds.origin + Point::new(layout.gutter, px(0.)) - Point::new(px(0.), scroll_top);

            let marked_text_cloned: Option<String> = {
                let ime_state = &self.terminal_view.read(cx).ime_state;
                ime_state.as_ref().map(|state| state.marked_text.clone())
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

            self.register_mouse_listeners(
                layout.mode,
                &layout.hitbox,
                &layout.content_mode,
                window,
            );
            if window.modifiers().secondary()
                && bounds.contains(&window.mouse_position())
                && self.terminal_view.read(cx).hover.is_some()
            {
                window.set_cursor_style(gpui::CursorStyle::PointingHand, &layout.hitbox);
            } else {
                window.set_cursor_style(gpui::CursorStyle::IBeam, &layout.hitbox);
            }

            let original_cursor = layout.cursor.take();
            let hyperlink_tooltip = layout.hyperlink_tooltip.take();
            let block_below_cursor_element = layout.block_below_cursor_element.take();
            self.interactivity.paint(
                global_id,
                inspector_id,
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
&                        layout.relative_highlighted_ranges
                    {
                        if let Some((start_y, highlighted_range_lines)) =
                            to_highlighted_range_lines(relative_highlighted_range, layout, origin)
                        {
                            let corner_radius = if EditorSettings::get_global(cx).rounded_selection {
                                0.15 * layout.dimensions.line_height
                            } else {
                                Pixels::ZERO
                            };
                            let hr = HighlightedRange {
                                start_y,
                                line_height: layout.dimensions.line_height,
                                lines: highlighted_range_lines,
                                color: *color,
                                corner_radius: corner_radius,
                            };
                            hr.paint(true, bounds, window);
                        }
                    }

                    // Paint batched text runs instead of individual cells
                    let text_paint_start = Instant::now();
                    for batch in &layout.batched_text_runs {
                        batch.paint(origin, &layout.dimensions, window, cx);
                    }
                    let text_paint_time = text_paint_start.elapsed();

                    if let Some(text_to_mark) = &marked_text_cloned
                        && !text_to_mark.is_empty()
                            && let Some(cursor_layout) = &original_cursor {
                                let ime_position = cursor_layout.bounding_rect(origin).origin;
                                let mut ime_style = layout.base_text_style.clone();
                                ime_style.underline = Some(UnderlineStyle {
                                    color: Some(ime_style.color),
                                    thickness: px(1.0),
                                    wavy: false,
                                });

                                let shaped_line = window.text_system().shape_line(
                                    text_to_mark.clone().into(),
                                    ime_style.font_size.to_pixels(window.rem_size()),
                                    &[TextRun {
                                        len: text_to_mark.len(),
                                        font: ime_style.font(),
                                        color: ime_style.color,
                                        underline: ime_style.underline,
                                        ..Default::default()
                                    }],
                                    None
                                );
                                shaped_line
                                    .paint(ime_position, layout.dimensions.line_height, window, cx)
                                    .log_err();
                            }

                    if self.cursor_visible && marked_text_cloned.is_none()
                        && let Some(mut cursor) = original_cursor {
                            cursor.paint(origin, window, cx);
                        }

                    if let Some(mut element) = block_below_cursor_element {
                        element.paint(window, cx);
                    }

                    if let Some(mut element) = hyperlink_tooltip {
                        element.paint(window, cx);
                    }
                    let total_paint_time = paint_start.elapsed();
                    log::debug!(
                        "Terminal paint: {} text runs, {} rects, text paint took {:?}, total paint took {:?}",
                        layout.batched_text_runs.len(),
                        layout.rects.len(),
                        text_paint_time,
                        total_paint_time
                    );
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
                telemetry.log_edit_event("terminal", project.is_via_remote_server());
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
        self.terminal_view.update(cx, |view, view_cx| {
            view.set_marked_text(new_text.to_string(), new_marked_range, view_cx);
        });
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

    // Convert the start of the range to pixels
    let start_y = origin.y + clamped_start_line as f32 * layout.dimensions.line_height;

    // Step 3. Expand ranges that cross lines into a collection of single-line ranges.
    //  (also convert to pixels)
    let mut highlighted_range_lines = Vec::new();
    for line in clamped_start_line..=clamped_end_line {
        let mut line_start = 0;
        let mut line_end = layout.dimensions.columns();

        if line == clamped_start_line && unclamped_start.line.0 >= 0 {
            line_start = unclamped_start.column.0;
        }
        if line == clamped_end_line && unclamped_end.line.0 <= layout.dimensions.num_lines() as i32
        {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AbsoluteLength, Hsla, font};
    use ui::utils::apca_contrast;

    #[test]
    fn test_is_decorative_character() {
        // Box Drawing characters (U+2500 to U+257F)
        assert!(TerminalElement::is_decorative_character('─')); // U+2500
        assert!(TerminalElement::is_decorative_character('│')); // U+2502
        assert!(TerminalElement::is_decorative_character('┌')); // U+250C
        assert!(TerminalElement::is_decorative_character('┐')); // U+2510
        assert!(TerminalElement::is_decorative_character('└')); // U+2514
        assert!(TerminalElement::is_decorative_character('┘')); // U+2518
        assert!(TerminalElement::is_decorative_character('┼')); // U+253C

        // Block Elements (U+2580 to U+259F)
        assert!(TerminalElement::is_decorative_character('▀')); // U+2580
        assert!(TerminalElement::is_decorative_character('▄')); // U+2584
        assert!(TerminalElement::is_decorative_character('█')); // U+2588
        assert!(TerminalElement::is_decorative_character('░')); // U+2591
        assert!(TerminalElement::is_decorative_character('▒')); // U+2592
        assert!(TerminalElement::is_decorative_character('▓')); // U+2593

        // Geometric Shapes - block/box-like subset (U+25A0 to U+25D7)
        assert!(TerminalElement::is_decorative_character('■')); // U+25A0
        assert!(TerminalElement::is_decorative_character('□')); // U+25A1
        assert!(TerminalElement::is_decorative_character('▲')); // U+25B2
        assert!(TerminalElement::is_decorative_character('▼')); // U+25BC
        assert!(TerminalElement::is_decorative_character('◆')); // U+25C6
        assert!(TerminalElement::is_decorative_character('●')); // U+25CF

        // The specific character from the issue
        assert!(TerminalElement::is_decorative_character('◗')); // U+25D7
        assert!(TerminalElement::is_decorative_character('◘')); // U+25D8 (now included in Geometric Shapes)
        assert!(TerminalElement::is_decorative_character('◙')); // U+25D9 (now included in Geometric Shapes)

        // Powerline symbols (Private Use Area)
        assert!(TerminalElement::is_decorative_character('\u{E0B0}')); // Powerline right triangle
        assert!(TerminalElement::is_decorative_character('\u{E0B2}')); // Powerline left triangle
        assert!(TerminalElement::is_decorative_character('\u{E0B4}')); // Powerline right half circle (the actual issue!)
        assert!(TerminalElement::is_decorative_character('\u{E0B6}')); // Powerline left half circle
        assert!(TerminalElement::is_decorative_character('\u{E0CA}')); // Powerline mirrored ice waveform
        assert!(TerminalElement::is_decorative_character('\u{E0D7}')); // Powerline left triangle inverted

        // Characters that should NOT be considered decorative
        assert!(!TerminalElement::is_decorative_character('A')); // Regular letter
        assert!(!TerminalElement::is_decorative_character('$')); // Symbol
        assert!(!TerminalElement::is_decorative_character(' ')); // Space
        assert!(!TerminalElement::is_decorative_character('←')); // U+2190 (Arrow, not in our ranges)
        assert!(!TerminalElement::is_decorative_character('→')); // U+2192 (Arrow, not in our ranges)
        assert!(!TerminalElement::is_decorative_character('\u{F00C}')); // Font Awesome check (icon, needs contrast)
        assert!(!TerminalElement::is_decorative_character('\u{E711}')); // Devicons (icon, needs contrast)
        assert!(!TerminalElement::is_decorative_character('\u{EA71}')); // Codicons folder (icon, needs contrast)
        assert!(!TerminalElement::is_decorative_character('\u{F401}')); // Octicons (icon, needs contrast)
        assert!(!TerminalElement::is_decorative_character('\u{1F600}')); // Emoji (not in our ranges)
    }

    #[test]
    fn test_decorative_character_boundary_cases() {
        // Test exact boundaries of our ranges
        // Box Drawing range boundaries
        assert!(TerminalElement::is_decorative_character('\u{2500}')); // First char
        assert!(TerminalElement::is_decorative_character('\u{257F}')); // Last char
        assert!(!TerminalElement::is_decorative_character('\u{24FF}')); // Just before

        // Block Elements range boundaries
        assert!(TerminalElement::is_decorative_character('\u{2580}')); // First char
        assert!(TerminalElement::is_decorative_character('\u{259F}')); // Last char

        // Geometric Shapes subset boundaries
        assert!(TerminalElement::is_decorative_character('\u{25A0}')); // First char
        assert!(TerminalElement::is_decorative_character('\u{25FF}')); // Last char
        assert!(!TerminalElement::is_decorative_character('\u{2600}')); // Just after
    }

    #[test]
    fn test_decorative_characters_bypass_contrast_adjustment() {
        // Decorative characters should not be affected by contrast adjustment

        // The specific character from issue #34234
        let problematic_char = '◗'; // U+25D7
        assert!(
            TerminalElement::is_decorative_character(problematic_char),
            "Character ◗ (U+25D7) should be recognized as decorative"
        );

        // Verify some other commonly used decorative characters
        assert!(TerminalElement::is_decorative_character('│')); // Vertical line
        assert!(TerminalElement::is_decorative_character('─')); // Horizontal line
        assert!(TerminalElement::is_decorative_character('█')); // Full block
        assert!(TerminalElement::is_decorative_character('▓')); // Dark shade
        assert!(TerminalElement::is_decorative_character('■')); // Black square
        assert!(TerminalElement::is_decorative_character('●')); // Black circle

        // Verify normal text characters are NOT decorative
        assert!(!TerminalElement::is_decorative_character('A'));
        assert!(!TerminalElement::is_decorative_character('1'));
        assert!(!TerminalElement::is_decorative_character('$'));
        assert!(!TerminalElement::is_decorative_character(' '));
    }

    #[test]
    fn test_contrast_adjustment_logic() {
        // Test the core contrast adjustment logic without needing full app context

        // Test case 1: Light colors (poor contrast)
        let white_fg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 1.0,
        };
        let light_gray_bg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.95,
            a: 1.0,
        };

        // Should have poor contrast
        let actual_contrast = apca_contrast(white_fg, light_gray_bg).abs();
        assert!(
            actual_contrast < 30.0,
            "White on light gray should have poor APCA contrast: {}",
            actual_contrast
        );

        // After adjustment with minimum APCA contrast of 45, should be darker
        let adjusted = ensure_minimum_contrast(white_fg, light_gray_bg, 45.0);
        assert!(
            adjusted.l < white_fg.l,
            "Adjusted color should be darker than original"
        );
        let adjusted_contrast = apca_contrast(adjusted, light_gray_bg).abs();
        assert!(adjusted_contrast >= 45.0, "Should meet minimum contrast");

        // Test case 2: Dark colors (poor contrast)
        let black_fg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        };
        let dark_gray_bg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.05,
            a: 1.0,
        };

        // Should have poor contrast
        let actual_contrast = apca_contrast(black_fg, dark_gray_bg).abs();
        assert!(
            actual_contrast < 30.0,
            "Black on dark gray should have poor APCA contrast: {}",
            actual_contrast
        );

        // After adjustment with minimum APCA contrast of 45, should be lighter
        let adjusted = ensure_minimum_contrast(black_fg, dark_gray_bg, 45.0);
        assert!(
            adjusted.l > black_fg.l,
            "Adjusted color should be lighter than original"
        );
        let adjusted_contrast = apca_contrast(adjusted, dark_gray_bg).abs();
        assert!(adjusted_contrast >= 45.0, "Should meet minimum contrast");

        // Test case 3: Already good contrast
        let good_contrast = ensure_minimum_contrast(black_fg, white_fg, 45.0);
        assert_eq!(
            good_contrast, black_fg,
            "Good contrast should not be adjusted"
        );
    }

    #[test]
    fn test_white_on_white_contrast_issue() {
        // This test reproduces the exact issue from the bug report
        // where white ANSI text on white background should be adjusted

        // Simulate One Light theme colors
        let white_fg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.98, // #fafafaff is approximately 98% lightness
            a: 1.0,
        };
        let white_bg = gpui::Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.98, // Same as foreground - this is the problem!
            a: 1.0,
        };

        // With minimum contrast of 0.0, no adjustment should happen
        let no_adjust = ensure_minimum_contrast(white_fg, white_bg, 0.0);
        assert_eq!(no_adjust, white_fg, "No adjustment with min_contrast 0.0");

        // With minimum APCA contrast of 15, it should adjust to a darker color
        let adjusted = ensure_minimum_contrast(white_fg, white_bg, 15.0);
        assert!(
            adjusted.l < white_fg.l,
            "White on white should become darker, got l={}",
            adjusted.l
        );

        // Verify the contrast is now acceptable
        let new_contrast = apca_contrast(adjusted, white_bg).abs();
        assert!(
            new_contrast >= 15.0,
            "Adjusted APCA contrast {} should be >= 15.0",
            new_contrast
        );
    }

    #[test]
    fn test_batched_text_run_can_append() {
        let style1 = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::red(),
            ..Default::default()
        };

        let style2 = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::red(),
            ..Default::default()
        };

        let style3 = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::blue(), // Different color
            ..Default::default()
        };

        let font_size = AbsoluteLength::Pixels(px(12.0));
        let batch = BatchedTextRun::new_from_char(AlacPoint::new(0, 0), 'a', style1, font_size);

        // Should be able to append same style
        assert!(batch.can_append(&style2));

        // Should not be able to append different style
        assert!(!batch.can_append(&style3));
    }

    #[test]
    fn test_batched_text_run_append() {
        let style = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::red(),
            ..Default::default()
        };

        let font_size = AbsoluteLength::Pixels(px(12.0));
        let mut batch = BatchedTextRun::new_from_char(AlacPoint::new(0, 0), 'a', style, font_size);

        assert_eq!(batch.text, "a");
        assert_eq!(batch.cell_count, 1);
        assert_eq!(batch.style.len, 1);

        batch.append_char('b');

        assert_eq!(batch.text, "ab");
        assert_eq!(batch.cell_count, 2);
        assert_eq!(batch.style.len, 2);

        batch.append_char('c');

        assert_eq!(batch.text, "abc");
        assert_eq!(batch.cell_count, 3);
        assert_eq!(batch.style.len, 3);
    }

    #[test]
    fn test_batched_text_run_append_char() {
        let style = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::red(),
            ..Default::default()
        };

        let font_size = AbsoluteLength::Pixels(px(12.0));
        let mut batch = BatchedTextRun::new_from_char(AlacPoint::new(0, 0), 'x', style, font_size);

        assert_eq!(batch.text, "x");
        assert_eq!(batch.cell_count, 1);
        assert_eq!(batch.style.len, 1);

        batch.append_char('y');

        assert_eq!(batch.text, "xy");
        assert_eq!(batch.cell_count, 2);
        assert_eq!(batch.style.len, 2);

        // Test with multi-byte character
        batch.append_char('😀');

        assert_eq!(batch.text, "xy😀");
        assert_eq!(batch.cell_count, 3);
        assert_eq!(batch.style.len, 6); // 1 + 1 + 4 bytes for emoji
    }

    #[test]
    fn test_batched_text_run_append_zero_width_char() {
        let style = TextRun {
            len: 1,
            font: font("Helvetica"),
            color: Hsla::red(),
            ..Default::default()
        };

        let font_size = AbsoluteLength::Pixels(px(12.0));
        let mut batch = BatchedTextRun::new_from_char(AlacPoint::new(0, 0), 'x', style, font_size);

        let combining = '\u{0301}';
        batch.append_zero_width_chars(&[combining]);

        assert_eq!(batch.text, format!("x{}", combining));
        assert_eq!(batch.cell_count, 1);
        assert_eq!(batch.style.len, 1 + combining.len_utf8());
    }

    #[test]
    fn test_background_region_can_merge() {
        let color1 = Hsla::red();
        let color2 = Hsla::blue();

        // Test horizontal merging
        let mut region1 = BackgroundRegion::new(0, 0, color1);
        region1.end_col = 5;
        let region2 = BackgroundRegion::new(0, 6, color1);
        assert!(region1.can_merge_with(&region2));

        // Test vertical merging with same column span
        let mut region3 = BackgroundRegion::new(0, 0, color1);
        region3.end_col = 5;
        let mut region4 = BackgroundRegion::new(1, 0, color1);
        region4.end_col = 5;
        assert!(region3.can_merge_with(&region4));

        // Test cannot merge different colors
        let region5 = BackgroundRegion::new(0, 0, color1);
        let region6 = BackgroundRegion::new(0, 1, color2);
        assert!(!region5.can_merge_with(&region6));

        // Test cannot merge non-adjacent regions
        let region7 = BackgroundRegion::new(0, 0, color1);
        let region8 = BackgroundRegion::new(0, 2, color1);
        assert!(!region7.can_merge_with(&region8));

        // Test cannot merge vertical regions with different column spans
        let mut region9 = BackgroundRegion::new(0, 0, color1);
        region9.end_col = 5;
        let mut region10 = BackgroundRegion::new(1, 0, color1);
        region10.end_col = 6;
        assert!(!region9.can_merge_with(&region10));
    }

    #[test]
    fn test_background_region_merge() {
        let color = Hsla::red();

        // Test horizontal merge
        let mut region1 = BackgroundRegion::new(0, 0, color);
        region1.end_col = 5;
        let mut region2 = BackgroundRegion::new(0, 6, color);
        region2.end_col = 10;
        region1.merge_with(&region2);
        assert_eq!(region1.start_col, 0);
        assert_eq!(region1.end_col, 10);
        assert_eq!(region1.start_line, 0);
        assert_eq!(region1.end_line, 0);

        // Test vertical merge
        let mut region3 = BackgroundRegion::new(0, 0, color);
        region3.end_col = 5;
        let mut region4 = BackgroundRegion::new(1, 0, color);
        region4.end_col = 5;
        region3.merge_with(&region4);
        assert_eq!(region3.start_col, 0);
        assert_eq!(region3.end_col, 5);
        assert_eq!(region3.start_line, 0);
        assert_eq!(region3.end_line, 1);
    }

    #[test]
    fn test_merge_background_regions() {
        let color = Hsla::red();

        // Test merging multiple adjacent regions
        let regions = vec![
            BackgroundRegion::new(0, 0, color),
            BackgroundRegion::new(0, 1, color),
            BackgroundRegion::new(0, 2, color),
            BackgroundRegion::new(1, 0, color),
            BackgroundRegion::new(1, 1, color),
            BackgroundRegion::new(1, 2, color),
        ];

        let merged = merge_background_regions(regions);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].start_line, 0);
        assert_eq!(merged[0].end_line, 1);
        assert_eq!(merged[0].start_col, 0);
        assert_eq!(merged[0].end_col, 2);

        // Test with non-mergeable regions
        let color2 = Hsla::blue();
        let regions2 = vec![
            BackgroundRegion::new(0, 0, color),
            BackgroundRegion::new(0, 2, color),  // Gap at column 1
            BackgroundRegion::new(1, 0, color2), // Different color
        ];

        let merged2 = merge_background_regions(regions2);
        assert_eq!(merged2.len(), 3);
    }
}
