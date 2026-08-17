// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_25519, z := (-x) mod p_25519 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[4]; output z[4]
//
//    extern void bignum_optneg_p25519(uint64_t z[static 4], uint64_t p,
//                                     const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = p, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p25519)
        .text
        .balign 4

#define z x0
#define p x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define n0 x7
#define n1 x8
#define n2 x9
#define n3 x10

#define c x11

S2N_BN_SYMBOL(bignum_optneg_p25519):
        CFI_START

// Load the digits of x as [d3;d2;d1;d0] and compute c = the OR of those digits
// as well as its proto-negation [n3;n2;n1;n0] = (2^255 - 19) - x

        ldp     d0, d1, [x]
        mov     n0, #-19
        orr     c, d0, d1
        subs    n0, n0, d0
        mov     n2, #-1
        sbcs    n1, n2, d1
        ldp     d2, d3, [x, #16]
        orr     c, c, d2
        sbcs    n2, n2, d2
        mov     n3, #0x7FFFFFFFFFFFFFFF
        orr     c, c, d3
        sbc     n3, n3, d3

// Now we return just x if p = 0 or if x = 0 (to avoid giving -0 = p, which
// is not strictly reduced even though it's correct modulo p). The conditional
// comparison uses immediate 4 which means ZF.

        cmp     p, xzr
        ccmp    c, xzr, #4, ne

        csel    d0, n0, d0, ne
        csel    d1, n1, d1, ne
        csel    d2, n2, d2, ne
        csel    d3, n3, d3, ne

// Write back result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
