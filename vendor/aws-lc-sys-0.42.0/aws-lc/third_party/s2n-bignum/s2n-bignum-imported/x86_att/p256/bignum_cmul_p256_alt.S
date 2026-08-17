// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_256, z := (c * x) mod p_256, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p256_alt(uint64_t z[static 4], uint64_t c,
//                                     const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256_alt)
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

#define ashort %eax
#define hshort %esi

// Multiplier again for second stage

#define q %rcx

S2N_BN_SYMBOL(bignum_cmul_p256_alt):
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

// Multiply, accumulating the result as 2^256 * h + [d3;d2;d1;d0]

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

// Writing the product as z = 2^256 * h + 2^192 * d3 + t = 2^192 * hl + t, our
// intended quotient approximation is (hl + hl>>32 + 1)>>64. Note that by
// hypothesis our product is <= (2^64 - 1) * (p_256 - 1), so there is no need
// to max this out to avoid wrapping.

        movq    h, a
        shldq   $32, d3, a
        movq    h, q
        shrq    $32, q
        xorq    d, d
        subq    $1, d
        adcq    d3, a
        adcq    h, q

// Now compute the initial pre-reduced result z - p_256 * q
// = z - (2^256 - 2^224 + 2^192 + 2^96 - 1) * q
// = z - 2^192 * 0xffffffff00000001 * q - 2^64 * 0x0000000100000000 * q + q

        movq    $0x0000000100000000, a
        mulq    q
        addq    q, d0
        sbbq    $0, a
        sbbq    $0, d
        subq    a, d1
        sbbq    d, d2
        sbbq    $0, d3
        sbbq    $0, h
        movq    $0xffffffff00000001, a
        mulq    q
        subq    a, d3
        sbbq    d, h

// Now our top word h is either zero or all 1s, and we use this to discriminate
// whether a correction is needed because our result is negative, as a bitmask
// Do a masked addition of p_256 and write back

        movl    $0x00000000ffffffff, ashort
        andq    h, a
        xorq    c, c
        subq    a, c
        addq    h, d0
        movq    d0, (z)
        adcq    a, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    c, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
