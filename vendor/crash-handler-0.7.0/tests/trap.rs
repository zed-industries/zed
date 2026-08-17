mod shared;

#[test]
fn handles_trap() {
    shared::handles_crash(shared::SadnessFlavor::Trap);
}
