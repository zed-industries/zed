// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_384, z := (-x) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_neg_p384(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p384)
        .text

#define z %rdi
#define x %rsi

#define n0 %rax
#define n1 %rcx
#define n2 %rdx
#define n3 %r8
#define n4 %r9
#define q %r10

#define n0short %eax

S2N_BN_SYMBOL(bignum_neg_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Or together the input digits and create a bitmask q if this is nonzero, so
// that we avoid doing -0 = p_384 and hence maintain strict modular reduction

        movq    (x), n0
        orq     8(x), n0
        movq    16(x), n1
        orq     24(x), n1
        movq    32(x), n2
        orq     40(x), n2
        orq     n1, n0
        orq     n2, n0
        negq    n0
        sbbq    q, q

// Let [q;n4;n3;n2;n1;n0] = if q then p_384 else 0

        movl    $0x00000000ffffffff, n0short
        andq    q, n0
        movq    $0xffffffff00000000, n1
        andq    q, n1
        movq    $0xfffffffffffffffe, n2
        andq    q, n2
        movq    q, n3
        movq    q, n4

// Do the subtraction

        subq    (x), n0
        sbbq    8(x), n1
        sbbq    16(x), n2
        sbbq    24(x), n3
        sbbq    32(x), n4
        sbbq    40(x), q

// Write back

        movq    n0, (z)
        movq    n1, 8(z)
        movq    n2, 16(z)
        movq    n3, 24(z)
        movq    n4, 32(z)
        movq    q, 40(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
