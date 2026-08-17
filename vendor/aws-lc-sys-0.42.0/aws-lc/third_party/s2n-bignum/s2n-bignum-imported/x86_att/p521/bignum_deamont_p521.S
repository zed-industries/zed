// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^576) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_deamont_p521(uint64_t z[static 9],
//                                    const uint64_t x[static 9]);
//
// Convert a 9-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 9-digit input will work, with no range restriction.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p521)
        .text

#define z %rdi
#define x %rsi

#define c %rax
#define h %rax
#define l %rbx

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define d6 %r12
#define d7 %r13
#define d8 %rbp

S2N_BN_SYMBOL(bignum_deamont_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%rbp)

// Stash the lowest 55 bits at the top of c, then shift the whole 576-bit
// input right by 9*64 - 521 = 576 - 521 = 55 bits.

        movq    (x), d0
        movq    d0, c
        shlq    $9, c
        movq    8(x), d1
        shrdq   $55, d1, d0
        movq    16(x), d2
        shrdq   $55, d2, d1
        movq    24(x), d3
        shrdq   $55, d3, d2
        movq    32(x), d4
        shrdq   $55, d4, d3
        movq    40(x), d5
        shrdq   $55, d5, d4
        movq    48(x), d6
        shrdq   $55, d6, d5
        movq    56(x), d7
        shrdq   $55, d7, d6
        movq    64(x), d8
        shrdq   $55, d8, d7
        shrq    $55, d8

// Now writing x = 2^55 * h + l (so here [d8;..d0] = h and c = 2^9 * l)
// we want (h + 2^{521-55} * l) mod p_521 = s mod p_521. Since s < 2 * p_521
// this is just "if s >= p_521 then s - p_521 else s". First get
// s + 1, but pad up the top to get a top-bit carry-out from it, so now
// CF <=> s + 1 >= 2^521 <=> s >= p_521, while the digits [d8;...d0] are
// now s + 1 except for bits above 521.

        movq    c, l
        shrq    $55, h
        shlq    $9, l
        orq     $~0x1FF, d8
        addq    $1, d0
        adcq    $0, d1
        adcq    $0, d2
        adcq    $0, d3
        adcq    $0, d4
        adcq    $0, d5
        adcq    $0, d6
        adcq    l, d7
        adcq    h, d8

// We want "if CF then (s + 1) - 2^521 else (s + 1) - 1" so subtract ~CF
// and mask to 521 bits, writing digits back as they are created.

        cmc
        sbbq    $0, d0
        movq    d0, (z)
        sbbq    $0, d1
        movq    d1, 8(z)
        sbbq    $0, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)
        sbbq    $0, d6
        movq    d6, 48(z)
        sbbq    $0, d7
        movq    d7, 56(z)
        sbbq    $0, d8
        andq    $0x1FF, d8
        movq    d8, 64(z)

// Restore registers and return

        CFI_POP(%rbp)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
