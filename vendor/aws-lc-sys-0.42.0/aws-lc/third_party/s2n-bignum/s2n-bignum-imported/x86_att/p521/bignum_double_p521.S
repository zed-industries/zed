// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_521, z := (2 * x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_double_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p521)
        .text

#define z %rdi
#define x %rsi

#define a %rax
#define c %rcx



S2N_BN_SYMBOL(bignum_double_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// We can decide whether 2 * x >= p_521 just by 2 * x >= 2^521, which
// as we assume x < p_521 amounts to looking at bit 8 of the top word

        movq    64(x), c
        btq     $8, c

// Now if 2 * x >= p_521 we want 2 * x - p_521 = (2 * x + 1) - 2^521
// and otherwise just 2 * x. Feed in the condition as the carry bit
// to get 2 * x + [2 * x >= p_521] then just mask it off to 521 bits.

        movq    (x), a
        adcq    a, a
        movq    a, (z)

        movq    8(x), a
        adcq    a, a
        movq    a, 8(z)

        movq    16(x), a
        adcq    a, a
        movq    a, 16(z)

        movq    24(x), a
        adcq    a, a
        movq    a, 24(z)

        movq    32(x), a
        adcq    a, a
        movq    a, 32(z)

        movq    40(x), a
        adcq    a, a
        movq    a, 40(z)

        movq    48(x), a
        adcq    a, a
        movq    a, 48(z)

        movq    56(x), a
        adcq    a, a
        movq    a, 56(z)

        adcq    c, c
        andq    $0x1FF, c
        movq    c, 64(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
