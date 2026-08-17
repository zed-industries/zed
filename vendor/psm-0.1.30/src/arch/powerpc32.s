#include "psm.h"
#include "gnu_stack_note.s"
/* FIXME: this probably does not cover all ABIs? Tested with sysv only, possibly works for AIX as
   well?
*/

.text
.globl rust_psm_stack_direction
.p2align 2
.type rust_psm_stack_direction,@function
rust_psm_stack_direction:
/* extern "C" fn() -> u8 */
.cfi_startproc
    li 3, STACK_DIRECTION_DESCENDING
    blr
.rust_psm_stack_direction_end:
.size       rust_psm_stack_direction,.rust_psm_stack_direction_end-rust_psm_stack_direction
.cfi_endproc


.globl rust_psm_stack_pointer
.p2align 2
.type rust_psm_stack_pointer,@function
rust_psm_stack_pointer:
/* extern "C" fn() -> *mut u8 */
.cfi_startproc
    mr 3, 1
    blr
.rust_psm_stack_pointer_end:
.size       rust_psm_stack_pointer,.rust_psm_stack_pointer_end-rust_psm_stack_pointer
.cfi_endproc


.globl rust_psm_replace_stack
.p2align 2
.type rust_psm_replace_stack,@function
rust_psm_replace_stack:
/* extern "C" fn(3: usize, 4: extern "C" fn(usize), 5: *mut u8) */
.cfi_startproc
/* NOTE: perhaps add a debug-assertion for stack alignment? */
    addi 5, 5, -16
    mr 1, 5
    mtctr 4
    bctr
.rust_psm_replace_stack_end:
.size       rust_psm_replace_stack,.rust_psm_replace_stack_end-rust_psm_replace_stack
.cfi_endproc


.globl rust_psm_on_stack
.p2align 2
.type rust_psm_on_stack,@function
rust_psm_on_stack:
/* extern "C" fn(3: usize, 4: usize, 5: extern "C" fn(usize, usize), 6: *mut u8) */
.cfi_startproc
    mflr 0
    stw 0, -24(6)
    sub 6, 6, 1
    addi 6, 6, -32
    stwux 1, 1, 6
    .cfi_def_cfa r1, 32
    .cfi_offset r1, -32
    .cfi_offset lr, -24
    mtctr 5
    bctrl
    lwz 0, 8(1)
    mtlr 0
    .cfi_restore lr
    /* FIXME: after this instruction backtrace breaks until control returns to the caller
       That being said compiler-generated code has the same issue, so I guess that is fine for now?
    */
    lwz 1, 0(1)
    .cfi_restore r1
    blr
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
