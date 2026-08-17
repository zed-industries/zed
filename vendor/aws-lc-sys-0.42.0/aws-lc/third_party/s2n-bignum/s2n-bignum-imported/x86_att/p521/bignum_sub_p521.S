// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_521, z := (x - y) mod p_521
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_sub_p521(uint64_t z[static 9], const uint64_t x[static 9],
//                                const uint64_t y[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p521)
        .text

#define z %rdi
#define x %rsi
#define y %rdx

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define d6 %r12
#define d7 %rbx

// Re-use one input pointer as other variable once safe to do so

#define d8 %rsi



S2N_BN_SYMBOL(bignum_sub_p521):
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
        CFI_PUSH(%r12)

// First just subtract the numbers as [d8;d7;d6;d5;d4;d3;d2;d1;d0] = x - y

        movq    (x), d0
        subq    (y), d0
        movq    8(x), d1
        sbbq    8(y), d1
        movq    16(x), d2
        sbbq    16(y), d2
        movq    24(x), d3
        sbbq    24(y), d3
        movq    32(x), d4
        sbbq    32(y), d4
        movq    40(x), d5
        sbbq    40(y), d5
        movq    48(x), d6
        sbbq    48(y), d6
        movq    56(x), d7
        sbbq    56(y), d7
        movq    64(x), d8
        sbbq    64(y), d8

// Now if x < y we want (x - y) + p_521 == (x - y) - 1 (mod 2^521)
// Otherwise we just want the existing x - y result. So subtract
// 1 iff the initial subtraction carried, then mask to 521 bits.
// Write back the results as generated.

        sbbq    $0, d0
        movq    d0, (z)
        sbbq    $0, d1
        movq    d1, 8(z)
        sbbq    $0, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)
        sbbq    $0, d6
        movq    d6, 48(z)
        sbbq    $0, d7
        movq    d7, 56(z)
        sbbq    $0, d8
        andq    $0x1FF, d8
        movq    d8, 64(z)

// Restore registers and return

        CFI_POP(%r12)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
