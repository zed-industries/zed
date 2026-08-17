// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add modulo m, z := (x + y) mod m, assuming x and y reduced
// Inputs x[k], y[k], m[k]; output z[k]
//
//    extern void bignum_modadd(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *y, const uint64_t *m);
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = y, R8 = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = y, [RSP+40] = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modadd)
        .text

#define k %rdi
#define z %rsi
#define x %rdx
#define y %rcx
#define m %r8
#define i %r9
#define j %r10
#define a %rax
#define c %r11

S2N_BN_SYMBOL(bignum_modadd):
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

// If k = 0 do nothing

        testq   k, k
        jz      Lbignum_modadd_end

// First just add (c::z) := x + y

        xorq    c, c
        movq    k, j
        xorq    i, i
Lbignum_modadd_addloop:
        movq    (x,i,8), a
        adcq    (y,i,8), a
        movq    a, (z,i,8)
        incq    i
        decq    j
        jnz     Lbignum_modadd_addloop
        adcq    $0, c

// Now do a comparison subtraction (c::z) - m, recording mask for (c::z) >= m

        movq    k, j
        xorq    i, i
Lbignum_modadd_cmploop:
        movq    (z,i,8), a
        sbbq    (m,i,8), a
        incq    i
        decq    j
        jnz     Lbignum_modadd_cmploop
        sbbq    $0, c
        notq    c

// Now do a masked subtraction z := z - [c] * m

        xorq    i, i
Lbignum_modadd_subloop:
        movq    (m,i,8), a
        andq    c, a
        negq    j
        sbbq    a, (z,i,8)
        sbbq    j, j
        incq    i
        cmpq    k, i
        jc      Lbignum_modadd_subloop

Lbignum_modadd_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
