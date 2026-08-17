// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^384) mod p_384
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_montmul_p384(uint64_t z[static 6],
//                                    const uint64_t x[static 6],
//                                    const uint64_t y[static 6]);
//
// Does z := (2^{-384} * x * y) mod p_384, assuming that the inputs x and y
// satisfy x * y <= 2^384 * p_384 (in particular this is true if we are in
// the "usual" case x < p_384 and y < p_384).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

// bignum_montmul_p384 is functionally equivalent to
// unopt/bignum_montmul_p384_base.
// It is written in a way that
// 1. A subset of scalar multiplications in bignum_montmul_p384 are carefully
//    chosen and vectorized
// 2. The vectorized assembly is rescheduled using the SLOTHY superoptimizer.
//    https://github.com/slothy-optimizer/slothy
//
// The output program of step 1. is as follows:
//
//        stp	x19, x20, [sp, #-16]!
//        stp	x21, x22, [sp, #-16]!
//        stp	x23, x24, [sp, #-16]!
//        ldp x3, x21, [x1]
//        ldr q30, [x1]
//        ldp x8, x24, [x1, #16]
//        ldp x5, x10, [x1, #32]
//        ldp x13, x23, [x2]
//        ldr q19, [x2]
//        ldp x6, x14, [x2, #16]
//        ldp x15, x17, [x2, #32]
//        ldr q1, [x1, #32]
//        ldr q28, [x2, #32]
//        uzp1 v5.4S, v19.4S, v30.4S
//        rev64 v19.4S, v19.4S
//        uzp1 v0.4S, v30.4S, v30.4S
//        mul v21.4S, v19.4S, v30.4S
//        uaddlp v19.2D, v21.4S
//        shl v19.2D, v19.2D, #32
//        umlal v19.2D, v0.2S, v5.2S
//        mov x12, v19.d[0]
//        mov x16, v19.d[1]
//        mul x20, x8, x6
//        umulh x4, x3, x13
//        umulh x1, x21, x23
//        umulh x2, x8, x6
//        adds x4, x4, x16
//        adcs x19, x1, x20
//        adc x20, x2, xzr
//        adds x11, x4, x12
//        adcs x16, x19, x4
//        adcs x1, x20, x19
//        adc x2, x20, xzr
//        adds x7, x16, x12
//        adcs x4, x1, x4
//        adcs x9, x2, x19
//        adc x19, x20, xzr
//        subs x2, x3, x21
//        cneg x20, x2, cc
//        csetm x16, cc
//        subs x2, x23, x13
//        cneg x2, x2, cc
//        mul x1, x20, x2
//        umulh x2, x20, x2
//        cinv x16, x16, cc
//        eor x1, x1, x16
//        eor x2, x2, x16
//        cmn x16, #0x1
//        adcs x11, x11, x1
//        adcs x7, x7, x2
//        adcs x4, x4, x16
//        adcs x9, x9, x16
//        adc x19, x19, x16
//        subs x2, x3, x8
//        cneg x20, x2, cc
//        csetm x16, cc
//        subs x2, x6, x13
//        cneg x2, x2, cc
//        mul x1, x20, x2
//        umulh x2, x20, x2
//        cinv x16, x16, cc
//        eor x1, x1, x16
//        eor x2, x2, x16
//        cmn x16, #0x1
//        adcs x7, x7, x1
//        adcs x4, x4, x2
//        adcs x9, x9, x16
//        adc x19, x19, x16
//        subs x2, x21, x8
//        cneg x20, x2, cc
//        csetm x16, cc
//        subs x2, x6, x23
//        cneg x2, x2, cc
//        mul x1, x20, x2
//        umulh x2, x20, x2
//        cinv x16, x16, cc
//        eor x1, x1, x16
//        eor x2, x2, x16
//        cmn x16, #0x1
//        adcs x4, x4, x1
//        adcs x20, x9, x2
//        adc x16, x19, x16
//        lsl x2, x12, #32
//        add x19, x2, x12
//        lsr x2, x19, #32
//        subs x1, x2, x19
//        sbc x2, x19, xzr
//        extr x1, x2, x1, #32
//        lsr x2, x2, #32
//        adds x12, x2, x19
//        adc x2, xzr, xzr
//        subs x1, x11, x1
//        sbcs x7, x7, x12
//        sbcs x4, x4, x2
//        sbcs x20, x20, xzr
//        sbcs x16, x16, xzr
//        sbc x9, x19, xzr
//        lsl x2, x1, #32
//        add x19, x2, x1
//        lsr x2, x19, #32
//        subs x1, x2, x19
//        sbc x2, x19, xzr
//        extr x1, x2, x1, #32
//        lsr x2, x2, #32
//        adds x12, x2, x19
//        adc x2, xzr, xzr
//        subs x1, x7, x1
//        sbcs x4, x4, x12
//        sbcs x20, x20, x2
//        sbcs x16, x16, xzr
//        sbcs x7, x9, xzr
//        sbc x9, x19, xzr
//        lsl x2, x1, #32
//        add x19, x2, x1
//        lsr x2, x19, #32
//        subs x1, x2, x19
//        sbc x2, x19, xzr
//        extr x12, x2, x1, #32
//        lsr x2, x2, #32
//        adds x1, x2, x19
//        adc x2, xzr, xzr
//        subs x4, x4, x12
//        sbcs x20, x20, x1
//        sbcs x16, x16, x2
//        sbcs x12, x7, xzr
//        sbcs x1, x9, xzr
//        sbc x2, x19, xzr
//        stp x4, x20, [x0]                       // @slothy:writes=buffer0
//        stp x16, x12, [x0, #16]                 // @slothy:writes=buffer16
//        stp x1, x2, [x0, #32]                   // @slothy:writes=buffer32
//        mul x22, x24, x14
//        movi v31.2D, #0x00000000ffffffff
//        uzp2 v16.4S, v28.4S, v28.4S
//        xtn v6.2S, v1.2D
//        xtn v30.2S, v28.2D
//        rev64 v28.4S, v28.4S
//        umull v5.2D, v6.2S, v30.2S
//        umull v0.2D, v6.2S, v16.2S
//        uzp2 v19.4S, v1.4S, v1.4S
//        mul v20.4S, v28.4S, v1.4S
//        usra v0.2D, v5.2D, #32
//        umull v1.2D, v19.2S, v16.2S
//        uaddlp v24.2D, v20.4S
//        and v5.16B, v0.16B, v31.16B
//        umlal v5.2D, v19.2S, v30.2S
//        shl v19.2D, v24.2D, #32
//        usra v1.2D, v0.2D, #32
//        umlal v19.2D, v6.2S, v30.2S
//        usra v1.2D, v5.2D, #32
//        mov x20, v19.d[0]
//        mov x16, v19.d[1]
//        umulh x12, x24, x14
//        mov x1, v1.d[0]
//        mov x2, v1.d[1]
//        adds x4, x12, x20
//        adcs x20, x1, x16
//        adc x16, x2, xzr
//        adds x7, x4, x22
//        adcs x12, x20, x4
//        adcs x1, x16, x20
//        adc x2, x16, xzr
//        adds x9, x12, x22
//        adcs x19, x1, x4
//        adcs x4, x2, x20
//        adc x20, x16, xzr
//        subs x2, x24, x5
//        cneg x16, x2, cc
//        csetm x12, cc
//        subs x2, x15, x14
//        cneg x2, x2, cc
//        mul x1, x16, x2
//        umulh x2, x16, x2
//        cinv x12, x12, cc
//        eor x1, x1, x12
//        eor x2, x2, x12
//        cmn x12, #0x1
//        adcs x11, x7, x1
//        adcs x9, x9, x2
//        adcs x19, x19, x12
//        adcs x4, x4, x12
//        adc x20, x20, x12
//        subs x2, x24, x10
//        cneg x16, x2, cc
//        csetm x12, cc
//        subs x2, x17, x14
//        cneg x2, x2, cc
//        mul x1, x16, x2
//        umulh x2, x16, x2
//        cinv x12, x12, cc
//        eor x1, x1, x12
//        eor x2, x2, x12
//        cmn x12, #0x1
//        adcs x7, x9, x1
//        adcs x19, x19, x2
//        adcs x4, x4, x12
//        adc x20, x20, x12
//        subs x2, x5, x10
//        cneg x16, x2, cc
//        csetm x12, cc
//        subs x2, x17, x15
//        cneg x2, x2, cc
//        mul x1, x16, x2
//        umulh x2, x16, x2
//        cinv x16, x12, cc
//        eor x1, x1, x16
//        eor x2, x2, x16
//        cmn x16, #0x1
//        adcs x19, x19, x1
//        adcs x12, x4, x2
//        adc x1, x20, x16
//        subs x2, x24, x3
//        sbcs x24, x5, x21
//        sbcs x21, x10, x8
//        ngc x5, xzr
//        cmn x5, #0x1
//        eor x2, x2, x5
//        adcs x4, x2, xzr
//        eor x2, x24, x5
//        adcs x20, x2, xzr
//        eor x2, x21, x5
//        adc x16, x2, xzr
//        subs x2, x13, x14
//        sbcs x24, x23, x15
//        sbcs x8, x6, x17
//        ngc x21, xzr
//        cmn x21, #0x1
//        eor x2, x2, x21
//        adcs x15, x2, xzr
//        eor x2, x24, x21
//        adcs x14, x2, xzr
//        eor x2, x8, x21
//        adc x6, x2, xzr
//        eor x9, x5, x21
//        ldp x21, x2, [x0]                       // @slothy:reads=buffer0
//        adds x10, x22, x21
//        adcs x5, x11, x2
//        ldp x21, x2, [x0, #16]                  // @slothy:reads=buffer16
//        adcs x24, x7, x21
//        adcs x8, x19, x2
//        ldp x21, x2, [x0, #32]                  // @slothy:reads=buffer32
//        adcs x21, x12, x21
//        adcs x2, x1, x2
//        adc x19, xzr, xzr
//        stp x10, x5, [x0]                       // @slothy:writes=buffer0
//        stp x24, x8, [x0, #16]                  // @slothy:writes=buffer16
//        stp x21, x2, [x0, #32]                  // @slothy:writes=buffer32
//        mul x12, x4, x15
//        mul x5, x20, x14
//        mul x24, x16, x6
//        umulh x8, x4, x15
//        umulh x21, x20, x14
//        umulh x2, x16, x6
//        adds x10, x8, x5
//        adcs x5, x21, x24
//        adc x24, x2, xzr
//        adds x23, x10, x12
//        adcs x8, x5, x10
//        adcs x21, x24, x5
//        adc x2, x24, xzr
//        adds x13, x8, x12
//        adcs x1, x21, x10
//        adcs x10, x2, x5
//        adc x5, x24, xzr
//        subs x2, x4, x20
//        cneg x24, x2, cc
//        csetm x8, cc
//        subs x2, x14, x15
//        cneg x2, x2, cc
//        mul x21, x24, x2
//        umulh x2, x24, x2
//        cinv x8, x8, cc
//        eor x21, x21, x8
//        eor x2, x2, x8
//        cmn x8, #0x1
//        adcs x23, x23, x21
//        adcs x13, x13, x2
//        adcs x1, x1, x8
//        adcs x10, x10, x8
//        adc x5, x5, x8
//        subs x2, x4, x16
//        cneg x24, x2, cc
//        csetm x8, cc
//        subs x2, x6, x15
//        cneg x2, x2, cc
//        mul x21, x24, x2
//        umulh x2, x24, x2
//        cinv x8, x8, cc
//        eor x21, x21, x8
//        eor x2, x2, x8
//        cmn x8, #0x1
//        adcs x4, x13, x21
//        adcs x13, x1, x2
//        adcs x1, x10, x8
//        adc x10, x5, x8
//        subs x2, x20, x16
//        cneg x24, x2, cc
//        csetm x8, cc
//        subs x2, x6, x14
//        cneg x2, x2, cc
//        mul x21, x24, x2
//        umulh x2, x24, x2
//        cinv x5, x8, cc
//        eor x21, x21, x5
//        eor x2, x2, x5
//        cmn x5, #0x1
//        adcs x24, x13, x21
//        adcs x8, x1, x2
//        adc x21, x10, x5
//        ldp x20, x16, [x0]                      // @slothy:reads=buffer0
//        ldp x17, x15, [x0, #16]                 // @slothy:reads=buffer16
//        ldp x14, x6, [x0, #32]                  // @slothy:reads=buffer32
//        cmn x9, #0x1
//        eor x2, x12, x9
//        adcs x12, x2, x20
//        eor x2, x23, x9
//        adcs x23, x2, x16
//        eor x2, x4, x9
//        adcs x13, x2, x17
//        eor x2, x24, x9
//        adcs x10, x2, x15
//        eor x2, x8, x9
//        adcs x5, x2, x14
//        eor x2, x21, x9
//        adcs x24, x2, x6
//        adcs x1, x9, x19
//        adcs x8, x9, xzr
//        adcs x21, x9, xzr
//        adc x2, x9, xzr
//        adds x10, x10, x20
//        adcs x5, x5, x16
//        adcs x24, x24, x17
//        adcs x17, x1, x15
//        adcs x15, x8, x14
//        adcs x14, x21, x6
//        adc x6, x2, x19
//        lsl x2, x12, #32
//        add x1, x2, x12
//        lsr x2, x1, #32
//        subs x21, x2, x1
//        sbc x2, x1, xzr
//        extr x21, x2, x21, #32
//        lsr x2, x2, #32
//        adds x8, x2, x1
//        adc x2, xzr, xzr
//        subs x21, x23, x21
//        sbcs x23, x13, x8
//        sbcs x10, x10, x2
//        sbcs x5, x5, xzr
//        sbcs x24, x24, xzr
//        sbc x13, x1, xzr
//        lsl x2, x21, #32
//        add x1, x2, x21
//        lsr x2, x1, #32
//        subs x21, x2, x1
//        sbc x2, x1, xzr
//        extr x21, x2, x21, #32
//        lsr x2, x2, #32
//        adds x8, x2, x1
//        adc x2, xzr, xzr
//        subs x21, x23, x21
//        sbcs x10, x10, x8
//        sbcs x5, x5, x2
//        sbcs x24, x24, xzr
//        sbcs x23, x13, xzr
//        sbc x13, x1, xzr
//        lsl x2, x21, #32
//        add x1, x2, x21
//        lsr x2, x1, #32
//        subs x21, x2, x1
//        sbc x2, x1, xzr
//        extr x8, x2, x21, #32
//        lsr x2, x2, #32
//        adds x21, x2, x1
//        adc x2, xzr, xzr
//        subs x10, x10, x8
//        sbcs x5, x5, x21
//        sbcs x24, x24, x2
//        sbcs x8, x23, xzr
//        sbcs x21, x13, xzr
//        sbc x2, x1, xzr
//        adds x23, x17, x8
//        adcs x13, x15, x21
//        adcs x1, x14, x2
//        adc x2, x6, xzr
//        add x8, x2, #0x1
//        lsl x2, x8, #32
//        subs x21, x8, x2
//        sbc x2, x2, xzr
//        adds x10, x10, x21
//        adcs x5, x5, x2
//        adcs x24, x24, x8
//        adcs x8, x23, xzr
//        adcs x21, x13, xzr
//        adcs x13, x1, xzr
//        csetm x1, cc
//        mov x2, #0xffffffff
//        and x2, x2, x1
//        adds x10, x10, x2
//        eor x2, x2, x1
//        adcs x5, x5, x2
//        mov x2, #0xfffffffffffffffe
//        and x2, x2, x1
//        adcs x24, x24, x2
//        adcs x8, x8, x1
//        adcs x21, x21, x1
//        adc x2, x13, x1
//        stp x10, x5, [x0]                       // @slothy:writes=buffer0
//        stp x24, x8, [x0, #16]                  // @slothy:writes=buffer16
//        stp x21, x2, [x0, #32]                  // @slothy:writes=buffer32
//        ldp	x23, x24, [sp], #16
//        ldp	x21, x22, [sp], #16
//        ldp	x19, x20, [sp], #16
//        ret
//
// The bash script used for step 2 is as follows:
//
//        # Store the assembly instructions except the last 'ret' and
//        # callee-register store/loads as, say, 'input.S'.
//        export OUTPUTS="[hint_buffer0,hint_buffer16,hint_buffer32]"
//        export RESERVED_REGS="[x18,x25,x26,x27,x28,x29,x30,sp,q8,q9,q10,q11,q12,q13,q14,q15,v8,v9,v10,v11,v12,v13,v14,v15]"
//        <s2n-bignum>/tools/external/slothy.sh input.S my_out_dir
//        # my_out_dir/3.opt.s is the optimized assembly. Its output may differ
//        # from this file since the sequence is non-deterministically chosen.
//        # Please add 'ret' at the end of the output assembly.


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p384)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_montmul_p384):
        CFI_START

// Save some registers

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)

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

// Restore registers and return

        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
