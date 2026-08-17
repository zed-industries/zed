.file	"asm_x86_64_sysv_pe.S"
.text
.p2align 4,,15
.globl	prefetch_asm
.def	prefetch_asm;	.scl	2;	.type	32;	.endef
.seh_proc	prefetch_asm
prefetch_asm:
.seh_endprologue
    prefetcht2 [rdi]
    ret
.seh_endproc

.section .drectve
.ascii " -export:\"prefetch_asm\""

.text
.p2align 4,,15
.globl	bootstrap_green_task
.def	bootstrap_green_task;	.scl	2;	.type	32;	.endef
.seh_proc	bootstrap_green_task
bootstrap_green_task:
.seh_endprologue
    mov rdi, r12     /* setup the function arg */
    mov rsi, r13     /* setup the function arg */
    and rsp, -16     /* align the stack pointer */
    mov [rsp], r14   /* this is the new return adrress */
    ret
.seh_endproc

.section .drectve
.ascii " -export:\"bootstrap_green_task\""

.text
.p2align 4,,15
.globl	swap_registers
.def	swap_registers;	.scl	2;	.type	32;	.endef
.seh_proc	swap_registers
swap_registers:
.seh_endprologue
    mov [rdi + 0*8], rbx
    mov [rdi + 1*8], rsp
    mov [rdi + 2*8], rbp
    mov [rdi + 4*8], r12
    mov [rdi + 5*8], r13
    mov [rdi + 6*8], r14
    mov [rdi + 7*8], r15
    mov rbx, [rsi + 0*8]
    mov rsp, [rsi + 1*8]
    mov rbp, [rsi + 2*8]
    mov r12, [rsi + 4*8]
    mov r13, [rsi + 5*8]
    mov r14, [rsi + 6*8]
    mov r15, [rsi + 7*8]

    /* load NT_TIB */
    mov r10, gs:[0x30]
    /* save current stack base */
    mov rax, [r10 + 0x08]
    mov [rdi + 11*8], rax
    /* save current stack limit */
    mov rax,  [r10 + 0x10]
    mov [rdi + 12*8], rax
    /* save current deallocation stack */
    mov rax, [r10 + 0x1478]
    mov [rdi + 13*8], rax
    /* save fiber local storage */
    /* movq  0x18(%r10), %rax */
    /* mov  %rax, (14*8)(%rcx) */

    /* mov %rcx, (3*8)(%rcx) */

    /* restore fiber local storage */
    /* mov (14*8)(%rdx), %rax */
    /* movq  %rax, 0x18(%r10) */
    /* restore deallocation stack */
    mov rax, [rsi + 13*8]
    mov [r10 + 0x1478], rax
    /* restore stack limit */
    mov rax, [rsi + 12*8]
    mov [r10 + 0x10], rax
    /* restore stack base */
    mov rax, [rsi + 11*8]
    mov [r10 + 0x08], rax

    /* mov (3*8)(%rdx), %rcx */

    pop rax
    jmp rax
.seh_endproc

.section .drectve
.ascii " -export:\"swap_registers\""
