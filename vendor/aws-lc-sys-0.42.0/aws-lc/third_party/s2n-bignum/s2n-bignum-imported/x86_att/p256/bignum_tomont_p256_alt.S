// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_p256_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p256_alt)
        .text

#define z %rdi
#define x %rsi

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

// End version not catching the top carry-out

#define mulpade(carry,high,low,m)       \
        movq    m, %rax ;                 \
        mulq    %rcx;                    \
        subq    carry, %rdx ;             \
        addq    %rax, low ;               \
        adcq    %rdx, high

S2N_BN_SYMBOL(bignum_tomont_p256_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Do row 0 computation, which is a bit different:
// set up initial window [%r12,%r11,%r10,%r9,%r8] = y[0] * x
// Unlike later, we only need a single carry chain

        movl    $0x0000000000000003, %ecx
        movq    (x), %rax
        mulq    %rcx
        movq    %rax, %r8
        movq    %rdx, %r9

        movq    8(x), %rax
        mulq    %rcx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10

        movq    16(x), %rax
        mulq    %rcx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11

        movq    24(x), %rax
        mulq    %rcx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12

// Add row 1

        movq    $0xfffffffbffffffff, %rcx
        xorl    %r13d, %r13d
        mulpadi(%r14,%r10,%r9,(x))
        mulpadd(%r14,%r11,%r10,8(x))
        mulpadd(%r14,%r12,%r11,16(x))
        mulpade(%r14,%r13,%r12,24(x))

// Montgomery reduce windows 0 and 1 together

        xorl    %r14d, %r14d
        movq    $0x0000000100000000, %rcx
        mulpadi(%r15,%r10,%r9,%r8)
        mulpadd(%r15,%r11,%r10,%r9)
        notq    %rcx
        leaq    2(%rcx), %rcx
        mulpadd(%r15,%r12,%r11,%r8)
        mulpade(%r15,%r13,%r12,%r9)
        adcq    %r14, %r14

// Add row 2

        movq    $0xfffffffffffffffe, %rcx
        xorl    %r15d, %r15d
        mulpadi(%r8,%r11,%r10,(x))
        mulpadd(%r8,%r12,%r11,8(x))
        mulpadd(%r8,%r13,%r12,16(x))
        mulpade(%r8,%r14,%r13,24(x))
        adcq    %r15, %r15

// Add row 3

        movq    $0x00000004fffffffd, %rcx
        xorl    %r8d, %r8d
        mulpadi(%r9,%r12,%r11,(x))
        mulpadd(%r9,%r13,%r12,8(x))
        mulpadd(%r9,%r14,%r13,16(x))
        mulpade(%r9,%r15,%r14,24(x))
        adcq    %r8, %r8

// Montgomery reduce windows 2 and 3 together

        movq    $0x0000000100000000, %rcx
        mulpadi(%r9,%r12,%r11,%r10)
        mulpadd(%r9,%r13,%r12,%r11)
        notq    %rcx
        leaq    2(%rcx), %rcx
        mulpadd(%r9,%r14,%r13,%r10)
        mulpadd(%r9,%r15,%r14,%r11)
        subq    %r9, %r8

// We now have a pre-reduced 5-word form [%r8; %r15;%r14;%r13;%r12]
// Load [%rax;%r11;%r9;%rcx;%rdx] = 2^320 - p_256, re-using earlier numbers a bit
// Do [%rax;%r11;%r9;%rcx;%rdx] = [%r8;%r15;%r14;%r13;%r12] + (2^320 - p_256)

        xorl    %edx, %edx
        leaq    -1(%rdx), %r9
        incq    %rdx
        addq    %r12, %rdx
        decq    %rcx
        adcq    %r13, %rcx
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0x00000000fffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax

// Now carry is set if r + (2^320 - p_256) >= 2^320, i.e. r >= p_256
// where r is the pre-reduced form. So conditionally select the
// output accordingly.

        cmovcq  %rdx, %r12
        cmovcq  %rcx, %r13
        cmovcq  %r9, %r14
        cmovcq  %r11, %r15

// Write back reduced value

        movq    %r12, (z)
        movq    %r13, 8(z)
        movq    %r14, 16(z)
        movq    %r15, 24(z)

// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
