// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^256) mod p_256, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_demont_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// This assumes the input is < p_256 for correctness. If this is not the case,
// use the variant "bignum_deamont_p256" instead.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_p256)
        .text

#define z %rdi
#define x %rsi

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rcx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

S2N_BN_SYMBOL(bignum_demont_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save one more register to play with

        CFI_PUSH(%rbx)

// Set up an initial 4-word window [%r11,%r10,%r9,%r8] = x

        movq    (x), %r8
        movq    8(x), %r9
        movq    16(x), %r10
        movq    24(x), %r11

// Fill in two zeros to the left

        xorq    %rbx, %rbx
        xorq    %rsi, %rsi

// Montgomery reduce windows 0 and 1 together

        movq    $0x0000000100000000, %rdx
        mulpadd(%r10,%r9,%r8)
        mulpadd(%r11,%r10,%r9)
        movq    $0xffffffff00000001, %rdx
        mulpadd(%rbx,%r11,%r8)
        mulpadd(%rsi,%rbx,%r9)
        movl    $0, %r8d
        adcxq   %r8, %rsi

// Append just one more leading zero (by the above %r8 = 0 already).

        xorq    %r9, %r9

// Montgomery reduce windows 2 and 3 together

        movq    $0x0000000100000000, %rdx
        mulpadd(%rbx,%r11,%r10)
        mulpadd(%rsi,%rbx,%r11)
        movq    $0xffffffff00000001, %rdx
        mulpadd(%r8,%rsi,%r10)
        mulpadd(%r9,%r8,%r11)
        movl    $0, %r10d
        adcxq   %r10, %r9

// Since the input was assumed reduced modulo, i.e. < p, we actually know that
// 2^256 * [carries; %r9;%r8;%rsi;%rbx] is <= (p - 1) + (2^256 - 1) p
// and hence [carries; %r9;%r8;%rsi;%rbx] < p. This means in fact carries = 0
// and [%r9;%r8;%rsi;%rbx] is already our answer, without further correction.
// Write that back.

        movq    %rbx, (z)
        movq    %rsi, 8(z)
        movq    %r8, 16(z)
        movq    %r9, 24(z)

// Restore saved register and return

        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
