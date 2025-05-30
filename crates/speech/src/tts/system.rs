use crate::{config::{TtsConfig, Voice}, tts::{TextToSpeech, TtsResult, AudioFormat}, Result};
use async_trait::async_trait;
use std::time::Duration;

#[allow(dead_code)]
pub struct SystemTts {
    config: TtsConfig,
}

impl SystemTts {
    pub async fn new(config: TtsConfig) -> Result<Self> {
        Ok(Self { config })
    }
}

#[async_trait]
impl TextToSpeech for SystemTts {
    async fn synthesize(&self, text: &str, config: &TtsConfig) -> Result<TtsResult> {
        #[cfg(target_os = "macos")]
        {
            self.synthesize_macos(text, config).await
        }
        #[cfg(target_os = "windows")]
        {
            self.synthesize_windows(text, config).await
        }
        #[cfg(target_os = "linux")]
        {
            self.synthesize_linux(text, config).await
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            Err(crate::SpeechError::Tts(crate::error::TtsError::ProviderNotAvailable(
                "System TTS not supported on this platform".to_string(),
            )))
        }
    }

    async fn get_voices(&self) -> Result<Vec<Voice>> {
        #[cfg(target_os = "macos")]
        {
            self.get_macos_voices().await
        }
        #[cfg(target_os = "windows")]
        {
            self.get_windows_voices().await
        }
        #[cfg(target_os = "linux")]
        {
            self.get_linux_voices().await
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            Ok(vec![])
        }
    }

    async fn is_available(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            true // AVSpeechSynthesizer is always available on macOS
        }
        #[cfg(target_os = "windows")]
        {
            true // SAPI is always available on Windows
        }
        #[cfg(target_os = "linux")]
        {
            // Check if speech-dispatcher or espeak is available
            std::process::Command::new("espeak")
                .arg("--version")
                .output()
                .is_ok()
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            false
        }
    }

    async fn get_supported_languages(&self) -> Result<Vec<String>> {
        Ok(vec![
            "en".to_string(),
            "en-US".to_string(),
            "en-GB".to_string(),
            "es".to_string(),
            "fr".to_string(),
            "de".to_string(),
            "it".to_string(),
            "pt".to_string(),
            "ru".to_string(),
            "ja".to_string(),
            "zh".to_string(),
        ])
    }

    async fn estimate_duration(&self, text: &str, config: &TtsConfig) -> Result<Duration> {
        // Estimate based on speech rate (words per minute)
        let word_count = text.split_whitespace().count() as f32;
        let base_wpm = 150.0; // Average speaking rate
        let adjusted_wpm = base_wpm * config.speed;
        let minutes = word_count / adjusted_wpm;
        Ok(Duration::from_secs_f32(minutes * 60.0))
    }
}

impl SystemTts {
    #[cfg(target_os = "macos")]
    async fn synthesize_macos(&self, text: &str, config: &TtsConfig) -> Result<TtsResult> {
        use std::process::{Command, Stdio};
        
        let mut cmd = Command::new("say");
        
        // Configure voice if specified
        if let Some(ref voice) = config.voice {
            cmd.arg("-v").arg(&voice.name);
        }
        
        // Configure speaking rate (words per minute)
        let rate = (150.0 * config.speed) as u32;
        cmd.arg("-r").arg(rate.to_string());
        
        // Output to audio file for capture
        let temp_file = format!("/tmp/tts_output_{}.aiff", std::process::id());
        cmd.arg("-o").arg(&temp_file);
        cmd.arg(text);
        
        log::info!("🔊 Synthesizing speech with macOS 'say' command: rate={}", rate);
        
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| crate::SpeechError::Tts(crate::error::TtsError::SynthesisFailed(
                format!("Failed to execute 'say' command: {}", e)
            )))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(crate::SpeechError::Tts(crate::error::TtsError::SynthesisFailed(
                format!("'say' command failed: {}", error_msg)
            )));
        }
        
        // Read the generated audio file
        let audio_data = std::fs::read(&temp_file)
            .map_err(|e| crate::SpeechError::Tts(crate::error::TtsError::SynthesisFailed(
                format!("Failed to read audio file: {}", e)
            )))?;
        
        // Clean up temp file
        let _ = std::fs::remove_file(&temp_file);
        
        // Estimate duration
        let duration = self.estimate_duration(text, config).await?;
        
        Ok(TtsResult {
            audio_data,
            sample_rate: 22050, // Default for macOS 'say'
            channels: 1,
            duration,
            format: AudioFormat::Raw,
        })
    }
    
    #[cfg(target_os = "macos")]
    async fn get_macos_voices(&self) -> Result<Vec<Voice>> {
        use std::process::Command;
        
        let output = Command::new("say")
            .arg("-v")
            .arg("?")
            .output()
            .map_err(|e| crate::SpeechError::Tts(crate::error::TtsError::ProviderNotAvailable(
                format!("Failed to get voices: {}", e)
            )))?;
        
        let voices_output = String::from_utf8_lossy(&output.stdout);
        let mut voices = Vec::new();
        
        for line in voices_output.lines() {
            if let Some((name_part, lang_part)) = line.split_once('#') {
                let name = name_part.trim().to_string();
                let language = lang_part.trim().to_string();
                
                // Determine gender based on common name patterns (heuristic)
                let gender = if ["Alex", "Daniel", "Diego", "Fred", "Jorge", "Juan", "Oliver", "Thomas"].contains(&name.as_str()) {
                    Some(crate::config::VoiceGender::Male)
                } else if ["Alice", "Allison", "Ava", "Fiona", "Joanna", "Kaitlyn", "Kate", "Samantha", "Susan", "Victoria"].contains(&name.as_str()) {
                    Some(crate::config::VoiceGender::Female)
                } else {
                    Some(crate::config::VoiceGender::Neutral)
                };
                
                voices.push(Voice {
                    id: name.clone(),
                    name,
                    language,
                    gender,
                    age: Some(crate::config::VoiceAge::Adult),
                    style: None,
                });
            }
        }
        
        // Add default voices if none found
        if voices.is_empty() {
            voices.push(Voice {
                id: "Alex".to_string(),
                name: "Alex".to_string(),
                language: "en-US".to_string(),
                gender: Some(crate::config::VoiceGender::Male),
                age: Some(crate::config::VoiceAge::Adult),
                style: None,
            });
        }
        
        Ok(voices)
    }
    
    #[cfg(target_os = "windows")]
    async fn synthesize_windows(&self, text: &str, _config: &TtsConfig) -> Result<TtsResult> {
        // Windows SAPI implementation would go here
        // For now, return a stub
        log::warn!("🚧 Windows TTS not fully implemented yet");
        Ok(TtsResult {
            audio_data: vec![0; 1024],
            sample_rate: 22050,
            channels: 1,
            duration: Duration::from_secs(1),
            format: AudioFormat::Raw,
        })
    }
    
    #[cfg(target_os = "windows")]
    async fn get_windows_voices(&self) -> Result<Vec<Voice>> {
        Ok(vec![
            Voice {
                id: "Microsoft David Desktop".to_string(),
                name: "David".to_string(),
                language: "en-US".to_string(),
                gender: Some(crate::config::VoiceGender::Male),
                age: Some(crate::config::VoiceAge::Adult),
                style: None,
            },
        ])
    }
    
    #[cfg(target_os = "linux")]
    async fn synthesize_linux(&self, text: &str, config: &TtsConfig) -> Result<TtsResult> {
        use std::process::{Command, Stdio};
        
        let mut cmd = Command::new("espeak");
        
        // Configure language if specified in voice
        if let Some(ref voice) = config.voice {
            let lang_code = Self::map_language_to_espeak(&voice.language);
            cmd.arg("-v").arg(lang_code);
        }
        
        // Configure speaking rate
        let speed = (175.0 * config.speed) as u32; // espeak default is ~175 wpm
        cmd.arg("-s").arg(speed.to_string());
        
        // Configure pitch
        let pitch = (50.0 + (config.pitch - 1.0) * 50.0) as u32; // 0-100 range
        cmd.arg("-p").arg(pitch.to_string());
        
        // Output to stdout as WAV
        cmd.arg("--stdout");
        cmd.arg(text);
        
        log::info!("🔊 Synthesizing speech with espeak: language={}, speed={}, pitch={}", 
            config.voice.as_ref().map(|v| v.language.as_str()).unwrap_or("en"), 
            speed, 
            pitch
        );
        
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| crate::SpeechError::Tts(crate::error::TtsError::SynthesisFailed(
                format!("Failed to execute espeak: {}", e)
            )))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(crate::SpeechError::Tts(crate::error::TtsError::SynthesisFailed(
                format!("espeak failed: {}", error_msg)
            )));
        }
        
        let duration = self.estimate_duration(text, config).await?;
        
        Ok(TtsResult {
            audio_data: output.stdout,
            sample_rate: 22050,
            channels: 1,
            duration,
            format: AudioFormat::Wav,
        })
    }
    
    #[cfg(target_os = "linux")]
    async fn get_linux_voices(&self) -> Result<Vec<Voice>> {
        use std::process::Command;
        
        // Get available voices from espeak
        let output = Command::new("espeak")
            .arg("--voices")
            .output();
            
        let mut voices = Vec::new();
        
        if let Ok(result) = output {
            let voices_output = String::from_utf8_lossy(&result.stdout);
            
            for line in voices_output.lines().skip(1) { // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let lang_code = parts[1];
                    let voice_name = parts[3];
                    let gender = if voice_name.contains("f") { 
                        Some(crate::config::VoiceGender::Female) 
                    } else { 
                        Some(crate::config::VoiceGender::Male) 
                    };
                    
                    voices.push(Voice {
                        id: format!("{}+{}", lang_code, voice_name),
                        name: voice_name.to_string(),
                        language: Self::map_espeak_to_language(lang_code),
                        gender,
                        age: Some(crate::config::VoiceAge::Adult),
                        style: None,
                    });
                }
            }
        }
        
        // Add default voices if none found
        if voices.is_empty() {
            voices.extend(vec![
                Voice {
                    id: "en".to_string(),
                    name: "Default".to_string(),
                    language: "en".to_string(),
                    gender: Some(crate::config::VoiceGender::Neutral),
                    age: Some(crate::config::VoiceAge::Adult),
                    style: None,
                },
                Voice {
                    id: "es".to_string(),
                    name: "Spanish".to_string(),
                    language: "es".to_string(),
                    gender: Some(crate::config::VoiceGender::Neutral),
                    age: Some(crate::config::VoiceAge::Adult),
                    style: None,
                },
                Voice {
                    id: "fr".to_string(),
                    name: "French".to_string(),
                    language: "fr".to_string(),
                    gender: Some(crate::config::VoiceGender::Neutral),
                    age: Some(crate::config::VoiceAge::Adult),
                    style: None,
                },
                Voice {
                    id: "de".to_string(),
                    name: "German".to_string(),
                    language: "de".to_string(),
                    gender: Some(crate::config::VoiceGender::Neutral),
                    age: Some(crate::config::VoiceAge::Adult),
                    style: None,
                },
            ]);
        }
        
        Ok(voices)
    }

    #[cfg(target_os = "linux")]
    /// Map language codes to espeak voice identifiers
    fn map_language_to_espeak(language: &str) -> String {
        match language.to_lowercase().as_str() {
            "en" | "en-us" => "en-us".to_string(),
            "en-gb" => "en-gb".to_string(),
            "en-au" => "en-au".to_string(),
            "es" | "es-es" => "es".to_string(),
            "es-mx" => "es-mx".to_string(),
            "fr" | "fr-fr" => "fr".to_string(),
            "fr-ca" => "fr-ca".to_string(),
            "de" | "de-de" => "de".to_string(),
            "it" | "it-it" => "it".to_string(),
            "pt" | "pt-pt" => "pt".to_string(),
            "pt-br" => "pt-br".to_string(),
            "ru" | "ru-ru" => "ru".to_string(),
            "ja" | "ja-jp" => "ja".to_string(),
            "zh" | "zh-cn" => "zh".to_string(),
            "ko" | "ko-kr" => "ko".to_string(),
            "ar" | "ar-sa" => "ar".to_string(),
            "hi" | "hi-in" => "hi".to_string(),
            "th" | "th-th" => "th".to_string(),
            "vi" | "vi-vn" => "vi".to_string(),
            "pl" | "pl-pl" => "pl".to_string(),
            "nl" | "nl-nl" => "nl".to_string(),
            "sv" | "sv-se" => "sv".to_string(),
            "da" | "da-dk" => "da".to_string(),
            "no" | "no-no" => "no".to_string(),
            "fi" | "fi-fi" => "fi".to_string(),
            _ => "en".to_string(), // Default fallback
        }
    }

    #[cfg(target_os = "linux")]
    /// Map espeak language codes back to standard language codes
    fn map_espeak_to_language(espeak_lang: &str) -> String {
        match espeak_lang {
            "en-us" => "en-US".to_string(),
            "en-gb" => "en-GB".to_string(),
            "en-au" => "en-AU".to_string(),
            "es-mx" => "es-MX".to_string(),
            "fr-ca" => "fr-CA".to_string(),
            "pt-br" => "pt-BR".to_string(),
            other => other.to_string(),
        }
    }
} 