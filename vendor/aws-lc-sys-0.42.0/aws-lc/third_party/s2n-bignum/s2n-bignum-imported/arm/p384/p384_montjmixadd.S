// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjmixadd(uint64_t p3[static 18],
//                                 const uint64_t p1[static 18],
//                                 const uint64_t p2[static 12]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity.
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjmixadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjmixadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjmixadd)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48

// Stable homes for input arguments during main code sequence

#define input_z x24
#define input_x x25
#define input_y x26

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

// Corresponds to bignum_montmul_p384 except x24 -> x0

#define montmul_p384(P0,P1,P2)                  \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P1+16] __LF               \
        ldp     x7, x8, [P1+32] __LF               \
        ldp     x9, x10, [P2] __LF                 \
        ldp     x11, x12, [P2+16] __LF             \
        ldp     x13, x14, [P2+32] __LF             \
        mul     x15, x3, x9 __LF                   \
        mul     x21, x4, x10 __LF                  \
        mul     x22, x5, x11 __LF                  \
        umulh   x23, x3, x9 __LF                   \
        umulh   x0, x4, x10 __LF                   \
        umulh   x1, x5, x11 __LF                   \
        adds    x23, x23, x21 __LF                 \
        adcs    x0, x0, x22 __LF                   \
        adc     x1, x1, xzr __LF                   \
        adds    x16, x23, x15 __LF                 \
        adcs    x17, x0, x23 __LF                  \
        adcs    x19, x1, x0 __LF                   \
        adc     x20, x1, xzr __LF                  \
        adds    x17, x17, x15 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x0, x3, x4 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x10, x9 __LF                  \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x16, x16, x21 __LF                 \
        adcs    x17, x17, x22 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x3, x5 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x11, x9 __LF                  \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x17, x17, x21 __LF                 \
        adcs    x19, x19, x22 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x4, x5 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x11, x10 __LF                 \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x19, x19, x21 __LF                 \
        adcs    x20, x20, x22 __LF                 \
        adc     x1, x1, x23 __LF                   \
        lsl     x23, x15, #32 __LF                 \
        add     x15, x23, x15 __LF                 \
        lsr     x23, x15, #32 __LF                 \
        subs    x23, x23, x15 __LF                 \
        sbc     x22, x15, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x15 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x16, x16, x23 __LF                 \
        sbcs    x17, x17, x22 __LF                 \
        sbcs    x19, x19, x21 __LF                 \
        sbcs    x20, x20, xzr __LF                 \
        sbcs    x1, x1, xzr __LF                   \
        sbc     x15, x15, xzr __LF                 \
        lsl     x23, x16, #32 __LF                 \
        add     x16, x23, x16 __LF                 \
        lsr     x23, x16, #32 __LF                 \
        subs    x23, x23, x16 __LF                 \
        sbc     x22, x16, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x16 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x17, x17, x23 __LF                 \
        sbcs    x19, x19, x22 __LF                 \
        sbcs    x20, x20, x21 __LF                 \
        sbcs    x1, x1, xzr __LF                   \
        sbcs    x15, x15, xzr __LF                 \
        sbc     x16, x16, xzr __LF                 \
        lsl     x23, x17, #32 __LF                 \
        add     x17, x23, x17 __LF                 \
        lsr     x23, x17, #32 __LF                 \
        subs    x23, x23, x17 __LF                 \
        sbc     x22, x17, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x17 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x19, x19, x23 __LF                 \
        sbcs    x20, x20, x22 __LF                 \
        sbcs    x1, x1, x21 __LF                   \
        sbcs    x15, x15, xzr __LF                 \
        sbcs    x16, x16, xzr __LF                 \
        sbc     x17, x17, xzr __LF                 \
        stp     x19, x20, [P0] __LF                \
        stp     x1, x15, [P0+16] __LF              \
        stp     x16, x17, [P0+32] __LF             \
        mul     x15, x6, x12 __LF                  \
        mul     x21, x7, x13 __LF                  \
        mul     x22, x8, x14 __LF                  \
        umulh   x23, x6, x12 __LF                  \
        umulh   x0, x7, x13 __LF                   \
        umulh   x1, x8, x14 __LF                   \
        adds    x23, x23, x21 __LF                 \
        adcs    x0, x0, x22 __LF                   \
        adc     x1, x1, xzr __LF                   \
        adds    x16, x23, x15 __LF                 \
        adcs    x17, x0, x23 __LF                  \
        adcs    x19, x1, x0 __LF                   \
        adc     x20, x1, xzr __LF                  \
        adds    x17, x17, x15 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x0, x6, x7 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x13, x12 __LF                 \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x16, x16, x21 __LF                 \
        adcs    x17, x17, x22 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x6, x8 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x14, x12 __LF                 \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x17, x17, x21 __LF                 \
        adcs    x19, x19, x22 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x7, x8 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x14, x13 __LF                 \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x19, x19, x21 __LF                 \
        adcs    x20, x20, x22 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x6, x6, x3 __LF                    \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x5 __LF                    \
        ngc     x3, xzr __LF                       \
        cmn     x3, #1 __LF                        \
        eor     x6, x6, x3 __LF                    \
        adcs    x6, x6, xzr __LF                   \
        eor     x7, x7, x3 __LF                    \
        adcs    x7, x7, xzr __LF                   \
        eor     x8, x8, x3 __LF                    \
        adc     x8, x8, xzr __LF                   \
        subs    x9, x9, x12 __LF                   \
        sbcs    x10, x10, x13 __LF                 \
        sbcs    x11, x11, x14 __LF                 \
        ngc     x14, xzr __LF                      \
        cmn     x14, #1 __LF                       \
        eor     x9, x9, x14 __LF                   \
        adcs    x9, x9, xzr __LF                   \
        eor     x10, x10, x14 __LF                 \
        adcs    x10, x10, xzr __LF                 \
        eor     x11, x11, x14 __LF                 \
        adc     x11, x11, xzr __LF                 \
        eor     x14, x3, x14 __LF                  \
        ldp     x21, x22, [P0] __LF                \
        adds    x15, x15, x21 __LF                 \
        adcs    x16, x16, x22 __LF                 \
        ldp     x21, x22, [P0+16] __LF             \
        adcs    x17, x17, x21 __LF                 \
        adcs    x19, x19, x22 __LF                 \
        ldp     x21, x22, [P0+32] __LF             \
        adcs    x20, x20, x21 __LF                 \
        adcs    x1, x1, x22 __LF                   \
        adc     x2, xzr, xzr __LF                  \
        stp     x15, x16, [P0] __LF                \
        stp     x17, x19, [P0+16] __LF             \
        stp     x20, x1, [P0+32] __LF              \
        mul     x15, x6, x9 __LF                   \
        mul     x21, x7, x10 __LF                  \
        mul     x22, x8, x11 __LF                  \
        umulh   x23, x6, x9 __LF                   \
        umulh   x0, x7, x10 __LF                   \
        umulh   x1, x8, x11 __LF                   \
        adds    x23, x23, x21 __LF                 \
        adcs    x0, x0, x22 __LF                   \
        adc     x1, x1, xzr __LF                   \
        adds    x16, x23, x15 __LF                 \
        adcs    x17, x0, x23 __LF                  \
        adcs    x19, x1, x0 __LF                   \
        adc     x20, x1, xzr __LF                  \
        adds    x17, x17, x15 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x0, x6, x7 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x10, x9 __LF                  \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x16, x16, x21 __LF                 \
        adcs    x17, x17, x22 __LF                 \
        adcs    x19, x19, x23 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x6, x8 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x11, x9 __LF                  \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x17, x17, x21 __LF                 \
        adcs    x19, x19, x22 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        adc     x1, x1, x23 __LF                   \
        subs    x0, x7, x8 __LF                    \
        cneg    x0, x0, lo __LF                    \
        csetm   x23, lo __LF                       \
        subs    x22, x11, x10 __LF                 \
        cneg    x22, x22, lo __LF                  \
        mul     x21, x0, x22 __LF                  \
        umulh   x22, x0, x22 __LF                  \
        cinv    x23, x23, lo __LF                  \
        eor     x21, x21, x23 __LF                 \
        eor     x22, x22, x23 __LF                 \
        cmn     x23, #1 __LF                       \
        adcs    x19, x19, x21 __LF                 \
        adcs    x20, x20, x22 __LF                 \
        adc     x1, x1, x23 __LF                   \
        ldp     x3, x4, [P0] __LF                  \
        ldp     x5, x6, [P0+16] __LF               \
        ldp     x7, x8, [P0+32] __LF               \
        cmn     x14, #1 __LF                       \
        eor     x15, x15, x14 __LF                 \
        adcs    x15, x15, x3 __LF                  \
        eor     x16, x16, x14 __LF                 \
        adcs    x16, x16, x4 __LF                  \
        eor     x17, x17, x14 __LF                 \
        adcs    x17, x17, x5 __LF                  \
        eor     x19, x19, x14 __LF                 \
        adcs    x19, x19, x6 __LF                  \
        eor     x20, x20, x14 __LF                 \
        adcs    x20, x20, x7 __LF                  \
        eor     x1, x1, x14 __LF                   \
        adcs    x1, x1, x8 __LF                    \
        adcs    x9, x14, x2 __LF                   \
        adcs    x10, x14, xzr __LF                 \
        adcs    x11, x14, xzr __LF                 \
        adc     x12, x14, xzr __LF                 \
        adds    x19, x19, x3 __LF                  \
        adcs    x20, x20, x4 __LF                  \
        adcs    x1, x1, x5 __LF                    \
        adcs    x9, x9, x6 __LF                    \
        adcs    x10, x10, x7 __LF                  \
        adcs    x11, x11, x8 __LF                  \
        adc     x12, x12, x2 __LF                  \
        lsl     x23, x15, #32 __LF                 \
        add     x15, x23, x15 __LF                 \
        lsr     x23, x15, #32 __LF                 \
        subs    x23, x23, x15 __LF                 \
        sbc     x22, x15, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x15 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x16, x16, x23 __LF                 \
        sbcs    x17, x17, x22 __LF                 \
        sbcs    x19, x19, x21 __LF                 \
        sbcs    x20, x20, xzr __LF                 \
        sbcs    x1, x1, xzr __LF                   \
        sbc     x15, x15, xzr __LF                 \
        lsl     x23, x16, #32 __LF                 \
        add     x16, x23, x16 __LF                 \
        lsr     x23, x16, #32 __LF                 \
        subs    x23, x23, x16 __LF                 \
        sbc     x22, x16, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x16 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x17, x17, x23 __LF                 \
        sbcs    x19, x19, x22 __LF                 \
        sbcs    x20, x20, x21 __LF                 \
        sbcs    x1, x1, xzr __LF                   \
        sbcs    x15, x15, xzr __LF                 \
        sbc     x16, x16, xzr __LF                 \
        lsl     x23, x17, #32 __LF                 \
        add     x17, x23, x17 __LF                 \
        lsr     x23, x17, #32 __LF                 \
        subs    x23, x23, x17 __LF                 \
        sbc     x22, x17, xzr __LF                 \
        extr    x23, x22, x23, #32 __LF            \
        lsr     x22, x22, #32 __LF                 \
        adds    x22, x22, x17 __LF                 \
        adc     x21, xzr, xzr __LF                 \
        subs    x19, x19, x23 __LF                 \
        sbcs    x20, x20, x22 __LF                 \
        sbcs    x1, x1, x21 __LF                   \
        sbcs    x15, x15, xzr __LF                 \
        sbcs    x16, x16, xzr __LF                 \
        sbc     x17, x17, xzr __LF                 \
        adds    x9, x9, x15 __LF                   \
        adcs    x10, x10, x16 __LF                 \
        adcs    x11, x11, x17 __LF                 \
        adc     x12, x12, xzr __LF                 \
        add     x22, x12, #1 __LF                  \
        lsl     x21, x22, #32 __LF                 \
        subs    x0, x22, x21 __LF                  \
        sbc     x21, x21, xzr __LF                 \
        adds    x19, x19, x0 __LF                  \
        adcs    x20, x20, x21 __LF                 \
        adcs    x1, x1, x22 __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adcs    x11, x11, xzr __LF                 \
        csetm   x22, lo __LF                       \
        mov     x23, #4294967295 __LF              \
        and     x23, x23, x22 __LF                 \
        adds    x19, x19, x23 __LF                 \
        eor     x23, x23, x22 __LF                 \
        adcs    x20, x20, x23 __LF                 \
        mov     x23, #-2 __LF                      \
        and     x23, x23, x22 __LF                 \
        adcs    x1, x1, x23 __LF                   \
        adcs    x9, x9, x22 __LF                   \
        adcs    x10, x10, x22 __LF                 \
        adc     x11, x11, x22 __LF                 \
        stp     x19, x20, [P0] __LF                \
        stp     x1, x9, [P0+16] __LF               \
        stp     x10, x11, [P0+32]

// Corresponds exactly to bignum_montsqr_p384

#define montsqr_p384(P0,P1)                     \
        ldp     x2, x3, [P1] __LF                  \
        ldp     x4, x5, [P1+16] __LF               \
        ldp     x6, x7, [P1+32] __LF               \
        mul     x14, x2, x3 __LF                   \
        mul     x15, x2, x4 __LF                   \
        mul     x16, x3, x4 __LF                   \
        mul     x8, x2, x2 __LF                    \
        mul     x10, x3, x3 __LF                   \
        mul     x12, x4, x4 __LF                   \
        umulh   x17, x2, x3 __LF                   \
        adds    x15, x15, x17 __LF                 \
        umulh   x17, x2, x4 __LF                   \
        adcs    x16, x16, x17 __LF                 \
        umulh   x17, x3, x4 __LF                   \
        adcs    x17, x17, xzr __LF                 \
        umulh   x9, x2, x2 __LF                    \
        umulh   x11, x3, x3 __LF                   \
        umulh   x13, x4, x4 __LF                   \
        adds    x14, x14, x14 __LF                 \
        adcs    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adcs    x17, x17, x17 __LF                 \
        adc     x13, x13, xzr __LF                 \
        adds    x9, x9, x14 __LF                   \
        adcs    x10, x10, x15 __LF                 \
        adcs    x11, x11, x16 __LF                 \
        adcs    x12, x12, x17 __LF                 \
        adc     x13, x13, xzr __LF                 \
        lsl     x16, x8, #32 __LF                  \
        add     x8, x16, x8 __LF                   \
        lsr     x16, x8, #32 __LF                  \
        subs    x16, x16, x8 __LF                  \
        sbc     x15, x8, xzr __LF                  \
        extr    x16, x15, x16, #32 __LF            \
        lsr     x15, x15, #32 __LF                 \
        adds    x15, x15, x8 __LF                  \
        adc     x14, xzr, xzr __LF                 \
        subs    x9, x9, x16 __LF                   \
        sbcs    x10, x10, x15 __LF                 \
        sbcs    x11, x11, x14 __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbc     x8, x8, xzr __LF                   \
        lsl     x16, x9, #32 __LF                  \
        add     x9, x16, x9 __LF                   \
        lsr     x16, x9, #32 __LF                  \
        subs    x16, x16, x9 __LF                  \
        sbc     x15, x9, xzr __LF                  \
        extr    x16, x15, x16, #32 __LF            \
        lsr     x15, x15, #32 __LF                 \
        adds    x15, x15, x9 __LF                  \
        adc     x14, xzr, xzr __LF                 \
        subs    x10, x10, x16 __LF                 \
        sbcs    x11, x11, x15 __LF                 \
        sbcs    x12, x12, x14 __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x8, x8, xzr __LF                   \
        sbc     x9, x9, xzr __LF                   \
        lsl     x16, x10, #32 __LF                 \
        add     x10, x16, x10 __LF                 \
        lsr     x16, x10, #32 __LF                 \
        subs    x16, x16, x10 __LF                 \
        sbc     x15, x10, xzr __LF                 \
        extr    x16, x15, x16, #32 __LF            \
        lsr     x15, x15, #32 __LF                 \
        adds    x15, x15, x10 __LF                 \
        adc     x14, xzr, xzr __LF                 \
        subs    x11, x11, x16 __LF                 \
        sbcs    x12, x12, x15 __LF                 \
        sbcs    x13, x13, x14 __LF                 \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        stp     x11, x12, [P0] __LF                \
        stp     x13, x8, [P0+16] __LF              \
        stp     x9, x10, [P0+32] __LF              \
        mul     x8, x2, x5 __LF                    \
        mul     x14, x3, x6 __LF                   \
        mul     x15, x4, x7 __LF                   \
        umulh   x16, x2, x5 __LF                   \
        umulh   x17, x3, x6 __LF                   \
        umulh   x1, x4, x7 __LF                    \
        adds    x16, x16, x14 __LF                 \
        adcs    x17, x17, x15 __LF                 \
        adc     x1, x1, xzr __LF                   \
        adds    x9, x16, x8 __LF                   \
        adcs    x10, x17, x16 __LF                 \
        adcs    x11, x1, x17 __LF                  \
        adc     x12, x1, xzr __LF                  \
        adds    x10, x10, x8 __LF                  \
        adcs    x11, x11, x16 __LF                 \
        adcs    x12, x12, x17 __LF                 \
        adc     x13, x1, xzr __LF                  \
        subs    x17, x2, x3 __LF                   \
        cneg    x17, x17, lo __LF                  \
        csetm   x14, lo __LF                       \
        subs    x15, x6, x5 __LF                   \
        cneg    x15, x15, lo __LF                  \
        mul     x16, x17, x15 __LF                 \
        umulh   x15, x17, x15 __LF                 \
        cinv    x14, x14, lo __LF                  \
        eor     x16, x16, x14 __LF                 \
        eor     x15, x15, x14 __LF                 \
        cmn     x14, #1 __LF                       \
        adcs    x9, x9, x16 __LF                   \
        adcs    x10, x10, x15 __LF                 \
        adcs    x11, x11, x14 __LF                 \
        adcs    x12, x12, x14 __LF                 \
        adc     x13, x13, x14 __LF                 \
        subs    x17, x2, x4 __LF                   \
        cneg    x17, x17, lo __LF                  \
        csetm   x14, lo __LF                       \
        subs    x15, x7, x5 __LF                   \
        cneg    x15, x15, lo __LF                  \
        mul     x16, x17, x15 __LF                 \
        umulh   x15, x17, x15 __LF                 \
        cinv    x14, x14, lo __LF                  \
        eor     x16, x16, x14 __LF                 \
        eor     x15, x15, x14 __LF                 \
        cmn     x14, #1 __LF                       \
        adcs    x10, x10, x16 __LF                 \
        adcs    x11, x11, x15 __LF                 \
        adcs    x12, x12, x14 __LF                 \
        adc     x13, x13, x14 __LF                 \
        subs    x17, x3, x4 __LF                   \
        cneg    x17, x17, lo __LF                  \
        csetm   x14, lo __LF                       \
        subs    x15, x7, x6 __LF                   \
        cneg    x15, x15, lo __LF                  \
        mul     x16, x17, x15 __LF                 \
        umulh   x15, x17, x15 __LF                 \
        cinv    x14, x14, lo __LF                  \
        eor     x16, x16, x14 __LF                 \
        eor     x15, x15, x14 __LF                 \
        cmn     x14, #1 __LF                       \
        adcs    x11, x11, x16 __LF                 \
        adcs    x12, x12, x15 __LF                 \
        adc     x13, x13, x14 __LF                 \
        adds    x8, x8, x8 __LF                    \
        adcs    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adc     x17, xzr, xzr __LF                 \
        ldp     x2, x3, [P0] __LF                  \
        adds    x8, x8, x2 __LF                    \
        adcs    x9, x9, x3 __LF                    \
        ldp     x2, x3, [P0+16] __LF               \
        adcs    x10, x10, x2 __LF                  \
        adcs    x11, x11, x3 __LF                  \
        ldp     x2, x3, [P0+32] __LF               \
        adcs    x12, x12, x2 __LF                  \
        adcs    x13, x13, x3 __LF                  \
        adc     x17, x17, xzr __LF                 \
        lsl     x4, x8, #32 __LF                   \
        add     x8, x4, x8 __LF                    \
        lsr     x4, x8, #32 __LF                   \
        subs    x4, x4, x8 __LF                    \
        sbc     x3, x8, xzr __LF                   \
        extr    x4, x3, x4, #32 __LF               \
        lsr     x3, x3, #32 __LF                   \
        adds    x3, x3, x8 __LF                    \
        adc     x2, xzr, xzr __LF                  \
        subs    x9, x9, x4 __LF                    \
        sbcs    x10, x10, x3 __LF                  \
        sbcs    x11, x11, x2 __LF                  \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbc     x8, x8, xzr __LF                   \
        lsl     x4, x9, #32 __LF                   \
        add     x9, x4, x9 __LF                    \
        lsr     x4, x9, #32 __LF                   \
        subs    x4, x4, x9 __LF                    \
        sbc     x3, x9, xzr __LF                   \
        extr    x4, x3, x4, #32 __LF               \
        lsr     x3, x3, #32 __LF                   \
        adds    x3, x3, x9 __LF                    \
        adc     x2, xzr, xzr __LF                  \
        subs    x10, x10, x4 __LF                  \
        sbcs    x11, x11, x3 __LF                  \
        sbcs    x12, x12, x2 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x8, x8, xzr __LF                   \
        sbc     x9, x9, xzr __LF                   \
        lsl     x4, x10, #32 __LF                  \
        add     x10, x4, x10 __LF                  \
        lsr     x4, x10, #32 __LF                  \
        subs    x4, x4, x10 __LF                   \
        sbc     x3, x10, xzr __LF                  \
        extr    x4, x3, x4, #32 __LF               \
        lsr     x3, x3, #32 __LF                   \
        adds    x3, x3, x10 __LF                   \
        adc     x2, xzr, xzr __LF                  \
        subs    x11, x11, x4 __LF                  \
        sbcs    x12, x12, x3 __LF                  \
        sbcs    x13, x13, x2 __LF                  \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        adds    x17, x17, x8 __LF                  \
        adcs    x8, x9, xzr __LF                   \
        adcs    x9, x10, xzr __LF                  \
        adcs    x10, xzr, xzr __LF                 \
        mul     x1, x5, x5 __LF                    \
        adds    x11, x11, x1 __LF                  \
        mul     x14, x6, x6 __LF                   \
        mul     x15, x7, x7 __LF                   \
        umulh   x1, x5, x5 __LF                    \
        adcs    x12, x12, x1 __LF                  \
        umulh   x1, x6, x6 __LF                    \
        adcs    x13, x13, x14 __LF                 \
        adcs    x17, x17, x1 __LF                  \
        umulh   x1, x7, x7 __LF                    \
        adcs    x8, x8, x15 __LF                   \
        adcs    x9, x9, x1 __LF                    \
        adc     x10, x10, xzr __LF                 \
        mul     x1, x5, x6 __LF                    \
        mul     x14, x5, x7 __LF                   \
        mul     x15, x6, x7 __LF                   \
        umulh   x16, x5, x6 __LF                   \
        adds    x14, x14, x16 __LF                 \
        umulh   x16, x5, x7 __LF                   \
        adcs    x15, x15, x16 __LF                 \
        umulh   x16, x6, x7 __LF                   \
        adc     x16, x16, xzr __LF                 \
        adds    x1, x1, x1 __LF                    \
        adcs    x14, x14, x14 __LF                 \
        adcs    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adc     x5, xzr, xzr __LF                  \
        adds    x12, x12, x1 __LF                  \
        adcs    x13, x13, x14 __LF                 \
        adcs    x17, x17, x15 __LF                 \
        adcs    x8, x8, x16 __LF                   \
        adcs    x9, x9, x5 __LF                    \
        adc     x10, x10, xzr __LF                 \
        mov     x1, #-4294967295 __LF              \
        mov     x14, #4294967295 __LF              \
        mov     x15, #1 __LF                       \
        cmn     x11, x1 __LF                       \
        adcs    xzr, x12, x14 __LF                 \
        adcs    xzr, x13, x15 __LF                 \
        adcs    xzr, x17, xzr __LF                 \
        adcs    xzr, x8, xzr __LF                  \
        adcs    xzr, x9, xzr __LF                  \
        adc     x10, x10, xzr __LF                 \
        neg     x10, x10 __LF                      \
        and     x1, x1, x10 __LF                   \
        adds    x11, x11, x1 __LF                  \
        and     x14, x14, x10 __LF                 \
        adcs    x12, x12, x14 __LF                 \
        and     x15, x15, x10 __LF                 \
        adcs    x13, x13, x15 __LF                 \
        adcs    x17, x17, xzr __LF                 \
        adcs    x8, x8, xzr __LF                   \
        adc     x9, x9, xzr __LF                   \
        stp     x11, x12, [P0] __LF                \
        stp     x13, x17, [P0+16] __LF             \
        stp     x8, x9, [P0+32]

// Corresponds exactly to bignum_sub_p384

#define sub_p384(P0,P1,P2)                      \
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
        csetm   x3, lo __LF                        \
        mov     x4, #4294967295 __LF               \
        and     x4, x4, x3 __LF                    \
        adds    x5, x5, x4 __LF                    \
        eor     x4, x4, x3 __LF                    \
        adcs    x6, x6, x4 __LF                    \
        mov     x4, #-2 __LF                       \
        and     x4, x4, x3 __LF                    \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        adcs    x9, x9, x3 __LF                    \
        adc     x10, x10, x3 __LF                  \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32]

S2N_BN_SYMBOL(p384_montjmixadd):
        CFI_START

// Save regs and make room on stack for temporary variables

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Main code, just a sequence of basic field operations
// 8 * multiply + 3 * square + 7 * subtract

        montsqr_p384(zp2,z_1)
        montmul_p384(y2a,z_1,y_2)

        montmul_p384(x2a,zp2,x_2)
        montmul_p384(y2a,zp2,y2a)

        sub_p384(xd,x2a,x_1)
        sub_p384(yd,y2a,y_1)

        montsqr_p384(zz,xd)
        montsqr_p384(ww,yd)

        montmul_p384(zzx1,zz,x_1)
        montmul_p384(zzx2,zz,x2a)

        sub_p384(resx,ww,zzx1)
        sub_p384(t1,zzx2,zzx1)

        montmul_p384(resz,xd,z_1)

        sub_p384(resx,resx,zzx2)

        sub_p384(t2,zzx1,resx)

        montmul_p384(t1,t1,y_1)
        montmul_p384(t2,yd,t2)

        sub_p384(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        ldp     x0, x1, [z_1]
        ldp     x2, x3, [z_1+16]
        ldp     x4, x5, [z_1+32]
        orr     x6, x0, x1
        orr     x7, x2, x3
        orr     x8, x4, x5
        orr     x6, x6, x7
        orr     x6, x6, x8
        cmp     x6, xzr

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with a z = 1 coordinate (in
// Montgomery form so not the simple constant 1 but rather 2^384 - p_384),
// hence giving 0 + p2 = p2 for the final result.

        ldp     x0, x1, [resx]
        ldp     x19, x20, [x_2]
        csel    x0, x0, x19, ne
        csel    x1, x1, x20, ne
        ldp     x2, x3, [resx+16]
        ldp     x19, x20, [x_2+16]
        csel    x2, x2, x19, ne
        csel    x3, x3, x20, ne
        ldp     x4, x5, [resx+32]
        ldp     x19, x20, [x_2+32]
        csel    x4, x4, x19, ne
        csel    x5, x5, x20, ne

        ldp     x6, x7, [resy]
        ldp     x19, x20, [y_2]
        csel    x6, x6, x19, ne
        csel    x7, x7, x20, ne
        ldp     x8, x9, [resy+16]
        ldp     x19, x20, [y_2+16]
        csel    x8, x8, x19, ne
        csel    x9, x9, x20, ne
        ldp     x10, x11, [resy+32]
        ldp     x19, x20, [y_2+32]
        csel    x10, x10, x19, ne
        csel    x11, x11, x20, ne

        ldp     x12, x13, [resz]
        mov     x19, #0xffffffff00000001
        mov     x20, #0x00000000ffffffff
        csel    x12, x12, x19, ne
        csel    x13, x13, x20, ne
        ldp     x14, x15, [resz+16]
        mov     x19, #1
        csel    x14, x14, x19, ne
        csel    x15, x15, xzr, ne
        ldp     x16, x17, [resz+32]
        csel    x16, x16, xzr, ne
        csel    x17, x17, xzr, ne

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [x_3+32]
        stp     x6, x7, [y_3]
        stp     x8, x9, [y_3+16]
        stp     x10, x11, [y_3+32]
        stp     x12, x13, [z_3]
        stp     x14, x15, [z_3+16]
        stp     x16, x17, [z_3+32]

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p384_montjmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
