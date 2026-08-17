// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate, z := -x (if p nonzero) or z := x (if p zero)
// Inputs p, x[k]; outputs function return (nonzero input) and z[k]
//
//    extern uint64_t bignum_optneg(uint64_t k, uint64_t *z, uint64_t p,
//                                  const uint64_t *x);
//
// It is assumed that both numbers x and z have the same size k digits.
// Returns a carry, which is equivalent to "x is nonzero".
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = p, X3 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg)
        .text
        .balign 4

#define k x0
#define z x1
#define p x2
#define x x3

#define a x4
#define i x5


S2N_BN_SYMBOL(bignum_optneg):
        CFI_START

// if k = 0 do nothing. This also has the right top carry zero in x0

        cbz     k, Lbignum_optneg_end

// Convert p into a strict bitmask

        cmp     p, xzr
        csetm   p, ne

// Generate an initial carry-in for the negating case only to add 1; this
// is because we are actually going to do complements of the words of x

        adds    xzr, p, p

// Main loop
        mov     i, xzr
Lbignum_optneg_loop:
        ldr     a, [x, i]
        eor     a, a, p
        adcs    a, a, xzr
        str     a, [z, i]
        add     i, i, #8
        sub     k, k, #1
        cbnz    k, Lbignum_optneg_loop

// Return carry flag, fixing up inversion for negative case

        adc     x0, xzr, xzr
        neg     p, p
        eor     x0, x0, p

Lbignum_optneg_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
