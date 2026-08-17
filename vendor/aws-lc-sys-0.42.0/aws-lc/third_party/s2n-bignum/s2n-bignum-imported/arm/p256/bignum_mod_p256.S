// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_256
// Input x[k]; output z[4]
//
//    extern void bignum_mod_p256(uint64_t z[static 4], uint64_t k,
//                                const uint64_t *x);
//
// Standard ARM ABI: X0 = z, X1 = k, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p256)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p256_alt)
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
#define t4 x11

#define n1 x12
#define n3 x13

#define q x14


S2N_BN_SYMBOL(bignum_mod_p256):

S2N_BN_SYMBOL(bignum_mod_p256_alt):
        CFI_START

// If the input is already <= 3 words long, go to a trivial "copy" path

        cmp     k, #4
        bcc     Lbignum_mod_p256_short

// Otherwise load the top 4 digits (top-down) and reduce k by 4

        sub     k, k, #4
        lsl     t0, k, #3
        add     t0, t0, x
        ldp     m2, m3, [t0, #16]
        ldp     m0, m1, [t0]

// Load the complicated words of p_256 = [n3;0;n1;-1]

        mov     n1, #0x00000000ffffffff
        mov     n3, #0xffffffff00000001

// Reduce the top 4 digits mod p_256 (a conditional subtraction of p_256)

        subs    t0, m0, #-1
        sbcs    t1, m1, n1
        sbcs    t2, m2, xzr
        sbcs    t3, m3, n3

        csel    m0, m0, t0, cc
        csel    m1, m1, t1, cc
        csel    m2, m2, t2, cc
        csel    m3, m3, t3, cc

// Now do (k-4) iterations of 5->4 word modular reduction

        cbz     k, Lbignum_mod_p256_writeback
Lbignum_mod_p256_loop:

// Decrement k and load the next digit as t0. We then want to reduce
// [m3;m2;m1;m0;t0] |-> [m3;m2;m1;m0]; the shuffling downwards is absorbed
// into the various ALU operations

        sub     k, k, #1
        ldr     t0, [x, k, lsl #3]

// Writing the input as z = 2^256 * h + 2^192 * l + t = 2^192 * hl + t, our
// intended quotient approximation is MIN ((hl + hl>>32 + 1)>>64) (2^64 - 1).

        subs    xzr, xzr, xzr   // set carry flag for +1
        extr    q, m3, m2, #32
        adcs    xzr, m2, q
        lsr     q, m3, #32
        adcs    q, m3, q
        csetm   t1, cs
        orr     q, q, t1

// First do [t2;t1] = 2^32 * q, which we use twice

        lsl     t1, q, #32
        lsr     t2, q, #32

// Add 2^224 * q to sum

        adds    t3, m2, t1
        adc     t4, m3, t2

// Accumulate [t2;t1;m3] = (2^96 - 1) * q, using m3 briefly as a temporary

        subs    m3, xzr, q
        sbcs    t1, t1, xzr
        sbc     t2, t2, xzr

// Subtract (2^256 + 2^192 + 2^96 - 1) * q

        subs    t0, t0, m3
        sbcs    t1, m0, t1
        sbcs    t2, m1, t2
        sbcs    t3, t3, q
        sbc     t4, t4, q

// Use top word as mask to correct

        adds    m0, t0, t4
        and     t0, n1, t4
        adcs    m1, t1, t0
        adcs    m2, t2, xzr
        and     t0, n3, t4
        adc     m3, t3, t0

        cbnz    k, Lbignum_mod_p256_loop

// Finally write back [m3;m2;m1;m0] and return

Lbignum_mod_p256_writeback:
        stp     m0, m1, [z]
        stp     m2, m3, [z, #16]
        CFI_RET

// Short case: just copy the input with zero-padding

Lbignum_mod_p256_short:
        mov     m0, xzr
        mov     m1, xzr
        mov     m2, xzr
        mov     m3, xzr

        cbz     k, Lbignum_mod_p256_writeback
        ldr     m0, [x]
        subs    k, k, #1
        beq     Lbignum_mod_p256_writeback
        ldr     m1, [x, #8]
        subs    k, k, #1
        beq     Lbignum_mod_p256_writeback
        ldr     m2, [x, #16]
        b       Lbignum_mod_p256_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
