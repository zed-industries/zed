// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended Montgomery reduce in 8-digit blocks, results in input-output buffer
// Inputs z[2*k], m[k], w; outputs function return (extra result bit) and z[2*k]
//
//    extern uint64_t bignum_emontredc_8n_base
//     (uint64_t k, uint64_t *z, uint64_t *m, uint64_t w);
//
// Functionally equivalent to bignum_emontredc (see that file for more detail).
// But in general assumes that the input k is a multiple of 8.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = w, returns X0
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc_8n_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc_8n_base)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Macro computing (c,h,l) = 3-word 1s complement (x - y) * (w - z)
// and adding it with carry to (b,a) so that we have CF+c in the 2 position.
//
// c,h,l,t should all be different
// t,h should not overlap w,z
// ---------------------------------------------------------------------------

#define muldiffnadd(b,a, c,h,l,t, x,y, w,z) \
        subs    t, x, y  __LF\
        cneg    t, t, cc  __LF\
        csetm   c, cc  __LF\
        subs    h, w, z  __LF\
        cneg    h, h, cc  __LF\
        mul     l, t, h  __LF\
        umulh   h, t, h  __LF\
        cinv    c, c, cc  __LF\
        adds    xzr, c, #1  __LF\
        eor     l, l, c  __LF\
        adcs    a, a, l  __LF\
        eor     h, h, c  __LF\
        adcs    b, b, h

// The inputs, though k gets processed so we use a different name

#define k4m1 x0
#define z x1
#define m x2
#define w x3

// Additional variables

#define a0 x4
#define a1 x5
#define a2 x6
#define a3 x7

#define b0 x8
#define b1 x9
#define b2 x10
#define b3 x11

#define c0 x12
#define c1 x13
#define c2 x14
#define c3 x15
#define c4 x16

#define u0 x17
#define u1 x19
#define u2 x20
#define u3 x21
#define u4 x22
#define u5 x23
#define u6 x24
#define u7 x25

// These temp registers are aliased to c0..c3, which is safe here

#define c x12
#define h x13
#define l x14
#define t x15

// Loop counters, outer and inner

#define i x26
#define j x27

// Top carry, eventually returned when aligned with top
// It's maintained as a bitmask since this seems slightly easier(?)

#define tc x28

// -----------------------------------------------------------------------
// The basic 4x4->8 multiply-add block, which does in ADK style (10 muls):
//
// [c3;c2;c1;c0;z_3;z_2;z_1;z_0] :=
//   [a3;a2;a1;a0] * [b3;b2;b1;b0] + [c3;c2;c1;c0] + [z_3;z_2;z_1;z_0]
// -----------------------------------------------------------------------

.macro madd4
        mul     u0, a0, b0
        mul     u4, a1, b1
        mul     u5, a2, b2
        mul     u6, a3, b3

// Accumulate the simple products as [u7,u6,u5,u4,u0]

        umulh   c4, a0, b0
        adds    u4, u4, c4
        umulh   c4, a1, b1
        adcs    u5, u5, c4
        umulh   c4, a2, b2
        adcs    u6, u6, c4
        umulh   c4, a3, b3
        adc     u7, c4, xzr

// Add up the carry-in and the existing z contents

        ldp     u2, u3, [z]
        adds    c0, c0, u2
        adcs    c1, c1, u3
        ldp     u2, u3, [z, #16]
        adcs    c2, c2, u2
        adcs    c3, c3, u3
        adc     c4, xzr, xzr

// Multiply by B + 1 to get [u7;u6;u5;u4;u1;u0]

        adds    u1, u4, u0
        adcs    u4, u5, u4
        adcs    u5, u6, u5
        adcs    u6, u7, u6
        adc     u7, xzr, u7

// Multiply by B^2 + 1 to get [u6;u5;u4;u3;u2;u1;u0]

        adds    u2, u4, u0
        adcs    u3, u5, u1
        adcs    u4, u6, u4
        adcs    u5, u7, u5
        adcs    u6, xzr, u6
        adc     u7, xzr, u7

// Add in the carry-in and original z contents

        adds    u0, u0, c0
        adcs    u1, u1, c1
        adcs    u2, u2, c2
        adcs    u3, u3, c3
        adcs    u4, u4, c4
        adcs    u5, u5, xzr
        adcs    u6, u6, xzr
        adc     u7, u7, xzr

// Now add in all the "complicated" terms.

        muldiffnadd (u6,u5, c,h,l,t, a2,a3, b3,b2)
        adc     u7, u7, c

        muldiffnadd (u2,u1, c,h,l,t, a0,a1, b1,b0)
        adcs    u3, u3, c
        adcs    u4, u4, c
        adcs    u5, u5, c
        adcs    u6, u6, c
        adc     u7, u7, c

        muldiffnadd (u5,u4, c,h,l,t, a1,a3, b3,b1)
        adcs    u6, u6, c
        adc     u7, u7, c

        muldiffnadd (u3,u2, c,h,l,t, a0,a2, b2,b0)
        adcs    u4, u4, c
        adcs    u5, u5, c
        adcs    u6, u6, c
        adc     u7, u7, c

        muldiffnadd (u4,u3, c,h,l,t, a0,a3, b3,b0)
        adcs    u5, u5, c
        adcs    u6, u6, c
        adc     u7, u7, c
        muldiffnadd (u4,u3, c,h,l,t, a1,a2, b2,b1)
        adcs    c1, u5, c
        adcs    c2, u6, c
        adc     c3, u7, c
        mov     c0, u4

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
.endm

// *****************************************************
// Main code
// *****************************************************

S2N_BN_SYMBOL(bignum_emontredc_8n_base):

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x24, [sp, #-16]!
        stp     x25, x26, [sp, #-16]!
        stp     x27, x28, [sp, #-16]!

// Set up (k/4 - 1)<<5 which is used as inner count and pointer fixup
// ns i = k/4 as the outer loop count.
// At this point skip everything if k/4 = 0, returning our x0 = 0 value

        lsr     k4m1, x0, #2
        mov     i, k4m1
        subs    c, k4m1, #1
        bcc     bignum_emontredc_8n_base_end
        mov     tc, xzr
        lsl     k4m1, c, #5

// Outer loop, one digit of Montgomery reduction adding in word * m.
// Rather than propagating the carry to the end each time, we
// stop at the "natural" end and store top carry in tc as a bitmask.

bignum_emontredc_8n_base_outerloop:

// Load [u3;u2;u1;u0] = bottom 4 digits of the input at current window

        ldp     u0, u1, [z]
        ldp     u2, u3, [z, #16]

// Load the bottom 4 digits of m

        ldp     b0, b1, [m]
        ldp     b2, b3, [m, #16]

// Montgomery step 0

        mul     a0, u0, w
        mul     c0, a0, b0
        mul     c1, a0, b1
        mul     c2, a0, b2
        mul     c3, a0, b3
        adds    u0, u0, c0
        umulh   c0, a0, b0
        adcs    u1, u1, c1
        umulh   c1, a0, b1
        adcs    u2, u2, c2
        umulh   c2, a0, b2
        adcs    u3, u3, c3
        umulh   c3, a0, b3
        adc     u4, xzr, xzr
        adds    u1, u1, c0
        adcs    u2, u2, c1
        adcs    u3, u3, c2
        adc     u4, u4, c3

// Montgomery step 1

        mul     a1, u1, w
        mul     c0, a1, b0
        mul     c1, a1, b1
        mul     c2, a1, b2
        mul     c3, a1, b3
        adds    u1, u1, c0
        umulh   c0, a1, b0
        adcs    u2, u2, c1
        umulh   c1, a1, b1
        adcs    u3, u3, c2
        umulh   c2, a1, b2
        adcs    u4, u4, c3
        umulh   c3, a1, b3
        adc     u5, xzr, xzr
        adds    u2, u2, c0
        adcs    u3, u3, c1
        adcs    u4, u4, c2
        adc     u5, u5, c3

// Montgomery step 2

        mul     a2, u2, w
        mul     c0, a2, b0
        mul     c1, a2, b1
        mul     c2, a2, b2
        mul     c3, a2, b3
        adds    u2, u2, c0
        umulh   c0, a2, b0
        adcs    u3, u3, c1
        umulh   c1, a2, b1
        adcs    u4, u4, c2
        umulh   c2, a2, b2
        adcs    u5, u5, c3
        umulh   c3, a2, b3
        adc     u6, xzr, xzr
        adds    u3, u3, c0
        adcs    u4, u4, c1
        adcs    u5, u5, c2
        adc     u6, u6, c3

// Montgomery step 3. In the last four instructions we put the top in
// the carry variables expected by the "madd" block next, which is why
// the pattern is slightly different.

        mul     a3, u3, w
        mul     c0, a3, b0
        mul     c1, a3, b1
        mul     c2, a3, b2
        mul     c3, a3, b3
        adds    u3, u3, c0
        umulh   c0, a3, b0
        adcs    u4, u4, c1
        umulh   c1, a3, b1
        adcs    u5, u5, c2
        umulh   c2, a3, b2
        adcs    u6, u6, c3
        umulh   c3, a3, b3
        adc     u7, xzr, xzr
        adds    c0, u4, c0
        adcs    c1, u5, c1
        adcs    c2, u6, c2
        adc     c3, u7, c3

// Stash the multipliers as expected by the bignum_emontredc interface
// We don't use these ourselves again though; they stay in [a3;a2;a1;a0]

        stp     a0, a1, [z]
        stp     a2, a3, [z, #16]

// Repeated multiply-add block to do the k/4-1 remaining 4-digit chunks

        mov     j, k4m1
bignum_emontredc_8n_base_maddloop:
        add     m, m, #32
        add     z, z, #32

        ldp     b0, b1, [m]
        ldp     b2, b3, [m, #16]
        madd4
        subs    j, j, #32
        bne     bignum_emontredc_8n_base_maddloop
bignum_emontredc_8n_base_madddone:

// Add the carry out to the existing z contents, propagating the
// top carry tc up by 32 places as we move "leftwards".

        ldp     u0, u1, [z, #32]
        ldp     u2, u3, [z, #48]
        adds    xzr, tc, tc
        adcs    u0, u0, c0
        adcs    u1, u1, c1
        adcs    u2, u2, c2
        adcs    u3, u3, c3
        csetm   tc, cs
        stp     u0, u1, [z, #32]
        stp     u2, u3, [z, #48]

// Compensate for the repeated bumps in m and z in the inner loop

        sub     z, z, k4m1
        sub     m, m, k4m1

// Bump up z only and keep going

        add     z, z, #32
        sub     i, i, #1
        cbnz    i, bignum_emontredc_8n_base_outerloop

// Return the top carry as 0 or 1 (it's currently a bitmask)

        neg     x0, tc

bignum_emontredc_8n_base_end:
        ldp     x27, x28, [sp], #16
        ldp     x25, x26, [sp], #16
        ldp     x23, x24, [sp], #16
        ldp     x21, x22, [sp], #16
        ldp     x19, x20, [sp], #16

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
