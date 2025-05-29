fn main() {
    // Set environment variable to skip whisper-rs binding generation
    // This avoids conflicts with GGML symbols from llama-cpp-2
    std::env::set_var("WHISPER_DONT_GENERATE_BINDINGS", "1");
    
    println!("cargo:rustc-env=WHISPER_DONT_GENERATE_BINDINGS=1");
    println!("cargo:rerun-if-changed=build.rs");
} 