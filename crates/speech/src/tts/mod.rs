use crate::{config::{TtsConfig, TtsProvider, Voice}, Result, error::TtsError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod system;

// Note: Additional TTS providers can be added as optional modules:
// #[cfg(feature = "openai")]
// pub mod openai;
// #[cfg(feature = "elevenlabs")]  
// pub mod elevenlabs;
// #[cfg(feature = "piper")]
// pub mod piper;

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
        TtsProvider::OpenAI => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "OpenAI TTS provider not implemented".to_string(),
            )))
        }
        TtsProvider::ElevenLabs => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "ElevenLabs TTS provider not implemented".to_string(),
            )))
        }
        TtsProvider::Piper => {
            Err(crate::SpeechError::Tts(TtsError::ProviderNotAvailable(
                "Piper TTS provider not implemented".to_string(),
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
                        if current_chunk.len() + word.len() < max_chars {
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

        let converted_data = match (&result.format, &target_format) {
            (AudioFormat::Raw, AudioFormat::Wav) => {
                raw_to_wav(&result.audio_data, result.sample_rate, result.channels)?
            }
            _ => {
                return Err(crate::SpeechError::Tts(TtsError::SynthesisFailed(
                    format!("Conversion from {:?} to {:?} not supported", result.format, target_format),
                )));
            }
        };

        Ok(TtsResult {
            audio_data: converted_data,
            sample_rate: result.sample_rate,
            channels: result.channels,
            duration: result.duration,
            format: target_format,
        })
    }

    fn raw_to_wav(raw_data: &[u8], sample_rate: u32, channels: u16) -> Result<Vec<u8>> {
        let mut wav_data = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut wav_data);
            let spec = hound::WavSpec {
                channels,
                sample_rate,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::new(&mut cursor, spec)
                .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                    format!("Failed to create WAV writer: {}", e)
                )))?;

            // Convert bytes to i16 samples
            for chunk in raw_data.chunks_exact(2) {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                writer.write_sample(sample)
                    .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                        format!("Failed to write WAV sample: {}", e)
                    )))?;
            }

            writer.finalize()
                .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                    format!("Failed to finalize WAV: {}", e)
                )))?;
        }
        Ok(wav_data)
    }

    /// Estimate reading duration based on text length and words per minute
    pub fn estimate_reading_duration(text: &str, words_per_minute: f32) -> Duration {
        let word_count = text.split_whitespace().count() as f32;
        let minutes = word_count / words_per_minute;
        Duration::from_secs_f32(minutes * 60.0)
    }

    /// Clean text for TTS processing
    pub fn clean_text_for_tts(text: &str) -> String {
        // Remove markdown formatting and other text that doesn't read well
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .replace("```", "")
            .replace("**", "")
            .replace("*", "")
    }
}

/// High-level TTS service that manages synthesis and playback
pub struct TtsService {
    tx: std::sync::mpsc::Sender<TtsCommand>,
    abort_tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::Sender<()>>>>,
}

enum TtsCommand {
    Speak { 
        text: String, 
        response_tx: std::sync::mpsc::Sender<Result<()>>,
    },
    Stop {
        response_tx: std::sync::mpsc::Sender<Result<()>>,
    },
    IsAvailable {
        response_tx: std::sync::mpsc::Sender<bool>,
    },
    GetVoices {
        response_tx: std::sync::mpsc::Sender<Result<Vec<crate::config::Voice>>>,
    },
    GetLanguages {
        response_tx: std::sync::mpsc::Sender<Result<Vec<String>>>,
    },
    EstimateDuration {
        text: String,
        response_tx: std::sync::mpsc::Sender<Result<Duration>>,
    },
    ChangeLanguage {
        language: String,
        response_tx: std::sync::mpsc::Sender<Result<()>>,
    },
    ChangeVoice {
        language: String,
        gender: crate::config::VoiceGender,
        response_tx: std::sync::mpsc::Sender<Result<()>>,
    },
}

impl Clone for TtsService {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            abort_tx: self.abort_tx.clone(),
        }
    }
}

impl TtsService {
    /// Create a new TTS service with the specified configuration
    pub async fn new(config: TtsConfig) -> Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel::<TtsCommand>();
        let (abort_tx, abort_rx) = std::sync::mpsc::channel::<()>();
        let abort_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(abort_tx)));
        
        // Create the background thread with its own Tokio runtime
        let _abort_tx_clone = abort_tx.clone();
        let config_clone = config.clone();
        std::thread::spawn(move || {
            // Create Tokio runtime for this background thread
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    log::error!("Failed to create Tokio runtime for TTS service: {}", e);
                    return;
                }
            };
            
            rt.block_on(async move {
                let provider = match create_provider(config_clone.clone()).await {
                    Ok(provider) => provider,
                    Err(e) => {
                        log::error!("Failed to create TTS provider: {}", e);
                        return;
                    }
                };
                
                let mut current_abort_handle: Option<futures::future::AbortHandle> = None;
                let mut current_config = config_clone.clone();
                
                loop {
                    // Check for abort signal first
                    if let Ok(()) = abort_rx.try_recv() {
                        if let Some(handle) = current_abort_handle.take() {
                            handle.abort();
                        }
                        continue;
                    }
                    
                    // Process TTS commands
                    match rx.try_recv() {
                        Ok(command) => {
                            match command {
                                TtsCommand::Speak { text, response_tx } => {
                                    // Cancel any ongoing speech
                                    if let Some(handle) = current_abort_handle.take() {
                                        handle.abort();
                                    }
                                    
                                    let result = Self::handle_speak_command(
                                        provider.clone(), 
                                        &current_config, 
                                        &text,
                                        &mut current_abort_handle
                                    ).await;
                                    
                                    let _ = response_tx.send(result);
                                }
                                TtsCommand::Stop { response_tx } => {
                                    if let Some(handle) = current_abort_handle.take() {
                                        handle.abort();
                                    }
                                    
                                    // Kill platform-specific audio processes
                                    #[cfg(target_os = "macos")]
                                    {
                                        let _ = std::process::Command::new("pkill")
                                            .arg("afplay")
                                            .output();
                                    }
                                    
                                    let _ = response_tx.send(Ok(()));
                                }
                                TtsCommand::IsAvailable { response_tx } => {
                                    let available = provider.is_available().await;
                                    let _ = response_tx.send(available);
                                }
                                TtsCommand::GetVoices { response_tx } => {
                                    let result = provider.get_voices().await;
                                    let _ = response_tx.send(result);
                                }
                                TtsCommand::GetLanguages { response_tx } => {
                                    let result = provider.get_supported_languages().await;
                                    let _ = response_tx.send(result);
                                }
                                TtsCommand::EstimateDuration { text, response_tx } => {
                                    let cleaned_text = utils::clean_text_for_tts(&text);
                                    let result = provider.estimate_duration(&cleaned_text, &current_config).await;
                                    let _ = response_tx.send(result);
                                }
                                TtsCommand::ChangeLanguage { language, response_tx } => {
                                    current_config.set_language(&language);
                                    log::info!("🌍 TTS language changed to: {}", language);
                                    let _ = response_tx.send(Ok(()));
                                }
                                TtsCommand::ChangeVoice { language, gender, response_tx } => {
                                    let gender_for_log = gender.clone();
                                    let new_config = crate::config::TtsConfig::with_language_and_gender(&language, gender);
                                    if let Some(voice) = new_config.voice {
                                        log::info!("🎭 TTS voice configuration changed: {} {:?} -> Voice: {} ({})", 
                                            language, gender_for_log, voice.name, voice.id);
                                        current_config.voice = Some(voice);
                                    } else {
                                        log::warn!("⚠️ No voice found for language {} with gender {:?}", language, gender_for_log);
                                    }
                                    let _ = response_tx.send(Ok(()));
                                }
                            }
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {
                            // No commands, sleep briefly
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            // Channel disconnected, exit
                            break;
                        }
                    }
                }
            });
        });
        
        Ok(Self { tx, abort_tx })
    }

    /// Create a TTS service with system default configuration
    pub async fn with_system_default() -> Result<Self> {
        Self::new(TtsConfig::default()).await
    }

    /// Create a TTS service with a specific language
    pub async fn with_language<L: Into<String>>(language: L) -> Result<Self> {
        let config = TtsConfig::with_language(language);
        Self::new(config).await
    }

    /// Create a TTS service with a specific language and gender preference
    pub async fn with_language_and_gender<L: Into<String>>(language: L, gender: crate::config::VoiceGender) -> Result<Self> {
        let config = TtsConfig::with_language_and_gender(language, gender);
        Self::new(config).await
    }

    /// Create a TTS service with a specific voice
    pub async fn with_voice<N: Into<String>, L: Into<String>>(voice_name: N, language: L) -> Result<Self> {
        let config = TtsConfig::with_voice(voice_name, language);
        Self::new(config).await
    }

    /// Speak the given text (non-blocking, runs in background thread)
    pub async fn speak_text(&self, text: &str) -> Result<()> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::Speak {
            text: text.to_string(),
            response_tx,
        }).map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service not available".to_string()
        )))?;
        
        // Wait for response from background thread
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    /// Stop any currently playing TTS audio
    pub async fn stop_speaking(&self) -> Result<()> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::Stop { response_tx })
            .map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                "TTS service not available".to_string()
            )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    /// Check if TTS is currently available
    pub async fn is_available(&self) -> bool {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        if self.tx.send(TtsCommand::IsAvailable { response_tx }).is_err() {
            return false;
        }
        
        response_rx.recv().unwrap_or(false)
    }

    /// Get available voices
    pub async fn get_voices(&self) -> Result<Vec<crate::config::Voice>> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::GetVoices { response_tx })
            .map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                "TTS service not available".to_string()
            )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    /// Get supported languages
    pub async fn get_supported_languages(&self) -> Result<Vec<String>> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::GetLanguages { response_tx })
            .map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                "TTS service not available".to_string()
            )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    /// Estimate how long it would take to speak the given text
    pub async fn estimate_duration(&self, text: &str) -> Result<Duration> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::EstimateDuration {
            text: text.to_string(),
            response_tx,
        }).map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service not available".to_string()
        )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    // Background thread implementation for speech synthesis and playback
    async fn handle_speak_command(
        provider: std::sync::Arc<dyn TextToSpeech>,
        config: &TtsConfig,
        text: &str,
        current_abort_handle: &mut Option<futures::future::AbortHandle>,
    ) -> Result<()> {
        if text.trim().is_empty() {
            return Ok(());
        }

        // Clean the text for better TTS processing
        let cleaned_text = utils::clean_text_for_tts(text);
        
        // Log the voice configuration being used
        if let Some(ref voice) = config.voice {
            log::info!("🔊 Starting TTS synthesis with voice: {} ({}) for language: {}", 
                voice.name, voice.id, voice.language);
        } else {
            log::info!("🔊 Starting TTS synthesis with default system voice");
        }
        
        log::info!("🎤 Speaking text: {}", 
            if cleaned_text.chars().count() > 50 { 
                format!("{}...", cleaned_text.chars().take(50).collect::<String>()) 
            } else { 
                cleaned_text.clone() 
            }
        );

        // Synthesize the text
        let tts_result = provider.synthesize(&cleaned_text, config).await?;
        log::info!("🎵 TTS synthesis completed, duration: {:?}", tts_result.duration);

        // Play the audio using platform-specific methods
        Self::play_audio_with_abort(tts_result, current_abort_handle).await?;
        
        Ok(())
    }

    /// Play synthesized audio using platform-specific methods with abort capability
    async fn play_audio_with_abort(
        tts_result: TtsResult, 
        current_abort_handle: &mut Option<futures::future::AbortHandle>
    ) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            Self::play_audio_macos_with_abort(tts_result, current_abort_handle).await
        }
        #[cfg(target_os = "windows")]
        {
            Self::play_audio_windows_with_abort(tts_result, current_abort_handle).await
        }
        #[cfg(target_os = "linux")]
        {
            Self::play_audio_linux_with_abort(tts_result, current_abort_handle).await
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            log::warn!("🚧 TTS audio playback not implemented for this platform");
            Ok(())
        }
    }

    #[cfg(target_os = "macos")]
    async fn play_audio_macos_with_abort(
        tts_result: TtsResult,
        current_abort_handle: &mut Option<futures::future::AbortHandle>
    ) -> Result<()> {
        let temp_file = format!("/tmp/tts_playback_{}.aiff", std::process::id());
        
        // Write audio data to temporary file
        tokio::fs::write(&temp_file, &tts_result.audio_data).await
            .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                format!("Failed to write audio file: {}", e)
            )))?;

        log::info!("🎵 Playing TTS audio file: {}", temp_file);

        // Create abortable task for playback
        let (abort_handle, abort_registration) = futures::future::AbortHandle::new_pair();
        *current_abort_handle = Some(abort_handle);

        let temp_file_clone = temp_file.clone();
        let playback_future = futures::future::Abortable::new(
            async move {
                // Play the audio file using afplay (macOS)
                let output = tokio::process::Command::new("afplay")
                    .arg(&temp_file_clone)
                    .output()
                    .await;

                match output {
                    Ok(result) => {
                        if result.status.success() {
                            log::info!("✅ TTS playback completed successfully");
                        } else {
                            let error_msg = String::from_utf8_lossy(&result.stderr);
                            log::error!("❌ TTS playback failed: {}", error_msg);
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Failed to execute afplay: {}", e);
                    }
                }

                // Clean up temp file
                let _ = tokio::fs::remove_file(&temp_file_clone).await;
            },
            abort_registration,
        );

        match playback_future.await {
            Ok(_) => {
                *current_abort_handle = None;
                Ok(())
            }
            Err(futures::future::Aborted) => {
                // Playback was cancelled
                let _ = tokio::fs::remove_file(&temp_file).await;
                *current_abort_handle = None;
                Ok(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    async fn play_audio_windows_with_abort(
        _tts_result: TtsResult,
        _current_abort_handle: &mut Option<futures::future::AbortHandle>
    ) -> Result<()> {
        log::warn!("🚧 Windows TTS audio playback not fully implemented yet");
        Ok(())
    }

    #[cfg(target_os = "linux")]
    async fn play_audio_linux_with_abort(
        tts_result: TtsResult,
        current_abort_handle: &mut Option<futures::future::AbortHandle>
    ) -> Result<()> {
        // Create abortable task for playback
        let (abort_handle, abort_registration) = futures::future::AbortHandle::new_pair();
        *current_abort_handle = Some(abort_handle);

        let playback_future = futures::future::Abortable::new(
            async move {
                // Play using aplay on Linux
                let mut child = tokio::process::Command::new("aplay")
                    .arg("-f")
                    .arg("S16_LE")
                    .arg("-r")
                    .arg(tts_result.sample_rate.to_string())
                    .arg("-c")
                    .arg(tts_result.channels.to_string())
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .map_err(|e| crate::SpeechError::Tts(TtsError::SynthesisFailed(
                        format!("Failed to spawn aplay: {}", e)
                    )))?;

                if let Some(stdin) = child.stdin.as_mut() {
                    use tokio::io::AsyncWriteExt;
                    let _ = stdin.write_all(&tts_result.audio_data).await;
                    let _ = stdin.flush().await;
                }

                let _ = child.wait().await;
                log::info!("✅ TTS playback completed");
                Ok::<(), crate::SpeechError>(())
            },
            abort_registration,
        );

        match playback_future.await {
            Ok(_) => {
                *current_abort_handle = None;
                Ok(())
            }
            Err(futures::future::Aborted) => {
                *current_abort_handle = None;
                Ok(())
            }
        }
    }

    /// Change the TTS language (synchronous)
    pub fn change_language(&self, language: String) -> Result<()> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::ChangeLanguage { 
            language: language.clone(), 
            response_tx 
        }).map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service not available".to_string()
        )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }

    /// Change the TTS voice (synchronous) 
    pub fn change_voice(&self, language: String, gender: crate::config::VoiceGender) -> Result<()> {
        let (response_tx, response_rx) = std::sync::mpsc::channel();
        
        self.tx.send(TtsCommand::ChangeVoice { 
            language: language.clone(),
            gender,
            response_tx 
        }).map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service not available".to_string()
        )))?;
        
        response_rx.recv().map_err(|_| crate::SpeechError::Tts(TtsError::SynthesisFailed(
            "TTS service communication failed".to_string()
        )))?
    }
} 