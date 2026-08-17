// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_384, z := (c * x) mod p_384, assuming
// x reduced
// Inputs c, x[6]; output z[6]
//
//    extern void bignum_cmul_p384(uint64_t z[static 6], uint64_t c,
//                                 const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p384)
        .text

#define z %rdi

// Temporarily moved here for initial multiply
#define x %rcx
// Likewise this is thrown away after initial multiply
#define m %rdx

#define a %rax
#define c %rcx

#define d0 %rsi
#define d1 %r8
#define d2 %r9
#define d3 %r10
#define d4 %r11
#define d5 %r12
// Multiplier again for second stage
#define q %rdx

#define ashort %eax
#define cshort %ecx
#define qshort %edx


S2N_BN_SYMBOL(bignum_cmul_p384):
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

// Shuffle inputs (since we want multiplier in %rdx)

        movq    %rdx, x
        movq    %rsi, m

// Multiply, accumulating the result as 2^384 * h + [d5;d4;d3;d2;d1;d0]
// but actually immediately producing q = h + 1, our quotient approximation,
// by adding 1 to it. Note that by hypothesis x is reduced mod p_384, so our
// product is <= (2^64 - 1) * (p_384 - 1) and hence  h <= 2^64 - 2, meaning
// there is no danger this addition of 1 could wrap.

        mulxq   (x), d0, d1
        mulxq   8(x), a, d2
        addq    a, d1
        mulxq   16(x), a, d3
        adcq    a, d2
        mulxq   24(x), a, d4
        adcq    a, d3
        mulxq   32(x), a, d5
        adcq    a, d4
        mulxq   40(x), a, q
        adcq    a, d5
        adcq    $1, q

// It's easy to see -p_384 <= z - q * p_384 < p_384, so we just need to
// subtract q * p_384 and then correct if that is negative by adding p_384.
//
// Write p_384 = 2^384 - r where r = 2^128 + 2^96 - 2^32 + 1
//
// We want z - q * (2^384 - r)
//       = (2^384 * h + l) - q * (2^384 - r)
//       = 2^384 * (h - q) + (l + q * r)
//       = 2^384 * (-1) + (l + q * r)

        xorq    c, c
        movq    $0xffffffff00000001, a
        mulxq   a, a, c
        adcxq   a, d0
        adoxq   c, d1
        movl    $0x00000000ffffffff, ashort
        mulxq   a, a, c
        adcxq   a, d1
        adoxq   c, d2
        adcxq   q, d2
        movl    $0, ashort
        movl    $0, cshort
        adoxq   a, a
        adcq    a, d3
        adcq    c, d4
        adcq    c, d5
        adcq    c, c
        subq    $1, c

// The net c value is now the top word of the 7-word answer, hence will
// be -1 if we need a corrective addition, 0 otherwise, usable as a mask.
// Now use that mask for a masked addition of p_384, which again is in
// fact done by a masked subtraction of 2^384 - p_384, so that we only
// have three nonzero digits and so can avoid using another register.

        movl    $0x00000000ffffffff, qshort
        xorq    a, a
        andq    c, q
        subq    q, a
        andq    $1, c

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

        CFI_POP(%r12)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
