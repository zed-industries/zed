// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Normalize bignum in-place by shifting left till top bit is 1
// Input z[k]; outputs function return (bits shifted left) and z[k]
//
//    extern uint64_t bignum_normalize(uint64_t k, uint64_t *z);
//
// Given a k-digit bignum z, this function shifts it left by its number of
// leading zero bits, to give result with top bit 1, unless the input number
// was 0. The return is the same as the output of bignum_clz, i.e. the number
// of bits shifted (nominally 64 * k in the case of zero input).
//
// Standard ARM ABI: X0 = k, X1 = z, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_normalize)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_normalize)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_normalize)
        .text
        .balign 4

#define k x0
#define z x1

// This is the return value we accumulate

#define r x2

// Other variables

#define a x3
#define b x4
#define c x5
#define d x6
#define i x7
#define j x8
#define l x9


S2N_BN_SYMBOL(bignum_normalize):
        CFI_START

// If k = 0 the whole operation is trivial. Otherwise initialize
// shift count r and top digit c, but then if k = 1 skip the digitwise part

        subs    i, k, #1
        bcc     Lbignum_normalize_end
        ldr     c, [z, i, lsl #3]
        mov     r, xzr
        beq     Lbignum_normalize_bitpart

// Do a rather stupid but constant-time digit normalization, conditionally
// shifting left (k-1) times based on whether the top word is zero.
// With careful binary striding this could be O(k*log(k)) instead of O(k^2)
// while still retaining the constant-time style.

Lbignum_normalize_normloop:
        mov     j, xzr
        cmp     c, xzr
        cinc    r, r, eq
        mov     a, xzr
Lbignum_normalize_shufloop:
        mov     c, a
        ldr     a, [z, j, lsl #3]
        csel    c, c, a, eq
        str     c, [z, j, lsl #3]
        add     j, j, #1
        sub     d, j, k
        cbnz    d, Lbignum_normalize_shufloop
        subs    i, i, #1
        bne     Lbignum_normalize_normloop

// We now have the top digit nonzero, assuming the input was nonzero,
// and as per the invariant of the loop above, c holds that digit. So
// now just count c's leading zeros and shift z bitwise that many bits.

Lbignum_normalize_bitpart:
        lsl     r, r, #6
        clz     c, c
        add     r, r, c

        mov     b, xzr
        mov     i, xzr
        ands    xzr, c, #63
        csetm   l, ne
        neg     d, c
Lbignum_normalize_bitloop:
        ldr     j, [z, i, lsl #3]
        lsl     a, j, c
        orr     a, a, b
        lsr     b, j, d
        and     b, b, l
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_normalize_bitloop

// Return the final shift count

        mov     x0, r

Lbignum_normalize_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_normalize)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
