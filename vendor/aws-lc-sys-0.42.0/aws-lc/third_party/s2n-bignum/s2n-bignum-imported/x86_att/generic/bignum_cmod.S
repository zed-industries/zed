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
// Standard x86-64 ABI: RDI = k, RSI = x, RDX = m, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, R8 = m, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmod)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmod)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmod)
        .text

#define k %rdi
#define x %rsi

// This has to be %rcx for variable shifts

#define e %rcx

// We share the same variable for m and n, just shifting left then right.
// And h is kept in %rdx which does work despite the special operands of mul.

#define m %r8
#define n %r8

#define w %r9
#define a %rax
#define r %r10
#define h %rdx
#define l %r11

#define ashort %eax
#define hshort %edx

S2N_BN_SYMBOL(bignum_cmod):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Initialize l = 0 now for convenience (we eventually need to do it).
// If the bignum is zero-length, l is already the right answer of 0

        xorq    l, l
        testq   k, k
        jz      Lbignum_cmod_end

// Move m into its permanent home (also used for n).
// Find number of leading zeros of m and let n = 2^e m so that for an
// in-scope (nonzero) input m we have n >= 2^63, e <= 63.

        movq    %rdx, m
        bsrq    m, e
        xorq    $63, e
        shlq    %cl, m

// A near-clone of word_recip so 2^64 + w = ceil(2^128 / n) - 1

        movq   n, r
        movq   $0x1FFFFFFFFFFFF, w
        shrq   $16, r
        xorq   r, w
        incq   r
        shrq   $32, w
        movq   r, h
        imulq  w, h
        negq   h
        movq   h, a
        shrq   $49, a
        imulq  a, a
        shrq   $34, h
        addq   a, h
        orq    $0x40000000, a
        imulq  h, a
        shrq   $30, a
        imulq  w, a
        shlq   $30, w
        addq   a, w
        shrq   $30, w
        movq   r, h
        imulq  w, h
        negq   h
        shrq   $24, h
        imulq  w, h
        shlq   $16, w
        shrq   $24, h
        addq   h, w
        movq   r, h
        imulq  w, h
        negq   h
        shrq   $32, h
        imulq  w, h
        shlq   $31, w
        shrq   $17, h
        addq   h, w
        movq   n, a
        mulq   w
        shrdq  $60, h, a
        movq   w, h
        shrq   $33, h
        notq   a
        imulq  h, a
        shlq   $1, w
        shrq   $33, a
        addq   a, w
        addq   $1, w
        movq   n, a
        sbbq   $0, w
        mulq   w
        addq   n, h
        sbbq   $0, w

// Take the residue r = 2^128 - (2^64 + w) * n, which by the above bound
// we know fits in 64 bits. We know 2^128 == r (mod n) and hence (mod m).

        movq    n, r
        imulq   w, r
        negq    r

// Now just go down through the digits accumulating [h;l] == x (mod n)
// by 2^64 * [h;l] + d = 2^128 * h + [l;d] == r * h + [l; d]. That addition
// may overflow with a carry, say 2^128 + [h';l'] = r * h + [l; d], in
// which case we subtract 2^128 - r (which is divisible by m and keeping
// things in 128 bits we just add r). Thus the overall bound when we initially
// overflow is r * h + [l; d] - (2^128 - r) = r * (h + 1) + [l; d] - 2^128
// < 2^128 so we stay inside 2 words

        xorl    hshort, hshort
Lbignum_cmod_loop:
        movq    h, a
        mulq    r
        addq    -8(x,k,8), a
        adcq    l, h
        movq    a, l
        sbbq    a, a
        andq    r, a
        addq    a, l
        adcq    $0, h
        decq    k
        jnz     Lbignum_cmod_loop

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

        movq    h, k // back up h for muls
        movq    w, a
        mulq    h
        addq    k, h
        sbbq    r, r
        andq    n, r // So q = (r;h)

        movq    h, a
        mulq    n
        addq    r, h
        xorq    r, r
        subq    a, l
        sbbq    h, k // (k,l) = first reduction

        cmovnzq n, r
        xorl    ashort, ashort
        subq    r, l
        sbbq    a, k

        cmovnzq n, a
        subq    a, l

// One more reciprocal multiplication to do a modular reduction, but now in
// one word and in terms of the original m. For the quotient estimate we want
// q = ((2^64 + w) * l) / 2^{128-e} = ((2^64 + w) * l) / 2^65 / 2^{63-e}.

        movq    w, a
        mulq    l
        addq    l, h
        rcr     $1, h

        shrq    %cl, m
        xorq    $63, e
        shrq    %cl, h

        imulq   m, h
        subq    h, l

// Note that since there is no neglected "low" part of the single word,
// one round of correction suffices; in the analog of the above l = 0
// and hence the residue so far is already < 2 * m.

        movq    l, a
        subq    m, l
Lbignum_cmod_end:
        cmovncq l, a
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmod)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
