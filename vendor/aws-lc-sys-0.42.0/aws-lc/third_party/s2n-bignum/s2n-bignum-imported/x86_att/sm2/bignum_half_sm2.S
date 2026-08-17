// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_sm2, z := (x / 2) mod p_sm2, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_half_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_sm2)
        .text

#define z %rdi
#define x %rsi

#define a %rax
#define d0 %rcx
#define d1 %rdx
#define d2 %r8
#define d3 %r9

#define d0short %ecx
#define d1short %edx

S2N_BN_SYMBOL(bignum_half_sm2):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load lowest digit and get a mask for its lowest bit in d0

        movq    (x), a
        movl    $1, d0short
        andq    a, d0
        negq    d0

// Create a masked version of p_sm2

        movq    $0xffffffff00000000, d1
        andq    d0, d1
        movq    d0, d2
        movq    $0xfffffffeffffffff, d3
        andq    d0, d3

// Perform addition with masked p_sm2. Catch the carry in a, as a bitmask
// for convenience though we only use its LSB below with SHRD

        addq    a, d0
        adcq    8(x), d1
        adcq    16(x), d2
        adcq    24(x), d3
        sbbq    a, a

// Shift right, pushing the carry back down, and store back

        shrdq   $1, d1, d0
        movq    d0, (z)
        shrdq   $1, d2, d1
        movq    d1, 8(z)
        shrdq   $1, d3, d2
        movq    d2, 16(z)
        shrdq   $1, a, d3
        movq    d3, 24(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
