// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Compute "montification" constant z := 2^{128k} mod m
// Input m[k]; output z[k]; temporary buffer t[>=k]
//
//    extern void bignum_montifier(uint64_t k, uint64_t *z, const uint64_t *m,
//                                 uint64_t *t);
//
// The last argument points to a temporary buffer t that should have size >= k.
// This is called "montifier" because given any other k-digit number x,
// whether or not it's reduced modulo m, it can be mapped to its Montgomery
// representation (2^{64k} * x) mod m just by Montgomery multiplication by z.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montifier)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montifier)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montifier)
        .text
        .balign 4

#define k x0
#define z x1
#define m x2
#define t x3

// Some variables
// Modular inverse w is aliased to i, but we never use them together

#define i x4
#define w x4
#define j x5
#define h x6
#define a x7
#define l x8
#define c x9
#define b x10
#define d x11

// Some aliases for the values b and d

#define r x10
#define q x11


S2N_BN_SYMBOL(bignum_montifier):
        CFI_START

// If k = 0 the whole operation is trivial

        cbz     k, Lbignum_montifier_end

// Copy the input m into the temporary buffer t. The temporary register
// c matters since we want it to hold the highest digit, ready for the
// normalization phase.

        mov     i, xzr
Lbignum_montifier_copyinloop:
        ldr     c, [m, i, lsl #3]
        str     c, [t, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_montifier_copyinloop

// Do a rather stupid but constant-time digit normalization, conditionally
// shifting left (k-1) times based on whether the top word is zero.
// With careful binary striding this could be O(k*log(k)) instead of O(k^2)
// while still retaining the constant-time style.
// The "cmp c, xzr" sets the zeroness predicate (ZF) for the entire inner loop

        subs    i, k, #1
        beq     Lbignum_montifier_normalized
Lbignum_montifier_normloop:
        mov     j, xzr
        cmp     c, xzr
        mov     a, xzr
Lbignum_montifier_shufloop:
        mov     c, a
        ldr     a, [t, j, lsl #3]
        csel    c, c, a, eq
        str     c, [t, j, lsl #3]
        add     j, j, #1
        sub     d, j, k
        cbnz    d, Lbignum_montifier_shufloop
        subs    i, i, #1
        bne     Lbignum_montifier_normloop

// We now have the top digit nonzero, assuming the input was nonzero,
// and as per the invariant of the loop above, c holds that digit. So
// now just count c's leading zeros and shift t bitwise that many bits.

Lbignum_montifier_normalized:
        clz     c, c

        mov     b, xzr
        mov     i, xzr
        ands    xzr, c, #63
        csetm   l, ne
        neg     d, c
Lbignum_montifier_bitloop:
        ldr     j, [t, i, lsl #3]
        lsl     a, j, c
        orr     a, a, b
        lsr     b, j, d
        and     b, b, l
        str     a, [t, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_montifier_bitloop

// Let h be the high word of n, which in all the in-scope cases is >= 2^63.
// Now successively form q = 2^i div h and r = 2^i mod h as i goes from
// 64 to 126. We avoid just using division out of constant-time concerns
// (at the least we would need to fix up h = 0 for out-of-scope inputs) and
// don't bother with Newton-Raphson, since this stupid simple loop doesn't
// contribute much of the overall runtime at typical sizes.

        sub     h, k, #1
        ldr     h, [t, h, lsl #3]
        mov     q, #1
        neg     r, h
        mov     i, #62
Lbignum_montifier_estloop:
        add     q, q, q
        mov     a, h
        sub     a, a, r
        cmp     r, a    // CF <=> r >= h - r <=> 2 * r >= h
        csetm   a, cs
        sub     q, q, a
        add     r, r, r
        and     a, a, h
        sub     r, r, a
        subs    i, i, #1
        bne     Lbignum_montifier_estloop

// Strictly speaking the above loop doesn't quite give the true remainder
// and quotient in the special case r = h = 2^63, so fix it up. We get
// q = 2^63 - 1 and r = 2^63 and really want q = 2^63 and r = 0. This is
// supererogatory, because the main property of q used below still holds
// in this case unless the initial m = 1, and then anyway the overall
// specification (congruence modulo m) holds degenerately. But it seems
// nicer to get a "true" quotient and remainder.

        cmp     r, h
        csinc   q, q, q, ne

// So now we have q and r with 2^126 = q * h + r (imagining r = 0 in the
// fixed-up case above: note that we never actually use the computed
// value of r below and so didn't adjust it). And we can assume the ranges
// q <= 2^63 and r < h < 2^64.
//
// The idea is to use q as a first quotient estimate for a remainder
// of 2^{p+62} mod n, where p = 64 * k. We have, splitting n into the
// high and low parts h and l:
//
// 2^{p+62} - q * n = 2^{p+62} - q * (2^{p-64} * h + l)
//                  = 2^{p+62} - (2^{p-64} * (q * h) + q * l)
//                  = 2^{p+62} - 2^{p-64} * (2^126 - r) - q * l
//                  = 2^{p-64} * r - q * l
//
// Note that 2^{p-64} * r < 2^{p-64} * h <= n
// and also  q * l < 2^63 * 2^{p-64} = 2^{p-1} <= n
// so |diff| = |2^{p-64} * r - q * l| < n.
//
// If in fact diff >= 0 then it is already 2^{p+62} mod n.
// otherwise diff + n is the right answer.
//
// To (maybe?) make the computation slightly easier we actually flip
// the sign and compute d = q * n - 2^{p+62}. Then the answer is either
// -d (when negative) or n - d; in either case we effectively negate d.
// This negating tweak in fact spoils the result for cases where
// 2^{p+62} mod n = 0, when we get n instead. However the only case
// where this can happen is m = 1, when the whole spec holds trivially,
// and actually the remainder of the logic below works anyway since
// the latter part of the code only needs a congruence for the k-digit
// result, not strict modular reduction (the doublings will maintain
// the non-strict inequality).

        mov     c, xzr
        adds    i, xzr, xzr
Lbignum_montifier_mulloop:
        ldr     a, [t, i, lsl #3]
        mul     l, q, a
        adcs    l, l, c
        umulh   c, q, a
        str     l, [z, i, lsl #3]
        add     i, i, #1
        sub     a, i, k
        cbnz    a, Lbignum_montifier_mulloop

        adc     c, c, xzr
        mov     a, #0x4000000000000000
        subs    c, c, a
        csetm   q, cs

// Now do [c] * n - d for our final answer

        subs    i, xzr, xzr
Lbignum_montifier_remloop:
        ldr     a, [t, i, lsl #3]
        ldr     b, [z, i, lsl #3]
        and     a, a, q
        sbcs    a, a, b
        str     a, [z, i, lsl #3]
        add     i, i, #1
        sub     a, i, k
        cbnz    a, Lbignum_montifier_remloop

// Now still need to do a couple of modular doublings to get us all the
// way up to 2^{p+64} == r from the initial 2^{p+62} == r (mod n).

        mov     c, xzr
        subs    j, xzr, xzr
Lbignum_montifier_dubloop1:
        ldr     a, [z, j, lsl #3]
        extr    c, a, c, #63
        ldr     b, [t, j, lsl #3]
        sbcs    c, c, b
        str     c, [z, j, lsl #3]
        mov     c, a
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_dubloop1
        lsr     c, c, #63
        sbc     c, c, xzr
        adds    j, xzr, xzr
Lbignum_montifier_corrloop1:
        ldr     a, [z, j, lsl #3]
        ldr     b, [t, j, lsl #3]
        and     b, b, c
        adcs    a, a, b
        str     a, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_corrloop1

// This is not exactly the same: we also copy output to t giving the
// initialization t_1 = r == 2^{p+64} mod n for the main loop next.

        mov     c, xzr
        subs    j, xzr, xzr
Lbignum_montifier_dubloop2:
        ldr     a, [z, j, lsl #3]
        extr    c, a, c, #63
        ldr     b, [t, j, lsl #3]
        sbcs    c, c, b
        str     c, [z, j, lsl #3]
        mov     c, a
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_dubloop2
        lsr     c, c, #63
        sbc     c, c, xzr
        adds    j, xzr, xzr
Lbignum_montifier_corrloop2:
        ldr     a, [z, j, lsl #3]
        ldr     b, [t, j, lsl #3]
        and     b, b, c
        adcs    a, a, b
        str     a, [z, j, lsl #3]
        str     a, [t, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_corrloop2

// We then successively generate (k+1)-digit values satisfying
// t_i == 2^{p+64*i} mod n, each of which is stored in h::t. Finish
// initialization by zeroing h initially

        mov     h, xzr

// Then if t_i = 2^{p} * h + l
// we have t_{i+1} == 2^64 * t_i
//         = (2^{p+64} * h) + (2^64 * l)
//        == r * h + l<<64
// Do this 2*k more times so we end up == 2^{192*k+64}, one more than we want
//
// Writing B = 2^{64k}, the possible correction of adding r, which for
// a (k+1)-digit result is equivalent to subtracting q = 2^{64*(k+1)} - r
// would give the overall worst-case value minus q of
// [ B * (B^k - 1) + (B - 1) * r ] - [B^{k+1} - r]
// = B * (r - 1) < B^{k+1} so we keep inside k+1 digits as required.
//
// This implementation makes the shift implicit by starting b with the
// "previous" digit (initially 0) to offset things by 1.

        add     i, k, k
Lbignum_montifier_modloop:
        mov     j, xzr
        mov     b, xzr
        adds    c, xzr, xzr
Lbignum_montifier_cmaloop:
        ldr     a, [z, j, lsl #3]
        mul     l, h, a
        adcs    b, b, c
        umulh   c, h, a
        adc     c, c, xzr
        adds    l, b, l
        ldr     b, [t, j, lsl #3]
        str     l, [t, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_cmaloop

        adcs    h, b, c

        csetm   l, cs

        adds    j, xzr, xzr
Lbignum_montifier_oaloop:
        ldr     a, [t, j, lsl #3]
        ldr     b, [z, j, lsl #3]
        and     b, b, l
        adcs    a, a, b
        str     a, [t, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_oaloop
        adc     h, h, xzr

        subs    i, i, #1
        bne     Lbignum_montifier_modloop

// Compute the negated modular inverse w (same register as i, not used again).

        ldr     a, [m]
        lsl     w, a, #2
        sub     w, a, w
        eor     w, w, #2
        mov     l, #1
        madd    c, a, w, l
        mul     b, c, c
        madd    w, c, w, w
        mul     c, b, b
        madd    w, b, w, w
        mul     b, c, c
        madd    w, c, w, w
        madd    w, b, w, w

// Now do one almost-Montgomery reduction w.r.t. the original m
// which lops off one 2^64 from the congruence and, with the usual
// almost-Montgomery correction, gets us back inside k digits for
// the end result.

        ldr     b, [t]
        mul     d, b, w

        mul     l, d, a
        umulh   c, d, a
        mov     j, #1
        sub     a, k, #1
        adds    xzr, b, l
        cbz     a, Lbignum_montifier_amontend

Lbignum_montifier_amontloop:
        ldr     a, [m, j, lsl #3]
        ldr     b, [t, j, lsl #3]
        mul     l, d, a
        adcs    b, b, c
        umulh   c, d, a
        adc     c, c, xzr
        adds    b, b, l
        sub     a, j, #1
        str     b, [t, a, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_amontloop
Lbignum_montifier_amontend:
        adcs    h, h, c
        csetm   l, cs
        sub     a, k, #1
        str     h, [t, a, lsl #3]

        subs    j, xzr, xzr
Lbignum_montifier_osloop:
        ldr     a, [t, j, lsl #3]
        ldr     b, [m, j, lsl #3]
        and     b, b, l
        sbcs    a, a, b
        str     a, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_osloop

// So far, the code(basically a variant of bignum_amontifier) has produced
// a k-digit value z == 2^{192k} (mod m), not necessarily fully reduced mod m.
// We now do a short Montgomery reduction (similar to bignum_demont) so that
// we achieve full reduction mod m while lopping 2^{64k} off the congruence.
// We recycle h as the somewhat strangely-named outer loop counter.

        mov     h, k

Lbignum_montifier_montouterloop:
        ldr     b, [z]
        mul     d, b, w
        ldr     a, [m]
        mul     l, d, a
        umulh   c, d, a
        mov     j, #1
        sub     a, k, #1
        adds    xzr, b, l
        cbz     a, Lbignum_montifier_montend
Lbignum_montifier_montloop:
        ldr     a, [m, j, lsl #3]
        ldr     b, [z, j, lsl #3]
        mul     l, d, a
        adcs    b, b, c
        umulh   c, d, a
        adc     c, c, xzr
        adds    b, b, l
        sub     a, j, #1
        str     b, [z, a, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_montloop
Lbignum_montifier_montend:
        adc     c, c, xzr
        sub     a, k, #1
        str     c, [z, a, lsl #3]

        subs    h, h, #1
        bne     Lbignum_montifier_montouterloop

// Now do a comparison of z with m to set a final correction mask
// indicating that z >= m and so we need to subtract m.

        subs    j, xzr, xzr
Lbignum_montifier_cmploop:
        ldr     a, [z, j, lsl #3]
        ldr     b, [m, j, lsl #3]
        sbcs    xzr, a, b
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_cmploop
        csetm   h, cs

// Now do a masked subtraction of m for the final reduced result.

        subs    j, xzr, xzr
Lbignum_montifier_corrloop:
        ldr     a, [z, j, lsl #3]
        ldr     b, [m, j, lsl #3]
        and     b, b, h
        sbcs    a, a, b
        str     a, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montifier_corrloop

Lbignum_montifier_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montifier)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
