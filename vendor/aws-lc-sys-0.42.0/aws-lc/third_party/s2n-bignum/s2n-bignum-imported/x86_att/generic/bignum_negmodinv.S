// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negated modular inverse, z := (-1/x) mod 2^{64k}
// Input x[k]; output z[k]
//
//    extern void bignum_negmodinv(uint64_t k, uint64_t *z, const uint64_t *x);
//
// Assuming x is odd (otherwise nothing makes sense) the result satisfies
//
//       x * z + 1 == 0 (mod 2^{64 * k})
//
// but is not necessarily reduced mod x.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_negmodinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_negmodinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_negmodinv)
        .text

#define k %rdi
#define z %rsi
// Moved from initial location to free %rdx
#define x %rcx

#define a %rax
#define d %rdx
#define i %r8
#define m %r9
#define h %r10
#define w %r11
#define t %r12
#define e %rbx

#define ashort %eax
#define ishort %r8d

S2N_BN_SYMBOL(bignum_negmodinv):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)

// If k = 0 do nothing (actually we could have avoiding the pushes and pops)

        testq   k, k
        jz      Lbignum_negmodinv_end

// Move the x pointer into its permanent home (%rdx is needed for muls)

        movq    %rdx, x

// Compute word-level negated modular inverse w for x[0].

        movq    (x), a

        movq    a, d
        movq    a, w
        shlq    $2, d
        subq    d, w
        xorq    $2, w

        movq    w, d
        imulq   a, d
        movl    $2, ashort
        addq    d, a
        addq    $1, d

        imulq   a, w

        imulq   d, d
        movl    $1, ashort
        addq    d, a
        imulq   a, w

        imulq   d, d
        movl    $1, ashort
        addq    d, a
        imulq   a, w

        imulq   d, d
        movl    $1, ashort
        addq    d, a
        imulq   a, w

// Write that as lowest word of the output, then if k = 1 we're finished

        movq    w, (z)
        cmpq    $1, k
        jz      Lbignum_negmodinv_end

// Otherwise compute and write the other digits (1..k-1) of w * x + 1

        movq    (x), a
        xorq    h, h
        mulq    w
        addq    $1, a
        adcq    d, h
        movl    $1, ishort
Lbignum_negmodinv_initloop:
        movq    (x,i,8), a
        mulq    w
        addq    h, a
        adcq    $0, d
        movq    a, (z,i,8)
        movq    d, h
        incq    i
        cmpq    k, i
        jc      Lbignum_negmodinv_initloop

// For simpler indexing, z := z + 8 and k := k - 1 per outer iteration
// Then we can use the same index for x and for z and effective size k.
//
// But we also offset k by 1 so the "real" size is k + 1; after doing
// the special zeroth bit we count with t through k more digits, so
// getting k + 1 total as required.
//
// This lets us avoid some special cases inside the loop at the cost
// of needing the additional "finale" tail for the final iteration
// since we do one outer loop iteration too few.

        subq    $2, k
        jz      Lbignum_negmodinv_finale

Lbignum_negmodinv_outerloop:
        addq    $8, z

        movq    (z), h
        movq    w, m
        imulq   h, m
        movq    m, (z)
        movq    (x), a
        mulq    m
        addq    h, a
        adcq    $0, d
        movq    d, h
        movl    $1, ishort
        movq    k, t
Lbignum_negmodinv_innerloop:
        adcq    (z,i,8), h
        sbbq    e, e
        movq    (x,i,8), a
        mulq    m
        subq    e, d
        addq    h, a
        movq    a, (z,i,8)
        movq    d, h
        incq    i
        decq    t
        jnz     Lbignum_negmodinv_innerloop

        decq    k
        jnz     Lbignum_negmodinv_outerloop

Lbignum_negmodinv_finale:
        movq    8(z), a
        imulq   w, a
        movq    a, 8(z)

Lbignum_negmodinv_end:
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_negmodinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
