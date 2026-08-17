// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply bignum by 10 and add word: z := 10 * z + d
// Inputs z[k], d; outputs function return (carry) and z[k]
//
//    extern uint64_t bignum_muladd10(uint64_t k, uint64_t *z, uint64_t d);
//
// Although typically the input d < 10, this is not actually required.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = d, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_muladd10)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_muladd10)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_muladd10)
        .text
        .balign 4

#define k x0
#define z x1
#define d x2

#define i x3
#define h x4

#define a x5
#define l x5

S2N_BN_SYMBOL(bignum_muladd10):
        CFI_START

// If k = 0 just return the input d as the carry (out of zero digits)

        cbz     k, Lbignum_muladd10_end

// Simple loop

        mov     i, xzr
Lbignum_muladd10_loop:
        ldr     a, [z, i, lsl #3]
        lsr     h, a, #61
        add     l, a, a
        add     h, h, h, lsr #2
        adds    l, l, l, lsl #2
        adc     h, h, xzr
        adds    a, l, d
        str     a, [z, i, lsl #3]
        adc     d, h, xzr
        add     i, i, 1
        cmp     i, k
        bcc     Lbignum_muladd10_loop

// Return the final carry

Lbignum_muladd10_end:
        mov     x0, d
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_muladd10)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
