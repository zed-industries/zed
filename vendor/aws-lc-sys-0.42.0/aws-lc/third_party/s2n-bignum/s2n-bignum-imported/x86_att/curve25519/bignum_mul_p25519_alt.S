// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_25519, z := (x * y) mod p_25519
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_mul_p25519_alt(uint64_t z[static 4],
//                                      const uint64_t x[static 4],
//                                      const uint64_t y[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p25519_alt)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// Re-use input pointers later for constant and top carry

#define d %rsi
#define c %rcx

#define dshort %esi

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

S2N_BN_SYMBOL(bignum_mul_p25519_alt):
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
        combadd(%r11,%r10,%r9,8(x),(y))

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
// and this is == 38 * h + l (mod p_25519)

        movl    $38, dshort

        movq    %r12, %rax
        mulq    d
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    c, c

        movq    %r13, %rax
        mulq    d
        subq    c, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    c, c

        movq    %r14, %rax
        mulq    d
        subq    c, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    c, c

        movq    %r15, %rax
        mulq    d
        subq    c, %rdx
        xorq    c, c
        addq    %rax, %r11
        movq    %rdx, %r12
        adcq    c, %r12

// Now we have reduced to 5 digits, 2^255 * h + l = [%r12,%r11,%r10,%r9,%r8]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.

        shldq   $1, %r11, %r12
        leaq    1(%r12), %rax
        movl    $19, dshort
        bts     $63, %r11
        imulq   d, %rax
        addq    %rax, %r8
        adcq    c, %r9
        adcq    c, %r10
        adcq    c, %r11

// Now the effective answer is 2^256 * (CF - 1) + [u3,u2,u1,u0]
// So we correct if CF = 0 by subtracting 19, either way masking to
// 255 bits, i.e. by effectively adding p_25519 to the "full" answer

        sbbq    %rax, %rax
        notq    %rax
        andq    d, %rax
        subq    %rax, %r8
        sbbq    c, %r9
        sbbq    c, %r10
        sbbq    c, %r11
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

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p25519_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
