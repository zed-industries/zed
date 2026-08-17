// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_256, z := (-x) mod p_256, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_neg_p256(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p256)
        .text

#define z %rdi
#define x %rsi

#define q %rdx

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9

#define n1 %r10
#define n3 %r11

#define d0short %eax
#define n1short %r10d

S2N_BN_SYMBOL(bignum_neg_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the input digits as [d3;d2;d1;d0] and also set a bitmask q
// for the input being nonzero, so that we avoid doing -0 = p_256
// and hence maintain strict modular reduction

        movq    (x), d0
        movq    8(x), d1
        movq    d0, n1
        orq     d1, n1
        movq    16(x), d2
        movq    24(x), d3
        movq    d2, n3
        orq     d3, n3
        orq     n1, n3
        negq    n3
        sbbq    q, q

// Load the non-trivial words of p_256 = [n3;0;n1;-1] and mask them with q

        movl    $0x00000000ffffffff, n1short
        movq    $0xffffffff00000001, n3
        andq    q, n1
        andq    q, n3

// Do the subtraction, getting it as [n3;d0;n1;q] to avoid moves

        subq    d0, q
        movl    $0, d0short
        sbbq    d1, n1
        sbbq    d2, d0
        sbbq    d3, n3

// Write back

        movq    q, (z)
        movq    n1, 8(z)
        movq    d0, 16(z)
        movq    n3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
