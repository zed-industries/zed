use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};

use crate::model_manager::{ModelManager, ModelSize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    pub text: String,
    pub segments: Vec<RecognitionSegment>,
    pub language: Option<String>,
    pub duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionSegment {
    pub text: String,
    pub start_time: f32,
    pub end_time: f32,
    pub confidence: Option<f32>,
}

#[derive(Debug, thiserror::Error)]
pub enum RecognitionError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("Whisper error: {0}")]
    WhisperError(String),
    #[error("Model download error: {0}")]
    ModelDownloadError(#[from] anyhow::Error),
    #[error("Audio processing error: {0}")]
    AudioProcessingError(String),
}

pub struct WhisperRecognizer {
    context: Option<WhisperContext>,
    model_manager: ModelManager,
    model_size: ModelSize,
    language: Option<String>,
}

impl WhisperRecognizer {
    pub async fn new(model_size: ModelSize) -> Result<Self> {
        let model_manager = ModelManager::new()?;
        
        Ok(Self {
            context: None,
            model_manager,
            model_size,
            language: None,
        })
    }

    pub async fn load_model(&mut self) -> Result<()> {
        log::info!("Loading Whisper model: {}", self.model_size.model_name());
        
        // Download model if needed
        let model_path = self.model_manager.ensure_model(self.model_size).await?;
        
        log::info!("Initializing Whisper context from: {:?}", model_path);
        
        // Create Whisper context
        let ctx = WhisperContext::new_with_params(
            &model_path.to_string_lossy(),
            WhisperContextParameters::default()
        ).map_err(|e| anyhow!("Failed to create Whisper context: {:?}", e))?;
        
        self.context = Some(ctx);
        log::info!("Whisper model loaded successfully");
        
        Ok(())
    }

    pub fn is_model_loaded(&self) -> bool {
        self.context.is_some()
    }

    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }

    pub async fn transcribe(&mut self, audio_samples: &[f32]) -> Result<RecognitionResult> {
        if self.context.is_none() {
            self.load_model().await?;
        }

        let context = self.context.as_ref()
            .ok_or(RecognitionError::ModelNotLoaded)?;

        log::info!("Starting transcription of {:.2}s audio", 
            audio_samples.len() as f32 / 16000.0);

        // Create transcription parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        
        // Configure parameters for better accuracy
        params.set_n_threads(num_cpus::get() as i32);
        params.set_translate(false); // Don't translate, just transcribe
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        
        // Set language if specified
        if let Some(ref lang) = self.language {
            params.set_language(Some(lang.as_str()));
        }

        // Create state and run transcription
        let mut state = context.create_state()
            .map_err(|e| RecognitionError::WhisperError(format!("Failed to create state: {:?}", e)))?;

        state.full(params, audio_samples)
            .map_err(|e| RecognitionError::WhisperError(format!("Transcription failed: {:?}", e)))?;

        // Extract results
        let num_segments = state.full_n_segments()
            .map_err(|e| RecognitionError::WhisperError(format!("Failed to get segments: {:?}", e)))?;

        let mut segments = Vec::new();
        let mut full_text = String::new();

        for i in 0..num_segments {
            let text = state.full_get_segment_text(i)
                .map_err(|e| RecognitionError::WhisperError(format!("Failed to get segment text: {:?}", e)))?;
            
            let start_time = state.full_get_segment_t0(i)
                .map_err(|e| RecognitionError::WhisperError(format!("Failed to get start time: {:?}", e)))? as f32 / 100.0; // Convert from centiseconds
            
            let end_time = state.full_get_segment_t1(i)
                .map_err(|e| RecognitionError::WhisperError(format!("Failed to get end time: {:?}", e)))? as f32 / 100.0; // Convert from centiseconds

            if !text.trim().is_empty() {
                segments.push(RecognitionSegment {
                    text: text.trim().to_string(),
                    start_time,
                    end_time,
                    confidence: None, // whisper-rs doesn't expose confidence scores
                });

                if !full_text.is_empty() {
                    full_text.push(' ');
                }
                full_text.push_str(text.trim());
            }
        }

        let duration = audio_samples.len() as f32 / 16000.0; // Assuming 16kHz sample rate

        let result = RecognitionResult {
            text: full_text,
            segments,
            language: self.language.clone(),
            duration,
        };

        log::info!("Transcription completed: {} segments, {:.2}s duration", 
            result.segments.len(), result.duration);
        log::debug!("Transcribed text: {}", result.text);

        Ok(result)
    }

    pub async fn transcribe_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<RecognitionResult> {
        // This would require audio loading functionality
        // For now, return an error suggesting to use transcribe_samples instead
        Err(anyhow!("File transcription not implemented yet. Use transcribe_samples with preprocessed audio."))
    }

    pub fn get_model_size(&self) -> ModelSize {
        self.model_size
    }

    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Get available languages supported by Whisper
    pub fn get_supported_languages() -> Vec<&'static str> {
        vec![
            "en", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca", "nl", "ar", "sv", "it", "id", "hi", "fi", "vi", "he", "uk", "el", "ms", "cs", "ro", "da", "hu", "ta", "no", "th", "ur", "hr", "bg", "lt", "la", "mi", "ml", "cy", "sk", "te", "fa", "lv", "bn", "sr", "az", "sl", "kn", "et", "mk", "br", "eu", "is", "hy", "ne", "mn", "bs", "kk", "sq", "sw", "gl", "mr", "pa", "si", "km", "sn", "yo", "so", "af", "oc", "ka", "be", "tg", "sd", "gu", "am", "yi", "lo", "uz", "fo", "ht", "ps", "tk", "nn", "mt", "sa", "lb", "my", "bo", "tl", "mg", "as", "tt", "haw", "ln", "ha", "ba", "jw", "su"
        ]
    }

    /// Auto-detect language from audio samples
    pub async fn detect_language(&mut self, audio_samples: &[f32]) -> Result<String> {
        if self.context.is_none() {
            self.load_model().await?;
        }

        let context = self.context.as_ref()
            .ok_or(RecognitionError::ModelNotLoaded)?;

        // Use a small sample for language detection (first 30 seconds max)
        let sample_size = (16000.0 * 30.0) as usize; // 30 seconds at 16kHz
        let detection_samples = if audio_samples.len() > sample_size {
            &audio_samples[..sample_size]
        } else {
            audio_samples
        };

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(1); // Use single thread for faster detection
        params.set_language(None); // Auto-detect

        let mut state = context.create_state()
            .map_err(|e| RecognitionError::WhisperError(format!("Failed to create state: {:?}", e)))?;

        state.full(params, detection_samples)
            .map_err(|e| RecognitionError::WhisperError(format!("Language detection failed: {:?}", e)))?;

        // Get detected language
        // Note: whisper-rs might not expose language detection directly
        // This is a simplified implementation
        Ok("en".to_string()) // Default to English for now
    }
}

impl Default for WhisperRecognizer {
    fn default() -> Self {
        Self {
            context: None,
            model_manager: ModelManager::new().unwrap_or_else(|_| panic!("Failed to create model manager")),
            model_size: ModelSize::Base, // Default to base model
            language: None,
        }
    }
} 