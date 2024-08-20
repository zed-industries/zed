use editor::Editor;
use gpui::{div, Element, Render, Subscription, View, ViewContext, WeakView};
use itertools::Itertools;
use workspace::{item::ItemHandle, ui::prelude::*, StatusItemView};

use crate::{Vim, VimAddon};

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    vim: Option<WeakView<Vim>>,
    pending_keys: Option<String>,
    _keys_subscription: Subscription,
    vim_subscription: Option<Subscription>,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let keys_subscription = cx.observe_pending_input(|this, cx| {
            this.update_pending_keys(cx);
            cx.notify();
        });

        let mut this = Self {
            vim: None,
            pending_keys: None,
            _keys_subscription: keys_subscription,
            vim_subscription: None,
        };
        this.update_mode(cx);
        this
    }

    fn update_mode(&mut self, _: &mut ViewContext<Self>) {
        // if let Some(vim) = self.vim(cx) {
        //     self.mode = Some(vim.mode);
        //     self.operators = self.current_operators_description(&vim, cx);
        // } else {
        //     self.mode = None;
        // }
    }

    fn update_pending_keys(&mut self, cx: &mut ViewContext<Self>) {
        self.pending_keys = cx.pending_input_keystrokes().map(|keystrokes| {
            keystrokes
                .iter()
                .map(|keystroke| format!("{}", keystroke))
                .join(" ")
        });
    }

    fn vim<'a>(&self) -> Option<View<Vim>> {
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
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(vim) = active_pane_item
            .and_then(|item| item.downcast::<Editor>())
            .and_then(|editor| editor.read(cx).addon::<VimAddon>())
            .map(|addon| addon.view.clone())
        else {
            self.vim.take();
            self.vim_subscription.take();
            return;
        };
        self.vim_subscription = Some(cx.observe(&vim, |_, _, cx| cx.notify()));
        self.vim = Some(vim.downgrade());
    }
}
