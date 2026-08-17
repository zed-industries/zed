// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_256k1, z := (2 * x) mod p_256k1, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_double_p256k1(uint64_t z[static 4],
//                                     const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p256k1)
        .text

#define z %rdi
#define x %rsi

#define d0 %rcx
#define d1 %r8
#define d2 %r9
#define d3 %r10

#define dd %rax
#define c %rdx

// Re-uses the input x when safe to do so

#define l %rsi

S2N_BN_SYMBOL(bignum_double_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the inputs and double top-down as z = 2^256 * c + [d3;d2;d1;d0]
// While doing this, create an AND dd of [d3;d2;d1] to condense comparison

        movq    24(x), d3
        movq    d3, c
        movq    16(x), d2
        shrq    $63, c
        shldq   $1, d2, d3
        movq    d3, dd
        movq    8(x), d1
        shldq   $1, d1, d2
        andq    d2, dd
        movq    (x), d0
        shldq   $1, d0, d1
        andq    d1, dd
        shlq    $1, d0

// Decide whether z >= p_256k1 <=> z + 4294968273 >= 2^256.
// For the lowest word use d0 + 4294968273 >= 2^64 <=> ~4294968273 < d0

        movq    $~4294968273, l
        cmpq    d0, l
        adcq    $0, dd
        adcq    $0, c

// Now c <> 0 <=> z >= p_256k1, so mask the constant l accordingly

        notq    l
        cmovzq  c, l

// If z >= p_256k1 do z := z - p_256k1, i.e. add l in 4 digits

        addq    l, d0
        movq    d0, (z)
        adcq    $0, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    $0, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
