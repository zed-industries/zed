// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_256k1, z := (x^2) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_sqr_p256k1(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p256k1)
        .text

#define z %rdi
#define x %rsi

// Use this fairly consistently for a zero

#define zero %rbx
#define zeroe %ebx

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rcx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

// mulpade(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax as a temporary, assuming high created from scratch
// and that zero has value zero.

#define mulpade(high,low,m)             \
        mulxq   m, %rax, high ;           \
        adcxq   %rax, low ;               \
        adoxq   zero, high

S2N_BN_SYMBOL(bignum_sqr_p256k1):
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

// Now we have the full 8-digit product 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// and this is == 4294968273 * h + l (mod p_256k1)

        movq    $4294968273, %rdx

        xorl    zeroe, zeroe

        mulpadd(%r9,%r8,%r12)
        mulpadd(%r10,%r9,%r13)
        mulpadd(%r11,%r10,%r14)
        mulpade(%r12,%r11,%r15)
        adcxq   zero, %r12

// Now we have reduced to 5 digits, 2^256 * h + l = [%r12,%r11,%r10,%r9,%r8]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.

        leaq    1(%r12), %rax
        mulxq   %rax, %rax, %rcx
        addq    %rax, %r8
        adcq    %rcx, %r9
        adcq    zero, %r10
        adcq    zero, %r11

// Now the effective answer is 2^256 * (CF - 1) + [%r11,%r10,%r9,%r8]
// So we correct if CF = 0 by subtracting 4294968273, i.e. by
// adding p_256k1 to the "full" answer

        sbbq    %rax, %rax
        notq    %rax
        andq    %rdx, %rax
        subq    %rax, %r8
        sbbq    zero, %r9
        sbbq    zero, %r10
        sbbq    zero, %r11

// Write everything back

        movq    %r8, (z)
        movq    %r9, 8(z)
        movq    %r10, 16(z)
        movq    %r11, 24(z)

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

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
