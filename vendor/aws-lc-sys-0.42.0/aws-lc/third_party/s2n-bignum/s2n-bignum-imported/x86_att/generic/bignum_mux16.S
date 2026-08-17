// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select element from 16-element table, z := xs[k*i]
// Inputs xs[16*k], i; output z[k]
//
//    extern void bignum_mux16(uint64_t k, uint64_t *z, const uint64_t *xs,
//                             uint64_t i);
//
// It is assumed that all numbers xs[16] and the target z have the same size k
// The pointer xs is to a contiguous array of size 16, elements size-k bignums
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = xs, RCX = i
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = xs, R9 = i
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mux16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mux16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mux16)
        .text

#define k %rdi
#define z %rsi

// These get moved from original registers

#define x %rcx
#define i %rax

// Other registers

#define a %rdx
#define b %r8
#define j %r9
#define n %r10



S2N_BN_SYMBOL(bignum_mux16):
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


// Copy size into decrementable counter, or skip everything if k = 0

        testq   k, k
        jz      Lbignum_mux16_end                     // If length = 0 do nothing
        movq    k, n

// Multiply i by k so we can compare pointer offsets directly with it

        movq    %rcx, %rax
        movq    %rdx, %rcx
        mulq    k

// Main loop

Lbignum_mux16_loop:
        movq    (x), a
        movq    k, j
.rep 15
        movq    (x,j,8), b
        cmpq    i, j
        cmoveq  b, a
        addq    k, j
.endr
        movq    a, (z)
        addq    $8, z
        addq    $8, x
        decq    n
        jnz     Lbignum_mux16_loop

Lbignum_mux16_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mux16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
