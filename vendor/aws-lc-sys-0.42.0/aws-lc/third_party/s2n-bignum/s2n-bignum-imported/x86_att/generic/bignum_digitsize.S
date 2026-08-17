// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Return size of bignum in digits (64-bit word)
// Input x[k]; output function return
//
//    extern uint64_t bignum_digitsize(uint64_t k, const uint64_t *x);
//
// In the case of a zero bignum as input the result is 0
//
// Standard x86-64 ABI: RDI = k, RSI = x, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_digitsize)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_digitsize)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_digitsize)
        .text

#define k %rdi
#define x %rsi
#define i %rax
#define a %rcx
#define j %rdx



S2N_BN_SYMBOL(bignum_digitsize):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Initialize the index i and also prepare default return value of 0 (i = %rax)

        xorq    i, i

// If the bignum is zero-length, just return 0

        testq   k, k
        jz      Lbignum_digitsize_end

// Run over the words j = 0..i-1, and set i := j + 1 when hitting nonzero a[j]

        xorq    j, j
Lbignum_digitsize_loop:
        movq    (x,j,8), a
        incq    j
        testq   a, a
        cmovnzq j, i
        cmpq    k, j
        jnz     Lbignum_digitsize_loop

Lbignum_digitsize_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_digitsize)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
