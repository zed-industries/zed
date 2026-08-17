// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add modulo p_sm2, z := (x + y) mod p_sm2, assuming x and y reduced
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_add_sm2(uint64_t z[static 4], const uint64_t x[static 4],
//                               const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_add_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_add_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_add_sm2)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2
#define c x3
#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7
#define n0 x8
#define n1 x9
#define n2 x10
#define n3 x11

S2N_BN_SYMBOL(bignum_add_sm2):
        CFI_START

// First just add the numbers as [c;d3;d2;d1;d0]

        ldp     d0, d1, [x]
        ldp     n0, n1, [y]
        adds    d0, d0, n0
        adcs    d1, d1, n1
        ldp     d2, d3, [x, #16]
        ldp     n2, n3, [y, #16]
        adcs    d2, d2, n2
        adcs    d3, d3, n3
        adc     c, xzr, xzr

// Now let [c;n3;n2;n1;n0] = [c;d3;d2;d1;d0] - p_sm2

        subs    n0, d0, #0xffffffffffffffff
        mov     n1, #0xffffffff00000000
        sbcs    n1, d1, n1
        adcs    n2, d2, xzr
        mov     n3, #0xfffffffeffffffff
        sbcs    n3, d3, n3
        sbcs    c, c, xzr

// Select result according to whether (x + y) - p_sm2 < 0

        csel    d0, d0, n0, cc
        csel    d1, d1, n1, cc
        csel    d2, d2, n2, cc
        csel    d3, d3, n3, cc

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_add_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
