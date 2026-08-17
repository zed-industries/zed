// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[8], y[8]; output z[16]
//
//    extern void bignum_mul_8_16_base
//     (uint64_t z[static 16], uint64_t x[static 8], uint64_t y[static 8]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_8_16_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_8_16_base)
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
#define a3 x6
#define b0 x7
#define b1 x8
#define b2 x9
#define b3 x10

#define s0 x11
#define s1 x12
#define s2 x13
#define s3 x14
#define s4 x15
#define s5 x16
#define s6 x17
#define s7 x19

#define c  x20
#define h  x21
#define l  x22
#define m  x23
#define t  x24

// These alias the ax and bx values, and are only used when they are done with

#define u0  x3
#define u1  x4
#define u2  x5
#define u3  x6
#define u4  x7
#define u5  x8
#define u6  x9
#define u7  x10

// These alias c,h,l,m but leave s, t and d safe, all we need

#define u8  x20
#define u9  x21
#define u10 x22
#define u11 x23

// We recycle the input pointers near the end

#define s  x1
#define d  x2

// ---------------------------------------------------------------------------
// Core 4x4->8 ADK multiplication macro
// Does [s7,s6,s5,s4,s3,s2,s1,s0] = [a3,a2,a1,a0] * [b3,b2,b1,b0]
//
// If the input parameter is 1, it also adds in [z+32,z+40,z+48,z+56]
// existing contents; if the parameter is 0 it just does the pure multiply
// ---------------------------------------------------------------------------

.macro  mul4 afl

// First accumulate all the "simple" products as [s7,s6,s5,s4,s0]

        mul     s0, a0, b0
        mul     s4, a1, b1
        mul     s5, a2, b2
        mul     s6, a3, b3

        umulh   s7, a0, b0
        adds    s4, s4, s7
        umulh   s7, a1, b1
        adcs    s5, s5, s7
        umulh   s7, a2, b2
        adcs    s6, s6, s7
        umulh   s7, a3, b3
        adc     s7, s7, xzr

// Multiply by B + 1 to get [s7;s6;s5;s4;s1;s0]

        adds    s1, s4, s0
        adcs    s4, s5, s4
        adcs    s5, s6, s5
        adcs    s6, s7, s6
        adc     s7, xzr, s7

// Multiply by B^2 + 1 to get [s7;s6;s5;s4;s3;s2;s1;s0]

        adds    s2, s4, s0
        adcs    s3, s5, s1
        adcs    s4, s6, s4
        adcs    s5, s7, s5
        adcs    s6, xzr, s6
        adc     s7, xzr, s7

// Optionally add the existing z contents

.rep \afl
        ldp     l, h, [z,#32]
        adds    s0, s0, l
        adcs    s1, s1, h
        ldp     l, h, [z,#48]
        adcs    s2, s2, l
        adcs    s3, s3, h
        adcs    s4, s4, xzr
        adcs    s5, s5, xzr
        adcs    s6, s6, xzr
        adc     s7, s7, xzr
.endr

// Now add in all the "complicated" terms.

        muldiffnadd s6,s5, c,h,l,t, a2,a3, b3,b2
        adc     s7, s7, c

        muldiffnadd s2,s1, c,h,l,t, a0,a1, b1,b0
        adcs    s3, s3, c
        adcs    s4, s4, c
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c

        muldiffnadd s5,s4, c,h,l,t, a1,a3, b3,b1
        adcs    s6, s6, c
        adc     s7, s7, c

        muldiffnadd s3,s2, c,h,l,t, a0,a2, b2,b0
        adcs    s4, s4, c
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c

        muldiffnadd s4,s3, c,h,l,t, a0,a3, b3,b0
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c
        muldiffnadd s4,s3, c,h,l,t, a1,a2, b2,b1
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c
.endm

// ---------------------------------------------------------------------------
// The main code
// ---------------------------------------------------------------------------

S2N_BN_SYMBOL(bignum_mul_8_16_base):

// Save registers

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x24, [sp, #-16]!

// Multiply the low halves and then the high halves using ADK 4x4->8.
// For the second one add the top of the low part (Q1) already into
// the bottom of the high part (Q2) so that is already dealt with.
//
// Write back the first one but defer the second till a bit later while
// we get on with the absolute difference computations

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]
        ldp     a2, a3, [x, #16]
        ldp     b2, b3, [y, #16]

        mul4    0

        ldp     a0, a1, [x, #32]
        stp     s0, s1, [z]
        ldp     b0, b1, [y, #32]
        stp     s2, s3, [z, #16]
        ldp     a2, a3, [x, #48]
        stp     s4, s5, [z, #32]
        ldp     b2, b3, [y, #48]
        stp     s6, s7, [z, #48]

        mul4    1

// Compute t,[a3,a2,a1,a0] = x_hi - x_lo
// and     s,[b3,b2,b1,b0] = y_lo - y_hi
// sign-magnitude differences, and scatter in belated high writeback

        ldp     l, h, [x]
        subs    a0, a0, l
        sbcs    a1, a1, h
        ldp     l, h, [x, #16]
        sbcs    a2, a2, l
        sbcs    a3, a3, h
        csetm   t, cc

        stp     s0, s1, [z, #64]

        ldp     l, h, [y]
        subs    b0, l, b0
        sbcs    b1, h, b1
        ldp     l, h, [y, #16]
        sbcs    b2, l, b2
        sbcs    b3, h, b3
        csetm   s, cc

        stp     s2, s3, [z, #80]

        eor     a0, a0, t
        subs    a0, a0, t
        eor     a1, a1, t
        sbcs    a1, a1, t
        eor     a2, a2, t
        sbcs    a2, a2, t
        eor     a3, a3, t
        sbc     a3, a3, t

        stp     s4, s5, [z, #96]

        eor     b0, b0, s
        subs    b0, b0, s
        eor     b1, b1, s
        sbcs    b1, b1, s
        eor     b2, b2, s
        sbcs    b2, b2, s
        eor     b3, b3, s
        sbc     b3, b3, s

        stp     s6, s7, [z, #112]

// Save the correct sign for the sub-product

        eor     s, s, t

// Now yet another 4x4->8 ADK core, but not writing back, keeping s0..s7

        mul4    0

// Now accumulate the positive mid-terms as [u7,u6,u5,u4,u3.u2,u1,u0]

        ldp     u0, u1, [z]
        ldp     u4, u5, [z,#64]
        adds    u0, u0, u4
        adcs    u1, u1, u5
        ldp     u2, u3, [z,#16]
        ldp     u6, u7, [z,#80]
        adcs    u2, u2, u6
        adcs    u3, u3, u7
        ldp     u8, u9, [z,#96]
        adcs    u4, u4, u8
        adcs    u5, u5, u9
        ldp     u10, u11, [z,#112]
        adcs    u6, u6, u10
        adcs    u7, u7, u11

// Stop the carry here so we can reintroduce it, taking into account the
// effective addition of s from sign-extension below. Note that we get
// a duplicated word c+carry beyond the first one, so this upper part is
// of the form [d,d,d,t].

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
        eor     s6, s6, s
        adcs    u6, s6, u6
        eor     s7, s7, s
        adcs    u7, s7, u7

// From this point on replace the sign with the suspended carry indication

        adcs    u8, u8, t
        adcs    u9, u9, d
        adcs    u10, u10, d
        adc     u11, u11, d

// Store it back

        stp     u0, u1, [z,#32]
        stp     u2, u3, [z,#48]
        stp     u4, u5, [z,#64]
        stp     u6, u7, [z,#80]
        stp     u8, u9, [z,#96]
        stp     u10, u11, [z,#112]

// Restore regs and return

        ldp     x23, x24, [sp], #16
        ldp     x21, x22, [sp], #16
        ldp     x19, x20, [sp], #16

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
