.text
.globl prefetch
.type prefetch,@function
.align 2
prefetch:
    preld 0, $a0, 0
    ret
.size prefetch,.-prefetch

.text
.globl bootstrap_green_task
.type bootstrap_green_task,@function
.align 2
bootstrap_green_task:
    move $a0, $s0   // arg0
    move $a1, $s1   // arg1
    move $ra, $zero // clear LR
    jirl $zero, $s2, 0
.size bootstrap_green_task,.-bootstrap_green_task

.text
.globl swap_registers
.type swap_registers,@function
.align 2
swap_registers:
    st.d $ra, $a0, 0
    st.d $sp, $a0, 8
    st.d $fp, $a0, 16
    st.d $s0, $a0, 24
    st.d $s1, $a0, 32
    st.d $s2, $a0, 40
    st.d $s3, $a0, 48
    st.d $s4, $a0, 56
    st.d $s5, $a0, 64
    st.d $s6, $a0, 72
    st.d $s7, $a0, 80
    st.d $s8, $a0, 88

    fst.d $fs0, $a0, 96
    fst.d $fs1, $a0, 104
    fst.d $fs2, $a0, 112
    fst.d $fs3, $a0, 120
    fst.d $fs4, $a0, 128
    fst.d $fs5, $a0, 136
    fst.d $fs6, $a0, 144
    fst.d $fs7, $a0, 152

    ld.d $ra, $a1, 0
    ld.d $sp, $a1, 8
    ld.d $fp, $a1, 16
    ld.d $s0, $a1, 24
    ld.d $s1, $a1, 32
    ld.d $s2, $a1, 40
    ld.d $s3, $a1, 48
    ld.d $s4, $a1, 56
    ld.d $s5, $a1, 64
    ld.d $s6, $a1, 72
    ld.d $s7, $a1, 80
    ld.d $s8, $a1, 88

    fld.d $fs0, $a1, 96
    fld.d $fs1, $a1, 104
    fld.d $fs2, $a1, 112
    fld.d $fs3, $a1, 120
    fld.d $fs4, $a1, 128
    fld.d $fs5, $a1, 136
    fld.d $fs6, $a1, 144
    fld.d $fs7, $a1, 152

    ret
.size swap_registers,.-swap_registers

/* Mark that we don't need executable stack. */
.section .note.GNU-stack,"",%progbits
