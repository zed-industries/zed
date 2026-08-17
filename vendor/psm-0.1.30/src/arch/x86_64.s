#include "psm.h"
#include "gnu_stack_note.s"
/* NOTE: sysv64 calling convention is used on all x86_64 targets, including Windows! */

.text

#if defined(CFG_TARGET_OS_darwin) || defined(CFG_TARGET_OS_macos) || defined(CFG_TARGET_OS_ios)

#define GLOBL(fnname) .globl _##fnname
#define TYPE(fnname)
#define FUNCTION(fnname) _##fnname
#define END_FUNCTION(fnname)

#else

#define GLOBL(fnname) .globl fnname
#define TYPE(fnname) .type fnname,@function
#define FUNCTION(fnname) fnname
#define END_FUNCTION(fnname) .size fnname,.-fnname

#endif


GLOBL(rust_psm_stack_direction)
.p2align 4
TYPE(rust_psm_stack_direction)
FUNCTION(rust_psm_stack_direction):
/* extern "sysv64" fn() -> u8 (%al) */
.cfi_startproc
    movb $STACK_DIRECTION_DESCENDING, %al # always descending on x86_64
    retq
END_FUNCTION(rust_psm_stack_direction)
.cfi_endproc


GLOBL(rust_psm_stack_pointer)
.p2align 4
TYPE(rust_psm_stack_pointer)
FUNCTION(rust_psm_stack_pointer):
/* extern "sysv64" fn() -> *mut u8 (%rax) */
.cfi_startproc
    leaq 8(%rsp), %rax
    retq
.rust_psm_stack_pointer_end:
END_FUNCTION(rust_psm_stack_pointer)
.cfi_endproc


GLOBL(rust_psm_replace_stack)
.p2align 4
TYPE(rust_psm_replace_stack)
FUNCTION(rust_psm_replace_stack):
/* extern "sysv64" fn(%rdi: usize, %rsi: extern "sysv64" fn(usize), %rdx: *mut u8) */
.cfi_startproc
/*
    All we gotta do is set the stack pointer to %rdx & tail-call the callback in %rsi.

    8-byte offset necessary to account for the "return" pointer that would otherwise be placed onto
    stack with a regular call
*/
    leaq -8(%rdx), %rsp
    jmpq *%rsi
.rust_psm_replace_stack_end:
END_FUNCTION(rust_psm_replace_stack)
.cfi_endproc


GLOBL(rust_psm_on_stack)
.p2align 4
TYPE(rust_psm_on_stack)
FUNCTION(rust_psm_on_stack):
/* extern "sysv64" fn(%rdi: usize, %rsi: usize, %rdx: extern "sysv64" fn(usize, usize), %rcx: *mut u8) */
.cfi_startproc
    pushq %rbp
    .cfi_def_cfa %rsp, 16
    .cfi_offset %rbp, -16
    movq  %rsp, %rbp
    .cfi_def_cfa_register %rbp
    movq  %rcx, %rsp
    callq *%rdx
    movq  %rbp, %rsp
    popq  %rbp
    .cfi_def_cfa %rsp, 8
    retq
END_FUNCTION(rust_psm_on_stack)
.cfi_endproc
