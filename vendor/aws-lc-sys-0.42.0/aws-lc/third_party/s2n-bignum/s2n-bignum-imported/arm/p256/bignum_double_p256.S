// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_256, z := (2 * x) mod p_256, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_double_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p256)
        .text
        .balign 4

#define z x0
#define x x1
#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define c x6
#define n0 x7
#define n1 x8
#define n2 x9
#define n3 x10


S2N_BN_SYMBOL(bignum_double_p256):
        CFI_START

// Double the input number as 2 * x = c + [d3; d2; d1; d0]
// It's worth considering doing this with extr...63 instead

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        adds    d0, d0, d0
        adcs    d1, d1, d1
        adcs    d2, d2, d2
        adcs    d3, d3, d3
        adc     c, xzr, xzr

// Subtract p_256 to give 2 * x - p_256 = c + [n3; n2; n1; n0]

        subs    n0, d0, #0xffffffffffffffff
        mov     n1, #0x00000000ffffffff
        sbcs    n1, d1, n1
        sbcs    n2, d2, xzr
        mov     n3, #0xffffffff00000001
        sbcs    n3, d3, n3
        sbcs    c, c, xzr

// Now CF is set (because of inversion) if 2 * x >= p_256, in which case the
// correct result is [n3; n2; n1; n0], otherwise [d3; d2; d1; d0]

        csel    d0, d0, n0, cc
        csel    d1, d1, n1, cc
        csel    d2, d2, n2, cc
        csel    d3, d3, n3, cc

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
