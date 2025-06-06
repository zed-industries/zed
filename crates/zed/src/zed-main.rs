// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn main() {
    // separated out so that the file containing the main function can be imported by other crates,
    // while having all gpui resources that are registered in main (primarily actions) initialized
    zed::main();
}
