// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_25519, z := (x^2) mod p_25519
// Input x[4]; output z[4]
//
//    extern void bignum_sqr_p25519(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p25519)
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

S2N_BN_SYMBOL(bignum_sqr_p25519):
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

// Double and add to the 00 + 11 + 22 + 33 terms, while also
// pre-estimating the quotient from early results.

        xorl    zeroe, zeroe
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    8(x), %rdx
        mulxq   %rdx, %rax, %rcx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rcx, %r11
        movq    16(x), %rdx
        mulxq   %rdx, %rax, %rcx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rcx, %r13
        movq    24(x), %rdx
        mulxq   %rdx, %rax, %r15

        movl    $38, %edx
        mulxq   %r15, %rdx, %rcx

        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   zero, %r15
        adoxq   zero, %r15

        addq    %r11, %rdx
        adcq    zero, %rcx
        shldq   $1, %rdx, %rcx
        leaq    1(%rcx), %rbx
        imulq   $19, %rbx

// Now we have the full 8-digit product 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// and this is == 38 * h + l (mod p_25519)
// We add in the precalculated 19 * q as well.
// This is kept in 4 words since we have enough information there.

        xorl    %eax, %eax
        adoxq   %rbx, %r8
        movl    $38, %edx
        mulpadd(%r9,%r8,%r12)
        mulpadd(%r10,%r9,%r13)
        mulpadd(%r11,%r10,%r14)
        mulxq   %r15, %rax, %rcx
        adcq    %rax, %r11

// We still haven't made the -2^255 * q contribution yet. Since we
// are now safely in 4 words we just need a single bit of q, and we
// can actually use the LSB of %rcx = 19 * q since 19 is odd. And we
// don't literally need to subtract, just to see whether we would
// have a top 1 bit if we did, meaning we need to correct in the
// last step by adding 2^255 - 19.

        xorl    %ecx, %ecx
        shlq    $63, %rbx
        cmpq    %rbx, %r11
        movl    $19, %eax
        cmovns  %rcx, %rax

// Now make that possible correction and finally mask to 255 bits

        subq    %rax, %r8
        sbbq    %rcx, %r9
        sbbq    %rcx, %r10
        sbbq    %rcx, %r11
        btr     $63, %r11

// Write everything back

        movq    %r8, (z)
        movq    %r9, 8(z)
        movq    %r10, 16(z)
        movq    %r11, 24(z)

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

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
