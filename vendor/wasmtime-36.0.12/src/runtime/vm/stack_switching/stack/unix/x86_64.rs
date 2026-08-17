// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!

use wasmtime_asm_macros::asm_func;

// This is a pretty special function that has no real signature. Its use is to
// be the "base" function of all fibers. This entrypoint is used in
// `wasmtime_continuation_init` to bootstrap the execution of a new fiber.
//
// We also use this function as a persistent frame on the stack to emit dwarf
// information to unwind into the caller. This allows us to unwind from the
// fiber's stack back to the initial stack that the fiber was called from. We use
// special dwarf directives here to do so since this is a pretty nonstandard
// function.
//
// If you're curious a decent introduction to CFI things and unwinding is at
// https://www.imperialviolet.org/2017/01/18/cfi.html
//
// Note that this function is never called directly. It is only ever entered
// when a `stack_switch` instruction loads its address when switching to a stack
// prepared by `FiberStack::initialize`.
//
// Executing `stack_switch` on a stack prepared by `FiberStack::initialize` as
// described in the comment on `FiberStack::initialize` leads to the following
// values in various registers when execution of wasmtime_continuation_start begins:
//
// RSP: TOS - 0x40 - (16 * `args_capacity`)
// RBP: TOS - 0x10
asm_func!(
    "wasmtime_continuation_start",
    "
        // TODO(frank-emrich): Restore DWARF information for this function. In
        // the meantime, debugging is possible using frame pointer walking.


        //
        // Note that the next 4 instructions amount to calling fiber_start
        // with the following arguments:
        // 1. func_ref
        // 2. caller_vmctx
        // 3. args (of type *mut ArrayRef<ValRaw>)
        // 4. return_value_count
        //

        pop rcx // return_value_count
        pop rdx // args
        pop rsi // caller_vmctx
        pop rdi // func_ref
        // Note that RBP already contains the right frame pointer to build a
        // frame pointer chain including the parent continuation:
        // The current value of RBP is where we store the parent RBP in the
        // control context!
        call {fiber_start}

        // Return to the parent continuation.
        // RBP is callee-saved (no matter if it's used as a frame pointe or
        // not), so its value is still TOS - 0x10.
        // Use that fact to obtain saved parent RBP, RSP, and RIP from control
        // context near TOS.
        mov rsi,  0x08[rbp] // putting new RIP in temp register
        mov rsp, -0x08[rbp]
        mov rbp,      [rbp]

        // The stack_switch instruction uses register RDI for the payload.
        // Here, the payload indicates that we are returning (value 0).
        // See the test case below to keep this in sync with
        // ControlEffect::return_()
        mov rdi, 0

        jmp rsi
    ",
    fiber_start = sym super::fiber_start,
);

#[test]
fn test_return_payload() {
    // The following assumption is baked into `wasmtime_continuation_start`.
    assert_eq!(wasmtime_environ::CONTROL_EFFECT_RETURN_DISCRIMINANT, 0);
}
