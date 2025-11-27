use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render,
};
use ui::{Button, ButtonStyle, Clickable, Headline, HeadlineSize, prelude::*};
use ui_input::InputField;
use workspace::ModalView;
use zeta::Zeta;

pub struct SweepApiKeyModal {
    api_key_input: Entity<InputField>,
    focus_handle: FocusHandle,
}

impl SweepApiKeyModal {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_input = cx.new(|cx| InputField::new(window, cx, "Enter your Sweep API token"));

        Self {
            api_key_input,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_input.read(cx).text(cx);
        let api_key = (!api_key.trim().is_empty()).then_some(api_key);

        if let Some(zeta) = Zeta::try_global(cx) {
            zeta.update(cx, |zeta, cx| {
                zeta.sweep_ai
                    .set_api_token(api_key, cx)
                    .detach_and_log_err(cx);
            });
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for SweepApiKeyModal {}

impl ModalView for SweepApiKeyModal {}

impl Focusable for SweepApiKeyModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SweepApiKeyModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SweepApiKeyModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(px(400.))
            .p_4()
            .gap_3()
            .child(Headline::new("Sweep API Token").size(HeadlineSize::Small))
            .child(self.api_key_input.clone())
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(Button::new("cancel", "Cancel").on_click(cx.listener(
                        |_, _, _window, cx| {
                            cx.emit(DismissEvent);
                        },
                    )))
                    .child(
                        Button::new("save", "Save")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx);
                            })),
                    ),
            )
    }
}
