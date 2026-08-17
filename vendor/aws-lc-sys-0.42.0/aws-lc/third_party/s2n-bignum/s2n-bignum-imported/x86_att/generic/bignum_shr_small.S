// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Shift bignum right by c < 64 bits z := floor(x / 2^c)
// Inputs x[n], c; outputs function return (bits shifted out) and z[k]
//
//    extern uint64_t bignum_shr_small(uint64_t k, uint64_t *z, uint64_t n,
//                                     const uint64_t *x, uint64_t c);
//
// Does the "z := x >> c" operation where x is n digits, result z is p.
// The shift count c is masked to 6 bits so it actually uses c' = c mod 64.
// The return value is the inout mod 2^c'.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x, R8 = c, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x, [RSP+40] = c, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_shr_small)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_shr_small)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_shr_small)
        .text

#define p %rdi
#define z %rsi
#define n %rdx

// These get moved from their initial positions

#define c %rcx
#define x %r9

// Other variables

#define b %rax
#define t %r8
#define a %r10

#define ashort %r10d



S2N_BN_SYMBOL(bignum_shr_small):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        movq    56(%rsp), %r8
#endif

// Reshuffle registers to put the shift count into CL

        movq    %rcx, x
        movq    %r8, c

// Set default carry-in word to 0, useful for other things too

        xorq    b, b

// First, if p > n then pad output on the left with p-n zeros

        cmpq    p, n
        jnc     Lbignum_shr_small_nopad
Lbignum_shr_small_padloop:
        decq    p
        movq    b, (z,p,8)
        cmpq    p, n
        jc      Lbignum_shr_small_padloop
Lbignum_shr_small_nopad:

// We now know that p <= n. If in fact p < n let carry word = x[p] instead of 0

        jz      Lbignum_shr_small_shiftstart
        movq    (x,p,8), b
Lbignum_shr_small_shiftstart:
        testq   p, p
        jz      Lbignum_shr_small_trivial

// Now the main loop

Lbignum_shr_small_loop:
        movq    -8(x,p,8), a
        movq    a, t
        shrdq   %cl, b, a
        movq    a, -8(z,p,8)
        movq    t, b
        decq    p
        jnz     Lbignum_shr_small_loop

// Mask the carry word and return with that as RAX = b

Lbignum_shr_small_trivial:
        movl    $1, ashort
        shlq    %cl, a
        decq    a
        andq    a, b

Lbignum_shr_small_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_shr_small)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
