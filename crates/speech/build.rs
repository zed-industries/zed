fn main() {
    // Set environment variable to prevent whisper-rs from including GGML
    // This avoids symbol conflicts with llama-cpp-2 used in the agent crate
    std::env::set_var("WHISPER_DONT_GENERATE_BINDINGS", "1");
    
    // Also set this to use system whisper if available
    std::env::set_var("WHISPER_NO_DOWNLOAD_MODELS", "1");
    
    println!("cargo:rerun-if-env-changed=WHISPER_DONT_GENERATE_BINDINGS");
    println!("cargo:rerun-if-env-changed=WHISPER_NO_DOWNLOAD_MODELS");
    
    // Platform-specific build configuration
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=sapi");
        }
        "linux" => {
            // speech-dispatcher will handle its own linking
        }
        _ => {}
    }
} 