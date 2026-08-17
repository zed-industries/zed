// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert 4-digit (256-bit) bignum to/from big-endian form
// Input x[4]; output z[4]
//
//    extern void bignum_bigendian_4(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// The same function is given two other prototypes whose names reflect the
// treatment of one or other argument as a byte array rather than word array:
//
//    extern void bignum_frombebytes_4(uint64_t z[static 4],
//                                     const uint8_t x[static 32]);
//
//    extern void bignum_tobebytes_4(uint8_t z[static 32],
//                                   const uint64_t x[static 4]);
//
// The implementation works by loading in bytes and storing in words (i.e.
// stylistically it is "frombebytes"); in the more common little-endian
// usage of ARM, this is just byte reversal.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_bigendian_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_bigendian_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_bigendian_4)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_frombebytes_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_frombebytes_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_frombebytes_4)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tobebytes_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tobebytes_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tobebytes_4)

        .text
        .balign 4

#define z x0
#define x x1

#define d x2
#define dshort w2
#define a x3
#define c x4

// The reads and writes are organized in mirror-image pairs (0-3 and 1-2)
// to allow x and z to point to the same buffer without using more
// intermediate registers.

S2N_BN_SYMBOL(bignum_bigendian_4):
S2N_BN_SYMBOL(bignum_frombebytes_4):
S2N_BN_SYMBOL(bignum_tobebytes_4):
        CFI_START

// 0 and 3 words

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
        str     c, [z]

// 1 and 2 words

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

        ldrb    dshort, [x, #23]
        extr    c, d, xzr, #8
        ldrb    dshort, [x, #22]
        extr    c, d, c, #8
        ldrb    dshort, [x, #21]
        extr    c, d, c, #8
        ldrb    dshort, [x, #20]
        extr    c, d, c, #8
        ldrb    dshort, [x, #19]
        extr    c, d, c, #8
        ldrb    dshort, [x, #18]
        extr    c, d, c, #8
        ldrb    dshort, [x, #17]
        extr    c, d, c, #8
        ldrb    dshort, [x, #16]
        extr    c, d, c, #8

        str     a, [z, #16]
        str     c, [z, #8]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_bigendian_4)
S2N_BN_SIZE_DIRECTIVE(bignum_frombebytes_4)
S2N_BN_SIZE_DIRECTIVE(bignum_tobebytes_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
