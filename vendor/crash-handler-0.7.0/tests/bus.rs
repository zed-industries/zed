#![cfg(unix)]

mod shared;

#[test]
fn handles_bus() {
    shared::handles_crash(shared::SadnessFlavor::Bus);
}
