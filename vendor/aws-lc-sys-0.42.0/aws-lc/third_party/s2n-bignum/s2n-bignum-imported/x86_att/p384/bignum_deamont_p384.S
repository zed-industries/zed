// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from almost-Montgomery form, z := (x / 2^384) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_deamont_p384(uint64_t z[static 6],
//                                    const uint64_t x[static 6]);
//
// Convert a 6-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 6-digit input will work, with no range restriction.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p384)
        .text

#define z %rdi
#define x %rsi

// Additional temps in the correction phase

#define u %rax
#define v %rcx
#define w %rdx

#define vshort %ecx

// Core one-step "short" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing contents of [d5;d4;d3;d2;d1;d0]. This
// is intended only for 6-word inputs as in mapping out of Montgomery,
// not for the general case of Montgomery multiplication. It is fine
// for d6 to be the same register as d0.
//
// Parms:  montreds(d6,d5,d4,d3,d2,d1,d0)
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64

#define montreds(d6,d5,d4,d3,d2,d1,d0)                                  \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64 */         \
        movq    d0, %rdx ;                                        \
        shlq    $32, %rdx ;                                        \
        addq    d0, %rdx ;                                        \
/* Construct [%rsi;%rcx;%rax;-] = (2^384 - p_384) * w           */         \
/* We know the lowest word will cancel so we can re-use d0   */         \
/* as a temp.                                                */         \
        xorq    %rsi, %rsi ;                                       \
        movq    $0xffffffff00000001, %rax ;                        \
        mulxq   %rax, %rcx, %rax ;                                  \
        movl    $0x00000000ffffffff, %ecx ;                        \
        mulxq   %rcx, d0, %rcx ;                                   \
        adcq    d0, %rax ;                                        \
        adcq    %rdx, %rcx ;                                       \
        adcq    $0, %rsi ;                                         \
/* Now subtract that and add 2^384 * w                       */         \
        subq    %rax, d1 ;                                        \
        sbbq    %rcx, d2 ;                                        \
        sbbq    %rsi, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        movq    %rdx, d6 ;                                        \
        sbbq    $0, d6

S2N_BN_SYMBOL(bignum_deamont_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%r12)
        CFI_PUSH(%r13)

// Set up an initial window [%r13,%r12,%r11,%r10,%r9,%r8] = x

        movq    (x), %r8
        movq    8(x), %r9
        movq    16(x), %r10
        movq    24(x), %r11
        movq    32(x), %r12
        movq    40(x), %r13

// Montgomery reduce window 0

        montreds(%r8,%r13,%r12,%r11,%r10,%r9,%r8)

// Montgomery reduce window 1

        montreds(%r9,%r8,%r13,%r12,%r11,%r10,%r9)

// Montgomery reduce window 2

        montreds(%r10,%r9,%r8,%r13,%r12,%r11,%r10)

// Montgomery reduce window 3

        montreds(%r11,%r10,%r9,%r8,%r13,%r12,%r11)

// Montgomery reduce window 4

        montreds(%r12,%r11,%r10,%r9,%r8,%r13,%r12)

// Montgomery reduce window 5

        montreds(%r13,%r12,%r11,%r10,%r9,%r8,%r13)

// Do a test addition of dd = [%r13;%r12;%r11;%r10;%r9;%r8] and
// 2^384 - p_384 = [0;0;0;1;v;u], hence setting CF iff
// dd + (2^384 - p_384) >= 2^384, hence iff dd >= p_384.

        movq    $0xffffffff00000001, u
        movl    $0x00000000ffffffff, vshort

        movq    %r8, w
        addq    u, w
        movq    %r9, w
        adcq    v, w
        movq    %r10, w
        adcq    $1, w
        movq    %r11, w
        adcq    $0, w
        movq    %r12, w
        adcq    $0, w
        movq    %r13, w
        adcq    $0, w

// Convert CF to a bitmask in w

        sbbq    w, w

// Masked addition of 2^384 - p_384, hence subtraction of p_384

        andq    w, u
        andq    w, v
        andq    $1, w

        addq   u, %r8
        adcq   v, %r9
        adcq   w, %r10
        adcq   $0, %r11
        adcq   $0, %r12
        adcq   $0, %r13

// Write back the result

        movq    %r8, (z)
        movq    %r9, 8(z)
        movq    %r10, 16(z)
        movq    %r11, 24(z)
        movq    %r12, 32(z)
        movq    %r13, 40(z)

// Restore registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
