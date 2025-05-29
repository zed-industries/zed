# Recognition Crate

This crate provides voice recognition capabilities for Zed using Whisper models.

## Overview

The recognition crate integrates OpenAI's Whisper speech-to-text models through the `whisper-rs` library, which provides Rust bindings to the C++ `whisper.cpp` implementation.

## GGML Conflict Resolution

This crate uses `whisper-rs` which depends on GGML (the machine learning library). Since Zed also uses `llama-cpp-2` (which also includes GGML), there was a symbol conflict during linking.

### Solution

We resolved this by:

1. **Using pre-built bindings**: Set `WHISPER_DONT_GENERATE_BINDINGS=1` to skip binding generation and use existing bindings
2. **Build script**: Added `build.rs` that automatically sets this environment variable
3. **Version pinning**: Use `whisper-rs = "0.12"` for stability

### Building

The crate can be built in several ways:

```bash
# Using the provided build script (recommended)
./scripts/build-with-whisper.sh check -p recognition

# Or manually setting the environment variable
WHISPER_DONT_GENERATE_BINDINGS=1 cargo check -p recognition

# For the entire project
WHISPER_DONT_GENERATE_BINDINGS=1 cargo check
```

## Features

- **Voice Recognition**: Transcribe audio samples to text using Whisper models
- **Multiple Model Sizes**: Support for different Whisper model sizes (tiny, base, small, medium, large)
- **Audio Processing**: Built-in audio format conversion and preprocessing
- **Language Detection**: Automatic language detection and multi-language support

## Usage

```rust
use recognition::{VoiceRecognizer, ModelSize};

// Create a new recognizer
let mut recognizer = VoiceRecognizer::new(ModelSize::Base).await?;

// Transcribe audio samples
let result = recognizer.transcribe_samples(&audio_samples, 16000, 1).await?;
println!("Transcribed text: {}", result.text);
```

## Dependencies

- `whisper-rs`: Rust bindings to whisper.cpp
- `hound`: Audio file loading and processing
- `tokio`: Async runtime support

## Notes

- The first run will download the selected Whisper model (~150MB for base model)
- Models are cached locally for subsequent uses
- Requires sufficient memory for model loading (varies by model size) 