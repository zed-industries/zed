// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// 384-bit multiplex/select z := x (if p nonzero) or z := y (if p zero)
// Inputs p, x[6], y[6]; output z[6]
//
//    extern void bignum_mux_6(uint64_t p, uint64_t z[static 6],
//                             const uint64_t x[static 6],
//                             const uint64_t y[static 6]);
//
// It is assumed that all numbers x, y and z have the same size 6 digits.
//
// Standard ARM ABI: X0 = p, X1 = z, X2 = x, X3 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mux_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mux_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mux_6)
        .text
        .balign 4

#define p x0
#define z x1
#define x x2
#define y x3
#define a x4


S2N_BN_SYMBOL(bignum_mux_6):
        CFI_START

cmp     p, #0                    // Set condition codes p = 0

        ldr     a, [x]
        ldr     p, [y]
        csel    a, a, p, ne
        str     a, [z]

        ldr     a, [x, #8]
        ldr     p, [y, #8]
        csel    a, a, p, ne
        str     a, [z, #8]

        ldr     a, [x, #16]
        ldr     p, [y, #16]
        csel    a, a, p, ne
        str     a, [z, #16]

        ldr     a, [x, #24]
        ldr     p, [y, #24]
        csel    a, a, p, ne
        str     a, [z, #24]

        ldr     a, [x, #32]
        ldr     p, [y, #32]
        csel    a, a, p, ne
        str     a, [z, #32]

        ldr     a, [x, #40]
        ldr     p, [y, #40]
        csel    a, a, p, ne
        str     a, [z, #40]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mux_6)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
