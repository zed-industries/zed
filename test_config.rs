use speech::SttConfig; fn main() { let config = SttConfig::whisper_with_auto_model(); println!("Model found: {:?}", config.model_path); println!("Valid: {}", config.is_valid()); }
