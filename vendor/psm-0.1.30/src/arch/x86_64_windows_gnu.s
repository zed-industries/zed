#include "gnu_stack_note.s"

.text

.def rust_psm_stack_direction
.scl 2
.type 32
.endef
.globl rust_psm_stack_direction
.p2align 4
rust_psm_stack_direction:
/* extern "sysv64" fn() -> u8 (%al) */
.cfi_startproc
    movb $2, %al # always descending on x86_64
    retq
.cfi_endproc

.def rust_psm_stack_pointer
.scl 2
.type 32
.endef
.globl rust_psm_stack_pointer
.p2align 4
rust_psm_stack_pointer:
/* extern "sysv64" fn() -> *mut u8 (%rax) */
.cfi_startproc
    leaq 8(%rsp), %rax
    retq
.cfi_endproc


.def rust_psm_replace_stack
.scl 2
.type 32
.endef
.globl rust_psm_replace_stack
.p2align 4
rust_psm_replace_stack:
/* extern "sysv64" fn(%rdi: usize, %rsi: extern "sysv64" fn(usize), %rdx: *mut u8, %rcx: *mut u8) */
.cfi_startproc
/*
    All we gotta do is set the stack pointer to %rdx & tail-call the callback in %rsi.

    8-byte offset necessary to account for the "return" pointer that would otherwise be placed onto
    stack with a regular call
*/
    movq %gs:0x08, %rdx
    movq %gs:0x10, %rcx
    leaq -8(%rdx), %rsp
    jmpq *%rsi
.cfi_endproc


.def rust_psm_on_stack
.scl 2
.type 32
.endef
.globl rust_psm_on_stack
.p2align 4
rust_psm_on_stack:
/*
extern "sysv64" fn(%rdi: usize, %rsi: usize,
                   %rdx: extern "sysv64" fn(usize, usize), %rcx: *mut u8, %r8: *mut u8)

NB: on Windows for SEH to work at all, the pointers in TIB, thread information block, need to be
fixed up. Otherwise, it seems that exception mechanism on Windows will not bother looking for
exception handlers at *all* if they happen to fall outside the are specified in TIB.

This necessitates an API difference from the usual 4-argument signature used elsewhere.

FIXME: this needs a catch-all exception handler that aborts in case somebody unwinds into here.
*/
.cfi_startproc
    pushq %rbp
    .cfi_def_cfa %rsp, 16
    .cfi_offset %rbp, -16
    pushq %gs:0x08
    .cfi_def_cfa %rsp, 24
    pushq %gs:0x10
    .cfi_def_cfa %rsp, 32

    movq  %rsp, %rbp
    .cfi_def_cfa_register %rbp
    movq %rcx, %gs:0x08
    movq %r8, %gs:0x10
    movq  %rcx, %rsp
    callq *%rdx
    movq  %rbp, %rsp

    popq %gs:0x10
    .cfi_def_cfa %rsp, 24
    popq %gs:0x08
    .cfi_def_cfa %rsp, 16
    popq  %rbp
    .cfi_def_cfa %rsp, 8
    retq
.cfi_endproc
