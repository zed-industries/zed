// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally subtract or add, z := x + sgn(p) * y interpreting p as signed
// Inputs x[k], p, y[k]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_optsubadd(uint64_t k, uint64_t *z, const uint64_t *x,
//                                     uint64_t p, const uint64_t *y);
//
// If p has top bit set (i.e. is negative as a signed int) return z := x - y
// Else if p is nonzero (i.e. is positive as a signed int) return z := x + y
// Otherwise (i.e. p is zero) return z := x
//
// Return in X0 = the top carry, which will be 0 or 1, and appropriate for
// addition or subtraction respectively (and always zero for p = 0)
//
// 2^{64*k} * -carryout + z = x - y [for subtraction]
// 2^{64*k} * carryout + z = x + y [for addition]
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = p, X4 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optsubadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optsubadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optsubadd)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define p x3
#define y x4

#define m x3
#define q x5
#define a x6
#define b x7
#define i x8


S2N_BN_SYMBOL(bignum_optsubadd):
        CFI_START

// if k = 0 do nothing. This is also the right top carry in X0

        cbz     k, Lbignum_optsubadd_end

// Turn the input p into two bitmasks, m indicating to use the y input at
// all (same register as p) and q indicating a sign-flip

        cmp     p, xzr
        csetm   m, ne
        csetm   q, mi

// Generate an initial carry-in for the negating case only to add 1; this
// is because we are actually going to do complements of the words of y

        adds    xzr, q, q

// Main loop

        mov     i, xzr
Lbignum_optsubadd_loop:
        ldr     b, [y, i]
        eor     b, b, q
        ldr     a, [x, i]
        and     b, b, m
        adcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     k, k, #1
        cbnz    k, Lbignum_optsubadd_loop

// Return carry flag, fixing up inversion for negative case

        adc     x0, xzr, xzr
        neg     q, q
        eor     x0, x0, q

Lbignum_optsubadd_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optsubadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
