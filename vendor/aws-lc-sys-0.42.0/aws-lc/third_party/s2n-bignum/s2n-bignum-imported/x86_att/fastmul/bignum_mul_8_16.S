// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[8], y[8]; output z[16]
//
//    extern void bignum_mul_8_16(uint64_t z[static 16], const uint64_t x[static 8],
//                                const uint64_t y[static 8]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_8_16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_8_16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_8_16)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// A zero register

#define zero %rbp
#define zeroe %ebp

// mulpadd i, j adds x[i] * rdx (now assumed = y[j]) into the window at i+j

.macro mulpadd arg1,arg2
        mulxq   8*\arg1(x), %rax, %rbx
.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
.endif

.endm

// mulpade i, j adds x[i] * rdx (now assumed = y[j]) into the window at i+j
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

// Add in the whole j'th row

.macro addrow arg1
        movq    8*\arg1(y), %rdx
        xorl    zeroe, zeroe

        mulpadd 0, \arg1

.if (\arg1 % 8 == 0)
        movq    %r8, 8*\arg1(z)
.elseif (\arg1 % 8 == 1)
        movq    %r9, 8*\arg1(z)
.elseif (\arg1 % 8 == 2)
        movq    %r10, 8*\arg1(z)
.elseif (\arg1 % 8 == 3)
        movq    %r11, 8*\arg1(z)
.elseif (\arg1 % 8 == 4)
        movq    %r12, 8*\arg1(z)
.elseif (\arg1 % 8 == 5)
        movq    %r13, 8*\arg1(z)
.elseif (\arg1 % 8 == 6)
        movq    %r14, 8*\arg1(z)
.elseif (\arg1 % 8 == 7)
        movq    %r15, 8*\arg1(z)
.endif

        mulpadd 1, \arg1
        mulpadd 2, \arg1
        mulpadd 3, \arg1
        mulpadd 4, \arg1
        mulpadd 5, \arg1
        mulpadd 6, \arg1
        mulpade 7, \arg1

.if (\arg1 % 8 == 0)
        adcq    zero, %r8
.elseif (\arg1 % 8 == 1)
        adcq    zero, %r9
.elseif (\arg1 % 8 == 2)
        adcq    zero, %r10
.elseif (\arg1 % 8 == 3)
        adcq    zero, %r11
.elseif (\arg1 % 8 == 4)
        adcq    zero, %r12
.elseif (\arg1 % 8 == 5)
        adcq    zero, %r13
.elseif (\arg1 % 8 == 6)
        adcq    zero, %r14
.elseif (\arg1 % 8 == 7)
        adcq    zero, %r15
.endif

.endm


S2N_BN_SYMBOL(bignum_mul_8_16):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save more registers to play with

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Copy y into a safe register to start with

        movq    %rdx, y

// Zero a register, which also makes sure we don't get a fake carry-in

        xorl    zeroe, zeroe

// Do the zeroth row, which is a bit different
// Write back the zero-zero product and then accumulate
// %r8,%r15,%r14,%r13,%r12,%r11,%r10,%r9 as y[0] * x from 1..8

        movq    (y), %rdx

        mulxq   (x), %r8, %r9
        movq    %r8, (z)

        mulxq   8(x), %rbx, %r10
        adcq    %rbx, %r9

        mulxq   16(x), %rbx, %r11
        adcq    %rbx, %r10

        mulxq   24(x), %rbx, %r12
        adcq    %rbx, %r11

        mulxq   32(x), %rbx, %r13
        adcq    %rbx, %r12

        mulxq   40(x), %rbx, %r14
        adcq    %rbx, %r13

        mulxq   48(x), %rbx, %r15
        adcq    %rbx, %r14

        mulxq   56(x), %rbx, %r8
        adcq    %rbx, %r15
        adcq    zero, %r8

// Now all the other rows in a uniform pattern

        addrow  1
        addrow  2
        addrow  3
        addrow  4
        addrow  5
        addrow  6
        addrow  7

// Now write back the additional columns

        movq    %r8, 64(z)
        movq    %r9, 72(z)
        movq    %r10, 80(z)
        movq    %r11, 88(z)
        movq    %r12, 96(z)
        movq    %r13, 104(z)
        movq    %r14, 112(z)
        movq    %r15, 120(z)

// Real epilog

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
