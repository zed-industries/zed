// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Divide bignum by 10, returning remainder: z' := z div 10, return = z mod 10
// Inputs z[k]; outputs function return (remainder) and z[k]
//
//    extern uint64_t bignum_divmod10(uint64_t k, uint64_t *z);
//
// Standard ARM ABI: X0 = k, X1 = z, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_divmod10)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_divmod10)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_divmod10)
        .text
        .balign 4

#define k x0
#define z x1

#define d x2

#define h x3
#define q x3

#define l x4
#define r x4

#define w x5
#define s x6

S2N_BN_SYMBOL(bignum_divmod10):
        CFI_START

// If k = 0 then return; the return in x0 is indeed 0 mod 10 = 0

        cbz     k, Lbignum_divmod10_end

// Straightforward top-down loop doing 10 * q + r' := 2^64 * r + d

        mov     r, xzr
        mov     w, 0x3333333333333333
        add     s, w, 1
        and     w, w, 0xfffffff
Lbignum_divmod10_divloop:
        sub     k, k, 1
        ldr     d, [z, k, lsl #3]

// First re-split and shift so 2^28 * h + l = (2^64 * r + d) / 2
// Then (2^64 * r + d) / 10 = [(2^28 - 1) / 5] * h + (h + l) / 5

        extr    h, r, d, 29
        ubfx    l, d, 1, 28

        add     l, h, l

        mul     h, h, w
        umulh   l, l, s
        add     q, h, l
        str     q, [z, k, lsl #3]

// Generate the new remainder r = d - 10 * q
// Since r <= 9 we only need the low part computation ignoring carries

        add     q, q, q, lsl #2
        sub     r, d, q, lsl #1

        cbnz    k, Lbignum_divmod10_divloop

// Return the final remainder

        mov     x0, r

Lbignum_divmod10_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_divmod10)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
