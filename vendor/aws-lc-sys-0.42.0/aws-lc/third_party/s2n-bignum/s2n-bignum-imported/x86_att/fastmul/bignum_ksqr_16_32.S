// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[16]; output z[32]; temporary buffer t[>=24]
//
//    extern void bignum_ksqr_16_32(uint64_t z[static 32],
//                                  const uint64_t x[static 16],
//                                  uint64_t t[static 24]);
//
// In this x86 code the final temporary space argument t is unused, but
// it is retained in the prototype above for API consistency with ARM.
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = t
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ksqr_16_32)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ksqr_16_32)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ksqr_16_32)
        .text

#define z %rdi
#define x %rsi

// A zero register

#define zero %rbp
#define zeroe %ebp

// ------------------------------------------------------------------------
// mulpadd i, j adds rdx * x[i] into the window  at the i+j point
// ------------------------------------------------------------------------

.macro mulpadd arg1,arg2
        mulxq   8*\arg1(x), %rax, %rcx
.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   %rax, %r12
        adoxq   %rcx, %r13
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   %rax, %r15
        adoxq   %rcx, %r8
.endif

.endm

// ------------------------------------------------------------------------
// mulpade i, j adds rdx * x[i] into the window at i+j
// but re-creates the top word assuming nothing to add there
// ------------------------------------------------------------------------

.macro mulpade arg1,arg2
.if ((\arg1 + \arg2) % 8 == 0)
        mulxq   8*\arg1(x), %rax, %r9
        adcxq   %rax, %r8
        adoxq   zero, %r9
.elseif ((\arg1 + \arg2) % 8 == 1)
        mulxq   8*\arg1(x), %rax, %r10
        adcxq   %rax, %r9
        adoxq   zero, %r10
.elseif ((\arg1 + \arg2) % 8 == 2)
        mulxq   8*\arg1(x), %rax, %r11
        adcxq   %rax, %r10
        adoxq   zero, %r11
.elseif ((\arg1 + \arg2) % 8 == 3)
        mulxq   8*\arg1(x), %rax, %r12
        adcxq   %rax, %r11
        adoxq   zero, %r12
.elseif ((\arg1 + \arg2) % 8 == 4)
        mulxq   8*\arg1(x), %rax, %r13
        adcxq   %rax, %r12
        adoxq   zero, %r13
.elseif ((\arg1 + \arg2) % 8 == 5)
        mulxq   8*\arg1(x), %rax, %r14
        adcxq   %rax, %r13
        adoxq   zero, %r14
.elseif ((\arg1 + \arg2) % 8 == 6)
        mulxq   8*\arg1(x), %rax, %r15
        adcxq   %rax, %r14
        adoxq   zero, %r15
.elseif ((\arg1 + \arg2) % 8 == 7)
        mulxq   8*\arg1(x), %rax, %r8
        adcxq   %rax, %r15
        adoxq   zero, %r8
.endif

.endm

// ------------------------------------------------------------------------
// addrow i,j adds z[i+j] + x[i..i+7] * x[j] into the window
// ------------------------------------------------------------------------

.macro addrow arg1,arg2
        movq    8*\arg2(x), %rdx
        xorl    zeroe, zeroe // Get a known flag state and give a zero reg

.if ((\arg1 + \arg2) % 8 == 0)
        adoxq   8*(\arg1+\arg2)(z), %r8
.elseif ((\arg1 + \arg2) % 8 == 1)
        adoxq   8*(\arg1+\arg2)(z), %r9
.elseif ((\arg1 + \arg2) % 8 == 2)
        adoxq   8*(\arg1+\arg2)(z), %r10
.elseif ((\arg1 + \arg2) % 8 == 3)
        adoxq   8*(\arg1+\arg2)(z), %r11
.elseif ((\arg1 + \arg2) % 8 == 4)
        adoxq   8*(\arg1+\arg2)(z), %r12
.elseif ((\arg1 + \arg2) % 8 == 5)
        adoxq   8*(\arg1+\arg2)(z), %r13
.elseif ((\arg1 + \arg2) % 8 == 6)
        adoxq   8*(\arg1+\arg2)(z), %r14
.elseif ((\arg1 + \arg2) % 8 == 7)
        adoxq   8*(\arg1+\arg2)(z), %r15
.endif

        mulpadd \arg1, \arg2

.if ((\arg1 + \arg2) % 8 == 0)
        movq    %r8, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 1)
        movq    %r9, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 2)
        movq    %r10, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 3)
        movq    %r11, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 4)
        movq    %r12, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 5)
        movq    %r13, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 6)
        movq    %r14, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 7)
        movq    %r15, 8*(\arg1+\arg2)(z)
.endif

        mulpadd (\arg1+1), \arg2
        mulpadd (\arg1+2), \arg2
        mulpadd (\arg1+3), \arg2
        mulpadd (\arg1+4), \arg2
        mulpadd (\arg1+5), \arg2
        mulpade (\arg1+6), \arg2
        mulpade (\arg1+7), \arg2

.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   zero, %r8
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   zero, %r9
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   zero, %r10
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   zero, %r11
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   zero, %r12
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   zero, %r13
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   zero, %r14
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   zero, %r15
.endif


.endm


// ------------------------------------------------------------------------
// Adds off-diagonal part of x[i..i+7]^2 into the window, writes 0..7 back
// ------------------------------------------------------------------------

.macro sqr arg1

        xorl    zeroe, zeroe

// Set up the initial window

        movq    16*\arg1+8(z), %r9
        movq    16*\arg1+16(z), %r10
        movq    16*\arg1+24(z), %r11
        movq    16*\arg1+32(z), %r12
        movq    16*\arg1+40(z), %r13
        movq    16*\arg1+48(z), %r14
        movq    16*\arg1+56(z), %r15

// Add in the first diagonal [%r8..%r10] + 2 wb = 10 + 20 + 30 + 40 + 50 + 60 + 70

        movq    8*\arg1(x), %rdx
        mulpadd (\arg1+1), (\arg1+0)
        movq    %r9, 16*\arg1+8(z)
        mulpadd (\arg1+2), (\arg1+0)
        movq    %r10, 16*\arg1+16(z)
        mulpadd (\arg1+3), (\arg1+0)
        mulpadd (\arg1+4), (\arg1+0)
        mulpadd (\arg1+5), (\arg1+0)
        mulpadd (\arg1+6), (\arg1+0)
        mulpade (\arg1+7), (\arg1+0)
        adcxq   zero, %r8

// Add in the next diagonal = 21 + 31 + 41 + 51 + 61 + 71 + 54

        xorl    zeroe, zeroe
        movq    8*\arg1+8(x), %rdx
        mulpadd (\arg1+2), (\arg1+1)
        movq    %r11, 16*\arg1+24(z)
        mulpadd (\arg1+3), (\arg1+1)
        movq    %r12, 16*\arg1+32(z)
        mulpadd (\arg1+4), (\arg1+1)
        mulpadd (\arg1+5), (\arg1+1)
        mulpadd (\arg1+6), (\arg1+1)
        mulpade (\arg1+7), (\arg1+1)
        movq    8*\arg1+32(x), %rdx
        mulpade (\arg1+5), (\arg1+4)
        adcxq   zero, %r10

// And the next one = 32 + 42 + 52 + 62 + 72 + 64 + 65

        xorl    zeroe, zeroe
        movq    8*\arg1+16(x), %rdx
        mulpadd (\arg1+3), (\arg1+2)
        movq    %r13, 16*\arg1+40(z)
        mulpadd (\arg1+4), (\arg1+2)
        movq    %r14, 16*\arg1+48(z)
        mulpadd (\arg1+5), (\arg1+2)
        mulpadd (\arg1+6), (\arg1+2)
        mulpadd (\arg1+7), (\arg1+2)
        movq    8*\arg1+48(x), %rdx
        mulpade (\arg1+4), (\arg1+6)
        mulpade (\arg1+5), (\arg1+6)
        adcxq   zero, %r12

// And the final one = 43 + 53 + 63 + 73 + 74 + 75 + 76

        xorl    zeroe, zeroe
        movq    8*\arg1+24(x), %rdx
        mulpadd (\arg1+4), (\arg1+3)
        movq    %r15, 16*\arg1+56(z)
        mulpadd (\arg1+5), (\arg1+3)
        mulpadd (\arg1+6), (\arg1+3)
        mulpadd (\arg1+7), (\arg1+3)
        movq    8*\arg1+56(x), %rdx
        mulpadd (\arg1+4), (\arg1+7)
        mulpade (\arg1+5), (\arg1+7)
        mulpade (\arg1+6), (\arg1+7)
        adcxq   zero, %r14
.endm

// ------------------------------------------------------------------------
// Multiply-add: z := z + x[i...i+7] * x
// ------------------------------------------------------------------------

.macro addrows arg1

        sqr \arg1

        .set I,  (\arg1+8)
.rep (8-\arg1)
        addrow \arg1, I
        .set I,  (I+1)
.endr

        movq    %r8, 8*(16+\arg1)(z)
        movq    %r9, 8*(17+\arg1)(z)
        movq    %r10, 8*(18+\arg1)(z)
        movq    %r11, 8*(19+\arg1)(z)
        movq    %r12, 8*(20+\arg1)(z)
        movq    %r13, 8*(21+\arg1)(z)
        movq    %r14, 8*(22+\arg1)(z)
.endm


// ------------------------------------------------------------------------
// mulrow i,j adds x[i..i+7] * x[j] into the window
// just like addrow but no addition of z[i+j]
// ------------------------------------------------------------------------

.macro mulrow arg1,arg2
        movq    8*\arg2(x), %rdx
        xorl    zeroe, zeroe // Get a known flag state and give a zero reg

        mulpadd \arg1, \arg2

.if ((\arg1 + \arg2) % 8 == 0)
        movq    %r8, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 1)
        movq    %r9, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 2)
        movq    %r10, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 3)
        movq    %r11, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 4)
        movq    %r12, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 5)
        movq    %r13, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 6)
        movq    %r14, 8*(\arg1+\arg2)(z)
.elseif ((\arg1 + \arg2) % 8 == 7)
        movq    %r15, 8*(\arg1+\arg2)(z)
.endif

        mulpadd (\arg1+1), \arg2
        mulpadd (\arg1+2), \arg2
        mulpadd (\arg1+3), \arg2
        mulpadd (\arg1+4), \arg2
        mulpadd (\arg1+5), \arg2
.if ((\arg1 + \arg2) % 8 == 0)
        mulpade (\arg1+6), \arg2
.else
        mulpadd (\arg1+6), \arg2
.endif

        mulpade (\arg1+7), \arg2

.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   zero, %r8
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   zero, %r9
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   zero, %r10
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   zero, %r11
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   zero, %r12
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   zero, %r13
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   zero, %r14
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   zero, %r15
.endif


.endm

// ------------------------------------------------------------------------
// Compute off-diagonal part of x[0..7]^2, write back 1..7 elements and
// set up the high part in the standard register window. DOES NOT WRITE z[0]!
// ------------------------------------------------------------------------

.macro sqrz

        xorl    zeroe, zeroe

// Set initial window [%r8..%r10] + 2 wb = 10 + 20 + 30 + 40 + 50 + 60 + 70

        movq    (x), %rdx
        mulxq   8(x), %r9, %rax
        movq    %r9, 8(z)
        mulxq   16(x), %r10, %rcx
        adcxq   %rax, %r10
        movq    %r10, 16(z)
        mulxq   24(x), %r11, %rax
        adcxq   %rcx, %r11
        mulxq   32(x), %r12, %rcx
        adcxq   %rax, %r12
        mulxq   40(x), %r13, %rax
        adcxq   %rcx, %r13
        mulxq   48(x), %r14, %rcx
        adcxq   %rax, %r14
        mulxq   56(x), %r15, %r8
        adcxq   %rcx, %r15
        adcxq   zero, %r8

// Add in the next diagonal = 21 + 31 + 41 + 51 + 61 + 71 + 54

        xorl    zeroe, zeroe
        movq    8(x), %rdx
        mulpadd 2, 1
        movq    %r11, 24(z)
        mulpadd 3, 1
        movq    %r12, 32(z)
        mulpadd 4, 1
        mulpadd 5, 1
        mulpadd 6, 1
        mulpade 7, 1
        movq    32(x), %rdx
        mulpade 5, 4
        adcxq   zero, %r10

// And the next one = 32 + 42 + 52 + 62 + 72 + 64 + 65

        xorl    zeroe, zeroe
        movq    16(x), %rdx
        mulpadd 3, 2
        movq    %r13, 40(z)
        mulpadd 4, 2
        movq    %r14, 48(z)
        mulpadd 5, 2
        mulpadd 6, 2
        mulpadd 7, 2
        movq    48(x), %rdx
        mulpade 4, 6
        mulpade 5, 6
        adcxq   zero, %r12

// And the final one = 43 + 53 + 63 + 73 + 74 + 75 + 76

        xorl    zeroe, zeroe
        movq    24(x), %rdx
        mulpadd 4, 3
        movq    %r15, 56(z)
        mulpadd 5, 3
        mulpadd 6, 3
        mulpadd 7, 3
        movq    56(x), %rdx
        mulpadd 4, 7
        mulpade 5, 7
        mulpade 6, 7
        adcxq   zero, %r14
.endm

// ------------------------------------------------------------------------
// Multiply-add: z := x[0...7] * x off-diagonal elements
// ------------------------------------------------------------------------

.macro mulrows
        sqrz

        .set I,  8
.rep 8
        mulrow 0, I
        .set I,  (I+1)
.endr

        movq    %r8, 128(z)
        movq    %r9, 136(z)
        movq    %r10, 144(z)
        movq    %r11, 152(z)
        movq    %r12, 160(z)
        movq    %r13, 168(z)
        movq    %r14, 176(z)
        movq    %r15, 184(z)
.endm

// ------------------------------------------------------------------------
// The actual code
// ------------------------------------------------------------------------



S2N_BN_SYMBOL(bignum_ksqr_16_32):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Now just systematically add in the rows to get all off-diagonal elements

        mulrows
        addrows 8

// Double and add the diagonal elements. Note that z[0] was never written above

        xorl    zeroe, zeroe
        movq    (x), %rdx
        mulxq   %rdx, %rax, %rcx
        movq    %rax, (z)

        movq    8(z), %rdx
        adcxq   %rdx, %rdx
        adoxq   %rcx, %rdx
        movq    %rdx, 8(z)

        .set I,  1
.rep 14
        movq    8*I(x), %rdx
        mulxq   %rdx, %rax, %rcx

        movq    8*(2*I)(z), %rdx
        adcxq   %rdx, %rdx
        adoxq   %rax, %rdx
        movq    %rdx, 8*(2*I)(z)

        movq    8*(2*I+1)(z), %rdx
        adcxq   %rdx, %rdx
        adoxq   %rcx, %rdx
        movq    %rdx, 8*(2*I+1)(z)
        .set I,  (I+1)
.endr

        movq    8*I(x), %rdx
        mulxq   %rdx, %rax, %rcx

        movq    8*(2*I)(z), %rdx
        adcxq   %rdx, %rdx
        adoxq   %rax, %rdx
        movq    %rdx, 8*(2*I)(z)

        adcxq   zero, %rcx
        adoxq   zero, %rcx
        movq    %rcx, 8*(2*I+1)(z)
        .set I,  (I+1)


// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_ksqr_16_32)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
