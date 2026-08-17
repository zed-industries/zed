// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert little-endian bytes to 9-digit 528-bit bignum
//
//    extern void bignum_fromlebytes_p521(uint64_t z[static 9],
//                                        const uint8_t x[static 66]);
//
// The result will be < 2^528 since it is translated from 66 bytes.
// It is mainly intended for inputs x < p_521 < 2^521 < 2^528.
//
// Since x86 is little-endian, this is just copying.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_fromlebytes_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_fromlebytes_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_fromlebytes_p521)
        .text

#define z %rdi
#define x %rsi
#define a %rax

S2N_BN_SYMBOL(bignum_fromlebytes_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

        movq    (x), a
        movq    a, (z)

        movq    8(x), a
        movq    a, 8(z)

        movq    16(x), a
        movq    a, 16(z)

        movq    24(x), a
        movq    a, 24(z)

        movq    32(x), a
        movq    a, 32(z)

        movq    40(x), a
        movq    a, 40(z)

        movq    48(x), a
        movq    a, 48(z)

        movq    56(x), a
        movq    a, 56(z)

        xorl    %eax, %eax
        movw    64(x), %ax
        movq    a, 64(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_fromlebytes_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
