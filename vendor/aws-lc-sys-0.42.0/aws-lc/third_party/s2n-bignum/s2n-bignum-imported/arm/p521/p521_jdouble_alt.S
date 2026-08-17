// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jdouble_alt(uint64_t p3[static 27],
//                                 const uint64_t p1[static 27]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input point are fully
// reduced mod p_521 and that the z coordinate is not zero.
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jdouble_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence

#define input_z x26
#define input_x x27

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries

#define z2 sp, #(NUMSIZE*0)
#define y2 sp, #(NUMSIZE*1)
#define x2p sp, #(NUMSIZE*2)
#define xy2 sp, #(NUMSIZE*3)

#define y4 sp, #(NUMSIZE*4)
#define t2 sp, #(NUMSIZE*4)

#define dx2 sp, #(NUMSIZE*5)
#define t1 sp, #(NUMSIZE*5)

#define d sp, #(NUMSIZE*6)
#define x4p sp, #(NUMSIZE*6)

// NUMSIZE*7 is not 16-aligned so we round it up

#define NSPACE 512

// Corresponds exactly to bignum_mul_p521_alt

#define mul_p521(P0,P1,P2)                      \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        mul     x15, x3, x5 __LF                   \
        umulh   x16, x3, x5 __LF                   \
        mul     x14, x3, x6 __LF                   \
        umulh   x17, x3, x6 __LF                   \
        adds    x16, x16, x14 __LF                 \
        ldp     x7, x8, [P2+16] __LF               \
        mul     x14, x3, x7 __LF                   \
        umulh   x19, x3, x7 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        umulh   x20, x3, x8 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        ldp     x9, x10, [P2+32] __LF              \
        mul     x14, x3, x9 __LF                   \
        umulh   x21, x3, x9 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        umulh   x22, x3, x10 __LF                  \
        adcs    x21, x21, x14 __LF                 \
        ldp     x11, x12, [P2+48] __LF             \
        mul     x14, x3, x11 __LF                  \
        umulh   x23, x3, x11 __LF                  \
        adcs    x22, x22, x14 __LF                 \
        ldr     x13, [P2+64] __LF                  \
        mul     x14, x3, x12 __LF                  \
        umulh   x24, x3, x12 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        umulh   x1, x3, x13 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        adc     x1, x1, xzr __LF                   \
        mul     x14, x4, x5 __LF                   \
        adds    x16, x16, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        cset    x0, hs __LF                        \
        umulh   x14, x4, x5 __LF                   \
        adds    x17, x17, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x13 __LF                  \
        adc     x0, x0, x14 __LF                   \
        stp     x15, x16, [P0] __LF                \
        ldp     x3, x4, [P1+16] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x17, x17, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x13 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        cset    x15, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x19, x19, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x12 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x13 __LF                  \
        adc     x15, x15, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x19, x19, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x12 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x13 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        cset    x16, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x20, x20, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x11 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x12 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x16, x16, x14 __LF                 \
        stp     x17, x19, [P0+16] __LF             \
        ldp     x3, x4, [P1+32] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x20, x20, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x11 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x12 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        cset    x17, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x21, x21, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x10 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x11 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x13 __LF                  \
        adc     x17, x17, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x21, x21, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x10 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x11 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        cset    x19, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x22, x22, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x9 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x10 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x19, x19, x14 __LF                 \
        stp     x20, x21, [P0+32] __LF             \
        ldp     x3, x4, [P1+48] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x22, x22, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x9 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x10 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        cset    x20, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x23, x23, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x9 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x3, x13 __LF                  \
        adc     x20, x20, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x23, x23, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x8 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x9 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        cset    x21, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x24, x24, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x7 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x8 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x21, x21, x14 __LF                 \
        stp     x22, x23, [P0+48] __LF             \
        ldr     x3, [P1+64] __LF                   \
        mul     x14, x3, x5 __LF                   \
        adds    x24, x24, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x7 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x8 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adc     x21, x21, x14 __LF                 \
        umulh   x14, x3, x5 __LF                   \
        adds    x1, x1, x14 __LF                   \
        umulh   x14, x3, x6 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x7 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adc     x21, x21, x14 __LF                 \
        cmp     xzr, xzr __LF                      \
        ldp     x5, x6, [P0] __LF                  \
        extr    x14, x1, x24, #9 __LF              \
        adcs    x5, x5, x14 __LF                   \
        extr    x14, x0, x1, #9 __LF               \
        adcs    x6, x6, x14 __LF                   \
        ldp     x7, x8, [P0+16] __LF               \
        extr    x14, x15, x0, #9 __LF              \
        adcs    x7, x7, x14 __LF                   \
        extr    x14, x16, x15, #9 __LF             \
        adcs    x8, x8, x14 __LF                   \
        ldp     x9, x10, [P0+32] __LF              \
        extr    x14, x17, x16, #9 __LF             \
        adcs    x9, x9, x14 __LF                   \
        extr    x14, x19, x17, #9 __LF             \
        adcs    x10, x10, x14 __LF                 \
        ldp     x11, x12, [P0+48] __LF             \
        extr    x14, x20, x19, #9 __LF             \
        adcs    x11, x11, x14 __LF                 \
        extr    x14, x21, x20, #9 __LF             \
        adcs    x12, x12, x14 __LF                 \
        orr     x13, x24, #0xfffffffffffffe00 __LF \
        lsr     x14, x21, #9 __LF                  \
        adcs    x13, x13, x14 __LF                 \
        sbcs    x5, x5, xzr __LF                   \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, xzr __LF                 \
        and     x13, x13, #0x1ff __LF              \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// Corresponds exactly to bignum_sqr_p521_alt

#define sqr_p521(P0,P1)                         \
        ldp     x2, x3, [P1] __LF                  \
        mul     x11, x2, x3 __LF                   \
        umulh   x12, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x10, x2, x4 __LF                   \
        umulh   x13, x2, x4 __LF                   \
        adds    x12, x12, x10 __LF                 \
        ldp     x6, x7, [P1+32] __LF               \
        mul     x10, x2, x5 __LF                   \
        umulh   x14, x2, x5 __LF                   \
        adcs    x13, x13, x10 __LF                 \
        ldp     x8, x9, [P1+48] __LF               \
        mul     x10, x2, x6 __LF                   \
        umulh   x15, x2, x6 __LF                   \
        adcs    x14, x14, x10 __LF                 \
        mul     x10, x2, x7 __LF                   \
        umulh   x16, x2, x7 __LF                   \
        adcs    x15, x15, x10 __LF                 \
        mul     x10, x2, x8 __LF                   \
        umulh   x17, x2, x8 __LF                   \
        adcs    x16, x16, x10 __LF                 \
        mul     x10, x2, x9 __LF                   \
        umulh   x19, x2, x9 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        adc     x19, x19, xzr __LF                 \
        mul     x10, x3, x4 __LF                   \
        adds    x13, x13, x10 __LF                 \
        mul     x10, x3, x5 __LF                   \
        adcs    x14, x14, x10 __LF                 \
        mul     x10, x3, x6 __LF                   \
        adcs    x15, x15, x10 __LF                 \
        mul     x10, x3, x7 __LF                   \
        adcs    x16, x16, x10 __LF                 \
        mul     x10, x3, x8 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        mul     x10, x3, x9 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        cset    x20, hs __LF                       \
        umulh   x10, x3, x4 __LF                   \
        adds    x14, x14, x10 __LF                 \
        umulh   x10, x3, x5 __LF                   \
        adcs    x15, x15, x10 __LF                 \
        umulh   x10, x3, x6 __LF                   \
        adcs    x16, x16, x10 __LF                 \
        umulh   x10, x3, x7 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        umulh   x10, x3, x8 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        umulh   x10, x3, x9 __LF                   \
        adc     x20, x20, x10 __LF                 \
        mul     x10, x6, x7 __LF                   \
        umulh   x21, x6, x7 __LF                   \
        adds    x20, x20, x10 __LF                 \
        adc     x21, x21, xzr __LF                 \
        mul     x10, x4, x5 __LF                   \
        adds    x15, x15, x10 __LF                 \
        mul     x10, x4, x6 __LF                   \
        adcs    x16, x16, x10 __LF                 \
        mul     x10, x4, x7 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        mul     x10, x4, x8 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        mul     x10, x4, x9 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        mul     x10, x6, x8 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        cset    x22, hs __LF                       \
        umulh   x10, x4, x5 __LF                   \
        adds    x16, x16, x10 __LF                 \
        umulh   x10, x4, x6 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        umulh   x10, x4, x7 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        umulh   x10, x4, x8 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        umulh   x10, x4, x9 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        umulh   x10, x6, x8 __LF                   \
        adc     x22, x22, x10 __LF                 \
        mul     x10, x7, x8 __LF                   \
        umulh   x23, x7, x8 __LF                   \
        adds    x22, x22, x10 __LF                 \
        adc     x23, x23, xzr __LF                 \
        mul     x10, x5, x6 __LF                   \
        adds    x17, x17, x10 __LF                 \
        mul     x10, x5, x7 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        mul     x10, x5, x8 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        mul     x10, x5, x9 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        mul     x10, x6, x9 __LF                   \
        adcs    x22, x22, x10 __LF                 \
        mul     x10, x7, x9 __LF                   \
        adcs    x23, x23, x10 __LF                 \
        cset    x24, hs __LF                       \
        umulh   x10, x5, x6 __LF                   \
        adds    x19, x19, x10 __LF                 \
        umulh   x10, x5, x7 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        umulh   x10, x5, x8 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        umulh   x10, x5, x9 __LF                   \
        adcs    x22, x22, x10 __LF                 \
        umulh   x10, x6, x9 __LF                   \
        adcs    x23, x23, x10 __LF                 \
        umulh   x10, x7, x9 __LF                   \
        adc     x24, x24, x10 __LF                 \
        mul     x10, x8, x9 __LF                   \
        umulh   x25, x8, x9 __LF                   \
        adds    x24, x24, x10 __LF                 \
        adc     x25, x25, xzr __LF                 \
        adds    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        adcs    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adcs    x17, x17, x17 __LF                 \
        adcs    x19, x19, x19 __LF                 \
        adcs    x20, x20, x20 __LF                 \
        adcs    x21, x21, x21 __LF                 \
        adcs    x22, x22, x22 __LF                 \
        adcs    x23, x23, x23 __LF                 \
        adcs    x24, x24, x24 __LF                 \
        adcs    x25, x25, x25 __LF                 \
        cset    x0, hs __LF                        \
        umulh   x10, x2, x2 __LF                   \
        adds    x11, x11, x10 __LF                 \
        mul     x10, x3, x3 __LF                   \
        adcs    x12, x12, x10 __LF                 \
        umulh   x10, x3, x3 __LF                   \
        adcs    x13, x13, x10 __LF                 \
        mul     x10, x4, x4 __LF                   \
        adcs    x14, x14, x10 __LF                 \
        umulh   x10, x4, x4 __LF                   \
        adcs    x15, x15, x10 __LF                 \
        mul     x10, x5, x5 __LF                   \
        adcs    x16, x16, x10 __LF                 \
        umulh   x10, x5, x5 __LF                   \
        adcs    x17, x17, x10 __LF                 \
        mul     x10, x6, x6 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        umulh   x10, x6, x6 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        mul     x10, x7, x7 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        umulh   x10, x7, x7 __LF                   \
        adcs    x22, x22, x10 __LF                 \
        mul     x10, x8, x8 __LF                   \
        adcs    x23, x23, x10 __LF                 \
        umulh   x10, x8, x8 __LF                   \
        adcs    x24, x24, x10 __LF                 \
        mul     x10, x9, x9 __LF                   \
        adcs    x25, x25, x10 __LF                 \
        umulh   x10, x9, x9 __LF                   \
        adc     x0, x0, x10 __LF                   \
        ldr     x1, [P1+64] __LF                   \
        add     x1, x1, x1 __LF                    \
        mul     x10, x1, x2 __LF                   \
        adds    x19, x19, x10 __LF                 \
        umulh   x10, x1, x2 __LF                   \
        adcs    x20, x20, x10 __LF                 \
        mul     x10, x1, x4 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        umulh   x10, x1, x4 __LF                   \
        adcs    x22, x22, x10 __LF                 \
        mul     x10, x1, x6 __LF                   \
        adcs    x23, x23, x10 __LF                 \
        umulh   x10, x1, x6 __LF                   \
        adcs    x24, x24, x10 __LF                 \
        mul     x10, x1, x8 __LF                   \
        adcs    x25, x25, x10 __LF                 \
        umulh   x10, x1, x8 __LF                   \
        adcs    x0, x0, x10 __LF                   \
        lsr     x4, x1, #1 __LF                    \
        mul     x4, x4, x4 __LF                    \
        adc     x4, x4, xzr __LF                   \
        mul     x10, x1, x3 __LF                   \
        adds    x20, x20, x10 __LF                 \
        umulh   x10, x1, x3 __LF                   \
        adcs    x21, x21, x10 __LF                 \
        mul     x10, x1, x5 __LF                   \
        adcs    x22, x22, x10 __LF                 \
        umulh   x10, x1, x5 __LF                   \
        adcs    x23, x23, x10 __LF                 \
        mul     x10, x1, x7 __LF                   \
        adcs    x24, x24, x10 __LF                 \
        umulh   x10, x1, x7 __LF                   \
        adcs    x25, x25, x10 __LF                 \
        mul     x10, x1, x9 __LF                   \
        adcs    x0, x0, x10 __LF                   \
        umulh   x10, x1, x9 __LF                   \
        adc     x4, x4, x10 __LF                   \
        mul     x2, x2, x2 __LF                    \
        cmp     xzr, xzr __LF                      \
        extr    x10, x20, x19, #9 __LF             \
        adcs    x2, x2, x10 __LF                   \
        extr    x10, x21, x20, #9 __LF             \
        adcs    x11, x11, x10 __LF                 \
        extr    x10, x22, x21, #9 __LF             \
        adcs    x12, x12, x10 __LF                 \
        extr    x10, x23, x22, #9 __LF             \
        adcs    x13, x13, x10 __LF                 \
        extr    x10, x24, x23, #9 __LF             \
        adcs    x14, x14, x10 __LF                 \
        extr    x10, x25, x24, #9 __LF             \
        adcs    x15, x15, x10 __LF                 \
        extr    x10, x0, x25, #9 __LF              \
        adcs    x16, x16, x10 __LF                 \
        extr    x10, x4, x0, #9 __LF               \
        adcs    x17, x17, x10 __LF                 \
        orr     x19, x19, #0xfffffffffffffe00 __LF \
        lsr     x10, x4, #9 __LF                   \
        adcs    x19, x19, x10 __LF                 \
        sbcs    x2, x2, xzr __LF                   \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x14, x14, xzr __LF                 \
        sbcs    x15, x15, xzr __LF                 \
        sbcs    x16, x16, xzr __LF                 \
        sbcs    x17, x17, xzr __LF                 \
        sbc     x19, x19, xzr __LF                 \
        and     x19, x19, #0x1ff __LF              \
        stp     x2, x11, [P0] __LF                 \
        stp     x12, x13, [P0+16] __LF             \
        stp     x14, x15, [P0+32] __LF             \
        stp     x16, x17, [P0+48] __LF             \
        str     x19, [P0+64]

// Corresponds exactly to bignum_add_p521

#define add_p521(P0,P1,P2)                      \
        cmp     xzr, xzr __LF                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adcs    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        adcs    x9, x9, x4 __LF                    \
        adcs    x10, x10, x3 __LF                  \
        ldp     x11, x12, [P1+48] __LF             \
        ldp     x4, x3, [P2+48] __LF               \
        adcs    x11, x11, x4 __LF                  \
        adcs    x12, x12, x3 __LF                  \
        ldr     x13, [P1+64] __LF                  \
        ldr     x4, [P2+64] __LF                   \
        adc     x13, x13, x4 __LF                  \
        subs    x4, x13, #512 __LF                 \
        csetm   x4, hs __LF                        \
        sbcs    x5, x5, xzr __LF                   \
        and     x4, x4, #0x200 __LF                \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, x4 __LF                  \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// Corresponds exactly to bignum_sub_p521

#define sub_p521(P0,P1,P2)                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        sbcs    x9, x9, x4 __LF                    \
        sbcs    x10, x10, x3 __LF                  \
        ldp     x11, x12, [P1+48] __LF             \
        ldp     x4, x3, [P2+48] __LF               \
        sbcs    x11, x11, x4 __LF                  \
        sbcs    x12, x12, x3 __LF                  \
        ldr     x13, [P1+64] __LF                  \
        ldr     x4, [P2+64] __LF                   \
        sbcs    x13, x13, x4 __LF                  \
        sbcs    x5, x5, xzr __LF                   \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        and     x13, x13, #0x1ff __LF              \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// Weak multiplication not fully reducing

#define weakmul_p521(P0,P1,P2)                  \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        mul     x15, x3, x5 __LF                   \
        umulh   x16, x3, x5 __LF                   \
        mul     x14, x3, x6 __LF                   \
        umulh   x17, x3, x6 __LF                   \
        adds    x16, x16, x14 __LF                 \
        ldp     x7, x8, [P2+16] __LF               \
        mul     x14, x3, x7 __LF                   \
        umulh   x19, x3, x7 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        umulh   x20, x3, x8 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        ldp     x9, x10, [P2+32] __LF              \
        mul     x14, x3, x9 __LF                   \
        umulh   x21, x3, x9 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        umulh   x22, x3, x10 __LF                  \
        adcs    x21, x21, x14 __LF                 \
        ldp     x11, x12, [P2+48] __LF             \
        mul     x14, x3, x11 __LF                  \
        umulh   x23, x3, x11 __LF                  \
        adcs    x22, x22, x14 __LF                 \
        ldr     x13, [P2+64] __LF                  \
        mul     x14, x3, x12 __LF                  \
        umulh   x24, x3, x12 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        umulh   x1, x3, x13 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        adc     x1, x1, xzr __LF                   \
        mul     x14, x4, x5 __LF                   \
        adds    x16, x16, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        cset    x0, hs __LF                        \
        umulh   x14, x4, x5 __LF                   \
        adds    x17, x17, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x13 __LF                  \
        adc     x0, x0, x14 __LF                   \
        stp     x15, x16, [P0] __LF                \
        ldp     x3, x4, [P1+16] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x17, x17, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x13 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        cset    x15, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x19, x19, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x12 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x13 __LF                  \
        adc     x15, x15, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x19, x19, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x12 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x13 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        cset    x16, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x20, x20, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x11 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x12 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x16, x16, x14 __LF                 \
        stp     x17, x19, [P0+16] __LF             \
        ldp     x3, x4, [P1+32] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x20, x20, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x21, x21, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x11 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x12 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        cset    x17, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x21, x21, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x10 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x11 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x13 __LF                  \
        adc     x17, x17, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x21, x21, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x22, x22, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x4, x8 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x9 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x10 __LF                  \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x11 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        cset    x19, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x22, x22, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        umulh   x14, x4, x7 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x4, x8 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x9 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x10 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x19, x19, x14 __LF                 \
        stp     x20, x21, [P0+32] __LF             \
        ldp     x3, x4, [P1+48] __LF               \
        mul     x14, x3, x5 __LF                   \
        adds    x22, x22, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x23, x23, x14 __LF                 \
        mul     x14, x3, x7 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x3, x8 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x9 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x10 __LF                  \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        cset    x20, hs __LF                       \
        umulh   x14, x3, x5 __LF                   \
        adds    x23, x23, x14 __LF                 \
        umulh   x14, x3, x6 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        umulh   x14, x3, x7 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x9 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x3, x13 __LF                  \
        adc     x20, x20, x14 __LF                 \
        mul     x14, x4, x5 __LF                   \
        adds    x23, x23, x14 __LF                 \
        mul     x14, x4, x6 __LF                   \
        adcs    x24, x24, x14 __LF                 \
        mul     x14, x4, x7 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x4, x8 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x4, x9 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x4, x10 __LF                  \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x4, x11 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x4, x12 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x4, x13 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        cset    x21, hs __LF                       \
        umulh   x14, x4, x5 __LF                   \
        adds    x24, x24, x14 __LF                 \
        umulh   x14, x4, x6 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        umulh   x14, x4, x7 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x4, x8 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x4, x9 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x4, x10 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x4, x11 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x4, x12 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x4, x13 __LF                  \
        adc     x21, x21, x14 __LF                 \
        stp     x22, x23, [P0+48] __LF             \
        ldr     x3, [P1+64] __LF                   \
        mul     x14, x3, x5 __LF                   \
        adds    x24, x24, x14 __LF                 \
        mul     x14, x3, x6 __LF                   \
        adcs    x1, x1, x14 __LF                   \
        mul     x14, x3, x7 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        mul     x14, x3, x8 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        mul     x14, x3, x9 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        mul     x14, x3, x10 __LF                  \
        adcs    x17, x17, x14 __LF                 \
        mul     x14, x3, x11 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        mul     x14, x3, x12 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        mul     x14, x3, x13 __LF                  \
        adc     x21, x21, x14 __LF                 \
        umulh   x14, x3, x5 __LF                   \
        adds    x1, x1, x14 __LF                   \
        umulh   x14, x3, x6 __LF                   \
        adcs    x0, x0, x14 __LF                   \
        umulh   x14, x3, x7 __LF                   \
        adcs    x15, x15, x14 __LF                 \
        umulh   x14, x3, x8 __LF                   \
        adcs    x16, x16, x14 __LF                 \
        umulh   x14, x3, x9 __LF                   \
        adcs    x17, x17, x14 __LF                 \
        umulh   x14, x3, x10 __LF                  \
        adcs    x19, x19, x14 __LF                 \
        umulh   x14, x3, x11 __LF                  \
        adcs    x20, x20, x14 __LF                 \
        umulh   x14, x3, x12 __LF                  \
        adc     x21, x21, x14 __LF                 \
        ldp     x5, x6, [P0] __LF                  \
        extr    x14, x1, x24, #9 __LF              \
        adds    x5, x5, x14 __LF                   \
        extr    x14, x0, x1, #9 __LF               \
        adcs    x6, x6, x14 __LF                   \
        ldp     x7, x8, [P0+16] __LF               \
        extr    x14, x15, x0, #9 __LF              \
        adcs    x7, x7, x14 __LF                   \
        extr    x14, x16, x15, #9 __LF             \
        adcs    x8, x8, x14 __LF                   \
        ldp     x9, x10, [P0+32] __LF              \
        extr    x14, x17, x16, #9 __LF             \
        adcs    x9, x9, x14 __LF                   \
        extr    x14, x19, x17, #9 __LF             \
        adcs    x10, x10, x14 __LF                 \
        ldp     x11, x12, [P0+48] __LF             \
        extr    x14, x20, x19, #9 __LF             \
        adcs    x11, x11, x14 __LF                 \
        extr    x14, x21, x20, #9 __LF             \
        adcs    x12, x12, x14 __LF                 \
        and     x13, x24, #0x1ff __LF              \
        lsr     x14, x21, #9 __LF                  \
        adc     x13, x13, x14 __LF                 \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// P0 = C * P1 - D * P2 == C * P1 + D * (p_521 - P2)

#define cmsub_p521(P0,C,P1,D,P2)                \
        ldp     x6, x7, [P1] __LF                  \
        mov     x1, #(C) __LF                      \
        mul     x3, x1, x6 __LF                    \
        mul     x4, x1, x7 __LF                    \
        umulh   x6, x1, x6 __LF                    \
        adds    x4, x4, x6 __LF                    \
        umulh   x7, x1, x7 __LF                    \
        ldp     x8, x9, [P1+16] __LF               \
        mul     x5, x1, x8 __LF                    \
        mul     x6, x1, x9 __LF                    \
        umulh   x8, x1, x8 __LF                    \
        adcs    x5, x5, x7 __LF                    \
        umulh   x9, x1, x9 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        ldp     x10, x11, [P1+32] __LF             \
        mul     x7, x1, x10 __LF                   \
        mul     x8, x1, x11 __LF                   \
        umulh   x10, x1, x10 __LF                  \
        adcs    x7, x7, x9 __LF                    \
        umulh   x11, x1, x11 __LF                  \
        adcs    x8, x8, x10 __LF                   \
        ldp     x12, x13, [P1+48] __LF             \
        mul     x9, x1, x12 __LF                   \
        mul     x10, x1, x13 __LF                  \
        umulh   x12, x1, x12 __LF                  \
        adcs    x9, x9, x11 __LF                   \
        umulh   x13, x1, x13 __LF                  \
        adcs    x10, x10, x12 __LF                 \
        ldr     x14, [P1+64] __LF                  \
        mul     x11, x1, x14 __LF                  \
        adc     x11, x11, x13 __LF                 \
        mov     x1, #(D) __LF                      \
        ldp     x20, x21, [P2] __LF                \
        mvn     x20, x20 __LF                      \
        mul     x0, x1, x20 __LF                   \
        umulh   x20, x1, x20 __LF                  \
        adds    x3, x3, x0 __LF                    \
        mvn     x21, x21 __LF                      \
        mul     x0, x1, x21 __LF                   \
        umulh   x21, x1, x21 __LF                  \
        adcs    x4, x4, x0 __LF                    \
        ldp     x22, x23, [P2+16] __LF             \
        mvn     x22, x22 __LF                      \
        mul     x0, x1, x22 __LF                   \
        umulh   x22, x1, x22 __LF                  \
        adcs    x5, x5, x0 __LF                    \
        mvn     x23, x23 __LF                      \
        mul     x0, x1, x23 __LF                   \
        umulh   x23, x1, x23 __LF                  \
        adcs    x6, x6, x0 __LF                    \
        ldp     x17, x19, [P2+32] __LF             \
        mvn     x17, x17 __LF                      \
        mul     x0, x1, x17 __LF                   \
        umulh   x17, x1, x17 __LF                  \
        adcs    x7, x7, x0 __LF                    \
        mvn     x19, x19 __LF                      \
        mul     x0, x1, x19 __LF                   \
        umulh   x19, x1, x19 __LF                  \
        adcs    x8, x8, x0 __LF                    \
        ldp     x2, x16, [P2+48] __LF              \
        mvn     x2, x2 __LF                        \
        mul     x0, x1, x2 __LF                    \
        umulh   x2, x1, x2 __LF                    \
        adcs    x9, x9, x0 __LF                    \
        mvn     x16, x16 __LF                      \
        mul     x0, x1, x16 __LF                   \
        umulh   x16, x1, x16 __LF                  \
        adcs    x10, x10, x0 __LF                  \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        mul     x0, x1, x0 __LF                    \
        adc     x11, x11, x0 __LF                  \
        adds    x4, x4, x20 __LF                   \
        adcs    x5, x5, x21 __LF                   \
        and     x15, x4, x5 __LF                   \
        adcs    x6, x6, x22 __LF                   \
        and     x15, x15, x6 __LF                  \
        adcs    x7, x7, x23 __LF                   \
        and     x15, x15, x7 __LF                  \
        adcs    x8, x8, x17 __LF                   \
        and     x15, x15, x8 __LF                  \
        adcs    x9, x9, x19 __LF                   \
        and     x15, x15, x9 __LF                  \
        adcs    x10, x10, x2 __LF                  \
        and     x15, x15, x10 __LF                 \
        adc     x11, x11, x16 __LF                 \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

// P0 = 3 * P1 - 8 * P2 == 3 * P1 + 8 * (p_521 - P2)

#define cmsub38_p521(P0,P1,P2)                  \
        ldp     x6, x7, [P1] __LF                  \
        lsl     x3, x6, #1 __LF                    \
        adds    x3, x3, x6 __LF                    \
        extr    x4, x7, x6, #63 __LF               \
        adcs    x4, x4, x7 __LF                    \
        ldp     x8, x9, [P1+16] __LF               \
        extr    x5, x8, x7, #63 __LF               \
        adcs    x5, x5, x8 __LF                    \
        extr    x6, x9, x8, #63 __LF               \
        adcs    x6, x6, x9 __LF                    \
        ldp     x10, x11, [P1+32] __LF             \
        extr    x7, x10, x9, #63 __LF              \
        adcs    x7, x7, x10 __LF                   \
        extr    x8, x11, x10, #63 __LF             \
        adcs    x8, x8, x11 __LF                   \
        ldp     x12, x13, [P1+48] __LF             \
        extr    x9, x12, x11, #63 __LF             \
        adcs    x9, x9, x12 __LF                   \
        extr    x10, x13, x12, #63 __LF            \
        adcs    x10, x10, x13 __LF                 \
        ldr     x14, [P1+64] __LF                  \
        extr    x11, x14, x13, #63 __LF            \
        adc     x11, x11, x14 __LF                 \
        ldp     x20, x21, [P2] __LF                \
        mvn     x20, x20 __LF                      \
        lsl     x0, x20, #3 __LF                   \
        adds    x3, x3, x0 __LF                    \
        mvn     x21, x21 __LF                      \
        extr    x0, x21, x20, #61 __LF             \
        adcs    x4, x4, x0 __LF                    \
        ldp     x22, x23, [P2+16] __LF             \
        mvn     x22, x22 __LF                      \
        extr    x0, x22, x21, #61 __LF             \
        adcs    x5, x5, x0 __LF                    \
        and     x15, x4, x5 __LF                   \
        mvn     x23, x23 __LF                      \
        extr    x0, x23, x22, #61 __LF             \
        adcs    x6, x6, x0 __LF                    \
        and     x15, x15, x6 __LF                  \
        ldp     x20, x21, [P2+32] __LF             \
        mvn     x20, x20 __LF                      \
        extr    x0, x20, x23, #61 __LF             \
        adcs    x7, x7, x0 __LF                    \
        and     x15, x15, x7 __LF                  \
        mvn     x21, x21 __LF                      \
        extr    x0, x21, x20, #61 __LF             \
        adcs    x8, x8, x0 __LF                    \
        and     x15, x15, x8 __LF                  \
        ldp     x22, x23, [P2+48] __LF             \
        mvn     x22, x22 __LF                      \
        extr    x0, x22, x21, #61 __LF             \
        adcs    x9, x9, x0 __LF                    \
        and     x15, x15, x9 __LF                  \
        mvn     x23, x23 __LF                      \
        extr    x0, x23, x22, #61 __LF             \
        adcs    x10, x10, x0 __LF                  \
        and     x15, x15, x10 __LF                 \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        extr    x0, x0, x23, #61 __LF              \
        adc     x11, x11, x0 __LF                  \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

// P0 = 4 * P1 - P2 = 4 * P1 + (p_521 - P2)

#define cmsub41_p521(P0,P1,P2)                  \
        ldp     x6, x7, [P1] __LF                  \
        lsl     x3, x6, #2 __LF                    \
        extr    x4, x7, x6, #62 __LF               \
        ldp     x8, x9, [P1+16] __LF               \
        extr    x5, x8, x7, #62 __LF               \
        extr    x6, x9, x8, #62 __LF               \
        ldp     x10, x11, [P1+32] __LF             \
        extr    x7, x10, x9, #62 __LF              \
        extr    x8, x11, x10, #62 __LF             \
        ldp     x12, x13, [P1+48] __LF             \
        extr    x9, x12, x11, #62 __LF             \
        extr    x10, x13, x12, #62 __LF            \
        ldr     x14, [P1+64] __LF                  \
        extr    x11, x14, x13, #62 __LF            \
        ldp     x0, x1, [P2] __LF                  \
        mvn     x0, x0 __LF                        \
        adds    x3, x3, x0 __LF                    \
        sbcs    x4, x4, x1 __LF                    \
        ldp     x0, x1, [P2+16] __LF               \
        sbcs    x5, x5, x0 __LF                    \
        and     x15, x4, x5 __LF                   \
        sbcs    x6, x6, x1 __LF                    \
        and     x15, x15, x6 __LF                  \
        ldp     x0, x1, [P2+32] __LF               \
        sbcs    x7, x7, x0 __LF                    \
        and     x15, x15, x7 __LF                  \
        sbcs    x8, x8, x1 __LF                    \
        and     x15, x15, x8 __LF                  \
        ldp     x0, x1, [P2+48] __LF               \
        sbcs    x9, x9, x0 __LF                    \
        and     x15, x15, x9 __LF                  \
        sbcs    x10, x10, x1 __LF                  \
        and     x15, x15, x10 __LF                 \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        adc     x11, x11, x0 __LF                  \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

S2N_BN_SYMBOL(p521_jdouble_alt):
        CFI_START

// Save regs and make room on stack for temporary variables

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1

// Main code, just a sequence of basic field operations

// z2 = z^2
// y2 = y^2

        sqr_p521(z2,z_1)
        sqr_p521(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        add_p521(t1,x_1,z2)
        sub_p521(t2,x_1,z2)
        mul_p521(x2p,t1,t2)

// t1 = y + z
// x4p = x2p^2
// xy2 = x * y^2

        add_p521(t1,y_1,z_1)
        sqr_p521(x4p,x2p)
        weakmul_p521(xy2,x_1,y2)

// t2 = (y + z)^2

        sqr_p521(t2,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p521(d,12,xy2,9,x4p)
        sub_p521(t1,t2,z2)

// y4 = y^4

        sqr_p521(y4,y2)

// z_3' = 2 * y * z
// dx2 = d * x2p

        sub_p521(z_3,t1,y2)
        weakmul_p521(dx2,d,x2p)

// x' = 4 * xy2 - d

        cmsub41_p521(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p521(y_3,dx2,y4)

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
