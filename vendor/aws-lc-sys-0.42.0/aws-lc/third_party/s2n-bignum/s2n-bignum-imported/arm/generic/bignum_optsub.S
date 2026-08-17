// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally subtract, z := x - y (if p nonzero) or z := x (if p zero)
// Inputs x[k], p, y[k]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_optsub(uint64_t k, uint64_t *z, const uint64_t *x,
//                                  uint64_t p, const uint64_t *y);
//
// It is assumed that all numbers x, y and z have the same size k digits.
// Returns carry-out as per usual subtraction, always 0 if p was zero.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = p, X4 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optsub)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optsub)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optsub)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define p x3
#define m x3
#define y x4
#define a x5
#define b x6
#define i x7


S2N_BN_SYMBOL(bignum_optsub):
        CFI_START

// if k = 0 do nothing. This is also the right top carry in X0

        cbz     k, Lbignum_optsub_end

// Convert p into a strict bitmask (same register in fact)

        cmp     p, xzr
        csetm   m, ne

// Set i = 0 *and* make sure initial ~CF = 0

        subs    i, xzr, xzr

// Main loop

Lbignum_optsub_loop:
        ldr     a, [x, i]
        ldr     b, [y, i]
        and     b, b, m
        sbcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     k, k, #1
        cbnz    k, Lbignum_optsub_loop

// Return (non-inverted) carry flag

        cset    x0, cc

Lbignum_optsub_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optsub)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
