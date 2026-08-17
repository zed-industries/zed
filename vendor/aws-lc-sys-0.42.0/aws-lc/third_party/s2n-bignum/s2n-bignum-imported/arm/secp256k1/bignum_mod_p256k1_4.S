// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_mod_p256k1_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p256k1_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p256k1_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p256k1_4)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d x6
#define c x7

S2N_BN_SYMBOL(bignum_mod_p256k1_4):
        CFI_START

// Load the inputs as [d3;d2;d1;d0] and let d be an AND of [d3;d2;d1] to
// condense the comparison below.

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        and     d, d1, d2
        and     d, d, d3

// Compare x >= p_256k1 = 2^256 - 4294968273 using condensed carry:
// we get a carry from the lowest digit and all other digits are 1.
// We end up with c and d as adjusted digits for x - p_256k1 if so.

        mov     c, #977
        orr     c, c, #0x100000000
        adds    c, c, d0
        adcs    d, d, xzr

// If indeed x >= p_256k1 then x := x - p_256k1, using c and d

        csel    d0, d0, c, cc
        csel    d1, d1, d, cc
        csel    d2, d2, d, cc
        csel    d3, d3, d, cc

// Store the end result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p256k1_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
