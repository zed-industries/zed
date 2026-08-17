// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Find bignum modulo a single word
// Input x[k], m; output function return
//
//    extern uint64_t bignum_cmod(uint64_t k, const uint64_t *x, uint64_t m);
//
// Returns x mod m, assuming m is nonzero.
//
// Standard ARM ABI: X0 = k, X1 = x, X2 = m, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmod)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmod)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmod)
        .text
        .balign 4

#define k x0
#define x x1
#define m x2

#define e x3
#define n x4
#define w x5

#define r x6
#define h x7
#define l x8
#define a x9
#define d x10

// We re-use the k argument for a quotient estimate when it is no longer
// needed for traversal (x0 is modified for the return value anyway).

#define q x0

S2N_BN_SYMBOL(bignum_cmod):
        CFI_START

// If the bignum is zero-length, x0 is already the right answer of 0

        cbz     k, Lbignum_cmod_end

// Find number of leading zeros of m and let n = 2^e m so that for an
// in-scope (nonzero) input m we have n >= 2^63, e <= 63.

        clz     e, m
        lsl     n, m, e

// A near-clone of word_recip so 2^64 + w = ceil(2^128 / n) - 1

        lsr     a, n, #16
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
        mul     d, n, w
        umulh   r, n, w
        extr    d, r, d, #60
        lsr     r, w, #33
        mvn     d, d
        mul     d, r, d
        lsl     w, w, #1
        lsr     d, d, #33
        add     w, w, d
        adds    d, w, #0x1
        cinv    d, d, eq
        umulh   r, n, d
        adds    xzr, r, n
        csel    w, w, d, cs

// Take the residue r = 2^128 - (2^64 + w) * n, which by the above bound
// we know fits in 64 bits. We know 2^128 == r (mod n) and hence (mod m).

        mneg    r, w, n

// Now just go down through the digits accumulating [h;l] == x (mod n)
// by 2^64 * [h;l] + d = 2^128 * h + [l;d] == r * h + [l; d]. That addition
// may overflow with a carry, say 2^128 + [h';l'] = r * h + [l; d], in
// which case we subtract 2^128 - r (which is divisible by m and keeping
// things in 128 bits we just add r). Thus the overall bound when we initially
// overflow is r * h + [l; d] - (2^128 - r) = r * (h + 1) + [l; d] - 2^128
// < 2^128 so we stay inside 2 words

        mov     h, xzr
        mov     l, xzr
Lbignum_cmod_loop:
        sub     k, k, #1
        ldr     d, [x, k, lsl #3]
        mul     a, r, h
        umulh   h, r, h
        adds    a, a, d
        adcs    h, h, l
        csel    l, r, xzr, cs
        adds    l, l, a
        adc     h, h, xzr
        cbnz    k, Lbignum_cmod_loop

// Now do reciprocal multiplication to reduce the 2-word modular equivalent
// [h;l] to the single word l. If we assume the truncations are as follows
//   2^64 + w = 2^128 / n - epsilon (0 <= epsilon <= 1)
//   q = (w * h / 2^64) - delta (0 <= delta <= 1)
// the net remainder is l + (h/2^64 * epsilon + delta) * n < l + 2 * n.
// In general this needs two rounds of comparison to guarantee getting
// into a single word (though one more mul could be used instead).
// Also, the quotient estimate can overflow so we use r as extra addend
// 2^64 * n when the initial addition overflows. The overall multiple
// of n can't itself overflow, since we know it's an underestimate of
// the initial residue.

        umulh   q, w, h
        adds    q, q, h
        csel    r, n, xzr, cs

        mul     a, q, n
        umulh   d, q, n
        add     d, d, r
        subs    l, l, a
        sbcs    h, h, d

        csel    a, n, xzr, ne
        subs    l, l, a
        sbcs    h, h, xzr

        csel    a, n, xzr, ne
        sub     l, l, a

// One more reciprocal multiplication to do a modular reduction, but now in
// one word and in terms of the original m. For the quotient estimate we want
// q = ((2^64 + w) * l) / 2^{128-e} = ((2^64 + w) * l) / 2^65 / 2^{63-e}.

        umulh   q, w, l
        adds    q, q, l
        cset    r, cs
        extr    q, r, q, #1

        eor     e, e, #63
        lsr     q, q, e

        mul     a, q, m
        sub     l, l, a

// Note that since there is no neglected "low" part of the single word,
// one round of correction suffices; in the analog of the above l = 0
// and hence the residue so far is already < 2 * m.

        subs    x0, l, m
        csel    x0, x0, l, cs

Lbignum_cmod_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmod)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
