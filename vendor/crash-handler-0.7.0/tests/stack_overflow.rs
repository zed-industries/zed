mod shared;

#[test]
fn handles_stack_overflow() {
    shared::handles_crash(shared::SadnessFlavor::StackOverflow {
        non_rust_thread: false,
        long_jumps: false,
    });
}
