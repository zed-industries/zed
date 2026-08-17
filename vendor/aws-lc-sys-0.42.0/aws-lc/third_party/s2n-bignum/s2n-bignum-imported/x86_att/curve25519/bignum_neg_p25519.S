// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_25519, z := (-x) mod p_25519, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_neg_p25519(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p25519)
        .text

#define z %rdi
#define x %rsi

#define q %rdx
#define n0 %rax
#define n1 %rcx
#define n2 %r8
#define n3 %r9

#define c %r10

#define qshort %esi

S2N_BN_SYMBOL(bignum_neg_p25519):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the 4 digits of x and let q be an OR of all the digits

        movq    (x), n0
        movq    n0, q
        movq    8(x), n1
        orq     n1, q
        movq    16(x), n2
        orq     n2, q
        movq    24(x), n3
        orq     n3, q

// Turn q into a strict x <> 0 bitmask, and c into a masked constant [-19]
// so that [q;q;q;c] = [2^256 - 19], masked according to nonzeroness of x

        negq    q
        sbbq    q, q
        movq    $-19, c
        andq    q, c

// Now just do [2^256 - 19] - x and then mask to 255 bits,
// which means in effect the required [2^255 - 19] - x

        subq    n0, c
        movq    c, (z)
        movq    q, c
        sbbq    n1, c
        movq    c, 8(z)
        movq    q, c
        sbbq    n2, c
        movq    c, 16(z)
        sbbq    n3, q
        btr     $63, q
        movq    q, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
