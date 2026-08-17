// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_p256k1(uint64_t z[static 4],
//                                     const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p256k1)
        .text

#define z %rdi
#define x %rsi

#define d %rdx

#define a %rax
#define ashort %eax
#define q %rax

#define d0 %rcx
#define d1 %r8
#define d2 %r9
#define d3 %r10

// Re-use the x argument later on when it's no longer needed

#define h %rsi
#define c %rsi

S2N_BN_SYMBOL(bignum_tomont_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Since 2^256 == 4294968273 (mod p_256k1) we more or less just set
// m = 4294968273 then devolve to a variant of bignum_cmul_p256k1;
// the logic that q = h + 1 < 2^64 and hence doesn't wrap still holds
// since the multiplier 4294968273 is known to be much less than 2^64.
// We keep this constant in %rdx throughout as it's used repeatedly.

        movq    $4294968273, d

// Multiply, accumulating the result as 2^256 * h + [d3;d2;d1;d0]
// But immediately add 1 to h to get q = h + 1 as the quotient estimate.

        mulxq   (x), d0, d1
        mulxq   8(x), a, d2
        addq    a, d1
        mulxq   16(x), a, d3
        adcq    a, d2
        mulxq   24(x), a, h
        adcq    a, d3
        adcq    $1, h

// Now the quotient estimate is q = h + 1, and then we do the reduction,
// writing z = [d3;d2;d1;d0], as z' = (2^256 * h + z) - q * p_256k1 =
// (2^256 * h + z) - q * (2^256 - 4294968273) = -2^256 + (z + 4294968273 * q)

        mulxq   h, a, c

        addq    a, d0
        adcq    c, d1
        adcq    $0, d2
        adcq    $0, d3

// Because of the implicit -2^256, CF means >= 0 so z' is the answer; ~CF
// means z' < 0 so we add p_256k1, which in 4 digits means subtracting c.

        movq    $0, a
        cmovcq  a, d

        subq    d, d0
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

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
