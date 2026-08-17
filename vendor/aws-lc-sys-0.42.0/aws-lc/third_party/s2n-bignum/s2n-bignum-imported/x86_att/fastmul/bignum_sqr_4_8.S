// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[4]; output z[8]
//
//    extern void bignum_sqr_4_8(uint64_t z[static 8], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_4_8)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_4_8)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_4_8)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// A zero register

#define zero %rbp
#define zeroe %ebp

// Other registers

#define d1 %r8
#define d2 %r9
#define d3 %r10
#define d4 %r11
#define d5 %r12
#define d6 %r13



S2N_BN_SYMBOL(bignum_sqr_4_8):
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

// Set up an initial window [d6;...d1] = [23;03;01]

        movq    (x), %rdx
        mulxq   8(x), d1, d2
        mulxq   24(x), d3, d4
        movq    16(x), %rdx
        mulxq   24(x), d5, d6

// Clear our zero register, and also initialize the flags for the carry chain

        xorl    zeroe, zeroe

// Chain in the addition of 02 + 12 + 13 to that window (no carry-out possible)
// This gives all the "heterogeneous" terms of the squaring ready to double

        mulxq   (x), %rax, %rcx
        adcxq   %rax, d2
        adoxq   %rcx, d3
        mulxq   8(x), %rax, %rcx
        adcxq   %rax, d3
        adoxq   %rcx, d4
        movq    24(x), %rdx
        mulxq   8(x), %rax, %rcx
        adcxq   %rax, d4
        adoxq   %rcx, d5
        adcxq   zero, d5
        adoxq   zero, d6
        adcxq   zero, d6

// In principle this is otiose as CF and OF carries are absorbed at this point
// However it seems helpful for the OOO engine to be told it's a fresh start

        xorl    zeroe, zeroe

// Double and add to the 00 + 11 + 22 + 33 terms
//
// We could use shift-double but this seems tidier and in larger squarings
// it was actually more efficient. I haven't experimented with this small
// case to see how much that matters. Note: the writeback here is sprinkled
// into the sequence in such a way that things still work if z = x, i.e. if
// the output overwrites the input buffer and beyond.

        movq    (x), %rdx
        mulxq   %rdx, %rax, %rdx
        movq    %rax, (z)
        adcxq   d1, d1
        adoxq   %rdx, d1
        movq    8(x), %rdx
        movq    d1, 8(z)
        mulxq   %rdx, %rax, %rdx
        adcxq   d2, d2
        adoxq   %rax, d2
        adcxq   d3, d3
        adoxq   %rdx, d3
        movq    16(x), %rdx
        movq    d2, 16(z)
        mulxq   %rdx, %rax, %rdx
        adcxq   d4, d4
        adoxq   %rax, d4
        adcxq   d5, d5
        adoxq   %rdx, d5
        movq    24(x), %rdx
        movq    d3, 24(z)
        mulxq   %rdx, %rax, %rdx
        movq    d4, 32(z)
        adcxq   d6, d6
        movq    d5, 40(z)
        adoxq   %rax, d6
        movq    d6, 48(z)
        adcxq   zero, %rdx
        adoxq   zero, %rdx
        movq    %rdx, 56(z)

// Restore saved registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_4_8)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
