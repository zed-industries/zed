// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[16], y[16]; output z[32]; temporary buffer t[>=32]
//
//    extern void bignum_kmul_16_32(uint64_t z[static 32],
//                                  const uint64_t x[static 16],
//                                  const uint64_t y[static 16],
//                                  uint64_t t[static 32]);
//
// In this x86 code the final temporary space argument t is unused, but
// it is retained in the prototype above for API consistency with ARM.
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y, RCX = t
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y, R9 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_kmul_16_32)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_kmul_16_32)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_kmul_16_32)
        .text

// These parameters are kept where they come in

#define z %rdi
#define x %rsi

// This one gets moved to free up %rdx for muls

#define y %rcx

// Often used for zero

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

// addrow i adds z[i] + x[0..7] * y[i] into the window

.macro addrow arg1
        movq    8*\arg1(y), %rdx
        xorl    zeroe, zeroe

.if (\arg1 % 8 == 0)
        adoxq   8*\arg1(z), %r8
.elseif (\arg1 % 8 == 1)
        adoxq   8*\arg1(z), %r9
.elseif (\arg1 % 8 == 2)
        adoxq   8*\arg1(z), %r10
.elseif (\arg1 % 8 == 3)
        adoxq   8*\arg1(z), %r11
.elseif (\arg1 % 8 == 4)
        adoxq   8*\arg1(z), %r12
.elseif (\arg1 % 8 == 5)
        adoxq   8*\arg1(z), %r13
.elseif (\arg1 % 8 == 6)
        adoxq   8*\arg1(z), %r14
.elseif (\arg1 % 8 == 7)
        adoxq   8*\arg1(z), %r15
.endif

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

// Special zero version of addrow, setting up the window from scratch

.macro addrowz
        movq    (y), %rdx
        xorl    zeroe, zeroe

        mulxq   (x), %rax, %r9
        adcq    %rax, (z)

        mulxq   8(x), %rax, %r10
        adcq    %rax, %r9

        mulxq   16(x), %rax, %r11
        adcq    %rax, %r10

        mulxq   24(x), %rax, %r12
        adcq    %rax, %r11

        mulxq   32(x), %rax, %r13
        adcq    %rax, %r12

        mulxq   40(x), %rax, %r14
        adcq    %rax, %r13

        mulxq   48(x), %rax, %r15
        adcq    %rax, %r14

        mulxq   56(x), %rax, %r8
        adcq    %rax, %r15

        adcq    zero, %r8
.endm

// This is a variant where we add the initial z[0..7] at the outset.
// This makes the initialization process a bit less wasteful. By doing
// a block of 8 we get the same effect except that we add z[0..7]
//
// adurow i adds 2^{7*64} * z[i+7] + x[0..7] * y[i] into the window

.macro adurow arg1
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

// Special "adurow 0" case to do first stage

.macro adurowz
        movq    (y), %rdx
        xorl    zeroe, zeroe

        movq    (z), %r8
        movq    8(z), %r9

        mulpadd 0, 0
        movq    %r8, (z)

        movq    16(z), %r10
        mulpadd 1, 0
        movq    24(z), %r11
        mulpadd 2, 0
        movq    32(z), %r12
        mulpadd 3, 0
        movq    40(z), %r13
        mulpadd 4, 0
        movq    48(z), %r14
        mulpadd 5, 0
        movq    56(z), %r15
        mulpadd 6, 0

        mulxq   56(x), %rax, %r8
        adcxq   %rax, %r15
        adoxq   zero, %r8
        adcxq   zero, %r8
.endm

// Multiply-add: z := z + x[0..7] * y

.macro addrows
        adurowz
        adurow  1
        adurow  2
        adurow  3
        adurow  4
        adurow  5
        adurow  6
        adurow  7
        addrow  8
        addrow  9
        addrow  10
        addrow  11
        addrow  12
        addrow  13
        addrow  14
        addrow  15

        movq    %r8, 128(z)
        movq    %r9, 136(z)
        movq    %r10, 144(z)
        movq    %r11, 152(z)
        movq    %r12, 160(z)
        movq    %r13, 168(z)
        movq    %r14, 176(z)
        movq    %r15, 184(z)

.endm

// mulrow i adds x[0..7] * y[i] into the window
// just like addrow but no addition of z[i]

.macro mulrow arg1
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

// Special zero version of mulrow, setting up the window from scratch

.macro mulrowz
        movq    (y), %rdx
        xorl    zeroe, zeroe

        mulxq   (x), %rax, %r9
        movq    %rax, (z)

        mulxq   8(x), %rax, %r10
        adcxq    %rax, %r9

        mulxq   16(x), %rax, %r11
        adcxq   %rax, %r10

        mulxq   24(x), %rax, %r12
        adcxq   %rax, %r11

        mulxq   32(x), %rax, %r13
        adcxq   %rax, %r12

        mulxq   40(x), %rax, %r14
        adcxq   %rax, %r13

        mulxq   48(x), %rax, %r15
        adcxq   %rax, %r14

        mulxq   56(x), %rax, %r8
        adcxq   %rax, %r15

        adcq    zero, %r8
.endm

// Multiply-add: z := x[0..7] * y plus window

.macro mulrows
        mulrowz
        mulrow  1
        mulrow  2
        mulrow  3
        mulrow  4
        mulrow  5
        mulrow  6
        mulrow  7

        mulrow  8
        mulrow  9
        mulrow  10
        mulrow  11
        mulrow  12
        mulrow  13
        mulrow  14
        mulrow  15

        movq    %r8, 128(z)
        movq    %r9, 136(z)
        movq    %r10, 144(z)
        movq    %r11, 152(z)
        movq    %r12, 160(z)
        movq    %r13, 168(z)
        movq    %r14, 176(z)
        movq    %r15, 184(z)

.endm


S2N_BN_SYMBOL(bignum_kmul_16_32):
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

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Move y into its permanent home, freeing up %rdx for its special role in muls

        movq    %rdx, y

// Do the zeroth row as a pure product then the next as multiply-add

        mulrows

        addq    $64, z
        addq    $64, x
        addrows

// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_kmul_16_32)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
