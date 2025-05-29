use std::env;
use std::process;
use std::io::{self, Read};

#[cfg(feature = "whisper-stt")]
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <model_path> <language>", args[0]);
        eprintln!("Audio samples should be provided via stdin as JSON");
        process::exit(1);
    }
    
    let model_path = &args[1];
    let language = &args[2];
    
    // Read audio samples from stdin
    let mut stdin = io::stdin();
    let mut audio_samples_json = String::new();
    stdin.read_to_string(&mut audio_samples_json)?;
    
    #[cfg(feature = "whisper-stt")]
    {
        // Parse audio samples from JSON
        let samples: Vec<f32> = serde_json::from_str(&audio_samples_json)?;
        
        // Initialize Whisper context
        let ctx_params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| format!("Failed to initialize Whisper context: {:?}", e))?;
        
        // Create a new state for this transcription
        let mut state = context.create_state()
            .map_err(|e| format!("Failed to create Whisper state: {:?}", e))?;
        
        // Set up transcription parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(1);
        params.set_translate(false);
        params.set_language(Some(language));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_no_context(false);
        params.set_single_segment(false);
        
        // Run the transcription
        state.full(params, &samples)
            .map_err(|e| format!("Whisper transcription failed: {:?}", e))?;
        
        // Extract the transcribed text
        let num_segments = state.full_n_segments()
            .map_err(|e| format!("Failed to get segment count: {:?}", e))?;
        
        let mut full_text = String::new();
        for i in 0..num_segments {
            let segment_text = state.full_get_segment_text(i)
                .map_err(|e| format!("Failed to get segment {} text: {:?}", i, e))?;
            
            if !full_text.is_empty() {
                full_text.push(' ');
            }
            full_text.push_str(&segment_text);
        }
        
        // Output the result as JSON
        let result = serde_json::json!({
            "text": full_text.trim(),
            "confidence": 1.0,
            "language": language,
            "duration_secs": samples.len() as f32 / 16000.0
        });
        
        println!("{}", result);
        Ok(())
    }
    
    #[cfg(not(feature = "whisper-stt"))]
    {
        eprintln!("Whisper STT feature not enabled");
        process::exit(1);
    }
} 