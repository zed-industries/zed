// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jmixadd_alt(uint64_t p3[static 27],
//                                 const uint64_t p1[static 27],
//                                 const uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity. It is assumed that
// all the coordinates of the input points p1 and p2 are fully reduced
// mod p_521, that the z coordinate of p1 is nonzero and that neither
// p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents the same affine
// point as".
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jmixadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jmixadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jmixadd_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence

#define input_z x26
#define input_x x27
#define input_y x28

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_2 input_y, #0
#define y_2 input_y, #NUMSIZE

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// #NSPACE is the total stack needed for these temporaries

#define zp2 sp, #(NUMSIZE*0)
#define ww sp, #(NUMSIZE*0)
#define resx sp, #(NUMSIZE*0)

#define yd sp, #(NUMSIZE*1)
#define y2a sp, #(NUMSIZE*1)

#define x2a sp, #(NUMSIZE*2)
#define zzx2 sp, #(NUMSIZE*2)

#define zz sp, #(NUMSIZE*3)
#define t1 sp, #(NUMSIZE*3)

#define t2 sp, #(NUMSIZE*4)
#define zzx1 sp, #(NUMSIZE*4)
#define resy sp, #(NUMSIZE*4)

#define xd sp, #(NUMSIZE*5)
#define resz sp, #(NUMSIZE*5)

#define NSPACE NUMSIZE*6

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

S2N_BN_SYMBOL(p521_jmixadd_alt):
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
        mov     input_y, x2

// Main code, just a sequence of basic field operations

        sqr_p521(zp2,z_1)
        mul_p521(y2a,z_1,y_2)

        mul_p521(x2a,zp2,x_2)
        mul_p521(y2a,zp2,y2a)

        sub_p521(xd,x2a,x_1)
        sub_p521(yd,y2a,y_1)

        sqr_p521(zz,xd)
        sqr_p521(ww,yd)

        mul_p521(zzx1,zz,x_1)
        mul_p521(zzx2,zz,x2a)

        sub_p521(resx,ww,zzx1)
        sub_p521(t1,zzx2,zzx1)

        mul_p521(resz,xd,z_1)

        sub_p521(resx,resx,zzx2)

        sub_p521(t2,zzx1,resx)

        mul_p521(t1,t1,y_1)
        mul_p521(t2,yd,t2)

        sub_p521(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        ldp     x0, x1, [z_1]
        orr     x0, x0, x1
        ldp     x2, x3, [z_1+16]
        orr     x2, x2, x3
        ldp     x4, x5, [z_1+32]
        orr     x4, x4, x5
        ldp     x6, x7, [z_1+48]
        orr     x6, x6, x7
        ldr     x8, [z_1+64]
        orr     x0, x0, x2
        orr     x4, x4, x6
        orr     x0, x0, x4
        orr     x0, x0, x8
        cmp     x0, xzr

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with an extra z = 1
// coordinate, hence giving 0 + p2 = p2 for the final result.

        ldp     x0, x1, [resx]
        ldp     x20, x21, [x_2]
        csel    x0, x0, x20, ne
        csel    x1, x1, x21, ne
        ldp     x2, x3, [resx+16]
        ldp     x20, x21, [x_2+16]
        csel    x2, x2, x20, ne
        csel    x3, x3, x21, ne
        ldp     x4, x5, [resx+32]
        ldp     x20, x21, [x_2+32]
        csel    x4, x4, x20, ne
        csel    x5, x5, x21, ne
        ldp     x6, x7, [resx+48]
        ldp     x20, x21, [x_2+48]
        csel    x6, x6, x20, ne
        csel    x7, x7, x21, ne
        ldr     x8, [resx+64]
        ldr     x20, [x_2+64]
        csel    x8, x8, x20, ne

        ldp     x10, x11, [resy]
        ldp     x20, x21, [y_2]
        csel    x10, x10, x20, ne
        csel    x11, x11, x21, ne
        ldp     x12, x13, [resy+16]
        ldp     x20, x21, [y_2+16]
        csel    x12, x12, x20, ne
        csel    x13, x13, x21, ne
        ldp     x14, x15, [resy+32]
        ldp     x20, x21, [y_2+32]
        csel    x14, x14, x20, ne
        csel    x15, x15, x21, ne
        ldp     x16, x17, [resy+48]
        ldp     x20, x21, [y_2+48]
        csel    x16, x16, x20, ne
        csel    x17, x17, x21, ne
        ldr     x19, [resy+64]
        ldr     x20, [y_2+64]
        csel    x19, x19, x20, ne

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [x_3+32]
        stp     x6, x7, [x_3+48]
        str     x8, [x_3+64]
        stp     x10, x11, [y_3]
        stp     x12, x13, [y_3+16]
        stp     x14, x15, [y_3+32]
        stp     x16, x17, [y_3+48]
        str     x19, [y_3+64]

        ldp     x0, x1, [resz]
        mov     x20, #1
        csel    x0, x0, x20, ne
        csel    x1, x1, xzr, ne
        ldp     x2, x3, [resz+16]
        csel    x2, x2, xzr, ne
        csel    x3, x3, xzr, ne
        ldp     x4, x5, [resz+32]
        csel    x4, x4, xzr, ne
        csel    x5, x5, xzr, ne
        ldp     x6, x7, [resz+48]
        csel    x6, x6, xzr, ne
        csel    x7, x7, xzr, ne
        ldr     x8, [resz+64]
        csel    x8, x8, xzr, ne

        stp     x0, x1, [z_3]
        stp     x2, x3, [z_3+16]
        stp     x4, x5, [z_3+32]
        stp     x6, x7, [z_3+48]
        str     x8, [z_3+64]

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jmixadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
