// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^384) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_montsqr_p384_alt(uint64_t z[static 6],
//                                        const uint64_t x[static 6]);
//
// Does z := (x^2 / 2^384) mod p_384, assuming x^2 <= 2^384 * p_384, which is
// guaranteed in particular if x < p_384 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p384_alt)
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
        movq    d0, %rbx ;                                        \
        shlq    $32, %rbx ;                                        \
        addq    d0, %rbx ;                                        \
/* Construct [%rax;%rdx;d0;-] = (2^384 - p_384) * w            */         \
/* We know the lowest word will cancel so we can re-use d0   */         \
/* and %rbx as temps.                                         */         \
        movq    $0xffffffff00000001, %rax ;                        \
        mulq    %rbx;                                            \
        movq    %rdx, d0 ;                                        \
        movq    $0x00000000ffffffff, %rax ;                        \
        mulq    %rbx;                                            \
        addq    %rax, d0 ;                                        \
        movl    $0, %eax ;                                         \
        adcq    %rbx, %rdx ;                                       \
        adcl    %eax, %eax ;                                       \
/* Now subtract that and add 2^384 * w                       */         \
        subq    d0, d1 ;                                         \
        sbbq    %rdx, d2 ;                                        \
        sbbq    %rax, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        movq    %rbx, d6 ;                                        \
        sbbq    $0, d6

S2N_BN_SYMBOL(bignum_montsqr_p384_alt):
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

        movq    (x), %rbx
        movq    8(x), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10

        movq    24(x), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12

        movq    40(x), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14

        movq    24(x), %rax
        mulq     32(x)
        movq    %rax, %r15
        movq    %rdx, %rcx

// Chain in the addition of 02 + 12 + 13 + 14 + 15 to that window
// (no carry-out possible)

        movq    16(x), %rbx
        mulpadi(%rbp,%r11,%r10,(x))
        mulpadd(%rbp,%r12,%r11,8(x))
        movq    8(x), %rbx
        mulpadd(%rbp,%r13,%r12,24(x))
        mulpadd(%rbp,%r14,%r13,32(x))
        mulpade(%rbp,%r15,%r14,40(x))
        adcq    $0, %rcx

// Now chain in the 04 + 23 + 24 + 25 + 35 + 45 terms
// We are running out of registers in our rotating window, so we start
// using %rbx (and hence need care with using mulpadd after this). Thus
// our result so far is in [%rbp;%rbx;%rcx;%r15;...%r9]

        movq    32(x), %rbx
        mulpadi(%rbp,%r13,%r12,(x))
        movq    16(x), %rbx
        mulpadd(%rbp,%r14,%r13,24(x))
        mulpadd(%rbp,%r15,%r14,32(x))
        mulpadd(%rbp,%rcx,%r15,40(x))

        xorl    %ebx, %ebx
        movq    24(x), %rax
        mulq     40(x)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    32(x), %rax
        mulq     40(x)
        addq    %rax, %rbx
        adcq    %rdx, %rbp

// Double the window as [%r8;%rbp;%rbx;%rcx;%r15;...%r9]

        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d

// Add the doubled window to the 00 + 11 + 22 + 33 + 44 + 55 terms
// For one glorious moment the entire squaring result is all in the
// register file as [%rsi;%rbp;%rbx;%rcx;%r15;...;%r8]
// (since we've now finished with x we can re-use %rsi). But since
// we are so close to running out of registers, we do a bit of
// reshuffling and temporary storage in the output buffer.

        movq    (x), %rax
        mulq    %rax
        movq    %r8, (z)
        movq    %rax, %r8
        movq    8(x), %rax
        movq    %rbp, 8(z)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp

        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp

        movq    16(x), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp

        movq    24(x), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp

        movq    32(x), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp

        movq    40(x), %rax
        mulq    %rax
        negq    %rbp
        adcq    8(z), %rax
        adcq    (z), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rsi

// We need just *one* more register as a temp for the Montgomery steps.
// Since we are writing to the z buffer anyway, make use of that again
// to stash %rbx.

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

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
