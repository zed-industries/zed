use speech::{
    TtsService, SttConfig, SttProvider,
    config::VoiceGender,
    create_provider,
};
use std::time::Duration;
use std::path::Path;

#[tokio::main]
async fn main() -> speech::Result<()> {
    env_logger::init();

    println!("🎤 Speech Processing Demo");
    println!("========================");

    // Demo 1: Simple TTS test
    println!("\n🔊 Demo 1: Text-to-Speech (Simple)");
    if let Err(e) = demo_simple_tts().await {
        println!("❌ Simple TTS demo failed: {}", e);
    }

    // Demo 2: TTS with different languages and voices
    println!("\n🌍 Demo 2: Multi-language TTS");
    if let Err(e) = demo_multilingual_tts().await {
        println!("❌ Multi-language TTS demo failed: {}", e);
    }

    // Demo 3: TTS with voice configuration  
    println!("\n🎭 Demo 3: Voice Configuration");
    if let Err(e) = demo_voice_configuration().await {
        println!("❌ Voice configuration demo failed: {}", e);
    }

    // Demo 4: STT (Speech-to-Text)
    println!("\n🎙️ Demo 4: Speech-to-Text");
    if let Err(e) = demo_speech_to_text().await {
        println!("❌ STT demo failed: {}", e);
    }

    // Demo 5: Combined STT + TTS (Voice Echo)
    println!("\n🔄 Demo 5: Voice Echo (STT + TTS)");
    if let Err(e) = demo_voice_echo().await {
        println!("❌ Voice echo demo failed: {}", e);
    }

    println!("\n✅ All demos completed!");
    Ok(())
}

async fn demo_simple_tts() -> speech::Result<()> {
    println!("Creating TTS service with system default...");
    
    let tts_service = TtsService::with_system_default().await?;
    
    let texts = [
        "Hello! This is a demonstration of text-to-speech synthesis.",
        "The speech processing system supports multiple providers and voices.",
        "You can use it for creating voice assistants and accessibility tools!",
    ];
    
    for (i, text) in texts.iter().enumerate() {
        println!("🔊 Synthesizing text {}: {}", i + 1, text);
        
        match tts_service.speak_text(text).await {
            Ok(_) => {
                println!("✅ Synthesis and playback successful!");
                
                // Wait a bit between samples
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
            Err(e) => {
                println!("❌ Synthesis failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn demo_multilingual_tts() -> speech::Result<()> {
    let language_tests = [
        ("en-US", "Hello! How are you doing today?"),
        ("es-ES", "¡Hola! ¿Cómo estás hoy?"),
        ("fr-FR", "Bonjour! Comment allez-vous aujourd'hui?"),
        ("de-DE", "Hallo! Wie geht es Ihnen heute?"),
        ("ru-RU", "Привет! Как дела сегодня?"),
    ];
    
    for (language, text) in language_tests {
        println!("🌍 Testing language: {} - {}", language, text);
        
        match TtsService::with_language(language).await {
            Ok(tts_service) => {
                match tts_service.speak_text(text).await {
                    Ok(_) => {
                        println!("✅ {} TTS successful!", language);
                    }
                    Err(e) => {
                        println!("❌ {} TTS failed: {}", language, e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to create TTS service for {}: {}", language, e);
            }
        }
        
        // Wait between languages
        tokio::time::sleep(Duration::from_millis(1500)).await;
    }
    
    Ok(())
}

async fn demo_voice_configuration() -> speech::Result<()> {
    println!("Testing voice configuration changes...");
    
    let tts_service = TtsService::with_system_default().await?;
    
    let voice_tests = [
        ("en-US", VoiceGender::Male, "Hello, I am a male English voice."),
        ("en-US", VoiceGender::Female, "Hello, I am a female English voice."),
        ("ru-RU", VoiceGender::Female, "Привет, я русский женский голос."),
        ("ru-RU", VoiceGender::Male, "Привет, я русский мужской голос."),
        ("es-ES", VoiceGender::Female, "Hola, soy una voz femenina española."),
        ("fr-FR", VoiceGender::Female, "Bonjour, je suis une voix française féminine."),
    ];
    
    for (language, gender, text) in voice_tests {
        println!("🎭 Testing {} {:?} voice: {}", language, gender, text);
        
        // Change voice configuration
        match tts_service.change_voice(language.to_string(), gender.clone()) {
            Ok(_) => {
                println!("✅ Voice changed to {} {:?}", language, gender);
                
                // Speak with the new voice
                match tts_service.speak_text(text).await {
                    Ok(_) => {
                        println!("✅ Speech synthesis successful!");
                    }
                    Err(e) => {
                        println!("❌ Speech synthesis failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to change voice to {} {:?}: {}", language, gender, e);
            }
        }
        
        // Wait between voice changes
        tokio::time::sleep(Duration::from_millis(2000)).await;
    }
    
    Ok(())
}

async fn demo_speech_to_text() -> speech::Result<()> {
    println!("Creating STT provider with Whisper...");
    
    // Create STT configuration
    let stt_config = SttConfig {
        provider: SttProvider::Whisper,
        language: "auto".to_string(),
        model_path: Some("/Users/vladislavstarshinov/ai/models/my/ggml-large-v3-turbo.bin".into()),
        api_key: None,
        api_url: None,
        chunk_duration_ms: 5000,
        enable_streaming: true,
    };
    
    // Create STT provider
    match create_provider(stt_config).await {
        Ok(_stt_provider) => {
            println!("✅ STT provider created successfully");
            
            // Test with a sample audio file if it exists
            let test_audio_file = "test_audio.wav";
            if Path::new(test_audio_file).exists() {
                println!("📁 Found test audio file, transcribing...");
                
                // For this demo, we'll create some dummy audio data
                // In a real implementation, you'd load actual audio
                let dummy_audio: Vec<f32> = vec![0.0; 16000]; // 1 second of silence
                
                match _stt_provider.transcribe_audio(&dummy_audio).await {
                    Ok(result) => {
                        println!("✅ Transcription successful!");
                        println!("📝 Text: {}", result.text);
                        println!("🌍 Language: {:?}", result.language);
                        println!("📊 Confidence: {}", result.confidence);
                    }
                    Err(e) => {
                        println!("❌ Transcription failed: {}", e);
                    }
                }
            } else {
                println!("ℹ️ No test audio file found, skipping file transcription");
            }
        }
        Err(e) => {
            println!("❌ Failed to create STT provider: {}", e);
        }
    }
    
    Ok(())
}

async fn demo_voice_echo() -> speech::Result<()> {
    println!("Creating combined STT + TTS demo...");
    
    // Create TTS service
    let tts_service = TtsService::with_system_default().await?;
    
    // Create STT provider
    let stt_config = SttConfig {
        provider: SttProvider::Whisper,
        language: "auto".to_string(),
        model_path: Some("/Users/vladislavstarshinov/ai/models/my/ggml-large-v3-turbo.bin".into()),
        api_key: None,
        api_url: None,
        chunk_duration_ms: 5000,
        enable_streaming: true,
    };
    
    match create_provider(stt_config).await {
        Ok(_stt_provider) => {
            println!("✅ Both STT and TTS services ready");
            
            // Simulate some voice commands and responses
            let demo_scenarios = [
                ("Hello, how are you?", "I heard you say: Hello, how are you. I'm doing great!"),
                ("What's the weather like?", "I heard your question about the weather. I don't have real weather data in this demo."),
                ("Tell me a joke.", "I heard you ask for a joke. Here's one: Why don't scientists trust atoms? Because they make up everything!"),
            ];
            
            for (input, response) in demo_scenarios {
                println!("\n🎭 Demo scenario:");
                println!("🎤 Simulated input: {}", input);
                
                // In a real implementation, this would be actual audio from microphone
                // For now, we'll just simulate the transcription result
                println!("🔄 Processing speech...");
                
                // Simulate STT processing time
                tokio::time::sleep(Duration::from_millis(500)).await;
                
                println!("✅ Transcribed: {}", input);
                
                // Generate and speak response
                println!("🤖 Response: {}", response);
                
                match tts_service.speak_text(response).await {
                    Ok(_) => {
                        println!("✅ Response spoken successfully");
                    }
                    Err(e) => {
                        println!("❌ Failed to speak response: {}", e);
                    }
                }
                
                // Wait between scenarios
                tokio::time::sleep(Duration::from_millis(2000)).await;
            }
        }
        Err(e) => {
            println!("❌ Failed to create STT provider: {}", e);
            println!("ℹ️ Falling back to TTS-only demo");
            
            // Fallback: just demonstrate TTS
            let demo_texts = [
                "This is a TTS-only demo since STT is not available.",
                "Text-to-speech is working perfectly though!",
                "You can still enjoy the speech synthesis capabilities.",
            ];
            
            for text in demo_texts {
                println!("🔊 Speaking: {}", text);
                if let Err(e) = tts_service.speak_text(text).await {
                    println!("❌ TTS failed: {}", e);
                } else {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }
    
    Ok(())
} 