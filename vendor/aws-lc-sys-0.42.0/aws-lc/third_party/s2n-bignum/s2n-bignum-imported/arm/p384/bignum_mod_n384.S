// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_384
// Input x[k]; output z[6]
//
//    extern void bignum_mod_n384(uint64_t z[static 6], uint64_t k,
//                                const uint64_t *x);
//
// Reduction is modulo the group order of the NIST curve P-384.
//
// Standard ARM ABI: X0 = z, X1 = k, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n384)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n384_alt)
        .text
        .balign 4

#define z x0
#define k x1
#define x x2

#define m0 x3
#define m1 x4
#define m2 x5
#define m3 x6
#define m4 x7
#define m5 x8

#define t0 x9
#define t1 x10
#define t2 x11
#define t3 x12
#define t4 x13
#define t5 x14

#define n0 x15
#define n1 x16
#define n2 x17

// Aliased to t4

#define q x13

// Aliased to t5

#define d x14

// This is aliased to t5; we get one extra (free-ish?) reg-reg move in the
// main loop by not using an additional register, which seems an OK decision.

#define t x14

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                              \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_mod_n384):

S2N_BN_SYMBOL(bignum_mod_n384_alt):
        CFI_START

// If the input is already <= 5 words long, go to a trivial "copy" path

        cmp     k, #6
        bcc     Lbignum_mod_n384_short

// Otherwise load the top 6 digits (top-down) and reduce k by 6

        sub     k, k, #6
        lsl     t0, k, #3
        add     t0, t0, x
        ldp     m4, m5, [t0, #32]
        ldp     m2, m3, [t0, #16]
        ldp     m0, m1, [t0]

// Load the complicated three words of 2^384 - n_384 = [0; 0; 0; n2; n1; n0]

        movbig( n0, #0x1313, #0xe695, #0x333a, #0xd68d)
        movbig( n1, #0xa7e5, #0xf24d, #0xb74f, #0x5885)
        movbig( n2, #0x389c, #0xb27e, #0x0bc8, #0xd220)

// Reduce the top 6 digits mod n_384 (a conditional subtraction of n_384)

        adds    t0, m0, n0
        adcs    t1, m1, n1
        adcs    t2, m2, n2
        adcs    t3, m3, xzr
        adcs    t4, m4, xzr
        adcs    t5, m5, xzr
        csel    m0, m0, t0, cc
        csel    m1, m1, t1, cc
        csel    m2, m2, t2, cc
        csel    m3, m3, t3, cc
        csel    m4, m4, t4, cc
        csel    m5, m5, t5, cc

// Now do (k-6) iterations of 7->6 word modular reduction

        cbz     k, Lbignum_mod_n384_writeback
Lbignum_mod_n384_loop:

// Compute q = min (m5 + 1) (2^64 - 1)

        adds    q, m5, #1
        csetm   t0, cs
        orr     q, q, t0

// [t3;t2;t1;t0] = q * (2^384 - n_384)

        mul     t0, n0, q
        mul     t1, n1, q
        mul     t2, n2, q

        umulh   t3, n0, q
        adds    t1, t1, t3
        umulh   t3, n1, q
        adcs    t2, t2, t3
        umulh   t3, n2, q
        adc     t3, xzr, t3

// Decrement k and load the next digit

        sub     k, k, #1
        ldr     d, [x, k, lsl #3]

// Compensate for 2^384 * q

        sub     m5, m5, q

// [m5;m4;t4;t3;t2;t1;t0] = [m5;m4;m3;m2;m1;m0;d] - q * n_384

        adds    t0, d, t0
        adcs    t1, m0, t1
        adcs    t2, m1, t2
        adcs    t3, m2, t3
        adcs    t4, m3, xzr
        adcs    m4, m4, xzr
        adc     m5, m5, xzr

// Now our top word m5 is either zero or all 1s. Use it for a masked
// addition of n_384, which we can do by a *subtraction* of
// 2^384 - n_384 from our portion, re-using the constants

        and     t, m5, n0
        subs    m0, t0, t
        and     t, m5, n1
        sbcs    m1, t1, t
        and     t, m5, n2
        sbcs    m2, t2, t
        sbcs    m3, t3, xzr
        sbcs    t, t4, xzr
        sbc     m5, m4, xzr
        mov     m4, t

        cbnz    k, Lbignum_mod_n384_loop

// Finally write back [m5;m4;m3;m2;m1;m0] and return

Lbignum_mod_n384_writeback:
        stp     m0, m1, [z]
        stp     m2, m3, [z, #16]
        stp     m4, m5, [z, #32]

        CFI_RET

// Short case: just copy the input with zero-padding

Lbignum_mod_n384_short:
        mov     m0, xzr
        mov     m1, xzr
        mov     m2, xzr
        mov     m3, xzr
        mov     m4, xzr
        mov     m5, xzr

        cbz     k, Lbignum_mod_n384_writeback
        ldr     m0, [x]
        subs    k, k, #1
        beq     Lbignum_mod_n384_writeback
        ldr     m1, [x, #8]
        subs    k, k, #1
        beq     Lbignum_mod_n384_writeback
        ldr     m2, [x, #16]
        subs    k, k, #1
        beq     Lbignum_mod_n384_writeback
        ldr     m3, [x, #24]
        subs    k, k, #1
        beq     Lbignum_mod_n384_writeback
        ldr     m4, [x, #32]
        b       Lbignum_mod_n384_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
