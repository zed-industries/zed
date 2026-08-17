// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjadd
//      (uint64_t p3[static 12],uint64_t p1[static 12],uint64_t p2[static 12]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjadd)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x21
#define input_x x22
#define input_y x23

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
// NSPACE is the total stack needed for these temporaries

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

#define NSPACE (NUMSIZE*7)


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


S2N_BN_SYMBOL(p256_montjadd):

// Save regs and make room on stack for temporary variables

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x30, [sp, #-16]!
        sub     sp, sp, NSPACE

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Main code, just a sequence of basic field operations
// 12 * multiply + 4 * square + 7 * subtract

        montsqr_p256(z1sq,z_1)
        montsqr_p256(z2sq,z_2)

        montmul_p256(y1a,z_2,y_1)
        montmul_p256(y2a,z_1,y_2)

        montmul_p256(x2a,z1sq,x_2)
        montmul_p256(x1a,z2sq,x_1)
        montmul_p256(y2a,z1sq,y2a)
        montmul_p256(y1a,z2sq,y1a)

        sub_p256(xd,x2a,x1a)
        sub_p256(yd,y2a,y1a)

        montsqr_p256(zz,xd)
        montsqr_p256(ww,yd)

        montmul_p256(zzx1,zz,x1a)
        montmul_p256(zzx2,zz,x2a)

        sub_p256(resx,ww,zzx1)
        sub_p256(t1,zzx2,zzx1)

        montmul_p256(xd,xd,z_1)

        sub_p256(resx,resx,zzx2)

        sub_p256(t2,zzx1,resx)

        montmul_p256(t1,t1,y1a)
        montmul_p256(resz,xd,z_2)
        montmul_p256(t2,yd,t2)

        sub_p256(resy,t2,t1)

// Load in the z coordinates of the inputs to check for P1 = 0 and P2 = 0
// The condition codes get set by a comparison (P2 != 0) - (P1 != 0)
// So  "HI" <=> CF /\ ~ZF <=> P1 = 0 /\ ~(P2 = 0)
// and "LO" <=> ~CF       <=> ~(P1 = 0) /\ P2 = 0

        ldp     x0, x1, [z_1]
        ldp     x2, x3, [z_1+16]

        orr     x12, x0, x1
        orr     x13, x2, x3
        orr     x12, x12, x13
        cmp     x12, xzr
        cset    x12, ne

        ldp     x4, x5, [z_2]
        ldp     x6, x7, [z_2+16]

        orr     x13, x4, x5
        orr     x14, x6, x7
        orr     x13, x13, x14
        cmp     x13, xzr
        cset    x13, ne

        cmp     x13, x12

// Multiplex the outputs accordingly, re-using the z's in registers

        ldp     x8, x9, [resz]
        csel    x8, x0, x8, lo
        csel    x9, x1, x9, lo
        csel    x8, x4, x8, hi
        csel    x9, x5, x9, hi
        ldp     x10, x11, [resz+16]
        csel    x10, x2, x10, lo
        csel    x11, x3, x11, lo
        csel    x10, x6, x10, hi
        csel    x11, x7, x11, hi

        ldp     x12, x13, [x_1]
        ldp     x0, x1, [resx]
        csel    x0, x12, x0, lo
        csel    x1, x13, x1, lo
        ldp     x12, x13, [x_2]
        csel    x0, x12, x0, hi
        csel    x1, x13, x1, hi

        ldp     x12, x13, [x_1+16]
        ldp     x2, x3, [resx+16]
        csel    x2, x12, x2, lo
        csel    x3, x13, x3, lo
        ldp     x12, x13, [x_2+16]
        csel    x2, x12, x2, hi
        csel    x3, x13, x3, hi

        ldp     x12, x13, [y_1]
        ldp     x4, x5, [resy]
        csel    x4, x12, x4, lo
        csel    x5, x13, x5, lo
        ldp     x12, x13, [y_2]
        csel    x4, x12, x4, hi
        csel    x5, x13, x5, hi

        ldp     x12, x13, [y_1+16]
        ldp     x6, x7, [resy+16]
        csel    x6, x12, x6, lo
        csel    x7, x13, x7, lo
        ldp     x12, x13, [y_2+16]
        csel    x6, x12, x6, hi
        csel    x7, x13, x7, hi

// Finally store back the multiplexed values

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [y_3]
        stp     x6, x7, [y_3+16]
        stp     x8, x9, [z_3]
        stp     x10, x11, [z_3+16]

// Restore registers and return

        add     sp, sp, NSPACE
        ldp     x23, x30, [sp], 16
        ldp     x21, x22, [sp], 16
        ldp     x19, x20, [sp], 16
        ret


#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
