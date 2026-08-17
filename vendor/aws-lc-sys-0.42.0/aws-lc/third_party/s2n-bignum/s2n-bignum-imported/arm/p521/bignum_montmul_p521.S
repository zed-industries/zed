// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^576) mod p_521
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_montmul_p521(uint64_t z[static 9],
//                                    const uint64_t x[static 9],
//                                    const uint64_t y[static 9]);
//
// Does z := (x * y / 2^576) mod p_521, assuming x < p_521, y < p_521. This
// means the Montgomery base is the "native size" 2^{9*64} = 2^576; since
// p_521 is a Mersenne prime the basic modular multiplication bignum_mul_p521
// can be considered a Montgomery operation to base 2^521.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// bignum_montmul_p521 is functionally equivalent to
// unopt/bignum_montmul_p521_base.
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
//        ldp x14, x7, [x1]
//        ldp x3, x25, [x1, #16]
//        ldp x10, x24, [x2]
//        ldr q0, [x1]
//        ldr q25, [x2]
//        ldp x12, x6, [x2, #16]
//        movi v18.2D, #0x00000000ffffffff
//        uzp2 v3.4S, v25.4S, v25.4S
//        xtn v26.2S, v0.2D
//        xtn v22.2S, v25.2D
//        rev64 v24.4S, v25.4S
//        umull v19.2D, v26.2S, v22.2S
//        umull v25.2D, v26.2S, v3.2S
//        uzp2 v20.4S, v0.4S, v0.4S
//        mul v0.4S, v24.4S, v0.4S
//        usra v25.2D, v19.2D, #32
//        umull v6.2D, v20.2S, v3.2S
//        uaddlp v0.2D, v0.4S
//        and v18.16B, v25.16B, v18.16B
//        umlal v18.2D, v20.2S, v22.2S
//        shl v0.2D, v0.2D, #32
//        usra v6.2D, v25.2D, #32
//        umlal v0.2D, v26.2S, v22.2S
//        usra v6.2D, v18.2D, #32
//        mov x23, v0.d[0]
//        mov x16, v0.d[1]
//        mul x5, x3, x12
//        mul x21, x25, x6
//        mov x19, v6.d[0]
//        adds x16, x16, x19
//        mov x19, v6.d[1]
//        adcs x5, x5, x19
//        umulh x19, x3, x12
//        adcs x21, x21, x19
//        umulh x19, x25, x6
//        adc x19, x19, xzr
//        adds x8, x16, x23
//        adcs x16, x5, x16
//        adcs x5, x21, x5
//        adcs x21, x19, x21
//        adc x19, xzr, x19
//        adds x11, x16, x23
//        adcs x15, x5, x8
//        adcs x16, x21, x16
//        adcs x5, x19, x5
//        adcs x21, xzr, x21
//        adc x19, xzr, x19
//        subs x20, x3, x25
//        cneg x20, x20, cc
//        csetm x9, cc
//        subs x13, x6, x12
//        cneg x13, x13, cc
//        mul x26, x20, x13
//        umulh x20, x20, x13
//        cinv x9, x9, cc
//        cmn x9, #0x1
//        eor x13, x26, x9
//        adcs x5, x5, x13
//        eor x20, x20, x9
//        adcs x21, x21, x20
//        adc x19, x19, x9
//        subs x20, x14, x7
//        cneg x20, x20, cc
//        csetm x9, cc
//        subs x13, x24, x10
//        cneg x13, x13, cc
//        mul x26, x20, x13
//        umulh x20, x20, x13
//        cinv x9, x9, cc
//        cmn x9, #0x1
//        eor x13, x26, x9
//        adcs x8, x8, x13
//        eor x20, x20, x9
//        adcs x11, x11, x20
//        adcs x15, x15, x9
//        adcs x16, x16, x9
//        adcs x5, x5, x9
//        adcs x21, x21, x9
//        adc x19, x19, x9
//        subs x20, x7, x25
//        cneg x20, x20, cc
//        csetm x9, cc
//        subs x13, x6, x24
//        cneg x13, x13, cc
//        mul x26, x20, x13
//        umulh x20, x20, x13
//        cinv x9, x9, cc
//        cmn x9, #0x1
//        eor x13, x26, x9
//        adcs x16, x16, x13
//        eor x20, x20, x9
//        adcs x5, x5, x20
//        adcs x21, x21, x9
//        adc x19, x19, x9
//        subs x20, x14, x3
//        cneg x20, x20, cc
//        csetm x9, cc
//        subs x13, x12, x10
//        cneg x13, x13, cc
//        mul x26, x20, x13
//        umulh x20, x20, x13
//        cinv x9, x9, cc
//        cmn x9, #0x1
//        eor x13, x26, x9
//        adcs x11, x11, x13
//        eor x20, x20, x9
//        adcs x15, x15, x20
//        adcs x16, x16, x9
//        adcs x5, x5, x9
//        adcs x21, x21, x9
//        adc x19, x19, x9
//        subs x25, x14, x25
//        cneg x25, x25, cc
//        csetm x20, cc
//        subs x10, x6, x10
//        cneg x10, x10, cc
//        mul x6, x25, x10
//        umulh x25, x25, x10
//        cinv x10, x20, cc
//        cmn x10, #0x1
//        eor x6, x6, x10
//        adcs x6, x15, x6
//        eor x25, x25, x10
//        adcs x25, x16, x25
//        adcs x16, x5, x10
//        adcs x5, x21, x10
//        adc x10, x19, x10
//        subs x7, x7, x3
//        cneg x7, x7, cc
//        csetm x3, cc
//        subs x24, x12, x24
//        cneg x24, x24, cc
//        mul x12, x7, x24
//        umulh x7, x7, x24
//        cinv x3, x3, cc
//        cmn x3, #0x1
//        eor x24, x12, x3
//        adcs x24, x6, x24
//        eor x7, x7, x3
//        adcs x7, x25, x7
//        adcs x25, x16, x3
//        adcs x12, x5, x3
//        adc x3, x10, x3
//        lsl x10, x23, #9
//        extr x6, x8, x23, #55
//        extr x23, x11, x8, #55
//        extr x16, x24, x11, #55
//        lsr x24, x24, #55
//        stp x7, x25, [sp]                        // @slothy:writes=stack0
//        stp x12, x3, [sp, #16]                   // @slothy:writes=stack16
//        stp x10, x6, [sp, #32]                   // @slothy:writes=stack32
//        stp x23, x16, [sp, #48]                  // @slothy:writes=stack48
//        str x24, [sp, #64]                       // @slothy:writes=stack64
//        ldp x7, x3, [x1, #32]
//        ldr q0, [x1, #32]
//        ldp x25, x10, [x1, #48]
//        ldp x24, x12, [x2, #32]
//        ldr q25, [x2, #32]
//        ldp x6, x23, [x2, #48]
//        ldr q18, [x1, #48]
//        ldr q3, [x2, #48]
//        uzp1 v26.4S, v25.4S, v0.4S
//        rev64 v25.4S, v25.4S
//        uzp1 v22.4S, v0.4S, v0.4S
//        mul v0.4S, v25.4S, v0.4S
//        uaddlp v0.2D, v0.4S
//        shl v0.2D, v0.2D, #32
//        umlal v0.2D, v22.2S, v26.2S
//        mov x16, v0.d[0]
//        mov x5, v0.d[1]
//        movi v0.2D, #0x00000000ffffffff
//        uzp2 v25.4S, v3.4S, v3.4S
//        xtn v26.2S, v18.2D
//        xtn v22.2S, v3.2D
//        rev64 v24.4S, v3.4S
//        umull v19.2D, v26.2S, v22.2S
//        umull v3.2D, v26.2S, v25.2S
//        uzp2 v20.4S, v18.4S, v18.4S
//        mul v18.4S, v24.4S, v18.4S
//        usra v3.2D, v19.2D, #32
//        umull v6.2D, v20.2S, v25.2S
//        uaddlp v25.2D, v18.4S
//        and v0.16B, v3.16B, v0.16B
//        umlal v0.2D, v20.2S, v22.2S
//        shl v25.2D, v25.2D, #32
//        usra v6.2D, v3.2D, #32
//        umlal v25.2D, v26.2S, v22.2S
//        usra v6.2D, v0.2D, #32
//        mov x21, v25.d[0]
//        mov x19, v25.d[1]
//        umulh x8, x7, x24
//        adds x5, x5, x8
//        umulh x8, x3, x12
//        adcs x21, x21, x8
//        mov x8, v6.d[0]
//        adcs x19, x19, x8
//        mov x8, v6.d[1]
//        adc x8, x8, xzr
//        adds x11, x5, x16
//        adcs x5, x21, x5
//        adcs x21, x19, x21
//        adcs x19, x8, x19
//        adc x8, xzr, x8
//        adds x15, x5, x16
//        adcs x20, x21, x11
//        adcs x5, x19, x5
//        adcs x21, x8, x21
//        adcs x19, xzr, x19
//        adc x8, xzr, x8
//        subs x9, x25, x10
//        cneg x9, x9, cc
//        csetm x13, cc
//        subs x26, x23, x6
//        cneg x26, x26, cc
//        mul x22, x9, x26
//        umulh x9, x9, x26
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x26, x22, x13
//        adcs x21, x21, x26
//        eor x9, x9, x13
//        adcs x19, x19, x9
//        adc x8, x8, x13
//        subs x9, x7, x3
//        cneg x9, x9, cc
//        csetm x13, cc
//        subs x26, x12, x24
//        cneg x26, x26, cc
//        mul x22, x9, x26
//        umulh x9, x9, x26
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x26, x22, x13
//        adcs x11, x11, x26
//        eor x9, x9, x13
//        adcs x15, x15, x9
//        adcs x20, x20, x13
//        adcs x5, x5, x13
//        adcs x21, x21, x13
//        adcs x19, x19, x13
//        adc x8, x8, x13
//        subs x9, x3, x10
//        cneg x9, x9, cc
//        csetm x13, cc
//        subs x26, x23, x12
//        cneg x26, x26, cc
//        mul x22, x9, x26
//        umulh x9, x9, x26
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x26, x22, x13
//        adcs x5, x5, x26
//        eor x9, x9, x13
//        adcs x14, x21, x9
//        adcs x21, x19, x13
//        adc x19, x8, x13
//        subs x9, x7, x25
//        cneg x8, x9, cc
//        csetm x9, cc
//        subs x13, x6, x24
//        cneg x13, x13, cc
//        mul x26, x8, x13
//        umulh x8, x8, x13
//        cinv x9, x9, cc
//        cmn x9, #0x1
//        eor x13, x26, x9
//        adcs x15, x15, x13
//        eor x8, x8, x9
//        adcs x8, x20, x8
//        adcs x5, x5, x9
//        adcs x20, x14, x9
//        adcs x21, x21, x9
//        adc x19, x19, x9
//        subs x9, x7, x10
//        cneg x9, x9, cc
//        csetm x13, cc
//        subs x26, x23, x24
//        cneg x26, x26, cc
//        mul x22, x9, x26
//        umulh x9, x9, x26
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x26, x22, x13
//        adcs x8, x8, x26
//        eor x9, x9, x13
//        adcs x5, x5, x9
//        adcs x20, x20, x13
//        adcs x21, x21, x13
//        adc x19, x19, x13
//        subs x9, x3, x25
//        cneg x9, x9, cc
//        csetm x13, cc
//        subs x26, x6, x12
//        cneg x26, x26, cc
//        mul x22, x9, x26
//        umulh x9, x9, x26
//        cinv x13, x13, cc
//        cmn x13, #0x1
//        eor x26, x22, x13
//        adcs x8, x8, x26
//        eor x9, x9, x13
//        adcs x5, x5, x9
//        adcs x20, x20, x13
//        adcs x21, x21, x13
//        adc x19, x19, x13
//        ldp x9, x13, [sp]                        // @slothy:reads=stack0
//        adds x16, x16, x9
//        adcs x11, x11, x13
//        stp x16, x11, [sp]                       // @slothy:writes=stack0
//        ldp x16, x11, [sp, #16]                  // @slothy:reads=stack16
//        adcs x16, x15, x16
//        adcs x8, x8, x11
//        stp x16, x8, [sp, #16]                   // @slothy:writes=stack16
//        ldp x16, x8, [sp, #32]                   // @slothy:reads=stack32
//        adcs x16, x5, x16
//        adcs x5, x20, x8
//        stp x16, x5, [sp, #32]                   // @slothy:writes=stack32
//        ldp x16, x5, [sp, #48]                   // @slothy:reads=stack48
//        adcs x16, x21, x16
//        adcs x5, x19, x5
//        stp x16, x5, [sp, #48]                   // @slothy:writes=stack48
//        ldr x16, [sp, #64]                       // @slothy:reads=stack64
//        adc x16, x16, xzr
//        str x16, [sp, #64]                       // @slothy:writes=stack64
//        ldp x16, x5, [x1]
//        subs x7, x7, x16
//        sbcs x3, x3, x5
//        ldp x16, x5, [x1, #16]
//        sbcs x25, x25, x16
//        sbcs x10, x10, x5
//        csetm x16, cc
//        ldp x5, x21, [x2]
//        subs x24, x5, x24
//        sbcs x12, x21, x12
//        ldp x5, x19, [x2, #16]
//        sbcs x6, x5, x6
//        sbcs x23, x19, x23
//        csetm x5, cc
//        eor x7, x7, x16
//        subs x7, x7, x16
//        eor x3, x3, x16
//        sbcs x3, x3, x16
//        eor x25, x25, x16
//        sbcs x25, x25, x16
//        eor x10, x10, x16
//        sbc x10, x10, x16
//        eor x24, x24, x5
//        subs x24, x24, x5
//        eor x12, x12, x5
//        sbcs x12, x12, x5
//        eor x6, x6, x5
//        sbcs x6, x6, x5
//        eor x23, x23, x5
//        sbc x23, x23, x5
//        eor x16, x5, x16
//        mul x21, x7, x24
//        mul x5, x3, x12
//        mul x19, x25, x6
//        mul x8, x10, x23
//        umulh x11, x7, x24
//        adds x5, x5, x11
//        umulh x11, x3, x12
//        adcs x19, x19, x11
//        umulh x11, x25, x6
//        adcs x8, x8, x11
//        umulh x11, x10, x23
//        adc x11, x11, xzr
//        adds x15, x5, x21
//        adcs x5, x19, x5
//        adcs x19, x8, x19
//        adcs x8, x11, x8
//        adc x11, xzr, x11
//        adds x20, x5, x21
//        adcs x9, x19, x15
//        adcs x5, x8, x5
//        adcs x19, x11, x19
//        adcs x8, xzr, x8
//        adc x11, xzr, x11
//        subs x13, x25, x10
//        cneg x13, x13, cc
//        csetm x26, cc
//        subs x22, x23, x6
//        cneg x22, x22, cc
//        mul x4, x13, x22
//        umulh x13, x13, x22
//        cinv x26, x26, cc
//        cmn x26, #0x1
//        eor x22, x4, x26
//        adcs x19, x19, x22
//        eor x13, x13, x26
//        adcs x8, x8, x13
//        adc x11, x11, x26
//        subs x13, x7, x3
//        cneg x13, x13, cc
//        csetm x26, cc
//        subs x22, x12, x24
//        cneg x22, x22, cc
//        mul x4, x13, x22
//        umulh x13, x13, x22
//        cinv x26, x26, cc
//        cmn x26, #0x1
//        eor x22, x4, x26
//        adcs x15, x15, x22
//        eor x13, x13, x26
//        adcs x20, x20, x13
//        adcs x9, x9, x26
//        adcs x5, x5, x26
//        adcs x19, x19, x26
//        adcs x8, x8, x26
//        adc x11, x11, x26
//        subs x13, x3, x10
//        cneg x13, x13, cc
//        csetm x26, cc
//        subs x22, x23, x12
//        cneg x22, x22, cc
//        mul x4, x13, x22
//        umulh x13, x13, x22
//        cinv x26, x26, cc
//        cmn x26, #0x1
//        eor x22, x4, x26
//        adcs x5, x5, x22
//        eor x13, x13, x26
//        adcs x19, x19, x13
//        adcs x8, x8, x26
//        adc x11, x11, x26
//        subs x13, x7, x25
//        cneg x13, x13, cc
//        csetm x26, cc
//        subs x22, x6, x24
//        cneg x22, x22, cc
//        mul x4, x13, x22
//        umulh x13, x13, x22
//        cinv x26, x26, cc
//        cmn x26, #0x1
//        eor x22, x4, x26
//        adcs x20, x20, x22
//        eor x13, x13, x26
//        adcs x9, x9, x13
//        adcs x5, x5, x26
//        adcs x19, x19, x26
//        adcs x8, x8, x26
//        adc x11, x11, x26
//        subs x7, x7, x10
//        cneg x7, x7, cc
//        csetm x10, cc
//        subs x24, x23, x24
//        cneg x24, x24, cc
//        mul x23, x7, x24
//        umulh x7, x7, x24
//        cinv x10, x10, cc
//        cmn x10, #0x1
//        eor x24, x23, x10
//        adcs x24, x9, x24
//        eor x7, x7, x10
//        adcs x7, x5, x7
//        adcs x23, x19, x10
//        adcs x5, x8, x10
//        adc x10, x11, x10
//        subs x3, x3, x25
//        cneg x3, x3, cc
//        csetm x25, cc
//        subs x12, x6, x12
//        cneg x12, x12, cc
//        mul x6, x3, x12
//        umulh x3, x3, x12
//        cinv x25, x25, cc
//        cmn x25, #0x1
//        eor x12, x6, x25
//        adcs x24, x24, x12
//        eor x3, x3, x25
//        adcs x7, x7, x3
//        adcs x3, x23, x25
//        adcs x12, x5, x25
//        adc x25, x10, x25
//        ldp x10, x6, [sp]                        // @slothy:reads=stack0
//        ldp x23, x5, [sp, #16]                   // @slothy:reads=stack16
//        eor x21, x21, x16
//        adds x21, x21, x10
//        eor x19, x15, x16
//        adcs x19, x19, x6
//        eor x8, x20, x16
//        adcs x8, x8, x23
//        eor x24, x24, x16
//        adcs x24, x24, x5
//        eor x7, x7, x16
//        ldp x11, x15, [sp, #32]                  // @slothy:reads=stack32
//        ldp x20, x9, [sp, #48]                   // @slothy:reads=stack48
//        ldr x13, [sp, #64]                       // @slothy:reads=stack64
//        adcs x7, x7, x11
//        eor x3, x3, x16
//        adcs x3, x3, x15
//        eor x12, x12, x16
//        adcs x12, x12, x20
//        eor x25, x25, x16
//        adcs x25, x25, x9
//        adc x26, x13, xzr
//        adds x7, x7, x10
//        adcs x3, x3, x6
//        adcs x10, x12, x23
//        adcs x25, x25, x5
//        and x12, x16, #0x1ff
//        lsl x6, x21, #9
//        orr x12, x6, x12
//        adcs x12, x11, x12
//        extr x6, x19, x21, #55
//        adcs x6, x15, x6
//        extr x23, x8, x19, #55
//        adcs x23, x20, x23
//        extr x16, x24, x8, #55
//        adcs x16, x9, x16
//        lsr x24, x24, #55
//        adc x24, x24, x13
//        ldr x5, [x2, #64]
//        ldp x21, x19, [x1]
//        and x8, x21, #0xfffffffffffff
//        mul x8, x5, x8
//        ldr x11, [x1, #64]
//        ldp x15, x20, [x2]
//        and x9, x15, #0xfffffffffffff
//        mul x9, x11, x9
//        add x8, x8, x9
//        extr x21, x19, x21, #52
//        and x21, x21, #0xfffffffffffff
//        mul x21, x5, x21
//        extr x15, x20, x15, #52
//        and x15, x15, #0xfffffffffffff
//        mul x15, x11, x15
//        add x21, x21, x15
//        lsr x15, x8, #52
//        add x21, x21, x15
//        lsl x8, x8, #12
//        extr x8, x21, x8, #12
//        adds x7, x7, x8
//        ldp x8, x15, [x1, #16]
//        ldp x9, x13, [x2, #16]
//        extr x19, x8, x19, #40
//        and x19, x19, #0xfffffffffffff
//        mul x19, x5, x19
//        extr x20, x9, x20, #40
//        and x20, x20, #0xfffffffffffff
//        mul x20, x11, x20
//        add x19, x19, x20
//        lsr x20, x21, #52
//        add x19, x19, x20
//        lsl x21, x21, #12
//        extr x21, x19, x21, #24
//        adcs x3, x3, x21
//        extr x21, x15, x8, #28
//        and x21, x21, #0xfffffffffffff
//        mul x21, x5, x21
//        extr x8, x13, x9, #28
//        and x8, x8, #0xfffffffffffff
//        mul x8, x11, x8
//        add x21, x21, x8
//        lsr x8, x19, #52
//        add x21, x21, x8
//        lsl x19, x19, #12
//        extr x19, x21, x19, #36
//        adcs x10, x10, x19
//        and x19, x3, x10
//        ldp x8, x20, [x1, #32]
//        ldp x9, x22, [x2, #32]
//        extr x15, x8, x15, #16
//        and x15, x15, #0xfffffffffffff
//        mul x4, x5, x15
//        extr x15, x9, x13, #16
//        and x15, x15, #0xfffffffffffff
//        mul x15, x11, x15
//        add x15, x4, x15
//        lsl x13, x26, #48
//        add x15, x15, x13
//        lsr x13, x21, #52
//        add x15, x15, x13
//        lsl x21, x21, #12
//        extr x21, x15, x21, #48
//        adcs x25, x25, x21
//        and x21, x19, x25
//        lsr x19, x8, #4
//        and x19, x19, #0xfffffffffffff
//        mul x19, x5, x19
//        lsr x26, x9, #4
//        and x13, x26, #0xfffffffffffff
//        mul x26, x11, x13
//        add x19, x19, x26
//        lsr x13, x15, #52
//        add x19, x19, x13
//        lsl x15, x15, #12
//        extr x15, x19, x15, #60
//        extr x8, x20, x8, #56
//        and x8, x8, #0xfffffffffffff
//        mul x8, x5, x8
//        extr x9, x22, x9, #56
//        and x9, x9, #0xfffffffffffff
//        mul x9, x11, x9
//        add x8, x8, x9
//        lsr x19, x19, #52
//        add x19, x8, x19
//        lsl x8, x15, #8
//        extr x8, x19, x8, #8
//        adcs x12, x12, x8
//        and x21, x21, x12
//        ldp x1, x8, [x1, #48]
//        ldp x2, x15, [x2, #48]
//        extr x20, x1, x20, #44
//        and x20, x20, #0xfffffffffffff
//        mul x20, x5, x20
//        extr x9, x2, x22, #44
//        and x9, x9, #0xfffffffffffff
//        mul x9, x11, x9
//        add x20, x20, x9
//        lsr x9, x19, #52
//        add x22, x20, x9
//        lsl x19, x19, #12
//        extr x19, x22, x19, #20
//        adcs x6, x6, x19
//        and x21, x21, x6
//        extr x1, x8, x1, #32
//        and x1, x1, #0xfffffffffffff
//        mul x1, x5, x1
//        extr x2, x15, x2, #32
//        and x2, x2, #0xfffffffffffff
//        mul x2, x11, x2
//        add x2, x1, x2
//        lsr x1, x22, #52
//        add x2, x2, x1
//        lsl x1, x22, #12
//        extr x1, x2, x1, #32
//        adcs x23, x23, x1
//        and x21, x21, x23
//        lsr x1, x8, #20
//        mul x1, x5, x1
//        lsr x19, x15, #20
//        mul x19, x11, x19
//        add x1, x1, x19
//        lsr x19, x2, #52
//        add x19, x1, x19
//        lsl x2, x2, #12
//        extr x2, x19, x2, #44
//        adcs x16, x16, x2
//        and x2, x21, x16
//        mul x5, x5, x11
//        lsr x1, x19, #44
//        add x5, x5, x1
//        adc x24, x24, x5
//        lsr x5, x24, #9
//        orr x24, x24, #0xfffffffffffffe00
//        cmp xzr, xzr
//        adcs xzr, x7, x5
//        adcs xzr, x2, xzr
//        adcs xzr, x24, xzr
//        adcs x7, x7, x5
//        adcs x2, x3, xzr
//        adcs x10, x10, xzr
//        adcs x25, x25, xzr
//        adcs x12, x12, xzr
//        adcs x6, x6, xzr
//        adcs x23, x23, xzr
//        adcs x16, x16, xzr
//        adc x3, x24, xzr
//        stp x2, x10, [x0]                        // @slothy:writes=buffer0
//        stp x25, x12, [x0, #16]                  // @slothy:writes=buffer16
//        stp x6, x23, [x0, #32]                   // @slothy:writes=buffer32
//        lsl x25, x7, #9
//        and x3, x3, #0x1ff
//        orr x3, x3, x25
//        stp x16, x3, [x0, #48]                   // @slothy:writes=buffer48
//        lsr x14, x7, #55
//        str x14, [x0, #64]                       // @slothy:writes=buffer64
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


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p521)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_montmul_p521):
        CFI_START

// Save registers and make space for the temporary buffer

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(80)

        ldr q24, [x2]
        ldr q21, [x1]
        ldr q1, [x2, #48]
        ldp x23, x20, [x1, #16]
        movi v18.2D, #0x00000000ffffffff
        ldp x19, x17, [x2, #16]
        uzp2 v3.4S, v24.4S, v24.4S
        xtn v6.2S, v21.2D
        ldp x11, x22, [x1]
        rev64 v5.4S, v24.4S
        xtn v24.2S, v24.2D
        subs x16, x23, x20
        umull v29.2D, v6.2S, v3.2S
        rev64 v31.4S, v1.4S
        cneg x26, x16, cc
        umull v27.2D, v6.2S, v24.2S
        ldr q19, [x1, #48]
        csetm x12, cc
        mul x15, x20, x17
        mul v26.4S, v5.4S, v21.4S
        uzp2 v28.4S, v21.4S, v21.4S
        subs x6, x17, x19
        xtn v7.2S, v1.2D
        cinv x10, x12, cc
        cneg x3, x6, cc
        uzp2 v21.4S, v1.4S, v1.4S
        umull v1.2D, v28.2S, v3.2S
        mul x12, x26, x3
        usra v29.2D, v27.2D, #32
        mul v25.4S, v31.4S, v19.4S
        usra v1.2D, v29.2D, #32
        uaddlp v31.2D, v26.4S
        umulh x14, x26, x3
        eor x12, x12, x10
        and v26.16B, v29.16B, v18.16B
        uaddlp v2.2D, v25.4S
        subs x16, x11, x22
        shl v0.2D, v31.2D, #32
        xtn v31.2S, v19.2D
        cneg x6, x16, cc
        shl v16.2D, v2.2D, #32
        umlal v26.2D, v28.2S, v24.2S
        umlal v0.2D, v6.2S, v24.2S
        uzp2 v30.4S, v19.4S, v19.4S
        umulh x26, x20, x17
        umull v22.2D, v31.2S, v21.2S
        umull v29.2D, v30.2S, v21.2S
        usra v1.2D, v26.2D, #32
        mul x13, x23, x19
        eor x9, x14, x10
        ldr q5, [x2, #32]
        umull v26.2D, v31.2S, v7.2S
        ldp x21, x4, [x2]
        csetm x8, cc
        mov x16, v0.d[1]
        ldr q6, [x1, #32]
        umlal v16.2D, v31.2S, v7.2S
        mov x3, v0.d[0]
        umulh x14, x23, x19
        mov x25, v1.d[1]
        mov x5, v1.d[0]
        usra v22.2D, v26.2D, #32
        rev64 v3.4S, v5.4S
        adds x16, x16, x5
        uzp1 v24.4S, v5.4S, v6.4S
        movi v26.2D, #0x00000000ffffffff
        adcs x7, x13, x25
        uzp1 v0.4S, v6.4S, v6.4S
        mul v5.4S, v3.4S, v6.4S
        adcs x25, x15, x14
        adc x13, x26, xzr
        adds x26, x16, x3
        and v6.16B, v22.16B, v26.16B
        usra v29.2D, v22.2D, #32
        adcs x16, x7, x16
        adcs x14, x25, x7
        umlal v6.2D, v30.2S, v7.2S
        adcs x7, x13, x25
        uaddlp v7.2D, v5.4S
        adc x13, xzr, x13
        adds x25, x16, x3
        adcs x24, x14, x26
        shl v1.2D, v7.2D, #32
        adcs x5, x7, x16
        usra v29.2D, v6.2D, #32
        adcs x16, x13, x14
        umlal v1.2D, v0.2S, v24.2S
        adcs x14, xzr, x7
        adc x13, xzr, x13
        subs x7, x4, x21
        cneg x7, x7, cc
        mul x15, x6, x7
        umulh x7, x6, x7
        cinv x6, x8, cc
        cmn x10, #0x1
        adcs x16, x16, x12
        eor x8, x15, x6
        adcs x14, x14, x9
        adc x9, x13, x10
        subs x13, x22, x20
        cneg x13, x13, cc
        csetm x10, cc
        subs x12, x17, x4
        cinv x15, x10, cc
        cneg x10, x12, cc
        cmn x6, #0x1
        umulh x12, x13, x10
        eor x7, x7, x6
        adcs x26, x26, x8
        adcs x7, x25, x7
        adcs x8, x24, x6
        adcs x24, x5, x6
        adcs x25, x16, x6
        mul x5, x13, x10
        adcs x13, x14, x6
        adc x14, x9, x6
        subs x10, x11, x23
        csetm x16, cc
        cneg x9, x10, cc
        subs x6, x19, x21
        cinv x10, x16, cc
        cneg x16, x6, cc
        eor x5, x5, x15
        subs x20, x11, x20
        mul x6, x9, x16
        csetm x11, cc
        cneg x20, x20, cc
        subs x17, x17, x21
        cneg x17, x17, cc
        cinv x11, x11, cc
        umulh x9, x9, x16
        eor x16, x12, x15
        subs x21, x22, x23
        cneg x22, x21, cc
        eor x12, x6, x10
        csetm x6, cc
        cmn x15, #0x1
        eor x9, x9, x10
        adcs x5, x24, x5
        umulh x23, x20, x17
        lsl x24, x3, #9
        adcs x25, x25, x16
        adcs x21, x13, x15
        adc x16, x14, x15
        subs x13, x19, x4
        cneg x14, x13, cc
        cinv x15, x6, cc
        cmn x10, #0x1
        mul x13, x20, x17
        extr x17, x26, x3, #55
        adcs x12, x7, x12
        adcs x8, x8, x9
        eor x19, x23, x11
        adcs x6, x5, x10
        eor x13, x13, x11
        mov x5, v29.d[0]
        adcs x25, x25, x10
        extr x26, x12, x26, #55
        mul x4, x22, x14
        adcs x7, x21, x10
        stp x24, x17, [sp, #32]
        ldp x20, x21, [x1, #48]
        adc x24, x16, x10
        cmn x11, #0x1
        mov x16, v16.d[0]
        umulh x17, x22, x14
        adcs x13, x8, x13
        eor x9, x4, x15
        adcs x10, x6, x19
        ldp x22, x23, [x1, #32]
        adcs x3, x25, x11
        ldp x4, x19, [x2, #32]
        eor x17, x17, x15
        adcs x7, x7, x11
        adc x14, x24, x11
        subs x6, x20, x21
        csetm x11, cc
        cneg x8, x6, cc
        cmn x15, #0x1
        umulh x25, x22, x4
        adcs x24, x13, x9
        adcs x10, x10, x17
        extr x13, x24, x12, #55
        adcs x9, x3, x15
        ldp x17, x3, [x2, #48]
        umulh x6, x23, x19
        adcs x7, x7, x15
        adc x14, x14, x15
        subs x12, x22, x23
        stp x10, x9, [sp]
        mov x9, v1.d[1]
        csetm x10, cc
        stp x7, x14, [sp, #16]
        cneg x12, x12, cc
        subs x14, x3, x17
        mov x7, v16.d[1]
        cneg x15, x14, cc
        mov x14, v29.d[1]
        cinv x11, x11, cc
        adds x9, x9, x25
        mul x25, x8, x15
        stp x26, x13, [sp, #48]
        lsr x24, x24, #55
        adcs x26, x16, x6
        mov x13, v1.d[0]
        str x24, [sp, #64]
        adcs x7, x7, x5
        adc x5, x14, xzr
        umulh x6, x8, x15
        eor x15, x25, x11
        subs x25, x19, x4
        cinv x16, x10, cc
        cneg x10, x25, cc
        eor x6, x6, x11
        adds x8, x9, x13
        adcs x14, x26, x9
        mul x9, x12, x10
        adcs x24, x7, x26
        adcs x7, x5, x7
        umulh x25, x12, x10
        adc x12, xzr, x5
        adds x26, x14, x13
        eor x10, x9, x16
        adcs x9, x24, x8
        adcs x5, x7, x14
        adcs x14, x12, x24
        adcs x7, xzr, x7
        adc x12, xzr, x12
        eor x24, x25, x16
        cmn x11, #0x1
        adcs x25, x14, x15
        adcs x14, x7, x6
        adc x11, x12, x11
        subs x12, x23, x21
        csetm x15, cc
        cneg x7, x12, cc
        subs x12, x3, x19
        cneg x12, x12, cc
        cinv x15, x15, cc
        cmn x16, #0x1
        adcs x6, x8, x10
        mul x10, x7, x12
        adcs x26, x26, x24
        adcs x9, x9, x16
        umulh x24, x7, x12
        eor x8, x10, x15
        adcs x5, x5, x16
        adcs x25, x25, x16
        adcs x7, x14, x16
        adc x16, x11, x16
        subs x11, x22, x20
        cneg x11, x11, cc
        csetm x14, cc
        subs x10, x17, x4
        cinv x14, x14, cc
        cneg x10, x10, cc
        cmn x15, #0x1
        eor x12, x24, x15
        adcs x5, x5, x8
        mul x24, x11, x10
        adcs x8, x25, x12
        adcs x25, x7, x15
        adc x16, x16, x15
        subs x12, x22, x21
        umulh x10, x11, x10
        cneg x15, x12, cc
        csetm x11, cc
        subs x12, x3, x4
        cneg x12, x12, cc
        cinv x7, x11, cc
        mul x11, x15, x12
        eor x24, x24, x14
        cmn x14, #0x1
        eor x10, x10, x14
        adcs x24, x26, x24
        eor x26, x11, x7
        adcs x10, x9, x10
        ldp x11, x9, [x1, #16]
        umulh x15, x15, x12
        adcs x5, x5, x14
        adcs x8, x8, x14
        adcs x25, x25, x14
        adc x12, x16, x14
        cmn x7, #0x1
        adcs x16, x10, x26
        eor x14, x15, x7
        adcs x26, x5, x14
        ldp x5, x10, [x1]
        adcs x14, x8, x7
        adcs x15, x25, x7
        adc x7, x12, x7
        subs x25, x23, x20
        cneg x25, x25, cc
        csetm x8, cc
        subs x22, x22, x5
        sbcs x10, x23, x10
        ldp x23, x12, [x2]
        sbcs x20, x20, x11
        sbcs x21, x21, x9
        csetm x9, cc
        subs x11, x17, x19
        cneg x5, x11, cc
        cinv x11, x8, cc
        subs x23, x23, x4
        sbcs x19, x12, x19
        eor x20, x20, x9
        ldp x12, x4, [x2, #16]
        eor x21, x21, x9
        umulh x8, x25, x5
        eor x22, x22, x9
        eor x10, x10, x9
        sbcs x17, x12, x17
        sbcs x3, x4, x3
        mul x25, x25, x5
        csetm x12, cc
        subs x22, x22, x9
        eor x4, x23, x12
        sbcs x23, x10, x9
        eor x10, x3, x12
        sbcs x20, x20, x9
        eor x5, x8, x11
        eor x3, x19, x12
        sbc x21, x21, x9
        subs x4, x4, x12
        eor x25, x25, x11
        sbcs x19, x3, x12
        eor x3, x17, x12
        sbcs x17, x3, x12
        umulh x8, x23, x19
        sbc x3, x10, x12
        cmn x11, #0x1
        adcs x25, x16, x25
        adcs x26, x26, x5
        ldp x10, x5, [sp]
        adcs x16, x14, x11
        mul x14, x22, x4
        adcs x15, x15, x11
        adc x7, x7, x11
        adds x11, x13, x10
        umulh x10, x21, x3
        adcs x13, x6, x5
        ldp x6, x5, [sp, #16]
        stp x11, x13, [sp]
        eor x13, x12, x9
        mul x9, x23, x19
        adcs x6, x24, x6
        ldp x11, x24, [sp, #32]
        mul x12, x20, x17
        adcs x25, x25, x5
        stp x6, x25, [sp, #16]
        ldp x6, x25, [sp, #48]
        umulh x5, x20, x17
        adcs x11, x26, x11
        ldr x26, [sp, #64]
        adcs x16, x16, x24
        stp x11, x16, [sp, #32]
        adcs x11, x15, x6
        umulh x24, x22, x4
        adcs x25, x7, x25
        adc x7, x26, xzr
        stp x11, x25, [sp, #48]
        subs x26, x20, x21
        csetm x15, cc
        cneg x25, x26, cc
        str x7, [sp, #64]
        mul x11, x21, x3
        subs x6, x22, x23
        cneg x6, x6, cc
        csetm x16, cc
        subs x26, x3, x17
        cneg x26, x26, cc
        cinv x7, x15, cc
        adds x24, x9, x24
        adcs x8, x12, x8
        umulh x12, x25, x26
        adcs x5, x11, x5
        adc x11, x10, xzr
        subs x15, x19, x4
        cinv x9, x16, cc
        mul x26, x25, x26
        eor x25, x12, x7
        cneg x12, x15, cc
        adds x16, x24, x14
        eor x15, x26, x7
        umulh x26, x6, x12
        adcs x10, x8, x24
        adcs x8, x5, x8
        adcs x24, x11, x5
        adc x5, xzr, x11
        adds x11, x10, x14
        mul x12, x6, x12
        adcs x6, x8, x16
        eor x14, x14, x13
        adcs x10, x24, x10
        adcs x8, x5, x8
        adcs x24, xzr, x24
        adc x5, xzr, x5
        cmn x7, #0x1
        adcs x15, x8, x15
        adcs x24, x24, x25
        eor x25, x26, x9
        adc x8, x5, x7
        eor x5, x12, x9
        subs x26, x23, x21
        cneg x12, x26, cc
        csetm x26, cc
        subs x7, x3, x19
        cneg x7, x7, cc
        cinv x26, x26, cc
        cmn x9, #0x1
        adcs x5, x16, x5
        mul x16, x12, x7
        adcs x25, x11, x25
        umulh x7, x12, x7
        adcs x12, x6, x9
        eor x11, x16, x26
        adcs x6, x10, x9
        adcs x10, x15, x9
        adcs x24, x24, x9
        adc x8, x8, x9
        subs x15, x22, x20
        cneg x15, x15, cc
        csetm x9, cc
        subs x16, x17, x4
        cneg x16, x16, cc
        cinv x9, x9, cc
        subs x21, x22, x21
        mul x22, x15, x16
        eor x7, x7, x26
        cneg x21, x21, cc
        umulh x16, x15, x16
        csetm x15, cc
        subs x4, x3, x4
        cneg x3, x4, cc
        eor x4, x22, x9
        cinv x15, x15, cc
        cmn x26, #0x1
        eor x22, x5, x13
        adcs x5, x6, x11
        adcs x6, x10, x7
        adcs x10, x24, x26
        eor x11, x16, x9
        adc x8, x8, x26
        subs x16, x23, x20
        cneg x7, x16, cc
        csetm x23, cc
        cmn x9, #0x1
        adcs x16, x25, x4
        mul x4, x21, x3
        adcs x24, x12, x11
        eor x11, x16, x13
        adcs x26, x5, x9
        adcs x16, x6, x9
        umulh x20, x21, x3
        adcs x6, x10, x9
        ldp x3, x10, [x1]
        adc x12, x8, x9
        subs x21, x17, x19
        cneg x8, x21, cc
        eor x25, x20, x15
        eor x20, x4, x15
        mul x19, x7, x8
        cinv x17, x23, cc
        cmn x15, #0x1
        adcs x4, x24, x20
        extr x21, x10, x3, #52
        umulh x9, x7, x8
        and x24, x21, #0xfffffffffffff
        adcs x26, x26, x25
        eor x7, x19, x17
        adcs x5, x16, x15
        and x23, x3, #0xfffffffffffff
        eor x9, x9, x17
        adcs x21, x6, x15
        adc x6, x12, x15
        cmn x17, #0x1
        adcs x25, x4, x7
        and x4, x13, #0x1ff
        ldp x16, x8, [sp]
        adcs x20, x26, x9
        adcs x12, x5, x17
        ldp x3, x5, [sp, #16]
        eor x15, x12, x13
        adcs x12, x21, x17
        adc x9, x6, x17
        adds x21, x14, x16
        lsl x7, x21, #9
        eor x26, x12, x13
        ldp x19, x17, [sp, #32]
        orr x4, x7, x4
        eor x14, x25, x13
        adcs x7, x22, x8
        adcs x12, x11, x3
        eor x11, x20, x13
        ldp x6, x25, [sp, #48]
        eor x20, x9, x13
        adcs x22, x14, x5
        ldr x14, [x2, #64]
        adcs x9, x11, x19
        ldr x11, [sp, #64]
        adcs x13, x15, x17
        adcs x26, x26, x6
        adcs x20, x20, x25
        adc x15, x11, xzr
        adds x16, x9, x16
        mul x9, x14, x23
        adcs x23, x13, x8
        extr x13, x7, x21, #55
        adcs x21, x26, x3
        ldp x3, x26, [x1, #16]
        extr x8, x22, x12, #55
        adcs x20, x20, x5
        adcs x19, x19, x4
        mul x4, x14, x24
        ldp x5, x24, [x2]
        adcs x17, x17, x13
        extr x13, x26, x3, #28
        extr x10, x3, x10, #40
        extr x7, x12, x7, #55
        and x12, x13, #0xfffffffffffff
        adcs x3, x6, x7
        ldr x6, [x1, #64]
        extr x7, x24, x5, #52
        and x5, x5, #0xfffffffffffff
        mul x12, x14, x12
        adcs x13, x25, x8
        and x7, x7, #0xfffffffffffff
        ldp x8, x25, [x2, #16]
        mul x5, x6, x5
        extr x24, x8, x24, #40
        and x24, x24, #0xfffffffffffff
        add x9, x9, x5
        lsr x5, x22, #55
        mul x7, x6, x7
        extr x22, x25, x8, #28
        and x10, x10, #0xfffffffffffff
        mul x10, x14, x10
        lsr x8, x9, #52
        lsl x9, x9, #12
        add x7, x4, x7
        adc x4, x5, x11
        ldp x11, x5, [x2, #32]
        add x8, x7, x8
        and x7, x22, #0xfffffffffffff
        extr x22, x8, x9, #12
        lsl x9, x15, #48
        mul x15, x6, x24
        add x10, x10, x15
        lsr x15, x8, #52
        extr x25, x11, x25, #16
        and x25, x25, #0xfffffffffffff
        mul x24, x6, x7
        add x7, x10, x15
        lsr x10, x7, #52
        lsl x8, x8, #12
        extr x8, x7, x8, #24
        adds x22, x16, x22
        ldp x16, x15, [x1, #32]
        adcs x23, x23, x8
        extr x8, x5, x11, #56
        mul x25, x6, x25
        add x24, x12, x24
        add x12, x24, x10
        lsr x10, x16, #4
        lsl x7, x7, #12
        extr x24, x12, x7, #36
        and x10, x10, #0xfffffffffffff
        extr x26, x16, x26, #16
        mul x10, x14, x10
        and x8, x8, #0xfffffffffffff
        adcs x21, x21, x24
        and x7, x26, #0xfffffffffffff
        mul x7, x14, x7
        lsr x24, x11, #4
        and x24, x24, #0xfffffffffffff
        extr x11, x15, x16, #56
        lsl x26, x12, #12
        and x16, x11, #0xfffffffffffff
        mul x11, x6, x24
        lsr x12, x12, #52
        ldp x2, x24, [x2, #48]
        add x25, x7, x25
        add x25, x25, x9
        and x9, x23, x21
        mul x8, x6, x8
        add x12, x25, x12
        add x25, x10, x11
        extr x11, x12, x26, #48
        ldp x7, x26, [x1, #48]
        extr x5, x2, x5, #44
        lsr x1, x12, #52
        mul x10, x14, x16
        lsr x16, x24, #20
        add x10, x10, x8
        extr x8, x26, x7, #32
        and x8, x8, #0xfffffffffffff
        extr x24, x24, x2, #32
        mul x2, x6, x16
        add x1, x25, x1
        lsr x25, x26, #20
        and x26, x24, #0xfffffffffffff
        and x24, x5, #0xfffffffffffff
        extr x16, x7, x15, #44
        mul x7, x6, x24
        adcs x11, x20, x11
        and x20, x16, #0xfffffffffffff
        lsl x5, x12, #12
        and x15, x9, x11
        mul x24, x14, x20
        lsr x9, x1, #52
        add x20, x10, x9
        extr x12, x1, x5, #60
        lsl x9, x20, #12
        lsl x5, x12, #8
        mul x10, x14, x8
        extr x12, x20, x5, #8
        lsr x1, x20, #52
        add x7, x24, x7
        adcs x8, x19, x12
        and x5, x15, x8
        add x7, x7, x1
        mul x20, x6, x26
        extr x24, x7, x9, #20
        lsr x19, x7, #52
        mul x25, x14, x25
        lsl x16, x7, #12
        add x20, x10, x20
        adcs x12, x17, x24
        add x19, x20, x19
        lsr x26, x19, #52
        mul x24, x14, x6
        and x5, x5, x12
        add x6, x25, x2
        lsl x17, x19, #12
        add x14, x6, x26
        extr x16, x19, x16, #32
        lsr x6, x14, #44
        extr x19, x14, x17, #44
        add x9, x24, x6
        adcs x17, x3, x16
        adcs x2, x13, x19
        and x7, x5, x17
        adc x15, x4, x9
        cmp xzr, xzr
        orr x1, x15, #0xfffffffffffffe00
        lsr x3, x15, #9
        adcs xzr, x22, x3
        and x15, x7, x2
        adcs xzr, x15, xzr
        adcs xzr, x1, xzr
        adcs x7, x22, x3
        lsl x3, x7, #9
        lsr x15, x7, #55
        str x15, [x0, #64]
        adcs x13, x23, xzr
        adcs x16, x21, xzr
        stp x13, x16, [x0]
        adcs x13, x11, xzr
        adcs x16, x8, xzr
        stp x13, x16, [x0, #16]
        adcs x19, x12, xzr
        adcs x16, x17, xzr
        adcs x13, x2, xzr
        stp x19, x16, [x0, #32]
        adc x16, x1, xzr
        and x16, x16, #0x1ff
        orr x16, x16, x3
        stp x13, x16, [x0, #48]

// Restore regs and return

        CFI_INC_SP(80)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
