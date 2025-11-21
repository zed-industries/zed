use async_channel::{Receiver, Sender};
use gpui::{actions, App, Global, Subscription, UpdateGlobal};
use log::{error, info, warn};
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::sync::Arc;

mod thread_loop;

actions!(
    transcription,
    [
        /// Toggles the speech recognizer on and off.
        ToggleDictationChannel,
        /// Toggles piping speech transcriptions to the AI assistant.
        ToggleSpeechAssistant
    ]
);

#[derive(Clone, Debug)]
pub enum TranscriptionNotification {
    ModelNotFound(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TranscriptionThreadState {
    Disabled,
    Idle,
    Listening,
    Transcribing,
}

pub struct TranscriptionNotificationStream {
    receiver: Receiver<TranscriptionNotification>,
}

impl TranscriptionNotificationStream {
    fn new(receiver: Receiver<TranscriptionNotification>) -> Self {
        Self { receiver }
    }

    pub async fn recv(&mut self) -> Option<TranscriptionNotification> {
        self.receiver.recv().await.ok()
    }
}

pub struct Transcription {
    state: Arc<Mutex<TranscriptionThreadState>>,
    task: Option<std::thread::JoinHandle<()>>,
    transcription_sender: Sender<String>,
    notification_sender: Sender<TranscriptionNotification>,
    notification_subscribers: Arc<Mutex<Vec<Sender<TranscriptionNotification>>>>,
    transcription_subscribers:
        Arc<Mutex<BTreeMap<usize, Box<dyn FnMut(String, &mut App) -> bool + Send>>>>,
    next_subscriber_id: usize,
}

impl Global for Transcription {}

impl Transcription {
    pub fn state(&self) -> TranscriptionThreadState {
        *self.state.lock()
    }

    fn new(cx: &mut App) -> Self {
        info!("Initializing speech global");
        let (transcription_sender, transcription_receiver) = async_channel::unbounded::<String>();
        let (notification_sender, notification_receiver) = async_channel::unbounded();
        let notification_subscribers = Arc::new(Mutex::new(Vec::new()));
        let state = Arc::new(Mutex::new(TranscriptionThreadState::Idle));
        let transcription_subscribers = Arc::new(Mutex::new(BTreeMap::<
            usize,
            Box<dyn FnMut(String, &mut App) -> bool + Send>,
        >::new()));

        {
            let notifications: Receiver<TranscriptionNotification> = notification_receiver.clone();
            let subscribers = notification_subscribers.clone();
            cx.spawn(async move |_| {
                while let Ok(notification) = notifications.recv().await {
                    Self::broadcast(&subscribers, notification.clone());
                    #[allow(irrefutable_let_patterns)] // More notifications to come
                    if let TranscriptionNotification::ModelNotFound(path) = notification {
                        warn!("Speech model not found at: {path}");
                    }
                }
            })
            .detach();
        }

        {
            let task_subscribers = transcription_subscribers.clone();
            let transcription_receiver = transcription_receiver.clone();
            cx.spawn(async move |cx| {
                while let Ok(text) = transcription_receiver.recv().await {
                    if task_subscribers.lock().is_empty() {
                        continue;
                    }

                    let text = text.clone();
                    cx.update(|cx| {
                        let mut subscribers = task_subscribers.lock();
                        for (_, callback) in subscribers.iter_mut() {
                            if callback(text.clone(), cx) {
                                break;
                            }
                        }
                    })
                    .ok();
                }
            })
            .detach();
        }

        Self {
            state,
            task: None,
            transcription_sender,
            notification_sender,
            notification_subscribers,
            transcription_subscribers,
            next_subscriber_id: 0,
        }
    }

    pub fn subscribe(
        &mut self,
        callback: impl FnMut(String, &mut App) -> bool + Send + 'static,
    ) -> Subscription {
        let id = self.next_subscriber_id;
        self.next_subscriber_id += 1;
        self.transcription_subscribers
            .lock()
            .insert(id, Box::new(callback));

        let subscribers = self.transcription_subscribers.clone();
        Subscription::new(move || {
            subscribers.lock().remove(&id);
        })
    }

    pub fn subscribe_notifications(&self) -> TranscriptionNotificationStream {
        let (sender, receiver) = async_channel::unbounded();
        self.notification_subscribers.lock().push(sender);
        TranscriptionNotificationStream::new(receiver)
    }

    fn broadcast<T: Clone>(subscribers: &Arc<Mutex<Vec<Sender<T>>>>, value: T) {
        let mut sinks = subscribers.lock();
        sinks.retain(|subscriber| subscriber.try_send(value.clone()).is_ok());
    }

    fn toggle_listening(&mut self, cx: &mut App) {
        let mut state = self.state.lock();
        if let Some(thread_handle) = self.task.take() {
            *state = TranscriptionThreadState::Disabled;
            drop(state);
            thread_handle
                .join()
                .unwrap_or_else(|_| warn!("Failed to join speech thread"));
            info!("Speech listening stopped");
        } else {
            *state = TranscriptionThreadState::Listening;
            drop(state);

            let transcription_sender = self.transcription_sender.clone();
            let notification_sender = self.notification_sender.clone();
            self.task = Some(Transcription::run_transcription_loop(
                self.state.clone(),
                transcription_sender,
                notification_sender,
                cx,
            ));
            info!("Speech listening started");
        }
    }

    fn run_transcription_loop(
        state: Arc<Mutex<TranscriptionThreadState>>,
        transcription_sender: Sender<String>,
        notification_sender: Sender<TranscriptionNotification>,
        _cx: &mut App,
    ) -> std::thread::JoinHandle<()> {
        info!("Launching transcription loop");
        std::thread::spawn(move || {
            if let Err(err) = thread_loop::transcription_loop_body(
                state,
                transcription_sender,
                notification_sender,
            ) {
                error!("error in transcription loop: {}", err);
            }
        })
    }
}

pub fn init(cx: &mut App) {
    let speech = Transcription::new(cx);
    cx.set_global(speech);

    cx.on_action(|_: &ToggleDictationChannel, cx| {
        Transcription::update_global(cx, |speech, cx| {
            speech.toggle_listening(cx);
        });
    });
}
