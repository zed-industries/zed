// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256k1_alt(uint64_t z[static 4],
//                                          const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256k1, assuming x^2 <= 2^256 * p_256k1, which
// is guaranteed in particular if x < p_256k1 initially (the "intended" case).
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p256k1_alt)
        .text

#define z %rdi
#define x %rsi

// Re-used for constants in second part

#define w %rsi

// Macro for the key "multiply and add to (c,h,l)" step, for square term

#define combadd1(c,h,l,numa)                    \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A short form where we don't expect a top carry

#define combads(h,l,numa)                       \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h

// A version doubling before adding, for non-square terms

#define combadd2(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0, c ;                           \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

S2N_BN_SYMBOL(bignum_montsqr_p256k1_alt):
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

// Result term 0

        movq    (x), %rax
        mulq    %rax

        movq    %rax, %r8
        movq    %rdx, %r9
        xorq    %r10, %r10

// Result term 1

       xorq    %r11, %r11
       combadd2(%r11,%r10,%r9,(x),8(x))

// Result term 2

        xorq    %r12, %r12
        combadd1(%r12,%r11,%r10,8(x))
        combadd2(%r12,%r11,%r10,(x),16(x))

// Result term 3

        xorq    %r13, %r13
        combadd2(%r13,%r12,%r11,(x),24(x))
        combadd2(%r13,%r12,%r11,8(x),16(x))

// Result term 4

        xorq    %r14, %r14
        combadd2(%r14,%r13,%r12,8(x),24(x))
        combadd1(%r14,%r13,%r12,16(x))

// Result term 5

        xorq    %r15, %r15
        combadd2(%r15,%r14,%r13,16(x),24(x))

// Result term 6

        combads(%r15,%r14,24(x))

// Now we have the full 8-digit product 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// Do Montgomery reductions, now using %rcx as a carry-saver.

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

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
