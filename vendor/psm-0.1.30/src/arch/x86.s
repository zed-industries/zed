#include "psm.h"
#include "gnu_stack_note.s"
/* NOTE: fastcall calling convention used on all x86 targets */

.text

#if defined(CFG_TARGET_OS_darwin) || defined(CFG_TARGET_OS_macos) || defined(CFG_TARGET_OS_ios)

#define GLOBL(fnname) .globl _##fnname
#define TYPE(fnname)
#define FUNCTION(fnname) _##fnname
#define SIZE(fnname,endlabel)

#else

#define GLOBL(fnname) .globl fnname
#define TYPE(fnname) .type fnname,@function
#define FUNCTION(fnname) fnname
#define SIZE(fnname,endlabel) .size fnname,endlabel-fnname

#endif


GLOBL(rust_psm_stack_direction)
.p2align 4
TYPE(rust_psm_stack_direction)
FUNCTION(rust_psm_stack_direction):
/* extern "fastcall" fn() -> u8 (%al) */
.cfi_startproc
    movb $STACK_DIRECTION_DESCENDING, %al # always descending on x86_64
    retl
.rust_psm_stack_direction_end:
SIZE(rust_psm_stack_direction,.rust_psm_stack_direction_end)
.cfi_endproc


GLOBL(rust_psm_stack_pointer)
.p2align 4
TYPE(rust_psm_stack_pointer)
FUNCTION(rust_psm_stack_pointer):
/* extern "fastcall" fn() -> *mut u8 (%rax) */
.cfi_startproc
    leal 4(%esp), %eax
    retl
.rust_psm_stack_pointer_end:
SIZE(rust_psm_stack_pointer,.rust_psm_stack_pointer_end)
.cfi_endproc


GLOBL(rust_psm_replace_stack)
.p2align 4
TYPE(rust_psm_replace_stack)
FUNCTION(rust_psm_replace_stack):
/* extern "fastcall" fn(%ecx: usize, %edx: extern "fastcall" fn(usize), 4(%esp): *mut u8) */
.cfi_startproc
/*
   All we gotta do is set the stack pointer to 4(%esp) & tail-call the callback in %edx

   Note, that the callee expects the stack to be offset by 4 bytes (normally, a return address
   would be store there) off the required stack alignment on entry. To offset the stack in such a
   way we use the `calll` instruction, however it would also be possible to to use plain `jmpl` but
   would require to adjust the stack manually, which cannot be easily done, because the stack
   pointer argument is already stored in memory.
 */
    movl 4(%esp), %esp
    calll *%edx
    ud2
.rust_psm_replace_stack_end:
SIZE(rust_psm_replace_stack,.rust_psm_replace_stack_end)
.cfi_endproc


GLOBL(rust_psm_on_stack)
.p2align 4
TYPE(rust_psm_on_stack)
FUNCTION(rust_psm_on_stack):
/* extern "fastcall" fn(%ecx: usize, %edx: usize, 4(%esp): extern "fastcall" fn(usize, usize), 8(%esp): *mut u8) */
.cfi_startproc
    pushl %ebp
    .cfi_def_cfa %esp, 8
    .cfi_offset %ebp, -8
    movl  %esp, %ebp
    .cfi_def_cfa_register %ebp
    movl  12(%ebp), %esp
    calll *8(%ebp)
    movl  %ebp, %esp
    popl  %ebp
    .cfi_def_cfa %esp, 4
    retl  $8
.rust_psm_on_stack_end:
SIZE(rust_psm_on_stack,.rust_psm_on_stack_end)
.cfi_endproc
