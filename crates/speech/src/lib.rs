//! # Speech Processing Library
//! 
//! This crate provides a unified interface for speech-to-text (STT) and text-to-speech (TTS)
//! functionality in Rust. It supports multiple providers and platforms.

pub mod audio;
pub mod config;
pub mod error;
pub mod stt;
pub mod tts;

// Re-export commonly used types and functions
pub use config::{SpeechConfig, SttConfig, TtsConfig, SttProvider, TtsProvider};
pub use error::{SpeechError, SttError, TtsError, Result};
pub use stt::{SpeechToText, SttResult, create_provider};
pub use tts::{TextToSpeech, TtsResult, create_provider as create_tts_provider, TtsService};

// Re-export audio utilities for convenience
pub use audio::{AudioProcessor, utils as audio_utils};

/// Main speech processing interface that combines STT and TTS
#[allow(dead_code)]
pub struct SpeechProcessor {
    stt: Box<dyn SpeechToText>,
    tts: std::sync::Arc<dyn TextToSpeech>,
    audio: AudioProcessor,
    config: SpeechConfig,
}

impl SpeechProcessor {
    /// Create a new speech processor with the given configuration
    pub async fn new(config: SpeechConfig) -> Result<Self> {
        config.validate()?;
        
        let stt = stt::create_provider(config.stt.clone()).await?;
        let tts = tts::create_provider(config.tts.clone()).await?;
        let audio = AudioProcessor::new(config.audio.clone())?;
        
        Ok(Self {
            stt,
            tts,
            audio,
            config,
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &SpeechConfig {
        &self.config
    }

    /// Transcribe audio samples to text
    pub async fn transcribe_audio(&self, audio_data: &[f32]) -> Result<SttResult> {
        self.stt.transcribe_audio(audio_data).await
    }

    /// Transcribe audio from a file
    pub async fn transcribe_file(&self, file_path: &str) -> Result<SttResult> {
        // Load audio file and convert to samples
        let audio_data = std::fs::read(file_path)?;
        let (samples, _sample_rate, _channels) = AudioProcessor::decode_audio(&audio_data)?;
        self.stt.transcribe_audio(&samples).await
    }

    /// Start real-time transcription
    pub async fn start_realtime_transcription(&self) -> Result<futures::stream::BoxStream<'static, Result<SttResult>>> {
        // For now, return an error indicating this is not implemented
        // In a real implementation, this would set up audio capture and streaming
        Err(SpeechError::Stt(SttError::TranscriptionFailed("Real-time transcription not yet implemented".to_string())))
    }

    /// Synthesize text to speech
    pub async fn synthesize_speech(&self, text: &str) -> Result<TtsResult> {
        self.tts.synthesize(text, &self.config.tts).await
    }

    /// Play audio data (placeholder)
    pub async fn play_audio(&self, _audio_data: &[u8]) -> Result<()> {
        // TODO: Implement audio playback
        log::warn!("🚧 Audio playback not implemented yet");
        Ok(())
    }
} 