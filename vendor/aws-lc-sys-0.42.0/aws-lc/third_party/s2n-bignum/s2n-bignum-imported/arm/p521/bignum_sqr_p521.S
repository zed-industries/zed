// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_521, z := (x^2) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_sqr_p521(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// bignum_sqr_p521 is functionally equivalent to unopt/bignum_sqr_p521_base.
// It is written in a way that
// 1. A subset of scalar multiplications in bignum_montmul_p384 are carefully
//    chosen and vectorized
// 2. The vectorized assembly is rescheduled using the SLOTHY superoptimizer.
//    https://github.com/slothy-optimizer/slothy
//
// The output program of step 1. is as follows:
//
//        stp     x19, x20, [sp, #-16]!
//        stp     x21, x22, [sp, #-16]!
//        stp     x23, x24, [sp, #-16]!
//        ldp x20, x19, [x1]
//        ldr q23, [x1]
//        ldr q1, [x1]
//        ldr q16, [x1]
//        ldp x14, x12, [x1, #16]
//        ldr q28, [x1, #16]
//        ldr q31, [x1, #16]
//        ldp x9, x2, [x1, #32]
//        ldr q29, [x1, #32]
//        ldr q4, [x1, #32]
//        ldr q5, [x1]
//        ldr q2, [x1, #32]
//        ldp x6, x13, [x1, #48]
//        ldr q24, [x1, #48]
//        ldr q27, [x1, #48]
//        ldr q0, [x1, #16]
//        ldr q30, [x1, #48]
//        mul x17, x9, x6
//        mul x10, x2, x13
//        umulh x24, x9, x6
//        subs x4, x9, x2
//        cneg x4, x4, cc
//        csetm x16, cc
//        subs x3, x13, x6
//        cneg x23, x3, cc
//        mul x3, x4, x23
//        umulh x4, x4, x23
//        cinv x22, x16, cc
//        eor x23, x3, x22
//        eor x16, x4, x22
//        adds x3, x17, x24
//        adc x24, x24, xzr
//        umulh x4, x2, x13
//        adds x3, x3, x10
//        adcs x24, x24, x4
//        adc x4, x4, xzr
//        adds x24, x24, x10
//        adc x10, x4, xzr
//        cmn x22, #0x1
//        adcs x4, x3, x23
//        adcs x24, x24, x16
//        adc x10, x10, x22
//        adds x8, x17, x17
//        adcs x22, x4, x4
//        adcs x5, x24, x24
//        adcs x11, x10, x10
//        adc x23, xzr, xzr
//        movi v25.2D, #0xffffffff
//        uzp2 v19.4S, v4.4S, v4.4S
//        xtn v26.2S, v29.2D
//        xtn v22.2S, v4.2D
//        rev64 v4.4S, v4.4S
//        umull v7.2D, v26.2S, v22.2S
//        umull v21.2D, v26.2S, v19.2S
//        uzp2 v17.4S, v29.4S, v29.4S
//        mul v4.4S, v4.4S, v29.4S
//        usra v21.2D, v7.2D, #32
//        umull v18.2D, v17.2S, v19.2S
//        uaddlp v4.2D, v4.4S
//        and v7.16B, v21.16B, v25.16B
//        umlal v7.2D, v17.2S, v22.2S
//        shl v4.2D, v4.2D, #32
//        usra v18.2D, v21.2D, #32
//        umlal v4.2D, v26.2S, v22.2S
//        usra v18.2D, v7.2D, #32
//        mov x15, v4.d[0]
//        mov x16, v4.d[1]
//        mul x3, x9, x2
//        mov x10, v18.d[0]
//        mov x17, v18.d[1]
//        umulh x4, x9, x2
//        adds x24, x10, x3
//        adcs x10, x16, x4
//        adc x17, x17, xzr
//        adds x7, x24, x3
//        adcs x10, x10, x4
//        adc x17, x17, xzr
//        adds x8, x8, x10
//        adcs x22, x22, x17
//        adcs x21, x5, xzr
//        adcs x5, x11, xzr
//        adc x11, x23, xzr
//        movi v25.2D, #0xffffffff
//        uzp2 v19.4S, v27.4S, v27.4S
//        xtn v26.2S, v24.2D
//        xtn v22.2S, v27.2D
//        rev64 v4.4S, v27.4S
//        umull v7.2D, v26.2S, v22.2S
//        umull v21.2D, v26.2S, v19.2S
//        uzp2 v17.4S, v24.4S, v24.4S
//        mul v4.4S, v4.4S, v24.4S
//        usra v21.2D, v7.2D, #32
//        umull v18.2D, v17.2S, v19.2S
//        uaddlp v4.2D, v4.4S
//        and v7.16B, v21.16B, v25.16B
//        umlal v7.2D, v17.2S, v22.2S
//        shl v4.2D, v4.2D, #32
//        usra v18.2D, v21.2D, #32
//        umlal v4.2D, v26.2S, v22.2S
//        usra v18.2D, v7.2D, #32
//        mov x23, v4.d[0]
//        mov x16, v4.d[1]
//        mul x3, x6, x13
//        mov x10, v18.d[0]
//        mov x17, v18.d[1]
//        umulh x4, x6, x13
//        adds x24, x10, x3
//        adcs x10, x16, x4
//        adc x17, x17, xzr
//        adds x24, x24, x3
//        adcs x10, x10, x4
//        adc x17, x17, xzr
//        adds x23, x23, x21
//        adcs x16, x24, x5
//        adcs x3, x10, x11
//        adc x21, x17, xzr
//        ldr x17, [x1, #64]
//        add x5, x17, x17
//        mul x11, x17, x17
//        and x17, x20, #0xfffffffffffff
//        mul x4, x5, x17
//        extr x17, x19, x20, #52
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x4, #52
//        add x24, x10, x17
//        lsl x17, x4, #12
//        extr x17, x24, x17, #12
//        adds x15, x15, x17
//        extr x17, x14, x19, #40
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x24, #52
//        add x4, x10, x17
//        lsl x17, x24, #12
//        extr x17, x4, x17, #24
//        adcs x7, x7, x17
//        extr x17, x12, x14, #28
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x4, #52
//        add x24, x10, x17
//        lsl x17, x4, #12
//        extr x17, x24, x17, #36
//        adcs x8, x8, x17
//        extr x17, x9, x12, #16
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x24, #52
//        add x4, x10, x17
//        lsl x17, x24, #12
//        extr x17, x4, x17, #48
//        adcs x22, x22, x17
//        lsr x17, x9, #4
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x4, #52
//        add x24, x10, x17
//        lsl x17, x4, #12
//        extr x4, x24, x17, #60
//        extr x17, x2, x9, #56
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x24, #52
//        add x24, x10, x17
//        lsl x17, x4, #8
//        extr x17, x24, x17, #8
//        adcs x23, x23, x17
//        extr x17, x6, x2, #44
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x24, #52
//        add x4, x10, x17
//        lsl x17, x24, #12
//        extr x17, x4, x17, #20
//        adcs x16, x16, x17
//        extr x17, x13, x6, #32
//        and x17, x17, #0xfffffffffffff
//        mul x10, x5, x17
//        lsr x17, x4, #52
//        add x24, x10, x17
//        lsl x17, x4, #12
//        extr x17, x24, x17, #32
//        adcs x3, x3, x17
//        lsr x17, x13, #20
//        mul x10, x5, x17
//        lsr x17, x24, #52
//        add x10, x10, x17
//        lsl x17, x24, #12
//        extr x17, x10, x17, #44
//        adcs x4, x21, x17
//        lsr x17, x10, #44
//        adc x24, x11, x17
//        extr x10, x7, x15, #9
//        extr x17, x8, x7, #9
//        stp x10, x17, [x0]                       // @slothy:writes=buffer0
//        extr x10, x22, x8, #9
//        extr x17, x23, x22, #9
//        stp x10, x17, [x0, #16]                  // @slothy:writes=buffer16
//        extr x10, x16, x23, #9
//        extr x17, x3, x16, #9
//        stp x10, x17, [x0, #32]                  // @slothy:writes=buffer32
//        extr x10, x4, x3, #9
//        extr x17, x24, x4, #9
//        stp x10, x17, [x0, #48]                  // @slothy:writes=buffer48
//        and x10, x15, #0x1ff
//        lsr x17, x24, #9
//        add x17, x10, x17
//        str x17, [x0, #64]                       // @slothy:writes=buffer64
//        uzp1 v17.4S, v28.4S, v23.4S
//        rev64 v4.4S, v28.4S
//        uzp1 v7.4S, v23.4S, v23.4S
//        mul v4.4S, v4.4S, v23.4S
//        uaddlp v4.2D, v4.4S
//        shl v4.2D, v4.2D, #32
//        umlal v4.2D, v7.2S, v17.2S
//        mov x8, v4.d[0]
//        mov x22, v4.d[1]
//        umulh x23, x20, x14
//        subs x17, x20, x19
//        cneg x4, x17, cc
//        csetm x24, cc
//        subs x17, x12, x14
//        cneg x17, x17, cc
//        mul x10, x4, x17
//        umulh x17, x4, x17
//        cinv x16, x24, cc
//        eor x3, x10, x16
//        eor x4, x17, x16
//        adds x24, x8, x23
//        adc x10, x23, xzr
//        umulh x17, x19, x12
//        adds x24, x24, x22
//        adcs x10, x10, x17
//        adc x17, x17, xzr
//        adds x10, x10, x22
//        adc x17, x17, xzr
//        cmn x16, #0x1
//        adcs x24, x24, x3
//        adcs x10, x10, x4
//        adc x17, x17, x16
//        adds x15, x8, x8
//        adcs x7, x24, x24
//        adcs x8, x10, x10
//        adcs x22, x17, x17
//        adc x23, xzr, xzr
//        movi v25.2D, #0xffffffff
//        uzp2 v19.4S, v16.4S, v16.4S
//        xtn v26.2S, v1.2D
//        xtn v22.2S, v16.2D
//        rev64 v4.4S, v16.4S
//        umull v7.2D, v26.2S, v22.2S
//        umull v21.2D, v26.2S, v19.2S
//        uzp2 v17.4S, v1.4S, v1.4S
//        mul v4.4S, v4.4S, v1.4S
//        usra v21.2D, v7.2D, #32
//        umull v18.2D, v17.2S, v19.2S
//        uaddlp v4.2D, v4.4S
//        and v7.16B, v21.16B, v25.16B
//        umlal v7.2D, v17.2S, v22.2S
//        shl v4.2D, v4.2D, #32
//        usra v18.2D, v21.2D, #32
//        umlal v4.2D, v26.2S, v22.2S
//        usra v18.2D, v7.2D, #32
//        mov x21, v4.d[0]
//        mov x16, v4.d[1]
//        mul x3, x20, x19
//        mov x10, v18.d[0]
//        mov x17, v18.d[1]
//        umulh x4, x20, x19
//        adds x24, x10, x3
//        adcs x10, x16, x4
//        adc x17, x17, xzr
//        adds x5, x24, x3
//        adcs x10, x10, x4
//        adc x17, x17, xzr
//        adds x11, x15, x10
//        adcs x15, x7, x17
//        adcs x7, x8, xzr
//        adcs x8, x22, xzr
//        adc x22, x23, xzr
//        xtn v7.2S, v31.2D
//        shrn v4.2S, v31.2D, #32
//        umull v4.2D, v7.2S, v4.2S
//        shl v4.2D, v4.2D, #33
//        umlal v4.2D, v7.2S, v7.2S
//        mov x23, v4.d[0]
//        mov x16, v4.d[1]
//        mul x3, x14, x12
//        umulh x10, x14, x14
//        umulh x17, x12, x12
//        umulh x4, x14, x12
//        adds x24, x10, x3
//        adcs x10, x16, x4
//        adc x17, x17, xzr
//        adds x24, x24, x3
//        adcs x10, x10, x4
//        adc x17, x17, xzr
//        adds x16, x23, x7
//        adcs x3, x24, x8
//        adcs x4, x10, x22
//        adc x24, x17, xzr
//        ldp x10, x17, [x0]                       // @slothy:reads=buffer0
//        adds x10, x10, x21
//        adcs x17, x17, x5
//        stp x10, x17, [x0]                       // @slothy:writes=buffer0
//        ldp x10, x17, [x0, #16]                  // @slothy:reads=buffer16
//        adcs x10, x10, x11
//        adcs x17, x17, x15
//        stp x10, x17, [x0, #16]                  // @slothy:writes=buffer16
//        ldp x10, x17, [x0, #32]                  // @slothy:reads=buffer32
//        adcs x10, x10, x16
//        adcs x17, x17, x3
//        stp x10, x17, [x0, #32]                  // @slothy:writes=buffer32
//        ldp x10, x17, [x0, #48]                  // @slothy:reads=buffer48
//        adcs x10, x10, x4
//        adcs x17, x17, x24
//        stp x10, x17, [x0, #48]                  // @slothy:writes=buffer48
//        ldr x17, [x0, #64]                       // @slothy:reads=buffer64
//        adc x17, x17, xzr
//        str x17, [x0, #64]                       // @slothy:writes=buffer64
//        movi v25.2D, #0xffffffff
//        uzp2 v19.4S, v2.4S, v2.4S
//        xtn v26.2S, v5.2D
//        xtn v22.2S, v2.2D
//        rev64 v4.4S, v2.4S
//        umull v7.2D, v26.2S, v22.2S
//        umull v21.2D, v26.2S, v19.2S
//        uzp2 v17.4S, v5.4S, v5.4S
//        mul v4.4S, v4.4S, v5.4S
//        usra v21.2D, v7.2D, #32
//        umull v18.2D, v17.2S, v19.2S
//        uaddlp v4.2D, v4.4S
//        and v7.16B, v21.16B, v25.16B
//        umlal v7.2D, v17.2S, v22.2S
//        shl v4.2D, v4.2D, #32
//        usra v18.2D, v21.2D, #32
//        umlal v4.2D, v26.2S, v22.2S
//        usra v18.2D, v7.2D, #32
//        mov x5, v4.d[0]
//        mov x4, v4.d[1]
//        movi v25.2D, #0xffffffff
//        uzp2 v17.4S, v30.4S, v30.4S
//        xtn v19.2S, v0.2D
//        xtn v26.2S, v30.2D
//        rev64 v4.4S, v30.4S
//        umull v7.2D, v19.2S, v26.2S
//        umull v22.2D, v19.2S, v17.2S
//        uzp2 v21.4S, v0.4S, v0.4S
//        mul v4.4S, v4.4S, v0.4S
//        usra v22.2D, v7.2D, #32
//        umull v17.2D, v21.2S, v17.2S
//        uaddlp v4.2D, v4.4S
//        and v7.16B, v22.16B, v25.16B
//        umlal v7.2D, v21.2S, v26.2S
//        shl v4.2D, v4.2D, #32
//        usra v17.2D, v22.2D, #32
//        umlal v4.2D, v19.2S, v26.2S
//        usra v17.2D, v7.2D, #32
//        mov x24, v4.d[0]
//        mov x10, v4.d[1]
//        mov x17, v18.d[0]
//        adds x4, x4, x17
//        mov x17, v18.d[1]
//        adcs x24, x24, x17
//        mov x17, v17.d[0]
//        adcs x10, x10, x17
//        mov x17, v17.d[1]
//        adc x17, x17, xzr
//        adds x15, x4, x5
//        adcs x4, x24, x4
//        adcs x24, x10, x24
//        adcs x10, x17, x10
//        adc x17, xzr, x17
//        adds x7, x4, x5
//        adcs x8, x24, x15
//        adcs x22, x10, x4
//        adcs x23, x17, x24
//        adcs x16, xzr, x10
//        adc x3, xzr, x17
//        subs x17, x14, x12
//        cneg x24, x17, cc
//        csetm x4, cc
//        subs x17, x13, x6
//        cneg x10, x17, cc
//        mul x17, x24, x10
//        umulh x24, x24, x10
//        cinv x10, x4, cc
//        cmn x10, #0x1
//        eor x17, x17, x10
//        adcs x23, x23, x17
//        eor x17, x24, x10
//        adcs x16, x16, x17
//        adc x3, x3, x10
//        subs x17, x20, x19
//        cneg x24, x17, cc
//        csetm x4, cc
//        subs x17, x2, x9
//        cneg x10, x17, cc
//        mul x17, x24, x10
//        umulh x24, x24, x10
//        cinv x10, x4, cc
//        cmn x10, #0x1
//        eor x17, x17, x10
//        adcs x11, x15, x17
//        eor x17, x24, x10
//        adcs x15, x7, x17
//        adcs x7, x8, x10
//        adcs x22, x22, x10
//        adcs x23, x23, x10
//        adcs x16, x16, x10
//        adc x3, x3, x10
//        subs x17, x19, x12
//        cneg x24, x17, cc
//        csetm x4, cc
//        subs x17, x13, x2
//        cneg x10, x17, cc
//        mul x17, x24, x10
//        umulh x24, x24, x10
//        cinv x10, x4, cc
//        cmn x10, #0x1
//        eor x17, x17, x10
//        adcs x8, x22, x17
//        eor x17, x24, x10
//        adcs x23, x23, x17
//        adcs x16, x16, x10
//        adc x3, x3, x10
//        subs x17, x20, x14
//        cneg x24, x17, cc
//        csetm x4, cc
//        subs x17, x6, x9
//        cneg x10, x17, cc
//        mul x17, x24, x10
//        umulh x24, x24, x10
//        cinv x10, x4, cc
//        cmn x10, #0x1
//        eor x17, x17, x10
//        adcs x22, x15, x17
//        eor x17, x24, x10
//        adcs x4, x7, x17
//        adcs x24, x8, x10
//        adcs x23, x23, x10
//        adcs x16, x16, x10
//        adc x3, x3, x10
//        subs x12, x20, x12
//        cneg x10, x12, cc
//        csetm x17, cc
//        subs x12, x13, x9
//        cneg x9, x12, cc
//        mul x12, x10, x9
//        umulh x13, x10, x9
//        cinv x9, x17, cc
//        cmn x9, #0x1
//        eor x12, x12, x9
//        adcs x4, x4, x12
//        eor x12, x13, x9
//        adcs x24, x24, x12
//        adcs x10, x23, x9
//        adcs x17, x16, x9
//        adc x13, x3, x9
//        subs x19, x19, x14
//        cneg x12, x19, cc
//        csetm x9, cc
//        subs x6, x6, x2
//        cneg x14, x6, cc
//        mul x19, x12, x14
//        umulh x12, x12, x14
//        cinv x14, x9, cc
//        cmn x14, #0x1
//        eor x19, x19, x14
//        adcs x23, x4, x19
//        eor x19, x12, x14
//        adcs x16, x24, x19
//        adcs x6, x10, x14
//        adcs x2, x17, x14
//        adc x9, x13, x14
//        ldp x12, x14, [x0]                       // @slothy:reads=buffer0
//        extr x19, x6, x16, #8
//        adds x10, x19, x12
//        extr x19, x2, x6, #8
//        adcs x17, x19, x14
//        ldp x14, x12, [x0, #16]                  // @slothy:reads=buffer16
//        extr x19, x9, x2, #8
//        adcs x13, x19, x14
//        and x14, x17, x13
//        lsr x19, x9, #8
//        adcs x6, x19, x12
//        and x9, x14, x6
//        ldp x14, x12, [x0, #32]                  // @slothy:reads=buffer32
//        lsl x19, x5, #1
//        adcs x2, x19, x14
//        and x14, x9, x2
//        extr x19, x11, x5, #63
//        adcs x3, x19, x12
//        and x9, x14, x3
//        ldp x14, x12, [x0, #48]                  // @slothy:reads=buffer48
//        extr x19, x22, x11, #63
//        adcs x4, x19, x14
//        and x14, x9, x4
//        extr x19, x23, x22, #63
//        adcs x24, x19, x12
//        and x12, x14, x24
//        ldr x14, [x0, #64]                       // @slothy:reads=buffer64
//        extr x19, x16, x23, #63
//        and x19, x19, #0x1ff
//        adc x19, x14, x19
//        lsr x14, x19, #9
//        orr x19, x19, #0xfffffffffffffe00
//        cmp xzr, xzr
//        adcs xzr, x10, x14
//        adcs xzr, x12, xzr
//        adcs xzr, x19, xzr
//        adcs x10, x10, x14
//        adcs x17, x17, xzr
//        adcs x13, x13, xzr
//        adcs x6, x6, xzr
//        adcs x2, x2, xzr
//        adcs x9, x3, xzr
//        adcs x12, x4, xzr
//        adcs x14, x24, xzr
//        adc x19, x19, xzr
//        and x19, x19, #0x1ff
//        stp x10, x17, [x0]                       // @slothy:writes=buffer0
//        stp x13, x6, [x0, #16]                   // @slothy:writes=buffer16
//        stp x2, x9, [x0, #32]                    // @slothy:writes=buffer32
//        stp x12, x14, [x0, #48]                  // @slothy:writes=buffer48
//        str x19, [x0, #64]                       // @slothy:writes=buffer64
//        ldp     x23, x24, [sp], #16
//        ldp     x21, x22, [sp], #16
//        ldp     x19, x20, [sp], #16
//        ret
//
// The bash script used for step 2 is as follows:
//
//        # Store the assembly instructions except the last 'ret',
//        # callee-register store/loads as, say, 'input.S'.
//        export OUTPUTS="[hint_buffer0,hint_buffer16,hint_buffer32,hint_buffer48,hint_buffer64]"
//        export RESERVED_REGS="[x18,x25,x26,x27,x28,x29,x30,sp,q8,q9,q10,q11,q12,q13,q14,q15,v8,v9,v10,v11,v12,v13,v14,v15]"
//        <s2n-bignum>/tools/external/slothy.sh input.S my_out_dir
//        # my_out_dir/3.opt.s is the optimized assembly. Its output may differ
//        # from this file since the sequence is non-deterministically chosen.
//        # Please add 'ret' at the end of the output assembly.

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p521)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_sqr_p521):
        CFI_START

// Save registers

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)

        ldr q23, [x1, #32]
        ldp x9, x2, [x1, #32]
        ldr q16, [x1, #32]
        ldr q20, [x1, #48]
        ldp x6, x13, [x1, #48]
        rev64 v2.4S, v23.4S
        mul x14, x9, x2
        ldr q31, [x1, #48]
        subs x22, x9, x2
        uzp2 v26.4S, v23.4S, v23.4S
        mul v30.4S, v2.4S, v16.4S
        xtn v0.2S, v20.2D
        csetm x12, cc
        xtn v21.2S, v16.2D
        xtn v23.2S, v23.2D
        umulh x10, x9, x6
        rev64 v27.4S, v31.4S
        umull v2.2D, v21.2S, v26.2S
        cneg x23, x22, cc
        uaddlp v25.2D, v30.4S
        umull v18.2D, v21.2S, v23.2S
        mul x22, x9, x6
        mul v6.4S, v27.4S, v20.4S
        uzp2 v17.4S, v20.4S, v20.4S
        shl v20.2D, v25.2D, #32
        uzp2 v27.4S, v31.4S, v31.4S
        mul x16, x2, x13
        umlal v20.2D, v21.2S, v23.2S
        usra v2.2D, v18.2D, #32
        adds x8, x22, x10
        umull v25.2D, v17.2S, v27.2S
        xtn v31.2S, v31.2D
        movi v1.2D, #0xffffffff
        adc x3, x10, xzr
        umulh x21, x2, x13
        uzp2 v21.4S, v16.4S, v16.4S
        umull v18.2D, v0.2S, v27.2S
        subs x19, x13, x6
        and v7.16B, v2.16B, v1.16B
        umull v27.2D, v0.2S, v31.2S
        cneg x20, x19, cc
        movi v30.2D, #0xffffffff
        umull v16.2D, v21.2S, v26.2S
        umlal v7.2D, v21.2S, v23.2S
        mul x19, x23, x20
        cinv x7, x12, cc
        uaddlp v6.2D, v6.4S
        eor x12, x19, x7
        adds x11, x8, x16
        umulh x10, x23, x20
        ldr q1, [x1]
        usra v16.2D, v2.2D, #32
        adcs x19, x3, x21
        shl v2.2D, v6.2D, #32
        adc x20, x21, xzr
        adds x17, x19, x16
        usra v18.2D, v27.2D, #32
        adc x19, x20, xzr
        cmn x7, #0x1
        umlal v2.2D, v0.2S, v31.2S
        umulh x16, x9, x2
        adcs x8, x11, x12
        usra v16.2D, v7.2D, #32
        ldr x12, [x1, #64]
        eor x20, x10, x7
        umulh x10, x6, x13
        mov x23, v2.d[0]
        mov x3, v2.d[1]
        adcs x21, x17, x20
        usra v25.2D, v18.2D, #32
        and v23.16B, v18.16B, v30.16B
        adc x7, x19, x7
        adds x22, x22, x22
        ldr q7, [x1, #16]
        adcs x17, x8, x8
        umlal v23.2D, v17.2S, v31.2S
        mov x19, v16.d[0]
        mul x11, x12, x12
        ldr q4, [x1]
        usra v25.2D, v23.2D, #32
        add x5, x12, x12
        adcs x15, x21, x21
        ldr q28, [x1]
        mov x12, v20.d[1]
        adcs x24, x7, x7
        mov x21, v16.d[1]
        adc x4, xzr, xzr
        adds x19, x19, x14
        ldr q18, [x1, #16]
        xtn v26.2S, v1.2D
        adcs x8, x12, x16
        adc x21, x21, xzr
        adds x7, x19, x14
        xtn v23.2S, v7.2D
        rev64 v21.4S, v28.4S
        adcs x12, x8, x16
        ldp x20, x19, [x1]
        mov x16, v25.d[1]
        xtn v22.2S, v28.2D
        adc x14, x21, xzr
        adds x8, x22, x12
        uzp2 v24.4S, v28.4S, v28.4S
        rev64 v28.4S, v18.4S
        mul x12, x6, x13
        mul v16.4S, v21.4S, v1.4S
        shrn v31.2S, v7.2D, #32
        adcs x22, x17, x14
        mov x14, v25.d[0]
        and x21, x20, #0xfffffffffffff
        umull v17.2D, v26.2S, v24.2S
        ldr q2, [x1, #32]
        adcs x17, x15, xzr
        ldr q30, [x1, #48]
        umull v7.2D, v26.2S, v22.2S
        adcs x15, x24, xzr
        ldr q0, [x1, #16]
        movi v6.2D, #0xffffffff
        adc x4, x4, xzr
        adds x14, x14, x12
        uzp1 v27.4S, v18.4S, v4.4S
        uzp2 v19.4S, v1.4S, v1.4S
        adcs x24, x3, x10
        mul x3, x5, x21
        umull v29.2D, v23.2S, v31.2S
        ldr q5, [x1]
        adc x21, x16, xzr
        adds x16, x14, x12
        extr x12, x19, x20, #52
        umull v18.2D, v19.2S, v24.2S
        adcs x24, x24, x10
        and x10, x12, #0xfffffffffffff
        ldp x14, x12, [x1, #16]
        usra v17.2D, v7.2D, #32
        adc x21, x21, xzr
        adds x23, x23, x17
        mul x17, x5, x10
        shl v21.2D, v29.2D, #33
        lsl x10, x3, #12
        lsr x1, x3, #52
        rev64 v29.4S, v2.4S
        uaddlp v25.2D, v16.4S
        add x17, x17, x1
        adcs x16, x16, x15
        extr x3, x14, x19, #40
        mov x15, v20.d[0]
        extr x10, x17, x10, #12
        and x3, x3, #0xfffffffffffff
        shl v3.2D, v25.2D, #32
        and v6.16B, v17.16B, v6.16B
        mul x1, x5, x3
        usra v18.2D, v17.2D, #32
        adcs x3, x24, x4
        extr x4, x12, x14, #28
        umlal v6.2D, v19.2S, v22.2S
        xtn v20.2S, v2.2D
        umlal v3.2D, v26.2S, v22.2S
        movi v26.2D, #0xffffffff
        lsr x24, x17, #52
        and x4, x4, #0xfffffffffffff
        uzp2 v19.4S, v2.4S, v2.4S
        add x1, x1, x24
        mul x24, x5, x4
        lsl x4, x17, #12
        xtn v24.2S, v5.2D
        extr x17, x1, x4, #24
        adc x21, x21, xzr
        umlal v21.2D, v23.2S, v23.2S
        adds x4, x15, x10
        lsl x10, x1, #12
        adcs x15, x7, x17
        mul v23.4S, v28.4S, v4.4S
        and x7, x4, #0x1ff
        lsr x17, x1, #52
        umulh x1, x19, x12
        uzp2 v17.4S, v5.4S, v5.4S
        extr x4, x15, x4, #9
        add x24, x24, x17
        mul v29.4S, v29.4S, v5.4S
        extr x17, x24, x10, #36
        extr x10, x9, x12, #16
        uzp1 v28.4S, v4.4S, v4.4S
        adcs x17, x8, x17
        and x8, x10, #0xfffffffffffff
        umull v16.2D, v24.2S, v20.2S
        extr x10, x17, x15, #9
        mul x15, x5, x8
        stp x4, x10, [x0]
        lsl x4, x24, #12
        lsr x8, x9, #4
        uaddlp v4.2D, v23.4S
        and x8, x8, #0xfffffffffffff
        umull v23.2D, v24.2S, v19.2S
        mul x8, x5, x8
        extr x10, x2, x9, #56
        lsr x24, x24, #52
        and x10, x10, #0xfffffffffffff
        add x15, x15, x24
        extr x4, x15, x4, #48
        mul x24, x5, x10
        lsr x10, x15, #52
        usra v23.2D, v16.2D, #32
        add x10, x8, x10
        shl v4.2D, v4.2D, #32
        adcs x22, x22, x4
        extr x4, x6, x2, #44
        lsl x15, x15, #12
        lsr x8, x10, #52
        extr x15, x10, x15, #60
        and x10, x4, #0xfffffffffffff
        umlal v4.2D, v28.2S, v27.2S
        add x8, x24, x8
        extr x4, x13, x6, #32
        mul x24, x5, x10
        uzp2 v16.4S, v30.4S, v30.4S
        lsl x10, x15, #8
        rev64 v28.4S, v30.4S
        and x15, x4, #0xfffffffffffff
        extr x4, x8, x10, #8
        mul x10, x5, x15
        lsl x15, x8, #12
        adcs x23, x23, x4
        lsr x4, x8, #52
        lsr x8, x13, #20
        add x4, x24, x4
        mul x8, x5, x8
        lsr x24, x4, #52
        extr x15, x4, x15, #20
        lsl x4, x4, #12
        add x10, x10, x24
        adcs x15, x16, x15
        extr x4, x10, x4, #32
        umulh x5, x20, x14
        adcs x3, x3, x4
        usra v18.2D, v6.2D, #32
        lsl x16, x10, #12
        extr x24, x15, x23, #9
        lsr x10, x10, #52
        uzp2 v27.4S, v0.4S, v0.4S
        add x8, x8, x10
        extr x10, x3, x15, #9
        extr x4, x22, x17, #9
        and v25.16B, v23.16B, v26.16B
        lsr x17, x8, #44
        extr x15, x8, x16, #44
        extr x16, x23, x22, #9
        xtn v7.2S, v30.2D
        mov x8, v4.d[0]
        stp x24, x10, [x0, #32]
        uaddlp v30.2D, v29.4S
        stp x4, x16, [x0, #16]
        umulh x24, x20, x19
        adcs x15, x21, x15
        adc x16, x11, x17
        subs x11, x20, x19
        xtn v5.2S, v0.2D
        csetm x17, cc
        extr x3, x15, x3, #9
        mov x22, v4.d[1]
        cneg x21, x11, cc
        subs x10, x12, x14
        mul v31.4S, v28.4S, v0.4S
        cneg x10, x10, cc
        cinv x11, x17, cc
        shl v4.2D, v30.2D, #32
        umull v28.2D, v5.2S, v16.2S
        extr x23, x16, x15, #9
        adds x4, x8, x5
        mul x17, x21, x10
        umull v22.2D, v5.2S, v7.2S
        adc x15, x5, xzr
        adds x4, x4, x22
        uaddlp v2.2D, v31.4S
        lsr x5, x16, #9
        adcs x16, x15, x1
        mov x15, v18.d[0]
        adc x1, x1, xzr
        umulh x10, x21, x10
        adds x22, x16, x22
        umlal v4.2D, v24.2S, v20.2S
        umull v30.2D, v27.2S, v16.2S
        stp x3, x23, [x0, #48]
        add x3, x7, x5
        adc x16, x1, xzr
        usra v28.2D, v22.2D, #32
        mul x23, x20, x19
        eor x1, x17, x11
        cmn x11, #0x1
        mov x17, v18.d[1]
        umull v18.2D, v17.2S, v19.2S
        adcs x7, x4, x1
        eor x1, x10, x11
        umlal v25.2D, v17.2S, v20.2S
        movi v16.2D, #0xffffffff
        adcs x22, x22, x1
        usra v18.2D, v23.2D, #32
        umulh x4, x14, x14
        adc x1, x16, x11
        adds x10, x8, x8
        shl v23.2D, v2.2D, #32
        str x3, [x0, #64]
        adcs x5, x7, x7
        and v16.16B, v28.16B, v16.16B
        usra v30.2D, v28.2D, #32
        adcs x7, x22, x22
        mov x21, v3.d[1]
        adcs x11, x1, x1
        umlal v16.2D, v27.2S, v7.2S
        adc x22, xzr, xzr
        adds x16, x15, x23
        mul x8, x14, x12
        umlal v23.2D, v5.2S, v7.2S
        usra v18.2D, v25.2D, #32
        umulh x15, x14, x12
        adcs x21, x21, x24
        usra v30.2D, v16.2D, #32
        adc x1, x17, xzr
        adds x3, x16, x23
        adcs x21, x21, x24
        adc x1, x1, xzr
        adds x24, x10, x21
        umulh x21, x12, x12
        adcs x16, x5, x1
        adcs x10, x7, xzr
        mov x17, v21.d[1]
        adcs x23, x11, xzr
        adc x5, x22, xzr
        adds x1, x4, x8
        adcs x22, x17, x15
        ldp x17, x4, [x0]
        mov x11, v21.d[0]
        adc x21, x21, xzr
        adds x1, x1, x8
        adcs x15, x22, x15
        adc x8, x21, xzr
        adds x22, x11, x10
        mov x21, v3.d[0]
        adcs x11, x1, x23
        ldp x1, x10, [x0, #16]
        adcs x15, x15, x5
        adc x7, x8, xzr
        adds x8, x17, x21
        mov x23, v4.d[1]
        ldp x5, x21, [x0, #32]
        adcs x17, x4, x3
        ldr x4, [x0, #64]
        mov x3, v18.d[0]
        adcs x24, x1, x24
        stp x8, x17, [x0]
        adcs x17, x10, x16
        ldp x1, x16, [x0, #48]
        adcs x5, x5, x22
        adcs x8, x21, x11
        stp x5, x8, [x0, #32]
        adcs x1, x1, x15
        mov x15, v23.d[1]
        adcs x21, x16, x7
        stp x1, x21, [x0, #48]
        adc x10, x4, xzr
        subs x7, x14, x12
        mov x16, v18.d[1]
        cneg x5, x7, cc
        csetm x4, cc
        subs x11, x13, x6
        mov x8, v23.d[0]
        cneg x7, x11, cc
        cinv x21, x4, cc
        mov x11, v30.d[0]
        adds x4, x23, x3
        mul x22, x5, x7
        mov x23, v30.d[1]
        adcs x8, x8, x16
        adcs x16, x15, x11
        adc x11, x23, xzr
        umulh x3, x5, x7
        stp x24, x17, [x0, #16]
        mov x5, v4.d[0]
        subs x15, x20, x19
        cneg x7, x15, cc
        str x10, [x0, #64]
        csetm x1, cc
        subs x24, x2, x9
        cneg x17, x24, cc
        cinv x15, x1, cc
        adds x23, x4, x5
        umulh x1, x7, x17
        adcs x24, x8, x4
        adcs x10, x16, x8
        eor x8, x22, x21
        adcs x16, x11, x16
        mul x22, x7, x17
        eor x17, x1, x15
        adc x1, xzr, x11
        adds x11, x24, x5
        eor x7, x3, x21
        adcs x3, x10, x23
        adcs x24, x16, x24
        adcs x4, x1, x10
        eor x10, x22, x15
        adcs x16, xzr, x16
        adc x1, xzr, x1
        cmn x21, #0x1
        adcs x8, x4, x8
        adcs x22, x16, x7
        adc x7, x1, x21
        subs x21, x19, x12
        csetm x4, cc
        cneg x1, x21, cc
        subs x21, x13, x2
        cinv x16, x4, cc
        cneg x4, x21, cc
        cmn x15, #0x1
        adcs x21, x23, x10
        mul x23, x1, x4
        adcs x11, x11, x17
        adcs x3, x3, x15
        umulh x1, x1, x4
        adcs x24, x24, x15
        adcs x8, x8, x15
        adcs x22, x22, x15
        eor x17, x23, x16
        adc x15, x7, x15
        subs x7, x20, x14
        cneg x7, x7, cc
        csetm x4, cc
        subs x10, x20, x12
        cneg x23, x10, cc
        csetm x10, cc
        subs x12, x6, x9
        cinv x20, x4, cc
        cneg x12, x12, cc
        cmn x16, #0x1
        eor x1, x1, x16
        adcs x17, x24, x17
        mul x4, x7, x12
        adcs x8, x8, x1
        umulh x1, x7, x12
        adcs x24, x22, x16
        adc x7, x15, x16
        subs x12, x13, x9
        cneg x12, x12, cc
        cinv x13, x10, cc
        subs x19, x19, x14
        mul x9, x23, x12
        cneg x19, x19, cc
        csetm x10, cc
        eor x16, x1, x20
        subs x22, x6, x2
        umulh x12, x23, x12
        eor x1, x4, x20
        cinv x4, x10, cc
        cneg x22, x22, cc
        cmn x20, #0x1
        adcs x15, x11, x1
        eor x6, x12, x13
        adcs x10, x3, x16
        adcs x17, x17, x20
        eor x23, x9, x13
        adcs x2, x8, x20
        mul x11, x19, x22
        adcs x24, x24, x20
        adc x7, x7, x20
        cmn x13, #0x1
        adcs x3, x10, x23
        umulh x22, x19, x22
        adcs x17, x17, x6
        eor x12, x22, x4
        extr x22, x15, x21, #63
        adcs x8, x2, x13
        extr x21, x21, x5, #63
        ldp x16, x23, [x0]
        adcs x20, x24, x13
        eor x1, x11, x4
        adc x6, x7, x13
        cmn x4, #0x1
        ldp x2, x7, [x0, #16]
        adcs x1, x3, x1
        extr x19, x1, x15, #63
        adcs x14, x17, x12
        extr x1, x14, x1, #63
        lsl x17, x5, #1
        adcs x8, x8, x4
        extr x12, x8, x14, #8
        ldp x15, x11, [x0, #32]
        adcs x9, x20, x4
        adc x3, x6, x4
        adds x16, x12, x16
        extr x6, x9, x8, #8
        ldp x14, x12, [x0, #48]
        extr x8, x3, x9, #8
        adcs x20, x6, x23
        ldr x24, [x0, #64]
        lsr x6, x3, #8
        adcs x8, x8, x2
        and x2, x1, #0x1ff
        and x1, x20, x8
        adcs x4, x6, x7
        adcs x3, x17, x15
        and x1, x1, x4
        adcs x9, x21, x11
        and x1, x1, x3
        adcs x6, x22, x14
        and x1, x1, x9
        and x21, x1, x6
        adcs x14, x19, x12
        adc x1, x24, x2
        cmp xzr, xzr
        orr x12, x1, #0xfffffffffffffe00
        lsr x1, x1, #9
        adcs xzr, x16, x1
        and x21, x21, x14
        adcs xzr, x21, xzr
        adcs xzr, x12, xzr
        adcs x21, x16, x1
        adcs x1, x20, xzr
        adcs x19, x8, xzr
        stp x21, x1, [x0]
        adcs x1, x4, xzr
        adcs x21, x3, xzr
        stp x19, x1, [x0, #16]
        adcs x1, x9, xzr
        stp x21, x1, [x0, #32]
        adcs x21, x6, xzr
        adcs x1, x14, xzr
        stp x21, x1, [x0, #48]
        adc x1, x12, xzr
        and x1, x1, #0x1ff
        str x1, [x0, #64]

// Restore regs and return

        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
