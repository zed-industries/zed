// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[6]; output z[12]
//
//    extern void bignum_sqr_6_12(uint64_t z[static 12], const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_6_12)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_6_12)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_6_12)
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
#define d7 %r14
#define d8 %r15
#define d9 %rbx

// Care is needed: re-using the zero register

#define d10 %rbp


S2N_BN_SYMBOL(bignum_sqr_6_12):
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
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Set up an initial window [d8;...d1] = [34;05;03;01]

        movq    (x), %rdx
        mulxq   8(x), d1, d2
        mulxq   24(x), d3, d4
        mulxq   40(x), d5, d6
        movq    24(x), %rdx
        mulxq   32(x), d7, d8

// Clear our zero register, and also initialize the flags for the carry chain

        xorl    zeroe, zeroe

// Chain in the addition of 02 + 12 + 13 + 14 + 15 to that window
// (no carry-out possible since we add it to the top of a product)

        movq    16(x), %rdx
        mulxq   (x), %rax, %rcx
        adcxq   %rax, d2
        adoxq   %rcx, d3
        mulxq   8(x), %rax, %rcx
        adcxq   %rax, d3
        adoxq   %rcx, d4
        movq    8(x), %rdx
        mulxq   24(x), %rax, %rcx
        adcxq   %rax, d4
        adoxq   %rcx, d5
        mulxq   32(x), %rax, %rcx
        adcxq   %rax, d5
        adoxq   %rcx, d6
        mulxq   40(x), %rax, %rcx
        adcxq   %rax, d6
        adoxq   %rcx, d7
        adcxq   zero, d7
        adoxq   zero, d8
        adcxq   zero, d8

// Again zero out the flags. Actually they are already cleared but it may
// help decouple these in the OOO engine not to wait for the chain above

        xorl    zeroe, zeroe

// Now chain in the 04 + 23 + 24 + 25 + 35 + 45 terms
// We are running out of registers and here our zero register is not zero!

        movq    32(x), %rdx
        mulxq   (x), %rax, %rcx
        adcxq   %rax, d4
        adoxq   %rcx, d5
        movq    16(x), %rdx
        mulxq   24(x), %rax, %rcx
        adcxq   %rax, d5
        adoxq   %rcx, d6
        mulxq   32(x), %rax, %rcx
        adcxq   %rax, d6
        adoxq   %rcx, d7
        mulxq   40(x), %rax, %rcx
        adcxq   %rax, d7
        adoxq   %rcx, d8
        movq    24(x), %rdx
        mulxq   40(x), %rax, d9
        adcxq   %rax, d8
        adoxq   zero, d9
        movq    32(x), %rdx
        mulxq   40(x), %rax, d10
        adcxq   %rax, d9
        movl    $0, %eax
        adoxq   %rax, d10
        adcxq   %rax, d10

// Again, just for a clear fresh start for the flags

        xorl    %eax, %eax

// Double and add to the 00 + 11 + 22 + 33 + 44 + 55 terms
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
        adcxq   d6, d6
        adoxq   %rax, d6
        adcxq   d7, d7
        adoxq   %rdx, d7
        movq    32(x), %rdx
        movq    d4, 32(z)
        mulxq   %rdx, %rax, %rdx
        adcxq   d8, d8
        adoxq   %rax, d8
        adcxq   d9, d9
        adoxq   %rdx, d9
        movq    40(x), %rdx
        movq    d5, 40(z)
        mulxq   %rdx, %rax, %rdx
        movq    d6, 48(z)
        adcxq   d10, d10
        movq    d7, 56(z)
        adoxq   %rax, d10
        movq    d8, 64(z)
        movl    $0, %eax
        movq    d9, 72(z)
        adcxq   %rax, %rdx
        movq    d10, 80(z)
        adoxq   %rax, %rdx
        movq    %rdx, 88(z)

// Restore saved registers and return

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

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_6_12)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
