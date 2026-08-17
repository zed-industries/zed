// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^576) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_montsqr_p521_base
//     (uint64_t z[static 9], uint64_t x[static 9]);
//
// Does z := (x^2 / 2^576) mod p_521, assuming x < p_521. This means the
// Montgomery base is the "native size" 2^{9*64} = 2^576; since p_521 is
// a Mersenne prime the basic modular squaring bignum_sqr_p521_base can be
// considered a Montgomery operation to base 2^521.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p521_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p521_base)
        .text
        .balign 4

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5
#define b0 x6
#define b1 x7
#define b2 x8
#define b3 x9

#define s0 x10
#define s1 x11
#define s2 x12
#define s3 x13
#define s4 x14
#define s5 x15
#define s6 x16
#define s7 x17

#define c  x19
#define h  x20
#define l  x21
#define t  x22
#define u  x23
#define v  x24

// Aliased to earlier ones we no longer need

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7
#define d6 x8
#define d7 x9
#define d8 x10

S2N_BN_SYMBOL(bignum_montsqr_p521_base):

// Save registers

        stp     x19, x20, [sp, #-16]!
        stp     x21, x22, [sp, #-16]!
        stp     x23, x24, [sp, #-16]!

// Load all the inputs first

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        ldp     b0, b1, [x, #32]
        ldp     b2, b3, [x, #48]

// Square the upper half with a register-renamed variant of bignum_sqr_4_8

        mul     s2, b0, b2
        mul     s7, b1, b3
        umulh   t, b0, b2
        subs    u, b0, b1
        cneg    u, u, cc
        csetm   s1, cc
        subs    s0, b3, b2
        cneg    s0, s0, cc
        mul     s6, u, s0
        umulh   s0, u, s0
        cinv    s1, s1, cc
        eor     s6, s6, s1
        eor     s0, s0, s1
        adds    s3, s2, t
        adc     t, t, xzr
        umulh   u, b1, b3
        adds    s3, s3, s7
        adcs    t, t, u
        adc     u, u, xzr
        adds    t, t, s7
        adc     u, u, xzr
        cmn     s1, #0x1
        adcs    s3, s3, s6
        adcs    t, t, s0
        adc     u, u, s1
        adds    s2, s2, s2
        adcs    s3, s3, s3
        adcs    t, t, t
        adcs    u, u, u
        adc     c, xzr, xzr
        mul     s0, b0, b0
        mul     s6, b1, b1
        mul     l, b0, b1
        umulh   s1, b0, b0
        umulh   s7, b1, b1
        umulh   h, b0, b1
        adds    s1, s1, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s1, s1, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s2, s2, s6
        adcs    s3, s3, s7
        adcs    t, t, xzr
        adcs    u, u, xzr
        adc     c, c, xzr
        mul     s4, b2, b2
        mul     s6, b3, b3
        mul     l, b2, b3
        umulh   s5, b2, b2
        umulh   s7, b3, b3
        umulh   h, b2, b3
        adds    s5, s5, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s5, s5, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s4, s4, t
        adcs    s5, s5, u
        adcs    s6, s6, c
        adc     s7, s7, xzr

// Augment the high part with the contribution from the top little word C.
// If we write the input as 2^512 * C + x then we are otherwise just doing
// x^2, so we need to add to the high part 2^512 * C^2 + (2 * C) * x.
// Accumulate it as [c;s7;...;s0] = H'. Since 2 * C is only 10 bits long
// we multiply 52-bit chunks of the x digits by 2 * C and solve the overlap
// with non-overflowing addition to get 52-bit chunks of the result with
// similar alignment. Then we stitch these back together and add them into
// the running total. This is quite a bit of palaver, but it avoids using
// the standard 2-part multiplications involving umulh, and on target
// microarchitectures seems to improve performance by about 5%. We could
// equally well use 53 or 54 since they are still <= 64 - 10, but below
// 52 we would end up using more multiplications.

        ldr     c, [x, #64]
        add     u, c, c
        mul     c, c, c

// 0 * 52 = 64 * 0 + 0

        and     l, a0, #0x000fffffffffffff
        mul     l, u, l

// 1 * 52 = 64 * 0 + 52

        extr    h, a1, a0, #52
        and     h, h, #0x000fffffffffffff
        mul     h, u, h
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #12
        adds    s0, s0, t

// 2 * 52 = 64 * 1 + 40

        extr    l, a2, a1, #40
        and     l, l, #0x000fffffffffffff
        mul     l, u, l
        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #24
        adcs    s1, s1, t

// 3 * 52 = 64 * 2 + 28

        extr    h, a3, a2, #28
        and     h, h, #0x000fffffffffffff
        mul     h, u, h
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #36
        adcs    s2, s2, t

// 4 * 52 = 64 * 3 + 16

        extr    l, b0, a3, #16
        and     l, l, #0x000fffffffffffff
        mul     l, u, l
        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #48
        adcs    s3, s3, t

// 5 * 52 = 64 * 4 + 4

        lsr     h, b0, #4
        and     h, h, #0x000fffffffffffff
        mul     h, u, h
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    v, h, l, #60

// 6 * 52 = 64 * 4 + 56

        extr    l, b1, b0, #56
        and     l, l, #0x000fffffffffffff
        mul     l, u, l
        lsr     t, h, #52
        add     l, l, t

        lsl     v, v, #8
        extr    t, l, v, #8
        adcs    s4, s4, t

// 7 * 52 = 64 * 5 + 44

        extr    h, b2, b1, #44
        and     h, h, #0x000fffffffffffff
        mul     h, u, h
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #20
        adcs    s5, s5, t

// 8 * 52 = 64 * 6 + 32

        extr    l, b3, b2, #32
        and     l, l, #0x000fffffffffffff
        mul     l, u, l
        lsr     t, h, #52
        add     l, l, t

        lsl     h, h, #12
        extr    t, l, h, #32
        adcs    s6, s6, t

// 9 * 52 = 64 * 7 + 20

        lsr     h, b3, #20
        mul     h, u, h
        lsr     t, l, #52
        add     h, h, t

        lsl     l, l, #12
        extr    t, h, l, #44
        adcs    s7, s7, t

// Top word

        lsr     h, h, #44
        adc     c, c, h

// Rotate [c;s7;...;s0] before storing in the buffer.
// We want to add 2^512 * H', which splitting H' at bit 9 is
// 2^521 * H_top + 2^512 * H_bot == 2^512 * H_bot + H_top (mod p_521)

        extr    l, s1, s0, #9
        extr    h, s2, s1, #9
        stp     l, h, [z]

        extr    l, s3, s2, #9
        extr    h, s4, s3, #9
        stp     l, h, [z, #16]

        extr    l, s5, s4, #9
        extr    h, s6, s5, #9
        stp     l, h, [z, #32]

        extr    l, s7, s6, #9
        extr    h, c, s7, #9
        stp     l, h, [z, #48]

        and     t, s0, #0x1FF
        lsr     c, c, #9
        add     t, t, c
        str     t, [z, #64]

// Square the lower half with an analogous variant of bignum_sqr_4_8

        mul     s2, a0, a2
        mul     s7, a1, a3
        umulh   t, a0, a2
        subs    u, a0, a1
        cneg    u, u, cc
        csetm   s1, cc
        subs    s0, a3, a2
        cneg    s0, s0, cc
        mul     s6, u, s0
        umulh   s0, u, s0
        cinv    s1, s1, cc
        eor     s6, s6, s1
        eor     s0, s0, s1
        adds    s3, s2, t
        adc     t, t, xzr
        umulh   u, a1, a3
        adds    s3, s3, s7
        adcs    t, t, u
        adc     u, u, xzr
        adds    t, t, s7
        adc     u, u, xzr
        cmn     s1, #0x1
        adcs    s3, s3, s6
        adcs    t, t, s0
        adc     u, u, s1
        adds    s2, s2, s2
        adcs    s3, s3, s3
        adcs    t, t, t
        adcs    u, u, u
        adc     c, xzr, xzr
        mul     s0, a0, a0
        mul     s6, a1, a1
        mul     l, a0, a1
        umulh   s1, a0, a0
        umulh   s7, a1, a1
        umulh   h, a0, a1
        adds    s1, s1, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s1, s1, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s2, s2, s6
        adcs    s3, s3, s7
        adcs    t, t, xzr
        adcs    u, u, xzr
        adc     c, c, xzr
        mul     s4, a2, a2
        mul     s6, a3, a3
        mul     l, a2, a3
        umulh   s5, a2, a2
        umulh   s7, a3, a3
        umulh   h, a2, a3
        adds    s5, s5, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s5, s5, l
        adcs    s6, s6, h
        adc     s7, s7, xzr
        adds    s4, s4, t
        adcs    s5, s5, u
        adcs    s6, s6, c
        adc     s7, s7, xzr

// Add it directly to the existing buffer

        ldp     l, h, [z]
        adds    l, l, s0
        adcs    h, h, s1
        stp     l, h, [z]

        ldp     l, h, [z, #16]
        adcs    l, l, s2
        adcs    h, h, s3
        stp     l, h, [z, #16]

        ldp     l, h, [z, #32]
        adcs    l, l, s4
        adcs    h, h, s5
        stp     l, h, [z, #32]

        ldp     l, h, [z, #48]
        adcs    l, l, s6
        adcs    h, h, s7
        stp     l, h, [z, #48]

        ldr     t, [z, #64]
        adc     t, t, xzr
        str     t, [z, #64]

// Now get the cross-product in [s7,...,s0] with variant of bignum_mul_4_8

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
        adds    s1, s4, s0
        adcs    s4, s5, s4
        adcs    s5, s6, s5
        adcs    s6, s7, s6
        adc     s7, xzr, s7
        adds    s2, s4, s0
        adcs    s3, s5, s1
        adcs    s4, s6, s4
        adcs    s5, s7, s5
        adcs    s6, xzr, s6
        adc     s7, xzr, s7
        subs    t, a2, a3
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b3, b2
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s5, s5, l
        eor     h, h, c
        adcs    s6, s6, h
        adc     s7, s7, c
        subs    t, a0, a1
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b1, b0
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s1, s1, l
        eor     h, h, c
        adcs    s2, s2, h
        adcs    s3, s3, c
        adcs    s4, s4, c
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c
        subs    t, a1, a3
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b3, b1
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s4, s4, l
        eor     h, h, c
        adcs    s5, s5, h
        adcs    s6, s6, c
        adc     s7, s7, c
        subs    t, a0, a2
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b2, b0
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s2, s2, l
        eor     h, h, c
        adcs    s3, s3, h
        adcs    s4, s4, c
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c
        subs    t, a0, a3
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b3, b0
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s3, s3, l
        eor     h, h, c
        adcs    s4, s4, h
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c
        subs    t, a1, a2
        cneg    t, t, cc
        csetm   c, cc
        subs    h, b2, b1
        cneg    h, h, cc
        mul     l, t, h
        umulh   h, t, h
        cinv    c, c, cc
        cmn     c, #0x1
        eor     l, l, c
        adcs    s3, s3, l
        eor     h, h, c
        adcs    s4, s4, h
        adcs    s5, s5, c
        adcs    s6, s6, c
        adc     s7, s7, c

// Let the cross product be M. We want to add 2^256 * 2 * M to the buffer
// Split M into M_top (248 bits) and M_bot (264 bits), so we add
// 2^521 * M_top + 2^257 * M_bot == 2^257 * M_bot + M_top (mod p_521)
// Accumulate the (non-reduced in general) 9-word answer [d8;...;d0]
// As this sum is built, accumulate t = AND of words d7...d1 to help
// in condensing the carry chain in the comparison that comes next

        ldp     l, h, [z]
        extr    d0, s5, s4, #8
        adds    d0, d0, l
        extr    d1, s6, s5, #8
        adcs    d1, d1, h

        ldp     l, h, [z, #16]
        extr    d2, s7, s6, #8
        adcs    d2, d2, l
        and     t, d1, d2
        lsr     d3, s7, #8
        adcs    d3, d3, h
        and     t, t, d3

        ldp     l, h, [z, #32]
        lsl     d4, s0, #1
        adcs    d4, d4, l
        and     t, t, d4
        extr    d5, s1, s0, #63
        adcs    d5, d5, h
        and     t, t, d5

        ldp     l, h, [z, #48]
        extr    d6, s2, s1, #63
        adcs    d6, d6, l
        and     t, t, d6
        extr    d7, s3, s2, #63
        adcs    d7, d7, h
        and     t, t, d7

        ldr     l, [z, #64]
        extr    d8, s4, s3, #63
        and     d8, d8, #0x1FF
        adc     d8, l, d8

// Extract the high part h and mask off the low part l = [d8;d7;...;d0]
// but stuff d8 with 1 bits at the left to ease a comparison below

        lsr     h, d8, #9
        orr     d8, d8, #~0x1FF

// Decide whether h + l >= p_521 <=> h + l + 1 >= 2^521. Since this can only
// happen if digits d7,...d1 are all 1s, we use the AND of them "t" to
// condense the carry chain, and since we stuffed 1 bits into d8 we get
// the result in CF without an additional comparison.

        subs    xzr, xzr, xzr
        adcs    xzr, d0, h
        adcs    xzr, t, xzr
        adcs    xzr, d8, xzr

// Now if CF is set we want (h + l) - p_521 = (h + l + 1) - 2^521
// while otherwise we want just h + l. So mask h + l + CF to 521 bits.
// This masking also gets rid of the stuffing with 1s we did above.

        adcs    d0, d0, h
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr
        adcs    d6, d6, xzr
        adcs    d7, d7, xzr
        adc     d8, d8, xzr
        and     d8, d8, #0x1FF

// So far, this has been the same as a pure modular squaring.
// Now finally the Montgomery ingredient, which is just a 521-bit
// rotation by 9*64 - 521 = 55 bits right.

        lsl     c, d0, #9
        extr    d0, d1, d0, #55
        extr    d1, d2, d1, #55
        extr    d2, d3, d2, #55
        extr    d3, d4, d3, #55
        orr     d8, d8, c
        extr    d4, d5, d4, #55
        extr    d5, d6, d5, #55
        extr    d6, d7, d6, #55
        extr    d7, d8, d7, #55
        lsr     d8, d8, #55

// Store the final result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]

// Restore regs and return

        ldp     x23, x24, [sp], #16
        ldp     x21, x22, [sp], #16
        ldp     x19, x20, [sp], #16

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
