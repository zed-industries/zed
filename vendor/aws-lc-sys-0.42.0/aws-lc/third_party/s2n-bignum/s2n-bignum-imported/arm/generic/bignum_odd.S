// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignum for odd-ness
// Input x[k]; output function return
//
//    extern uint64_t bignum_odd(uint64_t k, const uint64_t *x);
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_odd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_odd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_odd)
        .text
        .balign 4

S2N_BN_SYMBOL(bignum_odd):
        CFI_START

cbz     x0, Lbignum_odd_end                 // if k = 0, that's the return!
        ldr     x0, [x1]
        and     x0, x0, #1

Lbignum_odd_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_odd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
