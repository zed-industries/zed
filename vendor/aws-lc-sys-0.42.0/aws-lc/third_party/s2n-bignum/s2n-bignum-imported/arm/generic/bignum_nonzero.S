// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignum for nonzero-ness x =/= 0
// Input x[k]; output function return
//
//    extern uint64_t bignum_nonzero(uint64_t k, const uint64_t *x);
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_nonzero)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_nonzero)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_nonzero)
        .text
        .balign 4

#define k x0
#define x x1
#define a x2
#define c x3


S2N_BN_SYMBOL(bignum_nonzero):
        CFI_START

mov     c, xzr                  // c will be or of the digits
        cbz     k, Lbignum_nonzero_end                  // if k = 0 skip the Lbignum_nonzero_loop

Lbignum_nonzero_loop:
        sub     k, k, #1
        ldr     a, [x, k, lsl #3]
        orr     c, c, a
        cbnz    k, Lbignum_nonzero_loop

        cmp     c, xzr
        cset    x0, ne

Lbignum_nonzero_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_nonzero)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
