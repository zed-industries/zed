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
use gpui::{Bounds, ClipboardItem, Entity, FontStyle, TextStyle, WhiteSpace, canvas, size};
use language::Buffer;
use settings::Settings as _;
use terminal::terminal_settings::TerminalSettings;
use terminal_view::terminal_element::TerminalElement;
use theme::ThemeSettings;
use ui::{IntoElement, prelude::*};

use crate::outputs::OutputContent;
use crate::repl_settings::ReplSettings;

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
}

/// Returns the default text style for the terminal output.
pub fn text_style(window: &mut Window, cx: &mut App) -> TextStyle {
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

/// Returns the default terminal size for the terminal output.
pub fn terminal_size(window: &mut Window, cx: &mut App) -> terminal::TerminalBounds {
    let text_style = text_style(window, cx);
    let text_system = window.text_system();

    let line_height = window.line_height();

    let font_pixels = text_style.font_size.to_pixels(window.rem_size());
    let font_id = text_system.resolve_font(&text_style.font());

    let cell_width = text_system
        .advance(font_id, font_pixels, 'w')
        .unwrap()
        .width;

    let num_lines = ReplSettings::get_global(cx).max_lines;
    let columns = ReplSettings::get_global(cx).max_columns;

    // Reversed math from terminal::TerminalSize to get pixel width according to terminal width
    let width = columns as f32 * cell_width;
    let height = num_lines as f32 * window.line_height();

    terminal::TerminalBounds {
        cell_width,
        line_height,
        bounds: Bounds {
            origin: gpui::Point::default(),
            size: size(width, height),
        },
    }
}

impl TerminalOutput {
    /// Creates a new `TerminalOutput` instance.
    ///
    /// This method initializes a new terminal emulator with default configuration
    /// and sets up the necessary components for handling terminal events and rendering.
    ///
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let term = alacritty_terminal::Term::new(
            Config::default(),
            &terminal_size(window, cx),
            VoidListener,
        );

        Self {
            parser: Processor::new(),
            handler: term,
            full_buffer: None,
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
    pub fn from(text: &str, window: &mut Window, cx: &mut App) -> Self {
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

impl Render for TerminalOutput {
    /// Renders the terminal output as a GPUI element.
    ///
    /// Converts the current terminal state into a renderable GPUI element. It handles
    /// the layout of the terminal grid, calculates the dimensions of the output, and
    /// creates a canvas element that paints the terminal cells and background rectangles.
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = text_style(window, cx);
        let text_system = window.text_system();

        let grid = self
            .handler
            .renderable_content()
            .display_iter
            .map(|ic| terminal::IndexedCell {
                point: ic.point,
                cell: ic.cell.clone(),
            });
        let minimum_contrast = TerminalSettings::get_global(cx).minimum_contrast;
        let (rects, batched_text_runs) =
            TerminalElement::layout_grid(grid, 0, &text_style, None, minimum_contrast, cx);

        // lines are 0-indexed, so we must add 1 to get the number of lines
        let text_line_height = text_style.line_height_in_pixels(window.rem_size());
        let num_lines = batched_text_runs
            .iter()
            .map(|b| b.start_point.line)
            .max()
            .unwrap_or(0)
            + 1;
        let height = num_lines as f32 * text_line_height;

        let font_pixels = text_style.font_size.to_pixels(window.rem_size());
        let font_id = text_system.resolve_font(&text_style.font());

        let cell_width = text_system
            .advance(font_id, font_pixels, 'w')
            .map(|advance| advance.width)
            .unwrap_or(Pixels::ZERO);

        canvas(
            // prepaint
            move |_bounds, _, _| {},
            // paint
            move |bounds, _, window, cx| {
                for rect in rects {
                    rect.paint(
                        bounds.origin,
                        &terminal::TerminalBounds {
                            cell_width,
                            line_height: text_line_height,
                            bounds,
                        },
                        window,
                    );
                }

                for batch in batched_text_runs {
                    batch.paint(
                        bounds.origin,
                        &terminal::TerminalBounds {
                            cell_width,
                            line_height: text_line_height,
                            bounds,
                        },
                        window,
                        cx,
                    );
                }
            },
        )
        // We must set the height explicitly for the editor block to size itself correctly
        .h(height)
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
