// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// **********************************************************************
// This code is substantially derived from Emil Lenngren's implementation
//
//      https://github.com/Emill/X25519-AArch64/blob/master/X25519_AArch64.pdf
//      https://github.com/Emill/X25519-AArch64
//
// and the SLOTHY-based re-engineering of that code by Abdulrahman, Becker,
// Kannwischer and Klein:
//
//      https://eprint.iacr.org/2022/1303.pdf
//      https://github.com/slothy-optimizer/slothy/tree/main/paper
// **********************************************************************

// ----------------------------------------------------------------------------
// The x25519 function for curve25519 (byte array arguments)
// Inputs scalar[32] (bytes), point[32] (bytes); output res[32] (bytes)
//
// extern void curve25519_x25519_byte
//   (uint8_t res[static 32],const uint8_t scalar[static 32],
//    const uint8_t point[static 32]);
//
// Given a scalar n and the X coordinate of an input point P = (X,Y) on
// curve25519 (Y can live in any extension field of characteristic 2^255-19),
// this returns the X coordinate of n * P = (X, Y), or 0 when n * P is the
// point at infinity. Both n and X inputs are first slightly modified/mangled
// as specified in the relevant RFC (https://www.rfc-editor.org/rfc/rfc7748);
// in particular the lower three bits of n are set to zero. Does not implement
// the zero-check specified in Section 6.1.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519_byte)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519_byte)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519_byte)

        .text
        .balign 4

// Pointer-offset pairs for temporaries on stack

#define scalar sp, #0
#define pointx sp, #32
#define mask1 sp, #72
#define mask2 sp, #80
#define tmpa sp, #88
#define tmpb sp, #128
#define xn sp, #128
#define zn sp, #160

#define res sp, #192
#define i sp, #200
#define swap sp, #208

// Total size to reserve on the stack

#define NSPACE 224
#define regsave sp, #NSPACE

S2N_BN_SYMBOL(curve25519_x25519_byte):
        CFI_START

// Save registers and make additional room #NSPACE for temporaries.
// We only need to save the low 64-bits of the Q8...Q15 registers
// according to the ABI, so we use a save of the D8...D15 forms.

        CFI_DEC_SP(384)
        CFI_STACKSAVE2X(d8,d9,224,232)
        CFI_STACKSAVE2X(d10,d11,240,248)
        CFI_STACKSAVE2X(d12,d13,256,264)
        CFI_STACKSAVE2X(d14,d15,272,280)
        CFI_STACKSAVE2X(x19,x20,288,296)
        CFI_STACKSAVE2X(x21,x22,304,312)
        CFI_STACKSAVE2X(x23,x24,320,328)
        CFI_STACKSAVE2X(x25,x26,336,344)
        CFI_STACKSAVE2X(x27,x28,352,360)
        CFI_STACKSAVE2X(x29,x30,368,376)

// Move the output pointer to a stable place

        str     x0, [res]

// Copy the scalar to the corresponding local variable while
// mangling it. In principle it becomes 01xxx...xxx000 where
// the xxx are the corresponding bits of the original input
// scalar. We actually don't bother forcing the MSB to zero,
// but rather start the main loop below at 254 instead of 255.

        ldrb    w10, [x1]
        ldrb    w0, [x1, #1]
        orr     x10, x10, x0, lsl #8
        ldrb    w0, [x1, #2]
        orr     x10, x10, x0, lsl #16
        ldrb    w0, [x1, #3]
        orr     x10, x10, x0, lsl #24
        ldrb    w0, [x1, #4]
        orr     x10, x10, x0, lsl #32
        ldrb    w0, [x1, #5]
        orr     x10, x10, x0, lsl #40
        ldrb    w0, [x1, #6]
        orr     x10, x10, x0, lsl #48
        ldrb    w0, [x1, #7]
        orr     x10, x10, x0, lsl #56
        ldrb    w11, [x1, #8]
        ldrb    w0, [x1, #9]
        orr     x11, x11, x0, lsl #8
        ldrb    w0, [x1, #10]
        orr     x11, x11, x0, lsl #16
        ldrb    w0, [x1, #11]
        orr     x11, x11, x0, lsl #24
        ldrb    w0, [x1, #12]
        orr     x11, x11, x0, lsl #32
        ldrb    w0, [x1, #13]
        orr     x11, x11, x0, lsl #40
        ldrb    w0, [x1, #14]
        orr     x11, x11, x0, lsl #48
        ldrb    w0, [x1, #15]
        orr     x11, x11, x0, lsl #56
        bic     x10, x10, #7
        stp     x10, x11, [scalar]

        ldrb    w12, [x1, #16]
        ldrb    w0, [x1, #17]
        orr     x12, x12, x0, lsl #8
        ldrb    w0, [x1, #18]
        orr     x12, x12, x0, lsl #16
        ldrb    w0, [x1, #19]
        orr     x12, x12, x0, lsl #24
        ldrb    w0, [x1, #20]
        orr     x12, x12, x0, lsl #32
        ldrb    w0, [x1, #21]
        orr     x12, x12, x0, lsl #40
        ldrb    w0, [x1, #22]
        orr     x12, x12, x0, lsl #48
        ldrb    w0, [x1, #23]
        orr     x12, x12, x0, lsl #56
        ldrb    w13, [x1, #24]
        ldrb    w0, [x1, #25]
        orr     x13, x13, x0, lsl #8
        ldrb    w0, [x1, #26]
        orr     x13, x13, x0, lsl #16
        ldrb    w0, [x1, #27]
        orr     x13, x13, x0, lsl #24
        ldrb    w0, [x1, #28]
        orr     x13, x13, x0, lsl #32
        ldrb    w0, [x1, #29]
        orr     x13, x13, x0, lsl #40
        ldrb    w0, [x1, #30]
        orr     x13, x13, x0, lsl #48
        ldrb    w0, [x1, #31]
        orr     x13, x13, x0, lsl #56
        orr     x13, x13, #0x4000000000000000
        stp     x12, x13, [scalar+16]

// Discard the MSB of the point X coordinate (this is in
// accordance with the RFC, mod 2^255, *not* 2^255-19).
// Then recode it into the unsaturated base 25.5 form.

        ldrb    w4, [x2]
        ldrb    w0, [x2, #1]
        orr     x4, x4, x0, lsl #8
        ldrb    w0, [x2, #2]
        orr     x4, x4, x0, lsl #16
        ldrb    w0, [x2, #3]
        orr     x4, x4, x0, lsl #24
        ldrb    w0, [x2, #4]
        orr     x4, x4, x0, lsl #32
        ldrb    w0, [x2, #5]
        orr     x4, x4, x0, lsl #40
        ldrb    w0, [x2, #6]
        orr     x4, x4, x0, lsl #48
        ldrb    w0, [x2, #7]
        orr     x4, x4, x0, lsl #56
        ldrb    w5, [x2, #8]
        ldrb    w0, [x2, #9]
        orr     x5, x5, x0, lsl #8
        ldrb    w0, [x2, #10]
        orr     x5, x5, x0, lsl #16
        ldrb    w0, [x2, #11]
        orr     x5, x5, x0, lsl #24
        ldrb    w0, [x2, #12]
        orr     x5, x5, x0, lsl #32
        ldrb    w0, [x2, #13]
        orr     x5, x5, x0, lsl #40
        ldrb    w0, [x2, #14]
        orr     x5, x5, x0, lsl #48
        ldrb    w0, [x2, #15]
        orr     x5, x5, x0, lsl #56

        ldrb    w6, [x2, #16]
        ldrb    w0, [x2, #17]
        orr     x6, x6, x0, lsl #8
        ldrb    w0, [x2, #18]
        orr     x6, x6, x0, lsl #16
        ldrb    w0, [x2, #19]
        orr     x6, x6, x0, lsl #24
        ldrb    w0, [x2, #20]
        orr     x6, x6, x0, lsl #32
        ldrb    w0, [x2, #21]
        orr     x6, x6, x0, lsl #40
        ldrb    w0, [x2, #22]
        orr     x6, x6, x0, lsl #48
        ldrb    w0, [x2, #23]
        orr     x6, x6, x0, lsl #56
        ldrb    w7, [x2, #24]
        ldrb    w0, [x2, #25]
        orr     x7, x7, x0, lsl #8
        ldrb    w0, [x2, #26]
        orr     x7, x7, x0, lsl #16
        ldrb    w0, [x2, #27]
        orr     x7, x7, x0, lsl #24
        ldrb    w0, [x2, #28]
        orr     x7, x7, x0, lsl #32
        ldrb    w0, [x2, #29]
        orr     x7, x7, x0, lsl #40
        ldrb    w0, [x2, #30]
        orr     x7, x7, x0, lsl #48
        ldrb    w0, [x2, #31]
        orr     x7, x7, x0, lsl #56

        lsr     x12, x4, #51
        lsr     x17, x6, #51
        orr     x12, x12, x5, lsl #13
        orr     x17, x17, x7, lsl #13
        ubfx    x8, x7, #12, #26
        ubfx    x9, x7, #38, #25
        ubfx    x11, x4, #26, #25
        ubfx    x13, x5, #13, #25
        lsr     x14, x5, #38
        ubfx    x16, x6, #25, #26
        and     x10, x4, #0x3ffffff
        and     x12, x12, #0x3ffffff
        and     x15, x6, #0x1ffffff
        and     x17, x17, #0x1ffffff
        orr     x10, x10, x11, lsl #32
        orr     x11, x12, x13, lsl #32
        orr     x12, x14, x15, lsl #32
        orr     x13, x16, x17, lsl #32
        orr     x14, x8, x9, lsl #32

        stp     x10, x11, [pointx+0]
        stp     x12, x13, [pointx+16]
        str     x14, [pointx+32]

// Initialize (X2,Z2) = (1,0), the identity (projective point at infinity)

        mov     x1, #1
        mov     v0.d[0], x1
        mov     v2.d[0], xzr
        mov     v4.d[0], xzr
        mov     v6.d[0], xzr
        mov     v8.d[0], xzr

        mov     v1.d[0], xzr
        mov     v3.d[0], xzr
        mov     v5.d[0], xzr
        mov     v7.d[0], xzr
        mov     v9.d[0], xzr

// Initialize (X3,Z3) = (X,1), projective representation of X

        mov     v10.d[0], x10
        mov     v12.d[0], x11
        mov     v14.d[0], x12
        mov     v16.d[0], x13
        mov     v18.d[0], x14

        mov     v11.d[0], x1
        mov     v13.d[0], xzr
        mov     v15.d[0], xzr
        mov     v17.d[0], xzr
        mov     v19.d[0], xzr

// Set up some constants used repeatedly in the main loop:
//
// Q31 = 0x1300000013 (two 32-bit copies of 19)
// Q30 = 0x3ffffff0000000003ffffff (two 64-bit copies of 2^26-1)
// Q29 = mask1 = (0x07ffffc,0x07fffffe)
// Q28 = mask2 = (0x07ffffb4,0x07fffffe)

        mov     w0, #19
        add     x0, x0, x0, lsl #32
        mov     v31.d[0], x0
        mov     v31.d[1], xzr

        mov     x0, #67108863 // #(1<<26)-1
        mov     v30.d[0], x0
        mov     v30.d[1], x0

        mov     x0, #0x07fffffe07fffffe
        sub     x1, x0, #74 // #0xfe-0xb4
        sub     x0, x0, #2

        stp     x0, x1, [mask1]
        ldp     d29, d28, [mask1]

// The main loop over (modified) bits from i = 254, ..., i = 0 (inclusive);
// we explicitly skip bit 255 because it should be forced to zero initially.
// This is a classic Montgomery ladder using a "swap" variable.
// It's assumed x0 = i at the start of the loop, but that is volatile and
// needs to be reloaded from memory at the end of the loop.

        str     xzr, [swap]
        mov     x0, #254
        str     x0, [i]

Lcurve25519_x25519_byte_scalarloop:

        lsr     x1, x0, #6
        ldr     x2, [sp, x1, lsl #3]    // Exploiting scalar = sp exactly
        lsr     x2, x2, x0
        and     x2, x2, #1

        ldr     x0, [swap]
        cmp     x0, x2
        str     x2, [swap]

// The following inner loop code is derived closely following Lenngren's
// implementation available at "https://github.com/Emill/X25519-AArch64".
// In particular, the basic dataflow and the organization between integer
// and SIMD units is identical, with only a few minor changes to some
// individual instructions (for miscellaneous reasons). The scheduling
// was redone from scratch by SLOTHY starting from the un-interleaved
// form in the SLOTHY work cited above, and using the same scripts.
//
// The intermediate value annotations were added to provide data that
// is used in the formal proof, indicating which lines assign specific
// digits of the various intermediate results (mainly of field
// operations, sometimes other transformations). The names used for
// the intermediate results are similar but not identical to those in
// the abstract Algorithm 1 description in Lenngren's paper. Almost
// all equations are to be interpreted as field operations, i.e. as
// arithmetic modulo 2^255-19, not simple numeric equalities.
//
//      b = x2 - z2
//      d = x3 - z3
//      a = x2 + z2
//      c = x3 + z3
//      f = if flip then c else a
//      g = if flip then d else b
//      aa = f^2
//      bb = g^2
//      bbalt = bb (change of representation)
//      e = aa - bb
//      bce = bbalt + 121666 * e
//      z4 = bce * e
//      bc = b * c
//      ad = a * d
//      t1 = ad + bc
//      t2 = ad - bc
//      x5 = t1^2
//      t3 = t2^2
//      x4 = aa * bb
//      z5 = x * t3
//
// Then the main variables are updated for the next iteration as
//
//      (x2',z2') = (x4,z4)
//      (x3',z3') = (x5,z5)

        add     v22.2s, v2.2s, v3.2s            // ubignum_of_qreglist 1 // INTERMEDIATE a
        sub     v21.2s, v28.2s, v1.2s
        add     v25.2s, v0.2s, v1.2s            // ubignum_of_qreglist 0 // INTERMEDIATE a
        sub     v24.2s, v29.2s, v3.2s
        add     v3.2s, v18.2s, v19.2s           // ubignum_of_qreglist 4 // INTERMEDIATE c
        add     v0.2s, v0.2s, v21.2s            // ubignum_of_qreglist 0 // INTERMEDIATE b
        sub     v20.2s, v29.2s, v15.2s
        sub     v1.2s, v29.2s, v5.2s
        sub     v26.2s, v28.2s, v11.2s
        sub     v21.2s, v29.2s, v19.2s
        add     v19.2s, v10.2s, v11.2s          // ubignum_of_qreglist 0 // INTERMEDIATE c
        add     v11.2s, v14.2s, v20.2s          // ubignum_of_qreglist 2 // INTERMEDIATE d
        add     v21.2s, v18.2s, v21.2s          // ubignum_of_qreglist 4 // INTERMEDIATE d
        sub     v20.2s, v29.2s, v17.2s
        add     v18.2s, v2.2s, v24.2s           // ubignum_of_qreglist 1 // INTERMEDIATE b
        add     v14.2s, v14.2s, v15.2s          // ubignum_of_qreglist 2 // INTERMEDIATE c
        add     v15.2s, v16.2s, v17.2s          // ubignum_of_qreglist 3 // INTERMEDIATE c
        add     v2.2s, v16.2s, v20.2s           // ubignum_of_qreglist 3 // INTERMEDIATE d
        add     v24.2s, v12.2s, v13.2s          // ubignum_of_qreglist 1 // INTERMEDIATE c
        add     v26.2s, v10.2s, v26.2s          // ubignum_of_qreglist 0 // INTERMEDIATE d
        sub     v10.2s, v29.2s, v13.2s
        sub     v13.2s, v29.2s, v7.2s
        add     v23.2s, v6.2s, v7.2s            // ubignum_of_qreglist 3 // INTERMEDIATE a
        sub     v7.2s, v29.2s, v9.2s
        add     v27.2s, v12.2s, v10.2s          // ubignum_of_qreglist 1 // INTERMEDIATE d
        fcsel   d20, d22, d24, eq               // ubignum_of_qreglist 1 // INTERMEDIATE f
        add     v28.2s, v4.2s, v5.2s            // ubignum_of_qreglist 2 // INTERMEDIATE a
        fcsel   d12, d23, d15, eq               // ubignum_of_qreglist 3 // INTERMEDIATE f
        add     v7.2s, v8.2s, v7.2s             // ubignum_of_qreglist 4 // INTERMEDIATE b
        fcsel   d16, d25, d19, eq               // ubignum_of_qreglist 0 // INTERMEDIATE f
        mov     x0, v20.d[0]
        fcsel   d5, d28, d14, eq                // ubignum_of_qreglist 2 // INTERMEDIATE f
        mov     x21, v12.d[0]
        fcsel   d29, d7, d21, eq                // ubignum_of_qreglist 4 // INTERMEDIATE g
        mov     x5, v16.d[0]
        lsr     x26, x0, #32
        add     x29, x21, x21
        umull   x15, w5, w29
        add     v13.2s, v6.2s, v13.2s           // ubignum_of_qreglist 3 // INTERMEDIATE b
        add     x12, x26, x26
        mov     x30, v5.d[0]
        fcsel   d10, d18, d27, eq               // ubignum_of_qreglist 1 // INTERMEDIATE g
        lsr     x11, x5, #32
        lsr     x10, x30, #32
        trn2    v20.2s, v21.2s, v3.2s
        add     v9.2s, v8.2s, v9.2s             // ubignum_of_qreglist 4 // INTERMEDIATE a
        add     x14, x11, x11
        trn2    v6.2s, v2.2s, v15.2s
        trn1    v12.2s, v25.2s, v0.2s
        add     v1.2s, v4.2s, v1.2s             // ubignum_of_qreglist 2 // INTERMEDIATE b
        trn1    v16.2s, v23.2s, v13.2s
        fcsel   d8, d13, d2, eq                 // ubignum_of_qreglist 3 // INTERMEDIATE g
        trn2    v17.2s, v27.2s, v24.2s
        str     d29, [tmpb+32]
        add     x17, x10, x10
        trn2    v4.2s, v28.2s, v1.2s
        trn1    v5.2s, v28.2s, v1.2s
        trn1    v28.2s, v2.2s, v15.2s
        trn1    v2.2s, v22.2s, v18.2s
        fcsel   d29, d0, d26, eq                // ubignum_of_qreglist 0 // INTERMEDIATE g
        trn2    v15.2s, v22.2s, v18.2s
        umull   v22.2d, v12.2s, v20.2s
        umull   x22, w30, w17
        stp     d29, d10, [tmpb+0]
        trn2    v10.2s, v23.2s, v13.2s
        trn2    v23.2s, v11.2s, v14.2s
        trn1    v13.2s, v27.2s, v24.2s
        fcsel   d27, d1, d11, eq                // ubignum_of_qreglist 2 // INTERMEDIATE g
        trn1    v14.2s, v11.2s, v14.2s
        umlal   v22.2d, v2.2s, v6.2s
        umull   x25, w30, w30
        umlal   v22.2d, v5.2s, v23.2s
        add     x3, x30, x30
        umlal   v22.2d, v16.2s, v17.2s
        add     w30, w21, w21, lsl #1;
        stp     d27, d8, [tmpb+16]
        add     w30, w30, w21, lsl #4
        trn1    v11.2s, v26.2s, v19.2s
        trn2    v8.2s, v26.2s, v19.2s
        trn2    v19.2s, v25.2s, v0.2s
        mul     v29.2s, v20.2s, v31.2s
        ldr     x20, [tmpb+24]
        umull   v25.2d, v19.2s, v6.2s
        add     x1, x0, x0
        umull   v27.2d, v19.2s, v23.2s
        umull   x9, w5, w1
        umull   v0.2d, v12.2s, v23.2s
        lsr     x24, x20, #32
        mul     v20.2s, v23.2s, v31.2s
        lsr     x16, x21, #32
        umlal   v25.2d, v15.2s, v23.2s
        umaddl  x13, w11, w14, x9
        umlal   v25.2d, v4.2s, v17.2s
        umaddl  x9, w14, w17, x15
        umull   v24.2d, v12.2s, v6.2s
        add     w2, w16, w16, lsl #1;
        fcsel   d26, d9, d3, eq                 // ubignum_of_qreglist 4 // INTERMEDIATE f
        add     w2, w2, w16, lsl #4
        trn1    v18.2s, v21.2s, v3.2s
        umull   v3.2d, v19.2s, v29.2s
        umull   x28, w5, w3
        mul     v1.2s, v6.2s, v31.2s
        umull   x8, w5, w5
        umlal   v24.2d, v2.2s, v23.2s
        umaddl  x13, w21, w30, x13
        mul     v23.2s, v17.2s, v31.2s
        umaddl  x27, w14, w12, x28
        trn2    v6.2s, v9.2s, v7.2s
        mov     x6, v26.d[0]
        umlal   v3.2d, v15.2s, v1.2s
        add     x16, x16, x16
        umlal   v3.2d, v4.2s, v20.2s
        lsr     x4, x6, #32
        umlal   v3.2d, v10.2s, v23.2s
        add     x7, x6, x6
        umull   v26.2d, v19.2s, v8.2s
        add     x23, x4, x4
        umaddl  x28, w5, w23, x22
        trn1    v7.2s, v9.2s, v7.2s
        umlal   v27.2d, v15.2s, v17.2s
        add     w15, w4, w4, lsl #1;
        umlal   v27.2d, v4.2s, v8.2s
        add     w15, w15, w4, lsl #4
        add     w22, w10, w10, lsl #1;
        umlal   v24.2d, v5.2s, v17.2s
        add     w22, w22, w10, lsl #4
        umaddl  x10, w11, w7, x28
        umlal   v25.2d, v10.2s, v8.2s
        umull   x21, w5, w16
        umlal   v25.2d, v6.2s, v29.2s
        umaddl  x23, w15, w23, x25
        umlal   v27.2d, v10.2s, v29.2s
        umull   x19, w5, w12
        umlal   v27.2d, v6.2s, v1.2s
        umaddl  x25, w11, w29, x21
        umlal   v0.2d, v2.2s, v17.2s
        umaddl  x28, w0, w3, x9
        shl     v21.2d, v25.2d, #1
        umaddl  x4, w11, w1, x19
        umaddl  x21, w2, w29, x4
        mul     v25.2s, v8.2s, v31.2s
        umlal   v24.2d, v16.2s, v8.2s
        umaddl  x19, w0, w17, x25
        umlal   v24.2d, v7.2s, v29.2s
        umull   x25, w5, w17
        umlal   v24.2d, v19.2s, v28.2s
        umaddl  x4, w0, w16, x10
        umull   v9.2d, v12.2s, v8.2s
        umaddl  x23, w5, w7, x23
        umlal   v21.2d, v12.2s, v18.2s
        add     w10, w6, w6, lsl #1;
        shl     v27.2d, v27.2d, #1
        add     w10, w10, w6, lsl #4
        umaddl  x28, w26, w12, x28
        umlal   v26.2d, v15.2s, v29.2s
        umaddl  x9, w14, w16, x23
        umlal   v9.2d, v2.2s, v29.2s
        umaddl  x22, w22, w17, x8
        umlal   v21.2d, v2.2s, v28.2s
        umaddl  x28, w6, w10, x28
        umaddl  x27, w0, w0, x27
        add     x8, x14, x14
        umlal   v0.2d, v5.2s, v8.2s
        umull   x5, w5, w14
        umlal   v9.2d, v5.2s, v1.2s
        umaddl  x14, w0, w29, x9
        umlal   v26.2d, v4.2s, v1.2s
        umaddl  x6, w2, w16, x27
        umlal   v22.2d, v7.2s, v8.2s
        umaddl  x5, w30, w17, x5
        umaddl  x5, w2, w3, x5
        add     x23, x17, x17
        umlal   v27.2d, v12.2s, v28.2s
        umaddl  x13, w2, w23, x13
        umlal   v26.2d, v10.2s, v20.2s
        add     x9, x12, x12
        umlal   v9.2d, v16.2s, v20.2s
        umaddl  x27, w10, w29, x6
        umlal   v0.2d, v16.2s, v29.2s
        umaddl  x6, w11, w3, x25
        umlal   v22.2d, v19.2s, v18.2s
        umaddl  x19, w26, w3, x19
        mul     v18.2s, v18.2s, v31.2s
        umaddl  x23, w15, w23, x27
        umlal   v3.2d, v6.2s, v25.2s
        umaddl  x0, w0, w12, x6
        umlal   v0.2d, v7.2s, v1.2s
        add     x11, x16, x16
        umlal   v9.2d, v7.2s, v23.2s
        umaddl  x6, w12, w17, x14
        umlal   v9.2d, v19.2s, v11.2s
        umaddl  x25, w26, w29, x4
        umlal   v9.2d, v15.2s, v18.2s
        umaddl  x14, w10, w3, x13
        umull   v25.2d, v12.2s, v17.2s
        umaddl  x27, w10, w16, x0
        umlal   v26.2d, v6.2s, v23.2s
        add     x0, x25, x6, lsr #26
        mul     v23.2s, v28.2s, v31.2s
        umaddl  x12, w10, w12, x5
        shl     v3.2d, v3.2d, #1
        add     x16, x22, x0, lsr #25
        umlal   v21.2d, v5.2s, v14.2s
        bic     x22, x0, #0x1ffffff
        umlal   v3.2d, v12.2s, v11.2s
        add     x26, x16, x22, lsr #24
        umlal   v3.2d, v2.2s, v18.2s
        umaddl  x16, w10, w17, x21
        umlal   v3.2d, v5.2s, v23.2s
        add     x22, x26, x22, lsr #21
        umlal   v9.2d, v4.2s, v23.2s
        umaddl  x5, w15, w29, x27
        umull   v17.2d, v19.2s, v17.2s
        umaddl  x17, w30, w3, x22
        umlal   v25.2d, v2.2s, v8.2s
        umaddl  x25, w15, w3, x16
        umlal   v25.2d, v5.2s, v29.2s
        umaddl  x26, w15, w7, x19
        umlal   v0.2d, v19.2s, v14.2s
        umaddl  x17, w2, w9, x17
        umlal   v17.2d, v15.2s, v8.2s
        ldr     x19, [tmpb+0]
        umlal   v17.2d, v4.2s, v29.2s
        ldr     x7, [tmpb+8]
        shl     v29.2d, v26.2d, #1
        umaddl  x13, w10, w1, x17
        umlal   v0.2d, v15.2s, v13.2s
        lsr     x2, x19, #32
        umlal   v29.2d, v12.2s, v13.2s
        umaddl  x27, w15, w1, x12
        umlal   v29.2d, v2.2s, v11.2s
        umaddl  x30, w15, w8, x13
        umlal   v29.2d, v5.2s, v18.2s
        add     x4, x7, x7
        umlal   v29.2d, v16.2s, v23.2s
        umaddl  x29, w15, w9, x14
        umlal   v0.2d, v4.2s, v11.2s
        add     x17, x27, x30, lsr #26
        umlal   v0.2d, v10.2s, v18.2s
        umaddl  x16, w15, w11, x28
        umlal   v0.2d, v6.2s, v23.2s
        add     x1, x29, x17, lsr #25
        umlal   v25.2d, v16.2s, v1.2s
        umull   x11, w19, w4
        ldr     x8, [tmpb+32]
        mul     v26.2s, v14.2s, v31.2s
        umlal   v17.2d, v10.2s, v1.2s
        ldr     x15, [tmpb+16]
        umlal   v17.2d, v6.2s, v20.2s
        and     x9, x30, #0x3ffffff
        bfi     x9, x17, #32, #25               // ubignum_of_preglist 0 // INTERMEDIATE aa
        add     x17, x2, x2
        lsr     x10, x15, #32
        add     x27, x25, x1, lsr #26
        umlal   v25.2d, v7.2s, v20.2s
        add     x13, x10, x10
        umlal   v25.2d, v19.2s, v13.2s
        add     x29, x23, x27, lsr #25
        umlal   v25.2d, v15.2s, v11.2s
        lsr     x30, x8, #32
        umlal   v25.2d, v4.2s, v18.2s
        add     x23, x5, x29, lsr #26
        umlal   v25.2d, v10.2s, v23.2s
        and     x14, x29, #0x3ffffff
        umlal   v25.2d, v6.2s, v26.2s
        add     x5, x16, x23, lsr #25
        shl     v8.2d, v17.2d, #1
        umaddl  x12, w2, w17, x11
        and     x29, x5, #0x3ffffff
        umull   x21, w19, w19
        umlal   v29.2d, v7.2s, v26.2s
        add     w16, w10, w10, lsl #1;
        umlal   v3.2d, v16.2s, v26.2s
        add     w16, w16, w10, lsl #4
        bfi     x14, x23, #32, #25              // ubignum_of_preglist 2 // INTERMEDIATE aa
        add     w10, w24, w24, lsl #1;
        add     x22, x26, x5, lsr #26
        add     w10, w10, w24, lsl #4
        umlal   v8.2d, v12.2s, v14.2s
        umaddl  x25, w16, w13, x21
        umlal   v8.2d, v2.2s, v13.2s
        bfi     x29, x22, #32, #25              // ubignum_of_preglist 3 // INTERMEDIATE aa
        umlal   v8.2d, v5.2s, v11.2s
        add     x26, x24, x24
        umlal   v8.2d, v16.2s, v18.2s
        stp     x14, x29, [tmpa+16]
        umlal   v8.2d, v7.2s, v23.2s
        add     w24, w30, w30, lsl #1;
        usra    v25.2d, v29.2d, #26
        add     w24, w24, w30, lsl #4
        umull   x29, w15, w15
        umlal   v27.2d, v2.2s, v14.2s
        umull   x3, w15, w13
        umlal   v27.2d, v5.2s, v13.2s
        add     x21, x20, x20
        umlal   v24.2d, v15.2s, v14.2s
        umull   x5, w19, w21
        umlal   v24.2d, v4.2s, v13.2s
        and     x11, x1, #0x3ffffff
        usra    v8.2d, v25.2d, #25
        and     x1, x0, #0x1ffffff
        umlal   v27.2d, v16.2s, v11.2s
        umaddl  x23, w17, w13, x5
        umlal   v27.2d, v7.2s, v18.2s
        add     x5, x30, x30
        usra    v0.2d, v8.2d, #26
        add     x0, x15, x15
        umlal   v24.2d, v10.2s, v11.2s
        umaddl  x23, w7, w0, x23
        umlal   v24.2d, v6.2s, v18.2s
        lsr     x30, x7, #32
        usra    v27.2d, v0.2d, #25
        add     x16, x30, x30
        and     v20.16b, v8.16b, v30.16b        // ubignum_of_hreglist 4 + ubignum_of_lreglist 4 // INTERMEDIATE H|L = bc|ad
        umaddl  x15, w30, w16, x23
        ushr    v23.2d, v30.2d, #1
        add     w23, w8, w8, lsl #1;
        usra    v24.2d, v27.2d, #26
        add     w23, w23, w8, lsl #4
        umaddl  x14, w19, w5, x3
        and     v8.16b, v27.16b, v30.16b        // ubignum_of_hreglist 6 + ubignum_of_lreglist 6 // INTERMEDIATE H|L = bc|ad
        add     x28, x8, x8
        and     v27.16b, v0.16b, v23.16b        // ubignum_of_hreglist 5 + ubignum_of_lreglist 5 // INTERMEDIATE H|L = bc|ad
        umaddl  x8, w8, w23, x15
        and     v5.16b, v24.16b, v23.16b        // ubignum_of_hreglist 7 + ubignum_of_lreglist 7 // INTERMEDIATE H|L = bc|ad
        umaddl  x3, w2, w28, x14
        umlal   v22.2d, v15.2s, v28.2s
        bfi     x11, x27, #32, #25              // ubignum_of_preglist 1 // INTERMEDIATE aa
        uzp1    v5.4s, v8.4s, v5.4s
        umaddl  x14, w24, w5, x29
        umaddl  x5, w19, w28, x14
        ldr     d18, [mask1]
        mov     v18.d[1], v18.d[0]
        umaddl  x15, w7, w26, x3
        mul     v12.2s, v13.2s, v31.2s
        umlal   v21.2d, v16.2s, v13.2s
        stp     x9, x11, [tmpa+0]
        umlal   v21.2d, v7.2s, v11.2s
        umaddl  x29, w17, w26, x5
        umlal   v22.2d, v4.2s, v14.2s
        add     w14, w20, w20, lsl #1;
        umlal   v22.2d, v10.2s, v13.2s
        add     w14, w14, w20, lsl #4
        umull   x3, w19, w0
        umlal   v22.2d, v6.2s, v11.2s
        umaddl  x29, w7, w21, x29
        usra    v21.2d, v24.2d, #25
        umaddl  x11, w20, w14, x12
        and     v0.16b, v25.16b, v23.16b
        umaddl  x5, w30, w21, x15
        and     v14.16b, v29.16b, v30.16b
        umaddl  x12, w16, w13, x29
        usra    v22.2d, v21.2d, #26
        umaddl  x29, w17, w16, x3
        umlal   v3.2d, v7.2s, v12.2s
        add     x9, x26, x26
        and     v1.16b, v21.16b, v30.16b        // ubignum_of_hreglist 8 + ubignum_of_lreglist 8 // INTERMEDIATE H|L = bc|ad
        add     x27, x5, x12, lsr #26
        bic     v8.16b, v22.16b, v23.16b
        umaddl  x29, w7, w7, x29
        and     v17.16b, v22.16b, v23.16b       // ubignum_of_hreglist 9 + ubignum_of_lreglist 9 // INTERMEDIATE H|L = bc|ad
        add     x5, x25, x27, lsr #25
        usra    v3.2d, v8.2d, #25
        umaddl  x25, w24, w9, x8
        umlal   v9.2d, v10.2s, v26.2s
        add     x8, x13, x13
        trn1    v22.4s, v1.4s, v17.4s
        umaddl  x11, w10, w8, x11
        usra    v3.2d, v8.2d, #24
        umull   x20, w19, w16
        add     v26.2s, v22.2s, v18.2s
        ldr     d28, [mask2]
        umlal   v9.2d, v6.2s, v12.2s
        umaddl  x3, w23, w0, x11
        usra    v3.2d, v8.2d, #21
        umaddl  x29, w10, w26, x29
        uzp1    v11.4s, v20.4s, v27.4s
        umaddl  x20, w2, w4, x20
        umaddl  x9, w10, w21, x20
        mov     v17.d[0], v22.d[1]
        usra    v9.2d, v3.2d, #26
        umull   x15, w19, w13
        and     v7.16b, v3.16b, v30.16b         // ubignum_of_hreglist 0 + ubignum_of_lreglist 0 // INTERMEDIATE H|L = bc|ad
        add     x11, x16, x16
        uzp2    v1.4s, v11.4s, v5.4s
        umaddl  x20, w23, w13, x9
        and     v8.16b, v9.16b, v23.16b         // ubignum_of_hreglist 1 + ubignum_of_lreglist 1 // INTERMEDIATE H|L = bc|ad
        umaddl  x9, w2, w0, x15
        usra    v14.2d, v9.2d, #25
        and     x6, x6, #0x3ffffff
        uzp1    v7.4s, v7.4s, v8.4s
        umaddl  x29, w23, w21, x29
        uzp1    v27.4s, v11.4s, v5.4s
        umull   x15, w19, w26
        usra    v0.2d, v14.2d, #26              // ubignum_of_hreglist 3 + ubignum_of_lreglist 3 // INTERMEDIATE H|L = bc|ad
        add     x6, x6, x22, lsr #25
        and     v3.16b, v14.16b, v30.16b        // ubignum_of_hreglist 2 + ubignum_of_lreglist 2 // INTERMEDIATE H|L = bc|ad
        bic     x22, x27, #0x1ffffff
        sub     v2.2s, v26.2s, v17.2s
        add     v9.2s, v22.2s, v17.2s
        uzp1    v14.4s, v3.4s, v0.4s
        umaddl  x2, w2, w21, x15
        add     v5.4s, v27.4s, v18.4s
        add     x5, x5, x22, lsr #24
        zip1    v22.2s, v2.2s, v9.2s            // ubignum_of_h32reglist 8 + ubignum_of_l32reglist 8 // INTERMEDIATE H|L = t1|t2
        mov     v18.b[0], v28.b[0]
        uzp1    v8.4s, v7.4s, v14.4s
        add     x22, x5, x22, lsr #21
        uzp2    v3.4s, v7.4s, v14.4s
        umaddl  x5, w7, w16, x9
        add     v25.4s, v8.4s, v18.4s
        umaddl  x15, w14, w0, x22
        add     v12.4s, v27.4s, v1.4s
        add     x9, x17, x17
        sub     v14.4s, v5.4s, v1.4s
        umull   x19, w19, w17
        sub     v18.4s, v25.4s, v3.4s
        ldr     x22, [tmpa+8]
        add     v20.4s, v8.4s, v3.4s
        umaddl  x15, w10, w11, x15
        zip1    v16.4s, v14.4s, v12.4s          // ubignum_of_h32reglist 4 + ubignum_of_l32reglist 4 // INTERMEDIATE H|L = t1|t2
        umaddl  x14, w14, w13, x19
        zip2    v14.4s, v14.4s, v12.4s          // ubignum_of_h32reglist 6 + ubignum_of_l32reglist 6 // INTERMEDIATE H|L = t1|t2
        and     x17, x27, #0x1ffffff
        zip2    v0.4s, v18.4s, v20.4s           // ubignum_of_h32reglist 2 + ubignum_of_l32reglist 2 // INTERMEDIATE H|L = t1|t2
        umaddl  x15, w23, w4, x15
        zip1    v1.4s, v18.4s, v20.4s           // ubignum_of_h32reglist 0 + ubignum_of_l32reglist 0 // INTERMEDIATE H|L = t1|t2
        umaddl  x10, w10, w0, x14
        zip2    v5.2s, v2.2s, v9.2s             // ubignum_of_h32reglist 9 + ubignum_of_l32reglist 9 // INTERMEDIATE H|L = t1|t2
        shl     v24.2s, v0.2s, #1
        mov     v19.d[0], v1.d[1]               // ubignum_of_h32reglist 1 + ubignum_of_l32reglist 1 // INTERMEDIATE H|L = t1|t2
        shl     v26.2s, v22.2s, #1
        shl     v17.2s, v16.2s, #1
        mov     v15.d[0], v0.d[1]               // ubignum_of_h32reglist 3 + ubignum_of_l32reglist 3 // INTERMEDIATE H|L = t1|t2
        shl     v7.2s, v5.2s, #1
        shl     v18.2s, v19.2s, #1
        umull   v11.2d, v1.2s, v24.2s
        umaddl  x19, w23, w16, x10
        umull   v6.2d, v1.2s, v17.2s
        umaddl  x10, w7, w13, x2
        mov     v4.d[0], v16.d[1]               // ubignum_of_h32reglist 5 + ubignum_of_l32reglist 5 // INTERMEDIATE H|L = t1|t2
        mov     v10.d[0], v14.d[1]              // ubignum_of_h32reglist 7 + ubignum_of_l32reglist 7 // INTERMEDIATE H|L = t1|t2
        umull   v9.2d, v1.2s, v26.2s
        ldr     x13, [tmpa+0]
        shl     v28.2s, v15.2s, #1
        shl     v3.2s, v10.2s, #1
        ldr     x14, [tmpa+16]
        mul     v12.2s, v10.2s, v31.2s
        umull   v25.2d, v1.2s, v7.2s
        ldr     x2, [tmpa+24]
        umlal   v6.2d, v18.2s, v28.2s
        umaddl  x27, w30, w0, x10
        umaddl  x16, w24, w0, x20
        shl     v13.2s, v14.2s, #1
        umaddl  x5, w23, w26, x5
        mul     v2.2s, v22.2s, v31.2s
        umull   v21.2d, v1.2s, v13.2s
        umaddl  x23, w24, w8, x29
        umlal   v11.2d, v18.2s, v19.2s
        mov     x10, #0x07fffffe07fffffe
        sub     x10, x10, #2
        umaddl  x26, w24, w21, x5
        mul     v29.2s, v14.2s, v31.2s
        umlal   v25.2d, v19.2s, v26.2s
        add     x7, x1, x6, lsr #26
        mul     v20.2s, v4.2s, v31.2s
        and     x6, x6, #0x3ffffff
        shl     v8.2s, v18.2s, #1
        shl     v4.2s, v4.2s, #1
        umlal   v11.2d, v29.2s, v14.2s
        bfi     x6, x7, #32, #26                // ubignum_of_preglist 4 // INTERMEDIATE aa
        umlal   v25.2d, v0.2s, v3.2s
        umaddl  x0, w24, w4, x19
        umlal   v25.2d, v15.2s, v13.2s
        str     x6, [tmpa+32]
        umlal   v21.2d, v18.2s, v4.2s
        umaddl  x8, w24, w11, x3
        umlal   v21.2d, v0.2s, v17.2s
        ldr     x30, [tmpa+32]
        mul     v14.2s, v5.2s, v31.2s
        add     x2, x2, x10
        shl     v5.2s, v28.2s, #1
        shl     v27.2s, v4.2s, #1
        umlal   v6.2d, v0.2s, v0.2s
        umaddl  x11, w24, w9, x15
        umlal   v6.2d, v12.2s, v3.2s
        add     x4, x30, x10
        umlal   v11.2d, v14.2s, v5.2s
        add     x3, x22, x10
        umlal   v11.2d, v2.2s, v17.2s
        add     x6, x0, x11, lsr #26
        umlal   v11.2d, v12.2s, v27.2s
        add     x14, x14, x10
        umlal   v6.2d, v14.2s, v27.2s
        add     x8, x8, x6, lsr #25
        umlal   v6.2d, v2.2s, v13.2s
        movk    x10, #0xffb4
        umlal   v25.2d, v16.2s, v4.2s
        add     x29, x16, x8, lsr #26
        umull   v27.2d, v1.2s, v3.2s
        and     x11, x11, #0x3ffffff
        umlal   v9.2d, v18.2s, v3.2s
        add     x19, x13, x10
        umlal   v9.2d, v0.2s, v13.2s
        and     x5, x8, #0x3ffffff
        umlal   v9.2d, v28.2s, v4.2s
        bfi     x11, x6, #32, #25               // ubignum_of_preglist 0 // INTERMEDIATE bb
        umlal   v9.2d, v16.2s, v16.2s
        umaddl  x30, w24, w28, x27
        umlal   v9.2d, v14.2s, v7.2s
        sub     x13, x19, x11
        umull   v10.2d, v1.2s, v18.2s
        add     x7, x23, x29, lsr #25
        umlal   v21.2d, v28.2s, v15.2s
        lsr     x16, x13, #32                   // ubignum_of_wreglist 1 + ubignum_of_wreglist 0 // INTERMEDIATE e
        umlal   v21.2d, v2.2s, v22.2s
        add     x0, x26, x7, lsr #26
        usra    v25.2d, v9.2d, #26
        and     x20, x7, #0x3ffffff
        umull   v22.2d, v1.2s, v1.2s
        add     x8, x25, x0, lsr #25
        umull   v7.2d, v1.2s, v28.2s
        and     x1, x29, #0x1ffffff             // ubignum_of_xreglist 3 // INTERMEDIATE bbalt
        bic     v18.16b, v25.16b, v23.16b
        and     x19, x8, #0x3ffffff
        and     v16.16b, v9.16b, v30.16b
        and     x7, x12, #0x3ffffff
        usra    v22.2d, v18.2d, #25
        add     x10, x30, x8, lsr #26
        umlal   v7.2d, v19.2s, v24.2s
        bfi     x5, x29, #32, #25               // ubignum_of_preglist 1 // INTERMEDIATE bb
        and     v9.16b, v25.16b, v23.16b
        add     x27, x7, x10, lsr #25
        usra    v22.2d, v18.2d, #24
        mov     x21, #60833
        lsl     x21, x21, #1
        add     x15, x17, x27, lsr #26
        shl     v25.2s, v3.2s, #1
        umlal   v7.2d, v14.2s, v17.2s
        and     x29, x27, #0x3ffffff
        usra    v22.2d, v18.2d, #21
        bfi     x29, x15, #32, #26              // ubignum_of_preglist 4 // INTERMEDIATE bb // ***SOURCE*** ubignum_of_xreglist 9 // INTERMEDIATE bbalt
        umlal   v10.2d, v14.2s, v24.2s
        and     x17, x6, #0x1ffffff             // ubignum_of_xreglist 1 // INTERMEDIATE bbalt
        umlal   v10.2d, v2.2s, v28.2s
        sub     x6, x3, x5
        umlal   v10.2d, v12.2s, v17.2s
        umaddl  x25, w16, w21, x17
        umlal   v10.2d, v29.2s, v4.2s
        mov     w12, w5                         // ubignum_of_xreglist 2 // INTERMEDIATE bbalt
        umlal   v22.2d, v20.2s, v4.2s
        lsr     x26, x6, #32                    // ubignum_of_wreglist 3 + ubignum_of_wreglist 2 // INTERMEDIATE e
        umlal   v22.2d, v14.2s, v8.2s
        and     x24, x0, #0x1ffffff             // ubignum_of_xreglist 5 // INTERMEDIATE bbalt
        umlal   v22.2d, v2.2s, v24.2s
        stp     x11, x5, [tmpb+0]
        umlal   v22.2d, v12.2s, v5.2s
        bfi     x20, x0, #32, #25               // ubignum_of_preglist 2 // INTERMEDIATE bb
        umlal   v22.2d, v29.2s, v17.2s
        umaddl  x12, w6, w21, x12
        umull   v18.2d, v1.2s, v4.2s
        bfi     x19, x10, #32, #25              // ubignum_of_preglist 3 // INTERMEDIATE bb
        umlal   v7.2d, v2.2s, v4.2s
        sub     x7, x14, x20
        umlal   v27.2d, v19.2s, v13.2s
        mov     w8, w20                         // ubignum_of_xreglist 4 // INTERMEDIATE bbalt
        usra    v10.2d, v22.2d, #26
        lsr     x14, x7, #32                    // ubignum_of_wreglist 5 + ubignum_of_wreglist 4 // INTERMEDIATE e
        umlal   v18.2d, v19.2s, v17.2s
        and     x28, x10, #0x1ffffff            // ubignum_of_xreglist 7 // INTERMEDIATE bbalt
        umlal   v7.2d, v12.2s, v13.2s
        sub     x5, x2, x19
        usra    v11.2d, v10.2d, #25
        mov     w2, w19                         // ubignum_of_xreglist 6 // INTERMEDIATE bbalt
        umlal   v27.2d, v0.2s, v4.2s
        umlal   v21.2d, v14.2s, v25.2s
        sub     x23, x4, x29
        usra    v7.2d, v11.2d, #26
        mov     w0, w29                         // ubignum_of_xreglist 8 // INTERMEDIATE bbalt
        umlal   v18.2d, v0.2s, v28.2s
        lsr     x22, x23, #32                   // ubignum_of_wreglist 9 + ubignum_of_wreglist 8 // INTERMEDIATE e
        umlal   v27.2d, v15.2s, v17.2s
        str     x29, [tmpb+32]
        usra    v6.2d, v7.2d, #25
        mov     w17, w11                        // ubignum_of_xreglist 0 // INTERMEDIATE bbalt
        and     v0.16b, v22.16b, v30.16b        // ubignum_of_hreglist 0 + ubignum_of_lreglist 0 // INTERMEDIATE H|L = x5|t3
        umaddl  x27, w26, w21, x1
        umlal   v18.2d, v14.2s, v13.2s
        umaddl  x30, w23, w21, x0
        umlal   v18.2d, v2.2s, v3.2s
        lsr     x10, x5, #32                    // ubignum_of_wreglist 7 + ubignum_of_wreglist 6 // INTERMEDIATE e
        and     v4.16b, v6.16b, v30.16b         // ubignum_of_hreglist 4 + ubignum_of_lreglist 4 // INTERMEDIATE H|L = x5|t3
        and     v1.16b, v10.16b, v23.16b        // ubignum_of_hreglist 1 + ubignum_of_lreglist 1 // INTERMEDIATE H|L = x5|t3
        umaddl  x4, w14, w21, x24
        ldr     x0, [tmpa+0]
        mov     v0.s[1], w0
        lsr     x0, x0, #32
        mov     v1.s[1], w0
        umaddl  x9, w7, w21, x8
        usra    v18.2d, v6.2d, #26
        umaddl  x24, w10, w21, x28
        and     v3.16b, v7.16b, v23.16b         // ubignum_of_hreglist 3 + ubignum_of_lreglist 3 // INTERMEDIATE H|L = x5|t3
        umaddl  x8, w22, w21, x15
        umlal   v27.2d, v14.2s, v26.2s
        umaddl  x15, w13, w21, x17
        usra    v21.2d, v18.2d, #25
        stp     x20, x19, [tmpb+16]
        and     v2.16b, v11.16b, v30.16b        // ubignum_of_hreglist 2 + ubignum_of_lreglist 2 // INTERMEDIATE H|L = x5|t3
        lsr     x29, x8, #25
        ldr     x3, [tmpb+0]
        mov     v10.s[1], w3
        lsr     x3, x3, #32
        mov     v11.s[1], w3
        add     x17, x15, x29
        usra    v27.2d, v21.2d, #26
        add     x28, x17, x29, lsl #1
        and     v6.16b, v21.16b, v30.16b        // ubignum_of_hreglist 6 + ubignum_of_lreglist 6 // INTERMEDIATE H|L = x5|t3
        and     x20, x8, #0x1ffffff
        and     v5.16b, v18.16b, v23.16b        // ubignum_of_hreglist 5 + ubignum_of_lreglist 5 // INTERMEDIATE H|L = x5|t3
        add     x17, x28, x29, lsl #4
        and     v7.16b, v27.16b, v23.16b        // ubignum_of_hreglist 7 + ubignum_of_lreglist 7 // INTERMEDIATE H|L = x5|t3
        ldr     x3, [tmpb+8]
        mov     v22.s[1], w3
        lsr     x3, x3, #32
        mov     v23.s[1], w3
        add     x29, x25, x17, lsr #26
        ldr     x15, [pointx+0]
        mov     v10.s[0], w15
        lsr     x15, x15, #32
        mov     v11.s[0], w15
        and     x11, x17, #0x3ffffff            // ubignum_of_xreglist 0 // INTERMEDIATE bce
        usra    v16.2d, v27.2d, #25
        add     x8, x12, x29, lsr #25
        ldr     x3, [tmpb+16]
        mov     v14.s[1], w3
        lsr     x3, x3, #32
        mov     v15.s[1], w3
        and     x12, x29, #0x1ffffff            // ubignum_of_xreglist 1 // INTERMEDIATE bce
        ldr     x15, [pointx+8]
        mov     v22.s[0], w15
        lsr     x15, x15, #32
        mov     v23.s[0], w15
        add     x28, x27, x8, lsr #26
        and     v8.16b, v16.16b, v30.16b        // ubignum_of_hreglist 8 + ubignum_of_lreglist 8 // INTERMEDIATE H|L = x5|t3
        umull   x1, w12, w10
        ldr     x3, [tmpb+24]
        mov     v17.s[1], w3
        lsr     x3, x3, #32
        mov     v18.s[1], w3
        add     x25, x9, x28, lsr #25
        ldr     x15, [pointx+16]
        mov     v14.s[0], w15
        lsr     x15, x15, #32
        mov     v15.s[0], w15
        umaddl  x19, w5, w21, x2
        usra    v9.2d, v16.2d, #26              // ubignum_of_hreglist 8 + ubignum_of_lreglist 8 // INTERMEDIATE H|L = x5|t3
        add     x2, x4, x25, lsr #26
        ldr     x3, [tmpb+32]
        mov     v24.s[1], w3
        lsr     x3, x3, #32
        mov     v25.s[1], w3
        umull   x3, w12, w23
        ldr     x15, [pointx+24]
        mov     v17.s[0], w15
        lsr     x15, x15, #32
        mov     v18.s[0], w15
        add     x29, x19, x2, lsr #25
        umull   v26.2d, v0.2s, v23.2s
        and     x21, x28, #0x1ffffff            // ubignum_of_xreglist 3 // INTERMEDIATE bce
        ldr     x0, [tmpa+8]
        mov     v2.s[1], w0
        lsr     x0, x0, #32
        mov     v3.s[1], w0
        umaddl  x27, w21, w5, x3
        ldr     x15, [pointx+32]
        mov     v24.s[0], w15
        lsr     x15, x15, #32
        mov     v25.s[0], w15
        add     x17, x24, x29, lsr #26
        umull   v29.2d, v1.2s, v18.2s
        and     x15, x8, #0x3ffffff             // ubignum_of_xreglist 2 // INTERMEDIATE bce
        umull   v20.2d, v0.2s, v15.2s
        add     x19, x30, x17, lsr #25
        and     x3, x17, #0x1ffffff             // ubignum_of_xreglist 7 // INTERMEDIATE bce
        mul     v12.2s, v25.2s, v31.2s
        ldr     x0, [tmpa+16]
        mov     v4.s[1], w0
        lsr     x0, x0, #32
        mov     v5.s[1], w0
        add     x4, x20, x19, lsr #26           // ubignum_of_xreglist 9 // INTERMEDIATE bce
        umlal   v26.2d, v2.2s, v11.2s
        add     w28, w3, w3, lsl #1;
        umlal   v20.2d, v2.2s, v23.2s
        add     w28, w28, w3, lsl #4
        umull   x8, w12, w5
        ldr     x0, [tmpa+24]
        mov     v6.s[1], w0
        lsr     x0, x0, #32
        mov     v7.s[1], w0
        and     x30, x25, #0x3ffffff            // ubignum_of_xreglist 4 // INTERMEDIATE bce
        mul     v16.2s, v18.2s, v31.2s
        add     w17, w4, w4, lsl #1;
        umull   v21.2d, v1.2s, v15.2s
        add     w17, w17, w4, lsl #4
        umaddl  x25, w21, w7, x8
        umlal   v20.2d, v4.2s, v11.2s
        add     w8, w21, w21, lsl #1;
        ldr     x0, [tmpa+32]
        add     w8, w8, w21, lsl #4
        mov     v8.s[1], w0
        lsr     x0, x0, #32
        mov     v9.s[1], w0
        and     x2, x2, #0x1ffffff              // ubignum_of_xreglist 5 // INTERMEDIATE bce
        umlal   v29.2d, v3.2s, v15.2s
        umaddl  x24, w2, w6, x25
        umull   v13.2d, v0.2s, v25.2s
        umaddl  x25, w2, w7, x27
        umaddl  x0, w3, w6, x25
        mul     v19.2s, v15.2s, v31.2s
        umull   v27.2d, v0.2s, v18.2s
        umaddl  x20, w3, w13, x24
        umlal   v20.2d, v6.2s, v12.2s
        umaddl  x24, w21, w14, x1
        umlal   v13.2d, v2.2s, v18.2s
        umaddl  x9, w4, w13, x0
        umull   v25.2d, v0.2s, v11.2s
        umaddl  x20, w17, w23, x20
        umlal   v27.2d, v2.2s, v15.2s
        umaddl  x0, w2, w26, x24
        umull   v28.2d, v1.2s, v11.2s
        umull   x24, w17, w5
        umlal   v29.2d, v5.2s, v23.2s
        umaddl  x9, w11, w22, x9
        umlal   v13.2d, v4.2s, v15.2s
        umaddl  x27, w3, w16, x0
        umlal   v27.2d, v4.2s, v23.2s
        umull   x0, w17, w14
        umlal   v27.2d, v6.2s, v11.2s
        umull   x4, w12, w14
        umlal   v27.2d, v8.2s, v12.2s
        umaddl  x25, w11, w10, x20
        umlal   v27.2d, v1.2s, v17.2s
        umaddl  x0, w28, w10, x0
        umlal   v13.2d, v6.2s, v23.2s
        umull   x3, w17, w6
        umlal   v13.2d, v8.2s, v11.2s
        umaddl  x1, w21, w26, x4
        umlal   v20.2d, v8.2s, v16.2s
        umaddl  x4, w2, w13, x24
        umlal   v28.2d, v3.2s, v12.2s
        umaddl  x20, w28, w7, x3
        umlal   v29.2d, v7.2s, v11.2s
        and     x3, x19, #0x3ffffff             // ubignum_of_xreglist 9 // INTERMEDIATE bce
        umlal   v29.2d, v9.2s, v12.2s
        umaddl  x19, w17, w22, x27
        add     w27, w2, w2, lsl #1;
        mul     v18.2s, v24.2s, v31.2s
        add     w27, w27, w2, lsl #4
        umlal   v21.2d, v3.2s, v23.2s
        umull   x24, w17, w7
        umlal   v13.2d, v1.2s, v24.2s
        add     x19, x19, x19
        shl     v29.2d, v29.2d, #1
        umaddl  x1, w2, w16, x1
        umull   v15.2d, v1.2s, v23.2s
        umaddl  x0, w27, w22, x0
        umlal   v29.2d, v0.2s, v24.2s
        umaddl  x2, w28, w5, x24
        mul     v24.2s, v23.2s, v31.2s
        umaddl  x4, w28, w23, x4
        umlal   v21.2d, v5.2s, v11.2s
        umaddl  x24, w27, w5, x20
        umlal   v20.2d, v1.2s, v14.2s
        umaddl  x20, w11, w23, x19
        umlal   v26.2d, v4.2s, v12.2s
        umaddl  x19, w27, w23, x2
        umlal   v26.2d, v6.2s, v16.2s
        umaddl  x2, w21, w6, x4
        umlal   v29.2d, v2.2s, v17.2s
        umaddl  x24, w8, w23, x24
        umlal   v15.2d, v3.2s, v11.2s
        umaddl  x0, w21, w16, x0
        umaddl  x4, w21, w13, x19
        mul     v23.2s, v11.2s, v31.2s
        umlal   v20.2d, v3.2s, v22.2s
        umaddl  x2, w12, w7, x2
        umlal   v20.2d, v5.2s, v10.2s
        umaddl  x19, w12, w26, x0
        umlal   v29.2d, v4.2s, v14.2s
        umaddl  x0, w12, w13, x24
        umlal   v26.2d, v8.2s, v19.2s
        umaddl  x20, w15, w5, x20
        umlal   v26.2d, v1.2s, v22.2s
        umaddl  x21, w15, w10, x9
        umlal   v26.2d, v3.2s, v10.2s
        and     x9, x29, #0x3ffffff             // ubignum_of_xreglist 6 // INTERMEDIATE bce
        umlal   v29.2d, v6.2s, v22.2s
        umaddl  x20, w30, w7, x20
        umaddl  x1, w28, w22, x1
        add     x24, x19, x19
        umull   v11.2d, v1.2s, v12.2s
        add     w19, w3, w3, lsl #1;
        umlal   v26.2d, v5.2s, v18.2s
        add     w19, w19, w3, lsl #4
        umaddl  x20, w9, w6, x20
        umlal   v29.2d, v8.2s, v10.2s
        add     w29, w9, w9, lsl #1;
        umlal   v13.2d, v3.2s, v17.2s
        add     w29, w29, w9, lsl #4
        umaddl  x2, w19, w10, x2
        umlal   v11.2d, v3.2s, v16.2s
        umaddl  x21, w30, w14, x21
        umlal   v11.2d, v5.2s, v19.2s
        umaddl  x20, w3, w13, x20
        umlal   v11.2d, v7.2s, v24.2s
        umaddl  x2, w29, w22, x2
        umlal   v11.2d, v9.2s, v23.2s
        umaddl  x21, w9, w26, x21
        ushr    v23.2d, v30.2d, #1
        umaddl  x1, w17, w10, x1
        umlal   v13.2d, v5.2s, v14.2s
        umaddl  x24, w19, w5, x24
        umlal   v27.2d, v3.2s, v14.2s
        umaddl  x21, w3, w16, x21
        shl     v11.2d, v11.2d, #1
        add     w3, w30, w30, lsl #1;
        umlal   v28.2d, v5.2s, v16.2s
        add     w3, w3, w30, lsl #4
        umaddl  x24, w29, w23, x24
        umlal   v28.2d, v7.2s, v19.2s
        add     x1, x1, x1
        umlal   v28.2d, v9.2s, v24.2s
        umaddl  x1, w11, w5, x1
        umlal   v15.2d, v5.2s, v12.2s
        umaddl  x24, w30, w13, x24
        umlal   v15.2d, v7.2s, v16.2s
        umaddl  x25, w15, w14, x25
        umlal   v15.2d, v9.2s, v19.2s
        umaddl  x1, w15, w7, x1
        shl     v28.2d, v28.2d, #1
        umaddl  x24, w15, w6, x24
        umlal   v21.2d, v7.2s, v12.2s
        umaddl  x2, w30, w16, x2
        umlal   v21.2d, v9.2s, v16.2s
        umaddl  x25, w30, w26, x25
        shl     v15.2d, v15.2d, #1
        umaddl  x30, w30, w6, x1
        umlal   v28.2d, v0.2s, v22.2s
        umaddl  x1, w15, w26, x2
        umlal   v28.2d, v2.2s, v10.2s
        umaddl  x2, w9, w16, x25
        shl     v21.2d, v21.2d, #1
        umaddl  x24, w11, w7, x24
        umlal   v15.2d, v0.2s, v14.2s
        umaddl  x1, w11, w14, x1
        umlal   v21.2d, v0.2s, v17.2s
        umaddl  x25, w9, w13, x30
        umlal   v28.2d, v4.2s, v18.2s
        umaddl  x0, w19, w26, x0
        umlal   v25.2d, v2.2s, v12.2s
        add     x1, x1, x24, lsr #26
        umlal   v25.2d, v4.2s, v16.2s
        umaddl  x30, w19, w22, x2
        umlal   v21.2d, v2.2s, v14.2s
        umaddl  x4, w12, w6, x4
        mul     v14.2s, v14.2s, v31.2s
        umaddl  x25, w19, w23, x25
        and     x2, x1, #0x1ffffff
        mul     v16.2s, v17.2s, v31.2s
        umlal   v25.2d, v6.2s, v19.2s
        umaddl  x9, w19, w14, x4
        umlal   v13.2d, v7.2s, v22.2s
        add     x25, x25, x1, lsr #25
        umlal   v21.2d, v4.2s, v22.2s
        umaddl  x0, w29, w14, x0
        umlal   v26.2d, v7.2s, v16.2s
        add     x30, x30, x25, lsr #26
        umlal   v26.2d, v9.2s, v14.2s
        add     w1, w15, w15, lsl #1;
        umlal   v28.2d, v6.2s, v16.2s
        add     w1, w1, w15, lsl #4
        add     x4, x20, x30, lsr #25
        umlal   v28.2d, v8.2s, v14.2s
        and     x25, x25, #0x3ffffff
        umlal   v15.2d, v2.2s, v22.2s
        add     x21, x21, x4, lsr #26
        umlal   v11.2d, v0.2s, v10.2s
        bfi     x25, x30, #32, #25              // ubignum_of_preglist 3 // INTERMEDIATE z4
        umlal   v11.2d, v2.2s, v18.2s
        bic     x30, x21, #0x3ffffff
        usra    v26.2d, v28.2d, #26
        lsr     x20, x30, #26
        umlal   v15.2d, v4.2s, v10.2s
        add     x20, x20, x30, lsr #25
        umlal   v15.2d, v6.2s, v18.2s
        umaddl  x9, w29, w10, x9
        umlal   v15.2d, v8.2s, v16.2s
        add     x30, x20, x30, lsr #22
        umlal   v27.2d, v5.2s, v22.2s
        umull   x20, w17, w26
        umlal   v20.2d, v7.2s, v18.2s
        umaddl  x30, w17, w16, x30
        umlal   v20.2d, v9.2s, v16.2s
        umaddl  x17, w3, w10, x0
        usra    v15.2d, v26.2d, #25
        umaddl  x0, w28, w14, x20
        umlal   v27.2d, v7.2s, v10.2s
        umaddl  x20, w28, w26, x30
        umlal   v27.2d, v9.2s, v18.2s
        add     w28, w12, w12, lsl #1;
        usra    v20.2d, v15.2d, #26
        add     w28, w28, w12, lsl #4
        umaddl  x30, w27, w10, x0
        and     v17.16b, v15.16b, v30.16b       // ubignum_of_hreglist 4 + ubignum_of_lreglist 4 // INTERMEDIATE H|L = x4|z5
        umaddl  x27, w27, w14, x20
        umaddl  x0, w8, w10, x27
        mul     v12.2s, v22.2s, v31.2s
        and     v15.16b, v20.16b, v23.16b       // ubignum_of_hreglist 5 + ubignum_of_lreglist 5 // INTERMEDIATE H|L = x4|z5
        umaddl  x14, w3, w22, x9
        umlal   v21.2d, v6.2s, v10.2s
        umaddl  x27, w8, w22, x30
        trn1    v15.4s, v17.4s, v15.4s          // FINAL z3
        umaddl  x10, w28, w22, x0
        umlal   v11.2d, v4.2s, v16.2s
        umaddl  x30, w15, w16, x14
        and     v26.16b, v26.16b, v23.16b
        umaddl  x28, w12, w16, x27
        umlal   v21.2d, v8.2s, v18.2s
        add     x10, x10, x10
        umlal   v25.2d, v8.2s, v24.2s
        umaddl  x20, w19, w6, x10
        umlal   v25.2d, v1.2s, v10.2s
        add     x28, x28, x28
        umlal   v25.2d, v3.2s, v18.2s
        umaddl  x28, w19, w7, x28
        usra    v21.2d, v20.2d, #25
        umaddl  x0, w29, w7, x20
        umlal   v11.2d, v6.2s, v14.2s
        umaddl  x10, w11, w26, x30
        umlal   v13.2d, v9.2s, v10.2s
        umaddl  x19, w29, w5, x28
        usra    v27.2d, v21.2d, #26
        umaddl  x0, w3, w5, x0
        umlal   v25.2d, v5.2s, v16.2s
        umaddl  x20, w1, w22, x17
        and     v20.16b, v28.16b, v30.16b
        umaddl  x29, w3, w23, x19
        usra    v29.2d, v27.2d, #25
        umaddl  x3, w1, w23, x0
        and     v27.16b, v27.16b, v23.16b       // ubignum_of_hreglist 7 + ubignum_of_lreglist 7 // INTERMEDIATE H|L = x4|z5
        umlal   v11.2d, v8.2s, v12.2s
        umaddl  x12, w15, w13, x29
        usra    v13.2d, v29.2d, #26
        umaddl  x7, w11, w13, x3
        trn1    v6.4s, v6.4s, v7.4s
        umaddl  x17, w11, w16, x20
        umlal   v25.2d, v7.2s, v14.2s
        and     x23, x4, #0x3ffffff
        bic     v19.16b, v13.16b, v23.16b
        umaddl  x19, w11, w6, x12
        and     v28.16b, v13.16b, v23.16b       // ubignum_of_hreglist 9 + ubignum_of_lreglist 9 // INTERMEDIATE H|L = x4|z5
        add     x3, x17, x7, lsr #26
        usra    v11.2d, v19.2d, #25
        trn1    v2.4s, v2.4s, v3.4s
        add     x17, x19, x3, lsr #25
        and     v13.16b, v21.16b, v30.16b       // ubignum_of_hreglist 6 + ubignum_of_lreglist 6 // INTERMEDIATE H|L = x4|z5
        and     x5, x7, #0x3ffffff
        usra    v11.2d, v19.2d, #24
        add     x7, x10, x17, lsr #26
        trn1    v0.4s, v0.4s, v1.4s
        and     x19, x24, #0x3ffffff
        and     v21.16b, v29.16b, v30.16b       // ubignum_of_hreglist 8 + ubignum_of_lreglist 8 // INTERMEDIATE H|L = x4|z5
        add     x29, x19, x7, lsr #25
        usra    v11.2d, v19.2d, #21
        bfi     x5, x3, #32, #25                // ubignum_of_preglist 0 // INTERMEDIATE z4
        trn1    v17.4s, v13.4s, v27.4s          // FINAL z3
        add     x19, x2, x29, lsr #26
        trn1    v19.4s, v21.4s, v28.4s          // FINAL z3
        and     x3, x29, #0x3ffffff
        mov     v16.d[0], v6.d[1]               // FINAL x3
        mov     v6.d[0], v17.d[1]               // FINAL x2
        trn1    v8.4s, v8.4s, v9.4s
        bfi     x3, x19, #32, #26               // ubignum_of_preglist 2 // INTERMEDIATE z4
        and     v21.16b, v11.16b, v30.16b       // ubignum_of_hreglist 0 + ubignum_of_lreglist 0 // INTERMEDIATE H|L = x4|z5
        bfi     x23, x21, #32, #26              // ubignum_of_preglist 4 // INTERMEDIATE z4
        mov     v18.d[0], v8.d[1]               // FINAL x3
        mov     v8.d[0], v19.d[1]               // FINAL x2
        umlal   v25.2d, v9.2s, v12.2s
        mov     v9.d[0], x23                    // FINAL z2
        mov     v7.d[0], x25                    // FINAL z2
        ldr     d29, [mask1]
        mov     v12.d[0], v2.d[1]               // FINAL x3
        trn1    v4.4s, v4.4s, v5.4s
        and     x17, x17, #0x3ffffff
        usra    v25.2d, v11.2d, #26
        mov     v10.d[0], v0.d[1]               // FINAL x3
        mov     v14.d[0], v4.d[1]               // FINAL x3
        mov     v4.d[0], v15.d[1]               // FINAL x2
        usra    v20.2d, v25.2d, #25
        and     v27.16b, v25.16b, v23.16b       // ubignum_of_hreglist 1 + ubignum_of_lreglist 1 // INTERMEDIATE H|L = x4|z5
        bfi     x17, x7, #32, #25               // ubignum_of_preglist 1 // INTERMEDIATE z4
        mov     v5.d[0], x3
        mov     v1.d[0], x5                     // FINAL z2
        usra    v26.2d, v20.2d, #26             // ubignum_of_hreglist 3 + ubignum_of_lreglist 3 // INTERMEDIATE H|L = x4|z5
        and     v28.16b, v20.16b, v30.16b       // ubignum_of_hreglist 2 + ubignum_of_lreglist 2 // INTERMEDIATE H|L = x4|z5
        trn1    v11.4s, v21.4s, v27.4s          // FINAL z3
        trn1    v13.4s, v28.4s, v26.4s          // FINAL z3
        mov     v0.d[0], v11.d[1]               // FINAL x2
        mov     v3.d[0], x17                    // FINAL z2
        mov     v2.d[0], v13.d[1]               // FINAL x2
        ldr     d28, [mask2]

        ldr     x0, [i]
        subs    x0, x0, #1
        str     x0, [i]
        bcs     Lcurve25519_x25519_byte_scalarloop

// Repack X2 into the saturated representation as 256-bit value xn.
// This does not fully normalize mod 2^255-19 but stays within 256 bits.

        mov     w0, v0.s[0]
        mov     w1, v0.s[1]
        mov     w2, v2.s[0]
        mov     w3, v2.s[1]
        mov     w4, v4.s[0]
        mov     w5, v4.s[1]
        mov     w6, v6.s[0]
        mov     w7, v6.s[1]
        mov     w8, v8.s[0]
        mov     w9, v8.s[1]

        add     x0, x0, x1, lsl #26
        add     x1, x2, x3, lsl #26
        add     x2, x4, x5, lsl #26
        add     x3, x6, x7, lsl #26
        add     x4, x8, x9, lsl #26

        adds    x0, x0, x1, lsl #51
        lsr     x6, x1, #13
        lsl     x7, x2, #38
        adcs    x1, x6, x7
        lsr     x8, x2, #26
        lsl     x9, x3, #25
        adcs    x2, x8, x9
        lsr     x10, x3, #39
        lsl     x11, x4, #12
        adc     x3, x10, x11
        stp     x0, x1, [xn]
        stp     x2, x3, [xn+16]

// Repack Z2 into the saturated representation as 256-bit value zn.
// This does not fully normalize mod 2^255-19. However since Z2,
// unlike X2, was not repacked (within the last multiplication) in
// right-to-left order, its top digit can be any 26-bit value, on
// the face of it. To make sure we don't overflow 256 bits here
// we remove b = 25th bit of the 9th digit (now scaled by 2^230
// giving bit 25 a final weighting of 2^255) and add 19 * b to
// to the bottom of the sum here to compensate mod 2^255-19.

        mov     w0, v1.s[0]
        mov     w1, v1.s[1]
        mov     w2, v3.s[0]
        mov     w3, v3.s[1]
        mov     w4, v5.s[0]
        mov     w5, v5.s[1]
        mov     w6, v7.s[0]
        mov     w7, v7.s[1]
        mov     w8, v9.s[0]
        mov     w9, v9.s[1]

        mov     w10, #19
        add     x0, x0, x1, lsl #26
        tst     x9, #0x2000000
        add     x1, x2, x3, lsl #26
        csel    x10, x10, xzr, ne
        add     x2, x4, x5, lsl #26
        and     x9, x9, #0x1FFFFFF
        add     x3, x6, x7, lsl #26
        add     x0, x0, x10
        add     x4, x8, x9, lsl #26

        adds    x0, x0, x1, lsl #51
        lsr     x6, x1, #13
        lsl     x7, x2, #38
        adcs    x1, x6, x7
        lsr     x8, x2, #26
        lsl     x9, x3, #25
        adcs    x2, x8, x9
        lsr     x10, x3, #39
        lsl     x11, x4, #12
        adc     x3, x10, x11
        stp     x0, x1, [zn]
        stp     x2, x3, [zn+16]

// Because the lowest bit (indeed, the three lowest bits) of the scalar
// were forced to zero, we know that the projective result of the scalar
// multiplication was in (X2,Z2) and is now (xn,zn) in saturated form.
// Prepare to call the modular inverse function to get zn' = 1/zn.

        add     x0, zn
        add     x1, zn

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "arm/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 128 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, xn, and zn.

        mov     x20, x0
        mov     x10, #0xffffffffffffffed
        mov     x11, #0xffffffffffffffff
        stp     x10, x11, [sp]
        mov     x12, #0x7fffffffffffffff
        stp     x11, x12, [sp, #16]
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        mov     x7, #0x13
        lsr     x6, x5, #63
        madd    x6, x7, x6, x7
        adds    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        orr     x5, x5, #0x8000000000000000
        adcs    x5, x5, xzr
        csel    x6, x7, xzr, cc
        subs    x2, x2, x6
        sbcs    x3, x3, xzr
        sbcs    x4, x4, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        stp     x2, x3, [sp, #32]
        stp     x4, x5, [sp, #48]
        stp     xzr, xzr, [sp, #64]
        stp     xzr, xzr, [sp, #80]
        mov     x10, #0x2099
        movk    x10, #0x7502, lsl #16
        movk    x10, #0x9e23, lsl #32
        movk    x10, #0xa0f9, lsl #48
        mov     x11, #0x2595
        movk    x11, #0x1d13, lsl #16
        movk    x11, #0x8f3f, lsl #32
        movk    x11, #0xa8c6, lsl #48
        mov     x12, #0x5242
        movk    x12, #0x5ac, lsl #16
        movk    x12, #0x8938, lsl #32
        movk    x12, #0x6c6c, lsl #48
        mov     x13, #0x615
        movk    x13, #0x4177, lsl #16
        movk    x13, #0x8b2, lsl #32
        movk    x13, #0x2765, lsl #48
        stp     x10, x11, [sp, #96]
        stp     x12, x13, [sp, #112]
        mov     x21, #0xa
        mov     x22, #0x1
        b       Lcurve25519_x25519_byte_invmidloop
Lcurve25519_x25519_byte_invloop:
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        and     x0, x12, x16
        and     x1, x13, x17
        add     x19, x0, x1
        ldr     x7, [sp]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #32]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x7, [sp, #8]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #40]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [sp]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [sp, #32]
        ldr     x7, [sp, #16]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #48]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [sp, #8]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [sp, #40]
        ldr     x7, [sp, #24]
        eor     x1, x7, x14
        asr     x3, x1, #63
        and     x3, x3, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #56]
        eor     x1, x8, x15
        asr     x0, x1, #63
        and     x0, x0, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [sp, #16]
        extr    x5, x3, x5, #59
        str     x5, [sp, #24]
        eor     x1, x7, x16
        asr     x5, x1, #63
        and     x5, x5, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        asr     x0, x1, #63
        and     x0, x0, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [sp, #48]
        extr    x2, x5, x2, #59
        str     x2, [sp, #56]
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        str     x5, [sp, #96]
        adc     x3, x3, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        str     x3, [sp, #104]
        adc     x4, x4, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        str     x4, [sp, #112]
        adc     x2, x2, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        add     x6, x6, x3, asr #63
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x3, x6, x3
        ldr     x6, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x3
        asr     x3, x3, #63
        adcs    x6, x6, x3
        adc     x5, x5, x3
        stp     x0, x1, [sp, #64]
        stp     x6, x5, [sp, #80]
        eor     x1, x7, x16
        and     x5, x16, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        and     x0, x17, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x6, x5, x2, #63
        ldp     x0, x1, [sp, #96]
        add     x6, x6, x5, asr #63
        mov     x5, #0x13
        mul     x4, x6, x5
        add     x2, x2, x6, lsl #63
        smulh   x5, x6, x5
        ldr     x3, [sp, #112]
        adds    x0, x0, x4
        adcs    x1, x1, x5
        asr     x5, x5, #63
        adcs    x3, x3, x5
        adc     x2, x2, x5
        stp     x0, x1, [sp, #96]
        stp     x3, x2, [sp, #112]
Lcurve25519_x25519_byte_invmidloop:
        mov     x1, x22
        ldr     x2, [sp]
        ldr     x3, [sp, #32]
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x8, x4, #0x100, lsl #12
        sbfx    x8, x8, #21, #21
        mov     x11, #0x100000
        add     x11, x11, x11, lsl #21
        add     x9, x4, x11
        asr     x9, x9, #42
        add     x10, x5, #0x100, lsl #12
        sbfx    x10, x10, #21, #21
        add     x11, x5, x11
        asr     x11, x11, #42
        mul     x6, x8, x2
        mul     x7, x9, x3
        mul     x2, x10, x2
        mul     x3, x11, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #21, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #42
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #21, #21
        add     x15, x5, x15
        asr     x15, x15, #42
        mul     x6, x12, x2
        mul     x7, x13, x3
        mul     x2, x14, x2
        mul     x3, x15, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        mul     x2, x12, x8
        mul     x3, x12, x9
        mul     x6, x14, x8
        mul     x7, x14, x9
        madd    x8, x13, x10, x2
        madd    x9, x13, x11, x3
        madd    x16, x15, x10, x6
        madd    x17, x15, x11, x7
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #22, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #43
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #22, #21
        add     x15, x5, x15
        asr     x15, x15, #43
        mneg    x2, x12, x8
        mneg    x3, x12, x9
        mneg    x4, x14, x8
        mneg    x5, x14, x9
        msub    x10, x13, x16, x2
        msub    x11, x13, x17, x3
        msub    x12, x15, x16, x4
        msub    x13, x15, x17, x5
        mov     x22, x1
        subs    x21, x21, #0x1
        b.ne    Lcurve25519_x25519_byte_invloop
        ldr     x0, [sp]
        ldr     x1, [sp, #32]
        mul     x0, x0, x10
        madd    x1, x1, x11, x0
        asr     x0, x1, #63
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        eor     x14, x14, x0
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        eor     x15, x15, x0
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        eor     x16, x16, x0
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        eor     x17, x17, x0
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        tst     x3, x3
        cinc    x6, x6, pl
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x6, x6, x3
        ldr     x2, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x6
        asr     x6, x6, #63
        adcs    x2, x2, x6
        adcs    x5, x5, x6
        csel    x3, x3, xzr, mi
        subs    x0, x0, x3
        sbcs    x1, x1, xzr
        sbcs    x2, x2, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        mov     x4, x20
        stp     x0, x1, [x4]
        stp     x2, x5, [x4, #16]

// Now the result is xn * (1/zn), fully reduced modulo p.
// Note that in the degenerate case zn = 0 (mod p_25519), the
// modular inverse code above will produce 1/zn = 0, giving
// the correct overall X25519 result of zero for the point at
// infinity. The multiplication below is just an inlined
// version of bignum_mul_p25519 except for the detailed
// addressing of inputs and outputs

        ldp     x3, x4, [xn]
        ldp     x5, x6, [zn]
        umull   x7, w3, w5
        lsr     x0, x3, #32
        umull   x15, w0, w5
        lsr     x16, x5, #32
        umull   x8, w16, w0
        umull   x16, w3, w16
        adds    x7, x7, x15, lsl #32
        lsr     x15, x15, #32
        adc     x8, x8, x15
        adds    x7, x7, x16, lsl #32
        lsr     x16, x16, #32
        adc     x8, x8, x16
        mul     x9, x4, x6
        umulh   x10, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, cc
        csetm   x16, cc
        adds    x9, x9, x8
        adc     x10, x10, xzr
        subs    x3, x5, x6
        cneg    x3, x3, cc
        cinv    x16, x16, cc
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x8, x7, x9
        adcs    x9, x9, x10
        adc     x10, x10, xzr
        cmn     x16, #0x1
        eor     x15, x15, x16
        adcs    x8, x15, x8
        eor     x3, x3, x16
        adcs    x9, x3, x9
        adc     x10, x10, x16
        ldp     x3, x4, [xn+16]
        ldp     x5, x6, [zn+16]
        umull   x11, w3, w5
        lsr     x0, x3, #32
        umull   x15, w0, w5
        lsr     x16, x5, #32
        umull   x12, w16, w0
        umull   x16, w3, w16
        adds    x11, x11, x15, lsl #32
        lsr     x15, x15, #32
        adc     x12, x12, x15
        adds    x11, x11, x16, lsl #32
        lsr     x16, x16, #32
        adc     x12, x12, x16
        mul     x13, x4, x6
        umulh   x14, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, cc
        csetm   x16, cc
        adds    x13, x13, x12
        adc     x14, x14, xzr
        subs    x3, x5, x6
        cneg    x3, x3, cc
        cinv    x16, x16, cc
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x12, x11, x13
        adcs    x13, x13, x14
        adc     x14, x14, xzr
        cmn     x16, #0x1
        eor     x15, x15, x16
        adcs    x12, x15, x12
        eor     x3, x3, x16
        adcs    x13, x3, x13
        adc     x14, x14, x16
        ldp     x3, x4, [xn+16]
        ldp     x15, x16, [xn]
        subs    x3, x3, x15
        sbcs    x4, x4, x16
        csetm   x16, cc
        ldp     x15, x0, [zn]
        subs    x5, x15, x5
        sbcs    x6, x0, x6
        csetm   x0, cc
        eor     x3, x3, x16
        subs    x3, x3, x16
        eor     x4, x4, x16
        sbc     x4, x4, x16
        eor     x5, x5, x0
        subs    x5, x5, x0
        eor     x6, x6, x0
        sbc     x6, x6, x0
        eor     x16, x0, x16
        adds    x11, x11, x9
        adcs    x12, x12, x10
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        mul     x2, x3, x5
        umulh   x0, x3, x5
        mul     x15, x4, x6
        umulh   x1, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, cc
        csetm   x9, cc
        adds    x15, x15, x0
        adc     x1, x1, xzr
        subs    x6, x5, x6
        cneg    x6, x6, cc
        cinv    x9, x9, cc
        mul     x5, x4, x6
        umulh   x6, x4, x6
        adds    x0, x2, x15
        adcs    x15, x15, x1
        adc     x1, x1, xzr
        cmn     x9, #0x1
        eor     x5, x5, x9
        adcs    x0, x5, x0
        eor     x6, x6, x9
        adcs    x15, x6, x15
        adc     x1, x1, x9
        adds    x9, x11, x7
        adcs    x10, x12, x8
        adcs    x11, x13, x11
        adcs    x12, x14, x12
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        cmn     x16, #0x1
        eor     x2, x2, x16
        adcs    x9, x2, x9
        eor     x0, x0, x16
        adcs    x10, x0, x10
        eor     x15, x15, x16
        adcs    x11, x15, x11
        eor     x1, x1, x16
        adcs    x12, x1, x12
        adcs    x13, x13, x16
        adc     x14, x14, x16
        mov     x3, #0x26
        umull   x4, w11, w3
        add     x4, x4, w7, uxtw
        lsr     x7, x7, #32
        lsr     x11, x11, #32
        umaddl  x11, w11, w3, x7
        mov     x7, x4
        umull   x4, w12, w3
        add     x4, x4, w8, uxtw
        lsr     x8, x8, #32
        lsr     x12, x12, #32
        umaddl  x12, w12, w3, x8
        mov     x8, x4
        umull   x4, w13, w3
        add     x4, x4, w9, uxtw
        lsr     x9, x9, #32
        lsr     x13, x13, #32
        umaddl  x13, w13, w3, x9
        mov     x9, x4
        umull   x4, w14, w3
        add     x4, x4, w10, uxtw
        lsr     x10, x10, #32
        lsr     x14, x14, #32
        umaddl  x14, w14, w3, x10
        mov     x10, x4
        lsr     x0, x14, #31
        mov     x5, #0x13
        umaddl  x5, w5, w0, x5
        add     x7, x7, x5
        adds    x7, x7, x11, lsl #32
        extr    x3, x12, x11, #32
        adcs    x8, x8, x3
        extr    x3, x13, x12, #32
        adcs    x9, x9, x3
        extr    x3, x14, x13, #32
        lsl     x5, x0, #63
        eor     x10, x10, x5
        adc     x10, x10, x3
        mov     x3, #0x13
        tst     x10, #0x8000000000000000
        csel    x3, x3, xzr, pl
        subs    x7, x7, x3
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        and     x10, x10, #0x7fffffffffffffff
        stp     x7, x8, [zn]
        stp     x9, x10, [zn+16]

// Now copy bytewise to the output

        ldr     x17, [res]

        ldp     x10, x11, [zn]
        strb    w10, [x17]
        lsr     x10, x10, #8
        strb    w10, [x17, #1]
        lsr     x10, x10, #8
        strb    w10, [x17, #2]
        lsr     x10, x10, #8
        strb    w10, [x17, #3]
        lsr     x10, x10, #8
        strb    w10, [x17, #4]
        lsr     x10, x10, #8
        strb    w10, [x17, #5]
        lsr     x10, x10, #8
        strb    w10, [x17, #6]
        lsr     x10, x10, #8
        strb    w10, [x17, #7]

        strb    w11, [x17, #8]
        lsr     x11, x11, #8
        strb    w11, [x17, #9]
        lsr     x11, x11, #8
        strb    w11, [x17, #10]
        lsr     x11, x11, #8
        strb    w11, [x17, #11]
        lsr     x11, x11, #8
        strb    w11, [x17, #12]
        lsr     x11, x11, #8
        strb    w11, [x17, #13]
        lsr     x11, x11, #8
        strb    w11, [x17, #14]
        lsr     x11, x11, #8
        strb    w11, [x17, #15]

        ldp     x12, x13, [zn+16]
        strb    w12, [x17, #16]
        lsr     x12, x12, #8
        strb    w12, [x17, #17]
        lsr     x12, x12, #8
        strb    w12, [x17, #18]
        lsr     x12, x12, #8
        strb    w12, [x17, #19]
        lsr     x12, x12, #8
        strb    w12, [x17, #20]
        lsr     x12, x12, #8
        strb    w12, [x17, #21]
        lsr     x12, x12, #8
        strb    w12, [x17, #22]
        lsr     x12, x12, #8
        strb    w12, [x17, #23]

        strb    w13, [x17, #24]
        lsr     x13, x13, #8
        strb    w13, [x17, #25]
        lsr     x13, x13, #8
        strb    w13, [x17, #26]
        lsr     x13, x13, #8
        strb    w13, [x17, #27]
        lsr     x13, x13, #8
        strb    w13, [x17, #28]
        lsr     x13, x13, #8
        strb    w13, [x17, #29]
        lsr     x13, x13, #8
        strb    w13, [x17, #30]
        lsr     x13, x13, #8
        strb    w13, [x17, #31]

// Restore stack and registers (this will zero the tops of Q8...Q15).

        CFI_STACKLOAD2(d8,d9,224)
        CFI_STACKLOAD2(d10,d11,240)
        CFI_STACKLOAD2(d12,d13,256)
        CFI_STACKLOAD2(d14,d15,272)
        CFI_STACKLOAD2(x19,x20,288)
        CFI_STACKLOAD2(x21,x22,304)
        CFI_STACKLOAD2(x23,x24,320)
        CFI_STACKLOAD2(x25,x26,336)
        CFI_STACKLOAD2(x27,x28,352)
        CFI_STACKLOAD2(x29,x30,368)
        CFI_INC_SP(384)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(curve25519_x25519_byte)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
