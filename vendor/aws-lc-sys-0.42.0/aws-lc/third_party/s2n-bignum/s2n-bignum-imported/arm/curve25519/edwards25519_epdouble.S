// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended projective doubling for edwards25519
// Input p1[12]; output p3[16]
//
// extern void edwards25519_epdouble
//   (uint64_t p3[static 16],const uint64_t p1[static 12]);
//
// If p1 is a point on edwards25519, returns its double p3 = 2 * p1.
// The output p3 is in extended projective coordinates, representing
// affine (x,y) by a quadruple (X,Y,Z,T) where x = X / Z, y = Y / Z
// and x * y = T / Z. The input p1 may also be in the same extended
// projective representation, but the final T field is not used so
// a more basic projective triple (X,Y,Z) suffices.
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_epdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_epdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_epdouble)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define p3 x17
#define p1 x19

// Pointers to input and output coordinates

#define x_1 p1, #0
#define y_1 p1, #NUMSIZE
#define z_1 p1, #(2*NUMSIZE)

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

// Total size to reserve on the stack

#define NSPACE 5*NUMSIZE

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

// Plain 4-digit adding without any normalization.
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

S2N_BN_SYMBOL(edwards25519_epdouble):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     p3, x0
        mov     p1, x1

// Main sequence

        add_4(t0,x_1,y_1)
        sqr_4(t1,z_1)
        sqr_4(t2,x_1)
        sqr_4(t3,y_1)
        double_twice4(t1,t1)
        sqr_4(t0,t0)
        add_twice4(t4,t2,t3)
        sub_twice4(t2,t2,t3)
        add_twice4(t3,t1,t2)
        sub_twice4(t1,t4,t0)
        mul_p25519(y_3,t2,t4)
        mul_p25519(z_3,t3,t2)
        mul_p25519(w_3,t1,t4)
        mul_p25519(x_3,t1,t3)

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_epdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
