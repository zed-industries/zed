// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// 256-bit multiplex/select z := x (if p nonzero) or z := y (if p zero)
// Inputs p, x[4], y[4]; output z[4]
//
//    extern void bignum_mux_4(uint64_t p, uint64_t z[static 4],
//                             const uint64_t x[static 4],
//                             const uint64_t y[static 4]);
//
// It is assumed that all numbers x, y and z have the same size 4 digits.
//
// Standard x86-64 ABI: RDI = p, RSI = z, RDX = x, RCX = y
// Microsoft x64 ABI:   RCX = p, RDX = z, R8 = x, R9 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mux_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mux_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mux_4)
        .text

#define p %rdi
#define z %rsi
#define x %rdx
#define y %rcx
#define a %rax
#define b %r8


S2N_BN_SYMBOL(bignum_mux_4):
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
        testq   p, p

        movq    (x), a
        movq    (y), b
        cmovzq  b, a
        movq    a, (z)

        movq    8(x), a
        movq    8(y), b
        cmovzq  b, a
        movq    a, 8(z)

        movq    16(x), a
        movq    16(y), b
        cmovzq  b, a
        movq    a, 16(z)

        movq    24(x), a
        movq    24(y), b
        cmovzq  b, a
        movq    a, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mux_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
