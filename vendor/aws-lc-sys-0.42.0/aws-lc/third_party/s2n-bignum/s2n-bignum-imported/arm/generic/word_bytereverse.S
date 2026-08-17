// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reverse the order of bytes in a 64-bit word
//
//    extern uint64_t word_bytereverse(uint64_t a);
//
// Standard ARM ABI: X0 = a, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_bytereverse)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_bytereverse)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_bytereverse)
        .text
        .balign 4

S2N_BN_SYMBOL(word_bytereverse):
        CFI_START

        mov     x1, #0xFFFF0000FFFF0000
        mov     x2, #0x0000FFFF0000FFFF
        and     x1, x1, x0
        and     x2, x2, x0
        ror     x1, x1, #32
        orr     x0, x1, x2

        mov     x1, #0xFF00FF00FF00FF00
        mov     x2, #0x00FF00FF00FF00FF
        and     x1, x1, x0
        and     x2, x2, x0
        ror     x1, x1, #24
        ror     x2, x2, #8
        orr     x0, x1, x2

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_bytereverse)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
