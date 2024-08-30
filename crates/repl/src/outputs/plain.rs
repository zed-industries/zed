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
    grid::Dimensions as _,
    index::{Column, Line, Point},
    term::Config,
    vte::ansi::Processor,
};
use gpui::{canvas, size, ClipboardItem, FontStyle, Model, TextStyle, WhiteSpace};
use language::Buffer;
use settings::Settings as _;
use std::mem;
use terminal::ZedListener;
use terminal_view::terminal_element::TerminalElement;
use theme::ThemeSettings;
use ui::{prelude::*, IntoElement};

use crate::outputs::OutputContent;

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
    full_buffer: Option<Model<Buffer>>,
    /// ANSI escape sequence processor for parsing input text.
    parser: Processor,
    /// Alacritty terminal instance that manages the terminal state and content.
    handler: alacritty_terminal::Term<ZedListener>,
}

const DEFAULT_NUM_LINES: usize = 32;
const DEFAULT_NUM_COLUMNS: usize = 128;

/// Returns the default text style for the terminal output.
pub fn text_style(cx: &mut WindowContext) -> TextStyle {
    let settings = ThemeSettings::get_global(cx).clone();

    let font_family = settings.buffer_font.family;
    let font_features = settings.buffer_font.features;
    let font_weight = settings.buffer_font.weight;
    let font_fallbacks = settings.buffer_font.fallbacks;

    let theme = cx.theme();

    let text_style = TextStyle {
        font_family,
        font_features,
        font_weight,
        font_fallbacks,
        font_size: theme::get_buffer_font_size(cx).into(),
        font_style: FontStyle::Normal,
        line_height: cx.line_height().into(),
        background_color: Some(theme.colors().terminal_background),
        white_space: WhiteSpace::Normal,
        truncate: None,
        // These are going to be overridden per-cell
        underline: None,
        strikethrough: None,
        color: theme.colors().terminal_foreground,
    };

    text_style
}

/// Returns the default terminal size for the terminal output.
pub fn terminal_size(cx: &mut WindowContext) -> terminal::TerminalSize {
    let text_style = text_style(cx);
    let text_system = cx.text_system();

    let line_height = cx.line_height();

    let font_pixels = text_style.font_size.to_pixels(cx.rem_size());
    let font_id = text_system.resolve_font(&text_style.font());

    let cell_width = text_system
        .advance(font_id, font_pixels, 'w')
        .unwrap()
        .width;

    let num_lines = DEFAULT_NUM_LINES;
    let columns = DEFAULT_NUM_COLUMNS;

    // Reversed math from terminal::TerminalSize to get pixel width according to terminal width
    let width = columns as f32 * cell_width;
    let height = num_lines as f32 * cx.line_height();

    terminal::TerminalSize {
        cell_width,
        line_height,
        size: size(width, height),
    }
}

impl TerminalOutput {
    /// Creates a new `TerminalOutput` instance.
    ///
    /// This method initializes a new terminal emulator with default configuration
    /// and sets up the necessary components for handling terminal events and rendering.
    ///
    pub fn new(cx: &mut WindowContext) -> Self {
        let (events_tx, events_rx) = futures::channel::mpsc::unbounded();
        let term = alacritty_terminal::Term::new(
            Config::default(),
            &terminal_size(cx),
            terminal::ZedListener(events_tx.clone()),
        );

        mem::forget(events_rx);
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
    pub fn from(text: &str, cx: &mut WindowContext) -> Self {
        let mut output = Self::new(cx);
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
    /// ```rust
    /// terminal_output.append_text("Hello,")
    /// terminal_output.append_text(" world!")
    /// ```
    /// Resulting in a single output of "Hello, world!".
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice containing the text to be appended.
    pub fn append_text(&mut self, text: &str, cx: &mut WindowContext) {
        for byte in text.as_bytes() {
            if *byte == b'\n' {
                // Dirty (?) hack to move the cursor down
                self.parser.advance(&mut self.handler, b'\r');
                self.parser.advance(&mut self.handler, b'\n');
            } else {
                self.parser.advance(&mut self.handler, *byte);
            }
        }

        // This will keep the buffer up to date, though with some terminal codes it won't be perfect
        if let Some(buffer) = self.full_buffer.as_ref() {
            buffer.update(cx, |buffer, cx| {
                buffer.edit([(buffer.len()..buffer.len(), text)], None, cx);
            });
        }
    }

    fn full_text(&self) -> String {
        let mut full_text = String::new();

        // Get the total number of lines, including history
        let total_lines = self.handler.grid().total_lines();
        let visible_lines = self.handler.screen_lines();
        let history_lines = total_lines - visible_lines;

        // Capture history lines in correct order (oldest to newest)
        for line in (0..history_lines).rev() {
            let line_index = Line(-(line as i32) - 1);
            let start = Point::new(line_index, Column(0));
            let end = Point::new(line_index, Column(self.handler.columns() - 1));
            let line_content = self.handler.bounds_to_string(start, end);

            if !line_content.trim().is_empty() {
                full_text.push_str(&line_content);
                full_text.push('\n');
            }
        }

        // Capture visible lines
        for line in 0..visible_lines {
            let line_index = Line(line as i32);
            let start = Point::new(line_index, Column(0));
            let end = Point::new(line_index, Column(self.handler.columns() - 1));
            let line_content = self.handler.bounds_to_string(start, end);

            if !line_content.trim().is_empty() {
                full_text.push_str(&line_content);
                full_text.push('\n');
            }
        }

        // Trim any trailing newlines
        full_text.trim_end().to_string()
    }
}

impl Render for TerminalOutput {
    /// Renders the terminal output as a GPUI element.
    ///
    /// Converts the current terminal state into a renderable GPUI element. It handles
    /// the layout of the terminal grid, calculates the dimensions of the output, and
    /// creates a canvas element that paints the terminal cells and background rectangles.
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text_style = text_style(cx);
        let text_system = cx.text_system();

        let grid = self
            .handler
            .renderable_content()
            .display_iter
            .map(|ic| terminal::IndexedCell {
                point: ic.point,
                cell: ic.cell.clone(),
            });
        let (cells, rects) = TerminalElement::layout_grid(grid, &text_style, text_system, None, cx);

        // lines are 0-indexed, so we must add 1 to get the number of lines
        let text_line_height = text_style.line_height_in_pixels(cx.rem_size());
        let num_lines = cells.iter().map(|c| c.point.line).max().unwrap_or(0) + 1;
        let height = num_lines as f32 * text_line_height;

        let font_pixels = text_style.font_size.to_pixels(cx.rem_size());
        let font_id = text_system.resolve_font(&text_style.font());

        let cell_width = text_system
            .advance(font_id, font_pixels, 'w')
            .map(|advance| advance.width)
            .unwrap_or(Pixels(0.0));

        canvas(
            // prepaint
            move |_bounds, _| {},
            // paint
            move |bounds, _, cx| {
                for rect in rects {
                    rect.paint(
                        bounds.origin,
                        &terminal::TerminalSize {
                            cell_width,
                            line_height: text_line_height,
                            size: bounds.size,
                        },
                        cx,
                    );
                }

                for cell in cells {
                    cell.paint(
                        bounds.origin,
                        &terminal::TerminalSize {
                            cell_width,
                            line_height: text_line_height,
                            size: bounds.size,
                        },
                        bounds,
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
    fn clipboard_content(&self, _cx: &WindowContext) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.full_text()))
    }

    fn has_clipboard_content(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn has_buffer_content(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn buffer_content(&mut self, cx: &mut WindowContext) -> Option<Model<Buffer>> {
        if let Some(_) = self.full_buffer.as_ref() {
            return self.full_buffer.clone();
        }

        let buffer = cx.new_model(|cx| {
            let mut buffer =
                Buffer::local(self.full_text(), cx).with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });

        self.full_buffer = Some(buffer.clone());
        Some(buffer)
    }
}
