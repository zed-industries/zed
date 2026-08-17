// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo m, z := (x - y) mod m, assuming x and y reduced
// Inputs x[k], y[k], m[k]; output z[k]
//
//    extern void bignum_modsub(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *y, const uint64_t *m);
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = y, R8 = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = y, [RSP+40] = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modsub)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modsub)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modsub)
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

S2N_BN_SYMBOL(bignum_modsub):
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
        jz      Lbignum_modsub_end

// Subtract z := x - y and record a mask for the carry x - y < 0

        xorq    c, c
        movq    k, j
        xorq    i, i
Lbignum_modsub_subloop:
        movq    (x,i,8), a
        sbbq    (y,i,8), a
        movq    a, (z,i,8)
        incq    i
        decq    j
        jnz     Lbignum_modsub_subloop
        sbbq    c, c

// Now do a masked addition z := z + [c] * m

        xorq    i, i
Lbignum_modsub_addloop:
        movq    (m,i,8), a
        andq    c, a
        negq    j
        adcq    a, (z,i,8)
        sbbq    j, j
        incq    i
        cmpq    k, i
        jc      Lbignum_modsub_addloop

Lbignum_modsub_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modsub)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
