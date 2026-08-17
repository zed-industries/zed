// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select element from 16-element table, z := xs[k*i]
// Inputs xs[16*k], i; output z[k]
//
//    extern void bignum_mux16(uint64_t k, uint64_t *z, const uint64_t *xs,
//                             uint64_t i);
//
// It is assumed that all numbers xs[16] and the target z have the same size k
// The pointer xs is to a contiguous array of size 16, elements size-k bignums
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = xs, X3 = i
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mux16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mux16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mux16)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define i x3

#define a x4
#define b x5
#define j x6
#define n x7


S2N_BN_SYMBOL(bignum_mux16):
        CFI_START

// Copy size into decrementable counter, skip everything if k = 0

        adds    n, k, xzr
        beq     Lbignum_mux16_end

// Multiply i by k so we can compare pointer offsets directly with it

        mul     i, i, k

Lbignum_mux16_loop:

        ldr     a, [x]
        mov     j, k
        .rep 15
        ldr     b, [x, j, lsl #3]
        cmp     j, i
        csel    a, b, a, eq
        add     j, j, k
        .endr
        str     a, [z]

        add     z, z, #8
        add     x, x, #8
        subs    n, n, #1
        bne     Lbignum_mux16_loop

Lbignum_mux16_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mux16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
