use std::ops::Range;

use editor::display_map::{is_invisible, replacement};
use gpui::{
    Context, Element, Entity, HighlightStyle, Render, StyledText, Subscription, WeakEntity, Window,
    div,
};
use ui::{LabelLike, text_for_keystrokes};
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

        let handle = cx.entity().clone();
        let window_handle = window.window_handle();
        cx.observe_new::<Vim>(move |_, window, cx| {
            let Some(window) = window else {
                return;
            };
            if window.window_handle() != window_handle {
                return;
            }
            let vim = cx.entity().clone();
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
            .chain(
                vim.operator_stack
                    .iter()
                    .map(|item| item.status().to_string()),
            )
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
            return div().into_any();
        };

        let vim_readable = vim.read(cx);
        if let Some(label) = vim_readable.status_label.clone() {
            Label::new(label)
                .size(LabelSize::Small)
                .line_height_style(LineHeightStyle::UiLabel)
                .into_any_element()
        } else {
            let mode = if vim_readable.temp_mode {
                format!("(insert) {}", vim_readable.mode)
            } else {
                vim_readable.mode.to_string()
            };

            let current_operators_description = self.current_operators_description(vim.clone(), cx);
            let (pending, _, runs) = self
                .pending_keys
                .as_ref()
                .unwrap_or(&current_operators_description)
                .chars()
                .fold(
                    (String::new(), 0usize, Vec::<Range<usize>>::new()),
                    |(pending, index, mut ranges), c| {
                        let (extend, highlight) = match c {
                            '\t' => ("\\t".to_string(), true),
                            '\n' => ("\\n".to_string(), true),
                            '\r' => ("\\r".to_string(), true),
                            c if is_invisible(c) => {
                                if c <= '\x1f' {
                                    (replacement(c).unwrap_or("").to_string(), true)
                                } else {
                                    (format!("\\u{:04X}", c as u32), true)
                                }
                            }
                            c => (c.to_string(), false),
                        };
                        let end = index + extend.len();
                        if highlight {
                            ranges.push(index..end);
                        }
                        (pending + &extend, end, ranges)
                    },
                );
            let highlights = runs.iter().map(|range| {
                (
                    range.clone(),
                    HighlightStyle::color(cx.theme().colors().text_muted),
                )
            });

            // The pending string needs to be at the start of the label or the highlights will be in the wrong position
            let label = format!("{} -- {} --", pending, mode);
            LabelLike::new()
                .size(LabelSize::Small)
                .line_height_style(LineHeightStyle::UiLabel)
                .child(StyledText::new(label).with_highlights(highlights))
                .into_any_element()
        }
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
