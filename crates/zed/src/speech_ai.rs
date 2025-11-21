use gpui::{App, Global, Task, UpdateGlobal};
use log::{info, warn};
use speech::{Speech, ToggleSpeechAssistant, TranscriptionReceiverMutex};
use zed_actions::assistant::InlineAssist;

pub fn init(cx: &mut App) {
    cx.set_global(SpeechInlineAssistantBridge::new());

    cx.on_action(|_: &ToggleSpeechAssistant, cx| {
        SpeechInlineAssistantBridge::update_global(cx, |bridge, cx| {
            bridge.toggle(cx);
        });
    });
}

struct SpeechInlineAssistantBridge {
    enabled: bool,
    task: Option<Task<()>>,
}

impl SpeechInlineAssistantBridge {
    fn new() -> Self {
        Self {
            enabled: false,
            task: None,
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

        let stream = Speech::update_global(cx, |speech, _| speech.transcription_receiver());
        self.task = Some(Self::spawn_task(stream, cx));
        self.enabled = true;
        info!("Speech assistant bridge enabled");
    }

    fn disable(&mut self) {
        if !self.enabled {
            return;
        }

        self.enabled = false;
        if let Some(task) = self.task.take() {
            drop(task);
        }
        info!("Speech assistant bridge disabled");
    }

    fn spawn_task(stream: TranscriptionReceiverMutex, cx: &mut App) -> Task<()> {
        cx.spawn(async move |cx| {
            loop {
                let Ok(transcription) = stream.lock().recv().await else {
                    continue;
                };
                let trimmed = transcription.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let prompt = trimmed.to_owned();
                if let Err(err) = cx.update(|cx| {
                    let action = InlineAssist {
                        prompt: Some(prompt.clone()),
                        auto_start: true,
                    };
                    cx.dispatch_action(&action);
                }) {
                    warn!("Failed to dispatch inline assist for speech transcription: {err}");
                    break;
                }
            }
        })
    }
}

impl Global for SpeechInlineAssistantBridge {}
