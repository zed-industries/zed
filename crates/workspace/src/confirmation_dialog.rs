use futures::channel::oneshot;
use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight, IntoElement,
    Render, Task, Window, actions,
};
use menu;
use std::sync::Arc;
use ui::{Button, ButtonStyle, Label, LabelSize, TintColor, prelude::*};

use crate::modal_layer::ModalView;

actions!(confirmation_dialog, [ConfirmDontSave]);

pub struct ConfirmationDialog {
    message: Arc<str>,
    detail: Option<Arc<str>>,
    buttons: Vec<String>,
    selected_button: usize,
    focus_handle: FocusHandle,
    result_sender: Option<oneshot::Sender<usize>>,
    show_key_h_for_dont_save: bool,
}

impl ModalView for ConfirmationDialog {}

impl ConfirmationDialog {
    pub fn show(
        workspace: &mut crate::Workspace,
        message: impl Into<Arc<str>>,
        detail: Option<impl Into<Arc<str>>>,
        buttons: Vec<impl Into<String>>,
        show_key_h_for_dont_save: bool,
        window: &mut Window,
        cx: &mut Context<crate::Workspace>,
    ) -> Task<Option<usize>> {
        let message = message.into();
        let detail = detail.map(|d| d.into());
        let buttons: Vec<String> = buttons.into_iter().map(|b| b.into()).collect();

        let (sender, receiver) = oneshot::channel();

        workspace.toggle_modal(window, cx, |window, cx| {
            let modal = Self {
                message: message.clone(),
                detail: detail.clone(),
                buttons: buttons.clone(),
                selected_button: 0,
                focus_handle: cx.focus_handle(),
                result_sender: Some(sender),
                show_key_h_for_dont_save,
            };
            modal.focus_handle.focus(window, cx);
            modal
        });

        cx.spawn(async move |_workspace, _cx| receiver.await.ok())
    }

    fn select_button(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if index < self.buttons.len() {
            self.selected_button = index;
            cx.notify();
        }
    }

    fn confirm_selection(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sender) = self.result_sender.take() {
            let _ = sender.send(self.selected_button);
        }
        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sender) = self.result_sender.take() {
            // Find "Cancel" button index, or use last button as default
            let cancel_index = self
                .buttons
                .iter()
                .position(|b| b.to_lowercase().contains("cancel"))
                .unwrap_or(self.buttons.len().saturating_sub(1));
            let _ = sender.send(cancel_index);
        }
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ConfirmationDialog {}

impl Focusable for ConfirmationDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ConfirmationDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        v_flex()
            .key_context("ConfirmationDialog")
            .track_focus(&focus_handle)
            .w(rems(28.))
            .p_6()
            .gap_4()
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_lg()
            .shadow_lg()
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                this.confirm_selection(window, cx);
            }))
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| {
                this.cancel(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ConfirmDontSave, window, cx| {
                this.select_button(1, window, cx);
                this.confirm_selection(window, cx);
            }))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                match event.keystroke.key.as_str() {
                    "Enter" => {
                        this.select_button(0, window, cx);
                        this.confirm_selection(window, cx);
                    }
                    "Escape" => {
                        this.cancel(window, cx);
                    }
                    _ => {}
                }
            }))
            .child(
                // Warning icon and message
                h_flex()
                    .gap_3()
                    .items_start()
                    .flex_1()
                    .child(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Medium)
                            .color(Color::Warning),
                    )
                    .child(
                        v_flex()
                            .gap_3()
                            .flex_1()
                            .max_w_full()
                            .child(
                                div().w_full().overflow_hidden().pr_2().child(
                                    Label::new(self.message.clone())
                                        .size(LabelSize::Default)
                                        .weight(FontWeight::MEDIUM),
                                ),
                            )
                            .when_some(self.detail.clone(), |this, detail| {
                                this.child(
                                    div().w_full().overflow_hidden().pr_2().child(
                                        Label::new(detail)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                            }),
                    ),
            )
            .child(
                // Buttons
                h_flex()
                    .gap_2()
                    .justify_end()
                    .children(self.buttons.iter().enumerate().map(|(index, button_text)| {
                        let _is_selected = index == self.selected_button;
                        let is_primary = index == 0; // First button (Save) is primary
                        let is_destructive = button_text.to_lowercase().contains("don't save")
                            || button_text.to_lowercase().contains("discard");

                        // Add keybinding hint to button text
                        let mut button_text_with_key = button_text.clone();
                        if index == 0 {
                            button_text_with_key.push_str(" (Enter)");
                        } else if index == 1 && self.show_key_h_for_dont_save {
                            button_text_with_key.push_str(" (h)");
                        } else if index == self.buttons.len().saturating_sub(1) {
                            button_text_with_key.push_str(" (Escape)");
                        }

                        Button::new(("button", index), button_text_with_key)
                            .style(if is_primary {
                                ButtonStyle::Filled
                            } else if is_destructive {
                                ButtonStyle::Tinted(TintColor::Error)
                            } else {
                                ButtonStyle::Subtle
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.select_button(index, window, cx);
                                this.confirm_selection(window, cx);
                            }))
                    })),
            )
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.cancel(window, cx);
            }))
    }
}
