mod shared;

#[test]
fn handles_illegal_instruction() {
    shared::handles_crash(shared::SadnessFlavor::Illegal);
}
