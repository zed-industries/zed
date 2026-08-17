// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Compute "amontification" constant z :== 2^{128k} (congruent mod m)
// Input m[k]; output z[k]; temporary buffer t[>=k]
//
//    extern void bignum_amontifier(uint64_t k, uint64_t *z, const uint64_t *m,
//                                  uint64_t *t);
//
// This is called "amontifier" because any other value x can now be mapped into
// the almost-Montgomery domain with an almost-Montgomery multiplication by z.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = m, RCX = t
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = m, R9 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_amontifier)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_amontifier)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_amontifier)
        .text

#define k %rdi
#define z %rsi

// These two inputs get moved to different places since RCX and RDX are special

#define m %r12
#define t %r13

// Other variables
// Matters that c is RCX as CL=lo(c) is assumed in shifts

#define i %rbx
#define j %rbp
#define a %rax
#define c %rcx
#define h %r11
#define l %r10
#define b %r9
#define n %r8

// Some aliases for the values b and n

#define q %r8
#define r %r9

#define ashort %eax
#define ishort %ebx
#define jshort %ebp
#define qshort %r8d


S2N_BN_SYMBOL(bignum_amontifier):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
#endif

// Save some additional registers for use, copy args out of RCX and RDX

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)

        movq    %rdx, m
        movq    %rcx, t

// If k = 0 the whole operation is trivial

        testq   k, k
        jz      Lbignum_amontifier_end

// Copy the input m into the temporary buffer t. The temporary register
// c matters since we want it to hold the highest digit, ready for the
// normalization phase.

        xorq    i, i
Lbignum_amontifier_copyinloop:
        movq    (m,i,8), c
        movq    c, (t,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_amontifier_copyinloop

// Do a rather stupid but constant-time digit normalization, conditionally
// shifting left (k-1) times based on whether the top word is zero.
// With careful binary striding this could be O(k*log(k)) instead of O(k^2)
// while still retaining the constant-time style.
// The "neg c" sets the zeroness predicate (~CF) for the entire inner loop

        movq    k, i
        decq    i
        jz      Lbignum_amontifier_normalized
Lbignum_amontifier_normloop:
        xorq    j, j
        movq    k, h
        negq    c
        movl    $0, ashort
Lbignum_amontifier_shufloop:
        movq    a, c
        movq    (t,j,8), a
        cmovcq  a, c
        movq    c, (t,j,8)
        incq    j
        decq    h
        jnz     Lbignum_amontifier_shufloop
        decq    i
        jnz     Lbignum_amontifier_normloop

// We now have the top digit nonzero, assuming the input was nonzero,
// and as per the invariant of the loop above, c holds that digit. So
// now just count c's leading zeros and shift t bitwise that many bits.
// Note that we don't care about the result of bsr for zero inputs so
// the simple xor-ing with 63 is safe.

Lbignum_amontifier_normalized:

        bsrq    c, c
        xorq    $63, c

        xorq    b, b
        xorq    i, i
Lbignum_amontifier_bitloop:
        movq    (t,i,8), a
        movq    a, j
        shldq   %cl, b, a
        movq    a, (t,i,8)
        movq    j, b
        incq    i
        cmpq    k, i
        jc      Lbignum_amontifier_bitloop

// Let h be the high word of n, which in all the in-scope cases is >= 2^63.
// Now successively form q = 2^i div h and r = 2^i mod h as i goes from
// 64 to 126. We avoid just using division out of constant-time concerns
// (at the least we would need to fix up h = 0 for out-of-scope inputs) and
// don't bother with Newton-Raphson, since this stupid simple loop doesn't
// contribute much of the overall runtime at typical sizes.

        movq    -8(t,k,8), h
        movl    $1, qshort
        movq    h, r
        negq    r
        movl    $62, ishort
Lbignum_amontifier_estloop:

        addq    q, q
        movq    h, a
        subq    r, a
        cmpq    a, r // CF <=> r < h - r <=> 2 * r < h
        sbbq    a, a
        notq    a       // a = bitmask(2 * r >= h)
        subq    a, q
        addq    r, r
        andq    h, a
        subq    a, r
        decq    i
        jnz     Lbignum_amontifier_estloop

// Strictly speaking the above loop doesn't quite give the true remainder
// and quotient in the special case r = h = 2^63, so fix it up. We get
// q = 2^63 - 1 and r = 2^63 and really want q = 2^63 and r = 0. This is
// supererogatory, because the main property of q used below still holds
// in this case unless the initial m = 1, and then anyway the overall
// specification (congruence modulo m) holds degenerately. But it seems
// nicer to get a "true" quotient and remainder.

        incq    r
        cmpq    r, h
        adcq    $0, q

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

        xorq    c, c
        xorq    i, i
Lbignum_amontifier_mulloop:
        movq    (t,i,8), %rax
        mulq    q
        addq    c, %rax
        adcq    $0, %rdx
        movq    %rax, (z,i,8)
        movq    %rdx, c
        incq    i
        cmpq    k, i
        jc      Lbignum_amontifier_mulloop

// Now c is the high word of the product, so subtract 2^62
// and then turn it into a bitmask in q = h

        movq    $0x4000000000000000, %rax
        subq    a, c
        sbbq    q, q
        notq    q

// Now do [c] * n - d for our final answer

        xorq    c, c
        xorq    i, i
Lbignum_amontifier_remloop:
        movq    (t,i,8), a
        andq    q, a
        negq    c
        sbbq    (z,i,8), a
        sbbq    c, c
        movq    a, (z,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_amontifier_remloop

// Now still need to do a couple of modular doublings to get us all the
// way up to 2^{p+64} == r from initial 2^{p+62} == r (mod n).

        xorq    c, c
        xorq    j, j
        xorq    b, b
Lbignum_amontifier_dubloop1:
        movq    (z,j,8), a
        shrdq   $63, a, c
        negq    b
        sbbq    (t,j,8), c
        sbbq    b, b
        movq    c, (z,j,8)
        movq    a, c
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_dubloop1
        shrq    $63, c
        addq    b, c
        xorq    j, j
        xorq    b, b
Lbignum_amontifier_corrloop1:
        movq    (t,j,8), a
        andq    c, a
        negq    b
        adcq    (z,j,8), a
        sbbq    b, b
        movq    a, (z,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_corrloop1

// This is not exactly the same: we also copy output to t giving the
// initialization t_1 = r == 2^{p+64} mod n for the main loop next.

        xorq    c, c
        xorq    j, j
        xorq    b, b
Lbignum_amontifier_dubloop2:
        movq    (z,j,8), a
        shrdq   $63, a, c
        negq    b
        sbbq    (t,j,8), c
        sbbq    b, b
        movq    c, (z,j,8)
        movq    a, c
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_dubloop2
        shrq    $63, c
        addq    b, c
        xorq    j, j
        xorq    b, b
Lbignum_amontifier_corrloop2:
        movq    (t,j,8), a
        andq    c, a
        negq    b
        adcq    (z,j,8), a
        sbbq    b, b
        movq    a, (z,j,8)
        movq    a, (t,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_corrloop2

// We then successively generate (k+1)-digit values satisfying
// t_i == 2^{p+64*i} mod n, each of which is stored in h::t. Finish
// initialization by zeroing h initially

        xorq    h, h

// Then if t_i = 2^{p} * h + l
// we have t_{i+1} == 2^64 * t_i
//         = (2^{p+64} * h) + (2^64 * l)
//        == r * h + l<<64
// Do this k more times so we end up == 2^{128*k+64}, one more than we want
//
// Writing B = 2^{64k}, the possible correction of adding r, which for
// a (k+1)-digit result is equivalent to subtracting q = 2^{64*(k+1)} - r
// would give the overall worst-case value minus q of
// [ B * (B^k - 1) + (B - 1) * r ] - [B^{k+1} - r]
// = B * (r - 1) < B^{k+1} so we keep inside k+1 digits as required.
//
// This implementation makes the shift implicit by starting b with the
// "previous" digit (initially 0) to offset things by 1.

        movq    k, i
Lbignum_amontifier_modloop:
        xorq    b, b
        movq    k, n
        xorq    j, j
        xorq    c, c
Lbignum_amontifier_cmaloop:
        adcq    b, c
        sbbq    l, l
        movq    (z,j,8), %rax
        mulq    h
        subq    l, %rdx
        addq    c, %rax
        movq    (t,j,8), b
        movq    %rax, (t,j,8)
        movq    %rdx, c
        incq    j
        decq    n
        jnz     Lbignum_amontifier_cmaloop
        adcq    c, b
        movq    b, h

        sbbq    l, l

        xorq    j, j
        xorq    c, c
Lbignum_amontifier_oaloop:
        movq    (t,j,8), a
        movq    (z,j,8), b
        andq    l, b
        negq    c
        adcq    b, a
        sbbq    c, c
        movq    a, (t,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_oaloop
        subq    c, h

        decq    i
        jnz     Lbignum_amontifier_modloop

// Now do one almost-Montgomery reduction w.r.t. the original m
// which lops off one 2^64 from the congruence and, with the usual
// almost-Montgomery correction, gets us back inside k digits for
// the end result.

        movq    (m), a
        movq    a, c
        movq    a, b
        shlq    $2, c
        subq    c, b
        xorq    $2, b
        movq    b, c
        imulq   a, c
        movl    $2, ashort
        addq    c, a
        addq    $1, c
        imulq   a, b
        imulq   c, c
        movl    $1, ashort
        addq    c, a
        imulq   a, b
        imulq   c, c
        movl    $1, ashort
        addq    c, a
        imulq   a, b
        imulq   c, c
        movl    $1, ashort
        addq    c, a
        imulq   a, b

        movq    (t), c
        imulq   c, b

        movq    (m), %rax
        mulq    b
        addq    c, %rax
        movq    %rdx, c
        movl    $1, jshort
        movq    k, n
        decq    n
        jz      Lbignum_amontifier_montend

Lbignum_amontifier_montloop:
        adcq    (t,j,8), c
        sbbq    l, l
        movq    (m,j,8), %rax
        mulq    b
        subq    l, %rdx
        addq    c, %rax
        movq    %rax, -8(t,j,8)
        movq    %rdx, c
        incq    j
        decq    n
        jnz     Lbignum_amontifier_montloop
Lbignum_amontifier_montend:
        adcq    c, h
        sbbq    l, l
        movq    h, -8(t,k,8)

        xorq    j, j
        xorq    c, c
Lbignum_amontifier_osloop:
        movq    (t,j,8), a
        movq    (m,j,8), b
        andq    l, b
        negq    c
        sbbq    b, a
        sbbq    c, c
        movq    a, (z,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_amontifier_osloop

Lbignum_amontifier_end:
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_amontifier)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
