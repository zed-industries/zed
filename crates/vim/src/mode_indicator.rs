use gpui::{div, Element, Render, Subscription, ViewContext};
use workspace::{item::ItemHandle, ui::prelude::*, StatusItemView};

use crate::{state::Mode, Vim};

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    pub(crate) mode: Option<Mode>,
    _subscription: Subscription,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let _subscription = cx.observe_global::<Vim>(|this, cx| this.update_mode(cx));
        let mut this = Self {
            mode: None,
            _subscription,
        };
        this.update_mode(cx);
        this
    }

    fn update_mode(&mut self, cx: &mut ViewContext<Self>) {
        // Vim doesn't exist in some tests
        let Some(vim) = cx.try_global::<Vim>() else {
            return;
        };

        if vim.enabled {
            self.mode = Some(vim.state().mode);
        } else {
            self.mode = None;
        }
    }
}

impl Render for ModeIndicator {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(mode) = self.mode.as_ref() else {
            return div().into_any();
        };

        Label::new(format!("-- {} --", mode))
            .size(LabelSize::Small)
            .into_any_element()
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
