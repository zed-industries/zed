// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[6], y[6]; output z[12]
//
//    extern void bignum_mul_6_12(uint64_t z[static 12], const uint64_t x[static 6],
//                                const uint64_t y[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_6_12)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_6_12)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_6_12)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Macro computing [c,b,a] := [b,a] + (x - y) * (w - z), adding with carry
// to the [b,a] components but leaving CF aligned with the c term, which is
// a sign bitmask for (x - y) * (w - z). Continued add-with-carry operations
// with [c,...,c] will continue the carry chain correctly starting from
// the c position if desired to add to a longer term of the form [...,b,a].
//
// c,h,l,t should all be different and t,h should not overlap w,z.
// ---------------------------------------------------------------------------

.macro muldiffnadd b,a, c,h,l,t, x,y, w,z
        subs    \t, \x, \y
        cneg    \t, \t, cc
        csetm   \c, cc
        subs    \h, \w, \z
        cneg    \h, \h, cc
        mul     \l, \t, \h
        umulh   \h, \t, \h
        cinv    \c, \c, cc
        adds    xzr, \c, #1
        eor     \l, \l, \c
        adcs    \a, \a, \l
        eor     \h, \h, \c
        adcs    \b, \b, \h
.endm

#define z x0
#define x x1
#define y x2

#define a0 x3
#define a1 x4
#define a2 x5
#define b0 x6
#define b1 x7
#define b2 x8
#define l0 x9
#define l1 x10
#define l2 x11
#define h0 x12
#define h1 x13
#define h2 x14

#define s1 x15
#define s2 x16
#define s3 x17
#define s4 x19
#define s5 x9

#define c  x10
#define h  x11
#define l  x12
#define t  x13

#define s0 x20

#define u0      x3
#define u1      x4
#define u2      x5
#define u3      x6
#define u4      x7
#define u5      x8

// These alias c,h,l but it doesn't matter

#define u6      x10
#define u7      x11
#define u8      x12

// We recycle the input pointers near the end

#define s x1
#define d x2

// ---------------------------------------------------------------------------
// Core 3x3->6 ADK multiplication macro
// Does [s5,s4,s3,s2,s1,s0] = [a2,a1,a0] * [b2,b1,b0]
//
// If the input parameter is 1, it also adds in [z+24,z+32,z+40]
// existing contents; if the parameter is 0 it just does the pure multiply
// ---------------------------------------------------------------------------

.macro  mul3 afl
        mul     s0, a0, b0
        mul     l1, a1, b1
        mul     l2, a2, b2
        umulh   h0, a0, b0
        umulh   h1, a1, b1
        umulh   h2, a2, b2

        adds    h0, h0, l1
        adcs    h1, h1, l2
        adc     h2, h2, xzr

        adds    s1, h0, s0
        adcs    s2, h1, h0
        adcs    s3, h2, h1
        adc     s4, h2, xzr

        adds    s2, s2, s0
        adcs    s3, s3, h0
        adcs    s4, s4, h1
        adc     s5, h2, xzr

// Optionally add the existing z contents

.rep \afl
        ldr     l, [z,#24]
        adds    s0, s0, l
        ldp     l, h, [z,#32]
        adcs    s1, s1, l
        adcs    s2, s2, h
        adcs    s3, s3, xzr
        adcs    s4, s4, xzr
        adc     s5, s5, xzr
.endr

        muldiffnadd s2,s1, c,h,l, t, a0,a1, b1,b0
        adcs    s3, s3, c
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffnadd s3,s2, c,h,l, t, a0,a2, b2,b0
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffnadd s4,s3, c,h,l, t, a1,a2, b2,b1
        adc     s5, s5, c
.endm

S2N_BN_SYMBOL(bignum_mul_6_12):
        CFI_START

CFI_PUSH2(x19,x20)

// Multiply the low halves using ADK 3x3->6

        ldp     a0, a1, [x1]
        ldp     b0, b1, [x2]
        ldr     a2, [x1, #16]
        ldr     b2, [x2, #16]

        mul3    0
        stp     s0, s1, [x0]
        stp     s2, s3, [x0, #16]
        stp     s4, s5, [x0, #32]

// Multiply the high halves using ADK 3x3->6

        ldp     a0, a1, [x1,#24]
        ldp     b0, b1, [x2,#24]
        ldr     a2, [x1, #40]
        ldr     b2, [x2, #40]

        mul3    1

        stp     s0, s1, [x0, #48]
        stp     s2, s3, [x0, #64]
        stp     s4, s5, [x0, #80]

// Compute t,[a2,a1,a0] = x_hi - x_lo
// and     s,[b2,b1,b0] = y_lo - y_hi
// sign-magnitude differences

        ldr     t, [x1]
        subs    a0, a0, t
        ldr     t, [x1,#8]
        sbcs    a1, a1, t
        ldr     t, [x1,#16]
        sbcs    a2, a2, t
        csetm   t, cc

        ldr     s, [x2]
        subs    b0, s, b0
        ldr     s, [x2,#8]
        sbcs    b1, s, b1
        ldr     s, [x2,#16]
        sbcs    b2, s, b2
        csetm   s, cc

        eor     a0, a0, t
        subs    a0, a0, t
        eor     a1, a1, t
        sbcs    a1, a1, t
        eor     a2, a2, t
        sbc     a2, a2, t

        eor     b0, b0, s
        subs    b0, b0, s
        eor     b1, b1, s
        sbcs    b1, b1, s
        eor     b2, b2, s
        sbc     b2, b2, s

// Save the correct sign for the sub-product

        eor     s, s, t

// Now yet another 3x3->6 ADK core, but not writing back, keeping s0..s5

        mul3    0

// Now accumulate the positive mid-terms as [u5,u4,u3,u2,u1,u0]

        ldp     u0, u1, [z]
        ldp     u3, u4, [z,#48]
        adds    u0, u0, u3
        adcs    u1, u1, u4
        ldr     u2, [z,#16]
        ldp     u5, u6, [z,#64]
        adcs    u2, u2, u5
        adcs    u3, u3, u6
        ldp     u7, u8, [z,#80]
        adcs    u4, u4, u7
        adcs    u5, u5, u8

// Stop the carry here so we can reintroduce it, taking into account the
// effective addition of s from sign-extension below. Note that we get
// a duplicated word c+carry beyond the first one, so this upper part is
// of the form [d,d,t].

        adcs    t, s, xzr
        adc     d, s, xzr

// Add in the sign-adjusted complex term

        adds    xzr, s, #1
        eor     s0, s0, s
        adcs    u0, s0, u0
        eor     s1, s1, s
        adcs    u1, s1, u1
        eor     s2, s2, s
        adcs    u2, s2, u2
        eor     s3, s3, s
        adcs    u3, s3, u3
        eor     s4, s4, s
        adcs    u4, s4, u4
        eor     s5, s5, s
        adcs    u5, s5, u5
        adcs    u6, u6, t
        adcs    u7, u7, d
        adc     u8, u8, d

// Store it back

        str     u0, [x0,#24]
        stp     u1, u2, [x0,#32]
        stp     u3, u4, [x0,#48]
        stp     u5, u6, [x0,#64]
        stp     u7, u8, [x0,#80]

// Restore regs and return

        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_6_12)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
