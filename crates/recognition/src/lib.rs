mod whisper;
mod audio_processor;
mod model_manager;

pub use whisper::{WhisperRecognizer, RecognitionResult, RecognitionError};
pub use audio_processor::{AudioProcessor, AudioFormat};
pub use model_manager::{ModelManager, ModelSize};

use anyhow::Result;
use std::path::Path;

/// Main interface for voice recognition
pub struct VoiceRecognizer {
    whisper: WhisperRecognizer,
    audio_processor: AudioProcessor,
}

impl VoiceRecognizer {
    /// Create a new voice recognizer with the specified model size
    pub async fn new(model_size: ModelSize) -> Result<Self> {
        let whisper = WhisperRecognizer::new(model_size).await?;
        let audio_processor = AudioProcessor::new();
        
        Ok(Self {
            whisper,
            audio_processor,
        })
    }

    /// Transcribe audio data from raw samples
    pub async fn transcribe_samples(
        &mut self,
        samples: &[f32],
        sample_rate: u32,
        channels: u16,
    ) -> Result<RecognitionResult> {
        // Process audio to the format expected by Whisper
        let processed_audio = self.audio_processor.process_for_whisper(samples, sample_rate, channels)?;
        
        // Perform transcription
        self.whisper.transcribe(&processed_audio).await
    }

    /// Transcribe audio from a file
    pub async fn transcribe_file<P: AsRef<Path>>(&mut self, path: P) -> Result<RecognitionResult> {
        // Load and process audio file
        let (samples, sample_rate, channels) = self.audio_processor.load_audio_file(path)?;
        
        // Transcribe the processed audio
        self.transcribe_samples(&samples, sample_rate, channels).await
    }

    /// Check if the recognizer is ready to use
    pub fn is_ready(&self) -> bool {
        self.whisper.is_model_loaded()
    }
} 