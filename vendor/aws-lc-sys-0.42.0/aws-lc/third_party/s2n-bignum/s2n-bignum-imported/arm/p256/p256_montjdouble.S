// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjdouble(uint64_t p3[static 12],
//                                 const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// This is functionally equivalent to p256_montjdouble in unopt/p256_montjdouble.S.
// This is the result of doing the following sequence of optimizations:
//   1. Function inlining
//   2. Eliminating redundant load/store instructions
//   3. Folding (add addr, const) + load/store
// Function inlining is done manually. The second and third optimizations are
// done by a script.

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjdouble)
        .text
        .balign 4

#define NUMSIZE 32
#define NSPACE NUMSIZE*6

S2N_BN_SYMBOL(p256_montjdouble):
        CFI_START

        CFI_DEC_SP(NSPACE+80)
        CFI_STACKSAVE2(x19,x20,NSPACE)
        CFI_STACKSAVE2(x21,x22,NSPACE+16)
        CFI_STACKSAVE2(x23,x24,NSPACE+32)
        CFI_STACKSAVE2(x25,x26,NSPACE+48)
        CFI_STACKSAVE1Z(x27,NSPACE+64)

        mov x19, x0
        mov x20, x1
        mov x0, sp
        ldr q19, [x20, #64]
        ldp x9, x13, [x20, #64]
        ldr q23, [x20, #80]
        ldr q0, [x20, #64]
        ldp x1, x10, [x20, #80]
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
        csel x21, x3, x16, cs
        csel x22, x8, x14, cs
        csel x23, x11, x12, cs
        csel x24, x5, x2, cs
        stp x22, x23, [x0, #16]
        stp x21, x24, [x0]
        ldr q19, [x20, #32]
        ldp x9, x13, [x20, #32]
        ldr q23, [x20, #48]
        ldr q0, [x20, #32]
        ldp x1, x10, [x20, #48]
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
        stp x14, x12, [sp, #48]
        stp x16, x2, [sp, #32]
        ldp x5, x6, [x20, #0]
        subs x5, x5, x21
        sbcs x6, x6, x24
        ldp x7, x8, [x20, #16]
        sbcs x7, x7, x22
        sbcs x8, x8, x23
        csetm x3, cc
        adds x10, x5, x3
        and x4, x3, #0xffffffff
        adcs x25, x6, x4
        adcs x26, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x27, x8, x4
        stp x10, x25, [sp, #96]
        stp x26, x27, [sp, #112]
        ldp x5, x6, [x20]
        adds x5, x5, x21
        adcs x6, x6, x24
        ldp x7, x8, [x20, #16]
        adcs x7, x7, x22
        adcs x8, x8, x23
        csetm x3, cs
        subs x9, x5, x3
        and x1, x3, #0xffffffff
        sbcs x5, x6, x1
        sbcs x7, x7, xzr
        and x2, x3, #0xffffffff00000001
        sbc x8, x8, x2
        stp x9, x5, [sp, #64]
        stp x7, x8, [sp, #80]
        ldr q20, [sp, #96]
        ldr q0, [sp, #64]
        rev64 v16.4s, v20.4s
        subs x4, x9, x5
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x5, x25
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x7, x9
        ldr q20, [sp, #112]
        sbcs x5, x8, x5
        ngc x17, xzr
        subs x8, x7, x8
        uaddlp v27.2d, v16.4s
        umulh x4, x9, x10
        uzp1 v21.4s, v0.4s, v0.4s
        cneg x11, x8, cc
        shl v17.2d, v27.2d, #32
        csetm x15, cc
        subs x9, x25, x10
        eor x7, x14, x17
        umlal v17.2d, v21.2s, v28.2s
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [sp, #80]
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
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x10, x26
        sbcs x9, x25, x27
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x27, x26
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
        adds x21, x3, x13
        adcs x22, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x23, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x24, x15, xzr
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
        adds x7, x5, x21
        adcs x15, x16, x22
        eor x5, x17, x4
        adcs x9, x1, x23
        eor x1, x10, x5
        adcs x16, x2, x24
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
        adds x21, x11, x13
        and x1, x1, x13
        adcs x22, x4, x1
        and x1, x12, x13
        stp x21, x22, [sp, #96]
        adcs x23, x7, xzr
        adc x24, x17, x1
        stp x23, x24, [sp, #112]
        ldp x4, x5, [x20, #32]
        ldp x8, x9, [x20, #64]
        adds x4, x4, x8
        adcs x5, x5, x9
        ldp x6, x7, [x20, #48]
        ldp x10, x11, [x20, #80]
        adcs x6, x6, x10
        adcs x7, x7, x11
        adc x3, xzr, xzr
        adds x8, x4, #0x1
        mov x9, #0xffffffff
        sbcs x9, x5, x9
        sbcs x10, x6, xzr
        mov x11, #0xffffffff00000001
        sbcs x11, x7, x11
        sbcs x3, x3, xzr
        csel x4, x4, x8, cc
        csel x5, x5, x9, cc
        csel x6, x6, x10, cc
        csel x7, x7, x11, cc
        stp x4, x5, [sp, #64]
        stp x6, x7, [sp, #80]
        ldr q20, [sp, #32]
        ldp x7, x17, [x20, #0]
        ldr q0, [x20, #0]
        ldp x6, x10, [sp, #32]
        ldp x11, x15, [x20, #16]
        rev64 v16.4s, v20.4s
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x17, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x11, x7
        ldr q20, [sp, #48]
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
        ldr q28, [x20, #16]
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
        adds x20, x3, x13
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
        adds x7, x5, x20
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
        adds x20, x11, x13
        and x1, x1, x13
        adcs x25, x4, x1
        and x1, x12, x13
        stp x20, x25, [sp, #128]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [sp, #144]
        ldr q19, [sp, #96]
        ldr q23, [sp, #112]
        ldr q0, [sp, #96]
        uzp2 v29.4s, v19.4s, v19.4s
        xtn v4.2s, v19.2d
        umulh x8, x21, x22
        rev64 v20.4s, v23.4s
        umull v16.2d, v19.2s, v19.2s
        umull v1.2d, v29.2s, v4.2s
        mul v20.4s, v20.4s, v0.4s
        subs x14, x21, x22
        umulh x15, x21, x23
        mov x16, v16.d[1]
        umull2 v4.2d, v19.4s, v19.4s
        mov x4, v16.d[0]
        uzp1 v17.4s, v23.4s, v0.4s
        uaddlp v19.2d, v20.4s
        lsr x7, x8, #63
        mul x11, x21, x22
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x24, x23
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
        umulh x13, x22, x24
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x23, x24
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x23, x24
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x24, x24
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x24, x24
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x23, x23
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x23, x23
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
        csel x21, x3, x16, cs
        csel x22, x8, x14, cs
        csel x23, x11, x12, cs
        csel x24, x5, x2, cs
        ldr q19, [sp, #64]
        ldp x9, x13, [sp, #64]
        ldr q23, [sp, #80]
        ldr q0, [sp, #64]
        ldp x1, x10, [sp, #80]
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
        csel x13, x3, x16, cs
        csel x14, x8, x14, cs
        csel x15, x11, x12, cs
        csel x26, x5, x2, cs
        mov x1, #0x9
        mov x2, #0xffffffffffffffff
        subs x9, x2, x21
        mov x2, #0xffffffff
        sbcs x10, x2, x24
        ngcs x11, x22
        mov x2, #0xffffffff00000001
        sbc x12, x2, x23
        mul x3, x1, x9
        mul x4, x1, x10
        mul x5, x1, x11
        mul x6, x1, x12
        umulh x9, x1, x9
        umulh x10, x1, x10
        umulh x11, x1, x11
        umulh x7, x1, x12
        adds x4, x4, x9
        adcs x5, x5, x10
        adcs x6, x6, x11
        adc x7, x7, xzr
        mov x1, #0xc
        mul x8, x20, x1
        umulh x9, x20, x1
        adds x3, x3, x8
        mul x8, x25, x1
        umulh x10, x25, x1
        adcs x4, x4, x8
        ldp x11, x12, [sp, #144]
        mul x8, x11, x1
        umulh x11, x11, x1
        adcs x5, x5, x8
        mul x8, x12, x1
        umulh x12, x12, x1
        adcs x6, x6, x8
        adc x7, x7, xzr
        adds x4, x4, x9
        adcs x5, x5, x10
        adcs x6, x6, x11
        adc x7, x7, x12
        add x8, x7, #0x1
        lsl x10, x8, #32
        adds x6, x6, x10
        adc x7, x7, xzr
        neg x9, x8
        sub x10, x10, #0x1
        subs x3, x3, x9
        sbcs x4, x4, x10
        sbcs x5, x5, xzr
        sbcs x6, x6, x8
        sbc x8, x7, x8
        adds x20, x3, x8
        and x9, x8, #0xffffffff
        adcs x21, x4, x9
        adcs x22, x5, xzr
        neg x10, x9
        adc x23, x6, x10
        stp x20, x21, [sp, #160]
        stp x22, x23, [sp, #176]
        mov x2, sp
        ldp x4, x3, [x2]
        subs x5, x13, x4
        sbcs x6, x26, x3
        ldp x4, x3, [x2, #16]
        sbcs x7, x14, x4
        sbcs x8, x15, x3
        csetm x3, cc
        adds x5, x5, x3
        and x4, x3, #0xffffffff
        adcs x6, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x5, x6, [sp, #64]
        stp x7, x8, [sp, #80]
        mov x0, sp
        ldr q19, [sp, #32]
        ldp x9, x13, [sp, #32]
        ldr q23, [sp, #48]
        ldr q0, [sp, #32]
        ldp x1, x10, [sp, #48]
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
        csel x24, x3, x16, cs
        csel x25, x8, x14, cs
        csel x26, x11, x12, cs
        csel x27, x5, x2, cs
        stp x25, x26, [x0, #16]
        stp x24, x27, [x0]
        ldr q20, [sp, #96]
        ldr q0, [sp, #160]
        ldp x6, x10, [sp, #96]
        rev64 v16.4s, v20.4s
        subs x4, x20, x21
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4s, v16.4s, v0.4s
        umulh x12, x21, x10
        uzp1 v28.4s, v20.4s, v0.4s
        subs x14, x22, x20
        ldr q20, [sp, #112]
        sbcs x5, x23, x21
        ngc x17, xzr
        subs x8, x22, x23
        uaddlp v27.2d, v16.4s
        umulh x4, x20, x6
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
        ldp x7, x2, [sp, #112]
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
        adds x20, x3, x13
        adcs x21, x8, x7
        umulh x8, x14, x11
        umull v21.2d, v0.2s, v1.2s
        adcs x22, x10, x12
        umull v3.2d, v0.2s, v16.2s
        adc x23, x15, xzr
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
        adds x7, x5, x20
        adcs x15, x16, x21
        eor x5, x17, x4
        adcs x9, x1, x22
        eor x1, x10, x5
        adcs x16, x2, x23
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
        adds x14, x11, x13
        and x1, x1, x13
        adcs x15, x4, x1
        and x1, x12, x13
        stp x14, x15, [sp, #96]
        adcs x13, x7, xzr
        adc x20, x17, x1
        stp x13, x20, [sp, #112]
        ldp x5, x6, [sp, #64]
        ldp x4, x3, [sp, #32]
        subs x5, x5, x4
        sbcs x6, x6, x3
        ldp x7, x8, [sp, #80]
        ldp x4, x3, [sp, #48]
        sbcs x7, x7, x4
        sbcs x8, x8, x3
        csetm x3, cc
        adds x5, x5, x3
        and x4, x3, #0xffffffff
        adcs x6, x6, x4
        adcs x7, x7, xzr
        and x4, x3, #0xffffffff00000001
        adc x8, x8, x4
        stp x5, x6, [x19, #64]
        stp x7, x8, [x19, #80]
        ldp x1, x2, [sp, #128]
        lsl x0, x1, #2
        ldp x6, x7, [sp, #160]
        subs x0, x0, x6
        extr x1, x2, x1, #62
        sbcs x1, x1, x7
        ldp x3, x4, [sp, #144]
        extr x2, x3, x2, #62
        ldp x6, x7, [sp, #176]
        sbcs x2, x2, x6
        extr x3, x4, x3, #62
        sbcs x3, x3, x7
        lsr x4, x4, #62
        sbc x4, x4, xzr
        add x5, x4, #0x1
        lsl x8, x5, #32
        negs x6, x8
        ngcs x7, xzr
        sbc x8, x8, x5
        adds x0, x0, x5
        adcs x1, x1, x6
        adcs x2, x2, x7
        adcs x3, x3, x8
        csetm x5, cc
        adds x0, x0, x5
        and x6, x5, #0xffffffff
        adcs x1, x1, x6
        adcs x2, x2, xzr
        neg x7, x6
        adc x3, x3, x7
        stp x0, x1, [x19]
        stp x2, x3, [x19, #16]
        mov x2, #0xffffffffffffffff
        subs x9, x2, x24
        mov x2, #0xffffffff
        sbcs x10, x2, x27
        ngcs x11, x25
        mov x2, #0xffffffff00000001
        sbc x12, x2, x26
        lsl x3, x9, #3
        extr x4, x10, x9, #61
        extr x5, x11, x10, #61
        extr x6, x12, x11, #61
        lsr x7, x12, #61
        mov x1, #0x3
        mul x8, x14, x1
        umulh x9, x14, x1
        adds x3, x3, x8
        mul x8, x15, x1
        umulh x10, x15, x1
        adcs x4, x4, x8
        mul x8, x13, x1
        umulh x11, x13, x1
        adcs x5, x5, x8
        mul x8, x20, x1
        umulh x12, x20, x1
        adcs x6, x6, x8
        adc x7, x7, xzr
        adds x4, x4, x9
        adcs x5, x5, x10
        adcs x6, x6, x11
        adc x7, x7, x12
        add x8, x7, #0x1
        lsl x10, x8, #32
        adds x6, x6, x10
        adc x7, x7, xzr
        neg x9, x8
        sub x10, x10, #0x1
        subs x3, x3, x9
        sbcs x4, x4, x10
        sbcs x5, x5, xzr
        sbcs x6, x6, x8
        sbc x8, x7, x8
        adds x3, x3, x8
        and x9, x8, #0xffffffff
        adcs x4, x4, x9
        adcs x5, x5, xzr
        neg x10, x9
        adc x6, x6, x10
        stp x3, x4, [x19, #32]
        stp x5, x6, [x19, #48]

        CFI_STACKLOAD1Z(x27,NSPACE+64)
        CFI_STACKLOAD2(x25,x26,NSPACE+48)
        CFI_STACKLOAD2(x23,x24,NSPACE+32)
        CFI_STACKLOAD2(x21,x22,NSPACE+16)
        CFI_STACKLOAD2(x19,x20,NSPACE)
        CFI_INC_SP((NSPACE+80))
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
