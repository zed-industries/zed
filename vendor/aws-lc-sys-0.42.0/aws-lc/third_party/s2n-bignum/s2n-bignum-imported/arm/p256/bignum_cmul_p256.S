// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_256, z := (c * x) mod p_256, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p256(uint64_t z[static 4], uint64_t c,
//                                 const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256_alt)
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
#define c x8
#define a0 x9
#define a1 x10
#define a2 x11

// These are the same!
#define a3 x12
#define q x12


S2N_BN_SYMBOL(bignum_cmul_p256):

S2N_BN_SYMBOL(bignum_cmul_p256_alt):
        CFI_START

// First do the multiply, straightforwardly to [d;d3;d2;d1;d0]

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        mul     d0, m, a0
        mul     d1, m, a1
        mul     d2, m, a2
        mul     d3, m, a3
        umulh   a0, m, a0
        umulh   a1, m, a1
        umulh   a2, m, a2
        umulh   h, m, a3
        adds    d1, d1, a0
        adcs    d2, d2, a1
        adcs    d3, d3, a2
        adcs    h, h, xzr

// Writing the product as z = 2^256 * h + 2^192 * l + t = 2^192 * hl + t, our
// intended quotient approximation is (hl + hl>>32 + 1)>>64. Note that by
// hypothesis our product is <= (2^64 - 1) * (p_256 - 1), so there is no need
// to max this out to avoid wrapping, unlike in the more general case of
// bignum_mod_p256.

        subs    xzr, xzr, xzr   // set carry flag for +1
        extr    q, h, d3, #32
        adcs    xzr, d3, q
        lsr     q, h, #32
        adcs    q, h, q

// It's easy to see -p_256 <= z - q * p_256 < p_256, so we just need to
// subtract q * p_256 and then correct if that is negative by adding p_256.
// We want z - q * p_256
//       = (z + 2^224 * q) - (2^256 + 2^192 + 2^96 - 1) * q
//
// We just do that computation in 5 words, freely ignoring the carry,
// since we have plenty to make our later decision just based on one bit,
// so one extra word is ample.

// First do [a2;a1] = 2^32 * q, which we use twice

        lsl     a1, q, #32
        lsr     a2, q, #32

// Add that to hl, hence including the 2^224 * q part

        adds    d3, d3, a1
        adc     h, h, a2

// Now accumulate [a2;a1;a0] = (2^96 - 1) * q.
// Remember q might be zero so we truly need a (short) carry chain here.

        subs    a0, xzr, q
        sbcs    a1, a1, xzr
        sbc     a2, a2, xzr

// Hence load remaining digits and do the subtraction

        subs    d0, d0, a0
        sbcs    d1, d1, a1
        sbcs    d2, d2, a2
        sbcs    d3, d3, q
        sbcs    c, h, q

// Now our top word is either zero or all 1s, and we use this to discriminate
// whether a correction is needed because our result is negative
// So correct by adding masked p_256

        adds    d0, d0, c
        mov     h, #0x00000000ffffffff
        and     h, h, c
        adcs    d1, d1, h
        adcs    d2, d2, xzr
        mov     h, #0xffffffff00000001
        and     h, h, c
        adc     d3, d3, h

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
