// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_256k1, z := (x - y) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_sub_p256k1(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p256k1)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2
#define c x3
#define l x4
#define d0 x5
#define d1 x6
#define d2 x7
#define d3 x8

S2N_BN_SYMBOL(bignum_sub_p256k1):
        CFI_START

// First just subtract the numbers as [d3; d2; d1; d0] = x - y,
// with the inverted carry flag meaning CF <=> x >= y.

        ldp     d0, d1, [x]
        ldp     l, c, [y]
        subs    d0, d0, l
        sbcs    d1, d1, c
        ldp     d2, d3, [x, #16]
        ldp     l, c, [y, #16]
        sbcs    d2, d2, l
        sbcs    d3, d3, c

// Now if x < y we want to add back p_256k1, which staying within 4 digits
// means subtracting 4294968273, since p_256k1 = 2^256 - 4294968273.
// Let c be that constant 4294968273 when x < y, zero otherwise.

        mov     l, #977
        orr     c, l, #0x100000000
        csel    c, c, xzr, cc

// Now correct by adding masked p_256k1, i.e. subtracting c

        subs    d0, d0, c
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
