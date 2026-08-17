// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add modulo p_384, z := (x + y) mod p_384, assuming x and y reduced
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_add_p384(uint64_t z[static 6], const uint64_t x[static 6],
//                                const uint64_t y[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_add_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_add_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_add_p384)
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
#define d4 x9
#define d5 x10


S2N_BN_SYMBOL(bignum_add_p384):
        CFI_START

// First just add the numbers as c + [d5; d4; d3; d2; d1; d0]

        ldp     d0, d1, [x]
        ldp     l, c, [y]
        adds    d0, d0, l
        adcs    d1, d1, c
        ldp     d2, d3, [x, #16]
        ldp     l, c, [y, #16]
        adcs    d2, d2, l
        adcs    d3, d3, c
        ldp     d4, d5, [x, #32]
        ldp     l, c, [y, #32]
        adcs    d4, d4, l
        adcs    d5, d5, c
        adc     c, xzr, xzr

// Now compare [d5; d4; d3; d2; d1; d0] with p_384

        mov     l, #0x00000000ffffffff
        subs    xzr, d0, l
        mov     l, #0xffffffff00000000
        sbcs    xzr, d1, l
        mov     l, #0xfffffffffffffffe
        sbcs    xzr, d2, l
        adcs    xzr, d3, xzr
        adcs    xzr, d4, xzr
        adcs    xzr, d5, xzr

// Now CF is set (because of inversion) if (x + y) % 2^384 >= p_384
// Thus we want to correct if either this is set or the original carry c was

        adcs    c, c, xzr
        csetm   c, ne

// Now correct by subtracting masked p_384

        mov     l, #0x00000000ffffffff
        and     l, l, c
        subs    d0, d0, l
        eor     l, l, c
        sbcs    d1, d1, l
        mov     l, #0xfffffffffffffffe
        and     l, l, c
        sbcs    d2, d2, l
        sbcs    d3, d3, c
        sbcs    d4, d4, c
        sbc     d5, d5, c

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_add_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
