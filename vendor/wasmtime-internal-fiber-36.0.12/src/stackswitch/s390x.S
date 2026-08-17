// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!
//
// Also at this time this file is heavily based off the x86_64 file, so you'll
// probably want to read that one as well.

.text

#define CONCAT2(a, b) a ## b
#define CONCAT(a, b) CONCAT2(a , b)
#define VERSIONED_SYMBOL(a) CONCAT(a, VERSIONED_SUFFIX)
#define GLOBL(fnname) .globl VERSIONED_SYMBOL(fnname)
#define HIDDEN(fnname) .hidden VERSIONED_SYMBOL(fnname)
#define TYPE(fnname) .type VERSIONED_SYMBOL(fnname),@function
#define FUNCTION(fnname) VERSIONED_SYMBOL(fnname)
#define SIZE(fnname) .size VERSIONED_SYMBOL(fnname),.-VERSIONED_SYMBOL(fnname)

// fn(top_of_stack(%x0): *mut u8)
HIDDEN(wasmtime_fiber_switch)
GLOBL(wasmtime_fiber_switch)
.p2align 2
TYPE(wasmtime_fiber_switch)
FUNCTION(wasmtime_fiber_switch):
    // Save all callee-saved registers on the stack since we're assuming
    // they're clobbered as a result of the stack switch.
    stmg %r6, %r15, 48(%r15)
    aghi %r15, -64
    std %f8, 0(%r15)
    std %f9, 8(%r15)
    std %f10, 16(%r15)
    std %f11, 24(%r15)
    std %f12, 32(%r15)
    std %f13, 40(%r15)
    std %f14, 48(%r15)
    std %f15, 56(%r15)

    // Load our previously saved stack pointer to resume to, and save off our
    // current stack pointer on where to come back to eventually.
    lg %r1, -16(%r2)
    stg %r15, -16(%r2)

    // Switch to the new stack and restore all our callee-saved registers after
    // the switch and return to our new stack.
    ld %f8, 0(%r1)
    ld %f9, 8(%r1)
    ld %f10, 16(%r1)
    ld %f11, 24(%r1)
    ld %f12, 32(%r1)
    ld %f13, 40(%r1)
    ld %f14, 48(%r1)
    ld %f15, 56(%r1)
    lmg %r6, %r15, 112(%r1)
    br %r14
SIZE(wasmtime_fiber_switch)

// fn(
//    top_of_stack(%x0): *mut u8,
//    entry_point(%x1): extern fn(*mut u8, *mut u8),
//    entry_arg0(%x2): *mut u8,
// )
HIDDEN(wasmtime_fiber_init)
GLOBL(wasmtime_fiber_init)
.p2align 2
TYPE(wasmtime_fiber_init)
FUNCTION(wasmtime_fiber_init):
    larl %r1, FUNCTION(wasmtime_fiber_start)
    stg %r1, -48(%r2)  // wasmtime_fiber_start - restored into %r14
    stg %r2, -112(%r2) // top_of_stack - restored into %r6
    stg %r3, -104(%r2) // entry_point - restored into %r7
    stg %r4, -96(%r2)  // entry_arg0 - restored into %r8
    aghi %r2, -160     // 160 bytes register save area
    stg %r2, 120(%r2)  // bottom of register save area - restored into %r15

    // `wasmtime_fiber_switch` has a 64 byte stack.
    aghi %r2, -64
    stg %r2, 208(%r2)
    br %r14
SIZE(wasmtime_fiber_init)

.p2align 2
TYPE(wasmtime_fiber_start)
FUNCTION(wasmtime_fiber_start):
.cfi_startproc simple
.cfi_def_cfa_offset 0

    // See the x86_64 file for more commentary on what these CFI directives are
    // doing. Like over there note that the relative offsets to registers here
    // match the frame layout in `wasmtime_fiber_switch`.
    .cfi_escape 0x0f,    /* DW_CFA_def_cfa_expression */ \
        7,               /* the byte length of this expression */ \
        0x7f, 0x90, 0x1, /* DW_OP_breg15 0x90 */ \
        0x06,            /* DW_OP_deref */ \
        0x23, 0xe0, 0x1  /* DW_OP_plus_uconst 0xe0 */

    .cfi_rel_offset 6, -112
    .cfi_rel_offset 7, -104
    .cfi_rel_offset 8, -96
    .cfi_rel_offset 9, -88
    .cfi_rel_offset 10, -80
    .cfi_rel_offset 11, -72
    .cfi_rel_offset 12, -64
    .cfi_rel_offset 13, -56
    .cfi_rel_offset 14, -48
    .cfi_rel_offset 15, -40

    // Load our two arguments prepared by `wasmtime_fiber_init`.
    lgr %r2, %r8  // entry_arg0
    lgr %r3, %r6  // top_of_stack

    // ... and then we call the function! Note that this is a function call so
    // our frame stays on the stack to backtrace through.
    basr %r14, %r7  // entry_point
    // .. technically we shouldn't get here, so just trap.
    .word 0x0000
    .cfi_endproc
SIZE(wasmtime_fiber_start)

// Mark that we don't need executable stack.
.section .note.GNU-stack,"",%progbits
