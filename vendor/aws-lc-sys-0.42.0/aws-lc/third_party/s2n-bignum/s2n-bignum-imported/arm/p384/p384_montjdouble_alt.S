// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjdouble_alt(uint64_t p3[static 18],
//                                     const uint64_t p1[static 18]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjdouble_alt)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48

// Stable homes for input arguments during main code sequence

#define input_z x23
#define input_x x24

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// #NSPACE is the total stack needed for these temporaries

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

// Corresponds exactly to bignum_add_p384

#define add_p384(P0,P1,P2)                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adds    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        adcs    x9, x9, x4 __LF                    \
        adcs    x10, x10, x3 __LF                  \
        adc     x3, xzr, xzr __LF                  \
        mov     x4, #0xffffffff __LF               \
        cmp     x5, x4 __LF                        \
        mov     x4, #0xffffffff00000000 __LF       \
        sbcs    xzr, x6, x4 __LF                   \
        mov     x4, #0xfffffffffffffffe __LF       \
        sbcs    xzr, x7, x4 __LF                   \
        adcs    xzr, x8, xzr __LF                  \
        adcs    xzr, x9, xzr __LF                  \
        adcs    xzr, x10, xzr __LF                 \
        adcs    x3, x3, xzr __LF                   \
        csetm   x3, ne __LF                        \
        mov     x4, #0xffffffff __LF               \
        and     x4, x4, x3 __LF                    \
        subs    x5, x5, x4 __LF                    \
        eor     x4, x4, x3 __LF                    \
        sbcs    x6, x6, x4 __LF                    \
        mov     x4, #0xfffffffffffffffe __LF       \
        and     x4, x4, x3 __LF                    \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        sbcs    x9, x9, x3 __LF                    \
        sbc     x10, x10, x3 __LF                  \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32]

// P0 = 4 * P1 - P2

#define cmsub41_p384(P0,P1,P2)                  \
        ldp     x1, x2, [P1] __LF                  \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x5, x6, [P1+32] __LF               \
        lsl     x0, x1, #2 __LF                    \
        ldp     x7, x8, [P2] __LF                  \
        subs    x0, x0, x7 __LF                    \
        extr    x1, x2, x1, #62 __LF               \
        sbcs    x1, x1, x8 __LF                    \
        ldp     x7, x8, [P2+16] __LF               \
        extr    x2, x3, x2, #62 __LF               \
        sbcs    x2, x2, x7 __LF                    \
        extr    x3, x4, x3, #62 __LF               \
        sbcs    x3, x3, x8 __LF                    \
        extr    x4, x5, x4, #62 __LF               \
        ldp     x7, x8, [P2+32] __LF               \
        sbcs    x4, x4, x7 __LF                    \
        extr    x5, x6, x5, #62 __LF               \
        sbcs    x5, x5, x8 __LF                    \
        lsr     x6, x6, #62 __LF                   \
        adc     x6, x6, xzr __LF                   \
        lsl     x7, x6, #32 __LF                   \
        subs    x8, x6, x7 __LF                    \
        sbc     x7, x7, xzr __LF                   \
        adds    x0, x0, x8 __LF                    \
        adcs    x1, x1, x7 __LF                    \
        adcs    x2, x2, x6 __LF                    \
        adcs    x3, x3, xzr __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        csetm   x8, cc __LF                        \
        mov     x9, #0xffffffff __LF               \
        and     x9, x9, x8 __LF                    \
        adds    x0, x0, x9 __LF                    \
        eor     x9, x9, x8 __LF                    \
        adcs    x1, x1, x9 __LF                    \
        mov     x9, #0xfffffffffffffffe __LF       \
        and     x9, x9, x8 __LF                    \
        adcs    x2, x2, x9 __LF                    \
        adcs    x3, x3, x8 __LF                    \
        adcs    x4, x4, x8 __LF                    \
        adc     x5, x5, x8 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16] __LF               \
        stp     x4, x5, [P0+32]

// P0 = C * P1 - D * P2

#define cmsub_p384(P0,C,P1,D,P2)                \
        ldp     x0, x1, [P2] __LF                  \
        mov     x6, #0x00000000ffffffff __LF       \
        subs    x6, x6, x0 __LF                    \
        mov     x7, #0xffffffff00000000 __LF       \
        sbcs    x7, x7, x1 __LF                    \
        ldp     x0, x1, [P2+16] __LF               \
        mov     x8, #0xfffffffffffffffe __LF       \
        sbcs    x8, x8, x0 __LF                    \
        mov     x13, #0xffffffffffffffff __LF      \
        sbcs    x9, x13, x1 __LF                   \
        ldp     x0, x1, [P2+32] __LF               \
        sbcs    x10, x13, x0 __LF                  \
        sbc     x11, x13, x1 __LF                  \
        mov     x12, D __LF                        \
        mul     x0, x12, x6 __LF                   \
        mul     x1, x12, x7 __LF                   \
        mul     x2, x12, x8 __LF                   \
        mul     x3, x12, x9 __LF                   \
        mul     x4, x12, x10 __LF                  \
        mul     x5, x12, x11 __LF                  \
        umulh   x6, x12, x6 __LF                   \
        umulh   x7, x12, x7 __LF                   \
        umulh   x8, x12, x8 __LF                   \
        umulh   x9, x12, x9 __LF                   \
        umulh   x10, x12, x10 __LF                 \
        umulh   x12, x12, x11 __LF                 \
        adds    x1, x1, x6 __LF                    \
        adcs    x2, x2, x7 __LF                    \
        adcs    x3, x3, x8 __LF                    \
        adcs    x4, x4, x9 __LF                    \
        adcs    x5, x5, x10 __LF                   \
        mov     x6, #1 __LF                        \
        adc     x6, x12, x6 __LF                   \
        ldp     x8, x9, [P1] __LF                  \
        ldp     x10, x11, [P1+16] __LF             \
        ldp     x12, x13, [P1+32] __LF             \
        mov     x14, C __LF                        \
        mul     x15, x14, x8 __LF                  \
        umulh   x8, x14, x8 __LF                   \
        adds    x0, x0, x15 __LF                   \
        mul     x15, x14, x9 __LF                  \
        umulh   x9, x14, x9 __LF                   \
        adcs    x1, x1, x15 __LF                   \
        mul     x15, x14, x10 __LF                 \
        umulh   x10, x14, x10 __LF                 \
        adcs    x2, x2, x15 __LF                   \
        mul     x15, x14, x11 __LF                 \
        umulh   x11, x14, x11 __LF                 \
        adcs    x3, x3, x15 __LF                   \
        mul     x15, x14, x12 __LF                 \
        umulh   x12, x14, x12 __LF                 \
        adcs    x4, x4, x15 __LF                   \
        mul     x15, x14, x13 __LF                 \
        umulh   x13, x14, x13 __LF                 \
        adcs    x5, x5, x15 __LF                   \
        adc     x6, x6, xzr __LF                   \
        adds    x1, x1, x8 __LF                    \
        adcs    x2, x2, x9 __LF                    \
        adcs    x3, x3, x10 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adcs    x5, x5, x12 __LF                   \
        adcs    x6, x6, x13 __LF                   \
        lsl     x7, x6, #32 __LF                   \
        subs    x8, x6, x7 __LF                    \
        sbc     x7, x7, xzr __LF                   \
        adds    x0, x0, x8 __LF                    \
        adcs    x1, x1, x7 __LF                    \
        adcs    x2, x2, x6 __LF                    \
        adcs    x3, x3, xzr __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        csetm   x6, cc __LF                        \
        mov     x7, #0xffffffff __LF               \
        and     x7, x7, x6 __LF                    \
        adds    x0, x0, x7 __LF                    \
        eor     x7, x7, x6 __LF                    \
        adcs    x1, x1, x7 __LF                    \
        mov     x7, #0xfffffffffffffffe __LF       \
        and     x7, x7, x6 __LF                    \
        adcs    x2, x2, x7 __LF                    \
        adcs    x3, x3, x6 __LF                    \
        adcs    x4, x4, x6 __LF                    \
        adc     x5, x5, x6 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16] __LF               \
        stp     x4, x5, [P0+32]

// A weak version of add that only guarantees sum in 6 digits

#define weakadd_p384(P0,P1,P2)                  \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adds    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        adcs    x9, x9, x4 __LF                    \
        adcs    x10, x10, x3 __LF                  \
        csetm   x3, cs __LF                        \
        mov     x4, #0xffffffff __LF               \
        and     x4, x4, x3 __LF                    \
        subs    x5, x5, x4 __LF                    \
        eor     x4, x4, x3 __LF                    \
        sbcs    x6, x6, x4 __LF                    \
        mov     x4, #0xfffffffffffffffe __LF       \
        and     x4, x4, x3 __LF                    \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        sbcs    x9, x9, x3 __LF                    \
        sbc     x10, x10, x3 __LF                  \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32]

// P0 = 3 * P1 - 8 * P2

#define cmsub38_p384(P0,P1,P2)                  \
        ldp     x0, x1, [P2] __LF                  \
        mov     x6, #0x00000000ffffffff __LF       \
        subs    x6, x6, x0 __LF                    \
        mov     x7, #0xffffffff00000000 __LF       \
        sbcs    x7, x7, x1 __LF                    \
        ldp     x0, x1, [P2+16] __LF               \
        mov     x8, #0xfffffffffffffffe __LF       \
        sbcs    x8, x8, x0 __LF                    \
        mov     x13, #0xffffffffffffffff __LF      \
        sbcs    x9, x13, x1 __LF                   \
        ldp     x0, x1, [P2+32] __LF               \
        sbcs    x10, x13, x0 __LF                  \
        sbc     x11, x13, x1 __LF                  \
        lsl     x0, x6, #3 __LF                    \
        extr    x1, x7, x6, #61 __LF               \
        extr    x2, x8, x7, #61 __LF               \
        extr    x3, x9, x8, #61 __LF               \
        extr    x4, x10, x9, #61 __LF              \
        extr    x5, x11, x10, #61 __LF             \
        lsr     x6, x11, #61 __LF                  \
        add     x6, x6, #1 __LF                    \
        ldp     x8, x9, [P1] __LF                  \
        ldp     x10, x11, [P1+16] __LF             \
        ldp     x12, x13, [P1+32] __LF             \
        mov     x14, 3 __LF                        \
        mul     x15, x14, x8 __LF                  \
        umulh   x8, x14, x8 __LF                   \
        adds    x0, x0, x15 __LF                   \
        mul     x15, x14, x9 __LF                  \
        umulh   x9, x14, x9 __LF                   \
        adcs    x1, x1, x15 __LF                   \
        mul     x15, x14, x10 __LF                 \
        umulh   x10, x14, x10 __LF                 \
        adcs    x2, x2, x15 __LF                   \
        mul     x15, x14, x11 __LF                 \
        umulh   x11, x14, x11 __LF                 \
        adcs    x3, x3, x15 __LF                   \
        mul     x15, x14, x12 __LF                 \
        umulh   x12, x14, x12 __LF                 \
        adcs    x4, x4, x15 __LF                   \
        mul     x15, x14, x13 __LF                 \
        umulh   x13, x14, x13 __LF                 \
        adcs    x5, x5, x15 __LF                   \
        adc     x6, x6, xzr __LF                   \
        adds    x1, x1, x8 __LF                    \
        adcs    x2, x2, x9 __LF                    \
        adcs    x3, x3, x10 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adcs    x5, x5, x12 __LF                   \
        adcs    x6, x6, x13 __LF                   \
        lsl     x7, x6, #32 __LF                   \
        subs    x8, x6, x7 __LF                    \
        sbc     x7, x7, xzr __LF                   \
        adds    x0, x0, x8 __LF                    \
        adcs    x1, x1, x7 __LF                    \
        adcs    x2, x2, x6 __LF                    \
        adcs    x3, x3, xzr __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        csetm   x6, cc __LF                        \
        mov     x7, #0xffffffff __LF               \
        and     x7, x7, x6 __LF                    \
        adds    x0, x0, x7 __LF                    \
        eor     x7, x7, x6 __LF                    \
        adcs    x1, x1, x7 __LF                    \
        mov     x7, #0xfffffffffffffffe __LF       \
        and     x7, x7, x6 __LF                    \
        adcs    x2, x2, x7 __LF                    \
        adcs    x3, x3, x6 __LF                    \
        adcs    x4, x4, x6 __LF                    \
        adc     x5, x5, x6 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16] __LF               \
        stp     x4, x5, [P0+32]

S2N_BN_SYMBOL(p384_montjdouble_alt):
        CFI_START

// Save regs and make room on stack for temporary variables

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1

// Main code, just a sequence of basic field operations

// z2 = z^2
// y2 = y^2

        montsqr_p384(z2,z_1)
        montsqr_p384(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        weakadd_p384(t1,x_1,z2)
        sub_p384(t2,x_1,z2)
        montmul_p384(x2p,t1,t2)

// t1 = y + z
// x4p = x2p^2
// xy2 = x * y^2

        add_p384(t1,y_1,z_1)
        montsqr_p384(x4p,x2p)
        montmul_p384(xy2,x_1,y2)

// t2 = (y + z)^2

        montsqr_p384(t2,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p384(d,12,xy2,9,x4p)
        sub_p384(t1,t2,z2)

// y4 = y^4

        montsqr_p384(y4,y2)

// z_3' = 2 * y * z
// dx2 = d * x2p

        sub_p384(z_3,t1,y2)
        montmul_p384(dx2,d,x2p)

// x' = 4 * xy2 - d

        cmsub41_p384(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p384(y_3,dx2,y4)

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p384_montjdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
