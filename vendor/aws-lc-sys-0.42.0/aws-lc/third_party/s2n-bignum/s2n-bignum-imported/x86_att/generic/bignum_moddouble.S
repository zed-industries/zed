// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo m, z := (2 * x) mod m, assuming x reduced
// Inputs x[k], m[k]; output z[k]
//
//    extern void bignum_moddouble(uint64_t k, uint64_t *z, const uint64_t *x,
//                                 const uint64_t *m);
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = m
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_moddouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_moddouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_moddouble)
        .text

#define k %rdi
#define z %rsi
#define x %rdx
#define m %rcx
#define i %r8
#define a %r9
#define c %rax
#define b %r10

S2N_BN_SYMBOL(bignum_moddouble):
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

// If k = 0 do nothing

        testq   k, k
        jz      Lbignum_moddouble_end

// Do (_::z) = 2 * x - m and generate a mask in c for 2 * x < m

        xorq    c, c
        xorq    i, i
        xorq    b, b

Lbignum_moddouble_dubloop:
        movq    (x,i,8), a
        shrdq   $63, a, c
        negq    b
        sbbq    (m,i,8), c
        sbbq    b, b
        movq    c, (z,i,8)
        movq    a, c
        incq    i
        cmpq    k, i
        jc      Lbignum_moddouble_dubloop
        shrq    $63, c

        addq    b, c

// Now do a corrective masked addition z := z + [c] * m

        xorq    i, i
        xorq    b, b
Lbignum_moddouble_corrloop:
        movq    (m,i,8), a
        andq    c, a
        negq    b
        adcq    (z,i,8), a
        sbbq    b, b
        movq    a, (z,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_moddouble_corrloop

Lbignum_moddouble_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_moddouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
