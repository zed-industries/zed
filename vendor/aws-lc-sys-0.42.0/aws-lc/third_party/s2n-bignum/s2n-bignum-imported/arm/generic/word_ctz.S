// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count trailing zero bits in a single word
// Input a; output function return
//
//    extern uint64_t word_ctz(uint64_t a);
//
// Standard ARM ABI: X0 = a, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_ctz)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_ctz)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_ctz)
        .text
        .balign 4

S2N_BN_SYMBOL(word_ctz):
        CFI_START

// ARM doesn't have a direct word ctz instruction, so we emulate it via
// ctz(w) = 64 - clz(~w & (w-1)). This is depending, for cases of the form
// ctz(....1), on the behavior clz(0) = 64, which is guaranteed according
// to the ARM manual.

        mvn     x1, x0
        sub     x0, x0, #1
        and     x0, x0, x1
        clz     x1, x0
        mov     x0, #64
        sub     x0, x0, x1

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_ctz)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
