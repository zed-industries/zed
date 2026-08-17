// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square z := x^2
// Input x[n]; output z[k]
//
//    extern void bignum_sqr(uint64_t k, uint64_t *z, uint64_t n, const uint64_t *x);
//
// Does the "z := x^2" operation where x is n digits and result z is k.
// Truncates the result in general unless k >= 2 * n
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr)
        .text

// First three are where arguments come in, but n is moved.

#define p %rdi
#define z %rsi
#define x %rcx
#define n %r8

// These are always local scratch since multiplier result is in these

#define a %rax
#define d %rdx

// Other variables

#define i %rbx
#define ll %rbp
#define hh %r9
#define k %r10
#define y %r11
#define htop %r12
#define l %r13
#define h %r14
#define c %r15

// Short versions

#define llshort %ebp

S2N_BN_SYMBOL(bignum_sqr):
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

// We use too many registers, and also we need %rax:%rdx for multiplications

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        movq    %rdx, n

// If p = 0 the result is trivial and nothing needs doing

        testq   p, p
        jz      Lbignum_sqr_end

// initialize (hh,ll) = 0

        xorl    llshort, llshort
        xorq    hh, hh

// Iterate outer loop from k = 0 ... k = p - 1 producing result digits

        xorq    k, k

Lbignum_sqr_outerloop:

// First let bot = MAX 0 (k + 1 - n) and top = MIN (k + 1) n
// We want to accumulate all x[i] * x[k - i] for bot <= i < top
// For the optimization of squaring we avoid duplication and do
// 2 * x[i] * x[k - i] for i < htop, where htop = MIN ((k+1)/2) n
// Initialize i = bot; in fact just compute bot as i directly.

        xorq    c, c
        leaq    1(k), i
        movq    i, htop
        shrq    $1, htop
        subq    n, i
        cmovcq  c, i
        cmpq    n, htop
        cmovncq n, htop

// Initialize the three-part local sum (c,h,l); c was already done above

        xorq    l, l
        xorq    h, h

// If htop <= bot then main doubled part of the sum is empty

        cmpq    htop, i
        jnc     Lbignum_sqr_nosumming

// Use a moving pointer for [y] = x[k-i] for the cofactor

        movq    k, a
        subq    i, a
        leaq    (x,a,8), y

// Do the main part of the sum x[i] * x[k - i] for 2 * i < k

Lbignum_sqr_innerloop:
        movq    (x,i,8), a
        mulq     (y)
        addq    a, l
        adcq    d, h
        adcq    $0, c
        subq    $8, y
        incq    i
        cmpq    htop, i
        jc      Lbignum_sqr_innerloop

// Now double it

        addq    l, l
        adcq    h, h
        adcq    c, c

// If k is even (which means 2 * i = k) and i < n add the extra x[i]^2 term

Lbignum_sqr_nosumming:
        testq   $1, k
        jnz     Lbignum_sqr_innerend
        cmpq    n, i
        jnc     Lbignum_sqr_innerend

        movq    (x,i,8), a
        mulq    a
        addq    a, l
        adcq    d, h
        adcq    $0, c

// Now add the local sum into the global sum, store and shift

Lbignum_sqr_innerend:
        addq    ll, l
        movq    l, (z,k,8)
        adcq    hh, h
        movq    h, ll
        adcq    $0, c
        movq    c, hh

        incq    k
        cmpq    p, k
        jc      Lbignum_sqr_outerloop

// Restore registers and return

Lbignum_sqr_end:
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
