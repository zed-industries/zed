// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[16], y[16]; output z[32]; temporary buffer t[>=32]
//
//    extern void bignum_kmul_16_32(uint64_t z[static 32],
//                                  const uint64_t x[static 16],
//                                  const uint64_t y[static 16],
//                                  uint64_t t[static 32]);
//
// This is a Karatsuba-style function multiplying half-sized results
// internally and using temporary buffer t for intermediate results.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y, X3 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_kmul_16_32)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_kmul_16_32)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_kmul_16_32)


        .text
        .balign 4

// Subroutine-safe copies of the output, inputs and temporary buffer pointers

#define z x25
#define x x26
#define y x27
#define t x28

// More variables for sign masks, with s also necessarily subroutine-safe

#define s x29
#define m x19

S2N_BN_SYMBOL(bignum_kmul_16_32):
        CFI_START

// Save registers, including return address

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)

// Move parameters into subroutine-safe places

        mov     z, x0
        mov     x, x1
        mov     y, x2
        mov     t, x3

// Compute L = x_lo * y_lo in bottom half of buffer (size 8 x 8 -> 16)

        CFI_BL(Lbignum_kmul_16_32_local_mul_8_16)

// Compute absolute difference [t..] = |x_lo - x_hi|
// and the sign s = sgn(x_lo - x_hi) as a bitmask (all 1s for negative)

        ldp     x10, x11, [x]
        ldp     x8, x9, [x, #64]
        subs    x10, x10, x8
        sbcs    x11, x11, x9
        ldp     x12, x13, [x, #16]
        ldp     x8, x9, [x, #80]
        sbcs    x12, x12, x8
        sbcs    x13, x13, x9
        ldp     x14, x15, [x, #32]
        ldp     x8, x9, [x, #96]
        sbcs    x14, x14, x8
        sbcs    x15, x15, x9
        ldp     x16, x17, [x, #48]
        ldp     x8, x9, [x, #112]
        sbcs    x16, x16, x8
        sbcs    x17, x17, x9
        csetm   s, cc
        adds    xzr, s, s
        eor     x10, x10, s
        adcs    x10, x10, xzr
        eor     x11, x11, s
        adcs    x11, x11, xzr
        stp     x10, x11, [t]
        eor     x12, x12, s
        adcs    x12, x12, xzr
        eor     x13, x13, s
        adcs    x13, x13, xzr
        stp     x12, x13, [t, #16]
        eor     x14, x14, s
        adcs    x14, x14, xzr
        eor     x15, x15, s
        adcs    x15, x15, xzr
        stp     x14, x15, [t, #32]
        eor     x16, x16, s
        adcs    x16, x16, xzr
        eor     x17, x17, s
        adcs    x17, x17, xzr
        stp     x16, x17, [t, #48]

// Compute H = x_hi * y_hi in top half of buffer (size 8 x 8 -> 16)

        add     x0, z, #128
        add     x1, x, #64
        add     x2, y, #64
        CFI_BL(Lbignum_kmul_16_32_local_mul_8_16)

// Compute the other absolute difference [t+8..] = |y_hi - y_lo|
// Collect the combined product sign bitmask (all 1s for negative) in s

        ldp     x10, x11, [y]
        ldp     x8, x9, [y, #64]
        subs    x10, x8, x10
        sbcs    x11, x9, x11
        ldp     x12, x13, [y, #16]
        ldp     x8, x9, [y, #80]
        sbcs    x12, x8, x12
        sbcs    x13, x9, x13
        ldp     x14, x15, [y, #32]
        ldp     x8, x9, [y, #96]
        sbcs    x14, x8, x14
        sbcs    x15, x9, x15
        ldp     x16, x17, [y, #48]
        ldp     x8, x9, [y, #112]
        sbcs    x16, x8, x16
        sbcs    x17, x9, x17
        csetm   m, cc
        adds    xzr, m, m
        eor     x10, x10, m
        adcs    x10, x10, xzr
        eor     x11, x11, m
        adcs    x11, x11, xzr
        stp     x10, x11, [t, #64]
        eor     x12, x12, m
        adcs    x12, x12, xzr
        eor     x13, x13, m
        adcs    x13, x13, xzr
        stp     x12, x13, [t, #80]
        eor     x14, x14, m
        adcs    x14, x14, xzr
        eor     x15, x15, m
        adcs    x15, x15, xzr
        stp     x14, x15, [t, #96]
        eor     x16, x16, m
        adcs    x16, x16, xzr
        eor     x17, x17, m
        adcs    x17, x17, xzr
        stp     x16, x17, [t, #112]
        eor     s, s, m

// Compute H' = H + L_top in place of H (it cannot overflow)
// First add 8-sized block then propagate carry through next 8

        ldp     x10, x11, [z, #128]
        ldp     x12, x13, [z, #64]
        adds    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [z, #128]

        ldp     x10, x11, [z, #128+16]
        ldp     x12, x13, [z, #64+16]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [z, #128+16]

        ldp     x10, x11, [z, #128+32]
        ldp     x12, x13, [z, #64+32]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [z, #128+32]

        ldp     x10, x11, [z, #128+48]
        ldp     x12, x13, [z, #64+48]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [z, #128+48]

        ldp     x10, x11, [z, #128+64]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [z, #128+64]

        ldp     x10, x11, [z, #128+80]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [z, #128+80]

        ldp     x10, x11, [z, #128+96]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [z, #128+96]

        ldp     x10, x11, [z, #128+112]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [z, #128+112]

// Compute M = |x_lo - x_hi| * |y_hi - y_lo| in [t+16...], size 16

        add     x0, t, #128
        mov     x1, t
        add     x2, t, #64
        CFI_BL(Lbignum_kmul_16_32_local_mul_8_16)

// Add the interlocking H' and L_bot terms, storing in registers x15..x0
// Intercept the carry at the 8 + 16 = 24 position and store it in x.
// (Note that we no longer need the input x was pointing at.)

        ldp     x0, x1, [z]
        ldp     x16, x17, [z, #128]
        adds    x0, x0, x16
        adcs    x1, x1, x17
        ldp     x2, x3, [z, #16]
        ldp     x16, x17, [z, #144]
        adcs    x2, x2, x16
        adcs    x3, x3, x17
        ldp     x4, x5, [z, #32]
        ldp     x16, x17, [z, #160]
        adcs    x4, x4, x16
        adcs    x5, x5, x17
        ldp     x6, x7, [z, #48]
        ldp     x16, x17, [z, #176]
        adcs    x6, x6, x16
        adcs    x7, x7, x17
        ldp     x8, x9, [z, #128]
        ldp     x16, x17, [z, #192]
        adcs    x8, x8, x16
        adcs    x9, x9, x17
        ldp     x10, x11, [z, #144]
        ldp     x16, x17, [z, #208]
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        ldp     x12, x13, [z, #160]
        ldp     x16, x17, [z, #224]
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        ldp     x14, x15, [z, #176]
        ldp     x16, x17, [z, #240]
        adcs    x14, x14, x16
        adcs    x15, x15, x17

        cset    x, cs

// Add the sign-adjusted mid-term cross product M

        cmn     s, s

        ldp     x16, x17, [t, #128]
        eor     x16, x16, s
        adcs    x0, x0, x16
        eor     x17, x17, s
        adcs    x1, x1, x17
        stp     x0, x1, [z, #64]
        ldp     x16, x17, [t, #144]
        eor     x16, x16, s
        adcs    x2, x2, x16
        eor     x17, x17, s
        adcs    x3, x3, x17
        stp     x2, x3, [z, #80]
        ldp     x16, x17, [t, #160]
        eor     x16, x16, s
        adcs    x4, x4, x16
        eor     x17, x17, s
        adcs    x5, x5, x17
        stp     x4, x5, [z, #96]
        ldp     x16, x17, [t, #176]
        eor     x16, x16, s
        adcs    x6, x6, x16
        eor     x17, x17, s
        adcs    x7, x7, x17
        stp     x6, x7, [z, #112]
        ldp     x16, x17, [t, #192]
        eor     x16, x16, s
        adcs    x8, x8, x16
        eor     x17, x17, s
        adcs    x9, x9, x17
        stp     x8, x9, [z, #128]
        ldp     x16, x17, [t, #208]
        eor     x16, x16, s
        adcs    x10, x10, x16
        eor     x17, x17, s
        adcs    x11, x11, x17
        stp     x10, x11, [z, #144]
        ldp     x16, x17, [t, #224]
        eor     x16, x16, s
        adcs    x12, x12, x16
        eor     x17, x17, s
        adcs    x13, x13, x17
        stp     x12, x13, [z, #160]
        ldp     x16, x17, [t, #240]
        eor     x16, x16, s
        adcs    x14, x14, x16
        eor     x17, x17, s
        adcs    x15, x15, x17
        stp     x14, x15, [z, #176]

// Get the next digits effectively resulting so far starting at 24

        adcs    y, s, x
        adc     t, s, xzr

// Now the final 8 digits of padding; the first one is special in using y
// and also in getting the carry chain started

        ldp     x10, x11, [z, #192]
        adds    x10, x10, y
        adcs    x11, x11, t
        stp     x10, x11, [z, #192]
        ldp     x10, x11, [z, #208]
        adcs    x10, x10, t
        adcs    x11, x11, t
        stp     x10, x11, [z, #208]
        ldp     x10, x11, [z, #224]
        adcs    x10, x10, t
        adcs    x11, x11, t
        stp     x10, x11, [z, #224]
        ldp     x10, x11, [z, #240]
        adcs    x10, x10, t
        adcs    x11, x11, t
        stp     x10, x11, [z, #240]

// Restore registers and return

        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_kmul_16_32)

// ----------------------------------------------------------------------------
// Local copy of bignum_mul_8_16 without the scratch register save/restore
// ----------------------------------------------------------------------------

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_kmul_16_32_local_mul_8_16)

Lbignum_kmul_16_32_local_mul_8_16:
        CFI_START
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
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_kmul_16_32_local_mul_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
