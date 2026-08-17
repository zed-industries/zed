// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count number of set bits in a single 64-bit word (population count)
// Input a; output function return
//
//    extern uint64_t word_popcount(uint64_t a);
//
// Standard ARM ABI: X0 = a, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_popcount)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_popcount)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_popcount)
        .text
        .balign 4

// Very similar to the traditional algorithm, e.g. Hacker's Delight 5-2

S2N_BN_SYMBOL(word_popcount):
        CFI_START

        and     x1, x0, #0xAAAAAAAAAAAAAAAA
        sub     x0, x0, x1, lsr #1

        bic     x1, x0, #0x3333333333333333
        and     x0, x0, #0x3333333333333333
        add     x0, x0, x1, lsr #2

        add     x0, x0, x0, lsr #4
        and     x0, x0, #0x0F0F0F0F0F0F0F0F

        mov     x1, #0x101010101010101
        mul     x0, x0, x1
        lsr     x0, x0, #56

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_popcount)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
