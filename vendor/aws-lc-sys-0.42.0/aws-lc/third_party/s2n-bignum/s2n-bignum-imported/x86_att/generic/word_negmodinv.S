// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Single-word negated modular inverse (-1/a) mod 2^64
// Input a; output function return
//
//    extern uint64_t word_negmodinv(uint64_t a);
//
// A 64-bit function that returns a negated multiplicative inverse mod 2^64
// of its input, assuming that input is odd. Given odd input a, the result z
// will satisfy a * z + 1 == 0 (mod 2^64), i.e. a 64-bit word multiplication
// a * z will give -1.
//
// Standard x86-64 ABI: RDI = a, returns RAX
// Microsoft x64 ABI:   RCX = a, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_negmodinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_negmodinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_negmodinv)
        .text

S2N_BN_SYMBOL(word_negmodinv):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
#endif

// Initial magical 5-bit approximation x = (a - a<<2) xor 2

        movq    %rdi, %rcx
        movq    %rdi, %rax
        shlq    $2, %rcx
        subq    %rcx, %rax
        xorq    $2, %rax

// Now refine to 64-bit congruence

        movq    %rax, %rcx // %rcx = x
        imulq   %rdi, %rcx // %rcx = a * x
        movl    $2, %edx
        addq    %rcx, %rdx // %rdx = 1 + e = 2 + a * x
        addq    $1, %rcx // %rcx = e = a * x + 1

        imulq   %rdx, %rax // %rax = x * (1 + e)

        imulq   %rcx, %rcx // %rcx = e^2
        movl    $1, %edx
        addq    %rcx, %rdx
        imulq   %rdx, %rax // %rax = x * (1 + e) * (1 + e^2)

        imulq   %rcx, %rcx // %rcx = e^4
        movl    $1, %edx
        addq    %rcx, %rdx
        imulq   %rdx, %rax // %rax = x * (1 + e) * (1 + e^2) * (1 + e^4)

        imulq   %rcx, %rcx // %rcx = e^8
        movl    $1, %edx
        addq    %rcx, %rdx
        imulq   %rdx, %rax // %rax = x * (1 + e) * ... * * (1 + e^8)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_negmodinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
