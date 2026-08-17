// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Divide by a single (nonzero) word, z := x / m and return x mod m
// Inputs x[n], m; outputs function return (remainder) and z[k]
//
//    extern uint64_t bignum_cdiv(uint64_t k, uint64_t *z, uint64_t n,
//                                const uint64_t *x, uint64_t m);
//
// Does the "z := x / m" operation where x is n digits, result z is k.
// Truncates the quotient in general, but always (for nonzero m) returns
// the true remainder x mod m.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x, X4 = m, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cdiv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cdiv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cdiv)
        .text
        .balign 4

#define k x0
#define z x1
#define n x2
#define x x3
#define m x4

// Main variables

#define w x5
#define i x6
#define a x7
#define c x8
#define d x9
#define e x10
#define f x11
#define l x12

// These two are the same

#define h x13
#define q x13

// Variables for the negmodinv

#define one x6
#define e1 x6
#define e2 x7
#define e4 x6
#define e8 x7

// Variable to hold the remainder

#define r x14

S2N_BN_SYMBOL(bignum_cdiv):
        CFI_START

// Effectively the same dataflow as bignum_cmod, with some basic
// variable changes (using n for the size not k, returning r, etc.)
// and using the i counter instead of modifying the size as a loop
// counter.

        mov     r, xzr
        cbz     n, Lbignum_cdiv_nomodulus

        clz     e, m
        lsl     f, m, e

        lsr     a, f, #16
        eor     w, a, #0x1ffffffffffff
        add     a, a, #0x1
        lsr     w, w, #32
        mneg    r, a, w
        lsr     d, r, #49
        mul     d, d, d
        lsr     r, r, #34
        add     r, d, r
        orr     d, d, #0x40000000
        mul     d, r, d
        lsr     d, d, #30
        lsl     r, w, #30
        madd    w, w, d, r
        lsr     w, w, #30
        mneg    r, a, w
        lsr     r, r, #24
        mul     r, r, w
        lsl     w, w, #16
        lsr     r, r, #24
        add     w, w, r
        mneg    r, a, w
        lsr     r, r, #32
        mul     r, r, w
        lsl     w, w, #31
        lsr     r, r, #17
        add     w, w, r
        mul     d, f, w
        umulh   r, f, w
        extr    d, r, d, #60
        lsr     r, w, #33
        mvn     d, d
        mul     d, r, d
        lsl     w, w, #1
        lsr     d, d, #33
        add     w, w, d
        adds    d, w, #0x1
        cinv    d, d, eq
        umulh   r, f, d
        adds    xzr, r, f
        csel    w, w, d, cs

        mneg    r, w, f

        mov     h, xzr
        mov     l, xzr
        mov     i, n
Lbignum_cdiv_modloop:
        sub     i, i, #1
        ldr     d, [x, i, lsl #3]
        mul     a, r, h
        umulh   h, r, h
        adds    a, a, d
        adcs    h, h, l
        csel    l, r, xzr, cs
        adds    l, l, a
        adc     h, h, xzr
        cbnz    i, Lbignum_cdiv_modloop

        umulh   c, w, h
        adds    c, c, h
        csel    r, f, xzr, cs

        mul     a, c, f
        umulh   d, c, f
        add     d, d, r
        subs    l, l, a
        sbcs    h, h, d

        csel    a, f, xzr, ne
        subs    l, l, a
        sbcs    h, h, xzr

        csel    a, f, xzr, ne
        sub     l, l, a

        umulh   c, w, l
        adds    c, c, l
        cset    r, cs
        extr    c, r, c, #1

        eor     e, e, #63
        lsr     c, c, e

        mul     a, c, m
        sub     l, l, a

        subs    r, l, m
        csel    r, r, l, cs

Lbignum_cdiv_nomodulus:

// If k = 0 then there's no more to be done

        cbz     k, Lbignum_cdiv_end

// Let e be the number of trailing zeros in m. This implementation uses
// 63 - clz(-m & m) which is a bit slicker than the main word_ctz function
// but fails for m = 0. We don't have to worry about that case here.

        neg     e, m
        and     e, e, m
        clz     e, e
        eor     e, e, #63

// Also generate a corresponding bitmask f for selecting bottom 64 - e bits.

        mov     f, #-1
        lsr     f, f, e

// Now just shift m right by e bits. So hereafter we can assume m is odd
// but we first need to shift the input right by e bits then divide by m.

        lsr     m, m, e

// Compute the negated modular inverse w with w * m + 1 == 0 (mod 2^64)
// This is essentially the same as word_negmodinv.

        sub     w, m, m, lsl #2
        eor     w, w, #2
        mov     one, #1
        madd    e1, m, w, one
        mul     e2, e1, e1
        madd    w, e1, w, w
        mul     e4, e2, e2
        madd    w, e2, w, w
        mul     e8, e4, e4
        madd    w, e4, w, w
        madd    w, e8, w, w

// We have the remainder r, so now x = m * y + r for some quotient y
// to be computed. Consider x' = x + (m - r) = m * (y + 1) and do a
// Montgomery reduction, keeping the cofactor z. This gives us
// x' + m * z = 2^{64k} * c where c <= m. Thus since x' = m * (y + 1)
// we have
//
//     m * (y + z + 1) = 2^{64k} * c
//
// This means m * (y + z + 1) == 0 (mod 2^{64k}), even when we truncate
// x to k digits (if in fact k < n). Since m is odd, it's coprime to
// 2^{64k} so we can cancel and get y + z + 1 == 0 (mod 2^{64k}), and
// hence using logical complement y == ~z (mod 2^{64k}). Thus we can
// write back the logical complements of the cofactor as the answer.
// Start with carry word c = m - r/2^e to make the initial tweak
// x' = x + (m - r); since we've shifted everything initially by e
// we need to shift the remainder too before subtracting from the
// shifted m.

        lsr     c, r, e
        sub     c, m, c
        mov     i, xzr

// Unless n = 0, preload the zeroth digit shifted right e places and bump
// up the x pointer by 8 and n down by 1, to ease indexing and comparison
// using the same variable i in the main loop. When n = 0 we leave it alone,
// as the comparison i < n will always fail and the x pointer is unused.

        mov     d, xzr
        cbz     n, Lbignum_cdiv_loop
        ldr     d, [x], #8
        lsr     d, d, e
        sub     n, n, 1

Lbignum_cdiv_loop:

// Load the next digit up to get [l,d] then shift right e places,
// eventually setting d back to the other part of the newly loaded digit
// ready for the next time round the loop.

        mov     l, xzr
        cmp     i, n
        bcs     Lbignum_cdiv_noload
        ldr     l, [x, i, lsl #3]
Lbignum_cdiv_noload:
        rorv    l, l, e
        bic     a, l, f
        orr     a, d, a
        and     d, l, f

// Now a is the next digit after shifting right by e places, c the carry-in.
// Do the main Montgomery step with the (odd) m, writing back ~q.

        adds    a, a, c
        mul     q, a, w
        cset    c, cs
        mvn     l, q
        str     l, [z, i, lsl #3]

        mul     l, q, m
        umulh   h, q, m

        adds    l, l, a
        adc     c, h, c

        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_cdiv_loop

// And return the remainder

Lbignum_cdiv_end:

        mov     x0, r
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cdiv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
