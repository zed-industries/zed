// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add with single-word multiplier, z := z + c * y
// Inputs c, y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_cmadd(uint64_t k, uint64_t *z, uint64_t c, uint64_t n,
//                                 const uint64_t *y);
//
// Does the "z := z + c * y" operation where y is n digits, result z is p.
// Truncates the result in general.
//
// The return value is a high/carry word that is meaningful when p = n + 1, or
// more generally when n <= p and the result fits in p + 1 digits. In these
// cases it gives the top digit of the (p + 1)-digit result.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = c, RCX = n, R8 = y, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = c, R9 = n, [RSP+40] = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmadd)
        .text

#define p %rdi
#define z %rsi
#define c %r9
#define n %rcx
#define x %r8

#define i %r10
#define h %r11

#define r %rbx

#define hshort %r11d
#define ishort %r10d



S2N_BN_SYMBOL(bignum_cmadd):
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

// Seems hard to avoid one more register

        CFI_PUSH(%rbx)

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.
// Subtract p := p - min(n,p) so it holds the size of the extra tail needed

        cmpq    n, p
        cmovcq  p, n
        subq    n, p

// Initialize high part h = 0; if n = 0 do nothing but return that zero

        xorq    h, h
        testq   n, n
        jz      Lbignum_cmadd_end

// Move c into a safer register as multiplies overwrite %rdx

        movq    %rdx, c

// Initialization of the loop: 2^64 * CF + [h,z_0'] = z_0 + c * x_0

        movq    (x), %rax
        mulq    c
        addq    %rax, (z)
        movq    %rdx, h
        movl    $1, ishort
        decq    n
        jz      Lbignum_cmadd_hightail

// Main loop, where we always have CF + previous high part h to add in

Lbignum_cmadd_loop:
        adcq    (z,i,8), h
        sbbq    r, r
        movq    (x,i,8), %rax
        mulq    c
        subq    r, %rdx
        addq    h, %rax
        movq    %rax, (z,i,8)
        movq    %rdx, h
        incq    i
        decq    n
        jnz     Lbignum_cmadd_loop

Lbignum_cmadd_hightail:
        adcq    $0, h

// Propagate the carry all the way to the end with h as extra carry word

Lbignum_cmadd_tail:
        testq   p, p
        jz      Lbignum_cmadd_end

        addq    h, (z,i,8)
        movl    $0, hshort
        incq    i
        decq    p
        jz      Lbignum_cmadd_highend

Lbignum_cmadd_tloop:
        adcq    h, (z,i,8)
        incq    i
        decq    p
        jnz     Lbignum_cmadd_tloop

Lbignum_cmadd_highend:

        adcq    $0, h

// Return the high/carry word

Lbignum_cmadd_end:
        movq    h, %rax

        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
