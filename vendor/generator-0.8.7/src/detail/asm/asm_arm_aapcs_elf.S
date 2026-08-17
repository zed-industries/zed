.text
.globl prefetch
.type prefetch, %function
.align 2
prefetch:
    pld [r0]
    bx lr
.size prefetch,.-prefetch

.text
.globl bootstrap_green_task
.type bootstrap_green_task, %function
.align 2
bootstrap_green_task:
     mov r0, r4 // arg0
     mov r1, r5 // arg1
     mov lr, 0 // clear LR
     bx r6
.size bootstrap_green_task,.-bootstrap_green_task

.text
.globl swap_registers
.type swap_registers, %function
.align 2
swap_registers:
     // Android doesn't like to use sp directly
     stmia r0!, {{v1-v7, fp}}
     mov r2, sp
     stmia r0!, {{r2, lr}}
     vstmia r0!, {{s16-s31}}
     ldmia r1!, {{v1-v7, fp}}
     ldmia r1!, {{r2, lr}}
     mov sp, r2
     vldmia r1!, {{s16-s31}}
     bx lr
.size swap_registers,.-swap_registers

/* Mark that we don't need executable stack. */
.section .note.GNU-stack,"",%progbits
