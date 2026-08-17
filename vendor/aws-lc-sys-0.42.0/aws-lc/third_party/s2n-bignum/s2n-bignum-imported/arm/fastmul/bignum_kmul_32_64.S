// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[32], y[32]; output z[64]; temporary buffer t[>=96]
//
//    extern void bignum_kmul_32_64(uint64_t z[static 64],
//                                  const uint64_t x[static 32],
//                                  const uint64_t y[static 32],
//                                  uint64_t t[static 96]);
//
// This is a Karatsuba-style function multiplying half-sized results
// internally and using temporary buffer t for intermediate results.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y, X3 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_kmul_32_64)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_kmul_32_64)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_kmul_32_64)

        .text
        .balign 4

#define K 16
#define L 8 // this is (K/2)

#define z x19
#define x x20
#define y x21
#define t x22

#define c x16

S2N_BN_SYMBOL(bignum_kmul_32_64):
        CFI_START

// Save extra registers and return address, store parameters safely

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)

        mov     z, x0
        mov     x, x1
        mov     y, x2
        mov     t, x3

// Compute L = x_lo * y_lo in bottom half of buffer (size 16 x 16 -> 32)

        CFI_BL(Lbignum_kmul_32_64_local_kmul_16_32)

// Compute H = x_hi * y_hi in top half of buffer (size 16 x 16 -> 32)

        add     x0, z, #16*K
        add     x1, x, #8*K
        add     x2, y, #8*K
        mov     x3, t
        CFI_BL(Lbignum_kmul_32_64_local_kmul_16_32)

// Compute absolute difference [t..] = |x_lo - x_hi|
// and the sign x = sgn(x_lo - x_hi) as a bitmask (all 1s for negative)
// Note that we overwrite the pointer x itself with this sign,
// which is safe since we no longer need it.

        ldp     x0, x1, [x, #128]
        ldp     x16, x17, [x]
        subs    x0, x0, x16
        sbcs    x1, x1, x17

        ldp     x2, x3, [x, #144]
        ldp     x16, x17, [x, #16]
        sbcs    x2, x2, x16
        sbcs    x3, x3, x17

        ldp     x4, x5, [x, #160]
        ldp     x16, x17, [x, #32]
        sbcs    x4, x4, x16
        sbcs    x5, x5, x17

        ldp     x6, x7, [x, #176]
        ldp     x16, x17, [x, #48]
        sbcs    x6, x6, x16
        sbcs    x7, x7, x17

        ldp     x8, x9, [x, #192]
        ldp     x16, x17, [x, #64]
        sbcs    x8, x8, x16
        sbcs    x9, x9, x17

        ldp     x10, x11, [x, #208]
        ldp     x16, x17, [x, #80]
        sbcs    x10, x10, x16
        sbcs    x11, x11, x17

        ldp     x12, x13, [x, #224]
        ldp     x16, x17, [x, #96]
        sbcs    x12, x12, x16
        sbcs    x13, x13, x17

        ldp     x14, x15, [x, #240]
        ldp     x16, x17, [x, #112]
        sbcs    x14, x14, x16
        sbcs    x15, x15, x17

        sbc     x, xzr, xzr

        adds    xzr, x, x

        eor     x0, x0, x
        adcs    x0, x0, xzr
        eor     x1, x1, x
        adcs    x1, x1, xzr
        stp     x0, x1, [t]

        eor     x2, x2, x
        adcs    x2, x2, xzr
        eor     x3, x3, x
        adcs    x3, x3, xzr
        stp     x2, x3, [t, #16]

        eor     x4, x4, x
        adcs    x4, x4, xzr
        eor     x5, x5, x
        adcs    x5, x5, xzr
        stp     x4, x5, [t, #32]

        eor     x6, x6, x
        adcs    x6, x6, xzr
        eor     x7, x7, x
        adcs    x7, x7, xzr
        stp     x6, x7, [t, #48]

        eor     x8, x8, x
        adcs    x8, x8, xzr
        eor     x9, x9, x
        adcs    x9, x9, xzr
        stp     x8, x9, [t, #64]

        eor     x10, x10, x
        adcs    x10, x10, xzr
        eor     x11, x11, x
        adcs    x11, x11, xzr
        stp     x10, x11, [t, #80]

        eor     x12, x12, x
        adcs    x12, x12, xzr
        eor     x13, x13, x
        adcs    x13, x13, xzr
        stp     x12, x13, [t, #96]

        eor     x14, x14, x
        adcs    x14, x14, xzr
        eor     x15, x15, x
        adc     x15, x15, xzr
        stp     x14, x15, [t, #112]

// Compute the other absolute difference [t+8*K..] = |y_hi - y_lo|
// Collect the combined product sign bitmask (all 1s for negative) as
// y = sgn((x_lo - x_hi) * (y_hi - y_lo)), overwriting the y pointer.

        ldp     x0, x1, [y]
        ldp     x16, x17, [y, #128]
        subs    x0, x0, x16
        sbcs    x1, x1, x17

        ldp     x2, x3, [y, #16]
        ldp     x16, x17, [y, #144]
        sbcs    x2, x2, x16
        sbcs    x3, x3, x17

        ldp     x4, x5, [y, #32]
        ldp     x16, x17, [y, #160]
        sbcs    x4, x4, x16
        sbcs    x5, x5, x17

        ldp     x6, x7, [y, #48]
        ldp     x16, x17, [y, #176]
        sbcs    x6, x6, x16
        sbcs    x7, x7, x17

        ldp     x8, x9, [y, #64]
        ldp     x16, x17, [y, #192]
        sbcs    x8, x8, x16
        sbcs    x9, x9, x17

        ldp     x10, x11, [y, #80]
        ldp     x16, x17, [y, #208]
        sbcs    x10, x10, x16
        sbcs    x11, x11, x17

        ldp     x12, x13, [y, #96]
        ldp     x16, x17, [y, #224]
        sbcs    x12, x12, x16
        sbcs    x13, x13, x17

        ldp     x14, x15, [y, #112]
        ldp     x16, x17, [y, #240]
        sbcs    x14, x14, x16
        sbcs    x15, x15, x17

        sbc     y, xzr, xzr

        adds    xzr, y, y

        eor     x0, x0, y
        adcs    x0, x0, xzr
        eor     x1, x1, y
        adcs    x1, x1, xzr
        stp     x0, x1, [t, #128]

        eor     x2, x2, y
        adcs    x2, x2, xzr
        eor     x3, x3, y
        adcs    x3, x3, xzr
        stp     x2, x3, [t, #128+16]

        eor     x4, x4, y
        adcs    x4, x4, xzr
        eor     x5, x5, y
        adcs    x5, x5, xzr
        stp     x4, x5, [t, #128+32]

        eor     x6, x6, y
        adcs    x6, x6, xzr
        eor     x7, x7, y
        adcs    x7, x7, xzr
        stp     x6, x7, [t, #128+48]

        eor     x8, x8, y
        adcs    x8, x8, xzr
        eor     x9, x9, y
        adcs    x9, x9, xzr
        stp     x8, x9, [t, #128+64]

        eor     x10, x10, y
        adcs    x10, x10, xzr
        eor     x11, x11, y
        adcs    x11, x11, xzr
        stp     x10, x11, [t, #128+80]

        eor     x12, x12, y
        adcs    x12, x12, xzr
        eor     x13, x13, y
        adcs    x13, x13, xzr
        stp     x12, x13, [t, #128+96]

        eor     x14, x14, y
        adcs    x14, x14, xzr
        eor     x15, x15, y
        adc     x15, x15, xzr
        stp     x14, x15, [t, #128+112]

        eor     y, y, x

// Compute H' = H + L_top in place of H (it cannot overflow)

        ldp     x0, x1, [z, #16*16]
        ldp     x2, x3, [z, #16*L]
        adds    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*16]

        ldp     x0, x1, [z, #16*17]
        ldp     x2, x3, [z, #16*9]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*17]

        ldp     x0, x1, [z, #16*18]
        ldp     x2, x3, [z, #16*10]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*18]

        ldp     x0, x1, [z, #16*19]
        ldp     x2, x3, [z, #16*11]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*19]

        ldp     x0, x1, [z, #16*20]
        ldp     x2, x3, [z, #16*12]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*20]

        ldp     x0, x1, [z, #16*21]
        ldp     x2, x3, [z, #16*13]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*21]

        ldp     x0, x1, [z, #16*22]
        ldp     x2, x3, [z, #16*14]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*22]

        ldp     x0, x1, [z, #16*23]
        ldp     x2, x3, [z, #16*15]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*23]

        ldp     x0, x1, [z, #16*24]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*24]

        ldp     x0, x1, [z, #16*25]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*25]

        ldp     x0, x1, [z, #16*26]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*26]

        ldp     x0, x1, [z, #16*27]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*27]

        ldp     x0, x1, [z, #16*28]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*28]

        ldp     x0, x1, [z, #16*29]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*29]

        ldp     x0, x1, [z, #16*30]
        adcs    x0, x0, xzr
        adcs    x1, x1, xzr
        stp     x0, x1, [z, #16*30]

        ldp     x0, x1, [z, #16*31]
        adcs    x0, x0, xzr
        adc     x1, x1, xzr
        stp     x0, x1, [z, #16*31]

// Compute M = |x_lo - x_hi| * |y_hi - y_lo|, size 32

        add     x0, t, #16*K
        mov     x1, t
        add     x2, t, #8*K
        add     x3, t, #32*K
        CFI_BL(Lbignum_kmul_32_64_local_kmul_16_32)

// Add the interlocking H' and L_bot terms
// Intercept the carry at the 3k position and store it in x.
// Again, we no longer need the input x was pointing at.

        ldp     x0, x1, [z, #16*16]
        ldp     x2, x3, [z]
        adds    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*8]

        ldp     x0, x1, [z, #16*17]
        ldp     x2, x3, [z, #16*1]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*9]

        ldp     x0, x1, [z, #16*18]
        ldp     x2, x3, [z, #16*2]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*10]

        ldp     x0, x1, [z, #16*19]
        ldp     x2, x3, [z, #16*3]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*11]

        ldp     x0, x1, [z, #16*20]
        ldp     x2, x3, [z, #16*4]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*12]

        ldp     x0, x1, [z, #16*21]
        ldp     x2, x3, [z, #16*5]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*13]

        ldp     x0, x1, [z, #16*22]
        ldp     x2, x3, [z, #16*6]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*14]

        ldp     x0, x1, [z, #16*23]
        ldp     x2, x3, [z, #16*7]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*15]

        ldp     x0, x1, [z, #16*16]
        ldp     x2, x3, [z, #16*24]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*16]

        ldp     x0, x1, [z, #16*17]
        ldp     x2, x3, [z, #16*25]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*17]

        ldp     x0, x1, [z, #16*18]
        ldp     x2, x3, [z, #16*26]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*18]

        ldp     x0, x1, [z, #16*19]
        ldp     x2, x3, [z, #16*27]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*19]

        ldp     x0, x1, [z, #16*20]
        ldp     x2, x3, [z, #16*28]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*20]

        ldp     x0, x1, [z, #16*21]
        ldp     x2, x3, [z, #16*29]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*21]

        ldp     x0, x1, [z, #16*22]
        ldp     x2, x3, [z, #16*30]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*22]

        ldp     x0, x1, [z, #16*23]
        ldp     x2, x3, [z, #16*31]
        adcs    x0, x0, x2
        adcs    x1, x1, x3
        stp     x0, x1, [z, #16*23]

        cset      x, cs

// Add the sign-adjusted mid-term cross product M

        cmn     y, y

        ldp     x0, x1, [z, #128]
        ldp     x2, x3, [t, #128+128]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #128]

        ldp     x0, x1, [z, #144]
        ldp     x2, x3, [t, #128+144]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #144]

        ldp     x0, x1, [z, #160]
        ldp     x2, x3, [t, #128+160]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #160]

        ldp     x0, x1, [z, #176]
        ldp     x2, x3, [t, #128+176]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #176]

        ldp     x0, x1, [z, #192]
        ldp     x2, x3, [t, #128+192]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #192]

        ldp     x0, x1, [z, #208]
        ldp     x2, x3, [t, #128+208]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #208]

        ldp     x0, x1, [z, #224]
        ldp     x2, x3, [t, #128+224]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #224]

        ldp     x0, x1, [z, #240]
        ldp     x2, x3, [t, #128+240]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #240]

        ldp     x0, x1, [z, #256]
        ldp     x2, x3, [t, #128+256]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #256]

        ldp     x0, x1, [z, #272]
        ldp     x2, x3, [t, #128+272]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #272]

        ldp     x0, x1, [z, #288]
        ldp     x2, x3, [t, #128+288]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #288]

        ldp     x0, x1, [z, #304]
        ldp     x2, x3, [t, #128+304]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #304]

        ldp     x0, x1, [z, #320]
        ldp     x2, x3, [t, #128+320]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #320]

        ldp     x0, x1, [z, #336]
        ldp     x2, x3, [t, #128+336]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #336]

        ldp     x0, x1, [z, #352]
        ldp     x2, x3, [t, #128+352]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #352]

        ldp     x0, x1, [z, #368]
        ldp     x2, x3, [t, #128+368]
        eor     x2, x2, y
        adcs    x0, x0, x2
        eor     x3, x3, y
        adcs    x1, x1, x3
        stp     x0, x1, [z, #368]

// Get the next digits effectively resulting so far starting at 3k
// [...,c,c,c,c,x]

        adcs    x, y, x
        adc     c, y, xzr

// Now propagate through the top quarter of the result

        ldp     x0, x1, [z, #16*24]
        adds    x0, x0, x
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*24]

        ldp     x0, x1, [z, #16*25]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*25]

        ldp     x0, x1, [z, #16*26]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*26]

        ldp     x0, x1, [z, #16*27]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*27]

        ldp     x0, x1, [z, #16*28]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*28]

        ldp     x0, x1, [z, #16*29]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*29]

        ldp     x0, x1, [z, #16*30]
        adcs    x0, x0, c
        adcs    x1, x1, c
        stp     x0, x1, [z, #16*30]

        ldp     x0, x1, [z, #16*31]
        adcs    x0, x0, c
        adc     x1, x1, c
        stp     x0, x1, [z, #16*31]

// Restore and return

        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_kmul_32_64)

// Local copy of bignum_kmul_16_32, identical to main one except that it
// only preserves the key registers we need to be stable in the main code.
// This includes in turn a copy of bignum_mul_8_16.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_kmul_32_64_local_kmul_16_32)

Lbignum_kmul_32_64_local_kmul_16_32:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x30)
        mov     x25, x0
        mov     x26, x1
        mov     x27, x2
        mov     x28, x3
        CFI_BL(Lbignum_kmul_32_64_local_mul_8_16)
        ldp     x10, x11, [x26]
        ldp     x8, x9, [x26, #64]
        subs    x10, x10, x8
        sbcs    x11, x11, x9
        ldp     x12, x13, [x26, #16]
        ldp     x8, x9, [x26, #80]
        sbcs    x12, x12, x8
        sbcs    x13, x13, x9
        ldp     x14, x15, [x26, #32]
        ldp     x8, x9, [x26, #96]
        sbcs    x14, x14, x8
        sbcs    x15, x15, x9
        ldp     x16, x17, [x26, #48]
        ldp     x8, x9, [x26, #112]
        sbcs    x16, x16, x8
        sbcs    x17, x17, x9
        csetm   x29, cc
        cmn     x29, x29
        eor     x10, x10, x29
        adcs    x10, x10, xzr
        eor     x11, x11, x29
        adcs    x11, x11, xzr
        stp     x10, x11, [x28]
        eor     x12, x12, x29
        adcs    x12, x12, xzr
        eor     x13, x13, x29
        adcs    x13, x13, xzr
        stp     x12, x13, [x28, #16]
        eor     x14, x14, x29
        adcs    x14, x14, xzr
        eor     x15, x15, x29
        adcs    x15, x15, xzr
        stp     x14, x15, [x28, #32]
        eor     x16, x16, x29
        adcs    x16, x16, xzr
        eor     x17, x17, x29
        adcs    x17, x17, xzr
        stp     x16, x17, [x28, #48]
        add     x0, x25, #0x80
        add     x1, x26, #0x40
        add     x2, x27, #0x40
        CFI_BL(Lbignum_kmul_32_64_local_mul_8_16)
        ldp     x10, x11, [x27]
        ldp     x8, x9, [x27, #64]
        subs    x10, x8, x10
        sbcs    x11, x9, x11
        ldp     x12, x13, [x27, #16]
        ldp     x8, x9, [x27, #80]
        sbcs    x12, x8, x12
        sbcs    x13, x9, x13
        ldp     x14, x15, [x27, #32]
        ldp     x8, x9, [x27, #96]
        sbcs    x14, x8, x14
        sbcs    x15, x9, x15
        ldp     x16, x17, [x27, #48]
        ldp     x8, x9, [x27, #112]
        sbcs    x16, x8, x16
        sbcs    x17, x9, x17
        csetm   x19, cc
        cmn     x19, x19
        eor     x10, x10, x19
        adcs    x10, x10, xzr
        eor     x11, x11, x19
        adcs    x11, x11, xzr
        stp     x10, x11, [x28, #64]
        eor     x12, x12, x19
        adcs    x12, x12, xzr
        eor     x13, x13, x19
        adcs    x13, x13, xzr
        stp     x12, x13, [x28, #80]
        eor     x14, x14, x19
        adcs    x14, x14, xzr
        eor     x15, x15, x19
        adcs    x15, x15, xzr
        stp     x14, x15, [x28, #96]
        eor     x16, x16, x19
        adcs    x16, x16, xzr
        eor     x17, x17, x19
        adcs    x17, x17, xzr
        stp     x16, x17, [x28, #112]
        eor     x29, x29, x19
        ldp     x10, x11, [x25, #128]
        ldp     x12, x13, [x25, #64]
        adds    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x25, #128]
        ldp     x10, x11, [x25, #144]
        ldp     x12, x13, [x25, #80]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x25, #144]
        ldp     x10, x11, [x25, #160]
        ldp     x12, x13, [x25, #96]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x25, #160]
        ldp     x10, x11, [x25, #176]
        ldp     x12, x13, [x25, #112]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x25, #176]
        ldp     x10, x11, [x25, #192]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x25, #192]
        ldp     x10, x11, [x25, #208]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x25, #208]
        ldp     x10, x11, [x25, #224]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x25, #224]
        ldp     x10, x11, [x25, #240]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x25, #240]
        add     x0, x28, #0x80
        mov     x1, x28
        add     x2, x28, #0x40
        CFI_BL(Lbignum_kmul_32_64_local_mul_8_16)
        ldp     x0, x1, [x25]
        ldp     x16, x17, [x25, #128]
        adds    x0, x0, x16
        adcs    x1, x1, x17
        ldp     x2, x3, [x25, #16]
        ldp     x16, x17, [x25, #144]
        adcs    x2, x2, x16
        adcs    x3, x3, x17
        ldp     x4, x5, [x25, #32]
        ldp     x16, x17, [x25, #160]
        adcs    x4, x4, x16
        adcs    x5, x5, x17
        ldp     x6, x7, [x25, #48]
        ldp     x16, x17, [x25, #176]
        adcs    x6, x6, x16
        adcs    x7, x7, x17
        ldp     x8, x9, [x25, #128]
        ldp     x16, x17, [x25, #192]
        adcs    x8, x8, x16
        adcs    x9, x9, x17
        ldp     x10, x11, [x25, #144]
        ldp     x16, x17, [x25, #208]
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        ldp     x12, x13, [x25, #160]
        ldp     x16, x17, [x25, #224]
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        ldp     x14, x15, [x25, #176]
        ldp     x16, x17, [x25, #240]
        adcs    x14, x14, x16
        adcs    x15, x15, x17
        cset    x26, cs
        cmn     x29, x29
        ldp     x16, x17, [x28, #128]
        eor     x16, x16, x29
        adcs    x0, x0, x16
        eor     x17, x17, x29
        adcs    x1, x1, x17
        stp     x0, x1, [x25, #64]
        ldp     x16, x17, [x28, #144]
        eor     x16, x16, x29
        adcs    x2, x2, x16
        eor     x17, x17, x29
        adcs    x3, x3, x17
        stp     x2, x3, [x25, #80]
        ldp     x16, x17, [x28, #160]
        eor     x16, x16, x29
        adcs    x4, x4, x16
        eor     x17, x17, x29
        adcs    x5, x5, x17
        stp     x4, x5, [x25, #96]
        ldp     x16, x17, [x28, #176]
        eor     x16, x16, x29
        adcs    x6, x6, x16
        eor     x17, x17, x29
        adcs    x7, x7, x17
        stp     x6, x7, [x25, #112]
        ldp     x16, x17, [x28, #192]
        eor     x16, x16, x29
        adcs    x8, x8, x16
        eor     x17, x17, x29
        adcs    x9, x9, x17
        stp     x8, x9, [x25, #128]
        ldp     x16, x17, [x28, #208]
        eor     x16, x16, x29
        adcs    x10, x10, x16
        eor     x17, x17, x29
        adcs    x11, x11, x17
        stp     x10, x11, [x25, #144]
        ldp     x16, x17, [x28, #224]
        eor     x16, x16, x29
        adcs    x12, x12, x16
        eor     x17, x17, x29
        adcs    x13, x13, x17
        stp     x12, x13, [x25, #160]
        ldp     x16, x17, [x28, #240]
        eor     x16, x16, x29
        adcs    x14, x14, x16
        eor     x17, x17, x29
        adcs    x15, x15, x17
        stp     x14, x15, [x25, #176]
        adcs    x27, x29, x26
        adc     x28, x29, xzr
        ldp     x10, x11, [x25, #192]
        adds    x10, x10, x27
        adcs    x11, x11, x28
        stp     x10, x11, [x25, #192]
        ldp     x10, x11, [x25, #208]
        adcs    x10, x10, x28
        adcs    x11, x11, x28
        stp     x10, x11, [x25, #208]
        ldp     x10, x11, [x25, #224]
        adcs    x10, x10, x28
        adcs    x11, x11, x28
        stp     x10, x11, [x25, #224]
        ldp     x10, x11, [x25, #240]
        adcs    x10, x10, x28
        adcs    x11, x11, x28
        stp     x10, x11, [x25, #240]
        CFI_POP2(x23,x30)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_kmul_32_64_local_kmul_16_32)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_kmul_32_64_local_mul_8_16)

Lbignum_kmul_32_64_local_mul_8_16:
        CFI_START
        ldp x3, x4, [x1]
        ldr q0, [x1]
        ldp x7, x8, [x2]
        ldr q1, [x2]
        ldp x5, x6, [x1, #16]
        ldr q2, [x1, #16]
        ldp x9, x10, [x2, #16]
        ldr q3, [x2, #16]
        uzp1 v4.4s, v1.4s, v0.4s
        rev64 v1.4s, v1.4s
        uzp1 v5.4s, v0.4s, v0.4s
        mul v0.4s, v1.4s, v0.4s
        uaddlp v0.2d, v0.4s
        shl v0.2d, v0.2d, #32
        umlal v0.2d, v5.2s, v4.2s
        mov x11, v0.d[0]
        mov x15, v0.d[1]
        uzp1 v0.4s, v3.4s, v2.4s
        rev64 v1.4s, v3.4s
        uzp1 v3.4s, v2.4s, v2.4s
        mul v1.4s, v1.4s, v2.4s
        uaddlp v1.2d, v1.4s
        shl v1.2d, v1.2d, #32
        umlal v1.2d, v3.2s, v0.2s
        mov x16, v1.d[0]
        mov x17, v1.d[1]
        ldr q0, [x1, #32]
        ldr q1, [x2, #32]
        ldr q2, [x1, #48]
        ldr q3, [x2, #48]
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        uzp1 v4.4s, v1.4s, v0.4s
        rev64 v1.4s, v1.4s
        uzp1 v5.4s, v0.4s, v0.4s
        mul v0.4s, v1.4s, v0.4s
        uaddlp v0.2d, v0.4s
        shl v0.2d, v0.2d, #32
        umlal v0.2d, v5.2s, v4.2s
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x3, x4, [x1, #32]
        stp x11, x12, [x0]
        ldp x7, x8, [x2, #32]
        stp x13, x14, [x0, #16]
        ldp x5, x6, [x1, #48]
        stp x15, x16, [x0, #32]
        ldp x9, x10, [x2, #48]
        stp x17, x19, [x0, #48]
        mov x11, v0.d[0]
        mov x15, v0.d[1]
        uzp1 v0.4s, v3.4s, v2.4s
        rev64 v1.4s, v3.4s
        uzp1 v3.4s, v2.4s, v2.4s
        mul v1.4s, v1.4s, v2.4s
        uaddlp v1.2d, v1.4s
        shl v1.2d, v1.2d, #32
        umlal v1.2d, v3.2s, v0.2s
        mov x16, v1.d[0]
        mov x17, v1.d[1]
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        ldp x22, x21, [x0, #32]
        adds x11, x11, x22
        adcs x12, x12, x21
        ldp x22, x21, [x0, #48]
        adcs x13, x13, x22
        adcs x14, x14, x21
        adcs x15, x15, xzr
        adcs x16, x16, xzr
        adcs x17, x17, xzr
        adc x19, x19, xzr
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x22, x21, [x1]
        subs x3, x3, x22
        sbcs x4, x4, x21
        ldp x22, x21, [x1, #16]
        sbcs x5, x5, x22
        sbcs x6, x6, x21
        csetm x24, cc
        stp x11, x12, [x0, #64]
        ldp x22, x21, [x2]
        subs x7, x22, x7
        sbcs x8, x21, x8
        ldp x22, x21, [x2, #16]
        sbcs x9, x22, x9
        sbcs x10, x21, x10
        csetm x1, cc
        stp x13, x14, [x0, #80]
        eor x3, x3, x24
        subs x3, x3, x24
        eor x4, x4, x24
        sbcs x4, x4, x24
        eor x5, x5, x24
        sbcs x5, x5, x24
        eor x6, x6, x24
        sbc x6, x6, x24
        stp x15, x16, [x0, #96]
        eor x7, x7, x1
        subs x7, x7, x1
        eor x8, x8, x1
        sbcs x8, x8, x1
        eor x9, x9, x1
        sbcs x9, x9, x1
        eor x10, x10, x1
        sbc x10, x10, x1
        stp x17, x19, [x0, #112]
        eor x1, x1, x24
        mul x11, x3, x7
        mul x15, x4, x8
        mul x16, x5, x9
        mul x17, x6, x10
        umulh x19, x3, x7
        adds x15, x15, x19
        umulh x19, x4, x8
        adcs x16, x16, x19
        umulh x19, x5, x9
        adcs x17, x17, x19
        umulh x19, x6, x10
        adc x19, x19, xzr
        adds x12, x15, x11
        adcs x15, x16, x15
        adcs x16, x17, x16
        adcs x17, x19, x17
        adc x19, xzr, x19
        adds x13, x15, x11
        adcs x14, x16, x12
        adcs x15, x17, x15
        adcs x16, x19, x16
        adcs x17, xzr, x17
        adc x19, xzr, x19
        subs x24, x5, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x9
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x16, x16, x22
        eor x21, x21, x20
        adcs x17, x17, x21
        adc x19, x19, x20
        subs x24, x3, x4
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x8, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x12, x12, x22
        eor x21, x21, x20
        adcs x13, x13, x21
        adcs x14, x14, x20
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x15, x15, x22
        eor x21, x21, x20
        adcs x16, x16, x21
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x13, x13, x22
        eor x21, x21, x20
        adcs x14, x14, x21
        adcs x15, x15, x20
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x3, x6
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x10, x7
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        subs x24, x4, x5
        cneg x24, x24, cc
        csetm x20, cc
        subs x21, x9, x8
        cneg x21, x21, cc
        mul x22, x24, x21
        umulh x21, x24, x21
        cinv x20, x20, cc
        cmn x20, #0x1
        eor x22, x22, x20
        adcs x14, x14, x22
        eor x21, x21, x20
        adcs x15, x15, x21
        adcs x16, x16, x20
        adcs x17, x17, x20
        adc x19, x19, x20
        ldp x3, x4, [x0]
        ldp x7, x8, [x0, #64]
        adds x3, x3, x7
        adcs x4, x4, x8
        ldp x5, x6, [x0, #16]
        ldp x9, x10, [x0, #80]
        adcs x5, x5, x9
        adcs x6, x6, x10
        ldp x20, x21, [x0, #96]
        adcs x7, x7, x20
        adcs x8, x8, x21
        ldp x22, x23, [x0, #112]
        adcs x9, x9, x22
        adcs x10, x10, x23
        adcs x24, x1, xzr
        adc x2, x1, xzr
        cmn x1, #0x1
        eor x11, x11, x1
        adcs x3, x11, x3
        eor x12, x12, x1
        adcs x4, x12, x4
        eor x13, x13, x1
        adcs x5, x13, x5
        eor x14, x14, x1
        adcs x6, x14, x6
        eor x15, x15, x1
        adcs x7, x15, x7
        eor x16, x16, x1
        adcs x8, x16, x8
        eor x17, x17, x1
        adcs x9, x17, x9
        eor x19, x19, x1
        adcs x10, x19, x10
        adcs x20, x20, x24
        adcs x21, x21, x2
        adcs x22, x22, x2
        adc x23, x23, x2
        stp x3, x4, [x0, #32]
        stp x5, x6, [x0, #48]
        stp x7, x8, [x0, #64]
        stp x9, x10, [x0, #80]
        stp x20, x21, [x0, #96]
        stp x22, x23, [x0, #112]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_kmul_32_64_local_mul_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
