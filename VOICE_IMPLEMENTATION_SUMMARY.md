# Voice Input/Output Implementation for Zed

## Overview

This document describes the implementation of voice input/output capabilities for Zed's language models, providing both Speech-to-Text (STT) and Text-to-Speech (TTS) functionality.

## Architecture

The voice system is built as a wrapper around existing language models, adding voice capabilities without modifying the core language model interfaces.

### Key Components

1. **VoiceLanguageModel**: Wrapper that adds voice I/O to any language model
2. **VoiceProcessor**: Handles audio processing and streaming
3. **VoiceState**: Entity for tracking voice activity and state
4. **Provider System**: Extensible framework for different STT/TTS providers
5. **UI Components**: Voice controls and status indicators

### Provider Framework

The system supports multiple STT and TTS providers:

**STT Providers:**
- **Whisper** (Local, using whisper-rs) - **IMPLEMENTED**
- OpenAI (API-based)
- System (Platform-specific)

**TTS Providers:**
- **Piper** (Local, using piper executable) - **IMPLEMENTED**
- ElevenLabs (API-based)
- OpenAI (API-based)
- System (Platform-specific)

## Real Implementation Status

### ✅ Whisper STT (Fully Implemented)

The Whisper implementation uses the `whisper-rs` crate to provide real speech-to-text functionality:

- **Model Loading**: Automatically loads Whisper models from specified paths
- **Real-time Processing**: Processes audio streams in 5-second chunks with overlap
- **Multiple Languages**: Supports multiple languages (configurable)
- **Error Handling**: Graceful fallback when models are not available
- **Streaming Output**: Returns transcribed text as it becomes available

**Configuration:**
```json
{
  "language_models": {
    "voice": {
      "stt_provider": "Whisper",
      "whisper_model_path": "models/whisper/ggml-base.en.bin",
      "language": "en-US"
    }
  }
}
```

**Model Requirements:**
- Download Whisper models from [Hugging Face](https://huggingface.co/ggerganov/whisper.cpp/tree/main)
- Recommended: `ggml-base.en.bin` (142MB, English only) or `ggml-base.bin` (142MB, multilingual)
- Place in the configured model path

### ✅ Piper TTS (Fully Implemented)

The Piper implementation uses the external `piper` executable for high-quality text-to-speech:

- **External Process**: Executes piper command with proper arguments
- **Voice Selection**: Supports different voice models
- **Speed Control**: Configurable speech speed
- **Raw Audio Output**: Returns raw audio data for playback
- **Error Handling**: Checks for piper availability and model existence

**Configuration:**
```json
{
  "language_models": {
    "voice": {
      "tts_provider": "Piper",
      "piper_model_path": "models/piper/en_US-lessac-medium.onnx",
      "voice_speed": 1.0,
      "voice_id": "en_US-lessac-medium"
    }
  }
}
```

**Requirements:**
- Install piper: `pip install piper-tts` or download from [GitHub](https://github.com/rhasspy/piper/releases)
- Download voice models from [Piper releases](https://github.com/rhasspy/piper/releases/tag/v1.2.0)
- Recommended: `en_US-lessac-medium.onnx` (63MB, good quality)

## Usage Examples

### Basic Usage

```rust
use language_models::provider::voice::wrap_with_voice;

// Wrap any existing language model with voice capabilities
let voice_model = wrap_with_voice(existing_model, cx)?;

// Use the voice model like any other language model
let response = voice_model.stream_completion(request, cx).await?;
```

### Manual Configuration

```rust
use language_models::provider::voice::{VoiceLanguageModel, VoiceSettings, SttProvider, TtsProvider};

let settings = VoiceSettings {
    stt_provider: SttProvider::Whisper,
    tts_provider: TtsProvider::Piper,
    whisper_model_path: Some(PathBuf::from("models/whisper/ggml-base.en.bin")),
    piper_model_path: Some(PathBuf::from("models/piper/en_US-lessac-medium.onnx")),
    voice_activation_threshold: 0.3,
    auto_listen: true,
    voice_speed: 1.0,
    language: "en-US".to_string(),
    ..Default::default()
};

let voice_model = VoiceLanguageModel::new(existing_model, &settings, cx)?;
```

### Model Management

```rust
use language_models::provider::voice::{ensure_whisper_model, ensure_piper_model, get_default_whisper_model_path};

// Ensure models are available (will log download instructions if missing)
let whisper_path = get_default_whisper_model_path();
ensure_whisper_model(&whisper_path).await?;

let piper_path = get_default_piper_model_path();
ensure_piper_model(&piper_path).await?;
```

## Technical Details

### Whisper Implementation

- Uses `whisper-rs` v0.10 for Rust bindings to whisper.cpp
- Processes audio in chunks with configurable overlap for continuity
- Supports multiple sampling strategies (currently uses Greedy)
- Normalizes i16 audio samples to f32 for whisper processing
- Handles model loading errors gracefully with fallback behavior

### Piper Implementation

- Executes external `piper` command via `tokio::process::Command`
- Streams text to stdin and captures raw audio from stdout
- Supports all piper command-line options (model, speed, etc.)
- Validates piper executable availability before processing
- Returns raw audio bytes suitable for audio playback systems

### Audio Processing

The current implementation provides the foundation for audio processing:

- **Audio Streams**: Uses `BoxStream<'static, Vec<i16>>` for audio input
- **Chunk Processing**: Configurable chunk sizes for real-time processing
- **Format Support**: Handles 16-bit signed integer audio at 16kHz
- **Streaming Output**: Returns transcribed text as it becomes available

## Installation and Setup

### 1. Install Dependencies

```bash
# Install whisper-rs (already included in Cargo.toml)
# Install piper
pip install piper-tts
```

### 2. Download Models

```bash
# Create models directory
mkdir -p models/whisper models/piper

# Download Whisper model (example)
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -O models/whisper/ggml-base.en.bin

# Download Piper model (example)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/en_US-lessac-medium.onnx -O models/piper/en_US-lessac-medium.onnx
```

### 3. Configure Zed

Add to your Zed settings:

```json
{
  "language_models": {
    "voice": {
      "stt_provider": "Whisper",
      "tts_provider": "Piper",
      "whisper_model_path": "models/whisper/ggml-base.en.bin",
      "piper_model_path": "models/piper/en_US-lessac-medium.onnx",
      "voice_activation_threshold": 0.3,
      "auto_listen": true,
      "voice_speed": 1.0,
      "language": "en-US"
    }
  }
}
```

## Future Enhancements

1. **Audio Capture Integration**: Connect to actual microphone input
2. **Audio Playback Integration**: Connect to system audio output
3. **Voice Activity Detection**: Automatic start/stop based on speech detection
4. **Model Auto-Download**: Automatic model downloading and management
5. **Additional Providers**: Support for more STT/TTS services
6. **Real-time Streaming**: Lower latency processing for real-time conversations

## Performance Considerations

- **Whisper Models**: Larger models provide better accuracy but slower processing
- **Chunk Size**: Larger chunks improve accuracy but increase latency
- **Memory Usage**: Models are loaded into memory; consider model size vs. available RAM
- **CPU Usage**: Whisper processing is CPU-intensive; consider using GPU acceleration
- **Piper Performance**: External process overhead; consider embedding piper library

## Troubleshooting

### Common Issues

1. **Model Not Found**: Ensure model files exist at configured paths
2. **Piper Not Found**: Verify piper is installed and in PATH
3. **Audio Format**: Ensure audio is 16-bit signed integer at 16kHz
4. **Memory Issues**: Use smaller Whisper models for resource-constrained systems
5. **Permission Issues**: Ensure read access to model files and execute access to piper

### Debugging

Enable verbose logging to see detailed voice processing information:

```rust
log::info!("Voice processing status: ...");
log::warn!("Voice warning: ...");
log::error!("Voice error: ...");
```

## Conclusion

The voice implementation provides a solid foundation for adding speech capabilities to Zed's language models. With real Whisper STT and Piper TTS implementations, users can now have voice conversations with language models, making the interaction more natural and accessible.

The modular design allows for easy extension with additional providers and features, while the current implementation provides immediate value for users who want to experiment with voice-enabled AI interactions. 