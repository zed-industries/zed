// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on SECG curve secp256k1 in Jacobian coordinates
//
//    extern void secp256k1_jmixadd(uint64_t p3[static 12],
//                                  const uint64_t p1[static 12],
//                                  const uint64_t p2[static 8]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity. It is assumed that
// all the coordinates of the input points p1 and p2 are fully reduced
// mod p_256k1, that the z coordinate of p1 is nonzero and that neither
// p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents the same affine
// point as".
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(secp256k1_jmixadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(secp256k1_jmixadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(secp256k1_jmixadd)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x19
#define input_x x20
#define input_y x21

// The magic constant 2^256 - p_256k1

#define pconst x17

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

// Corresponds exactly to bignum_mul_p256k1 except for registers and
// re-use of the pconst register for the constant 4294968273

#define mul_p256k1(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x5, x6, [P2] __LF                  \
        mul     x7, x3, x5 __LF                    \
        umulh   x8, x3, x5 __LF                    \
        mul     x9, x4, x6 __LF                    \
        umulh   x10, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, lo __LF                    \
        csetm   x16, lo __LF                       \
        adds    x9, x9, x8 __LF                    \
        adc     x10, x10, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, lo __LF                    \
        cinv    x16, x16, lo __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x8, x7, x9 __LF                    \
        adcs    x9, x9, x10 __LF                   \
        adc     x10, x10, xzr __LF                 \
        cmn     x16, #1 __LF                       \
        eor     x15, x15, x16 __LF                 \
        adcs    x8, x15, x8 __LF                   \
        eor     x3, x3, x16 __LF                   \
        adcs    x9, x3, x9 __LF                    \
        adc     x10, x10, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x5, x6, [P2+16] __LF               \
        mul     x11, x3, x5 __LF                   \
        umulh   x12, x3, x5 __LF                   \
        mul     x13, x4, x6 __LF                   \
        umulh   x14, x4, x6 __LF                   \
        subs    x4, x4, x3 __LF                    \
        cneg    x4, x4, lo __LF                    \
        csetm   x16, lo __LF                       \
        adds    x13, x13, x12 __LF                 \
        adc     x14, x14, xzr __LF                 \
        subs    x3, x5, x6 __LF                    \
        cneg    x3, x3, lo __LF                    \
        cinv    x16, x16, lo __LF                  \
        mul     x15, x4, x3 __LF                   \
        umulh   x3, x4, x3 __LF                    \
        adds    x12, x11, x13 __LF                 \
        adcs    x13, x13, x14 __LF                 \
        adc     x14, x14, xzr __LF                 \
        cmn     x16, #1 __LF                       \
        eor     x15, x15, x16 __LF                 \
        adcs    x12, x15, x12 __LF                 \
        eor     x3, x3, x16 __LF                   \
        adcs    x13, x3, x13 __LF                  \
        adc     x14, x14, x16 __LF                 \
        ldp     x3, x4, [P1+16] __LF               \
        ldp     x15, x16, [P1] __LF                \
        subs    x3, x3, x15 __LF                   \
        sbcs    x4, x4, x16 __LF                   \
        csetm   x16, lo __LF                       \
        ldp     x15, x0, [P2] __LF                 \
        subs    x5, x15, x5 __LF                   \
        sbcs    x6, x0, x6 __LF                    \
        csetm   x0, lo __LF                        \
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
        cneg    x4, x4, lo __LF                    \
        csetm   x9, lo __LF                        \
        adds    x15, x15, x0 __LF                  \
        adc     x1, x1, xzr __LF                   \
        subs    x6, x5, x6 __LF                    \
        cneg    x6, x6, lo __LF                    \
        cinv    x9, x9, lo __LF                    \
        mul     x5, x4, x6 __LF                    \
        umulh   x6, x4, x6 __LF                    \
        adds    x0, x2, x15 __LF                   \
        adcs    x15, x15, x1 __LF                  \
        adc     x1, x1, xzr __LF                   \
        cmn     x9, #1 __LF                        \
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
        cmn     x16, #1 __LF                       \
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
        mov     x16, #977 __LF                     \
        mul     x3, pconst, x11 __LF               \
        umulh   x5, pconst, x11 __LF               \
        and     x15, x12, #0xffffffff __LF         \
        lsr     x2, x12, #32 __LF                  \
        mul     x4, x16, x15 __LF                  \
        madd    x15, x16, x2, x15 __LF             \
        adds    x4, x4, x15, lsl #32 __LF          \
        lsr     x15, x15, #32 __LF                 \
        adc     x6, x2, x15 __LF                   \
        mul     x11, pconst, x13 __LF              \
        umulh   x13, pconst, x13 __LF              \
        and     x15, x14, #0xffffffff __LF         \
        lsr     x2, x14, #32 __LF                  \
        mul     x12, x16, x15 __LF                 \
        madd    x15, x16, x2, x15 __LF             \
        adds    x12, x12, x15, lsl #32 __LF        \
        lsr     x15, x15, #32 __LF                 \
        adc     x14, x2, x15 __LF                  \
        adds    x7, x7, x3 __LF                    \
        adcs    x8, x8, x4 __LF                    \
        adcs    x9, x9, x11 __LF                   \
        adcs    x10, x10, x12 __LF                 \
        cset    x11, hs __LF                       \
        adds    x8, x8, x5 __LF                    \
        adcs    x9, x9, x6 __LF                    \
        adcs    x10, x10, x13 __LF                 \
        adc     x11, x11, x14 __LF                 \
        add     x0, x11, #1 __LF                   \
        mul     x3, x16, x0 __LF                   \
        lsr     x4, x0, #32 __LF                   \
        adds    x3, x3, x0, lsl #32 __LF           \
        adc     x4, xzr, x4 __LF                   \
        adds    x7, x7, x3 __LF                    \
        adcs    x8, x8, x4 __LF                    \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        csel    x1, pconst, xzr, lo __LF           \
        subs    x7, x7, x1 __LF                    \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbc     x10, x10, xzr __LF                 \
        stp     x7, x8, [P0] __LF                  \
        stp     x9, x10, [P0+16]

// Corresponds exactly to bignum_sqr_p256k1 except for
// re-use of the pconst register for the constant 4294968273

#define sqr_p256k1(P0,P1)                       \
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
        csetm   x16, lo __LF                       \
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
        csetm   x16, lo __LF                       \
        subs    x4, x4, x12 __LF                   \
        sbcs    x5, x5, x13 __LF                   \
        sbcs    x6, x6, x15 __LF                   \
        sbcs    x7, x7, x14 __LF                   \
        adcs    x8, x8, x16 __LF                   \
        adc     x9, x9, x16 __LF                   \
        mov     x16, #977 __LF                     \
        mul     x10, pconst, x6 __LF               \
        umulh   x13, pconst, x6 __LF               \
        and     x6, x7, #0xffffffff __LF           \
        lsr     x7, x7, #32 __LF                   \
        mul     x11, x16, x6 __LF                  \
        madd    x6, x16, x7, x6 __LF               \
        adds    x11, x11, x6, lsl #32 __LF         \
        lsr     x6, x6, #32 __LF                   \
        adc     x14, x7, x6 __LF                   \
        mul     x12, pconst, x8 __LF               \
        umulh   x8, pconst, x8 __LF                \
        and     x6, x9, #0xffffffff __LF           \
        lsr     x7, x9, #32 __LF                   \
        mul     x9, x16, x6 __LF                   \
        madd    x6, x16, x7, x6 __LF               \
        adds    x9, x9, x6, lsl #32 __LF           \
        lsr     x6, x6, #32 __LF                   \
        adc     x15, x7, x6 __LF                   \
        adds    x2, x2, x10 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adcs    x4, x4, x12 __LF                   \
        adcs    x5, x5, x9 __LF                    \
        cset    x6, hs __LF                        \
        adds    x3, x3, x13 __LF                   \
        adcs    x4, x4, x14 __LF                   \
        adcs    x5, x5, x8 __LF                    \
        adc     x6, x6, x15 __LF                   \
        add     x6, x6, #1 __LF                    \
        mul     x10, x16, x6 __LF                  \
        lsr     x11, x6, #32 __LF                  \
        adds    x10, x10, x6, lsl #32 __LF         \
        adc     x11, xzr, x11 __LF                 \
        adds    x2, x2, x10 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        csel    x16, pconst, xzr, lo __LF          \
        subs    x2, x2, x16 __LF                   \
        sbcs    x3, x3, xzr __LF                   \
        sbcs    x4, x4, xzr __LF                   \
        sbc     x5, x5, xzr __LF                   \
        stp     x2, x3, [P0] __LF                  \
        stp     x4, x5, [P0+16]

// Corresponds exactly to bignum_sub_p256k1

#define sub_p256k1(P0,P1,P2)                    \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #0x3d1 __LF                    \
        orr     x3, x4, #0x100000000 __LF          \
        csel    x3, x3, xzr, cc __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

S2N_BN_SYMBOL(secp256k1_jmixadd):
        CFI_START

// Save registers and make room on stack for temporary variables

        CFI_DEC_SP(NSPACE+32)
        CFI_STACKSAVE2(x19,x20,NSPACE)
        CFI_STACKSAVE2(x21,x22,NSPACE+16)

// Move the input arguments to stable place

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Set up pconst =  4294968273, so p_256k1 = 2^256 - pconst

        mov     pconst, #977
        orr     pconst, pconst, #0x100000000

// Main code, just a sequence of basic field operations

        sqr_p256k1(zp2,z_1)
        mul_p256k1(y2a,z_1,y_2)

        mul_p256k1(x2a,zp2,x_2)
        mul_p256k1(y2a,zp2,y2a)

        sub_p256k1(xd,x2a,x_1)
        sub_p256k1(yd,y2a,y_1)

        sqr_p256k1(zz,xd)
        sqr_p256k1(ww,yd)

        mul_p256k1(zzx1,zz,x_1)
        mul_p256k1(zzx2,zz,x2a)

        sub_p256k1(resx,ww,zzx1)
        sub_p256k1(t1,zzx2,zzx1)

        mul_p256k1(resz,xd,z_1)

        sub_p256k1(resx,resx,zzx2)

        sub_p256k1(t2,zzx1,resx)

        mul_p256k1(t1,t1,y_1)
        mul_p256k1(t2,yd,t2)

        sub_p256k1(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        ldp     x0, x1, [z_1]
        ldp     x2, x3, [z_1+16]
        orr     x4, x0, x1
        orr     x5, x2, x3
        orr     x4, x4, x5
        cmp     x4, xzr

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with an extra z = 1
// coordinate, hence giving 0 + p2 = p2 for the final result.

        ldp     x0, x1, [resx]
        ldp     x12, x13, [x_2]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x2, x3, [resx+16]
        ldp     x12, x13, [x_2+16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne

        ldp     x4, x5, [resy]
        ldp     x12, x13, [y_2]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x6, x7, [resy+16]
        ldp     x12, x13, [y_2+16]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne

        ldp     x8, x9, [resz]
        mov     x12, #1
        csel    x8, x8, x12, ne
        csel    x9, x9, xzr, ne
        ldp     x10, x11, [resz+16]
        csel    x10, x10, xzr, ne
        csel    x11, x11, xzr, ne

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [y_3]
        stp     x6, x7, [y_3+16]
        stp     x8, x9, [z_3]
        stp     x10, x11, [z_3+16]

// Restore stack and return

        CFI_STACKLOAD2(x19,x20,NSPACE)
        CFI_STACKLOAD2(x21,x22,NSPACE+16)
        CFI_INC_SP((NSPACE+32))
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(secp256k1_jmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
