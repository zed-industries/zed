// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_521, z := (c * x) mod p_521, assuming
// x reduced
// Inputs c, x[9]; output z[9]
//
//    extern void bignum_cmul_p521_alt(uint64_t z[static 9], uint64_t c,
//                                     const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p521_alt)
        .text

#define z %rdi

// Temporarily moved here for initial multiply

#define x %rcx

// Likewise this is thrown away after initial multiply

#define m %rsi

#define c %rdx
#define cshort %edx

#define a %rax
#define d %rdx

#define dd %rax

// Digits: last ones aliased to inputs that are no longer used then

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11
#define d4 %rbx
#define d5 %rbp
#define d6 %r12
#define d7 %r13
#define d8 %rcx
#define d9 %rsi

// Same as d9

#define h d9

S2N_BN_SYMBOL(bignum_cmul_p521_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save additional registers to use

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)

// Shuffle inputs (since we want %rdx for the high parts of products)

        movq    %rdx, x

// Multiply as [d9; ...; d0] = c * x.

        movq    (x), a
        mulq    m
        movq    a, d0
        movq    d, d1

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
        xorq    d7, d7
        addq    a, d6
        adcq    d, d7

        movq    56(x), a
        mulq    m
        addq    a, d7
        movq    64(x), a
        movq    $0, d8
        adcq    d, d8
        mulq    m
        xorq    d9, d9
        addq    a, d8
        adcq    d, d9

// Create an AND "dd" of digits d7,...,d1, a computation we hope will
// get nicely interleaved with the multiplication chain above, though
// we can't do so directly as we are using the same register %rax.

        movq    d1, dd
        andq    d2, dd
        andq    d3, dd
        andq    d4, dd
        andq    d5, dd
        andq    d6, dd
        andq    d7, dd

// Extract the high part h==d9 and mask off the low part l = [d8;d7;...;d0]
// but stuff d8 with 1 bits at the left to ease a comparison below

        shldq   $55, d8, h
        orq     $~0x1FF, d8

// Decide whether h + l >= p_521 <=> h + l + 1 >= 2^521. Since this can only
// happen if digits d7,...d1 are all 1s, we use the AND of them "dd" to
// condense the carry chain, and since we stuffed 1 bits into d8 we get
// the result in CF without an additional comparison. Hereafter we use c = 0.
// Since x was assumed reduced, h cannot be maximal, so the "lea" is safe,
// i.e. does not carry or wrap round.

        leaq    1(h), c
        addq    d0, c
        movl    $0, cshort
        adcq    c, dd
        movq    d8, a
        adcq    c, a

// Now if CF is set we want (h + l) - p_521 = (h + l + 1) - 2^521
// while otherwise we want just h + l. So mask h + l + CF to 521 bits.
// This masking also gets rid of the stuffing with 1s we did above.
// Write back the digits as they are generated.

        adcq    h, d0
        movq    d0, (z)
        adcq    c, d1
        movq    d1, 8(z)
        adcq    c, d2
        movq    d2, 16(z)
        adcq    c, d3
        movq    d3, 24(z)
        adcq    c, d4
        movq    d4, 32(z)
        adcq    c, d5
        movq    d5, 40(z)
        adcq    c, d6
        movq    d6, 48(z)
        adcq    c, d7
        movq    d7, 56(z)
        adcq    c, d8
        andq    $0x1FF, d8
        movq    d8, 64(z)

// Restore registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p521_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
