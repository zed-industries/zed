// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod m_25519
// Input x[4]; output z[4]
//
//    extern void bignum_mod_m25519_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Reduction is modulo the group order of curve25519/edwards25519.
// This is the full group order, 8 * the standard basepoint order.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_m25519_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_m25519_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_m25519_4)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9

#define n0 %rax
#define n1 %r10
#define n3 %r11

// Can re-use this as a temporary once we've loaded the input

#define c %rsi

S2N_BN_SYMBOL(bignum_mod_m25519_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load a set of registers [n3; 0; n1; n0] = m_25519

        movq    $0xc09318d2e7ae9f68, n0
        movq    $0xa6f7cef517bce6b2, n1
        movq    $0x8000000000000000, n3

// Load the input and compute x - m_25519

        movq    (x), d0
        subq    n0, d0
        movq    8(x), d1
        sbbq    n1, d1
        movq    16(x), d2
        sbbq    $0, d2
        movq    24(x), d3
        sbbq    n3, d3

// Now CF is set iff x < m_25519. Create a mask for that condition and mask
// the three nontrivial digits ready to undo the previous subtraction with
// a compensating addition

        sbbq    c, c
        andq    c, n0
        andq    c, n1
        andq    c, n3

// Now add mask * m_25519 again and store

        addq    n0, d0
        movq    d0, (z)
        adcq    n1, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    n3, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_m25519_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
