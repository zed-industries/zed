// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word, z := c * y
// Inputs c, y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_cmul(uint64_t k, uint64_t *z, uint64_t c, uint64_t n,
//                                const uint64_t *y);
//
// Does the "z := c * y" operation where y is n digits, result z is p.
// Truncates the result in general unless p >= n + 1.
//
// The return value is a high/carry word that is meaningful when p >= n as
// giving the high part of the result. Since this is always zero if p > n,
// it is mainly of interest in the special case p = n, i.e. where the source
// and destination have the same nominal size, when it gives the extra word
// of the full result.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = c, RCX = n, R8 = y, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = c, R9 = n, [RSP+40] = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul)
        .text

#define p %rdi
#define z %rsi
#define c %r9
#define n %rcx
#define x %r8

#define i %r10
#define h %r11



S2N_BN_SYMBOL(bignum_cmul):
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
// past the p'th term of the input to generate p-digit output. Now we can
// assume that n <= p

        cmpq    n, p
        cmovcq  p, n

// Initialize current input/output pointer offset i and high part h.
// But then if n = 0 skip the multiplication and go to the tail part

        xorq    h, h
        xorq    i, i
        testq   n, n
        jz      Lbignum_cmul_tail

// Move c into a safer register as multiplies overwrite %rdx

        movq    %rdx, c

// Initialization of the loop: [h,l] = c * x_0

        movq    (x), %rax
        mulq    c
        movq    %rax, (z)
        movq    %rdx, h
        incq    i
        cmpq    n, i
        jz      Lbignum_cmul_tail

// Main loop doing the multiplications

Lbignum_cmul_loop:
        movq    (x,i,8), %rax
        mulq    c
        addq    h, %rax
        adcq    $0, %rdx
        movq    %rax, (z,i,8)
        movq    %rdx, h
        incq    i
        cmpq    n, i
        jc      Lbignum_cmul_loop

// Add a tail when the destination is longer

Lbignum_cmul_tail:
        cmpq    p, i
        jnc     Lbignum_cmul_end
        movq    h, (z,i,8)
        xorq    h, h
        incq    i
        cmpq    p, i
        jnc     Lbignum_cmul_end

Lbignum_cmul_tloop:
        movq    h, (z,i,8)
        incq    i
        cmpq    p, i
        jc      Lbignum_cmul_tloop

// Return the high/carry word

Lbignum_cmul_end:
        movq    h, %rax

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
