// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended Montgomery reduce, returning results in input-output buffer
// Inputs z[2*k], m[k], w; outputs function return (extra result bit) and z[2*k]
//
//    extern uint64_t bignum_emontredc(uint64_t k, uint64_t *z, const uint64_t *m,
//                                     uint64_t w);
//
// Assumes that z initially holds a 2k-digit bignum z_0, m is a k-digit odd
// bignum and m * w == -1 (mod 2^64). This function also uses z for the output
// as well as returning a carry c of 0 or 1. This encodes two numbers: in the
// lower half of the z buffer we have q = z[0..k-1], while the upper half
// together with the carry gives r = 2^{64k}*c + z[k..2k-1]. These values
// satisfy z_0 + q * m = 2^{64k} * r, i.e. r gives a raw (unreduced) Montgomery
// reduction while q gives the multiplier that was used. Another way of
// thinking of it is that if z' is the output z with the lower half replaced
// with zeros, then z_0 + q * m = 2^{128k} * c + z'.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = m, RCX = w, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = m, R9 = w, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_emontredc)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc)
        .text

// Argument m comes in in %rdx but we copy it to %r8

#define k %rdi
#define z %rsi
#define m %r8
#define w %rcx

// General temp, low part of product and mul input
#define a %rax
// General temp, High part of product
#define b %rdx
// Home for i'th digit or Montgomery multiplier
#define d %rbx

// Outer loop counter
#define i %r9
// Inner loop counter
#define j %r10

#define h %r11
#define e %r12
#define t %r13
#define c %r14

#define cshort %r14d
#define jshort %r10d


S2N_BN_SYMBOL(bignum_emontredc):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
#endif

// Save registers

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)

// Initialize top carry to zero immediately to catch the k = 0 case

        xorq    c, c

// If k = 0 the whole operation is trivial

        testq   k, k
        jz      Lbignum_emontredc_end

// Move m into its permanent home since we need RDX for muls

        movq    %rdx, m

// Launch into the outer loop

        xorq    i, i
Lbignum_emontredc_outerloop:
        movq    (z), e
        movq    w, d
        imulq   e, d
        movq    (m), a
        mulq    d
        movq    d, (z)
        addq    e, a // Will be zero but want the carry
        movq    %rdx, h
        movl    $1, jshort
        movq    k, t
        decq    t
        jz      Lbignum_emontredc_montend

Lbignum_emontredc_montloop:
        adcq    (z,j,8), h
        sbbq    e, e
        movq    (m,j,8), a
        mulq    d
        subq    e, %rdx
        addq    h, a
        movq    a, (z,j,8)
        movq    %rdx, h
        incq    j
        decq    t
        jnz     Lbignum_emontredc_montloop

Lbignum_emontredc_montend:
        adcq    c, h
        movl    $0, cshort
        adcq    $0, c
        movq    (z,k,8), a
        addq    h, a
        movq    a, (z,k,8)
        adcq    $0, c

// End of outer loop.

        addq    $8, z // For simple indexing, z pointer moves
        incq    i
        cmpq    k, i
        jc      Lbignum_emontredc_outerloop

Lbignum_emontredc_end:

// Put the top carry in the expected place, restore registers and return

        movq    c, %rax
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_emontredc)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
