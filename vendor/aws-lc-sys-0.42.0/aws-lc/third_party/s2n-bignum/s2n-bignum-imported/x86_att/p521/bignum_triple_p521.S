// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_521, z := (3 * x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_triple_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p521)
        .text

#define z %rdi
#define x %rsi

// d7 re-uses the input pointer when safe to do so

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define d6 %r12
#define d7 %rsi
#define d8 %rdx

#define m %rbx
#define mshort %ebx



S2N_BN_SYMBOL(bignum_triple_p521):
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

// Load the top (short) word first to compute the initial carry-in
// Set OF according to bit 520, but *always* set CF to get a +1 bump

        movq    64(x), m
        movq    m, d8
        shlq    $54, m
        addq    m, m
        stc

// Use a double carry chain to compute x' + x + 1 where x' is a
// 1-bit left rotation of x; this is then == 3 * x + 1 (mod p_521)
// This gives us s = [d8;d7;d6;d5;d4;d3;d2;d1;d0] = x + x' + 1.

        movq    (x), m
        movq    m, d0
        adcxq   m, m
        adoxq   m, d0
        movq    8(x), m
        movq    m, d1
        adcxq   m, m
        adoxq   m, d1
        movq    16(x), m
        movq    m, d2
        adcxq   m, m
        adoxq   m, d2
        movq    24(x), m
        movq    m, d3
        adcxq   m, m
        adoxq   m, d3
        movq    32(x), m
        movq    m, d4
        adcxq   m, m
        adoxq   m, d4
        movq    40(x), m
        movq    m, d5
        adcxq   m, m
        adoxq   m, d5
        movq    48(x), m
        movq    m, d6
        adcxq   m, m
        adoxq   m, d6
        movq    56(x), m
        movq    m, d7
        adcxq   m, m
        adoxq   m, d7

// The last word is slightly more intricate: we naturally end up adding
// 2 * top bit when we shouldn't (because it's a rotation and we've already
// added it at the LSB position) but then compensate by subtracting it.

        movq    d8, m
        adcxq   m, m
        adoxq   m, d8
        andq    $0x200, m
        subq    m, d8

// Now x + x' >= p_521 <=> s = x + x' + 1 >= 2^521
// Make m = 512 * [x + x' >= p_521]

        movl    $512, mshort
        andq    d8, m

// Now if x + x' >= p_521, we want (x + x') - p_521 = s - 2^521
// while otherwise we want x + x' = s - 1
// We use the mask m both as an operand and to generate the dual carry
// Write back the results as generated

        cmpq    $512, m

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
        sbbq    m, d8
        movq    d8, 64(z)

// Restore registers and return

        CFI_POP(%r12)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
