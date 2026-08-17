// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_25519
// Input x[4]; output z[4]
//
//    extern void bignum_mod_p25519_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p25519_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p25519_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p25519_4)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define q x6
#define c x7

S2N_BN_SYMBOL(bignum_mod_p25519_4):
        CFI_START

// Load the inputs as [d3;d2;d1;d0]

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Letting x = 2^255 * h + l where h is the top bit, the provisional quotient
// is q = h + 1, which is either correct or 1 too high.

        mov     c, #19
        lsr     q, d3, #63
        madd    q, c, q, c

// Writing the provisional remainder as r = x - (2^255 - 19) * q we
// compute r' = (2^255 + l) + 19 * q = r + 2^256

        adds    d0, d0, q
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        orr     d3, d3, #0x8000000000000000
        adcs    d3, d3, xzr

// Now r < 0 <=> r' < 2^256 <=> ~CF and in this case we correct by adding
// 2^255 - 19, or in fact subtracting 19 and masking to 255 bits.

        csel    q, c, xzr, cc
        subs    d0, d0, q
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr
        and     d3, d3, #~0x8000000000000000

// Store the end result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p25519_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
