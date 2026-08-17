// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256k1(uint64_t z[static 4],
//                                      const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256k1, assuming x^2 <= 2^256 * p_256k1, which
// is guaranteed in particular if x < p_256k1 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p256k1)
        .text

#define z %rdi
#define x %rsi

// Use this fairly consistently for a zero

#define zero %rbp
#define zeroe %ebp

// Also use the same register for multiplicative inverse in Montgomery stage

#define w %rbp

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

S2N_BN_SYMBOL(bignum_montsqr_p256k1):
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

// Now we have the full 8-digit square 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// Do Montgomery reductions, now using %rcx as a carry save

        movq    $0xd838091dd2253531, w
        movq    $4294968273, %rbx

// Montgomery reduce row 0

        movq    %rbx, %rax
        imulq   w, %r8
        mulq    %r8
        subq    %rdx, %r9
        sbbq    %rcx, %rcx

// Montgomery reduce row 1

        movq    %rbx, %rax
        imulq   w, %r9
        mulq    %r9
        negq    %rcx
        sbbq    %rdx, %r10
        sbbq    %rcx, %rcx

// Montgomery reduce row 2

        movq    %rbx, %rax
        imulq   w, %r10
        mulq    %r10
        negq    %rcx
        sbbq    %rdx, %r11
        sbbq    %rcx, %rcx

// Montgomery reduce row 3

        movq    %rbx, %rax
        imulq   w, %r11
        mulq    %r11
        negq    %rcx

// Now [%r15,%r14,%r13,%r12] := [%r15,%r14,%r13,%r12] + [%r11,%r10,%r9,%r8] - (%rdx + CF)

        sbbq    %rdx, %r8
        sbbq    $0, %r9
        sbbq    $0, %r10
        sbbq    $0, %r11

        addq    %r8, %r12
        adcq    %r9, %r13
        adcq    %r10, %r14
        adcq    %r11, %r15
        sbbq    w, w

// Let b be the top carry captured just above as w = (2^64-1) * b
// Now if [b,%r15,%r14,%r13,%r12] >= p_256k1, subtract p_256k1, i.e. add 4294968273
// and either way throw away the top word. [b,%r15,%r14,%r13,%r12] - p_256k1 =
// [(b - 1),%r15,%r14,%r13,%r12] + 4294968273. If [%r15,%r14,%r13,%r12] + 4294968273
// gives carry flag CF then >= comparison is top = 0 <=> b - 1 + CF = 0 which
// is equivalent to b \/ CF, and so to (2^64-1) * b + (2^64 - 1) + CF >= 2^64

        movq    %r12, %r8
        addq    %rbx, %r8
        movq    %r13, %r9
        adcq    $0, %r9
        movq    %r14, %r10
        adcq    $0, %r10
        movq    %r15, %r11
        adcq    $0, %r11

        adcq    $-1, w

// Write everything back

        cmovcq  %r8, %r12
        movq    %r12, (z)
        cmovcq  %r9, %r13
        movq    %r13, 8(z)
        cmovcq  %r10, %r14
        movq    %r14, 16(z)
        cmovcq  %r11, %r15
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

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
