// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert little-endian bytes to 9-digit 528-bit bignum
//
//    extern void bignum_fromlebytes_p521(uint64_t z[static 9],
//                                        const uint8_t x[static 66]);
//
// The result will be < 2^528 since it is translated from 66 bytes.
// It is mainly intended for inputs x < p_521 < 2^521 < 2^528.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_fromlebytes_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_fromlebytes_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_fromlebytes_p521)
        .text
        .balign 4

#define z x0
#define x x1

#define d x2
#define dshort w2
#define a x3

S2N_BN_SYMBOL(bignum_fromlebytes_p521):
        CFI_START

// word 0

        ldrb    dshort, [x]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #1]
        extr    a, d, a, #8
        ldrb    dshort, [x, #2]
        extr    a, d, a, #8
        ldrb    dshort, [x, #3]
        extr    a, d, a, #8
        ldrb    dshort, [x, #4]
        extr    a, d, a, #8
        ldrb    dshort, [x, #5]
        extr    a, d, a, #8
        ldrb    dshort, [x, #6]
        extr    a, d, a, #8
        ldrb    dshort, [x, #7]
        extr    a, d, a, #8
        str     a, [z]

// word 1

        ldrb    dshort, [x, #8]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #9]
        extr    a, d, a, #8
        ldrb    dshort, [x, #10]
        extr    a, d, a, #8
        ldrb    dshort, [x, #11]
        extr    a, d, a, #8
        ldrb    dshort, [x, #12]
        extr    a, d, a, #8
        ldrb    dshort, [x, #13]
        extr    a, d, a, #8
        ldrb    dshort, [x, #14]
        extr    a, d, a, #8
        ldrb    dshort, [x, #15]
        extr    a, d, a, #8
        str     a, [z, #8]

// word 2

        ldrb    dshort, [x, #16]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #17]
        extr    a, d, a, #8
        ldrb    dshort, [x, #18]
        extr    a, d, a, #8
        ldrb    dshort, [x, #19]
        extr    a, d, a, #8
        ldrb    dshort, [x, #20]
        extr    a, d, a, #8
        ldrb    dshort, [x, #21]
        extr    a, d, a, #8
        ldrb    dshort, [x, #22]
        extr    a, d, a, #8
        ldrb    dshort, [x, #23]
        extr    a, d, a, #8
        str     a, [z, #16]

// word 3

        ldrb    dshort, [x, #24]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #25]
        extr    a, d, a, #8
        ldrb    dshort, [x, #26]
        extr    a, d, a, #8
        ldrb    dshort, [x, #27]
        extr    a, d, a, #8
        ldrb    dshort, [x, #28]
        extr    a, d, a, #8
        ldrb    dshort, [x, #29]
        extr    a, d, a, #8
        ldrb    dshort, [x, #30]
        extr    a, d, a, #8
        ldrb    dshort, [x, #31]
        extr    a, d, a, #8
        str     a, [z, #24]

// word 4

        ldrb    dshort, [x, #32]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #33]
        extr    a, d, a, #8
        ldrb    dshort, [x, #34]
        extr    a, d, a, #8
        ldrb    dshort, [x, #35]
        extr    a, d, a, #8
        ldrb    dshort, [x, #36]
        extr    a, d, a, #8
        ldrb    dshort, [x, #37]
        extr    a, d, a, #8
        ldrb    dshort, [x, #38]
        extr    a, d, a, #8
        ldrb    dshort, [x, #39]
        extr    a, d, a, #8
        str     a, [z, #32]

// word 5

        ldrb    dshort, [x, #40]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #41]
        extr    a, d, a, #8
        ldrb    dshort, [x, #42]
        extr    a, d, a, #8
        ldrb    dshort, [x, #43]
        extr    a, d, a, #8
        ldrb    dshort, [x, #44]
        extr    a, d, a, #8
        ldrb    dshort, [x, #45]
        extr    a, d, a, #8
        ldrb    dshort, [x, #46]
        extr    a, d, a, #8
        ldrb    dshort, [x, #47]
        extr    a, d, a, #8
        str     a, [z, #40]

// word 6

        ldrb    dshort, [x, #48]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #49]
        extr    a, d, a, #8
        ldrb    dshort, [x, #50]
        extr    a, d, a, #8
        ldrb    dshort, [x, #51]
        extr    a, d, a, #8
        ldrb    dshort, [x, #52]
        extr    a, d, a, #8
        ldrb    dshort, [x, #53]
        extr    a, d, a, #8
        ldrb    dshort, [x, #54]
        extr    a, d, a, #8
        ldrb    dshort, [x, #55]
        extr    a, d, a, #8
        str     a, [z, #48]

// word 7

        ldrb    dshort, [x, #56]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #57]
        extr    a, d, a, #8
        ldrb    dshort, [x, #58]
        extr    a, d, a, #8
        ldrb    dshort, [x, #59]
        extr    a, d, a, #8
        ldrb    dshort, [x, #60]
        extr    a, d, a, #8
        ldrb    dshort, [x, #61]
        extr    a, d, a, #8
        ldrb    dshort, [x, #62]
        extr    a, d, a, #8
        ldrb    dshort, [x, #63]
        extr    a, d, a, #8
        str     a, [z, #56]

// word 8

        ldrb    dshort, [x, #64]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #65]
        extr    a, d, a, #56
        str     a, [z, #64]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_fromlebytes_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
