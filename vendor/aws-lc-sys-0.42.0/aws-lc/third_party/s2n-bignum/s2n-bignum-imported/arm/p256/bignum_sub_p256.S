// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_256, z := (x - y) mod p_256
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_sub_p256(uint64_t z[static 4], const uint64_t x[static 4],
//                                const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p256)
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


S2N_BN_SYMBOL(bignum_sub_p256):
        CFI_START

// First just subtract the numbers as [d3; d2; d1; d0]
// Set a mask based on (inverted) carry indicating x < y = correction is needed

        ldp     d0, d1, [x]
        ldp     l, c, [y]
        subs    d0, d0, l
        sbcs    d1, d1, c
        ldp     d2, d3, [x, #16]
        ldp     l, c, [y, #16]
        sbcs    d2, d2, l
        sbcs    d3, d3, c

// Create a mask for the condition x < y, when we need to correct

        csetm   c, cc

// Now correct by adding masked p_256

        adds    d0, d0, c
        and     l, c, #0x00000000ffffffff
        adcs    d1, d1, l
        adcs    d2, d2, xzr
        and     l, c, #0xffffffff00000001
        adc     d3, d3, l

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
