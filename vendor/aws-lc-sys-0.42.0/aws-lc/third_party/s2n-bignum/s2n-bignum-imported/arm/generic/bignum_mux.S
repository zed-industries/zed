// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiplex/select z := x (if p nonzero) or z := y (if p zero)
// Inputs p, x[k], y[k]; output z[k]
//
//    extern void bignum_mux(uint64_t p, uint64_t k, uint64_t *z, const uint64_t *x,
//                           const uint64_t *y);
//
// It is assumed that all numbers x, y and z have the same size k digits.
//
// Standard ARM ABI: X0 = p, X1 = k, X2 = z, X3 = x, X4 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mux)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mux)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mux)
        .text
        .balign 4

#define b x0
#define k x1
#define z x2
#define x x3
#define y x4
#define a x5


S2N_BN_SYMBOL(bignum_mux):
        CFI_START

cbz     k, Lbignum_mux_end                  // if k = 0 skip the Lbignum_mux_loop
        cmp     b, #0                    // Set condition codes b = 0

// We've set cc's from b once and for all and can now re-use "b" as a temporary

Lbignum_mux_loop:
        sub     k, k, #1
        ldr     a, [x, k, lsl #3]
        ldr     b, [y, k, lsl #3]
        csel    a, a, b, ne
        str     a, [z, k, lsl #3]
        cbnz    k, Lbignum_mux_loop

Lbignum_mux_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mux)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
