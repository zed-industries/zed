// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_384
// Input x[k]; output z[6]
//
//    extern void bignum_mod_p384(uint64_t z[static 6], uint64_t k,
//                                const uint64_t *x);
//
// Standard ARM ABI: X0 = z, X1 = k, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p384)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p384_alt)
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


S2N_BN_SYMBOL(bignum_mod_p384):

S2N_BN_SYMBOL(bignum_mod_p384_alt):
        CFI_START

// If the input is already <= 5 words long, go to a trivial "copy" path

        cmp     k, #6
        bcc     Lbignum_mod_p384_short

// Otherwise load the top 6 digits (top-down) and reduce k by 6

        sub     k, k, #6
        lsl     t0, k, #3
        add     t0, t0, x
        ldp     m4, m5, [t0, #32]
        ldp     m2, m3, [t0, #16]
        ldp     m0, m1, [t0]

// Load the complicated lower three words of p_384 = [-1;-1;-1;n2;n1;n0]

        mov     n0, #0x00000000ffffffff
        mov     n1, #0xffffffff00000000
        mov     n2, #0xfffffffffffffffe

// Reduce the top 6 digits mod p_384 (a conditional subtraction of p_384)

        subs    t0, m0, n0
        sbcs    t1, m1, n1
        sbcs    t2, m2, n2
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

        cbz     k, Lbignum_mod_p384_writeback
Lbignum_mod_p384_loop:

// Decrement k and load the next digit as t5. We now want to reduce
// [m5;m4;m3;m2;m1;m0;t5] |-> [m5;m4;m3;m2;m1;m0]; the shuffling downwards is
// absorbed into the various ALU operations

        sub     k, k, #1
        ldr     t5, [x, k, lsl #3]

// Initial quotient approximation q = min (h + 1) (2^64 - 1)

        adds    m5, m5, #1
        csetm   t3, cs
        add     m5, m5, t3
        orn     n1, xzr, t3
        sub     t2, m5, #1
        sub     t1, xzr, m5

// Correction term [m5;t2;t1;t0] = q * (2^384 - p_384), using m5 as a temp

        lsl     t0, t1, #32
        extr    t1, t2, t1, #32
        lsr     t2, t2, #32
        adds    t0, t0, m5
        adcs    t1, t1, xzr
        adcs    t2, t2, m5
        adc     m5, xzr, xzr

// Addition to the initial value

        adds    t0, t5, t0
        adcs    t1, m0, t1
        adcs    t2, m1, t2
        adcs    t3, m2, m5
        adcs    t4, m3, xzr
        adcs    t5, m4, xzr
        adc     n1, n1, xzr

// Use net top of the 7-word answer (now in n1) for masked correction

        and     m5, n0, n1
        adds    m0, t0, m5
        eor     m5, m5, n1
        adcs    m1, t1, m5
        and     m5, n2, n1
        adcs    m2, t2, m5
        adcs    m3, t3, n1
        adcs    m4, t4, n1
        adc     m5, t5, n1

        cbnz    k, Lbignum_mod_p384_loop

// Finally write back [m5;m4;m3;m2;m1;m0] and return

Lbignum_mod_p384_writeback:
        stp     m0, m1, [z]
        stp     m2, m3, [z, #16]
        stp     m4, m5, [z, #32]

        CFI_RET

// Short case: just copy the input with zero-padding

Lbignum_mod_p384_short:
        mov     m0, xzr
        mov     m1, xzr
        mov     m2, xzr
        mov     m3, xzr
        mov     m4, xzr
        mov     m5, xzr

        cbz     k, Lbignum_mod_p384_writeback
        ldr     m0, [x]
        subs    k, k, #1
        beq     Lbignum_mod_p384_writeback
        ldr     m1, [x, #8]
        subs    k, k, #1
        beq     Lbignum_mod_p384_writeback
        ldr     m2, [x, #16]
        subs    k, k, #1
        beq     Lbignum_mod_p384_writeback
        ldr     m3, [x, #24]
        subs    k, k, #1
        beq     Lbignum_mod_p384_writeback
        ldr     m4, [x, #32]
        b       Lbignum_mod_p384_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
