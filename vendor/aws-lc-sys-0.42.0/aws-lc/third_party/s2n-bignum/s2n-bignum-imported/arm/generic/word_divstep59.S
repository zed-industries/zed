// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Perform 59 "divstep" iterations and return signed matrix of updates
// Inputs d, f, g; output m[2][2] and function return (updated d)
//
// extern int64_t word_divstep59
//  (int64_t m[2][2],int64_t d,uint64_t f,uint64_t g);
//
// Standard ARM ABI: X0 = m, X1 = d, X2 = f, X3 = g, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_divstep59)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_divstep59)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_divstep59)
        .text
        .balign 4

#define m x0
#define d x1
#define f x2
#define g x3

#define fuv x4
#define grs x5
#define t x6
#define n x7

#define m00 x8
#define m01 x9
#define m10 x10
#define m11 x11

#define n00 x12
#define n01 x13
#define n10 x14
#define n11 x15

S2N_BN_SYMBOL(word_divstep59):
        CFI_START

// Pack f and g into single registers with (negated) update matrices,
// initially the identity matrix. The f_lo and g_lo are initially
// the 20 lowest bits of f and g.
//
// fuv = f_lo - 2^41 * 1 - 2^62 * 0
// grs = g_lo - 2^41 * 0 - 2^62 * 1

        and     fuv, f, #0xFFFFF
        orr     fuv, fuv, 0xFFFFFE0000000000

        and     grs, g, #0xFFFFF
        orr     grs, grs, 0xc000000000000000

        tst     grs, #1

// Now do 20 divsteps on that packed format.
//
// At the i'th iteration (starting at i = 0, ending at i = 20)
// the intermediate packed values are of the form
//
// fuv = f_lo - 2^{41-i} * m00 - 2^{62-i} * m01
// grs = g_lo - 2^{41-i} * m10 - 2^{62-i} * m11
//
// where the following matrix indicates the updates to apply
// to the original (full-sized) f and g for those iterations.
//
// [m00 m01] * [f_0] = [f_i]
// [m10 m11]   [g_0]   [g_i]

.set i, 0
.rep 20

        csel    t, fuv, xzr, ne
        ccmp    d, xzr, #8, ne

        cneg    d, d, ge
        cneg    t, t, ge
        csel    fuv, grs, fuv, ge

        add     grs, grs, t
        add     d, d, #2

.if (i< 19)
        tst     grs, #2
.endif
        asr     grs, grs, #1

.set i, (i+1)
.endr

// Extract the matrix entries, but keep them in negated form.

        add     m00, fuv, #1048576
        sbfx    m00, m00, #21, #21

        mov     m11, #1048576
        add     m11, m11, m11, lsl #21
        add     m01, fuv, m11
        asr     m01, m01, #42

        add     m10, grs, #1048576
        sbfx    m10, m10, #21, #21

        add     m11, grs, m11
        asr     m11, m11, #42

// Compute updated f and g using the negated matrix entries;
// this flips the signs of f and g but it doesn't matter.
//
//   f = (m00 * f + m01 * g) / 2^20
//   g = (m10 * f + m11 * g) / 2^20
//
// Since we only need another 40 bits, we can do all of that
// computation naively using (implicitly signed) 64-bit words.

        mul     t, m00, f
        mul     n, m01, g
        mul     f, m10, f
        mul     g, m11, g

        add     fuv, t, n
        add     grs, f, g

        asr     f, fuv, #20
        asr     g, grs, #20

// Re-pack for 20 more rounds

        and     fuv, f, #0xFFFFF
        orr     fuv, fuv, 0xFFFFFE0000000000

        and     grs, g, #0xFFFFF
        orr     grs, grs, 0xc000000000000000

        tst     grs, #1

// Second block of 20 divsteps in the same style

.set i, 0
.rep 20

        csel    t, fuv, xzr, ne
        ccmp    d, xzr, #8, ne

        cneg    d, d, ge
        cneg    t, t, ge
        csel    fuv, grs, fuv, ge

        add     grs, grs, t
        add     d, d, #2

.if (i< 19)
        tst     grs, #2
.endif
        asr     grs, grs, #1

.set i, (i+1)
.endr

// Extract the next matrix entries, in negated form again

        add     n00, fuv, #1048576
        sbfx    n00, n00, #21, #21

        mov     n11, #1048576
        add     n11, n11, n11, lsl #21
        add     n01, fuv, n11
        asr     n01, n01, #42

        add     n10, grs, #1048576
        sbfx    n10, n10, #21, #21

        add     n11, grs, n11
        asr     n11, n11, #42

// Compute updated f and g using the negated matrix entries,
// and so again flipping (thus actually restoring) the signs.
//
//   f = (n00 * f + n01 * g) / 2^20
//   g = (n10 * f + n11 * g) / 2^20

        mul     t, n00, f
        mul     n, n01, g
        mul     f, n10, f
        mul     g, n11, g

        add     fuv, t, n
        add     grs, f, g

        asr     f, fuv, #20
        asr     g, grs, #20

// Re-pack for 19 more rounds

        and     fuv, f, #0xFFFFF
        orr     fuv, fuv, 0xFFFFFE0000000000

        and     grs, g, #0xFFFFF
        orr     grs, grs, 0xc000000000000000

        tst     grs, #1

// Split the last divsteps into two blocks of 10 and 9 to insert the matrix
// multiplication in between them. The first ten iterations:

.set i, 0
.rep 10

        csel    t, fuv, xzr, ne
        ccmp    d, xzr, #8, ne

        cneg    d, d, ge
        cneg    t, t, ge
        csel    fuv, grs, fuv, ge

        add     grs, grs, t
        add     d, d, #2

        tst     grs, #2
        asr     grs, grs, #1

.set i, (i+1)
.endr

// Multiply the first two matrices.
//
// [m00  m01] = [n00  n01] * [m00  m01]
// [m10  m11]   [n10  n11]   [m10  m11]
//
// The resulting matrix entries are:
//
//   m00' = n00 * m00 + n01 * m10
//   m01' = n00 * m01 + n01 * m11
//   m10' = n10 * m00 + n11 * m10
//   m11' = n10 * m01 + n11 * m11

        mul     f, n00, m00
        mul     g, n00, m01
        mul     t, n10, m00
        mul     n, n10, m01

        madd    m00, n01, m10, f
        madd    m01, n01, m11, g
        madd    m10, n11, m10, t
        madd    m11, n11, m11, n

// Now the final 9 divsteps

.rep 9

        csel    t, fuv, xzr, ne
        ccmp    d, xzr, #8, ne

        cneg    d, d, ge
        cneg    t, t, ge
        csel    fuv, grs, fuv, ge

        add     grs, grs, t
        add     d, d, #2

.if (i< 18)
        tst     grs, #2
.endif
        asr     grs, grs, #1

.set i, (i+1)
.endr

// Extract the matrix entries from the final 19 divsteps

        add     n00, fuv, #1048576
        sbfx    n00, n00, #22, #21

        mov     n11, #1048576
        add     n11, n11, n11, lsl #21
        add     n01, fuv, n11
        asr     n01, n01, #43

        add     n10, grs, #1048576
        sbfx    n10, n10, #22, #21

        add     n11, grs, n11
        asr     n11, n11, #43

// Multiply by this new matrix
//
// [m00  m01] = [n00  n01] * [m00  m01]
// [m10  m11]   [n10  n11]   [m10  m11]
//
// The resulting matrix entries are:
//
//   m00' = n00 * m00 + n01 * m10
//   m01' = n00 * m01 + n01 * m11
//   m10' = n10 * m00 + n11 * m10
//   m11' = n10 * m01 + n11 * m11
//
// Since we didn't negate the n matrix, all products are negated
// here using "mneg" and "msub" in place of "mul" and "madd", so
// we have the correct sign for the returned composite matrix.

        mneg    f, n00, m00
        mneg    g, n00, m01
        mneg    fuv, n10, m00
        mneg    grs, n10, m01

        msub    m00, n01, m10, f
        msub    m01, n01, m11, g
        msub    m10, n11, m10, fuv
        msub    m11, n11, m11, grs

// Finally store back and return final d.

        stp     m00, m01, [m]
        stp     m10, m11, [m, #16]

        mov     x0, d

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_divstep59)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
