// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_256, z := (2 * x) mod p_256, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_double_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p256)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9

#define n1 %r10
#define n3 %r11
#define c %rax

#define n1short %r10d



S2N_BN_SYMBOL(bignum_double_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the input and double it so that 2^256 * c + [d3;d2;d1;d0] = 2 * x
// Could also consider using shld to decouple carries

        xorq    c, c
        movq    (x), d0
        addq    d0, d0
        movq    8(x), d1
        adcq    d1, d1
        movq    16(x), d2
        adcq    d2, d2
        movq    24(x), d3
        adcq    d3, d3
        adcq    c, c

// Now subtract 2^256 * c + [d3;d3;d1;d1] = 2 * x - p_256
// The constants n1 and n3 in [n3; 0; n1; -1] = p_256 are saved for later

        subq    $-1, d0
        movl    $0x00000000ffffffff, n1short
        sbbq    n1, d1
        sbbq    $0, d2
        movq    $0xffffffff00000001, n3
        sbbq    n3, d3

// Since by hypothesis x < p_256 we know 2 * x - p_256 < 2^256, so the top
// carry c actually gives us a bitmask for 2 * x - p_256 < 0, which we
// now use to make a masked p_256' = [n3; 0; n1; c]

        sbbq    $0, c
        andq    c, n1
        andq    c, n3

// Do the corrective addition and copy to output

        addq    c, d0
        movq    d0, (z)
        adcq    n1, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    n3, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
