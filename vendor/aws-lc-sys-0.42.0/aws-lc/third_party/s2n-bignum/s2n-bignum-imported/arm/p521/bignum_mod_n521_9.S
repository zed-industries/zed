// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_521
// Input x[9]; output z[9]
//
//    extern void bignum_mod_n521_9(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Reduction is modulo the group order of the NIST curve P-521.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n521_9)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n521_9)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n521_9)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n521_9_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n521_9_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n521_9_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define n0 x2
#define n1 x3
#define n2 x4
#define n3 x5

#define d0 x6
#define d1 x7
#define d2 x8
#define d3 x9
#define d4 x10
#define d5 x11
#define d6 x12
#define d7 x13
#define d8 x14

#define q x15

// Re-use d6 and d7 as temporaries before they are needed

#define s d6
#define t d7

#define movbig(nn,n3,n2,n1,n0)                                              \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_mod_n521_9):

S2N_BN_SYMBOL(bignum_mod_n521_9_alt):
        CFI_START

// Load the top digit first into d8.
// The initial quotient estimate is q = h + 1 where x = 2^521 * h + t

        ldr     d8, [x, #64]
        lsr     q, d8, #9
        add     q, q, #1

// Let [5; n3; n2; n1; n0] = r_521 = 2^521 - n_521
// and form [d4;d3;d2;d1;d0] = q * r_521

        movbig( n0, #0x4490, #0x48e1, #0x6ec7, #0x9bf7)
        mul     d0, n0, q
        movbig( n1, #0xc44a, #0x3647, #0x7663, #0xb851)
        mul     d1, n1, q
        movbig( n2, #0x8033, #0xfeb7, #0x08f6, #0x5a2f)
        mul     d2, n2, q
        movbig( n3, #0xae79, #0x787c, #0x40d0, #0x6994)
        mul     d3, n3, q
        lsl     d4, q, #2
        add     d4, d4, q
        umulh   t, n0, q
        adds    d1, d1, t
        umulh   t, n1, q
        adcs    d2, d2, t
        umulh   t, n2, q
        adcs    d3, d3, t
        umulh   t, n3, q
        adc     d4, d4, t

// Now load other digits and form r = x - q * n_521 = (q * r_521 + t) - 2^521.
// But the computed result stuffs in 1s from bit 521 onwards and actually
// gives r' = (q * r_521 + t) + (2^576 - 2^521) = r + 2^576, including the
// top carry. Hence CF <=> r >= 0, while r' == r (mod 2^521).

        ldp     s, t, [x]
        adds    d0, d0, s
        adcs    d1, d1, t
        ldp     s, t, [x, #16]
        adcs    d2, d2, s
        adcs    d3, d3, t
        ldp     t, d5, [x, #32]
        adcs    d4, d4, t
        adcs    d5, d5, xzr
        ldp     d6, d7, [x, #48]
        adcs    d6, d6, xzr
        adcs    d7, d7, xzr
        orr     d8, d8, #~0x1FF
        adcs    d8, d8, xzr

// We already know r < n_521, but if it actually went negative then
// we need to add back n_521 again. Recycle q as a bitmask for r < n_521,
// and just subtract r_521 and mask rather than literally adding 2^521.
// This also gets rid of the bit-stuffing above.

        csetm   q, cc
        and     n0, n0, q
        subs    d0, d0, n0
        and     n1, n1, q
        sbcs    d1, d1, n1
        and     n2, n2, q
        sbcs    d2, d2, n2
        and     n3, n3, q
        sbcs    d3, d3, n3
        mov     n0, #5
        and     n0, n0, q
        sbcs    d4, d4, n0
        sbcs    d5, d5, xzr
        sbcs    d6, d6, xzr
        sbcs    d7, d7, xzr
        sbc     d8, d8, xzr
        and     d8, d8, #0x1FF

// Store the end result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n521_9)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
