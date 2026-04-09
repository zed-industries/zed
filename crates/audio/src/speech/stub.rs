use super::{SpeechEvent, SpeechRecognizer};
use anyhow::{Result, anyhow};
use futures::channel::mpsc;
use gpui::{App, Task};

pub struct StubSpeechRecognizer;

impl SpeechRecognizer for StubSpeechRecognizer {
    fn is_available() -> bool {
        false
    }

    fn request_authorization(_cx: &mut App) -> Task<Result<bool>> {
        Task::ready(Ok(false))
    }

    fn start(_cx: &mut App) -> Result<mpsc::UnboundedReceiver<SpeechEvent>> {
        Err(anyhow!("Speech recognition is not available on this platform"))
    }

    fn stop() {}
}
