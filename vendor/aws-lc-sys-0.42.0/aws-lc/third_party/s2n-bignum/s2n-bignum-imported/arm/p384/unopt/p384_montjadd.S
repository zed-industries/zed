// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjadd
//      (uint64_t p3[static 18],uint64_t p1[static 18],uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjadd)
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

// Corresponds to bignum_montmul_p384, with callee-save register spills
// rewritten to update sp in advance

.montmul_p384:
        sub sp, sp, 48
        stp     x19, x20, [sp, 32]
        stp     x21, x22, [sp, 16]
        stp     x23, x24, [sp]
        ldr q3, [x1]
        ldr q25, [x2]
        ldp x13, x23, [x2]
        ldp x3, x21, [x1]
        rev64 v23.4S, v25.4S
        uzp1 v17.4S, v25.4S, v3.4S
        umulh x15, x3, x13
        mul v6.4S, v23.4S, v3.4S
        uzp1 v3.4S, v3.4S, v3.4S
        ldr q27, [x2, #32]
        ldp x8, x24, [x1, #16]
        subs x6, x3, x21
        ldr q0, [x1, #32]
        movi v23.2D, #0x00000000ffffffff
        csetm x10, cc
        umulh x19, x21, x23
        rev64 v4.4S, v27.4S
        uzp2 v25.4S, v27.4S, v27.4S
        cneg x4, x6, cc
        subs x7, x23, x13
        xtn v22.2S, v0.2D
        xtn v24.2S, v27.2D
        cneg x20, x7, cc
        ldp x6, x14, [x2, #16]
        mul v27.4S, v4.4S, v0.4S
        uaddlp v20.2D, v6.4S
        cinv x5, x10, cc
        mul x16, x4, x20
        uzp2 v6.4S, v0.4S, v0.4S
        umull v21.2D, v22.2S, v25.2S
        shl v0.2D, v20.2D, #32
        umlal v0.2D, v3.2S, v17.2S
        mul x22, x8, x6
        umull v1.2D, v6.2S, v25.2S
        subs x12, x3, x8
        umull v20.2D, v22.2S, v24.2S
        cneg x17, x12, cc
        umulh x9, x8, x6
        mov x12, v0.d[1]
        eor x11, x16, x5
        mov x7, v0.d[0]
        csetm x10, cc
        usra v21.2D, v20.2D, #32
        adds x15, x15, x12
        adcs x12, x19, x22
        umulh x20, x4, x20
        adc x19, x9, xzr
        usra v1.2D, v21.2D, #32
        adds x22, x15, x7
        and v26.16B, v21.16B, v23.16B
        adcs x16, x12, x15
        uaddlp v25.2D, v27.4S
        adcs x9, x19, x12
        umlal v26.2D, v6.2S, v24.2S
        adc x4, x19, xzr
        adds x16, x16, x7
        shl v27.2D, v25.2D, #32
        adcs x9, x9, x15
        adcs x4, x4, x12
        eor x12, x20, x5
        adc x15, x19, xzr
        subs x20, x6, x13
        cneg x20, x20, cc
        cinv x10, x10, cc
        cmn x5, #0x1
        mul x19, x17, x20
        adcs x11, x22, x11
        adcs x12, x16, x12
        adcs x9, x9, x5
        umulh x17, x17, x20
        adcs x22, x4, x5
        adc x5, x15, x5
        subs x16, x21, x8
        cneg x20, x16, cc
        eor x19, x19, x10
        csetm x4, cc
        subs x16, x6, x23
        cneg x16, x16, cc
        umlal v27.2D, v22.2S, v24.2S
        mul x15, x20, x16
        cinv x4, x4, cc
        cmn x10, #0x1
        usra v1.2D, v26.2D, #32
        adcs x19, x12, x19
        eor x17, x17, x10
        adcs x9, x9, x17
        adcs x22, x22, x10
        lsl x12, x7, #32
        umulh x20, x20, x16
        eor x16, x15, x4
        ldp x15, x17, [x2, #32]
        add x2, x12, x7
        adc x7, x5, x10
        ldp x5, x10, [x1, #32]
        lsr x1, x2, #32
        eor x12, x20, x4
        subs x1, x1, x2
        sbc x20, x2, xzr
        cmn x4, #0x1
        adcs x9, x9, x16
        extr x1, x20, x1, #32
        lsr x20, x20, #32
        adcs x22, x22, x12
        adc x16, x7, x4
        adds x12, x20, x2
        umulh x7, x24, x14
        adc x4, xzr, xzr
        subs x1, x11, x1
        sbcs x20, x19, x12
        sbcs x12, x9, x4
        lsl x9, x1, #32
        add x1, x9, x1
        sbcs x9, x22, xzr
        mul x22, x24, x14
        sbcs x16, x16, xzr
        lsr x4, x1, #32
        sbc x19, x2, xzr
        subs x4, x4, x1
        sbc x11, x1, xzr
        extr x2, x11, x4, #32
        lsr x4, x11, #32
        adds x4, x4, x1
        adc x11, xzr, xzr
        subs x2, x20, x2
        sbcs x4, x12, x4
        sbcs x20, x9, x11
        lsl x12, x2, #32
        add x2, x12, x2
        sbcs x9, x16, xzr
        lsr x11, x2, #32
        sbcs x19, x19, xzr
        sbc x1, x1, xzr
        subs x16, x11, x2
        sbc x12, x2, xzr
        extr x16, x12, x16, #32
        lsr x12, x12, #32
        adds x11, x12, x2
        adc x12, xzr, xzr
        subs x16, x4, x16
        mov x4, v27.d[0]
        sbcs x11, x20, x11
        sbcs x20, x9, x12
        stp x16, x11, [x0]
        sbcs x11, x19, xzr
        sbcs x9, x1, xzr
        stp x20, x11, [x0, #16]
        mov x1, v1.d[0]
        sbc x20, x2, xzr
        subs x12, x24, x5
        mov x11, v27.d[1]
        cneg x16, x12, cc
        csetm x2, cc
        subs x19, x15, x14
        mov x12, v1.d[1]
        cinv x2, x2, cc
        cneg x19, x19, cc
        stp x9, x20, [x0, #32]
        mul x9, x16, x19
        adds x4, x7, x4
        adcs x11, x1, x11
        adc x1, x12, xzr
        adds x20, x4, x22
        umulh x19, x16, x19
        adcs x7, x11, x4
        eor x16, x9, x2
        adcs x9, x1, x11
        adc x12, x1, xzr
        adds x7, x7, x22
        adcs x4, x9, x4
        adcs x9, x12, x11
        adc x12, x1, xzr
        cmn x2, #0x1
        eor x1, x19, x2
        adcs x11, x20, x16
        adcs x19, x7, x1
        adcs x1, x4, x2
        adcs x20, x9, x2
        adc x2, x12, x2
        subs x12, x24, x10
        cneg x16, x12, cc
        csetm x12, cc
        subs x9, x17, x14
        cinv x12, x12, cc
        cneg x9, x9, cc
        subs x3, x24, x3
        sbcs x21, x5, x21
        mul x24, x16, x9
        sbcs x4, x10, x8
        ngc x8, xzr
        subs x10, x5, x10
        eor x5, x24, x12
        csetm x7, cc
        cneg x24, x10, cc
        subs x10, x17, x15
        cinv x7, x7, cc
        cneg x10, x10, cc
        subs x14, x13, x14
        sbcs x15, x23, x15
        eor x13, x21, x8
        mul x23, x24, x10
        sbcs x17, x6, x17
        eor x6, x3, x8
        ngc x21, xzr
        umulh x9, x16, x9
        cmn x8, #0x1
        eor x3, x23, x7
        adcs x23, x6, xzr
        adcs x13, x13, xzr
        eor x16, x4, x8
        adc x16, x16, xzr
        eor x4, x17, x21
        umulh x17, x24, x10
        cmn x21, #0x1
        eor x24, x14, x21
        eor x6, x15, x21
        adcs x15, x24, xzr
        adcs x14, x6, xzr
        adc x6, x4, xzr
        cmn x12, #0x1
        eor x4, x9, x12
        adcs x19, x19, x5
        umulh x5, x23, x15
        adcs x1, x1, x4
        adcs x10, x20, x12
        eor x4, x17, x7
        ldp x20, x9, [x0]
        adc x2, x2, x12
        cmn x7, #0x1
        adcs x12, x1, x3
        ldp x17, x24, [x0, #16]
        mul x1, x16, x6
        adcs x3, x10, x4
        adc x2, x2, x7
        ldp x7, x4, [x0, #32]
        adds x20, x22, x20
        mul x10, x13, x14
        adcs x11, x11, x9
        eor x9, x8, x21
        adcs x21, x19, x17
        stp x20, x11, [x0]
        adcs x12, x12, x24
        mul x8, x23, x15
        adcs x3, x3, x7
        stp x21, x12, [x0, #16]
        adcs x12, x2, x4
        adc x19, xzr, xzr
        subs x21, x23, x16
        umulh x2, x16, x6
        stp x3, x12, [x0, #32]
        cneg x3, x21, cc
        csetm x24, cc
        umulh x11, x13, x14
        subs x21, x13, x16
        eor x7, x8, x9
        cneg x17, x21, cc
        csetm x16, cc
        subs x21, x6, x15
        cneg x22, x21, cc
        cinv x21, x24, cc
        subs x20, x23, x13
        umulh x12, x3, x22
        cneg x23, x20, cc
        csetm x24, cc
        subs x20, x14, x15
        cinv x24, x24, cc
        mul x22, x3, x22
        cneg x3, x20, cc
        subs x13, x6, x14
        cneg x20, x13, cc
        cinv x15, x16, cc
        adds x13, x5, x10
        mul x4, x23, x3
        adcs x11, x11, x1
        adc x14, x2, xzr
        adds x5, x13, x8
        adcs x16, x11, x13
        umulh x23, x23, x3
        adcs x3, x14, x11
        adc x1, x14, xzr
        adds x10, x16, x8
        adcs x6, x3, x13
        adcs x8, x1, x11
        umulh x13, x17, x20
        eor x1, x4, x24
        adc x4, x14, xzr
        cmn x24, #0x1
        adcs x1, x5, x1
        eor x16, x23, x24
        eor x11, x1, x9
        adcs x23, x10, x16
        eor x2, x22, x21
        adcs x3, x6, x24
        mul x14, x17, x20
        eor x17, x13, x15
        adcs x13, x8, x24
        adc x8, x4, x24
        cmn x21, #0x1
        adcs x6, x23, x2
        mov x16, #0xfffffffffffffffe
        eor x20, x12, x21
        adcs x20, x3, x20
        eor x23, x14, x15
        adcs x2, x13, x21
        adc x8, x8, x21
        cmn x15, #0x1
        ldp x5, x4, [x0]
        ldp x21, x12, [x0, #16]
        adcs x22, x20, x23
        eor x23, x22, x9
        adcs x17, x2, x17
        adc x22, x8, x15
        cmn x9, #0x1
        adcs x15, x7, x5
        ldp x10, x14, [x0, #32]
        eor x1, x6, x9
        lsl x2, x15, #32
        adcs x8, x11, x4
        adcs x13, x1, x21
        eor x1, x22, x9
        adcs x24, x23, x12
        eor x11, x17, x9
        adcs x23, x11, x10
        adcs x7, x1, x14
        adcs x17, x9, x19
        adcs x20, x9, xzr
        add x1, x2, x15
        lsr x3, x1, #32
        adcs x11, x9, xzr
        adc x9, x9, xzr
        subs x3, x3, x1
        sbc x6, x1, xzr
        adds x24, x24, x5
        adcs x4, x23, x4
        extr x3, x6, x3, #32
        lsr x6, x6, #32
        adcs x21, x7, x21
        adcs x15, x17, x12
        adcs x7, x20, x10
        adcs x20, x11, x14
        mov x14, #0xffffffff
        adc x22, x9, x19
        adds x12, x6, x1
        adc x10, xzr, xzr
        subs x3, x8, x3
        sbcs x12, x13, x12
        lsl x9, x3, #32
        add x3, x9, x3
        sbcs x10, x24, x10
        sbcs x24, x4, xzr
        lsr x9, x3, #32
        sbcs x21, x21, xzr
        sbc x1, x1, xzr
        subs x9, x9, x3
        sbc x13, x3, xzr
        extr x9, x13, x9, #32
        lsr x13, x13, #32
        adds x13, x13, x3
        adc x6, xzr, xzr
        subs x12, x12, x9
        sbcs x17, x10, x13
        lsl x2, x12, #32
        sbcs x10, x24, x6
        add x9, x2, x12
        sbcs x6, x21, xzr
        lsr x5, x9, #32
        sbcs x21, x1, xzr
        sbc x13, x3, xzr
        subs x8, x5, x9
        sbc x19, x9, xzr
        lsr x12, x19, #32
        extr x3, x19, x8, #32
        adds x8, x12, x9
        adc x1, xzr, xzr
        subs x2, x17, x3
        sbcs x12, x10, x8
        sbcs x5, x6, x1
        sbcs x3, x21, xzr
        sbcs x19, x13, xzr
        sbc x24, x9, xzr
        adds x23, x15, x3
        adcs x8, x7, x19
        adcs x11, x20, x24
        adc x9, x22, xzr
        add x24, x9, #0x1
        lsl x7, x24, #32
        subs x21, x24, x7
        sbc x10, x7, xzr
        adds x6, x2, x21
        adcs x7, x12, x10
        adcs x24, x5, x24
        adcs x13, x23, xzr
        adcs x8, x8, xzr
        adcs x15, x11, xzr
        csetm x23, cc
        and x11, x16, x23
        and x20, x14, x23
        adds x22, x6, x20
        eor x3, x20, x23
        adcs x5, x7, x3
        adcs x14, x24, x11
        stp x22, x5, [x0]
        adcs x5, x13, x23
        adcs x21, x8, x23
        stp x14, x5, [x0, #16]
        adc x12, x15, x23
        stp x21, x12, [x0, #32]
        ldp     x23, x24, [sp]
        ldp     x21, x22, [sp, 16]
        ldp     x19, x20, [sp, 32]
        add sp, sp, 48
        ret

// Corresponds exactly to bignum_montsqr_p384

.montsqr_p384:
        ldr q1, [x1]
        ldp x9, x2, [x1]
        ldr q0, [x1]
        ldp x4, x6, [x1, #16]
        rev64 v21.4S, v1.4S
        uzp2 v28.4S, v1.4S, v1.4S
        umulh x7, x9, x2
        xtn v17.2S, v1.2D
        mul v27.4S, v21.4S, v0.4S
        ldr q20, [x1, #32]
        xtn v30.2S, v0.2D
        ldr q1, [x1, #32]
        uzp2 v31.4S, v0.4S, v0.4S
        ldp x5, x10, [x1, #32]
        umulh x8, x9, x4
        uaddlp v3.2D, v27.4S
        umull v16.2D, v30.2S, v17.2S
        mul x16, x9, x4
        umull v27.2D, v30.2S, v28.2S
        shrn v0.2S, v20.2D, #32
        xtn v7.2S, v20.2D
        shl v20.2D, v3.2D, #32
        umull v3.2D, v31.2S, v28.2S
        mul x3, x2, x4
        umlal v20.2D, v30.2S, v17.2S
        umull v22.2D, v7.2S, v0.2S
        usra v27.2D, v16.2D, #32
        umulh x11, x2, x4
        movi v21.2D, #0x00000000ffffffff
        uzp2 v28.4S, v1.4S, v1.4S
        adds x15, x16, x7
        and v5.16B, v27.16B, v21.16B
        adcs x3, x3, x8
        usra v3.2D, v27.2D, #32
        dup v29.2D, x6
        adcs x16, x11, xzr
        mov x14, v20.d[0]
        umlal v5.2D, v31.2S, v17.2S
        mul x8, x9, x2
        mov x7, v20.d[1]
        shl v19.2D, v22.2D, #33
        xtn v25.2S, v29.2D
        rev64 v31.4S, v1.4S
        lsl x13, x14, #32
        uzp2 v6.4S, v29.4S, v29.4S
        umlal v19.2D, v7.2S, v7.2S
        usra v3.2D, v5.2D, #32
        adds x1, x8, x8
        umulh x8, x4, x4
        add x12, x13, x14
        mul v17.4S, v31.4S, v29.4S
        xtn v4.2S, v1.2D
        adcs x14, x15, x15
        lsr x13, x12, #32
        adcs x15, x3, x3
        umull v31.2D, v25.2S, v28.2S
        adcs x11, x16, x16
        umull v21.2D, v25.2S, v4.2S
        mov x17, v3.d[0]
        umull v18.2D, v6.2S, v28.2S
        adc x16, x8, xzr
        uaddlp v16.2D, v17.4S
        movi v1.2D, #0x00000000ffffffff
        subs x13, x13, x12
        usra v31.2D, v21.2D, #32
        sbc x8, x12, xzr
        adds x17, x17, x1
        mul x1, x4, x4
        shl v28.2D, v16.2D, #32
        mov x3, v3.d[1]
        adcs x14, x7, x14
        extr x7, x8, x13, #32
        adcs x13, x3, x15
        and v3.16B, v31.16B, v1.16B
        adcs x11, x1, x11
        lsr x1, x8, #32
        umlal v3.2D, v6.2S, v4.2S
        usra v18.2D, v31.2D, #32
        adc x3, x16, xzr
        adds x1, x1, x12
        umlal v28.2D, v25.2S, v4.2S
        adc x16, xzr, xzr
        subs x15, x17, x7
        sbcs x7, x14, x1
        lsl x1, x15, #32
        sbcs x16, x13, x16
        add x8, x1, x15
        usra v18.2D, v3.2D, #32
        sbcs x14, x11, xzr
        lsr x1, x8, #32
        sbcs x17, x3, xzr
        sbc x11, x12, xzr
        subs x13, x1, x8
        umulh x12, x4, x10
        sbc x1, x8, xzr
        extr x13, x1, x13, #32
        lsr x1, x1, #32
        adds x15, x1, x8
        adc x1, xzr, xzr
        subs x7, x7, x13
        sbcs x13, x16, x15
        lsl x3, x7, #32
        umulh x16, x2, x5
        sbcs x15, x14, x1
        add x7, x3, x7
        sbcs x3, x17, xzr
        lsr x1, x7, #32
        sbcs x14, x11, xzr
        sbc x11, x8, xzr
        subs x8, x1, x7
        sbc x1, x7, xzr
        extr x8, x1, x8, #32
        lsr x1, x1, #32
        adds x1, x1, x7
        adc x17, xzr, xzr
        subs x13, x13, x8
        umulh x8, x9, x6
        sbcs x1, x15, x1
        sbcs x15, x3, x17
        sbcs x3, x14, xzr
        mul x17, x2, x5
        sbcs x11, x11, xzr
        stp x13, x1, [x0]
        sbc x14, x7, xzr
        mul x7, x4, x10
        subs x1, x9, x2
        stp x15, x3, [x0, #16]
        csetm x15, cc
        cneg x1, x1, cc
        stp x11, x14, [x0, #32]
        mul x14, x9, x6
        adds x17, x8, x17
        adcs x7, x16, x7
        adc x13, x12, xzr
        subs x12, x5, x6
        cneg x3, x12, cc
        cinv x16, x15, cc
        mul x8, x1, x3
        umulh x1, x1, x3
        eor x12, x8, x16
        adds x11, x17, x14
        adcs x3, x7, x17
        adcs x15, x13, x7
        adc x8, x13, xzr
        adds x3, x3, x14
        adcs x15, x15, x17
        adcs x17, x8, x7
        eor x1, x1, x16
        adc x13, x13, xzr
        subs x9, x9, x4
        csetm x8, cc
        cneg x9, x9, cc
        subs x4, x2, x4
        cneg x4, x4, cc
        csetm x7, cc
        subs x2, x10, x6
        cinv x8, x8, cc
        cneg x2, x2, cc
        cmn x16, #0x1
        adcs x11, x11, x12
        mul x12, x9, x2
        adcs x3, x3, x1
        adcs x15, x15, x16
        umulh x9, x9, x2
        adcs x17, x17, x16
        adc x13, x13, x16
        subs x1, x10, x5
        cinv x2, x7, cc
        cneg x1, x1, cc
        eor x9, x9, x8
        cmn x8, #0x1
        eor x7, x12, x8
        mul x12, x4, x1
        adcs x3, x3, x7
        adcs x7, x15, x9
        adcs x15, x17, x8
        ldp x9, x17, [x0, #16]
        umulh x4, x4, x1
        adc x8, x13, x8
        cmn x2, #0x1
        eor x1, x12, x2
        adcs x1, x7, x1
        ldp x7, x16, [x0]
        eor x12, x4, x2
        adcs x4, x15, x12
        ldp x15, x12, [x0, #32]
        adc x8, x8, x2
        adds x13, x14, x14
        umulh x14, x5, x10
        adcs x2, x11, x11
        adcs x3, x3, x3
        adcs x1, x1, x1
        adcs x4, x4, x4
        adcs x11, x8, x8
        adc x8, xzr, xzr
        adds x13, x13, x7
        adcs x2, x2, x16
        mul x16, x5, x10
        adcs x3, x3, x9
        adcs x1, x1, x17
        umulh x5, x5, x5
        lsl x9, x13, #32
        add x9, x9, x13
        adcs x4, x4, x15
        mov x13, v28.d[1]
        adcs x15, x11, x12
        lsr x7, x9, #32
        adc x11, x8, xzr
        subs x7, x7, x9
        umulh x10, x10, x10
        sbc x17, x9, xzr
        extr x7, x17, x7, #32
        lsr x17, x17, #32
        adds x17, x17, x9
        adc x12, xzr, xzr
        subs x8, x2, x7
        sbcs x17, x3, x17
        lsl x7, x8, #32
        sbcs x2, x1, x12
        add x3, x7, x8
        sbcs x12, x4, xzr
        lsr x1, x3, #32
        sbcs x7, x15, xzr
        sbc x15, x9, xzr
        subs x1, x1, x3
        sbc x4, x3, xzr
        lsr x9, x4, #32
        extr x8, x4, x1, #32
        adds x9, x9, x3
        adc x4, xzr, xzr
        subs x1, x17, x8
        lsl x17, x1, #32
        sbcs x8, x2, x9
        sbcs x9, x12, x4
        add x17, x17, x1
        mov x1, v18.d[1]
        lsr x2, x17, #32
        sbcs x7, x7, xzr
        mov x12, v18.d[0]
        sbcs x15, x15, xzr
        sbc x3, x3, xzr
        subs x4, x2, x17
        sbc x2, x17, xzr
        adds x12, x13, x12
        adcs x16, x16, x1
        lsr x13, x2, #32
        extr x1, x2, x4, #32
        adc x2, x14, xzr
        adds x4, x13, x17
        mul x13, x6, x6
        adc x14, xzr, xzr
        subs x1, x8, x1
        sbcs x4, x9, x4
        mov x9, v28.d[0]
        sbcs x7, x7, x14
        sbcs x8, x15, xzr
        sbcs x3, x3, xzr
        sbc x14, x17, xzr
        adds x17, x9, x9
        adcs x12, x12, x12
        mov x15, v19.d[0]
        adcs x9, x16, x16
        umulh x6, x6, x6
        adcs x16, x2, x2
        adc x2, xzr, xzr
        adds x11, x11, x8
        adcs x3, x3, xzr
        adcs x14, x14, xzr
        adcs x8, xzr, xzr
        adds x13, x1, x13
        mov x1, v19.d[1]
        adcs x6, x4, x6
        mov x4, #0xffffffff
        adcs x15, x7, x15
        adcs x7, x11, x5
        adcs x1, x3, x1
        adcs x14, x14, x10
        adc x11, x8, xzr
        adds x6, x6, x17
        adcs x8, x15, x12
        adcs x3, x7, x9
        adcs x15, x1, x16
        mov x16, #0xffffffff00000001
        adcs x14, x14, x2
        mov x2, #0x1
        adc x17, x11, xzr
        cmn x13, x16
        adcs xzr, x6, x4
        adcs xzr, x8, x2
        adcs xzr, x3, xzr
        adcs xzr, x15, xzr
        adcs xzr, x14, xzr
        adc x1, x17, xzr
        neg x9, x1
        and x1, x16, x9
        adds x11, x13, x1
        and x13, x4, x9
        adcs x5, x6, x13
        and x1, x2, x9
        adcs x7, x8, x1
        stp x11, x5, [x0]
        adcs x11, x3, xzr
        adcs x2, x15, xzr
        stp x7, x11, [x0, #16]
        adc x17, x14, xzr
        stp x2, x17, [x0, #32]
        ret

// Corresponds exactly to bignum_sub_p384

.sub_p384:
        ldp     x5, x6, [x1]
        ldp     x4, x3, [x2]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x1, #32]
        ldp     x4, x3, [x2, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, cc
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [x0]
        stp     x7, x8, [x0, #16]
        stp     x9, x10, [x0, #32]
        ret


#define montmul_p384(P0,P1,P2) \
        add x0, P0;\
        add x1, P1;\
        add x2, P2;\
        bl .montmul_p384

#define montsqr_p384(P0,P1) \
        add x0, P0;\
        add x1, P1;\
        bl .montsqr_p384

#define sub_p384(P0,P1,P2) \
        add x0, P0;\
        add x1, P1;\
        add x2, P2;\
        bl .sub_p384


S2N_BN_SYMBOL(p384_montjadd):

// Save regs and make room on stack for temporary variables

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x24, [sp, #-16]!
        stp     x25, x26, [sp, #-16]!
        stp     x30, xzr, [sp, #-16]!
        sub     sp, sp, NSPACE

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Main code, just a sequence of basic field operations
// 8 * multiply + 3 * square + 7 * subtract

        montsqr_p384(z1sq,z_1)
        montsqr_p384(z2sq,z_2)

        montmul_p384(y1a,z_2,y_1)
        montmul_p384(y2a,z_1,y_2)

        montmul_p384(x2a,z1sq,x_2)
        montmul_p384(x1a,z2sq,x_1)
        montmul_p384(y2a,z1sq,y2a)
        montmul_p384(y1a,z2sq,y1a)

        sub_p384(xd,x2a,x1a)
        sub_p384(yd,y2a,y1a)

        montsqr_p384(zz,xd)
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

        add     sp, sp, NSPACE

        ldp     x30, xzr, [sp], 16
        ldp     x25, x26, [sp], 16
        ldp     x23, x24, [sp], 16
        ldp     x21, x22, [sp], 16
        ldp     x19, x20, [sp], 16

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
