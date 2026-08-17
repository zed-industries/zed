// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_521, z := (-x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_neg_p521(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p521)
        .text

#define z %rdi
#define x %rsi

#define p %rax
#define d0 %rcx
#define d1 %rdx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11

S2N_BN_SYMBOL(bignum_neg_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load most inputs (into the limited registers) and OR all of them to get p

        movq    (x), d0
        movq    d0, p
        movq    8(x), d1
        orq     d1, p
        movq    16(x), d2
        orq     d2, p
        movq    24(x), d3
        orq     d3, p
        movq    32(x), d4
        orq     d4, p
        movq    40(x), d5
        orq     d5, p
        orq     48(x), p
        orq     56(x), p
        orq     64(x), p

// Turn p into a bitmask for "input is nonzero", so that we avoid doing
// -0 = p_521 and hence maintain strict modular reduction

        negq    p
        sbbq    p, p

// Since p_521 is all 1s, the subtraction is just an exclusive-or with p
// to give an optional inversion, with a slight fiddle for the top digit.

        xorq    p, d0
        movq    d0, (z)
        xorq    p, d1
        movq    d1, 8(z)
        xorq    p, d2
        movq    d2, 16(z)
        xorq    p, d3
        movq    d3, 24(z)
        xorq    p, d4
        movq    d4, 32(z)
        xorq    p, d5
        movq    d5, 40(z)
        movq    48(x), d0
        xorq    p, d0
        movq    d0, 48(z)
        movq    56(x), d1
        xorq    p, d1
        movq    d1, 56(z)
        movq    64(x), d2
        andq    $0x1FF, p
        xorq    p, d2
        movq    d2, 64(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
