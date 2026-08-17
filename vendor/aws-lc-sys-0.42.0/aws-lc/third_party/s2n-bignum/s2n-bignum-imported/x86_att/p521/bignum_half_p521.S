// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_521, z := (x / 2) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_half_p521(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p521)
        .text

#define z %rdi
#define x %rsi

// We use distinct variables for clarity, but these are heavily aliased

#define d0 %rcx
#define d1 %rdx
#define d2 %rcx
#define d3 %rdx
#define d4 %rcx
#define d5 %rdx
#define d6 %rcx
#define d7 %rdx
#define d8 %rcx
#define a %rax
#define ashort %eax



S2N_BN_SYMBOL(bignum_half_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// We do a 521-bit rotation one bit right, since 2^521 == 1 (mod p_521)

        movq    (x), d0
        movl    $1, ashort
        andq    d0, a

        movq    8(x), d1
        shrdq   $1, d1, d0
        movq    d0, (z)

        movq    16(x), d2
        shrdq   $1, d2, d1
        movq    d1, 8(z)

        movq    24(x), d3
        shrdq   $1, d3, d2
        movq    d2, 16(z)

        movq    32(x), d4
        shrdq   $1, d4, d3
        movq    d3, 24(z)

        movq    40(x), d5
        shrdq   $1, d5, d4
        movq    d4, 32(z)

        movq    48(x), d6
        shrdq   $1, d6, d5
        movq    d5, 40(z)

        movq    56(x), d7
        shrdq   $1, d7, d6
        movq    d6, 48(z)

        movq    64(x), d8
        shrdq   $1, d8, d7
        movq    d7, 56(z)

        shlq    $55, d8
        shrdq   $56, a, d8
        movq    d8, 64(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
