// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Single-word reciprocal, underestimate of 2^128 / a with implicit 1 added
// Input a; output function return
//
//    extern uint64_t word_recip(uint64_t a);
//
// Given an input word "a" with its top bit set (i.e. 2^63 <= a < 2^64), the
// result "x" is implicitly augmented with a leading 1 giving x' = 2^64 + x.
// The result is x' = ceil(2^128 / a) - 1, which except for the single
// special case a = 2^63 is the same thing as x' = floor(2^128 / a).
//
// Standard x86-64 ABI: RDI = a, returns RAX
// Microsoft x64 ABI:   RCX = a, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_recip)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_recip)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_recip)
        .text

#define a %rdi
#define x %rcx
#define b %rsi

# Some aliasing here

#define t %rax
#define l %rax

#define d %rdx
#define h %rdx

S2N_BN_SYMBOL(word_recip):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
#endif

// Scale the input down: b overestimates a/2^16 with b <= 2^48 and
// x underestimates 2^64/b with b * x =~= 2^64, accurate to ~2 bits.

        movq    a, b
        movq    $0x1FFFFFFFFFFFF, x
        shrq    $16, b
        xorq    b, x
        incq    b
        shrq    $32, x

// Suppose x = 2^64/b * (1 - e). and get scaled error d = 2^64 * e

        movq    b, d
        imulq   x, d
        negq    d

// Rescale to give c = 2^15 * e (so c <= 2^13) and compute
// e + e^2 + e^3 + e^4 = (1 + e^2) (e + e^2)
// = (2^30 + c^2) * (2^15 * c + c^2) / 2^60
// and then x * (1 + e + e^2 + e^3 + e^4)
// = (2^30 * x + x * (2^30 + c^2) * (2^30 * c + c^2) / 2^30) / 2^30

        movq    d, t
        shrq    $49, t
        imulq   t, t
        shrq    $34, d
        addq    t, d
        orq     $0x40000000, t
        imulq   d, t
        shrq    $30, t
        imulq   x, t
        shlq    $30, x
        addq    t, x
        shrq    $30, x

// Now b * x =~= 2^64, accurate to ~10 bits.
// Do a 64-bit Newton step, scaling up x by 16 bits in the process.

        movq    b, d
        imulq   x, d
        negq    d
        shrq    $24, d
        imulq   x, d
        shlq    $16, x
        shrq    $24, d
        addq    d, x

// Now b * x =~= 2^80, accurate to ~20 bits.
// Do a 64-bit Newton step, scaling up x by 31 bits in the process

        movq    b, d
        imulq   x, d
        negq    d
        shrq    $32, d
        imulq   x, d
        shlq    $31, x
        shrq    $17, d
        addq    d, x

// Now a * x =~= 2^127, accurate to ~40 bits. Do a Newton step at full size.
// Instead of literally negating the product (h,l) we complement bits in
// the extracted bitfield, which is close enough and a bit faster.
// At the end we also shift x one more bit left, losing the known-1 top bit
// so that a * (2^64 + x) =~= 2^128.

        movq    a, l
        mulq    x
        shrdq   $60, h, l
        movq    x, h
        shrq    $33, h
        notq    l
        imulq   h, l
        shlq    $1, x
        shrq    $33, l
        addq    l, x

// Test if (x' + 1) * a < 2^128 where x' = 2^64 + x, catching the special
// case where x + 1 would wrap, corresponding to input a = 2^63.

        addq    $1, x
        movq    a, l
        sbbq    $0, x
        mulq    x
        movq    x, %rax
        addq    a, h

// Select either x or x + 1 accordingly as the final answer

        sbbq    $0, %rax
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_recip)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
