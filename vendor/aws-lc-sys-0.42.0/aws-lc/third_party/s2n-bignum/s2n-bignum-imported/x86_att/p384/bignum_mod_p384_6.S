// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_mod_p384_6(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p384_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p384_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p384_6)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define c %rax

#define cshort %eax

// Re-use the input pointer as a temporary once we're done

#define a %rsi



S2N_BN_SYMBOL(bignum_mod_p384_6):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the input and subtract p_384 from it

        movq    (x), d0
        movl    $0x00000000ffffffff, cshort
        subq    c, d0
        movq    8(x), d1
        notq    c
        sbbq    c, d1
        movq    16(x), d2
        sbbq    $-2, d2
        movq    24(x), d3
        sbbq    $-1, d3
        movq    32(x), d4
        sbbq    $-1, d4
        movq    40(x), d5
        sbbq    $-1, d5

// Capture the top carry as a bitmask to indicate we need to add p_384 back on,
// which we actually do in a more convenient way by subtracting r_384
// where r_384 = [-1; 0; 0; 0; 1; 0x00000000ffffffff; 0xffffffff00000001]
// We don't quite have enough ABI-modifiable registers to create all three
// nonzero digits of r while maintaining d0..d5, but make the first two now.

        notq    c
        sbbq    a, a
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

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p384_6)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
