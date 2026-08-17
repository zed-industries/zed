// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjadd(uint64_t p3[static 12], const uint64_t p1[static 12],
//                              const uint64_t p2[static 12]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// This is functionally equivalent to p256_montjadd in unopt/p256_montjadd.S.
// This is the result of doing the following sequence of optimizations:
//   1. Function inlining
//   2. Eliminating redundant load/store instructions
//   3. Folding (add addr, const) + load/store
// Function inlining is done manually. The second and third optimizations are
// done by a script.

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjadd)

        .text
        .balign 4

#define NUMSIZE 32
#define NSPACE NUMSIZE*7

S2N_BN_SYMBOL(p256_montjadd):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x30)
        CFI_DEC_SP(NSPACE)

        mov x21, x0
        mov x22, x1
        mov x23, x2
        mov x0, sp
        ldr q19, [x22, #64]
        ldp x9, x13, [x22, #64]
        ldr q23, [x22, #80]
        ldr q0, [x22, #64]
        ldp x1, x10, [x22, #80]
        uzp2 v29.4s, v19.4s, v19.4s
        xtn v4.2s, v19.2d
        umulh x8, x9, x13
        rev64 v20.4s, v23.4s
        umull v16.2d, v19.2s, v19.2s
        umull v1.2d, v29.2s, v4.2s
        mul v20.4s, v20.4s, v0.4s
        subs x14, x9, x13
        umulh x15, x9, x1
        mov x16, v16.d[1]
        umull2 v4.2d, v19.4s, v19.4s
        mov x4, v16.d[0]
        uzp1 v17.4s, v23.4s, v0.4s
        uaddlp v19.2d, v20.4s
        lsr x7, x8, #63
        mul x11, x9, x13
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x10, x1
        mov x9, v1.d[1]
        cneg x17, x2, cc
        cinv x2, x5, cc
        adds x5, x4, x12, lsl #33
        extr x4, x8, x11, #63
        lsr x8, x12, #31
        uzp1 v20.4s, v0.4s, v0.4s
        shl v19.2d, v19.2d, #32
        adc x16, x16, x8
        adds x8, x14, x9, lsl #33
        lsr x14, x9, #31
        lsl x9, x5, #32
        umlal v19.2d, v20.2s, v17.2s
        adc x14, x3, x14
        adds x16, x16, x11, lsl #1
        lsr x3, x5, #32
        umulh x12, x6, x17
        adcs x4, x8, x4
        adc x11, x14, x7
        subs x8, x5, x9
        sbc x5, x5, x3
        adds x16, x16, x9
        mov x14, v19.d[0]
        mul x17, x6, x17
        adcs x3, x4, x3
        lsl x7, x16, #32
        umulh x13, x13, x10
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x1, x10
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x1, x10
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x10, x10
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x10, x10
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x1, x1
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x1, x1
        adcs x2, x2, x9
        mov x6, #0xffffffff
        adc x15, x15, xzr
        adds x8, x4, x4
        adcs x1, x11, x11
        mov x11, #0xffffffff00000001
        adc x4, xzr, xzr
        subs x9, x14, x5
        sbc x14, x14, x7
        adds x12, x12, x5
        adcs x16, x16, x7
        lsl x5, x12, #32
        lsr x7, x12, #32
        adcs x2, x2, x9
        adcs x14, x15, x14
        adc x15, xzr, xzr
        subs x9, x12, x5
        sbc x12, x12, x7
        adds x16, x16, x5
        adcs x2, x2, x7
        adcs x14, x14, x9
        adcs x12, x15, x12
        adc x15, xzr, xzr
        adds x16, x16, x3
        adcs x2, x2, x13
        adcs x14, x14, x17
        adcs x12, x12, x10
        adc x15, x15, xzr
        adds x2, x2, x8
        adcs x14, x14, x1
        adcs x12, x12, x4
        adcs x15, x15, xzr
        adds x3, x16, #0x1
        sbcs x5, x2, x6
        sbcs x8, x14, xzr
        sbcs x11, x12, x11
        sbcs xzr, x15, xzr
        csel x19, x3, x16, cs
        csel x14, x8, x14, cs
        csel x12, x11, x12, cs
        csel x20, x5, x2, cs
        stp x14, x12, [x0, #16]
        stp x19, x20, [x0]
        ldr q19, [x23, #64]
        ldp x9, x13, [x23, #64]
        ldr q23, [x23, #80]
        ldr q0, [x23, #64]
        ldp x1, x10, [x23, #80]
        uzp2 v29.4s, v19.4s, v19.4s
        xtn v4.2s, v19.2d
        umulh x8, x9, x13
        rev64 v20.4s, v23.4s
        umull v16.2d, v19.2s, v19.2s
        umull v1.2d, v29.2s, v4.2s
        mul v20.4s, v20.4s, v0.4s
        subs x14, x9, x13
        umulh x15, x9, x1
        mov x16, v16.d[1]
        umull2 v4.2d, v19.4s, v19.4s
        mov x4, v16.d[0]
        uzp1 v17.4s, v23.4s, v0.4s
        uaddlp v19.2d, v20.4s
        lsr x7, x8, #63
        mul x11, x9, x13
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x10, x1
        mov x9, v1.d[1]
        cneg x17, x2, cc
        cinv x2, x5, cc
        adds x5, x4, x12, lsl #33
        extr x4, x8, x11, #63
        lsr x8, x12, #31
        uzp1 v20.4s, v0.4s, v0.4s
        shl v19.2d, v19.2d, #32
        adc x16, x16, x8
        adds x8, x14, x9, lsl #33
        lsr x14, x9, #31
        lsl x9, x5, #32
        umlal v19.2d, v20.2s, v17.2s
        adc x14, x3, x14
        adds x16, x16, x11, lsl #1
        lsr x3, x5, #32
        umulh x12, x6, x17
        adcs x4, x8, x4
        adc x11, x14, x7
        subs x8, x5, x9
        sbc x5, x5, x3
        adds x16, x16, x9
        mov x14, v19.d[0]
        mul x17, x6, x17
        adcs x3, x4, x3
        lsl x7, x16, #32
        umulh x13, x13, x10
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x1, x10
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x1, x10
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x10, x10
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x10, x10
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x1, x1
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x1, x1
        adcs x2, x2, x9
        mov x6, #0xffffffff
        adc x15, x15, xzr
        adds x8, x4, x4
        adcs x1, x11, x11
        mov x11, #0xffffffff00000001
        adc x4, xzr, xzr
        subs x9, x14, x5
        sbc x14, x14, x7
        adds x12, x12, x5
        adcs x16, x16, x7
        lsl x5, x12, #32
        lsr x7, x12, #32
        adcs x2, x2, x9
        adcs x14, x15, x14
        adc x15, xzr, xzr
        subs x9, x12, x5
        sbc x12, x12, x7
        adds x16, x16, x5
        adcs x2, x2, x7
        adcs x14, x14, x9
        adcs x12, x15, x12
        adc x15, xzr, xzr
        adds x16, x16, x3
        adcs x2, x2, x13
        adcs x14, x14, x17
        adcs x12, x12, x10
        adc x15, x15, xzr
        adds x2, x2, x8
        adcs x14, x14, x1
        adcs x12, x12, x4
        adcs x15, x15, xzr
        adds x3, x16, #0x1
        sbcs x5, x2, x6
        sbcs x8, x14, xzr
        sbcs x11, x12, x11
        sbcs xzr, x15, xzr
        csel x16, x3, x16, cs
        csel x14, x8, x14, cs
        csel x12, x11, x12, cs
        csel x2, x5, x2, cs
        stp x14, x12, [sp, #176]
        stp x16, x2, [sp, #160]
        ldr q20, [x22, #32]
        ldp x7, x17, [x23, #64]
        ldr q0, [x23, #64]
        ldp x6, x10, [x22, #32]
        ldp x11, x15, [x23, #80]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [x22, #48]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [x23, #80]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x22, #48]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [sp, #192]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #208]
        ldr q20, [x23, #32]
        ldp x7, x17, [x22, #64]
        ldr q0, [x22, #64]
        ldp x6, x10, [x23, #32]
        ldp x11, x15, [x22, #80]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [x23, #48]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [x22, #80]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x23, #48]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x24, x11, x13
        and x1, x1, x13
        adcs x25, x4, x1
        and x1, x12, x13
        stp x24, x25, [sp, #32]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #48]
        mov x1, sp
        ldr q20, [x23, #0]
        ldr q0, [x1]
        ldp x6, x10, [x23, #0]
        ldp x11, x15, [x1, #16]
        rev64 v16.4s, v20.4s
        subs x4, x19, x20
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x20, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x19
        ldr q20, [x23, #16]
        sbcs x5, x15, x20
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x19, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [x1, #16]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x23, #16]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [sp, #64]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #80]
        ldr q20, [x22, #0]
        ldp x7, x17, [sp, #160]
        ldr q0, [sp, #160]
        ldp x6, x10, [x22, #0]
        ldp x11, x15, [sp, #176]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [x22, #16]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #176]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x22, #16]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [sp, #128]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #144]
        mov x1, sp
        ldr q20, [sp, #32]
        ldp x7, x17, [x1]
        ldr q0, [x1]
        ldp x11, x15, [x1, #16]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x25
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [sp, #48]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x24
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x25, x24
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [x1, #16]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #48]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x24, x7
        sbcs x9, x25, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x24, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x25, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x24
        eor x1, x10, x5
        adcs x16, x2, x25
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x19, x11, x13
        and x1, x1, x13
        adcs x20, x4, x1
        and x1, x12, x13
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #48]
        ldr q20, [sp, #192]
        ldp x7, x17, [sp, #160]
        ldr q0, [sp, #160]
        ldp x6, x10, [sp, #192]
        ldp x11, x15, [sp, #176]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [sp, #208]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #176]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #208]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x9, x11, x13
        and x1, x1, x13
        adcs x10, x4, x1
        and x1, x12, x13
        stp x9, x10, [sp, #192]
        adcs x11, x7, xzr
        adc x12, x17, x1
        stp x11, x12, [sp, #208]
        ldp x5, x6, [sp, #64]
        ldp x4, x3, [sp, #128]
        subs x5, x5, x4
        sbcs x6, x6, x3
        ldp x7, x8, [sp, #80]
        ldp x4, x3, [sp, #144]
        sbcs x7, x7, x4
        sbcs x8, x8, x3
        csetm x3, cc
        adds x13, x5, x3
        and x4, x3, #0xffffffff
        adcs x24, x6, x4
        adcs x25, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x26, x8, x4
        stp x13, x24, [sp, #160]
        stp x25, x26, [sp, #176]
        subs x5, x19, x9
        sbcs x6, x20, x10
        ldp x7, x8, [sp, #48]
        sbcs x7, x7, x11
        sbcs x8, x8, x12
        csetm x3, cc
        adds x19, x5, x3
        and x4, x3, #0xffffffff
        adcs x20, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x19, x20, [sp, #32]
        stp x7, x8, [sp, #48]
        ldr q19, [sp, #160]
        ldr q23, [sp, #176]
        ldr q0, [sp, #160]
        uzp2 v29.4s, v19.4s, v19.4s
        xtn v4.2s, v19.2d
        umulh x8, x13, x24
        rev64 v20.4s, v23.4s
        umull v16.2d, v19.2s, v19.2s
        umull v1.2d, v29.2s, v4.2s
        mul v20.4s, v20.4s, v0.4s
        subs x14, x13, x24
        umulh x15, x13, x25
        mov x16, v16.d[1]
        umull2 v4.2d, v19.4s, v19.4s
        mov x4, v16.d[0]
        uzp1 v17.4s, v23.4s, v0.4s
        uaddlp v19.2d, v20.4s
        lsr x7, x8, #63
        mul x11, x13, x24
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x26, x25
        mov x9, v1.d[1]
        cneg x17, x2, cc
        cinv x2, x5, cc
        adds x5, x4, x12, lsl #33
        extr x4, x8, x11, #63
        lsr x8, x12, #31
        uzp1 v20.4s, v0.4s, v0.4s
        shl v19.2d, v19.2d, #32
        adc x16, x16, x8
        adds x8, x14, x9, lsl #33
        lsr x14, x9, #31
        lsl x9, x5, #32
        umlal v19.2d, v20.2s, v17.2s
        adc x14, x3, x14
        adds x16, x16, x11, lsl #1
        lsr x3, x5, #32
        umulh x12, x6, x17
        adcs x4, x8, x4
        adc x11, x14, x7
        subs x8, x5, x9
        sbc x5, x5, x3
        adds x16, x16, x9
        mov x14, v19.d[0]
        mul x17, x6, x17
        adcs x3, x4, x3
        lsl x7, x16, #32
        umulh x13, x24, x26
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x25, x26
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x25, x26
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x26, x26
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x26, x26
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x25, x25
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x25, x25
        adcs x2, x2, x9
        mov x6, #0xffffffff
        adc x15, x15, xzr
        adds x8, x4, x4
        adcs x1, x11, x11
        mov x11, #0xffffffff00000001
        adc x4, xzr, xzr
        subs x9, x14, x5
        sbc x14, x14, x7
        adds x12, x12, x5
        adcs x16, x16, x7
        lsl x5, x12, #32
        lsr x7, x12, #32
        adcs x2, x2, x9
        adcs x14, x15, x14
        adc x15, xzr, xzr
        subs x9, x12, x5
        sbc x12, x12, x7
        adds x16, x16, x5
        adcs x2, x2, x7
        adcs x14, x14, x9
        adcs x12, x15, x12
        adc x15, xzr, xzr
        adds x16, x16, x3
        adcs x2, x2, x13
        adcs x14, x14, x17
        adcs x12, x12, x10
        adc x15, x15, xzr
        adds x2, x2, x8
        adcs x14, x14, x1
        adcs x12, x12, x4
        adcs x15, x15, xzr
        adds x3, x16, #0x1
        sbcs x5, x2, x6
        sbcs x8, x14, xzr
        sbcs x11, x12, x11
        sbcs xzr, x15, xzr
        csel x24, x3, x16, cs
        csel x25, x8, x14, cs
        csel x26, x11, x12, cs
        csel x27, x5, x2, cs
        stp x25, x26, [sp, #112]
        stp x24, x27, [sp, #96]
        mov x0, sp
        ldr q19, [sp, #32]
        ldr q23, [sp, #48]
        ldr q0, [sp, #32]
        ldp x1, x10, [sp, #48]
        uzp2 v29.4s, v19.4s, v19.4s
        xtn v4.2s, v19.2d
        umulh x8, x19, x20
        rev64 v20.4s, v23.4s
        umull v16.2d, v19.2s, v19.2s
        umull v1.2d, v29.2s, v4.2s
        mul v20.4s, v20.4s, v0.4s
        subs x14, x19, x20
        umulh x15, x19, x1
        mov x16, v16.d[1]
        umull2 v4.2d, v19.4s, v19.4s
        mov x4, v16.d[0]
        uzp1 v17.4s, v23.4s, v0.4s
        uaddlp v19.2d, v20.4s
        lsr x7, x8, #63
        mul x11, x19, x20
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x10, x1
        mov x9, v1.d[1]
        cneg x17, x2, cc
        cinv x2, x5, cc
        adds x5, x4, x12, lsl #33
        extr x4, x8, x11, #63
        lsr x8, x12, #31
        uzp1 v20.4s, v0.4s, v0.4s
        shl v19.2d, v19.2d, #32
        adc x16, x16, x8
        adds x8, x14, x9, lsl #33
        lsr x14, x9, #31
        lsl x9, x5, #32
        umlal v19.2d, v20.2s, v17.2s
        adc x14, x3, x14
        adds x16, x16, x11, lsl #1
        lsr x3, x5, #32
        umulh x12, x6, x17
        adcs x4, x8, x4
        adc x11, x14, x7
        subs x8, x5, x9
        sbc x5, x5, x3
        adds x16, x16, x9
        mov x14, v19.d[0]
        mul x17, x6, x17
        adcs x3, x4, x3
        lsl x7, x16, #32
        umulh x13, x20, x10
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x1, x10
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x1, x10
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x10, x10
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x10, x10
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x1, x1
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x1, x1
        adcs x2, x2, x9
        mov x6, #0xffffffff
        adc x15, x15, xzr
        adds x8, x4, x4
        adcs x1, x11, x11
        mov x11, #0xffffffff00000001
        adc x4, xzr, xzr
        subs x9, x14, x5
        sbc x14, x14, x7
        adds x12, x12, x5
        adcs x16, x16, x7
        lsl x5, x12, #32
        lsr x7, x12, #32
        adcs x2, x2, x9
        adcs x14, x15, x14
        adc x15, xzr, xzr
        subs x9, x12, x5
        sbc x12, x12, x7
        adds x16, x16, x5
        adcs x2, x2, x7
        adcs x14, x14, x9
        adcs x12, x15, x12
        adc x15, xzr, xzr
        adds x16, x16, x3
        adcs x2, x2, x13
        adcs x14, x14, x17
        adcs x12, x12, x10
        adc x15, x15, xzr
        adds x2, x2, x8
        adcs x14, x14, x1
        adcs x12, x12, x4
        adcs x15, x15, xzr
        adds x3, x16, #0x1
        sbcs x5, x2, x6
        sbcs x8, x14, xzr
        sbcs x11, x12, x11
        sbcs xzr, x15, xzr
        csel x16, x3, x16, cs
        csel x14, x8, x14, cs
        csel x12, x11, x12, cs
        csel x2, x5, x2, cs
        stp x14, x12, [x0, #16]
        stp x16, x2, [x0]
        ldr q20, [sp, #128]
        ldr q0, [sp, #96]
        ldp x6, x10, [sp, #128]
        rev64 v16.4s, v20.4s
        subs x4, x24, x27
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x27, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x25, x24
        ldr q20, [sp, #144]
        sbcs x5, x26, x27
        ngc x17, xzr
        subs x8, x25, x26
        uaddlp v27.2d, v16.4s
        umulh x4, x24, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #112]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #144]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x25, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x26, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x25
        eor x1, x10, x5
        adcs x16, x2, x26
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x19, x11, x13
        and x1, x1, x13
        adcs x20, x4, x1
        and x1, x12, x13
        stp x19, x20, [sp, #128]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #144]
        ldr q20, [sp, #64]
        ldr q0, [sp, #96]
        ldp x6, x10, [sp, #64]
        ldp x11, x15, [sp, #112]
        rev64 v16.4s, v20.4s
        subs x4, x24, x27
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x27, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x24
        ldr q20, [sp, #80]
        sbcs x5, x15, x27
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x24, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #112]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #80]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x9, x11, x13
        and x1, x1, x13
        adcs x10, x4, x1
        and x1, x12, x13
        stp x9, x10, [sp, #64]
        adcs x11, x7, xzr
        adc x12, x17, x1
        stp x11, x12, [sp, #80]
        mov x0, sp
        mov x1, sp
        ldp x5, x6, [x1]
        subs x5, x5, x19
        sbcs x6, x6, x20
        ldp x7, x8, [x1, #16]
        ldp x4, x3, [sp, #144]
        sbcs x7, x7, x4
        sbcs x8, x8, x3
        csetm x3, cc
        adds x24, x5, x3
        and x4, x3, #0xffffffff
        adcs x25, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x7, x8, [x0, #16]
        subs x5, x9, x19
        sbcs x6, x10, x20
        ldp x4, x3, [sp, #144]
        sbcs x7, x11, x4
        sbcs x8, x12, x3
        csetm x3, cc
        adds x5, x5, x3
        and x4, x3, #0xffffffff
        adcs x6, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x5, x6, [sp, #96]
        stp x7, x8, [sp, #112]
        ldr q20, [x22, #64]
        ldp x7, x17, [sp, #160]
        ldr q0, [sp, #160]
        ldp x6, x10, [x22, #64]
        ldp x11, x15, [sp, #176]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [x22, #80]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #176]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x22, #80]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [sp, #160]
        adcs x19, x7, xzr
        adc x20, x17, x1
        stp x19, x20, [sp, #176]
        mov x0, sp
        mov x1, sp
        ldp x4, x3, [sp, #64]
        subs x5, x24, x4
        sbcs x6, x25, x3
        ldp x7, x8, [x1, #16]
        ldp x4, x3, [sp, #80]
        sbcs x7, x7, x4
        sbcs x8, x8, x3
        csetm x3, cc
        adds x9, x5, x3
        and x4, x3, #0xffffffff
        adcs x10, x6, x4
        adcs x11, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x3, x8, x4
        stp x9, x10, [x0]
        stp x11, x3, [x0, #16]
        ldp x5, x6, [sp, #128]
        subs x5, x5, x9
        sbcs x6, x6, x10
        ldp x7, x8, [sp, #144]
        sbcs x7, x7, x11
        sbcs x8, x8, x3
        csetm x3, cc
        adds x5, x5, x3
        and x4, x3, #0xffffffff
        adcs x6, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x5, x6, [sp, #128]
        stp x7, x8, [sp, #144]
        ldr q20, [sp, #192]
        ldp x7, x17, [sp, #96]
        ldr q0, [sp, #96]
        ldp x6, x10, [sp, #192]
        ldp x11, x15, [sp, #112]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [sp, #208]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #112]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #208]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [sp, #96]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #112]
        ldr q20, [x23, #64]
        ldp x7, x17, [sp, #160]
        ldr q0, [sp, #160]
        ldp x6, x10, [x23, #64]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x19, x7
        ldr q20, [x23, #80]
        sbcs x5, x20, x17
        ngc x17, xzr
        subs x8, x19, x20
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #176]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x23, #80]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x19, x3, x13
        adcs x20, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x24, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x25, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x19
        adcs x15, x16, x20
        eor x5, x17, x4
        adcs x9, x1, x24
        eor x1, x10, x5
        adcs x16, x2, x25
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x19, x11, x13
        and x1, x1, x13
        adcs x20, x4, x1
        and x1, x12, x13
        stp x19, x20, [sp, #160]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #176]
        ldr q20, [sp, #128]
        ldp x7, x17, [sp, #32]
        ldr q0, [sp, #32]
        ldp x6, x10, [sp, #128]
        ldp x11, x15, [sp, #48]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [sp, #144]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2d, v16.4s
        umulh x4, x7, x6
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #48]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2s, v20.2d
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4s, v20.4s, v20.4s
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2s, v28.2d
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [sp, #144]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x24, x3, x13
        adcs x25, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x26, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x27, x15, xzr
        rev64 v24.4s, v20.4s
        movi v2.2d, #0xffffffff
        mul x10, x14, x11
        mul v4.4s, v24.4s, v28.4s
        subs x13, x14, x5
        uzp2 v19.4s, v28.4s, v28.4s
        csetm x15, cc
        usra v3.2d, v21.2d, #32
        mul x7, x5, x1
        umull v21.2d, v19.2s, v16.2s
        cneg x13, x13, cc
        uaddlp v5.2d, v4.4s
        subs x11, x1, x11
        and v16.16b, v3.16b, v2.16b
        umulh x5, x5, x1
        shl v24.2d, v5.2d, #32
        cneg x11, x11, cc
        umlal v16.2d, v19.2s, v1.2s
        cinv x12, x15, cc
        umlal v24.2d, v0.2s, v1.2s
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        usra v21.2d, v3.2d, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2d, v16.2d, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x24
        adcs x15, x16, x25
        eor x5, x17, x4
        adcs x9, x1, x26
        eor x1, x10, x5
        adcs x16, x2, x27
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x5, x11, x13
        and x1, x1, x13
        adcs x6, x4, x1
        and x1, x12, x13
        adcs x7, x7, xzr
        adc x9, x17, x1
        ldp x4, x3, [sp, #96]
        subs x5, x5, x4
        sbcs x6, x6, x3
        ldp x4, x3, [sp, #112]
        sbcs x7, x7, x4
        sbcs x8, x9, x3
        csetm x3, cc
        adds x15, x5, x3
        and x4, x3, #0xffffffff
        adcs x24, x6, x4
        adcs x25, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x26, x8, x4
        stp x15, x24, [sp, #128]
        stp x25, x26, [sp, #144]
        ldp x0, x1, [x22, #64]
        ldp x2, x3, [x22, #80]
        orr x12, x0, x1
        orr x13, x2, x3
        orr x12, x12, x13
        cmp x12, xzr
        cset x12, ne
        ldp x4, x5, [x23, #64]
        ldp x6, x7, [x23, #80]
        orr x13, x4, x5
        orr x14, x6, x7
        orr x13, x13, x14
        cmp x13, xzr
        cset x13, ne
        cmp x13, x12
        csel x8, x0, x19, cc
        csel x9, x1, x20, cc
        csel x8, x4, x8, hi
        csel x9, x5, x9, hi
        ldp x10, x11, [sp, #176]
        csel x10, x2, x10, cc
        csel x11, x3, x11, cc
        csel x10, x6, x10, hi
        csel x11, x7, x11, hi
        ldp x12, x13, [x22]
        ldp x0, x1, [sp]
        csel x0, x12, x0, cc
        csel x1, x13, x1, cc
        ldp x12, x13, [x23]
        csel x0, x12, x0, hi
        csel x1, x13, x1, hi
        ldp x12, x13, [x22, #16]
        ldp x2, x3, [sp, #16]
        csel x2, x12, x2, cc
        csel x3, x13, x3, cc
        ldp x12, x13, [x23, #16]
        csel x2, x12, x2, hi
        csel x3, x13, x3, hi
        ldp x12, x13, [x22, #32]
        csel x4, x12, x15, cc
        csel x5, x13, x24, cc
        ldp x12, x13, [x23, #32]
        csel x4, x12, x4, hi
        csel x5, x13, x5, hi
        ldp x12, x13, [x22, #48]
        csel x6, x12, x25, cc
        csel x7, x13, x26, cc
        ldp x12, x13, [x23, #48]
        csel x6, x12, x6, hi
        csel x7, x13, x7, hi
        stp x0, x1, [x21]
        stp x2, x3, [x21, #16]
        stp x4, x5, [x21, #32]
        stp x6, x7, [x21, #48]
        stp x8, x9, [x21, #64]
        stp x10, x11, [x21, #80]

        CFI_INC_SP(NSPACE)
        CFI_POP2(x27,x30)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_montjadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
