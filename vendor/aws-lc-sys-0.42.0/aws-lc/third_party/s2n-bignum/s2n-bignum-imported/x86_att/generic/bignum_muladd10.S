// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply bignum by 10 and add word: z := 10 * z + d
// Inputs z[k], d; outputs function return (carry) and z[k]
//
//    extern uint64_t bignum_muladd10(uint64_t k, uint64_t *z, uint64_t d);
//
// Although typically the input d < 10, this is not actually required.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = d, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = d, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_muladd10)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_muladd10)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_muladd10)
        .text

#define k %rdi
#define z %rsi
#define d %rcx

#define a %rax
#define l %rax

#define h %rdx
#define i %r8
#define ten %r9
#define tenshort %r9d

S2N_BN_SYMBOL(bignum_muladd10):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Move carry input to permanent home, and if k = 0 skip the main loop

        movq    %rdx, d
        testq   k, k
        jz      Lbignum_muladd10_end

// Simple loop

        xorq    i, i
        movl    $10, tenshort
Lbignum_muladd10_loop:
        movq    (z,i,8), a
        mulq    ten
        addq    d, l
        movq    l, (z,i,8)
        adcq    $0, h
        movq    h, d
        incq    i
        cmpq    k, i
        jc      Lbignum_muladd10_loop

// Return the final carry

Lbignum_muladd10_end:
        movq    d, %rax
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_muladd10)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
