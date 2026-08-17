// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_521, z := (x * y) mod p_521, assuming x and y reduced
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_mul_p521_base
//     (uint64_t z[static 9], uint64_t x[static 9], uint64_t y[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p521_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p521_base)
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

#define muldiffnadd(b,a,x,y,w,z)        \
        subs    t, x, y __LF               \
        cneg    t, t, cc __LF              \
        csetm   c, cc __LF                 \
        subs    h, w, z __LF               \
        cneg    h, h, cc __LF              \
        mul     l, t, h __LF               \
        umulh   h, t, h __LF               \
        cinv    c, c, cc __LF              \
        adds    xzr, c, #1 __LF            \
        eor     l, l, c __LF               \
        adcs    a, a, l __LF               \
        eor     h, h, c __LF               \
        adcs    b, b, h

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
#define s8 x20

#define c  x21
#define h  x22
#define l  x23
#define t  x24
#define s  x25
#define u  x26

// ---------------------------------------------------------------------------
// Core 4x4->8 ADK multiplication macro
// Does [s7,s6,s5,s4,s3,s2,s1,s0] = [a3,a2,a1,a0] * [b3,b2,b1,b0]
// ---------------------------------------------------------------------------

#define mul4                                                            \
/*  First accumulate all the "simple" products as [s7,s6,s5,s4,s0] */   \
        \
        mul     s0, a0, b0 __LF                                    \
        mul     s4, a1, b1 __LF                                    \
        mul     s5, a2, b2 __LF                                    \
        mul     s6, a3, b3 __LF                                    \
        \
        umulh   s7, a0, b0 __LF                                    \
        adds    s4, s4, s7 __LF                                    \
        umulh   s7, a1, b1 __LF                                    \
        adcs    s5, s5, s7 __LF                                    \
        umulh   s7, a2, b2 __LF                                    \
        adcs    s6, s6, s7 __LF                                    \
        umulh   s7, a3, b3 __LF                                    \
        adc     s7, s7, xzr __LF                                   \
        \
/*  Multiply by B + 1 to get [s7;s6;s5;s4;s1;s0] */                     \
        \
        adds    s1, s4, s0 __LF                                    \
        adcs    s4, s5, s4 __LF                                    \
        adcs    s5, s6, s5 __LF                                    \
        adcs    s6, s7, s6 __LF                                    \
        adc     s7, xzr, s7 __LF                                   \
        \
/*  Multiply by B^2 + 1 to get [s7;s6;s5;s4;s3;s2;s1;s0] */             \
        \
        adds    s2, s4, s0 __LF                                    \
        adcs    s3, s5, s1 __LF                                    \
        adcs    s4, s6, s4 __LF                                    \
        adcs    s5, s7, s5 __LF                                    \
        adcs    s6, xzr, s6 __LF                                   \
        adc     s7, xzr, s7 __LF                                   \
        \
/*  Now add in all the "complicated" terms. */                          \
        \
        muldiffnadd(s6,s5, a2,a3, b3,b2) __LF                      \
        adc     s7, s7, c __LF                                     \
        \
        muldiffnadd(s2,s1, a0,a1, b1,b0) __LF                      \
        adcs    s3, s3, c __LF                                     \
        adcs    s4, s4, c __LF                                     \
        adcs    s5, s5, c __LF                                     \
        adcs    s6, s6, c __LF                                     \
        adc     s7, s7, c __LF                                     \
        \
        muldiffnadd(s5,s4, a1,a3, b3,b1) __LF                      \
        adcs    s6, s6, c __LF                                     \
        adc     s7, s7, c __LF                                     \
        \
        muldiffnadd(s3,s2, a0,a2, b2,b0) __LF                      \
        adcs    s4, s4, c __LF                                     \
        adcs    s5, s5, c __LF                                     \
        adcs    s6, s6, c __LF                                     \
        adc     s7, s7, c __LF                                     \
        \
        muldiffnadd(s4,s3, a0,a3, b3,b0) __LF                      \
        adcs    s5, s5, c __LF                                     \
        adcs    s6, s6, c __LF                                     \
        adc     s7, s7, c __LF                                     \
        muldiffnadd(s4,s3, a1,a2, b2,b1) __LF                      \
        adcs    s5, s5, c __LF                                     \
        adcs    s6, s6, c __LF                                     \
        adc     s7, s7, c                                       \

S2N_BN_SYMBOL(bignum_mul_p521_base):

// Save registers and make space for the temporary buffer

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x24, [sp, #-16]!
        stp     x25, x26, [sp, #-16]!
        sub     sp, sp, #80

// Load 4-digit low parts and multiply them to get L

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        ldp     b0, b1, [y]
        ldp     b2, b3, [y, #16]
        mul4

// Shift right 256 bits modulo p_521 and stash in temp buffer

        lsl     c, s0, #9
        extr    s0, s1, s0, #55
        extr    s1, s2, s1, #55
        extr    s2, s3, s2, #55
        lsr     s3, s3, #55
        stp     s4, s5, [sp]
        stp     s6, s7, [sp, #16]
        stp     c, s0, [sp, #32]
        stp     s1, s2, [sp, #48]
        str     s3, [sp, #64]

// Load 4-digit low parts and multiply them to get H

        ldp     a0, a1, [x, #32]
        ldp     a2, a3, [x, #48]
        ldp     b0, b1, [y, #32]
        ldp     b2, b3, [y, #48]
        mul4

// Add to the existing temporary buffer and re-stash.
// This gives a result HL congruent to (2^256 * H + L) / 2^256 modulo p_521

        ldp     l, h, [sp]
        adds    s0, s0, l
        adcs    s1, s1, h
        stp     s0, s1, [sp]
        ldp     l, h, [sp, #16]
        adcs    s2, s2, l
        adcs    s3, s3, h
        stp     s2, s3, [sp, #16]
        ldp     l, h, [sp, #32]
        adcs    s4, s4, l
        adcs    s5, s5, h
        stp     s4, s5, [sp, #32]
        ldp     l, h, [sp, #48]
        adcs    s6, s6, l
        adcs    s7, s7, h
        stp     s6, s7, [sp, #48]
        ldr     c, [sp, #64]
        adc     c, c, xzr
        str     c, [sp, #64]

// Compute t,[a3,a2,a1,a0] = x_hi - x_lo
// and     s,[b3,b2,b1,b0] = y_lo - y_hi
// sign-magnitude differences, then XOR overall sign bitmask into s

        ldp     l, h, [x]
        subs    a0, a0, l
        sbcs    a1, a1, h
        ldp     l, h, [x, #16]
        sbcs    a2, a2, l
        sbcs    a3, a3, h
        csetm   t, cc
        ldp     l, h, [y]
        subs    b0, l, b0
        sbcs    b1, h, b1
        ldp     l, h, [y, #16]
        sbcs    b2, l, b2
        sbcs    b3, h, b3
        csetm   s, cc

        eor     a0, a0, t
        subs    a0, a0, t
        eor     a1, a1, t
        sbcs    a1, a1, t
        eor     a2, a2, t
        sbcs    a2, a2, t
        eor     a3, a3, t
        sbc     a3, a3, t

        eor     b0, b0, s
        subs    b0, b0, s
        eor     b1, b1, s
        sbcs    b1, b1, s
        eor     b2, b2, s
        sbcs    b2, b2, s
        eor     b3, b3, s
        sbc     b3, b3, s

        eor     s, s, t

// Now do yet a third 4x4 multiply to get mid-term product M

        mul4

// We now want, at the 256 position, 2^256 * HL + HL + (-1)^s * M
// To keep things positive we use M' = p_521 - M in place of -M,
// and this notion of negation just amounts to complementation in 521 bits.
// Fold in the re-addition of the appropriately scaled lowest 4 words
// The initial result is [s8; b3;b2;b1;b0; s7;s6;s5;s4;s3;s2;s1;s0]
// Rebase it as a 9-word value at the 512 bit position using
// [s8; b3;b2;b1;b0; s7;s6;s5;s4;s3;s2;s1;s0] ==
// [s8; b3;b2;b1;b0; s7;s6;s5;s4] + 2^265 * [s3;s2;s1;s0] =
// ([s8; b3;b2;b1;b0] + 2^9 * [s3;s2;s1;s0]); s7;s6;s5;s4]
//
// Accumulate as [s8; b3;b2;b1;b0; s7;s6;s5;s4] but leave out an additional
// small c (s8 + suspended carry) to add at the 256 position here (512
// overall). This can be added in the next block (to b0 = sum4).

        ldp     a0, a1, [sp]
        ldp     a2, a3, [sp, #16]

        eor     s0, s0, s
        adds    s0, s0, a0
        eor     s1, s1, s
        adcs    s1, s1, a1
        eor     s2, s2, s
        adcs    s2, s2, a2
        eor     s3, s3, s
        adcs    s3, s3, a3
        eor     s4, s4, s

        ldp     b0, b1, [sp, #32]
        ldp     b2, b3, [sp, #48]
        ldr     s8, [sp, #64]

        adcs    s4, s4, b0
        eor     s5, s5, s
        adcs    s5, s5, b1
        eor     s6, s6, s
        adcs    s6, s6, b2
        eor     s7, s7, s
        adcs    s7, s7, b3
        adc     c, s8, xzr

        adds    s4, s4, a0
        adcs    s5, s5, a1
        adcs    s6, s6, a2
        adcs    s7, s7, a3
        and     s, s, #0x1FF
        lsl     t, s0, #9
        orr     t, t, s
        adcs    b0, b0, t
        extr    t, s1, s0, #55
        adcs    b1, b1, t
        extr    t, s2, s1, #55
        adcs    b2, b2, t
        extr    t, s3, s2, #55
        adcs    b3, b3, t
        lsr     t, s3, #55
        adc     s8, t, s8

// Augment the total with the contribution from the top little words
// w and v. If we write the inputs as 2^512 * w + x and 2^512 * v + y
// then we are otherwise just doing x * y so we actually need to add
// 2^512 * (2^512 * w * v + w * y + v * x). We do this is an involved
// way chopping x and y into 52-bit chunks so we can do most of the core
// arithmetic using only basic muls, no umulh (since w, v are only 9 bits).
// This does however involve some intricate bit-splicing plus arithmetic.
// To make things marginally less confusing we introduce some new names
// at the human level: x = [c7;...;c0] and y = [d7;...d0], which are
// not all distinct, and [sum8;sum7;...;sum0] for the running sum.
// Also accumulate u = sum1 AND ... AND sum7 for the later comparison

#define sum0 s4
#define sum1 s5
#define sum2 s6
#define sum3 s7
#define sum4 b0
#define sum5 b1
#define sum6 b2
#define sum7 b3
#define sum8 s8

#define c0 a0
#define c1 a1
#define c2 a2
#define c3 a0
#define c4 a1
#define c5 a2
#define c6 a0
#define c7 a1

#define d0 s0
#define d1 s1
#define d2 s2
#define d3 s0
#define d4 s1
#define d5 s2
#define d6 s0
#define d7 s1

#define v a3
#define w s3

// 0 * 52 = 64 * 0 + 0

        ldr     v, [y, #64]
        ldp     c0, c1, [x]
        and     l, c0, #0x000fffffffffffff
        mul     l, v, l
        ldr     w, [x, #64]
        ldp     d0, d1, [y]
        and     t, d0, #0x000fffffffffffff
        mul     t, w, t
        add     l, l, t

// 1 * 52 = 64 * 0 + 52

        extr    t, c1, c0, #52
        and     t, t, #0x000fffffffffffff
        mul     h, v, t
        extr    t, d1, d0, #52
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     h, h, t
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #12
        adds    sum0, sum0, t

// 2 * 52 = 64 * 1 + 40

        ldp     c2, c3, [x, #16]
        ldp     d2, d3, [y, #16]
        extr    t, c2, c1, #40
        and     t, t, #0x000fffffffffffff
        mul     l, v, t
        extr    t, d2, d1, #40
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     l, l, t
        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #24
        adcs    sum1, sum1, t

// 3 * 52 = 64 * 2 + 28

        extr    t, c3, c2, #28
        and     t, t, #0x000fffffffffffff
        mul     h, v, t
        extr    t, d3, d2, #28
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     h, h, t
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #36
        adcs    sum2, sum2, t
        and     u, sum1, sum2

// 4 * 52 = 64 * 3 + 16
// At this point we also fold in the addition of c at the right place.
// Note that 4 * 64 = 4 * 52 + 48 so we shift c left 48 places to align.

        ldp     c4, c5, [x, #32]
        ldp     d4, d5, [y, #32]
        extr    t, c4, c3, #16
        and     t, t, #0x000fffffffffffff
        mul     l, v, t
        extr    t, d4, d3, #16
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     l, l, t

        lsl     c, c, #48
        add     l, l, c

        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #48
        adcs    sum3, sum3, t
        and     u, u, sum3

// 5 * 52 = 64 * 4 + 4

        lsr     t, c4, #4
        and     t, t, #0x000fffffffffffff
        mul     h, v, t
        lsr     t, d4, #4
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     h, h, t

        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    s, h, l, #60

// 6 * 52 = 64 * 4 + 56

        extr    t, c5, c4, #56
        and     t, t, #0x000fffffffffffff
        mul     l, v, t
        extr    t, d5, d4, #56
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     l, l, t

        lsr     t, h, #52
        add     l, l, t

        lsl     s, s, #8
        extr    t, l, s, #8
        adcs    sum4, sum4, t
        and     u, u, sum4

// 7 * 52 = 64 * 5 + 44

        ldp     c6, c7, [x, #48]
        ldp     d6, d7, [y, #48]
        extr    t, c6, c5, #44
        and     t, t, #0x000fffffffffffff
        mul     h, v, t
        extr    t, d6, d5, #44
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     h, h, t

        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #20
        adcs    sum5, sum5, t
        and     u, u, sum5

// 8 * 52 = 64 * 6 + 32

        extr    t, c7, c6, #32
        and     t, t, #0x000fffffffffffff
        mul     l, v, t
        extr    t, d7, d6, #32
        and     t, t, #0x000fffffffffffff
        mul     t, w, t
        add     l, l, t

        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #32
        adcs    sum6, sum6, t
        and     u, u, sum6

// 9 * 52 = 64 * 7 + 20

        lsr     t, c7, #20
        mul     h, v, t
        lsr     t, d7, #20
        mul     t, w, t
        add     h, h, t

        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #44
        adcs    sum7, sum7, t
        and     u, u, sum7

// Top word

        mul     t, v, w
        lsr     h, h, #44
        add     t, t, h
        adc     sum8, sum8, t

// Extract the high part h and mask off the low part l = [sum8;sum7;...;sum0]
// but stuff sum8 with 1 bits at the left to ease a comparison below

        lsr     h, sum8, #9
        orr     sum8, sum8, #~0x1FF

// Decide whether h + l >= p_521 <=> h + l + 1 >= 2^521. Since this can only
// happen if digits sum7,...sum1 are all 1s, we use the AND of them "u" to
// condense the carry chain, and since we stuffed 1 bits into sum8 we get
// the result in CF without an additional comparison.

        subs    xzr, xzr, xzr
        adcs    xzr, sum0, h
        adcs    xzr, u, xzr
        adcs    xzr, sum8, xzr

// Now if CF is set we want (h + l) - p_521 = (h + l + 1) - 2^521
// while otherwise we want just h + l. So mask h + l + CF to 521 bits.
// We don't need to mask away bits above 521 since they disappear below.

        adcs    sum0, sum0, h
        adcs    sum1, sum1, xzr
        adcs    sum2, sum2, xzr
        adcs    sum3, sum3, xzr
        adcs    sum4, sum4, xzr
        adcs    sum5, sum5, xzr
        adcs    sum6, sum6, xzr
        adcs    sum7, sum7, xzr
        adc     sum8, sum8, xzr

// The result is actually 2^512 * [sum8;...;sum0] == 2^-9 * [sum8;...;sum0]
// so we rotate right by 9 bits

        and     h, sum0, #0x1FF
        extr    sum0, sum1, sum0, #9
        extr    sum1, sum2, sum1, #9
        stp     sum0, sum1, [z]
        extr    sum2, sum3, sum2, #9
        extr    sum3, sum4, sum3, #9
        stp     sum2, sum3, [z, #16]
        extr    sum4, sum5, sum4, #9
        extr    sum5, sum6, sum5, #9
        stp     sum4, sum5, [z, #32]
        extr    sum6, sum7, sum6, #9
        extr    sum7, sum8, sum7, #9
        stp     sum6, sum7, [z, #48]
        str     h, [z, #64]

// Restore regs and return

        add     sp, sp, #80
        ldp     x25, x26, [sp], #16
        ldp     x23, x24, [sp], #16
        ldp     x21, x22, [sp], #16
        ldp     x19, x20, [sp], #16
        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
