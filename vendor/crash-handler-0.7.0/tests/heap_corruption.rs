#![cfg(windows)]

mod shared;

#[test]
fn handles_heap_corruption() {
    shared::handles_crash(shared::SadnessFlavor::HeapCorruption);
}
