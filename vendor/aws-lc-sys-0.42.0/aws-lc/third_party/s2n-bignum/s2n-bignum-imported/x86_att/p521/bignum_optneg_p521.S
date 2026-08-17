// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_521, z := (-x) mod p_521 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[9]; output z[9]
//
//    extern void bignum_optneg_p521(uint64_t z[static 9], uint64_t p,
//                                   const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = p, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = p, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p521)
        .text

#define z %rdi
#define p %rsi
#define x %rdx

#define q %rax
#define d0 %rcx
#define d1 %r8
#define d2 %r9
#define d3 %r10
#define d4 %r11

S2N_BN_SYMBOL(bignum_optneg_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Load most inputs (into the limited registers) and OR all of them to get q

        movq    (x), d0
        movq    d0, q
        movq    8(x), d1
        orq     d1, q
        movq    16(x), d2
        orq     d2, q
        movq    24(x), d3
        orq     d3, q
        movq    32(x), d4
        orq     d4, q
        orq     40(x), q
        orq     48(x), q
        orq     56(x), q
        orq     64(x), q

// Turn q into a bitmask for "input is nonzero and p is nonzero", so that
// we avoid doing -0 = p_521 and hence maintain strict modular reduction

        negq    q
        sbbq    q, q
        testq   p, p
        cmovzq  p, q

// Since p_521 is all 1s, the subtraction is just an exclusive-or with q
// to give an optional inversion, with a slight fiddle for the top digit.

        xorq    q, d0
        movq    d0, (z)
        xorq    q, d1
        movq    d1, 8(z)
        xorq    q, d2
        movq    d2, 16(z)
        xorq    q, d3
        movq    d3, 24(z)
        xorq    q, d4
        movq    d4, 32(z)
        movq    40(x), d0
        xorq    q, d0
        movq    d0, 40(z)
        movq    48(x), d1
        xorq    q, d1
        movq    d1, 48(z)
        movq    56(x), d2
        xorq    q, d2
        movq    d2, 56(z)
        movq    64(x), d3
        andq    $0x1FF, q
        xorq    q, d3
        movq    d3, 64(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
