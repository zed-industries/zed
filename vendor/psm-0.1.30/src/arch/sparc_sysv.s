#include "psm.h"
#include "gnu_stack_note.s"

/* FIXME: this ABI has definitely not been verified at all */

.text
.globl rust_psm_stack_direction
.p2align 2
.type rust_psm_stack_direction,@function
rust_psm_stack_direction:
/* extern "C" fn() -> u8 */
.cfi_startproc
    jmpl %o7 + 8, %g0
    mov STACK_DIRECTION_DESCENDING, %o0
.rust_psm_stack_direction_end:
.size       rust_psm_stack_direction,.rust_psm_stack_direction_end-rust_psm_stack_direction
.cfi_endproc


.globl rust_psm_stack_pointer
.p2align 2
.type rust_psm_stack_pointer,@function
rust_psm_stack_pointer:
/* extern "C" fn() -> *mut u8 */
.cfi_startproc
    jmpl %o7 + 8, %g0
    mov %o6, %o0
.rust_psm_stack_pointer_end:
.size       rust_psm_stack_pointer,.rust_psm_stack_pointer_end-rust_psm_stack_pointer
.cfi_endproc


.globl rust_psm_replace_stack
.p2align 2
.type rust_psm_replace_stack,@function
rust_psm_replace_stack:
/* extern "C" fn(%i0: usize, %i1: extern "C" fn(usize), %i2: *mut u8) */
.cfi_startproc
    .cfi_def_cfa 0, 0
    .cfi_return_column 0
    jmpl %o1, %g0
    /* WEIRD: Why is the LSB set for the %sp and %fp on SPARC?? */
    add %o2, -0x3ff, %o6
.rust_psm_replace_stack_end:
.size       rust_psm_replace_stack,.rust_psm_replace_stack_end-rust_psm_replace_stack
.cfi_endproc


.globl rust_psm_on_stack
.p2align 2
.type rust_psm_on_stack,@function
rust_psm_on_stack:
/* extern "C" fn(%i0: usize, %i1: usize, %i2: extern "C" fn(usize, usize), %i3: *mut u8) */
.cfi_startproc
    save %o3, -0x43f, %o6
    .cfi_def_cfa_register %fp
    .cfi_window_save
    .cfi_register %r15, %r31
    mov %i1, %o1
    jmpl %i2, %o7
    mov %i0, %o0
    ret
    restore
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
