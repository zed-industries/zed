// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_384, z := (3 * x) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_triple_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// The input x can be any 6-digit bignum, not necessarily reduced modulo p_384,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_384.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p384)
        .text

#define z %rdi
#define x %rsi

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define d4 %rbx
#define d5 %rsi

#define a %rax
#define c %rcx
#define q %rdx

#define ashort %eax
#define qshort %edx

S2N_BN_SYMBOL(bignum_triple_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// We seem to need (just!) one extra register, which we need to save and restore

        CFI_PUSH(%rbx)

// Multiply, accumulating the result as 2^384 * h + [d5;d4;d3;d2;d1;d0]
// but actually immediately producing q = h + 1, our quotient approximation,
// by adding 1 to it.

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
        movq    32(x), q
        movq    q, d4
        adcxq   q, q
        adoxq   q, d4
        movq    40(x), q
        movq    q, d5
        adcxq   q, q
        adoxq   q, d5

        movl    $1, qshort
        adcxq   a, q
        adoxq   a, q

// Initial subtraction of z - q * p_384, with bitmask c for the carry
// Actually done as an addition of (z - 2^384 * h) + q * (2^384 - p_384)
// which, because q = h + 1, is exactly 2^384 + (z - q * p_384), and
// therefore CF <=> 2^384 + (z - q * p_384) >= 2^384 <=> z >= q * p_384.

        movq    q, c
        shlq    $32, c
        movq    q, a
        subq    c, a
        sbbq    $0, c

        addq    a, d0
        adcq    c, d1
        adcq    q, d2
        adcq    $0, d3
        adcq    $0, d4
        adcq    $0, d5
        sbbq    c, c
        notq    c

// Now use that mask for a masked addition of p_384, which again is in
// fact done by a masked subtraction of 2^384 - p_384, so that we only
// have three nonzero digits and so can avoid using another register.

        movl    $0x00000000ffffffff, qshort
        xorl    ashort, ashort
        andq    c, q
        subq    q, a
        negq    c

        subq    a, d0
        movq    d0, (z)
        sbbq    q, d1
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

        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
