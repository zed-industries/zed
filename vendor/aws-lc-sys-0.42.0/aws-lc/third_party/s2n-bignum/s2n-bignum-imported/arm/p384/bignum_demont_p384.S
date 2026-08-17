// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^384) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_demont_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// This assumes the input is < p_384 for correctness. If this is not the case,
// use the variant "bignum_deamont_p384" instead.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_p384)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_p384_alt)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing contents of [d5;d4;d3;d2;d1]. It is fine
// for d6 to be the same register as d0.
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
// ---------------------------------------------------------------------------

#define montreds(d6,d5,d4,d3,d2,d1,d0, t3,t2,t1)                            \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64            */  \
/* Recycle d0 (which we know gets implicitly cancelled) to store it     */  \
        lsl     t1, d0, #32 __LF                                       \
        add     d0, t1, d0 __LF                                        \
/* Now let [t2;t1] = 2^64 * w - w + w_hi where w_hi = floor(w/2^32)     */  \
/* We need to subtract 2^32 * this, and we can ignore its lower 32      */  \
/* bits since by design it will cancel anyway; we only need the w_hi    */  \
/* part to get the carry propagation going.                             */  \
        lsr     t1, d0, #32 __LF                                       \
        subs    t1, t1, d0 __LF                                        \
        sbc     t2, d0, xzr __LF                                       \
/* Now select in t1 the field to subtract from d1                       */  \
        extr    t1, t2, t1, #32 __LF                                   \
/* And now get the terms to subtract from d2 and d3                     */  \
        lsr     t2, t2, #32 __LF                                       \
        adds    t2, t2, d0 __LF                                        \
        adc     t3, xzr, xzr __LF                                      \
/* Do the subtraction of that portion                                   */  \
        subs    d1, d1, t1 __LF                                        \
        sbcs    d2, d2, t2 __LF                                        \
        sbcs    d3, d3, t3 __LF                                        \
        sbcs    d4, d4, xzr __LF                                       \
        sbcs    d5, d5, xzr __LF                                       \
/* Now effectively add 2^384 * w by taking d0 as the input for last sbc */  \
        sbc     d6, d0, xzr

// Input parameters

#define z x0
#define x x1

// Rotating registers for the intermediate windows

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7

// Other temporaries

#define u x8
#define v x9
#define w x10

S2N_BN_SYMBOL(bignum_demont_p384):

S2N_BN_SYMBOL(bignum_demont_p384_alt):
        CFI_START

// Set up an initial window with the input x and an extra leading zero

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]

// Systematically scroll left doing 1-step reductions

        montreds(d0,d5,d4,d3,d2,d1,d0, u,v,w)

        montreds(d1,d0,d5,d4,d3,d2,d1, u,v,w)

        montreds(d2,d1,d0,d5,d4,d3,d2, u,v,w)

        montreds(d3,d2,d1,d0,d5,d4,d3, u,v,w)

        montreds(d4,d3,d2,d1,d0,d5,d4, u,v,w)

        montreds(d5,d4,d3,d2,d1,d0,d5, u,v,w)

// This is already our answer with no correction needed

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
