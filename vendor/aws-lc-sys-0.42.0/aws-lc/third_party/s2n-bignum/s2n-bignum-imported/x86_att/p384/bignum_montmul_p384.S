// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^384) mod p_384
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_montmul_p384(uint64_t z[static 6],
//                                    const uint64_t x[static 6],
//                                    const uint64_t y[static 6]);
//
// Does z := (2^{-384} * x * y) mod p_384, assuming that the inputs x and y
// satisfy x * y <= 2^384 * p_384 (in particular this is true if we are in
// the "usual" case x < p_384 and y < p_384).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// -----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p384)
        .text

#define z %rdi
#define x %rsi

// We move the y argument here so we can use %rdx for multipliers

#define y %rcx

// Some temp registers for the last correction stage

#define d %rax
#define u %rdx
#define v %rcx
#define w %rbx

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

// Core one-step Montgomery reduction macro. Takes input in
// [d7;d6;d5;d4;d3;d2;d1;d0] and returns result in [d7;d6;d5;d4;d3;d2;d1],
// adding to the existing contents, re-using d0 as a temporary internally
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
//
//       montredc(d7,d6,d5,d4,d3,d2,d1,d0)
//
// This particular variant, with its mix of addition and subtraction
// at the top, is not intended to maintain a coherent carry or borrow out.
// It is assumed the final result would fit in [d7;d6;d5;d4;d3;d2;d1].
// which is always the case here as the top word is even always in {0,1}

#define montredc(d7,d6,d5,d4,d3,d2,d1,d0)                               \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64 */         \
        movq    d0, %rdx ;                                        \
        shlq    $32, %rdx ;                                        \
        addq    d0, %rdx ;                                        \
/* Construct [%rbp;%rbx;%rax;-] = (2^384 - p_384) * w */                   \
/* We know the lowest word will cancel so we can re-use d0 as a temp */ \
        xorl    %ebp, %ebp ;                                       \
        movq    $0xffffffff00000001, %rax ;                        \
        mulxq   %rax, %rbx, %rax ;                                  \
        movl    $0x00000000ffffffff, %ebx ;                        \
        mulxq   %rbx, d0, %rbx ;                                   \
        adcq    d0, %rax ;                                        \
        adcq    %rdx, %rbx ;                                       \
        adcl    %ebp, %ebp ;                                       \
/*  Now subtract that and add 2^384 * w */                              \
        subq    %rax, d1 ;                                        \
        sbbq    %rbx, d2 ;                                        \
        sbbq    %rbp, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        sbbq    $0, %rdx ;                                         \
        addq    %rdx, d6 ;                                        \
        adcq    $0, d7

S2N_BN_SYMBOL(bignum_montmul_p384):
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

// Do row 0 computation, which is a bit different:
// set up initial window [%r14,%r13,%r12,%r11,%r10,%r9,%r8] = y[0] * x
// Unlike later, we only need a single carry chain

        movq    (y), %rdx
        xorl    %r15d, %r15d
        mulxq   (x), %r8, %r9
        mulxq   8(x), %rbx, %r10
        addq    %rbx, %r9
        mulxq   16(x), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   24(x), %rbx, %r12
        adcq    %rbx, %r11
        mulxq   32(x), %rbx, %r13
        adcq    %rbx, %r12
        mulxq   40(x), %rbx, %r14
        adcq    %rbx, %r13
        adcq    %r15, %r14

// Montgomery reduce the zeroth window

        montredc(%r15, %r14,%r13,%r12,%r11,%r10,%r9,%r8)

// Add row 1

        movq    8(y), %rdx
        xorl    %r8d, %r8d
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10, 8(x))
        mulpadd(%r12,%r11,16(x))
        mulpadd(%r13,%r12,24(x))
        mulpadd(%r14,%r13,32(x))
        adoxq   %r8, %r15
        mulxq   40(x), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r8, %r8

// Montgomery reduce window 1

        montredc(%r8, %r15,%r14,%r13,%r12,%r11,%r10,%r9)

// Add row 2

        movq    16(y), %rdx
        xorl    %r9d, %r9d
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpadd(%r14,%r13,24(x))
        mulpadd(%r15,%r14,32(x))
        adoxq   %r9, %r8
        mulxq   40(x), %rax, %rbx
        adcq    %rax, %r15
        adcq    %rbx, %r8
        adcq    %r9, %r9

// Montgomery reduce window 2

        montredc(%r9, %r8,%r15,%r14,%r13,%r12,%r11,%r10)

// Add row 3

        movq    24(y), %rdx
        xorl    %r10d, %r10d
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))
        mulpadd(%r15,%r14,24(x))
        mulpadd(%r8,%r15,32(x))
        adoxq   %r10, %r9
        mulxq   40(x), %rax, %rbx
        adcq    %rax, %r8
        adcq    %rbx, %r9
        adcq    %r10, %r10

// Montgomery reduce window 3

        montredc(%r10, %r9,%r8,%r15,%r14,%r13,%r12,%r11)

// Add row 4

        movq    32(y), %rdx
        xorl    %r11d, %r11d
        mulpadd(%r13,%r12,(x))
        mulpadd(%r14,%r13,8(x))
        mulpadd(%r15,%r14,16(x))
        mulpadd(%r8,%r15,24(x))
        mulpadd(%r9,%r8,32(x))
        adoxq   %r11, %r10
        mulxq   40(x), %rax, %rbx
        adcq    %rax, %r9
        adcq    %rbx, %r10
        adcq    %r11, %r11

// Montgomery reduce window 4

        montredc(%r11, %r10,%r9,%r8,%r15,%r14,%r13,%r12)

// Add row 5

        movq    40(y), %rdx
        xorl    %r12d, %r12d
        mulpadd(%r14,%r13,(x))
        mulpadd(%r15,%r14,8(x))
        mulpadd(%r8,%r15,16(x))
        mulpadd(%r9,%r8,24(x))
        mulpadd(%r10,%r9,32(x))
        adoxq   %r12, %r11
        mulxq   40(x), %rax, %rbx
        adcq    %rax, %r10
        adcq    %rbx, %r11
        adcq    %r12, %r12

// Montgomery reduce window 5

        montredc(%r12, %r11,%r10,%r9,%r8,%r15,%r14,%r13)

// We now have a pre-reduced 7-word form z = [%r12; %r11;%r10;%r9;%r8;%r15;%r14]
// Next, accumulate in different registers z - p_384, or more precisely
//
//   [%r12; %r13;%rbp;%rdx;%rcx;%rbx;%rax] = z + (2^384 - p_384)

        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d

        movq    $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0x00000000ffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x0000000000000001, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0, %r12

// ~ZF <=> %r12 >= 1 <=> z + (2^384 - p_384) >= 2^384 <=> z >= p_384, which
// determines whether to use the further reduced argument or the original z.

        cmovnzq %rax, %r14
        cmovnzq %rbx, %r15
        cmovnzq %rcx, %r8
        cmovnzq %rdx, %r9
        cmovnzq %rbp, %r10
        cmovnzq %r13, %r11

// Write back the result

        movq    %r14, (z)
        movq    %r15, 8(z)
        movq    %r8, 16(z)
        movq    %r9, 24(z)
        movq    %r10, 32(z)
        movq    %r11, 40(z)

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
