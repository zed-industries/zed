use gpui::{div, AnyElement, Div, Element, Entity, IntoElement, Render, Subscription, ViewContext};
use settings::{Settings, SettingsStore};
use workspace::{item::ItemHandle, ui::Label, StatusItemView};

use crate::{state::Mode, Vim, VimEvent, VimModeSetting};

pub struct ModeIndicator {
    pub mode: Option<Mode>,
    // _subscription: Subscription,
}

impl ModeIndicator {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_global::<Vim>(|this, cx| this.set_mode(Vim::read(cx).state().mode, cx))
            .detach();

        cx.observe_global::<SettingsStore>(move |mode_indicator, cx| {
            if VimModeSetting::get_global(cx).0 {
                mode_indicator.mode = cx
                    .has_global::<Vim>()
                    .then(|| cx.global::<Vim>().state().mode);
            } else {
                mode_indicator.mode.take();
            }
        })
        .detach();

        // Vim doesn't exist in some tests
        let mode = cx
            .has_global::<Vim>()
            .then(|| {
                let vim = cx.global::<Vim>();
                vim.enabled.then(|| vim.state().mode)
            })
            .flatten();

        Self {
            mode,
            //    _subscription,
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement {
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
