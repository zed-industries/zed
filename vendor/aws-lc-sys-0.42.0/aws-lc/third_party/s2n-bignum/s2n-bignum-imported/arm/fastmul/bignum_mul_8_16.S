// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[8], y[8]; output z[16]
//
//    extern void bignum_mul_8_16(uint64_t z[static 16], const uint64_t x[static 8],
//                                const uint64_t y[static 8]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_8_16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_8_16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_8_16)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_mul_8_16):
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        ldp x3, x4, [x1]
        ldr q0, [x1]
        ldp x7, x8, [x2]
        ldr q1, [x2]
        ldp x5, x6, [x1, #16]
        ldr q2, [x1, #16]
        ldp x9, x10, [x2, #16]
        ldr q3, [x2, #16]
        uzp1 v4.4s, v1.4s, v0.4s
        rev64 v1.4s, v1.4s
        uzp1 v5.4s, v0.4s, v0.4s
        mul v0.4s, v1.4s, v0.4s
        uaddlp v0.2d, v0.4s
        shl v0.2d, v0.2d, #32
        umlal v0.2d, v5.2s, v4.2s
        mov x11, v0.d[0]
        mov x15, v0.d[1]
        uzp1 v0.4s, v3.4s, v2.4s
        rev64 v1.4s, v3.4s
        uzp1 v3.4s, v2.4s, v2.4s
        mul v1.4s, v1.4s, v2.4s
        uaddlp v1.2d, v1.4s
        shl v1.2d, v1.2d, #32
        umlal v1.2d, v3.2s, v0.2s
        mov x16, v1.d[0]
        mov x17, v1.d[1]
        ldr q0, [x1, #32]
        ldr q1, [x2, #32]
        ldr q2, [x1, #48]
        ldr q3, [x2, #48]
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        uzp1 v4.4s, v1.4s, v0.4s
        rev64 v1.4s, v1.4s
        uzp1 v5.4s, v0.4s, v0.4s
        mul v0.4s, v1.4s, v0.4s
        uaddlp v0.2d, v0.4s
        shl v0.2d, v0.2d, #32
        umlal v0.2d, v5.2s, v4.2s
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x3, x4, [x1, #32]
        stp x11, x12, [x0]
        ldp x7, x8, [x2, #32]
        stp x13, x14, [x0, #16]
        ldp x5, x6, [x1, #48]
        stp x15, x16, [x0, #32]
        ldp x9, x10, [x2, #48]
        stp x17, x19, [x0, #48]
        mov x11, v0.d[0]
        mov x15, v0.d[1]
        uzp1 v0.4s, v3.4s, v2.4s
        rev64 v1.4s, v3.4s
        uzp1 v3.4s, v2.4s, v2.4s
        mul v1.4s, v1.4s, v2.4s
        uaddlp v1.2d, v1.4s
        shl v1.2d, v1.2d, #32
        umlal v1.2d, v3.2s, v0.2s
        mov x16, v1.d[0]
        mov x17, v1.d[1]
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        ldp x22, x21, [x0, #32]
        adds x11, x11, x22
        adcs x12, x12, x21
        ldp x22, x21, [x0, #48]
        adcs x13, x13, x22
        adcs x14, x14, x21
        adcs x15, x15, xzr
        adcs x16, x16, xzr
        adcs x17, x17, xzr
        adc x19, x19, xzr
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x22, x21, [x1]
        subs x3, x3, x22
        sbcs x4, x4, x21
        ldp x22, x21, [x1, #16]
        sbcs x5, x5, x22
        sbcs x6, x6, x21
        csetm x24, cc
        stp x11, x12, [x0, #64]
        ldp x22, x21, [x2]
        subs x7, x22, x7
        sbcs x8, x21, x8
        ldp x22, x21, [x2, #16]
        sbcs x9, x22, x9
        sbcs x10, x21, x10
        csetm x1, cc
        stp x13, x14, [x0, #80]
        eor x3, x3, x24
        subs x3, x3, x24
        eor x4, x4, x24
        sbcs x4, x4, x24
        eor x5, x5, x24
        sbcs x5, x5, x24
        eor x6, x6, x24
        sbc x6, x6, x24
        stp x15, x16, [x0, #96]
        eor x7, x7, x1
        subs x7, x7, x1
        eor x8, x8, x1
        sbcs x8, x8, x1
        eor x9, x9, x1
        sbcs x9, x9, x1
        eor x10, x10, x1
        sbc x10, x10, x1
        stp x17, x19, [x0, #112]
        eor x1, x1, x24
        mul x11, x3, x7
        mul x15, x4, x8
        mul x16, x5, x9
        mul x17, x6, x10
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x3, x4, [x0]
        ldp x7, x8, [x0, #64]
        adds x3, x3, x7
        adcs x4, x4, x8
        ldp x5, x6, [x0, #16]
        ldp x9, x10, [x0, #80]
        adcs x5, x5, x9
        adcs x6, x6, x10
        ldp x20, x21, [x0, #96]
        adcs x7, x7, x20
        adcs x8, x8, x21
        ldp x22, x23, [x0, #112]
        adcs x9, x9, x22
        adcs x10, x10, x23
        adcs x24, x1, xzr
        adc x2, x1, xzr
        cmn x1, #0x1
        eor x11, x11, x1
        adcs x3, x11, x3
        eor x12, x12, x1
        adcs x4, x12, x4
        eor x13, x13, x1
        adcs x5, x13, x5
        eor x14, x14, x1
        adcs x6, x14, x6
        eor x15, x15, x1
        adcs x7, x15, x7
        eor x16, x16, x1
        adcs x8, x16, x8
        eor x17, x17, x1
        adcs x9, x17, x9
        eor x19, x19, x1
        adcs x10, x19, x10
        adcs x20, x20, x24
        adcs x21, x21, x2
        adcs x22, x22, x2
        adc x23, x23, x2
        stp x3, x4, [x0, #32]
        stp x5, x6, [x0, #48]
        stp x7, x8, [x0, #64]
        stp x9, x10, [x0, #80]
        stp x20, x21, [x0, #96]
        stp x22, x23, [x0, #112]
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_8_16)

