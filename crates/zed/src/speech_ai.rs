use gpui::{App, Global, Task, UpdateGlobal};
use log::{info, warn};
use transcription::{Transcription, ToggleSpeechAssistant, TranscriptionReceiverMutex};
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
    task: Option<Task<()>>,
}

impl TranscriptionInlineAssistantBridge {
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

        let stream = Transcription::update_global(cx, |speech, _| speech.transcription_receiver());
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
                    info!("Transcription channel closed, stopping speech assistant bridge task.");
                    break;
                };

                if transcription.is_empty() {
                    continue;
                }

                let update = cx.update(|cx| {
                    let action = InlineAssist {
                        prompt: Some(transcription.clone()),
                        auto_start: true,
                    };
                    cx.dispatch_action(&action);
                });

                if let Err(err) = update {
                    warn!("Failed to dispatch inline assist for speech transcription: {err}");
                    break;
                }
            }
        })
    }
}

impl Global for TranscriptionInlineAssistantBridge {}
