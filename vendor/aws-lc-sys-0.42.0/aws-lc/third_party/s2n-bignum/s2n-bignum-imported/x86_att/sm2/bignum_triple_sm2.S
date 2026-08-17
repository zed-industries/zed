// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_sm2, z := (3 * x) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_triple_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// The input x can be any 4-digit bignum, not necessarily reduced modulo p_sm2,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_sm2.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_sm2)
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
#define h %rdx
#define qshort %edx

// Other temporary variables and their short version

#define a %rax
#define c %rcx

#define ashort %eax

S2N_BN_SYMBOL(bignum_triple_sm2):
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
// h = floor(z / 2^256). Then -p_sm2 <= z - q * p_sm2 < p_sm2, so we just need
// to subtract q * p_sm2 and then if that's negative, add back p_sm2.

        movl    $1, qshort
        adcxq   a, q
        adoxq   a, q

// Now compute the initial pre-reduced [h;d3;d2;d1;d0] = z - p_sm2 * q
// = z - (2^256 - 2^224 - 2^96 + 2^64 - 1) * q

        movq    q, a
        shlq    $32, a
        movq    a, c
        subq    q, a

        addq    q, d0
        adcq    a, d1
        adcq    $0, d2
        adcq    c, d3
        sbbq    h, h
        notq    h

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

S2N_BN_SIZE_DIRECTIVE(bignum_triple_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
