// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select bitfield starting at bit n with length l <= 64
// Inputs x[k], n, l; output function return
//
//    extern uint64_t bignum_bitfield(uint64_t k, const uint64_t *x, uint64_t n,
//                                    uint64_t l);
//
// One-word bitfield from a k-digit (digit=64 bits) bignum, in constant-time
// style. Bitfield starts at bit n and has length l, indexing from 0 (=LSB).
// Digits above the top are treated uniformly as zero, as usual. Since the
// result is returned in a single word, effectively we use l' = min(64,l)
// for the length.
//
// Standard x86-64 ABI: RDI = k, RSI = x, RDX = n, RCX = l, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, R8 = n, R9 = l, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_bitfield)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_bitfield)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_bitfield)
        .text

#define k %rdi
#define x %rsi
#define n %rdx
#define l %rcx

#define d %r8
#define e %rax
#define i %r9
#define a %r10
#define m %r11

#define mshort %r11d



S2N_BN_SYMBOL(bignum_bitfield):
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

// Initialize second of digit pair to zero and if length is zero finish
// immediately; the digit e is also the return value in RAX

        xorq    e, e
        testq   k, k
        jz      Lbignum_bitfield_end

// Decompose the index into n = 64 * n + m, then increment n for next part

        movl    $63, mshort
        andq    n, m
        shrq    $6, n
        incq    n

// Run over the digits setting d = n'th and e = (n+1)'th

        xorq    i, i
Lbignum_bitfield_loop:
        movq    (x,i,8), a
        cmpq    n, i
        cmovcq  a, d
        cmovzq  a, e
        incq    i
        cmpq    k, i
        jc      Lbignum_bitfield_loop

// Put zero in a register, for several purposes

        xorq    a, a

// Override d with 0 if we ran off the end (e will retain original 0).

        cmpq    n, i
        cmovcq  a, d

// Override e if we have m = 0 (i.e. original n was divisible by 64)
// This is because then we want to shift it right by 64 below.

        testq   m, m
        cmovzq  a, e

// Create a size-l bitmask first (while the shift is conveniently in CL)

        cmpq    $64, l
        adcq    a, a
        shlq    %cl, a
        decq    a

// Combine shifted digits to get the bitfield(n,64)

        movq    m, l
        shrq    %cl, d
        negq    %rcx
        shlq    %cl, e
        orq     d, e

// Now mask it down to get bitfield (n,l)

        andq    a, e

Lbignum_bitfield_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_bitfield)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
