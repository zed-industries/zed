// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_sm2, z := (c * x) mod p_sm2, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_sm2_alt(uint64_t z[static 4], uint64_t c,
//                                    const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_sm2_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_sm2_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_sm2_alt)
        .text

#define z %rdi

// Temporarily moved here for initial multiply then thrown away

#define x %rcx
#define m %rsi

// Other variables

#define d %rdx
#define a %rax
#define c %rcx

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define h %rsi

#define hshort %esi

// Multiplier again for second stage
#define q %rdx
#define qshort %edx

S2N_BN_SYMBOL(bignum_cmul_sm2_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Shuffle inputs (since we want %rdx for the high parts of products)

        movq    %rdx, x

// Multiply, accumulating the result as ca = 2^256 * h + [d3;d2;d1;d0]

        movq    (x), a
        mulq    m
        movq    a, d0
        movq    d, d1

        movq    8(x), a
        mulq    m
        xorq    d2, d2
        addq    a, d1
        adcq    d, d2

        movq    16(x), a
        mulq    m
        xorq    d3, d3
        addq    a, d2
        adcq    d, d3

        movq    24(x), a
        mulq    m
        xorl    hshort, hshort
        addq    a, d3
        adcq    d, h

// Quotient approximation is (h * (1 + 2^32 + 2^64) + d3 + 2^64) >> 64.
// Note that by hypothesis our product is <= (2^64 - 1) * (p_sm2 - 1),
// so there is no need to max this out to avoid wrapping, unlike in the
// more general case of bignum_mod_sm2.

        movq    d3, a
        movl    $1, qshort
        addq    h, a
        adcq    h, q

        shrq    $32, a
        addq    h, a
        shrq    $32, a
        addq    a, q

// Now compute the initial pre-reduced [h;d3;d2;d1;d0] = ca - p_sm2 * q
// = ca - (2^256 - 2^224 - 2^96 + 2^64 - 1) * q

        movq    q, a
        movq    q, c
        shlq    $32, a
        shrq    $32, c

        addq    a, d3
        adcq    c, h

        subq    q, a
        sbbq    $0, c

        subq    q, h

        addq    q, d0
        adcq    a, d1
        adcq    c, d2
        adcq    $0, d3
        adcq    $0, h

// Now our top word h is either zero or all 1s, and we use this to discriminate
// whether a correction is needed because our result is negative, as a bitmask
// Do a masked addition of p_sm2

        movq    $0xffffffff00000000, a
        andq    h, a
        movq    $0xfffffffeffffffff, c
        andq    h, c
        addq    h, d0
        movq    d0, (z)
        adcq    a, d1
        movq    d1, 8(z)
        adcq    h, d2
        movq    d2, 16(z)
        adcq    c, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_sm2_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
