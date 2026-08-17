// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!
//
// Also at this time this file is heavily based off the x86_64 file, so you'll
// probably want to read that one as well.

use wasmtime_asm_macros::asm_func;

// fn(top_of_stack(%r0): *mut u8)
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_switch),
    "
        // Save callee-saved registers
        push {{r4-r11,lr}}

        // Swap stacks, recording our current stack pointer
        ldr r4, [r0, #-0x08]
        str sp, [r0, #-0x08]
        mov sp, r4

        // Restore and return
        pop {{r4-r11,lr}}
        bx lr
    ",
);

// fn(
//    top_of_stack(%r0): *mut u8,
//    entry_point(%r1): extern fn(*mut u8, *mut u8),
//    entry_arg0(%r2): *mut u8,
// )
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_init),
    "
        adr r3, {start}
        str r3, [r0, #-0x0c] // => lr
        str r0, [r0, #-0x10] // => r11
        str r1, [r0, #-0x14] // => r10
        str r2, [r0, #-0x18] // => r9

        add r3, r0, #-0x2c
        str r3, [r0, #-0x08]
        bx lr
    ",
    start = sym super::wasmtime_fiber_start,
);

asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_start),
    "
        .cfi_startproc simple
        .cfi_def_cfa_offset 0
        // See the x86_64 file for more commentary on what these CFI directives
        // are doing. Like over there note that the relative offsets to
        // registers here match the frame layout in `wasmtime_fiber_switch`.
        //
        // TODO: this is only lightly tested. This gets backtraces in gdb but
        // not at runtime. Perhaps the libgcc at runtime was too old? Doesn't
        // support something here? Unclear. Will need investigation if someone
        // ends up needing this and it still doesn't work.
        .cfi_escape 0x0f,    /* DW_CFA_def_cfa_expression */ \
            5,               /* the byte length of this expression */ \
            0x7d, 0x00,      /* DW_OP_breg14(%sp) + 0 */ \
            0x06,            /* DW_OP_deref */ \
            0x23, 0x24	 /* DW_OP_plus_uconst 0x24 */

        .cfi_rel_offset lr, -0x04
        .cfi_rel_offset r11, -0x08
        .cfi_rel_offset r10, -0x0c
        .cfi_rel_offset r9, -0x10
        .cfi_rel_offset r8, -0x14
        .cfi_rel_offset r7, -0x18
        .cfi_rel_offset r6, -0x1c
        .cfi_rel_offset r5, -0x20
        .cfi_rel_offset r4, -0x24

        mov r1, r11
        mov r0, r9
        blx r10
        .cfi_endproc
    ",
);
