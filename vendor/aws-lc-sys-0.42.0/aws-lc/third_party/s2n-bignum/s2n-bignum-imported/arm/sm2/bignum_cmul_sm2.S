// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_sm2, z := (c * x) mod p_sm2, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_sm2(uint64_t z[static 4], uint64_t c,
//                                const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_sm2)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_sm2_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_sm2_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_sm2_alt)
        .text
        .balign 4

#define z x0
#define m x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define h x7
#define q x8
#define a1 x9
#define a2 x10
#define a3 x11
#define a4 x12

S2N_BN_SYMBOL(bignum_cmul_sm2):
S2N_BN_SYMBOL(bignum_cmul_sm2_alt):
        CFI_START

// First do the multiply, straightforwardly to get [h;d3;d2;d1;d0]

        ldp     a1, a2, [x]
        ldp     a3, a4, [x, #16]
        mul     d0, m, a1
        mul     d1, m, a2
        mul     d2, m, a3
        mul     d3, m, a4
        umulh   a1, m, a1
        umulh   a2, m, a2
        umulh   a3, m, a3
        umulh   h, m, a4
        adds    d1, d1, a1
        adcs    d2, d2, a2
        adcs    d3, d3, a3
        adc     h, h, xzr

// Quotient approximation is (h * (1 + 2^32 + 2^64) + d3 + 2^64) >> 64.
// Note that by hypothesis our product is <= (2^64 - 1) * (p_sm2 - 1),
// so there is no need to max this out to avoid wrapping, unlike in the
// more general case of bignum_mod_sm2.

        adds    a3, d3, h
        mov     a2, #1
        adc     a1, h, a2
        add     a2, h, a3, lsr #32
        add     q, a1, a2, lsr #32

// Let a3 = q<<32 and a4 = q>>32 then [a2;a1] = 2^32 * q - q

        lsl     a3, q, #32
        subs    a1, a3, q
        lsr     a4, q, #32
        sbc     a2, a4, xzr

// Do the basic correction as [h;d3;d2;d1;d0] := [h;d3;d2;d1;d0] - q * p_sm2

        sub     h, h, q
        adds    d0, d0, q
        adcs    d1, d1, a1
        adcs    d2, d2, a2
        adcs    d3, d3, a3
        adc     h, h, a4

// Use top word (which will be all zeros or all ones) as a mask to correct

        adds    d0, d0, h
        and     a1, h, #0xffffffff00000000
        adcs    d1, d1, a1
        adcs    d2, d2, h
        and     a3, h, #0xfffffffeffffffff
        adc     d3, d3, a3

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_sm2)
S2N_BN_SIZE_DIRECTIVE(bignum_cmul_sm2_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
