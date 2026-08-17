mod shared;

#[test]
fn handles_segv() {
    shared::handles_crash(shared::SadnessFlavor::Segfault);
}
