// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Decode compressed 256-bit form of edwards25519 point
// Input c[32] (bytes); output function return and z[8]
//
// extern uint64_t edwards25519_decode(uint64_t z[static 8], const uint8_t c[static 32]);
//
// This interprets the input byte string as a little-endian number
// representing a point (x,y) on the edwards25519 curve, encoded as
// 2^255 * x_0 + y where x_0 is the least significant bit of x. It
// returns the full pair of coordinates x (at z) and y (at z+4). The
// return code is 0 for success and 1 for failure, which means that
// the input does not correspond to the encoding of any edwards25519
// point. This can happen for three reasons, where y = the lowest
// 255 bits of the input:
//
//  * y >= p_25519
//    Input y coordinate is not reduced
//  * (y^2 - 1) * (1 + d_25519 * y^2) has no modular square root
//    There is no x such that (x,y) is on the curve
//  * y^2 = 1 and top bit of input is set
//    Cannot be the canonical encoding of (0,1) or (0,-1)
//
// Standard ARM ABI: X0 = z, X1 = c
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_decode)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_decode)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_decode)


        .text
        .balign 4

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack

#define y sp, #0
#define s sp, #(4*N)
#define t sp, #(8*N)
#define u sp, #(12*N)
#define v sp, #(16*N)
#define w sp, #(20*N)

// Other temporary variables in register

#define res x19
#define sgnbit x20
#define badun x21

// Total size to reserve on the stack

#define NSPACE 24*N

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

// Macros wrapping up calls to the local subroutines

#define mulp(dest,src1,src2)                                            \
        add     x0, dest __LF                                              \
        add     x1, src1 __LF                                              \
        add     x2, src2 __LF                                              \
        CFI_BL(Ledwards25519_decode_mul_p25519)

#define nsqr(dest,n,src)                                                \
        add     x0, dest __LF                                              \
        mov     x1, n __LF                                                 \
        add     x2, src __LF                                               \
        CFI_BL(Ledwards25519_decode_nsqr_p25519)

S2N_BN_SYMBOL(edwards25519_decode):
        CFI_START

// Save registers and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x30)
        CFI_DEC_SP(NSPACE)

// Save the return pointer for the end so we can overwrite x0 later

        mov     res, x0

// Load the inputs, using byte operations in case of big-endian setting.
// Let y be the lowest 255 bits of the input and sgnbit the desired parity.
// If y >= p_25519 then already flag the input as invalid (badun = 1).

        ldrb    w0, [x1]
        lsl     x4, x0, #56
        ldrb    w0, [x1, #1]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #2]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #3]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #4]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #5]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #6]
        extr    x4, x0, x4, #8
        ldrb    w0, [x1, #7]
        extr    x4, x0, x4, #8

        ldrb    w0, [x1, #8]
        lsl     x5, x0, #56
        ldrb    w0, [x1, #9]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #10]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #11]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #12]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #13]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #14]
        extr    x5, x0, x5, #8
        ldrb    w0, [x1, #15]
        extr    x5, x0, x5, #8

        ldrb    w0, [x1, #16]
        lsl     x6, x0, #56
        ldrb    w0, [x1, #17]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #18]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #19]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #20]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #21]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #22]
        extr    x6, x0, x6, #8
        ldrb    w0, [x1, #23]
        extr    x6, x0, x6, #8

        ldrb    w0, [x1, #24]
        lsl     x7, x0, #56
        ldrb    w0, [x1, #25]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #26]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #27]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #28]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #29]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #30]
        extr    x7, x0, x7, #8
        ldrb    w0, [x1, #31]
        extr    x7, x0, x7, #8

        stp     x4, x5, [y]
        lsr     sgnbit, x7, #63
        and     x7, x7, #0x7FFFFFFFFFFFFFFF
        stp     x6, x7, [y+16]

        adds    xzr, x4, #19
        adcs    xzr, x5, xzr
        adcs    xzr, x6, xzr
        adcs    xzr, x7, xzr
        cset    badun, mi

// u = y^2 - 1 (actually y + 2^255-20, not reduced modulo)
// v = 1 + d * y^2 (not reduced modulo from the +1)
// w = u * v

        nsqr(v,1,y)
        ldp     x0, x1, [v]
        ldp     x2, x3, [v+16]
        mov     x4, #0x8000000000000000
        subs    x0, x0, #20
        sbcs    x1, x1, xzr
        sbcs    x2, x2, xzr
        sbc     x3, x3, x4
        stp     x0, x1, [u]
        stp     x2, x3, [u+16]

        movbig(x0,#0x75eb,#0x4dca,#0x1359,#0x78a3)
        movbig(x1,#0x0070,#0x0a4d,#0x4141,#0xd8ab)
        movbig(x2,#0x8cc7,#0x4079,#0x7779,#0xe898)
        movbig(x3,#0x5203,#0x6cee,#0x2b6f,#0xfe73)
        stp     x0, x1, [w]
        stp     x2, x3, [w+16]
        mulp(v,w,v)
        ldp     x0, x1, [v]
        ldp     x2, x3, [v+16]
        adds    x0, x0, #1
        adcs    x1, x1, xzr
        adcs    x2, x2, xzr
        adcs    x3, x3, xzr
        stp     x0, x1, [v]
        stp     x2, x3, [v+16]

        mulp(w,u,v)

// Get s = w^{252-3} as a candidate inverse square root 1/sqrt(w).
// This power tower computation is the same as bignum_invsqrt_p25519

        nsqr(t,1,w)
        mulp(t,t,w)
        nsqr(s,2,t)
        mulp(t,s,t)
        nsqr(s,1,t)
        mulp(v,s,w)
        nsqr(s,5,v)
        mulp(t,s,v)
        nsqr(s,10,t)
        mulp(t,s,t)
        nsqr(s,5,t)
        mulp(v,s,v)
        nsqr(s,25,v)
        mulp(t,s,v)
        nsqr(s,50,t)
        mulp(t,s,t)
        nsqr(s,25,t)
        mulp(v,s,v)
        nsqr(s,125,v)
        mulp(v,s,v)
        nsqr(s,2,v)
        mulp(s,s,w)

// Compute v' = s^2 * w to discriminate whether the square root sqrt(u/v)
// exists, in which case we should get 0, 1 or -1.

        nsqr(v,1,s)
        mulp(v,v,w)

// Get the two candidates for sqrt(u / v), one being s = u * w^{252-3}
// and the other being t = s * j_25519 where j_25519 = sqrt(-1).

        mulp(s,u,s)
        movbig(x0, #0xc4ee, #0x1b27, #0x4a0e, #0xa0b0)
        movbig(x1, #0x2f43, #0x1806, #0xad2f, #0xe478)
        movbig(x2, #0x2b4d, #0x0099, #0x3dfb, #0xd7a7)
        movbig(x3, #0x2b83, #0x2480, #0x4fc1, #0xdf0b)
        stp     x0, x1, [t]
        stp     x2, x3, [t+16]
        mulp(t,s,t)

// x4 = 0 <=> s^2 * w = 0 or 1

        ldp     x0, x1, [v]
        ldp     x2, x3, [v+16]
        bic     x4, x0, #1
        orr     x4, x4, x1
        orr     x5, x2, x3
        orr     x4, x4, x5

// x0 = 0 <=> s^2 * w = -1 (mod p_25519, i.e. s^2 * w = 2^255 - 20)

        add     x0, x0, #20
        add     x1, x1, #1
        orr     x0, x0, x1
        add     x2, x2, #1
        eor     x3, x3, #0x7FFFFFFFFFFFFFFF
        orr     x2, x2, x3
        orr     x0, x0, x2

// If s^2 * w is not 0 or 1 then replace s by t

        cmp     x4, xzr
        ldp     x10, x11, [s]
        ldp     x14, x15, [t]
        csel    x10, x10, x14, eq
        csel    x11, x11, x15, eq
        ldp     x12, x13, [s+16]
        ldp     x16, x17, [t+16]
        csel    x12, x12, x16, eq
        csel    x13, x13, x17, eq
        stp     x10, x11, [s]
        stp     x12, x13, [s+16]

// Check invalidity, occurring if s^2 * w is not in {0,1,-1}

        ccmp    x0, xzr, 4, ne
        cset    x0, ne
        orr     badun, badun, x0

// Let [x3;x2;x1;x0] = s and [x7;x6;x5;x4] = p_25519 - s

        ldp     x0, x1, [s]
        ldp     x2, x3, [s+16]
        mov     x4, #-19
        subs    x4, x4, x0
        mov     x6, #-1
        sbcs    x5, x6, x1
        sbcs    x6, x6, x2
        mov     x7, #0x7FFFFFFFFFFFFFFF
        sbc     x7, x7, x3

// Decide whether a flip is apparently indicated, s_0 <=> sgnbit
// Decide also if s = 0 by OR-ing its digits. Now if a flip is indicated:
//  - if s = 0 then mark as invalid
//  - if s <> 0 then indeed flip

        and     x9, x0, #1
        eor     sgnbit, x9, sgnbit
        orr     x8, x0, x1
        orr     x9, x2, x3
        orr     x8, x8, x9
        orr     x10, badun, sgnbit
        cmp     x8, xzr
        csel    badun, x10, badun, eq
        ccmp    sgnbit, xzr, #4, ne

// Actual selection of x as s or -s, copying of y and return of validity

        csel    x0, x0, x4, eq
        csel    x1, x1, x5, eq
        csel    x2, x2, x6, eq
        csel    x3, x3, x7, eq
        ldp     x8, x9, [y]
        ldp     x10, x11, [y+16]

        stp     x0, x1, [res]
        stp     x2, x3, [res, #16]
        stp     x8, x9, [res, #32]
        stp     x10, x11, [res, #48]

        mov     x0, badun

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_decode)

// *************************************************************
// Local z = x * y
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_decode_mul_p25519)

Ledwards25519_decode_mul_p25519:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x5, x6, [x2]
        umull   x7, w3, w5
        lsr     x17, x3, #32
        umull   x15, w17, w5
        lsr     x16, x5, #32
        umull   x8, w16, w17
        umull   x16, w3, w16
        adds    x7, x7, x15, lsl #32
        lsr     x15, x15, #32
        adc     x8, x8, x15
        adds    x7, x7, x16, lsl #32
        lsr     x16, x16, #32
        adc     x8, x8, x16
        mul     x9, x4, x6
        umulh   x10, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x16, lo
        adds    x9, x9, x8
        adc     x10, x10, xzr
        subs    x3, x5, x6
        cneg    x3, x3, lo
        cinv    x16, x16, lo
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x8, x7, x9
        adcs    x9, x9, x10
        adc     x10, x10, xzr
        cmn     x16, #1
        eor     x15, x15, x16
        adcs    x8, x15, x8
        eor     x3, x3, x16
        adcs    x9, x3, x9
        adc     x10, x10, x16
        ldp     x3, x4, [x1, #16]
        ldp     x5, x6, [x2, #16]
        umull   x11, w3, w5
        lsr     x17, x3, #32
        umull   x15, w17, w5
        lsr     x16, x5, #32
        umull   x12, w16, w17
        umull   x16, w3, w16
        adds    x11, x11, x15, lsl #32
        lsr     x15, x15, #32
        adc     x12, x12, x15
        adds    x11, x11, x16, lsl #32
        lsr     x16, x16, #32
        adc     x12, x12, x16
        mul     x13, x4, x6
        umulh   x14, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x16, lo
        adds    x13, x13, x12
        adc     x14, x14, xzr
        subs    x3, x5, x6
        cneg    x3, x3, lo
        cinv    x16, x16, lo
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x12, x11, x13
        adcs    x13, x13, x14
        adc     x14, x14, xzr
        cmn     x16, #1
        eor     x15, x15, x16
        adcs    x12, x15, x12
        eor     x3, x3, x16
        adcs    x13, x3, x13
        adc     x14, x14, x16
        ldp     x3, x4, [x1, #16]
        ldp     x15, x16, [x1]
        subs    x3, x3, x15
        sbcs    x4, x4, x16
        csetm   x16, lo
        ldp     x15, x17, [x2]
        subs    x5, x15, x5
        sbcs    x6, x17, x6
        csetm   x17, lo
        eor     x3, x3, x16
        subs    x3, x3, x16
        eor     x4, x4, x16
        sbc     x4, x4, x16
        eor     x5, x5, x17
        subs    x5, x5, x17
        eor     x6, x6, x17
        sbc     x6, x6, x17
        eor     x16, x17, x16
        adds    x11, x11, x9
        adcs    x12, x12, x10
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        mul     x2, x3, x5
        umulh   x17, x3, x5
        mul     x15, x4, x6
        umulh   x1, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x9, lo
        adds    x15, x15, x17
        adc     x1, x1, xzr
        subs    x6, x5, x6
        cneg    x6, x6, lo
        cinv    x9, x9, lo
        mul     x5, x4, x6
        umulh   x6, x4, x6
        adds    x17, x2, x15
        adcs    x15, x15, x1
        adc     x1, x1, xzr
        cmn     x9, #1
        eor     x5, x5, x9
        adcs    x17, x5, x17
        eor     x6, x6, x9
        adcs    x15, x6, x15
        adc     x1, x1, x9
        adds    x9, x11, x7
        adcs    x10, x12, x8
        adcs    x11, x13, x11
        adcs    x12, x14, x12
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        cmn     x16, #1
        eor     x2, x2, x16
        adcs    x9, x2, x9
        eor     x17, x17, x16
        adcs    x10, x17, x10
        eor     x15, x15, x16
        adcs    x11, x15, x11
        eor     x1, x1, x16
        adcs    x12, x1, x12
        adcs    x13, x13, x16
        adc     x14, x14, x16
        mov     x3, #38
        umull   x4, w11, w3
        add     x4, x4, w7, uxtw
        lsr     x7, x7, #32
        lsr     x11, x11, #32
        umaddl  x11, w11, w3, x7
        mov     x7, x4
        umull   x4, w12, w3
        add     x4, x4, w8, uxtw
        lsr     x8, x8, #32
        lsr     x12, x12, #32
        umaddl  x12, w12, w3, x8
        mov     x8, x4
        umull   x4, w13, w3
        add     x4, x4, w9, uxtw
        lsr     x9, x9, #32
        lsr     x13, x13, #32
        umaddl  x13, w13, w3, x9
        mov     x9, x4
        umull   x4, w14, w3
        add     x4, x4, w10, uxtw
        lsr     x10, x10, #32
        lsr     x14, x14, #32
        umaddl  x14, w14, w3, x10
        mov     x10, x4
        lsr     x17, x14, #31
        mov     x5, #19
        umaddl  x5, w5, w17, x5
        add     x7, x7, x5
        adds    x7, x7, x11, lsl #32
        extr    x3, x12, x11, #32
        adcs    x8, x8, x3
        extr    x3, x13, x12, #32
        adcs    x9, x9, x3
        extr    x3, x14, x13, #32
        lsl     x5, x17, #63
        eor     x10, x10, x5
        adc     x10, x10, x3
        mov     x3, #19
        tst     x10, #0x8000000000000000
        csel    x3, x3, xzr, pl
        subs    x7, x7, x3
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        and     x10, x10, #0x7fffffffffffffff
        stp     x7, x8, [x0]
        stp     x9, x10, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_decode_mul_p25519)

// *************************************************************
// Local z = 2^n * x
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_decode_nsqr_p25519)

Ledwards25519_decode_nsqr_p25519:
        CFI_START

// Copy input argument into [x13;x12;x11;x10]

        ldp     x10, x11, [x2]
        ldp     x12, x13, [x2, #16]

// Main squaring loop, accumulating in [x13;x12;x11;x10] consistently and
// only ensuring the intermediates are < 2 * p_25519 = 2^256 - 38

Ledwards25519_decode_loop:
        umull   x2, w10, w10
        lsr     x14, x10, #32
        umull   x3, w14, w14
        umull   x14, w10, w14
        adds    x2, x2, x14, lsl #33
        lsr     x14, x14, #31
        adc     x3, x3, x14
        umull   x4, w11, w11
        lsr     x14, x11, #32
        umull   x5, w14, w14
        umull   x14, w11, w14
        mul     x15, x10, x11
        umulh   x16, x10, x11
        adds    x4, x4, x14, lsl #33
        lsr     x14, x14, #31
        adc     x5, x5, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x5, x5, xzr
        adds    x3, x3, x15
        adcs    x4, x4, x16
        adc     x5, x5, xzr
        umull   x6, w12, w12
        lsr     x14, x12, #32
        umull   x7, w14, w14
        umull   x14, w12, w14
        adds    x6, x6, x14, lsl #33
        lsr     x14, x14, #31
        adc     x7, x7, x14
        umull   x8, w13, w13
        lsr     x14, x13, #32
        umull   x9, w14, w14
        umull   x14, w13, w14
        mul     x15, x12, x13
        umulh   x16, x12, x13
        adds    x8, x8, x14, lsl #33
        lsr     x14, x14, #31
        adc     x9, x9, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x9, x9, xzr
        adds    x7, x7, x15
        adcs    x8, x8, x16
        adc     x9, x9, xzr
        subs    x10, x10, x12
        sbcs    x11, x11, x13
        csetm   x16, lo
        eor     x10, x10, x16
        subs    x10, x10, x16
        eor     x11, x11, x16
        sbc     x11, x11, x16
        adds    x6, x6, x4
        adcs    x7, x7, x5
        adcs    x8, x8, xzr
        adc     x9, x9, xzr
        umull   x12, w10, w10
        lsr     x5, x10, #32
        umull   x13, w5, w5
        umull   x5, w10, w5
        adds    x12, x12, x5, lsl #33
        lsr     x5, x5, #31
        adc     x13, x13, x5
        umull   x15, w11, w11
        lsr     x5, x11, #32
        umull   x14, w5, w5
        umull   x5, w11, w5
        mul     x4, x10, x11
        umulh   x16, x10, x11
        adds    x15, x15, x5, lsl #33
        lsr     x5, x5, #31
        adc     x14, x14, x5
        adds    x4, x4, x4
        adcs    x16, x16, x16
        adc     x14, x14, xzr
        adds    x13, x13, x4
        adcs    x15, x15, x16
        adc     x14, x14, xzr
        adds    x4, x2, x6
        adcs    x5, x3, x7
        adcs    x6, x6, x8
        adcs    x7, x7, x9
        csetm   x16, lo
        subs    x4, x4, x12
        sbcs    x5, x5, x13
        sbcs    x6, x6, x15
        sbcs    x7, x7, x14
        adcs    x8, x8, x16
        adc     x9, x9, x16
        mov     x10, #38
        umull   x12, w6, w10
        add     x12, x12, w2, uxtw
        lsr     x2, x2, #32
        lsr     x6, x6, #32
        umaddl  x6, w6, w10, x2
        mov     x2, x12
        umull   x12, w7, w10
        add     x12, x12, w3, uxtw
        lsr     x3, x3, #32
        lsr     x7, x7, #32
        umaddl  x7, w7, w10, x3
        mov     x3, x12
        umull   x12, w8, w10
        add     x12, x12, w4, uxtw
        lsr     x4, x4, #32
        lsr     x8, x8, #32
        umaddl  x8, w8, w10, x4
        mov     x4, x12
        umull   x12, w9, w10
        add     x12, x12, w5, uxtw
        lsr     x5, x5, #32
        lsr     x9, x9, #32
        umaddl  x9, w9, w10, x5
        mov     x5, x12
        lsr     x13, x9, #31
        mov     x11, #19
        umull   x11, w11, w13
        add     x2, x2, x11
        adds    x10, x2, x6, lsl #32
        extr    x12, x7, x6, #32
        adcs    x11, x3, x12
        extr    x12, x8, x7, #32
        adcs    x12, x4, x12
        extr    x14, x9, x8, #32
        lsl     x15, x13, #63
        eor     x5, x5, x15
        adc     x13, x5, x14

// Loop as applicable

        subs    x1, x1, #1
        bne     Ledwards25519_decode_loop

// We know the intermediate result x < 2^256 - 38, and now we do strict
// modular reduction mod 2^255 - 19. Note x < 2^255 - 19 <=> x + 19 < 2^255
// which is equivalent to a "pl" condition.

        adds    x6, x10, #19
        adcs    x7, x11, xzr
        adcs    x8, x12, xzr
        adcs    x9, x13, xzr

        csel    x10, x10, x6, pl
        csel    x11, x11, x7, pl
        csel    x12, x12, x8, pl
        csel    x13, x13, x9, pl
        bic     x13, x13, #0x8000000000000000

// Copy result back into destination and return

        stp     x10, x11, [x0]
        stp     x12, x13, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_decode_nsqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
