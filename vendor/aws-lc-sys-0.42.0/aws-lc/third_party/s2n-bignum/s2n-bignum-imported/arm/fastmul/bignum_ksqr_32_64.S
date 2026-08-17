// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[32]; output z[64]; temporary buffer t[>=72]
//
//    extern void bignum_ksqr_32_64(uint64_t z[static 64],
//                                  const uint64_t x[static 32],
//                                  uint64_t t[static 72]);
//
// This is a Karatsuba-style function squaring half-sized results
// and using temporary buffer t for intermediate results.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ksqr_32_64)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ksqr_32_64)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ksqr_32_64)

        .text
        .balign 4

#define K 16
#define L 8 // (K/2)

#define z x19
#define x x20
#define t x21

#define c x16


S2N_BN_SYMBOL(bignum_ksqr_32_64):
        CFI_START

// Save extra registers and return address, store parameters safely

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x30)

        mov     z, x0
        mov     x, x1
        mov     t, x2

// Compute L = x_lo * y_lo in bottom half of buffer (size 16 x 16 -> 32)

        CFI_BL(Lbignum_ksqr_32_64_local_ksqr_16_32)

// Compute H = x_hi * y_hi in top half of buffer (size 16 x 16 -> 32)

        add     x0, z, #16*K
        add     x1, x, #8*K
        mov     x2, t
        CFI_BL(Lbignum_ksqr_32_64_local_ksqr_16_32)

// Compute H' = H + L_top in place of H (it cannot overflow)

        ldp     x0, x1, [z, #16*16]
        ldp     x2, x3, [z, #16*8]
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

// Compute absolute difference [t..] = |x_lo - x_hi|

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

        sbc     c, xzr, xzr

        adds    xzr, c, c

        eor     x0, x0, c
        adcs    x0, x0, xzr
        eor     x1, x1, c
        adcs    x1, x1, xzr
        stp     x0, x1, [t]

        eor     x2, x2, c
        adcs    x2, x2, xzr
        eor     x3, x3, c
        adcs    x3, x3, xzr
        stp     x2, x3, [t, #16]

        eor     x4, x4, c
        adcs    x4, x4, xzr
        eor     x5, x5, c
        adcs    x5, x5, xzr
        stp     x4, x5, [t, #32]

        eor     x6, x6, c
        adcs    x6, x6, xzr
        eor     x7, x7, c
        adcs    x7, x7, xzr
        stp     x6, x7, [t, #48]

        eor     x8, x8, c
        adcs    x8, x8, xzr
        eor     x9, x9, c
        adcs    x9, x9, xzr
        stp     x8, x9, [t, #64]

        eor     x10, x10, c
        adcs    x10, x10, xzr
        eor     x11, x11, c
        adcs    x11, x11, xzr
        stp     x10, x11, [t, #80]

        eor     x12, x12, c
        adcs    x12, x12, xzr
        eor     x13, x13, c
        adcs    x13, x13, xzr
        stp     x12, x13, [t, #96]

        eor     x14, x14, c
        adcs    x14, x14, xzr
        eor     x15, x15, c
        adc     x15, x15, xzr
        stp     x14, x15, [t, #112]

// Compute M = |x_lo - x_hi|^2, size 32

        add     x0, t, #8*K
        mov     x1, t
        add     x2, t, #24*K
        CFI_BL(Lbignum_ksqr_32_64_local_ksqr_16_32)

// Add the interlocking H' and L_bot terms
// Intercept the carry at the 3k position and store it in x.
// (Note that we no longer need the input x was pointing at.)

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

// Subtract the mid-term cross product M

        ldp     x0, x1, [z, #16*L]
        ldp     x2, x3, [t, #16*L]
        subs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*L]

        ldp     x0, x1, [z, #16*9]
        ldp     x2, x3, [t, #16*9]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*9]

        ldp     x0, x1, [z, #16*10]
        ldp     x2, x3, [t, #16*10]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*10]

        ldp     x0, x1, [z, #16*11]
        ldp     x2, x3, [t, #16*11]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*11]

        ldp     x0, x1, [z, #16*12]
        ldp     x2, x3, [t, #16*12]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*12]

        ldp     x0, x1, [z, #16*13]
        ldp     x2, x3, [t, #16*13]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*13]

        ldp     x0, x1, [z, #16*14]
        ldp     x2, x3, [t, #16*14]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*14]

        ldp     x0, x1, [z, #16*15]
        ldp     x2, x3, [t, #16*15]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*15]

        ldp     x0, x1, [z, #16*16]
        ldp     x2, x3, [t, #16*16]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*16]

        ldp     x0, x1, [z, #16*17]
        ldp     x2, x3, [t, #16*17]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*17]

        ldp     x0, x1, [z, #16*18]
        ldp     x2, x3, [t, #16*18]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*18]

        ldp     x0, x1, [z, #16*19]
        ldp     x2, x3, [t, #16*19]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*19]

        ldp     x0, x1, [z, #16*20]
        ldp     x2, x3, [t, #16*20]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*20]

        ldp     x0, x1, [z, #16*21]
        ldp     x2, x3, [t, #16*21]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*21]

        ldp     x0, x1, [z, #16*22]
        ldp     x2, x3, [t, #16*22]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*22]

        ldp     x0, x1, [z, #16*23]
        ldp     x2, x3, [t, #16*23]
        sbcs    x0, x0, x2
        sbcs    x1, x1, x3
        stp     x0, x1, [z, #16*23]

// Get the next digits effectively resulting so far starting at 3k
// [...,c,c,c,c,x]

        sbcs    x, x, xzr
        csetm   c, cc

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

// Restore

        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_ksqr_32_64)

// Local copy of bignum_ksqr_16_32, identical to main one.
// This includes in turn a copy of bignum_sqr_8_16.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_ksqr_32_64_local_ksqr_16_32)

Lbignum_ksqr_32_64_local_ksqr_16_32:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x30)
        mov     x23, x0
        mov     x24, x1
        mov     x25, x2
        CFI_BL(Lbignum_ksqr_32_64_local_sqr_8_16)
        ldp     x10, x11, [x24]
        ldp     x8, x9, [x24, #64]
        subs    x10, x10, x8
        sbcs    x11, x11, x9
        ldp     x12, x13, [x24, #16]
        ldp     x8, x9, [x24, #80]
        sbcs    x12, x12, x8
        sbcs    x13, x13, x9
        ldp     x14, x15, [x24, #32]
        ldp     x8, x9, [x24, #96]
        sbcs    x14, x14, x8
        sbcs    x15, x15, x9
        ldp     x16, x17, [x24, #48]
        ldp     x8, x9, [x24, #112]
        sbcs    x16, x16, x8
        sbcs    x17, x17, x9
        csetm   x19, cc
        cmn     x19, x19
        eor     x10, x10, x19
        adcs    x10, x10, xzr
        eor     x11, x11, x19
        adcs    x11, x11, xzr
        stp     x10, x11, [x25]
        eor     x12, x12, x19
        adcs    x12, x12, xzr
        eor     x13, x13, x19
        adcs    x13, x13, xzr
        stp     x12, x13, [x25, #16]
        eor     x14, x14, x19
        adcs    x14, x14, xzr
        eor     x15, x15, x19
        adcs    x15, x15, xzr
        stp     x14, x15, [x25, #32]
        eor     x16, x16, x19
        adcs    x16, x16, xzr
        eor     x17, x17, x19
        adcs    x17, x17, xzr
        stp     x16, x17, [x25, #48]
        add     x0, x23, #0x80
        add     x1, x24, #0x40
        CFI_BL(Lbignum_ksqr_32_64_local_sqr_8_16)
        ldp     x10, x11, [x23, #128]
        ldp     x12, x13, [x23, #64]
        adds    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x23, #128]
        ldp     x10, x11, [x23, #144]
        ldp     x12, x13, [x23, #80]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x23, #144]
        ldp     x10, x11, [x23, #160]
        ldp     x12, x13, [x23, #96]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x23, #160]
        ldp     x10, x11, [x23, #176]
        ldp     x12, x13, [x23, #112]
        adcs    x10, x10, x12
        adcs    x11, x11, x13
        stp     x10, x11, [x23, #176]
        ldp     x10, x11, [x23, #192]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x23, #192]
        ldp     x10, x11, [x23, #208]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x23, #208]
        ldp     x10, x11, [x23, #224]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x23, #224]
        ldp     x10, x11, [x23, #240]
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        stp     x10, x11, [x23, #240]
        add     x0, x25, #0x40
        mov     x1, x25
        CFI_BL(Lbignum_ksqr_32_64_local_sqr_8_16)
        ldp     x0, x1, [x23]
        ldp     x16, x17, [x23, #128]
        adds    x0, x0, x16
        adcs    x1, x1, x17
        ldp     x2, x3, [x23, #16]
        ldp     x16, x17, [x23, #144]
        adcs    x2, x2, x16
        adcs    x3, x3, x17
        ldp     x4, x5, [x23, #32]
        ldp     x16, x17, [x23, #160]
        adcs    x4, x4, x16
        adcs    x5, x5, x17
        ldp     x6, x7, [x23, #48]
        ldp     x16, x17, [x23, #176]
        adcs    x6, x6, x16
        adcs    x7, x7, x17
        ldp     x8, x9, [x23, #128]
        ldp     x16, x17, [x23, #192]
        adcs    x8, x8, x16
        adcs    x9, x9, x17
        ldp     x10, x11, [x23, #144]
        ldp     x16, x17, [x23, #208]
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        ldp     x12, x13, [x23, #160]
        ldp     x16, x17, [x23, #224]
        adcs    x12, x12, x16
        adcs    x13, x13, x17
        ldp     x14, x15, [x23, #176]
        ldp     x16, x17, [x23, #240]
        adcs    x14, x14, x16
        adcs    x15, x15, x17
        cset    x24, cs
        ldp     x16, x17, [x25, #64]
        subs    x0, x0, x16
        sbcs    x1, x1, x17
        stp     x0, x1, [x23, #64]
        ldp     x16, x17, [x25, #80]
        sbcs    x2, x2, x16
        sbcs    x3, x3, x17
        stp     x2, x3, [x23, #80]
        ldp     x16, x17, [x25, #96]
        sbcs    x4, x4, x16
        sbcs    x5, x5, x17
        stp     x4, x5, [x23, #96]
        ldp     x16, x17, [x25, #112]
        sbcs    x6, x6, x16
        sbcs    x7, x7, x17
        stp     x6, x7, [x23, #112]
        ldp     x16, x17, [x25, #128]
        sbcs    x8, x8, x16
        sbcs    x9, x9, x17
        stp     x8, x9, [x23, #128]
        ldp     x16, x17, [x25, #144]
        sbcs    x10, x10, x16
        sbcs    x11, x11, x17
        stp     x10, x11, [x23, #144]
        ldp     x16, x17, [x25, #160]
        sbcs    x12, x12, x16
        sbcs    x13, x13, x17
        stp     x12, x13, [x23, #160]
        ldp     x16, x17, [x25, #176]
        sbcs    x14, x14, x16
        sbcs    x15, x15, x17
        stp     x14, x15, [x23, #176]
        sbcs    x24, x24, xzr
        csetm   x25, cc
        ldp     x10, x11, [x23, #192]
        adds    x10, x10, x24
        adcs    x11, x11, x25
        stp     x10, x11, [x23, #192]
        ldp     x10, x11, [x23, #208]
        adcs    x10, x10, x25
        adcs    x11, x11, x25
        stp     x10, x11, [x23, #208]
        ldp     x10, x11, [x23, #224]
        adcs    x10, x10, x25
        adcs    x11, x11, x25
        stp     x10, x11, [x23, #224]
        ldp     x10, x11, [x23, #240]
        adcs    x10, x10, x25
        adcs    x11, x11, x25
        stp     x10, x11, [x23, #240]
        CFI_POP2(x25,x30)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_ksqr_32_64_local_ksqr_16_32)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_ksqr_32_64_local_sqr_8_16)

Lbignum_ksqr_32_64_local_sqr_8_16:
        CFI_START
// Load registers.
        ldp     x2, x3, [x1]
ldr     q20, [x1]
        ldp     x4, x5, [x1, #16]
ldr     q21, [x1, #16]
        ldp     x6, x7, [x1, #32]
ldr     q22, [x1, #32]
        ldp     x8, x9, [x1, #48]
ldr     q23, [x1, #48]
movi    v30.2d, #0xffffffff

        mul     x17, x2, x4
        mul     x14, x3, x5

// Scalar+NEON: square the lower half with a near-clone of bignum_sqr_4_8
// NEON: prepare 64x64->128 squaring of two 64-bit ints (x2, x3)
ext     v1.16b, v20.16b, v20.16b, #8
        umulh   x20, x2, x4
shrn    v2.2s, v20.2d, #32
        subs    x21, x2, x3
zip1    v0.2s, v20.2s, v1.2s
        cneg    x21, x21, cc  // cc = lo, ul, last
umull   v5.2d, v2.2s, v2.2s
        csetm   x11, cc  // cc = lo, ul, last
umull   v6.2d, v2.2s, v0.2s
        subs    x12, x5, x4
umull   v3.2d, v0.2s, v0.2s
        cneg    x12, x12, cc  // cc = lo, ul, last
mov     v1.16b, v6.16b
        mul     x13, x21, x12
usra    v1.2d, v3.2d, #32
        umulh   x12, x21, x12
and     v4.16b, v1.16b, v30.16b
        cinv    x11, x11, cc  // cc = lo, ul, last
add     v4.2d, v4.2d, v6.2d
        eor     x13, x13, x11
usra    v5.2d, v4.2d, #32
        eor     x12, x12, x11
sli     v3.2d, v4.2d, #32
        adds    x19, x17, x20
usra    v5.2d, v1.2d, #32
        adc     x20, x20, xzr
  // NEON: prepare 64x64->128 squaring of two 64-bit ints (x4, x5)
  ext   v1.16b, v21.16b, v21.16b, #8
        umulh   x21, x3, x5
  shrn  v2.2s, v21.2d, #32
        adds    x19, x19, x14
  zip1  v0.2s, v21.2s, v1.2s
        adcs    x20, x20, x21
        adc     x21, x21, xzr
        adds    x20, x20, x14
        adc     x21, x21, xzr
        cmn     x11, #0x1
        adcs    x19, x19, x13
mov     x13, v3.d[1] // mul     x13, x3, x3
        adcs    x20, x20, x12
mov     x14, v5.d[1] // umulh   x14, x3, x3
        adc     x21, x21, x11
mov     x12, v3.d[0] // mul     x12, x2, x2
        adds    x17, x17, x17
mov     x11, v5.d[0] // umulh   x11, x2, x2
        adcs    x19, x19, x19
  umull v5.2d, v2.2s, v2.2s
        adcs    x20, x20, x20
  umull v6.2d, v2.2s, v0.2s
        adcs    x21, x21, x21
  umull v3.2d, v0.2s, v0.2s
        adc     x10, xzr, xzr
  mov   v1.16b, v6.16b

        mul     x15, x2, x3
  usra  v1.2d, v3.2d, #32
        umulh   x16, x2, x3
  and   v4.16b, v1.16b, v30.16b
        adds    x11, x11, x15
  add   v4.2d, v4.2d, v6.2d
        adcs    x13, x13, x16
  usra  v5.2d, v4.2d, #32
        adc     x14, x14, xzr
  sli   v3.2d, v4.2d, #32
        adds    x11, x11, x15
  usra  v5.2d, v1.2d, #32
        adcs    x13, x13, x16
        adc     x14, x14, xzr
        stp     x12, x11, [x0]
  mov   x11, v5.d[0] // umulh   x11, x4, x4
        adds    x17, x17, x13
  mov   x13, v3.d[1] // mul     x13, x5, x5
        adcs    x19, x19, x14
  mov   x14, v5.d[1] // umulh   x14, x5, x5
        adcs    x20, x20, xzr
  mov   x12, v3.d[0] // mul     x12, x4, x4
        adcs    x21, x21, xzr
// NEON: prepare muls in the upper half
ext     v1.16b, v22.16b, v22.16b, #8
        adc     x10, x10, xzr
shrn    v2.2s, v22.2d, #32
        stp     x17, x19, [x0, #16]
zip1    v0.2s, v22.2s, v1.2s
        mul     x15, x4, x5
umull   v5.2d, v2.2s, v2.2s
        umulh   x16, x4, x5
umull   v6.2d, v2.2s, v0.2s
        adds    x11, x11, x15
umull   v3.2d, v0.2s, v0.2s
        adcs    x13, x13, x16
mov     v1.16b, v6.16b
        adc     x14, x14, xzr
usra    v1.2d, v3.2d, #32
        adds    x11, x11, x15
and     v4.16b, v1.16b, v30.16b
        adcs    x13, x13, x16
add     v4.2d, v4.2d, v6.2d
        adc     x14, x14, xzr
usra    v5.2d, v4.2d, #32
        adds    x12, x12, x20
sli     v3.2d, v4.2d, #32
        adcs    x11, x11, x21
usra    v5.2d, v1.2d, #32
        stp     x12, x11, [x0, #32]
  // NEON: prepare muls in the upper half
  ext   v1.16b, v23.16b, v23.16b, #8
        adcs    x13, x13, x10
  shrn  v2.2s, v23.2d, #32
        adc     x14, x14, xzr
  zip1  v0.2s, v23.2s, v1.2s
        stp     x13, x14, [x0, #48]

// Scalar: square the upper half with a slight variant of the previous block
        mul     x17, x6, x8
  umull v16.2d, v2.2s, v2.2s
        mul     x14, x7, x9
  umull v6.2d, v2.2s, v0.2s
        umulh   x20, x6, x8
  umull v18.2d, v0.2s, v0.2s
        subs    x21, x6, x7
        cneg    x21, x21, cc  // cc = lo, ul, last
  mov   v1.16b, v6.16b
        csetm   x11, cc  // cc = lo, ul, last
        subs    x12, x9, x8
        cneg    x12, x12, cc  // cc = lo, ul, last
  usra  v1.2d, v18.2d, #32
        mul     x13, x21, x12
  and   v4.16b, v1.16b, v30.16b
        umulh   x12, x21, x12
  add   v4.2d, v4.2d, v6.2d
        cinv    x11, x11, cc  // cc = lo, ul, last
        eor     x13, x13, x11
        eor     x12, x12, x11
  usra  v16.2d, v4.2d, #32
        adds    x19, x17, x20
        adc     x20, x20, xzr
  sli   v18.2d, v4.2d, #32
        umulh   x21, x7, x9
        adds    x19, x19, x14
        adcs    x20, x20, x21
        adc     x21, x21, xzr
        adds    x20, x20, x14
mov     x14, v5.d[1]
        adc     x21, x21, xzr
        cmn     x11, #0x1
        adcs    x19, x19, x13
mov     x13, v3.d[1]
        adcs    x20, x20, x12
mov     x12, v3.d[0]
        adc     x21, x21, x11
mov     x11, v5.d[0]
        adds    x17, x17, x17
        adcs    x19, x19, x19
  usra  v16.2d, v1.2d, #32
        adcs    x20, x20, x20
        adcs    x21, x21, x21
        adc     x10, xzr, xzr
// NEON: two mul+umulhs for the next stage
uzp2    v17.4s, v21.4s, v23.4s
        mul     x15, x6, x7
xtn     v4.2s, v23.2d
        umulh   x16, x6, x7
  mov   x22, v16.d[0]
        adds    x11, x11, x15
        adcs    x13, x13, x16
xtn     v5.2s, v21.2d
        adc     x14, x14, xzr
        adds    x11, x11, x15
rev64   v1.4s, v21.4s
        adcs    x13, x13, x16
        adc     x14, x14, xzr
        stp     x12, x11, [x0, #64]
        adds    x17, x17, x13
  mov   x13, v18.d[1]
        adcs    x19, x19, x14
  mov   x14, v16.d[1]
        adcs    x20, x20, xzr
  mov   x12, v18.d[0]
        adcs    x21, x21, xzr
        adc     x10, x10, xzr
umull   v6.2d, v4.2s, v5.2s
        stp     x17, x19, [x0, #80]
umull   v7.2d, v4.2s, v17.2s
        mul     x15, x8, x9
uzp2    v16.4s, v23.4s, v23.4s
        umulh   x16, x8, x9
mul     v0.4s, v1.4s, v23.4s
        adds    x11, x22, x15
        adcs    x13, x13, x16
usra    v7.2d, v6.2d, #32
        adc     x14, x14, xzr
        adds    x11, x11, x15
umull   v1.2d, v16.2s, v17.2s
        adcs    x13, x13, x16
        adc     x14, x14, xzr
uaddlp  v0.2d, v0.4s
        adds    x12, x12, x20
        adcs    x11, x11, x21
and     v2.16b, v7.16b, v30.16b
umlal   v2.2d, v16.2s, v5.2s
shl     v0.2d, v0.2d, #32
usra    v1.2d, v7.2d, #32
umlal   v0.2d, v4.2s, v5.2s
mov     x16, v0.d[1]
mov     x15, v0.d[0]
usra    v1.2d, v2.2d, #32
mov     x20, v1.d[0]
mov     x21, v1.d[1]
        stp     x12, x11, [x0, #96]
        adcs    x13, x13, x10
        adc     x14, x14, xzr
        stp     x13, x14, [x0, #112]

// Now get the cross-product in [s7,...,s0] and double it as [c,s7,...,s0]

        mul     x10, x2, x6
        mul     x14, x3, x7
        umulh   x17, x2, x6
        adds    x14, x14, x17
        umulh   x17, x3, x7
        adcs    x15, x15, x17
        adcs    x16, x16, x20
        adc     x17, x21, xzr
        adds    x11, x14, x10
        adcs    x14, x15, x14
        adcs    x15, x16, x15
        adcs    x16, x17, x16
        adc     x17, xzr, x17
        adds    x12, x14, x10
        adcs    x13, x15, x11
        adcs    x14, x16, x14
        adcs    x15, x17, x15
        adcs    x16, xzr, x16
        adc     x17, xzr, x17
        subs    x22, x4, x5
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x9, x8
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x15, x15, x21
        eor     x20, x20, x19
        adcs    x16, x16, x20
        adc     x17, x17, x19
        subs    x22, x2, x3
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x7, x6
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x11, x11, x21
        eor     x20, x20, x19
        adcs    x12, x12, x20
        adcs    x13, x13, x19
        adcs    x14, x14, x19
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x3, x5
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x9, x7
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x14, x14, x21
        eor     x20, x20, x19
        adcs    x15, x15, x20
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x2, x4
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x8, x6
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x12, x12, x21
        eor     x20, x20, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x19
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x2, x5
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x9, x6
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x13, x13, x21
        eor     x20, x20, x19
        adcs    x14, x14, x20
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x3, x4
        cneg    x22, x22, cc  // cc = lo, ul, last
        csetm   x19, cc  // cc = lo, ul, last
        subs    x20, x8, x7
        cneg    x20, x20, cc  // cc = lo, ul, last
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc  // cc = lo, ul, last
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x13, x13, x21
        eor     x20, x20, x19
        adcs    x14, x14, x20
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        adds    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adc     x19, xzr, xzr

// Add it back to the buffer

        ldp     x2, x3, [x0, #32]
        adds    x10, x10, x2
        adcs    x11, x11, x3
        stp     x10, x11, [x0, #32]

        ldp     x2, x3, [x0, #48]
        adcs    x12, x12, x2
        adcs    x13, x13, x3
        stp     x12, x13, [x0, #48]

        ldp     x2, x3, [x0, #64]
        adcs    x14, x14, x2
        adcs    x15, x15, x3
        stp     x14, x15, [x0, #64]

        ldp     x2, x3, [x0, #80]
        adcs    x16, x16, x2
        adcs    x17, x17, x3
        stp     x16, x17, [x0, #80]

        ldp     x2, x3, [x0, #96]
        adcs    x2, x2, x19
        adcs    x3, x3, xzr
        stp     x2, x3, [x0, #96]

        ldp     x2, x3, [x0, #112]
        adcs    x2, x2, xzr
        adc     x3, x3, xzr
        stp     x2, x3, [x0, #112]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_ksqr_32_64_local_sqr_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
