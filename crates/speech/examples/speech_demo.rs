use speech::{
    SpeechProcessor, SpeechConfig, SttConfig, TtsConfig, SttProvider, TtsProvider,
    audio::{capture::AudioCapture, AudioProcessor},
    config::{AudioConfig, Voice, VoiceGender, VoiceAge},
};
use futures::StreamExt;
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> speech::Result<()> {
    env_logger::init();

    println!("🎤 Speech Processing Demo");
    println!("========================");

    // Create configuration
    let config = create_demo_config();
    
    // Initialize speech processor
    let mut processor = SpeechProcessor::new(config).await?;
    
    // Demo 1: File-based STT
    println!("\n📁 Demo 1: File-based Speech-to-Text");
    if let Err(e) = demo_file_stt(&processor).await {
        println!("❌ File STT demo failed: {}", e);
    }
    
    // Demo 2: Real-time STT
    println!("\n🎙️ Demo 2: Real-time Speech-to-Text");
    if let Err(e) = demo_realtime_stt(&processor).await {
        println!("❌ Real-time STT demo failed: {}", e);
    }
    
    // Demo 3: Text-to-Speech
    println!("\n🔊 Demo 3: Text-to-Speech");
    if let Err(e) = demo_tts(&processor).await {
        println!("❌ TTS demo failed: {}", e);
    }
    
    // Demo 4: Voice conversation
    println!("\n💬 Demo 4: Voice Conversation");
    if let Err(e) = demo_voice_conversation(&mut processor).await {
        println!("❌ Voice conversation demo failed: {}", e);
    }

    println!("\n✅ All demos completed!");
    Ok(())
}

fn create_demo_config() -> SpeechConfig {
    SpeechConfig {
        audio: AudioConfig {
            sample_rate: 16000,
            channels: 1,
            bits_per_sample: 16,
            buffer_size: 1024,
            voice_activation_threshold: 0.3,
        },
        stt: SttConfig {
            provider: SttProvider::Llama,
            language: "en".to_string(),
            model_path: Some("/Users/vladislavstarshinov/ai/models/my/ggml-large-v3-turbo.bin".into()),
            api_key: None,
            api_url: None,
            chunk_duration_ms: 5000,
            enable_streaming: true,
        },
        tts: TtsConfig {
            provider: TtsProvider::System,
            voice: Some(Voice {
                id: "default".to_string(),
                name: "Default Voice".to_string(),
                language: "en-US".to_string(),
                gender: Some(VoiceGender::Neutral),
                age: Some(VoiceAge::Adult),
                style: None,
            }),
            speed: 1.0,
            pitch: 1.0,
            volume: 0.8,
            api_key: None,
            api_url: None,
            model_path: None,
        },
    }
}

async fn demo_file_stt(processor: &SpeechProcessor) -> speech::Result<()> {
    println!("Loading audio file for transcription...");
    
    // Use the actual voice.wav file in the examples directory
    let audio_file = "voice.wav";
    
    if !Path::new(audio_file).exists() {
        println!("❌ Audio file '{}' not found in examples directory.", audio_file);
        return Ok(());
    }
    
    println!("📁 Found audio file: {}", audio_file);
    
    // Transcribe the audio file
    match processor.transcribe_file(audio_file).await {
        Ok(result) => {
            println!("✅ Transcription successful!");
            println!("📝 Text: {}", result.text);
            println!("🌍 Language: {:?}", result.language);
            println!("📊 Confidence: {:?}", result.confidence);
            println!("⏱️ Duration: {:?}", result.duration);
            
            if !result.segments.is_empty() {
                println!("📋 Segments:");
                for (i, segment) in result.segments.iter().enumerate() {
                    println!("  {}. [{:?} - {:?}] {}", 
                        i + 1, segment.start_time, segment.end_time, segment.text);
                }
            }
        }
        Err(e) => {
            println!("❌ Transcription failed: {}", e);
        }
    }
    
    Ok(())
}

async fn demo_realtime_stt(processor: &SpeechProcessor) -> speech::Result<()> {
    println!("Starting real-time speech recognition...");
    println!("🎤 Speak into your microphone for 10 seconds...");
    
    // Start real-time transcription
    let mut stream = processor.start_realtime_transcription().await?;
    
    // Listen for 10 seconds
    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);
    
    loop {
        tokio::select! {
            result = stream.next() => {
                match result {
                    Some(Ok(transcription)) => {
                        if !transcription.text.trim().is_empty() {
                            println!("🗣️ Heard: {}", transcription.text);
                        }
                    }
                    Some(Err(e)) => {
                        println!("❌ Transcription error: {}", e);
                    }
                    None => break,
                }
            }
            _ = &mut timeout => {
                println!("⏰ Time's up!");
                break;
            }
        }
    }
    
    Ok(())
}

async fn demo_tts(processor: &SpeechProcessor) -> speech::Result<()> {
    let texts = vec![
        "Hello! This is a demonstration of text-to-speech synthesis.",
        "The speech processing system supports multiple providers and voices.",
        "You can use it for creating voice assistants, accessibility tools, and more!",
    ];
    
    for (i, text) in texts.iter().enumerate() {
        println!("🔊 Synthesizing text {}: {}", i + 1, text);
        
        match processor.synthesize_speech(text).await {
            Ok(audio_result) => {
                println!("✅ Synthesis successful!");
                println!("📊 Sample rate: {} Hz", audio_result.sample_rate);
                println!("🎵 Channels: {}", audio_result.channels);
                println!("⏱️ Duration: {:?}", audio_result.duration);
                println!("📦 Audio data size: {} bytes", audio_result.audio_data.len());
                
                // Play the synthesized audio
                println!("🎵 Playing audio...");
                if let Err(e) = processor.play_audio(&audio_result.audio_data).await {
                    println!("❌ Failed to play audio: {}", e);
                } else {
                    println!("✅ Audio playback completed");
                }
                
                // Wait a bit between samples
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => {
                println!("❌ Synthesis failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn demo_voice_conversation(processor: &mut SpeechProcessor) -> speech::Result<()> {
    println!("Starting voice conversation demo...");
    println!("🗣️ Say something, and I'll repeat it back to you!");
    println!("💡 Say 'stop' or 'exit' to end the conversation.");
    
    loop {
        println!("\n🎤 Listening... (speak now)");
        
        // Record audio for 5 seconds
        let audio_data = record_audio_chunk(5).await?;
        
        // Check if there's actual speech
        let audio_processor = AudioProcessor::new(processor.config().audio.clone())?;
        if !audio_processor.detect_voice_activity(&audio_data) {
            println!("🔇 No speech detected, trying again...");
            continue;
        }
        
        // Transcribe the audio
        match processor.transcribe_audio(&audio_data).await {
            Ok(result) => {
                let text = result.text.trim();
                if text.is_empty() {
                    println!("🤔 Couldn't understand that, please try again.");
                    continue;
                }
                
                println!("👂 I heard: {}", text);
                
                // Check for exit commands
                let lower_text = text.to_lowercase();
                if lower_text.contains("stop") || lower_text.contains("exit") || lower_text.contains("quit") {
                    println!("👋 Goodbye!");
                    break;
                }
                
                // Generate a response
                let response = format!("You said: {}", text);
                println!("🤖 Response: {}", response);
                
                // Synthesize and play the response
                match processor.synthesize_speech(&response).await {
                    Ok(audio_result) => {
                        println!("🔊 Playing response...");
                        if let Err(e) = processor.play_audio(&audio_result.audio_data).await {
                            println!("❌ Failed to play response: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to synthesize response: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to transcribe audio: {}", e);
            }
        }
    }
    
    Ok(())
}

// Helper functions

fn create_test_audio_file(filename: &str) -> speech::Result<()> {
    use hound::{WavWriter, WavSpec};
    
    let spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(filename, spec)
        .map_err(|e| speech::SpeechError::Io(std::io::Error::new(
            std::io::ErrorKind::Other, 
            e.to_string()
        )))?;
    
    // Generate a simple sine wave (440 Hz for 2 seconds)
    let sample_rate = 16000;
    let frequency = 440.0;
    let duration = 2.0;
    
    for i in 0..(sample_rate as f32 * duration) as usize {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();
        let amplitude = (sample * i16::MAX as f32 * 0.3) as i16;
        writer.write_sample(amplitude)
            .map_err(|e| speech::SpeechError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                e.to_string()
            )))?;
    }
    
    writer.finalize()
        .map_err(|e| speech::SpeechError::Io(std::io::Error::new(
            std::io::ErrorKind::Other, 
            e.to_string()
        )))?;
    
    println!("✅ Created test audio file: {}", filename);
    Ok(())
}

async fn record_audio_chunk(duration_secs: u64) -> speech::Result<Vec<f32>> {
    use futures::StreamExt;
    
    let config = AudioConfig::default();
    let mut capture = AudioCapture::new(config)?;
    let mut stream = capture.start_capture()?;
    
    let mut audio_data = Vec::new();
    let timeout = tokio::time::sleep(Duration::from_secs(duration_secs));
    tokio::pin!(timeout);
    
    loop {
        tokio::select! {
            chunk = stream.next() => {
                match chunk {
                    Some(Ok(data)) => {
                        audio_data.extend_from_slice(&data);
                    }
                    Some(Err(e)) => {
                        return Err(e);
                    }
                    None => break,
                }
            }
            _ = &mut timeout => {
                break;
            }
        }
    }
    
    Ok(audio_data)
} 