// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_256k1, z := (x / 2) mod p_256k1, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_half_p256k1(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p256k1)
        .text

#define z %rdi
#define x %rsi

#define d0 %rcx
#define d1 %rdx
#define d2 %r8
#define d3 %r9

#define c %rax

S2N_BN_SYMBOL(bignum_half_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the 4 digits of x, and letting b be the LSB (whether it's odd)
// construct the constant c = 4294968273 * b

        movq    (x), d0
        movq    $4294968273, c
        movq    8(x), d1
        movq    $1, d3
        andq    d0, d3
        movq    16(x), d2
        cmovzq  d3, c
        movq    24(x), d3

// We want (x + b * p_256k1) / 2 where b is that LSB, in {0,1}.
// That amounts to (2^256 * b + x - 4294968273 * b) / 2, and
// modulo 4 words that's the same as ([2^256 * c + x] - c) / 2.
// So do that subtraction and shift a place right as we go.

        subq    c, d0
        sbbq    $0, d1
        sbbq    $0, d2
        sbbq    $0, d3
        sbbq    $0, c

// Shift right, pushing the carry back down, and store back

        shrdq   $1, d1, d0
        movq    d0, (z)
        shrdq   $1, d2, d1
        movq    d1, 8(z)
        shrdq   $1, d3, d2
        movq    d2, 16(z)
        shrdq   $1, c, d3
        movq    d3, 24(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
