// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_521
// Input x[9]; output z[9]
//
//    extern void bignum_mod_n521_9(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Reduction is modulo the group order of the NIST curve P-521.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n521_9)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n521_9)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n521_9)
        .text

#define z %rdi
#define x %rsi

#define q %rdx
#define a %rax

#define c %rcx
#define d %r8

#define n0 %r9
#define n1 %r10
#define n2 %r11
#define n3 d

#define ashort %eax
#define cshort %ecx
#define qshort %edx

S2N_BN_SYMBOL(bignum_mod_n521_9):
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

        movq    $0x449048e16ec79bf7, n0
        mulxq   n0, a, c
        adcxq   (x), a
        movq    a, (z)

        movq    $0xc44a36477663b851, n1
        mulxq   n1, a, d
        adcxq   8(x), a
        adoxq   c, a
        movq    a, 8(z)

        movq    $0x8033feb708f65a2f, n2
        mulxq   n2, a, c
        adcxq   16(x), a
        adoxq   d, a
        movq    a, 16(z)

        movq    $0xae79787c40d06994, a
        mulxq   a, a, d
        adcxq   24(x), a
        adoxq   c, a
        movq    a, 24(z)

        movl    $5, ashort
        mulxq   a, a, c
        adcxq   32(x), a
        adoxq   d, a
        movq    a, 32(z)

        movq    c, a // a is now used for zero hereafter
        adoxq   c, c
        adcq    40(x), c
        movq    c, 40(z)

        movq    48(x), c
        adcq    a, c
        movq    c, 48(z)

        movq    56(x), c
        adcq    a, c
        movq    c, 56(z)

        movq    64(z), c
        adcq    a, c

// We already know r < n_521, but if it actually went negative then
// we need to add back n_521 again. Recycle q as a bitmask for r < n_521,
// and just subtract r_521 and mask rather than literally adding 2^521.
// This also gets rid of the bit-stuffing above.

        cmc
        sbbq    q, q
        andq    q, n0
        andq    q, n1
        andq    q, n2
        movq    $0xae79787c40d06994, n3
        andq    q, n3
        andl    $5, qshort
        subq    n0, (z)
        sbbq    n1, 8(z)
        sbbq    n2, 16(z)
        sbbq    n3, 24(z)
        sbbq    q, 32(z)
        sbbq    a, 40(z)
        sbbq    a, 48(z)
        sbbq    a, 56(z)
        sbbl    ashort, cshort
        andl    $0x1FF, cshort
        movq    c, 64(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n521_9)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
