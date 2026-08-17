// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from almost-Montgomery form, z := (x / 2^256) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_deamont_p256_alt(uint64_t z[static 4],
//                                        const uint64_t x[static 4]);
//
// Convert a 4-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 4-digit input will work, with no range restriction.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p256_alt)
        .text

#define z %rdi
#define x %rsi

// Re-use these as temporaries in the correction phase

#define d %rdx
#define u %rax
#define v %rcx

#define dshort %edx
#define ushort %eax

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rcx as temporaries

#define mulpado(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

// Add %rcx * m into a register-pair (high,low) maintaining consistent
// carry-catching with carry (negated, as bitmask) and using %rax and %rdx
// as temporaries

#define mulpadd(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rcx;                    \
        subq    carry, %rdx ;             \
        addq    %rax, low ;               \
        adcq    %rdx, high ;              \
        sbbq    carry, carry

// Initial version assuming no carry-in

#define mulpadi(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rcx;                    \
        addq    %rax, low ;               \
        adcq    %rdx, high ;              \
        sbbq    carry, carry

// Version with no carry in or out

#define mulpadn(high,low,m)             \
        movq    m, %rax ;                 \
        mulq    %rcx;                    \
        addq    %rax, low ;               \
        adcq    %rdx, high

S2N_BN_SYMBOL(bignum_deamont_p256_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Set up an initial 4-word window [%r11,%r10,%r9,%r8] = x

        movq    (x), %r8
        movq    8(x), %r9
        movq    16(x), %r10
        movq    24(x), %r11

// Load constant 2^32; %rcx toggles between this and (1 - %rcx) below

        movq    $0x0000000100000000, %rcx

// Montgomery reduce windows 0 and 1 together as [%r8;%rsi;%r11;%r10]

        mulpadi(%rsi,%r10,%r9,%r8)
        mulpadd(%rsi,%r11,%r10,%r9)
        negq    %rcx
        negq    %rsi
        incq    %rcx
        mulpadi(%r8,%rsi,%r11,%r8)
        negq    %r8
        mulpadn(%r8,%rsi,%r9)

// Montgomery reduce windows 2 and 3 together as [%r10;%r9;%r8;%rsi]

        negq    %rcx
        incq    %rcx
        mulpadi(%r9,%rsi,%r11,%r10)
        mulpadd(%r9,%r8,%rsi,%r11)
        negq    %rcx
        negq    %r9
        incq    %rcx
        mulpadi(%r10,%r9,%r8,%r10)
        negq    %r10
        mulpadn(%r10,%r9,%r11)

// We now have a pre-reduced result z = [%r10;%r9;%r8;%rsi].
// From the above we have %rcx = 0xffffffff00000001, which we use to generate
// [0x00000000fffffffe; -1; 0xffffffff00000000; 1] = 2^256 - p_256 and
// then compute [%rcx;%rdx;%r11;%rax] = z + (2^256 - p_256)

        xorl    %edx, %edx
        leaq    1(%rdx), %rax
        addq    %rsi, %rax
        leaq    -1(%rcx), %r11
        adcq    %r8, %r11
        notq    %rdx
        adcq    %r9, %rdx
        notq    %rcx
        adcq    %r10, %rcx

// CF is set iff z + (2^256 - p_256) >= 2^256, i.e. if z >= p_256.
// If so we want the result of the subtraction (in 4 words)

        cmovcq  %rax, %rsi
        cmovcq  %r11, %r8
        cmovcq  %rdx, %r9
        cmovcq  %rcx, %r10

// Write back

        movq    %rsi, (z)
        movq    %r8, 8(z)
        movq    %r9, 16(z)
        movq    %r10, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
