# Voice Input/Output Implementation Plan for Zed

## Overview

This document outlines the implementation of voice input and output capabilities for language models in Zed, building on the existing audio infrastructure used for collaboration features.

## Architecture

### 1. Voice Provider System

The voice system is implemented as a wrapper around existing language models, adding speech-to-text (STT) and text-to-speech (TTS) capabilities without modifying the core language model interfaces.

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Voice UI      │    │  Voice Provider  │    │ Language Model  │
│   Controls      │◄──►│     Wrapper      │◄──►│   (Any Model)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │
         │                       ▼
         │              ┌──────────────────┐
         │              │ Voice Processor  │
         │              │  ┌─────────────┐ │
         │              │  │ STT Client  │ │
         │              │  └─────────────┘ │
         │              │  ┌─────────────┐ │
         │              │  │ TTS Client  │ │
         │              │  └─────────────┘ │
         │              └──────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌──────────────────┐
│  Audio Capture  │    │ Audio Playback   │
│   (Existing)    │    │   (Existing)     │
└─────────────────┘    └──────────────────┘
```

### 2. Key Components

#### VoiceLanguageModel
- Wraps any existing language model
- Intercepts text output for TTS
- Provides voice input capabilities
- Maintains compatibility with all existing features

#### VoiceProcessor
- Manages STT and TTS clients
- Handles audio capture and playback
- Coordinates with existing audio infrastructure

#### Voice Settings
- Configurable STT/TTS providers
- Voice activation settings
- Audio quality preferences
- Language and voice selection

## Implementation Phases

### Phase 1: Foundation (Current)
- [x] Basic voice provider structure
- [x] Settings integration
- [x] UI components framework
- [x] Provider registration system

### Phase 2: Audio Integration
- [ ] Integrate with existing `livekit_client` audio capture
- [ ] Implement audio streaming for STT
- [ ] Add audio playback for TTS
- [ ] Voice activation detection

### Phase 3: STT Implementation
- [ ] Whisper.cpp integration (local)
- [ ] OpenAI Whisper API
- [ ] System STT (macOS Speech Recognition)
- [ ] Real-time transcription streaming

### Phase 4: TTS Implementation
- [ ] System TTS (macOS Speech Synthesis)
- [ ] OpenAI TTS API
- [ ] ElevenLabs integration
- [ ] Voice cloning capabilities

### Phase 5: Advanced Features
- [ ] Voice interruption handling
- [ ] Conversation mode (continuous listening)
- [ ] Voice commands and shortcuts
- [ ] Multi-language support

## Technical Details

### Audio Pipeline

```
Microphone → Audio Capture → Voice Activation → STT → Language Model → TTS → Audio Playback → Speakers
     ▲              ▲              ▲           ▲           ▲         ▲           ▲              ▲
     │              │              │           │           │         │           │              │
Existing      Existing        New Voice    New STT    Existing   New TTS   Existing      Existing
Hardware   Infrastructure    Detection   Provider    Model     Provider  Infrastructure  Hardware
```

### STT Providers

#### 1. Whisper.cpp (Local)
```rust
// Add to Cargo.toml
whisper-rs = "0.10"

// Implementation
impl SpeechToTextClient for WhisperClient {
    fn transcribe_stream(&self, audio_stream: BoxStream<'static, Vec<i16>>) 
        -> BoxFuture<'static, Result<BoxStream<'static, Result<String, anyhow::Error>>>> {
        // Real-time transcription using whisper.cpp
        // Buffer audio chunks and process in segments
    }
}
```

#### 2. OpenAI Whisper API
```rust
impl SpeechToTextClient for OpenAISttClient {
    fn transcribe_stream(&self, audio_stream: BoxStream<'static, Vec<i16>>) 
        -> BoxFuture<'static, Result<BoxStream<'static, Result<String, anyhow::Error>>>> {
        // Send audio to OpenAI Whisper API
        // Handle streaming responses
    }
}
```

#### 3. System STT (macOS)
```rust
// Use macOS Speech Recognition framework
impl SpeechToTextClient for SystemSttClient {
    fn transcribe_stream(&self, audio_stream: BoxStream<'static, Vec<i16>>) 
        -> BoxFuture<'static, Result<BoxStream<'static, Result<String, anyhow::Error>>>> {
        // Interface with macOS SFSpeechRecognizer
    }
}
```

### TTS Providers

#### 1. System TTS (macOS)
```rust
// Use macOS AVSpeechSynthesizer
impl TextToSpeechClient for SystemTtsClient {
    fn synthesize(&self, text: &str, voice_id: Option<&str>, speed: f32) 
        -> BoxFuture<'static, Result<Vec<u8>>> {
        // Generate audio using system TTS
    }
}
```

#### 2. OpenAI TTS
```rust
impl TextToSpeechClient for OpenAITtsClient {
    fn synthesize(&self, text: &str, voice_id: Option<&str>, speed: f32) 
        -> BoxFuture<'static, Result<Vec<u8>>> {
        // Call OpenAI TTS API
        // Return audio data for playback
    }
}
```

#### 3. ElevenLabs
```rust
impl TextToSpeechClient for ElevenLabsClient {
    fn synthesize(&self, text: &str, voice_id: Option<&str>, speed: f32) 
        -> BoxFuture<'static, Result<Vec<u8>>> {
        // High-quality voice synthesis
        // Support for voice cloning
    }
}
```

### Audio Integration

#### Leveraging Existing Infrastructure
```rust
// Use existing audio capture from livekit_client
use crate::livekit_client::playback::AudioStack;

impl VoiceProcessor {
    pub async fn start_listening(&mut self) -> Result<BoxStream<'static, Result<String, anyhow::Error>>> {
        // Reuse existing microphone capture
        let (audio_track, audio_stream) = self.audio_stack.capture_local_microphone_track()?;
        
        // Convert audio frames to STT input
        let stt_stream = self.stt_client.transcribe_stream(audio_frames_to_stream(audio_stream));
        
        Ok(stt_stream)
    }
    
    pub async fn speak(&mut self, text: &str) -> Result<()> {
        // Generate audio using TTS
        let audio_data = self.tts_client.synthesize(text, self.voice_id.as_deref(), self.voice_speed).await?;
        
        // Play using existing audio infrastructure
        self.audio_stack.play_audio_data(audio_data)?;
        
        Ok(())
    }
}
```

## User Experience

### Voice Controls in Agent Panel
```
┌─────────────────────────────────────────────────────────────┐
│ Agent Panel                                          [🎤] [⚙️] │
├─────────────────────────────────────────────────────────────┤
│ [🎤 Recording...] [████████░░] Voice Ready                   │
│                                                             │
│ User: How do I implement async functions in Rust?          │
│                                                             │
│ Assistant: [🔊 Speaking...] To implement async functions... │
│                                                             │
│ [Voice Input] [Send] [Stop Speaking]                       │
└─────────────────────────────────────────────────────────────┘
```

### Status Bar Integration
```
┌─────────────────────────────────────────────────────────────┐
│ File Explorer | Editor | Terminal    [🎤 Listening] [Status] │
└─────────────────────────────────────────────────────────────┘
```

### Settings Configuration
```json
{
  "language_models": {
    "voice": {
      "stt_provider": "Whisper",
      "tts_provider": "System",
      "voice_activation_threshold": 0.3,
      "auto_listen": true,
      "voice_speed": 1.0,
      "voice_id": "en-US-Neural2-A",
      "language": "en-US"
    }
  }
}
```

## Dependencies

### Required Crates
```toml
# For local Whisper STT
whisper-rs = "0.10"

# For audio processing
cpal = "0.15" # Already available
rodio = "0.17" # Already available

# For HTTP clients (TTS APIs)
reqwest = { version = "0.11", features = ["json", "stream"] }

# For audio format conversion
hound = "3.5" # WAV file handling
```

### Platform-Specific Dependencies

#### macOS
```toml
# For system STT/TTS
objc = "0.2"
cocoa = "0.24"
core-foundation = "0.9"
```

#### Windows
```toml
# For Windows Speech Platform
windows = { version = "0.48", features = ["Win32_Media_Speech"] }
```

#### Linux
```toml
# For espeak/festival TTS
libpulse-binding = "2.27"
```

## Security Considerations

### Privacy
- Local STT (Whisper) for sensitive conversations
- Option to disable cloud-based services
- Audio data encryption for API calls
- Clear indication when audio is being processed

### Permissions
- Microphone access permissions
- User consent for cloud STT/TTS services
- Ability to revoke permissions

## Testing Strategy

### Unit Tests
- STT/TTS client implementations
- Audio format conversions
- Voice activation detection

### Integration Tests
- End-to-end voice conversations
- Multiple provider switching
- Error handling and recovery

### Performance Tests
- Latency measurements (STT/TTS)
- Memory usage during long conversations
- Audio quality assessments

## Future Enhancements

### Advanced Features
1. **Voice Commands**: "Zed, open file X", "Zed, run tests"
2. **Conversation Mode**: Continuous listening with wake words
3. **Voice Profiles**: Multiple user voice recognition
4. **Real-time Translation**: Multi-language conversations
5. **Voice Emotions**: Emotional context in TTS
6. **Background Noise Filtering**: Advanced audio processing

### Integration Opportunities
1. **Code Reading**: TTS for code review
2. **Documentation**: Voice-generated comments
3. **Accessibility**: Full voice-driven IDE experience
4. **Pair Programming**: Voice-enhanced collaboration

## Migration Path

### Existing Users
- Voice features are opt-in
- No changes to existing workflows
- Gradual feature rollout
- Comprehensive documentation

### Model Compatibility
- All existing language models supported
- No breaking changes to model interfaces
- Voice wrapper is transparent
- Fallback to text-only mode

## Conclusion

This implementation provides a robust foundation for voice interaction with language models while leveraging Zed's existing audio infrastructure. The modular design allows for incremental development and easy extension with new STT/TTS providers.

The voice system enhances the user experience without disrupting existing workflows, making AI assistance more natural and accessible. 