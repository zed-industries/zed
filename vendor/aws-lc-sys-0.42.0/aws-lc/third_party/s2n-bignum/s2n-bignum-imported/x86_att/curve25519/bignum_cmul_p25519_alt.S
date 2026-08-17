// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_25519, z := (c * x) mod p_25519, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p25519_alt(uint64_t z[static 4], uint64_t c,
//                                       const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p25519_alt)
        .text

#define z %rdi

// Temporarily moved here for initial multiply

#define x %rcx

// Used as a zero register after the initial move

#define zero %rsi
#define zeroe %esi

// Likewise this is thrown away after initial multiply

#define d %rdx
#define a %rax
#define ashort %eax

#define c %rcx
#define cshort %ecx

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define d4 %rdx

S2N_BN_SYMBOL(bignum_cmul_p25519_alt):
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

// Multiply, accumulating the result as [d4;d3;d2;d1;d0]

        movq    (x), a
        mulq    %rsi
        movq    a, d0
        movq    d, d1

        movq    8(x), a
        xorq    d2, d2
        mulq    %rsi
        addq    a, d1
        adcq    d, d2

        movq    16(x), a
        mulq    %rsi
        addq    a, d2
        adcq    $0, d

        movq    24(x), a
        movq    d, d3
        mulq    %rsi
        xorl    zeroe, zeroe
        addq    a, d3
        adcq    zero, d4

// Let [d4;d3;d2;d1;d0] = 2^255 * h + l, and use q = h + 1 as the initial
// quotient estimate, which is either right or 1 too big.

        shldq   $1, d3, d4
        movl    $19, cshort
        leaq    1(d4), a
        bts     $63, d3
        mulq    c
        addq    a, d0
        adcq    d, d1
        adcq    zero, d2
        adcq    zero, d3

// Correct if CF = 0 by subtracting 19, either way masking to
// 255 bits, i.e. by effectively adding p_25519 to the "full" answer

        cmovcq  zero, c
        subq    c, d0
        sbbq    zero, d1
        sbbq    zero, d2
        sbbq    zero, d3
        btr     $63, d3

// Write everything back

        movq    d0, (z)
        movq    d1, 8(z)
        movq    d2, 16(z)
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p25519_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
