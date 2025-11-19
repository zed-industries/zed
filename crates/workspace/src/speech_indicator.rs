use crate::StatusItemView;
use gpui::{
    Action, Context, IntoElement, MouseButton, Render, Subscription, Window, div, prelude::*, svg,
};
use speech::{Speech, SpeechState, ToggleListening};
use theme::ActiveTheme;

pub struct SpeechIndicator {
    subscription: Option<Subscription>,
    state: SpeechState,
}

impl SpeechIndicator {
    pub fn new() -> Self {
        Self {
            subscription: None,
            state: SpeechState::Idle,
        }
    }
}

impl Render for SpeechIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.subscription.is_none() {
            self.subscription = Some(cx.observe_global::<Speech>(|this, cx| {
                let speech_state = cx.global::<Speech>().state();
                if this.state != speech_state {
                    this.state = speech_state;
                    cx.notify();
                }
            }));
        }

        let color = match self.state {
            SpeechState::Listening => cx.theme().colors().text_accent,
            _ => cx.theme().colors().text,
        };

        div()
            .child(svg().path("icons/mic.svg").w_4().h_4().text_color(color))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_, _, window, cx| {
                    window.dispatch_action(ToggleListening.boxed_clone(), cx);
                }),
            )
    }
}

impl StatusItemView for SpeechIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn crate::ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // Not needed for this indicator
    }
}
