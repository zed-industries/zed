// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_256k1, z := (c * x) mod p_256k1, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p256k1_alt(uint64_t z[static 4], uint64_t c,
//                                       const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256k1_alt)
        .text

#define z %rdi

// Temporarily moved here for initial multiply

#define x %rcx
#define c %rsi

// Likewise this is thrown away after initial multiply

#define d %rdx
#define h %rdx

#define a %rax
#define ashort %eax
#define q %rax

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %rcx

S2N_BN_SYMBOL(bignum_cmul_p256k1_alt):
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
        mulq    c
        movq    a, d0
        movq    d, d1

        movq    8(x), a
        xorq    d2, d2
        mulq    c
        addq    a, d1
        adcq    d, d2

        movq    16(x), a
        mulq    c
        addq    a, d2
        adcq    $0, d

        movq    24(x), a
        movq    d, d3
        mulq    c
        addq    a, d3
        adcq    $0, h

// Now the quotient estimate is q = h + 1, and then we do the reduction,
// writing z = [d3;d2;d1;d0], as z' = (2^256 * h + z) - q * p_256k1 =
// (2^256 * h + z) - q * (2^256 - 4294968273) = -2^256 + (z + 4294968273 * q)

        leaq    1(h), q
        movq    $4294968273, c
        mulq    c

        addq    %rax, d0
        adcq    %rdx, d1
        adcq    $0, d2
        adcq    $0, d3

// Because of the implicit -2^256, CF means >= 0 so z' is the answer; ~CF
// means z' < 0 so we add p_256k1, which in 4 digits means subtracting c.

        movq    $0, a
        cmovcq  a, c

        subq    c, d0
        movq    d0, (z)
        sbbq    a, d1
        movq    d1, 8(z)
        sbbq    a, d2
        movq    d2, 16(z)
        sbbq    a, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
