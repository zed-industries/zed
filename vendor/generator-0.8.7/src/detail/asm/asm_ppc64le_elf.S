// unfortunately the IBM assembler just uses numbers for registers
// making the assembly hard to read when registers are mixed with offsets.
// therefore are here some defines for readability:

#pragma region defines

#define r0   0
#define r1   1
#define fp   1
#define r2   2
#define r3   3
#define r4   4
#define r5   5
#define r6   6
#define r7   7
#define r8   8
#define r9   9
#define r10  10
#define r11  11
#define r12  12
#define r13  13
#define r14  14
#define r15  15
#define r16  16
#define r17  17
#define r18  18
#define r19  19
#define r20  20
#define r21  21
#define r22  22
#define r23  23
#define r24  24
#define r25  25
#define r26  26
#define r27  27
#define r28  28
#define r29  29
#define r30  30
#define r31  31

// floating-point registers
#define f14 14
#define f15 15
#define f16 16
#define f17 17
#define f18 18
#define f19 19
#define f20 20
#define f21 21
#define f22 22
#define f23 23
#define f24 24
#define f25 25
#define f26 26
#define f27 27
#define f28 28
#define f29 29
#define f30 30
#define f31 31

// vector registers
#define v20 20
#define v21 21
#define v22 22
#define v23 23
#define v24 24
#define v25 25
#define v26 26
#define v27 27
#define v28 28
#define v29 29
#define v30 30
#define v31 31

#pragma endregion defines

.text
.globl prefetch
.type prefetch,@function
.align 16
prefetch:
    addis 2,12,.TOC.-prefetch@ha
    addi 2,2,.TOC.-prefetch@l
    .localentry prefetch, .-prefetch
    // NOTE: dcbt prefetches data, not instructions!
    dcbt 0, r3
    blr
.size prefetch,.-prefetch


.text
.globl bootstrap_green_task
.type bootstrap_green_task,@function
.align 16
bootstrap_green_task:
    // setting parameters from loaded non-volatile regs
    mr r3, r14 
    mr r4, r15

    mr r12, r16 // setup entrypoint since  position independent code can asssume 
                // r12 to contain its GEP address (page 61 Power ABI)

    mtlr r16
    blr

.size bootstrap_green_task,.-bootstrap_green_task


.text
.globl swap_registers
.type swap_registers,@function
.align 16
swap_registers:
    // save non-volatile registers to the buffer in Registers (r3)
    // load non-volatile registers from new context buffer given via r4
    
    // standard function preamble:
    addis 2,12,.TOC.-swap_registers@ha
    addi 2,2,.TOC.-swap_registers@l
    .localentry swap_registers, .-swap_registers

    // save link & control registers
    mflr r0
    std r0,0(r3)
    std r0, 2*8(r1)
    mfcr r0
    std r0,1*8(r3)


    // non-volatile registers: r1 (fp), r2 (toc), r13, r14-r31, f14-f31, v20-v31, vrsave,
    // arguments passed in r3-r10, stack
    // => previous reg list (r3), new reg list (r4)

    // saving non-volatile gprs:
    std r1,   2*8(r3) // stack pointer
    std r2,   3*8(r3) // TOC pointer
    std r12, 4*8(r3)  // gloabl entrypoint address (GEP)
    std r14,  5*8(r3) // local vars
    std r15,  6*8(r3)
    std r16,  7*8(r3)
    std r17,  8*8(r3)
    std r18, 9*8(r3)
    std r19, 10*8(r3)
    std r20, 11*8(r3)
    std r21, 12*8(r3)
    std r22, 13*8(r3)
    std r23, 14*8(r3)
    std r24, 15*8(r3)
    std r25, 16*8(r3)
    std r26, 17*8(r3)
    std r27, 18*8(r3)
    std r28, 19*8(r3)
    std r29, 20*8(r3)
    std r30, 21*8(r3)
    std r31, 22*8(r3) // end local vars

    // save non-volatile floating point registers
    addi r3, r3, 32*8  // start of fp array
    stfd f14,  0*8(r3) // local vars (floating point)
    stfd f15,  1*8(r3)
    stfd f16,  2*8(r3)
    stfd f17,  3*8(r3)
    stfd f18,  4*8(r3)
    stfd f19,  5*8(r3)
    stfd f20,  6*8(r3)
    stfd f21,  7*8(r3)
    stfd f22,  8*8(r3)
    stfd f23,  9*8(r3)
    stfd f24, 10*8(r3)
    stfd f25, 11*8(r3)
    stfd f26, 12*8(r3)
    stfd f27, 13*8(r3)
    stfd f28, 14*8(r3)
    stfd f29, 15*8(r3)
    stfd f30, 16*8(r3)
    stfd f31, 17*8(r3) // end local  vars (fp)

    // and finally the vector registers
    addi r3, r3, 18*8 // start of vr area
    li r6, 0
 
    stvx v20, r3, r6 // start of vr saving
    addi r6, r6, 16
    stvx v21, r3, r6
    addi r6, r6, 16
    stvx v22, r3, r6
    addi r6, r6, 16
    stvx v23, r3, r6
    addi r6, r6, 16
    stvx v24, r3, r6
    addi r6, r6, 16
    stvx v25, r3, r6
    addi r6, r6, 16
    stvx v26, r3, r6
    addi r6, r6, 16
    stvx v27, r3, r6
    addi r6, r6, 16
    stvx v28, r3, r6
    addi r6, r6, 16
    stvx v29, r3, r6
    addi r6, r6, 16
    stvx v30, r3, r6
    addi r6, r6, 16
    stvx v31, r3, r6 // end of vr saving

    // begin restoration

    // restore floating point registers
    mr r5, r4
    addi r4, r4, 32*8  // start of fp array
    lfd f14,  0*8(r4) // start of fp restore
    lfd f15,  1*8(r4)
    lfd f16,  2*8(r4)
    lfd f17,  3*8(r4)
    lfd f18,  4*8(r4)
    lfd f19,  5*8(r4)
    lfd f20,  6*8(r4)
    lfd f21,  7*8(r4)
    lfd f22,  8*8(r4)
    lfd f23,  9*8(r4)
    lfd f24, 10*8(r4)
    lfd f25, 11*8(r4)
    lfd f26, 12*8(r4)
    lfd f27, 13*8(r4)
    lfd f28, 14*8(r4)
    lfd f29, 15*8(r4)
    lfd f30, 16*8(r4)
    lfd f31, 17*8(r4) // end of fp restore

    // restore vector registers
    addi r4, r4, 18*8 // start of vr array
    li r6, 0

    lvx v20, r4, r6 // start of vr restore
    addi r6, r6, 16
    lvx v21, r4, r6
    addi r6, r6, 16
    lvx v22, r4, r6
    addi r6, r6, 16
    lvx v23, r4, r6
    addi r6, r6, 16
    lvx v24, r4, r6
    addi r6, r6, 16
    lvx v25, r4, r6
    addi r6, r6, 16
    lvx v26, r4, r6
    addi r6, r6, 16
    lvx v27, r4, r6
    addi r6, r6, 16
    lvx v28, r4, r6
    addi r6, r6, 16
    lvx v29, r4, r6
    addi r6, r6, 16
    lvx v30, r4, r6
    addi r6, r6, 16
    lvx v31, r4, r6 // end of vr restore

    // restore gpr registers
    mr r4, r5          
    ld r1,   2*8(r4) // start of gpr restore
    ld r2,   3*8(r4)
    ld r12, 4*8(r4)
    ld r14,  5*8(r4)
    ld r15,  6*8(r4)
    ld r16,  7*8(r4)
    ld r17,  8*8(r4)
    ld r18, 9*8(r4)
    ld r19, 10*8(r4)
    ld r20, 11*8(r4)
    ld r21, 12*8(r4)
    ld r22, 13*8(r4)
    ld r23, 14*8(r4)
    ld r24, 15*8(r4)
    ld r25, 16*8(r4)
    ld r26, 17*8(r4)
    ld r27, 18*8(r4)
    ld r28, 19*8(r4)
    ld r29, 20*8(r4)
    ld r30, 21*8(r4)
    ld r31, 22*8(r4) // end of gpr restore

    // restore/load lr and ctr registers
    ld r0, 8(r4)
    mtcr r0
    ld r0, 0(r4) // new link register
    mtlr r0

    blr

.size swap_registers,.-swap_registers

/* Mark that we don't need executable stack. */
.section .note.GNU-stack,"",%progbits