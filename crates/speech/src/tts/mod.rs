use crate::{config::{TtsConfig, TtsProvider, Voice}, Result, error::TtsError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod system;

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "elevenlabs")]
pub mod elevenlabs;

#[cfg(feature = "piper")]
pub mod piper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResult {
    pub audio_data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration: Duration,
    pub format: AudioFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Ogg,
    Raw,
}

#[async_trait]
pub trait TextToSpeech: Send + Sync {
    /// Synthesize text to speech
    async fn synthesize(&self, text: &str, config: &TtsConfig) -> Result<TtsResult>;

    /// Get available voices
    async fn get_voices(&self) -> Result<Vec<Voice>>;

    /// Check if the provider is available and ready
    async fn is_available(&self) -> bool;

    /// Get supported languages
    async fn get_supported_languages(&self) -> Result<Vec<String>>;

    /// Estimate synthesis duration for given text
    async fn estimate_duration(&self, text: &str, config: &TtsConfig) -> Result<Duration>;
}

/// Create a TTS provider based on configuration
pub async fn create_provider(config: TtsConfig) -> Result<std::sync::Arc<dyn TextToSpeech>> {
    match config.provider {
        TtsProvider::System => {
            let provider = system::SystemTts::new(config).await?;
            Ok(std::sync::Arc::new(provider))
        }
        #[cfg(feature = "openai")]
        TtsProvider::OpenAI => {
            let provider = openai::OpenAITts::new(config).await?;
            Ok(std::sync::Arc::new(provider))
        }
        #[cfg(not(feature = "openai"))]
        TtsProvider::OpenAI => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "OpenAI feature not enabled".to_string(),
            )))
        }
        #[cfg(feature = "elevenlabs")]
        TtsProvider::ElevenLabs => {
            let provider = elevenlabs::ElevenLabsTts::new(config).await?;
            Ok(std::sync::Arc::new(provider))
        }
        #[cfg(not(feature = "elevenlabs"))]
        TtsProvider::ElevenLabs => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "ElevenLabs feature not enabled".to_string(),
            )))
        }
        #[cfg(feature = "piper")]
        TtsProvider::Piper => {
            let provider = piper::PiperTts::new(config).await?;
            Ok(std::sync::Arc::new(provider))
        }
        #[cfg(not(feature = "piper"))]
        TtsProvider::Piper => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "Piper feature not enabled".to_string(),
            )))
        }
        TtsProvider::Custom(name) => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                format!("Custom provider '{}' not implemented", name),
            )))
        }
    }
}

/// Utility functions for TTS processing
pub mod utils {
    use super::*;

    /// Split long text into chunks suitable for TTS processing
    pub fn chunk_text_for_tts(text: &str, max_chars: usize) -> Vec<String> {
        if text.len() <= max_chars {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        // Split by sentences first
        for sentence in text.split_inclusive(&['.', '!', '?']) {
            if current_chunk.len() + sentence.len() <= max_chars {
                current_chunk.push_str(sentence);
            } else {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk.clear();
                }

                // If single sentence is too long, split by words
                if sentence.len() > max_chars {
                    for word in sentence.split_whitespace() {
                        if current_chunk.len() + word.len() + 1 <= max_chars {
                            if !current_chunk.is_empty() {
                                current_chunk.push(' ');
                            }
                            current_chunk.push_str(word);
                        } else {
                            if !current_chunk.is_empty() {
                                chunks.push(current_chunk.trim().to_string());
                                current_chunk.clear();
                            }
                            current_chunk.push_str(word);
                        }
                    }
                } else {
                    current_chunk.push_str(sentence);
                }
            }
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    /// Merge multiple TTS results into a single audio stream
    pub fn merge_tts_results(results: Vec<TtsResult>) -> Result<TtsResult> {
        if results.is_empty() {
            return Err(crate::SpeechError::Tts(TtsError::SynthesisFailed(
                "No results to merge".to_string(),
            )));
        }

        if results.len() == 1 {
            return Ok(results.into_iter().next().unwrap());
        }

        // Store the format info before consuming the vector
        let sample_rate = results[0].sample_rate;
        let channels = results[0].channels;
        let format = results[0].format.clone();

        // Ensure all results have the same format
        for result in &results[1..] {
            if result.sample_rate != sample_rate
                || result.channels != channels
                || std::mem::discriminant(&result.format) != std::mem::discriminant(&format)
            {
                return Err(crate::SpeechError::Tts(TtsError::SynthesisFailed(
                    "Cannot merge TTS results with different formats".to_string(),
                )));
            }
        }

        // Concatenate audio data
        let mut merged_data = Vec::new();
        let mut total_duration = Duration::ZERO;

        for result in results {
            merged_data.extend_from_slice(&result.audio_data);
            total_duration += result.duration;
        }

        Ok(TtsResult {
            audio_data: merged_data,
            sample_rate,
            channels,
            duration: total_duration,
            format,
        })
    }

    /// Convert TTS result to different audio format
    pub fn convert_audio_format(
        result: TtsResult,
        target_format: AudioFormat,
    ) -> Result<TtsResult> {
        if std::mem::discriminant(&result.format) == std::mem::discriminant(&target_format) {
            return Ok(result);
        }

        // For now, only support conversion from raw to WAV
        match (&result.format, &target_format) {
            (AudioFormat::Raw, AudioFormat::Wav) => {
                let wav_data = raw_to_wav(&result.audio_data, result.sample_rate, result.channels)?;
                Ok(TtsResult {
                    audio_data: wav_data,
                    format: AudioFormat::Wav,
                    ..result
                })
            }
            _ => Err(crate::SpeechError::Tts(TtsError::SynthesisFailed(
                format!(
                    "Audio format conversion from {:?} to {:?} not supported",
                    result.format, target_format
                ),
            ))),
        }
    }

    fn raw_to_wav(raw_data: &[u8], sample_rate: u32, channels: u16) -> Result<Vec<u8>> {
        use hound::{WavSpec, WavWriter};
        use std::io::Cursor;

        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut cursor = Cursor::new(Vec::new());
        {
            let mut writer = WavWriter::new(&mut cursor, spec)
                .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(e.to_string())))?;

            // Convert raw bytes to i16 samples and write
            for chunk in raw_data.chunks_exact(2) {
                if let Ok(bytes) = chunk.try_into() {
                    let sample = i16::from_le_bytes(bytes);
                    writer
                        .write_sample(sample)
                        .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(e.to_string())))?;
                }
            }

            writer
                .finalize()
                .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(e.to_string())))?;
        }

        Ok(cursor.into_inner())
    }

    /// Estimate text reading duration (rough approximation)
    pub fn estimate_reading_duration(text: &str, words_per_minute: f32) -> Duration {
        let word_count = text.split_whitespace().count() as f32;
        let minutes = word_count / words_per_minute;
        Duration::from_secs_f32(minutes * 60.0)
    }

    /// Clean text for better TTS synthesis
    pub fn clean_text_for_tts(text: &str) -> String {
        text
            // Remove markdown formatting
            .replace("**", "")
            .replace("*", "")
            .replace("_", "")
            .replace("`", "")
            // Replace common abbreviations
            .replace("e.g.", "for example")
            .replace("i.e.", "that is")
            .replace("etc.", "and so on")
            // Normalize whitespace
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }
} 