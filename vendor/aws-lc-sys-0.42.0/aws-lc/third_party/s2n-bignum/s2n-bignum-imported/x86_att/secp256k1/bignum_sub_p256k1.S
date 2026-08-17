// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_256k1, z := (x - y) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_sub_p256k1(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p256k1)
        .text

#define z %rdi
#define x %rsi
#define y %rdx

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

#define zero %rax
#define zeroe %eax
#define c %rcx

S2N_BN_SYMBOL(bignum_sub_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Zero a register first

        xorl    zeroe, zeroe

// Load and subtract the two inputs as [d3;d2;d1;d0] = x - y (modulo 2^256)

        movq    (x), d0
        subq    (y), d0
        movq    8(x), d1
        sbbq    8(y), d1
        movq    16(x), d2
        sbbq    16(y), d2
        movq    24(x), d3
        sbbq    24(y), d3

// Now if x < y we want to add back p_256k1, which staying within 4 digits
// means subtracting 4294968273, since p_256k1 = 2^256 - 4294968273.
// Let c be that constant 4294968273 when x < y, zero otherwise.

        movq    $4294968273, c
        cmovncq zero, c

// Now correct by adding masked p_256k1, i.e. subtracting c, and write back

        subq    c, d0
        movq    d0, (z)
        sbbq    zero, d1
        movq    d1, 8(z)
        sbbq    zero, d2
        movq    d2, 16(z)
        sbbq    zero, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
