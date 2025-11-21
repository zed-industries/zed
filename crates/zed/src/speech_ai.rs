use gpui::{App, Global, Subscription, UpdateGlobal};
use log::{info};
use transcription::{Transcription, ToggleSpeechAssistant};
use zed_actions::assistant::InlineAssist;

pub fn init(cx: &mut App) {
    cx.set_global(TranscriptionInlineAssistantBridge::new());

    cx.on_action(|_: &ToggleSpeechAssistant, cx| {
        TranscriptionInlineAssistantBridge::update_global(cx, |bridge, cx| {
            bridge.toggle(cx);
        });
    });
}

struct TranscriptionInlineAssistantBridge {
    enabled: bool,
    subscription: Option<Subscription>,
}

impl TranscriptionInlineAssistantBridge {
    fn new() -> Self {
        Self {
            enabled: false,
            subscription: None,
        }
    }

    fn toggle(&mut self, cx: &mut App) {
        if self.enabled {
            self.disable();
        } else {
            self.enable(cx);
        }
    }

    fn enable(&mut self, cx: &mut App) {
        if self.enabled {
            return;
        }

        let subscription = Transcription::update_global(cx, |speech, _| speech.subscribe(|text, cx| {
            cx.spawn(async move |cx| {
                let action = InlineAssist {
                    prompt: Some(text),
                    auto_start: true,
                };
                cx.update(|cx| {
                    cx.dispatch_action(&action);
                }).ok();
            }).detach();

            true
        }));
        self.subscription = Some(subscription);
        self.enabled = true;
        info!("Speech assistant bridge enabled");
    }

    fn disable(&mut self) {
        if !self.enabled {
            return;
        }

        self.enabled = false;
        let _ = self.subscription.take();
        info!("Speech assistant bridge disabled");
    }
}

impl Global for TranscriptionInlineAssistantBridge {}
