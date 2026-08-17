// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_256k1, z := (-x) mod p_256k1 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[4]; output z[4]
//
//    extern void bignum_optneg_p256k1(uint64_t z[static 4], uint64_t p,
//                                     const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = p, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = p, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p256k1)
        .text

#define z %rdi
#define q %rsi
#define x %rdx

#define n0 %rax
#define n1 %rcx
#define n2 %r8
#define n3 %r9

#define c %r10

#define qshort %esi

S2N_BN_SYMBOL(bignum_optneg_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Load the 4 digits of x and let c be an OR of all the digits

        movq    (x), n0
        movq    n0, c
        movq    8(x), n1
        orq     n1, c
        movq    16(x), n2
        orq     n2, c
        movq    24(x), n3
        orq     n3, c

// Turn q into a strict bitmask. Force it to zero if the input is zero,
// to avoid giving -0 = p_256k1, which is not reduced though correct modulo.

        cmovzq  c, q
        negq    q
        sbbq    q, q

// We want z := if q then (2^256 - 4294968273) - x else x
// which is: [if q then ~x else x] - [if q then 4294968272 else 0]

        xorq    q, n0
        xorq    q, n1
        xorq    q, n2
        xorq    q, n3

        movq    $4294968272, c
        andq    q, c
        xorl    qshort, qshort

        subq    c, n0
        movq    n0, (z)
        sbbq    q, n1
        movq    n1, 8(z)
        sbbq    q, n2
        movq    n2, 16(z)
        sbbq    q, n3
        movq    n3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
