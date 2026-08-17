// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod m_25519
// Input x[k]; output z[4]
//
//    extern void bignum_mod_m25519(uint64_t z[static 4], uint64_t k,
//                                  const uint64_t *x);
//
// Reduction is modulo the group order of curve25519/edwards25519.
// This is the full group order, 8 * the standard basepoint order.
//
// Standard ARM ABI: X0 = z, X1 = k, X2 = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_m25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_m25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_m25519)
        .text
        .balign 4

#define z x0
#define k x1
#define x x2

#define m0 x3
#define m1 x4
#define m2 x5
#define m3 x6

#define t0 x7
#define t1 x8
#define t2 x9
#define t3 x10
#define n3 x10

#define n0 x11
#define n1 x12

// These two are aliased: we only load d when finished with q

#define q x13
#define d x13

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                         \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_mod_m25519):

// If the input is already <= 3 words long, go to a trivial "copy" path

        cmp     k, #4
        bcc     bignum_mod_m25519_short

// Otherwise load the top 4 digits (top-down) and reduce k by 4
// This [m3;m2;m1;m0] is the initial x where we begin reduction.

        sub     k, k, #4
        lsl     t0, k, #3
        add     t0, t0, x
        ldp     m2, m3, [t0, #16]
        ldp     m0, m1, [t0]

// Load the complicated three words of m_25519 (the other one is n2 = 0)

        movbig( n0, #0xc093, #0x18d2, #0xe7ae, #0x9f68)
        movbig( n1, #0xa6f7, #0xcef5, #0x17bc, #0xe6b2)
        mov     n3, #0x8000000000000000

// Do the subtraction; since m_25519 > 2^255 we only need either
// subtraction of m_25519 or nothing to reduce it modulo.

        subs    t0, m0, n0
        sbcs    t1, m1, n1
        sbcs    t2, m2, xzr
        sbcs    t3, m3, n3

// Now if the carry is *clear* (inversion at work) the subtraction carried
// and hence we should have done nothing, so we choose the original m_i

        csel    m0, m0, t0, cc
        csel    m1, m1, t1, cc
        csel    m2, m2, t2, cc
        csel    m3, m3, t3, cc

// Now do (k-4) iterations of 5->4 word modular reduction.

        cbz     k, bignum_mod_m25519_writeback

bignum_mod_m25519_loop:

// Assume that the new 5-digit x is 2^64 * previous_x + next_digit.
// Get the quotient estimate q = max (floor(x/2^255)) (2^64 - 1)
// and first compute x' = x - 2^255 * q.

        extr    q, m3, m2, #63
        and     m2, m2, #0x7FFFFFFFFFFFFFFF
        sub     q, q, m3, lsr #63
        and     m3, m3, #0x8000000000000000
        add     m2, m2, m3

// Multiply [t2;t1;t0] = q * [n1;n0]

        mul     t0, n0, q
        mul     t1, n1, q
        umulh   t2, n0, q
        adds    t1, t1, t2
        umulh   t2, n1, q
        adc     t2, t2, xzr

// Decrement k and load the next digit (note that d aliases to q)

        sub     k, k, #1
        ldr     d, [x, k, lsl #3]

// Subtract [t3;t2;t1;t0] = x' - q * [n1;n0] = x - q * m_25519

        subs    t0, d, t0
        sbcs    t1, m0, t1
        sbcs    t2, m1, t2
        sbcs    t3, m2, xzr

// If this borrows (CF = 0 because of inversion), add back m_25519.
// The masked n3 digit exploits the fact that bit 63 of n1 is set.

        csel    m0, n0, xzr, cc
        csel    m1, n1, xzr, cc
        adds    m0, t0, m0
        and     m3, m1, #0x8000000000000000
        adcs    m1, t1, m1
        adcs    m2, t2, xzr
        adc     m3, t3, m3

        cbnz    k, bignum_mod_m25519_loop

// Finally write back [m3;m2;m1;m0] and return

bignum_mod_m25519_writeback:
        stp     m0, m1, [z]
        stp     m2, m3, [z, #16]
        ret

// Short case: just copy the input with zero-padding

bignum_mod_m25519_short:
        mov     m0, xzr
        mov     m1, xzr
        mov     m2, xzr
        mov     m3, xzr

        cbz     k, bignum_mod_m25519_writeback
        ldr     m0, [x]
        subs    k, k, #1
        beq     bignum_mod_m25519_writeback
        ldr     m1, [x, #8]
        subs    k, k, #1
        beq     bignum_mod_m25519_writeback
        ldr     m2, [x, #16]
        b       bignum_mod_m25519_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_m25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
