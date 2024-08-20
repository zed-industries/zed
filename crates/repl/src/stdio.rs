use crate::outputs::ExecutionView;
use alacritty_terminal::{term::Config, vte::ansi::Processor};
use gpui::{canvas, size, AnyElement, FontStyle, TextStyle, WhiteSpace};
use settings::Settings as _;
use std::mem;
use terminal::ZedListener;
use terminal_view::terminal_element::TerminalElement;
use theme::ThemeSettings;
use ui::{prelude::*, IntoElement, ViewContext};

/// Implements the most basic of terminal output for use by Jupyter outputs
/// whether:
///
/// * stdout
/// * stderr
/// * text/plain
/// * traceback from an error output
///
pub struct TerminalOutput {
    parser: Processor,
    handler: alacritty_terminal::Term<ZedListener>,
}

const DEFAULT_NUM_LINES: usize = 32;
const DEFAULT_NUM_COLUMNS: usize = 128;

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
        // todo
        line_height: cx.line_height().into(),
        background_color: Some(theme.colors().terminal_background),
        white_space: WhiteSpace::Normal,
        // These are going to be overridden per-cell
        underline: None,
        strikethrough: None,
        color: theme.colors().terminal_foreground,
    };

    text_style
}

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
        }
    }

    pub fn from(text: &str, cx: &mut WindowContext) -> Self {
        let mut output = Self::new(cx);
        output.append_text(text);
        output
    }

    pub fn append_text(&mut self, text: &str) {
        for byte in text.as_bytes() {
            if *byte == b'\n' {
                // Dirty (?) hack to move the cursor down
                self.parser.advance(&mut self.handler, b'\r');
                self.parser.advance(&mut self.handler, b'\n');
            } else {
                self.parser.advance(&mut self.handler, *byte);
            }

            // self.parser.advance(&mut self.handler, *byte);
        }
    }

    pub fn render(&self, cx: &mut ViewContext<ExecutionView>) -> AnyElement {
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
        .into_any_element()
    }
}
