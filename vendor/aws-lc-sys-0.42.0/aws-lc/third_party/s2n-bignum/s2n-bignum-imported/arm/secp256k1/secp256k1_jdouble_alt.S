// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on SECG curve secp256k1 in Jacobian coordinates
//
//    extern void secp256k1_jdouble_alt(uint64_t p3[static 12],
//                                      const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input point are fully
// reduced mod p_256k1 and that the z coordinate is not zero.
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(secp256k1_jdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(secp256k1_jdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(secp256k1_jdouble_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x15
#define input_x x16

// The magic constant 2^256 - p_256k1

#define pconst x17

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries

#define x_2 sp, #(NUMSIZE*0)
#define y_2 sp, #(NUMSIZE*1)
#define d sp, #(NUMSIZE*2)
#define tmp sp, #(NUMSIZE*3)
#define x_4 sp, #(NUMSIZE*4)
#define y_4 sp, #(NUMSIZE*6)
#define dx2 sp, #(NUMSIZE*8)
#define xy2 sp, #(NUMSIZE*10)

#define NSPACE NUMSIZE*12

// Corresponds exactly to bignum_mul_p256k1_alt except for
// re-use of the pconst register for the constant 4294968273

#define mul_p256k1(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        mul     x12, x3, x7 __LF                   \
        umulh   x13, x3, x7 __LF                   \
        mul     x11, x3, x8 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x9, x10, [P2+16] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x0, x3, x9 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x1, x3, x10 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        adc     x1, x1, xzr __LF                   \
        ldp     x5, x6, [P1+16] __LF               \
        mul     x11, x4, x7 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x4, x10 __LF                  \
        adcs    x1, x1, x11 __LF                   \
        umulh   x3, x4, x10 __LF                   \
        adc     x3, x3, xzr __LF                   \
        umulh   x11, x4, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        umulh   x11, x4, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        adc     x3, x3, xzr __LF                   \
        mul     x11, x5, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x5, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x5, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x5, x10 __LF                  \
        adcs    x3, x3, x11 __LF                   \
        umulh   x4, x5, x10 __LF                   \
        adc     x4, x4, xzr __LF                   \
        umulh   x11, x5, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        umulh   x11, x5, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        umulh   x11, x5, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adc     x4, x4, xzr __LF                   \
        mul     x11, x6, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        mul     x11, x6, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x6, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        mul     x11, x6, x10 __LF                  \
        adcs    x4, x4, x11 __LF                   \
        umulh   x5, x6, x10 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umulh   x11, x6, x7 __LF                   \
        adds    x1, x1, x11 __LF                   \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        mul     x11, pconst, x1 __LF               \
        umulh   x9, pconst, x1 __LF                \
        adds    x12, x12, x11 __LF                 \
        mul     x11, pconst, x3 __LF               \
        umulh   x3, pconst, x3 __LF                \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, pconst, x4 __LF               \
        umulh   x4, pconst, x4 __LF                \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, pconst, x5 __LF               \
        umulh   x5, pconst, x5 __LF                \
        adcs    x0, x0, x11 __LF                   \
        cset    x1, cs __LF                        \
        adds    x13, x13, x9 __LF                  \
        adcs    x14, x14, x3 __LF                  \
        adcs    x0, x0, x4 __LF                    \
        adc     x1, x1, x5 __LF                    \
        add     x8, x1, #0x1 __LF                  \
        mul     x11, pconst, x8 __LF               \
        umulh   x9, pconst, x8 __LF                \
        adds    x12, x12, x11 __LF                 \
        adcs    x13, x13, x9 __LF                  \
        adcs    x14, x14, xzr __LF                 \
        adcs    x0, x0, xzr __LF                   \
        csel    x7, pconst, xzr, cc __LF           \
        subs    x12, x12, x7 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x14, x14, xzr __LF                 \
        sbc     x0, x0, xzr __LF                   \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x0, [P0+16]

// Corresponds exactly to bignum_sqr_p256k1_alt except for
// re-use of the pconst register for the constant 4294968273

#define sqr_p256k1(P0,P1)                       \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x11, x2, x5 __LF                   \
        umulh   x12, x2, x5 __LF                   \
        mul     x7, x2, x4 __LF                    \
        umulh   x6, x2, x4 __LF                    \
        adds    x10, x10, x7 __LF                  \
        adcs    x11, x11, x6 __LF                  \
        mul     x7, x3, x4 __LF                    \
        umulh   x6, x3, x4 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x11, x11, x7 __LF                  \
        mul     x13, x4, x5 __LF                   \
        umulh   x14, x4, x5 __LF                   \
        adcs    x12, x12, x6 __LF                  \
        mul     x7, x3, x5 __LF                    \
        umulh   x6, x3, x5 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x12, x12, x7 __LF                  \
        adcs    x13, x13, x6 __LF                  \
        adc     x14, x14, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        cset    x6, cs __LF                        \
        umulh   x7, x2, x2 __LF                    \
        mul     x8, x2, x2 __LF                    \
        adds    x9, x9, x7 __LF                    \
        mul     x7, x3, x3 __LF                    \
        adcs    x10, x10, x7 __LF                  \
        umulh   x7, x3, x3 __LF                    \
        adcs    x11, x11, x7 __LF                  \
        mul     x7, x4, x4 __LF                    \
        adcs    x12, x12, x7 __LF                  \
        umulh   x7, x4, x4 __LF                    \
        adcs    x13, x13, x7 __LF                  \
        mul     x7, x5, x5 __LF                    \
        adcs    x14, x14, x7 __LF                  \
        umulh   x7, x5, x5 __LF                    \
        adc     x6, x6, x7 __LF                    \
        mul     x7, pconst, x12 __LF               \
        umulh   x4, pconst, x12 __LF               \
        adds    x8, x8, x7 __LF                    \
        mul     x7, pconst, x13 __LF               \
        umulh   x13, pconst, x13 __LF              \
        adcs    x9, x9, x7 __LF                    \
        mul     x7, pconst, x14 __LF               \
        umulh   x14, pconst, x14 __LF              \
        adcs    x10, x10, x7 __LF                  \
        mul     x7, pconst, x6 __LF                \
        umulh   x6, pconst, x6 __LF                \
        adcs    x11, x11, x7 __LF                  \
        cset    x12, cs __LF                       \
        adds    x9, x9, x4 __LF                    \
        adcs    x10, x10, x13 __LF                 \
        adcs    x11, x11, x14 __LF                 \
        adc     x12, x12, x6 __LF                  \
        add     x2, x12, #0x1 __LF                 \
        mul     x7, pconst, x2 __LF                \
        umulh   x6, pconst, x2 __LF                \
        adds    x8, x8, x7 __LF                    \
        adcs    x9, x9, x6 __LF                    \
        adcs    x10, x10, xzr __LF                 \
        adcs    x11, x11, xzr __LF                 \
        csel    x3, pconst, xzr, cc __LF           \
        subs    x8, x8, x3 __LF                    \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbc     x11, x11, xzr __LF                 \
        stp     x8, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16]

// Rough versions producing 5-word results

#define roughmul_p256k1(P0,P1,P2)               \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        mul     x12, x3, x7 __LF                   \
        umulh   x13, x3, x7 __LF                   \
        mul     x11, x3, x8 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x9, x10, [P2+16] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x0, x3, x9 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x1, x3, x10 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        adc     x1, x1, xzr __LF                   \
        ldp     x5, x6, [P1+16] __LF               \
        mul     x11, x4, x7 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x4, x10 __LF                  \
        adcs    x1, x1, x11 __LF                   \
        umulh   x3, x4, x10 __LF                   \
        adc     x3, x3, xzr __LF                   \
        umulh   x11, x4, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        umulh   x11, x4, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        adc     x3, x3, xzr __LF                   \
        mul     x11, x5, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x5, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x5, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x5, x10 __LF                  \
        adcs    x3, x3, x11 __LF                   \
        umulh   x4, x5, x10 __LF                   \
        adc     x4, x4, xzr __LF                   \
        umulh   x11, x5, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        umulh   x11, x5, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        umulh   x11, x5, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adc     x4, x4, xzr __LF                   \
        mul     x11, x6, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        mul     x11, x6, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x6, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        mul     x11, x6, x10 __LF                  \
        adcs    x4, x4, x11 __LF                   \
        umulh   x5, x6, x10 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umulh   x11, x6, x7 __LF                   \
        adds    x1, x1, x11 __LF                   \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        mul     x11, pconst, x1 __LF               \
        umulh   x9, pconst, x1 __LF                \
        adds    x12, x12, x11 __LF                 \
        mul     x11, pconst, x3 __LF               \
        umulh   x3, pconst, x3 __LF                \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, pconst, x4 __LF               \
        umulh   x4, pconst, x4 __LF                \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, pconst, x5 __LF               \
        umulh   x5, pconst, x5 __LF                \
        adcs    x0, x0, x11 __LF                   \
        cset    x1, cs __LF                        \
        adds    x13, x13, x9 __LF                  \
        adcs    x14, x14, x3 __LF                  \
        adcs    x0, x0, x4 __LF                    \
        adc     x1, x1, x5 __LF                    \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x0, [P0+16] __LF              \
        str     x1, [P0+32]

#define roughsqr_p256k1(P0,P1)                  \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x11, x2, x5 __LF                   \
        umulh   x12, x2, x5 __LF                   \
        mul     x7, x2, x4 __LF                    \
        umulh   x6, x2, x4 __LF                    \
        adds    x10, x10, x7 __LF                  \
        adcs    x11, x11, x6 __LF                  \
        mul     x7, x3, x4 __LF                    \
        umulh   x6, x3, x4 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x11, x11, x7 __LF                  \
        mul     x13, x4, x5 __LF                   \
        umulh   x14, x4, x5 __LF                   \
        adcs    x12, x12, x6 __LF                  \
        mul     x7, x3, x5 __LF                    \
        umulh   x6, x3, x5 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x12, x12, x7 __LF                  \
        adcs    x13, x13, x6 __LF                  \
        adc     x14, x14, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        cset    x6, cs __LF                        \
        umulh   x7, x2, x2 __LF                    \
        mul     x8, x2, x2 __LF                    \
        adds    x9, x9, x7 __LF                    \
        mul     x7, x3, x3 __LF                    \
        adcs    x10, x10, x7 __LF                  \
        umulh   x7, x3, x3 __LF                    \
        adcs    x11, x11, x7 __LF                  \
        mul     x7, x4, x4 __LF                    \
        adcs    x12, x12, x7 __LF                  \
        umulh   x7, x4, x4 __LF                    \
        adcs    x13, x13, x7 __LF                  \
        mul     x7, x5, x5 __LF                    \
        adcs    x14, x14, x7 __LF                  \
        umulh   x7, x5, x5 __LF                    \
        adc     x6, x6, x7 __LF                    \
        mul     x7, pconst, x12 __LF               \
        umulh   x4, pconst, x12 __LF               \
        adds    x8, x8, x7 __LF                    \
        mul     x7, pconst, x13 __LF               \
        umulh   x13, pconst, x13 __LF              \
        adcs    x9, x9, x7 __LF                    \
        mul     x7, pconst, x14 __LF               \
        umulh   x14, pconst, x14 __LF              \
        adcs    x10, x10, x7 __LF                  \
        mul     x7, pconst, x6 __LF                \
        umulh   x6, pconst, x6 __LF                \
        adcs    x11, x11, x7 __LF                  \
        cset    x12, cs __LF                       \
        adds    x9, x9, x4 __LF                    \
        adcs    x10, x10, x13 __LF                 \
        adcs    x11, x11, x14 __LF                 \
        adc     x12, x12, x6 __LF                  \
        stp     x8, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16] __LF             \
        str     x12, [P0+32]

// Weak doubling operation, staying in 4 digits but not in general
// fully normalizing modulo p_256k1

#define weakdouble_p256k1(P0,P1)                \
        ldp     x1, x2, [P1] __LF                  \
        lsl     x0, x1, #1 __LF                    \
        ldp     x3, x4, [P1+16] __LF               \
        ands    xzr, x4, #0x8000000000000000 __LF  \
        csel    x5, pconst, xzr, ne __LF           \
        extr    x1, x2, x1, #63 __LF               \
        adds    x0, x0, x5 __LF                    \
        extr    x2, x3, x2, #63 __LF               \
        adcs    x1, x1, xzr __LF                   \
        extr    x3, x4, x3, #63 __LF               \
        adcs    x2, x2, xzr __LF                   \
        stp     x0, x1, [P0] __LF                  \
        adc     x3, x3, xzr __LF                   \
        stp     x2, x3, [P0+16]

// P0 = C * P1 - D * P2 with 5-word inputs P1 and P2
// Only used here with C = 12, D = 9, but could be used more generally.
// We start with (2^40 * 2^256 + C * P1) - (D * P2 + 2^40 * k)
// where p_256k1 = 2^256 - k (so k = 4294968273)

#define cmsub_p256k1(P0,C,P1,D,P2)              \
        mov     x10, C __LF                        \
        ldp     x4, x5, [P1] __LF                  \
        mul     x0, x4, x10 __LF                   \
        mul     x1, x5, x10 __LF                   \
        ldp     x6, x7, [P1+16] __LF               \
        mul     x2, x6, x10 __LF                   \
        mul     x3, x7, x10 __LF                   \
        ldr     x13, [P1+32] __LF                  \
        umulh   x4, x4, x10 __LF                   \
        adds    x1, x1, x4 __LF                    \
        umulh   x5, x5, x10 __LF                   \
        adcs    x2, x2, x5 __LF                    \
        umulh   x6, x6, x10 __LF                   \
        adcs    x3, x3, x6 __LF                    \
        umulh   x4, x7, x10 __LF                   \
        mul     x13, x13, x10 __LF                 \
        adc     x9, x4, x13 __LF                   \
        orr     x9, x9, #0x10000000000 __LF        \
        /* [x9; x3;x2;x1;x0] = 2^40 * 2^256 + C * P1 */ \
        mov     x10, D __LF                        \
        ldp     x13, x14, [P2] __LF                \
        mul     x5, x14, x10 __LF                  \
        umulh   x6, x14, x10 __LF                  \
        adds    x5, x5, pconst, lsr #24 __LF       \
        adc     x6, x6, xzr __LF                   \
        mul     x4, x13, x10 __LF                  \
        adds    x4, x4, pconst, lsl #40 __LF       \
        umulh   x13, x13, x10 __LF                 \
        adcs    x5, x5, x13 __LF                   \
        ldp     x13, x14, [P2+16] __LF             \
        mul     x12, x13, x10 __LF                 \
        umulh   x7, x13, x10 __LF                  \
        ldr     x13, [P2+32] __LF                  \
        adcs    x6, x6, x12 __LF                   \
        mul     x12, x14, x10 __LF                 \
        umulh   x8, x14, x10 __LF                  \
        mul     x13, x13, x10 __LF                 \
        adcs    x7, x7, x12 __LF                   \
        adc     x8, x8, x13 __LF                   \
        /* [x8; x7;x6;x5;x4] = D * P2 + 2^40 * k */ \
        subs    x0, x0, x4 __LF                    \
        sbcs    x1, x1, x5 __LF                    \
        sbcs    x2, x2, x6 __LF                    \
        sbcs    x3, x3, x7 __LF                    \
        sbc     x4, x9, x8 __LF                    \
        /* [x4; x3;x2;x1;x0] = 2^40*p_256k1+result */ \
        add     x10, x4, #1 __LF                   \
        /* (h + 1) is quotient estimate */      \
        mul     x4, pconst, x10 __LF               \
        umulh   x5, pconst, x10 __LF               \
        adds    x0, x0, x4 __LF                    \
        adcs    x1, x1, x5 __LF                    \
        adcs    x2, x2, xzr __LF                   \
        adcs    x3, x3, xzr __LF                   \
        csel    x11, pconst, xzr, cc __LF          \
        /* If un-correction needed */           \
        subs    x0, x0, x11 __LF                   \
        sbcs    x1, x1, xzr __LF                   \
        stp     x0, x1, [P0] __LF                  \
        sbcs    x2, x2, xzr __LF                   \
        sbc     x3, x3, xzr __LF                   \
        stp     x2, x3, [P0+16]

// P0 = 3 * P1 - 8 * P2 with 5-digit P1 and P2
// We start with (2^40 * 2^256 + 3 * P1) - (8 * P2 + 2^40 * k)
// where p_256k1 = 2^256 - k (so k = 4294968273)

#define cmsub38_p256k1(P0,P1,P2)                \
        mov     x10, #3 __LF                       \
        ldp     x4, x5, [P1] __LF                  \
        mul     x0, x4, x10 __LF                   \
        mul     x1, x5, x10 __LF                   \
        ldp     x6, x7, [P1+16] __LF               \
        mul     x2, x6, x10 __LF                   \
        mul     x3, x7, x10 __LF                   \
        ldr     x13, [P1+32] __LF                  \
        umulh   x4, x4, x10 __LF                   \
        adds    x1, x1, x4 __LF                    \
        umulh   x5, x5, x10 __LF                   \
        adcs    x2, x2, x5 __LF                    \
        umulh   x6, x6, x10 __LF                   \
        adcs    x3, x3, x6 __LF                    \
        umulh   x4, x7, x10 __LF                   \
        mul     x13, x13, x10 __LF                 \
        adc     x9, x4, x13 __LF                   \
        orr     x9, x9, #0x10000000000 __LF        \
        /*  [x9; x3;x2;x1;x0] = 2^40 * 2^256 + 3 * P1 */ \
        lsl     x12, pconst, #40 __LF              \
        ldp     x13, x14, [P2] __LF                \
        lsl     x4, x13, #3 __LF                   \
        adds    x4, x4, x12 __LF                   \
        extr    x5, x14, x13, #61 __LF             \
        lsr     x12, pconst, #24 __LF              \
        adcs    x5, x5, x12 __LF                   \
        ldp     x11, x12, [P2+16] __LF             \
        extr    x6, x11, x14, #61 __LF             \
        adcs    x6, x6, xzr __LF                   \
        ldr     x13, [P2+32] __LF                  \
        extr    x7, x12, x11, #61 __LF             \
        adcs    x7, x7, xzr __LF                   \
        extr    x8, x13, x12, #61 __LF             \
        adc     x8, x8, xzr __LF                   \
        /* [x8; x7;x6;x5;x4] = 8 * P2 + 2^40 * k */ \
        subs    x0, x0, x4 __LF                    \
        sbcs    x1, x1, x5 __LF                    \
        sbcs    x2, x2, x6 __LF                    \
        sbcs    x3, x3, x7 __LF                    \
        sbc     x4, x9, x8 __LF                    \
        /* [x4; x3;x2;x1;x0] = 2^40*p_256k1+result */ \
        add     x10, x4, #1 __LF                   \
        /* (h + 1) is quotient estimate */      \
        mul     x4, pconst, x10 __LF               \
        umulh   x5, pconst, x10 __LF               \
        adds    x0, x0, x4 __LF                    \
        adcs    x1, x1, x5 __LF                    \
        adcs    x2, x2, xzr __LF                   \
        adcs    x3, x3, xzr __LF                   \
        csel    x11, pconst, xzr, cc __LF          \
        /*  If un-correction needed */          \
        subs    x0, x0, x11 __LF                   \
        sbcs    x1, x1, xzr __LF                   \
        stp     x0, x1, [P0] __LF                  \
        sbcs    x2, x2, xzr __LF                   \
        sbc     x3, x3, xzr __LF                   \
        stp     x2, x3, [P0+16]

// P0 = 4 * P1 - P2 with 5-digit P1, 4-digit P2 and result.
// This is done by direct subtraction of P2 since the method
// in bignum_cmul_p256k1 etc. for quotient estimation still
// works when the value to be reduced is negative, as
// long as it is  > -p_256k1, which is the case here.

#define cmsub41_p256k1(P0,P1,P2)                \
        ldp     x1, x2, [P1] __LF                  \
        lsl     x0, x1, #2 __LF                    \
        ldp     x6, x7, [P2] __LF                  \
        subs    x0, x0, x6 __LF                    \
        extr    x1, x2, x1, #62 __LF               \
        sbcs    x1, x1, x7 __LF                    \
        ldp     x3, x4, [P1+16] __LF               \
        extr    x2, x3, x2, #62 __LF               \
        ldp     x6, x7, [P2+16] __LF               \
        sbcs    x2, x2, x6 __LF                    \
        extr    x3, x4, x3, #62 __LF               \
        sbcs    x3, x3, x7 __LF                    \
        ldr     x5, [P1+32] __LF                   \
        extr    x4, x5, x4, #62 __LF               \
        sbc     x4, x4, xzr __LF                   \
        add     x5, x4, #1 __LF                    \
        /* (h + 1) is quotient estimate */      \
        mul     x4, pconst, x5 __LF                \
        adds    x0, x0, x4 __LF                    \
        umulh   x5, pconst, x5 __LF                \
        adcs    x1, x1, x5 __LF                    \
        adcs    x2, x2, xzr __LF                   \
        adcs    x3, x3, xzr __LF                   \
        csel    x4, pconst, xzr, cc __LF           \
        /*  If un-correction needed */          \
        subs    x0, x0, x4 __LF                    \
        sbcs    x1, x1, xzr __LF                   \
        stp     x0, x1, [P0] __LF                  \
        sbcs    x2, x2, xzr __LF                   \
        sbc     x3, x3, xzr __LF                   \
        stp     x2, x3, [P0+16]

S2N_BN_SYMBOL(secp256k1_jdouble_alt):
        CFI_START

// Make room on stack for temp registers

        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable place

        mov     input_z, x0
        mov     input_x, x1

// Set up pconst =  4294968273, so p_256k1 = 2^256 - pconst

        mov     pconst, #977
        orr     pconst, pconst, #0x100000000

// Main sequence of operations

        // y_2 = y^2

        sqr_p256k1(y_2,y_1)

        // x_2 = x^2

        sqr_p256k1(x_2,x_1)

        // tmp = 2 * y_1 (in 4 words but not fully normalized)

        weakdouble_p256k1(tmp,y_1)

        // xy2 = x * y^2 (5-digit partially reduced)
        // x_4 = x^4 (5-digit partially reduced)

        roughmul_p256k1(xy2,x_1,y_2)
        roughsqr_p256k1(x_4,x_2)

        // z_3 = 2 * y_1 * z_1

        mul_p256k1(z_3,z_1,tmp)

        // d = 12 * xy2 - 9 * x_4

        cmsub_p256k1(d,12,xy2,9,x_4)

        // y4 = y2^2 (5-digit partially reduced)

        roughsqr_p256k1(y_4,y_2)

        // dx2 = d * x_2 (5-digit partially reduced)

        roughmul_p256k1(dx2,x_2,d)

        // x_3 = 4 * xy2 - d

        cmsub41_p256k1(x_3,xy2,d)

        // y_3 = 3 * dx2 - 8 * y_4

        cmsub38_p256k1(y_3,dx2,y_4)

// Restore stack and return

        CFI_INC_SP(NSPACE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(secp256k1_jdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
