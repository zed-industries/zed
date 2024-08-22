use gpui::{div, Element, Render, Subscription, View, ViewContext, WeakView};
use itertools::Itertools;
use workspace::{item::ItemHandle, ui::prelude::*, StatusItemView};

use crate::{Vim, VimEvent};

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    vim: Option<WeakView<Vim>>,
    pending_keys: Option<String>,
    vim_subscription: Option<Subscription>,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_pending_input(|this, cx| {
            this.update_pending_keys(cx);
            cx.notify();
        })
        .detach();

        let handle = cx.view().clone();
        let window = cx.window_handle();
        cx.observe_new_views::<Vim>(move |_, cx| {
            if cx.window_handle() != window {
                return;
            }
            let vim = cx.view().clone();
            handle.update(cx, |_, cx| {
                cx.subscribe(&vim, |mode_indicator, vim, event, cx| match event {
                    VimEvent::Focused => {
                        mode_indicator.vim_subscription =
                            Some(cx.observe(&vim, |_, _, cx| cx.notify()));
                        mode_indicator.vim = Some(vim.downgrade());
                    }
                })
                .detach()
            })
        })
        .detach();

        Self {
            vim: None,
            pending_keys: None,
            vim_subscription: None,
        }
    }

    fn update_pending_keys(&mut self, cx: &mut ViewContext<Self>) {
        self.pending_keys = cx.pending_input_keystrokes().map(|keystrokes| {
            keystrokes
                .iter()
                .map(|keystroke| format!("{}", keystroke))
                .join(" ")
        });
    }

    fn vim(&self) -> Option<View<Vim>> {
        self.vim.as_ref().and_then(|vim| vim.upgrade())
    }

    fn current_operators_description(&self, vim: View<Vim>, cx: &mut ViewContext<Self>) -> String {
        let recording = Vim::globals(cx)
            .recording_register
            .map(|reg| format!("recording @{reg} "))
            .into_iter();

        let vim = vim.read(cx);
        recording
            .chain(vim.pre_count.map(|count| format!("{}", count)))
            .chain(vim.selected_register.map(|reg| format!("\"{reg}")))
            .chain(vim.operator_stack.iter().map(|item| item.id().to_string()))
            .chain(vim.post_count.map(|count| format!("{}", count)))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Render for ModeIndicator {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let vim = self.vim();
        let Some(vim) = vim else {
            return div().into_any();
        };

        let current_operators_description = self.current_operators_description(vim.clone(), cx);
        let pending = self
            .pending_keys
            .as_ref()
            .unwrap_or(&current_operators_description);
        Label::new(format!("{} -- {} --", pending, vim.read(cx).mode))
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
    }
}
