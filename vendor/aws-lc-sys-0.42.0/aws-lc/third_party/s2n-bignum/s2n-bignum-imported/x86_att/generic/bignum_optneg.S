// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate, z := -x (if p nonzero) or z := x (if p zero)
// Inputs p, x[k]; outputs function return (nonzero input) and z[k]
//
//    extern uint64_t bignum_optneg(uint64_t k, uint64_t *z, uint64_t p,
//                                  const uint64_t *x);
//
// It is assumed that both numbers x and z have the same size k digits.
// Returns a carry, which is equivalent to "x is nonzero".
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = p, RCX = x, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = p, R9 = x, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg)
        .text

#define k %rdi
#define z %rsi
#define p %rdx
#define x %rcx

#define c %rax
#define a %r8
#define i %r9

#define cshort %eax

S2N_BN_SYMBOL(bignum_optneg):
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

// If k = 0 do nothing, but need to set zero return for the carry (c = %rax)

        xorq    c, c
        testq   k, k
        jz      Lbignum_optneg_end

// Convert p into a strict bitmask and set initial carry-in in c

        negq    p
        sbbq    p, p
        subq    p, c

// Main loop

        xorq    i, i
Lbignum_optneg_loop:

        movq    (x,i,8), a
        xorq    p, a
        addq    c, a
        movl    $0, cshort
        movq    a, (z,i,8)
        adcq    $0, c
        incq    i
        cmpq    k, i
        jc      Lbignum_optneg_loop

// Return carry flag, fixing up inversion for negative case

        xorq    p, %rax
        andq    $1, %rax

Lbignum_optneg_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
