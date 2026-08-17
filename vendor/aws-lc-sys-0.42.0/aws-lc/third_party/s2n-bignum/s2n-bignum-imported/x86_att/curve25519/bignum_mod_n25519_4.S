// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo basepoint order, z := x mod n_25519
// Input x[4]; output z[4]
//
//    extern void bignum_mod_n25519_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Reduction is modulo the order of the curve25519/edwards25519 basepoint.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n25519_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n25519_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n25519_4)
        .text

#define z %rdi
#define x %rsi

#define q %rcx

#define d0 %r8
#define d1 %r9
#define d2 %r10
#define d3 %r11

S2N_BN_SYMBOL(bignum_mod_n25519_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the top digit first, get the quotient estimate q = floor(x/2^252)
// and delete it from that digit, in effect doing x' = x - q * 2^252.
// Now we only need x' - q * n_25519' where n_25519' = n_25519 - 2^252

        movq    24(x), q
        movq    q, d3
        shrq    $60, q
        shlq    $4, d3
        shrq    $4, d3

// Compute [%rdx;d2;d1] = q * n_25519'

        movq    $0x5812631a5cf5d3ed, %rax
        mulq    q
        movq    %rax, d1
        movq    %rdx, d2

        movq    $0x14def9dea2f79cd6, %rax
        mulq    q
        addq    %rax, d2
        adcq    $0, %rdx

// Subtract to get [d3;d2;d1;d0] = x - q * n_25519

        movq    (x), d0
        subq    d1, d0
        movq    8(x), d1
        sbbq    d2, d1
        movq    16(x), d2
        sbbq    %rdx, d2
        sbbq    $0, d3

// Get a bitmask q for the borrow and create masked version of
// non-trivial digits of [%rcx;0;%rdx;%rax] = n_25519. Note that
// %rcx = q but we can get it from the corresponding bit of %rax.

        sbbq    q, q

        movq    $0x5812631a5cf5d3ed, %rax
        andq    q, %rax
        movq    $0x14def9dea2f79cd6, %rdx
        andq    q, %rdx
        movq    $0x1000000000000000, %rcx
        andq    %rax, %rcx

// Do compensating addition (iff subtraction borrowed) and store

        addq    %rax, d0
        movq    d0, (z)
        adcq    %rdx, d1
        movq    d1, 8(z)
        adcq    $0, d2
        movq    d2, 16(z)
        adcq    %rcx, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)

#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n25519_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
