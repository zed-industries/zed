use gpui::{div, AnyElement, Element, IntoElement, Render, Subscription, ViewContext};
use settings::SettingsStore;
use workspace::{item::ItemHandle, ui::Label, StatusItemView};

use crate::{state::Mode, Vim};

pub struct ModeIndicator {
    pub mode: Option<Mode>,
    _subscriptions: Vec<Subscription>,
}

impl ModeIndicator {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let _subscriptions = vec![
            cx.observe_global::<Vim>(|this, cx| this.update_mode(cx)),
            cx.observe_global::<SettingsStore>(|this, cx| this.update_mode(cx)),
        ];

        let mut this = Self {
            mode: None,
            _subscriptions,
        };
        this.update_mode(cx);
        this
    }

    fn update_mode(&mut self, cx: &mut ViewContext<Self>) {
        // Vim doesn't exist in some tests
        if !cx.has_global::<Vim>() {
            return;
        }

        let vim = Vim::read(cx);
        if vim.enabled {
            self.mode = Some(vim.state().mode);
        } else {
            self.mode = None;
        }
    }

    pub fn set_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        if self.mode != Some(mode) {
            self.mode = Some(mode);
            cx.notify();
        }
    }
}

impl Render for ModeIndicator {
    type Element = AnyElement;

    fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement {
        let Some(mode) = self.mode.as_ref() else {
            return div().into_any();
        };

        let text = match mode {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Visual => "-- VISUAL --",
            Mode::VisualLine => "-- VISUAL LINE --",
            Mode::VisualBlock => "-- VISUAL BLOCK --",
        };
        Label::new(text).into_any_element()
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
