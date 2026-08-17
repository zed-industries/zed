// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Invert modulo m, z = (1/a) mod b, assuming b is an odd number > 1, coprime a
// Inputs a[k], b[k]; output z[k]; temporary buffer t[>=3*k]
//
//    extern void bignum_modinv(uint64_t k, uint64_t *z, const uint64_t *a,
//                              const uint64_t *b, uint64_t *t);
//
// k-digit (digit=64 bits) "z := a^-1 mod b" (modular inverse of a modulo b)
// using t as a temporary buffer (t at least 3*k words = 24*k bytes), and
// assuming that a and b are coprime *and* that b is an odd number > 1.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = a, X3 = b, X4 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modinv)
        .text
        .balign 4

// We get CHUNKSIZE bits per outer iteration, 64 minus a few for proxy errors

#define CHUNKSIZE 58

// Pervasive variables

#define k x0
#define z x1
#define b x3
#define w x4

// This one is recycled after initial copying in of a as outer loop counter

#define a x2
#define t x2

// Additional variables; later ones are currently rather high regs

#define l x5

#define m x21
#define n x22

// The matrix of update factors to apply to m and n
// Also used a couple of additional temporary variables for the swapping loop
// Also used as an extra down-counter in corrective negation loops

#define m_m x6
#define m_n x7
#define n_m x8
#define n_n x9

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

// Negated modular inverse for Montgomery

#define v x20

// Some more intuitive names for temp regs in initial word-level negmodinv.
// These just use t1 and t2 again, though carefully since t1 = initial b[0]

#define one t2
#define e1 t2
#define e2 t1
#define e4 t2
#define e8 t1

S2N_BN_SYMBOL(bignum_modinv):
        CFI_START

// We make use of registers beyond the modifiable

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)

// If k = 0 then do nothing (this is out of scope anyway)

        cbz     k, Lbignum_modinv_end

// Set up the additional two buffers m and n beyond w in temp space

        lsl     i, k, #3
        add     m, w, i
        add     n, m, i

// Initialize the main buffers with their starting values:
// m = a, n = b, w = b (to be tweaked to b - 1) and z = 0

        mov     i, xzr
Lbignum_modinv_copyloop:
        ldr     t1, [a, i, lsl #3]
        ldr     t2, [b, i, lsl #3]
        str     t1, [m, i, lsl #3]
        str     t2, [n, i, lsl #3]
        str     t2, [w, i, lsl #3]
        str     xzr, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_modinv_copyloop

// Tweak down w to b - 1 (this crude approach is safe as b needs to be odd
// for it to be in scope). We have then established the congruence invariant:
//
//   a * w == -m (mod b)
//   a * z == n (mod b)
//
// This, with the bound w <= b and z <= b, is maintained round the outer loop

        ldr     t1, [w]
        sub     t2, t1, #1
        str     t2, [w]

// Compute v = negated modular inverse of b mod 2^64, reusing t1 from above
// This is used for Montgomery reduction operations each time round the loop

        lsl     v, t1, #2
        sub     v, t1, v
        eor     v, v, #2
        mov     one, #1
        madd    e1, t1, v, one
        mul     e2, e1, e1
        madd    v, e1, v, v
        mul     e4, e2, e2
        madd    v, e2, v, v
        mul     e8, e4, e4
        madd    v, e4, v, v
        madd    v, e8, v, v

// Set up the outer loop count of 128 * k
// The invariant is that m * n < 2^t at all times.

        lsl     t, k, #7

// Start of the main outer loop iterated t / CHUNKSIZE times

Lbignum_modinv_outerloop:

// We need only bother with sharper l = min k (ceil(t/64)) digits
// for the computations on m and n (but we still need k for w and z).
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
Lbignum_modinv_toploop:
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
        bcc     Lbignum_modinv_toploop

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

Lbignum_modinv_innerloop:

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
        cbnz    i, Lbignum_modinv_innerloop

// Apply the update to w and z, using addition in this case, and also take
// the chance to shift an additional 6 = 64-CHUNKSIZE bits to be ready for a
// Montgomery multiplication. Because we know that m_m + m_n <= 2^58 and
// w, z <= b < 2^{64k}, we know that both of these fit in k+1 words.
// We do this before the m-n update to allow us to play with c1 and c2 here.
//
//    h1::w = 2^6 * (m_m * w + m_n * z)
//    h2::z = 2^6 * (n_m * w + n_n * z)
//
// with c1 and c2 recording previous words for the shifting part

        mov     h1, xzr
        mov     h2, xzr
        mov     c1, xzr
        mov     c2, xzr

        mov     i, xzr
Lbignum_modinv_congloop:
        ldr     t1, [w, i, lsl #3]
        ldr     t2, [z, i, lsl #3]

        mul     l1, m_m, t1
        mul     l2, m_n, t2
        adds    l1, l1, h1
        umulh   h1, m_m, t1
        adc     h1, h1, xzr
        adds    l1, l1, l2
        extr    c1, l1, c1, #CHUNKSIZE
        str     c1, [w, i, lsl #3]
        mov     c1, l1
        umulh   l1, m_n, t2
        adc     h1, h1, l1

        mul     l1, n_m, t1
        mul     l2, n_n, t2
        adds    l1, l1, h2
        umulh   h2, n_m, t1
        adc     h2, h2, xzr
        adds    l1, l1, l2
        extr    c2, l1, c2, #CHUNKSIZE
        str     c2, [z, i, lsl #3]
        mov     c2, l1
        umulh   l1, n_n, t2
        adc     h2, h2, l1

        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_modinv_congloop

        extr    h1, h1, c1, #CHUNKSIZE
        extr    h2, h2, c2, #CHUNKSIZE

// Do a Montgomery reduction of h1::w

        ldr     t1, [w]
        mul     c1, t1, v
        ldr     t2, [b]
        mul     l1, c1, t2
        umulh   l2, c1, t2
        adds    t1, t1, l1      // Will be zero but want the carry

        mov     i, #1
        sub     t1, k, #1
        cbz     t1, Lbignum_modinv_wmontend
Lbignum_modinv_wmontloop:
        ldr     t1, [b, i, lsl #3]
        ldr     t2, [w, i, lsl #3]
        mul     l1, c1, t1
        adcs    t2, t2, l2
        umulh   l2, c1, t1
        adc     l2, l2, xzr
        adds    t2, t2, l1
        sub     l1, i, #1
        str     t2, [w, l1, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_wmontloop
Lbignum_modinv_wmontend:
        adcs    l2, l2, h1
        adc     h1, xzr, xzr
        sub     l1, i, #1
        str     l2, [w, l1, lsl #3]

        subs    i, xzr, xzr
Lbignum_modinv_wcmploop:
        ldr     t1, [w, i, lsl #3]
        ldr     t2, [b, i, lsl #3]
        sbcs    xzr, t1, t2
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_wcmploop

        sbcs    xzr, h1, xzr
        csetm   h1, cs

        subs    i, xzr, xzr
Lbignum_modinv_wcorrloop:
        ldr     t1, [w, i, lsl #3]
        ldr     t2, [b, i, lsl #3]
        and     t2, t2, h1
        sbcs    t1, t1, t2
        str     t1, [w, i, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_wcorrloop

// Do a Montgomery reduction of h2::z

        ldr     t1, [z]
        mul     c1, t1, v
        ldr     t2, [b]
        mul     l1, c1, t2
        umulh   l2, c1, t2
        adds    t1, t1, l1      // Will be zero but want the carry

        mov     i, #1
        sub     t1, k, #1
        cbz     t1, Lbignum_modinv_zmontend
Lbignum_modinv_zmontloop:
        ldr     t1, [b, i, lsl #3]
        ldr     t2, [z, i, lsl #3]
        mul     l1, c1, t1
        adcs    t2, t2, l2
        umulh   l2, c1, t1
        adc     l2, l2, xzr
        adds    t2, t2, l1
        sub     l1, i, #1
        str     t2, [z, l1, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_zmontloop
Lbignum_modinv_zmontend:
        adcs    l2, l2, h2
        adc     h2, xzr, xzr
        sub     l1, i, #1
        str     l2, [z, l1, lsl #3]

        subs    i, xzr, xzr
Lbignum_modinv_zcmploop:
        ldr     t1, [z, i, lsl #3]
        ldr     t2, [b, i, lsl #3]
        sbcs    xzr, t1, t2
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_zcmploop

        sbcs    xzr, h2, xzr
        csetm   h2, cs

        subs    i, xzr, xzr
Lbignum_modinv_zcorrloop:
        ldr     t1, [z, i, lsl #3]
        ldr     t2, [b, i, lsl #3]
        and     t2, t2, h2
        sbcs    t1, t1, t2
        str     t1, [z, i, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_zcorrloop

// Now actually compute the updates to m and n corresponding to the matrix,
// and correct the signs if they have gone negative. First we compute the
// (k+1)-sized updates with the following invariant (here c1 and c2 are in
// fact carry bitmasks, either 0 or -1):
//
//    c1::h1::m = m_m * m - m_n * n
//    c2::h2::n = n_m * m - n_n * n

        mov     h1, xzr
        mov     h2, xzr
        mov     c1, xzr
        mov     c2, xzr
        mov     i, xzr
Lbignum_modinv_crossloop:
        ldr     t1, [m, i, lsl #3]
        ldr     t2, [n, i, lsl #3]

        mul     l1, m_m, t1
        mul     l2, m_n, t2
        adds    l1, l1, h1
        umulh   h1, m_m, t1
        adc     h1, h1, xzr
        subs    l1, l1, l2
        str     l1, [m, i, lsl #3]
        umulh   l1, m_n, t2
        sub     c1, l1, c1
        sbcs    h1, h1, c1
        csetm   c1, cc

        mul     l1, n_m, t1
        mul     l2, n_n, t2
        adds    l1, l1, h2
        umulh   h2, n_m, t1
        adc     h2, h2, xzr
        subs    l1, l1, l2
        str     l1, [n, i, lsl #3]
        umulh   l1, n_n, t2
        sub     c2, l1, c2
        sbcs    h2, h2, c2
        csetm   c2, cc

        add     i, i, #1
        cmp     i, l
        bcc     Lbignum_modinv_crossloop

// Write back m optionally negated and shifted right CHUNKSIZE bits

        adds    xzr, c1, c1

        ldr     l1, [m]
        mov     i, xzr
        sub     j, l, #1
        cbz     j, Lbignum_modinv_negskip1

Lbignum_modinv_negloop1:
        add     t1, i, #8
        ldr     t2, [m, t1]
        extr    l1, t2, l1, #CHUNKSIZE
        eor     l1, l1, c1
        adcs    l1, l1, xzr
        str     l1, [m, i]
        mov     l1, t2
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modinv_negloop1
Lbignum_modinv_negskip1:
        extr    l1, h1, l1, #CHUNKSIZE
        eor     l1, l1, c1
        adcs    l1, l1, xzr
        str     l1, [m, i]

// Write back n optionally negated and shifted right CHUNKSIZE bits

        adds    xzr, c2, c2

        ldr     l1, [n]
        mov     i, xzr
        sub     j, l, #1
        cbz     j, Lbignum_modinv_negskip2
Lbignum_modinv_negloop2:
        add     t1, i, #8
        ldr     t2, [n, t1]
        extr    l1, t2, l1, #CHUNKSIZE
        eor     l1, l1, c2
        adcs    l1, l1, xzr
        str     l1, [n, i]
        mov     l1, t2
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modinv_negloop2
Lbignum_modinv_negskip2:
        extr    l1, h2, l1, #CHUNKSIZE
        eor     l1, l1, c2
        adcs    l1, l1, xzr
        str     l1, [n, i]

// Finally, use the signs c1 and c2 to do optional modular negations of
// w and z respectively, flipping c2 to make signs work. We don't make
// any checks for zero values, but we certainly retain w <= b and z <= b.
// This is enough for the Montgomery step in the next iteration to give
// strict reduction w < b amd z < b, and anyway when we terminate we
// could not have z = b since it violates the coprimality assumption for
// in-scope cases.

        mov     i, xzr
        adds    xzr, c1, c1
Lbignum_modinv_wfliploop:
        ldr     t1, [b, i, lsl #3]
        ldr     t2, [w, i, lsl #3]
        and     t1, t1, c1
        eor     t2, t2, c1
        adcs    t1, t1, t2
        str     t1, [w, i, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_wfliploop

        mvn     c2, c2

        mov     i, xzr
        adds    xzr, c2, c2
Lbignum_modinv_zfliploop:
        ldr     t1, [b, i, lsl #3]
        ldr     t2, [z, i, lsl #3]
        and     t1, t1, c2
        eor     t2, t2, c2
        adcs    t1, t1, t2
        str     t1, [z, i, lsl #3]
        add     i, i, #1
        sub     t1, i, k
        cbnz    t1, Lbignum_modinv_zfliploop

// End of main loop. We can stop if t' <= 0 since then m * n < 2^0, which
// since n is odd and m and n are coprime (in the in-scope cases) means
// m = 0, n = 1 and hence from the congruence invariant a * z == 1 (mod b).
// Moreover we do in fact need to maintain strictly t > 0 in the main loop,
// or the computation of the optimized digit bound l could collapse to 0.

        subs    t, t, #CHUNKSIZE
        bhi     Lbignum_modinv_outerloop

Lbignum_modinv_end:
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
