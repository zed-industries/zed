// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_256
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_p256(uint64_t z[static 4],
//                                    const uint64_t x[static 4],
//                                    const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_256, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_256 (in particular this is true if we are in
// the "usual" case x < p_256 and y < p_256).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p256)
        .text

#define z %rdi
#define x %rsi

// We move the y argument here so we can use %rdx for multipliers

#define y %rcx

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

S2N_BN_SYMBOL(bignum_montmul_p256):
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

        xorl    %r13d, %r13d
        movq    (y), %rdx
        mulxq   (x), %r8, %r9
        mulxq   8(x), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   16(x), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   24(x), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12

// Add row 1

        movq    8(y), %rdx
        xorl    %r14d, %r14d
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10,8(x))
        mulpadd(%r12,%r11,16(x))
        mulpadd(%r13,%r12,24(x))
        adcq   %r14, %r13

// Montgomery reduce windows 0 and 1 together

        xorl    %r15d, %r15d
        movq    $0x0000000100000000, %rdx
        mulpadd(%r10,%r9,%r8)
        mulpadd(%r11,%r10,%r9)
        notq    %rdx
        leaq    2(%rdx), %rdx
        mulpadd(%r12,%r11,%r8)
        mulpadd(%r13,%r12,%r9)
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14

// Add row 2

        movq    16(y), %rdx
        xorl    %r8d, %r8d
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        adoxq   %r8, %r14
        mulxq   24(x), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15

// Add row 3

        movq    24(y), %rdx
        xorl    %r9d, %r9d
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))
        adoxq   %r9, %r15
        mulxq   24(x), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8

// Montgomery reduce windows 2 and 3 together

        xorl    %r9d, %r9d
        movq    $0x0000000100000000, %rdx
        mulpadd(%r12,%r11,%r10)
        mulpadd(%r13,%r12,%r11)
        notq    %rdx
        leaq    2(%rdx), %rdx
        mulpadd(%r14,%r13,%r10)
        mulpadd(%r15,%r14,%r11)
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8

// We now have a pre-reduced 5-word form [%r8; %r15;%r14;%r13;%r12]
// Load [%rax;%r11;%rbx;%rdx;%rcx] = 2^320 - p_256, re-using earlier numbers a bit
// Do [%rax;%r11;%rbx;%rdx;%rcx] = [%r8;%r15;%r14;%r13;%r12] + (2^320 - p_256)

        movl    $1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
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
        cmovcq  %rdx, %r13
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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
