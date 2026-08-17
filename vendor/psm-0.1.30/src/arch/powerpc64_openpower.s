/* Implementation of stack swtiching routines for OpenPOWER 64-bit ELF ABI
   The specification can be found at
   http://openpowerfoundation.org/wp-content/uploads/resources/leabi/content/ch_preface.html

   This ABI is usually used by the ppc64le targets.
*/

#include "psm.h"
#include "gnu_stack_note.s"

.text
.abiversion 2


.globl rust_psm_stack_direction
.p2align 4
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
.p2align 4
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
.p2align 4
.type rust_psm_replace_stack,@function
rust_psm_replace_stack:
/* extern "C" fn(3: usize, 4: extern "C" fn(usize), 5: *mut u8) */
.cfi_startproc
    addi 5, 5, -32
    mtctr 4
    mr 12, 4
    mr 1, 5
    bctr
.rust_psm_replace_stack_end:
.size       rust_psm_replace_stack,.rust_psm_replace_stack_end-rust_psm_replace_stack
.cfi_endproc



.globl rust_psm_on_stack
.p2align 4
.type rust_psm_on_stack,@function
rust_psm_on_stack:
/* extern "C" fn(3: usize, 4: usize, 5: extern "C" fn(usize, usize), 6: *mut u8) */
.cfi_startproc
    mflr 0
    std 0, -8(6)
    std 2, -24(6)
    sub 6, 6, 1
    addi 6, 6, -48
    stdux 1, 1, 6
    .cfi_def_cfa r1, 48
    .cfi_offset r1, -48
    .cfi_offset r2, -24
    .cfi_offset lr, -8
    mr 12, 5
    mtctr 5
    bctrl
    ld 2, 24(1)
    .cfi_restore r2
    ld 0, 40(1)
    mtlr 0
    .cfi_restore lr
    /* FIXME: after this instructin backtrace breaks until control returns to the caller */
    ld 1, 0(1)
    blr
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
