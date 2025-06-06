// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Call the main function from the lib.rs
    if let Err(e) = codeorbit::main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
