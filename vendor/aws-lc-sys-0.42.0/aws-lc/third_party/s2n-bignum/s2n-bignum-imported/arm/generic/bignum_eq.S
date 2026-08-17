// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignums for equality, x = y
// Inputs x[m], y[n]; output function return
//
//    extern uint64_t bignum_eq(uint64_t m, const uint64_t *x, uint64_t n,
//                              const uint64_t *y);
//
// Standard ARM ABI: X0 = m, X1 = x, X2 = n, X3 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_eq)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_eq)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_eq)
        .text
        .balign 4

#define m x0
#define x x1
#define n x2
#define y x3
#define a x4
#define c x5
//  We can re-use n for this, not needed when d appears
#define d x2


S2N_BN_SYMBOL(bignum_eq):
        CFI_START

// Initialize the accumulated OR of differences to zero

        mov     c, xzr

// If m >= n jump into the m > n loop at the final equality test
// This will drop through for m = n

        cmp     m, n
        bcs     Lbignum_eq_mtest

// Toploop for the case n > m

Lbignum_eq_nloop:
        sub     n, n, #1
        ldr     a, [y, n, lsl #3]
        orr     c, c, a
        cmp     m, n
        bne     Lbignum_eq_nloop
        b       Lbignum_eq_mmain

// Toploop for the case m > n (or n = m which enters at "mtest")

Lbignum_eq_mloop:
        sub     m, m, #1
        ldr     a, [x, m, lsl #3]
        orr     c, c, a
        cmp     m, n
Lbignum_eq_mtest:
        bne     Lbignum_eq_mloop

// Combined main loop for the min(m,n) lower words

Lbignum_eq_mmain:
        cbz     m, Lbignum_eq_end

Lbignum_eq_loop:
        sub     m, m, #1
        ldr     a, [x, m, lsl #3]
        ldr     d, [y, m, lsl #3]
        eor     a, a, d
        orr     c, c, a
        cbnz    m, Lbignum_eq_loop

Lbignum_eq_end:
        cmp     c, xzr
        cset    x0, eq
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_eq)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
