// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_sm2
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_sm2_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4],
//                                       const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_sm2, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_sm2 (in particular this is true if we are in
// the "usual" case x < p_sm2 and y < p_sm2).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_sm2_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_sm2_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_sm2_alt)
        .text

#define z %rdi
#define x %rsi

// We move the y argument here so we can use %rdx for multipliers

#define y %rcx

// Macro for the key "multiply and add to (c,h,l)" step

#define combadd(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A minutely shorter form for when c = 0 initially

#define combadz(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    c, c

// A short form where we don't expect a top carry

#define combads(h,l,numa,numb)                  \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d3;d2;d1;d0] and returns result in [d0;d3;d2;d1], adding to the
// existing contents of [d3;d2;d1], and using %rax, %rcx, %rdx and %rbx
// as temporaries.
// ---------------------------------------------------------------------------

#define montreds(d3,d2,d1,d0)                                               \
        movq    d0, %rax ;                                                    \
        shlq    $32, %rax ;                                                    \
        movq    d0, %rcx ;                                                    \
        shrq    $32, %rcx ;                                                    \
        movq    %rax, %rdx ;                                                   \
        movq    %rcx, %rbx ;                                                   \
        subq    d0, %rax ;                                                    \
        sbbq    $0, %rcx ;                                                     \
        subq    %rax, d1 ;                                                    \
        sbbq    %rcx, d2 ;                                                    \
        sbbq    %rdx, d3 ;                                                    \
        sbbq    %rbx, d0

S2N_BN_SYMBOL(bignum_montmul_sm2_alt):
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

// Start the window as [%r10;%r9;%r8] with 00 product

        movq    (x), %rax
        mulq     (y)
        movq    %rax, %r8
        movq    %rdx, %r9
        xorq    %r10, %r10

// Column 1

        xorq    %r11, %r11
        combads(%r10,%r9,(x),8(y))
        combadz(%r11,%r10,%r9,8(x),(y))

// Column 2

        xorq    %r12, %r12
        combadz(%r12,%r11,%r10,(x),16(y))
        combadd(%r12,%r11,%r10,8(x),8(y))
        combadd(%r12,%r11,%r10,16(x),(y))

// Column 3

        xorq    %r13, %r13
        combadz(%r13,%r12,%r11,(x),24(y))
        combadd(%r13,%r12,%r11,8(x),16(y))
        combadd(%r13,%r12,%r11,16(x),8(y))
        combadd(%r13,%r12,%r11,24(x),(y))

// Column 4

        xorq    %r14, %r14
        combadz(%r14,%r13,%r12,8(x),24(y))
        combadd(%r14,%r13,%r12,16(x),16(y))
        combadd(%r14,%r13,%r12,24(x),8(y))

// Column 5

        xorq    %r15, %r15
        combadz(%r15,%r14,%r13,16(x),24(y))
        combadd(%r15,%r14,%r13,24(x),16(y))

// Final work for columns 6 and 7

        movq    24(x), %rax
        mulq     24(y)
        addq    %rax, %r14
        adcq    %rdx, %r15

// Multiplication complete. Perform 4 Montgomery steps to rotate the lower half

        montreds(%r11,%r10,%r9,%r8)
        montreds(%r8,%r11,%r10,%r9)
        montreds(%r9,%r8,%r11,%r10)
        montreds(%r10,%r9,%r8,%r11)

// Add high and low parts, catching carry in %rax

        xorl    %eax, %eax
        addq    %r8, %r12
        adcq    %r9, %r13
        adcq    %r10, %r14
        adcq    %r11, %r15
        adcq    %rax, %rax

// Load [%r8;%r11;%rbx;%rdx;%rcx] = 2^320 - p_sm2 then do
// [%r8;%r11;%rbx;%rdx;%rcx] = [%rax;%r15;%r14;%r13;%r12] + (2^320 - p_sm2)

        movl    $1, %ecx
        movl    $0x00000000FFFFFFFF, %edx
        xorl    %ebx, %ebx
        addq    %r12, %rcx
        leaq    1(%rdx), %r11
        adcq    %r13, %rdx
        leaq    -1(%rbx), %r8
        adcq    %r14, %rbx
        adcq    %r15, %r11
        adcq    %rax, %r8

// Now carry is set if r + (2^320 - p_sm2) >= 2^320, i.e. r >= p_sm2
// where r is the pre-reduced form. So conditionally select the
// output accordingly.

        cmovcq  %rcx, %r12
        cmovcq  %rdx, %r13
        cmovcq  %rbx, %r14
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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_sm2_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
