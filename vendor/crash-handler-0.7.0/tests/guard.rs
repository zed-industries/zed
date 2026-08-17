#![cfg(target_os = "macos")]

mod shared;

#[test]
fn handles_guard() {
    shared::handles_crash(shared::SadnessFlavor::Guard);
}
