// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from (almost-)Montgomery form z := (x / 2^{64k}) mod m
// Inputs x[k], m[k]; output z[k]
//
//    extern void bignum_demont(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *m);
//
// Does z := (x / 2^{64k}) mod m, hence mapping out of Montgomery domain.
// In other words, this is a k-fold Montgomery reduction with same-size input.
// This can handle almost-Montgomery inputs, i.e. any k-digit bignum.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont)
        .text

#define k %rdi
#define z %rsi
#define x %rdx
#define m %rcx

// General temp, low part of product and mul input
#define a %rax
// General temp, high part of product (no longer x)
#define b %rdx
// Negated modular inverse
#define w %r8
// Outer loop counter
#define i %r9
// Inner loop counter
#define j %rbx
// Home for Montgomery multiplier
#define d %rbp
#define h %r10
#define e %r11
#define n %r12

// A temp reg in the initial word-level negmodinv, same as j

#define t %rbx

#define ashort %eax
#define jshort %ebx


S2N_BN_SYMBOL(bignum_demont):
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

// Save registers

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)

// If k = 0 the whole operation is trivial

        testq   k, k
        jz      Lbignum_demont_end

// Compute word-level negated modular inverse w for m == m[0].

        movq    (m), a

        movq    a, t
        movq    a, w
        shlq    $2, t
        subq    t, w
        xorq    $2, w

        movq    w, t
        imulq   a, t
        movl    $2, ashort
        addq    t, a
        addq    $1, t

        imulq   a, w

        imulq   t, t
        movl    $1, ashort
        addq    t, a
        imulq   a, w

        imulq   t, t
        movl    $1, ashort
        addq    t, a
        imulq   a, w

        imulq   t, t
        movl    $1, ashort
        addq    t, a
        imulq   a, w

// Initially just copy the input to the output. It would be a little more
// efficient but somewhat fiddlier to tweak the zeroth iteration below instead.
// After this we never use x again and can safely recycle RDX for muls

        xorq    j, j
Lbignum_demont_iloop:
        movq    (x,j,8), a
        movq    a, (z,j,8)
        incq    j
        cmpq    k, j
        jc      Lbignum_demont_iloop

// Outer loop, just doing a standard Montgomery reduction on z

        xorq    i, i

Lbignum_demont_outerloop:
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
        jz      Lbignum_demont_montend

Lbignum_demont_montloop:
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
        jnz     Lbignum_demont_montloop

Lbignum_demont_montend:
        adcq    $0, h
        movq    h, -8(z,j,8)

// End of outer loop.

        incq    i
        cmpq    k, i
        jc      Lbignum_demont_outerloop

// Now do a comparison of z with m to set a final correction mask
// indicating that z >= m and so we need to subtract m.

        xorq    j, j
        movq    k, n
Lbignum_demont_cmploop:
        movq    (z,j,8), a
        sbbq    (m,j,8), a
        incq    j
        decq    n
        jnz     Lbignum_demont_cmploop
        sbbq    d, d
        notq    d

// Now do a masked subtraction of m for the final reduced result.

        xorq    e, e
        xorq    j, j
Lbignum_demont_corrloop:
        movq    (m,j,8), a
        andq    d, a
        negq    e
        sbbq    a, (z,j,8)
        sbbq    e, e
        incq    j
        cmpq    k, j
        jc      Lbignum_demont_corrloop

Lbignum_demont_end:
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
