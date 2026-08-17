// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[8]; output z[16]
//
//    extern void bignum_sqr_8_16(uint64_t z[static 16], const uint64_t x[static 8]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_8_16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_8_16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_8_16)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// A zero register

#define zero %rbp
#define zeroe %ebp

// mulpadd i, j adds rdx * x[i] into the window  at the i+j point

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

// mulpade i, j adds rdx * x[i] into the window at i+j
// but re-creates the top word assuming nothing to add there

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

.macro diagonals

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
        movq    %r8, 64(z)
        mulpadd 6, 3
        mulpadd 7, 3
        movq    56(x), %rdx
        mulpadd 4, 7
        mulpade 5, 7
        mulpade 6, 7
        adcxq   zero, %r14

// Double and add things; use z[1]..z[8] and thereafter the registers
// %r9..%r15 which haven't been written back yet

        xorl    zeroe, zeroe
        movq    (x), %rdx
        mulxq   %rdx, %rax, %rcx
        movq    %rax, (z)
        movq    8(z), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 8(z)

        movq    16(z), %rax
        movq    8(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 16(z)
        movq    24(z), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 24(z)

        movq    32(z), %rax
        movq    16(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 32(z)
        movq    40(z), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 40(z)

        movq    48(z), %rax
        movq    24(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 48(z)
        movq    56(z), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 56(z)

        movq    64(z), %rax
        movq    32(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 64(z)
        adcxq   %r9, %r9
        adoxq   %rcx, %r9
        movq    %r9, 72(z)

        movq    40(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %r10, %r10
        adoxq   %rdx, %r10
        movq    %r10, 80(z)
        adcxq   %r11, %r11
        adoxq   %rcx, %r11
        movq    %r11, 88(z)

        movq    48(x), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %r12, %r12
        adoxq   %rdx, %r12
        movq    %r12, 96(z)
        adcxq   %r13, %r13
        adoxq   %rcx, %r13
        movq    %r13, 104(z)

        movq    56(x), %rdx
        mulxq   %rdx, %rdx, %r15
        adcxq   %r14, %r14
        adoxq   %rdx, %r14
        movq    %r14, 112(z)
        adcxq   zero, %r15
        adoxq   zero, %r15
        movq    %r15, 120(z)

.endm


S2N_BN_SYMBOL(bignum_sqr_8_16):
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

// Do the multiplication

        diagonals

// Real epilog

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

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
