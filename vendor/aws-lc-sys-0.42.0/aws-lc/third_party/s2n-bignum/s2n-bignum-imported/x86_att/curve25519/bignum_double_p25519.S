// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_25519, z := (2 * x) mod p_25519, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_double_p25519(uint64_t z[static 4],
//                                     const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p25519)
        .text

#define z %rdi
#define x %rsi

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

// These also re-use input x when safe to do so

#define c0 %rax
#define c1 %rcx
#define c2 %rsi
#define c3 %rdx
#define c0short %eax
#define c1short %ecx
#define c2short %esi
#define c3short %edx

S2N_BN_SYMBOL(bignum_double_p25519):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Add as [d3; d2; d1; d0] = 2 * x; since we assume x < 2^255 - 19
// this result fits in 256 bits.

        movq    (x), d0
        addq    d0, d0
        movq    8(x), d1
        adcq    d1, d1
        movq    16(x), d2
        adcq    d2, d2
        movq    24(x), d3
        adcq    d3, d3

// Now 2 * x >= 2^255 - 19 <=> 2 * x + 19 >= 2^255.
// Form [c3; c2; c1; c0] = (2 * x) + 19

        movl    $19, c0short
        xorl    c1short, c1short
        xorl    c2short, c2short
        xorl    c3short, c3short

        addq    d0, c0
        adcq    d1, c1
        adcq    d2, c2
        adcq    d3, c3

// Test the top bit to see if this is >= 2^255, and clear it as a masking
// so that in that case the result is exactly (2 * x) - (2^255 - 19).
// Then select the output according to that top bit as that or just 2 * x.

        btr     $63, c3
        cmovcq  c0, d0
        cmovcq  c1, d1
        cmovcq  c2, d2
        cmovcq  c3, d3

// Store the result

        movq    d0, (z)
        movq    d1, 8(z)
        movq    d2, 16(z)
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
