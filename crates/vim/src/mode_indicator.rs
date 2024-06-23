use gpui::{div, Element, Render, Subscription, ViewContext};
use itertools::Itertools;
use workspace::{item::ItemHandle, ui::prelude::*, StatusItemView};

use crate::{state::Mode, Vim};

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    pub(crate) mode: Option<Mode>,
    pub(crate) operators: String,
    pending_keys: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let _subscriptions = vec![
            cx.observe_global::<Vim>(|this, cx| this.update_mode(cx)),
            cx.observe_pending_input(|this, cx| {
                this.update_pending_keys(cx);
                cx.notify();
            }),
        ];

        let mut this = Self {
            mode: None,
            operators: "".to_string(),
            pending_keys: None,
            _subscriptions,
        };
        this.update_mode(cx);
        this
    }

    fn update_mode(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(vim) = self.vim(cx) {
            self.mode = Some(vim.state().mode);
            self.operators = self.current_operators_description(&vim);
        } else {
            self.mode = None;
        }
    }

    fn update_pending_keys(&mut self, cx: &mut ViewContext<Self>) {
        if self.vim(cx).is_some() {
            self.pending_keys = cx.pending_input_keystrokes().map(|keystrokes| {
                keystrokes
                    .iter()
                    .map(|keystroke| format!("{}", keystroke))
                    .join(" ")
            });
        } else {
            self.pending_keys = None;
        }
    }

    fn vim<'a>(&self, cx: &'a mut ViewContext<Self>) -> Option<&'a Vim> {
        // In some tests Vim isn't enabled, so we use try_global.
        cx.try_global::<Vim>().filter(|vim| vim.enabled)
    }

    fn current_operators_description(&self, vim: &Vim) -> String {
        vim.state()
            .pre_count
            .map(|count| format!("{}", count))
            .into_iter()
            .chain(vim.state().selected_register.map(|reg| format!("\"{reg}")))
            .chain(
                vim.state()
                    .operator_stack
                    .iter()
                    .map(|item| item.id().to_string()),
            )
            .chain(vim.state().post_count.map(|count| format!("{}", count)))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Render for ModeIndicator {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(mode) = self.mode.as_ref() else {
            return div().into_any();
        };

        let pending = self.pending_keys.as_ref().unwrap_or(&self.operators);

        Label::new(format!("{} -- {} --", pending, mode))
            .size(LabelSize::Small)
            .line_height_style(LineHeightStyle::UiLabel)
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
