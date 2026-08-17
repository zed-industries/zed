// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add modulo m, z := (x + y) mod m, assuming x and y reduced
// Inputs x[k], y[k], m[k]; output z[k]
//
//    extern void bignum_modadd(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *y, const uint64_t *m);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = y, X4 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modadd)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define y x3
#define m x4
#define i x5
#define j x6
#define a x7
#define b x8
#define c x9


S2N_BN_SYMBOL(bignum_modadd):
        CFI_START

adds    j, k, xzr               // j = k and ZF = (k = 0)
        beq     Lbignum_modadd_end                     // if k = 0 do nothing
        adds    i, xzr, xzr             // i = 0 and CF = 0

// First just add (c::z) := x + y

Lbignum_modadd_addloop:
        ldr     a, [x, i]
        ldr     b, [y, i]
        adcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modadd_addloop
        cset    c, cs

// Now do a comparison subtraction (c::z) - m, recording mask for (c::z) >= m

        mov     j, k
        subs    i, xzr, xzr
Lbignum_modadd_cmploop:
        ldr     a, [z, i]
        ldr     b, [m, i]
        sbcs    xzr, a, b
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modadd_cmploop
        sbcs    c, c, xzr
        mvn     c, c

// Now do a masked subtraction z := z - [c] * m

        mov     j, k
        subs    i, xzr, xzr
Lbignum_modadd_subloop:
        ldr     a, [z, i]
        ldr     b, [m, i]
        and     b, b, c
        sbcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modadd_subloop

Lbignum_modadd_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
