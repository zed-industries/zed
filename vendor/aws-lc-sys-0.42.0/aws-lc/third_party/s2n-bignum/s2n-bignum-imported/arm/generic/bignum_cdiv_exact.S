// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Divide by a single word, z := x / m *when known to be exact*
// Inputs x[n], m; output z[k]
//
//    extern void bignum_cdiv_exact(uint64_t k, uint64_t *z, uint64_t n,
//                                  const uint64_t *x, uint64_t m);
//
// Does the "z := x / m" operation where x is n digits and result z is k,
// *assuming* that m is nonzero and that the input x is in fact an
// exact multiple of m. (If this isn't known, use the general bignum_cdiv
// function instead.) In general the result is truncated to k digits.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x, X4 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cdiv_exact)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cdiv_exact)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cdiv_exact)
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

S2N_BN_SYMBOL(bignum_cdiv_exact):
        CFI_START

// If k = 0 then there's nothing to be done

        cbz     k, Lbignum_cdiv_exact_end

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

// Consider x' = x + m and do a Montgomery reduction, keeping the cofactor z.
// This gives us x' + m * z = 2^{64k} * c where c <= m. Assuming x = m * y
// we then have m * y + m + m * z = 2^{64k} * c, i.e.
//
//     m * (y + z + 1) = 2^{64k} * c
//
// This means m * (y + z + 1) == 0 (mod 2^{64k}), even when we truncate
// x to k digits (if in fact k < n). Since m is odd, it's coprime to
// 2^{64k} so we can cancel and get y + z + 1 == 0 (mod 2^{64k}), and
// hence using logical complement y == ~z (mod 2^{64k}). Thus we can
// write back the logical complements of the cofactor as the answer.
// Start with carry word c = m to make the initial tweak x' = x + m.

        mov     c, m
        mov     i, xzr

// Unless n = 0, preload the zeroth digit shifted right e places and bump
// up the x pointer by 8 and n down by 1, to ease indexing and comparison
// using the same variable i in the main loop. When n = 0 we leave it alone,
// as the comparison i < n will always fail and the x pointer is unused.

        mov     d, xzr
        cbz     n, Lbignum_cdiv_exact_loop
        ldr     d, [x], #8
        lsr     d, d, e
        sub     n, n, 1

Lbignum_cdiv_exact_loop:

// Load the next digit up to get [l,d] then shift right e places,
// eventually setting d back to the other part of the newly loaded digit
// ready for the next time round the loop.

        mov     l, xzr
        cmp     i, n
        bcs     Lbignum_cdiv_exact_noload
        ldr     l, [x, i, lsl #3]
Lbignum_cdiv_exact_noload:
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
        bcc     Lbignum_cdiv_exact_loop

Lbignum_cdiv_exact_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cdiv_exact)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
