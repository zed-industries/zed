mod shared;

#[test]
fn handles_abort() {
    shared::handles_crash(shared::SadnessFlavor::Abort);
}
