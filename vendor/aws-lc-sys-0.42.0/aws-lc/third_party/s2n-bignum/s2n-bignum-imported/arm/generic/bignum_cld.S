// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count leading zero digits (64-bit words)
// Input x[k]; output function return
//
//    extern uint64_t bignum_cld(uint64_t k, const uint64_t *x);
//
// In the case of a zero bignum as input the result is k
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cld)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cld)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cld)
        .text
        .balign 4

#define k x0
#define x x1
#define i x2
#define a x3
#define j x4


S2N_BN_SYMBOL(bignum_cld):
        CFI_START

// If the bignum is zero-length, x0 is already the right answer of k = 0

        cbz     k, Lbignum_cld_end

// Run over the words j = 0..i-1, and set i := j + 1 when hitting nonzero a[j]

        mov     i, xzr
        mov     j, xzr
Lbignum_cld_loop:
        ldr     a, [x, j, lsl #3]
        add     j, j, #1
        cmp     a, #0
        csel    i, j, i, ne
        cmp     j, k
        bne     Lbignum_cld_loop

        sub     x0, x0, i
Lbignum_cld_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cld)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
