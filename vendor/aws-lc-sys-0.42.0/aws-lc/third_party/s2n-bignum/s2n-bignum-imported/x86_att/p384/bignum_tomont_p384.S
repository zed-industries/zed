// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^384 * x) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_tomont_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p384)
        .text

#define z %rdi
#define x %rsi

// Fairly consistently used as a zero register

#define zero %rbp

// Some temp registers for the last correction stage

#define d %rax
#define u %rdx
#define v %rcx
#define w %rsi

#define vshort %ecx
#define wshort %esi

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rcx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

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
/* Construct [%rbp;%rcx;%rax;-] = (2^384 - p_384) * w */                   \
/* We know the lowest word will cancel so we can re-use d0 as a temp */ \
        xorl    %ebp, %ebp ;                                       \
        movq    $0xffffffff00000001, %rax ;                        \
        mulxq   %rax, %rcx, %rax ;                                  \
        movl    $0x00000000ffffffff, %ecx ;                        \
        mulxq   %rcx, d0, %rcx ;                                   \
        adcq    d0, %rax ;                                        \
        adcq    %rdx, %rcx ;                                       \
        adcl    %ebp, %ebp ;                                       \
/*  Now subtract that and add 2^384 * w */                              \
        subq    %rax, d1 ;                                        \
        sbbq    %rcx, d2 ;                                        \
        sbbq    %rbp, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        sbbq    $0, %rdx ;                                         \
        addq    %rdx, d6 ;                                        \
        adcq    $0, d7

S2N_BN_SYMBOL(bignum_tomont_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// We are essentially just doing a Montgomery multiplication of x and the
// precomputed constant y = 2^768 mod p, so the code is almost the same
// modulo a few registers and the change from loading y[i] to using constants,
// plus the easy digits y[4] = 1 and y[5] = 0 being treated specially.
// Because there is no y pointer to keep, we use one register less.

        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Do row 0 computation, which is a bit different:
// set up initial window [%r14,%r13,%r12,%r11,%r10,%r9,%r8] = y[0] * x
// Unlike later, we only need a single carry chain

        movq    $0xfffffffe00000001, %rdx
        mulxq   (x), %r8, %r9
        mulxq   8(x), %rcx, %r10
        addq    %rcx, %r9
        mulxq   16(x), %rcx, %r11
        adcq    %rcx, %r10
        mulxq   24(x), %rcx, %r12
        adcq    %rcx, %r11
        mulxq   32(x), %rcx, %r13
        adcq    %rcx, %r12
        mulxq   40(x), %rcx, %r14
        adcq    %rcx, %r13
        adcq    $0, %r14

// Montgomery reduce the zeroth window

        xorq    %r15, %r15
        montredc(%r15, %r14,%r13,%r12,%r11,%r10,%r9,%r8)

// Add row 1

        xorq    zero, zero
        movq    $0x0000000200000000, %rdx
        xorq    %r8, %r8
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10,8(x))
        mulpadd(%r12,%r11,16(x))
        mulpadd(%r13,%r12,24(x))
        mulpadd(%r14,%r13,32(x))
        mulpadd(%r15,%r14,40(x))
        adcxq   zero, %r15
        adoxq   zero, %r8
        adcxq   zero, %r8

// Montgomery reduce window 1

        montredc(%r8, %r15,%r14,%r13,%r12,%r11,%r10,%r9)

// Add row 2

        xorq    zero, zero
        movq    $0xfffffffe00000000, %rdx
        xorq    %r9, %r9
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpadd(%r14,%r13,24(x))
        mulpadd(%r15,%r14,32(x))
        mulpadd(%r8,%r15,40(x))
        adcxq   zero, %r8
        adoxq   zero, %r9
        adcxq   zero, %r9

// Montgomery reduce window 2

        montredc(%r9, %r8,%r15,%r14,%r13,%r12,%r11,%r10)

// Add row 3

        xorq    zero, zero
        movq    $0x0000000200000000, %rdx
        xorq    %r10, %r10
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))
        mulpadd(%r15,%r14,24(x))
        mulpadd(%r8,%r15,32(x))
        mulpadd(%r9,%r8,40(x))
        adcxq   zero, %r9
        adoxq   zero, %r10
        adcxq   zero, %r10

// Montgomery reduce window 3

        montredc(%r10, %r9,%r8,%r15,%r14,%r13,%r12,%r11)

// Add row 4. The multiplier y[4] = 1, so we just add x to the window
// while extending it with one more digit, initially this carry

        xorq    %r11, %r11
        addq    (x), %r12
        adcq    8(x), %r13
        adcq    16(x), %r14
        adcq    24(x), %r15
        adcq    32(x), %r8
        adcq    40(x), %r9
        adcq    $0, %r10
        adcq    $0, %r11

// Montgomery reduce window 4

        montredc(%r11, %r10,%r9,%r8,%r15,%r14,%r13,%r12)

// Add row 5, The multiplier y[5] = 0, so this is trivial: all we do is
// bring down another zero digit into the window.

        xorq    %r12, %r12

// Montgomery reduce window 5

        montredc(%r12, %r11,%r10,%r9,%r8,%r15,%r14,%r13)

// We now have a pre-reduced 7-word form [%r12;%r11;%r10;%r9;%r8;%r15;%r14]

// We know, writing B = 2^{6*64} that the full implicit result is
// B^2 c <= z + (B - 1) * p < B * p + (B - 1) * p < 2 * B * p,
// so the top half is certainly < 2 * p. If c = 1 already, we know
// subtracting p will give the reduced modulus. But now we do a
// comparison to catch cases where the residue is >= p.
// First set [0;0;0;w;v;u] = 2^384 - p_384

        movq    $0xffffffff00000001, u
        movl    $0x00000000ffffffff, vshort
        movl    $0x0000000000000001, wshort

// Let dd = [%r11;%r10;%r9;%r8;%r15;%r14] be the topless 6-word intermediate result.
// Set CF if the addition dd + (2^384 - p_384) >= 2^384, hence iff dd >= p_384.

        movq    %r14, d
        addq    u, d
        movq    %r15, d
        adcq    v, d
        movq    %r8, d
        adcq    w, d
        movq    %r9, d
        adcq    $0, d
        movq    %r10, d
        adcq    $0, d
        movq    %r11, d
        adcq    $0, d

// Now just add this new carry into the existing %r12. It's easy to see they
// can't both be 1 by our range assumptions, so this gives us a {0,1} flag

        adcq    $0, %r12

// Now convert it into a bitmask

        negq    %r12

// Masked addition of 2^384 - p_384, hence subtraction of p_384

        andq    %r12, u
        andq    %r12, v
        andq    %r12, w

        addq   u, %r14
        adcq   v, %r15
        adcq   w, %r8
        adcq   $0, %r9
        adcq   $0, %r10
        adcq   $0, %r11

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

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
