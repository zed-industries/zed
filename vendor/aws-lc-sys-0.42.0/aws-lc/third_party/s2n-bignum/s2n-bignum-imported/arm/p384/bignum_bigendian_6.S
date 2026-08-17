// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert 6-digit (384-bit) bignum to/from big-endian form
// Input x[6]; output z[6]
//
//    extern void bignum_bigendian_6(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// The same function is given two other prototypes whose names reflect the
// treatment of one or other argument as a byte array rather than word array:
//
//    extern void bignum_frombebytes_6(uint64_t z[static 6],
//                                     const uint8_t x[static 48]);
//
//    extern void bignum_tobebytes_6(uint8_t z[static 48],
//                                   const uint64_t x[static 6]);
//
// The implementation works by loading in bytes and storing in words (i.e.
// stylistically it is "frombebytes"); in the more common little-endian
// usage of ARM, this is just byte reversal.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_bigendian_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_bigendian_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_bigendian_6)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_frombebytes_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_frombebytes_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_frombebytes_6)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tobebytes_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tobebytes_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tobebytes_6)

        .text
        .balign 4

#define z x0
#define x x1

#define d x2
#define dshort w2
#define a x3
#define c x4

// The reads and writes are organized in mirror-image pairs (0-5, 1-4, 2-3)
// to allow x and z to point to the same buffer without using more
// intermediate registers.

S2N_BN_SYMBOL(bignum_bigendian_6):
S2N_BN_SYMBOL(bignum_frombebytes_6):
S2N_BN_SYMBOL(bignum_tobebytes_6):
        CFI_START

// 0 and 5 words

        ldrb    dshort, [x, #7]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #6]
        extr    a, d, a, #8
        ldrb    dshort, [x, #5]
        extr    a, d, a, #8
        ldrb    dshort, [x, #4]
        extr    a, d, a, #8
        ldrb    dshort, [x, #3]
        extr    a, d, a, #8
        ldrb    dshort, [x, #2]
        extr    a, d, a, #8
        ldrb    dshort, [x, #1]
        extr    a, d, a, #8
        ldrb    dshort, [x]
        extr    a, d, a, #8

        ldrb    dshort, [x, #47]
        extr    c, d, xzr, #8
        ldrb    dshort, [x, #46]
        extr    c, d, c, #8
        ldrb    dshort, [x, #45]
        extr    c, d, c, #8
        ldrb    dshort, [x, #44]
        extr    c, d, c, #8
        ldrb    dshort, [x, #43]
        extr    c, d, c, #8
        ldrb    dshort, [x, #42]
        extr    c, d, c, #8
        ldrb    dshort, [x, #41]
        extr    c, d, c, #8
        ldrb    dshort, [x, #40]
        extr    c, d, c, #8

        str     a, [z, #40]
        str     c, [z]

// 1 and 4 words

        ldrb    dshort, [x, #15]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #14]
        extr    a, d, a, #8
        ldrb    dshort, [x, #13]
        extr    a, d, a, #8
        ldrb    dshort, [x, #12]
        extr    a, d, a, #8
        ldrb    dshort, [x, #11]
        extr    a, d, a, #8
        ldrb    dshort, [x, #10]
        extr    a, d, a, #8
        ldrb    dshort, [x, #9]
        extr    a, d, a, #8
        ldrb    dshort, [x, #8]
        extr    a, d, a, #8

        ldrb    dshort, [x, #39]
        extr    c, d, xzr, #8
        ldrb    dshort, [x, #38]
        extr    c, d, c, #8
        ldrb    dshort, [x, #37]
        extr    c, d, c, #8
        ldrb    dshort, [x, #36]
        extr    c, d, c, #8
        ldrb    dshort, [x, #35]
        extr    c, d, c, #8
        ldrb    dshort, [x, #34]
        extr    c, d, c, #8
        ldrb    dshort, [x, #33]
        extr    c, d, c, #8
        ldrb    dshort, [x, #32]
        extr    c, d, c, #8

        str     a, [z, #32]
        str     c, [z, #8]

// 2 and 3 words

        ldrb    dshort, [x, #23]
        extr    a, d, xzr, #8
        ldrb    dshort, [x, #22]
        extr    a, d, a, #8
        ldrb    dshort, [x, #21]
        extr    a, d, a, #8
        ldrb    dshort, [x, #20]
        extr    a, d, a, #8
        ldrb    dshort, [x, #19]
        extr    a, d, a, #8
        ldrb    dshort, [x, #18]
        extr    a, d, a, #8
        ldrb    dshort, [x, #17]
        extr    a, d, a, #8
        ldrb    dshort, [x, #16]
        extr    a, d, a, #8

        ldrb    dshort, [x, #31]
        extr    c, d, xzr, #8
        ldrb    dshort, [x, #30]
        extr    c, d, c, #8
        ldrb    dshort, [x, #29]
        extr    c, d, c, #8
        ldrb    dshort, [x, #28]
        extr    c, d, c, #8
        ldrb    dshort, [x, #27]
        extr    c, d, c, #8
        ldrb    dshort, [x, #26]
        extr    c, d, c, #8
        ldrb    dshort, [x, #25]
        extr    c, d, c, #8
        ldrb    dshort, [x, #24]
        extr    c, d, c, #8

        str     a, [z, #24]
        str     c, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_bigendian_6)
S2N_BN_SIZE_DIRECTIVE(bignum_frombebytes_6)
S2N_BN_SIZE_DIRECTIVE(bignum_tobebytes_6)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
