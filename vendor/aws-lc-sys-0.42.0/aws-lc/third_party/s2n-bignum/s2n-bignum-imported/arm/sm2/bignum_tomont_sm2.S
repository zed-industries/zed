// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_sm2)
        .text
        .balign 4

// ----------------------------------------------------------------------------
// Core "x |-> (2^64 * x) mod p_sm2" macro, with x assumed to be < p_sm2.
// We write it as a macro to be repeated instead of using .rep in assembler.
// The code here is very similar to the core of bignum_mod_sm2, just
// implicitly inserting zeros instead of fresh digits.
// ----------------------------------------------------------------------------

#define modstep_sm2()                                                   \
/* Writing the input, with a lowest zero digit appended, as     */      \
/* z = 2^256 * d3 + 2^192 * d2 + t, quotient approximation is   */      \
/* MIN ((d3 * (1 + 2^32 + 2^64) + d2 + 2^64) >> 64) (2^64 - 1)  */      \
        adds    t3, d2, d3 __LF                                            \
        mov     t2, #1 __LF                                                \
        adc     t1, d3, t2 __LF                                            \
        add     t2, d3, t3, lsr #32 __LF                                   \
        adds    q, t1, t2, lsr #32 __LF                                    \
        cinv    q, q, cs __LF                                              \
/* Let t3 = q<<32 and t4 = q>>32 then [t2;t1] = 2^32 * q - q    */      \
        lsl     t3, q, #32 __LF                                            \
        subs    t1, t3, q __LF                                             \
        lsr     t4, q, #32 __LF                                            \
        sbc     t2, t4, xzr __LF                                           \
/* Do the basic correction [t4;t3;t2;t1;q] = 2^256 * x - q * p  */      \
        adds    t1, t1, d0 __LF                                            \
        sub     d3, d3, q __LF                                             \
        adcs    t2, t2, d1 __LF                                            \
        adcs    t3, t3, d2 __LF                                            \
        adc     t4, t4, d3 __LF                                            \
/* Use top word as mask to correct */                                   \
        adds    d0, q, t4 __LF                                             \
        and     t0, t4, #0xffffffff00000000 __LF                           \
        adcs    d1, t1, t0 __LF                                            \
        adcs    d2, t2, t4 __LF                                            \
        and     t0, t4, #0xfffffffeffffffff __LF                           \
        adc     d3, t3, t0

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5

#define t1 x6
#define t2 x7
#define t3 x8

#define t4 x9

#define q x1
#define t0 x1

S2N_BN_SYMBOL(bignum_tomont_sm2):
        CFI_START

// Load the input

        ldp     d0, d1, [x1]
        ldp     d2, d3, [x1, #16]

// Do an initial reduction to make sure this is < p_sm2, using just
// a copy of the bignum_mod_sm2_4 code. This is needed to set up the
// invariant "input < p_sm2" for the main modular reduction steps.

        subs    t0, d0, #-1
        mov     t1, #0xffffffff00000000
        sbcs    t1, d1, t1
        adcs    t2, d2, xzr
        mov     t3, #0xfffffffeffffffff
        sbcs    t3, d3, t3
        csel    d0, d0, t0, cc
        csel    d1, d1, t1, cc
        csel    d2, d2, t2, cc
        csel    d3, d3, t3, cc

// Now do 4 iterations of a basic x |-> (2^64 * x) mod p_sm2 step.

        modstep_sm2()
        modstep_sm2()
        modstep_sm2()
        modstep_sm2()

// Store the result and return

        stp     d0, d1, [x0]
        stp     d2, d3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
