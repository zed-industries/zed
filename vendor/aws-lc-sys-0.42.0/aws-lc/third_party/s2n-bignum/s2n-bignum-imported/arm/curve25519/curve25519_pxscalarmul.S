// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Projective scalar multiplication, x coordinate only, for curve25519
// Inputs scalar[4], point[4]; output res[8]
//
// extern void curve25519_pxscalarmul
//   (uint64_t res[static 8],const uint64_t scalar[static 4],
//    const uint64_t point[static 4]);
//
// Given the X coordinate of an input point = (X,Y) on curve25519, which
// could also be part of a projective representation (X,Y,1) of the same
// point, returns a projective representation (X,Z) = scalar * point, where
// scalar is a 256-bit number. The corresponding affine form is (X/Z,Y'),
// X/Z meaning division modulo 2^255-19, and Y' not being computed by
// this function (nor is any Y coordinate of the input point used).
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_pxscalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_pxscalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_pxscalarmul)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence
// and additional registers for loop counter and swap flag

#define res x17
#define point x19
#define scalar x20
#define i x21
#define swap x22

// Pointers to input x coord (we don't use y or z) and output coords.

#define x point, #0
#define resx res, #0
#define resz res, #NUMSIZE

// Pointer-offset pairs for temporaries on stack with some aliasing.

#define zm sp, #(0*NUMSIZE)
#define sm sp, #(0*NUMSIZE)
#define dpro sp, #(0*NUMSIZE)

#define sn sp, #(1*NUMSIZE)

#define dm sp, #(2*NUMSIZE)

#define zn sp, #(3*NUMSIZE)
#define dn sp, #(3*NUMSIZE)
#define e sp, #(3*NUMSIZE)

#define dmsn sp, #(4*NUMSIZE)
#define p sp, #(4*NUMSIZE)

#define xm sp, #(5*NUMSIZE)
#define dnsm sp, #(5*NUMSIZE)
#define spro sp, #(5*NUMSIZE)

#define xn sp, #(6*NUMSIZE)
#define s sp, #(6*NUMSIZE)

#define d sp, #(7*NUMSIZE)

// Total size to reserve on the stack

#define NSPACE 8*NUMSIZE

// Macros wrapping up the basic field operations bignum_mul_p25519
// and bignum_sqr_p25519, only trivially different from pure function
// call to those subroutines.

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

#define sqr_p25519(P0,P1)                       \
        ldp     x10, x11, [P1] __LF                \
        ldp     x12, x13, [P1+16] __LF             \
        umull   x2, w10, w10 __LF                  \
        lsr     x14, x10, #32 __LF                 \
        umull   x3, w14, w14 __LF                  \
        umull   x14, w10, w14 __LF                 \
        adds    x2, x2, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x3, x3, x14 __LF                   \
        umull   x4, w11, w11 __LF                  \
        lsr     x14, x11, #32 __LF                 \
        umull   x5, w14, w14 __LF                  \
        umull   x14, w11, w14 __LF                 \
        mul     x15, x10, x11 __LF                 \
        umulh   x16, x10, x11 __LF                 \
        adds    x4, x4, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x5, x5, x14 __LF                   \
        adds    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adc     x5, x5, xzr __LF                   \
        adds    x3, x3, x15 __LF                   \
        adcs    x4, x4, x16 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umull   x6, w12, w12 __LF                  \
        lsr     x14, x12, #32 __LF                 \
        umull   x7, w14, w14 __LF                  \
        umull   x14, w12, w14 __LF                 \
        adds    x6, x6, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x7, x7, x14 __LF                   \
        umull   x8, w13, w13 __LF                  \
        lsr     x14, x13, #32 __LF                 \
        umull   x9, w14, w14 __LF                  \
        umull   x14, w13, w14 __LF                 \
        mul     x15, x12, x13 __LF                 \
        umulh   x16, x12, x13 __LF                 \
        adds    x8, x8, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x9, x9, x14 __LF                   \
        adds    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adc     x9, x9, xzr __LF                   \
        adds    x7, x7, x15 __LF                   \
        adcs    x8, x8, x16 __LF                   \
        adc     x9, x9, xzr __LF                   \
        subs    x10, x10, x12 __LF                 \
        sbcs    x11, x11, x13 __LF                 \
        csetm   x16, cc __LF                       \
        eor     x10, x10, x16 __LF                 \
        subs    x10, x10, x16 __LF                 \
        eor     x11, x11, x16 __LF                 \
        sbc     x11, x11, x16 __LF                 \
        adds    x6, x6, x4 __LF                    \
        adcs    x7, x7, x5 __LF                    \
        adcs    x8, x8, xzr __LF                   \
        adc     x9, x9, xzr __LF                   \
        umull   x12, w10, w10 __LF                 \
        lsr     x5, x10, #32 __LF                  \
        umull   x13, w5, w5 __LF                   \
        umull   x5, w10, w5 __LF                   \
        adds    x12, x12, x5, lsl #33 __LF         \
        lsr     x5, x5, #31 __LF                   \
        adc     x13, x13, x5 __LF                  \
        umull   x15, w11, w11 __LF                 \
        lsr     x5, x11, #32 __LF                  \
        umull   x14, w5, w5 __LF                   \
        umull   x5, w11, w5 __LF                   \
        mul     x4, x10, x11 __LF                  \
        umulh   x16, x10, x11 __LF                 \
        adds    x15, x15, x5, lsl #33 __LF         \
        lsr     x5, x5, #31 __LF                   \
        adc     x14, x14, x5 __LF                  \
        adds    x4, x4, x4 __LF                    \
        adcs    x16, x16, x16 __LF                 \
        adc     x14, x14, xzr __LF                 \
        adds    x13, x13, x4 __LF                  \
        adcs    x15, x15, x16 __LF                 \
        adc     x14, x14, xzr __LF                 \
        adds    x4, x2, x6 __LF                    \
        adcs    x5, x3, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        adcs    x7, x7, x9 __LF                    \
        csetm   x16, cc __LF                       \
        subs    x4, x4, x12 __LF                   \
        sbcs    x5, x5, x13 __LF                   \
        sbcs    x6, x6, x15 __LF                   \
        sbcs    x7, x7, x14 __LF                   \
        adcs    x8, x8, x16 __LF                   \
        adc     x9, x9, x16 __LF                   \
        mov     x10, #0x26 __LF                    \
        umull   x12, w6, w10 __LF                  \
        add     x12, x12, w2, uxtw __LF            \
        lsr     x2, x2, #32 __LF                   \
        lsr     x6, x6, #32 __LF                   \
        umaddl  x6, w6, w10, x2 __LF               \
        mov     x2, x12 __LF                       \
        umull   x12, w7, w10 __LF                  \
        add     x12, x12, w3, uxtw __LF            \
        lsr     x3, x3, #32 __LF                   \
        lsr     x7, x7, #32 __LF                   \
        umaddl  x7, w7, w10, x3 __LF               \
        mov     x3, x12 __LF                       \
        umull   x12, w8, w10 __LF                  \
        add     x12, x12, w4, uxtw __LF            \
        lsr     x4, x4, #32 __LF                   \
        lsr     x8, x8, #32 __LF                   \
        umaddl  x8, w8, w10, x4 __LF               \
        mov     x4, x12 __LF                       \
        umull   x12, w9, w10 __LF                  \
        add     x12, x12, w5, uxtw __LF            \
        lsr     x5, x5, #32 __LF                   \
        lsr     x9, x9, #32 __LF                   \
        umaddl  x9, w9, w10, x5 __LF               \
        mov     x5, x12 __LF                       \
        lsr     x13, x9, #31 __LF                  \
        mov     x11, #0x13 __LF                    \
        umaddl  x11, w11, w13, x11 __LF            \
        add     x2, x2, x11 __LF                   \
        adds    x2, x2, x6, lsl #32 __LF           \
        extr    x10, x7, x6, #32 __LF              \
        adcs    x3, x3, x10 __LF                   \
        extr    x10, x8, x7, #32 __LF              \
        adcs    x4, x4, x10 __LF                   \
        extr    x10, x9, x8, #32 __LF              \
        lsl     x11, x13, #63 __LF                 \
        eor     x5, x5, x11 __LF                   \
        adc     x5, x5, x10 __LF                   \
        mov     x10, #0x13 __LF                    \
        tst     x5, #0x8000000000000000 __LF       \
        csel    x10, x10, xzr, pl __LF             \
        subs    x2, x2, x10 __LF                   \
        sbcs    x3, x3, xzr __LF                   \
        sbcs    x4, x4, xzr __LF                   \
        sbc     x5, x5, xzr __LF                   \
        and     x5, x5, #0x7fffffffffffffff __LF   \
        stp     x2, x3, [P0] __LF                  \
        stp     x4, x5, [P0+16]

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

// Squaring just giving a result < 2 * p_25519, which is done by
// basically skipping the +1 in the quotient estimate and the final
// optional correction.

#define sqr_4(P0,P1)                            \
        ldp     x10, x11, [P1] __LF                \
        ldp     x12, x13, [P1+16] __LF             \
        umull   x2, w10, w10 __LF                  \
        lsr     x14, x10, #32 __LF                 \
        umull   x3, w14, w14 __LF                  \
        umull   x14, w10, w14 __LF                 \
        adds    x2, x2, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x3, x3, x14 __LF                   \
        umull   x4, w11, w11 __LF                  \
        lsr     x14, x11, #32 __LF                 \
        umull   x5, w14, w14 __LF                  \
        umull   x14, w11, w14 __LF                 \
        mul     x15, x10, x11 __LF                 \
        umulh   x16, x10, x11 __LF                 \
        adds    x4, x4, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x5, x5, x14 __LF                   \
        adds    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adc     x5, x5, xzr __LF                   \
        adds    x3, x3, x15 __LF                   \
        adcs    x4, x4, x16 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umull   x6, w12, w12 __LF                  \
        lsr     x14, x12, #32 __LF                 \
        umull   x7, w14, w14 __LF                  \
        umull   x14, w12, w14 __LF                 \
        adds    x6, x6, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x7, x7, x14 __LF                   \
        umull   x8, w13, w13 __LF                  \
        lsr     x14, x13, #32 __LF                 \
        umull   x9, w14, w14 __LF                  \
        umull   x14, w13, w14 __LF                 \
        mul     x15, x12, x13 __LF                 \
        umulh   x16, x12, x13 __LF                 \
        adds    x8, x8, x14, lsl #33 __LF          \
        lsr     x14, x14, #31 __LF                 \
        adc     x9, x9, x14 __LF                   \
        adds    x15, x15, x15 __LF                 \
        adcs    x16, x16, x16 __LF                 \
        adc     x9, x9, xzr __LF                   \
        adds    x7, x7, x15 __LF                   \
        adcs    x8, x8, x16 __LF                   \
        adc     x9, x9, xzr __LF                   \
        subs    x10, x10, x12 __LF                 \
        sbcs    x11, x11, x13 __LF                 \
        csetm   x16, cc __LF                       \
        eor     x10, x10, x16 __LF                 \
        subs    x10, x10, x16 __LF                 \
        eor     x11, x11, x16 __LF                 \
        sbc     x11, x11, x16 __LF                 \
        adds    x6, x6, x4 __LF                    \
        adcs    x7, x7, x5 __LF                    \
        adcs    x8, x8, xzr __LF                   \
        adc     x9, x9, xzr __LF                   \
        umull   x12, w10, w10 __LF                 \
        lsr     x5, x10, #32 __LF                  \
        umull   x13, w5, w5 __LF                   \
        umull   x5, w10, w5 __LF                   \
        adds    x12, x12, x5, lsl #33 __LF         \
        lsr     x5, x5, #31 __LF                   \
        adc     x13, x13, x5 __LF                  \
        umull   x15, w11, w11 __LF                 \
        lsr     x5, x11, #32 __LF                  \
        umull   x14, w5, w5 __LF                   \
        umull   x5, w11, w5 __LF                   \
        mul     x4, x10, x11 __LF                  \
        umulh   x16, x10, x11 __LF                 \
        adds    x15, x15, x5, lsl #33 __LF         \
        lsr     x5, x5, #31 __LF                   \
        adc     x14, x14, x5 __LF                  \
        adds    x4, x4, x4 __LF                    \
        adcs    x16, x16, x16 __LF                 \
        adc     x14, x14, xzr __LF                 \
        adds    x13, x13, x4 __LF                  \
        adcs    x15, x15, x16 __LF                 \
        adc     x14, x14, xzr __LF                 \
        adds    x4, x2, x6 __LF                    \
        adcs    x5, x3, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        adcs    x7, x7, x9 __LF                    \
        csetm   x16, cc __LF                       \
        subs    x4, x4, x12 __LF                   \
        sbcs    x5, x5, x13 __LF                   \
        sbcs    x6, x6, x15 __LF                   \
        sbcs    x7, x7, x14 __LF                   \
        adcs    x8, x8, x16 __LF                   \
        adc     x9, x9, x16 __LF                   \
        mov     x10, #0x26 __LF                    \
        umull   x12, w6, w10 __LF                  \
        add     x12, x12, w2, uxtw __LF            \
        lsr     x2, x2, #32 __LF                   \
        lsr     x6, x6, #32 __LF                   \
        umaddl  x6, w6, w10, x2 __LF               \
        mov     x2, x12 __LF                       \
        umull   x12, w7, w10 __LF                  \
        add     x12, x12, w3, uxtw __LF            \
        lsr     x3, x3, #32 __LF                   \
        lsr     x7, x7, #32 __LF                   \
        umaddl  x7, w7, w10, x3 __LF               \
        mov     x3, x12 __LF                       \
        umull   x12, w8, w10 __LF                  \
        add     x12, x12, w4, uxtw __LF            \
        lsr     x4, x4, #32 __LF                   \
        lsr     x8, x8, #32 __LF                   \
        umaddl  x8, w8, w10, x4 __LF               \
        mov     x4, x12 __LF                       \
        umull   x12, w9, w10 __LF                  \
        add     x12, x12, w5, uxtw __LF            \
        lsr     x5, x5, #32 __LF                   \
        lsr     x9, x9, #32 __LF                   \
        umaddl  x9, w9, w10, x5 __LF               \
        mov     x5, x12 __LF                       \
        lsr     x13, x9, #31 __LF                  \
        mov     x11, #0x13 __LF                    \
        umull   x11, w11, w13 __LF                 \
        add     x2, x2, x11 __LF                   \
        adds    x2, x2, x6, lsl #32 __LF           \
        extr    x10, x7, x6, #32 __LF              \
        adcs    x3, x3, x10 __LF                   \
        extr    x10, x8, x7, #32 __LF              \
        adcs    x4, x4, x10 __LF                   \
        extr    x10, x9, x8, #32 __LF              \
        lsl     x11, x13, #63 __LF                 \
        eor     x5, x5, x11 __LF                   \
        adc     x5, x5, x10 __LF                   \
        stp     x2, x3, [P0] __LF                  \
        stp     x4, x5, [P0+16]

// Plain 4-digit add without any normalization
// With inputs < p_25519 (indeed < 2^255) it still gives a 4-digit result

#define add_4(p0,p1,p2)                         \
        ldp     x0, x1, [p1] __LF                  \
        ldp     x4, x5, [p2] __LF                  \
        adds    x0, x0, x4 __LF                    \
        adcs    x1, x1, x5 __LF                    \
        ldp     x2, x3, [p1+16] __LF               \
        ldp     x6, x7, [p2+16] __LF               \
        adcs    x2, x2, x6 __LF                    \
        adc     x3, x3, x7 __LF                    \
        stp     x0, x1, [p0] __LF                  \
        stp     x2, x3, [p0+16]

// Subtraction of a pair of numbers < p_25519 just sufficient
// to give a 4-digit result. It actually always does (x - z) + (2^255-19)
// which in turn is done by (x - z) - (2^255+19) discarding the 2^256
// implicitly

#define sub_4(p0,p1,p2)                         \
        ldp     x5, x6, [p1] __LF                  \
        ldp     x4, x3, [p2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [p1+16] __LF               \
        ldp     x4, x3, [p2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x3, #19 __LF                       \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        mov     x4, #0x8000000000000000 __LF       \
        sbc     x8, x8, x4 __LF                    \
        stp     x5, x6, [p0] __LF                  \
        stp     x7, x8, [p0+16]

// Modular addition with double modulus 2 * p_25519 = 2^256 - 38.
// This only ensures that the result fits in 4 digits, not that it is reduced
// even w.r.t. double modulus. The result is always correct modulo provided
// the sum of the inputs is < 2^256 + 2^256 - 38, so in particular provided
// at least one of them is reduced double modulo.

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

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(p0,p1,p2)                    \
        ldp     x5, x6, [p1] __LF                  \
        ldp     x4, x3, [p2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [p1+16] __LF               \
        ldp     x4, x3, [p2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #38 __LF                       \
        csel    x3, x4, xzr, lo __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [p0] __LF                  \
        stp     x7, x8, [p0+16]

// Combined z = c * x + y with reduction only < 2 * p_25519
// where c is initially in the X1 register. It is assumed
// that 19 * (c * x + y) < 2^60 * 2^256 so we don't need a
// high mul in the final part.

#define cmadd_4(p0,p2,p3)                       \
        ldp     x7, x8, [p2] __LF                  \
        ldp     x9, x10, [p2+16] __LF              \
        mul     x3, x1, x7 __LF                    \
        mul     x4, x1, x8 __LF                    \
        mul     x5, x1, x9 __LF                    \
        mul     x6, x1, x10 __LF                   \
        umulh   x7, x1, x7 __LF                    \
        umulh   x8, x1, x8 __LF                    \
        umulh   x9, x1, x9 __LF                    \
        umulh   x10, x1, x10 __LF                  \
        adds    x4, x4, x7 __LF                    \
        adcs    x5, x5, x8 __LF                    \
        adcs    x6, x6, x9 __LF                    \
        adc     x10, x10, xzr __LF                 \
        ldp     x7, x8, [p3] __LF                  \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, x8 __LF                    \
        ldp     x7, x8, [p3+16] __LF               \
        adcs    x5, x5, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        adc     x10, x10, xzr __LF                 \
        cmn     x6, x6 __LF                        \
        bic     x6, x6, #0x8000000000000000 __LF   \
        adc     x8, x10, x10 __LF                  \
        mov     x9, #19 __LF                       \
        mul     x7, x8, x9 __LF                    \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adc     x6, x6, xzr __LF                   \
        stp     x3, x4, [p0] __LF                  \
        stp     x5, x6, [p0+16]

// Multiplex: z := if NZ then x else y

#define mux_4(p0,p1,p2)                         \
        ldp     x0, x1, [p1] __LF                  \
        ldp     x2, x3, [p2] __LF                  \
        csel    x0, x0, x2, ne __LF                \
        csel    x1, x1, x3, ne __LF                \
        stp     x0, x1, [p0] __LF                  \
        ldp     x0, x1, [p1+16] __LF               \
        ldp     x2, x3, [p2+16] __LF               \
        csel    x0, x0, x2, ne __LF                \
        csel    x1, x1, x3, ne __LF                \
        stp     x0, x1, [p0+16]

S2N_BN_SYMBOL(curve25519_pxscalarmul):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x22)
        CFI_PUSH2(x20,x21)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     res, x0
        mov     scalar, x1
        mov     point, x2

// Initialize (xn,zn) = (1,0) and (xm,zm) = (x,1) with swap = 0

        mov     x2, #1
        stp     x2, xzr, [xn]
        stp     xzr, xzr, [xn+16]
        stp     xzr, xzr, [zn]
        stp     xzr, xzr, [zn+16]
        ldp     x0, x1, [x]
        stp     x0, x1, [xm]
        ldp     x0, x1, [x+16]
        stp     x0, x1, [xm+16]
        ldp     x0, x1, [x+32]
        stp     x2, xzr, [zm]
        stp     xzr, xzr, [zm+16]
        mov     swap, xzr

// The outer loop from i = 255, ..., i = 0 (inclusive)

        mov     i, #255

Lcurve25519_pxscalarmul_loop:

// sm = xm + zm; sn = xn + zn; dm = xm - zm; dn = xn - zn
// The adds don't need any normalization as they're fed to muls
// Just make sure the subs fit in 4 digits

        sub_4(dm, xm, zm)
        add_4(sn, xn, zn)
        sub_4(dn, xn, zn)
        add_4(sm, xm, zm)

// ADDING: dmsn = dm * sn; dnsm = sm * dn
// DOUBLING: mux d = xt - zt and s = xt + zt for appropriate choice of (xt,zt)

        mul_4(dmsn,sn,dm)

        lsr     x0, i, #6
        ldr     x2, [scalar, x0, lsl #3]
        lsr     x2, x2, i
        and     x2, x2, #1

        cmp     swap, x2
        mov     swap, x2

        mux_4(d,dm,dn)
        mux_4(s,sm,sn)

        mul_4(dnsm,sm,dn)

// DOUBLING: d = (xt - zt)^2 normalized only to 4 digits

        sqr_4(d,d)

// ADDING: dpro = (dmsn - dnsm)^2, spro = (dmsn + dnsm)^2
// DOUBLING: s = (xt + zt)^2, normalized only to 4 digits

        sub_twice4(dpro,dmsn,dnsm)
        sqr_4(s,s)
        add_twice4(spro,dmsn,dnsm)
        sqr_4(dpro,dpro)

// DOUBLING: p = 4 * xt * zt = s - d

        sub_twice4(p,s,d)

// ADDING: xm' = (dmsn + dnsm)^2

        sqr_p25519(xm,spro)

// DOUBLING: e = 121666 * p + d

        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)

// DOUBLING: xn' = (xt + zt)^2 * (xt - zt)^2 = s * d

        mul_p25519(xn,s,d)

// ADDING: zm' = x * (dmsn - dnsm)^2

        mul_p25519(zm,dpro,x)

// DOUBLING: zn' = (4 * xt * zt) * ((xt - zt)^2 + 121666 * (4 * xt * zt))
//               = p * (d + 121666 * p)

        mul_p25519(zn,p,e)

// Loop down as far as 0 (inclusive)

        subs    i, i, #1
        bcs     Lcurve25519_pxscalarmul_loop

// The main loop does not handle the special input of the 2-torsion
// point = (0,0). In that case we may get a spurious (0,0) as output
// when we want (0,1) [for odd scalar] or (1,0) [for even scalar].
// Test if x = 0 (this is equivalent for curve25519 to y = 0) and if
// so, patch zm = 1 [for odd multiple], xn = 1 [for even multiple].

        ldp     x0, x1, [point]
        orr     x0, x0, x1
        ldp     x2, x3, [point, #16]
        orr     x2, x2, x3
        orr     x0, x0, x2
        cmp     x0, xzr
        cset    x0, eq
        ldr     x1, [zm]
        orr     x1, x1, x0
        str     x1, [zm]
        ldr     x2, [xn]
        orr     x2, x2, x0
        str     x2, [xn]

// Multiplex into the final outputs

        cmp     swap, xzr

        mux_4(resx,xm,xn)
        mux_4(resz,zm,zn)

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x20,x21)
        CFI_POP2(x19,x22)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(curve25519_pxscalarmul)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
