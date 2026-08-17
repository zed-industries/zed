// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Almost-Montgomery multiply, z :== (x * y / 2^{64k}) (congruent mod m)
// Inputs x[k], y[k], m[k]; output z[k]
//
//    extern void bignum_amontmul(uint64_t k, uint64_t *z, const uint64_t *x,
//                                const uint64_t *y, const uint64_t *m);
//
// Does z :== (x * y / 2^{64k}) mod m, meaning that the result, in the native
// size k, is congruent modulo m, but might not be fully reduced mod m. This
// is why it is called *almost* Montgomery multiplication.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = y, R8 = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = y, [RSP+40] = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_amontmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_amontmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_amontmul)
        .text

// We copy x into %r9 but it comes in in %rdx originally

#define k %rdi
#define z %rsi
#define x %r9
#define y %rcx
#define m %r8

// General temp, low part of product and mul input
#define a %rax
// General temp, High part of product
#define b %rdx
// Inner loop counter
#define j %rbx
// Home for i'th digit or Montgomery multiplier
#define d %rbp
#define h %r10
#define e %r11
#define n %r12
#define i %r13
#define c0 %r14
#define c1 %r15

// This one variable we store on the stack as we are a register short.
// At least it's only used once per iteration of the outer loop (k times)
// and with a single read each time, after one initial write. The variable
// is the word-level negated modular inverse

#define w  (%rsp)

// Some more intuitive names for temp regs in initial word-level negmodinv.

#define t1 %rbx
#define t2 %rdx

#define ashort %eax
#define jshort %ebx


S2N_BN_SYMBOL(bignum_amontmul):
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

// Save registers and allocate space on stack for non-register variable w

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(8)

// If k = 0 the whole operation is trivial

        testq   k, k
        jz      Lbignum_amontmul_end

// Move x input into its permanent home, since we need %rdx for multiplications

        movq    %rdx, x

// Compute word-level negated modular inverse w for m == m[0].

        movq    (m), a

        movq    a, t2
        movq    a, t1
        shlq    $2, t2
        subq    t2, t1
        xorq    $2, t1

        movq    t1, t2
        imulq   a, t2
        movl    $2, ashort
        addq    t2, a
        addq    $1, t2

        imulq   a, t1

        imulq   t2, t2
        movl    $1, ashort
        addq    t2, a
        imulq   a, t1

        imulq   t2, t2
        movl    $1, ashort
        addq    t2, a
        imulq   a, t1

        imulq   t2, t2
        movl    $1, ashort
        addq    t2, a
        imulq   a, t1

        movq    t1, w

// Initialize the output c0::z to zero so we can then consistently add rows.
// It would be a bit more efficient to special-case the zeroth row, but
// this keeps the code slightly simpler.

        xorq    i, i // Also initializes i for main loop
        xorq    j, j
Lbignum_amontmul_zoop:
        movq    i, (z,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_amontmul_zoop

        xorq    c0, c0

// Outer loop pulling down digits d=x[i], multiplying by y and reducing

Lbignum_amontmul_outerloop:

// Multiply-add loop where we always have CF + previous high part h to add in.
// Note that in general we do need yet one more carry in this phase and hence
// initialize c1 with the top carry.

        movq    (x,i,8), d
        xorq    j, j
        xorq    h, h
        xorq    c1, c1
        movq    k, n

Lbignum_amontmul_maddloop:
        adcq    (z,j,8), h
        sbbq    e, e
        movq    (y,j,8), a
        mulq    d
        subq    e, %rdx
        addq    h, a
        movq    a, (z,j,8)
        movq    %rdx, h
        incq    j
        decq    n
        jnz     Lbignum_amontmul_maddloop
        adcq    h, c0
        adcq    c1, c1

// Montgomery reduction loop, similar but offsetting writebacks

        movq    (z), e
        movq    w, d
        imulq   e, d
        movq    (m), a
        mulq    d
        addq    e, a // Will be zero but want the carry
        movq    %rdx, h
        movl    $1, jshort
        movq    k, n
        decq    n
        jz      Lbignum_amontmul_montend

Lbignum_amontmul_montloop:
        adcq    (z,j,8), h
        sbbq    e, e
        movq    (m,j,8), a
        mulq    d
        subq    e, %rdx
        addq    h, a
        movq    a, -8(z,j,8)
        movq    %rdx, h
        incq    j
        decq    n
        jnz     Lbignum_amontmul_montloop

Lbignum_amontmul_montend:
        adcq    c0, h
        adcq    $0, c1
        movq    c1, c0
        movq    h, -8(z,j,8)

// End of outer loop.

        incq    i
        cmpq    k, i
        jc      Lbignum_amontmul_outerloop

// Now convert carry word, which is always in {0,1}, into a mask "d"
// and do a masked subtraction of m for the final almost-Montgomery result.

        xorq    d, d
        subq    c0, d
        xorq    e, e
        xorq    j, j
Lbignum_amontmul_corrloop:
        movq    (m,j,8), a
        andq    d, a
        negq    e
        sbbq    a, (z,j,8)
        sbbq    e, e
        incq    j
        cmpq    k, j
        jc      Lbignum_amontmul_corrloop

Lbignum_amontmul_end:

        CFI_INC_RSP(8)
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

S2N_BN_SIZE_DIRECTIVE(bignum_amontmul)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
