// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended projective + precomputed mixed addition for edwards25519
// Inputs p1[16], p2[12]; output p3[16]
//
// extern void edwards25519_pepadd
//   (uint64_t p3[static 16],const uint64_t p1[static 16],
//    const uint64_t p2[static 12]);
//
// The output p3 and the first input p1 are points (x,y) on edwards25519
// represented in extended projective quadruples (X,Y,Z,T) where
// x = X / Z, y = Y / Z and x * y = T / Z. The second input p2 is a triple
// encoding its point (x,y) as (y - x,y + x,2 * d * x * y) where d is the
// usual Edwards curve parameter for edwards25519.
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_pepadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_pepadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_pepadd)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define p3 x17
#define p1 x19
#define p2 x20

// Pointers to input and output coordinates

#define x_1 p1, #0
#define y_1 p1, #NUMSIZE
#define z_1 p1, #(2*NUMSIZE)
#define w_1 p1, #(3*NUMSIZE)

#define ymx_2 p2, #0
#define xpy_2 p2, #NUMSIZE
#define kxy_2 p2, #(2*NUMSIZE)

#define x_3 p3, #0
#define y_3 p3, #NUMSIZE
#define z_3 p3, #(2*NUMSIZE)
#define w_3 p3, #(3*NUMSIZE)

// Pointer-offset pairs for temporaries on stack

#define t0 sp, #(0*NUMSIZE)
#define t1 sp, #(1*NUMSIZE)
#define t2 sp, #(2*NUMSIZE)
#define t3 sp, #(3*NUMSIZE)
#define t4 sp, #(4*NUMSIZE)
#define t5 sp, #(5*NUMSIZE)

// Total size to reserve on the stack

#define NSPACE 6*NUMSIZE

// Macro wrapping up the basic field operation bignum_mul_p25519, only
// trivially different from a pure function call to that subroutine.

#define mul_p25519(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        umull   x7, w3, w5 __LF                    \
        lsr     x0, x3, #32 __LF                   \
        umull   x15, w0, w5 __LF                   \
        lsr     x16, x5, #32 __LF                  \
        umull   x8, w16, w0 __LF                   \
        umull   x16, w3, w16 __LF                  \
        adds    x7, x7, x15, lsl #32 __LF          \
        lsr     x15, x15, #32 __LF                 \
        adc     x8, x8, x15 __LF                   \
        adds    x7, x7, x16, lsl #32 __LF          \
        lsr     x16, x16, #32 __LF                 \
        adc     x8, x8, x16 __LF                   \
        mul     x9, x4, x6 __LF                    \
        umulh   x10, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x16, cc __LF                       \
        adds    x9, x9, x8 __LF                    \
        adc     x10, x10, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, cc __LF                    \
        cinv    x16, x16, cc __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x8, x7, x9 __LF                    \
        adcs    x9, x9, x10 __LF                   \
        adc     x10, x10, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x15, x15, x16 __LF                 \
        adcs    x8, x15, x8 __LF                   \
        eor     x3, x3, x16 __LF                   \
        adcs    x9, x3, x9 __LF                    \
        adc     x10, x10, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x5, x6, [P2+16] __LF               \
        umull   x11, w3, w5 __LF                   \
        lsr     x0, x3, #32 __LF                   \
        umull   x15, w0, w5 __LF                   \
        lsr     x16, x5, #32 __LF                  \
        umull   x12, w16, w0 __LF                  \
        umull   x16, w3, w16 __LF                  \
        adds    x11, x11, x15, lsl #32 __LF        \
        lsr     x15, x15, #32 __LF                 \
        adc     x12, x12, x15 __LF                 \
        adds    x11, x11, x16, lsl #32 __LF        \
        lsr     x16, x16, #32 __LF                 \
        adc     x12, x12, x16 __LF                 \
        mul     x13, x4, x6 __LF                   \
        umulh   x14, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x16, cc __LF                       \
        adds    x13, x13, x12 __LF                 \
        adc     x14, x14, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, cc __LF                    \
        cinv    x16, x16, cc __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x12, x11, x13 __LF                 \
        adcs    x13, x13, x14 __LF                 \
        adc     x14, x14, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x15, x15, x16 __LF                 \
        adcs    x12, x15, x12 __LF                 \
        eor     x3, x3, x16 __LF                   \
        adcs    x13, x3, x13 __LF                  \
        adc     x14, x14, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x15, x16, [P1] __LF                \
        subs    x3, x3, x15 __LF                   \
        sbcs    x4, x4, x16 __LF                   \
        csetm   x16, cc __LF                       \
        ldp     x15, x0, [P2] __LF                 \
        subs    x5, x15, x5 __LF                   \
        sbcs    x6, x0, x6 __LF                    \
        csetm   x0, cc __LF                        \
        eor     x3, x3, x16 __LF                   \
        subs    x3, x3, x16 __LF                   \
        eor     x4, x4, x16 __LF                   \
        sbc     x4, x4, x16 __LF                   \
        eor     x5, x5, x0 __LF                    \
        subs    x5, x5, x0 __LF                    \
        eor     x6, x6, x0 __LF                    \
        sbc     x6, x6, x0 __LF                    \
        eor     x16, x0, x16 __LF                  \
        adds    x11, x11, x9 __LF                  \
        adcs    x12, x12, x10 __LF                 \
        adcs    x13, x13, xzr __LF                 \
        adc     x14, x14, xzr __LF                 \
        mul     x2, x3, x5 __LF                    \
        umulh   x0, x3, x5 __LF                    \
        mul     x15, x4, x6 __LF                   \
        umulh   x1, x4, x6 __LF                    \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x9, cc __LF                        \
        adds    x15, x15, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x6, x5, x6 __LF                    \
        cneg    x6, x6, cc __LF                    \
        cinv    x9, x9, cc __LF                    \
        mul     x5, x4, x6 __LF                    \
        umulh   x6, x4, x6 __LF                    \
        adds    x0, x2, x15 __LF                   \
        adcs    x15, x15, x1 __LF                  \
        adc     x1, x1, xzr __LF                   \
        cmn     x9, #0x1 __LF                      \
        eor     x5, x5, x9 __LF                    \
        adcs    x0, x5, x0 __LF                    \
        eor     x6, x6, x9 __LF                    \
        adcs    x15, x6, x15 __LF                  \
        adc     x1, x1, x9 __LF                    \
        adds    x9, x11, x7 __LF                   \
        adcs    x10, x12, x8 __LF                  \
        adcs    x11, x13, x11 __LF                 \
        adcs    x12, x14, x12 __LF                 \
        adcs    x13, x13, xzr __LF                 \
        adc     x14, x14, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x2, x2, x16 __LF                   \
        adcs    x9, x2, x9 __LF                    \
        eor     x0, x0, x16 __LF                   \
        adcs    x10, x0, x10 __LF                  \
        eor     x15, x15, x16 __LF                 \
        adcs    x11, x15, x11 __LF                 \
        eor     x1, x1, x16 __LF                   \
        adcs    x12, x1, x12 __LF                  \
        adcs    x13, x13, x16 __LF                 \
        adc     x14, x14, x16 __LF                 \
        mov     x3, #0x26 __LF                     \
        umull   x4, w11, w3 __LF                   \
        add     x4, x4, w7, uxtw __LF              \
        lsr     x7, x7, #32 __LF                   \
        lsr     x11, x11, #32 __LF                 \
        umaddl  x11, w11, w3, x7 __LF              \
        mov     x7, x4 __LF                        \
        umull   x4, w12, w3 __LF                   \
        add     x4, x4, w8, uxtw __LF              \
        lsr     x8, x8, #32 __LF                   \
        lsr     x12, x12, #32 __LF                 \
        umaddl  x12, w12, w3, x8 __LF              \
        mov     x8, x4 __LF                        \
        umull   x4, w13, w3 __LF                   \
        add     x4, x4, w9, uxtw __LF              \
        lsr     x9, x9, #32 __LF                   \
        lsr     x13, x13, #32 __LF                 \
        umaddl  x13, w13, w3, x9 __LF              \
        mov     x9, x4 __LF                        \
        umull   x4, w14, w3 __LF                   \
        add     x4, x4, w10, uxtw __LF             \
        lsr     x10, x10, #32 __LF                 \
        lsr     x14, x14, #32 __LF                 \
        umaddl  x14, w14, w3, x10 __LF             \
        mov     x10, x4 __LF                       \
        lsr     x0, x14, #31 __LF                  \
        mov     x5, #0x13 __LF                     \
        umaddl  x5, w5, w0, x5 __LF                \
        add     x7, x7, x5 __LF                    \
        adds    x7, x7, x11, lsl #32 __LF          \
        extr    x3, x12, x11, #32 __LF             \
        adcs    x8, x8, x3 __LF                    \
        extr    x3, x13, x12, #32 __LF             \
        adcs    x9, x9, x3 __LF                    \
        extr    x3, x14, x13, #32 __LF             \
        lsl     x5, x0, #63 __LF                   \
        eor     x10, x10, x5 __LF                  \
        adc     x10, x10, x3 __LF                  \
        mov     x3, #0x13 __LF                     \
        tst     x10, #0x8000000000000000 __LF      \
        csel    x3, x3, xzr, pl __LF               \
        subs    x7, x7, x3 __LF                    \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        and     x10, x10, #0x7fffffffffffffff __LF \
        stp     x7, x8, [P0] __LF                  \
        stp     x9, x10, [P0+16]

// A version of multiplication that only guarantees output < 2 * p_25519.
// This basically skips the +1 and final correction in quotient estimation.

#define mul_4(P0,P1,P2)                         \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        umull   x7, w3, w5 __LF                    \
        lsr     x0, x3, #32 __LF                   \
        umull   x15, w0, w5 __LF                   \
        lsr     x16, x5, #32 __LF                  \
        umull   x8, w16, w0 __LF                   \
        umull   x16, w3, w16 __LF                  \
        adds    x7, x7, x15, lsl #32 __LF          \
        lsr     x15, x15, #32 __LF                 \
        adc     x8, x8, x15 __LF                   \
        adds    x7, x7, x16, lsl #32 __LF          \
        lsr     x16, x16, #32 __LF                 \
        adc     x8, x8, x16 __LF                   \
        mul     x9, x4, x6 __LF                    \
        umulh   x10, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x16, cc __LF                       \
        adds    x9, x9, x8 __LF                    \
        adc     x10, x10, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, cc __LF                    \
        cinv    x16, x16, cc __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x8, x7, x9 __LF                    \
        adcs    x9, x9, x10 __LF                   \
        adc     x10, x10, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x15, x15, x16 __LF                 \
        adcs    x8, x15, x8 __LF                   \
        eor     x3, x3, x16 __LF                   \
        adcs    x9, x3, x9 __LF                    \
        adc     x10, x10, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x5, x6, [P2+16] __LF               \
        umull   x11, w3, w5 __LF                   \
        lsr     x0, x3, #32 __LF                   \
        umull   x15, w0, w5 __LF                   \
        lsr     x16, x5, #32 __LF                  \
        umull   x12, w16, w0 __LF                  \
        umull   x16, w3, w16 __LF                  \
        adds    x11, x11, x15, lsl #32 __LF        \
        lsr     x15, x15, #32 __LF                 \
        adc     x12, x12, x15 __LF                 \
        adds    x11, x11, x16, lsl #32 __LF        \
        lsr     x16, x16, #32 __LF                 \
        adc     x12, x12, x16 __LF                 \
        mul     x13, x4, x6 __LF                   \
        umulh   x14, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x16, cc __LF                       \
        adds    x13, x13, x12 __LF                 \
        adc     x14, x14, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, cc __LF                    \
        cinv    x16, x16, cc __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x12, x11, x13 __LF                 \
        adcs    x13, x13, x14 __LF                 \
        adc     x14, x14, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x15, x15, x16 __LF                 \
        adcs    x12, x15, x12 __LF                 \
        eor     x3, x3, x16 __LF                   \
        adcs    x13, x3, x13 __LF                  \
        adc     x14, x14, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x15, x16, [P1] __LF                \
        subs    x3, x3, x15 __LF                   \
        sbcs    x4, x4, x16 __LF                   \
        csetm   x16, cc __LF                       \
        ldp     x15, x0, [P2] __LF                 \
        subs    x5, x15, x5 __LF                   \
        sbcs    x6, x0, x6 __LF                    \
        csetm   x0, cc __LF                        \
        eor     x3, x3, x16 __LF                   \
        subs    x3, x3, x16 __LF                   \
        eor     x4, x4, x16 __LF                   \
        sbc     x4, x4, x16 __LF                   \
        eor     x5, x5, x0 __LF                    \
        subs    x5, x5, x0 __LF                    \
        eor     x6, x6, x0 __LF                    \
        sbc     x6, x6, x0 __LF                    \
        eor     x16, x0, x16 __LF                  \
        adds    x11, x11, x9 __LF                  \
        adcs    x12, x12, x10 __LF                 \
        adcs    x13, x13, xzr __LF                 \
        adc     x14, x14, xzr __LF                 \
        mul     x2, x3, x5 __LF                    \
        umulh   x0, x3, x5 __LF                    \
        mul     x15, x4, x6 __LF                   \
        umulh   x1, x4, x6 __LF                    \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, cc __LF                    \
        csetm   x9, cc __LF                        \
        adds    x15, x15, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x6, x5, x6 __LF                    \
        cneg    x6, x6, cc __LF                    \
        cinv    x9, x9, cc __LF                    \
        mul     x5, x4, x6 __LF                    \
        umulh   x6, x4, x6 __LF                    \
        adds    x0, x2, x15 __LF                   \
        adcs    x15, x15, x1 __LF                  \
        adc     x1, x1, xzr __LF                   \
        cmn     x9, #0x1 __LF                      \
        eor     x5, x5, x9 __LF                    \
        adcs    x0, x5, x0 __LF                    \
        eor     x6, x6, x9 __LF                    \
        adcs    x15, x6, x15 __LF                  \
        adc     x1, x1, x9 __LF                    \
        adds    x9, x11, x7 __LF                   \
        adcs    x10, x12, x8 __LF                  \
        adcs    x11, x13, x11 __LF                 \
        adcs    x12, x14, x12 __LF                 \
        adcs    x13, x13, xzr __LF                 \
        adc     x14, x14, xzr __LF                 \
        cmn     x16, #0x1 __LF                     \
        eor     x2, x2, x16 __LF                   \
        adcs    x9, x2, x9 __LF                    \
        eor     x0, x0, x16 __LF                   \
        adcs    x10, x0, x10 __LF                  \
        eor     x15, x15, x16 __LF                 \
        adcs    x11, x15, x11 __LF                 \
        eor     x1, x1, x16 __LF                   \
        adcs    x12, x1, x12 __LF                  \
        adcs    x13, x13, x16 __LF                 \
        adc     x14, x14, x16 __LF                 \
        mov     x3, #0x26 __LF                     \
        umull   x4, w11, w3 __LF                   \
        add     x4, x4, w7, uxtw __LF              \
        lsr     x7, x7, #32 __LF                   \
        lsr     x11, x11, #32 __LF                 \
        umaddl  x11, w11, w3, x7 __LF              \
        mov     x7, x4 __LF                        \
        umull   x4, w12, w3 __LF                   \
        add     x4, x4, w8, uxtw __LF              \
        lsr     x8, x8, #32 __LF                   \
        lsr     x12, x12, #32 __LF                 \
        umaddl  x12, w12, w3, x8 __LF              \
        mov     x8, x4 __LF                        \
        umull   x4, w13, w3 __LF                   \
        add     x4, x4, w9, uxtw __LF              \
        lsr     x9, x9, #32 __LF                   \
        lsr     x13, x13, #32 __LF                 \
        umaddl  x13, w13, w3, x9 __LF              \
        mov     x9, x4 __LF                        \
        umull   x4, w14, w3 __LF                   \
        add     x4, x4, w10, uxtw __LF             \
        lsr     x10, x10, #32 __LF                 \
        lsr     x14, x14, #32 __LF                 \
        umaddl  x14, w14, w3, x10 __LF             \
        mov     x10, x4 __LF                       \
        lsr     x0, x14, #31 __LF                  \
        mov     x5, #0x13 __LF                     \
        umull   x5, w5, w0 __LF                    \
        add     x7, x7, x5 __LF                    \
        adds    x7, x7, x11, lsl #32 __LF          \
        extr    x3, x12, x11, #32 __LF             \
        adcs    x8, x8, x3 __LF                    \
        extr    x3, x13, x12, #32 __LF             \
        adcs    x9, x9, x3 __LF                    \
        extr    x3, x14, x13, #32 __LF             \
        lsl     x5, x0, #63 __LF                   \
        eor     x10, x10, x5 __LF                  \
        adc     x10, x10, x3 __LF                  \
        stp     x7, x8, [P0] __LF                  \
        stp     x9, x10, [P0+16]

// Plain 4-digit add and doubling without any normalization
// With inputs < p_25519 (indeed < 2^255) it still gives a 4-digit result,
// indeed one < 2 * p_25519 for normalized inputs.

#define add_4(P0,P1,P2)                         \
        ldp     x0, x1, [P1] __LF                  \
        ldp     x4, x5, [P2] __LF                  \
        adds    x0, x0, x4 __LF                    \
        adcs    x1, x1, x5 __LF                    \
        ldp     x2, x3, [P1+16] __LF               \
        ldp     x6, x7, [P2+16] __LF               \
        adcs    x2, x2, x6 __LF                    \
        adc     x3, x3, x7 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

#define double_4(P0,P1)                         \
        ldp     x0, x1, [P1] __LF                  \
        adds    x0, x0, x0 __LF                    \
        adcs    x1, x1, x1 __LF                    \
        ldp     x2, x3, [P1+16] __LF               \
        adcs    x2, x2, x2 __LF                    \
        adc     x3, x3, x3 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

// Subtraction of a pair of numbers < p_25519 just sufficient
// to give a 4-digit result. It actually always does (x - z) + (2^255-19)
// which in turn is done by (x - z) - (2^255+19) discarding the 2^256
// implicitly

#define sub_4(P0,P1,P2)                         \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x3, #19 __LF                       \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        mov     x4, #0x8000000000000000 __LF       \
        sbc     x8, x8, x4 __LF                    \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(P0,P1,P2)                    \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #38 __LF                       \
        csel    x3, x4, xzr, lo __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// Modular addition with inputs double modulus 2 * p_25519 = 2^256 - 38
// and in general only guaranteeing a 4-digit result, not even < 2 * p_25519.

#define add_twice4(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, x8 __LF                    \
        ldp     x5, x6, [P1+16] __LF               \
        ldp     x7, x8, [P2+16] __LF               \
        adcs    x5, x5, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        mov     x9, #38 __LF                       \
        csel    x9, x9, xzr, cs __LF               \
        adds    x3, x3, x9 __LF                    \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adc     x6, x6, xzr __LF                   \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16]

S2N_BN_SYMBOL(edwards25519_pepadd):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     p3, x0
        mov     p1, x1
        mov     p2, x2

// Main sequence

        double_4(t0,z_1);

        sub_4(t1,y_1,x_1);
        add_4(t2,y_1,x_1);

        mul_4(t3,w_1,kxy_2);

        mul_4(t1,t1,ymx_2);
        mul_4(t2,t2,xpy_2);

        sub_twice4(t4,t0,t3);
        add_twice4(t0,t0,t3);
        sub_twice4(t5,t2,t1);
        add_twice4(t1,t2,t1);

        mul_p25519(z_3,t4,t0);
        mul_p25519(x_3,t5,t4);
        mul_p25519(y_3,t0,t1);
        mul_p25519(w_3,t5,t1);

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_pepadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
