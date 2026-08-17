// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^576) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_demont_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// This assumes the input is < p_521 for correctness. If this is not the case,
// use the variant "bignum_deamont_p521" instead.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_p521)
        .text

#define z %rdi
#define x %rsi

// Rotating (aliased) registers for the digits

#define c %rax
#define d0 %rdx
#define d1 %rcx

#define d2 %rdx
#define d3 %rcx
#define d4 %rdx
#define d5 %rcx
#define d6 %rdx
#define d7 %rcx
#define d8 %rdx

S2N_BN_SYMBOL(bignum_demont_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Rotate, as a 521-bit quantity, by 9*64 - 521 = 55 bits right.

        movq    (x), d0
        movq    d0, c
        movq    8(x), d1
        shrdq   $55, d1, d0
        movq    d0, (z)
        movq    16(x), d2
        shrdq   $55, d2, d1
        movq    d1, 8(z)
        movq    24(x), d3
        shrdq   $55, d3, d2
        shlq    $9, c
        movq    d2, 16(z)
        movq    32(x), d4
        shrdq   $55, d4, d3
        movq    d3, 24(z)
        movq    40(x), d5
        shrdq   $55, d5, d4
        movq    d4, 32(z)
        movq    48(x), d6
        shrdq   $55, d6, d5
        movq    d5, 40(z)
        movq    56(x), d7
        shrdq   $55, d7, d6
        movq    d6, 48(z)
        movq    64(x), d8
        orq     c, d8
        shrdq   $55, d8, d7
        movq    d7, 56(z)
        shrq    $55, d8
        movq    d8, 64(z)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
