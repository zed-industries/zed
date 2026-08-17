// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p256)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p256_alt)
        .text
        .balign 4

// ----------------------------------------------------------------------------
// Core "x |-> (2^64 * x) mod p_256" macro, with x assumed to be < p_256.
// Input is in [d4;d3;d2;d1] and output in [d3;d2;d1;d0]
// using d4 as well as t1, t2, t3 as temporaries.
// ----------------------------------------------------------------------------

#define modstep_p256(d4, d3,d2,d1,d0, t1,t2,t3)                             \
/* Writing the input as z = 2^256 * h + 2^192 * l + t = 2^192 * hl + t,  */ \
/* our quotient approximation is MIN ((hl + hl>>32 + 1)>>64) (2^64 - 1). */ \
        subs    xzr, xzr, xzr __LF/* Set carry flag for +1 */          \
        extr    t3, d4, d3, #32 __LF                                   \
        adcs    xzr, d3, t3 __LF                                       \
        lsr     t3, d4, #32 __LF                                       \
        adcs    t3, d4, t3 __LF                                        \
        csetm   d0, cs __LF                                            \
        orr     t3, t3, d0 __LF                                        \
/* First do [t2;t1] = 2^32 * q, which we use twice                       */ \
        lsl     t1, t3, #32 __LF                                       \
        lsr     t2, t3, #32 __LF                                       \
/* Add 2^224 * q to sum                                                  */ \
        adds    d3, d3, t1 __LF                                        \
        adc     d4, d4, t2 __LF                                        \
/* Accumulate [t2;t1;d0] = (2^96 - 1) * q                                */ \
        subs    d0, xzr, t3 __LF                                       \
        sbcs    t1, t1, xzr __LF                                       \
        sbc     t2, t2, xzr __LF                                       \
/* Subtract (2^256 + 2^192 + 2^96 - 1) * q                               */ \
        subs    d0, xzr, d0 __LF                                       \
        sbcs    d1, d1, t1 __LF                                        \
        sbcs    d2, d2, t2 __LF                                        \
        sbcs    d3, d3, t3 __LF                                        \
        sbcs    d4, d4, t3 __LF                                        \
/* Use top word as mask to correct                                       */ \
        adds    d0, d0, d4 __LF                                        \
        mov     t1, #0x00000000ffffffff __LF                           \
        and     t1, t1, d4 __LF                                        \
        adcs    d1, d1, t1 __LF                                        \
        adcs    d2, d2, xzr __LF                                       \
        mov     t1, #0xffffffff00000001 __LF                           \
        and     t1, t1, d4 __LF                                        \
        adc     d3, d3, t1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6

#define t0 x1
#define t1 x7
#define t2 x8
#define t3 x9

S2N_BN_SYMBOL(bignum_tomont_p256):

S2N_BN_SYMBOL(bignum_tomont_p256_alt):
        CFI_START

// Load the input

        ldp     d0, d1, [x1]
        ldp     d2, d3, [x1, #16]

// Do an initial reduction to make sure this is < p_256, using just
// a copy of the bignum_mod_p256_4 code. This is needed to set up the
// invariant "input < p_256" for the main modular reduction steps.

        mov     t0, #0xffffffffffffffff
        mov     t1, #0x00000000ffffffff
        mov     t3, #0xffffffff00000001
        subs    t0, d0, t0
        sbcs    t1, d1, t1
        sbcs    t2, d2, xzr
        sbcs    t3, d3, t3
        csel    d0, d0, t0, cc
        csel    d1, d1, t1, cc
        csel    d2, d2, t2, cc
        csel    d3, d3, t3, cc

// Successively multiply by 2^64 and reduce

        modstep_p256(d3,d2,d1,d0,d4, t1,t2,t3)
        modstep_p256(d2,d1,d0,d4,d3, t1,t2,t3)
        modstep_p256(d1,d0,d4,d3,d2, t1,t2,t3)
        modstep_p256(d0,d4,d3,d2,d1, t1,t2,t3)

// Store the result and return

        stp     d1, d2, [x0]
        stp     d3, d4, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
