// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_521, z := (x / 2) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_half_p521(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p521)
        .text
        .balign 4

#define z x0
#define x x1

// We use distinct variables for clarity, but these are heavily aliased

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x2
#define d5 x3
#define d6 x4
#define d7 x5
#define d8 x2
#define a x6


S2N_BN_SYMBOL(bignum_half_p521):
        CFI_START

// We do a 521-bit rotation one bit right, since 2^521 == 1 (mod p_521)

        ldp     d0, d1, [x]
        and     a, d0, #1
        extr    d0, d1, d0, #1

        ldp     d2, d3, [x, #16]
        extr    d1, d2, d1, #1
        stp     d0, d1, [z]
        extr    d2, d3, d2, #1

        ldp     d4, d5, [x, #32]
        extr    d3, d4, d3, #1
        stp     d2, d3, [z, #16]
        extr    d4, d5, d4, #1

        ldp     d6, d7, [x, #48]
        extr    d5, d6, d5, #1
        stp     d4, d5, [z, #32]
        extr    d6, d7, d6, #1

        ldr     d8, [x, #64]
        extr    d7, d8, d7, #1
        stp     d6, d7, [z, #48]
        lsl     d8, d8, #55
        extr    d8, a, d8, #56
        str     d8, [z, #64]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
