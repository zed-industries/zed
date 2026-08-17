// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_384, z := (3 * x) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_triple_p384_alt(uint64_t z[static 6],
//                                       const uint64_t x[static 6]);
//
// The input x can be any 6-digit bignum, not necessarily reduced modulo p_384,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_384.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p384_alt)
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
#define q %rcx
#define d %rdx

#define ashort %eax
#define cshort %ecx
#define qshort %ecx
#define dshort %edx

S2N_BN_SYMBOL(bignum_triple_p384_alt):
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
        xorq    d4, d4
        mulq    c
        addq    a, d3
        adcq    d, d4

        movq    32(x), a
        mulq    c
        addq    a, d4
        adcq    $0, d

        movq    40(x), a
        movq    d, d5
        mulq    c
        addq    a, d5

        movl    $1, qshort
        adcq    d, q

// Initial subtraction of z - q * p_384, with bitmask c for the carry
// Actually done as an addition of (z - 2^384 * h) + q * (2^384 - p_384)
// which, because q = h + 1, is exactly 2^384 + (z - q * p_384), and
// therefore CF <=> 2^384 + (z - q * p_384) >= 2^384 <=> z >= q * p_384.

        movq    q, d
        shlq    $32, d
        movq    q, a
        subq    d, a
        sbbq    $0, d

        addq    a, d0
        adcq    d, d1
        adcq    q, d2
        adcq    $0, d3
        adcq    $0, d4
        adcq    $0, d5
        sbbq    d, d
        notq    d

// Now use that mask for a masked addition of p_384, which again is in
// fact done by a masked subtraction of 2^384 - p_384, so that we only
// have three nonzero digits and so can avoid using another register.

        movl    $0x00000000ffffffff, qshort
        xorl    ashort, ashort
        andq    d, q
        subq    q, a
        negq    d

        subq    a, d0
        movq    d0, (z)
        sbbq    q, d1
        movq    d1, 8(z)
        sbbq    d, d2
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

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
