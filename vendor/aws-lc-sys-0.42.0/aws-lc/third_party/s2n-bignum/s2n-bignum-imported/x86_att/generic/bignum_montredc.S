// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery reduce, z := (x' / 2^{64p}) MOD m
// Inputs x[n], m[k], p; output z[k]
//
//    extern void bignum_montredc(uint64_t k, uint64_t *z, uint64_t n,
//                                const uint64_t *x, const uint64_t *m, uint64_t p);
//
// Does a := (x' / 2^{64p}) mod m where x' = x if n <= p + k and in general
// is the lowest (p+k) digits of x, assuming x' <= 2^{64p} * m. That is,
// p-fold Montgomery reduction w.r.t. a k-digit modulus m giving a k-digit
// answer.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x, R8 = m, R9 = p
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x, [RSP+40] = m, [RSP+48] = p
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montredc)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montredc)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montredc)
        .text

 // We copy n into %r10 but it comes in in %rdx originally

#define k %rdi
#define z %rsi
#define n %r10
#define x %rcx
#define m %r8
#define p %r9

// General temp, low part of product and mul input
#define a %rax
// General temp, High part of product
#define b %rdx
// Negated modular inverse
#define w  (%rsp)
// Inner loop counter
#define j %rbx
// Home for i'th digit or Montgomery multiplier
#define d %rbp
#define h %r11
#define e %r12
#define t %r13
#define i %r14
#define c %r15

// Some more intuitive names for temp regs in initial word-level negmodinv.

#define t1 %rbx
#define t2 %r14

#define ashort %eax
#define cshort %r15d
#define jshort %ebx


S2N_BN_SYMBOL(bignum_montredc):
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
        movq    64(%rsp), %r9
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
        jz      Lbignum_montredc_end

// Move n input into its permanent home, since we need %rdx for multiplications

        movq    %rdx, n

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

// Initialize z to the lowest k digits of the input, zero-padding if n < k.

        movq    k, j
        cmpq    k, n
        cmovcq  n, j
        xorq    i, i
        testq   j, j
        jz      Lbignum_montredc_padloop
Lbignum_montredc_copyloop:
        movq    (x,i,8), a
        movq    a, (z,i,8)
        incq    i
        cmpq    j, i
        jc      Lbignum_montredc_copyloop

        cmpq    k, i
        jnc     Lbignum_montredc_initialized

        xorq    j, j
Lbignum_montredc_padloop:
        movq    j, (z,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_montredc_padloop

Lbignum_montredc_initialized:
        xorq    c, c

// Now if p = 0 we just need the corrective tail, and even that is
// only needed for the case when the input is exactly the modulus,
// to maintain the <= 2^64p * n precondition

        testq   p, p
        jz      Lbignum_montredc_corrective

// Outer loop, just doing a standard Montgomery reduction on z

        xorq    i, i
Lbignum_montredc_outerloop:
        movq    (z), e
        movq    w, d
        imulq   e, d
        movq    (m), a
        mulq    d
        addq    e, a // Will be zero but want the carry
        movq    %rdx, h
        movl    $1, jshort
        movq    k, t
        decq    t
        jz      Lbignum_montredc_montend

Lbignum_montredc_montloop:
        adcq    (z,j,8), h
        sbbq    e, e
        movq    (m,j,8), a
        mulq    d
        subq    e, %rdx
        addq    h, a
        movq    a, -8(z,j,8)
        movq    %rdx, h
        incq    j
        decq    t
        jnz     Lbignum_montredc_montloop

Lbignum_montredc_montend:
        adcq    c, h
        movl    $0, cshort
        adcq    $0, c

        addq    i, j
        cmpq    n, j
        jnc     Lbignum_montredc_offtheend
        movq    (x,j,8), a
        addq    a, h
        adcq    $0, c
Lbignum_montredc_offtheend:
        movq    h, -8(z,k,8)

// End of outer loop.

        incq    i
        cmpq    p, i
        jc      Lbignum_montredc_outerloop

// Now do a comparison of (c::z) with (0::m) to set a final correction mask
// indicating that (c::z) >= m and so we need to subtract m.

Lbignum_montredc_corrective:

        xorq    j, j
        movq    k, n
Lbignum_montredc_cmploop:
        movq    (z,j,8), a
        sbbq    (m,j,8), a
        incq    j
        decq    n
        jnz     Lbignum_montredc_cmploop

        sbbq    $0, c
        sbbq    d, d
        notq    d

// Now do a masked subtraction of m for the final reduced result.

        xorq    e, e
        xorq    j, j
Lbignum_montredc_corrloop:
        movq    (m,j,8), a
        andq    d, a
        negq    e
        sbbq    a, (z,j,8)
        sbbq    e, e
        incq    j
        cmpq    k, j
        jc      Lbignum_montredc_corrloop

Lbignum_montredc_end:
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

S2N_BN_SIZE_DIRECTIVE(bignum_montredc)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
