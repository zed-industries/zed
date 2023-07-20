use gpui::{
    elements::{Empty, Label},
    AnyElement, Element, Entity, View, ViewContext,
};
use workspace::{item::ItemHandle, StatusItemView};

use crate::{state::Mode, Vim};

pub struct ModeIndicator {
    mode: Option<Mode>,
}

impl ModeIndicator {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_global::<Vim, _>(|this, cx| {
            let vim = Vim::read(cx);
            if vim.enabled {
                this.set_mode(Some(Vim::read(cx).state.mode), cx)
            } else {
                this.set_mode(None, cx)
            }
        })
        .detach();
        Self { mode: None }
    }

    pub fn set_mode(&mut self, mode: Option<Mode>, cx: &mut ViewContext<Self>) {
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
        "ModeIndicator"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(mode) = self.mode {
            let theme = &theme::current(cx).workspace.status_bar;
            let text = match mode {
                Mode::Normal => "",
                Mode::Insert => "--- INSERT ---",
                Mode::Visual { line: false } => "--- VISUAL ---",
                Mode::Visual { line: true } => "--- VISUAL LINE ---",
            };
            Label::new(text, theme.vim_mode.clone()).into_any()
        } else {
            Empty::new().into_any()
        }
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
