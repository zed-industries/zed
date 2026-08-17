#![cfg(windows)]

mod shared;

#[test]
fn handles_invalid_param() {
    shared::handles_crash(shared::SadnessFlavor::InvalidParameter);
}
