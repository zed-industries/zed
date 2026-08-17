// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjdouble
//      (uint64_t p3[static 12],uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjdouble)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x19
#define input_x x20

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// NSPACE is the total stack needed for these temporaries

#define z2 sp, #(NUMSIZE*0)
#define y4 sp, #(NUMSIZE*0)

#define y2 sp, #(NUMSIZE*1)

#define t1 sp, #(NUMSIZE*2)

#define t2 sp, #(NUMSIZE*3)
#define x2p sp, #(NUMSIZE*3)
#define dx2 sp, #(NUMSIZE*3)

#define xy2 sp, #(NUMSIZE*4)

#define x4p sp, #(NUMSIZE*5)
#define d_ sp, #(NUMSIZE*5)

#define NSPACE #(NUMSIZE*6)

// Corresponds exactly to bignum_montmul_p256

.montmul_p256:
        ldr     q20, [x2]
        ldp     x7, x17, [x1]
        ldr     q0, [x1]
        ldp     x6, x10, [x2]
        ldp     x11, x15, [x1, #16]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc
        cneg    x13, x4, cc
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [x2, #16]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc
        cinv    x9, x3, cc
        cmn     x17, #0x1
        ldr     q28, [x1, #16]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x2, #16]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc
        cneg    x6, x6, cc
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x9, x3, x13
        adcs    x3, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x12, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x15, x15, xzr
        rev64   v24.4s, v20.4s
        stp     x12, x15, [x0, #16]
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        stp     x9, x3, [x0]
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        ldp     x15, x8, [x0]
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        ldp     x9, x13, [x0, #16]
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x15
        adcs    x15, x16, x8
        eor     x5, x17, x4
        adcs    x9, x1, x9
        eor     x1, x10, x5
        adcs    x16, x2, x13
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [x0]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [x0, #16]
        ret

// Corresponds exactly to bignum_montsqr_p256

.montsqr_p256:
        ldr     q19, [x1]
        ldp     x9, x13, [x1]
        ldr     q23, [x1, #16]
        ldr     q0, [x1]
        ldp     x1, x10, [x1, #16]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc
        cneg    x6, x14, cc
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc
        cinv    x2, x5, cc
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x16, x3, x16, cs
        csel    x14, x8, x14, cs
        csel    x12, x11, x12, cs
        csel    x2, x5, x2, cs
        stp     x14, x12, [x0, #16]
        stp     x16, x2, [x0]
        ret

// Corresponds exactly to bignum_sub_p256

.sub_p256:
        ldp     x5, x6, [x1]
        ldp     x4, x3, [x2]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [x0]
        stp     x7, x8, [x0, #16]
        ret

// Corresponds exactly to bignum_add_p256

.add_p256:
        ldp     x4, x5, [x1]
        ldp     x8, x9, [x2]
        adds    x4, x4, x8
        adcs    x5, x5, x9
        ldp     x6, x7, [x1, #16]
        ldp     x10, x11, [x2, #16]
        adcs    x6, x6, x10
        adcs    x7, x7, x11
        adc     x3, xzr, xzr
        adds    x8, x4, #0x1
        mov     x9, #0xffffffff
        sbcs    x9, x5, x9
        sbcs    x10, x6, xzr
        mov     x11, #0xffffffff00000001
        sbcs    x11, x7, x11
        sbcs    x3, x3, xzr
        csel    x4, x4, x8, cc
        csel    x5, x5, x9, cc
        csel    x6, x6, x10, cc
        csel    x7, x7, x11, cc
        stp     x4, x5, [x0]
        stp     x6, x7, [x0, #16]
        ret


#define montmul_p256(P0,P1,P2) \
        add x0, P0;\
        add x1, P1;\
        add x2, P2;\
        bl .montmul_p256

#define montsqr_p256(P0,P1) \
        add x0, P0;\
        add x1, P1;\
        bl .montsqr_p256

#define sub_p256(P0,P1,P2) \
        add x0, P0;\
        add x1, P1;\
        add x2, P2;\
        bl .sub_p256

#define add_p256(P0,P1,P2) \
        add x0, P0;\
        add x1, P1;\
        add x2, P2;\
        bl .add_p256


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

S2N_BN_SYMBOL(p256_montjdouble):

// Save registers and make room on stack for temporary variables

        sub     sp, sp, NSPACE+32
        stp     x30, xzr, [sp, NSPACE+16]
        stp     x19, x20, [sp, NSPACE]

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

        cmsub_p256(d_,12,xy2,9,x4p)
        sub_p256(t1,t1,z2)

// y4 = y^4

        montsqr_p256(y4,y2)

// dx2 = d * x2p

        montmul_p256(dx2,d_,x2p)

// z_3' = 2 * y * z

        sub_p256(z_3,t1,y2)

// x' = 4 * xy2 - d

        cmsub41_p256(x_3,xy2,d_)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p256(y_3,dx2,y4)

// Restore registers and stack and return

        ldp     x19, x20, [sp, NSPACE]
        ldp     x30, xzr, [sp, NSPACE+16]
        add     sp, sp, NSPACE+32
        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
