// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_mod_n256k1_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Reduction is modulo the group order of the secp256k1 curve.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n256k1_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n256k1_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n256k1_4)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9

#define n0 %rax
#define n1 %r10
#define n2 %r11

#define n2short %r11d

// Can re-use this as a temporary once we've loaded the input

#define c %rsi

S2N_BN_SYMBOL(bignum_mod_n256k1_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load a set of registers [0; n2; n1; n0] = 2^256 - n_256k1

        movq    $0x402da1732fc9bebf, n0
        movq    $0x4551231950b75fc4, n1
        movl    $1, n2short

// Load the input and compute x + (2^256 - n_256k1)

        movq    (x), d0
        addq    n0, d0
        movq    8(x), d1
        adcq    n1, d1
        movq    16(x), d2
        adcq    n2, d2
        movq    24(x), d3
        adcq    $0, d3

// Now CF is set iff 2^256 <= x + (2^256 - n_256k1), i.e. iff n_256k1 <= x.
// Create a mask for the condition x < n, and mask the three nontrivial digits
// ready to undo the previous addition with a compensating subtraction

        sbbq    c, c
        notq    c
        andq    c, n0
        andq    c, n1
        andq    c, n2

// Now subtract mask * (2^256 - n_256k1) again and store

        subq    n0, d0
        movq    d0, (z)
        sbbq    n1, d1
        movq    d1, 8(z)
        sbbq    n2, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n256k1_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
