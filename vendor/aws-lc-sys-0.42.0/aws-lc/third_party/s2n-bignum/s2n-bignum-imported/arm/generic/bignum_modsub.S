// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Subtract modulo m, z := (x - y) mod m, assuming x and y reduced
// Inputs x[k], y[k], m[k]; output z[k]
//
//    extern void bignum_modsub(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *y, const uint64_t *m);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = y, X4 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modsub)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modsub)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modsub)
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


S2N_BN_SYMBOL(bignum_modsub):
        CFI_START

adds    j, k, xzr               // j = k and ZF = (k = 0)
        beq     Lbignum_modsub_end                     // if k = 0 do nothing
        subs    i, xzr, xzr             // i = 0 and CF = 1

// Subtract z := x - y and record a mask for the carry x - y < 0

Lbignum_modsub_subloop:
        ldr     a, [x, i]
        ldr     b, [y, i]
        sbcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modsub_subloop
        csetm   c, cc

// Now do a masked addition z := z + [c] * m

        mov     j, k
        adds    i, xzr, xzr
Lbignum_modsub_addloop:
        ldr     a, [z, i]
        ldr     b, [m, i]
        and     b, b, c
        adcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_modsub_addloop

Lbignum_modsub_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modsub)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
