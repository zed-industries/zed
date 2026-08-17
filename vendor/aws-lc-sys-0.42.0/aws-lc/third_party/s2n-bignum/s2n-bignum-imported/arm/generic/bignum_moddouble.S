// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo m, z := (2 * x) mod m, assuming x reduced
// Inputs x[k], m[k]; output z[k]
//
//    extern void bignum_moddouble(uint64_t k, uint64_t *z, const uint64_t *x,
//                                 const uint64_t *m);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_moddouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_moddouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_moddouble)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define m x3
#define i x4
#define j x5
#define a x6
#define b x7
#define c x8


S2N_BN_SYMBOL(bignum_moddouble):
        CFI_START

adds    j, k, xzr               // j = k and ZF = (k = 0)
        beq     Lbignum_moddouble_end                     // if k = 0 do nothing

// Do (_::z) = 2 * x - m and generate a mask in c for 2 * x < m

        mov     c, xzr
        subs    i, xzr, xzr             // i = 0 and CF = 1
Lbignum_moddouble_dubloop:
        ldr     a, [x, i]
        extr    c, a, c, #63
        ldr     b, [m, i]
        sbcs    c, c, b
        str     c, [z, i]
        mov     c, a
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_moddouble_dubloop
        lsr     c, c, #63
        sbc     c, c, xzr

// Now do a corrective masked addition z := z + [c] * m

        mov     j, k
        adds    i, xzr, xzr
Lbignum_moddouble_corrloop:
        ldr     a, [z, i]
        ldr     b, [m, i]
        and     b, b, c
        adcs    a, a, b
        str     a, [z, i]
        add     i, i, #8
        sub     j, j, #1
        cbnz    j, Lbignum_moddouble_corrloop

Lbignum_moddouble_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_moddouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
