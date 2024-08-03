use crate::outputs::ExecutionView;
use alacritty_terminal::{term::Config, vte::ansi::Processor};
use gpui::{canvas, AnyElement};
use std::mem;
use terminal::ZedListener;
use terminal_view::terminal_element::TerminalElement;
use ui::{div, prelude::*, IntoElement, ViewContext};

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

impl TerminalOutput {
    pub fn new() -> Self {
        let (events_tx, events_rx) = futures::channel::mpsc::unbounded();
        let term = alacritty_terminal::Term::new(
            Config::default(),
            &terminal::TerminalSize::default(),
            terminal::ZedListener(events_tx.clone()),
        );

        mem::forget(events_rx);
        Self {
            parser: Processor::new(),
            handler: term,
        }
    }

    pub fn from(text: &str) -> Self {
        let mut output = Self::new();
        output.append_text(text);
        output
    }

    pub fn append_text(&mut self, text: &str) {
        for byte in text.as_bytes() {
            self.parser.advance(&mut self.handler, *byte);
        }
    }

    // Fixed width because it's output as an editor block
    // Max height is u8 lines

    pub fn render(&self, cx: &ViewContext<ExecutionView>) -> AnyElement {
        let text_style = cx.text_style();
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

        let line_height = cx.line_height();

        let font_pixels = text_style.font_size.to_pixels(cx.rem_size());
        let font_id = text_system.resolve_font(&text_style.font());

        let cell_width = text_system
            .advance(font_id, font_pixels, 'w')
            .unwrap()
            .width;

        let el = canvas(
            // prepaint
            |_, _| {},
            // paint
            move |bounds, _, cx| {
                for rect in rects {
                    rect.paint(
                        bounds.origin,
                        &terminal::TerminalSize {
                            cell_width,
                            line_height,
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
                            line_height,
                            size: bounds.size,
                        },
                        bounds,
                        cx,
                    );
                }
            },
        );

        div().child(el).into_any_element()
    }
}
