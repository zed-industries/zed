// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^256) mod p_sm2, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_demont_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// This assumes the input is < p_sm2 for correctness. If this is not the case,
// use the variant "bignum_deamont_sm2" instead.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_sm2)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d3;d2;d1;d0] and returns result in [d4;d3;d2;d1], adding to the
// existing contents of [d3;d2;d1], and using t0, t1, t2 and t3 as
// temporaries. It is fine for d4 to be the same register as d0,
// and it often is.
// ---------------------------------------------------------------------------

#define montreds(d4,d3,d2,d1,d0, t3,t2,t1,t0)                               \
/* First let [t3;t2] = 2^32 * d0 and [t1;t0] = (2^32-1) * d0             */ \
        lsl     t2, d0, #32 __LF                                               \
        lsr     t3, d0, #32 __LF                                               \
        subs    t0, t2, d0 __LF                                                \
        sbc     t1, t3, xzr __LF                                               \
/* Now [d4;d3;d2;d1] := [d0;d3;d2;d1] - [t3;t2;t1;t0]                    */ \
        subs    d1, d1, t0 __LF                                                \
        sbcs    d2, d2, t1 __LF                                                \
        sbcs    d3, d3, t2 __LF                                                \
        sbc     d4, d0, t3

// Input parameters

#define z x0
#define x x1

// Rotating registers for the intermediate windows (with repetitions)

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5

// Other temporaries

#define t x6
#define u x7
#define v x8
#define w x9

S2N_BN_SYMBOL(bignum_demont_sm2):
        CFI_START

// Set up an initial window with the input x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Systematically scroll left doing 1-step reductions. This process
// keeps things reduced < p_sm2 at each stage, since we have
// w * p_sm2 + x <= (2^64 - 1) * p_sm2 + (p_sm2 - 1) < 2^64 * p_sm2

        montreds(d0,d3,d2,d1,d0, t,u,v,w)

        montreds(d1,d0,d3,d2,d1, t,u,v,w)

        montreds(d2,d1,d0,d3,d2, t,u,v,w)

        montreds(d3,d2,d1,d0,d3, t,u,v,w)

// Write back result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
