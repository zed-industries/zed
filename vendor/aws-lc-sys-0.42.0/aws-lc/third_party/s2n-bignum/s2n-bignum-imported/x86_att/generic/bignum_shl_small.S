// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Shift bignum left by c < 64 bits z := x * 2^c
// Inputs x[n], c; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_shl_small(uint64_t k, uint64_t *z, uint64_t n,
//                                     const uint64_t *x, uint64_t c);
//
// Does the "z := x << c" operation where x is n digits, result z is p.
// The shift count c is masked to 6 bits so it actually uses c' = c mod 64.
// The return value is the "next word" of a p+1 bit result, if n <= p.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = n, RCX = x, R8 = c, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = n, R9 = x, [RSP+40] = c, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_shl_small)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_shl_small)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_shl_small)
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
#define i %r11



S2N_BN_SYMBOL(bignum_shl_small):
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

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.

        cmpq    n, p
        cmovcq  p, n

// Initialize "previous word" carry b to zero and main index i also to zero.
// Then just skip the main loop if n = 0

        xorq    b, b
        xorq    i, i

        testq   n, n
        jz      Lbignum_shl_small_tail

// Reshuffle registers to put the shift count into CL

        movq    %rcx, x
        movq    %r8, c

// Now the main loop

Lbignum_shl_small_loop:
        movq    (x,i,8), a
        movq    a, t
        shldq   %cl, b, a
        movq    a, (z,i,8)
        movq    t, b
        incq    i
        cmpq    n, i
        jc      Lbignum_shl_small_loop

// Shift the top word correspondingly. Using shld one more time is easier
// than carefully producing a complementary shift with care over the zero case

        xorq    a, a
        shldq   %cl, b, a
        movq    a, b

// If we are at the end, finish, otherwise write carry word then zeros

Lbignum_shl_small_tail:
        cmpq    p, i
        jnc     Lbignum_shl_small_end
        movq    b, (z,i,8)
        xorq    b, b
        incq    i
        cmpq    p, i
        jnc     Lbignum_shl_small_end

Lbignum_shl_small_tloop:
        movq    b, (z,i,8)
        incq    i
        cmpq    p, i
        jc      Lbignum_shl_small_tloop

// Return, with RAX = b as the top word

Lbignum_shl_small_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_shl_small)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
