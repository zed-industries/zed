// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_384, z := (c * x) mod p_384, assuming
// x reduced
// Inputs c, x[6]; output z[6]
//
//    extern void bignum_cmul_p384_alt(uint64_t z[static 6], uint64_t c,
//                                     const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p384_alt)
        .text

#define z %rdi

// Temporarily moved here for initial multiply
#define x %rcx

// Likewise this is thrown away after initial multiply
#define m %rsi

#define a %rax
#define c %rcx
#define d %rdx

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define d4 %r12
#define d5 %rsi

// Multiplier again for second stage
#define q %rcx

#define ashort %eax
#define dshort %edx

#define cshort %ecx
#define qshort %ecx

S2N_BN_SYMBOL(bignum_cmul_p384_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// We seem to need (just!) one extra register, which we need to save and restore

        CFI_PUSH(%r12)

// Shuffle inputs (since we want %rdx for the high parts of products)

        movq    %rdx, x

// Multiply, accumulating the result as 2^384 * h + [d5;d4;d3;d2;d1;d0]
// but actually immediately producing q = h + 1, our quotient approximation,
// by adding 1 to it. Note that by hypothesis x is reduced mod p_384, so our
// product is <= (2^64 - 1) * (p_384 - 1) and hence  h <= 2^64 - 2, meaning
// there is no danger this addition of 1 could wrap.

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
        xorq    d4, d4
        addq    a, d3
        adcq    d, d4

        movq    32(x), a
        mulq    m
        addq    a, d4
        adcq    $0, d

        movq    m, a
        movq    d, d5
        mulq     40(x)
        movl    $1, qshort

        addq    a, d5
        adcq    d, q

// It's easy to see -p_384 <= z - q * p_384 < p_384, so we just need to
// subtract q * p_384 and then correct if that is negative by adding p_384.
//
// Write p_384 = 2^384 - r where r = 2^128 + 2^96 - 2^32 + 1
//
// We want z - q * (2^384 - r)
//       = (2^384 * h + l) - q * (2^384 - r)
//       = 2^384 * (h - q) + (l + q * r)
//       = 2^384 * (-1) + (l + q * r)

        movq    $0xffffffff00000001, a
        mulq    q
        addq    a, d0
        adcq    d, d1
        adcq    q, d2
        movq    q, a
        sbbq    c, c
        movl    $0x00000000ffffffff, dshort
        negq    c
        mulq    d
        addq    a, d1
        adcq    d, d2
        adcq    c, d3
        adcq    $0, d4
        adcq    $0, d5
        sbbq    c, c
        notq    c

// The net c value is now the top word of the 7-word answer, hence will
// be -1 if we need a corrective addition, 0 otherwise, usable as a mask.
// Now use that mask for a masked addition of p_384, which again is in
// fact done by a masked subtraction of 2^384 - p_384, so that we only
// have three nonzero digits and so can avoid using another register.

        movl    $0x00000000ffffffff, dshort
        xorq    a, a
        andq    c, d
        subq    d, a
        andq    $1, c

        subq    a, d0
        movq    d0, (z)
        sbbq    d, d1
        movq    d1, 8(z)
        sbbq    c, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)

// Return

        CFI_POP(%r12)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
