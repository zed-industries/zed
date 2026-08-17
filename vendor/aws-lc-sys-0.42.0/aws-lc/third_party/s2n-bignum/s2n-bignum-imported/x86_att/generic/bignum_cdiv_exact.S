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
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x, R8 = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x, [RSP+40] = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cdiv_exact)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cdiv_exact)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cdiv_exact)
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
#define t %r14

#define a %rax
#define h %rdx

#define ishort %ebx
#define hshort %edx

S2N_BN_SYMBOL(bignum_cdiv_exact):
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

// If k = 0 then there's nothing to be done

        testq   k, k
        jz      Lbignum_cdiv_exact_end

// Move parameters that need a new home

        movq    %rdx, n
        movq    %rcx, x

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

        movq    m, c
        xorl    ishort, ishort

// Unless n = 0, preload the zeroth digit and bump up the x pointer by
// 8 and n down by 1, to ease indexing and comparison using the same
// variable i in the main loop. When n = 0 we leave it alone, as the
// comparison i < n will always fail and the x pointer is unused.

        xorq    d, d
        testq   n, n
        jz      Lbignum_cdiv_exact_loop
        movq    (x), d
        addq    $8, x
        decq    n

Lbignum_cdiv_exact_loop:

// Load the next digit up to get [t,d] then shift right e places

        xorq    t, t
        cmpq    n, i
        jnc     Lbignum_cdiv_exact_noload
        movq    (x,i,8), t
Lbignum_cdiv_exact_noload:
        shrdq   %cl, t, d
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

        movq    t, d

        incq    i
        cmpq    k, i
        jc      Lbignum_cdiv_exact_loop

Lbignum_cdiv_exact_end:
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cdiv_exact)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
