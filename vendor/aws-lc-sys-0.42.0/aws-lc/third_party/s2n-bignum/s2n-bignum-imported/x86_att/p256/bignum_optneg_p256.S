// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_256, z := (-x) mod p_256 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[4]; output z[4]
//
//    extern void bignum_optneg_p256(uint64_t z[static 4], uint64_t p,
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = p, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = p, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p256)
        .text

#define z %rdi
#define q %rsi
#define x %rdx

#define n0 %rax
#define n1 %rcx
#define n2 %r8
#define n3 %r9

#define n1short %ecx

S2N_BN_SYMBOL(bignum_optneg_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Adjust q by zeroing it if the input is zero (to avoid giving -0 = p_256,
// which is not strictly reduced even though it's correct modulo p_256).
// This step is redundant if we know a priori that the input is nonzero, which
// is the case for the y coordinate of points on the P-256 curve, for example.

        movq    (x), n0
        orq     8(x), n0
        movq    16(x), n1
        orq     24(x), n1
        orq     n1, n0
        negq    n0
        sbbq    n0, n0
        andq    n0, q

// Turn q into a bitmask, all 1s for q=false, all 0s for q=true

        negq    q
        sbbq    q, q
        notq    q

// Let [n3;n2;n1;n0] = if q then p_256 else -1

        movq    $0xffffffffffffffff, n0
        movl    $0x00000000ffffffff, n1short
        orq     q, n1
        movq    q, n2
        movq    $0xffffffff00000001, n3
        orq     q, n3

// Subtract so [n3;n2;n1;n0] = if q then p_256 - x else -1 - x

        subq    (x), n0
        sbbq    8(x), n1
        sbbq    16(x), n2
        sbbq    24(x), n3

// XOR the words with the bitmask, which in the case q = false has the
// effect of restoring ~(-1 - x) = -(-1 - x) - 1 = 1 + x - 1 = x
// and write back the digits to the output

        xorq    q, n0
        movq    n0, (z)
        xorq    q, n1
        movq    n1, 8(z)
        xorq    q, n2
        movq    n2, 16(z)
        xorq    q, n3
        movq    n3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
