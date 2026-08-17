// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjdouble_alt(uint64_t p3[static 12],
//                                     const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjdouble_alt)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x15
#define input_x x16

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
#define y4 sp, #(NUMSIZE*0)

#define y2 sp, #(NUMSIZE*1)

#define t1 sp, #(NUMSIZE*2)

#define t2 sp, #(NUMSIZE*3)
#define x2p sp, #(NUMSIZE*3)
#define dx2 sp, #(NUMSIZE*3)

#define xy2 sp, #(NUMSIZE*4)

#define x4p sp, #(NUMSIZE*5)
#define d sp, #(NUMSIZE*5)

#define NSPACE NUMSIZE*6

// Corresponds exactly to bignum_montmul_p256_alt except registers

#define montmul_p256(P0,P1,P2)                  \
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
        mov     x10, #0xffffffff00000001 __LF      \
        adds    x13, x13, x12, lsl #32 __LF        \
        lsr     x11, x12, #32 __LF                 \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x12, x10 __LF                 \
        umulh   x12, x12, x10 __LF                 \
        adcs    x0, x0, x11 __LF                   \
        adc     x12, x12, xzr __LF                 \
        umulh   x11, x6, x7 __LF                   \
        adds    x1, x1, x11 __LF                   \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        adds    x14, x14, x13, lsl #32 __LF        \
        lsr     x11, x13, #32 __LF                 \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x13, x10 __LF                 \
        umulh   x13, x13, x10 __LF                 \
        adcs    x12, x12, x11 __LF                 \
        adc     x13, x13, xzr __LF                 \
        adds    x0, x0, x14, lsl #32 __LF          \
        lsr     x11, x14, #32 __LF                 \
        adcs    x12, x12, x11 __LF                 \
        mul     x11, x14, x10 __LF                 \
        umulh   x14, x14, x10 __LF                 \
        adcs    x13, x13, x11 __LF                 \
        adc     x14, x14, xzr __LF                 \
        adds    x12, x12, x0, lsl #32 __LF         \
        lsr     x11, x0, #32 __LF                  \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, x0, x10 __LF                  \
        umulh   x0, x0, x10 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        adc     x0, x0, xzr __LF                   \
        adds    x12, x12, x1 __LF                  \
        adcs    x13, x13, x3 __LF                  \
        adcs    x14, x14, x4 __LF                  \
        adcs    x0, x0, x5 __LF                    \
        cset    x8, cs __LF                        \
        mov     x11, #0xffffffff __LF              \
        adds    x1, x12, #0x1 __LF                 \
        sbcs    x3, x13, x11 __LF                  \
        sbcs    x4, x14, xzr __LF                  \
        sbcs    x5, x0, x10 __LF                   \
        sbcs    xzr, x8, xzr __LF                  \
        csel    x12, x12, x1, cc __LF              \
        csel    x13, x13, x3, cc __LF              \
        csel    x14, x14, x4, cc __LF              \
        csel    x0, x0, x5, cc __LF                \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x0, [P0+16]

// Corresponds exactly to bignum_montsqr_p256_alt

#define montsqr_p256(P0,P1)                     \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x11, x2, x5 __LF                   \
        umulh   x12, x2, x5 __LF                   \
        mul     x6, x2, x4 __LF                    \
        umulh   x7, x2, x4 __LF                    \
        adds    x10, x10, x6 __LF                  \
        adcs    x11, x11, x7 __LF                  \
        mul     x6, x3, x4 __LF                    \
        umulh   x7, x3, x4 __LF                    \
        adc     x7, x7, xzr __LF                   \
        adds    x11, x11, x6 __LF                  \
        mul     x13, x4, x5 __LF                   \
        umulh   x14, x4, x5 __LF                   \
        adcs    x12, x12, x7 __LF                  \
        mul     x6, x3, x5 __LF                    \
        umulh   x7, x3, x5 __LF                    \
        adc     x7, x7, xzr __LF                   \
        adds    x12, x12, x6 __LF                  \
        adcs    x13, x13, x7 __LF                  \
        adc     x14, x14, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        cset    x7, hs __LF                        \
        umulh   x6, x2, x2 __LF                    \
        mul     x8, x2, x2 __LF                    \
        adds    x9, x9, x6 __LF                    \
        mul     x6, x3, x3 __LF                    \
        adcs    x10, x10, x6 __LF                  \
        umulh   x6, x3, x3 __LF                    \
        adcs    x11, x11, x6 __LF                  \
        mul     x6, x4, x4 __LF                    \
        adcs    x12, x12, x6 __LF                  \
        umulh   x6, x4, x4 __LF                    \
        adcs    x13, x13, x6 __LF                  \
        mul     x6, x5, x5 __LF                    \
        adcs    x14, x14, x6 __LF                  \
        umulh   x6, x5, x5 __LF                    \
        adc     x7, x7, x6 __LF                    \
        mov     x5, #-4294967295 __LF              \
        adds    x9, x9, x8, lsl #32 __LF           \
        lsr     x3, x8, #32 __LF                   \
        adcs    x10, x10, x3 __LF                  \
        mul     x2, x8, x5 __LF                    \
        umulh   x8, x8, x5 __LF                    \
        adcs    x11, x11, x2 __LF                  \
        adc     x8, x8, xzr __LF                   \
        adds    x10, x10, x9, lsl #32 __LF         \
        lsr     x3, x9, #32 __LF                   \
        adcs    x11, x11, x3 __LF                  \
        mul     x2, x9, x5 __LF                    \
        umulh   x9, x9, x5 __LF                    \
        adcs    x8, x8, x2 __LF                    \
        adc     x9, x9, xzr __LF                   \
        adds    x11, x11, x10, lsl #32 __LF        \
        lsr     x3, x10, #32 __LF                  \
        adcs    x8, x8, x3 __LF                    \
        mul     x2, x10, x5 __LF                   \
        umulh   x10, x10, x5 __LF                  \
        adcs    x9, x9, x2 __LF                    \
        adc     x10, x10, xzr __LF                 \
        adds    x8, x8, x11, lsl #32 __LF          \
        lsr     x3, x11, #32 __LF                  \
        adcs    x9, x9, x3 __LF                    \
        mul     x2, x11, x5 __LF                   \
        umulh   x11, x11, x5 __LF                  \
        adcs    x10, x10, x2 __LF                  \
        adc     x11, x11, xzr __LF                 \
        adds    x8, x8, x12 __LF                   \
        adcs    x9, x9, x13 __LF                   \
        adcs    x10, x10, x14 __LF                 \
        adcs    x11, x11, x7 __LF                  \
        cset    x2, hs __LF                        \
        mov     x3, #4294967295 __LF               \
        adds    x12, x8, #1 __LF                   \
        sbcs    x13, x9, x3 __LF                   \
        sbcs    x14, x10, xzr __LF                 \
        sbcs    x7, x11, x5 __LF                   \
        sbcs    xzr, x2, xzr __LF                  \
        csel    x8, x8, x12, lo __LF               \
        csel    x9, x9, x13, lo __LF               \
        csel    x10, x10, x14, lo __LF             \
        csel    x11, x11, x7, lo __LF              \
        stp     x8, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16]

// Corresponds exactly to bignum_sub_p256

#define sub_p256(P0,P1,P2)                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        csetm   x3, lo __LF                        \
        adds    x5, x5, x3 __LF                    \
        and     x4, x3, #0xffffffff __LF           \
        adcs    x6, x6, x4 __LF                    \
        adcs    x7, x7, xzr __LF                   \
        and     x4, x3, #0xffffffff00000001 __LF   \
        adc     x8, x8, x4 __LF                    \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// Corresponds exactly to bignum_add_p256

#define add_p256(P0,P1,P2)                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adds    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        adc     x3, xzr, xzr __LF                  \
        cmn     x5, #1 __LF                        \
        mov     x4, #4294967295 __LF               \
        sbcs    xzr, x6, x4 __LF                   \
        sbcs    xzr, x7, xzr __LF                  \
        mov     x4, #-4294967295 __LF              \
        sbcs    xzr, x8, x4 __LF                   \
        adcs    x3, x3, xzr __LF                   \
        csetm   x3, ne __LF                        \
        subs    x5, x5, x3 __LF                    \
        and     x4, x3, #0xffffffff __LF           \
        sbcs    x6, x6, x4 __LF                    \
        sbcs    x7, x7, xzr __LF                   \
        and     x4, x3, #0xffffffff00000001 __LF   \
        sbc     x8, x8, x4 __LF                    \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// A weak version of add that only guarantees sum in 4 digits

#define weakadd_p256(P0,P1,P2)                  \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adds    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        csetm   x3, cs __LF                        \
        subs    x5, x5, x3 __LF                    \
        and     x1, x3, #4294967295 __LF           \
        sbcs    x6, x6, x1 __LF                    \
        sbcs    x7, x7, xzr __LF                   \
        and     x2, x3, #-4294967295 __LF          \
        sbc     x8, x8, x2 __LF                    \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// P0 = C * P1 - D * P2 computed as D * (p_256 - P2) + C * P1
// Quotient estimation is done just as q = h + 1 as in bignum_triple_p256
// This also applies to the other functions following.

#define cmsub_p256(P0,C,P1,D,P2)                \
        mov     x1, D __LF                         \
        mov     x2, #-1 __LF                       \
        ldp     x9, x10, [P2] __LF                 \
        subs    x9, x2, x9 __LF                    \
        mov     x2, #4294967295 __LF               \
        sbcs    x10, x2, x10 __LF                  \
        ldp     x11, x12, [P2+16] __LF             \
        sbcs    x11, xzr, x11 __LF                 \
        mov     x2, #-4294967295 __LF              \
        sbc     x12, x2, x12 __LF                  \
        mul     x3, x1, x9 __LF                    \
        mul     x4, x1, x10 __LF                   \
        mul     x5, x1, x11 __LF                   \
        mul     x6, x1, x12 __LF                   \
        umulh   x9, x1, x9 __LF                    \
        umulh   x10, x1, x10 __LF                  \
        umulh   x11, x1, x11 __LF                  \
        umulh   x7, x1, x12 __LF                   \
        adds    x4, x4, x9 __LF                    \
        adcs    x5, x5, x10 __LF                   \
        adcs    x6, x6, x11 __LF                   \
        adc     x7, x7, xzr __LF                   \
        mov     x1, C __LF                         \
        ldp     x9, x10, [P1] __LF                 \
        mul     x8, x9, x1 __LF                    \
        umulh   x9, x9, x1 __LF                    \
        adds    x3, x3, x8 __LF                    \
        mul     x8, x10, x1 __LF                   \
        umulh   x10, x10, x1 __LF                  \
        adcs    x4, x4, x8 __LF                    \
        ldp     x11, x12, [P1+16] __LF             \
        mul     x8, x11, x1 __LF                   \
        umulh   x11, x11, x1 __LF                  \
        adcs    x5, x5, x8 __LF                    \
        mul     x8, x12, x1 __LF                   \
        umulh   x12, x12, x1 __LF                  \
        adcs    x6, x6, x8 __LF                    \
        adc     x7, x7, xzr __LF                   \
        adds    x4, x4, x9 __LF                    \
        adcs    x5, x5, x10 __LF                   \
        adcs    x6, x6, x11 __LF                   \
        adc     x7, x7, x12 __LF                   \
        add     x8, x7, #1 __LF                    \
        lsl     x10, x8, #32 __LF                  \
        adds    x6, x6, x10 __LF                   \
        adc     x7, x7, xzr __LF                   \
        neg     x9, x8 __LF                        \
        sub     x10, x10, #1 __LF                  \
        subs    x3, x3, x9 __LF                    \
        sbcs    x4, x4, x10 __LF                   \
        sbcs    x5, x5, xzr __LF                   \
        sbcs    x6, x6, x8 __LF                    \
        sbc     x8, x7, x8 __LF                    \
        adds    x3, x3, x8 __LF                    \
        and     x9, x8, #4294967295 __LF           \
        adcs    x4, x4, x9 __LF                    \
        adcs    x5, x5, xzr __LF                   \
        neg     x10, x9 __LF                       \
        adc     x6, x6, x10 __LF                   \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16]

// P0 = 4 * P1 - P2, by direct subtraction of P2; the method
// in bignum_cmul_p256 etc. for quotient estimation still
// works when the value to be reduced is negative, as
// long as it is  > -p_256, which is the case here. The
// actual accumulation of q * p_256 is done a bit differently
// so it works for the q = 0 case.

#define cmsub41_p256(P0,P1,P2)                  \
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
        lsr     x4, x4, #62 __LF                   \
        sbc     x4, x4, xzr __LF                   \
        add     x5, x4, #1 __LF                    \
        lsl     x8, x5, #32 __LF                   \
        subs    x6, xzr, x8 __LF                   \
        sbcs    x7, xzr, xzr __LF                  \
        sbc     x8, x8, x5 __LF                    \
        adds    x0, x0, x5 __LF                    \
        adcs    x1, x1, x6 __LF                    \
        adcs    x2, x2, x7 __LF                    \
        adcs    x3, x3, x8 __LF                    \
        csetm   x5, cc __LF                        \
        adds    x0, x0, x5 __LF                    \
        and     x6, x5, #4294967295 __LF           \
        adcs    x1, x1, x6 __LF                    \
        adcs    x2, x2, xzr __LF                   \
        neg     x7, x6 __LF                        \
        adc     x3, x3, x7 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

// P0 = 3 * P1 - 8 * P2, computed as (p_256 - P2) << 3 + 3 * P1

#define cmsub38_p256(P0,P1,P2)                  \
        mov     x1, 8 __LF                         \
        mov     x2, #-1 __LF                       \
        ldp     x9, x10, [P2] __LF                 \
        subs    x9, x2, x9 __LF                    \
        mov     x2, #4294967295 __LF               \
        sbcs    x10, x2, x10 __LF                  \
        ldp     x11, x12, [P2+16] __LF             \
        sbcs    x11, xzr, x11 __LF                 \
        mov     x2, #-4294967295 __LF              \
        sbc     x12, x2, x12 __LF                  \
        lsl     x3, x9, #3 __LF                    \
        extr    x4, x10, x9, #61 __LF              \
        extr    x5, x11, x10, #61 __LF             \
        extr    x6, x12, x11, #61 __LF             \
        lsr     x7, x12, #61 __LF                  \
        mov     x1, 3 __LF                         \
        ldp     x9, x10, [P1] __LF                 \
        mul     x8, x9, x1 __LF                    \
        umulh   x9, x9, x1 __LF                    \
        adds    x3, x3, x8 __LF                    \
        mul     x8, x10, x1 __LF                   \
        umulh   x10, x10, x1 __LF                  \
        adcs    x4, x4, x8 __LF                    \
        ldp     x11, x12, [P1+16] __LF             \
        mul     x8, x11, x1 __LF                   \
        umulh   x11, x11, x1 __LF                  \
        adcs    x5, x5, x8 __LF                    \
        mul     x8, x12, x1 __LF                   \
        umulh   x12, x12, x1 __LF                  \
        adcs    x6, x6, x8 __LF                    \
        adc     x7, x7, xzr __LF                   \
        adds    x4, x4, x9 __LF                    \
        adcs    x5, x5, x10 __LF                   \
        adcs    x6, x6, x11 __LF                   \
        adc     x7, x7, x12 __LF                   \
        add     x8, x7, #1 __LF                    \
        lsl     x10, x8, #32 __LF                  \
        adds    x6, x6, x10 __LF                   \
        adc     x7, x7, xzr __LF                   \
        neg     x9, x8 __LF                        \
        sub     x10, x10, #1 __LF                  \
        subs    x3, x3, x9 __LF                    \
        sbcs    x4, x4, x10 __LF                   \
        sbcs    x5, x5, xzr __LF                   \
        sbcs    x6, x6, x8 __LF                    \
        sbc     x8, x7, x8 __LF                    \
        adds    x3, x3, x8 __LF                    \
        and     x9, x8, #4294967295 __LF           \
        adcs    x4, x4, x9 __LF                    \
        adcs    x5, x5, xzr __LF                   \
        neg     x10, x9 __LF                       \
        adc     x6, x6, x10 __LF                   \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16]

S2N_BN_SYMBOL(p256_montjdouble_alt):
        CFI_START

// Make room on stack for temporary variables

        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1

// Main code, just a sequence of basic field operations

// z2 = z^2
// y2 = y^2

        montsqr_p256(z2,z_1)
        montsqr_p256(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        sub_p256(t2,x_1,z2)
        weakadd_p256(t1,x_1,z2)
        montmul_p256(x2p,t1,t2)

// t1 = y + z
// xy2 = x * y^2
// x4p = x2p^2

        add_p256(t1,y_1,z_1)
        montmul_p256(xy2,x_1,y2)
        montsqr_p256(x4p,x2p)

// t1 = (y + z)^2

        montsqr_p256(t1,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p256(d,12,xy2,9,x4p)
        sub_p256(t1,t1,z2)

// y4 = y^4

        montsqr_p256(y4,y2)

// dx2 = d * x2p

        montmul_p256(dx2,d,x2p)

// z_3' = 2 * y * z

        sub_p256(z_3,t1,y2)

// x' = 4 * xy2 - d

        cmsub41_p256(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p256(y_3,dx2,y4)

// Restore stack and return

        CFI_INC_SP(NSPACE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_montjdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
