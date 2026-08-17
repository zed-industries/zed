// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_sm2
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_sm2(uint64_t z[static 4], const uint64_t x[static 4],
//                                   const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_sm2, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_sm2 (in particular this is true if we are in
// the "usual" case x < p_sm2 and y < p_sm2).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_sm2)
        .text

#define z %rdi
#define x %rsi

// We move the y argument here so we can use %rdx for multipliers

#define y %rcx

// Use this fairly consistently for a zero

#define zero %rbp
#define zeroe %ebp

// mulpadd(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries.

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

// mulpade(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax as a temporary, assuming high created from scratch
// and that zero has value zero.

#define mulpade(high,low,m)             \
        mulxq   m, %rax, high ;           \
        adcxq   %rax, low ;               \
        adoxq   zero, high

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

S2N_BN_SYMBOL(bignum_montmul_sm2):
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
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Copy y into a safe register to start with

        movq    %rdx, y

// Zero a register, which also makes sure we don't get a fake carry-in

        xorl    zeroe, zeroe

// Do the zeroth row, which is a bit different

        movq    (y), %rdx

        mulxq   (x), %r8, %r9
        mulxq   8(x), %rax, %r10
        addq    %rax, %r9
        mulxq   16(x), %rax, %r11
        adcq    %rax, %r10
        mulxq   24(x), %rax, %r12
        adcq    %rax, %r11
        adcq    zero, %r12

// Add row 1

        xorl    zeroe, zeroe
        movq    8(y), %rdx
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10,8(x))
        mulpadd(%r12,%r11,16(x))
        mulpade(%r13,%r12,24(x))
        adcxq   zero, %r13

// Add row 2

        xorl    zeroe, zeroe
        movq    16(y), %rdx
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpade(%r14,%r13,24(x))
        adcxq   zero, %r14

// Add row 3

        xorl    zeroe, zeroe
        movq    24(y), %rdx
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))
        mulpade(%r15,%r14,24(x))
        adcxq   zero, %r15

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
