use gpui::{elements::Label, AnyElement, Element, Entity, View, ViewContext};
use workspace::{item::ItemHandle, StatusItemView};

use crate::state::Mode;

pub struct ModeIndicator {
    pub mode: Mode,
}

impl ModeIndicator {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }

    pub fn set_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        if mode != self.mode {
            self.mode = mode;
            cx.notify();
        }
    }
}

impl Entity for ModeIndicator {
    type Event = ();
}

impl View for ModeIndicator {
    fn ui_name() -> &'static str {
        "ModeIndicatorView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &theme::current(cx).workspace.status_bar;
        // we always choose text to be 12 monospace characters
        // so that as the mode indicator changes, the rest of the
        // UI stays still.
        let text = match self.mode {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Visual { line: false } => "-- VISUAL --",
            Mode::Visual { line: true } => "VISUAL LINE ",
        };
        Label::new(text, theme.vim_mode_indicator.text.clone())
            .contained()
            .with_style(theme.vim_mode_indicator.container)
            .into_any()
    }
}

impl StatusItemView for ModeIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        // nothing to do.
    }
}
