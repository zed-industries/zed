// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_25519, z := (c * x) mod p_25519, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p25519(uint64_t z[static 4], uint64_t c,
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = c, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = c, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p25519)
        .text

#define z %rdi

// Temporarily moved here for initial multiply

#define x %rcx
#define c %rcx
#define cshort %ecx

// Used as a zero register after the initial move

#define zero %rsi
#define zeroe %esi

// Likewise this is thrown away after initial multiply

#define d %rdx
#define a %rax
#define ashort %eax

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

S2N_BN_SYMBOL(bignum_cmul_p25519):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Shuffle inputs (since we want multiplier in %rdx)

        movq    %rdx, x
        movq    %rsi, d

// Multiply, accumulating the result as [d4;d3;d2;d1;d0]

        mulxq   (x), d0, d1
        mulxq   8(x), a, d2
        addq    a, d1
        mulxq   16(x), a, d3
        adcq    a, d2
        mulxq   24(x), d, a
        adcq    d, d3
        adcq    $0, a

// Let [d4;d3;d2;d1;d0] = 2^255 * h + l, and use q = h + 1 as the initial
// quotient estimate, which is either right or 1 too big.

        shldq   $1, d3, a
        movl    $19, cshort
        incq    a
        bts     $63, d3
        mulq    c
        xorl    zeroe, zeroe
        addq    a, d0
        adcq    d, d1
        adcq    zero, d2
        adcq    zero, d3

// Correct if CF = 0 by subtracting 19, either way masking to
// 255 bits, i.e. by effectively adding p_25519 to the "full" answer

        cmovcq  zero, c
        subq    c, d0
        sbbq    zero, d1
        sbbq    zero, d2
        sbbq    zero, d3
        btr     $63, d3

// Write everything back

        movq    d0, (z)
        movq    d1, 8(z)
        movq    d2, 16(z)
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
