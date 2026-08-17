.text
.globl prefetch
.type prefetch,@function
.align 16
prefetch:
    prefetcht2 [rdi]
    ret
.size prefetch,.-prefetch

.text
.globl bootstrap_green_task
.type bootstrap_green_task,@function
.align 16
bootstrap_green_task:
    mov rdi, r12     /* setup the function arg */
    mov rsi, r13     /* setup the function arg */
    and rsp, -16     /* align the stack pointer */
    mov [rsp], r14   /* this is the new return adrress */
    ret
.size bootstrap_green_task,.-bootstrap_green_task

.text
.globl swap_registers
.type swap_registers,@function
.align 16
swap_registers:
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
    pop rax
    jmp rax

.size swap_registers,.-swap_registers

/* Mark that we don't need executable stack. */
.section .note.GNU-stack,"",%progbits
