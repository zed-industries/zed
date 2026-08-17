// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count trailing zero digits (64-bit words)
// Input x[k]; output function return
//
//    extern uint64_t bignum_ctd(uint64_t k, const uint64_t *x);
//
// In the case of a zero bignum as input the result is k
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ctd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ctd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ctd)
        .text
        .balign 4

#define k x0
#define x x1
#define i x2
#define a x3


S2N_BN_SYMBOL(bignum_ctd):
        CFI_START

// If the bignum is zero-length, x0 is already the right answer of 0

        cbz     k, Lbignum_ctd_end

// Record in i that the lowest nonzero word is i, where i = k means
// that the bignum was entirely zero

        mov     i, k
Lbignum_ctd_loop:
        sub     k, k, #1
        ldr     a, [x, k, lsl #3]
        cmp     a, #0
        csel    i, k, i, ne
        cbnz    k, Lbignum_ctd_loop

// Now return i

        mov     x0, i

Lbignum_ctd_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_ctd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
