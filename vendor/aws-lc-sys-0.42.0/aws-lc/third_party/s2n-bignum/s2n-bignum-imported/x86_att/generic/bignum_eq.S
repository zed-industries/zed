// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignums for equality, x = y
// Inputs x[m], y[n]; output function return
//
//    extern uint64_t bignum_eq(uint64_t m, const uint64_t *x, uint64_t n,
//                              const uint64_t *y);
//
// Standard x86-64 ABI: RDI = m, RSI = x, RDX = n, RCX = y, returns RAX
// Microsoft x64 ABI:   RCX = m, RDX = x, R8 = n, R9 = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_eq)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_eq)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_eq)
        .text

#define m %rdi
#define x %rsi
#define n %rdx
#define y %rcx
#define c %rax
// We can re-use n for this, not needed when d appears
#define d %rdx

S2N_BN_SYMBOL(bignum_eq):
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

// Initialize the accumulated OR of differences to zero

        xorq    c, c

// If m >= n jump into the m > n loop at the final equality test
// This will drop through for m = n

        cmpq    n, m
        jnc     Lbignum_eq_mtest

// Toploop for the case n > m

Lbignum_eq_nloop:
        decq    n
        orq     (y,n,8), c
        cmpq    n, m
        jnz     Lbignum_eq_nloop
        jmp     Lbignum_eq_mmain

// Toploop for the case m > n (or n = m which enters at "mtest")

Lbignum_eq_mloop:
        decq    m
        orq     (x,m,8), c
        cmpq    n, m
Lbignum_eq_mtest:
        jnz     Lbignum_eq_mloop

// Combined main loop for the min(m,n) lower words

Lbignum_eq_mmain:
        testq   m, m
        jz      Lbignum_eq_end

Lbignum_eq_loop:
        movq    -8(x,m,8), d
        xorq    -8(y,m,8), d
        orq     d, c
        decq    m
        jnz     Lbignum_eq_loop

// Set a standard C condition based on whether c is nonzero

Lbignum_eq_end:
        negq    c
        sbbq    c, c
        incq    c
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_eq)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
