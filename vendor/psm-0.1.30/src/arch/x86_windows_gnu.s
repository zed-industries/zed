/* FIXME: this works locally but not on appveyor??!? */
/* NOTE: fastcall calling convention used on all x86 targets */
.text

.def @rust_psm_stack_direction@0
.scl 2
.type 32
.endef
.globl @rust_psm_stack_direction@0
.p2align 4
@rust_psm_stack_direction@0:
/* extern "fastcall" fn() -> u8 (%al) */
.cfi_startproc
    movb $2, %al # always descending on x86_64
    retl
.cfi_endproc


.def @rust_psm_stack_pointer@0
.scl 2
.type 32
.endef
.globl @rust_psm_stack_pointer@0
.p2align 4
@rust_psm_stack_pointer@0:
/* extern "fastcall" fn() -> *mut u8 (%rax) */
.cfi_startproc
    leal 4(%esp), %eax
    retl
.cfi_endproc


.def @rust_psm_replace_stack@16
.scl 2
.type 32
.endef
.globl @rust_psm_replace_stack@16
.p2align 4
@rust_psm_replace_stack@16:
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
    movl 8(%esp), %eax
    mov %eax, %fs:0x08
    movl 4(%esp), %esp
    mov %esp, %fs:0x04
    calll *%edx
    ud2
.cfi_endproc


.def @rust_psm_on_stack@16
.scl 2
.type 32
.endef
.globl @rust_psm_on_stack@16
.p2align 4
@rust_psm_on_stack@16:
/* extern "fastcall" fn(%ecx: usize, %edx: usize, 4(%esp): extern "fastcall" fn(usize, usize), 8(%esp): *mut u8) */
.cfi_startproc
    pushl %ebp
    .cfi_def_cfa %esp, 8
    .cfi_offset %ebp, -8
    pushl %fs:0x04
    .cfi_def_cfa %esp, 12
    pushl %fs:0x08
    .cfi_def_cfa %esp, 16
    movl  %esp, %ebp
    .cfi_def_cfa_register %ebp

    movl 24(%ebp), %eax
    movl %eax, %fs:0x08
    movl  20(%ebp), %esp
    movl %esp, %fs:0x04
    calll *16(%ebp)

    movl  %ebp, %esp
    popl %fs:0x08
    .cfi_def_cfa %esp, 12
    popl %fs:0x04
    .cfi_def_cfa %esp, 8
    popl  %ebp
    .cfi_def_cfa %esp, 4
    retl  $12
.cfi_endproc

