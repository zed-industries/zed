// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_256, z := (3 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_triple_p256_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4]);
//
// The input x can be any 4-digit bignum, not necessarily reduced modulo p_256,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_256.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256_alt)
        .text

#define z %rdi
#define x %rsi

// Main digits of intermediate results

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

// Quotient estimate = top of product + 1

#define q %rcx

// Other temporary variables and their short version

#define a %rax
#define c %rcx
#define d %rdx

#define ashort %eax
#define cshort %ecx
#define qshort %ecx
#define dshort %edx

S2N_BN_SYMBOL(bignum_triple_p256_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// First do the multiplication by 3, getting z = [h; d3; ...; d0]
// but immediately form the quotient estimate q = h + 1

        movl    $3, cshort

        movq    (x), a
        mulq    c
        movq    a, d0
        movq    d, d1

        movq    8(x), a
        xorq    d2, d2
        mulq    c
        addq    a, d1
        adcq    d, d2

        movq    16(x), a
        xorq    d3, d3
        mulq    c
        addq    a, d2
        adcq    d, d3

        movq    24(x), a
        mulq    c
        addq    a, d3

// For this limited range a simple quotient estimate of q = h + 1 works, where
// h = floor(z / 2^256). Then -p_256 <= z - q * p_256 < p_256, so we just need
// to subtract q * p_256 and then if that's negative, add back p_256.

        movl    $1, qshort
        adcq    d, q

// Now compute the initial pre-reduced result z - p_256 * q
// = z - (2^256 - 2^224 + 2^192 + 2^96 - 1) * q
// = z - 2^192 * 0xffffffff00000001 * q - 2^64 * 0x0000000100000000 * q + q
// Since q is small just use q<<32 for 0x0000000100000000 * q.

        movq    $0xffffffff00000001, a
        mulq    q
        movq    q, x
        shlq    $32, x
        addq    q, d0
        sbbq    $0, x
        subq    x, d1
        sbbq    $0, d2
        sbbq    a, d3
        sbbq    d, q

// q is now effectively the top word of the 5-digit result; this step
// compensates for q = h + 1

        decq    q

// Use that as a bitmask for a masked addition of p_256 and write back

        movl    $0x00000000ffffffff, ashort
        andq    q, a
        xorl    dshort, dshort
        subq    a, d
        addq    q, d0
        movq    d0, (z)
        adcq    a, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    d, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
