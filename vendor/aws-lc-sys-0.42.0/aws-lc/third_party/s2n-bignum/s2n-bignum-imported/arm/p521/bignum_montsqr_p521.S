// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^576) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_montsqr_p521(uint64_t z[static 9],
//                                    const uint64_t x[static 9]);
//
// Does z := (x^2 / 2^576) mod p_521, assuming x < p_521. This means the
// Montgomery base is the "native size" 2^{9*64} = 2^576; since p_521 is
// a Mersenne prime the basic modular squaring bignum_sqr_p521 can be
// considered a Montgomery operation to base 2^521.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

// bignum_montsqr_p521 is functionally equivalent to
// unopt/bignum_montsqr_p521.
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
//        ldp x16, x8, [x1]
//        ldr q18, [x1]
//        ldr q5, [x1]
//        ldr q20, [x1]
//        ldp x17, x13, [x1, #16]
//        ldr q17, [x1, #16]
//        ldr q1, [x1, #16]
//        ldr q28, [x1, #16]
//        ldp x9, x15, [x1, #32]
//        ldr q27, [x1]
//        ldr q29, [x1, #32]
//        ldp x23, x2, [x1, #48]
//        ldr q6, [x1, #48]
//        ldr q4, [x1, #48]
//        mul x24, x9, x23
//        mul x11, x15, x2
//        umulh x20, x9, x23
//        subs x4, x9, x15
//        cneg x22, x4, cc
//        csetm x12, cc
//        subs x4, x2, x23
//        cneg x4, x4, cc
//        mul x19, x22, x4
//        umulh x4, x22, x4
//        cinv x7, x12, cc
//        eor x14, x19, x7
//        eor x22, x4, x7
//        adds x12, x24, x20
//        adc x19, x20, xzr
//        umulh x4, x15, x2
//        adds x12, x12, x11
//        adcs x19, x19, x4
//        adc x4, x4, xzr
//        adds x19, x19, x11
//        adc x4, x4, xzr
//        cmn x7, #0x1
//        adcs x12, x12, x14
//        adcs x19, x19, x22
//        adc x4, x4, x7
//        adds x11, x24, x24
//        adcs x20, x12, x12
//        adcs x10, x19, x19
//        adcs x3, x4, x4
//        adc x5, xzr, xzr
//        ldr q30, [x1, #32]
//        umull v0.2D, v30.2S, v30.2S
//        umull2 v2.2D, v30.4S, v30.4S
//        xtn v24.2S, v30.2D
//        uzp2 v30.4S, v30.4S, v30.4S
//        umull v30.2D, v30.2S, v24.2S
//        mov x7, v0.d[0]
//        mov x14, v0.d[1]
//        mov x19, v2.d[0]
//        mov x22, v2.d[1]
//        mov x4, v30.d[0]
//        mov x12, v30.d[1]
//        adds x21, x7, x4, lsl #33
//        lsr x4, x4, #31
//        adc x14, x14, x4
//        adds x19, x19, x12, lsl #33
//        lsr x4, x12, #31
//        adc x22, x22, x4
//        mul x4, x9, x15
//        umulh x12, x9, x15
//        adds x24, x14, x4, lsl #1
//        extr x4, x12, x4, #63
//        adcs x19, x19, x4
//        lsr x4, x12, #63
//        adc x4, x22, x4
//        adds x11, x11, x19
//        adcs x20, x20, x4
//        adcs x10, x10, xzr
//        adcs x3, x3, xzr
//        adc x6, x5, xzr
//        movi v3.2D, #0x00000000ffffffff
//        uzp2 v16.4S, v4.4S, v4.4S
//        xtn v25.2S, v6.2D
//        xtn v23.2S, v4.2D
//        rev64 v30.4S, v4.4S
//        umull v24.2D, v25.2S, v23.2S
//        umull v0.2D, v25.2S, v16.2S
//        uzp2 v2.4S, v6.4S, v6.4S
//        mul v30.4S, v30.4S, v6.4S
//        usra v0.2D, v24.2D, #32
//        umull v19.2D, v2.2S, v16.2S
//        uaddlp v30.2D, v30.4S
//        and v24.16B, v0.16B, v3.16B
//        umlal v24.2D, v2.2S, v23.2S
//        shl v30.2D, v30.2D, #32
//        usra v19.2D, v0.2D, #32
//        umlal v30.2D, v25.2S, v23.2S
//        usra v19.2D, v24.2D, #32
//        mov x5, v30.d[0]
//        mov x7, v30.d[1]
//        mul x14, x23, x2
//        mov x19, v19.d[0]
//        mov x4, v19.d[1]
//        umulh x22, x23, x2
//        adds x12, x19, x14
//        adcs x19, x7, x22
//        adc x4, x4, xzr
//        adds x12, x12, x14
//        adcs x19, x19, x22
//        adc x4, x4, xzr
//        adds x7, x5, x10
//        adcs x3, x12, x3
//        adcs x14, x19, x6
//        adc x10, x4, xzr
//        ldr x4, [x1, #64]
//        add x6, x4, x4
//        mul x5, x4, x4
//        and x4, x16, #0xfffffffffffff
//        mul x22, x6, x4
//        extr x4, x8, x16, #52
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x22, #52
//        add x12, x19, x4
//        lsl x4, x22, #12
//        extr x4, x12, x4, #12
//        adds x21, x21, x4
//        extr x4, x17, x8, #40
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x12, #52
//        add x22, x19, x4
//        lsl x4, x12, #12
//        extr x4, x22, x4, #24
//        adcs x24, x24, x4
//        extr x4, x13, x17, #28
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x22, #52
//        add x12, x19, x4
//        lsl x4, x22, #12
//        extr x4, x12, x4, #36
//        adcs x11, x11, x4
//        extr x4, x9, x13, #16
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x12, #52
//        add x22, x19, x4
//        lsl x4, x12, #12
//        extr x4, x22, x4, #48
//        adcs x20, x20, x4
//        lsr x4, x9, #4
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x22, #52
//        add x12, x19, x4
//        lsl x4, x22, #12
//        extr x22, x12, x4, #60
//        extr x4, x15, x9, #56
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x12, #52
//        add x12, x19, x4
//        lsl x4, x22, #8
//        extr x4, x12, x4, #8
//        adcs x7, x7, x4
//        extr x4, x23, x15, #44
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x12, #52
//        add x22, x19, x4
//        lsl x4, x12, #12
//        extr x4, x22, x4, #20
//        adcs x1, x3, x4
//        extr x4, x2, x23, #32
//        and x4, x4, #0xfffffffffffff
//        mul x19, x6, x4
//        lsr x4, x22, #52
//        add x12, x19, x4
//        lsl x4, x22, #12
//        extr x4, x12, x4, #32
//        adcs x14, x14, x4
//        lsr x4, x2, #20
//        mul x19, x6, x4
//        lsr x4, x12, #52
//        add x19, x19, x4
//        lsl x4, x12, #12
//        extr x4, x19, x4, #44
//        adcs x22, x10, x4
//        lsr x4, x19, #44
//        adc x12, x5, x4
//        extr x19, x24, x21, #9
//        extr x4, x11, x24, #9
//        stp x19, x4, [x0]                      // @slothy:writes=buffer0
//        extr x19, x20, x11, #9
//        extr x4, x7, x20, #9
//        stp x19, x4, [x0, #16]                 // @slothy:writes=buffer16
//        extr x19, x1, x7, #9
//        extr x4, x14, x1, #9
//        stp x19, x4, [x0, #32]                 // @slothy:writes=buffer32
//        extr x19, x22, x14, #9
//        extr x4, x12, x22, #9
//        stp x19, x4, [x0, #48]                 // @slothy:writes=buffer48
//        and x19, x21, #0x1ff
//        lsr x4, x12, #9
//        add x4, x19, x4
//        str x4, [x0, #64]
//        uzp1 v2.4S, v28.4S, v18.4S
//        rev64 v30.4S, v28.4S
//        uzp1 v24.4S, v18.4S, v18.4S
//        mul v30.4S, v30.4S, v18.4S
//        uaddlp v30.2D, v30.4S
//        shl v30.2D, v30.2D, #32
//        umlal v30.2D, v24.2S, v2.2S
//        mov x11, v30.d[0]
//        mov x20, v30.d[1]
//        umulh x7, x16, x17
//        subs x4, x16, x8
//        cneg x22, x4, cc
//        csetm x12, cc
//        subs x4, x13, x17
//        cneg x4, x4, cc
//        mul x19, x22, x4
//        umulh x4, x22, x4
//        cinv x1, x12, cc
//        eor x14, x19, x1
//        eor x22, x4, x1
//        adds x12, x11, x7
//        adc x19, x7, xzr
//        umulh x4, x8, x13
//        adds x12, x12, x20
//        adcs x19, x19, x4
//        adc x4, x4, xzr
//        adds x19, x19, x20
//        adc x4, x4, xzr
//        cmn x1, #0x1
//        adcs x12, x12, x14
//        adcs x19, x19, x22
//        adc x4, x4, x1
//        adds x21, x11, x11
//        adcs x24, x12, x12
//        adcs x11, x19, x19
//        adcs x20, x4, x4
//        adc x7, xzr, xzr
//        movi v3.2D, #0x00000000ffffffff
//        uzp2 v16.4S, v20.4S, v20.4S
//        xtn v25.2S, v5.2D
//        xtn v23.2S, v20.2D
//        rev64 v30.4S, v20.4S
//        umull v24.2D, v25.2S, v23.2S
//        umull v0.2D, v25.2S, v16.2S
//        uzp2 v2.4S, v5.4S, v5.4S
//        mul v30.4S, v30.4S, v5.4S
//        usra v0.2D, v24.2D, #32
//        umull v19.2D, v2.2S, v16.2S
//        uaddlp v30.2D, v30.4S
//        and v24.16B, v0.16B, v3.16B
//        umlal v24.2D, v2.2S, v23.2S
//        shl v30.2D, v30.2D, #32
//        usra v19.2D, v0.2D, #32
//        umlal v30.2D, v25.2S, v23.2S
//        usra v19.2D, v24.2D, #32
//        mov x10, v30.d[0]
//        mov x1, v30.d[1]
//        mul x14, x16, x8
//        mov x19, v19.d[0]
//        mov x4, v19.d[1]
//        umulh x22, x16, x8
//        adds x12, x19, x14
//        adcs x19, x1, x22
//        adc x4, x4, xzr
//        adds x3, x12, x14
//        adcs x19, x19, x22
//        adc x4, x4, xzr
//        adds x5, x21, x19
//        adcs x21, x24, x4
//        adcs x24, x11, xzr
//        adcs x11, x20, xzr
//        adc x20, x7, xzr
//        movi v3.2D, #0x00000000ffffffff
//        uzp2 v16.4S, v1.4S, v1.4S
//        xtn v25.2S, v17.2D
//        xtn v23.2S, v1.2D
//        rev64 v30.4S, v1.4S
//        umull v24.2D, v25.2S, v23.2S
//        umull v0.2D, v25.2S, v16.2S
//        uzp2 v2.4S, v17.4S, v17.4S
//        mul v30.4S, v30.4S, v17.4S
//        usra v0.2D, v24.2D, #32
//        umull v19.2D, v2.2S, v16.2S
//        uaddlp v30.2D, v30.4S
//        and v24.16B, v0.16B, v3.16B
//        umlal v24.2D, v2.2S, v23.2S
//        shl v30.2D, v30.2D, #32
//        usra v19.2D, v0.2D, #32
//        umlal v30.2D, v25.2S, v23.2S
//        usra v19.2D, v24.2D, #32
//        mov x7, v30.d[0]
//        mov x1, v30.d[1]
//        mul x14, x17, x13
//        mov x19, v19.d[0]
//        mov x4, v19.d[1]
//        umulh x22, x17, x13
//        adds x12, x19, x14
//        adcs x19, x1, x22
//        adc x4, x4, xzr
//        adds x12, x12, x14
//        adcs x19, x19, x22
//        adc x4, x4, xzr
//        adds x1, x7, x24
//        adcs x14, x12, x11
//        adcs x22, x19, x20
//        adc x12, x4, xzr
//        ldp x19, x4, [x0]                      // @slothy:reads=buffer0
//        adds x19, x19, x10
//        adcs x4, x4, x3
//        stp x19, x4, [x0]                      // @slothy:writes=buffer0
//        ldp x19, x4, [x0, #16]                 // @slothy:reads=buffer16
//        adcs x19, x19, x5
//        adcs x4, x4, x21
//        stp x19, x4, [x0, #16]                 // @slothy:writes=buffer16
//        ldp x19, x4, [x0, #32]                 // @slothy:reads=buffer32
//        adcs x19, x19, x1
//        adcs x4, x4, x14
//        stp x19, x4, [x0, #32]                 // @slothy:writes=buffer32
//        ldp x19, x4, [x0, #48]                 // @slothy:reads=buffer48
//        adcs x19, x19, x22
//        adcs x4, x4, x12
//        stp x19, x4, [x0, #48]                 // @slothy:writes=buffer48
//        ldr x4, [x0, #64]
//        adc x4, x4, xzr
//        str x4, [x0, #64]
//        movi v3.2D, #0x00000000ffffffff
//        uzp2 v2.4S, v29.4S, v29.4S
//        xtn v16.2S, v27.2D
//        xtn v25.2S, v29.2D
//        rev64 v30.4S, v29.4S
//        umull v24.2D, v16.2S, v25.2S
//        umull v23.2D, v16.2S, v2.2S
//        uzp2 v0.4S, v27.4S, v27.4S
//        mul v30.4S, v30.4S, v27.4S
//        usra v23.2D, v24.2D, #32
//        umull v2.2D, v0.2S, v2.2S
//        uaddlp v30.2D, v30.4S
//        and v24.16B, v23.16B, v3.16B
//        umlal v24.2D, v0.2S, v25.2S
//        shl v30.2D, v30.2D, #32
//        usra v2.2D, v23.2D, #32
//        umlal v30.2D, v16.2S, v25.2S
//        usra v2.2D, v24.2D, #32
//        mov x6, v30.d[0]
//        mov x22, v30.d[1]
//        mul x12, x17, x23
//        mul x19, x13, x2
//        mov x4, v2.d[0]
//        adds x22, x22, x4
//        mov x4, v2.d[1]
//        adcs x12, x12, x4
//        umulh x4, x17, x23
//        adcs x19, x19, x4
//        umulh x4, x13, x2
//        adc x4, x4, xzr
//        adds x21, x22, x6
//        adcs x22, x12, x22
//        adcs x12, x19, x12
//        adcs x19, x4, x19
//        adc x4, xzr, x4
//        adds x24, x22, x6
//        adcs x11, x12, x21
//        adcs x20, x19, x22
//        adcs x1, x4, x12
//        adcs x14, xzr, x19
//        adc x7, xzr, x4
//        subs x4, x17, x13
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x2, x23
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x1, x1, x4
//        eor x4, x12, x19
//        adcs x14, x14, x4
//        adc x7, x7, x19
//        subs x4, x16, x8
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x15, x9
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x10, x21, x4
//        eor x4, x12, x19
//        adcs x24, x24, x4
//        adcs x11, x11, x19
//        adcs x20, x20, x19
//        adcs x1, x1, x19
//        adcs x14, x14, x19
//        adc x7, x7, x19
//        subs x4, x8, x13
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x2, x15
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x20, x20, x4
//        eor x4, x12, x19
//        adcs x1, x1, x4
//        adcs x14, x14, x19
//        adc x7, x7, x19
//        subs x4, x16, x17
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x23, x9
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x24, x24, x4
//        eor x4, x12, x19
//        adcs x11, x11, x4
//        adcs x20, x20, x19
//        adcs x1, x1, x19
//        adcs x14, x14, x19
//        adc x7, x7, x19
//        subs x4, x16, x13
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x2, x9
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x11, x11, x4
//        eor x4, x12, x19
//        adcs x20, x20, x4
//        adcs x1, x1, x19
//        adcs x14, x14, x19
//        adc x7, x7, x19
//        subs x4, x8, x17
//        cneg x12, x4, cc
//        csetm x22, cc
//        subs x4, x23, x15
//        cneg x19, x4, cc
//        mul x4, x12, x19
//        umulh x12, x12, x19
//        cinv x19, x22, cc
//        cmn x19, #0x1
//        eor x4, x4, x19
//        adcs x3, x11, x4
//        eor x4, x12, x19
//        adcs x5, x20, x4
//        adcs x1, x1, x19
//        adcs x14, x14, x19
//        adc x22, x7, x19
//        ldp x12, x19, [x0]                     // @slothy:reads=buffer0
//        extr x4, x1, x5, #8
//        adds x11, x4, x12
//        extr x4, x14, x1, #8
//        adcs x20, x4, x19
//        ldp x19, x12, [x0, #16]                // @slothy:reads=buffer16
//        extr x4, x22, x14, #8
//        adcs x7, x4, x19
//        and x19, x20, x7
//        lsr x4, x22, #8
//        adcs x1, x4, x12
//        and x22, x19, x1
//        ldp x19, x12, [x0, #32]                // @slothy:reads=buffer32
//        lsl x4, x6, #1
//        adcs x14, x4, x19
//        and x19, x22, x14
//        extr x4, x10, x6, #63
//        adcs x21, x4, x12
//        and x22, x19, x21
//        ldp x19, x12, [x0, #48]                // @slothy:reads=buffer48
//        extr x4, x24, x10, #63
//        adcs x2, x4, x19
//        and x19, x22, x2
//        extr x4, x3, x24, #63
//        adcs x24, x4, x12
//        and x12, x19, x24
//        ldr x19, [x0, #64]
//        extr x4, x5, x3, #63
//        and x4, x4, #0x1ff
//        adc x4, x19, x4
//        lsr x19, x4, #9
//        orr x4, x4, #0xfffffffffffffe00
//        cmp xzr, xzr
//        adcs xzr, x11, x19
//        adcs xzr, x12, xzr
//        adcs xzr, x4, xzr
//        adcs x11, x11, x19
//        adcs x20, x20, xzr
//        adcs x7, x7, xzr
//        adcs x1, x1, xzr
//        adcs x14, x14, xzr
//        adcs x22, x21, xzr
//        adcs x12, x2, xzr
//        adcs x24, x24, xzr
//        adc x4, x4, xzr
//        and x19, x4, #0x1ff
//        lsl x4, x11, #9
//        extr x11, x20, x11, #55
//        extr x20, x7, x20, #55
//        extr x7, x1, x7, #55
//        extr x1, x14, x1, #55
//        orr x4, x19, x4
//        extr x14, x22, x14, #55
//        extr x22, x12, x22, #55
//        extr x12, x24, x12, #55
//        extr x19, x4, x24, #55
//        lsr x4, x4, #55
//        stp x11, x20, [x0]                     // @slothy:writes=buffer0
//        stp x7, x1, [x0, #16]                  // @slothy:writes=buffer16
//        stp x14, x22, [x0, #32]                // @slothy:writes=buffer32
//        stp x12, x19, [x0, #48]                // @slothy:writes=buffer48
//        str x4, [x0, #64]
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

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p521)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_montsqr_p521):
        CFI_START

// Save registers

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)

// The optimized body

        ldr q31, [x1, #48]
        ldp x9, x15, [x1, #32]
        ldp x23, x2, [x1, #48]
        ldr q0, [x1, #48]
        ldr q29, [x1, #32]
        rev64 v21.4S, v31.4S
        umulh x13, x9, x23
        mul v23.4S, v21.4S, v0.4S
        xtn v21.2S, v0.2D
        uzp2 v19.4S, v31.4S, v31.4S
        xtn v2.2S, v29.2D
        xtn v30.2S, v31.2D
        uzp2 v3.4S, v29.4S, v29.4S
        umull v6.2D, v21.2S, v19.2S
        mul x10, x9, x23
        uaddlp v23.2D, v23.4S
        umull v22.2D, v21.2S, v30.2S
        adds x22, x10, x13
        mul x17, x9, x15
        movi v25.2D, #0x00000000ffffffff
        uzp2 v1.4S, v0.4S, v0.4S
        adc x8, x13, xzr
        subs x19, x9, x15
        umull v28.2D, v3.2S, v2.2S
        shl v31.2D, v23.2D, #32
        csetm x5, cc
        cneg x3, x19, cc
        umull v19.2D, v1.2S, v19.2S
        ldr q4, [x1, #16]
        subs x24, x2, x23
        mul x6, x15, x2
        usra v6.2D, v22.2D, #32
        ldr q23, [x1]
        cneg x13, x24, cc
        umulh x24, x15, x2
        umull v5.2D, v29.2S, v29.2S
        rev64 v3.4S, v4.4S
        cinv x19, x5, cc
        adds x16, x22, x6
        mov x14, v28.d[1]
        umlal v31.2D, v21.2S, v30.2S
        umull2 v17.2D, v29.4S, v29.4S
        mov x20, v28.d[0]
        mul v29.4S, v3.4S, v23.4S
        and v22.16B, v6.16B, v25.16B
        mul x5, x3, x13
        mov x4, v5.d[1]
        mov x7, v5.d[0]
        adcs x11, x8, x24
        ldr q5, [x1]
        ldr q0, [x1]
        adc x22, x24, xzr
        adds x8, x11, x6
        usra v19.2D, v6.2D, #32
        umlal v22.2D, v1.2S, v30.2S
        adc x11, x22, xzr
        adds x21, x7, x20, lsl #33
        mov x24, v17.d[1]
        mov x22, v17.d[0]
        lsr x12, x20, #31
        uzp1 v2.4S, v4.4S, v23.4S
        uzp1 v20.4S, v23.4S, v23.4S
        usra v19.2D, v22.2D, #32
        adc x4, x4, x12
        lsr x6, x14, #31
        adds x20, x22, x14, lsl #33
        ldr q17, [x1, #16]
        uzp2 v22.4S, v0.4S, v0.4S
        eor x12, x5, x19
        umulh x7, x3, x13
        xtn v23.2S, v0.2D
        adc x5, x24, x6
        cmn x19, #0x1
        xtn v25.2S, v5.2D
        ldr q27, [x1]
        adcs x16, x16, x12
        uaddlp v1.2D, v29.4S
        umulh x3, x9, x15
        eor x13, x7, x19
        adcs x24, x8, x13
        adc x11, x11, x19
        adds x12, x10, x10
        adcs x13, x16, x16
        mul x19, x23, x2
        umull v21.2D, v25.2S, v23.2S
        adcs x7, x24, x24
        ldp x16, x8, [x1]
        umull v3.2D, v25.2S, v22.2S
        uzp2 v6.4S, v5.4S, v5.4S
        adcs x10, x11, x11
        ldr q29, [x1, #32]
        adc x14, xzr, xzr
        adds x24, x4, x17, lsl #1
        mov x4, v31.d[1]
        shl v30.2D, v1.2D, #32
        lsr x6, x3, #63
        extr x11, x3, x17, #63
        ldr q1, [x1, #16]
        mov x22, v19.d[1]
        adcs x20, x20, x11
        umulh x3, x23, x2
        movi v4.2D, #0x00000000ffffffff
        usra v3.2D, v21.2D, #32
        adc x5, x5, x6
        adds x11, x12, x20
        mov x6, v19.d[0]
        umull v19.2D, v6.2S, v22.2S
        adcs x20, x13, x5
        rev64 v22.4S, v0.4S
        ldr x5, [x1, #64]
        ldp x17, x13, [x1, #16]
        adcs x7, x7, xzr
        umlal v30.2D, v20.2S, v2.2S
        adcs x12, x10, xzr
        and x1, x16, #0xfffffffffffff
        mul v22.4S, v22.4S, v5.4S
        adc x14, x14, xzr
        adds x6, x6, x19
        xtn v5.2S, v1.2D
        adcs x10, x4, x3
        mov x4, v31.d[0]
        adc x22, x22, xzr
        adds x19, x6, x19
        add x6, x5, x5
        and v21.16B, v3.16B, v4.16B
        adcs x10, x10, x3
        extr x3, x8, x16, #52
        mul x1, x6, x1
        usra v19.2D, v3.2D, #32
        adc x22, x22, xzr
        adds x7, x4, x7
        umlal v21.2D, v6.2S, v23.2S
        and x4, x3, #0xfffffffffffff
        adcs x3, x19, x12
        uzp2 v28.4S, v1.4S, v1.4S
        extr x19, x17, x8, #40
        mul x12, x6, x4
        adcs x14, x10, x14
        rev64 v4.4S, v1.4S
        mul x5, x5, x5
        lsr x4, x9, #4
        adc x10, x22, xzr
        lsl x22, x1, #12
        lsr x1, x1, #52
        add x12, x12, x1
        and x1, x19, #0xfffffffffffff
        extr x19, x12, x22, #12
        mul x1, x6, x1
        extr x22, x13, x17, #28
        adds x21, x21, x19
        mul v31.4S, v4.4S, v17.4S
        and x19, x22, #0xfffffffffffff
        lsr x22, x12, #52
        lsl x12, x12, #12
        mul x19, x6, x19
        add x22, x1, x22
        extr x1, x22, x12, #24
        and x4, x4, #0xfffffffffffff
        adcs x12, x24, x1
        extr x1, x9, x13, #16
        mul x24, x6, x4
        and x1, x1, #0xfffffffffffff
        lsr x4, x22, #52
        add x4, x19, x4
        lsl x22, x22, #12
        mul x1, x6, x1
        extr x22, x4, x22, #36
        adcs x11, x11, x22
        extr x22, x11, x12, #9
        extr x19, x12, x21, #9
        uaddlp v3.2D, v22.4S
        lsl x12, x4, #12
        stp x19, x22, [x0]
        umulh x19, x16, x17
        uaddlp v31.2D, v31.4S
        lsr x22, x4, #52
        extr x4, x15, x9, #56
        usra v19.2D, v21.2D, #32
        add x22, x1, x22
        extr x1, x23, x15, #44
        shl v4.2D, v31.2D, #32
        extr x12, x22, x12, #48
        and x4, x4, #0xfffffffffffff
        uzp2 v7.4S, v17.4S, v17.4S
        adcs x20, x20, x12
        xtn v17.2S, v17.2D
        lsl x12, x22, #12
        lsr x22, x22, #52
        mul x4, x6, x4
        add x22, x24, x22
        and x24, x1, #0xfffffffffffff
        extr x1, x2, x23, #32
        extr x12, x22, x12, #60
        lsl x12, x12, #8
        lsr x22, x22, #52
        mul x24, x6, x24
        add x4, x4, x22
        and x22, x1, #0xfffffffffffff
        extr x12, x4, x12, #8
        lsl x1, x4, #12
        lsr x4, x4, #52
        adcs x7, x7, x12
        mul x12, x6, x22
        add x24, x24, x4
        extr x1, x24, x1, #20
        extr x22, x20, x11, #9
        extr x20, x7, x20, #9
        lsr x11, x2, #20
        mul x6, x6, x11
        lsr x4, x24, #52
        add x4, x12, x4
        lsl x12, x24, #12
        adcs x3, x3, x1
        extr x24, x4, x12, #32
        lsr x11, x4, #52
        adcs x12, x14, x24
        umull v31.2D, v17.2S, v28.2S
        add x24, x6, x11
        lsl x1, x4, #12
        extr x7, x3, x7, #9
        rev64 v6.4S, v29.4S
        umull v22.2D, v17.2S, v5.2S
        extr x11, x12, x3, #9
        extr x14, x24, x1, #44
        umlal v4.2D, v17.2S, v5.2S
        adcs x3, x10, x14
        umulh x10, x8, x13
        lsr x14, x24, #44
        adc x24, x5, x14
        subs x5, x16, x8
        stp x22, x20, [x0, #16]
        csetm x1, cc
        shl v21.2D, v3.2D, #32
        movi v17.2D, #0x00000000ffffffff
        cneg x20, x5, cc
        subs x5, x13, x17
        usra v31.2D, v22.2D, #32
        cneg x14, x5, cc
        lsr x6, x24, #9
        and x22, x21, #0x1ff
        mov x4, v30.d[0]
        add x6, x22, x6
        stp x7, x11, [x0, #32]
        umulh x22, x20, x14
        mov x5, v30.d[1]
        str x6, [x0, #64]
        extr x12, x3, x12, #9
        umull v28.2D, v7.2S, v28.2S
        mul x11, x20, x14
        mul v6.4S, v6.4S, v27.4S
        and v1.16B, v31.16B, v17.16B
        cinv x21, x1, cc
        adds x6, x4, x19
        uzp2 v22.4S, v27.4S, v27.4S
        adc x20, x19, xzr
        adds x6, x6, x5
        umlal v1.2D, v7.2S, v5.2S
        xtn v20.2S, v29.2D
        eor x22, x22, x21
        adcs x7, x20, x10
        usra v28.2D, v31.2D, #32
        eor x20, x11, x21
        usra v28.2D, v1.2D, #32
        xtn v0.2S, v27.2D
        adc x10, x10, xzr
        adds x1, x7, x5
        umlal v21.2D, v25.2S, v23.2S
        uzp2 v29.4S, v29.4S, v29.4S
        adc x19, x10, xzr
        cmn x21, #0x1
        umull v3.2D, v0.2S, v20.2S
        adcs x5, x6, x20
        extr x10, x24, x3, #9
        umull v31.2D, v0.2S, v29.2S
        adcs x1, x1, x22
        stp x12, x10, [x0, #48]
        mul x24, x16, x8
        mov x3, v28.d[1]
        usra v31.2D, v3.2D, #32
        adc x10, x19, x21
        adds x7, x4, x4
        umulh x14, x16, x8
        uaddlp v3.2D, v6.4S
        mov x4, v28.d[0]
        adcs x12, x5, x5
        mov x5, v19.d[0]
        movi v23.2D, #0x00000000ffffffff
        adcs x20, x1, x1
        mov x19, v21.d[1]
        mov x1, v19.d[1]
        adcs x22, x10, x10
        and v17.16B, v31.16B, v23.16B
        adc x6, xzr, xzr
        umlal v17.2D, v22.2S, v20.2S
        adds x10, x5, x24
        mul x11, x17, x13
        mov x5, v21.d[0]
        umull v28.2D, v22.2S, v29.2S
        adcs x19, x19, x14
        shl v5.2D, v3.2D, #32
        adc x21, x1, xzr
        adds x10, x10, x24
        adcs x1, x19, x14
        umulh x14, x17, x13
        adc x19, x21, xzr
        adds x7, x7, x1
        adcs x1, x12, x19
        adcs x24, x20, xzr
        mov x20, v4.d[1]
        usra v28.2D, v31.2D, #32
        mov x21, v4.d[0]
        adcs x19, x22, xzr
        adc x6, x6, xzr
        adds x4, x4, x11
        adcs x20, x20, x14
        adc x22, x3, xzr
        adds x12, x4, x11
        umulh x11, x13, x2
        adcs x3, x20, x14
        adc x20, x22, xzr
        adds x21, x21, x24
        ldp x22, x24, [x0]
        adcs x4, x12, x19
        ldp x19, x14, [x0, #16]
        usra v28.2D, v17.2D, #32
        adcs x3, x3, x6
        umlal v5.2D, v0.2S, v20.2S
        adc x6, x20, xzr
        umulh x20, x17, x23
        adds x12, x22, x5
        ldp x22, x5, [x0, #32]
        adcs x10, x24, x10
        adcs x19, x19, x7
        stp x12, x10, [x0]
        ldp x12, x7, [x0, #48]
        adcs x10, x14, x1
        mul x14, x13, x2
        ldr x24, [x0, #64]
        adcs x22, x22, x21
        adcs x5, x5, x4
        mov x21, v28.d[1]
        stp x22, x5, [x0, #32]
        mul x1, x17, x23
        adcs x3, x12, x3
        mov x4, v28.d[0]
        mov x12, v5.d[1]
        stp x19, x10, [x0, #16]
        adcs x19, x7, x6
        mov x6, v5.d[0]
        adc x10, x24, xzr
        subs x7, x16, x8
        cneg x5, x7, cc
        csetm x24, cc
        subs x7, x15, x9
        cneg x22, x7, cc
        cinv x7, x24, cc
        adds x12, x12, x4
        umulh x4, x5, x22
        adcs x1, x1, x21
        stp x3, x19, [x0, #48]
        str x10, [x0, #64]
        adcs x20, x14, x20
        adc x21, x11, xzr
        subs x14, x17, x13
        cneg x10, x14, cc
        csetm x3, cc
        subs x19, x2, x23
        cneg x19, x19, cc
        cinv x11, x3, cc
        adds x14, x12, x6
        mul x24, x5, x22
        adcs x22, x1, x12
        eor x3, x4, x7
        mul x4, x10, x19
        adcs x1, x20, x1
        adcs x12, x21, x20
        adc x5, xzr, x21
        umulh x19, x10, x19
        adds x20, x22, x6
        eor x10, x24, x7
        adcs x21, x1, x14
        eor x24, x4, x11
        adcs x4, x12, x22
        adcs x1, x5, x1
        adcs x12, xzr, x12
        adc x22, xzr, x5
        eor x5, x19, x11
        cmn x11, #0x1
        adcs x19, x1, x24
        adcs x5, x12, x5
        adc x24, x22, x11
        subs x1, x8, x13
        cneg x22, x1, cc
        csetm x1, cc
        subs x11, x2, x15
        cinv x1, x1, cc
        cneg x12, x11, cc
        cmn x7, #0x1
        adcs x10, x14, x10
        mul x14, x22, x12
        adcs x20, x20, x3
        eor x11, x14, x1
        adcs x3, x21, x7
        umulh x21, x22, x12
        adcs x22, x4, x7
        adcs x4, x19, x7
        adcs x12, x5, x7
        adc x7, x24, x7
        subs x14, x16, x17
        csetm x5, cc
        cneg x19, x14, cc
        subs x24, x23, x9
        cneg x14, x24, cc
        cinv x5, x5, cc
        cmn x1, #0x1
        mul x24, x19, x14
        adcs x22, x22, x11
        eor x11, x21, x1
        eor x24, x24, x5
        umulh x19, x19, x14
        adcs x4, x4, x11
        adcs x14, x12, x1
        adc x1, x7, x1
        subs x17, x8, x17
        cneg x12, x17, cc
        csetm x17, cc
        subs x16, x16, x13
        cneg x11, x16, cc
        csetm x16, cc
        subs x23, x23, x15
        cinv x7, x17, cc
        cneg x13, x23, cc
        mul x15, x12, x13
        subs x23, x2, x9
        cinv x8, x16, cc
        cneg x17, x23, cc
        eor x16, x19, x5
        mul x23, x11, x17
        cmn x5, #0x1
        adcs x20, x20, x24
        eor x15, x15, x7
        adcs x3, x3, x16
        adcs x2, x22, x5
        umulh x16, x11, x17
        adcs x19, x4, x5
        ldp x4, x22, [x0, #48]
        extr x21, x10, x6, #63
        adcs x24, x14, x5
        eor x23, x23, x8
        adc x1, x1, x5
        cmn x8, #0x1
        umulh x9, x12, x13
        eor x14, x16, x8
        adcs x3, x3, x23
        ldp x11, x5, [x0, #16]
        ldp x13, x16, [x0]
        adcs x23, x2, x14
        adcs x14, x19, x8
        extr x19, x20, x10, #63
        lsl x12, x6, #1
        adcs x17, x24, x8
        adc x1, x1, x8
        cmn x7, #0x1
        adcs x24, x3, x15
        eor x9, x9, x7
        ldp x15, x3, [x0, #32]
        adcs x9, x23, x9
        ldr x8, [x0, #64]
        extr x20, x24, x20, #63
        adcs x23, x14, x7
        extr x2, x9, x24, #63
        adcs x14, x17, x7
        and x24, x2, #0x1ff
        extr x9, x23, x9, #8
        extr x6, x14, x23, #8
        adc x23, x1, x7
        adds x10, x9, x13
        adcs x13, x6, x16
        extr x1, x23, x14, #8
        lsr x23, x23, #8
        adcs x7, x1, x11
        adcs x2, x23, x5
        and x23, x13, x7
        adcs x16, x12, x15
        and x23, x23, x2
        adcs x14, x21, x3
        and x23, x23, x16
        adcs x5, x19, x4
        and x23, x23, x14
        adcs x22, x20, x22
        and x23, x23, x5
        and x1, x23, x22
        adc x9, x8, x24
        lsr x23, x9, #9
        cmp xzr, xzr
        orr x17, x9, #0xfffffffffffffe00
        adcs xzr, x10, x23
        adcs xzr, x1, xzr
        adcs xzr, x17, xzr
        adcs x23, x10, x23
        adcs x9, x13, xzr
        lsl x4, x23, #9
        adcs x1, x7, xzr
        extr x23, x9, x23, #55
        extr x9, x1, x9, #55
        adcs x10, x2, xzr
        extr x1, x10, x1, #55
        stp x23, x9, [x0]
        adcs x19, x16, xzr
        adcs x9, x14, xzr
        extr x23, x19, x10, #55
        adcs x10, x5, xzr
        stp x1, x23, [x0, #16]
        extr x5, x9, x19, #55
        adcs x1, x22, xzr
        extr x23, x10, x9, #55
        adc x9, x17, xzr
        stp x5, x23, [x0, #32]
        extr x10, x1, x10, #55
        and x23, x9, #0x1ff
        orr x23, x23, x4
        extr x9, x23, x1, #55
        lsr x23, x23, #55
        stp x10, x9, [x0, #48]
        str x23, [x0, #64]

// Restore regs and return

        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
