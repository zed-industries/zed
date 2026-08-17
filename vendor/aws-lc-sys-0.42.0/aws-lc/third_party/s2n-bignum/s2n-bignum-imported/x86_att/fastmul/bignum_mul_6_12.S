// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[6], y[6]; output z[12]
//
//    extern void bignum_mul_6_12(uint64_t z[static 12], const uint64_t x[static 6],
//                                const uint64_t y[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_6_12)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_6_12)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_6_12)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// A zero register

#define zero %rbp
#define zeroe %ebp

// Add in x[i] * %rdx to the (i,i+1) position with the register window
// Would be nice to have conditional expressions reg[i], reg[i+1] ...

.macro mulpadd arg1,arg2
        mulxq   8*\arg2(x), %rax, %rbx
.if ((\arg1 + \arg2) % 6 == 0)
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
.elseif ((\arg1 + \arg2) % 6 == 1)
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
.elseif ((\arg1 + \arg2) % 6 == 2)
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
.elseif ((\arg1 + \arg2) % 6 == 3)
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
.elseif ((\arg1 + \arg2) % 6 == 4)
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
.elseif ((\arg1 + \arg2) % 6 == 5)
        adcxq   %rax, %r13
        adoxq   %rbx, %r8
.endif

.endm


// Add in the whole j'th row

.macro addrow arg1
        movq    8*\arg1(y), %rdx
        xorl    zeroe, zeroe

        mulpadd \arg1, 0

.if (\arg1 % 6 == 0)
        movq    %r8, 8*\arg1(z)
.elseif (\arg1 % 6 == 1)
        movq    %r9, 8*\arg1(z)
.elseif (\arg1 % 6 == 2)
        movq    %r10, 8*\arg1(z)
.elseif (\arg1 % 6 == 3)
        movq    %r11, 8*\arg1(z)
.elseif (\arg1 % 6 == 4)
        movq    %r12, 8*\arg1(z)
.elseif (\arg1 % 6 == 5)
        movq    %r13, 8*\arg1(z)
.endif

        mulpadd \arg1, 1
        mulpadd \arg1, 2
        mulpadd \arg1, 3
        mulpadd \arg1, 4

.if (\arg1 % 6 == 0)
        mulxq   40(x), %rax, %r8
        adcxq   %rax, %r13
        adoxq   zero, %r8
        adcxq   zero, %r8
.elseif (\arg1 % 6 == 1)
        mulxq   40(x), %rax, %r9
        adcxq   %rax, %r8
        adoxq   zero, %r9
        adcxq   zero, %r9
.elseif (\arg1 % 6 == 2)
        mulxq   40(x), %rax, %r10
        adcxq   %rax, %r9
        adoxq   zero, %r10
        adcxq   zero, %r10
.elseif (\arg1 % 6 == 3)
        mulxq   40(x), %rax, %r11
        adcxq   %rax, %r10
        adoxq   zero, %r11
        adcxq   zero, %r11
.elseif (\arg1 % 6 == 4)
        mulxq   40(x), %rax, %r12
        adcxq   %rax, %r11
        adoxq   zero, %r12
        adcxq   zero, %r12
.elseif (\arg1 % 6 == 5)
        mulxq   40(x), %rax, %r13
        adcxq   %rax, %r12
        adoxq   zero, %r13
        adcxq   zero, %r13
.endif

.endm



S2N_BN_SYMBOL(bignum_mul_6_12):
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

// Copy y into a safe register to start with

        movq    %rdx, y

// Zero a register, which also makes sure we don't get a fake carry-in

        xorl    zeroe, zeroe

// Do the zeroth row, which is a bit different
// Write back the zero-zero product and then accumulate
// %r8,%r13,%r12,%r11,%r10,%r9 as y[0] * x from 1..6

        movq    (y), %rdx

        mulxq   (x), %r8, %r9
        movq    %r8, (z)

        mulxq   8(x), %rbx, %r10
        adcxq   %rbx, %r9

        mulxq   16(x), %rbx, %r11
        adcxq   %rbx, %r10

        mulxq   24(x), %rbx, %r12
        adcxq   %rbx, %r11

        mulxq   32(x), %rbx, %r13
        adcxq   %rbx, %r12

        mulxq   40(x), %rbx, %r8
        adcxq   %rbx, %r13
        adcxq   zero, %r8

// Now all the other rows in a uniform pattern

        addrow  1
        addrow  2
        addrow  3
        addrow  4
        addrow  5

// Now write back the additional columns

        movq    %r8, 48(z)
        movq    %r9, 56(z)
        movq    %r10, 64(z)
        movq    %r11, 72(z)
        movq    %r12, 80(z)
        movq    %r13, 88(z)

// Restore registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_6_12)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
