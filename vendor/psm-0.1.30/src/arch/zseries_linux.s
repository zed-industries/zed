/* Implementation of stack swtiching routines for zSeries LINUX ABI.

   This ABI is used by the s390x-unknown-linux-gnu target.

   Documents used:

   * LINUX for zSeries: ELF Application Binary Interface Supplement (1st ed., 2001) (LNUX-1107-01)
   * z/Architecture: Principles of Operation (4th ed., 2004) (SA22-7832-03)
*/

#include "psm.h"
#include "gnu_stack_note.s"

.text


.globl rust_psm_stack_direction
.p2align 4
.type rust_psm_stack_direction,@function
rust_psm_stack_direction:
/* extern "C" fn() -> u8 */
.cfi_startproc
    lghi %r2, STACK_DIRECTION_DESCENDING
    br %r14
.rust_psm_stack_direction_end:
.size       rust_psm_stack_direction,.rust_psm_stack_direction_end-rust_psm_stack_direction
.cfi_endproc


.globl rust_psm_stack_pointer
.p2align 4
.type rust_psm_stack_pointer,@function
rust_psm_stack_pointer:
/* extern "C" fn() -> *mut u8 */
.cfi_startproc
    la %r2, 0(%r15)
    br %r14
.rust_psm_stack_pointer_end:
.size       rust_psm_stack_pointer,.rust_psm_stack_pointer_end-rust_psm_stack_pointer
.cfi_endproc


.globl rust_psm_replace_stack
.p2align 4
.type rust_psm_replace_stack,@function
rust_psm_replace_stack:
/* extern "C" fn(r2: usize, r3: extern "C" fn(usize), r4: *mut u8) */
.cfi_startproc
    /* FIXME: backtrace does not terminate cleanly for some reason */
    lay %r15, -160(%r4)
    /* FIXME: this is `basr` instead of `br` purely to remove the backtrace link to the caller */
    basr %r14, %r3
.rust_psm_replace_stack_end:
.size       rust_psm_replace_stack,.rust_psm_replace_stack_end-rust_psm_replace_stack
.cfi_endproc


.globl rust_psm_on_stack
.p2align 4
.type rust_psm_on_stack,@function
rust_psm_on_stack:
/* extern "C" fn(r2: usize, r3: usize, r4: extern "C" fn(usize, usize), r5: *mut u8) */
.cfi_startproc
    stmg %r14, %r15, -16(%r5)
    lay %r15, -176(%r5)
    .cfi_def_cfa %r15, 176
    .cfi_offset %r14, -16
    .cfi_offset %r15, -8
    basr %r14, %r4
    lmg %r14, %r15, 160(%r15)
    .cfi_restore %r14
    .cfi_restore %r15
    br %r14
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
