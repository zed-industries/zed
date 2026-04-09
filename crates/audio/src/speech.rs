use anyhow::Result;
use futures::channel::mpsc;
use gpui::{App, AsyncApp, Task};

/// Events emitted by the speech recognizer during a dictation session.
#[derive(Debug, Clone)]
pub enum SpeechEvent {
    /// Partial (in-progress) transcription that may change.
    PartialResult(String),
    /// Final transcription for a segment.
    FinalResult(String),
    /// Recognition encountered an error.
    Error(String),
}

/// Platform-agnostic trait for speech-to-text.
pub trait SpeechRecognizer: Send + Sync + 'static {
    /// Whether speech recognition is available on this platform.
    fn is_available() -> bool;

    /// Request user authorization for speech recognition.
    /// Returns true if authorized.
    fn request_authorization(cx: &mut App) -> Task<Result<bool>>;

    /// Start live speech recognition from the default microphone.
    /// Returns a channel that receives transcription events.
    fn start(cx: &mut App) -> Result<mpsc::UnboundedReceiver<SpeechEvent>>;

    /// Stop the current recognition session.
    fn stop();
}

// Platform implementations

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::MacSpeechRecognizer as PlatformSpeechRecognizer;

#[cfg(not(target_os = "macos"))]
mod stub;

#[cfg(not(target_os = "macos"))]
pub use stub::StubSpeechRecognizer as PlatformSpeechRecognizer;
