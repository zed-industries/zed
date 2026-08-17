#include "psm.h"
#include "gnu_stack_note.s"

.text
.syntax unified

#if defined(CFG_TARGET_OS_darwin) || defined(CFG_TARGET_OS_macos) || defined(CFG_TARGET_OS_ios)

#define GLOBL(fnname) .globl _##fnname
#define THUMBTYPE(fnname) .thumb_func _##fnname
#define FUNCTION(fnname) _##fnname
#define THUMBFN .code 16
#define SIZE(fnname,endlabel)
#define FNSTART
#define CANTUNWIND
#define FNEND

#else

#define GLOBL(fnname) .globl fnname
#define THUMBTYPE(fnname) .type fnname,%function
#define FUNCTION(fnname) fnname
#define THUMBFN .code 16
#define SIZE(fnname,endlabel) .size fnname,endlabel-fnname
#define FNSTART .fnstart
#define CANTUNWIND .cantunwind
#define FNEND .fnend

#endif


GLOBL(rust_psm_stack_direction)
.p2align 2
THUMBTYPE(rust_psm_stack_direction)
THUMBFN
FUNCTION(rust_psm_stack_direction):
/* extern "C" fn() -> u8 */
FNSTART
.cfi_startproc
    /* movs to support Thumb-1 */
    movs r0, #STACK_DIRECTION_DESCENDING
    bx lr
.rust_psm_stack_direction_end:
SIZE(rust_psm_stack_direction,.rust_psm_stack_direction_end)
.cfi_endproc
CANTUNWIND
FNEND

GLOBL(rust_psm_stack_pointer)
.p2align 2
THUMBTYPE(rust_psm_stack_pointer)
THUMBFN
FUNCTION(rust_psm_stack_pointer):
/* extern "C" fn() -> *mut u8 */
FNSTART
.cfi_startproc
    mov r0, sp
    bx  lr
.rust_psm_stack_pointer_end:
SIZE(rust_psm_stack_pointer,.rust_psm_stack_pointer_end)
.cfi_endproc
CANTUNWIND
FNEND


GLOBL(rust_psm_replace_stack)
.p2align 2
THUMBTYPE(rust_psm_replace_stack)
THUMBFN
FUNCTION(rust_psm_replace_stack):
/* extern "C" fn(r0: usize, r1: extern "C" fn(usize), r2: *mut u8) */
FNSTART
.cfi_startproc
/* All we gotta do is set the stack pointer to %rdx & tail-call the callback in %rsi */
    mov sp, r2
    bx r1
.rust_psm_replace_stack_end:
SIZE(rust_psm_replace_stack,.rust_psm_replace_stack_end)
.cfi_endproc
CANTUNWIND
FNEND


GLOBL(rust_psm_on_stack)
.p2align 2
THUMBTYPE(rust_psm_on_stack)
THUMBFN
FUNCTION(rust_psm_on_stack):
/* extern "C" fn(r0: usize, r1: usize, r2: extern "C" fn(usize, usize), r3: *mut u8) */
FNSTART
.cfi_startproc
    push {r4, lr}
    .cfi_def_cfa_offset 8
    mov r4, sp
    .cfi_def_cfa_register r4
    .cfi_offset lr, -4
    .cfi_offset r4, -8
    mov sp, r3
    blx r2
    mov sp, r4
    .cfi_restore sp
    pop {r4, pc}
.rust_psm_on_stack_end:
SIZE(rust_psm_on_stack,.rust_psm_on_stack_end)
.cfi_endproc
CANTUNWIND
FNEND
