// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from almost-Montgomery form, z := (x / 2^256) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_deamont_p256(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Convert a 4-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 4-digit input will work, with no range restriction.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p256)
        .text

#define z %rdi
#define x %rsi

// Re-use these as temporaries in the correction phase

#define d %rdx
#define u %r10
#define v %r11

#define dshort %edx
#define ushort %r10d

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rcx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

S2N_BN_SYMBOL(bignum_deamont_p256):
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

// We now have a pre-reduced dd = [%r9;%r8;%rsi;%rbx]. Load non-trivial digits
// of p_256 = [v; 0; u; -1]

        movl    $0x00000000ffffffff, ushort
        movq    $0xffffffff00000001, v

// Now do the subtraction (p_256-1) - (%r9;%r8;%rsi;%rbx) to get the carry

        movq    $-2, d
        subq    %rbx, d
        movq    u, d
        sbbq    %rsi, d
        movl    $0, dshort
        sbbq    %r8, d
        movq    v, d
        sbbq    %r9, d

// Convert the carry CF <=> dd >= p_256 to a bitmask and do a masked subtraction

        sbbq    d, d
        andq    d, u
        andq    d, v

        subq    d, %rbx
        sbbq    u, %rsi
        sbbq    $0, %r8
        sbbq    v, %r9

// Write back

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

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
