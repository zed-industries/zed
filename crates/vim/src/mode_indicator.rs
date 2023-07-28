use gpui::{
    elements::{Empty, Label},
    AnyElement, Element, Entity, Subscription, View, ViewContext,
};
use settings::SettingsStore;
use workspace::{item::ItemHandle, StatusItemView};

use crate::{state::Mode, Vim, VimEvent, VimModeSetting};

pub struct ModeIndicator {
    pub mode: Option<Mode>,
    _subscription: Subscription,
}

impl ModeIndicator {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.handle().downgrade();

        let _subscription = cx.subscribe_global::<VimEvent, _>(move |&event, cx| {
            if let Some(mode_indicator) = handle.upgrade(cx) {
                match event {
                    VimEvent::ModeChanged { mode } => {
                        cx.update_window(mode_indicator.window_id(), |cx| {
                            mode_indicator.update(cx, move |mode_indicator, cx| {
                                mode_indicator.set_mode(mode, cx);
                            })
                        });
                    }
                }
            }
        });

        cx.observe_global::<SettingsStore, _>(move |mode_indicator, cx| {
            if settings::get::<VimModeSetting>(cx).0 {
                mode_indicator.mode = cx
                    .has_global::<Vim>()
                    .then(|| cx.global::<Vim>().state.mode);
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
                vim.enabled.then(|| vim.state.mode)
            })
            .flatten();

        Self {
            mode,
            _subscription,
        }
    }

    pub fn set_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        if self.mode != Some(mode) {
            self.mode = Some(mode);
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
        let Some(mode) = self.mode.as_ref() else {
            return Empty::new().into_any();
        };

        let theme = &theme::current(cx).workspace.status_bar;

        // we always choose text to be 12 monospace characters
        // so that as the mode indicator changes, the rest of the
        // UI stays still.
        let text = match mode {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Visual { line: false } => "-- VISUAL --",
            Mode::Visual { line: true } => "VISUAL  LINE",
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
