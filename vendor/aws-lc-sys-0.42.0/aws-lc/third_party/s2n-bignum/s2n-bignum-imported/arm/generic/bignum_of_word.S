// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert single digit to bignum, z := n
// Input n; output z[k]
//
//    extern void bignum_of_word(uint64_t k, uint64_t *z, uint64_t n);
//
// Create a k-digit (digit=64 bits) bignum at z with value n (mod 2^k)
// where n is a word. The "mod 2^k" only matters in the degenerate k = 0 case.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_of_word)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_of_word)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_of_word)
        .text
        .balign 4

#define k x0
#define z x1
#define n x2


S2N_BN_SYMBOL(bignum_of_word):
        CFI_START

cbz     k, Lbignum_of_word_end                  // if k = 0 do nothing

        str     n, [z]                  // Set zeroth word to n
        subs    k, k, #1                 // k := k - 1
        beq     Lbignum_of_word_end                     // and if that's 0, finish

Lbignum_of_word_loop:
        str     xzr, [z, k, lsl #3]
        subs    k, k, #1
        bne     Lbignum_of_word_loop

Lbignum_of_word_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_of_word)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
