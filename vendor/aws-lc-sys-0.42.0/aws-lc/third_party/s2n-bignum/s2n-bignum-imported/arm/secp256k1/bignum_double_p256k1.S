// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_256k1, z := (2 * x) mod p_256k1, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_double_p256k1(uint64_t z[static 4],
//                                     const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p256k1)
        .text
        .balign 4

#define z x0
#define x x1
#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define c x6
#define dd x7
#define l x8

S2N_BN_SYMBOL(bignum_double_p256k1):
        CFI_START

// Load the inputs and double top-down as z = 2^256 * c + [d3;d2;d1;d0]
// While doing this, create an AND dd of [d3;d2;d1] to condense comparison

        ldp     d2, d3, [x, #16]
        lsr     c, d3, #63
        extr    d3, d3, d2, #63
        ldp     d0, d1, [x]
        extr    d2, d2, d1, #63
        and     dd, d2, d3
        extr    d1, d1, d0, #63
        and     dd, dd, d1
        lsl     d0, d0, #1

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

S2N_BN_SIZE_DIRECTIVE(bignum_double_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
