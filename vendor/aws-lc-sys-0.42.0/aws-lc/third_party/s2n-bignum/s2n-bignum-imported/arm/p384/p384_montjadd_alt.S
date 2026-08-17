// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjadd_alt(uint64_t p3[static 18],
//                                  const uint64_t p1[static 18],
//                                  const uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjadd_alt)
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
#define z_2 input_y, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// #NSPACE is the total stack needed for these temporaries

#define z1sq sp, #(NUMSIZE*0)
#define ww sp, #(NUMSIZE*0)
#define resx sp, #(NUMSIZE*0)

#define yd sp, #(NUMSIZE*1)
#define y2a sp, #(NUMSIZE*1)

#define x2a sp, #(NUMSIZE*2)
#define zzx2 sp, #(NUMSIZE*2)

#define zz sp, #(NUMSIZE*3)
#define t1 sp, #(NUMSIZE*3)

#define t2 sp, #(NUMSIZE*4)
#define x1a sp, #(NUMSIZE*4)
#define zzx1 sp, #(NUMSIZE*4)
#define resy sp, #(NUMSIZE*4)

#define xd sp, #(NUMSIZE*5)
#define z2sq sp, #(NUMSIZE*5)
#define resz sp, #(NUMSIZE*5)

#define y1a sp, #(NUMSIZE*6)

#define NSPACE NUMSIZE*7

// Corresponds exactly to bignum_montmul_p384_alt

#define montmul_p384(P0,P1,P2)                  \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        mul     x12, x3, x5 __LF                   \
        umulh   x13, x3, x5 __LF                   \
        mul     x11, x3, x6 __LF                   \
        umulh   x14, x3, x6 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x7, x8, [P2+16] __LF               \
        mul     x11, x3, x7 __LF                   \
        umulh   x15, x3, x7 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x8 __LF                   \
        umulh   x16, x3, x8 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        ldp     x9, x10, [P2+32] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x17, x3, x9 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x19, x3, x10 __LF                  \
        adcs    x17, x17, x11 __LF                 \
        adc     x19, x19, xzr __LF                 \
        mul     x11, x4, x5 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x6 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x7 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        mul     x11, x4, x10 __LF                  \
        adcs    x19, x19, x11 __LF                 \
        cset    x20, cs __LF                       \
        umulh   x11, x4, x5 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x6 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        umulh   x11, x4, x7 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        umulh   x11, x4, x9 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        umulh   x11, x4, x10 __LF                  \
        adc     x20, x20, x11 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        mul     x11, x3, x5 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x3, x6 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x3, x7 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x3, x8 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        mul     x11, x3, x9 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        adcs    x20, x20, x11 __LF                 \
        cset    x21, cs __LF                       \
        umulh   x11, x3, x5 __LF                   \
        adds    x15, x15, x11 __LF                 \
        umulh   x11, x3, x6 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        umulh   x11, x3, x7 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        umulh   x11, x3, x8 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        umulh   x11, x3, x9 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        umulh   x11, x3, x10 __LF                  \
        adc     x21, x21, x11 __LF                 \
        mul     x11, x4, x5 __LF                   \
        adds    x15, x15, x11 __LF                 \
        mul     x11, x4, x6 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x4, x7 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        mul     x11, x4, x10 __LF                  \
        adcs    x21, x21, x11 __LF                 \
        cset    x22, cs __LF                       \
        umulh   x11, x4, x5 __LF                   \
        adds    x16, x16, x11 __LF                 \
        umulh   x11, x4, x6 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        umulh   x11, x4, x7 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        umulh   x11, x4, x9 __LF                   \
        adcs    x21, x21, x11 __LF                 \
        umulh   x11, x4, x10 __LF                  \
        adc     x22, x22, x11 __LF                 \
        ldp     x3, x4, [P1+32] __LF               \
        mul     x11, x3, x5 __LF                   \
        adds    x16, x16, x11 __LF                 \
        mul     x11, x3, x6 __LF                   \
        adcs    x17, x17, x11 __LF                 \
        mul     x11, x3, x7 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        mul     x11, x3, x8 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        mul     x11, x3, x9 __LF                   \
        adcs    x21, x21, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        adcs    x22, x22, x11 __LF                 \
        cset    x2, cs __LF                        \
        umulh   x11, x3, x5 __LF                   \
        adds    x17, x17, x11 __LF                 \
        umulh   x11, x3, x6 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        umulh   x11, x3, x7 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        umulh   x11, x3, x8 __LF                   \
        adcs    x21, x21, x11 __LF                 \
        umulh   x11, x3, x9 __LF                   \
        adcs    x22, x22, x11 __LF                 \
        umulh   x11, x3, x10 __LF                  \
        adc     x2, x2, x11 __LF                   \
        mul     x11, x4, x5 __LF                   \
        adds    x17, x17, x11 __LF                 \
        mul     x11, x4, x6 __LF                   \
        adcs    x19, x19, x11 __LF                 \
        mul     x11, x4, x7 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x21, x21, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x22, x22, x11 __LF                 \
        mul     x11, x4, x10 __LF                  \
        adcs    x2, x2, x11 __LF                   \
        cset    x1, cs __LF                        \
        umulh   x11, x4, x5 __LF                   \
        adds    x19, x19, x11 __LF                 \
        umulh   x11, x4, x6 __LF                   \
        adcs    x20, x20, x11 __LF                 \
        umulh   x11, x4, x7 __LF                   \
        adcs    x21, x21, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x22, x22, x11 __LF                 \
        umulh   x11, x4, x9 __LF                   \
        adcs    x2, x2, x11 __LF                   \
        umulh   x11, x4, x10 __LF                  \
        adc     x1, x1, x11 __LF                   \
        lsl     x7, x12, #32 __LF                  \
        add     x12, x7, x12 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x12 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x12 __LF                   \
        umulh   x6, x6, x12 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x12 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x13, x13, x7 __LF                  \
        sbcs    x14, x14, x6 __LF                  \
        sbcs    x15, x15, x5 __LF                  \
        sbcs    x16, x16, xzr __LF                 \
        sbcs    x17, x17, xzr __LF                 \
        sbc     x12, x12, xzr __LF                 \
        lsl     x7, x13, #32 __LF                  \
        add     x13, x7, x13 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x13 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x13 __LF                   \
        umulh   x6, x6, x13 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x13 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x14, x14, x7 __LF                  \
        sbcs    x15, x15, x6 __LF                  \
        sbcs    x16, x16, x5 __LF                  \
        sbcs    x17, x17, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, xzr __LF                 \
        lsl     x7, x14, #32 __LF                  \
        add     x14, x7, x14 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x14 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x14 __LF                   \
        umulh   x6, x6, x14 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x14 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x15, x15, x7 __LF                  \
        sbcs    x16, x16, x6 __LF                  \
        sbcs    x17, x17, x5 __LF                  \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbc     x14, x14, xzr __LF                 \
        lsl     x7, x15, #32 __LF                  \
        add     x15, x7, x15 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x15 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x15 __LF                   \
        umulh   x6, x6, x15 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x15 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x16, x16, x7 __LF                  \
        sbcs    x17, x17, x6 __LF                  \
        sbcs    x12, x12, x5 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x14, x14, xzr __LF                 \
        sbc     x15, x15, xzr __LF                 \
        lsl     x7, x16, #32 __LF                  \
        add     x16, x7, x16 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x16 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x16 __LF                   \
        umulh   x6, x6, x16 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x16 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x17, x17, x7 __LF                  \
        sbcs    x12, x12, x6 __LF                  \
        sbcs    x13, x13, x5 __LF                  \
        sbcs    x14, x14, xzr __LF                 \
        sbcs    x15, x15, xzr __LF                 \
        sbc     x16, x16, xzr __LF                 \
        lsl     x7, x17, #32 __LF                  \
        add     x17, x7, x17 __LF                  \
        mov     x7, #0xffffffff00000001 __LF       \
        umulh   x7, x7, x17 __LF                   \
        mov     x6, #0xffffffff __LF               \
        mul     x5, x6, x17 __LF                   \
        umulh   x6, x6, x17 __LF                   \
        adds    x7, x7, x5 __LF                    \
        adcs    x6, x6, x17 __LF                   \
        adc     x5, xzr, xzr __LF                  \
        subs    x12, x12, x7 __LF                  \
        sbcs    x13, x13, x6 __LF                  \
        sbcs    x14, x14, x5 __LF                  \
        sbcs    x15, x15, xzr __LF                 \
        sbcs    x16, x16, xzr __LF                 \
        sbc     x17, x17, xzr __LF                 \
        adds    x12, x12, x19 __LF                 \
        adcs    x13, x13, x20 __LF                 \
        adcs    x14, x14, x21 __LF                 \
        adcs    x15, x15, x22 __LF                 \
        adcs    x16, x16, x2 __LF                  \
        adcs    x17, x17, x1 __LF                  \
        adc     x10, xzr, xzr __LF                 \
        mov     x11, #0xffffffff00000001 __LF      \
        adds    x19, x12, x11 __LF                 \
        mov     x11, #0xffffffff __LF              \
        adcs    x20, x13, x11 __LF                 \
        mov     x11, #0x1 __LF                     \
        adcs    x21, x14, x11 __LF                 \
        adcs    x22, x15, xzr __LF                 \
        adcs    x2, x16, xzr __LF                  \
        adcs    x1, x17, xzr __LF                  \
        adcs    x10, x10, xzr __LF                 \
        csel    x12, x12, x19, eq __LF             \
        csel    x13, x13, x20, eq __LF             \
        csel    x14, x14, x21, eq __LF             \
        csel    x15, x15, x22, eq __LF             \
        csel    x16, x16, x2, eq __LF              \
        csel    x17, x17, x1, eq __LF              \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x15, [P0+16] __LF             \
        stp     x16, x17, [P0+32]

// Corresponds exactly to bignum_montsqr_p384_alt

#define montsqr_p384(P0,P1)                     \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x8, x2, x4 __LF                    \
        adds    x10, x10, x8 __LF                  \
        mul     x11, x2, x5 __LF                   \
        mul     x8, x3, x4 __LF                    \
        adcs    x11, x11, x8 __LF                  \
        umulh   x12, x2, x5 __LF                   \
        mul     x8, x3, x5 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        ldp     x6, x7, [P1+32] __LF               \
        mul     x13, x2, x7 __LF                   \
        mul     x8, x3, x6 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        umulh   x14, x2, x7 __LF                   \
        mul     x8, x3, x7 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        mul     x15, x5, x6 __LF                   \
        adcs    x15, x15, xzr __LF                 \
        umulh   x16, x5, x6 __LF                   \
        adc     x16, x16, xzr __LF                 \
        umulh   x8, x2, x4 __LF                    \
        adds    x11, x11, x8 __LF                  \
        umulh   x8, x3, x4 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        umulh   x8, x3, x5 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        umulh   x8, x3, x6 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x3, x7 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        adc     x16, x16, xzr __LF                 \
        mul     x8, x2, x6 __LF                    \
        adds    x12, x12, x8 __LF                  \
        mul     x8, x4, x5 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        mul     x8, x4, x6 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        mul     x8, x4, x7 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        mul     x8, x5, x7 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        mul     x17, x6, x7 __LF                   \
        adcs    x17, x17, xzr __LF                 \
        umulh   x19, x6, x7 __LF                   \
        adc     x19, x19, xzr __LF                 \
        umulh   x8, x2, x6 __LF                    \
        adds    x13, x13, x8 __LF                  \
        umulh   x8, x4, x5 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x4, x6 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        umulh   x8, x4, x7 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        umulh   x8, x5, x7 __LF                    \
        adcs    x17, x17, x8 __LF                  \
        adc     x19, x19, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        adcs    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adcs    x17, x17, x17 __LF                 \
        adcs    x19, x19, x19 __LF                 \
        cset    x20, hs __LF                       \
        umulh   x8, x2, x2 __LF                    \
        mul     x2, x2, x2 __LF                    \
        adds    x9, x9, x8 __LF                    \
        mul     x8, x3, x3 __LF                    \
        adcs    x10, x10, x8 __LF                  \
        umulh   x8, x3, x3 __LF                    \
        adcs    x11, x11, x8 __LF                  \
        mul     x8, x4, x4 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        umulh   x8, x4, x4 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        mul     x8, x5, x5 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x5, x5 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        mul     x8, x6, x6 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        umulh   x8, x6, x6 __LF                    \
        adcs    x17, x17, x8 __LF                  \
        mul     x8, x7, x7 __LF                    \
        adcs    x19, x19, x8 __LF                  \
        umulh   x8, x7, x7 __LF                    \
        adc     x20, x20, x8 __LF                  \
        lsl     x5, x2, #32 __LF                   \
        add     x2, x5, x2 __LF                    \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x2 __LF                    \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x2 __LF                    \
        umulh   x4, x4, x2 __LF                    \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x2 __LF                    \
        adc     x3, xzr, xzr __LF                  \
        subs    x9, x9, x5 __LF                    \
        sbcs    x10, x10, x4 __LF                  \
        sbcs    x11, x11, x3 __LF                  \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbc     x2, x2, xzr __LF                   \
        lsl     x5, x9, #32 __LF                   \
        add     x9, x5, x9 __LF                    \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x9 __LF                    \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x9 __LF                    \
        umulh   x4, x4, x9 __LF                    \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x9 __LF                    \
        adc     x3, xzr, xzr __LF                  \
        subs    x10, x10, x5 __LF                  \
        sbcs    x11, x11, x4 __LF                  \
        sbcs    x12, x12, x3 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x2, x2, xzr __LF                   \
        sbc     x9, x9, xzr __LF                   \
        lsl     x5, x10, #32 __LF                  \
        add     x10, x5, x10 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x10 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x10 __LF                   \
        umulh   x4, x4, x10 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x10 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x11, x11, x5 __LF                  \
        sbcs    x12, x12, x4 __LF                  \
        sbcs    x13, x13, x3 __LF                  \
        sbcs    x2, x2, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        lsl     x5, x11, #32 __LF                  \
        add     x11, x5, x11 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x11 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x11 __LF                   \
        umulh   x4, x4, x11 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x11 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x12, x12, x5 __LF                  \
        sbcs    x13, x13, x4 __LF                  \
        sbcs    x2, x2, x3 __LF                    \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbc     x11, x11, xzr __LF                 \
        lsl     x5, x12, #32 __LF                  \
        add     x12, x5, x12 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x12 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x12 __LF                   \
        umulh   x4, x4, x12 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x12 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x13, x13, x5 __LF                  \
        sbcs    x2, x2, x4 __LF                    \
        sbcs    x9, x9, x3 __LF                    \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbc     x12, x12, xzr __LF                 \
        lsl     x5, x13, #32 __LF                  \
        add     x13, x5, x13 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x13 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x13 __LF                   \
        umulh   x4, x4, x13 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x13 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x2, x2, x5 __LF                    \
        sbcs    x9, x9, x4 __LF                    \
        sbcs    x10, x10, x3 __LF                  \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, xzr __LF                 \
        adds    x2, x2, x14 __LF                   \
        adcs    x9, x9, x15 __LF                   \
        adcs    x10, x10, x16 __LF                 \
        adcs    x11, x11, x17 __LF                 \
        adcs    x12, x12, x19 __LF                 \
        adcs    x13, x13, x20 __LF                 \
        adc     x6, xzr, xzr __LF                  \
        mov     x8, #-4294967295 __LF              \
        adds    x14, x2, x8 __LF                   \
        mov     x8, #4294967295 __LF               \
        adcs    x15, x9, x8 __LF                   \
        mov     x8, #1 __LF                        \
        adcs    x16, x10, x8 __LF                  \
        adcs    x17, x11, xzr __LF                 \
        adcs    x19, x12, xzr __LF                 \
        adcs    x20, x13, xzr __LF                 \
        adcs    x6, x6, xzr __LF                   \
        csel    x2, x2, x14, eq __LF               \
        csel    x9, x9, x15, eq __LF               \
        csel    x10, x10, x16, eq __LF             \
        csel    x11, x11, x17, eq __LF             \
        csel    x12, x12, x19, eq __LF             \
        csel    x13, x13, x20, eq __LF             \
        stp     x2, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16] __LF             \
        stp     x12, x13, [P0+32]

// Almost-Montgomery variant which we use when an input to other muls
// with the other argument fully reduced (which is always safe). In
// fact, with the Karatsuba-based Montgomery mul here, we don't even
// *need* the restriction that the other argument is reduced.

#define amontsqr_p384(P0,P1)                    \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x8, x2, x4 __LF                    \
        adds    x10, x10, x8 __LF                  \
        mul     x11, x2, x5 __LF                   \
        mul     x8, x3, x4 __LF                    \
        adcs    x11, x11, x8 __LF                  \
        umulh   x12, x2, x5 __LF                   \
        mul     x8, x3, x5 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        ldp     x6, x7, [P1+32] __LF               \
        mul     x13, x2, x7 __LF                   \
        mul     x8, x3, x6 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        umulh   x14, x2, x7 __LF                   \
        mul     x8, x3, x7 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        mul     x15, x5, x6 __LF                   \
        adcs    x15, x15, xzr __LF                 \
        umulh   x16, x5, x6 __LF                   \
        adc     x16, x16, xzr __LF                 \
        umulh   x8, x2, x4 __LF                    \
        adds    x11, x11, x8 __LF                  \
        umulh   x8, x3, x4 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        umulh   x8, x3, x5 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        umulh   x8, x3, x6 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x3, x7 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        adc     x16, x16, xzr __LF                 \
        mul     x8, x2, x6 __LF                    \
        adds    x12, x12, x8 __LF                  \
        mul     x8, x4, x5 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        mul     x8, x4, x6 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        mul     x8, x4, x7 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        mul     x8, x5, x7 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        mul     x17, x6, x7 __LF                   \
        adcs    x17, x17, xzr __LF                 \
        umulh   x19, x6, x7 __LF                   \
        adc     x19, x19, xzr __LF                 \
        umulh   x8, x2, x6 __LF                    \
        adds    x13, x13, x8 __LF                  \
        umulh   x8, x4, x5 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x4, x6 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        umulh   x8, x4, x7 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        umulh   x8, x5, x7 __LF                    \
        adcs    x17, x17, x8 __LF                  \
        adc     x19, x19, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        adcs    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adcs    x17, x17, x17 __LF                 \
        adcs    x19, x19, x19 __LF                 \
        cset    x20, hs __LF                       \
        umulh   x8, x2, x2 __LF                    \
        mul     x2, x2, x2 __LF                    \
        adds    x9, x9, x8 __LF                    \
        mul     x8, x3, x3 __LF                    \
        adcs    x10, x10, x8 __LF                  \
        umulh   x8, x3, x3 __LF                    \
        adcs    x11, x11, x8 __LF                  \
        mul     x8, x4, x4 __LF                    \
        adcs    x12, x12, x8 __LF                  \
        umulh   x8, x4, x4 __LF                    \
        adcs    x13, x13, x8 __LF                  \
        mul     x8, x5, x5 __LF                    \
        adcs    x14, x14, x8 __LF                  \
        umulh   x8, x5, x5 __LF                    \
        adcs    x15, x15, x8 __LF                  \
        mul     x8, x6, x6 __LF                    \
        adcs    x16, x16, x8 __LF                  \
        umulh   x8, x6, x6 __LF                    \
        adcs    x17, x17, x8 __LF                  \
        mul     x8, x7, x7 __LF                    \
        adcs    x19, x19, x8 __LF                  \
        umulh   x8, x7, x7 __LF                    \
        adc     x20, x20, x8 __LF                  \
        lsl     x5, x2, #32 __LF                   \
        add     x2, x5, x2 __LF                    \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x2 __LF                    \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x2 __LF                    \
        umulh   x4, x4, x2 __LF                    \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x2 __LF                    \
        adc     x3, xzr, xzr __LF                  \
        subs    x9, x9, x5 __LF                    \
        sbcs    x10, x10, x4 __LF                  \
        sbcs    x11, x11, x3 __LF                  \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        sbc     x2, x2, xzr __LF                   \
        lsl     x5, x9, #32 __LF                   \
        add     x9, x5, x9 __LF                    \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x9 __LF                    \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x9 __LF                    \
        umulh   x4, x4, x9 __LF                    \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x9 __LF                    \
        adc     x3, xzr, xzr __LF                  \
        subs    x10, x10, x5 __LF                  \
        sbcs    x11, x11, x4 __LF                  \
        sbcs    x12, x12, x3 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x2, x2, xzr __LF                   \
        sbc     x9, x9, xzr __LF                   \
        lsl     x5, x10, #32 __LF                  \
        add     x10, x5, x10 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x10 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x10 __LF                   \
        umulh   x4, x4, x10 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x10 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x11, x11, x5 __LF                  \
        sbcs    x12, x12, x4 __LF                  \
        sbcs    x13, x13, x3 __LF                  \
        sbcs    x2, x2, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        lsl     x5, x11, #32 __LF                  \
        add     x11, x5, x11 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x11 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x11 __LF                   \
        umulh   x4, x4, x11 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x11 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x12, x12, x5 __LF                  \
        sbcs    x13, x13, x4 __LF                  \
        sbcs    x2, x2, x3 __LF                    \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbc     x11, x11, xzr __LF                 \
        lsl     x5, x12, #32 __LF                  \
        add     x12, x5, x12 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x12 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x12 __LF                   \
        umulh   x4, x4, x12 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x12 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x13, x13, x5 __LF                  \
        sbcs    x2, x2, x4 __LF                    \
        sbcs    x9, x9, x3 __LF                    \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbc     x12, x12, xzr __LF                 \
        lsl     x5, x13, #32 __LF                  \
        add     x13, x5, x13 __LF                  \
        mov     x5, #-4294967295 __LF              \
        umulh   x5, x5, x13 __LF                   \
        mov     x4, #4294967295 __LF               \
        mul     x3, x4, x13 __LF                   \
        umulh   x4, x4, x13 __LF                   \
        adds    x5, x5, x3 __LF                    \
        adcs    x4, x4, x13 __LF                   \
        adc     x3, xzr, xzr __LF                  \
        subs    x2, x2, x5 __LF                    \
        sbcs    x9, x9, x4 __LF                    \
        sbcs    x10, x10, x3 __LF                  \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, xzr __LF                 \
        adds    x2, x2, x14 __LF                   \
        adcs    x9, x9, x15 __LF                   \
        adcs    x10, x10, x16 __LF                 \
        adcs    x11, x11, x17 __LF                 \
        adcs    x12, x12, x19 __LF                 \
        adcs    x13, x13, x20 __LF                 \
        mov     x14, #-4294967295 __LF             \
        mov     x15, #4294967295 __LF              \
        csel    x14, x14, xzr, cs __LF             \
        csel    x15, x15, xzr, cs __LF             \
        cset    x16, cs __LF                       \
        adds    x2, x2, x14 __LF                   \
        adcs    x9, x9, x15 __LF                   \
        adcs    x10, x10, x16 __LF                 \
        adcs    x11, x11, xzr __LF                 \
        adcs    x12, x12, xzr __LF                 \
        adc     x13, x13, xzr __LF                 \
        stp     x2, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16] __LF             \
        stp     x12, x13, [P0+32]

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

S2N_BN_SYMBOL(p384_montjadd_alt):
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

        amontsqr_p384(z1sq,z_1)
        amontsqr_p384(z2sq,z_2)

        montmul_p384(y1a,z_2,y_1)
        montmul_p384(y2a,z_1,y_2)

        montmul_p384(x2a,z1sq,x_2)
        montmul_p384(x1a,z2sq,x_1)
        montmul_p384(y2a,z1sq,y2a)
        montmul_p384(y1a,z2sq,y1a)

        sub_p384(xd,x2a,x1a)
        sub_p384(yd,y2a,y1a)

        amontsqr_p384(zz,xd)
        montsqr_p384(ww,yd)

        montmul_p384(zzx1,zz,x1a)
        montmul_p384(zzx2,zz,x2a)

        sub_p384(resx,ww,zzx1)
        sub_p384(t1,zzx2,zzx1)

        montmul_p384(xd,xd,z_1)

        sub_p384(resx,resx,zzx2)

        sub_p384(t2,zzx1,resx)

        montmul_p384(t1,t1,y1a)
        montmul_p384(resz,xd,z_2)
        montmul_p384(t2,yd,t2)

        sub_p384(resy,t2,t1)

// Load in the z coordinates of the inputs to check for P1 = 0 and P2 = 0
// The condition codes get set by a comparison (P2 != 0) - (P1 != 0)
// So  "HI" <=> CF /\ ~ZF <=> P1 = 0 /\ ~(P2 = 0)
// and "LO" <=> ~CF       <=> ~(P1 = 0) /\ P2 = 0

        ldp     x0, x1, [z_1]
        ldp     x2, x3, [z_1+16]
        ldp     x4, x5, [z_1+32]

        orr     x20, x0, x1
        orr     x21, x2, x3
        orr     x22, x4, x5
        orr     x20, x20, x21
        orr     x20, x20, x22
        cmp     x20, xzr
        cset    x20, ne

        ldp     x6, x7, [z_2]
        ldp     x8, x9, [z_2+16]
        ldp     x10, x11, [z_2+32]

        orr     x21, x6, x7
        orr     x22, x8, x9
        orr     x23, x10, x11
        orr     x21, x21, x22
        orr     x21, x21, x23
        cmp     x21, xzr
        cset    x21, ne

        cmp     x21, x20

// Multiplex the outputs accordingly, re-using the z's in registers

        ldp     x12, x13, [resz]
        csel    x12, x0, x12, lo
        csel    x13, x1, x13, lo
        csel    x12, x6, x12, hi
        csel    x13, x7, x13, hi
        ldp     x14, x15, [resz+16]
        csel    x14, x2, x14, lo
        csel    x15, x3, x15, lo
        csel    x14, x8, x14, hi
        csel    x15, x9, x15, hi
        ldp     x16, x17, [resz+32]
        csel    x16, x4, x16, lo
        csel    x17, x5, x17, lo
        csel    x16, x10, x16, hi
        csel    x17, x11, x17, hi

        ldp     x20, x21, [x_1]
        ldp     x0, x1, [resx]
        csel    x0, x20, x0, lo
        csel    x1, x21, x1, lo
        ldp     x20, x21, [x_2]
        csel    x0, x20, x0, hi
        csel    x1, x21, x1, hi

        ldp     x20, x21, [x_1+16]
        ldp     x2, x3, [resx+16]
        csel    x2, x20, x2, lo
        csel    x3, x21, x3, lo
        ldp     x20, x21, [x_2+16]
        csel    x2, x20, x2, hi
        csel    x3, x21, x3, hi

        ldp     x20, x21, [x_1+32]
        ldp     x4, x5, [resx+32]
        csel    x4, x20, x4, lo
        csel    x5, x21, x5, lo
        ldp     x20, x21, [x_2+32]
        csel    x4, x20, x4, hi
        csel    x5, x21, x5, hi

        ldp     x20, x21, [y_1]
        ldp     x6, x7, [resy]
        csel    x6, x20, x6, lo
        csel    x7, x21, x7, lo
        ldp     x20, x21, [y_2]
        csel    x6, x20, x6, hi
        csel    x7, x21, x7, hi

        ldp     x20, x21, [y_1+16]
        ldp     x8, x9, [resy+16]
        csel    x8, x20, x8, lo
        csel    x9, x21, x9, lo
        ldp     x20, x21, [y_2+16]
        csel    x8, x20, x8, hi
        csel    x9, x21, x9, hi

        ldp     x20, x21, [y_1+32]
        ldp     x10, x11, [resy+32]
        csel    x10, x20, x10, lo
        csel    x11, x21, x11, lo
        ldp     x20, x21, [y_2+32]
        csel    x10, x20, x10, hi
        csel    x11, x21, x11, hi

// Finally store back the multiplexed values

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

S2N_BN_SIZE_DIRECTIVE(p384_montjadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
