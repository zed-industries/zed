//! # Plain Text Output
//!
//! This module provides functionality for rendering plain text output in a terminal-like format.
//! It uses the Alacritty terminal emulator backend to process and display text, supporting
//! ANSI escape sequences for formatting, colors, and other terminal features.
//!
//! The main component of this module is the `TerminalOutput` struct, which handles the parsing
//! and rendering of text input, simulating a basic terminal environment within REPL output.
//!
//! This module is used for displaying:
//!
//! - Standard output (stdout)
//! - Standard error (stderr)
//! - Plain text content
//! - Error tracebacks
//!

use alacritty_terminal::{
    event::VoidListener,
    grid::Dimensions as _,
    index::{Column, Line, Point},
    term::Config,
    vte::ansi::Processor,
};
use gpui::{
    Bounds, ClipboardItem, Entity, FocusHandle, FontStyle, Hsla, MouseButton, Pixels, ScrollHandle,
    ScrollWheelEvent, Subscription, TextStyle, WhiteSpace, canvas, linear_color_stop,
    linear_gradient, size,
};
use language::Buffer;
use menu;
use settings::Settings as _;
use terminal::terminal_settings::TerminalSettings;
use terminal_view::terminal_element::TerminalElement;
use theme::{ActiveTheme, ThemeSettings};
use ui::{IntoElement, prelude::*};

use crate::outputs::OutputContent;
use crate::repl_settings::ReplSettings;

/// Tolerance for floating-point scroll position comparisons, accounting for
/// sub-pixel rounding during layout.
const SCROLL_POSITION_TOLERANCE: Pixels = px(1.);

enum GradientEdge {
    Top,
    Bottom,
}

fn gradient_overlay(edge: GradientEdge, bg_color: Hsla, height: Pixels) -> Div {
    let angle = match edge {
        GradientEdge::Top => 0.,
        GradientEdge::Bottom => 180.,
    };

    let base = div()
        .absolute()
        .left_0()
        .w_full()
        .h(height)
        .bg(linear_gradient(
            angle,
            linear_color_stop(bg_color.opacity(0.), 0.),
            linear_color_stop(bg_color, 1.),
        ));

    match edge {
        GradientEdge::Top => base.top_0(),
        GradientEdge::Bottom => base.bottom_0(),
    }
}

/// The `TerminalOutput` struct handles the parsing and rendering of text input,
/// simulating a basic terminal environment within REPL output.
///
/// `TerminalOutput` is designed to handle various types of text-based output, including:
///
/// * stdout (standard output)
/// * stderr (standard error)
/// * text/plain content
/// * error tracebacks
///
/// It uses the Alacritty terminal emulator backend to process and render text,
/// supporting ANSI escape sequences for text formatting and colors.
///
pub struct TerminalOutput {
    full_buffer: Option<Entity<Buffer>>,
    /// ANSI escape sequence processor for parsing input text.
    parser: Processor,
    /// Alacritty terminal instance that manages the terminal state and content.
    handler: alacritty_terminal::Term<VoidListener>,
    scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    scroll_active: bool,
    _focus_subscription: Subscription,
}

/// Returns the default text style for the terminal output.
pub fn text_style(window: &mut Window, cx: &App) -> TextStyle {
    let settings = ThemeSettings::get_global(cx).clone();

    let font_size = settings.buffer_font_size(cx).into();
    let font_family = settings.buffer_font.family;
    let font_features = settings.buffer_font.features;
    let font_weight = settings.buffer_font.weight;
    let font_fallbacks = settings.buffer_font.fallbacks;

    let theme = cx.theme();

    TextStyle {
        font_family,
        font_features,
        font_weight,
        font_fallbacks,
        font_size,
        font_style: FontStyle::Normal,
        line_height: window.line_height().into(),
        background_color: Some(theme.colors().terminal_ansi_background),
        white_space: WhiteSpace::Normal,
        // These are going to be overridden per-cell
        color: theme.colors().terminal_foreground,
        ..Default::default()
    }
}

fn cell_width(window: &mut Window, cx: &App) -> Pixels {
    let text_style = text_style(window, cx);
    let text_system = window.text_system();
    let font_pixels = text_style.font_size.to_pixels(window.rem_size());
    let font_id = text_system.resolve_font(&text_style.font());
    text_system
        .advance(font_id, font_pixels, 'w')
        .map(|advance| advance.width)
        .unwrap_or(Pixels::ZERO)
}

/// Computes the number of terminal columns that fit in the available viewport width,
/// accounting for the editor gutter and margins. If `max_columns` is set to a nonzero
/// value in settings, that value is used instead.
fn columns_for_viewport(window: &mut Window, cx: &App) -> usize {
    let max_columns = ReplSettings::get_global(cx).max_columns;
    if max_columns > 0 {
        return max_columns;
    }

    let cell_width = cell_width(window, cx);
    if cell_width == Pixels::ZERO {
        return 80;
    }

    let viewport_width = window.viewport_size().width;
    let gutter_estimate = cell_width * 8.0;
    let available_width = (viewport_width - gutter_estimate).max(cell_width * 20.0);
    (available_width / cell_width).floor() as usize
}

/// Returns the default terminal size for the terminal output.
pub fn terminal_size(window: &mut Window, cx: &mut App) -> terminal::TerminalBounds {
    let line_height = window.line_height();
    let cell_width = cell_width(window, cx);

    let num_lines = ReplSettings::get_global(cx).max_lines;
    let columns = columns_for_viewport(window, cx);

    let width = columns as f32 * cell_width;
    let height = num_lines as f32 * line_height;

    terminal::TerminalBounds {
        cell_width,
        line_height,
        bounds: Bounds {
            origin: gpui::Point::default(),
            size: size(width, height),
        },
    }
}

pub fn max_width_for_columns(
    columns: usize,
    window: &mut Window,
    cx: &App,
) -> Option<gpui::Pixels> {
    if columns == 0 {
        return None;
    }

    let text_style = text_style(window, cx);
    let text_system = window.text_system();
    let font_pixels = text_style.font_size.to_pixels(window.rem_size());
    let font_id = text_system.resolve_font(&text_style.font());
    let cell_width = text_system
        .advance(font_id, font_pixels, 'w')
        .map(|advance| advance.width)
        .unwrap_or(Pixels::ZERO);

    Some(cell_width * columns as f32)
}

impl TerminalOutput {
    /// Creates a new `TerminalOutput` instance.
    ///
    /// This method initializes a new terminal emulator with default configuration
    /// and sets up the necessary components for handling terminal events and rendering.
    ///
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let term = alacritty_terminal::Term::new(
            Config::default(),
            &terminal_size(window, cx),
            VoidListener,
        );

        let focus_handle = cx.focus_handle();
        let focus_subscription = cx.on_focus_out(&focus_handle, window, |this, _, _, cx| {
            this.scroll_active = false;
            cx.notify();
        });

        Self {
            parser: Processor::new(),
            handler: term,
            full_buffer: None,
            scroll_handle: ScrollHandle::new(),
            focus_handle,
            scroll_active: false,
            _focus_subscription: focus_subscription,
        }
    }

    /// Creates a new `TerminalOutput` instance with initial content.
    ///
    /// Initializes a new terminal output and populates it with the provided text.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice containing the initial text for the terminal output.
    /// * `cx` - A mutable reference to the `WindowContext` for initialization.
    ///
    /// # Returns
    ///
    /// A new instance of `TerminalOutput` containing the provided text.
    pub fn from(text: &str, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut output = Self::new(window, cx);
        output.append_text(text, cx);
        output
    }

    /// Appends text to the terminal output.
    ///
    /// Processes each byte of the input text, handling newline characters specially
    /// to ensure proper cursor movement. Uses the ANSI parser to process the input
    /// and update the terminal state.
    ///
    /// As an example, if the user runs the following Python code in this REPL:
    ///
    /// ```python
    /// import time
    /// print("Hello,", end="")
    /// time.sleep(1)
    /// print(" world!")
    /// ```
    ///
    /// Then append_text will be called twice, with the following arguments:
    ///
    /// ```ignore
    /// terminal_output.append_text("Hello,");
    /// terminal_output.append_text(" world!");
    /// ```
    /// Resulting in a single output of "Hello, world!".
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice containing the text to be appended.
    pub fn append_text(&mut self, text: &str, cx: &mut App) {
        let max_offset = self.scroll_handle.max_offset();
        let offset = self.scroll_handle.offset();
        let at_bottom =
            max_offset.y <= Pixels::ZERO || offset.y <= -max_offset.y + SCROLL_POSITION_TOLERANCE;

        for byte in text.as_bytes() {
            if *byte == b'\n' {
                // Dirty (?) hack to move the cursor down
                self.parser.advance(&mut self.handler, &[b'\r']);
                self.parser.advance(&mut self.handler, &[b'\n']);
            } else {
                self.parser.advance(&mut self.handler, &[*byte]);
            }
        }

        // This will keep the buffer up to date, though with some terminal codes it won't be perfect
        if let Some(buffer) = self.full_buffer.as_ref() {
            buffer.update(cx, |buffer, cx| {
                buffer.edit([(buffer.len()..buffer.len(), text)], None, cx);
            });
        }

        if at_bottom {
            self.scroll_handle.scroll_to_bottom();
        }
    }

    pub fn full_text(&self) -> String {
        fn sanitize(mut line: String) -> Option<String> {
            line.retain(|ch| ch != '\u{0}' && ch != '\r');
            if line.trim().is_empty() {
                return None;
            }
            let trimmed = line.trim_end_matches([' ', '\t']);
            Some(trimmed.to_owned())
        }

        let mut lines = Vec::new();

        // Get the total number of lines, including history
        let total_lines = self.handler.grid().total_lines();
        let visible_lines = self.handler.screen_lines();
        let history_lines = total_lines - visible_lines;

        // Capture history lines in correct order (oldest to newest)
        for line in (0..history_lines).rev() {
            let line_index = Line(-(line as i32) - 1);
            let start = Point::new(line_index, Column(0));
            let end = Point::new(line_index, Column(self.handler.columns() - 1));
            if let Some(cleaned) = sanitize(self.handler.bounds_to_string(start, end)) {
                lines.push(cleaned);
            }
        }

        // Capture visible lines
        for line in 0..visible_lines {
            let line_index = Line(line as i32);
            let start = Point::new(line_index, Column(0));
            let end = Point::new(line_index, Column(self.handler.columns() - 1));
            if let Some(cleaned) = sanitize(self.handler.bounds_to_string(start, end)) {
                lines.push(cleaned);
            }
        }

        if lines.is_empty() {
            String::new()
        } else {
            let mut full_text = lines.join("\n");
            full_text.push('\n');
            full_text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) -> &mut VisualTestContext {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
        cx.add_empty_window()
    }

    #[gpui::test]
    fn test_max_width_for_columns_zero(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        let result = cx.update(|window, cx| max_width_for_columns(0, window, cx));
        assert!(result.is_none());
    }

    #[gpui::test]
    fn test_max_width_for_columns_matches_cell_width(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        let columns = 5;
        let (result, expected) = cx.update(|window, cx| {
            let text_style = text_style(window, cx);
            let text_system = window.text_system();
            let font_pixels = text_style.font_size.to_pixels(window.rem_size());
            let font_id = text_system.resolve_font(&text_style.font());
            let cell_width = text_system
                .advance(font_id, font_pixels, 'w')
                .map(|advance| advance.width)
                .unwrap_or(gpui::Pixels::ZERO);
            let result = max_width_for_columns(columns, window, cx);
            (result, cell_width * columns as f32)
        });

        let Some(result) = result else {
            panic!("expected max width for columns {columns}");
        };
        let result_f32: f32 = result.into();
        let expected_f32: f32 = expected.into();
        assert!((result_f32 - expected_f32).abs() < 0.01);
    }

    #[gpui::test]
    fn test_full_text_preserves_all_lines_beyond_max_lines(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        cx.update(|window, cx| {
            let output = cx.new(|cx| TerminalOutput::new(window, cx));
            output.update(cx, |output, cx| {
                // Default max_lines is 32; generate 50 lines to exceed it.
                let lines: Vec<String> = (0..50).map(|i| format!("Line {i}")).collect();
                output.append_text(&lines.join("\n"), cx);

                let text = output.full_text();
                assert!(
                    text.contains("Line 0"),
                    "first line should be preserved in scrollback"
                );
                assert!(text.contains("Line 49"), "last line should be preserved");
            });
        });
    }

    #[gpui::test]
    fn test_columns_for_viewport_returns_reasonable_value(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        let columns = cx.update(|window, cx| columns_for_viewport(window, cx));
        assert!(
            columns >= 20,
            "viewport columns should be at least 20, got {columns}"
        );
    }

    #[gpui::test]
    fn test_streaming_append_accumulates_content(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        cx.update(|window, cx| {
            let output = cx.new(|cx| TerminalOutput::new(window, cx));
            output.update(cx, |output, cx| {
                output.append_text("Hello,", cx);
                output.append_text(" world!", cx);

                let text = output.full_text();
                assert!(
                    text.contains("Hello, world!"),
                    "streaming appends should concatenate: got {text:?}"
                );
            });
        });
    }

    #[gpui::test]
    fn test_append_empty_string_is_noop(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        cx.update(|window, cx| {
            let output = cx.new(|cx| TerminalOutput::new(window, cx));
            output.update(cx, |output, cx| {
                output.append_text("first line", cx);
                let before = output.full_text();
                output.append_text("", cx);
                let after = output.full_text();
                assert_eq!(
                    before, after,
                    "appending empty string should not change output"
                );
            });
        });
    }

    #[gpui::test]
    fn test_empty_output_produces_empty_text(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        cx.update(|window, cx| {
            let output = cx.new(|cx| TerminalOutput::new(window, cx));
            assert!(
                output.read(cx).full_text().is_empty(),
                "new output should have empty text"
            );
        });
    }

    #[gpui::test]
    fn test_multiline_append_preserves_line_order(cx: &mut TestAppContext) {
        let cx = init_test(cx);
        cx.update(|window, cx| {
            let output = cx.new(|cx| TerminalOutput::new(window, cx));
            output.update(cx, |output, cx| {
                output.append_text("alpha\nbeta\ngamma", cx);

                let text = output.full_text();
                let alpha_pos = text.find("alpha").expect("should contain alpha");
                let beta_pos = text.find("beta").expect("should contain beta");
                let gamma_pos = text.find("gamma").expect("should contain gamma");
                assert!(
                    alpha_pos < beta_pos && beta_pos < gamma_pos,
                    "lines should appear in order: {text:?}"
                );
            });
        });
    }
}

impl Render for TerminalOutput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = text_style(window, cx);
        let cell_width = cell_width(window, cx);

        // Resize terminal to match current viewport width so text reflows appropriately.
        let target_columns = columns_for_viewport(window, cx);
        if target_columns != self.handler.columns() {
            let num_lines = ReplSettings::get_global(cx).max_lines;
            let width = target_columns as f32 * cell_width;
            let height = num_lines as f32 * window.line_height();
            let bounds = terminal::TerminalBounds {
                cell_width,
                line_height: window.line_height(),
                bounds: Bounds {
                    origin: gpui::Point::default(),
                    size: size(width, height),
                },
            };
            self.handler.resize(bounds);
        }

        // Iterate ALL grid lines (history + visible) so output is never truncated.
        let grid = self.handler.grid();
        let start = Point::new(Line(grid.topmost_line().0 - 1), grid.last_column());
        let grid = grid.iter_from(start).map(|ic| terminal::IndexedCell {
            point: ic.point,
            cell: ic.cell.clone(),
        });

        let minimum_contrast = TerminalSettings::get_global(cx).minimum_contrast;
        let (rects, batched_text_runs) =
            TerminalElement::layout_grid(grid, 0, &text_style, None, minimum_contrast, cx);

        let text_line_height = text_style.line_height_in_pixels(window.rem_size());
        // lines are 0-indexed, so we must add 1 to get the number of lines
        let num_lines = batched_text_runs
            .iter()
            .map(|b| b.start_point.line)
            .max()
            .unwrap_or(0)
            + 1;
        let content_height = num_lines as f32 * text_line_height;

        let canvas_element = canvas(
            move |_bounds, _, _| {},
            move |bounds, _, window, cx| {
                let terminal_bounds = terminal::TerminalBounds {
                    cell_width,
                    line_height: text_line_height,
                    bounds,
                };
                for rect in rects {
                    rect.paint(bounds.origin, &terminal_bounds, window);
                }
                for batch in batched_text_runs {
                    batch.paint(bounds.origin, &terminal_bounds, window, cx);
                }
            },
        )
        .h(content_height);

        let output_settings = ReplSettings::get_global(cx);
        let output_max_height = if output_settings.output_max_height_lines > 0 {
            Some(text_line_height * output_settings.output_max_height_lines as f32)
        } else {
            None
        };

        // If no max height is configured or the content fits, render the canvas directly.
        let Some(max_height) = output_max_height.filter(|&max_h| content_height > max_h) else {
            return canvas_element.into_any_element();
        };
        let focus_handle = self.focus_handle.clone();

        let max_offset = self.scroll_handle.max_offset();
        let offset = self.scroll_handle.offset();
        let has_overflow = max_offset.y > text_line_height;
        let not_at_top = has_overflow && offset.y < -SCROLL_POSITION_TOLERANCE;
        let not_at_bottom = has_overflow && offset.y > -max_offset.y + SCROLL_POSITION_TOLERANCE;

        let bg_color = cx.theme().colors().background;
        let fade_height = text_line_height * 1.5;
        let entity_id = cx.entity_id();
        let scroll_active = self.scroll_active;

        div()
            .relative()
            .child(
                div()
                    .id(("terminal-scroll", entity_id))
                    .key_context("TerminalOutput")
                    .track_focus(&focus_handle)
                    .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                        if this.scroll_active {
                            this.scroll_active = false;
                            cx.notify();
                        } else {
                            cx.propagate();
                        }
                    }))
                    .max_h(max_height)
                    .track_scroll(&self.scroll_handle)
                    .when_else(
                        scroll_active,
                        |this| this.overflow_y_scroll(),
                        |this| this.overflow_y_hidden(),
                    )
                    .on_scroll_wheel(cx.listener(|this, _: &ScrollWheelEvent, _, cx| {
                        if this.scroll_active {
                            cx.stop_propagation();
                        }
                    }))
                    .child(canvas_element),
            )
            .when(!scroll_active && has_overflow, |this| {
                this.child(
                    div()
                        .id(("terminal-scroll-overlay", entity_id))
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .cursor_pointer()
                        .bg(bg_color.opacity(0.6))
                        .hover(|style| style.bg(bg_color.opacity(0.4)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| {
                                this.scroll_active = true;
                                this.focus_handle.focus(window, cx);
                                cx.notify();
                            }),
                        )
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            Label::new("Click to scroll")
                                .size(LabelSize::Large)
                                .color(Color::Default)
                                .weight(gpui::FontWeight::BOLD),
                        ),
                )
            })
            .when(not_at_top, |this| {
                this.child(gradient_overlay(GradientEdge::Top, bg_color, fade_height))
            })
            .when(not_at_bottom, |this| {
                this.child(gradient_overlay(
                    GradientEdge::Bottom,
                    bg_color,
                    fade_height,
                ))
            })
            .into_any_element()
    }
}

impl OutputContent for TerminalOutput {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.full_text()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        if self.full_buffer.as_ref().is_some() {
            return self.full_buffer.clone();
        }

        let buffer = cx.new(|cx| {
            let mut buffer =
                Buffer::local(self.full_text(), cx).with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });

        self.full_buffer = Some(buffer.clone());
        Some(buffer)
    }
}
