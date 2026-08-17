// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add modulo p_256k1, z := (x + y) mod p_256k1, assuming x and y reduced
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_add_p256k1(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_add_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_add_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_add_p256k1)
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
#define dd x9

S2N_BN_SYMBOL(bignum_add_p256k1):
        CFI_START

// First just add the numbers as z = x + y = 2^256 * c + [d3; d2; d1; d0]
// Also create dd = d3 AND d2 AND d1 to condense the later comparison

        ldp     d0, d1, [x]
        ldp     l, c, [y]
        adds    d0, d0, l
        adcs    d1, d1, c
        ldp     d2, d3, [x, #16]
        ldp     l, c, [y, #16]
        adcs    d2, d2, l
        and     dd, d1, d2
        adcs    d3, d3, c
        and     dd, dd, d3
        adc     c, xzr, xzr

// Let l = 4294968273 so that p_256k1 = 2^256 - l

        mov     l, #977
        orr     l, l, #0x100000000

// Decide whether z >= p_256k1 <=> z + 4294968273 >= 2^256

        adds    xzr, d0, l
        adcs    xzr, dd, xzr
        adcs    c, c, xzr

// Now c <> 0 <=> z >= p_256k1, so mask the constant l accordingly

        csel    l, l, xzr, ne

// If z >= p_256k1 do z := z - p_256k1, i.e. add l in 4 digits

        adds    d0, d0, l
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adc     d3, d3, xzr

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_add_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
