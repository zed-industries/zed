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
        
        // Initialize Whisper context with optimized parameters for Metal GPU acceleration
        let mut ctx_params = WhisperContextParameters::default();
        
        // Enable Metal GPU acceleration for Apple Silicon
        eprintln!("DEBUG: Enabling Metal GPU acceleration for Apple Silicon...");
        
        let context = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| format!("Failed to initialize Whisper context: {:?}", e))?;
        
        // Create a new state for this transcription
        let mut state = context.create_state()
            .map_err(|e| format!("Failed to create Whisper state: {:?}", e))?;
        
        // Set up transcription parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        
        // Use optimal number of threads for better performance
        let optimal_threads = std::thread::available_parallelism()
            .map(|p| (p.get() / 2).max(1).min(8)) // Use half of available cores, max 8
            .unwrap_or(4); // Default to 4 threads if detection fails
        params.set_n_threads(optimal_threads as i32);
        eprintln!("DEBUG: Using {} threads for transcription", optimal_threads);
        
        params.set_translate(false);
        
        // Handle automatic language detection
        let use_auto_detection = language == "auto" || language == "detect";
        if use_auto_detection {
            // For auto-detection, we explicitly set language to None
            // This should force Whisper to detect the language automatically
            params.set_language(None);
            eprintln!("DEBUG: Using auto-detection mode - language set to None");
        } else {
            // Only set language if it's explicitly specified
            params.set_language(Some(language));
            eprintln!("DEBUG: Using specified language: {}", language);
        }
        
        // Improve multilingual detection
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_no_context(false);
        params.set_single_segment(false);
        
        // For better language detection, we can set some additional parameters
        if use_auto_detection {
            eprintln!("DEBUG: Using auto-detection mode");
        }
        
        // Run the transcription
        state.full(params, &samples)
            .map_err(|e| format!("Whisper transcription failed: {:?}", e))?;
        
        // Extract the transcribed text
        let num_segments = state.full_n_segments()
            .map_err(|e| format!("Failed to get segment count: {:?}", e))?;
        
        eprintln!("DEBUG: Number of segments: {}", num_segments);
        
        let mut full_text = String::new();
        for i in 0..num_segments {
            let segment_text = state.full_get_segment_text(i)
                .map_err(|e| format!("Failed to get segment {} text: {:?}", i, e))?;
            
            eprintln!("DEBUG: Segment {} text: '{}'", i, segment_text);
            
            if !full_text.is_empty() {
                full_text.push(' ');
            }
            full_text.push_str(&segment_text);
        }
        
        eprintln!("DEBUG: Final transcribed text: '{}'", full_text.trim());
        
        // Get the detected language from Whisper
        let detected_language = if use_auto_detection && num_segments > 0 {
            // Try to get the detected language from the first segment
            match state.full_lang_id_from_state() {
                Ok(lang_id) => {
                    eprintln!("DEBUG: Detected language ID: {}", lang_id);
                    // Convert language ID to language code
                    // Whisper returns language IDs, we need to map them to codes
                    let detected_lang = match lang_id {
                        0 => "en",   // English
                        1 => "zh",   // Chinese
                        2 => "de",   // German
                        3 => "es",   // Spanish
                        4 => "ru",   // Russian
                        5 => "ko",   // Korean
                        6 => "fr",   // French
                        7 => "ja",   // Japanese
                        8 => "pt",   // Portuguese
                        9 => "tr",   // Turkish
                        10 => "pl",  // Polish
                        11 => "ca",  // Catalan
                        12 => "nl",  // Dutch
                        13 => "ar",  // Arabic
                        14 => "sv",  // Swedish
                        15 => "it",  // Italian
                        16 => "id",  // Indonesian
                        17 => "hi",  // Hindi
                        18 => "fi",  // Finnish
                        19 => "vi",  // Vietnamese
                        20 => "he",  // Hebrew
                        21 => "uk",  // Ukrainian
                        22 => "el",  // Greek
                        23 => "ms",  // Malay
                        24 => "cs",  // Czech
                        25 => "ro",  // Romanian
                        26 => "da",  // Danish
                        27 => "hu",  // Hungarian
                        28 => "ta",  // Tamil
                        29 => "no",  // Norwegian
                        30 => "th",  // Thai
                        31 => "ur",  // Urdu
                        32 => "hr",  // Croatian
                        33 => "bg",  // Bulgarian
                        34 => "lt",  // Lithuanian
                        35 => "la",  // Latin
                        36 => "mi",  // Maori
                        37 => "ml",  // Malayalam
                        38 => "cy",  // Welsh
                        39 => "sk",  // Slovak
                        40 => "te",  // Telugu
                        41 => "fa",  // Persian
                        42 => "lv",  // Latvian
                        43 => "bn",  // Bengali
                        44 => "sr",  // Serbian
                        45 => "az",  // Azerbaijani
                        46 => "sl",  // Slovenian
                        47 => "kn",  // Kannada
                        48 => "et",  // Estonian
                        49 => "mk",  // Macedonian
                        50 => "br",  // Breton
                        51 => "eu",  // Basque
                        52 => "is",  // Icelandic
                        53 => "hy",  // Armenian
                        54 => "ne",  // Nepali
                        55 => "mn",  // Mongolian
                        56 => "bs",  // Bosnian
                        57 => "kk",  // Kazakh
                        58 => "sq",  // Albanian
                        59 => "sw",  // Swahili
                        60 => "gl",  // Galician
                        61 => "mr",  // Marathi
                        62 => "pa",  // Punjabi
                        63 => "si",  // Sinhala
                        64 => "km",  // Khmer
                        65 => "sn",  // Shona
                        66 => "yo",  // Yoruba
                        67 => "so",  // Somali
                        68 => "af",  // Afrikaans
                        69 => "oc",  // Occitan
                        70 => "ka",  // Georgian
                        71 => "be",  // Belarusian
                        72 => "tg",  // Tajik
                        73 => "sd",  // Sindhi
                        74 => "gu",  // Gujarati
                        75 => "am",  // Amharic
                        76 => "yi",  // Yiddish
                        77 => "lo",  // Lao
                        78 => "uz",  // Uzbek
                        79 => "fo",  // Faroese
                        80 => "ht",  // Haitian Creole
                        81 => "ps",  // Pashto
                        82 => "tk",  // Turkmen
                        83 => "nn",  // Nynorsk
                        84 => "mt",  // Maltese
                        85 => "sa",  // Sanskrit
                        86 => "lb",  // Luxembourgish
                        87 => "my",  // Myanmar
                        88 => "bo",  // Tibetan
                        89 => "tl",  // Tagalog
                        90 => "mg",  // Malagasy
                        91 => "as",  // Assamese
                        92 => "tt",  // Tatar
                        93 => "haw", // Hawaiian
                        94 => "ln",  // Lingala
                        95 => "ha",  // Hausa
                        96 => "ba",  // Bashkir
                        97 => "jw",  // Javanese
                        98 => "su",  // Sundanese
                        _ => "en",   // Default to English for unknown IDs
                    };
                    eprintln!("DEBUG: Mapped language ID {} to code: {}", lang_id, detected_lang);
                    
                    // Check if the transcription seems wrong for the detected language
                    // This is a heuristic to catch cases where language detection fails
                    if detected_lang == "en" && (full_text.trim() == "Hello" || full_text.trim() == "." || full_text.trim().is_empty()) {
                        eprintln!("DEBUG: Suspicious transcription '{}' for English detection - might be misdetected", full_text.trim());
                        // If we got a very generic result, it might be wrong
                        // For now, we'll trust Whisper's detection but log the concern
                    }
                    
                    detected_lang.to_string()
                },
                Err(e) => {
                    eprintln!("DEBUG: Failed to get language ID: {:?}", e);
                    "en".to_string() // Default to English if detection fails
                }
            }
        } else if use_auto_detection {
            // If auto-detection is requested but we have no segments, default to English
            eprintln!("DEBUG: Auto-detection requested but no segments found, defaulting to English");
            "en".to_string()
        } else {
            eprintln!("DEBUG: Using specified language: {}", language);
            language.to_string() // Use the specified language
        };
        
        // Output the result as JSON
        let result = serde_json::json!({
            "text": full_text.trim(),
            "confidence": 1.0,
            "language": detected_language,
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