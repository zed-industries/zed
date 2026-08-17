// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select digit x[n]
// Inputs x[k], n; output function return
//
//    extern uint64_t bignum_digit(uint64_t k, const uint64_t *x, uint64_t n);
//
// n'th digit of a k-digit (digit=64 bits) bignum, in constant-time style.
// Indexing starts at 0, which is the least significant digit (little-endian).
// Returns zero if n >= k, i.e. we read a digit off the end of the bignum.
//
// Standard x86-64 ABI: RDI = k, RSI = x, RDX = n, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, R8 = n, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_digit)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_digit)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_digit)
        .text

#define k %rdi
#define x %rsi
#define n %rdx

#define d %rax
#define i %rcx
#define a %r8

S2N_BN_SYMBOL(bignum_digit):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Set the default digit to 0, and for length zero finish immediately

        xorq    d, d
        testq   k, k
        jz      Lbignum_digit_end

// Main loop: run over all the digits and take note of the n'th one

        xorq    i, i
Lbignum_digit_loop:
        movq    (x,i,8), a
        cmpq    n, i
        cmovzq  a, d
        incq    i
        cmpq    k, i
        jc      Lbignum_digit_loop

// Return

Lbignum_digit_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_digit)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
