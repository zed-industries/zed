// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!
//
// This file is modeled after x86_64.rs and comments are not copied over. For
// reference be sure to review the other file. Note that the pointer size is
// different so the reserved space at the top of the stack is 8 bytes, not 16
// bytes. Still two pointers though.

use wasmtime_asm_macros::asm_func;

// fn(top_of_stack: *mut u8)
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_switch),
    "
        // Load our stack-to-use
        mov eax, 0x4[esp]
        mov ecx, -0x8[eax]

        // Save callee-saved registers
        push ebp
        push ebx
        push esi
        push edi

        // Save our current stack and jump to the stack-to-use
        mov -0x8[eax], esp
        mov esp, ecx

        // Restore callee-saved registers
        pop edi
        pop esi
        pop ebx
        pop ebp
        ret
    ",
);

// fn(
//    top_of_stack: *mut u8,
//    entry_point: extern fn(*mut u8, *mut u8),
//    entry_arg0: *mut u8,
// )
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_init),
    "
        mov eax, 4[esp]

        // move top_of_stack to the 2nd argument
        mov -0x0c[eax], eax

        // move entry_arg0 to the 1st argument
        mov ecx, 12[esp]
        mov -0x10[eax], ecx

        // Move our start function to the return address which the `ret` in
        // `wasmtime_fiber_start` will return to.
        lea ecx, {start}
        mov -0x14[eax], ecx

        // And move `entry_point` to get loaded into `%ebp` through the context
        // switch. This'll get jumped to in `wasmtime_fiber_start`.
        mov ecx, 8[esp]
        mov -0x18[eax], ecx

        // Our stack from top-to-bottom looks like:
        //
        //	  * 8 bytes of reserved space per unix.rs (two-pointers space)
        //	  * 8 bytes of arguments (two arguments wasmtime_fiber_start forwards)
        //	  * 4 bytes of return address
        //	  * 16 bytes of saved registers
        //
        // Note that after the return address the stack is conveniently 16-byte
        // aligned as required, so we just leave the arguments on the stack in
        // `wasmtime_fiber_start` and immediately do the call.
        lea ecx, -0x24[eax]
        mov -0x08[eax], ecx
        ret
    ",
    start = sym super::wasmtime_fiber_start,
);

asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_start),
    "
        .cfi_startproc simple
        .cfi_def_cfa_offset 0
        .cfi_escape 0x0f, /* DW_CFA_def_cfa_expression */ \
            5,            /* the byte length of this expression */ \
            0x74, 0x08,   /* DW_OP_breg4 (%esp) + 8 */ \
            0x06,         /* DW_OP_deref */ \
            0x23, 0x14    /* DW_OP_plus_uconst 0x14 */

        .cfi_rel_offset eip, -4
        .cfi_rel_offset ebp, -8
        .cfi_rel_offset ebx, -12
        .cfi_rel_offset esi, -16
        .cfi_rel_offset edi, -20

        // Our arguments and stack alignment are all prepped by
        // `wasmtime_fiber_init`.
        call ebp
        ud2
        .cfi_endproc
    ",
);
