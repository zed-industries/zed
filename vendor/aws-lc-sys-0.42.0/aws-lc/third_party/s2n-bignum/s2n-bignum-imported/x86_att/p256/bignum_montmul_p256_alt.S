// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_256
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_p256_alt(uint64_t z[static 4],
//                                        const uint64_t x[static 4],
//                                        const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_256, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_256 (in particular this is true if we are in
// the "usual" case x < p_256 and y < p_256).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p256_alt)
        .text

#define z %rdi
#define x %rsi

// We move the y argument here so we can use %rdx for multipliers

#define y %rcx

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

S2N_BN_SYMBOL(bignum_montmul_p256_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save more registers to play with

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Copy y into a safe register to start with

        movq    %rdx, y

// Do row 0 computation, which is a bit different:
// set up initial window [%r12,%r11,%r10,%r9,%r8] = y[0] * x
// Unlike later, we only need a single carry chain

        movq    (y), %rbx
        movq    (x), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9

        movq    8(x), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10

        movq    16(x), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11

        movq    24(x), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12

// Add row 1

        movq    8(y), %rbx
        xorl    %r13d, %r13d
        mulpadi(%r14,%r10,%r9,(x))
        mulpadd(%r14,%r11,%r10,8(x))
        mulpadd(%r14,%r12,%r11,16(x))
        mulpade(%r14,%r13,%r12,24(x))

// Montgomery reduce windows 0 and 1 together

        xorl    %r14d, %r14d
        movq    $0x0000000100000000, %rbx
        mulpadi(%r15,%r10,%r9,%r8)
        mulpadd(%r15,%r11,%r10,%r9)
        notq    %rbx
        leaq    2(%rbx), %rbx
        mulpadd(%r15,%r12,%r11,%r8)
        mulpade(%r15,%r13,%r12,%r9)
        adcq    %r14, %r14

// Add row 2

        movq    16(y), %rbx
        xorl    %r15d, %r15d
        mulpadi(%r8,%r11,%r10,(x))
        mulpadd(%r8,%r12,%r11,8(x))
        mulpadd(%r8,%r13,%r12,16(x))
        mulpade(%r8,%r14,%r13,24(x))
        adcq    %r15, %r15

// Add row 3

        movq    24(y), %rbx
        xorl    %r8d, %r8d
        mulpadi(%r9,%r12,%r11,(x))
        mulpadd(%r9,%r13,%r12,8(x))
        mulpadd(%r9,%r14,%r13,16(x))
        mulpade(%r9,%r15,%r14,24(x))
        adcq    %r8, %r8

// Montgomery reduce windows 2 and 3 together

        xorl    %r9d, %r9d
        movq    $0x0000000100000000, %rbx
        mulpadi(%rcx,%r12,%r11,%r10)
        mulpadd(%rcx,%r13,%r12,%r11)
        notq    %rbx
        leaq    2(%rbx), %rbx
        mulpadd(%rcx,%r14,%r13,%r10)
        mulpade(%rcx,%r15,%r14,%r11)
        adcq    %r9, %r8

// We now have a pre-reduced 5-word form [%r8; %r15;%r14;%r13;%r12]
// Load [%rax;%r11;%r9;%rbx;%rcx] = 2^320 - p_256, re-using earlier numbers a bit
// Do [%rax;%r11;%r9;%rbx;%rcx] = [%r8;%r15;%r14;%r13;%r12] + (2^320 - p_256)

        movl    $1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
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

// Restore registers and return

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
