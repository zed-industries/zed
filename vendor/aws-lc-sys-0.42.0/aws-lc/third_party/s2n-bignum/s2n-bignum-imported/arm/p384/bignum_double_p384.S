// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_384, z := (2 * x) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_double_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p384)
        .text
        .balign 4

#define z x0
#define x x1
#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7
#define c x8
#define n0 x9
#define n1 x10
#define n2 x11
#define n3 x12
#define n4 x13
#define n5 x14


S2N_BN_SYMBOL(bignum_double_p384):
        CFI_START

// Double the input number as 2 * x = c + [d5; d4; d3; d2; d1; d0]
// It's worth considering doing this with extr...63 instead

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]
        adds    d0, d0, d0
        adcs    d1, d1, d1
        adcs    d2, d2, d2
        adcs    d3, d3, d3
        adcs    d4, d4, d4
        adcs    d5, d5, d5
        adc     c, xzr, xzr

// Subtract p_384 to give 2 * x - p_384 = c + [n5; n4; n3; n2; n1; n0]

        mov     n0, #0x00000000ffffffff
        subs    n0, d0, n0
        mov     n1, #0xffffffff00000000
        sbcs    n1, d1, n1
        mov     n2, #0xfffffffffffffffe
        sbcs    n2, d2, n2
        adcs    n3, d3, xzr
        adcs    n4, d4, xzr
        adcs    n5, d5, xzr
        sbcs    c, c, xzr

// Now CF is set (because of inversion) if 2 * x >= p_384, in which case the
// correct result is [n5; n4; n3; n2; n1; n0], otherwise
// [d5; d4; d3; d2; d1; d0]

        csel    d0, d0, n0, cc
        csel    d1, d1, n1, cc
        csel    d2, d2, n2, cc
        csel    d3, d3, n3, cc
        csel    d4, d4, n4, cc
        csel    d5, d5, n5, cc

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
