// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert 9-digit 528-bit bignum to little-endian bytes
//
//    extern void bignum_tolebytes_p521(uint8_t z[static 66],
//                                      const uint64_t x[static 9]);
//
// This is assuming the input x is < 2^528 so that it fits in 66 bytes.
// In particular this holds if x < p_521 < 2^521 < 2^528.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tolebytes_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tolebytes_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tolebytes_p521)
        .text
        .balign 4

#define z x0
#define x x1

#define d x2
#define dshort w2

S2N_BN_SYMBOL(bignum_tolebytes_p521):
        CFI_START

// word 0

        ldr     d, [x]
        strb    dshort, [z]
        lsr     d, d, #8
        strb    dshort, [z, #1]
        lsr     d, d, #8
        strb    dshort, [z, #2]
        lsr     d, d, #8
        strb    dshort, [z, #3]
        lsr     d, d, #8
        strb    dshort, [z, #4]
        lsr     d, d, #8
        strb    dshort, [z, #5]
        lsr     d, d, #8
        strb    dshort, [z, #6]
        lsr     d, d, #8
        strb    dshort, [z, #7]

// word 1

        ldr     d, [x, #8]
        strb    dshort, [z, #8]
        lsr     d, d, #8
        strb    dshort, [z, #9]
        lsr     d, d, #8
        strb    dshort, [z, #10]
        lsr     d, d, #8
        strb    dshort, [z, #11]
        lsr     d, d, #8
        strb    dshort, [z, #12]
        lsr     d, d, #8
        strb    dshort, [z, #13]
        lsr     d, d, #8
        strb    dshort, [z, #14]
        lsr     d, d, #8
        strb    dshort, [z, #15]

// word 2

        ldr     d, [x, #16]
        strb    dshort, [z, #16]
        lsr     d, d, #8
        strb    dshort, [z, #17]
        lsr     d, d, #8
        strb    dshort, [z, #18]
        lsr     d, d, #8
        strb    dshort, [z, #19]
        lsr     d, d, #8
        strb    dshort, [z, #20]
        lsr     d, d, #8
        strb    dshort, [z, #21]
        lsr     d, d, #8
        strb    dshort, [z, #22]
        lsr     d, d, #8
        strb    dshort, [z, #23]

// word 3

        ldr     d, [x, #24]
        strb    dshort, [z, #24]
        lsr     d, d, #8
        strb    dshort, [z, #25]
        lsr     d, d, #8
        strb    dshort, [z, #26]
        lsr     d, d, #8
        strb    dshort, [z, #27]
        lsr     d, d, #8
        strb    dshort, [z, #28]
        lsr     d, d, #8
        strb    dshort, [z, #29]
        lsr     d, d, #8
        strb    dshort, [z, #30]
        lsr     d, d, #8
        strb    dshort, [z, #31]

// word 4

        ldr     d, [x, #32]
        strb    dshort, [z, #32]
        lsr     d, d, #8
        strb    dshort, [z, #33]
        lsr     d, d, #8
        strb    dshort, [z, #34]
        lsr     d, d, #8
        strb    dshort, [z, #35]
        lsr     d, d, #8
        strb    dshort, [z, #36]
        lsr     d, d, #8
        strb    dshort, [z, #37]
        lsr     d, d, #8
        strb    dshort, [z, #38]
        lsr     d, d, #8
        strb    dshort, [z, #39]

// word 5

        ldr     d, [x, #40]
        strb    dshort, [z, #40]
        lsr     d, d, #8
        strb    dshort, [z, #41]
        lsr     d, d, #8
        strb    dshort, [z, #42]
        lsr     d, d, #8
        strb    dshort, [z, #43]
        lsr     d, d, #8
        strb    dshort, [z, #44]
        lsr     d, d, #8
        strb    dshort, [z, #45]
        lsr     d, d, #8
        strb    dshort, [z, #46]
        lsr     d, d, #8
        strb    dshort, [z, #47]

// word 6

        ldr     d, [x, #48]
        strb    dshort, [z, #48]
        lsr     d, d, #8
        strb    dshort, [z, #49]
        lsr     d, d, #8
        strb    dshort, [z, #50]
        lsr     d, d, #8
        strb    dshort, [z, #51]
        lsr     d, d, #8
        strb    dshort, [z, #52]
        lsr     d, d, #8
        strb    dshort, [z, #53]
        lsr     d, d, #8
        strb    dshort, [z, #54]
        lsr     d, d, #8
        strb    dshort, [z, #55]

// word 7

        ldr     d, [x, #56]
        strb    dshort, [z, #56]
        lsr     d, d, #8
        strb    dshort, [z, #57]
        lsr     d, d, #8
        strb    dshort, [z, #58]
        lsr     d, d, #8
        strb    dshort, [z, #59]
        lsr     d, d, #8
        strb    dshort, [z, #60]
        lsr     d, d, #8
        strb    dshort, [z, #61]
        lsr     d, d, #8
        strb    dshort, [z, #62]
        lsr     d, d, #8
        strb    dshort, [z, #63]

// word 8

        ldr     d, [x, #64]
        strb    dshort, [z, #64]
        lsr     d, d, #8
        strb    dshort, [z, #65]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tolebytes_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
