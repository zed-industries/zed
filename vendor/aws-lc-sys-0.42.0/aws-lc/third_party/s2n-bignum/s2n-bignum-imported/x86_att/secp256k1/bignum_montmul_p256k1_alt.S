// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_p256k1_alt(uint64_t z[static 4],
//                                          const uint64_t x[static 4],
//                                          const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_256k1, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_256k2 (in particular this is true if we are in
// the "usual" case x < p_256k1 and y < p_256k1).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p256k1_alt)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// Re-used for constants in second part

#define w %rsi

// Macro for the key "multiply and add to (c,h,l)" step

#define combadd(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A minutely shorter form for when c = 0 initially

#define combadz(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    c, c

// A short form where we don't expect a top carry

#define combads(h,l,numa,numb)                  \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h

S2N_BN_SYMBOL(bignum_montmul_p256k1_alt):
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
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Copy y into a safe register to start with

        movq    %rdx, y

// Start the window as [%r10;%r9;%r8] with 00 product

        movq    (x), %rax
        mulq     (y)
        movq    %rax, %r8
        movq    %rdx, %r9
        xorq    %r10, %r10

// Column 1

        xorq    %r11, %r11
        combads(%r10,%r9,(x),8(y))
        combadz(%r11,%r10,%r9,8(x),(y))

// Column 2

        xorq    %r12, %r12
        combadz(%r12,%r11,%r10,(x),16(y))
        combadd(%r12,%r11,%r10,8(x),8(y))
        combadd(%r12,%r11,%r10,16(x),(y))

// Column 3

        xorq    %r13, %r13
        combadz(%r13,%r12,%r11,(x),24(y))
        combadd(%r13,%r12,%r11,8(x),16(y))
        combadd(%r13,%r12,%r11,16(x),8(y))
        combadd(%r13,%r12,%r11,24(x),(y))

// Column 4

        xorq    %r14, %r14
        combadz(%r14,%r13,%r12,8(x),24(y))
        combadd(%r14,%r13,%r12,16(x),16(y))
        combadd(%r14,%r13,%r12,24(x),8(y))

// Column 5

        xorq    %r15, %r15
        combadz(%r15,%r14,%r13,16(x),24(y))
        combadd(%r15,%r14,%r13,24(x),16(y))

// Final work for columns 6 and 7

        movq    24(x), %rax
        mulq     24(y)
        addq    %rax, %r14
        adcq    %rdx, %r15

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
