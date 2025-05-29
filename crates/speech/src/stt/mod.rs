use crate::{config::{SttConfig, SttProvider}, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use futures::stream::BoxStream;

pub mod system;
#[cfg(feature = "whisper-stt")]
pub mod whisper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResult {
    pub text: String,
    pub language: Option<String>,
    pub confidence: f32,
    pub duration: Option<Duration>,
}

#[async_trait]
pub trait SpeechToText: Send + Sync {
    async fn transcribe_audio(&self, audio_samples: &[f32]) -> Result<SttResult>;
    async fn transcribe_stream(&self, audio_stream: BoxStream<'_, Vec<f32>>) -> Result<BoxStream<'_, SttResult>>;
    fn is_available(&self) -> bool;
    fn get_supported_languages(&self) -> Vec<String>;
}

pub async fn create_provider(config: SttConfig) -> Result<Box<dyn SpeechToText>> {
    match config.provider {
        SttProvider::System => {
            let provider = system::SystemStt::new(config).await?;
            Ok(Box::new(provider))
        }
        #[cfg(feature = "whisper-stt")]
        SttProvider::Whisper => {
            let provider = whisper::WhisperStt::new(config).await?;
            Ok(Box::new(provider))
        }
        #[cfg(not(feature = "whisper-stt"))]
        SttProvider::Whisper => {
            log::error!("❌ Whisper STT feature not enabled");
            Err(crate::error::SpeechError::Stt(crate::error::SttError::ModelNotLoaded))
        }
        SttProvider::OpenAI => {
            log::error!("❌ OpenAI STT not yet implemented");
            Err(crate::error::SpeechError::Stt(crate::error::SttError::ModelNotLoaded))
        }
    }
}

/// Utility functions for STT processing
pub mod utils {
    use super::*;

    /// Split long audio into chunks suitable for STT processing
    pub fn chunk_audio_for_stt(
        samples: &[f32],
        sample_rate: u32,
        max_duration_secs: u32,
        overlap_secs: u32,
    ) -> Vec<Vec<f32>> {
        let max_samples = (sample_rate * max_duration_secs) as usize;
        let overlap_samples = (sample_rate * overlap_secs) as usize;
        let step_size = max_samples - overlap_samples;

        if samples.len() <= max_samples {
            return vec![samples.to_vec()];
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < samples.len() {
            let end = (start + max_samples).min(samples.len());
            chunks.push(samples[start..end].to_vec());

            if end == samples.len() {
                break;
            }

            start += step_size;
        }

        chunks
    }

    /// Merge overlapping STT results
    pub fn merge_stt_results(results: Vec<SttResult>) -> SttResult {
        if results.is_empty() {
            return SttResult {
                text: String::new(),
                language: None,
                confidence: 0.0,
                duration: None,
            };
        }

        if results.len() == 1 {
            return results.into_iter().next().unwrap();
        }

        let mut merged_text = String::new();
        let mut total_duration = Duration::ZERO;
        let mut confidence_sum = 0.0;
        let mut confidence_count = 0;
        let language = results[0].language.clone();

        for (i, result) in results.iter().enumerate() {
            if i > 0 && !merged_text.is_empty() && !result.text.trim().is_empty() {
                merged_text.push(' ');
            }
            merged_text.push_str(&result.text);

            total_duration += result.duration.unwrap_or_default();

            if result.confidence > 0.0 {
                confidence_sum += result.confidence;
                confidence_count += 1;
            }
        }

        let average_confidence = if confidence_count > 0 {
            Some(confidence_sum / confidence_count as f32)
        } else {
            None
        };

        SttResult {
            text: merged_text,
            language,
            confidence: average_confidence.unwrap_or(0.0),
            duration: Some(total_duration),
        }
    }
} 