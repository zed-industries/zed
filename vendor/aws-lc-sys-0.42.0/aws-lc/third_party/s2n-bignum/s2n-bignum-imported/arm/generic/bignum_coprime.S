// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignums for coprimality, gcd(x,y) = 1
// Inputs x[m], y[n]; output function return; temporary buffer t[>=2*max(m,n)]
//
//    extern uint64_t bignum_coprime(uint64_t m, const uint64_t *x, uint64_t n,
//                                   const uint64_t *y, uint64_t *t);
//
// Test for whether two bignums are coprime (no common factor besides 1).
// This is equivalent to testing if their gcd is 1, but a bit faster than
// doing those two computations separately.
//
// Here bignum x is m digits long, y is n digits long and the temporary
// buffer t needs to be 2 * max(m,n) digits long. The return value is
// 1 if coprime(x,y) and 0 otherwise.
//
// Standard ARM ABI: X0 = m, X1 = x, X2 = n, X3 = y, X4 = t, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_coprime)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_coprime)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_coprime)
        .text
        .balign 4

#define CHUNKSIZE 58

// Pervasive variables

#define k x9
#define m x4
#define n x5

// Used via parameters in copy-in loop, then re-used as outer loop
// counter t and adaptive precision digit size l, which becomes a
// reduced version of k in later iterations but starts at l = k

#define x x1
#define y x3

#define t x2
#define l x3

// The matrix of update factors to apply to m and n
// Also used a couple of additional temporary variables for the swapping loop
// Also used as an extra down-counter in corrective negation loops

#define m_m x6
#define m_n x7
#define n_m x8
#define n_n x1

#define t3 x6
#define t4 x7

#define j x6

// General temporary variables and loop counters

#define i x10
#define t1 x11
#define t2 x12

// High and low proxies for the inner loop
// Then re-used for high and carry words during actual cross-multiplications

#define m_hi x13
#define n_hi x14
#define m_lo x15
#define n_lo x16

#define h1 x13
#define h2 x14
#define l1 x15
#define l2 x16

#define c1 x17
#define c2 x19
#define tt x20


S2N_BN_SYMBOL(bignum_coprime):
        CFI_START

// We make use of just a couple of additional registers

        CFI_PUSH2(x19,x20)

// Compute k = max(m,n), and if this is zero skip to the end. Note that
// in this case x0 = m = 0 so we return the right answer of "false"

        cmp     x0, x2
        csel    k, x2, x0, cc
        cbz     k, Lbignum_coprime_end

// Set up inside w two size-k buffers m and n

        lsl     i, k, #3
        add     n, m, i

// Copy the input x into the buffer m, padding with zeros as needed

        mov     i, xzr
        cbz     x0, Lbignum_coprime_xpadloop
Lbignum_coprime_xloop:
        ldr     t1, [x1, i, lsl #3]
        str     t1, [m, i, lsl #3]
        add     i, i, #1
        cmp     i, x0
        bcc     Lbignum_coprime_xloop
        cmp     i, k
        bcs     Lbignum_coprime_xskip
Lbignum_coprime_xpadloop:
        str     xzr, [m, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_coprime_xpadloop
Lbignum_coprime_xskip:

// Copy the input y into the buffer n, padding with zeros as needed

        mov     i, xzr
        cbz     x2, Lbignum_coprime_ypadloop
Lbignum_coprime_yloop:
        ldr     t1, [x3, i, lsl #3]
        str     t1, [n, i, lsl #3]
        add     i, i, #1
        cmp     i, x2
        bcc     Lbignum_coprime_yloop
        cmp     i, k
        bcs     Lbignum_coprime_yskip
Lbignum_coprime_ypadloop:
        str     xzr, [n, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_coprime_ypadloop
Lbignum_coprime_yskip:

// Set up the outer loop count of 64 * sum of input sizes.
// The invariant is that m * n < 2^t at all times.

        add     t, x0, x2
        lsl     t, t, #6

// Record for the very end the OR of the lowest words.
// If the bottom bit is zero we know both are even so the answer is false.
// But since this is constant-time code we still execute all the main part.

        ldr     x0, [m]
        ldr     t3, [n]
        orr     x0, x0, t3

// Now if n is even trigger a swap of m and n. This ensures that if
// one or other of m and n is odd then we make sure now that n is,
// as expected by our invariant later on.

        and     t3, t3, #1
        sub     t3, t3, #1

        mov     i, xzr
Lbignum_coprime_swaploop:
        ldr     t1, [m, i, lsl #3]
        ldr     t2, [n, i, lsl #3]
        eor     t4, t1, t2
        and     t4, t4, t3
        eor     t1, t1, t4
        eor     t2, t2, t4
        str     t1, [m, i, lsl #3]
        str     t2, [n, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_coprime_swaploop

// Start of the main outer loop iterated t / CHUNKSIZE times

Lbignum_coprime_outerloop:

// We need only bother with sharper l = min k (ceil(t/64)) digits
// Either both m and n fit in l digits, or m has become zero and so
// nothing happens in the loop anyway and this makes no difference.

        add     i, t, #63
        lsr     l, i, #6
        cmp     l, k
        csel    l, k, l, cs

// Select upper and lower proxies for both m and n to drive the inner
// loop. The lower proxies are simply the lowest digits themselves,
// m_lo = m[0] and n_lo = n[0], while the upper proxies are bitfields
// of the two inputs selected so their top bit (63) aligns with the
// most significant bit of *either* of the two inputs.

        mov     h1, xzr // Previous high and low for m
        mov     l1, xzr
        mov     h2, xzr // Previous high and low for n
        mov     l2, xzr
        mov     c2, xzr // Mask flag: previous word of one was nonzero
        // and in this case h1 and h2 are those words

        mov     i, xzr
Lbignum_coprime_toploop:
        ldr     t1, [m, i, lsl #3]
        ldr     t2, [n, i, lsl #3]
        orr     c1, t1, t2
        cmp     c1, xzr
        and     c1, c2, h1
        csel    l1, c1, l1, ne
        and     c1, c2, h2
        csel    l2, c1, l2, ne
        csel    h1, t1, h1, ne
        csel    h2, t2, h2, ne
        csetm   c2, ne
        add     i, i, #1
        cmp     i, l
        bcc     Lbignum_coprime_toploop

        orr     t1, h1, h2
        clz     t2, t1
        negs    c1, t2
        lsl     h1, h1, t2
        csel    l1, l1, xzr, ne
        lsl     h2, h2, t2
        csel    l2, l2, xzr, ne
        lsr     l1, l1, c1
        lsr     l2, l2, c1
        orr     m_hi, h1, l1
        orr     n_hi, h2, l2

        ldr     m_lo, [m]
        ldr     n_lo, [n]

// Now the inner loop, with i as loop counter from CHUNKSIZE down.
// This records a matrix of updates to apply to the initial
// values of m and n with, at stage j:
//
//     sgn * m' = (m_m * m - m_n * n) / 2^j
//    -sgn * n' = (n_m * m - n_n * n) / 2^j
//
// where "sgn" is either +1 or -1, and we lose track of which except
// that both instance above are the same. This throwing away the sign
// costs nothing (since we have to correct in general anyway because
// of the proxied comparison) and makes things a bit simpler. But it
// is simply the parity of the number of times the first condition,
// used as the swapping criterion, fires in this loop.

        mov     m_m, #1
        mov     m_n, xzr
        mov     n_m, xzr
        mov     n_n, #1

        mov     i, #CHUNKSIZE

// Conceptually in the inner loop we follow these steps:
//
// * If m_lo is odd and m_hi < n_hi, then swap the four pairs
//    (m_hi,n_hi); (m_lo,n_lo); (m_m,n_m); (m_n,n_n)
//
// * Now, if m_lo is odd (old or new, doesn't matter as initial n_lo is odd)
//    m_hi := m_hi - n_hi, m_lo := m_lo - n_lo
//    m_m  := m_m + n_m, m_n := m_n + n_n
//
// * Halve and double them
//     m_hi := m_hi / 2, m_lo := m_lo / 2
//     n_m := n_m * 2, n_n := n_n * 2
//
// The actual computation computes updates before actually swapping and
// then corrects as needed. It also maintains the invariant ~ZF <=> odd(m_lo),
// since it seems to reduce the dependent latency. Set that up first.

        ands    xzr, m_lo, #1

Lbignum_coprime_innerloop:

// At the start of the loop ~ZF <=> m_lo is odd; mask values accordingly
// Set the flags for m_hi - [~ZF] * n_hi so we know to flip things.

        csel    t1, n_hi, xzr, ne
        csel    t2, n_lo, xzr, ne
        csel    c1, n_m, xzr, ne
        csel    c2, n_n, xzr, ne
        ccmp    m_hi, n_hi, #0x2, ne

// Compute subtractive updates, trivial in the case ZF <=> even(m_lo).

        sub     t1, m_hi, t1
        sub     t2, m_lo, t2

// If the subtraction borrows, swap things appropriately, negating where
// we've already subtracted so things are as if we actually swapped first.

        csel    n_hi, n_hi, m_hi, cs
        cneg    t1, t1, cc
        csel    n_lo, n_lo, m_lo, cs
        cneg    m_lo, t2, cc
        csel    n_m, n_m, m_m, cs
        csel    n_n, n_n, m_n, cs

// Update and shift while setting oddness flag for next iteration
// We look at bit 1 of t2 (m_lo before possible negation), which is
// safe because it is even.

        ands    xzr, t2, #2
        add     m_m, m_m, c1
        add     m_n, m_n, c2
        lsr     m_hi, t1, #1
        lsr     m_lo, m_lo, #1
        add     n_m, n_m, n_m
        add     n_n, n_n, n_n

// Next iteration; don't disturb the flags since they are used at entry

        sub     i, i, #1
        cbnz    i, Lbignum_coprime_innerloop

// Now actually compute the updates to m and n corresponding to that matrix,
// and correct the signs if they have gone negative. First we compute the
// (k+1)-sized updates
//
//    c1::h1::m = m_m * m - m_n * n
//    c2::h2::n = n_m * m - n_n * n
//
// then for each one, sign-correct and shift by CHUNKSIZE

        mov     h1, xzr
        mov     h2, xzr
        mov     c1, xzr
        mov     c2, xzr
        mov     i, xzr
Lbignum_coprime_crossloop:
        ldr     t1, [m, i, lsl #3]
        ldr     t2, [n, i, lsl #3]

        mul     l1, m_m, t1
        mul     l2, m_n, t2
        adds    l1, l1, h1
        umulh   h1, m_m, t1
        adc     h1, h1, xzr
        umulh   tt, m_n, t2
        sub     c1, tt, c1
        subs    l1, l1, l2
        str     l1, [m, i, lsl #3]
        sbcs    h1, h1, c1
        csetm   c1, cc

        mul     l1, n_m, t1
        mul     l2, n_n, t2
        adds    l1, l1, h2
        umulh   h2, n_m, t1
        adc     h2, h2, xzr
        umulh   tt, n_n, t2
        sub     c2, tt, c2
        subs    l1, l1, l2
        str     l1, [n, i, lsl #3]
        sbcs    h2, h2, c2
        csetm   c2, cc

        add     i, i, #1
        cmp     i, l
        bcc     Lbignum_coprime_crossloop

// Write back m optionally negated and shifted right CHUNKSIZE bits

        adds    xzr, c1, c1

        ldr     l1, [m]
        mov     i, xzr
        sub     j, l, #1
        cbz     j, Lbignum_coprime_negskip1

Lbignum_coprime_negloop1:
        add     t1, i, #8
        ldr     t2, [m, t1]
        extr    l1, t2, l1, #CHUNKSIZE
        eor     l1, l1, c1
        adcs    l1, l1, xzr
        str     l1, [m, i]
        mov     l1, t2
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_coprime_negloop1
Lbignum_coprime_negskip1:
        extr    l1, h1, l1, #CHUNKSIZE
        eor     l1, l1, c1
        adcs    l1, l1, xzr
        str     l1, [m, i]

// Write back n optionally negated and shifted right CHUNKSIZE bits

        adds    xzr, c2, c2

        ldr     l1, [n]
        mov     i, xzr
        sub     j, l, #1
        cbz     j, Lbignum_coprime_negskip2
Lbignum_coprime_negloop2:
        add     t1, i, #8
        ldr     t2, [n, t1]
        extr    l1, t2, l1, #CHUNKSIZE
        eor     l1, l1, c2
        adcs    l1, l1, xzr
        str     l1, [n, i]
        mov     l1, t2
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_coprime_negloop2
Lbignum_coprime_negskip2:
        extr    l1, h2, l1, #CHUNKSIZE
        eor     l1, l1, c2
        adcs    l1, l1, xzr
        str     l1, [n, i]

// End of main loop. We can stop if t' <= 0 since then m * n < 2^0, which
// since n is odd (in the main cases where we had one or other input odd)
// means that m = 0 and n is the final gcd. Moreover we do in fact need to
// maintain strictly t > 0 in the main loop, or the computation of the
// optimized digit bound l could collapse to 0.

        subs    t, t, #CHUNKSIZE
        bhi     Lbignum_coprime_outerloop

// Now compare n with 1 (OR of the XORs in t1)

        ldr     t1, [n]
        eor     t1, t1, #1
        cmp     k, #1
        beq     Lbignum_coprime_finalcomb
        mov     i, #1
Lbignum_coprime_compareloop:
        ldr     t2, [n, i, lsl #3]
        orr     t1, t1, t2
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_coprime_compareloop

// Now combine that with original oddness flag, which is still in x0

Lbignum_coprime_finalcomb:
        cmp     t1, xzr
        cset    t1, eq
        and     x0, x0, t1

Lbignum_coprime_end:
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_coprime)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
