use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SpeechConfig {
    pub audio: AudioConfig,
    pub stt: SttConfig,
    pub tts: TtsConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Target sample rate for processing
    pub sample_rate: u32,
    /// Number of audio channels
    pub channels: u16,
    /// Bits per sample
    pub bits_per_sample: u16,
    /// Buffer size for real-time processing
    pub buffer_size: usize,
    /// Voice activation threshold (0.0 to 1.0)
    pub voice_activation_threshold: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000, // Standard for most STT systems
            channels: 1,        // Mono
            bits_per_sample: 16,
            buffer_size: 1024,
            voice_activation_threshold: 0.3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SttConfig {
    pub provider: SttProvider,
    pub language: String,
    pub model_path: Option<PathBuf>,
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub chunk_duration_ms: u64,
    pub enable_streaming: bool,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            provider: SttProvider::Whisper,
            language: "auto".to_string(),
            model_path: None,
            api_key: None,
            api_url: None,
            chunk_duration_ms: 5000, // 5 seconds
            enable_streaming: true,
        }
    }
}

impl SttConfig {
    /// Create a default Whisper configuration with automatic model detection
    pub fn whisper_with_auto_model() -> Self {
        let model_path = Self::find_whisper_model();
        
        Self {
            provider: SttProvider::Whisper,
            language: "auto".to_string(), // Enable automatic language detection
            model_path,
            api_key: None,
            api_url: None,
            chunk_duration_ms: 5000,
            enable_streaming: false,
        }
    }
    
    /// Create a configuration for a specific language
    pub fn whisper_with_language<P: Into<PathBuf>, L: Into<String>>(model_path: P, language: L) -> Self {
        Self {
            provider: SttProvider::Whisper,
            language: language.into(),
            model_path: Some(model_path.into()),
            api_key: None,
            api_url: None,
            chunk_duration_ms: 5000,
            enable_streaming: false,
        }
    }
    
    /// Get the most likely language for transcription based on system locale
    pub fn detect_system_language() -> String {
        // Try to detect language from system locale
        if let Ok(locale) = std::env::var("LANG") {
            // Extract language code from locale (e.g., "en_US.UTF-8" -> "en")
            if let Some(lang) = locale.split('_').next() {
                if lang.len() == 2 {
                    log::info!("🌍 Detected system language from LANG: {}", lang);
                    return lang.to_string();
                }
            }
        }
        
        // Try other locale environment variables
        for env_var in &["LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
            if let Ok(locale) = std::env::var(env_var) {
                if let Some(lang) = locale.split('_').next() {
                    if lang.len() == 2 {
                        log::info!("🌍 Detected system language from {}: {}", env_var, lang);
                        return lang.to_string();
                    }
                }
            }
        }
        
        log::info!("🌍 Could not detect system language, defaulting to English");
        "en".to_string()
    }
    
    /// Check if this configuration uses automatic language detection
    pub fn uses_auto_detection(&self) -> bool {
        self.language == "auto" || self.language == "detect"
    }
    
    /// Update the configuration with a detected language for future use
    pub fn with_detected_language(&self, detected_language: &str) -> Self {
        let mut new_config = self.clone();
        new_config.language = detected_language.to_string();
        log::info!("🎯 Updated STT config with detected language: {}", detected_language);
        new_config
    }
    
    /// Find available Whisper model files in common locations
    fn find_whisper_model() -> Option<PathBuf> {
        let possible_paths = [
            // User's AI models directory
            "/Users/vladislavstarshinov/ai/models/my/ggml-large-v3-turbo.bin",
            // Common Whisper model locations
            "models/ggml-large-v3-turbo.bin",
            "models/ggml-large-v3.bin", 
            "models/ggml-base.en.bin",
            "models/ggml-small.en.bin",
            "models/ggml-tiny.en.bin",
            // Whisper.cpp locations
            "whisper.cpp/models/ggml-large-v3-turbo.bin",
            "whisper.cpp/models/ggml-large-v3.bin",
            "whisper.cpp/models/ggml-base.en.bin",
            "whisper.cpp/models/ggml-small.en.bin",
            "whisper.cpp/models/ggml-tiny.en.bin",
            // Alternative common locations
            "~/.cache/whisper/ggml-base.en.bin",
            "/usr/local/share/whisper/models/ggml-base.en.bin",
        ];
        
        for path_str in &possible_paths {
            let path = if path_str.starts_with('~') {
                // Expand home directory
                if let Ok(home) = std::env::var("HOME") {
                    PathBuf::from(path_str.replace('~', &home))
                } else {
                    continue;
                }
            } else {
                PathBuf::from(path_str)
            };
            
            if path.exists() {
                log::info!("🔍 Found Whisper model at: {}", path.display());
                return Some(path);
            }
        }
        
        log::warn!("⚠️ No Whisper model found in common locations. Please install a Whisper model.");
        log::info!("💡 You can download models from: https://huggingface.co/ggerganov/whisper.cpp");
        None
    }
    
    /// Create a configuration for a specific model file
    pub fn whisper_with_model<P: Into<PathBuf>>(model_path: P) -> Self {
        Self {
            provider: SttProvider::Whisper,
            language: "auto".to_string(), // Enable automatic language detection
            model_path: Some(model_path.into()),
            api_key: None,
            api_url: None,
            chunk_duration_ms: 5000,
            enable_streaming: false,
        }
    }
    
    /// Validate that the configuration is complete and usable
    pub fn is_valid(&self) -> bool {
        match self.provider {
            SttProvider::Whisper => {
                if let Some(ref model_path) = self.model_path {
                    model_path.exists()
                } else {
                    false
                }
            }
            SttProvider::System => true,
            SttProvider::OpenAI => self.api_key.is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtsConfig {
    pub provider: TtsProvider,
    pub voice: Option<Voice>,
    pub speed: f32,
    pub pitch: f32,
    pub volume: f32,
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub model_path: Option<PathBuf>,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            provider: TtsProvider::System,
            voice: None,
            speed: 1.0,
            pitch: 1.0,
            volume: 1.0,
            api_key: None,
            api_url: None,
            model_path: None,
        }
    }
}

impl TtsConfig {
    /// Create TTS config with a specific language
    pub fn with_language<L: Into<String>>(language: L) -> Self {
        let mut config = Self::default();
        let lang = language.into();
        
        // Set a default voice for the language if we have common ones
        config.voice = Self::get_default_voice_for_language(&lang);
        
        config
    }
    
    /// Create TTS config with a specific voice by name and language
    pub fn with_voice<N: Into<String>, L: Into<String>>(voice_name: N, language: L) -> Self {
        let mut config = Self::default();
        let name = voice_name.into();
        config.voice = Some(Voice {
            id: name.clone(),
            name,
            language: language.into(),
            gender: None,
            age: Some(VoiceAge::Adult),
            style: None,
        });
        config
    }
    
    /// Set the language by updating the voice configuration
    pub fn set_language<L: Into<String>>(&mut self, language: L) {
        let lang = language.into();
        
        if let Some(ref mut voice) = self.voice {
            voice.language = lang.clone();
        } else {
            self.voice = Self::get_default_voice_for_language(&lang);
        }
    }
    
    /// Get the current language from the voice configuration
    pub fn get_language(&self) -> Option<String> {
        self.voice.as_ref().map(|v| v.language.clone())
    }
    
    /// Get a list of supported languages
    pub fn supported_languages() -> Vec<&'static str> {
        vec![
            "en", "en-US", "en-GB", "en-AU", "en-CA",
            "es", "es-ES", "es-MX", "es-AR",
            "fr", "fr-FR", "fr-CA",
            "de", "de-DE", "de-AT",
            "it", "it-IT",
            "pt", "pt-PT", "pt-BR",
            "ru", "ru-RU",
            "ja", "ja-JP",
            "zh", "zh-CN", "zh-TW",
            "ko", "ko-KR",
            "ar", "ar-SA",
            "hi", "hi-IN",
            "th", "th-TH",
            "vi", "vi-VN",
            "pl", "pl-PL",
            "nl", "nl-NL",
            "sv", "sv-SE",
            "da", "da-DK",
            "no", "no-NO",
            "fi", "fi-FI",
        ]
    }
    
    /// Get default voice for a language (platform-specific)
    fn get_default_voice_for_language(language: &str) -> Option<Voice> {
        let (voice_name, voice_id, gender) = match language.to_lowercase().as_str() {
            "en" | "en-us" => ("Alex", "Alex", VoiceGender::Male),
            "en-gb" => ("Daniel", "Daniel", VoiceGender::Male),
            "en-au" => ("Karen", "Karen", VoiceGender::Female),
            "es" | "es-es" => ("Jorge", "Jorge", VoiceGender::Male),
            "es-mx" => ("Juan", "Juan", VoiceGender::Male),
            "fr" | "fr-fr" => ("Thomas", "Thomas", VoiceGender::Male),
            "de" | "de-de" => ("Anna", "Anna", VoiceGender::Female),
            "it" | "it-it" => ("Alice", "Alice", VoiceGender::Female),
            "pt" | "pt-pt" => ("Joana", "Joana", VoiceGender::Female),
            "pt-br" => ("Luciana", "Luciana", VoiceGender::Female),
            "ru" | "ru-ru" => ("Milena", "Milena", VoiceGender::Female),
            "ja" | "ja-jp" => ("Kyoko", "Kyoko", VoiceGender::Female),
            "zh" | "zh-cn" => ("Ting-Ting", "Ting-Ting", VoiceGender::Female),
            "ko" | "ko-kr" => ("Yuna", "Yuna", VoiceGender::Female),
            _ => ("Alex", "Alex", VoiceGender::Male), // Default fallback
        };
        
        Some(Voice {
            id: voice_id.to_string(),
            name: voice_name.to_string(),
            language: language.to_string(),
            gender: Some(gender),
            age: Some(VoiceAge::Adult),
            style: None,
        })
    }
    
    /// Create a configuration for a specific language and gender preference
    pub fn with_language_and_gender<L: Into<String>>(language: L, gender: VoiceGender) -> Self {
        let mut config = Self::default();
        let lang = language.into();
        
        if let Some(mut voice) = Self::get_default_voice_for_language(&lang) {
            voice.gender = Some(gender.clone());
            
            // Adjust voice name based on gender preference for some languages
            match (lang.to_lowercase().as_str(), gender) {
                ("en" | "en-us", VoiceGender::Female) => {
                    voice.name = "Samantha".to_string();
                    voice.id = "Samantha".to_string();
                },
                ("en" | "en-us", VoiceGender::Male) => {
                    voice.name = "Alex".to_string();
                    voice.id = "Alex".to_string();
                },
                ("es" | "es-es", VoiceGender::Female) => {
                    voice.name = "Monica".to_string();
                    voice.id = "Monica".to_string();
                },
                ("fr" | "fr-fr", VoiceGender::Female) => {
                    voice.name = "Amélie".to_string();
                    voice.id = "Amélie".to_string();
                },
                ("ru" | "ru-ru", VoiceGender::Female | VoiceGender::Male | VoiceGender::Neutral) => {
                    // Russian only has Milena available on this system
                    voice.name = "Milena".to_string();
                    voice.id = "Milena".to_string();
                    voice.gender = Some(VoiceGender::Female); // Milena is female
                },
                ("de" | "de-de", VoiceGender::Female) => {
                    voice.name = "Anna".to_string();
                    voice.id = "Anna".to_string();
                },
                ("ja" | "ja-jp", VoiceGender::Male) => {
                    voice.name = "Otoya".to_string();
                    voice.id = "Otoya".to_string();
                },
                ("zh" | "zh-cn", VoiceGender::Male) => {
                    voice.name = "Yu-shu".to_string();
                    voice.id = "Yu-shu".to_string();
                },
                _ => {} // Keep default
            }
            
            config.voice = Some(voice);
        }
        
        config
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SttProvider {
    /// OpenAI Whisper API
    OpenAI,
    /// System speech recognition
    System,
    /// Whisper model for speech recognition
    Whisper,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TtsProvider {
    /// System text-to-speech (platform-specific)
    System,
    /// OpenAI TTS API
    OpenAI,
    /// ElevenLabs API
    ElevenLabs,
    /// Local Piper TTS
    Piper,
    /// Custom provider
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    pub id: String,
    pub name: String,
    pub language: String,
    pub gender: Option<VoiceGender>,
    pub age: Option<VoiceAge>,
    pub style: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoiceGender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoiceAge {
    Child,
    Young,
    Adult,
    Senior,
}

impl SpeechConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get default configuration for a specific platform
    pub fn for_platform() -> Self {
        let mut config = Self::default();
        
        #[cfg(target_os = "macos")]
        {
            config.tts.provider = TtsProvider::System;
        }
        
        #[cfg(target_os = "windows")]
        {
            config.tts.provider = TtsProvider::System;
        }
        
        #[cfg(target_os = "linux")]
        {
            config.tts.provider = TtsProvider::System;
        }
        
        config
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Validate audio config
        if self.audio.sample_rate == 0 {
            return Err(crate::SpeechError::Config("Sample rate must be greater than 0".to_string()));
        }
        
        if self.audio.channels == 0 {
            return Err(crate::SpeechError::Config("Channels must be greater than 0".to_string()));
        }
        
        if self.audio.voice_activation_threshold < 0.0 || self.audio.voice_activation_threshold > 1.0 {
            return Err(crate::SpeechError::Config("Voice activation threshold must be between 0.0 and 1.0".to_string()));
        }
        
        // Validate STT config
        if self.stt.chunk_duration_ms == 0 {
            return Err(crate::SpeechError::Config("Chunk duration must be greater than 0".to_string()));
        }
        
        // Validate TTS config
        if self.tts.speed <= 0.0 {
            return Err(crate::SpeechError::Config("TTS speed must be greater than 0".to_string()));
        }
        
        if self.tts.volume < 0.0 || self.tts.volume > 1.0 {
            return Err(crate::SpeechError::Config("TTS volume must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
} 