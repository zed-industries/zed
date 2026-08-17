// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_256, z := (3 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_triple_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// The input x can be any 4-digit bignum, not necessarily reduced modulo p_256,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_256.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256)
        .text

#define z %rdi
#define x %rsi

// Main digits of intermediate results

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

// Quotient estimate = top of product + 1

#define q %rdx

// Other temporary variables and their short version

#define a %rax
#define c %rcx

#define ashort %eax
#define cshort %ecx
#define qshort %edx

S2N_BN_SYMBOL(bignum_triple_p256):
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

        xorl    ashort, ashort

        movq    (x), q
        movq    q, d0
        adcxq   q, q
        adoxq   q, d0
        movq    8(x), q
        movq    q, d1
        adcxq   q, q
        adoxq   q, d1
        movq    16(x), q
        movq    q, d2
        adcxq   q, q
        adoxq   q, d2
        movq    24(x), q
        movq    q, d3
        adcxq   q, q
        adoxq   q, d3

// For this limited range a simple quotient estimate of q = h + 1 works, where
// h = floor(z / 2^256). Then -p_256 <= z - q * p_256 < p_256, so we just need
// to subtract q * p_256 and then if that's negative, add back p_256.

        movl    $1, qshort
        adcxq   a, q
        adoxq   a, q

// Now compute the initial pre-reduced result z - p_256 * q
// = z - (2^256 - 2^224 + 2^192 + 2^96 - 1) * q
// = z - 2^192 * 0xffffffff00000001 * q - 2^64 * 0x0000000100000000 * q + q

        addq    q, d0
        movq    $0x0000000100000000, a
        mulxq   a, a, c
        sbbq    $0, a
        sbbq    $0, c
        subq    a, d1
        sbbq    c, d2
        movq    $0xffffffff00000001, a
        mulxq   a, a, c
        sbbq    a, d3
        sbbq    c, q

// q is now effectively the top word of the 5-digits result; this step
// compensates for q = h + 1

        decq    q

// Use that as a bitmask for a masked addition of p_256 and write back

        movl    $0x00000000ffffffff, ashort
        andq    q, a
        xorl    cshort, cshort
        subq    a, c
        addq    q, d0
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

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
