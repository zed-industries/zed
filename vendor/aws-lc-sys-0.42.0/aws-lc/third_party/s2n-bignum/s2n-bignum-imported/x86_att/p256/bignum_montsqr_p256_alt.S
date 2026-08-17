// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256_alt(uint64_t z[static 4],
//                                        const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256, assuming x^2 <= 2^256 * p_256, which is
// guaranteed in particular if x < p_256 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p256_alt)
        .text

#define z %rdi
#define x %rsi

// Add %rbx * m into a register-pair (high,low) maintaining consistent
// carry-catching with carry (negated, as bitmask) and using %rax and %rdx
// as temporaries

#define mulpadd(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rbx;                    \
        subq    carry, %rdx ;             \
        addq    %rax, low ;               \
        adcq    %rdx, high ;              \
        sbbq    carry, carry

// Initial version assuming no carry-in

#define mulpadi(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rbx;                    \
        addq    %rax, low ;               \
        adcq    %rdx, high ;              \
        sbbq    carry, carry

// End version not catching the top carry-out

#define mulpade(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rbx;                    \
        subq    carry, %rdx ;             \
        addq    %rax, low ;               \
        adcq    %rdx, high

S2N_BN_SYMBOL(bignum_montsqr_p256_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Compute [%r15;%r8] = [00] which we use later, but mainly
// set up an initial window [%r14;...;%r9] = [23;03;01]

        movq    (x), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    8(x), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    24(x), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    16(x), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14

// Chain in the addition of 02 + 12 + 13 to that window (no carry-out possible)
// This gives all the "heterogeneous" terms of the squaring ready to double

        mulpadi(%rcx,%r11,%r10,(x))
        mulpadd(%rcx,%r12,%r11,8(x))
        movq    24(x), %rbx
        mulpade(%rcx,%r13,%r12,8(x))
        adcq    $0, %r14

// Double the window [%r14;...;%r9], catching top carry in %rcx

        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx

// Add to the 00 + 11 + 22 + 33 terms

        movq    8(x), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    16(x), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    24(x), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15

// First two waves of Montgomery reduction, now re-using %r8 for top carry

        movq    $0x0000000100000000, %rbx
        mulpadi(%rcx,%r10,%r9,%r8)
        mulpadd(%rcx,%r11,%r10,%r9)
        notq    %rbx
        leaq    2(%rbx), %rbx
        mulpadd(%rcx,%r12,%r11,%r8)
        xorl    %r8d, %r8d
        mulpade(%rcx,%r13,%r12,%r9)
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8

// Now two more steps of Montgomery reduction, again with %r8 = top carry

        movq    $0x0000000100000000, %rbx
        mulpadi(%rcx,%r12,%r11,%r10)
        mulpadd(%rcx,%r13,%r12,%r11)
        notq    %rbx
        leaq    2(%rbx), %rbx
        mulpadd(%rcx,%r14,%r13,%r10)
        xorl    %r9d, %r9d
        mulpade(%rcx,%r15,%r14,%r11)
        adcq    %r9, %r8

// Load [%rax;%r11;%r9;%rbx;%rcx] = 2^320 - p_256, re-using earlier numbers a bit
// Do [%rax;%r11;%r9;%rbx;%rcx] = [%r8;%r15;%r14;%r13;%r12] + (2^320 - p_256)

        movl    $1, %ecx
        addq    %r12, %rcx
        leaq    -1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0x00000000fffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax

// Now carry is set if r + (2^320 - p_256) >= 2^320, i.e. r >= p_256
// where r is the pre-reduced form. So conditionally select the
// output accordingly.

        cmovcq  %rcx, %r12
        cmovcq  %rbx, %r13
        cmovcq  %r9, %r14
        cmovcq  %r11, %r15

// Write back reduced value

        movq    %r12, (z)
        movq    %r13, 8(z)
        movq    %r14, 16(z)
        movq    %r15, 24(z)

// Restore saved registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
