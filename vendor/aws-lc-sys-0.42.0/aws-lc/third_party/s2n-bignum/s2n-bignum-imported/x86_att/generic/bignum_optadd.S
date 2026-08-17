// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally add, z := x + y (if p nonzero) or z := x (if p zero)
// Inputs x[k], p, y[k]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_optadd(uint64_t k, uint64_t *z, const uint64_t *x,
//                                  uint64_t p, const uint64_t *y);
//
// It is assumed that all numbers x, y and z have the same size k digits.
// Returns carry-out as per usual addition, always 0 if p was zero.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = p, R8 = y, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = p, [RSP+40] = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optadd)
        .text

#define k %rdi
#define z %rsi
#define x %rdx
#define p %rcx
#define y %r8

#define c %rax
#define i %r9
#define b %r10
#define a %r11


S2N_BN_SYMBOL(bignum_optadd):
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

// Initialize top carry to zero in all cases (also return value)

        xorq    c, c

// If k = 0 do nothing

        testq   k, k
        jz      Lbignum_optadd_end

// Convert the nonzero/zero status of p into an all-1s or all-0s mask

        negq    p
        sbbq    p, p

// Now go round the loop for i=0...k-1, saving the carry in c each iteration

        xorq    i, i
Lbignum_optadd_loop:
        movq    (x,i,8), a
        movq    (y,i,8), b
        andq    p, b
        negq    c
        adcq    b, a
        sbbq    c, c
        movq    a, (z,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_optadd_loop

// Return top carry

        negq    %rax

Lbignum_optadd_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
