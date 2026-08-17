// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^576 * x) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_tomont_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p521)
        .text

#define z %rdi
#define x %rsi

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define d6 %rbx

#define d8 %rdx

#define d8short %edx

// Re-use the input pointer as other variable once safe to do so

#define d7 %rsi

S2N_BN_SYMBOL(bignum_tomont_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save one more register

        CFI_PUSH(%rbx)

// Separate out the input into x = 2^521 * H + L, so that x mod p_521 =
// (H + L) mod p_521 = if H + L >= p_521 then H + L - p_521 else H + L.

        movq    64(x), d0
        movl    $0x1FF, d8short
        andq    d0, d8
        shrq    $9, d0

// Force carry-in to get s = [d8;d7;d6;d5;d4;d3;d2;d1;d0] = H + L + 1.

        stc
        adcq    (x), d0
        movq    8(x), d1
        adcq    $0, d1
        movq    16(x), d2
        adcq    $0, d2
        movq    24(x), d3
        adcq    $0, d3
        movq    32(x), d4
        adcq    $0, d4
        movq    40(x), d5
        adcq    $0, d5
        movq    48(x), d6
        adcq    $0, d6
        movq    56(x), d7
        adcq    $0, d7
        adcq    $0, d8

// Set CF <=> s < 2^521 <=> H + L < p_521, so that if CF is set
// we want H + L = s - 1, otherwise (H + L) - p_521 = s - 2^521.
// This is done with just s - CF then masking to 521 bits.

        cmpq    $512, d8

        sbbq    $0, d0
        sbbq    $0, d1
        sbbq    $0, d2
        sbbq    $0, d3
        sbbq    $0, d4
        sbbq    $0, d5
        sbbq    $0, d6
        sbbq    $0, d7
        sbbq    $0, d8

// So far, this is just a modular reduction as in bignum_mod_p521_9,
// except that the final masking of d8 is skipped since that comes out
// in the wash anyway from the next block, which is the Montgomery map,
// multiplying by 2^576 modulo p_521. Because 2^521 == 1 (mod p_521)
// this is just rotation left by 576 - 521 = 55 bits. Store back
// digits as created, though in a slightly peculiar order because we
// want to avoid using another register.

        shldq   $55, d7, d8
        shldq   $55, d6, d7
        movq    d7, 56(z)
        shldq   $55, d5, d6
        movq    d6, 48(z)
        shldq   $55, d4, d5
        movq    d5, 40(z)
        shldq   $55, d3, d4
        movq    d4, 32(z)
        shldq   $55, d2, d3
        movq    d3, 24(z)
        shldq   $55, d1, d2
        movq    d2, 16(z)
        shldq   $55, d0, d1
        movq    d1, 8(z)
        shldq   $55, d8, d0
        movq    d0, (z)
        andq    $0x1FF, d8
        movq    d8, 64(z)

// Restore register

        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
