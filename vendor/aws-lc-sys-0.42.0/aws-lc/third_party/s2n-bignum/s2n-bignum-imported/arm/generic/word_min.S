// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Return minimum of two unsigned 64-bit words
// Inputs a, b; output function return
//
//    extern uint64_t word_min(uint64_t a, uint64_t b);
//
// Standard ARM ABI: X0 = a, X1 = b, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_min)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_min)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_min)
        .text
        .balign 4

#define a x0
#define b x1

S2N_BN_SYMBOL(word_min):
        CFI_START

        cmp     a, b
        csel    x0, a, b, cc
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_min)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
