use thiserror::Error;

pub type Result<T> = std::result::Result<T, SpeechError>;

#[derive(Error, Debug)]
pub enum SpeechError {
    #[error("Audio processing error: {0}")]
    Audio(#[from] AudioError),
    
    #[error("Speech-to-text error: {0}")]
    Stt(#[from] SttError),
    
    #[error("Text-to-speech error: {0}")]
    Tts(#[from] TtsError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[cfg(any(feature = "openai", feature = "elevenlabs"))]
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Unsupported audio format")]
    UnsupportedFormat,
    
    #[error("Audio device not available")]
    DeviceNotAvailable,
    
    #[error("Audio stream error: {0}")]
    StreamError(String),
    
    #[error("Sample rate conversion failed")]
    ResamplingFailed,
    
    #[error("Audio file parsing failed: {0}")]
    ParseError(String),
    
    #[error("Invalid audio data")]
    InvalidData,
}

#[derive(Error, Debug)]
pub enum SttError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    
    #[error("Model load error: {0}")]
    ModelLoadError(String),
    
    #[error("Audio processing error: {0}")]
    AudioProcessingError(String),
    
    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),
    
    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),
    
    #[error("Language not supported: {0}")]
    LanguageNotSupported(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Error, Debug)]
pub enum TtsError {
    #[error("TTS provider not available: {0}")]
    ProviderNotAvailable(String),
    
    #[error("Voice not found: {0}")]
    VoiceNotFound(String),
    
    #[error("Synthesis failed: {0}")]
    SynthesisFailed(String),
    
    #[error("Text too long for synthesis")]
    TextTooLong,
    
    #[error("Invalid voice parameters")]
    InvalidVoiceParameters,
    
    #[error("API quota exceeded")]
    QuotaExceeded,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
}

impl From<hound::Error> for SpeechError {
    fn from(err: hound::Error) -> Self {
        SpeechError::Audio(AudioError::ParseError(err.to_string()))
    }
}

impl From<cpal::BuildStreamError> for SpeechError {
    fn from(err: cpal::BuildStreamError) -> Self {
        SpeechError::Audio(AudioError::StreamError(err.to_string()))
    }
}

impl From<cpal::PlayStreamError> for SpeechError {
    fn from(err: cpal::PlayStreamError) -> Self {
        SpeechError::Audio(AudioError::StreamError(err.to_string()))
    }
} 