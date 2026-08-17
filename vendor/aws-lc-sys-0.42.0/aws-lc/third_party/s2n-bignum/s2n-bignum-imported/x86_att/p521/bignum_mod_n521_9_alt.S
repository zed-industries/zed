// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_521
// Input x[9]; output z[9]
//
//    extern void bignum_mod_n521_9_alt(uint64_t z[static 9],
//                                      const uint64_t x[static 9]);
//
// Reduction is modulo the group order of the NIST curve P-521.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n521_9_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n521_9_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n521_9_alt)
        .text

#define z %rdi
#define x %rsi

#define q %rcx
#define a %rax
#define d %rdx

#define c %rcx

#define n0 %r8
#define n1 %r9
#define n2 %r10
#define n3 %r11

#define ashort %eax
#define cshort %ecx
#define qshort %edx

S2N_BN_SYMBOL(bignum_mod_n521_9_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the top digit, putting a bit-stuffed version in output buffer.
// The initial quotient estimate is q = h + 1 where x = 2^521 * h + t
// The last add also clears the CF and OF flags ready for the carry chain.

        movq    64(x), q
        movq    $~0x1FF, a
        orq     q, a
        movq    a, 64(z)
        shrq    $9, q
        addq    $1, q

// Now load other digits and form r = x - q * n_521 = (q * r_521 + t) - 2^521,
// which is stored in the output buffer. Thanks to the bit-stuffing at the
// start, we get r' = (q * r_521 + t) + (2^576 - 2^521) = r + 2^576 as the
// computed result including the top carry. Hence CF <=> r >= 0, while
// r' == r (mod 2^521) because things below bit 521 are uncorrupted. We
// keep the top word in the register c since we at least have that one free.

        movq    $0x449048e16ec79bf7, %rax
        mulq    q
        movq    %rax, n0
        movq    %rdx, n1

        movq    $0xc44a36477663b851, %rax
        mulq    q
        xorq    n2, n2
        addq    %rax, n1
        adcq    %rdx, n2

        movq    $0x8033feb708f65a2f, %rax
        mulq    q
        xorq    n3, n3
        addq    %rax, n2
        adcq    %rdx, n3

        movq    $0xae79787c40d06994, %rax
        mulq    q
        imulq   $5, q
        addq    %rax, n3
        adcq    %rdx, q
        sbbq    %rdx, %rdx
        negq    %rdx

// [%rdx;q;n3;n2;n1;n0] = q * r_521

        xorl    %eax, %eax // %rax is used as a zero hereafter
        addq    (x), n0
        movq    n0, (z)
        adcq    8(x), n1
        movq    n1, 8(z)
        adcq    16(x), n2
        movq    n2, 16(z)
        adcq    24(x), n3
        movq    n3, 24(z)
        adcq    32(x), q
        movq    q, 32(z)
        adcq    40(x), %rdx
        movq    %rdx, 40(z)
        movq    48(x), d
        adcq    %rax, d
        movq    d, 48(z)
        movq    56(x), d
        adcq    %rax, d
        movq    d, 56(z)
        movq    64(z), c
        adcq    %rax, c

// We already know r < n_521, but if it actually went negative then
// we need to add back n_521 again. Use d as a bitmask for r < n_521,
// and just subtract r_521 and mask rather than literally adding 2^521.
// This also gets rid of the bit-stuffing above.

        cmc
        sbbq    d, d

        movq    $0x449048e16ec79bf7, n0
        andq    d, n0
        movq    $0xc44a36477663b851, n1
        andq    d, n1
        movq    $0x8033feb708f65a2f, n2
        andq    d, n2
        movq    $0xae79787c40d06994, n3
        andq    d, n3
        andq    $5, d

        subq    n0, (z)
        sbbq    n1, 8(z)
        sbbq    n2, 16(z)
        sbbq    n3, 24(z)
        sbbq    d, 32(z)
        sbbq    %rax, 40(z)
        sbbq    %rax, 48(z)
        sbbq    %rax, 56(z)
        sbbl    ashort, cshort
        andl    $0x1FF, cshort
        movq    c, 64(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n521_9_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
