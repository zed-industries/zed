// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_521, z := (x * y) mod p_521, assuming x and y reduced
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_mul_p521(uint64_t z[static 9], const uint64_t x[static 9],
//                                const uint64_t y[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// bignum_mul_p521 is functionally equivalent to unopt/bignum_mul_p521_base.
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
//        stp     x25, x26, [sp, #-16]!
//        sub     sp, sp, #80
//        ldp x15, x21, [x1]
//        ldp x10, x17, [x1, #16]
//        ldp x13, x16, [x2]
//        ldr q18, [x1]
//        ldr q28, [x2]
//        ldp x5, x20, [x2, #16]
//        movi v16.2D, #0x00000000ffffffff
//        uzp2 v7.4S, v28.4S, v28.4S
//        xtn v4.2S, v18.2D
//        xtn v1.2S, v28.2D
//        rev64 v27.4S, v28.4S
//        umull v21.2D, v4.2S, v1.2S
//        umull v28.2D, v4.2S, v7.2S
//        uzp2 v5.4S, v18.4S, v18.4S
//        mul v18.4S, v27.4S, v18.4S
//        usra v28.2D, v21.2D, #32
//        umull v29.2D, v5.2S, v7.2S
//        uaddlp v18.2D, v18.4S
//        and v16.16B, v28.16B, v16.16B
//        umlal v16.2D, v5.2S, v1.2S
//        shl v18.2D, v18.2D, #32
//        usra v29.2D, v28.2D, #32
//        umlal v18.2D, v4.2S, v1.2S
//        usra v29.2D, v16.2D, #32
//        mov x8, v18.d[0]
//        mov x9, v18.d[1]
//        mul x6, x10, x5
//        mul x19, x17, x20
//        mov x14, v29.d[0]
//        adds x9, x9, x14
//        mov x14, v29.d[1]
//        adcs x6, x6, x14
//        umulh x14, x10, x5
//        adcs x19, x19, x14
//        umulh x14, x17, x20
//        adc x14, x14, xzr
//        adds x11, x9, x8
//        adcs x9, x6, x9
//        adcs x6, x19, x6
//        adcs x19, x14, x19
//        adc x14, xzr, x14
//        adds x3, x9, x8
//        adcs x24, x6, x11
//        adcs x9, x19, x9
//        adcs x6, x14, x6
//        adcs x19, xzr, x19
//        adc x14, xzr, x14
//        subs x4, x10, x17
//        cneg x4, x4, cc
//        csetm x7, cc
//        subs x23, x20, x5
//        cneg x23, x23, cc
//        mul x22, x4, x23
//        umulh x4, x4, x23
//        cinv x7, x7, cc
//        cmn x7, #0x1
//        eor x23, x22, x7
//        adcs x6, x6, x23
//        eor x4, x4, x7
//        adcs x19, x19, x4
//        adc x14, x14, x7
//        subs x4, x15, x21
//        cneg x4, x4, cc
//        csetm x7, cc
//        subs x23, x16, x13
//        cneg x23, x23, cc
//        mul x22, x4, x23
//        umulh x4, x4, x23
//        cinv x7, x7, cc
//        cmn x7, #0x1
//        eor x23, x22, x7
//        adcs x11, x11, x23
//        eor x4, x4, x7
//        adcs x3, x3, x4
//        adcs x24, x24, x7
//        adcs x9, x9, x7
//        adcs x6, x6, x7
//        adcs x19, x19, x7
//        adc x14, x14, x7
//        subs x4, x21, x17
//        cneg x4, x4, cc
//        csetm x7, cc
//        subs x23, x20, x16
//        cneg x23, x23, cc
//        mul x22, x4, x23
//        umulh x4, x4, x23
//        cinv x7, x7, cc
//        cmn x7, #0x1
//        eor x23, x22, x7
//        adcs x9, x9, x23
//        eor x4, x4, x7
//        adcs x6, x6, x4
//        adcs x19, x19, x7
//        adc x14, x14, x7
//        subs x4, x15, x10
//        cneg x4, x4, cc
//        csetm x7, cc
//        subs x23, x5, x13
//        cneg x23, x23, cc
//        mul x22, x4, x23
//        umulh x4, x4, x23
//        cinv x7, x7, cc
//        cmn x7, #0x1
//        eor x23, x22, x7
//        adcs x3, x3, x23
//        eor x4, x4, x7
//        adcs x24, x24, x4
//        adcs x9, x9, x7
//        adcs x6, x6, x7
//        adcs x19, x19, x7
//        adc x14, x14, x7
//        subs x17, x15, x17
//        cneg x17, x17, cc
//        csetm x4, cc
//        subs x13, x20, x13
//        cneg x13, x13, cc
//        mul x20, x17, x13
//        umulh x17, x17, x13
//        cinv x13, x4, cc
//        cmn x13, #0x1
//        eor x20, x20, x13
//        adcs x20, x24, x20
//        eor x17, x17, x13
//        adcs x17, x9, x17
//        adcs x9, x6, x13
//        adcs x6, x19, x13
//        adc x13, x14, x13
//        subs x21, x21, x10
//        cneg x21, x21, cc
//        csetm x10, cc
//        subs x16, x5, x16
//        cneg x16, x16, cc
//        mul x5, x21, x16
//        umulh x21, x21, x16
//        cinv x10, x10, cc
//        cmn x10, #0x1
//        eor x16, x5, x10
//        adcs x16, x20, x16
//        eor x21, x21, x10
//        adcs x21, x17, x21
//        adcs x17, x9, x10
//        adcs x5, x6, x10
//        adc x10, x13, x10
//        lsl x13, x8, #9
//        extr x20, x11, x8, #55
//        extr x8, x3, x11, #55
//        extr x9, x16, x3, #55
//        lsr x16, x16, #55
//        stp x21, x17, [sp]                       // @slothy:writes=stack0
//        stp x5, x10, [sp, #16]                   // @slothy:writes=stack16
//        stp x13, x20, [sp, #32]                  // @slothy:writes=stack32
//        stp x8, x9, [sp, #48]                    // @slothy:writes=stack48
//        str x16, [sp, #64]                       // @slothy:writes=stack64
//        ldp x21, x10, [x1, #32]
//        ldp x17, x13, [x1, #48]
//        ldp x16, x5, [x2, #32]
//        ldr q18, [x1, #32]
//        ldr q28, [x2, #32]
//        ldp x20, x8, [x2, #48]
//        movi v16.2D, #0x00000000ffffffff
//        uzp2 v7.4S, v28.4S, v28.4S
//        xtn v4.2S, v18.2D
//        xtn v1.2S, v28.2D
//        rev64 v28.4S, v28.4S
//        umull v27.2D, v4.2S, v1.2S
//        umull v29.2D, v4.2S, v7.2S
//        uzp2 v21.4S, v18.4S, v18.4S
//        mul v28.4S, v28.4S, v18.4S
//        usra v29.2D, v27.2D, #32
//        umull v18.2D, v21.2S, v7.2S
//        uaddlp v28.2D, v28.4S
//        and v16.16B, v29.16B, v16.16B
//        umlal v16.2D, v21.2S, v1.2S
//        shl v28.2D, v28.2D, #32
//        usra v18.2D, v29.2D, #32
//        umlal v28.2D, v4.2S, v1.2S
//        usra v18.2D, v16.2D, #32
//        mov x9, v28.d[0]
//        mov x6, v28.d[1]
//        mul x19, x17, x20
//        mul x14, x13, x8
//        mov x11, v18.d[0]
//        adds x6, x6, x11
//        mov x11, v18.d[1]
//        adcs x19, x19, x11
//        umulh x11, x17, x20
//        adcs x14, x14, x11
//        umulh x11, x13, x8
//        adc x11, x11, xzr
//        adds x3, x6, x9
//        adcs x6, x19, x6
//        adcs x19, x14, x19
//        adcs x14, x11, x14
//        adc x11, xzr, x11
//        adds x24, x6, x9
//        adcs x4, x19, x3
//        adcs x6, x14, x6
//        adcs x19, x11, x19
//        adcs x14, xzr, x14
//        adc x11, xzr, x11
//        subs x7, x17, x13
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x8, x20
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x19, x19, x22
//        eor x7, x7, x23
//        adcs x14, x14, x7
//        adc x11, x11, x23
//        subs x7, x21, x10
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x5, x16
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x3, x3, x22
//        eor x7, x7, x23
//        adcs x24, x24, x7
//        adcs x4, x4, x23
//        adcs x6, x6, x23
//        adcs x19, x19, x23
//        adcs x14, x14, x23
//        adc x11, x11, x23
//        subs x7, x10, x13
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x8, x5
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x6, x6, x22
//        eor x7, x7, x23
//        adcs x19, x19, x7
//        adcs x14, x14, x23
//        adc x11, x11, x23
//        subs x7, x21, x17
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x20, x16
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x24, x24, x22
//        eor x7, x7, x23
//        adcs x4, x4, x7
//        adcs x6, x6, x23
//        adcs x19, x19, x23
//        adcs x14, x14, x23
//        adc x11, x11, x23
//        subs x7, x21, x13
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x8, x16
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x4, x4, x22
//        eor x7, x7, x23
//        adcs x6, x6, x7
//        adcs x19, x19, x23
//        adcs x14, x14, x23
//        adc x11, x11, x23
//        subs x7, x10, x17
//        cneg x7, x7, cc
//        csetm x23, cc
//        subs x22, x20, x5
//        cneg x22, x22, cc
//        mul x12, x7, x22
//        umulh x7, x7, x22
//        cinv x23, x23, cc
//        cmn x23, #0x1
//        eor x22, x12, x23
//        adcs x4, x4, x22
//        eor x7, x7, x23
//        adcs x6, x6, x7
//        adcs x19, x19, x23
//        adcs x14, x14, x23
//        adc x11, x11, x23
//        ldp x7, x23, [sp]                        // @slothy:reads=stack0
//        adds x9, x9, x7
//        adcs x3, x3, x23
//        stp x9, x3, [sp]                         // @slothy:writes=stack0
//        ldp x9, x3, [sp, #16]                    // @slothy:reads=stack16
//        adcs x9, x24, x9
//        adcs x3, x4, x3
//        stp x9, x3, [sp, #16]                    // @slothy:writes=stack16
//        ldp x9, x3, [sp, #32]                    // @slothy:reads=stack32
//        adcs x9, x6, x9
//        adcs x6, x19, x3
//        stp x9, x6, [sp, #32]                    // @slothy:writes=stack32
//        ldp x9, x6, [sp, #48]                    // @slothy:reads=stack48
//        adcs x9, x14, x9
//        adcs x6, x11, x6
//        stp x9, x6, [sp, #48]                    // @slothy:writes=stack48
//        ldr x9, [sp, #64]                        // @slothy:reads=stack64
//        adc x9, x9, xzr
//        str x9, [sp, #64]                        // @slothy:writes=stack64
//        ldp x9, x6, [x1]
//        subs x21, x21, x9
//        sbcs x10, x10, x6
//        ldp x9, x6, [x1, #16]
//        sbcs x17, x17, x9
//        sbcs x13, x13, x6
//        csetm x9, cc
//        ldp x6, x19, [x2]
//        subs x16, x6, x16
//        sbcs x5, x19, x5
//        ldp x6, x19, [x2, #16]
//        sbcs x20, x6, x20
//        sbcs x8, x19, x8
//        csetm x6, cc
//        eor x21, x21, x9
//        subs x21, x21, x9
//        eor x10, x10, x9
//        sbcs x10, x10, x9
//        eor x17, x17, x9
//        sbcs x17, x17, x9
//        eor x13, x13, x9
//        sbc x13, x13, x9
//        eor x16, x16, x6
//        subs x16, x16, x6
//        eor x5, x5, x6
//        sbcs x5, x5, x6
//        eor x20, x20, x6
//        sbcs x20, x20, x6
//        eor x8, x8, x6
//        sbc x8, x8, x6
//        eor x9, x6, x9
//        mul x6, x21, x16
//        mul x19, x10, x5
//        mul x14, x17, x20
//        mul x11, x13, x8
//        umulh x3, x21, x16
//        adds x19, x19, x3
//        umulh x3, x10, x5
//        adcs x14, x14, x3
//        umulh x3, x17, x20
//        adcs x11, x11, x3
//        umulh x3, x13, x8
//        adc x3, x3, xzr
//        adds x24, x19, x6
//        adcs x19, x14, x19
//        adcs x14, x11, x14
//        adcs x11, x3, x11
//        adc x3, xzr, x3
//        adds x4, x19, x6
//        adcs x7, x14, x24
//        adcs x19, x11, x19
//        adcs x14, x3, x14
//        adcs x11, xzr, x11
//        adc x3, xzr, x3
//        subs x23, x17, x13
//        cneg x23, x23, cc
//        csetm x22, cc
//        subs x12, x8, x20
//        cneg x12, x12, cc
//        mul x15, x23, x12
//        umulh x23, x23, x12
//        cinv x22, x22, cc
//        cmn x22, #0x1
//        eor x12, x15, x22
//        adcs x14, x14, x12
//        eor x23, x23, x22
//        adcs x11, x11, x23
//        adc x3, x3, x22
//        subs x23, x21, x10
//        cneg x23, x23, cc
//        csetm x22, cc
//        subs x12, x5, x16
//        cneg x12, x12, cc
//        mul x15, x23, x12
//        umulh x23, x23, x12
//        cinv x22, x22, cc
//        cmn x22, #0x1
//        eor x12, x15, x22
//        adcs x24, x24, x12
//        eor x23, x23, x22
//        adcs x4, x4, x23
//        adcs x7, x7, x22
//        adcs x19, x19, x22
//        adcs x14, x14, x22
//        adcs x11, x11, x22
//        adc x3, x3, x22
//        subs x23, x10, x13
//        cneg x23, x23, cc
//        csetm x22, cc
//        subs x12, x8, x5
//        cneg x12, x12, cc
//        mul x15, x23, x12
//        umulh x23, x23, x12
//        cinv x22, x22, cc
//        cmn x22, #0x1
//        eor x12, x15, x22
//        adcs x19, x19, x12
//        eor x23, x23, x22
//        adcs x14, x14, x23
//        adcs x11, x11, x22
//        adc x3, x3, x22
//        subs x23, x21, x17
//        cneg x23, x23, cc
//        csetm x22, cc
//        subs x12, x20, x16
//        cneg x12, x12, cc
//        mul x15, x23, x12
//        umulh x23, x23, x12
//        cinv x22, x22, cc
//        cmn x22, #0x1
//        eor x12, x15, x22
//        adcs x4, x4, x12
//        eor x23, x23, x22
//        adcs x7, x7, x23
//        adcs x19, x19, x22
//        adcs x14, x14, x22
//        adcs x11, x11, x22
//        adc x3, x3, x22
//        subs x21, x21, x13
//        cneg x21, x21, cc
//        csetm x13, cc
//        subs x16, x8, x16
//        cneg x16, x16, cc
//        mul x8, x21, x16
//        umulh x21, x21, x16
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x16, x8, x13
//        adcs x16, x7, x16
//        eor x21, x21, x13
//        adcs x21, x19, x21
//        adcs x8, x14, x13
//        adcs x19, x11, x13
//        adc x13, x3, x13
//        subs x10, x10, x17
//        cneg x10, x10, cc
//        csetm x17, cc
//        subs x5, x20, x5
//        cneg x5, x5, cc
//        mul x20, x10, x5
//        umulh x10, x10, x5
//        cinv x17, x17, cc
//        cmn x17, #0x1
//        eor x5, x20, x17
//        adcs x16, x16, x5
//        eor x10, x10, x17
//        adcs x21, x21, x10
//        adcs x10, x8, x17
//        adcs x5, x19, x17
//        adc x17, x13, x17
//        ldp x13, x20, [sp]                       // @slothy:reads=stack0
//        ldp x8, x19, [sp, #16]                   // @slothy:reads=stack16
//        eor x6, x6, x9
//        adds x6, x6, x13
//        eor x14, x24, x9
//        adcs x14, x14, x20
//        eor x11, x4, x9
//        adcs x11, x11, x8
//        eor x16, x16, x9
//        adcs x16, x16, x19
//        eor x21, x21, x9
//        ldp x3, x24, [sp, #32]                   // @slothy:reads=stack32
//        ldp x4, x7, [sp, #48]                    // @slothy:reads=stack48
//        ldr x23, [sp, #64]                       // @slothy:reads=stack64
//        adcs x21, x21, x3
//        eor x10, x10, x9
//        adcs x10, x10, x24
//        eor x5, x5, x9
//        adcs x5, x5, x4
//        eor x17, x17, x9
//        adcs x17, x17, x7
//        adc x22, x23, xzr
//        adds x21, x21, x13
//        adcs x10, x10, x20
//        adcs x13, x5, x8
//        adcs x17, x17, x19
//        and x5, x9, #0x1ff
//        lsl x20, x6, #9
//        orr x5, x20, x5
//        adcs x5, x3, x5
//        extr x20, x14, x6, #55
//        adcs x20, x24, x20
//        extr x8, x11, x14, #55
//        adcs x8, x4, x8
//        extr x9, x16, x11, #55
//        adcs x9, x7, x9
//        lsr x16, x16, #55
//        adc x16, x16, x23
//        ldr x6, [x2, #64]
//        ldp x19, x14, [x1]
//        and x11, x19, #0xfffffffffffff
//        mul x11, x6, x11
//        ldr x3, [x1, #64]
//        ldp x24, x4, [x2]
//        and x7, x24, #0xfffffffffffff
//        mul x7, x3, x7
//        add x11, x11, x7
//        extr x19, x14, x19, #52
//        and x19, x19, #0xfffffffffffff
//        mul x19, x6, x19
//        extr x24, x4, x24, #52
//        and x24, x24, #0xfffffffffffff
//        mul x24, x3, x24
//        add x19, x19, x24
//        lsr x24, x11, #52
//        add x19, x19, x24
//        lsl x11, x11, #12
//        extr x11, x19, x11, #12
//        adds x21, x21, x11
//        ldp x11, x24, [x1, #16]
//        ldp x7, x23, [x2, #16]
//        extr x14, x11, x14, #40
//        and x14, x14, #0xfffffffffffff
//        mul x14, x6, x14
//        extr x4, x7, x4, #40
//        and x4, x4, #0xfffffffffffff
//        mul x4, x3, x4
//        add x14, x14, x4
//        lsr x4, x19, #52
//        add x14, x14, x4
//        lsl x19, x19, #12
//        extr x19, x14, x19, #24
//        adcs x10, x10, x19
//        extr x19, x24, x11, #28
//        and x19, x19, #0xfffffffffffff
//        mul x19, x6, x19
//        extr x11, x23, x7, #28
//        and x11, x11, #0xfffffffffffff
//        mul x11, x3, x11
//        add x19, x19, x11
//        lsr x11, x14, #52
//        add x19, x19, x11
//        lsl x14, x14, #12
//        extr x14, x19, x14, #36
//        adcs x13, x13, x14
//        and x14, x10, x13
//        ldp x11, x4, [x1, #32]
//        ldp x7, x12, [x2, #32]
//        extr x24, x11, x24, #16
//        and x24, x24, #0xfffffffffffff
//        mul x24, x6, x24
//        extr x23, x7, x23, #16
//        and x23, x23, #0xfffffffffffff
//        mul x23, x3, x23
//        add x24, x24, x23
//        lsl x23, x22, #48
//        add x24, x24, x23
//        lsr x23, x19, #52
//        add x24, x24, x23
//        lsl x19, x19, #12
//        extr x19, x24, x19, #48
//        adcs x17, x17, x19
//        and x19, x14, x17
//        lsr x14, x11, #4
//        and x14, x14, #0xfffffffffffff
//        mul x14, x6, x14
//        lsr x23, x7, #4
//        and x23, x23, #0xfffffffffffff
//        mul x23, x3, x23
//        add x14, x14, x23
//        lsr x23, x24, #52
//        add x14, x14, x23
//        lsl x24, x24, #12
//        extr x24, x14, x24, #60
//        extr x11, x4, x11, #56
//        and x11, x11, #0xfffffffffffff
//        mul x11, x6, x11
//        extr x7, x12, x7, #56
//        and x7, x7, #0xfffffffffffff
//        mul x7, x3, x7
//        add x11, x11, x7
//        lsr x14, x14, #52
//        add x14, x11, x14
//        lsl x11, x24, #8
//        extr x11, x14, x11, #8
//        adcs x5, x5, x11
//        and x19, x19, x5
//        ldp x11, x24, [x1, #48]
//        ldp x2, x7, [x2, #48]
//        extr x4, x11, x4, #44
//        and x4, x4, #0xfffffffffffff
//        mul x4, x6, x4
//        extr x23, x2, x12, #44
//        and x23, x23, #0xfffffffffffff
//        mul x23, x3, x23
//        add x4, x4, x23
//        lsr x23, x14, #52
//        add x4, x4, x23
//        lsl x14, x14, #12
//        extr x14, x4, x14, #20
//        adcs x20, x20, x14
//        and x19, x19, x20
//        extr x14, x24, x11, #32
//        and x14, x14, #0xfffffffffffff
//        mul x14, x6, x14
//        extr x2, x7, x2, #32
//        and x2, x2, #0xfffffffffffff
//        mul x2, x3, x2
//        add x2, x14, x2
//        lsr x14, x4, #52
//        add x2, x2, x14
//        lsl x14, x4, #12
//        extr x14, x2, x14, #32
//        adcs x8, x8, x14
//        and x19, x19, x8
//        lsr x14, x24, #20
//        mul x14, x6, x14
//        lsr x11, x7, #20
//        mul x11, x3, x11
//        add x14, x14, x11
//        lsr x11, x2, #52
//        add x14, x14, x11
//        lsl x2, x2, #12
//        extr x2, x14, x2, #44
//        adcs x9, x9, x2
//        and x2, x19, x9
//        mul x6, x6, x3
//        lsr x19, x14, #44
//        add x6, x6, x19
//        adc x16, x16, x6
//        lsr x6, x16, #9
//        orr x16, x16, #0xfffffffffffffe00
//        cmp xzr, xzr
//        adcs xzr, x21, x6
//        adcs xzr, x2, xzr
//        adcs xzr, x16, xzr
//        adcs x21, x21, x6
//        adcs x10, x10, xzr
//        adcs x13, x13, xzr
//        adcs x17, x17, xzr
//        adcs x5, x5, xzr
//        adcs x20, x20, xzr
//        adcs x8, x8, xzr
//        adcs x9, x9, xzr
//        adc x16, x16, xzr
//        and x2, x21, #0x1ff
//        extr x21, x10, x21, #9
//        extr x10, x13, x10, #9
//        stp x21, x10, [x0]                       // @slothy:writes=buffer0
//        extr x21, x17, x13, #9
//        extr x10, x5, x17, #9
//        stp x21, x10, [x0, #16]                  // @slothy:writes=buffer16
//        extr x21, x20, x5, #9
//        extr x10, x8, x20, #9
//        stp x21, x10, [x0, #32]                  // @slothy:writes=buffer32
//        extr x21, x9, x8, #9
//        extr x10, x16, x9, #9
//        stp x21, x10, [x0, #48]                  // @slothy:writes=buffer48
//        str x2, [x0, #64]                        // @slothy:writes=buffer64
//        add     sp, sp, #80
//        ldp     x25, x26, [sp], #16
//        ldp     x23, x24, [sp], #16
//        ldp     x21, x22, [sp], #16
//        ldp     x19, x20, [sp], #16
//        ret
//
// The bash script used for step 2 is as follows:
//
//        # Store the assembly instructions except the last 'ret',
//        # callee-register store/loads and add/sub sp #80 as, say, 'input.S'.
//        export OUTPUTS="[hint_buffer0,hint_buffer16,hint_buffer32,hint_buffer48,hint_buffer64]"
//        export RESERVED_REGS="[x18,x27,x28,x29,x30,sp,q8,q9,q10,q11,q12,q13,q14,q15,v8,v9,v10,v11,v12,v13,v14,v15]"
//        <s2n-bignum>/tools/external/slothy.sh input.S my_out_dir
//        # my_out_dir/3.opt.s is the optimized assembly. Its output may differ
//        # from this file since the sequence is non-deterministically chosen.
//        # Please add 'ret' at the end of the output assembly.

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p521)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_mul_p521):
        CFI_START

// Save registers and make space for the temporary buffer

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(80)

        ldr q6, [x2]
        ldp x10, x17, [x1, #16]
        ldr q4, [x1]
        ldr q16, [x2, #32]
        ldp x5, x20, [x2, #16]
        ldr q2, [x1, #32]
        movi v31.2D, #0x00000000ffffffff
        uzp2 v17.4S, v6.4S, v6.4S
        rev64 v7.4S, v6.4S
        ldp x15, x21, [x1]
        xtn v25.2S, v6.2D
        xtn v22.2S, v4.2D
        subs x14, x10, x17
        mul v7.4S, v7.4S, v4.4S
        csetm x8, cc
        rev64 v3.4S, v16.4S
        xtn v1.2S, v16.2D
        ldp x13, x16, [x2]
        mul x26, x10, x5
        uzp2 v16.4S, v16.4S, v16.4S
        uaddlp v26.2D, v7.4S
        cneg x4, x14, cc
        subs x24, x15, x21
        xtn v5.2S, v2.2D
        mul v28.4S, v3.4S, v2.4S
        shl v26.2D, v26.2D, #32
        mul x22, x17, x20
        umull v20.2D, v22.2S, v25.2S
        uzp2 v6.4S, v4.4S, v4.4S
        umull v18.2D, v22.2S, v17.2S
        uzp2 v4.4S, v2.4S, v2.4S
        cneg x14, x24, cc
        csetm x7, cc
        umulh x11, x17, x20
        usra v18.2D, v20.2D, #32
        uaddlp v7.2D, v28.4S
        subs x19, x16, x13
        umlal v26.2D, v22.2S, v25.2S
        cneg x19, x19, cc
        shl v28.2D, v7.2D, #32
        umull v7.2D, v5.2S, v1.2S
        umull v30.2D, v5.2S, v16.2S
        cinv x6, x7, cc
        mul x25, x14, x19
        umlal v28.2D, v5.2S, v1.2S
        umull v21.2D, v6.2S, v17.2S
        umulh x14, x14, x19
        usra v30.2D, v7.2D, #32
        subs x9, x20, x5
        and v29.16B, v18.16B, v31.16B
        cinv x23, x8, cc
        mov x8, v26.d[1]
        cneg x12, x9, cc
        usra v21.2D, v18.2D, #32
        umlal v29.2D, v6.2S, v25.2S
        mul x24, x4, x12
        umull v18.2D, v4.2S, v16.2S
        movi v25.2D, #0x00000000ffffffff
        eor x9, x14, x6
        and v7.16B, v30.16B, v25.16B
        usra v21.2D, v29.2D, #32
        umulh x7, x10, x5
        usra v18.2D, v30.2D, #32
        umlal v7.2D, v4.2S, v1.2S
        mov x19, v21.d[0]
        umulh x3, x4, x12
        mov x14, v21.d[1]
        usra v18.2D, v7.2D, #32
        adds x4, x8, x19
        mov x8, v26.d[0]
        adcs x19, x26, x14
        adcs x14, x22, x7
        adc x12, x11, xzr
        adds x11, x4, x8
        adcs x26, x19, x4
        adcs x22, x14, x19
        eor x4, x24, x23
        adcs x14, x12, x14
        eor x7, x25, x6
        adc x25, xzr, x12
        eor x19, x3, x23
        adds x3, x26, x8
        adcs x24, x22, x11
        adcs x12, x14, x26
        adcs x22, x25, x22
        adcs x26, xzr, x14
        adc x14, xzr, x25
        cmn x23, #0x1
        adcs x22, x22, x4
        adcs x19, x26, x19
        adc x25, x14, x23
        subs x14, x21, x17
        cneg x23, x14, cc
        csetm x26, cc
        subs x4, x20, x16
        cneg x14, x4, cc
        cinv x4, x26, cc
        cmn x6, #0x1
        adcs x11, x11, x7
        mul x7, x23, x14
        adcs x9, x3, x9
        adcs x26, x24, x6
        umulh x3, x23, x14
        adcs x14, x12, x6
        adcs x22, x22, x6
        adcs x12, x19, x6
        extr x24, x11, x8, #55
        adc x6, x25, x6
        subs x19, x15, x17
        csetm x17, cc
        cneg x23, x19, cc
        subs x19, x20, x13
        lsl x25, x8, #9
        eor x8, x7, x4
        cneg x20, x19, cc
        umulh x7, x23, x20
        cinv x19, x17, cc
        subs x17, x15, x10
        csetm x15, cc
        stp x25, x24, [sp, #32]
        cneg x24, x17, cc
        mul x20, x23, x20
        subs x25, x5, x13
        cneg x13, x25, cc
        cinv x15, x15, cc
        mul x25, x24, x13
        subs x21, x21, x10
        csetm x23, cc
        cneg x17, x21, cc
        subs x21, x5, x16
        umulh x13, x24, x13
        cinv x10, x23, cc
        cneg x23, x21, cc
        cmn x4, #0x1
        adcs x14, x14, x8
        eor x21, x3, x4
        adcs x21, x22, x21
        eor x5, x20, x19
        adcs x24, x12, x4
        mul x12, x17, x23
        eor x8, x25, x15
        adc x25, x6, x4
        cmn x15, #0x1
        adcs x6, x9, x8
        ldp x20, x8, [x2, #48]
        eor x9, x13, x15
        adcs x4, x26, x9
        umulh x26, x17, x23
        ldp x17, x13, [x1, #48]
        adcs x9, x14, x15
        adcs x16, x21, x15
        adcs x14, x24, x15
        eor x21, x7, x19
        mul x23, x17, x20
        adc x24, x25, x15
        cmn x19, #0x1
        adcs x7, x4, x5
        adcs x9, x9, x21
        umulh x3, x13, x8
        adcs x16, x16, x19
        adcs x22, x14, x19
        eor x5, x12, x10
        adc x12, x24, x19
        cmn x10, #0x1
        adcs x19, x7, x5
        eor x14, x26, x10
        mov x7, v28.d[1]
        adcs x24, x9, x14
        extr x4, x19, x6, #55
        umulh x15, x17, x20
        mov x14, v18.d[1]
        lsr x9, x19, #55
        adcs x5, x16, x10
        mov x16, v18.d[0]
        adcs x19, x22, x10
        str x9, [sp, #64]
        extr x25, x6, x11, #55
        adc x21, x12, x10
        subs x26, x17, x13
        stp x25, x4, [sp, #48]
        stp x19, x21, [sp, #16]
        csetm x6, cc
        cneg x4, x26, cc
        mul x19, x13, x8
        subs x11, x8, x20
        stp x24, x5, [sp]
        ldp x21, x10, [x1, #32]
        cinv x12, x6, cc
        cneg x6, x11, cc
        mov x9, v28.d[0]
        umulh x25, x4, x6
        adds x22, x7, x16
        ldp x16, x5, [x2, #32]
        adcs x14, x23, x14
        adcs x11, x19, x15
        adc x24, x3, xzr
        adds x3, x22, x9
        adcs x15, x14, x22
        mul x22, x4, x6
        adcs x6, x11, x14
        adcs x4, x24, x11
        eor x14, x25, x12
        adc x26, xzr, x24
        subs x7, x21, x10
        csetm x23, cc
        cneg x19, x7, cc
        subs x24, x5, x16
        cneg x11, x24, cc
        cinv x7, x23, cc
        adds x25, x15, x9
        eor x23, x22, x12
        adcs x22, x6, x3
        mul x24, x19, x11
        adcs x15, x4, x15
        adcs x6, x26, x6
        umulh x19, x19, x11
        adcs x11, xzr, x4
        adc x26, xzr, x26
        cmn x12, #0x1
        adcs x4, x6, x23
        eor x6, x24, x7
        adcs x14, x11, x14
        adc x26, x26, x12
        subs x11, x10, x13
        cneg x12, x11, cc
        csetm x11, cc
        eor x19, x19, x7
        subs x24, x8, x5
        cinv x11, x11, cc
        cneg x24, x24, cc
        cmn x7, #0x1
        adcs x3, x3, x6
        mul x23, x12, x24
        adcs x25, x25, x19
        adcs x6, x22, x7
        umulh x19, x12, x24
        adcs x22, x15, x7
        adcs x12, x4, x7
        eor x24, x23, x11
        adcs x4, x14, x7
        adc x26, x26, x7
        eor x19, x19, x11
        subs x14, x21, x17
        cneg x7, x14, cc
        csetm x14, cc
        subs x23, x20, x16
        cinv x14, x14, cc
        cneg x23, x23, cc
        cmn x11, #0x1
        adcs x22, x22, x24
        mul x24, x7, x23
        adcs x15, x12, x19
        adcs x4, x4, x11
        adc x19, x26, x11
        umulh x26, x7, x23
        subs x7, x21, x13
        eor x11, x24, x14
        cneg x23, x7, cc
        csetm x12, cc
        subs x7, x8, x16
        cneg x7, x7, cc
        cinv x12, x12, cc
        cmn x14, #0x1
        eor x26, x26, x14
        adcs x11, x25, x11
        mul x25, x23, x7
        adcs x26, x6, x26
        adcs x6, x22, x14
        adcs x24, x15, x14
        umulh x23, x23, x7
        adcs x4, x4, x14
        adc x22, x19, x14
        eor x14, x25, x12
        eor x7, x23, x12
        cmn x12, #0x1
        adcs x14, x26, x14
        ldp x19, x25, [x2]
        ldp x15, x23, [x2, #16]
        adcs x26, x6, x7
        adcs x24, x24, x12
        adcs x7, x4, x12
        adc x4, x22, x12
        subs x19, x19, x16
        ldp x16, x22, [x1]
        sbcs x6, x25, x5
        ldp x12, x25, [x1, #16]
        sbcs x15, x15, x20
        sbcs x8, x23, x8
        csetm x23, cc
        subs x21, x21, x16
        eor x16, x19, x23
        sbcs x19, x10, x22
        eor x22, x6, x23
        eor x8, x8, x23
        sbcs x6, x17, x12
        sbcs x13, x13, x25
        csetm x12, cc
        subs x10, x10, x17
        cneg x17, x10, cc
        csetm x25, cc
        subs x5, x20, x5
        eor x10, x19, x12
        cneg x19, x5, cc
        eor x20, x15, x23
        eor x21, x21, x12
        cinv x15, x25, cc
        mul x25, x17, x19
        subs x16, x16, x23
        sbcs x5, x22, x23
        eor x6, x6, x12
        sbcs x20, x20, x23
        eor x22, x13, x12
        sbc x8, x8, x23
        subs x21, x21, x12
        umulh x19, x17, x19
        sbcs x10, x10, x12
        sbcs x17, x6, x12
        eor x6, x19, x15
        eor x19, x25, x15
        umulh x25, x17, x20
        sbc x13, x22, x12
        cmn x15, #0x1
        adcs x22, x14, x19
        adcs x19, x26, x6
        ldp x6, x26, [sp]
        adcs x14, x24, x15
        umulh x24, x21, x16
        adcs x7, x7, x15
        adc x15, x4, x15
        adds x4, x9, x6
        eor x9, x23, x12
        adcs x12, x3, x26
        stp x4, x12, [sp]
        ldp x4, x26, [sp, #16]
        umulh x12, x10, x5
        ldp x6, x23, [sp, #32]
        adcs x3, x11, x4
        mul x4, x13, x8
        adcs x26, x22, x26
        ldp x22, x11, [sp, #48]
        adcs x6, x19, x6
        stp x3, x26, [sp, #16]
        mul x26, x10, x5
        adcs x14, x14, x23
        stp x6, x14, [sp, #32]
        ldr x6, [sp, #64]
        adcs x22, x7, x22
        adcs x14, x15, x11
        mul x11, x17, x20
        adc x19, x6, xzr
        stp x22, x14, [sp, #48]
        adds x14, x26, x24
        str x19, [sp, #64]
        umulh x19, x13, x8
        adcs x7, x11, x12
        adcs x22, x4, x25
        mul x6, x21, x16
        adc x19, x19, xzr
        subs x11, x17, x13
        cneg x12, x11, cc
        csetm x11, cc
        subs x24, x8, x20
        cinv x11, x11, cc
        cneg x24, x24, cc
        adds x4, x14, x6
        adcs x14, x7, x14
        mul x3, x12, x24
        adcs x7, x22, x7
        adcs x22, x19, x22
        umulh x12, x12, x24
        adc x24, xzr, x19
        adds x19, x14, x6
        eor x3, x3, x11
        adcs x26, x7, x4
        adcs x14, x22, x14
        adcs x25, x24, x7
        adcs x23, xzr, x22
        eor x7, x12, x11
        adc x12, xzr, x24
        subs x22, x21, x10
        cneg x24, x22, cc
        csetm x22, cc
        subs x15, x5, x16
        cinv x22, x22, cc
        cneg x15, x15, cc
        cmn x11, #0x1
        adcs x3, x25, x3
        mul x25, x24, x15
        adcs x23, x23, x7
        adc x11, x12, x11
        subs x7, x10, x13
        umulh x15, x24, x15
        cneg x12, x7, cc
        csetm x7, cc
        eor x24, x25, x22
        eor x25, x15, x22
        cmn x22, #0x1
        adcs x24, x4, x24
        adcs x19, x19, x25
        adcs x15, x26, x22
        adcs x4, x14, x22
        adcs x26, x3, x22
        adcs x25, x23, x22
        adc x23, x11, x22
        subs x14, x21, x17
        cneg x3, x14, cc
        csetm x11, cc
        subs x14, x8, x5
        cneg x14, x14, cc
        cinv x7, x7, cc
        subs x13, x21, x13
        cneg x21, x13, cc
        csetm x13, cc
        mul x22, x12, x14
        subs x8, x8, x16
        cinv x13, x13, cc
        umulh x14, x12, x14
        cneg x12, x8, cc
        subs x8, x20, x16
        cneg x8, x8, cc
        cinv x16, x11, cc
        eor x22, x22, x7
        cmn x7, #0x1
        eor x14, x14, x7
        adcs x4, x4, x22
        mul x11, x3, x8
        adcs x22, x26, x14
        adcs x14, x25, x7
        eor x25, x24, x9
        adc x26, x23, x7
        umulh x7, x3, x8
        subs x17, x10, x17
        cneg x24, x17, cc
        eor x3, x11, x16
        csetm x11, cc
        subs x20, x20, x5
        cneg x5, x20, cc
        cinv x11, x11, cc
        cmn x16, #0x1
        mul x17, x21, x12
        eor x8, x7, x16
        adcs x10, x19, x3
        and x19, x9, #0x1ff
        adcs x20, x15, x8
        umulh x15, x21, x12
        eor x12, x10, x9
        eor x8, x6, x9
        adcs x6, x4, x16
        adcs x4, x22, x16
        adcs x21, x14, x16
        adc x7, x26, x16
        mul x10, x24, x5
        cmn x13, #0x1
        ldp x3, x14, [x1]
        eor x17, x17, x13
        umulh x5, x24, x5
        adcs x20, x20, x17
        eor x17, x15, x13
        adcs x16, x6, x17
        eor x22, x10, x11
        adcs x23, x4, x13
        extr x10, x14, x3, #52
        and x26, x3, #0xfffffffffffff
        adcs x24, x21, x13
        and x15, x10, #0xfffffffffffff
        adc x6, x7, x13
        cmn x11, #0x1
        adcs x17, x20, x22
        eor x4, x5, x11
        ldp x21, x10, [sp]
        adcs x7, x16, x4
        eor x16, x17, x9
        eor x13, x7, x9
        ldp x3, x17, [sp, #16]
        adcs x7, x23, x11
        eor x23, x7, x9
        ldp x5, x22, [sp, #32]
        adcs x7, x24, x11
        adc x24, x6, x11
        ldr x6, [x2, #64]
        adds x20, x8, x21
        lsl x11, x20, #9
        eor x4, x7, x9
        orr x7, x11, x19
        eor x8, x24, x9
        adcs x11, x25, x10
        mul x26, x6, x26
        ldp x19, x24, [sp, #48]
        adcs x12, x12, x3
        adcs x16, x16, x17
        adcs x9, x13, x5
        ldr x25, [sp, #64]
        extr x20, x11, x20, #55
        adcs x13, x23, x22
        adcs x4, x4, x19
        extr x23, x12, x11, #55
        adcs x8, x8, x24
        adc x11, x25, xzr
        adds x21, x9, x21
        extr x9, x16, x12, #55
        lsr x12, x16, #55
        adcs x10, x13, x10
        mul x15, x6, x15
        adcs x13, x4, x3
        ldp x16, x4, [x2]
        ldr x3, [x1, #64]
        adcs x17, x8, x17
        adcs x5, x5, x7
        adcs x20, x22, x20
        adcs x8, x19, x23
        and x22, x16, #0xfffffffffffff
        ldp x19, x7, [x1, #16]
        adcs x9, x24, x9
        extr x24, x4, x16, #52
        adc x16, x12, x25
        mul x22, x3, x22
        and x25, x24, #0xfffffffffffff
        extr x14, x19, x14, #40
        and x12, x14, #0xfffffffffffff
        extr x23, x7, x19, #28
        ldp x19, x24, [x2, #16]
        mul x14, x3, x25
        and x23, x23, #0xfffffffffffff
        add x22, x26, x22
        lsl x11, x11, #48
        lsr x26, x22, #52
        lsl x25, x22, #12
        mul x22, x6, x12
        extr x12, x19, x4, #40
        add x4, x15, x14
        mul x15, x6, x23
        add x4, x4, x26
        extr x23, x24, x19, #28
        ldp x14, x19, [x1, #32]
        and x26, x12, #0xfffffffffffff
        extr x12, x4, x25, #12
        and x25, x23, #0xfffffffffffff
        adds x21, x21, x12
        mul x12, x3, x26
        extr x23, x14, x7, #16
        and x23, x23, #0xfffffffffffff
        mul x7, x3, x25
        ldp x25, x26, [x2, #32]
        add x12, x22, x12
        extr x22, x19, x14, #56
        mul x23, x6, x23
        lsr x14, x14, #4
        extr x24, x25, x24, #16
        add x7, x15, x7
        and x15, x24, #0xfffffffffffff
        and x22, x22, #0xfffffffffffff
        lsr x24, x4, #52
        mul x15, x3, x15
        and x14, x14, #0xfffffffffffff
        add x12, x12, x24
        lsl x24, x4, #12
        lsr x4, x12, #52
        extr x24, x12, x24, #24
        adcs x10, x10, x24
        lsl x24, x12, #12
        add x12, x7, x4
        mul x22, x6, x22
        add x4, x23, x15
        extr x7, x12, x24, #36
        adcs x13, x13, x7
        lsl x15, x12, #12
        add x7, x4, x11
        lsr x24, x12, #52
        ldp x23, x11, [x2, #48]
        add x4, x7, x24
        mul x12, x6, x14
        extr x7, x26, x25, #56
        extr x14, x4, x15, #48
        and x2, x7, #0xfffffffffffff
        extr x24, x11, x23, #32
        ldp x15, x7, [x1, #48]
        and x1, x24, #0xfffffffffffff
        lsr x24, x4, #52
        mul x2, x3, x2
        extr x26, x23, x26, #44
        lsr x23, x25, #4
        and x23, x23, #0xfffffffffffff
        and x25, x26, #0xfffffffffffff
        extr x26, x7, x15, #32
        extr x19, x15, x19, #44
        mul x23, x3, x23
        and x15, x26, #0xfffffffffffff
        lsl x26, x4, #12
        and x4, x19, #0xfffffffffffff
        lsr x11, x11, #20
        mul x19, x6, x4
        adcs x17, x17, x14
        add x14, x22, x2
        add x22, x12, x23
        lsr x7, x7, #20
        add x22, x22, x24
        extr x2, x22, x26, #60
        mul x24, x3, x25
        lsr x22, x22, #52
        add x14, x14, x22
        lsl x22, x2, #8
        extr x22, x14, x22, #8
        lsl x2, x14, #12
        mul x1, x3, x1
        adcs x12, x5, x22
        mul x5, x6, x15
        and x26, x10, x13
        and x4, x26, x17
        add x23, x19, x24
        lsr x14, x14, #52
        mul x22, x3, x11
        add x11, x23, x14
        extr x25, x11, x2, #20
        lsl x19, x11, #12
        adcs x25, x20, x25
        and x14, x4, x12
        add x1, x5, x1
        and x14, x14, x25
        mul x15, x6, x7
        add x26, x15, x22
        mul x6, x6, x3
        lsr x22, x11, #52
        add x4, x1, x22
        lsr x1, x4, #52
        extr x3, x4, x19, #32
        lsl x15, x4, #12
        add x7, x26, x1
        adcs x23, x8, x3
        extr x20, x7, x15, #44
        and x3, x14, x23
        lsr x19, x7, #44
        adcs x7, x9, x20
        add x11, x6, x19
        adc x4, x16, x11
        lsr x14, x4, #9
        cmp xzr, xzr
        and x15, x3, x7
        orr x3, x4, #0xfffffffffffffe00
        adcs xzr, x21, x14
        adcs xzr, x15, xzr
        adcs xzr, x3, xzr
        adcs x11, x21, x14
        and x14, x11, #0x1ff
        adcs x1, x10, xzr
        extr x10, x1, x11, #9
        str x14, [x0, #64]
        adcs x14, x13, xzr
        extr x11, x14, x1, #9
        adcs x1, x17, xzr
        extr x4, x1, x14, #9
        stp x10, x11, [x0]
        adcs x11, x12, xzr
        extr x14, x11, x1, #9
        adcs x10, x25, xzr
        extr x11, x10, x11, #9
        stp x4, x14, [x0, #16]
        adcs x14, x23, xzr
        extr x10, x14, x10, #9
        adcs x1, x7, xzr
        stp x11, x10, [x0, #32]
        extr x14, x1, x14, #9
        adc x10, x3, xzr
        extr x26, x10, x1, #9
        stp x14, x26, [x0, #48]

// Restore regs and return

        CFI_INC_SP(80)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
