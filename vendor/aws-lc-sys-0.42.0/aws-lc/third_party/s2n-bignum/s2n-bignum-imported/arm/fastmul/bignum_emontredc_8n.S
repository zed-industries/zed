// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended Montgomery reduce in 8-digit blocks, results in input-output buffer
// Inputs z[2*k], m[k], w; outputs function return (extra result bit) and z[2*k]
//
//    extern uint64_t bignum_emontredc_8n(uint64_t k, uint64_t *z, const uint64_t *m,
//                                        uint64_t w);
//
// Functionally equivalent to bignum_emontredc (see that file for more detail).
// But in general assumes that the input k is a multiple of 8.
// bignum_emontredc_8n is a vectorized version of
// unopt/bignum_emontredc_8n_base.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = w, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

                                        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc_8n)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_emontredc_8n)
                                        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc_8n)
                                        .text
                                        .balign 4


S2N_BN_SYMBOL(bignum_emontredc_8n):
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_DEC_SP(32)
        lsr x0, x0, #2
        mov x26, x0
        subs x12, x0, #1
        bcc Lbignum_emontredc_8n_end

        stp x3, xzr, [sp]
        stp x26, xzr, [sp, #16]
        mov x28, xzr
        lsl x0, x12, #5

Lbignum_emontredc_8n_outerloop:
        ldp x3, xzr, [sp]
        ldp x17, x19, [x1]
        ldp x20, x21, [x1, #16]
        ldp x8, x9, [x2]
        ldp x10, x11, [x2, #16]
        ldr q21, [x2, #16]

        // Montgomery step 0

        mul x4, x17, x3
// NEON: Calculate x4 * (x10, x11) that does two 64x64->128-bit multiplications.
dup v0.2d, x4
uzp2    v3.4s, v21.4s, v0.4s
xtn     v4.2s, v0.2d
xtn     v5.2s, v21.2d
        mul   x12, x4, x8
        adds x17, x17, x12
        umulh x12, x4, x8
        mul   x13, x4, x9
rev64   v1.4s, v21.4s
umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, v0.4s, v0.4s
mul     v0.4s, v1.4s, v0.4s
movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   v1.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s
and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     v0.2d, v0.2d, #32
usra    v1.2d, v7.2d, #32
umlal   v0.2d, v4.2s, v5.2s
mov x14, v0.d[0]
mov x15, v0.d[1]
        adcs x19, x19, x13
        umulh x13, x4, x9
        adcs x20, x20, x14
usra    v1.2d, v2.2d, #32
mov x14, v1.d[0]
        adcs x21, x21, x15
mov x15, v1.d[1]
        adc x22, xzr, xzr
        adds x19, x19, x12
        mul x5, x19, x3 // hoisted from step 1
        adcs x20, x20, x13
        adcs x21, x21, x14
        adc x22, x22, x15

        // Montgomery step 1

// NEON: Calculate x5 * (x10, x11) that does two 64x64->128-bit multiplications.
dup v0.2d, x5
uzp2    v3.4s, v21.4s, v0.4s
xtn     v4.2s, v0.2d
xtn     v5.2s, v21.2d

        mul   x12, x5, x8
        adds  x19, x19, x12
        umulh x12, x5, x8
        mul   x13, x5, x9

rev64   v1.4s, v21.4s
umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, v0.4s, v0.4s
mul     v0.4s, v1.4s, v0.4s
movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   v1.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s
and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     v0.2d, v0.2d, #32
usra    v1.2d, v7.2d, #32
umlal   v0.2d, v4.2s, v5.2s
mov x14, v0.d[0]
mov x15, v0.d[1]
        adcs  x20, x20, x13
        umulh x13, x5, x9
        adcs x21, x21, x14
usra    v1.2d, v2.2d, #32
mov x14, v1.d[0]
        adcs x22, x22, x15
mov x15, v1.d[1]
        adc x23, xzr, xzr
        adds x20, x20, x12
        mul   x6, x20, x3 // hoisted from step 2

// NEON: For montgomery step 2,
// calculate x6 * (x10, x11) that does two 64x64->128-bit multiplications.
dup v0.2d, x6
#define in1  v21
#define in2  v0
#define out_lo v0
#define out_hi v1
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d

        adcs x21, x21, x13
        adcs x22, x22, x14
        adc x23, x23, x15

        stp x4, x5, [x1]

// hoisted from maddloop_firstitr
ldr q20, [x1]
// q21 will be loaded later.

ldr q22, [x2, #32]
ldr q23, [x2, #48]

        // Montgomery step 2

rev64   v1.4s, in2.4s
umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s

        mul     x12, x6, x8
        adds    x20, x20, x12

mul     v0.4s, v1.4s, in1.4s
movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s

        umulh   x12, x6, x8
        mul     x13, x6, x9

uaddlp  v0.2d, v0.4s
and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32

        adcs    x21, x21, x13
        umulh   x13, x6, x9

usra    out_hi.2d, v7.2d, #32
umlal   out_lo.2d, v4.2s, v5.2s
mov x14, out_lo.d[0]
mov x15, out_lo.d[1]

usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x22, x22, x14
        adcs x23, x23, x15

mov x14, v1.d[0]
mov x15, v1.d[1]

        adc   x24, xzr, xzr
        adds  x21, x21, x12
        mul   x7, x21, x3
        adcs  x22, x22, x13
        adcs  x23, x23, x14
        adc   x24, x24, x15

        stp x6, x7, [x1, #16]

// hoisted from maddloop_firstitr
ldr q21, [x1, #16]

// pre-calculate 2mul+2umulhs in maddloop_firstitr
// v25++v24 = hi and lo of (x4 * x8, x5 * x9)
#define in1  v20
#define in2  v22
#define out_lo v24
#define out_hi v25
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d

        // Montgomery step 3

        mul  x12, x7, x8
        mul  x13, x7, x9

xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s
umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s

        mul  x14, x7, x10
        mul  x15, x7, x11

uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s
movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s
and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s

        adds x21, x21, x12
        umulh        x12, x7, x8
        adcs x22, x22, x13
        umulh        x13, x7, x9

shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32
umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x23, x23, x14
        umulh        x14, x7, x10
        adcs x24, x24, x15
        umulh        x15, x7, x11

// v27++v26 = hi and lo of (x6 * x10, x7 * x11)
#define in1  v21
#define in2  v23
#define out_lo v26
#define out_hi v27
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s

// hoisted from maddloop_firstitr and maddloop_x0one
        ldp x8, x9, [x2, #32]
        ldp x10, x11, [x2, #48]

umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s

        adc   x25, xzr, xzr
        adds  x12, x22, x12
        adcs  x13, x23, x13
        adcs  x14, x24, x14
        adc   x15, x25, x15

movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s
and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32
umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        cbz x0, Lbignum_emontredc_8n_madddone
        mov x27, x0
        cmp x0, #32
        bne Lbignum_emontredc_8n_maddloop_firstitr

Lbignum_emontredc_8n_maddloop_x0one:
        add     x2, x2, #0x20
        add     x1, x1, #0x20
        mul     x17, x4, x8
        mul     x22, x5, x9
        mul     x23, x6, x10
        mul     x24, x7, x11
        umulh   x16, x4, x8
        adds    x22, x22, x16
        umulh   x16, x5, x9
        adcs    x23, x23, x16
        umulh   x16, x6, x10
        adcs    x24, x24, x16
        umulh   x16, x7, x11
        adc     x25, x16, xzr
        ldp     x20, x21, [x1]
        adds    x12, x12, x20
        adcs    x13, x13, x21
        ldp     x20, x21, [x1, #16]
        adcs    x14, x14, x20
        adcs    x15, x15, x21
        adc     x16, xzr, xzr
        adds    x19, x22, x17
        adcs    x22, x23, x22
        adcs    x23, x24, x23
        adcs    x24, x25, x24
        adc     x25, xzr, x25
        adds    x20, x22, x17
        adcs    x21, x23, x19
        adcs    x22, x24, x22
        adcs    x23, x25, x23
        adcs    x24, xzr, x24
        adc     x25, xzr, x25
        adds    x17, x17, x12
        adcs    x19, x19, x13
        adcs    x20, x20, x14
        adcs    x21, x21, x15
        adcs    x22, x22, x16
        adcs    x23, x23, xzr
        adcs    x24, x24, xzr
        adc     x25, x25, xzr
        subs    x15, x6, x7
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x11, x10
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x23, x23, x14
        eor     x13, x13, x12
        adcs    x24, x24, x13
        adc     x25, x25, x12
        subs    x15, x4, x5
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x9, x8
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x19, x19, x14
        eor     x13, x13, x12
        adcs    x20, x20, x13
        adcs    x21, x21, x12
        adcs    x22, x22, x12
        adcs    x23, x23, x12
        adcs    x24, x24, x12
        adc     x25, x25, x12
        subs    x15, x5, x7
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x11, x9
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x22, x22, x14
        eor     x13, x13, x12
        adcs    x23, x23, x13
        adcs    x24, x24, x12
        adc     x25, x25, x12
        subs    x15, x4, x6
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x10, x8
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x20, x20, x14
        eor     x13, x13, x12
        adcs    x21, x21, x13
        adcs    x22, x22, x12
        adcs    x23, x23, x12
        adcs    x24, x24, x12
        adc     x25, x25, x12
        subs    x15, x4, x7
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x11, x8
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x21, x21, x14
        eor     x13, x13, x12
        adcs    x22, x22, x13
        adcs    x23, x23, x12
        adcs    x24, x24, x12
        adc     x25, x25, x12
        subs    x15, x5, x6
        cneg    x15, x15, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x13, x10, x9
        cneg    x13, x13, cc  // cc = lo, ul, last
        mul     x14, x15, x13
        umulh   x13, x15, x13
        cinv    x12, x12, cc  // cc = lo, ul, last
        cmn     x12, #0x1
        eor     x14, x14, x12
        adcs    x21, x21, x14
        eor     x13, x13, x12
        adcs    x22, x22, x13
        adcs    x13, x23, x12
        adcs    x14, x24, x12
        adc     x15, x25, x12
        mov     x12, x22
        stp     x17, x19, [x1]
        stp     x20, x21, [x1, #16]
        sub     x27, x27, #0x20
        b Lbignum_emontredc_8n_madddone


Lbignum_emontredc_8n_maddloop_firstitr:

mov x16, v25.d[0] //umulh x16,x4,x8
mov x22, v24.d[1] //mul x22, x5, x9

mov x20, v25.d[1] //umulh x20,x5,x9
mov x23, v26.d[0] //mul x23, x6, x10

mov x21, v27.d[0] //umulh x21,x6,x10
mov x24, v26.d[1] //mul x24, x7, x11

mov x3, v27.d[1] //umulh x3,x7,x11
mov x17, v24.d[0] //mul x17, x4, x8

        adds x22,x22,x16
        adcs x23,x23,x20
        adcs x24,x24,x21
        adc x25,x3,xzr

// pre-calculate the multiplications for the next iter.
// v25 ++ v24 = hi, lo of (x4 * x8, x5 * x9)
ldr q22, [x2, #64]
ldr q23, [x2, #80]

        add x2, x2, #32
        add x1, x1, #32

#define in1  v20
#define in2  v22
#define out_lo v24
#define out_hi v25
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s

        ldp x20,x21,[x1]
        adds x12,x12,x20
        adcs x13,x13,x21
        ldp x20,x21,[x1,#16]

umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s

        adcs x14,x14,x20
        adcs x15,x15,x21
        adc x16,xzr,xzr
        adds x19,x22,x17

movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s

        adcs x22,x23,x22
        adcs x23,x24,x23
        adcs x24,x25,x24
        adc x25,xzr,x25

and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32

        adds x20,x22,x17
        adcs x21,x23,x19
        adcs x22,x24,x22
        adcs x23,x25,x23

umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x24,xzr,x24
        adc x25,xzr,x25
        adds x17,x17,x12
        adcs x19,x19,x13

#define in1  v21
#define in2  v23
#define out_lo v26
#define out_hi v27
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s

        adcs x20,x20,x14
        adcs x21,x21,x15
        adcs x22,x22,x16
        adcs x23,x23,xzr

umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s

        adcs x24,x24,xzr
        adc x25,x25,xzr
        subs x15,x6,x7
        cneg x15,x15,cc

movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s

        csetm x12,cc
        subs x13,x11,x10
        cneg x13,x13,cc
        mul x14,x15,x13

and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32

        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12

umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x23,x23,x14
        eor x13,x13,x12
        adcs x24,x24,x13
        adc x25,x25,x12
        subs x15,x4,x5
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x9,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x19,x19,x14
        eor x13,x13,x12
        adcs x20,x20,x13
        adcs x21,x21,x12
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12

        stp x17,x19,[x1]

mov x16, v25.d[0] // hi bits of (x4 * x8)
mov x26, v27.d[0] // hi bits of (x6 * x10)
mov x3, v25.d[1] // hi bits of (x5 * x9)
mov x17, v27.d[1] // hi bits of (x6 * x10)

        subs x15,x5,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x22,x22,x14
        eor x13,x13,x12
        adcs x23,x23,x13
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x20,x20,x14
        eor x13,x13,x12
        adcs x21,x21,x13
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14
        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x5,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14

        stp x20,x21,[x1,#16]
mov x20, v24.d[1] // lo bits of (x5 * x9)
mov x21, v26.d[0] // lo bits of (x6 * x10)

        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x13,x23,x12
        adcs x14,x24,x12
        adc x15,x25,x12
        mov x12,x22

mov x24, v26.d[1] // lo bits of (x7 * x11)

        sub x27, x27, #32
        cmp x27, #32
        beq Lbignum_emontredc_8n_maddloop_last


Lbignum_emontredc_8n_maddloop:
        ldp x8, x9, [x2, #32]
        ldp x10, x11, [x2, #48]

// pre-calculate the multiplications for the next iter.
// v25 ++ v24 = hi, lo of (x4 * x8, x5 * x9)
ldr q22, [x2, #64]
ldr q23, [x2, #80]

        add x2, x2, #32
        add x1, x1, #32

        adds x22,x20,x16
        adcs x23,x21,x3
        adcs x24,x24,x26
        adc x25,x17,xzr
mov x17, v24.d[0] // lo bits of (x4 * x8)

#define in1  v20
#define in2  v22
#define out_lo v24
#define out_hi v25
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s

        ldp x20,x21,[x1]
        adds x12,x12,x20
        adcs x13,x13,x21
        ldp x20,x21,[x1,#16]

umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s

        adcs x14,x14,x20
        adcs x15,x15,x21
        adc x16,xzr,xzr
        adds x19,x22,x17

movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s

        adcs x22,x23,x22
        adcs x23,x24,x23
        adcs x24,x25,x24
        adc x25,xzr,x25

and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32

        adds x20,x22,x17
        adcs x21,x23,x19
        adcs x22,x24,x22
        adcs x23,x25,x23

umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x24,xzr,x24
        adc x25,xzr,x25
        adds x17,x17,x12
        adcs x19,x19,x13

#define in1  v21
#define in2  v23
#define out_lo v26
#define out_hi v27
uzp2    v3.4s, in2.4s, in1.4s
xtn     v4.2s, in1.2d
xtn     v5.2s, in2.2d
rev64   v1.4s, in2.4s

        adcs x20,x20,x14
        adcs x21,x21,x15
        adcs x22,x22,x16
        adcs x23,x23,xzr

umull   v6.2d, v4.2s, v5.2s
umull   v7.2d, v4.2s, v3.2s
uzp2    v16.4s, in1.4s, in1.4s
mul     v0.4s, v1.4s, in1.4s

        adcs x24,x24,xzr
        adc x25,x25,xzr
        subs x15,x6,x7
        cneg x15,x15,cc

movi    v2.2d, #0x000000ffffffff
usra    v7.2d, v6.2d, #32
umull   out_hi.2d, v16.2s, v3.2s
uaddlp  v0.2d, v0.4s

        csetm x12,cc
        subs x13,x11,x10
        cneg x13,x13,cc
        mul x14,x15,x13

and     v2.16b, v7.16b, v2.16b
umlal   v2.2d, v16.2s, v5.2s
shl     out_lo.2d, v0.2d, #32
usra    out_hi.2d, v7.2d, #32

        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12

umlal   out_lo.2d, v4.2s, v5.2s
usra    out_hi.2d, v2.2d, #32
#undef in1
#undef in2
#undef out_lo
#undef out_hi

        adcs x23,x23,x14
        eor x13,x13,x12
        adcs x24,x24,x13
        adc x25,x25,x12
        subs x15,x4,x5
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x9,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x19,x19,x14
        eor x13,x13,x12
        adcs x20,x20,x13
        adcs x21,x21,x12
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12

        stp x17,x19,[x1]

mov x16, v25.d[0] // hi bits of (x4 * x8)
mov x26, v27.d[0] // hi bits of (x6 * x10)
mov x3, v25.d[1] // hi bits of (x5 * x9)
mov x17, v27.d[1] // hi bits of (x6 * x10)

        subs x15,x5,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x22,x22,x14
        eor x13,x13,x12
        adcs x23,x23,x13
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x20,x20,x14
        eor x13,x13,x12
        adcs x21,x21,x13
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14
        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x5,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14

        stp x20,x21,[x1,#16]
mov x20, v24.d[1] // lo bits of (x5 * x9)
mov x21, v26.d[0] // lo bits of (x6 * x10)

        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x13,x23,x12
        adcs x14,x24,x12
        adc x15,x25,x12
        mov x12,x22

mov x24, v26.d[1] // lo bits of (x7 * x11)

        sub x27, x27, #32
        cmp x27, #32
        bne Lbignum_emontredc_8n_maddloop


Lbignum_emontredc_8n_maddloop_last:
        ldp x8, x9, [x2, #32]
        ldp x10, x11, [x2, #48]

        add x2, x2, #32
        add x1, x1, #32

        adds x22,x20,x16
        adcs x23,x21,x3
        adcs x24,x24,x26
        adc x25,x17,xzr
mov x17, v24.d[0] // lo bits of (x4 * x8)

        ldp x20,x21,[x1]
        adds x12,x12,x20
        adcs x13,x13,x21
        ldp x20,x21,[x1,#16]
        adcs x14,x14,x20
        adcs x15,x15,x21
        adc x16,xzr,xzr
        adds x19,x22,x17
        adcs x22,x23,x22
        adcs x23,x24,x23
        adcs x24,x25,x24
        adc x25,xzr,x25
        adds x20,x22,x17
        adcs x21,x23,x19
        adcs x22,x24,x22
        adcs x23,x25,x23
        adcs x24,xzr,x24
        adc x25,xzr,x25
        adds x17,x17,x12
        adcs x19,x19,x13
        adcs x20,x20,x14
        adcs x21,x21,x15
        adcs x22,x22,x16
        adcs x23,x23,xzr
        adcs x24,x24,xzr
        adc x25,x25,xzr
        subs x15,x6,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x10
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x23,x23,x14
        eor x13,x13,x12
        adcs x24,x24,x13
        adc x25,x25,x12
        subs x15,x4,x5
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x9,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x19,x19,x14
        eor x13,x13,x12
        adcs x20,x20,x13
        adcs x21,x21,x12
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x5,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x22,x22,x14
        eor x13,x13,x12
        adcs x23,x23,x13
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x20,x20,x14
        eor x13,x13,x12
        adcs x21,x21,x13
        adcs x22,x22,x12
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x4,x7
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x11,x8
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14
        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x23,x23,x12
        adcs x24,x24,x12
        adc x25,x25,x12
        subs x15,x5,x6
        cneg x15,x15,cc
        csetm x12,cc
        subs x13,x10,x9
        cneg x13,x13,cc
        mul x14,x15,x13
        umulh x13,x15,x13
        cinv x12,x12,cc
        adds xzr,x12,#1
        eor x14,x14,x12
        adcs x21,x21,x14
        eor x13,x13,x12
        adcs x22,x22,x13
        adcs x13,x23,x12
        adcs x14,x24,x12
        adc x15,x25,x12
        mov x12,x22
        stp x17,x19,[x1]
        stp x20,x21,[x1,#16]
        subs x27, x27, #64

Lbignum_emontredc_8n_madddone:
        ldp x17, x19, [x1, #32]
        ldp x20, x21, [x1, #48]
        ldp x26, xzr, [sp, #16]
        adds xzr, x28, x28
        adcs x17, x17, x12
        adcs x19, x19, x13
        adcs x20, x20, x14
        adcs x21, x21, x15
        csetm x28, cs
        stp x17, x19, [x1, #32]
        stp x20, x21, [x1, #48]
        sub x1, x1, x0
        sub x2, x2, x0
        add x1, x1, #32
        subs x26, x26, #1
        stp x26, xzr, [sp, #16]
        bne Lbignum_emontredc_8n_outerloop
        neg x0, x28

Lbignum_emontredc_8n_end:
        CFI_INC_SP(32)

        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_emontredc_8n)

