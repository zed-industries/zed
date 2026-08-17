// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double scalar multiplication for edwards25519, fresh and base point
// Input scalar[4], point[8], bscalar[4]; output res[8]
//
// extern void edwards25519_scalarmuldouble
//   (uint64_t res[static 8],const uint64_t scalar[static 4],
//    const uint64_t point[static 8],const uint64_t bscalar[static 4]);
//
// Given scalar = n, point = P and bscalar = m, returns in res
// the point (X,Y) = n * P + m * B where B = (...,4/5) is
// the standard basepoint for the edwards25519 (Ed25519) curve.
//
// Both 256-bit coordinates of the input point P are implicitly
// reduced modulo 2^255-19 if they are not already in reduced form,
// but the conventional usage is that they *are* already reduced.
// The scalars can be arbitrary 256-bit numbers but may also be
// considered as implicitly reduced modulo the group order.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point, X3 = bscalar
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_scalarmuldouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_scalarmuldouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_scalarmuldouble)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable home for the input result argument during the whole body

#define res x25

// Additional pointer variables for local subroutines

#define p0 x22
#define p1 x23
#define p2 x24

// Other variables that are only needed prior to the modular inverse.

#define i x19
#define bf x20
#define cf x21

// Pointer-offset pairs for result and temporaries on stack with some aliasing.

#define resx res, #(0*NUMSIZE)
#define resy res, #(1*NUMSIZE)

#define scalar sp, #(0*NUMSIZE)
#define bscalar sp, #(1*NUMSIZE)

#define btabent sp, #(2*NUMSIZE)
#define acc sp, #(5*NUMSIZE)
#define acc_x sp, #(5*NUMSIZE)
#define acc_y sp, #(6*NUMSIZE)
#define acc_z sp, #(7*NUMSIZE)
#define acc_w sp, #(8*NUMSIZE)

#define tabent sp, #(9*NUMSIZE)

#define tab sp, #(13*NUMSIZE)

// Total size to reserve on the stack (excluding local subroutines)

#define NSPACE 45*NUMSIZE

// Sub-references used in local subroutines with local stack

#define x_0 p0, #0
#define y_0 p0, #NUMSIZE
#define z_0 p0, #(2*NUMSIZE)
#define w_0 p0, #(3*NUMSIZE)

#define x_1 p1, #0
#define y_1 p1, #NUMSIZE
#define z_1 p1, #(2*NUMSIZE)
#define w_1 p1, #(3*NUMSIZE)

#define x_2 p2, #0
#define y_2 p2, #NUMSIZE
#define z_2 p2, #(2*NUMSIZE)
#define w_2 p2, #(3*NUMSIZE)

#define t0 sp, #(0*NUMSIZE)
#define t1 sp, #(1*NUMSIZE)
#define t2 sp, #(2*NUMSIZE)
#define t3 sp, #(3*NUMSIZE)
#define t4 sp, #(4*NUMSIZE)
#define t5 sp, #(5*NUMSIZE)

// Load 64-bit immediate into a register

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

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

// Modular addition and doubling with double modulus 2 * p_25519 = 2^256 - 38.
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

#define double_twice4(P0,P1)                    \
        ldp     x3, x4, [P1] __LF                  \
        adds    x3, x3, x3 __LF                    \
        adcs    x4, x4, x4 __LF                    \
        ldp     x5, x6, [P1+16] __LF               \
        adcs    x5, x5, x5 __LF                    \
        adcs    x6, x6, x6 __LF                    \
        mov     x9, #38 __LF                       \
        csel    x9, x9, xzr, cs __LF               \
        adds    x3, x3, x9 __LF                    \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adc     x6, x6, xzr __LF                   \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16]

// Load the constant k_25519 = 2 * d_25519 using immediate operations

#define load_k25519(P0)                         \
        movz    x0, #0xf159 __LF                   \
        movz    x1, #0xb156 __LF                   \
        movz    x2, #0xd130 __LF                   \
        movz    x3, #0xfce7 __LF                   \
        movk    x0, #0x26b2, lsl #16 __LF          \
        movk    x1, #0x8283, lsl #16 __LF          \
        movk    x2, #0xeef3, lsl #16 __LF          \
        movk    x3, #0x56df, lsl #16 __LF          \
        movk    x0, #0x9b94, lsl #32 __LF          \
        movk    x1, #0x149a, lsl #32 __LF          \
        movk    x2, #0x80f2, lsl #32 __LF          \
        movk    x3, #0xd9dc, lsl #32 __LF          \
        movk    x0, #0xebd6, lsl #48 __LF          \
        movk    x1, #0x00e0, lsl #48 __LF          \
        movk    x2, #0x198e, lsl #48 __LF          \
        movk    x3, #0x2406, lsl #48 __LF          \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

S2N_BN_SYMBOL(edwards25519_scalarmuldouble):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x30)
        CFI_DEC_SP(NSPACE)

// Move the output pointer to a stable place

        mov     res, x0

// Copy scalars while recoding all 4-bit nybbles except the top
// one (bits 252..255) into signed 4-bit digits. This is essentially
// done just by adding the recoding constant 0x0888..888, after
// which all digits except the first have an implicit bias of -8,
// so 0 -> -8, 1 -> -7, ... 7 -> -1, 8 -> 0, 9 -> 1, ... 15 -> 7.
// (We could literally create 2s complement signed nybbles by
// XORing with the same constant 0x0888..888 afterwards, but it
// doesn't seem to make the end usage any simpler.)
//
// In order to ensure that the unrecoded top nybble (bits 252..255)
// does not become > 8 as a result of carries lower down from the
// recoding, we first (conceptually) subtract the group order iff
// the top digit of the scalar is > 2^63. In the implementation the
// reduction and recoding are combined by optionally using the
// modified recoding constant 0x0888...888 + (2^256 - group_order).

        movbig(x4,#0xc7f5, #0x6fb5, #0xa0d9, #0xe920)
        movbig(x5,#0xe190, #0xb993, #0x70cb, #0xa1d5)
        mov     x7, #0x8888888888888888
        sub     x6, x7, #1
        bic     x8, x7, #0xF000000000000000

        ldp     x10, x11, [x3]
        ldp     x12, x13, [x3, #16]
        mov     x3, 0x8000000000000000
        cmp     x3, x13
        csel    x14, x7, x4, cs
        csel    x15, x7, x5, cs
        csel    x16, x7, x6, cs
        csel    x17, x8, x7, cs
        adds    x10, x10, x14
        adcs    x11, x11, x15
        adcs    x12, x12, x16
        adc     x13, x13, x17
        stp     x10, x11, [bscalar]
        stp     x12, x13, [bscalar+16]

        ldp     x10, x11, [x1]
        ldp     x12, x13, [x1, #16]
        mov     x3, 0x8000000000000000
        cmp     x3, x13
        csel    x14, x7, x4, cs
        csel    x15, x7, x5, cs
        csel    x16, x7, x6, cs
        csel    x17, x8, x7, cs
        adds    x10, x10, x14
        adcs    x11, x11, x15
        adcs    x12, x12, x16
        adc     x13, x13, x17
        stp     x10, x11, [scalar]
        stp     x12, x13, [scalar+16]

// Create table of multiples 1..8 of the general input point at "tab".
// Reduce the input coordinates x and y modulo 2^256 - 38 first, for the
// sake of definiteness; this is the reduction that will be maintained.
// We could slightly optimize the additions because we know the input
// point is affine (so Z = 1), but it doesn't seem worth the complication.

        ldp     x10, x11, [x2]
        ldp     x12, x13, [x2, #16]
        adds    x14, x10, #38
        adcs    x15, x11, xzr
        adcs    x16, x12, xzr
        adcs    x17, x13, xzr
        csel    x10, x14, x10, cs
        csel    x11, x15, x11, cs
        csel    x12, x16, x12, cs
        csel    x13, x17, x13, cs
        stp     x10, x11, [tab]
        stp     x12, x13, [tab+16]

        ldp     x10, x11, [x2, #32]
        ldp     x12, x13, [x2, #48]
        adds    x14, x10, #38
        adcs    x15, x11, xzr
        adcs    x16, x12, xzr
        adcs    x17, x13, xzr
        csel    x10, x14, x10, cs
        csel    x11, x15, x11, cs
        csel    x12, x16, x12, cs
        csel    x13, x17, x13, cs
        stp     x10, x11, [tab+32]
        stp     x12, x13, [tab+48]

        mov     x1, #1
        stp     x1, xzr, [tab+64]
        stp     xzr, xzr, [tab+80]

        add     p0, tab+96
        add     p1, tab
        add     p2, tab+32
        mul_4(x_0,x_1,x_2)

// Multiple 2

        add     p0, tab+1*128
        add     p1, tab
        CFI_BL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 3

        add     p0, tab+2*128
        add     p1, tab
        add     p2, tab+1*128
        CFI_BL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 4

        add     p0, tab+3*128
        add     p1, tab+1*128
        CFI_BL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 5

        add     p0, tab+4*128
        add     p1, tab
        add     p2, tab+3*128
        CFI_BL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 6

        add     p0, tab+5*128
        add     p1, tab+2*128
        CFI_BL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 7

        add     p0, tab+6*128
        add     p1, tab
        add     p2, tab+5*128
        CFI_BL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 8

        add     p0, tab+7*128
        add     p1, tab+3*128
        CFI_BL(Ledwards25519_scalarmuldouble_epdouble)

// Handle the initialization, starting the loop counter at i = 252
// and initializing acc to the sum of the table entries for the
// top nybbles of the scalars (the ones with no implicit -8 bias).

        mov     i, #252

// Index for btable entry...

        ldr     x0, [bscalar+24]
        lsr     bf, x0, #60

// ...and constant-time indexing based on that index

#if defined(__ELF__)
        adrp    x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)
        add     x14, x14, :lo12:S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)
#else
        adrp    x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)@PAGE
        add     x14, x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)@PAGEOFF
#endif

        mov     x0, #1
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, #1
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr

        cmp     bf, #1
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #2
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #3
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #4
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #5
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #6
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #7
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #8
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne

        stp     x0, x1, [btabent]
        stp     x2, x3, [btabent+16]
        stp     x4, x5, [btabent+32]
        stp     x6, x7, [btabent+48]
        stp     x8, x9, [btabent+64]
        stp     x10, x11, [btabent+80]

// Index for table entry...

        ldr     x0, [scalar+24]
        lsr     bf, x0, #60

// ...and constant-time indexing based on that index

        add     p0, tab

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, #1
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, #1
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        mov     x12, xzr
        mov     x13, xzr
        mov     x14, xzr
        mov     x15, xzr

        cmp     bf, #1
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #2
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #3
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #4
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #5
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #6
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #7
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #8
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]
        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]
        stp     x8, x9, [tabent+64]
        stp     x10, x11, [tabent+80]
        stp     x12, x13, [tabent+96]
        stp     x14, x15, [tabent+112]

// Add those elements to initialize the accumulator for bit position 252

        add     p0, acc
        add     p1, tabent
        add     p2, btabent
        CFI_BL(Ledwards25519_scalarmuldouble_pepadd)

// Main loop with acc = [scalar/2^i] * point + [bscalar/2^i] * basepoint
// Start with i = 252 for bits 248..251 and go down four at a time to 3..0

Ledwards25519_scalarmuldouble_loop:

        sub     i, i, #4

// Double to acc' = 2 * acc

        add     p0, acc
        add     p1, acc
        CFI_BL(Ledwards25519_scalarmuldouble_pdouble)

// Get btable entry, first getting the adjusted bitfield...

        lsr     x0, i, #6
        add     x1, bscalar
        ldr     x2, [x1, x0, lsl #3]
        lsr     x3, x2, i
        and     x0, x3, #15
        subs    bf, x0, #8
        cneg    bf, bf, cc
        csetm   cf, cc

// ... then doing constant-time lookup with the appropriate index...

#if defined(__ELF__)
        adrp    x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)
        add     x14, x14, :lo12:S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)
#else
        adrp    x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)@PAGE
        add     x14, x14, S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant)@PAGEOFF
#endif

        mov     x0, #1
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, #1
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr

        cmp     bf, #1
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #2
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #3
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #4
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #5
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #6
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #7
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        add     x14, x14, #96

        cmp     bf, #8
        ldp     x12, x13, [x14]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x12, x13, [x14, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x12, x13, [x14, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x12, x13, [x14, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x12, x13, [x14, #64]
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x12, x13, [x14, #80]
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne

// ... then optionally negating before storing. The table entry
// is in precomputed form and we currently have
//
//      [x3;x2;x1;x0] = y - x
//      [x7;x6;x5;x4] = x + y
//      [x11;x10;x9;x8] = 2 * d * x * y
//
// Negation for Edwards curves is -(x,y) = (-x,y), which in this modified
// form amounts to swapping the first two fields and negating the third.
// The negation does not always fully reduce even mod 2^256-38 in the zero
// case, instead giving -0 = 2^256-38. But that is fine since the result is
// always fed to a multiplication inside the "pepadd" function below that
// handles any 256-bit input.

        cmp     cf, xzr

        csel    x12, x0, x4, eq
        csel    x4, x0, x4, ne
        csel    x13, x1, x5, eq
        csel    x5, x1, x5, ne
        csel    x14, x2, x6, eq
        csel    x6, x2, x6, ne
        csel    x15, x3, x7, eq
        csel    x7, x3, x7, ne

        eor     x8, x8, cf
        eor     x9, x9, cf
        eor     x10, x10, cf
        eor     x11, x11, cf
        mov     x0, #37
        and     x0, x0, cf
        subs    x8, x8, x0
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr

        stp     x12, x13, [btabent]
        stp     x14, x15, [btabent+16]
        stp     x4, x5, [btabent+32]
        stp     x6, x7, [btabent+48]
        stp     x8, x9, [btabent+64]
        stp     x10, x11, [btabent+80]

// Get table entry, first getting the adjusted bitfield...

        lsr     x0, i, #6
        ldr     x1, [sp, x0, lsl #3]
        lsr     x2, x1, i
        and     x0, x2, #15
        subs    bf, x0, #8
        cneg    bf, bf, cc
        csetm   cf, cc

// ... then getting the unadjusted table entry

        add     p0, tab

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, #1
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, #1
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        mov     x12, xzr
        mov     x13, xzr
        mov     x14, xzr
        mov     x15, xzr

        cmp     bf, #1
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #2
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #3
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #4
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #5
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #6
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #7
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne
        add     p0, p0, #128

        cmp     bf, #8
        ldp     x16, x17, [p0]
        csel    x0, x0, x16, ne
        csel    x1, x1, x17, ne
        ldp     x16, x17, [p0, #16]
        csel    x2, x2, x16, ne
        csel    x3, x3, x17, ne
        ldp     x16, x17, [p0, #32]
        csel    x4, x4, x16, ne
        csel    x5, x5, x17, ne
        ldp     x16, x17, [p0, #48]
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        ldp     x16, x17, [p0, #64]
        csel    x8, x8, x16, ne
        csel    x9, x9, x17, ne
        ldp     x16, x17, [p0, #80]
        csel    x10, x10, x16, ne
        csel    x11, x11, x17, ne
        ldp     x16, x17, [p0, #96]
        csel    x12, x12, x16, ne
        csel    x13, x13, x17, ne
        ldp     x16, x17, [p0, #112]
        csel    x14, x14, x16, ne
        csel    x15, x15, x17, ne

// ... then optionally negating before storing. This time the table
// entry is extended-projective, and is in registers thus:
//
//      [x3;x2;x1;x0] = X
//      [x7;x6;x5;x4] = Y
//      [x11;x10;x9;x8] = Z
//      [x15;x14;x13;x12] = W
//
// This time we just need to negate the X and the W fields.
// The crude way negation is done can result in values of X or W
// (when initially zero before negation) being exactly equal to
// 2^256-38, but the "pepadd" function handles that correctly.

        eor     x0, x0, cf
        eor     x1, x1, cf
        eor     x2, x2, cf
        eor     x3, x3, cf
        mov     x16, #37
        and     x16, x16, cf
        subs    x0, x0, x16
        sbcs    x1, x1, xzr
        sbcs    x2, x2, xzr
        sbc     x3, x3, xzr

        eor     x12, x12, cf
        eor     x13, x13, cf
        eor     x14, x14, cf
        eor     x15, x15, cf
        subs    x12, x12, x16
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]
        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]
        stp     x8, x9, [tabent+64]
        stp     x10, x11, [tabent+80]
        stp     x12, x13, [tabent+96]
        stp     x14, x15, [tabent+112]

// Double to acc' = 4 * acc

        add     p0, acc
        add     p1, acc
        CFI_BL(Ledwards25519_scalarmuldouble_pdouble)

// Add tabent := tabent + btabent

        add     p0, tabent
        add     p1, tabent
        add     p2, btabent
        CFI_BL(Ledwards25519_scalarmuldouble_pepadd)

// Double to acc' = 8 * acc

        add     p0, acc
        add     p1, acc
        CFI_BL(Ledwards25519_scalarmuldouble_pdouble)

// Double to acc' = 16 * acc

        add     p0, acc
        add     p1, acc
        CFI_BL(Ledwards25519_scalarmuldouble_epdouble)

// Add table entry, acc := acc + tabent

        add     p0, acc
        add     p1, acc
        add     p2, tabent
        CFI_BL(Ledwards25519_scalarmuldouble_epadd)

// Loop down

        cbnz    i, Ledwards25519_scalarmuldouble_loop

// Modular inverse setup

        add     x0, tabent
        add     x1, acc+64

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "arm/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 128 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, acc, tabent.

        mov     x20, x0
        mov     x10, #0xffffffffffffffed
        mov     x11, #0xffffffffffffffff
        stp     x10, x11, [sp]
        mov     x12, #0x7fffffffffffffff
        stp     x11, x12, [sp, #16]
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        mov     x7, #0x13
        lsr     x6, x5, #63
        madd    x6, x7, x6, x7
        adds    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        orr     x5, x5, #0x8000000000000000
        adcs    x5, x5, xzr
        csel    x6, x7, xzr, cc
        subs    x2, x2, x6
        sbcs    x3, x3, xzr
        sbcs    x4, x4, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        stp     x2, x3, [sp, #32]
        stp     x4, x5, [sp, #48]
        stp     xzr, xzr, [sp, #64]
        stp     xzr, xzr, [sp, #80]
        mov     x10, #0x2099
        movk    x10, #0x7502, lsl #16
        movk    x10, #0x9e23, lsl #32
        movk    x10, #0xa0f9, lsl #48
        mov     x11, #0x2595
        movk    x11, #0x1d13, lsl #16
        movk    x11, #0x8f3f, lsl #32
        movk    x11, #0xa8c6, lsl #48
        mov     x12, #0x5242
        movk    x12, #0x5ac, lsl #16
        movk    x12, #0x8938, lsl #32
        movk    x12, #0x6c6c, lsl #48
        mov     x13, #0x615
        movk    x13, #0x4177, lsl #16
        movk    x13, #0x8b2, lsl #32
        movk    x13, #0x2765, lsl #48
        stp     x10, x11, [sp, #96]
        stp     x12, x13, [sp, #112]
        mov     x21, #0xa
        mov     x22, #0x1
        b       Ledwards25519_scalarmuldouble_invmidloop
Ledwards25519_scalarmuldouble_invloop:
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        and     x0, x12, x16
        and     x1, x13, x17
        add     x19, x0, x1
        ldr     x7, [sp]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #32]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x7, [sp, #8]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #40]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [sp]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [sp, #32]
        ldr     x7, [sp, #16]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #48]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [sp, #8]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [sp, #40]
        ldr     x7, [sp, #24]
        eor     x1, x7, x14
        asr     x3, x1, #63
        and     x3, x3, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #56]
        eor     x1, x8, x15
        asr     x0, x1, #63
        and     x0, x0, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [sp, #16]
        extr    x5, x3, x5, #59
        str     x5, [sp, #24]
        eor     x1, x7, x16
        asr     x5, x1, #63
        and     x5, x5, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        asr     x0, x1, #63
        and     x0, x0, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [sp, #48]
        extr    x2, x5, x2, #59
        str     x2, [sp, #56]
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        str     x5, [sp, #96]
        adc     x3, x3, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        str     x3, [sp, #104]
        adc     x4, x4, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        str     x4, [sp, #112]
        adc     x2, x2, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        add     x6, x6, x3, asr #63
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x3, x6, x3
        ldr     x6, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x3
        asr     x3, x3, #63
        adcs    x6, x6, x3
        adc     x5, x5, x3
        stp     x0, x1, [sp, #64]
        stp     x6, x5, [sp, #80]
        eor     x1, x7, x16
        and     x5, x16, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        and     x0, x17, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x6, x5, x2, #63
        ldp     x0, x1, [sp, #96]
        add     x6, x6, x5, asr #63
        mov     x5, #0x13
        mul     x4, x6, x5
        add     x2, x2, x6, lsl #63
        smulh   x5, x6, x5
        ldr     x3, [sp, #112]
        adds    x0, x0, x4
        adcs    x1, x1, x5
        asr     x5, x5, #63
        adcs    x3, x3, x5
        adc     x2, x2, x5
        stp     x0, x1, [sp, #96]
        stp     x3, x2, [sp, #112]
Ledwards25519_scalarmuldouble_invmidloop:
        mov     x1, x22
        ldr     x2, [sp]
        ldr     x3, [sp, #32]
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x8, x4, #0x100, lsl #12
        sbfx    x8, x8, #21, #21
        mov     x11, #0x100000
        add     x11, x11, x11, lsl #21
        add     x9, x4, x11
        asr     x9, x9, #42
        add     x10, x5, #0x100, lsl #12
        sbfx    x10, x10, #21, #21
        add     x11, x5, x11
        asr     x11, x11, #42
        mul     x6, x8, x2
        mul     x7, x9, x3
        mul     x2, x10, x2
        mul     x3, x11, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #21, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #42
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #21, #21
        add     x15, x5, x15
        asr     x15, x15, #42
        mul     x6, x12, x2
        mul     x7, x13, x3
        mul     x2, x14, x2
        mul     x3, x15, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        mul     x2, x12, x8
        mul     x3, x12, x9
        mul     x6, x14, x8
        mul     x7, x14, x9
        madd    x8, x13, x10, x2
        madd    x9, x13, x11, x3
        madd    x16, x15, x10, x6
        madd    x17, x15, x11, x7
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #22, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #43
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #22, #21
        add     x15, x5, x15
        asr     x15, x15, #43
        mneg    x2, x12, x8
        mneg    x3, x12, x9
        mneg    x4, x14, x8
        mneg    x5, x14, x9
        msub    x10, x13, x16, x2
        msub    x11, x13, x17, x3
        msub    x12, x15, x16, x4
        msub    x13, x15, x17, x5
        mov     x22, x1
        subs    x21, x21, #0x1
        b.ne    Ledwards25519_scalarmuldouble_invloop
        ldr     x0, [sp]
        ldr     x1, [sp, #32]
        mul     x0, x0, x10
        madd    x1, x1, x11, x0
        asr     x0, x1, #63
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        eor     x14, x14, x0
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        eor     x15, x15, x0
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        eor     x16, x16, x0
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        eor     x17, x17, x0
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        tst     x3, x3
        cinc    x6, x6, pl
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x6, x6, x3
        ldr     x2, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x6
        asr     x6, x6, #63
        adcs    x2, x2, x6
        adcs    x5, x5, x6
        csel    x3, x3, xzr, mi
        subs    x0, x0, x3
        sbcs    x1, x1, xzr
        sbcs    x2, x2, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        mov     x4, x20
        stp     x0, x1, [x4]
        stp     x2, x5, [x4, #16]

// Store result. Note that these are the only reductions mod 2^255-19

        mov     p0, res
        add     p1, acc
        add     p2, tabent
        mul_p25519(x_0,x_1,x_2)

        add     p0, res, #32
        add     p1, acc+32
        add     p2, tabent
        mul_p25519(x_0,x_1,x_2)

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x25,x30)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_scalarmuldouble)

// ****************************************************************************
// Localized versions of subroutines.
// These are close to the standalone functions "edwards25519_epdouble" etc.,
// but are only maintaining reduction modulo 2^256 - 38, not 2^255 - 19.
// ****************************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_epdouble)

Ledwards25519_scalarmuldouble_epdouble:
        CFI_START
        CFI_DEC_SP(5*NUMSIZE)
        add_twice4(t0,x_1,y_1)
        sqr_4(t1,z_1)
        sqr_4(t2,x_1)
        sqr_4(t3,y_1)
        double_twice4(t1,t1)
        sqr_4(t0,t0)
        add_twice4(t4,t2,t3)
        sub_twice4(t2,t2,t3)
        add_twice4(t3,t1,t2)
        sub_twice4(t1,t4,t0)
        mul_4(y_0,t2,t4)
        mul_4(z_0,t3,t2)
        mul_4(w_0,t1,t4)
        mul_4(x_0,t1,t3)
        CFI_INC_SP(5*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_epdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_pdouble)

Ledwards25519_scalarmuldouble_pdouble:
        CFI_START
        CFI_DEC_SP(5*NUMSIZE)
        add_twice4(t0,x_1,y_1)
        sqr_4(t1,z_1)
        sqr_4(t2,x_1)
        sqr_4(t3,y_1)
        double_twice4(t1,t1)
        sqr_4(t0,t0)
        add_twice4(t4,t2,t3)
        sub_twice4(t2,t2,t3)
        add_twice4(t3,t1,t2)
        sub_twice4(t1,t4,t0)
        mul_4(y_0,t2,t4)
        mul_4(z_0,t3,t2)
        mul_4(x_0,t1,t3)
        CFI_INC_SP(5*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_pdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_epadd)

Ledwards25519_scalarmuldouble_epadd:
        CFI_START
        CFI_DEC_SP(6*NUMSIZE)
        mul_4(t0,w_1,w_2)
        sub_twice4(t1,y_1,x_1)
        sub_twice4(t2,y_2,x_2)
        add_twice4(t3,y_1,x_1)
        add_twice4(t4,y_2,x_2)
        double_twice4(t5,z_2)
        mul_4(t1,t1,t2)
        mul_4(t3,t3,t4)
        load_k25519(t2)
        mul_4(t2,t2,t0)
        mul_4(t4,z_1,t5)
        sub_twice4(t0,t3,t1)
        add_twice4(t5,t3,t1)
        sub_twice4(t1,t4,t2)
        add_twice4(t3,t4,t2)
        mul_4(w_0,t0,t5)
        mul_4(x_0,t0,t1)
        mul_4(y_0,t3,t5)
        mul_4(z_0,t1,t3)
        CFI_INC_SP(6*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_epadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_pepadd)

Ledwards25519_scalarmuldouble_pepadd:
        CFI_START
        CFI_DEC_SP(6*NUMSIZE)
        double_twice4(t0,z_1);
        sub_twice4(t1,y_1,x_1);
        add_twice4(t2,y_1,x_1);
        mul_4(t3,w_1,z_2);
        mul_4(t1,t1,x_2);
        mul_4(t2,t2,y_2);
        sub_twice4(t4,t0,t3);
        add_twice4(t0,t0,t3);
        sub_twice4(t5,t2,t1);
        add_twice4(t1,t2,t1);
        mul_4(z_0,t4,t0);
        mul_4(x_0,t5,t4);
        mul_4(y_0,t0,t1);
        mul_4(w_0,t5,t1);
        CFI_INC_SP(6*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_pepadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif

// ****************************************************************************
// The precomputed data (all read-only).
// ****************************************************************************

#if defined(__ELF__)
.section .rodata
.type S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant), %object
.size S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant), 768
#elif defined(__APPLE__)
.const_data
#endif

// Precomputed table of multiples of generator for edwards25519
// all in precomputed extended-projective (y-x,x+y,2*d*x*y) triples.

S2N_BN_SYMBOL(edwards25519_scalarmuldouble_constant):

        // 1 * G

        .quad   0x9d103905d740913e
        .quad   0xfd399f05d140beb3
        .quad   0xa5c18434688f8a09
        .quad   0x44fd2f9298f81267
        .quad   0x2fbc93c6f58c3b85
        .quad   0xcf932dc6fb8c0e19
        .quad   0x270b4898643d42c2
        .quad   0x07cf9d3a33d4ba65
        .quad   0xabc91205877aaa68
        .quad   0x26d9e823ccaac49e
        .quad   0x5a1b7dcbdd43598c
        .quad   0x6f117b689f0c65a8

        // 2 * G

        .quad   0x8a99a56042b4d5a8
        .quad   0x8f2b810c4e60acf6
        .quad   0xe09e236bb16e37aa
        .quad   0x6bb595a669c92555
        .quad   0x9224e7fc933c71d7
        .quad   0x9f469d967a0ff5b5
        .quad   0x5aa69a65e1d60702
        .quad   0x590c063fa87d2e2e
        .quad   0x43faa8b3a59b7a5f
        .quad   0x36c16bdd5d9acf78
        .quad   0x500fa0840b3d6a31
        .quad   0x701af5b13ea50b73

        // 3 * G

        .quad   0x56611fe8a4fcd265
        .quad   0x3bd353fde5c1ba7d
        .quad   0x8131f31a214bd6bd
        .quad   0x2ab91587555bda62
        .quad   0xaf25b0a84cee9730
        .quad   0x025a8430e8864b8a
        .quad   0xc11b50029f016732
        .quad   0x7a164e1b9a80f8f4
        .quad   0x14ae933f0dd0d889
        .quad   0x589423221c35da62
        .quad   0xd170e5458cf2db4c
        .quad   0x5a2826af12b9b4c6

        // 4 * G

        .quad   0x95fe050a056818bf
        .quad   0x327e89715660faa9
        .quad   0xc3e8e3cd06a05073
        .quad   0x27933f4c7445a49a
        .quad   0x287351b98efc099f
        .quad   0x6765c6f47dfd2538
        .quad   0xca348d3dfb0a9265
        .quad   0x680e910321e58727
        .quad   0x5a13fbe9c476ff09
        .quad   0x6e9e39457b5cc172
        .quad   0x5ddbdcf9102b4494
        .quad   0x7f9d0cbf63553e2b

        // 5 * G

        .quad   0x7f9182c3a447d6ba
        .quad   0xd50014d14b2729b7
        .quad   0xe33cf11cb864a087
        .quad   0x154a7e73eb1b55f3
        .quad   0xa212bc4408a5bb33
        .quad   0x8d5048c3c75eed02
        .quad   0xdd1beb0c5abfec44
        .quad   0x2945ccf146e206eb
        .quad   0xbcbbdbf1812a8285
        .quad   0x270e0807d0bdd1fc
        .quad   0xb41b670b1bbda72d
        .quad   0x43aabe696b3bb69a

        // 6 * G

        .quad   0x499806b67b7d8ca4
        .quad   0x575be28427d22739
        .quad   0xbb085ce7204553b9
        .quad   0x38b64c41ae417884
        .quad   0x3a0ceeeb77157131
        .quad   0x9b27158900c8af88
        .quad   0x8065b668da59a736
        .quad   0x51e57bb6a2cc38bd
        .quad   0x85ac326702ea4b71
        .quad   0xbe70e00341a1bb01
        .quad   0x53e4a24b083bc144
        .quad   0x10b8e91a9f0d61e3

        // 7 * G

        .quad   0xba6f2c9aaa3221b1
        .quad   0x6ca021533bba23a7
        .quad   0x9dea764f92192c3a
        .quad   0x1d6edd5d2e5317e0
        .quad   0x6b1a5cd0944ea3bf
        .quad   0x7470353ab39dc0d2
        .quad   0x71b2528228542e49
        .quad   0x461bea69283c927e
        .quad   0xf1836dc801b8b3a2
        .quad   0xb3035f47053ea49a
        .quad   0x529c41ba5877adf3
        .quad   0x7a9fbb1c6a0f90a7

        // 8 * G

        .quad   0xe2a75dedf39234d9
        .quad   0x963d7680e1b558f9
        .quad   0x2c2741ac6e3c23fb
        .quad   0x3a9024a1320e01c3
        .quad   0x59b7596604dd3e8f
        .quad   0x6cb30377e288702c
        .quad   0xb1339c665ed9c323
        .quad   0x0915e76061bce52f
        .quad   0xe7c1f5d9c9a2911a
        .quad   0xb8a371788bcca7d7
        .quad   0x636412190eb62a32
        .quad   0x26907c5c2ecc4e95
