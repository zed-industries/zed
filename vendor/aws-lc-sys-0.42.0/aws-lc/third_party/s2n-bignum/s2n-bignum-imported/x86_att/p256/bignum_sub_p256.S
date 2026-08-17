// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_256, z := (x - y) mod p_256
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_sub_p256(uint64_t z[static 4], const uint64_t x[static 4],
//                                const uint64_t y[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p256)
        .text

#define z %rdi
#define x %rsi
#define y %rdx

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9

#define n1 %r10
#define n3 %rdx
#define c %r11

#define n1short %r10d



S2N_BN_SYMBOL(bignum_sub_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Load and subtract the two inputs as [d3;d2;d1;d0] = x - y (modulo 2^256)

        movq    (x), d0
        subq    (y), d0
        movq    8(x), d1
        sbbq    8(y), d1
        movq    16(x), d2
        sbbq    16(y), d2
        movq    24(x), d3
        sbbq    24(y), d3

// Capture the carry, which indicates x < y, and create corresponding masked
// correction p_256' = [n3; 0; n1; c] to add

        movl    $0x00000000ffffffff, n1short
        sbbq    c, c
        xorq    n3, n3
        andq    c, n1
        subq    n1, n3

// Do the corrective addition and copy to output

        addq    c, d0
        movq    d0, (z)
        adcq    n1, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    n3, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
