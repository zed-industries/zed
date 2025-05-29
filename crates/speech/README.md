# Speech Processing Crate

A comprehensive speech processing library for Rust that provides both Speech-to-Text (STT) and Text-to-Speech (TTS) capabilities with support for multiple providers and platforms.

## Features

### Speech-to-Text (STT)
- **LLaMA STT**: Uses `llama-cpp-2` backend with GGML/GGUF format support
- **System STT**: Platform-specific speech recognition
- **OpenAI STT**: Cloud-based transcription (planned)

### Text-to-Speech (TTS)
- **System TTS**: Platform-specific speech synthesis
- **OpenAI TTS**: Cloud-based synthesis (planned)
- **ElevenLabs TTS**: High-quality voice synthesis (planned)

### Audio Processing
- Real-time audio capture and playback
- Audio format conversion (WAV, MP3, OGG)
- Voice activity detection
- Audio resampling and normalization

## GGML Format Support

This crate includes comprehensive support for the old GGML format used by Whisper models:

### Automatic Format Detection
The library automatically detects if your model file is in the old GGML format (`lmgg` magic bytes) and provides helpful conversion instructions:

```rust
use speech::stt::ggml_loader::{needs_conversion, suggest_conversion_command};

// Check if a model needs conversion
if needs_conversion("path/to/model.bin")? {
    println!("Model needs conversion!");
    println!("{}", suggest_conversion_command("path/to/model.bin"));
}
```

### Supported GGML Formats
- **GGML** (`lmgg`): Original format
- **GGMF** (`fmgg`): GGML with metadata
- **GGJT** (`tjgg`): GGML with JSON metadata

### Model Information Extraction
```rust
use speech::stt::ggml_loader::GGMLLoader;

let mut loader = GGMLLoader::new("path/to/model.bin")?;
let info = loader.get_model_info()?;
println!("{}", info);
```

### Conversion Instructions
When an old GGML format is detected, the library provides detailed conversion instructions:

```bash
# Example conversion command
python3 extra/llama.cpp/convert_llama_ggml_to_gguf.py \
--input ggml-large-v3-turbo.bin \
--output ggml-large-v3-turbo.gguf \
--name whisper-model
```

## Usage

### Basic STT Example
```rust
use speech::{SpeechProcessor, config::{SpeechConfig, SttConfig, SttProvider}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SpeechConfig {
        stt: Some(SttConfig {
            provider: SttProvider::Llama,
            model_path: Some("path/to/whisper-model.gguf".into()),
            language: "en".to_string(),
        }),
        tts: None,
    };
    
    let processor = SpeechProcessor::new(config).await?;
    
    // Transcribe audio samples
    let audio_data: Vec<f32> = vec![/* your audio samples */];
    let result = processor.transcribe_audio(&audio_data).await?;
    println!("Transcription: {}", result.text);
    
    Ok(())
}
```

### GGML Detection Example
```rust
use speech::stt::ggml_loader::{GGMLLoader, needs_conversion};

fn main() -> anyhow::Result<()> {
    let model_path = "ggml-large-v3-turbo.bin";
    
    if needs_conversion(model_path)? {
        println!("⚠️ Model is in old GGML format and needs conversion");
        
        let mut loader = GGMLLoader::new(model_path)?;
        let info = loader.get_model_info()?;
        println!("Model info:\n{}", info);
    } else {
        println!("✅ Model is in GGUF format and ready to use");
    }
    
    Ok(())
}
```

## Building

### Basic Build
```bash
cargo build
```

### With LLaMA STT Support
```bash
cargo build --features llama-stt
```

### With All Features
```bash
cargo build --features all
```

## Examples

### Test GGML Format Detection
```bash
cargo run --example test_ggml_detection
```

### Speech Processing Demo
```bash
cargo run --example speech_demo --features llama-stt
```

## Error Handling

The library provides comprehensive error handling for GGML format issues:

```rust
use speech::stt::llama::LlamaStt;
use speech::config::SttConfig;

match LlamaStt::new(config).await {
    Ok(stt) => println!("STT initialized successfully"),
    Err(e) => {
        if e.to_string().contains("lmgg") {
            println!("Model is in old GGML format - conversion required");
        } else {
            println!("Other error: {}", e);
        }
    }
}
```

## Dependencies

- `llama-cpp-2`: For GGUF format model loading
- `anyhow`: Error handling
- `tokio`: Async runtime
- `cpal`: Audio capture/playback
- `hound`: WAV file support
- `rubato`: Audio resampling

## Platform Support

- **macOS**: Full support with system frameworks
- **Windows**: SAPI integration
- **Linux**: Speech-dispatcher integration

## License

GPL-3.0-or-later 