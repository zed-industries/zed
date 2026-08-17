// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_sm2(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_sm2, assuming x^2 <= 2^256 * p_sm2, which is
// guaranteed in particular if x < p_sm2 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_sm2)
        .text

#define z %rdi
#define x %rsi

// Use this fairly consistently for a zero

#define zero %rbp
#define zeroe %ebp

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

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

S2N_BN_SYMBOL(bignum_montsqr_sm2):
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
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Compute [%r15;%r8] = [00] which we use later, but mainly
// set up an initial window [%r14;...;%r9] = [23;03;01]

        movq    (x), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   8(x), %r9, %r10
        mulxq   24(x), %r11, %r12
        movq    16(x), %rdx
        mulxq   24(x), %r13, %r14

// Clear our zero register, and also initialize the flags for the carry chain

        xorl    zeroe, zeroe

// Chain in the addition of 02 + 12 + 13 to that window (no carry-out possible)
// This gives all the "heterogeneous" terms of the squaring ready to double

        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        movq    24(x), %rdx
        mulpadd(%r13,%r12,8(x))
        adcxq   zero, %r13
        adoxq   zero, %r14
        adcq    zero, %r14

// Double and add to the 00 + 11 + 22 + 33 terms

        xorl    zeroe, zeroe
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    8(x), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    16(x), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    24(x), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   zero, %r15
        adoxq   zero, %r15

// Squaring complete. Perform 4 Montgomery steps to rotate the lower half

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

// Load [%r8;%r11;%rbp;%rdx;%rcx] = 2^320 - p_sm2 then do
// [%r8;%r11;%rbp;%rdx;%rcx] = [%rax;%r15;%r14;%r13;%r12] + (2^320 - p_sm2)

        movl    $1, %ecx
        movl    $0x00000000FFFFFFFF, %edx
        xorl    %ebp, %ebp
        addq    %r12, %rcx
        leaq    1(%rdx), %r11
        adcq    %r13, %rdx
        leaq    -1(%rbp), %r8
        adcq    %r14, %rbp
        adcq    %r15, %r11
        adcq    %rax, %r8

// Now carry is set if r + (2^320 - p_sm2) >= 2^320, i.e. r >= p_sm2
// where r is the pre-reduced form. So conditionally select the
// output accordingly.

        cmovcq  %rcx, %r12
        cmovcq  %rdx, %r13
        cmovcq  %rbp, %r14
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
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
