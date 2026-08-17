// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from almost-Montgomery form, z := (x / 2^256) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_deamont_sm2(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Convert a 4-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 4-digit input will work, with no range restriction.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_sm2)
        .text

#define z %rdi
#define x %rsi

#define c %rcx
#define n1 %rax
#define n3 %rdx

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d3;d2;d1;d0] and returns result in [d0;d3;d2;d1], adding to the
// existing contents of [d3;d2;d1], and using %rax, %rcx, %rdx and %rsi
// as temporaries.
// ---------------------------------------------------------------------------

#define montreds(d3,d2,d1,d0)                                               \
        movq    d0, %rax ;                                                    \
        shlq    $32, %rax ;                                                    \
        movq    d0, %rcx ;                                                    \
        shrq    $32, %rcx ;                                                    \
        movq    %rax, %rdx ;                                                   \
        movq    %rcx, %rsi ;                                                   \
        subq    d0, %rax ;                                                    \
        sbbq    $0, %rcx ;                                                     \
        subq    %rax, d1 ;                                                    \
        sbbq    %rcx, d2 ;                                                    \
        sbbq    %rdx, d3 ;                                                    \
        sbbq    %rsi, d0

S2N_BN_SYMBOL(bignum_deamont_sm2):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Set up an initial 4-word window [%r11,%r10,%r9,%r8] = x

        movq    (x), %r8
        movq    8(x), %r9
        movq    16(x), %r10
        movq    24(x), %r11

// Systematically scroll left doing 1-step reductions. This process
// keeps things inside 4 digits (i.e. < 2^256) at each stage, since
// we have w * p_sm2 + x <= (2^64 - 1) * p_sm2 + (2 EXP 256 - 1)
// <= (2^64 - 1) * (2^256 - 1) + (2 EXP 256 - 1) <= 2^64 * (2^256 - 1)

        montreds(%r11,%r10,%r9,%r8)

        montreds(%r8,%r11,%r10,%r9)

        montreds(%r9,%r8,%r11,%r10)

        montreds(%r10,%r9,%r8,%r11)

// Let [%r11;%r10;%r9;%r8] := [%r11;%r10;%r9;%r8] - p_sm2, saving constants
// n1 and n3 in [n3; -1; n1; -1] = p_sm2 for later use.

        subq    $-1, %r8
        movq    $0xffffffff00000000, n1
        sbbq    n1, %r9
        sbbq    $-1, %r10
        movq    $0xfffffffeffffffff, n3
        sbbq    n3, %r11

// Capture the carry to determine whether to add back p_sm2, and use
// it to create a masked p_sm2' = [n3; c; n1; c]

        sbbq    c, c
        andq    c, n1
        andq    c, n3

// Do the corrective addition and copy to output

        addq    c, %r8
        movq    %r8, (z)
        adcq    n1, %r9
        movq    %r9, 8(z)
        adcq    c, %r10
        movq    %r10, 16(z)
        adcq    n3, %r11
        movq    %r11, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
