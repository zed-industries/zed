// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for GM/T 0003-2012 curve SM2
// Input scalar[4], point[12]; output res[12]
//
// extern void sm2_montjscalarmul
//   (uint64_t res[static 12],
//    const uint64_t scalar[static 4],
//    const uint64_t point[static 12]);
//
// This function is a variant of its affine point version sm2_scalarmul.
// Here, input and output points are assumed to be in Jacobian form with
// their coordinates in the Montgomery domain. Thus, if priming indicates
// Montgomery form, x' = (2^256 * x) mod p_sm2 etc., each point argument
// is a triple (x',y',z') representing the affine point (x/z^2,y/z^3) when
// z' is nonzero or the point at infinity (group identity) if z' = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve SM2, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_sm2) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sm2_montjscalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sm2_montjscalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sm2_montjscalarmul)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Safe copies of inputs (res lasts the whole code, point not so long)
// and additional values in variables, with some aliasing

#define res x19
#define sgn x20
#define j x20
#define point x21

// Intermediate variables on the stack.

#define scalarb sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define tabent sp, #(4*NUMSIZE)

#define tab sp, #(7*NUMSIZE)

#define NSPACE 31*NUMSIZE

// Avoid using .rep for the sake of the BoringSSL/AWS-LC delocator,
// which doesn't accept repetitions, assembler macros etc.

#define selectblock(I)                          \
        cmp     x14, #(1*I) __LF                   \
        ldp     x12, x13, [x15] __LF               \
        csel    x0, x12, x0, eq __LF               \
        csel    x1, x13, x1, eq __LF               \
        ldp     x12, x13, [x15, #16] __LF          \
        csel    x2, x12, x2, eq __LF               \
        csel    x3, x13, x3, eq __LF               \
        ldp     x12, x13, [x15, #32] __LF          \
        csel    x4, x12, x4, eq __LF               \
        csel    x5, x13, x5, eq __LF               \
        ldp     x12, x13, [x15, #48] __LF          \
        csel    x6, x12, x6, eq __LF               \
        csel    x7, x13, x7, eq __LF               \
        ldp     x12, x13, [x15, #64] __LF          \
        csel    x8, x12, x8, eq __LF               \
        csel    x9, x13, x9, eq __LF               \
        ldp     x12, x13, [x15, #80] __LF          \
        csel    x10, x12, x10, eq __LF             \
        csel    x11, x13, x11, eq __LF             \
        add     x15, x15, #96

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(sm2_montjscalarmul):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x30)
        CFI_DEC_SP(NSPACE)

// Preserve the "res" and "point" input arguments. We load and process the
// scalar immediately so we don't bother preserving that input argument.
// Also, "point" is only needed early on and so its register gets re-used.

        mov     res, x0
        mov     point, x2

// Load the digits of group order n_sm2 = [x12;x13;x14;x15]

        movbig(x12, #0x53bb, #0xf409, #0x39d5, #0x4123)
        movbig(x13, #0x7203, #0xdf6b, #0x21c6, #0x052b)
        mov     x14, #0xffffffffffffffff
        mov     x15, #0xfffffffeffffffff

// First, reduce the input scalar mod n_sm2, i.e. conditionally subtract n_sm2

        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]

        subs    x6, x2, x12
        sbcs    x7, x3, x13
        sbcs    x8, x4, x14
        sbcs    x9, x5, x15

        csel    x2, x2, x6, cc
        csel    x3, x3, x7, cc
        csel    x4, x4, x8, cc
        csel    x5, x5, x9, cc

// Now if the top bit of the reduced scalar is set, negate it mod n_sm2,
// i.e. do n |-> n_sm2 - n. Remember the sign as "sgn" so we can
// correspondingly negate the point below.

        subs    x6, x12, x2
        sbcs    x7, x13, x3
        sbcs    x8, x14, x4
        sbc     x9, x15, x5

        tst     x5, #0x8000000000000000
        csel    x2, x2, x6, eq
        csel    x3, x3, x7, eq
        csel    x4, x4, x8, eq
        csel    x5, x5, x9, eq
        cset    sgn, ne

// In either case then add the recoding constant 0x08888...888 to allow
// signed digits.

        mov     x6, 0x8888888888888888
        adds    x2, x2, x6
        adcs    x3, x3, x6
        bic     x7, x6, #0xF000000000000000
        adcs    x4, x4, x6
        adc     x5, x5, x7

        stp     x2, x3, [scalarb]
        stp     x4, x5, [scalarb+16]

// Set the tab[0] table entry to the input point = 1 * P, except
// that we negate it if the top bit of the scalar was set. This
// negation takes care over the y = 0 case to maintain all the
// coordinates < p_sm2 throughout, even though triples (x,y,z)
// with y = 0 can only represent a point on the curve when z = 0
// and it represents the point at infinity regardless of x and y.

        ldp     x0, x1, [point]
        stp     x0, x1, [tab]
        ldp     x2, x3, [point, #16]
        stp     x2, x3, [tab+16]

        ldp     x4, x5, [point, #32]
        ldp     x6, x7, [point, #48]

        mov     x0, #0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, #0xffffffff00000000
        sbcs    x1, x1, x5
        mov     x2, #0xffffffffffffffff
        sbcs    x2, x2, x6
        mov     x3, #0xfffffffeffffffff
        sbc     x3, x3, x7

        orr     x8, x4, x5
        orr     x9, x6, x7
        orr     x8, x8, x9
        cmp     x8, xzr
        ccmp    sgn, xzr, #4, ne
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tab+32]
        stp     x6, x7, [tab+48]

        ldp     x0, x1, [point, #64]
        stp     x0, x1, [tab+64]
        ldp     x2, x3, [point, #80]
        stp     x2, x3, [tab+80]

// Compute and record tab[1] = 2 * p, ..., tab[7] = 8 * P

        add     x0, tab+96*1
        add     x1, tab
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, tab+96*2
        add     x1, tab+96*1
        add     x2, tab
        CFI_BL(Lsm2_montjscalarmul_sm2_montjadd)

        add     x0, tab+96*3
        add     x1, tab+96*1
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, tab+96*4
        add     x1, tab+96*3
        add     x2, tab
        CFI_BL(Lsm2_montjscalarmul_sm2_montjadd)

        add     x0, tab+96*5
        add     x1, tab+96*2
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, tab+96*6
        add     x1, tab+96*5
        add     x2, tab
        CFI_BL(Lsm2_montjscalarmul_sm2_montjadd)

        add     x0, tab+96*7
        add     x1, tab+96*3
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

// Initialize the accumulator as a table entry for top 4 bits (unrecoded)

        ldr     x14, [scalarb+24]
        lsr     x14, x14, #60

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        add     x15, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

        stp     x0, x1, [acc]
        stp     x2, x3, [acc+16]
        stp     x4, x5, [acc+32]
        stp     x6, x7, [acc+48]
        stp     x8, x9, [acc+64]
        stp     x10, x11, [acc+80]

        mov     j, #252

// Main loop over size-4 bitfields: double 4 times then add signed digit

Lsm2_montjscalarmul_mainloop:
        sub     j, j, #4

        add     x0, acc
        add     x1, acc
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lsm2_montjscalarmul_sm2_montjdouble)

        lsr     x2, j, #6
        ldr     x14, [sp, x2, lsl #3]   // Exploits scalarb = sp exactly
        lsr     x14, x14, j
        and     x14, x14, #15

        subs    x14, x14, #8
        cset    x16, lo                 // x16 = sign of digit (1 = negative)
        cneg    x14, x14, lo            // x14 = absolute value of digit

// Conditionally select the table entry tab[i-1] = i * P in constant time

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        add     x15, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

// Store it to "tabent" with the y coordinate optionally negated
// Again, do it carefully to give coordinates < p_sm2 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]

        mov     x0, #0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, #0xffffffff00000000
        sbcs    x1, x1, x5
        mov     x2, #0xffffffffffffffff
        sbcs    x2, x2, x6
        mov     x3, #0xfffffffeffffffff
        sbc     x3, x3, x7

        orr     x12, x4, x5
        orr     x13, x6, x7
        orr     x12, x12, x13
        cmp     x12, xzr
        ccmp    x16, xzr, #4, ne
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]
        stp     x8, x9, [tabent+64]
        stp     x10, x11, [tabent+80]

        add     x0, acc
        add     x1, acc
        add     x2, tabent
        CFI_BL(Lsm2_montjscalarmul_sm2_montjadd)

        cbnz    j, Lsm2_montjscalarmul_mainloop

// That's the end of the main loop, and we just need to copy the
// result in "acc" to the output.

        ldp     x0, x1, [acc]
        stp     x0, x1, [res]
        ldp     x0, x1, [acc+16]
        stp     x0, x1, [res, #16]
        ldp     x0, x1, [acc+32]
        stp     x0, x1, [res, #32]
        ldp     x0, x1, [acc+48]
        stp     x0, x1, [res, #48]
        ldp     x0, x1, [acc+64]
        stp     x0, x1, [res, #64]
        ldp     x0, x1, [acc+80]
        stp     x0, x1, [res, #80]

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sm2_montjscalarmul)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lsm2_montjscalarmul_sm2_montjadd)

Lsm2_montjscalarmul_sm2_montjadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_DEC_SP(224)
        mov     x17, x0
        mov     x19, x1
        mov     x20, x2
        ldp     x2, x3, [x19, #0x40]
        ldp     x4, x5, [x19, #0x50]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x0, x0, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x0, x0, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #0x10]
        ldp     x2, x3, [x20, #0x40]
        ldp     x4, x5, [x20, #0x50]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x0, x0, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x0, x0, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp, #0xa0]
        stp     x10, x6, [sp, #0xb0]
        ldp     x3, x4, [x20, #0x40]
        ldp     x5, x6, [x20, #0x50]
        ldp     x7, x8, [x19, #0x20]
        ldp     x9, x10, [x19, #0x30]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0xc0]
        stp     x11, x12, [sp, #0xd0]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0xc0]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0xd0]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0xc0]
        stp     x3, x4, [sp, #0xd0]
        ldp     x3, x4, [x19, #0x40]
        ldp     x5, x6, [x19, #0x50]
        ldp     x7, x8, [x20, #0x20]
        ldp     x9, x10, [x20, #0x30]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x20]
        stp     x11, x12, [sp, #0x30]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x20]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x30]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x20]
        stp     x3, x4, [sp, #0x30]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [sp, #0x10]
        ldp     x7, x8, [x20]
        ldp     x9, x10, [x20, #0x10]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x40]
        stp     x11, x12, [sp, #0x50]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x40]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x50]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x40]
        stp     x3, x4, [sp, #0x50]
        ldp     x3, x4, [sp, #0xa0]
        ldp     x5, x6, [sp, #0xb0]
        ldp     x7, x8, [x19]
        ldp     x9, x10, [x19, #0x10]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x80]
        stp     x11, x12, [sp, #0x90]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x80]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x90]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x80]
        stp     x3, x4, [sp, #0x90]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [sp, #0x10]
        ldp     x7, x8, [sp, #0x20]
        ldp     x9, x10, [sp, #0x30]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x20]
        stp     x11, x12, [sp, #0x30]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x20]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x30]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x20]
        stp     x3, x4, [sp, #0x30]
        ldp     x3, x4, [sp, #0xa0]
        ldp     x5, x6, [sp, #0xb0]
        ldp     x7, x8, [sp, #0xc0]
        ldp     x9, x10, [sp, #0xd0]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0xc0]
        stp     x11, x12, [sp, #0xd0]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0xc0]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0xd0]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0xc0]
        stp     x3, x4, [sp, #0xd0]
        ldp     x5, x6, [sp, #0x40]
        ldp     x4, x3, [sp, #0x80]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x50]
        ldp     x4, x3, [sp, #0x90]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0xa0]
        stp     x7, x8, [sp, #0xb0]
        ldp     x5, x6, [sp, #0x20]
        ldp     x4, x3, [sp, #0xc0]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x30]
        ldp     x4, x3, [sp, #0xd0]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x20]
        stp     x7, x8, [sp, #0x30]
        ldp     x2, x3, [sp, #0xa0]
        ldp     x4, x5, [sp, #0xb0]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x0, x0, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x0, x0, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp, #0x60]
        stp     x10, x6, [sp, #0x70]
        ldp     x2, x3, [sp, #0x20]
        ldp     x4, x5, [sp, #0x30]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x0, x0, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x0, x0, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #0x10]
        ldp     x3, x4, [sp, #0x60]
        ldp     x5, x6, [sp, #0x70]
        ldp     x7, x8, [sp, #0x80]
        ldp     x9, x10, [sp, #0x90]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x80]
        stp     x11, x12, [sp, #0x90]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x80]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x90]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x80]
        stp     x3, x4, [sp, #0x90]
        ldp     x3, x4, [sp, #0x60]
        ldp     x5, x6, [sp, #0x70]
        ldp     x7, x8, [sp, #0x40]
        ldp     x9, x10, [sp, #0x50]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x40]
        stp     x11, x12, [sp, #0x50]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x40]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x50]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x40]
        stp     x3, x4, [sp, #0x50]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #0x80]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x10]
        ldp     x4, x3, [sp, #0x90]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #0x10]
        ldp     x5, x6, [sp, #0x40]
        ldp     x4, x3, [sp, #0x80]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x50]
        ldp     x4, x3, [sp, #0x90]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x60]
        stp     x7, x8, [sp, #0x70]
        ldp     x3, x4, [sp, #0xa0]
        ldp     x5, x6, [sp, #0xb0]
        ldp     x7, x8, [x19, #0x40]
        ldp     x9, x10, [x19, #0x50]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0xa0]
        stp     x11, x12, [sp, #0xb0]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0xa0]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0xb0]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0xa0]
        stp     x3, x4, [sp, #0xb0]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #0x40]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x10]
        ldp     x4, x3, [sp, #0x50]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #0x10]
        ldp     x5, x6, [sp, #0x80]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x90]
        ldp     x4, x3, [sp, #0x10]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x80]
        stp     x7, x8, [sp, #0x90]
        ldp     x3, x4, [sp, #0x60]
        ldp     x5, x6, [sp, #0x70]
        ldp     x7, x8, [sp, #0xc0]
        ldp     x9, x10, [sp, #0xd0]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x60]
        stp     x11, x12, [sp, #0x70]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x60]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x70]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x60]
        stp     x3, x4, [sp, #0x70]
        ldp     x3, x4, [sp, #0xa0]
        ldp     x5, x6, [sp, #0xb0]
        ldp     x7, x8, [x20, #0x40]
        ldp     x9, x10, [x20, #0x50]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0xa0]
        stp     x11, x12, [sp, #0xb0]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0xa0]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0xb0]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0xa0]
        stp     x3, x4, [sp, #0xb0]
        ldp     x3, x4, [sp, #0x20]
        ldp     x5, x6, [sp, #0x30]
        ldp     x7, x8, [sp, #0x80]
        ldp     x9, x10, [sp, #0x90]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x80]
        stp     x11, x12, [sp, #0x90]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x0, x10, x9
        cneg    x0, x0, lo
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x80]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x90]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x0, x8, x7
        cneg    x0, x0, lo
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x0, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x0
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x0, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x0
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x80]
        stp     x3, x4, [sp, #0x90]
        ldp     x5, x6, [sp, #0x80]
        ldp     x4, x3, [sp, #0x60]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x90]
        ldp     x4, x3, [sp, #0x70]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x80]
        stp     x7, x8, [sp, #0x90]
        ldp     x0, x1, [x19, #0x40]
        ldp     x2, x3, [x19, #0x50]
        orr     x12, x0, x1
        orr     x13, x2, x3
        orr     x12, x12, x13
        cmp     x12, xzr
        cset    x12, ne
        ldp     x4, x5, [x20, #0x40]
        ldp     x6, x7, [x20, #0x50]
        orr     x13, x4, x5
        orr     x14, x6, x7
        orr     x13, x13, x14
        cmp     x13, xzr
        cset    x13, ne
        cmp     x13, x12
        ldp     x8, x9, [sp, #0xa0]
        csel    x8, x0, x8, lo
        csel    x9, x1, x9, lo
        csel    x8, x4, x8, hi
        csel    x9, x5, x9, hi
        ldp     x10, x11, [sp, #0xb0]
        csel    x10, x2, x10, lo
        csel    x11, x3, x11, lo
        csel    x10, x6, x10, hi
        csel    x11, x7, x11, hi
        ldp     x12, x13, [x19]
        ldp     x0, x1, [sp]
        csel    x0, x12, x0, lo
        csel    x1, x13, x1, lo
        ldp     x12, x13, [x20]
        csel    x0, x12, x0, hi
        csel    x1, x13, x1, hi
        ldp     x12, x13, [x19, #0x10]
        ldp     x2, x3, [sp, #0x10]
        csel    x2, x12, x2, lo
        csel    x3, x13, x3, lo
        ldp     x12, x13, [x20, #0x10]
        csel    x2, x12, x2, hi
        csel    x3, x13, x3, hi
        ldp     x12, x13, [x19, #0x20]
        ldp     x4, x5, [sp, #0x80]
        csel    x4, x12, x4, lo
        csel    x5, x13, x5, lo
        ldp     x12, x13, [x20, #0x20]
        csel    x4, x12, x4, hi
        csel    x5, x13, x5, hi
        ldp     x12, x13, [x19, #0x30]
        ldp     x6, x7, [sp, #0x90]
        csel    x6, x12, x6, lo
        csel    x7, x13, x7, lo
        ldp     x12, x13, [x20, #0x30]
        csel    x6, x12, x6, hi
        csel    x7, x13, x7, hi
        stp     x0, x1, [x17]
        stp     x2, x3, [x17, #0x10]
        stp     x4, x5, [x17, #0x20]
        stp     x6, x7, [x17, #0x30]
        stp     x8, x9, [x17, #0x40]
        stp     x10, x11, [x17, #0x50]
        CFI_INC_SP(224)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lsm2_montjscalarmul_sm2_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lsm2_montjscalarmul_sm2_montjdouble)

Lsm2_montjscalarmul_sm2_montjdouble:
        CFI_START
        CFI_DEC_SP(208)
        stp     x19, x20, [sp, #0xc0]
        mov     x19, x0
        mov     x20, x1
        ldp     x2, x3, [x20, #0x40]
        ldp     x4, x5, [x20, #0x50]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x17, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x17, x17, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x17, x17, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x17, x17, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x17, x17, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x17
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #0x10]
        ldp     x2, x3, [x20, #0x20]
        ldp     x4, x5, [x20, #0x30]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x17, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x17, x17, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x17, x17, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x17, x17, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x17, x17, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x17
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp, #0x20]
        stp     x10, x6, [sp, #0x30]
        ldp     x5, x6, [x20]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x20, #0x10]
        ldp     x4, x3, [sp, #0x10]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x60]
        stp     x7, x8, [sp, #0x70]
        ldp     x4, x5, [x20]
        ldp     x8, x9, [sp]
        adds    x4, x4, x8
        adcs    x5, x5, x9
        ldp     x6, x7, [x20, #0x10]
        ldp     x10, x11, [sp, #0x10]
        adcs    x6, x6, x10
        adcs    x7, x7, x11
        csetm   x2, hs
        subs    x4, x4, x2
        and     x3, x2, #0xffffffff00000000
        sbcs    x5, x5, x3
        and     x1, x2, #0xfffffffeffffffff
        sbcs    x6, x6, x2
        sbc     x7, x7, x1
        stp     x4, x5, [sp, #0x40]
        stp     x6, x7, [sp, #0x50]
        ldp     x3, x4, [sp, #0x40]
        ldp     x5, x6, [sp, #0x50]
        ldp     x7, x8, [sp, #0x60]
        ldp     x9, x10, [sp, #0x70]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x60]
        stp     x11, x12, [sp, #0x70]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x10, x9
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x60]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x70]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x3, x17
        umulh   x17, x3, x17
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x17, x17, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x60]
        stp     x3, x4, [sp, #0x70]
        ldp     x4, x5, [x20, #0x20]
        ldp     x8, x9, [x20, #0x40]
        adds    x4, x4, x8
        adcs    x5, x5, x9
        ldp     x6, x7, [x20, #0x30]
        ldp     x10, x11, [x20, #0x50]
        adcs    x6, x6, x10
        adcs    x7, x7, x11
        adc     x3, xzr, xzr
        adds    x8, x4, #0x1
        mov     x9, #-0x100000000
        sbcs    x9, x5, x9
        adcs    x10, x6, xzr
        mov     x11, #-0x100000001
        sbcs    x11, x7, x11
        sbcs    x3, x3, xzr
        csel    x4, x4, x8, lo
        csel    x5, x5, x9, lo
        csel    x6, x6, x10, lo
        csel    x7, x7, x11, lo
        stp     x4, x5, [sp, #0x40]
        stp     x6, x7, [sp, #0x50]
        ldp     x3, x4, [x20]
        ldp     x5, x6, [x20, #0x10]
        ldp     x7, x8, [sp, #0x20]
        ldp     x9, x10, [sp, #0x30]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x80]
        stp     x11, x12, [sp, #0x90]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x10, x9
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x80]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x90]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x3, x17
        umulh   x17, x3, x17
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x17, x17, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x80]
        stp     x3, x4, [sp, #0x90]
        ldp     x2, x3, [sp, #0x60]
        ldp     x4, x5, [sp, #0x70]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x17, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x17, x17, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x17, x17, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x17, x17, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x17, x17, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x17
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp, #0xa0]
        stp     x10, x6, [sp, #0xb0]
        ldp     x2, x3, [sp, #0x40]
        ldp     x4, x5, [sp, #0x50]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x17, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x17, x17, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x17, x17, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x17, x17, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x17, x17, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x17
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp, #0x40]
        stp     x10, x6, [sp, #0x50]
        mov     x1, #0x9
        mov     x2, #-0x1
        ldp     x9, x10, [sp, #0xa0]
        subs    x9, x2, x9
        mov     x3, #-0x100000000
        sbcs    x10, x3, x10
        ldp     x11, x12, [sp, #0xb0]
        sbcs    x11, x2, x11
        mov     x4, #-0x100000001
        sbc     x12, x4, x12
        mul     x3, x1, x9
        mul     x4, x1, x10
        mul     x5, x1, x11
        mul     x6, x1, x12
        umulh   x9, x1, x9
        umulh   x10, x1, x10
        umulh   x11, x1, x11
        umulh   x7, x1, x12
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, xzr
        mov     x1, #0xc
        ldp     x9, x10, [sp, #0x80]
        mul     x8, x9, x1
        umulh   x9, x9, x1
        adds    x3, x3, x8
        mul     x8, x10, x1
        umulh   x10, x10, x1
        adcs    x4, x4, x8
        ldp     x11, x12, [sp, #0x90]
        mul     x8, x11, x1
        umulh   x11, x11, x1
        adcs    x5, x5, x8
        mul     x8, x12, x1
        umulh   x12, x12, x1
        adcs    x6, x6, x8
        adc     x7, x7, xzr
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, x12
        add     x7, x7, #0x1
        lsl     x8, x7, #32
        sub     x9, x8, x7
        adds    x3, x3, x7
        adcs    x4, x4, x9
        adcs    x5, x5, xzr
        adcs    x6, x6, x8
        csetm   x7, lo
        adds    x3, x3, x7
        and     x9, x7, #0xffffffff00000000
        adcs    x4, x4, x9
        adcs    x5, x5, x7
        and     x8, x7, #0xfffffffeffffffff
        adc     x6, x6, x8
        stp     x3, x4, [sp, #0xa0]
        stp     x5, x6, [sp, #0xb0]
        ldp     x5, x6, [sp, #0x40]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x50]
        ldp     x4, x3, [sp, #0x10]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [sp, #0x40]
        stp     x7, x8, [sp, #0x50]
        ldp     x2, x3, [sp, #0x20]
        ldp     x4, x5, [sp, #0x30]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x17, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x17, x17, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x17, x17, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        lsr     x11, x15, #32
        subs    x14, x12, x15
        sbc     x13, x11, xzr
        subs    x16, x16, x14
        sbcs    x17, x17, x13
        sbcs    x1, x1, x12
        sbc     x15, x15, x11
        lsl     x12, x16, #32
        lsr     x11, x16, #32
        subs    x14, x12, x16
        sbc     x13, x11, xzr
        subs    x17, x17, x14
        sbcs    x1, x1, x13
        sbcs    x15, x15, x12
        sbc     x16, x16, x11
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, lo
        csetm   x13, lo
        subs    x12, x5, x4
        cneg    x12, x12, lo
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, lo
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x17
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        lsr     x11, x6, #32
        subs    x14, x12, x6
        sbc     x13, x11, xzr
        subs    x7, x7, x14
        sbcs    x8, x8, x13
        sbcs    x9, x9, x12
        sbc     x14, x6, x11
        adds    x10, x10, x14
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        lsr     x11, x7, #32
        subs    x14, x12, x7
        sbc     x13, x11, xzr
        subs    x8, x8, x14
        sbcs    x9, x9, x13
        sbcs    x10, x10, x12
        sbc     x14, x7, x11
        adds    x6, x6, x14
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #-0x100000000
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #-0x100000001
        adcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, hs
        csel    x9, x11, x9, hs
        csel    x10, x12, x10, hs
        csel    x6, x13, x6, hs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #0x10]
        ldp     x3, x4, [sp, #0xa0]
        ldp     x5, x6, [sp, #0xb0]
        ldp     x7, x8, [sp, #0x60]
        ldp     x9, x10, [sp, #0x70]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        stp     x13, x14, [sp, #0x60]
        stp     x11, x12, [sp, #0x70]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, lo
        csetm   x1, lo
        subs    x17, x10, x9
        cneg    x17, x17, lo
        mul     x16, x15, x17
        umulh   x17, x15, x17
        cinv    x1, x1, lo
        eor     x16, x16, x1
        eor     x17, x17, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #0x60]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #0x70]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x17, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, lo
        csetm   x4, lo
        subs    x17, x8, x7
        cneg    x17, x17, lo
        mul     x16, x3, x17
        umulh   x17, x3, x17
        cinv    x4, x4, lo
        eor     x16, x16, x4
        eor     x17, x17, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x16, x11, #32
        lsr     x15, x11, #32
        subs    x1, x16, x11
        sbc     x17, x15, xzr
        subs    x12, x12, x1
        sbcs    x13, x13, x17
        sbcs    x14, x14, x16
        sbc     x11, x11, x15
        lsl     x16, x12, #32
        lsr     x15, x12, #32
        subs    x1, x16, x12
        sbc     x17, x15, xzr
        subs    x13, x13, x1
        sbcs    x14, x14, x17
        sbcs    x11, x11, x16
        sbc     x12, x12, x15
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x15, x2, #32
        sub     x16, x15, x2
        adds    x13, x13, x2
        adcs    x14, x14, x16
        adcs    x3, x3, xzr
        adcs    x4, x4, x15
        csetm   x7, lo
        adds    x13, x13, x7
        and     x16, x7, #0xffffffff00000000
        adcs    x14, x14, x16
        adcs    x3, x3, x7
        and     x15, x7, #0xfffffffeffffffff
        adc     x4, x4, x15
        stp     x13, x14, [sp, #0x60]
        stp     x3, x4, [sp, #0x70]
        ldp     x5, x6, [sp, #0x40]
        ldp     x4, x3, [sp, #0x20]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #0x50]
        ldp     x4, x3, [sp, #0x30]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, lo
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff00000000
        adcs    x6, x6, x4
        adcs    x7, x7, x3
        and     x4, x3, #0xfffffffeffffffff
        adc     x8, x8, x4
        stp     x5, x6, [x19, #0x40]
        stp     x7, x8, [x19, #0x50]
        ldp     x1, x2, [sp, #0x80]
        lsl     x0, x1, #2
        ldp     x6, x7, [sp, #0xa0]
        subs    x0, x0, x6
        extr    x1, x2, x1, #0x3e
        sbcs    x1, x1, x7
        ldp     x3, x4, [sp, #0x90]
        extr    x2, x3, x2, #0x3e
        ldp     x6, x7, [sp, #0xb0]
        sbcs    x2, x2, x6
        extr    x3, x4, x3, #0x3e
        sbcs    x3, x3, x7
        lsr     x4, x4, #62
        sbc     x4, x4, xzr
        add     x4, x4, #0x1
        lsl     x5, x4, #32
        sub     x6, x5, x4
        adds    x0, x0, x4
        adcs    x1, x1, x6
        adcs    x2, x2, xzr
        adcs    x3, x3, x5
        csetm   x4, lo
        adds    x0, x0, x4
        and     x6, x4, #0xffffffff00000000
        adcs    x1, x1, x6
        adcs    x2, x2, x4
        and     x5, x4, #0xfffffffeffffffff
        adc     x3, x3, x5
        stp     x0, x1, [x19]
        stp     x2, x3, [x19, #0x10]
        mov     x1, #0x8
        mov     x2, #-0x1
        ldp     x9, x10, [sp]
        subs    x9, x2, x9
        mov     x3, #-0x100000000
        sbcs    x10, x3, x10
        ldp     x11, x12, [sp, #0x10]
        sbcs    x11, x2, x11
        mov     x4, #-0x100000001
        sbc     x12, x4, x12
        lsl     x3, x9, #3
        extr    x4, x10, x9, #0x3d
        extr    x5, x11, x10, #0x3d
        extr    x6, x12, x11, #0x3d
        lsr     x7, x12, #61
        mov     x1, #0x3
        ldp     x9, x10, [sp, #0x60]
        mul     x8, x9, x1
        umulh   x9, x9, x1
        adds    x3, x3, x8
        mul     x8, x10, x1
        umulh   x10, x10, x1
        adcs    x4, x4, x8
        ldp     x11, x12, [sp, #0x70]
        mul     x8, x11, x1
        umulh   x11, x11, x1
        adcs    x5, x5, x8
        mul     x8, x12, x1
        umulh   x12, x12, x1
        adcs    x6, x6, x8
        adc     x7, x7, xzr
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, x12
        add     x7, x7, #0x1
        lsl     x8, x7, #32
        sub     x9, x8, x7
        adds    x3, x3, x7
        adcs    x4, x4, x9
        adcs    x5, x5, xzr
        adcs    x6, x6, x8
        csetm   x7, lo
        adds    x3, x3, x7
        and     x9, x7, #0xffffffff00000000
        adcs    x4, x4, x9
        adcs    x5, x5, x7
        and     x8, x7, #0xfffffffeffffffff
        adc     x6, x6, x8
        stp     x3, x4, [x19, #0x20]
        stp     x5, x6, [x19, #0x30]
        ldp     x19, x20, [sp, #0xc0]
        CFI_INC_SP(208)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lsm2_montjscalarmul_sm2_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
