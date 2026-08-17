/* Implementation of the AIX-like PowerPC ABI. Seems to be used by the big-endian PowerPC targets.
   The following references were used during the implementation of this code:

   https://www.ibm.com/support/knowledgecenter/en/ssw_aix_72/com.ibm.aix.alangref/idalangref_rntime_stack.htm
   https://www.ibm.com/support/knowledgecenter/en/ssw_aix_72/com.ibm.aix.alangref/idalangref_reg_use_conv.htm
   https://www.ibm.com/developerworks/library/l-powasm4/index.html
*/

#include "psm.h"
#include "gnu_stack_note.s"

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
    ld 2, 8(4)
    ld 4, 0(4)
    /* do not allocate the whole 112-byte sized frame, we know wont be used */
    addi 5, 5, -48
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
    std 2, -72(6)
    std 0, -8(6)
    sub 6, 6, 1
    addi 6, 6, -112
    stdux 1, 1, 6
    .cfi_def_cfa r1, 112
    .cfi_offset r1, -112
    .cfi_offset r2, -72
    .cfi_offset lr, -8
    /* load the function pointer from TOC and make the call */
    ld 2, 8(5)
    ld 5, 0(5)
    mtctr 5
    bctrl
    ld 2, 40(1)
    .cfi_restore r2
    ld 0, 104(1)
    mtlr 0
    .cfi_restore lr
    /* FIXME: after this instruction backtrace breaks until control returns to the caller.
       That being said compiler-generated code has the same issue, so I guess that is fine for now?
    */
    ld 1, 0(1)
    .cfi_restore r1
    blr
.rust_psm_on_stack_end:
.size       rust_psm_on_stack,.rust_psm_on_stack_end-rust_psm_on_stack
.cfi_endproc
