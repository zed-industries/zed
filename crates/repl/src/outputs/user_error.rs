use gpui::{AnyElement, App, AppContext as _, Entity, FontWeight, Pixels, Window, px};
use settings::Settings as _;
use ui::{Label, h_flex, prelude::*, v_flex};

use crate::outputs::plain::{self, TerminalOutput};
use crate::repl_settings::ReplSettings;

const TRACEBACK_BORDER_WIDTH: Pixels = px(1.);

/// Userspace error from the kernel
#[derive(Clone)]
pub struct ErrorView {
    pub ename: String,
    pub evalue: String,
    pub traceback: Entity<TerminalOutput>,
}

impl ErrorView {
    pub fn new(
        ename: String,
        evalue: String,
        traceback: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let columns = Self::traceback_columns(window, cx);
        Self {
            ename,
            evalue,
            traceback: cx
                .new(|cx| TerminalOutput::from_with_columns(traceback, columns, window, cx)),
        }
    }

    fn traceback_padding(window: &Window) -> Pixels {
        window.line_height() / 2.
    }

    /// The output container is capped at `max_columns` cells wide, but the traceback is
    /// inset by padding and a border. Wrapping at `max_columns` would let the end of long
    /// lines get clipped, so wrap early enough for every column to stay visible.
    fn traceback_columns(window: &mut Window, cx: &App) -> usize {
        let max_columns = ReplSettings::get_global(cx).max_columns;
        let Some(cell_width) = plain::max_width_for_columns(1, window, cx) else {
            return max_columns;
        };
        if cell_width <= Pixels::ZERO {
            return max_columns;
        }
        let inset = Self::traceback_padding(window) * 2. + TRACEBACK_BORDER_WIDTH;
        let inset_columns = (inset / cell_width).ceil() as usize;
        max_columns.saturating_sub(inset_columns).max(1)
    }

    pub fn render(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let theme = cx.theme();

        let padding = Self::traceback_padding(window);

        Some(
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .font_buffer(cx)
                        .child(
                            Label::new(format!("{}: ", self.ename.clone()))
                                .color(Color::Error)
                                .weight(FontWeight::BOLD),
                        )
                        .child(Label::new(self.evalue.clone()).weight(FontWeight::BOLD)),
                )
                .child(
                    div()
                        .w_full()
                        .px(padding)
                        .py(padding)
                        .border_l(TRACEBACK_BORDER_WIDTH)
                        .border_color(theme.status().error_border)
                        .child(self.traceback.clone()),
                )
                .into_any_element(),
        )
    }
}
