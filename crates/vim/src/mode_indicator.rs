use gpui::{Context, Element, Entity, FontWeight, Render, Subscription, WeakEntity, Window, div};
use ui::text_for_keystrokes;
use workspace::{StatusItemView, item::ItemHandle, ui::prelude::*};

use crate::{Vim, VimEvent, VimGlobals};

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    vim: Option<WeakEntity<Vim>>,
    pending_keys: Option<String>,
    vim_subscription: Option<Subscription>,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending_keys(window, cx);
            cx.notify();
        })
        .detach();

        let handle = cx.entity();
        let window_handle = window.window_handle();
        cx.observe_new::<Vim>(move |_, window, cx| {
            let Some(window) = window else {
                return;
            };
            if window.window_handle() != window_handle {
                return;
            }
            let vim = cx.entity();
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

    fn update_pending_keys(&mut self, window: &mut Window, cx: &App) {
        self.pending_keys = window
            .pending_input_keystrokes()
            .map(|keystrokes| text_for_keystrokes(keystrokes, cx));
    }

    fn vim(&self) -> Option<Entity<Vim>> {
        self.vim.as_ref().and_then(|vim| vim.upgrade())
    }

    fn current_operators_description(&self, vim: Entity<Vim>, cx: &mut Context<Self>) -> String {
        let recording = Vim::globals(cx)
            .recording_register
            .map(|reg| format!("recording @{reg} "))
            .into_iter();

        let vim = vim.read(cx);
        recording
            .chain(
                cx.global::<VimGlobals>()
                    .pre_count
                    .map(|count| format!("{}", count)),
            )
            .chain(vim.selected_register.map(|reg| format!("\"{reg}")))
            .chain(vim.operator_stack.iter().map(|item| item.status()))
            .chain(
                cx.global::<VimGlobals>()
                    .post_count
                    .map(|count| format!("{}", count)),
            )
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Render for ModeIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vim = self.vim();
        let Some(vim) = vim else {
            return div().hidden().into_any_element();
        };

        let vim_readable = vim.read(cx);
        let status_label = vim_readable.status_label.clone();
        let temp_mode = vim_readable.temp_mode;
        let mode = vim_readable.mode;

        let theme = cx.theme();
        let colors = theme.colors();
        let system_transparent = gpui::hsla(0.0, 0.0, 0.0, 0.0);
        let vim_mode_text = match mode {
            crate::state::Mode::Normal => colors.vim_normal_foreground,
            crate::state::Mode::Insert => colors.vim_insert_foreground,
            crate::state::Mode::Replace => colors.vim_replace_foreground,
            crate::state::Mode::Visual => colors.vim_visual_foreground,
            crate::state::Mode::VisualLine => colors.vim_visual_line_foreground,
            crate::state::Mode::VisualBlock => colors.vim_visual_block_foreground,
            crate::state::Mode::HelixNormal => colors.vim_helix_normal_foreground,
            crate::state::Mode::HelixSelect => colors.vim_helix_select_foreground,
        };
        let bg_color = match mode {
            crate::state::Mode::Normal => colors.vim_normal_background,
            crate::state::Mode::Insert => colors.vim_insert_background,
            crate::state::Mode::Replace => colors.vim_replace_background,
            crate::state::Mode::Visual => colors.vim_visual_background,
            crate::state::Mode::VisualLine => colors.vim_visual_line_background,
            crate::state::Mode::VisualBlock => colors.vim_visual_block_background,
            crate::state::Mode::HelixNormal => colors.vim_helix_normal_background,
            crate::state::Mode::HelixSelect => colors.vim_helix_select_background,
        };

        let (label, mode): (SharedString, Option<SharedString>) = if let Some(label) = status_label
        {
            (label, None)
        } else {
            let mode_str = if temp_mode {
                format!("(insert) {}", mode)
            } else {
                mode.to_string()
            };

            let current_operators_description = self.current_operators_description(vim.clone(), cx);
            let pending = self
                .pending_keys
                .as_ref()
                .unwrap_or(&current_operators_description);
            let mode = if bg_color != system_transparent {
                mode_str.into()
            } else {
                format!("-- {} --", mode_str).into()
            };
            (pending.into(), Some(mode))
        };
        h_flex()
            .gap_1()
            .when(!label.is_empty(), |el| {
                el.child(
                    Label::new(label)
                        .line_height_style(LineHeightStyle::UiLabel)
                        .weight(FontWeight::MEDIUM),
                )
            })
            .when_some(mode, |el, mode| {
                el.child(
                    v_flex()
                        .when(bg_color != system_transparent, |el| el.px_2())
                        // match with other icons at the bottom that use default buttons
                        .h(ButtonSize::Default.rems())
                        .justify_center()
                        .rounded_sm()
                        .bg(bg_color)
                        .child(
                            Label::new(mode)
                                .size(LabelSize::Small)
                                .line_height_style(LineHeightStyle::UiLabel)
                                .weight(FontWeight::MEDIUM)
                                .when(
                                    bg_color != system_transparent
                                        && vim_mode_text != system_transparent,
                                    |el| el.color(Color::Custom(vim_mode_text)),
                                ),
                        ),
                )
            })
            .into_any()
    }
}

impl StatusItemView for ModeIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
