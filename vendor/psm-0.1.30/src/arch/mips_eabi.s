/*
Not only MIPS has 20 different ABIs... nobody tells anybody what specific variant of which ABI is
used where.

This is an "EABI" implementation based on the following page:

http://www.cygwin.com/ml/binutils/2003-06/msg00436.html
*/

#include "psm.h"
#include "gnu_stack_note.s"

.set noreorder /* we’ll manage the delay slots on our own, thanks! */

.text
.abicalls
.globl rust_psm_stack_direction
.p2align 2
.type rust_psm_stack_direction,@function
.ent rust_psm_stack_direction
/* extern "C" fn() -> u8 */
rust_psm_stack_direction:
.cfi_startproc
    jr $31
    addiu $2, $zero, STACK_DIRECTION_DESCENDING
.end rust_psm_stack_direction
.rust_psm_stack_direction_end:
.size       rust_psm_stack_direction,.rust_psm_stack_direction_end-rust_psm_stack_direction
.cfi_endproc


.globl rust_psm_stack_pointer
.p2align 2
.type rust_psm_stack_pointer,@function
.ent rust_psm_stack_pointer
/* extern "C" fn() -> *mut u8 */
rust_psm_stack_pointer:
.cfi_startproc
    jr $31
    move $2, $29
.end rust_psm_stack_pointer
.rust_psm_stack_pointer_end:
.size       rust_psm_stack_pointer,.rust_psm_stack_pointer_end-rust_psm_stack_pointer
.cfi_endproc



.globl rust_psm_replace_stack
.p2align 2
.type rust_psm_replace_stack,@function
.ent rust_psm_replace_stack
/* extern "C" fn(r4: usize, r5: extern "C" fn(usize), r6: *mut u8) */
rust_psm_replace_stack:
.cfi_startproc
    move $25, $5
    jr $5
    move $29, $6
.end rust_psm_replace_stack
.rust_psm_replace_stack_end:
.size       rust_psm_replace_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc


.globl rust_psm_on_stack
.p2align 2
.type rust_psm_on_stack,@function
.ent rust_psm_on_stack
/* extern "C" fn(r4: usize, r5: usize, r6: extern "C" fn(usize), r7: *mut u8) */
rust_psm_on_stack:
.cfi_startproc
    sw $29, -4($7)
    sw $31, -8($7)
    .cfi_def_cfa 7, 0
    .cfi_offset 31, -8
    .cfi_offset 29, -4
    move $25, $6
    jalr $31, $6
    addiu $29, $7, -8
    .cfi_def_cfa 29, 8
    lw $31, 0($29)
    .cfi_restore 31
    lw $29, 4($29)
    .cfi_restore 29
    jr $31
    nop
.end rust_psm_on_stack
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
