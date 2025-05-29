use crate::stt::{SpeechToText, SttResult, BoxStream};
use crate::config::SttConfig;
use crate::error::{Result, SpeechError, SttError};
use async_trait::async_trait;
use std::time::Duration;
use std::process::Command;
use std::path::PathBuf;

pub struct WhisperStt {
    model_path: String,
    language: String,
    binary_path: Option<PathBuf>,
}

impl WhisperStt {
    pub async fn new(config: SttConfig) -> Result<Self> {
        let model_path = config.model_path
            .ok_or_else(|| SpeechError::Stt(SttError::ModelNotLoaded))?
            .to_string_lossy()
            .to_string();
        
        log::info!("🎤 Initializing process-based Whisper STT with model: {}", model_path);
        
        // Check if model file exists
        if !std::path::Path::new(&model_path).exists() {
            return Err(SpeechError::Stt(SttError::ModelNotLoaded));
        }

        // Try to find the whisper_transcribe binary
        let binary_path = Self::find_whisper_binary().await?;
        
        log::info!("✅ Process-based Whisper STT initialized successfully");
        
        Ok(Self {
            model_path,
            language: config.language,
            binary_path: Some(binary_path),
        })
    }
    
    async fn find_whisper_binary() -> Result<PathBuf> {
        // Try to find the whisper_transcribe binary in common locations
        let possible_paths = [
            // In the target directory (development)
            "target/debug/whisper_transcribe",
            "target/release/whisper_transcribe",
            // In the same directory as the current executable
            "./whisper_transcribe",
        ];
        
        for path in &possible_paths {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                log::info!("🔍 Found whisper_transcribe binary at: {}", path);
                return Ok(path_buf);
            }
        }
        
        // If not found in common paths, try to find it in PATH
        if let Ok(output) = Command::new("which")
            .arg("whisper_transcribe")
            .output()
        {
            if output.status.success() {
                let path_string = String::from_utf8_lossy(&output.stdout);
                let path_str = path_string.trim();
                if !path_str.is_empty() {
                    log::info!("🔍 Found whisper_transcribe binary in PATH at: {}", path_str);
                    return Ok(PathBuf::from(path_str));
                }
            }
        }
        
        // Try to build the binary if not found
        log::info!("🔨 Building whisper_transcribe binary...");
        let output = Command::new("cargo")
            .args(&["build", "-p", "speech", "--bin", "whisper_transcribe", "--features", "whisper-stt"])
            .output()
            .map_err(|e| {
                log::error!("❌ Failed to run cargo build: {}", e);
                SpeechError::Stt(SttError::TranscriptionFailed("Failed to build whisper_transcribe binary".to_string()))
            })?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("❌ Failed to build whisper_transcribe binary: {}", stderr);
            return Err(SpeechError::Stt(SttError::TranscriptionFailed("Failed to build whisper_transcribe binary".to_string())));
        }
        
        // Check again for the binary after building
        for path in &["target/debug/whisper_transcribe", "target/release/whisper_transcribe"] {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                log::info!("✅ Built whisper_transcribe binary at: {}", path_buf.display());
                return Ok(path_buf);
            }
        }
        
        Err(SpeechError::Stt(SttError::TranscriptionFailed("Could not find or build whisper_transcribe binary".to_string())))
    }
    
    async fn transcribe_with_process(&self, audio_data: &[f32]) -> Result<SttResult> {
        let binary_path = self.binary_path.as_ref()
            .ok_or_else(|| SpeechError::Stt(SttError::ModelNotLoaded))?;
        
        // Serialize audio data to JSON
        let audio_json = serde_json::to_string(audio_data)
            .map_err(|e| {
                log::error!("❌ Failed to serialize audio data: {}", e);
                SpeechError::Stt(SttError::TranscriptionFailed("Failed to serialize audio data".to_string()))
            })?;
        
        log::debug!("🔄 Running whisper_transcribe process with {} bytes of audio data...", audio_json.len());
        
        // Run the whisper_transcribe process, passing audio data via stdin
        let mut child = std::process::Command::new(binary_path)
            .arg(&self.model_path)
            .arg(&self.language)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| {
                log::error!("❌ Failed to spawn whisper_transcribe process: {}", e);
                SpeechError::Stt(SttError::TranscriptionFailed("Failed to spawn whisper_transcribe process".to_string()))
            })?;
        
        // Write audio data to stdin
        if let Some(stdin) = child.stdin.take() {
            use std::io::Write;
            let mut stdin = stdin;
            stdin.write_all(audio_json.as_bytes())
                .map_err(|e| {
                    log::error!("❌ Failed to write audio data to stdin: {}", e);
                    SpeechError::Stt(SttError::TranscriptionFailed("Failed to write audio data to stdin".to_string()))
                })?;
            stdin.flush()
                .map_err(|e| {
                    log::error!("❌ Failed to flush stdin: {}", e);
                    SpeechError::Stt(SttError::TranscriptionFailed("Failed to flush stdin".to_string()))
                })?;
        }
        
        // Wait for the process to complete and get output
        let output = child.wait_with_output()
            .map_err(|e| {
                log::error!("❌ Failed to wait for whisper_transcribe process: {}", e);
                SpeechError::Stt(SttError::TranscriptionFailed("Failed to wait for whisper_transcribe process".to_string()))
            })?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("❌ whisper_transcribe process failed: {}", stderr);
            return Err(SpeechError::Stt(SttError::TranscriptionFailed(format!("Transcription process failed: {}", stderr))));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::debug!("📝 whisper_transcribe output: {}", stdout);
        
        // Parse the JSON result
        let result: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| {
                log::error!("❌ Failed to parse transcription result: {}", e);
                SpeechError::Stt(SttError::TranscriptionFailed("Failed to parse transcription result".to_string()))
            })?;
        
        let text = result["text"].as_str().unwrap_or("").to_string();
        let confidence = result["confidence"].as_f64().unwrap_or(1.0) as f32;
        let language = result["language"].as_str().map(|s| s.to_string());
        let duration_secs = result["duration_secs"].as_f64().unwrap_or(0.0);
        
        log::info!("✅ Transcription completed: \"{}\"", text);
        
        Ok(SttResult {
            text,
            confidence,
            language,
            duration: Some(Duration::from_secs_f64(duration_secs)),
        })
    }
}

#[async_trait]
impl SpeechToText for WhisperStt {
    async fn transcribe_audio(&self, audio_data: &[f32]) -> Result<SttResult> {
        self.transcribe_with_process(audio_data).await
    }
    
    async fn transcribe_stream(&self, _audio_stream: BoxStream<'_, Vec<f32>>) -> Result<BoxStream<'_, SttResult>> {
        // Whisper doesn't support streaming transcription in the same way
        // For now, return an error indicating streaming is not supported
        log::warn!("⚠️ Streaming transcription not yet implemented for Whisper STT");
        Err(SpeechError::Stt(SttError::TranscriptionFailed("Streaming not implemented".to_string())))
    }

    fn is_available(&self) -> bool {
        self.binary_path.is_some()
    }

    fn get_supported_languages(&self) -> Vec<String> {
        // Whisper supports many languages
        vec![
            "en".to_string(), "zh".to_string(), "de".to_string(), "es".to_string(),
            "ru".to_string(), "ko".to_string(), "fr".to_string(), "ja".to_string(),
            "pt".to_string(), "tr".to_string(), "pl".to_string(), "ca".to_string(),
            "nl".to_string(), "ar".to_string(), "sv".to_string(), "it".to_string(),
            "id".to_string(), "hi".to_string(), "fi".to_string(), "vi".to_string(),
            "he".to_string(), "uk".to_string(), "el".to_string(), "ms".to_string(),
            "cs".to_string(), "ro".to_string(), "da".to_string(), "hu".to_string(),
            "ta".to_string(), "no".to_string(), "th".to_string(), "ur".to_string(),
            "hr".to_string(), "bg".to_string(), "lt".to_string(), "la".to_string(),
            "mi".to_string(), "ml".to_string(), "cy".to_string(), "sk".to_string(),
            "te".to_string(), "fa".to_string(), "lv".to_string(), "bn".to_string(),
            "sr".to_string(), "az".to_string(), "sl".to_string(), "kn".to_string(),
            "et".to_string(), "mk".to_string(), "br".to_string(), "eu".to_string(),
            "is".to_string(), "hy".to_string(), "ne".to_string(), "mn".to_string(),
            "bs".to_string(), "kk".to_string(), "sq".to_string(), "sw".to_string(),
            "gl".to_string(), "mr".to_string(), "pa".to_string(), "si".to_string(),
            "km".to_string(), "sn".to_string(), "yo".to_string(), "so".to_string(),
            "af".to_string(), "oc".to_string(), "ka".to_string(), "be".to_string(),
            "tg".to_string(), "sd".to_string(), "gu".to_string(), "am".to_string(),
            "yi".to_string(), "lo".to_string(), "uz".to_string(), "fo".to_string(),
            "ht".to_string(), "ps".to_string(), "tk".to_string(), "nn".to_string(),
            "mt".to_string(), "sa".to_string(), "lb".to_string(), "my".to_string(),
            "bo".to_string(), "tl".to_string(), "mg".to_string(), "as".to_string(),
            "tt".to_string(), "haw".to_string(), "ln".to_string(), "ha".to_string(),
            "ba".to_string(), "jw".to_string(), "su".to_string(),
        ]
    }
} 