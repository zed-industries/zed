// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo p_384, z := (x - y) mod p_384
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_sub_p384(uint64_t z[static 6], const uint64_t x[static 6],
//                                const uint64_t y[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sub_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sub_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sub_p384)
        .text

#define z %rdi
#define x %rsi
#define y %rdx

#define d0 %rax
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11

// Re-use the input pointers as temporaries once we're done

#define a %rsi
#define c %rdx

#define ashort %esi



S2N_BN_SYMBOL(bignum_sub_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Subtract the inputs as [d5;d4;d3;d2;d1;d0] = x - y (modulo 2^384)
// Capture the top carry as a bitmask for the condition x < y

        movq    (x), d0
        subq    (y), d0
        movq    8(x), d1
        sbbq    8(y), d1
        movq    16(x), d2
        sbbq    16(y), d2
        movq    24(x), d3
        sbbq    24(y), d3
        movq    32(x), d4
        sbbq    32(y), d4
        movq    40(x), d5
        sbbq    40(y), d5
        sbbq    c, c

// Use mask to make r' = mask * (2^384 - p_384) for a compensating subtraction
// of r_384 = 2^384 - p_384, equivalent to an addition of p_384.
// We don't quite have enough ABI-modifiable registers to create all three
// nonzero digits of r while maintaining d0..d5, but make the first two now.

        movl    $0x00000000ffffffff, ashort
        andq    a, c // c = masked 0x00000000ffffffff
        xorq    a, a
        subq    c, a // a = masked 0xffffffff00000001

// Do the first two digits of addition and writeback

        subq    a, d0
        movq    d0, (z)
        sbbq    c, d1
        movq    d1, 8(z)

// Preserve the carry chain while creating the extra masked digit since
// the logical operation will clear CF

        sbbq    d0, d0
        andq    a, c // c = masked 0x0000000000000001
        negq    d0

// Do the rest of the addition and writeback

        sbbq    c, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sub_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
