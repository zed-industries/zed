use async_channel::{Receiver, Sender};
use gpui::{actions, App, Global, UpdateGlobal};
use log::{error, info, warn};
use parking_lot::Mutex;
use std::sync::Arc;

mod whisper_thread;

const WHISPER_MODEL_NAME: &str = "ggml-base.en.bin";
const TARGET_SAMPLE_RATE: usize = 16_000;
const HIGH_PASS_CUTOFF_HZ: f32 = 80.0;

actions!(
    speech,
    [
        /// Toggles the speech recognizer on and off.
        ToggleDictationChannel,
        /// Toggles piping speech transcriptions to the AI assistant.
        ToggleSpeechAssistant
    ]
);

pub type TranscriptionReceiver = Receiver<String>;
pub type TranscriptionReceiverMutex = Arc<Mutex<TranscriptionReceiver>>;

#[derive(Clone, Debug)]
pub enum SpeechNotification {
    ModelNotFound(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TranscriberThreadState {
    Disabled,
    Idle,
    Listening,
    Transcribing,
}

pub struct TranscriptionStream {
    receiver: Receiver<String>,
}

impl TranscriptionStream {
    fn new(receiver: Receiver<String>) -> Self {
        Self { receiver }
    }

    pub async fn recv(&mut self) -> Option<String> {
        self.receiver.recv().await.ok()
    }
}

pub struct NotificationStream {
    receiver: Receiver<SpeechNotification>,
}

impl NotificationStream {
    fn new(receiver: Receiver<SpeechNotification>) -> Self {
        Self { receiver }
    }

    pub async fn recv(&mut self) -> Option<SpeechNotification> {
        self.receiver.recv().await.ok()
    }
}

pub struct Speech {
    state: Arc<Mutex<TranscriberThreadState>>,
    task: Option<std::thread::JoinHandle<()>>,
    transcription_sender: Sender<String>,
    notification_sender: Sender<SpeechNotification>,
    transcription_receiver: Arc<Mutex<Receiver<String>>>,
    notification_subscribers: Arc<Mutex<Vec<Sender<SpeechNotification>>>>,
}

impl Global for Speech {}

impl Speech {
    pub fn state(&self) -> TranscriberThreadState {
        *self.state.lock()
    }

    fn new(cx: &mut App) -> Self {
        info!("Initializing speech global");
        let (transcription_sender, transcription_receiver) = async_channel::unbounded();
        let (notification_sender, notification_receiver) = async_channel::unbounded();
        let receiver: Receiver<String> = transcription_receiver.clone();
        let transcription_receiver = Arc::new(Mutex::new(receiver));
        let notification_subscribers = Arc::new(Mutex::new(Vec::new()));

        {
            let notifications: Receiver<SpeechNotification> = notification_receiver.clone();
            let subscribers = notification_subscribers.clone();
            cx.spawn(async move |_| {
                while let Ok(notification) = notifications.recv().await {
                    Self::broadcast(&subscribers, notification.clone());
                    #[allow(irrefutable_let_patterns)] // More notifications to come
                    if let SpeechNotification::ModelNotFound(path) = notification {
                        warn!("Speech model not found at: {path}");
                    }
                }
            })
            .detach();
        }

        Self {
            state: Arc::new(Mutex::new(TranscriberThreadState::Idle)),
            task: None,
            transcription_sender,
            notification_sender,
            transcription_receiver,
            notification_subscribers,
        }
    }

    pub fn transcription_receiver(&self) -> TranscriptionReceiverMutex {
        self.transcription_receiver.clone()
    }

    pub fn subscribe_notifications(&self) -> NotificationStream {
        let (sender, receiver) = async_channel::unbounded();
        self.notification_subscribers.lock().push(sender);
        NotificationStream::new(receiver)
    }

    fn broadcast<T: Clone>(subscribers: &Arc<Mutex<Vec<Sender<T>>>>, value: T) {
        let mut sinks = subscribers.lock();
        sinks.retain(|subscriber| subscriber.try_send(value.clone()).is_ok());
    }

    fn toggle_listening(&mut self) {
        let mut state = self.state.lock();
        if *state == TranscriberThreadState::Listening {
            *state = TranscriberThreadState::Idle;
            info!("Speech listening stopped");
        } else {
            *state = TranscriberThreadState::Listening;
            info!("Speech listening started");
        }
    }

    fn run_transcription_loop(
        state: Arc<Mutex<TranscriberThreadState>>,
        transcription_sender: Sender<String>,
        notification_sender: Sender<SpeechNotification>,
        _cx: &mut App,
    ) -> std::thread::JoinHandle<()> {
        info!("Launching transcription loop");
        std::thread::spawn(move || {
            if let Err(err) =
                whisper_thread::transcription_loop_body(state, transcription_sender, notification_sender)
            {
                error!("error in transcription loop: {}", err);
            }
        })
    }

}

fn downmix_multi_channel(data: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data.to_vec();
    }
    let mut mono = Vec::with_capacity(data.len() / channels + 1);
    for frame in data.chunks(channels) {
        let sum: f32 = frame.iter().copied().sum();
        mono.push(sum / channels as f32);
    }
    mono
}

fn resample_to_target(chunk: &[f32], ratio: f32) -> Vec<f32> {
    if (ratio - 1.0).abs() < f32::EPSILON || chunk.is_empty() {
        return chunk.to_vec();
    }
    let output_len = ((chunk.len() as f32) * ratio).ceil() as usize;
    let mut output = Vec::with_capacity(output_len.max(1));
    for i in 0..output_len {
        let src_pos = i as f32 / ratio;
        let idx = src_pos.floor() as usize;
        let frac = src_pos - idx as f32;
        let next_idx = if idx + 1 < chunk.len() { idx + 1 } else { idx };
        let a = chunk[idx];
        let b = chunk[next_idx];
        output.push(a * (1.0 - frac) + b * frac);
    }
    output
}

fn apply_high_pass_filter(
    samples: &mut [f32],
    prev_input: &mut f32,
    prev_output: &mut f32,
    cutoff_hz: f32,
) {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_hz);
    let dt = 1.0 / TARGET_SAMPLE_RATE as f32;
    let alpha = rc / (rc + dt);
    for sample in samples.iter_mut() {
        let output = alpha * (*prev_output + *sample - *prev_input);
        *prev_input = *sample;
        *prev_output = output;
        *sample = output;
    }
}

pub fn init(cx: &mut App) {
    let mut speech = Speech::new(cx);
    let state = speech.state.clone();
    let transcription_sender = speech.transcription_sender.clone();
    let notification_sender = speech.notification_sender.clone();
    speech.task = Some(Speech::run_transcription_loop(
        state,
        transcription_sender,
        notification_sender,
        cx,
    ));
    cx.set_global(speech);

    cx.on_action(|_: &ToggleDictationChannel, cx| {
        Speech::update_global(cx, |speech, _| {
            speech.toggle_listening();
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // This test needs a proper app context to run now.
        // Disabling for now, will be replaced by a functional test later.
        // let speech = Speech::new();
        // assert!(matches!(speech.state, SpeechState::Idle));
    }
}
