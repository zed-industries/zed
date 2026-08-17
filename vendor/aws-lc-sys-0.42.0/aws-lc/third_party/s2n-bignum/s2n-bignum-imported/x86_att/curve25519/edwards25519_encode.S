// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Encode edwards25519 point into compressed form as 256-bit number
// Input p[8]; output z[32] (bytes)
//
//    extern void edwards25519_encode(uint8_t z[static 32],
//                                    const uint64_t p[static 8]);
//
// This assumes that the input buffer p points to a pair of 256-bit
// numbers x (at p) and y (at p+4) representing a point (x,y) on the
// edwards25519 curve. It is assumed that both x and y are < p_25519
// but there is no checking of this, nor of the fact that (x,y) is
// in fact on the curve.
//
// The output in z is a little-endian array of bytes corresponding to
// the standard compressed encoding of a point as 2^255 * x_0 + y
// where x_0 is the least significant bit of x.
// See "https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.2"
// In this implementation, y is simply truncated to 255 bits, but if
// it is reduced mod p_25519 as expected this does not affect values.
//
// Standard x86-64 ABI: RDI = z, RSI = p
// Microsoft x64 ABI:   RCX = z, RDX = p
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_encode)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_encode)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_encode)
        .text

#define z %rdi
#define p %rsi
#define y0 %rax
#define y1 %rcx
#define y2 %rdx
#define y3 %r8
#define xb %r9

S2N_BN_SYMBOL(edwards25519_encode):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load lowest word of x coordinate in xb and full y as [y3;y2;y1;y0].

        movq    (p), xb
        movq    32(p), y0
        movq    40(p), y1
        movq    48(p), y2
        movq    56(p), y3

// Compute the encoded form, making the LSB of x the MSB of the encoding

        btr     $63, y3
        shlq    $63, xb
        orq     xb, y3

// Store back (by the word, since x86 is little-endian anyway)

        movq    y0, (z)
        movq    y1, 8(z)
        movq    y2, 16(z)
        movq    y3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_encode)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
