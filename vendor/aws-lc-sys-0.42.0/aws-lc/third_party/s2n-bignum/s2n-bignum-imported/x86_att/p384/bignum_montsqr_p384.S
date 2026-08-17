// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^384) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_montsqr_p384(uint64_t z[static 6],
//                                    const uint64_t x[static 6]);
//
// Does z := (x^2 / 2^384) mod p_384, assuming x^2 <= 2^384 * p_384, which is
// guaranteed in particular if x < p_384 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p384)
        .text

#define z %rdi
#define x %rsi

// Some temp registers for the last correction stage

#define d %rax
#define u %rdx
#define v %r10
#define w %r11

// A zero register, very often

#define zero %rbp
#define zeroe %ebp

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

// Core one-step "short" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing [d5;d4;d3;d2;d1] and re-using d0 as a
// temporary internally, as well as %rax, %rbx and %rdx.
// It is OK for d6 and d0 to be the same register (they often are)
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
//
//       montreds(d6,d5,d4,d3,d2,d1,d0)

#define montreds(d6,d5,d4,d3,d2,d1,d0)                                  \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64 */         \
        movq    d0, %rdx ;                                        \
        shlq    $32, %rdx ;                                        \
        addq    d0, %rdx ;                                        \
/* Construct [%rbx;d0;%rax;-] = (2^384 - p_384) * w            */         \
/* We know the lowest word will cancel so we can re-use d0   */         \
/* and %rbx as temps.                                         */         \
        movq    $0xffffffff00000001, %rax ;                        \
        mulxq   %rax, d0, %rax ;                                   \
        movl    $0x00000000ffffffff, %ebx ;                        \
        mulxq   %rbx, %rbx, d0 ;                                   \
        addq    %rbx, %rax ;                                       \
        adcq    %rdx, d0 ;                                        \
        movl    $0, %ebx ;                                         \
        adcq    %rbx, %rbx ;                                       \
/* Now subtract that and add 2^384 * w                       */         \
        subq    %rax, d1 ;                                        \
        sbbq    d0, d2 ;                                         \
        sbbq    %rbx, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        movq    %rdx, d6 ;                                        \
        sbbq    $0, d6

S2N_BN_SYMBOL(bignum_montsqr_p384):
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

// Set up an initial window [%rcx;%r15;...%r9] = [34;05;03;01]
// Note that we are using %rcx as the first step past the rotating window

        movq    (x), %rdx
        mulxq   8(x), %r9, %r10
        mulxq   24(x), %r11, %r12
        mulxq   40(x), %r13, %r14
        movq    24(x), %rdx
        mulxq   32(x), %r15, %rcx

// Clear our zero register, and also initialize the flags for the carry chain

        xorl    zeroe, zeroe

// Chain in the addition of 02 + 12 + 13 + 14 + 15 to that window
// (no carry-out possible)

        movq    16(x), %rdx
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        movq    8(x), %rdx
        mulpadd(%r13,%r12,24(x))
        mulpadd(%r14,%r13,32(x))
        mulpadd(%r15,%r14,40(x))
        adcxq   zero, %r15
        adoxq   zero, %rcx
        adcq    zero, %rcx

// Again zero out the flags. Actually they are already cleared but it may
// help decouple these in the OOO engine not to wait for the chain above

        xorl    zeroe, zeroe

// Now chain in the 04 + 23 + 24 + 25 + 35 + 45 terms
// We are running out of registers in our rotating window, so we start
// using %rbx (and hence need care with using mulpadd after this). Thus
// our result so far is in [%rbp;%rbx;%rcx;%r15;...%r9]

        movq    32(x), %rdx
        mulpadd(%r13,%r12,(x))
        movq    16(x), %rdx
        mulpadd(%r14,%r13,24(x))
        mulpadd(%r15,%r14,32(x))
        mulxq   40(x), %rax, %rdx
        adcxq   %rax, %r15
        adoxq   %rdx, %rcx

// First set up the last couple of spots in our window, [%rbp;%rbx] = 45
// then add the last other term 35

        movq    40(x), %rdx
        mulxq   32(x), %rbx, %rbp
        mulxq   24(x), %rax, %rdx
        adcxq   %rax, %rcx
        adoxq   %rdx, %rbx
        movl    $0, %eax
        adcxq   %rax, %rbx
        adoxq   %rax, %rbp
        adcq    %rax, %rbp

// Just for a clear fresh start for the flags; we don't use the zero

        xorq    %rax, %rax

// Double and add to the 00 + 11 + 22 + 33 + 44 + 55 terms
// For one glorious moment the entire squaring result is all in the
// register file as [%rsi;%rbp;%rbx;%rcx;%r15;...;%r8]
// (since we've now finished with x we can re-use %rsi)

        movq    (x), %rdx
        mulxq   (x), %r8, %rax
        adcxq   %r9, %r9
        adoxq   %rax, %r9
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
        mulxq   %rdx, %rax, %rdx
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %r15, %r15
        adoxq   %rdx, %r15
        movq    32(x), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %rcx, %rcx
        adoxq   %rax, %rcx
        adcxq   %rbx, %rbx
        adoxq   %rdx, %rbx
        movq    40(x), %rdx
        mulxq   %rdx, %rax, %rsi
        adcxq   %rbp, %rbp
        adoxq   %rax, %rbp
        movl    $0, %eax
        adcxq   %rax, %rsi
        adoxq   %rax, %rsi

// We need just *one* more register as a temp for the Montgomery steps.
// Since we are writing to the z buffer anyway, make use of that to stash %rbx.

        movq    %rbx, (z)

// Montgomery reduce the %r13,...,%r8 window 6 times

        montreds(%r8,%r13,%r12,%r11,%r10,%r9,%r8)
        montreds(%r9,%r8,%r13,%r12,%r11,%r10,%r9)
        montreds(%r10,%r9,%r8,%r13,%r12,%r11,%r10)
        montreds(%r11,%r10,%r9,%r8,%r13,%r12,%r11)
        montreds(%r12,%r11,%r10,%r9,%r8,%r13,%r12)
        montreds(%r13,%r12,%r11,%r10,%r9,%r8,%r13)

// Now we can safely restore %rbx before accumulating

        movq    (z), %rbx

        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rsi
        movl    $0, %r8d
        adcq    %r8, %r8

// We now have a pre-reduced 7-word form z = [%r8; %rsi;%rbp;%rbx;%rcx;%r15;%r14]
// Next, accumulate in different registers z - p_384, or more precisely
//
//   [%r8; %r13;%r12;%r11;%r10;%r9;%rax] = z + (2^384 - p_384)

        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movq    $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0x00000000ffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x0000000000000001, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rsi, %r13
        adcq    $0, %r8

// ~ZF <=> %r12 >= 1 <=> z + (2^384 - p_384) >= 2^384 <=> z >= p_384, which
// determines whether to use the further reduced argument or the original z.

        cmovnzq %rax, %r14
        cmovnzq %r9, %r15
        cmovnzq %r10, %rcx
        cmovnzq %r11, %rbx
        cmovnzq %r12, %rbp
        cmovnzq %r13, %rsi

// Write back the result

        movq    %r14, (z)
        movq    %r15, 8(z)
        movq    %rcx, 16(z)
        movq    %rbx, 24(z)
        movq    %rbp, 32(z)
        movq    %rsi, 40(z)

// Restore registers and return

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

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
