.attribute arch, "rv64gc"

.text
.globl prefetch
.type prefetch,@function
.align 1
prefetch:
    ret
.size prefetch,.-prefetch

.text
.globl bootstrap_green_task
.type bootstrap_green_task,@function
.align 1
bootstrap_green_task:
     mv a0, s2   // arg0
     mv a1, s3   // arg1
     mv ra, zero // clear RA
     jr s4
.size bootstrap_green_task,.-bootstrap_green_task

.text
.globl swap_registers
.type swap_registers,@function
.align 1
swap_registers:
     sd s2, 0*8(a0)
     sd s3, 1*8(a0)
     sd s4, 2*8(a0)
     sd s5, 3*8(a0)
     sd s6, 4*8(a0)
     sd s7, 5*8(a0)
     sd s8, 6*8(a0)
     sd s9, 7*8(a0)
     sd s10, 8*8(a0)
     sd s11, 9*8(a0)
     sd fp, 10*8(a0)
     sd s1, 11*8(a0)
     sd sp, 12*8(a0)
     sd ra, 13*8(a0)
     fsd fs0, 14*8(a0)
     fsd fs1, 15*8(a0)
     fsd fs2, 16*8(a0)
     fsd fs3, 17*8(a0)
     fsd fs4, 18*8(a0)
     fsd fs5, 19*8(a0)
     fsd fs6, 20*8(a0)
     fsd fs7, 21*8(a0)
     fsd fs8, 22*8(a0)
     fsd fs9, 23*8(a0)
     fsd fs10, 24*8(a0)
     fsd fs11, 25*8(a0)
     ld s2, 0*8(a1)
     ld s3, 1*8(a1)
     ld s4, 2*8(a1)
     ld s5, 3*8(a1)
     ld s6, 4*8(a1)
     ld s7, 5*8(a1)
     ld s8, 6*8(a1)
     ld s9, 7*8(a1)
     ld s10, 8*8(a1)
     ld s11, 9*8(a1)
     ld fp, 10*8(a1)
     ld s1, 11*8(a1)
     ld sp, 12*8(a1)
     ld ra, 13*8(a1)
     fld fs0, 14*8(a1)
     fld fs1, 15*8(a1)
     fld fs2, 16*8(a1)
     fld fs3, 17*8(a1)
     fld fs4, 18*8(a1)
     fld fs5, 19*8(a1)
     fld fs6, 20*8(a1)
     fld fs7, 21*8(a1)
     fld fs8, 22*8(a1)
     fld fs9, 23*8(a1)
     fld fs10, 24*8(a1)
     fld fs11, 25*8(a1)
     jr ra
.size swap_registers,.-swap_registers

/* Mark that we don't need executable stack. */
.section .note.GNU-stack,"",%progbits
