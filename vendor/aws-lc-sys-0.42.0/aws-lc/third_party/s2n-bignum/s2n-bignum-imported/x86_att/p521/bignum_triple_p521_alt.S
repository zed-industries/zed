// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_521, z := (3 * x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_triple_p521_alt(uint64_t z[static 9],
//                                       const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p521_alt)
        .text

#define z %rdi
#define x %rsi

// Digits; d8 is safely also used for the multiplier 3

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define d4 %rbx
#define d5 %rbp
#define d6 %r12
#define d7 %rcx // Also used for multiplier m = 3
#define d8 %rsi // Overwrites input pointer

// Other variables

#define m %rcx
#define a %rax
#define d %rdx

S2N_BN_SYMBOL(bignum_triple_p521_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save additional registers to use

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)

// Let [d8;...;d0] = x' + x + 1 where x' is a rotation left by 1 bit
// as a 521-bit quantity. This is == 3 * x + 1 (mod p_521) and keeps
// in a more limited range so that the correction is easier. Mostly
// we do just multiply by 3, except that 2 * bit_520 is stuffed in
// at the bottom instead of the top, so the top two digits are a bit
// more intricate.

        movq    $3, m
        movq    64(x), d0
        shrq    $8, d0
        incq    d0

        movq    (x), a
        mulq    m
        xorq    d1, d1
        addq    a, d0
        adcq    d, d1

        movq    8(x), a
        mulq    m
        xorq    d2, d2
        addq    a, d1
        adcq    d, d2

        movq    16(x), a
        mulq    m
        xorq    d3, d3
        addq    a, d2
        adcq    d, d3

        movq    24(x), a
        mulq    m
        xorq    d4, d4
        addq    a, d3
        adcq    d, d4

        movq    32(x), a
        mulq    m
        xorq    d5, d5
        addq    a, d4
        adcq    d, d5

        movq    40(x), a
        mulq    m
        xorq    d6, d6
        addq    a, d5
        adcq    d, d6

        movq    48(x), a
        mulq    m
        movq    56(x), d7
        movq    64(x), d8
        addq    a, d6
        adcq    $0, d

        movq    $0xFF, a
        andq    d8, a
        leaq    (d8,a,2), d8

        xorl    %eax, %eax
        addq    d7, d
        adcq    a, d8
        addq    d7, d7
        adcq    a, d8
        addq    d, d7
        adcq    a, d8

// Now d8 >= 2^9 <=> x' + x + 1 >= 2^521 <=> x' + x >= p_521.
// If that is the case we want (x' + x) - p_521 = (x' + x + 1) - 2^521
// while otherwise we want just x' + x = (x' + x + 1) - 1.

        cmpq    $0x200, d8

        sbbq    a, d0
        movq    d0, (z)
        sbbq    a, d1
        movq    d1, 8(z)
        sbbq    a, d2
        movq    d2, 16(z)
        sbbq    a, d3
        movq    d3, 24(z)
        sbbq    a, d4
        movq    d4, 32(z)
        sbbq    a, d5
        movq    d5, 40(z)
        sbbq    a, d6
        movq    d6, 48(z)
        sbbq    a, d7
        movq    d7, 56(z)
        sbbq    a, d8
        andq    $0x1FF, d8
        movq    d8, 64(z)

// Restore registers and return

        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p521_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
