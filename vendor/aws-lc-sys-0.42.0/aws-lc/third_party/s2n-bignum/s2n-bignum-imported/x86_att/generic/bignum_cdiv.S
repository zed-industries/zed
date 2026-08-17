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
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x, R8 = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x, [RSP+40] = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cdiv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cdiv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cdiv)
        .text

#define k %rdi
#define z %rsi
#define m %r8

// These parameters get moved because of special uses for %rcx, %rdx

#define n %r9
#define x %r10

// This needs to be in %rcx for variable shifts with %cl

#define e %rcx

// Other variables

#define w %r11
#define d %r12
#define i %rbx
#define c %r13
#define l %r14

#define a %rax
#define h %rdx

#define ashort %eax
#define ishort %ebx
#define hshort %edx

// The remainder

#define r %r15

S2N_BN_SYMBOL(bignum_cdiv):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        movq    56(%rsp), %r8
#endif

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Move parameters that need a new home

        movq    %rdx, n
        movq    %rcx, x

// First do a modulus computation, slightly tweaked from bignum_cmod,
// changing variables and avoiding modification of the size parameter.
// Initialize l = 0 now for convenience (we eventually need to do it).
// If the bignum is zero-length, l is already the right answer of 0

        xorq    l, l
        testq   n, n
        jz      Lbignum_cdiv_nomodulus

        bsrq    m, e
        xorq    $63, e
        shlq    %cl, m

        movq   m, r
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
        movq   m, a
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
        movq   m, a
        sbbq   $0, w
        mulq   w
        addq   m, h
        sbbq   $0, w

        movq    m, r
        imulq   w, r
        negq    r

        xorl    hshort, hshort
        movq    n, i
Lbignum_cdiv_modloop:
        movq    h, a
        mulq    r
        addq    -8(x,i,8), a
        adcq    l, h
        movq    a, l
        sbbq    a, a
        andq    r, a
        addq    a, l
        adcq    $0, h
        decq    i
        jnz     Lbignum_cdiv_modloop

        movq    h, i
        movq    w, a
        mulq    h
        addq    i, h
        sbbq    r, r
        andq    m, r

        movq    h, a
        mulq    m
        addq    r, h
        xorq    r, r
        subq    a, l
        sbbq    h, i

        cmovnzq m, r
        xorl    ashort, ashort
        subq    r, l
        sbbq    a, i

        cmovnzq m, a
        subq    a, l

        movq    w, a
        mulq    l
        addq    l, h
        rcr     $1, h

        shrq    %cl, m
        xorq    $63, e
        shrq    %cl, h

        imulq   m, h
        subq    h, l

        movq    l, r
        subq    m, l
Lbignum_cdiv_nomodulus:
        cmovncq l, r

// If k = 0 then there's no more to be done

        testq   k, k
        jz      Lbignum_cdiv_end

// Let e be the number of trailing zeros in m (we can ignore m = 0)

        bsfq    m, e

// Now just shift m right by e bits. So hereafter we can assume m is odd
// but we first need to shift the input right by e bits then divide by m.

        shrq   %cl, m

// Compute the negated modular inverse w with w * m + 1 == 0 (mod 2^64)
// This is essentially the same as word_negmodinv.

        movq    m, a
        movq    m, w
        shlq    $2, a
        subq    a, w
        xorq    $2, w
        movq    w, a
        imulq   m, a
        movl    $2, hshort
        addq    a, h
        addq    $1, a
        imulq   h, w
        imulq   a, a
        movl    $1, hshort
        addq    a, h
        imulq   h, w
        imulq   a, a
        movl    $1, hshort
        addq    a, h
        imulq   h, w
        imulq   a, a
        movl    $1, hshort
        addq    a, h
        imulq   h, w

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

        movq    r, d
        shrq    %cl, d
        movq    m, c
        subq    d, c
        xorl    ishort, ishort

// Unless n = 0, preload the zeroth digit and bump up the x pointer by
// 8 and n down by 1, to ease indexing and comparison using the same
// variable i in the main loop. When n = 0 we leave it alone, as the
// comparison i < n will always fail and the x pointer is unused.

        xorq    d, d
        testq   n, n
        jz      Lbignum_cdiv_loop
        movq    (x), d
        addq    $8, x
        decq    n

Lbignum_cdiv_loop:

// Load the next digit up to get [l,d] then shift right e places

        xorq    l, l
        cmpq    n, i
        jnc     Lbignum_cdiv_noload
        movq    (x,i,8), l
Lbignum_cdiv_noload:
        shrdq   %cl, l, d
        addq    c, d
        sbbq    c, c
        negq    c

// Now the effective sum is [c,a] where the carry-in has been absorbed.
// Do the main Montgomery step with the (odd) m, writing back ~q. Finally
// set d to the next digit ready for the following iteration.

        movq    w, a
        imulq   d, a
        notq    a
        movq    a, (z,i,8)
        notq    a

        mulq    m
        addq    d, a
        adcq    h, c

        movq    l, d

        incq    i
        cmpq    k, i
        jc      Lbignum_cdiv_loop

// Return the modulus

Lbignum_cdiv_end:
        movq    r, %rax

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cdiv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
