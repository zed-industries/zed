// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Copy bignum with zero-extension or truncation, z := x
// Input x[n]; output z[k]
//
//    extern void bignum_copy(uint64_t k, uint64_t *z, uint64_t n, const uint64_t *x);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_copy)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_copy)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_copy)
        .text
        .balign 4

#define k x0
#define z x1
#define n x2
#define x x3

#define i x4
#define a x5


S2N_BN_SYMBOL(bignum_copy):
        CFI_START

// Replace n with min(k,n) so we are definitely safe copying those
// Initialize the element counter to 0

        cmp     k, n
        csel    n, k, n, cc
        mov     i, #0

// If min(k,n) = 0 jump to the padding stage

        cbz     n, Lbignum_copy_padding

Lbignum_copy_copyloop:
        ldr     a, [x, i, lsl #3]
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, n
        bcc     Lbignum_copy_copyloop

Lbignum_copy_padding:
        cmp     i, k
        bcs     Lbignum_copy_end
Lbignum_copy_padloop:
        str     xzr, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_copy_padloop

Lbignum_copy_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_copy)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
