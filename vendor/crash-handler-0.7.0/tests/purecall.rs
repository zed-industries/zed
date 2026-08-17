#![cfg(windows)]

mod shared;

#[test]
fn handles_purecall() {
    shared::handles_crash(shared::SadnessFlavor::Purecall);
}
